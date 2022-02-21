
ulong __stdcall16far pass1_1010_8ebc(ulong param_1, byte param_2)

{
    ushort unaff_SS;

    pass1_1010_8c78((ushort *)param_1, unaff_SS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1010_9044(ulong param_1)

{
    code **ppcVar1;

    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0x10);
    (**ppcVar1)();
    return;
}


void __stdcall16far fn_ptr_1010_905e(ulong param_1, ulong param_2)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    astruct_169 *iVar4;
    undefined2   uVar4;

    uVar4  = (undefined2)(param_1 >> 0x10);
    iVar4  = (astruct_169 *)param_1;
    puVar1 = *(undefined4 **)&iVar4->field_0x4;
    uVar2  = *(uint *)((int)&iVar4->field_0x4 + 0x2);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    iVar4->field_0x4 = param_2;
    return;
}


void __stdcall16far pass1_1010_9092(ulong param_1, ushort param_2, ushort param_3)

{
    code     **ppcVar1;
    ulong      uVar2;
    uchar     *extraout_DX;
    uchar     *puVar3;
    uchar     *puVar4;
    uint       extraout_DX_00;
    int        iVar5;
    undefined2 uVar6;
    undefined2 uVar7;
    undefined4 uVar8;
    ulong      uStack14;
    ulong      uStack6;

    uVar6   = (undefined2)(param_1 >> 0x10);
    iVar5   = (int)param_1;
    uVar8   = *(undefined4 *)(iVar5 + 0x4);
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar5 + 0x4) + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(extraout_DX, param_2);
    uVar7   = 0xc;
    puVar3  = extraout_DX;
    mem_op_1000_179c(0xc, extraout_DX, 0x1000);
    puVar4 = (uchar *)((uint)puVar3 | param_2);
    if(puVar4 == (uchar *)0x0)
    {
        param_2 = 0x0;
        puVar4  = (uchar *)0x0;
    }
    else
    {
        pass1_1010_8ef2((ushort *)CONCAT22(puVar3, param_2), puVar4, param_3);
    }
    for(uStack14 = 0x0; uStack14 < uStack6; uStack14 = uStack14 + 0x1)
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar5 + 0x4) + 0x4);
        uVar2   = uStack6;
        (**ppcVar1)(0x1000, *(undefined4 *)(iVar5 + 0x4), uStack14, uVar7, uVar8);
        if((extraout_DX_00 | (uint)uVar2) != 0x0)
        {
            ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_2 + 0x4) + 0xc);
            (**ppcVar1)(0x1000, *(undefined4 *)(param_2 + 0x4), (uint)uVar2, extraout_DX_00);
        }
    }
    return;
}

ushort *__stdcall16far pass1_1010_922e(ushort *param_1, byte param_2)

{
    pass1_1010_8f78(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

ushort *__stdcall16far pass1_1010_951a(ushort *param_1, byte param_2)

{
    pass1_1010_927a(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}


ushort *__stdcall16far pass1_1010_9540(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1010_92e6(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

ulong __stdcall16far pass1_1010_7dc6(ulong param_1, byte param_2)

{
    undefined2 unaff_SS;

    param_1 = param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xa);
    pass1_1010_6bb2((ushort *)param_1, unaff_SS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}


ushort *__stdcall16far pass1_1010_7dd2(ushort *param_1, byte param_2)

{
    *param_1                            = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1010_7efc(ulong *param_1, ushort param_2)

{
    uint         uVar1;
    uint         uVar2;
    undefined4  *puVar3;
    code       **ppcVar4;
    astruct_448 *iVar5;
    undefined2   uVar5;
    astruct_18  *paStack8;
    int          iStack4;

    uVar5    = (undefined2)((ulong)param_1 >> 0x10);
    iVar5    = (astruct_448 *)param_1;
    uVar1    = iVar5->field_0x67c;
    uVar2    = iVar5->field_0x67e;
    paStack8 = (astruct_18 *)CONCAT22(uVar2, uVar1);
    if((uVar2 | uVar1) != 0x0)
    {
        pass1_1008_64a2((uint *)CONCAT22(uVar2, uVar1));
        param_2 = 0x1000;
        fn_ptr_1000_17ce(paStack8, 0x1000);
    }
    for(iStack4 = 0x0; iStack4 < 0x8a; iStack4 = iStack4 + 0x1)
    {
        puVar3 = (undefined4 *)*(uint *)(&iVar5->field_0x4 + iStack4 * 0x4);
        uVar1  = *(uint *)(&iVar5->field_0x4 + iStack4 * 0x4 + 0x2);
        if((uVar1 | (uint)puVar3) != 0x0)
        {
            ppcVar4 = (code **)*puVar3;
            (**ppcVar4)(param_2, puVar3, uVar1, 0x1);
        }
        puVar3 = (undefined4 *)*(uint *)(&iVar5->field_0x22c + iStack4 * 0x4);
        uVar1  = *(uint *)(&iVar5->field_0x22c + iStack4 * 0x4 + 0x2);
        if((uVar1 | (uint)puVar3) != 0x0)
        {
            ppcVar4 = (code **)*puVar3;
            (**ppcVar4)(param_2, puVar3);
        }
        puVar3 = (undefined4 *)*(uint *)(&iVar5->field_0x454 + iStack4 * 0x4);
        uVar1  = *(uint *)(&iVar5->field_0x454 + iStack4 * 0x4 + 0x2);
        if((uVar1 | (uint)puVar3) != 0x0)
        {
            ppcVar4 = (code **)*puVar3;
            (**ppcVar4)(param_2, puVar3);
        }
    }
    fn_ptr_1000_17ce((astruct_18 *)*param_1, 0x1000);
    _PTR_LOOP_1050_14cc = 0x0;
    return;
}


void __stdcall16far pass1_1010_7fd6(ulong param_1)

{
    uint         uVar1;
    uint         uVar2;
    astruct_489 *iVar3;
    undefined2   uVar3;
    astruct_18  *paStack6;

    uVar3    = (undefined2)(param_1 >> 0x10);
    iVar3    = (astruct_489 *)param_1;
    uVar1    = iVar3->field_0x67c;
    uVar2    = iVar3->field_0x67e;
    paStack6 = (astruct_18 *)CONCAT22(uVar2, uVar1);
    if((uVar2 | uVar1) != 0x0)
    {
        pass1_1008_64a2((uint *)CONCAT22(uVar2, uVar1));
        fn_ptr_1000_17ce(paStack6, 0x1000);
    }
    *(undefined4 *)&iVar3->field_0x67c = 0x0;
    iVar3->field_0x680                 = 0x0;
    return;
}

ushort *__stdcall16far pass1_1010_66ca(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1010_1d80(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

ushort *__stdcall16far pass1_1010_6a86(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1010_1d80(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1010_533c(ulong param_1, ulong param_2, uchar *param_3, ushort param_4)

{
    uint       *puVar1;
    ulong       uVar2;
    undefined4  uVar3;
    int         iVar4;
    ushort      uVar5;
    ushort      uVar6;
    astruct_18 *paVar7;
    uint        uStack6;
    undefined   local_4[0x2];

    pass1_1010_519a(param_1, (int *)CONCAT22(param_4, local_4), param_3, param_4);
    uStack6 = 0x0;
    while(true)
    {
        uVar6  = (ushort)(param_1 >> 0x10);
        uVar5  = (ushort)param_1;
        puVar1 = (uint *)(uVar5 + 0x10);
        if(*puVar1 < uStack6 || *puVar1 == uStack6)
        {
            return;
        }
        uVar3   = *(undefined4 *)(uVar5 + 0xc);
        uVar2   = *(ulong *)((int)uVar3 + uStack6 * 0x4);
        paVar7  = (astruct_18 *)string_1010_5286(uVar5, uVar6, uVar2, (char *)uVar2, (uint)param_3);
        param_3 = (uchar *)((ulong)paVar7 >> 0x10);
        iVar4   = pass1_1000_3d7a(param_2, (ulong)paVar7 & 0xffff | ZEXT24(param_3) << 0x10);
        if(iVar4 == 0x0)
            break;
        fn_ptr_1000_17ce(paVar7, 0x1000);
        uStack6 = uStack6 + 0x1;
    }
    return;
}


ushort *__stdcall16far pass1_1010_53ce(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1010_50f2(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far struct_1010_5f1e(astruct_73 *param_1, char *param_2, ushort param_3)

{
    ushort      uVar1;
    astruct_73 *iVar3;
    astruct_73 *uVar3;

    uVar3 = (astruct_73 *)((ulong)param_1 >> 0x10);
    iVar3 = (astruct_73 *)param_1;
    fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0x16, 0x1000);
    uVar1             = str_op_1008_60e8(param_2, param_3);
    iVar3->field_0x16 = uVar1;
    iVar3->field_0x18 = param_3;
    return;
}


void __stdcall16far pass1_1010_5f4c(ulong param_1, char *param_2, ushort param_3)

{
    ushort       uVar1;
    astruct_484 *iVar3;
    undefined2   uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar3 = (astruct_484 *)param_1;
    fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0x12, 0x1000);
    uVar1             = str_op_1008_60e8(param_2, param_3);
    iVar3->field_0x12 = uVar1;
    iVar3->field_0x14 = param_3;
    return;
}

void __stdcall16far pass1_1010_5fd8(ulong param_1, char *param_2, ushort param_3)

{
    ushort       uVar1;
    astruct_485 *iVar3;
    undefined2   uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar3 = (astruct_485 *)param_1;
    fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0x68, 0x1000);
    uVar1             = str_op_1008_60e8(param_2, param_3);
    iVar3->field_0x68 = uVar1;
    iVar3->field_0x6a = param_3;
    return;
}


void __stdcall16far pass1_1010_6006(ulong param_1, char *param_2, ushort param_3)

{
    ushort       uVar1;
    astruct_486 *iVar3;
    undefined2   uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar3 = (astruct_486 *)param_1;
    fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0x6c, 0x1000);
    uVar1             = str_op_1008_60e8(param_2, param_3);
    iVar3->field_0x6c = uVar1;
    iVar3->field_0x6e = param_3;
    return;
}

void __stdcall16far pass1_1010_60cc(ulong param_1, char *param_2, ushort param_3)

{
    ushort       uVar1;
    astruct_487 *iVar3;
    undefined2   uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar3 = (astruct_487 *)param_1;
    fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0x1a, 0x1000);
    uVar1             = str_op_1008_60e8(param_2, param_3);
    iVar3->field_0x1a = uVar1;
    iVar3->field_0x1c = param_3;
    return;
}

void __stdcall16far pass1_1010_62a4(ushort *param_1, byte param_2)

{
    astruct_488 *uVar2;
    undefined2   uVar1;

    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    uVar2            = (astruct_488 *)param_1;
    *param_1         = 0x6322;
    uVar2->field_0x2 = 0x1010;
    fn_ptr_1000_17ce((astruct_18 *)uVar2->field_0x4, 0x1000);
    *param_1         = 0x389a;
    uVar2->field_0x2 = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return;
}
astruct_18 *pass1_1010_4994(ushort param_1, astruct_18 *param_2, byte param_3, ushort param_4)

{
    param_2 = (astruct_18 *)((ulong)param_2 & 0xffff0000 | (ulong)((int)param_2 - 0x20));
    pass1_1010_3f00((ushort *)param_2, param_4);
    if((param_3 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_2, 0x1000);
    }
    return param_2;
}

ushort *__stdcall16far pass1_1010_4a20(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1010_3f00(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1010_50f2(ushort *param_1, ushort param_2)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    *param_1                            = 0x53f4;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
    fn_ptr_1000_17ce(*(astruct_18 **)((int)param_1 + 0xc), 0x1000);
    pass1_1010_1d80(param_1, param_2);
    return;
}

ushort *__stdcall16far pass1_1010_36b4(ushort *param_1, byte param_2)

{
    ushort unaff_SS;

    pass1_1010_2db2(param_1, unaff_SS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1010_3730(ushort *param_1, ushort param_2)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    *param_1                            = 0x37c4;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
    fn_ptr_1000_17ce(*(astruct_18 **)((int)param_1 + 0xa), 0x1000);
    pass1_1010_1d80(param_1, param_2);
    return;
}

void __stdcall16far pass1_1010_3770(ulong param_1, char *param_2, ushort param_3)

{
    ushort       uVar1;
    astruct_477 *iVar3;
    undefined2   uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar3 = (astruct_477 *)param_1;
    fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0xa, 0x1000);
    uVar1            = str_op_1008_60e8(param_2, param_3);
    iVar3->field_0xa = uVar1;
    iVar3->field_0xc = param_3;
    return;
}

ushort *__stdcall16far pass1_1010_379e(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1010_3730(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1010_3800(ushort *param_1)

{
    astruct_478 *iVar2;
    undefined2   uVar1;

    uVar1            = (undefined2)((ulong)param_1 >> 0x10);
    iVar2            = (astruct_478 *)param_1;
    *param_1         = 0x3b3e;
    iVar2->field_0x2 = 0x1010;
    if(iVar2->field_0x16 != (undefined4 *)0x0)
    {
        fn_ptr_1000_17ce(*(astruct_18 **)iVar2->field_0x16, 0x1000);
    }
    pass1_1010_3880(param_1);
    return;
}
void __stdcall16far pass1_1010_3880(ushort *param_1)

{
    int         *piVar1;
    undefined4  *puVar2;
    uint         uVar3;
    code       **ppcVar4;
    long         lVar5;
    astruct_472 *iVar6;
    int          iVar7;
    undefined2   uVar8;
    undefined2   uVar9;
    int          iStack4;

    uVar8            = (undefined2)((ulong)param_1 >> 0x10);
    iVar6            = (astruct_472 *)param_1;
    *param_1         = 0x3b5e;
    iVar6->field_0x2 = 0x1010;
    if(iVar6->field_0x8 != 0x0)
    {
        iStack4 = 0x0;
        while(true)
        {
            piVar1 = &iVar6->field_0x10;
            if(*piVar1 == iStack4 || *piVar1 < iStack4)
                break;
            lVar5  = iVar6->field_0x8;
            uVar9  = (undefined2)((ulong)lVar5 >> 0x10);
            iVar7  = (int)lVar5;
            puVar2 = (undefined4 *)*(uint *)(iVar7 + iStack4 * 0x4);
            uVar3  = *(uint *)(iVar7 + iStack4 * 0x4 + 0x2);
            if((uVar3 | (uint)puVar2) != 0x0)
            {
                ppcVar4 = (code **)*puVar2;
                (**ppcVar4)();
            }
            iStack4 = iStack4 + 0x1;
        }
        fn_ptr_1000_17ce((astruct_18 *)iVar6->field_0x8, 0x1000);
    }
    *param_1         = 0x389a;
    iVar6->field_0x2 = 0x1008;
    return;
}

ushort *__stdcall16far pass1_1010_3af2(ushort *param_1, byte param_2)

{
    pass1_1010_3800(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}


ushort *__stdcall16far pass1_1010_3b18(ushort *param_1, byte param_2)

{
    pass1_1010_3880(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

ushort *__stdcall16far pass1_1010_3d38(ushort *param_1, byte param_2)

{
    ushort unaff_SS;

    param_1 = (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xa));
    pass1_1010_3bde(param_1, unaff_SS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

ushort *__stdcall16far pass1_1010_3e06(ushort *param_1, byte param_2)

{
    ushort unaff_SS;

    pass1_1010_3dc8(param_1, unaff_SS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1010_3f00(ushort *param_1, ushort param_2)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    int         *piVar4;
    astruct_481 *iVar5;
    undefined2   uVar5;
    undefined2  *puStack16;
    int          iStack4;

    uVar5             = (undefined2)((ulong)param_1 >> 0x10);
    iVar5             = (astruct_481 *)param_1;
    *param_1          = (ushort)&PTR_LOOP_1050_4a46;
    iVar5->field_0x2  = 0x1010;
    iVar5->field_0x20 = (int)&PTR_LOOP_1050_4a82;
    iVar5->field_0x22 = 0x1010;
    iStack4           = 0x0;
    do
    {
        puVar1 = (undefined4 *)*(uint *)(&iVar5->field_0x26 + iStack4 * 0x4);
        uVar2  = *(uint *)(&iVar5->field_0x26 + iStack4 * 0x4 + 0x2);
        if((uVar2 | (uint)puVar1) != 0x0)
        {
            ppcVar3 = (code **)*puVar1;
            (**ppcVar3)();
        }
        iStack4 = iStack4 + 0x1;
    } while(iStack4 < 0x10);
    puVar1 = iVar5->field_0x66;
    uVar2  = iVar5->field_0x68;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x70, 0x1000);
    if(param_1 == (ushort *)0x0)
    {
        piVar4 = (int *)0x0;
        uVar5  = 0x0;
    }
    else
    {
        piVar4 = &iVar5->field_0x20;
    }
    puStack16   = (undefined2 *)CONCAT22(uVar5, piVar4);
    *puStack16  = 0x389a;
    piVar4[0x1] = 0x1008;
    pass1_1010_1d80(param_1, param_2);
    return;
}

astruct_11 *__stdcall16far pass1_1010_2bbe(astruct_11 *param_1, byte param_2)

{
    pass1_1010_29c6(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

ushort *__stdcall16far pass1_1010_2c9c(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1010_1d80(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

void __stdcall16far pass1_1010_2db2(ushort *param_1, ushort param_2)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    astruct_473 *iVar5;
    undefined2   uVar4;

    uVar4            = (undefined2)((ulong)param_1 >> 0x10);
    iVar5            = (astruct_473 *)param_1;
    *param_1         = 0x36da;
    iVar5->field_0x2 = 0x1010;
    puVar1           = iVar5->field_0x56;
    uVar2            = iVar5->field_0x58;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x5c, 0x1000);
    pass1_1010_1d80(param_1, param_2);
    return;
}

ushort *__stdcall16far pass1_1010_18f4(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1010_0f76(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

ushort *__stdcall16far pass1_1010_1b04(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1010_0f76(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

ushort *__stdcall16far pass1_1010_1cde(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1010_0f76(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

ushort *__stdcall16far pass1_1010_1fbe(ushort *param_1, byte param_2)

{
    *param_1                            = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}


ushort *__stdcall16far pass1_1010_1fea(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1010_1d80(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

ushort *__stdcall16far pass1_1010_0e46(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1010_0350(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}


ushort *__stdcall16far pass1_1010_0e6c(ushort *param_1, byte param_2)

{
    *param_1                            = 0x389a;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

ushort *__stdcall16far pass1_1008_ea86(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1008_ddca(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

ushort *__stdcall16far pass1_1008_eaf4(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1010_1d80(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}


ushort *__stdcall16far pass1_1008_ebda(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1010_1d80(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

ushort *__stdcall16far pass1_1008_ec3c(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1010_1d80(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

ushort *__stdcall16far pass1_1008_ef50(ushort *param_1, byte param_2)

{
    pass1_1008_ec94(param_1);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}


astruct_18 *__stdcall16far pass1_1008_ef76(astruct_18 *param_1, byte param_2)

{
    ushort unaff_SS;

    pass1_1008_ed00(&param_1->field_0x0, unaff_SS);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce(param_1, 0x1000);
    }
    return param_1;
}


ushort *__stdcall16far pass1_1010_02a2(ushort *param_1, byte param_2, ushort param_3)

{
    pass1_1010_0052(param_1, param_3);
    if((param_2 & 0x1) != 0x0)
    {
        fn_ptr_1000_17ce((astruct_18 *)param_1, 0x1000);
    }
    return param_1;
}

ushort * __stdcall16far pass1_1008_d6f4(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1008_caa0(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


ushort * __stdcall16far pass1_1008_d75a(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


ushort * __stdcall16far pass1_1008_d968(ushort *param_1,byte param_2)

{
  ushort unaff_SS;
  
  pass1_1008_d7da(param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}

astruct_11 * __stdcall16far pass1_1008_d9d4(astruct_11 *param_1,byte param_2)

{
  clenaup_win_ui_1018_4d22(param_1,0x1018);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


void __stdcall16far pass1_1008_dc2c(ushort *param_1,ushort param_2)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0xdc80;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  fn_ptr_1000_17ce(*(astruct_18 **)((int)param_1 + 0x18),0x1000);
  pass1_1010_1d80(param_1,param_2);
  return;
}


ushort * __stdcall16far pass1_1008_dc5a(ushort *param_1,byte param_2)

{
  ushort unaff_SS;
  
  pass1_1008_dc2c(param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_dd1e(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


void __stdcall16far pass1_1008_c626(ulong *param_1)

{
  _PTR_LOOP_1050_06e0 = 0x0;
  fn_ptr_1000_17ce((astruct_18 *)*param_1,0x1000);
  return;
}



ulong __stdcall16far pass1_1008_ca24(ulong param_1,byte param_2,ushort param_3)

{
  pass1_1008_c75c((ushort *)param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ulong __stdcall16far pass1_1008_bd02(ulong param_1,byte param_2)

{
  ushort unaff_SS;
  
  pass1_1008_afde((ushort *)param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_bd28(ushort *param_1,byte param_2)

{
  pass1_1008_b08c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_18 * __stdcall16far pass1_1008_bd4e(astruct_18 *param_1,byte param_2)

{
  pass1_1008_b08c(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_18 * __stdcall16far pass1_1008_bd74(astruct_18 *param_1,byte param_2)

{
  pass1_1008_b08c(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_18 * __stdcall16far pass1_1008_bd9a(astruct_18 *param_1,byte param_2)

{
  pass1_1008_b08c(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_ad38(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ulong __stdcall16far pass1_1008_ad64(ulong param_1,byte param_2)

{
  ushort unaff_SS;
  
  pass1_1008_a086((ushort *)param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ulong __stdcall16far pass1_1008_af56(ulong param_1,byte param_2)

{
  pass1_1008_af38((astruct_11 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


void __stdcall16far pass1_1008_b08c(ushort *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0xbdc8;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x4),0x1000);
  *param_1 = 0x389a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  return;
}


void __stdcall16far pass1_1008_b1a6(ulong param_1,char *param_2)

{
  long lVar1;
  ushort uVar2;
  ushort in_DX;
  astruct_467 *iVar3;
  astruct_466 *iVar4;
  undefined2 uVar3;
  undefined2 uVar4;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar3 = (astruct_467 *)param_1;
  if (iVar3->field_0x16 != 0x0) {
    lVar1 = iVar3->field_0x16;
    fn_ptr_1000_17ce(*(astruct_18 **)((int)lVar1 + 0x4),0x1000);
    uVar2 = str_op_1008_60e8(param_2,in_DX);
    lVar1 = iVar3->field_0x16;
    uVar4 = (undefined2)((ulong)lVar1 >> 0x10);
    iVar4 = (astruct_466 *)lVar1;
    iVar4->field_0x4 = uVar2;
    iVar4->field_0x6 = in_DX;
    iVar3->field_0x16 = 0x0;
  }
  return;
}

void __stdcall16far pass1_1008_9466(ushort *param_1)

{
  *param_1 = 0x52a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  fn_ptr_1000_17ce(_PTR_LOOP_1050_0392,0x1000);
  _PTR_LOOP_1050_0392 = (astruct_18 *)0x0;
  return;
}



ushort * __stdcall16far pass1_1008_9d02(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



astruct_11 * __stdcall16far pass1_1008_9f80(astruct_11 *param_1,byte param_2)

{
  param_1 = (astruct_11 *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 - 0x1c));
  pass1_1008_9e5a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ulong __stdcall16far pass1_1008_87a2(ulong param_1,byte param_2)

{
  pass1_1008_8168((ushort *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


void __stdcall16far pass1_1008_8aa2(ushort *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  uint uVar3;
  code **ppcVar4;
  undefined4 uVar5;
  astruct_462 *iVar6;
  undefined2 uVar6;
  astruct_18 *paStack16;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (astruct_462 *)param_1;
  *param_1 = 0x8e9a;
  iVar6->field_0x2 = 0x1008;
  uVar5 = *(undefined4 *)&iVar6->field_0x4;
  if (*(int *)((int)uVar5 + 0x1c) != 0x0) {
    puVar1 = iVar6->field_0x4;
    uVar2 = iVar6->field_0x6;
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar4 = (code **)*puVar1;
      (**ppcVar4)();
    }
  }
  uVar2 = iVar6->field_0x3a;
  uVar3 = iVar6->field_0x3c;
  paStack16 = (astruct_18 *)CONCAT22(uVar3,uVar2);
  if ((uVar3 | uVar2) != 0x0) {
    pass1_1008_5118(CONCAT22(uVar3,uVar2));
    fn_ptr_1000_17ce(paStack16,0x1000);
  }
  *param_1 = 0x389a;
  iVar6->field_0x2 = 0x1008;
  return;
}


ushort * __stdcall16far pass1_1008_8e74(ushort *param_1,byte param_2)

{
  pass1_1008_8aa2(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


void __stdcall16far pass1_1008_8f24(ushort *param_1)

{
  ulong *puVar1;
  undefined4 *puVar2;
  uint uVar3;
  code **ppcVar4;
  undefined4 uVar5;
  astruct_463 *iVar6;
  int iVar7;
  int iVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  ulong uStack6;
  
  uVar9 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (astruct_463 *)param_1;
  *param_1 = 0x9170;
  iVar6->field_0x2 = 0x1008;
  if (iVar6->field_0x1a != 0x0) {
    uStack6 = 0x0;
    while( true ) {
      puVar1 = &iVar6->field_0xa;
      if (*puVar1 < uStack6 || *puVar1 == uStack6) break;
      iVar8 = (int)uStack6 * 0x4;
      uVar5 = iVar6->field_0x6;
      uVar10 = (undefined2)((ulong)uVar5 >> 0x10);
      iVar7 = (int)uVar5;
      puVar2 = (undefined4 *)*(uint *)(iVar7 + iVar8);
      uVar3 = *(uint *)(iVar7 + iVar8 + 0x2);
      if ((uVar3 | (uint)puVar2) != 0x0) {
        ppcVar4 = (code **)*puVar2;
        (**ppcVar4)();
      }
      uStack6 = uStack6 + 0x1;
    }
  }
  fn_ptr_1000_17ce((astruct_18 *)iVar6->field_0x6,0x1000);
  *param_1 = 0x389a;
  iVar6->field_0x2 = 0x1008;
  return;
}



ushort * __stdcall16far pass1_1008_914a(ushort *param_1,byte param_2)

{
  pass1_1008_8f24(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


ushort * __stdcall16far pass1_1008_93c0(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_93ec(ushort *param_1,byte param_2)

{
  undefined2 unaff_CS;
  
  kill_timer_1008_921c(param_1,unaff_CS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1008_766e(ulong param_1,ulong *param_2,ushort param_3,ushort param_4,uchar *param_5)

{
  ulong *puVar1;
  uchar *puVar2;
  ulong local_6;
  
  *param_2 = 0x0;
  local_6 = 0x0;
  puVar1 = &local_6;
  file_1008_76e4(param_1,(long *)CONCAT22(param_3,puVar1),param_4,param_3,(ushort)param_5);
  if (puVar1 != (ulong *)0x0) {
    if (local_6 != 0x0) {
      mem_op_1000_179c(0xc,param_5,0x1000);
      puVar2 = (uchar *)((uint)param_5 | (uint)puVar1);
      if (puVar2 == (uchar *)0x0) {
        puVar1 = (ulong *)0x0;
        puVar2 = (uchar *)0x0;
      }
      else {
        pass1_1010_8ef2((ushort *)CONCAT22(param_5,puVar1),puVar2,param_3);
      }
      *(ulong **)param_2 = puVar1;
      *(uchar **)((int)param_2 + 0x2) = puVar2;
      fn_ptr_1010_905e(*param_2,local_6);
    }
    return;
  }
  return;
}

void __stdcall16far pass1_1008_7ffa(ushort *param_1,byte param_2)

{
  astruct_461 *uVar1;
  undefined2 uVar2;
  
  uVar1 = (astruct_461 *)param_1;
  uVar1 = uVar1 + 0x1;
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(uVar1)));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}


void __stdcall16far pass1_1008_6732(ushort *param_1)

{
  long lVar1;
  astruct_457 *iVar2;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_457 *)param_1;
  *param_1 = 0x685a;
  iVar2->field_0x2 = 0x1008;
  if (iVar2->field_0x10 != 0x0) {
    lVar1 = iVar2->field_0x10;
    call_fn_ptr_1000_0dc6((u16)lVar1,(u16)((ulong)lVar1 >> 0x10),0x1000);
  }
  iVar2->field_0x10 = 0x0;
  pass1_1008_41bc(param_1);
  return;
}


ulong __stdcall16far pass1_1008_6834(ulong param_1,byte param_2)

{
  pass1_1008_6732((ushort *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


ushort * __stdcall16far pass1_1008_6b5a(ushort *param_1,byte param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_458 *uVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  uVar4 = (astruct_458 *)param_1;
  *param_1 = 0x6c8c;
  uVar4->field_0x2 = 0x1008;
  puVar1 = uVar4->field_0x4;
  uVar2 = uVar4->field_0x6;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *param_1 = 0x389a;
  uVar4->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1008_6bb4(ushort *param_1,byte param_2)

{
  astruct_459 *uVar1;
  undefined2 uVar2;
  
  uVar1 = (astruct_459 *)param_1;
  uVar1 = uVar1 + 0x1;
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(uVar1)));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}


ushort * __stdcall16far pass1_1008_5b9a(ushort *param_1,byte param_2)

{
  pass1_1008_57c4(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


ulong __stdcall16far pass1_1008_5fa2(ulong param_1,byte param_2)

{
  pass1_1008_5c34((ushort *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


void __stdcall16far pass1_1008_6330(ushort *param_1,byte param_2)

{
  astruct_456 *uVar1;
  undefined2 uVar2;
  
  uVar1 = (astruct_456 *)param_1;
  uVar1 = uVar1 + 0x1;
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(uVar1)));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}


void __stdcall16far pass1_1008_4cdc(ushort *param_1)

{
  astruct_454 *iVar2;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_454 *)param_1;
  *param_1 = 0x4f1c;
  iVar2->field_0x2 = 0x1008;
  fn_ptr_1000_17ce((astruct_18 *)iVar2->field_0xe,0x1000);
  if (iVar2->field_0x12 != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)iVar2->field_0x4,0x1000);
  }
  *param_1 = 0x389a;
  iVar2->field_0x2 = 0x1008;
  return;
}


ushort * __stdcall16far pass1_1008_4ef6(ushort *param_1,byte param_2)

{
  pass1_1008_4cdc(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}





ushort * __stdcall16far pass1_1008_507c(ushort *param_1,byte param_2)

{
  pass1_1008_41bc(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}

void __stdcall16far pass1_1008_5118(ulong param_1)

{
  undefined4 uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x10) != 0x0) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x10);
    call_fn_ptr_1000_0dc6((u16)uVar1,(u16)((ulong)uVar1 >> 0x10),0x1000);
  }
  return;
}


ushort * __stdcall16far pass1_1008_570e(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


ushort * __stdcall16far pass_1008_3d44(ushort *param_1,byte param_2)

{
  astruct_453 *uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  uVar1 = (astruct_453 *)param_1;
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


void __stdcall16far pass1_1008_41bc(ushort *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  long lVar3;
  code **ppcVar4;
  astruct_288 *iVar5;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_288 *)param_1;
  *param_1 = (ushort)&PTR_LOOP_1050_48de;
  iVar5->field_0x2 = 0x1008;
  puVar1 = iVar5->field_0xa;
  uVar2 = iVar5->field_0xc;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)();
  }
  if (iVar5->field_0x6 != 0x0) {
    lVar3 = iVar5->field_0x6;
    call_fn_ptr_1000_0dc6((u16)lVar3,(u16)((ulong)lVar3 >> 0x10),0x1000);
  }
  *param_1 = 0x389a;
  iVar5->field_0x2 = 0x1008;
  return;
}


ushort * __stdcall16far pass1_1008_48b8(ushort *param_1,byte param_2)

{
  pass1_1008_41bc(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


ushort * __stdcall16far pass1_1008_377e(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_37aa(ushort *param_1,byte param_2)

{
  astruct_450 *uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  uVar1 = (astruct_450 *)param_1;
  *param_1 = 0x380a;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ulong __stdcall16far pass1_1008_37e4(ulong param_1,byte param_2)

{
  cleanup_ui_op_1008_0618(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


ushort * __stdcall16far pass1_1008_3a14(ushort *param_1,byte param_2)

{
  *param_1 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_3a40(ushort *param_1,byte param_2)

{
  astruct_451 *uVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  uVar1 = (astruct_451 *)param_1;
  *param_1 = 0x3ab0;
  uVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  uVar1->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ulong __stdcall16far pass1_1008_3a7a(ulong param_1,byte param_2)

{
  pass1_1008_397a((ushort *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


void __stdcall16far pass1_1008_3afe(ushort *param_1,byte param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  iVar1 = (int)param_1;
  pass1_1008_57c4((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0xd2)));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x380a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}

void __stdcall16far pass1_1008_0036(ushort *param_1,ushort param_2)

{
  uint uVar1;
  undefined4 *puVar2;
  astruct_18 *paVar3;
  code **ppcVar4;
  uint uVar5;
  astruct_449 *iVar6;
  undefined2 uVar6;
  
  uVar6 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (astruct_449 *)param_1;
  *param_1 = 0x51e;
  iVar6->field_0x2 = 0x1008;
  paVar3 = *(astruct_18 **)&iVar6->field_0x8;
  uVar1 = iVar6->field_0xa;
  uVar5 = (uint)paVar3;
  if ((uVar1 | uVar5) != 0x0) {
    pass1_1008_53aa(uVar5,uVar1);
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  paVar3 = _PTR_LOOP_1050_5748;
  _PTR_LOOP_1050_0298 = 0x0;
  if (_PTR_LOOP_1050_5748 != (astruct_18 *)0x0) {
    pass1_1030_8210(&_PTR_LOOP_1050_5748->field_0x0);
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  paVar3 = _PTR_LOOP_1050_0ed0;
  if (_PTR_LOOP_1050_0ed0 != (astruct_18 *)0x0) {
    pass1_1010_2050((ulong)_PTR_LOOP_1050_0ed0);
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  paVar3 = _PTR_LOOP_1050_14cc;
  if (_PTR_LOOP_1050_14cc != (astruct_18 *)0x0) {
    pass1_1010_7efc((ulong *)_PTR_LOOP_1050_14cc,0x1010);
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  paVar3 = _PTR_LOOP_1050_5b7c;
  if (_PTR_LOOP_1050_5b7c != (astruct_18 *)0x0) {
    pass1_1038_af34();
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paVar3,0x1000);
  }
  if (_PTR_LOOP_1050_5bc8 != (undefined4 *)0x0) {
    ppcVar4 = (code **)*_PTR_LOOP_1050_5bc8;
    (**ppcVar4)(param_2,(int)_PTR_LOOP_1050_5bc8,(int)((ulong)_PTR_LOOP_1050_5bc8 >> 0x10),0x1);
  }
  if (_PTR_LOOP_1050_02a0 != (undefined4 *)0x0) {
    ppcVar4 = (code **)*_PTR_LOOP_1050_02a0;
    (**ppcVar4)(param_2,(int)_PTR_LOOP_1050_02a0,(int)((ulong)_PTR_LOOP_1050_02a0 >> 0x10),0x1);
  }
  puVar2 = iVar6->field_0x4;
  uVar1 = iVar6->field_0x6;
  if ((uVar1 | (uint)puVar2) != 0x0) {
    ppcVar4 = (code **)*puVar2;
    (**ppcVar4)(param_2,puVar2,uVar1,0x1);
  }
  pass1_1008_9466(param_1);
  return;
}


ushort * __stdcall16far pass1_1008_04d2(ushort *param_1,byte param_2)

{
  pass1_1008_9466(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far pass1_1008_04f8(ushort *param_1,byte param_2,ushort param_3)

{
  pass1_1008_0036(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


void __cdecl16far fn_ptr_op_1000_24cd(ushort param_1,int param_2)

{
  code *pcVar1;
  int iVar2;
  undefined2 uVar6;
  char cVar7;
  ushort uVar5;
  ushort uVar3;
  uint16_t uVar4;
  
  iVar2 = param_2 + 0x1;
  uVar5 = (ushort)&USHORT_1050_1050;
  PTR_LOOP_1050_5fc9._0_1_ = 0x0;
  uVar6 = 0x0;
  fn_ptr_op_1000_2594(0x68b6,0x68b6);
  fn_ptr_op_1000_2594((int)&PTR_LOOP_1050_6210,0x620c);
  ret_op_1000_55ac(param_1,uVar6,uVar5,iVar2);
  cVar7 = (char)((uint)uVar6 >> 0x8);
  fn_ptr_op_1000_2594((int)&PTR_LOOP_1050_6210,(int)&PTR_LOOP_1050_6210);
  fn_ptr_op_1000_2594((int)&PTR_LOOP_1050_6210,(int)&PTR_LOOP_1050_6210);
  dos3_op_1000_256b();
  if (cVar7 == '\0') {
    pcVar1 = (code *)swi(0x21);
    (*pcVar1)();
  }
  return;
}


void __cdecl16far pass1_1000_24db(undefined2 param_1,ushort param_2)

{
  code *pcVar1;
  int iVar2;
  undefined2 uVar3;
  undefined2 uVar4;
  
  iVar2 = param_2 + 0x1;
  uVar4 = SUB42(&USHORT_1050_1050,0x0);
  PTR_LOOP_1050_5fc9._0_1_ = 0x0;
  uVar3 = 0x1;
  fn_ptr_op_1000_2594((int)&PTR_LOOP_1050_6210,(int)&PTR_LOOP_1050_6210);
  fn_ptr_op_1000_2594((int)&PTR_LOOP_1050_6210,(int)&PTR_LOOP_1050_6210);
  dos3_op_1000_256b(uVar3,uVar4,iVar2);
  if ((char)((uint)uVar3 >> 0x8) == '\0') {
    pcVar1 = (code *)swi(0x21);
    (*pcVar1)();
  }
  return;
}


void __cdecl16near fn_ptr_op_1000_2594(code **param_1,code **param_2)

{
  code **ppcVar1;
  code **ppcVar2;
  code **fn_ptr_1;
  
  while (param_2 < param_1) {
    ppcVar2 = param_1 + -0x2;
    ppcVar1 = param_1 + -0x1;
    param_1 = ppcVar2;
    if (((uint)*ppcVar2 | (uint)*ppcVar1) != 0x0) {
      fn_ptr_1 = ppcVar2;
      (**fn_ptr_1)();
    }
  }
  return;
}


BOOL16 __stdcall16far call_fn_ptr_1000_0dc6(u16 param_1,u16 param_2,u16 param_3)

{
  if ((*(uint *)&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0) {
    pass1_1000_1e61(param_3,0xe,0x0,0x0);
    return 0x0;
  }
  (**(code **)0x8)((int)&USHORT_1050_1050);
  return 0x1;
}


void __cdecl16far pass1_1000_16ee(uint param_1,uint param_2,u16 param_3)

{
  if ((param_2 | param_1) != 0x0) {
    call_fn_ptr_1000_0dc6(param_1,param_2,param_3);
  }
  return;
}



ushort __cdecl16far fn_ptr_op_1000_1708(uint param_1,uint param_2,uint param_3,uint param_4,uint param_5,ushort param_6)

{
  int iVar1;
  bool bVar2;
  long lVar3;
  
  if ((param_2 | param_1) == 0x0) {
    bVar2 = 0xfffe < param_1;
    param_1 = param_1 + 0x1;
    param_2 = param_2 + bVar2;
  }
LAB_1000_1724:
  do {
    if ((param_5 | param_4) != 0x0) {
      lVar3 = mem_op_1000_0a48((byte)param_3,param_1,param_2,CONCAT22(param_5,param_4),param_6);
      if (lVar3 != 0x0) {
        return (ushort)lVar3;
      }
    }
    if (((param_3 & 0x8000) == 0x0) || (((uint)PTR_LOOP_1050_5f3a | (uint)PTR_LOOP_1050_5f38) == 0x0)) {
      if (((uint)PTR_LOOP_1050_5f36 | (uint)PTR_LOOP_1050_5f34) == 0x0) {
        if (((uint)PTR_LOOP_1050_5f3e | (uint)PTR_LOOP_1050_5f3c) == 0x0) {
          return 0x0;
        }
        (*(code *)PTR_LOOP_1050_5f3c)();
        goto LAB_1000_1724;
      }
      iVar1 = (*(code *)PTR_LOOP_1050_5f34)();
    }
    else {
      iVar1 = (*(code *)PTR_LOOP_1050_5f38)(param_6,param_1);
    }
    if (iVar1 == 0x0) {
      return 0x0;
    }
  } while( true );
}



void mem_op_1000_179c(ushort param_1,uchar *param_2,ushort param_3)

{
  uchar *puVar1;
  uchar *puVar2;
  
  puVar1 = PTR_LOOP_1050_5f2c;
  puVar2 = PTR_LOOP_1050_5f2e;
  if (((uint)PTR_LOOP_1050_5f2e | (uint)PTR_LOOP_1050_5f2c) == 0x0) {
    puVar1 = mem_op_1000_160a((ushort)param_2,param_3);
    puVar2 = param_2;
  }
  fn_ptr_op_1000_1708(param_1,0x0,0x0,(uint)puVar1,(uint)puVar2,param_3);
  return;
}



void __cdecl16far fn_ptr_1000_17ce(astruct_18 *param_1,ushort param_2)

{
  if (param_1 != (astruct_18 *)0x0) {
    call_fn_ptr_1000_0dc6((u16)param_1,param_1._2_2_,param_2);
  }
  return;
}


void __cdecl16far pass1_1000_18d2(uint param_1,uint param_2,ushort param_3)

{
  if ((param_2 | param_1) != 0x0) {
    call_fn_ptr_1000_0dc6(param_1,param_2,param_3);
  }
  return;
}
