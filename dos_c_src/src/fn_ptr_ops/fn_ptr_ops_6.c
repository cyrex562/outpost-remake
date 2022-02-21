
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
