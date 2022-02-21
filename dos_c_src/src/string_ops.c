
void __stdcall16far string_1040_a626(ushort *param_1, char *param_2, ushort param_3)

{
    ushort uVar1;

    uVar1                           = str_op_1008_60e8(param_2, param_3);
    *param_1                        = uVar1;
    *(ushort *)((int)param_1 + 0x2) = param_3;
    return;
}


char *__stdcall16far pass1_1040_4dcc(ulong param_1, int param_2, ushort param_3)

{
    undefined4 uVar1;
    undefined4 uVar2;
    undefined2 uVar3;
    char      *pcVar4;

    uVar3  = (undefined2)(param_1 >> 0x10);
    uVar2  = *(undefined4 *)((int)param_1 + 0x90);
    uVar1  = *(undefined4 *)((int)param_1 + 0x94);
    pcVar4 = string_op_1010_ada6(
      0x1010, param_3, (ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), param_2, *(int *)((int)uVar2 + 0xa));
    return pcVar4;
}


void __stdcall16far pass1_1040_5d42(ulong param_1)

{
    uint       uVar1;
    char       cVar2;
    int        iVar3;
    undefined2 uVar4;
    ulong      uVar5;

    uVar5 = pass1_1040_5d12(param_1);
    if(uVar5 != 0x0)
    {
        uVar1 = *(uint *)((int)uVar5 + 0xc);
        iVar3 = (int)param_1;
        uVar4 = (undefined2)(param_1 >> 0x10);
        if(uVar1 == 0x5f)
        {
            *(undefined2 *)(iVar3 + 0x96) = 0x53;
            return;
        }
        if(uVar1 < 0x60)
        {
            cVar2 = (char)uVar1;
            if(cVar2 == '(')
            {
                *(undefined2 *)(iVar3 + 0x96) = 0x54;
                return;
            }
            if(cVar2 == ')')
            {
                *(undefined2 *)(iVar3 + 0x96) = 0x55;
                return;
            }
            if(cVar2 == ']')
            {
                *(undefined2 *)(iVar3 + 0x96) = 0x51;
                return;
            }
            if(cVar2 == '^')
            {
                *(undefined2 *)(iVar3 + 0x96) = 0x52;
                return;
            }
        }
    }
    return;
}


void __stdcall16far pass1_1038_4d3c(ulong param_1, char *param_2, ushort param_3)

{
    ushort     uVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    fn_ptr_1000_17ce(*(astruct_18 **)(iVar2 + 0x1fa), 0x1000);
    uVar1                      = str_op_1008_60e8(param_2, param_3);
    *(ushort *)(iVar2 + 0x1fa) = uVar1;
    *(ushort *)(iVar2 + 0x1fc) = param_3;
    return;
}


void __stdcall16far pass1_1030_4dbc(ulong param_1, ulong param_2, long param_3)

{
    long      *plVar1;
    int       *piVar2;
    long       lVar3;
    uint       uVar4;
    int        iVar5;
    undefined2 uVar6;

    iVar5 = (int)param_1;
    uVar6 = (undefined2)(param_1 >> 0x10);
    if(0x0 < param_3)
    {
        *(ulong *)(iVar5 + 0x160) = param_2;
        *(long *)(iVar5 + 0x164)  = param_3;
    }
    if((*(long *)(iVar5 + 0x160) == 0x0)
       || (lVar3 = *(long *)(iVar5 + 0x164), plVar1 = (long *)(iVar5 + 0x164), *plVar1 = *plVar1 + -0x1, lVar3 == 0x0))
    {
        *(undefined4 *)(iVar5 + 0x160) = 0x0;
    }
    else
    {
        uVar4   = str_op_1000_3da4(*(char **)(iVar5 + 0x160));
        piVar2  = (int *)(iVar5 + 0x160);
        *piVar2 = *piVar2 + uVar4 + 0x2;
    }
    return;
}
