
uint __stdcall16far pass1_1018_5932(ulong param_1, ushort param_2, ushort param_3)

{
    code     **ppcVar1;
    uint       uVar2;
    ushort     uVar3;
    undefined2 uVar4;
    undefined4 uVar5;

    uVar4 = (undefined2)(param_1 >> 0x10);
    uVar3 = (ushort)param_1;
    uVar2 = *(uint *)(uVar3 + 0xf0) | *(uint *)(uVar3 + 0xee);
    if(uVar2 != 0x0)
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(uVar3 + 0xee) + 0x8);
        uVar5   = (**ppcVar1)();
        param_2 = (ushort)((ulong)uVar5 >> 0x10);
        uVar2   = (uint)uVar5;
    }
    if(*(int *)(uVar3 + 0xea) == 0x0)
    {
        *(undefined2 *)(uVar3 + 0xea) = 0x1;
        uVar5                         = pass1_1038_af40(_PTR_LOOP_1050_5b7c, *(ushort *)(uVar3 + 0x8), *(int *)(*(int *)(uVar3 + 0xf6) * 0x2 + 0x4238), param_2, uVar3, (ushort)&PTR_LOOP_1050_1038, param_3);
        uVar2                         = (uint)uVar5;
    }
    return uVar2;
}

undefined4 __stdcall16far switch_1018_3b9e(ulong param_1, ushort param_2, ushort param_3, uint param_4, ushort param_5)

{
    undefined4   uVar1;
    astruct_263 *iVar2;
    uint         uVar2;
    ulong        uStack14;
    undefined2   uStack6;
    undefined2   uStack4;

    uStack6 = 0x0;
    uStack4 = 0x0;
    uVar2   = (uint)(param_1 >> 0x10);
    iVar2   = (astruct_263 *)param_1;
    uVar1   = iVar2->field_0x122;
    pass1_1008_e852((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), iVar2->field_0x126, param_5, param_4);
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), CONCAT22(param_4, param_3));
    uStack14 = CONCAT22(param_4, param_3);
    switch(param_2)
    {
    case 0x188:
        if(iVar2->field_0xa == 0x0)
        {
            pass1_1008_d3ae(param_1 & 0xffff | (ulong)uVar2 << 0x10);
        }
        uStack6 = *(undefined2 *)&iVar2->field_0xa;
        uStack4 = *(undefined2 *)((int)&iVar2->field_0xa + 0x2);
        break;
    case 0x189:
        if(iVar2->field_0xe == 0x0)
        {
            unk_str_op_1008_d4f6(param_1 & 0xffff | (ulong)uVar2 << 0x10, uStack14);
        }
        uStack6 = *(undefined2 *)&iVar2->field_0xe;
        uStack4 = *(undefined2 *)((int)&iVar2->field_0xe + 0x2);
        break;
    case 0x18a:
        if(iVar2->field_0x12 == 0x0)
        {
            unk_str_op_1008_d1c6(param_1 & 0xffff | (ulong)uVar2 << 0x10, uStack14);
        }
        uStack6 = *(undefined2 *)&iVar2->field_0x12;
        uStack4 = *(undefined2 *)((int)&iVar2->field_0x12 + 0x2);
        break;
    case 0x18b:
        if(iVar2->field_0x16 == 0x0)
        {
            pass1_1008_cfa0(param_1 & 0xffff | (ulong)uVar2 << 0x10, uStack14);
        }
        uStack6 = *(undefined2 *)&iVar2->field_0x16;
        uStack4 = *(undefined2 *)((int)&iVar2->field_0x16 + 0x2);
        break;
    case 0x18c:
        if(iVar2->field_0x1a == 0x0)
        {
            pass1_1008_cda2(param_1 & 0xffff | (ulong)uVar2 << 0x10, uStack14, param_5);
        }
        uStack6 = *(undefined2 *)&iVar2->field_0x1a;
        uStack4 = *(undefined2 *)((int)&iVar2->field_0x1a + 0x2);
        break;
    case 0x18d:
        if(iVar2->field_0x1e == 0x0)
        {
            pass1_1008_cbc4(param_1 & 0xffff | (ulong)uVar2 << 0x10, uStack14, param_5);
        }
        uStack6 = *(undefined2 *)&iVar2->field_0x1e;
        uStack4 = *(undefined2 *)((int)&iVar2->field_0x1e + 0x2);
    }
    return CONCAT22(uStack4, uStack6);
}

void __stdcall16far pass1_1018_3d44(ulong param_1, ulong *param_2, ulong *param_3)

{
    undefined2 uVar1;

    uVar1    = (undefined2)(param_1 >> 0x10);
    *param_3 = *(ulong *)((int)param_1 + 0x126);
    *param_2 = *(ulong *)((int)param_1 + 0x12a);
    return;
}

void __stdcall16far pass1_1018_3d6c(ulong param_1)

{
    byte         bVar1;
    uint         uVar2;
    undefined   *puVar3;
    uint         uVar4;
    astruct_679 *iVar6;
    undefined2   uVar5;
    ulong        uVar6;
    ulong        uVar7;

    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar6 = (astruct_679 *)param_1;
    uVar4 = iVar6->field_0x142;
    uVar2 = uVar4 + 0x1e;
    if(iVar6->field_0x144 + 0x1U == (uint)(uVar4 < 0xffe2))
    {
        if(uVar2 != 0x3c)
        {
            if(0x3c < uVar2)
            {
                return;
            }
            bVar1 = (byte)uVar2;
            if(bVar1 == 0x14)
            {
                iVar6->field_0x142 = 0xffec;
            LAB_1018_3e3d:
                iVar6->field_0x144 = -0x1;
                return;
            }
            if(0x14 < bVar1)
            {
                if(bVar1 == 0x1e)
                {
                    if((int)PTR_LOOP_1050_13ae < 0x1)
                    {
                        return;
                    }
                    if(SBORROW2((int)PTR_LOOP_1050_13ae, 0x1))
                    {
                        return;
                    }
                    if(PTR_LOOP_1050_13ae != (undefined *)&PTR_LOOP_1050_0002 && 0x0 < (int)(PTR_LOOP_1050_13ae + -0x1))
                    {
                        puVar3 = PTR_LOOP_1050_13ae + -0x3;
                        if(puVar3 == (undefined *)0x0)
                        {
                            pass1_1008_612e(0x1, 0x64, 0x0);
                            if((int)puVar3 < 0x32)
                            {
                                uVar4 = 0xa;
                            }
                            else
                            {
                                uVar4 = 0xfff6;
                            }
                            iVar6->field_0x142 = uVar4;
                            iVar6->field_0x144 = (int)uVar4 >> 0xf;
                            return;
                        }
                        if(puVar3 != (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1))
                        {
                            return;
                        }
                        iVar6->field_0x142 = 0xfff6;
                        goto LAB_1018_3e3d;
                    }
                    iVar6->field_0x142 = 0xa;
                }
                else
                {
                    if(bVar1 == 0x28)
                    {
                        iVar6->field_0x142 = 0x14;
                    }
                    else
                    {
                        if(bVar1 != 0x32)
                        {
                            return;
                        }
                        iVar6->field_0x142 = 0x1e;
                    }
                }
                iVar6->field_0x144 = 0x0;
                return;
            }
            if(bVar1 != 0x0)
            {
                if(bVar1 != 0xa)
                {
                    return;
                }
                iVar6->field_0x142 = 0xffe2;
                goto LAB_1018_3e3d;
            }
        }
        uVar7 = 0x5;
        uVar6 = pass1_1030_8326();
        if(uVar6 % uVar7 == 0x0)
        {
            *(undefined4 *)&iVar6->field_0x142 = 0x0;
            return;
        }
    }
    return;
}


void __stdcall16far pass1_1018_3e8c(ushort param_1, ushort param_2, ushort *param_3, ushort *param_4)

{
    *param_4 = 0x1;
    *param_3 = 0x19;
    return;
}


void __stdcall16far pass1_1018_3ea4(ulong param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;

    pass1_1008_cac6(param_1);
    uVar5  = (undefined2)(param_1 >> 0x10);
    iVar4  = (int)param_1;
    puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x136);
    uVar2  = *(uint *)(iVar4 + 0x138);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)(0x1008, puVar1, uVar2, 0x1);
    }
    *(undefined4 *)(iVar4 + 0x136) = 0x0;
    return;
}

void __stdcall16far pass1_1018_427c(ulong param_1)

{
    int        iVar1;
    ushort     in_AX;
    undefined2 in_DX;
    ushort     uVar2;
    ushort     uVar3;
    ushort     unaff_SS;
    ulong      uVar4;
    long       lVar5;

    uVar3 = (ushort)(param_1 >> 0x10);
    uVar2 = (ushort)param_1;
    uVar4 = switch_1018_3b9e(param_1, *(ushort *)(uVar2 + 0x12e), in_AX, in_DX, unaff_SS);
    iVar1 = *(int *)(uVar2 + 0x12e);
    if(iVar1 == 0x188)
    {
        lVar5 = pass1_1008_57f0(uVar4, *(int *)(uVar2 + 0x130), unaff_SS);
        pass1_1018_456a(uVar2, uVar3, *(ushort *)((int)lVar5 + 0xe));
    }
    else
    {
        if(iVar1 == 0x18b)
        {
            lVar5 = pass1_1008_57f0(uVar4, *(int *)(uVar2 + 0x130), unaff_SS);
            pass1_1018_45d4(uVar2, uVar3, *(int *)((int)lVar5 + 0xe));
        }
        else
        {
            if(iVar1 == 0x18c)
            {
                lVar5 = pass1_1008_57f0(uVar4, *(int *)(uVar2 + 0x130), unaff_SS);
                pass1_1018_451e(uVar2, uVar3, *(int *)((int)lVar5 + 0xe));
            }
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_435e(ulong param_1, long param_2, int param_3, int param_4, uint param_5, ushort param_6)

{
    undefined4 uVar1;
    uint       uVar2;
    uint       uVar3;
    undefined2 uVar4;

    if(param_3 < param_4)
    {
        param_4 = param_3;
    }
    uVar2 = 0x0;
    uVar4 = (undefined2)(param_1 >> 0x10);
    uVar1 = *(undefined4 *)((int)param_1 + 0x122);
    pass1_1008_e852((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), *(ulong *)((int)param_1 + 0x126), param_6, param_5);
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), CONCAT22(param_5, uVar2));
    do
    {
        do
        {
            uVar3 = uVar2;
            pass1_1008_612e(param_4, param_3, uVar3);
            uVar2 = *(uint *)(uVar3 * 0x2 + 0x411c);
        } while(uVar2 == 0x0);
        if(uVar2 != 0x1)
        {
            pass1_1008_612e(0x1, uVar2, uVar2);
        }
        uVar2 = uVar2 - 0x1;
        switch_1018_3ee6(param_1, param_2, uVar2, uVar3, param_5);
        param_5 = param_5 | uVar2;
    } while(param_5 == 0x0);
    return;
}

ushort __stdcall16far switch_1018_43ec(ushort param_1, ushort param_2, ushort param_3)

{
    ushort uStack6;

    switch(param_3)
    {
    case 0xf:
    case 0x35:
    case 0x36:
        uStack6 = 0x7;
        break;
    default:
        uStack6 = 0x1;
        break;
    case 0x11:
    case 0x13:
    case 0x14:
    case 0x15:
    case 0x2d:
    case 0x2e:
    case 0x6e:
        uStack6 = 0x9;
        break;
    case 0x12:
    case 0x31:
    case 0x32:
    case 0x52:
    case 0x53:
    case 0x54:
    case 0x55:
    case 0x56:
    case 0x5a:
    case 0x5b:
    case 0x5c:
    case 0x5d:
    case 0x5e:
    case 0x5f:
        uStack6 = 0x4;
        break;
    case 0x1b:
    case 0x1c:
    case 0x1d:
    case 0x28:
    case 0x29:
    case 0x2c:
    case 0x2f:
    case 0x30:
    case 0x68:
    case 0x69:
        uStack6 = 0x5;
        break;
    case 0x1e:
    case 0x1f:
    case 0x20:
    case 0x33:
    case 0x34:
        uStack6 = 0x6;
        break;
    case 0x22:
    case 0x23:
    case 0x24:
        uStack6 = 0x8;
        break;
    case 0x25:
    case 0x26:
    case 0x27:
        uStack6 = 0x2;
        break;
    case 0x38:
    case 0x39:
    case 0x4f:
    case 0x50:
    case 0x51:
    case 0x57:
    case 0x58:
    case 0x59:
    case 0x66:
    case 0x67:
    case 0x6c:
    case 0x6d:
        uStack6 = 0x3;
    }
    return uStack6;
}


ushort __stdcall16far pass1_1018_451e(ushort param_1, ushort param_2, int param_3)

{
    ushort uStack6;

    if(param_3 == 0x7)
    {
        uStack6 = 0x9;
    }
    else
    {
        if(param_3 == 0x8)
        {
            uStack6 = 0xa;
        }
        else
        {
            if(param_3 == 0xc)
            {
                uStack6 = 0x19;
            }
            else
            {
                if(param_3 == 0xd)
                {
                    uStack6 = 0x3;
                }
                else
                {
                    uStack6 = 0x8;
                }
            }
        }
    }
    return uStack6;
}


ushort __stdcall16far pass1_1018_456a(ushort param_1, ushort param_2, ushort param_3)

{
    ushort uStack6;

    switch(param_3)
    {
    case 0x11:
    case 0x12:
    case 0x13:
    case 0x14:
    case 0x15:
        uStack6 = 0x2;
        break;
    case 0x16:
    case 0x1e:
        uStack6 = 0x3;
        break;
    default:
        uStack6 = 0x1;
        break;
    case 0x1d:
    case 0x21:
        uStack6 = 0x4;
    }
    return uStack6;
}


ushort __stdcall16far pass1_1018_45d4(ushort param_1, ushort param_2, int param_3)

{
    ushort uStack6;

    if(param_3 == 0x3)
    {
        uStack6 = 0x16;
    }
    else
    {
        if(param_3 == 0x4)
        {
            uStack6 = 0x17;
        }
        else
        {
            uStack6 = 0x14;
        }
    }
    return uStack6;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

long __stdcall16far pass1_1018_4608(ushort param_1, ulong param_2, ulong param_3, ulong param_4)

{
    undefined4 uVar1;
    uint       uVar2;
    uint       uVar3;
    int        iVar4;
    uint       uVar5;
    uint       uVar6;
    long       lVar7;
    char      *pcVar8;
    char      *pcVar9;
    ulong      uStack26;
    ulong      uStack22;
    undefined  local_a[0x8];

    uVar1 = *(undefined4 *)((int)param_2 + 0x122);
    pass1_1008_5784((ulong *)CONCAT22(param_1, local_a), *(ulong *)((int)uVar1 + 0xa));
    while(true)
    {
        lVar7 = pass1_1008_5b12(local_a, param_1);
        uVar5 = (uint)((ulong)lVar7 >> 0x10);
        uVar2 = (uint)lVar7;
        uVar6 = uVar5 | uVar2;
        if(lVar7 == 0x0)
        {
            return 0x0;
        }
        uVar1 = *(undefined4 *)(uVar2 + 0x4);
        uVar3 = uVar2;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
        uStack22 = CONCAT22(uVar6, uVar3);
        uVar1    = *(undefined4 *)(uVar2 + 0x8);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
        uStack26 = CONCAT22(uVar6, uVar3);
        pcVar8   = pass1_1038_4d28(uStack22);
        pcVar9   = pass1_1038_4d28(uStack26);
        iVar4    = pass1_1000_3d7a(param_4, (ulong)pcVar8);
        if((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_3, (ulong)pcVar9), iVar4 == 0x0))
            break;
        iVar4 = pass1_1000_3d7a(param_3, (ulong)pcVar8);
        if((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_4, (ulong)pcVar9), iVar4 == 0x0))
        {
            return lVar7;
        }
    }
    return lVar7;
}

void __stdcall16far pass1_1018_2d22(ulong param_1, int *param_2, ushort *param_3, int param_4)

{
    ulong uVar1;

    *param_3 = 0x0;
    *param_2 = 0x0;
    uVar1    = pass1_1008_4772(*(astruct_76 **)((int)param_1 + 0x24));
    *param_2 = *(int *)((int)uVar1 + 0x8) + -0x14;
    if(param_4 == 0xbb8)
    {
        *param_3 = 0x5;
    }
    if(param_4 == 0xbba)
    {
        *param_3 = 0x23;
    }
    if(param_4 == 0xbb9)
    {
        *param_3 = 0x75;
    }
    return;
}


void __stdcall16far pass1_1018_2d84(ushort param_1, ulong param_2)

{
    pass1_1018_2e28(param_2);
    pass1_1020_bd80(param_1);
    return;
}


void __stdcall16far pass1_1018_2d9a(ulong param_1)

{
    int       *piVar1;
    undefined4 uVar2;
    uint       uVar3;
    int        iVar4;
    undefined2 uVar5;

    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    uVar3 = *(uint *)(iVar4 + 0x180) | *(uint *)(iVar4 + 0x17e);
    if(uVar3 != 0x0)
    {
        piVar1  = (int *)(iVar4 + 0x174);
        *piVar1 = *piVar1 + -0x1;
        if(*piVar1 < 0x0)
        {
            uVar2                    = *(undefined4 *)(iVar4 + 0x17e);
            uVar3                    = *(int *)((int)uVar2 + 0xa) - 0x1;
            *(uint *)(iVar4 + 0x174) = uVar3;
        }
        pass1_1018_2e28(param_1);
        *(uint *)(iVar4 + 0x176) = uVar3;
    }
    return;
}


void __stdcall16far pass1_1018_2dde(ulong param_1)

{
    int       *piVar1;
    undefined4 uVar2;
    int        iVar3;
    int        iVar4;
    undefined2 uVar5;

    uVar5 = (undefined2)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    if((*(uint *)(iVar4 + 0x180) | *(uint *)(iVar4 + 0x17e)) != 0x0)
    {
        piVar1  = (int *)(iVar4 + 0x174);
        *piVar1 = *piVar1 + 0x1;
        iVar3   = *(int *)(iVar4 + 0x174);
        uVar2   = *(undefined4 *)(iVar4 + 0x17e);
        piVar1  = (int *)((int)uVar2 + 0xa);
        if(*piVar1 == iVar3 || *piVar1 < iVar3)
        {
            *(undefined2 *)(iVar4 + 0x174) = 0x0;
        }
        pass1_1018_2e28(param_1);
        *(int *)(iVar4 + 0x176) = iVar3;
    }
    return;
}
void __stdcall16far pass1_1018_2e28(ulong param_1)

{
    long       lVar1;
    uint       extraout_DX;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    lVar1 = (long)*(int *)((int)param_1 + 0x174);
    empty_1008_8fc4(*(undefined4 *)((int)param_1 + 0x17e), lVar1);
    if((extraout_DX | (uint)lVar1) != 0x0)
    {
        return;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_2e5e(ushort param_1, ushort param_2, ushort param_3, ulong param_4)

{
    ushort       uVar1;
    long         lVar1;
    astruct_126 *iVar4;
    ushort       uVar2;

    uVar2 = (ushort)(param_4 >> 0x10);
    iVar4 = (astruct_126 *)param_4;
    if(iVar4->field_0x17e == 0x0)
    {
        pass1_1030_82f0(param_1, _PTR_LOOP_1050_5748, iVar4->field_0x17a);
        *(ushort *)&iVar4->field_0x17e              = param_2;
        *(ushort *)((int)&iVar4->field_0x17e + 0x2) = param_3;
    }
    if((iVar4->field_0x17e != 0x0) && (lVar1 = iVar4->field_0x17e, *(int *)((int)lVar1 + 0xa) != 0x0))
    {
        lVar1 = (long)iVar4->field_0x174;
        empty_1008_8fc4(iVar4->field_0x17e, lVar1);
        uVar1 = (ushort)lVar1;
        pass1_1018_2e28(param_4);
        iVar4->field_0x176 = uVar1;
        return;
    }
    return;
}

void __stdcall16far pass1_1018_2ee4(ulong param_1, uint param_2, ushort param_3)

{
    undefined4 uVar1;
    char       cVar2;
    ushort     uVar3;

    if(param_2 != 0x12)
    {
        if(param_2 < 0x13)
        {
            cVar2 = (char)param_2;
            if(cVar2 == '\x01')
            {
                *(undefined4 *)((int)param_1 + 0x162) = 0x0;
                return;
            }
            if(('\x02' < (char)(cVar2 + -0x1)) && ((char)(cVar2 + -0x4) < '\x03'))
                goto LAB_1018_2f06;
        }
        return;
    }
    uVar1                                 = *(undefined4 *)((int)param_1 + 0x162);
    *(undefined4 *)((int)param_1 + 0x15a) = *(undefined4 *)((int)uVar1 + 0x24);
LAB_1018_2f06:
    uVar3 = (int)param_1 - 0x20;
    pass1_1018_31fa(param_1 & 0xffff0000 | (ulong)uVar3, uVar3, param_1._2_2_, param_3);
    pass1_1010_1f62(param_3, param_1 & 0xffff0000 | (ulong)uVar3, param_2);
    return;
}

void __stdcall16far pass1_1018_2fe8(ulong param_1, uint16_t param_2, uint16_t param_3)

{
    int       *piVar1;
    undefined4 uVar2;
    uint       uVar3;
    ushort     uVar4;
    uint       uVar5;
    undefined2 uVar6;
    int        iVar7;
    undefined2 extraout_DX;
    undefined2 uVar8;
    int        iVar9;
    undefined2 uVar10;

    uVar10 = (undefined2)(param_1 >> 0x10);
    iVar9  = (int)param_1;
    uVar6  = *(undefined2 *)(iVar9 + 0x174);
    uVar2  = *(undefined4 *)(iVar9 + 0x17e);
    iVar7  = *(int *)((int)uVar2 + 0xa);
    if(iVar7 != 0x0)
    {
        if(*(long *)(iVar9 + 0x186) != 0x0)
        {
            uVar3                          = str_op_1000_3da4(*(char **)(iVar9 + 0x186));
            *(undefined2 *)(iVar9 + 0x174) = 0x0;
            while(true)
            {
                if(iVar7 <= *(int *)(iVar9 + 0x174))
                    break;
                uVar4 = *(ushort *)(iVar9 + 0x174);
                uVar2 = *(undefined4 *)(iVar9 + 0x17e);
                empty_1008_8fc4((int)uVar2, (int)((ulong)uVar2 >> 0x10), uVar4, (int)uVar4 >> 0xf);
                uVar8 = extraout_DX;
                pass1_1018_2e28(param_1);
                uVar4 = pass1_1020_bd80(uVar4);
                uVar5 = pass1_1000_3de8((char *)CONCAT22(uVar8, uVar4), *(char **)(iVar9 + 0x186), uVar3, param_2, param_3);
                if(uVar5 == 0x0)
                    break;
                piVar1  = (int *)(iVar9 + 0x174);
                *piVar1 = *piVar1 + 0x1;
            }
            if(*(int *)(iVar9 + 0x174) < iVar7)
            {
                pass1_1018_2e28(param_1);
                *(int *)(iVar9 + 0x176) = iVar7;
                return;
            }
            *(undefined2 *)(iVar9 + 0x174) = uVar6;
            pass1_1018_2e28(param_1);
            *(undefined2 *)(iVar9 + 0x176) = uVar6;
        }
    }
    return;
}

ushort __stdcall16far pass1_1018_31d0(ulong param_1)

{
    undefined4 uVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    if((*(long *)((int)param_1 + 0x17e) != 0x0) && (uVar1 = *(undefined4 *)((int)param_1 + 0x17e), *(long *)((int)uVar1 + 0xa) != 0x0))
    {
        return 0x1;
    }
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_31fa(ulong param_1, ushort param_2, uint param_3, ushort param_4)

{
    int       *piVar1;
    undefined4 uVar2;
    int        iVar3;
    int        iVar4;
    long       lVar5;
    int        iVar6;
    undefined2 uVar7;

    uVar7 = (undefined2)(param_1 >> 0x10);
    iVar6 = (int)param_1;
    pass1_1030_82f0(param_4, _PTR_LOOP_1050_5748, *(ulong *)(iVar6 + 0x17a));
    *(ushort *)(iVar6 + 0x17e) = param_2;
    *(uint *)(iVar6 + 0x180)   = param_3;
    if(((param_3 | *(uint *)(iVar6 + 0x17e)) != 0x0) && (uVar2 = *(undefined4 *)(iVar6 + 0x17e), iVar4 = *(int *)((int)uVar2 + 0xa), iVar4 != 0x0))
    {
        *(undefined2 *)(iVar6 + 0x174) = 0x0;
        while(true)
        {
            if(iVar4 <= *(int *)(iVar6 + 0x174))
                break;
            lVar5 = (long)*(int *)(iVar6 + 0x174);
            empty_1008_8fc4(*(undefined4 *)(iVar6 + 0x17e), lVar5);
            iVar3 = (int)lVar5;
            pass1_1018_2e28(param_1);
            if(*(int *)(iVar6 + 0x176) == iVar3)
                break;
            piVar1  = (int *)(iVar6 + 0x174);
            *piVar1 = *piVar1 + 0x1;
        }
        if(iVar4 <= *(int *)(iVar6 + 0x174))
        {
            *(undefined2 *)(iVar6 + 0x174) = 0x0;
        }
        pass1_1018_2e28(param_1);
        *(int *)(iVar6 + 0x176) = iVar4;
    }
    return;
}

void __stdcall16far pass1_1018_331c(astruct_638 *param_1, ushort param_2, ushort param_3, ushort param_4, uchar *param_5)

{
    uint    uVar1;
    int     unaff_DI;
    ushort *puVar2;

    pass1_1008_ca5a((astruct_639 *)param_1, param_2, param_3);
    *(undefined4 *)&param_1->field_0x122 = 0x0;
    param_1->field_0x126                 = 0x0;
    param_1->field_0x12a                 = 0x0;
    param_1->field_0x12e                 = 0x0;
    param_1->field_0x130                 = 0x0;
    param_1->field_0x132                 = 0x0;
    param_1->field_0x136                 = 0x0;
    param_1->field_0x13a                 = 0x0;
    param_1->field_0x13c                 = 0x0;
    param_1->field_0x13e                 = 0x0;
    param_1->field_0x142                 = 0x0;
    *(int *)CONCAT22(param_2, param_1)   = (int)&PTR_LOOP_1050_470c;
    param_1->field_0x2                   = 0x1018;
    puVar2                               = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x3b, param_4, param_5, unaff_DI);
    uVar1                                = (uint)puVar2;
    param_1->field_0x122                 = uVar1;
    param_1->field_0x124                 = (int)((ulong)puVar2 >> 0x10);
    *(undefined *)&param_1->field_0x22   = 0x0;
    pass1_1008_612e(0x8, 0xc, uVar1);
    param_1->field_0x13c = uVar1;
    return;
}

void __stdcall16far pass1_1018_3424(ulong param_1, int param_2, uint param_3, ushort param_4)

{
    undefined4 uVar1;
    uint       uVar2;
    int        iVar3;
    undefined2 uVar4;
    ulong      uStack10;
    ulong      uStack6;

    uVar4 = (undefined2)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    uVar1 = *(undefined4 *)(iVar3 + 0x122);
    pass1_1008_e852((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), *(ulong *)(iVar3 + 0x126), param_4, param_3);
    uStack6 = CONCAT22(param_3, param_2);
    uVar1   = *(undefined4 *)(iVar3 + 0x122);
    pass1_1008_e852((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), *(ulong *)(iVar3 + 0x12a), param_4, param_3);
    uStack10 = CONCAT22(param_3, param_2);
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), uStack6);
    uVar2 = param_3;
    iVar3 = param_2;
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), uStack10);
    if(*(long *)(iVar3 + 0x200) == *(long *)(param_2 + 0x200))
    {
        return;
    }
    return;
}


void __stdcall16far pass1_1018_34a6(ulong param_1)

{
    pass1_1018_3d6c(param_1);
    return;
}

void __stdcall16far pass1_1018_36e6(ulong param_1, ushort param_2, ushort param_3, ushort param_4)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2                          = (undefined2)(param_1 >> 0x10);
    iVar1                          = (int)param_1;
    *(ushort *)(iVar1 + 0x12e)     = param_4;
    *(ushort *)(iVar1 + 0x130)     = param_3;
    *(ushort *)(iVar1 + 0x132)     = param_2;
    *(undefined2 *)(iVar1 + 0x134) = 0x0;
    return;
}

void __stdcall16far pass1_1018_3a42(ulong param_1, ulong param_2, uint param_3, ushort param_4)

{
    undefined4 uVar1;

    uVar1 = *(undefined4 *)((int)param_1 + 0x122);
    pass1_1008_e852((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), param_2, param_4, param_3);
    return;
}


void __stdcall16far pass1_1018_3a5c(ulong param_1, ulong param_2, ulong param_3, ushort param_4)

{
    pass1_1008_e320(*(astruct_102 **)((int)param_1 + 0x122), param_2, param_3, param_4);
    return;
}

void __stdcall16far pass1_1018_3a94(ulong param_1, ulong *param_2, ulong *param_3, ushort param_4)

{
    pass1_1008_e3ec(*(ulong *)((int)param_1 + 0x122), param_2, param_3, param_4);
    return;
}


ushort __stdcall16far pass1_1018_3ab2(ulong param_1, int param_2, int param_3, ushort param_4)

{
    undefined2 uVar1;
    undefined2 uVar2;
    int        iVar3;
    long       lVar4;
    ushort     uStack22;
    undefined  local_10[0x8];
    int        iStack8;
    ulong      uStack6;

    if(0x5 < param_3 - 0x188U)
    {
        return 0x0;
    }
    iVar3 = (int)param_1;
    uVar2 = (undefined2)(param_1 >> 0x10);
    switch(param_3)
    {
    case 0x188:
        uVar1 = *(undefined2 *)(iVar3 + 0xa);
        uVar2 = *(undefined2 *)(iVar3 + 0xc);
        break;
    case 0x189:
        uVar1 = *(undefined2 *)(iVar3 + 0xe);
        uVar2 = *(undefined2 *)(iVar3 + 0x10);
        break;
    case 0x18a:
        uVar1 = *(undefined2 *)(iVar3 + 0x12);
        uVar2 = *(undefined2 *)(iVar3 + 0x14);
        break;
    case 0x18b:
        uVar1 = *(undefined2 *)(iVar3 + 0x16);
        uVar2 = *(undefined2 *)(iVar3 + 0x18);
        break;
    case 0x18c:
        uVar1 = *(undefined2 *)(iVar3 + 0x1a);
        uVar2 = *(undefined2 *)(iVar3 + 0x1c);
        break;
    case 0x18d:
        uVar1 = *(undefined2 *)(iVar3 + 0x1e);
        uVar2 = *(undefined2 *)(iVar3 + 0x20);
    }
    uStack6 = CONCAT22(uVar2, uVar1);
    iStack8 = 0x0;
    pass1_1008_5784((ulong *)CONCAT22(param_4, local_10), uStack6);
    while(true)
    {
        lVar4 = pass1_1008_5b12(local_10, param_4);
        uVar2 = (undefined2)((ulong)lVar4 >> 0x10);
        if((lVar4 == 0x0) || (iStack8 == param_2))
            break;
        iStack8 = iStack8 + 0x1;
    }
    uStack22 = 0x0;
    if(lVar4 != 0x0)
    {
        if(*(int *)((int)lVar4 + 0xa) == 0x0)
        {
            uStack22 = *(ushort *)((int)lVar4 + 0x8);
        }
        else
        {
            uStack22 = 0xffff;
        }
    }
    return uStack22;
}


ushort __stdcall16far pass1_1018_1c9a(ulong param_1, int param_2)

{
    int       *piVar1;
    ulong      uVar2;
    uint       uVar3;
    undefined2 uVar4;
    undefined2 unaff_SS;
    int        iStack10;

    iStack10 = 0x0;
    while(true)
    {
        uVar4  = (undefined2)(param_1 >> 0x10);
        piVar1 = (int *)((int)param_1 + 0x44);
        if(*piVar1 == iStack10 || *piVar1 < iStack10)
        {
            return 0x0;
        }
        uVar2 = *(ulong *)((int)param_1 + 0x40);
        uVar3 = (int)uVar2 + iStack10 * 0x18;
        if(*(int *)(*(int *)(uVar3 + 0xc) * 0x1e + 0x3c32) == param_2)
            break;
        iStack10 = iStack10 + 0x1;
    }
    pass1_1018_1eda(param_1, uVar2 & 0xffff0000 | (ulong)uVar3, unaff_SS);
    return 0x1;
}


// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1018_1ce8(ushort param_1, ulong param_2)

{
    int       *piVar1;
    int        iVar2;
    int        iVar3;
    uint       uVar4;
    undefined2 uVar5;
    int        iStack26;
    undefined  local_18[0x2];
    undefined  local_16[0x2];
    int        iStack20;
    int        iStack18;
    int        iStack16;
    uint       local_e;
    int        local_c;
    int        local_a;
    int        iStack8;
    undefined4 uStack6;

    uVar5   = (undefined2)(param_2 >> 0x10);
    iVar3   = (int)param_2;
    uStack6 = *(ulong *)(iVar3 + 0x40);
    iStack8 = 0x0;
    do
    {
        piVar1 = (int *)(iVar3 + 0x44);
        if(*piVar1 == iStack8 || *piVar1 < iStack8)
        {
            return;
        }
        pass1_1008_3eb4((ushort *)(uStack6 & 0xffff0000 | (ulong)(uint)(iStack8 * 0x18 + (int)uStack6)), (ushort *)CONCAT22(param_1, &local_e), (ushort *)CONCAT22(param_1, &local_c), (ushort *)CONCAT22(param_1, &local_a));
        local_a  = local_a / 0xa;
        iStack16 = local_c % 0xa;
        if(iStack16 != 0x0)
        {
            if(iStack16 < 0x6)
            {
                local_c = local_c - iStack16;
            }
            else
            {
                local_c = local_c + (0xa - iStack16);
            }
        }
        iStack18 = pass1_1000_49b2(local_e);
        iStack18 = iStack18 / 0x5;
        if(0x14 < iStack18)
        {
            iStack18 = 0x14;
            iVar2    = pass1_1000_49b2(local_e);
            local_e  = ((int)local_e / iVar2) * 0x64;
        }
        iStack16 = pass1_1000_49b2(local_e);
        iStack16 = iStack16 % 0x5;
        if(iStack16 != 0x0)
        {
            if((int)local_e < 0x0)
            {
                iVar2 = iStack16;
                if(0x2 < iStack16)
                {
                    iVar2 = iStack16 + -0x5;
                }
                local_e = local_e + iVar2;
            }
            else
            {
                if(iStack16 < 0x3)
                {
                    local_e = local_e - iStack16;
                }
                else
                {
                    local_e = local_e + (0x5 - iStack16);
                }
            }
        }
        iStack20 = *(int *)(iStack18 * 0x48 + 0x3c20);
        if(local_c < iStack20)
        {
            for(iStack26 = iStack18; iStack26 < 0x15; iStack26 = iStack26 + 0x1)
            {
                piVar1 = (int *)(iStack26 * 0x48 + 0x3c20);
                if(*piVar1 == local_c || *piVar1 < local_c)
                {
                    iStack18 = iStack26;
                    break;
                }
            }
        }
        pass1_1008_3e94((ushort *)(param_2 & 0xffff0000 | (ulong)(iVar3 + 0x3a)), (ushort *)CONCAT22(param_1, local_18), (ushort *)CONCAT22(param_1, local_16));
        uVar4                 = iStack8 * 0x18 + (int)uStack6;
        *(int *)(uVar4 + 0x6) = local_a;
        *(int *)(uVar4 + 0x8) = iStack18;
        pass1_1008_3e76((ushort *)(uStack6 & 0xffff0000 | (ulong)uVar4), 0x0, local_e, *(ushort *)((iStack18 * 0x24 + local_a) * 0x2 + 0x3c20));
        *(undefined2 *)(uVar4 + 0xa) = *(undefined2 *)(local_a * 0x2 + 0x3966);
        iStack8                      = iStack8 + 0x1;
    } while(true);
}

ulong __stdcall16far pass1_1018_1e78(ulong param_1, int param_2)

{
    undefined4 uVar1;

    if(param_2 == -0x1)
    {
        uVar1   = *(undefined4 *)((int)param_1 + 0x46);
        param_2 = *(int *)((int)uVar1 + 0xc);
    }
    return CONCAT22(0x1050, param_2 * 0x1e + 0x3c18);
}

void __stdcall16far pass1_1018_1eda(ulong param_1, ulong param_2, ushort param_3)

{
    undefined4 uVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    if(*(long *)(iVar2 + 0x46) != 0x0)
    {
        uVar1                             = *(undefined4 *)(iVar2 + 0x46);
        *(undefined2 *)((int)uVar1 + 0xe) = 0x2;
    }
    *(ulong *)(iVar2 + 0x46)            = param_2;
    *(undefined2 *)((int)param_2 + 0xe) = 0x1;
    pass1_1010_1f62(param_3, param_1, 0xd);
    return;
}


ushort __stdcall16far pass1_1018_1f1a(ulong param_1, int param_2)

{
    int       *piVar1;
    int        iVar2;
    undefined2 uVar3;
    int        iStack6;

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    if(*(int *)(iVar2 + 0x56) == 0x0)
    {
        return 0x0;
    }
    iStack6 = 0x0;
    while(true)
    {
        piVar1 = (int *)(iVar2 + 0x56);
        if(*piVar1 == iStack6 || *piVar1 < iStack6)
        {
            return 0x0;
        }
        if(*(int *)(iVar2 + 0x4e + iStack6 * 0x2) == param_2)
            break;
        iStack6 = iStack6 + 0x1;
    }
    return 0x1;
}

ulong __stdcall16far pass1_1018_1f7a(int param_1, ushort param_2)

{
    return CONCAT22(param_2, param_1 + 0x2a);
}

void __stdcall16far pass1_1018_2076(ushort *param_1, ushort param_2)

{
    uint uVar1;

    uVar1                               = (uint)((ulong)param_1 >> 0x10);
    *param_1                            = 0x21e8;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1018;
    pass1_1018_209c((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
    pass1_1010_1d80(param_1, param_2);
    return;
}


void __stdcall16far pass1_1018_209c(ulong param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    undefined2  uVar5;
    int         iStack4;

    iStack4 = 0x0;
    do
    {
        uVar5  = (undefined2)(param_1 >> 0x10);
        iVar4  = (int)param_1 + 0x12;
        puVar1 = (undefined4 *)*(uint *)(iVar4 + iStack4 * 0x4);
        uVar2  = *(uint *)(iVar4 + iStack4 * 0x4 + 0x2);
        if((uVar2 | (uint)puVar1) != 0x0)
        {
            ppcVar3 = (code **)*puVar1;
            (**ppcVar3)();
        }
        *(undefined4 *)((int)param_1 + iStack4 * 0x4 + 0x12) = 0x0;
        iStack4                                              = iStack4 + 0x1;
    } while(iStack4 < 0x1fd);
    return;
}


void __stdcall16far pass1_1018_20ee(ulong param_1, int *param_2)

{
    BOOL16 BVar1;
    ushort in_DX;
    uint   uVar2;

    BVar1 = pass1_1008_aed8((ulong)param_2);
    if(BVar1 == 0x0)
    {
        return;
    }
    uVar2 = (uint)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + *param_2 * 0x4 + 0x12) == 0x0)
    {
        pass1_1018_216e(param_1 & 0xffff | (ulong)uVar2 << 0x10, (ulong)param_2, in_DX);
    }
    pass1_1008_ae26(param_2);
    return;
}


void __stdcall16far pass1_1018_214e(ushort param_1, ushort param_2, ushort *param_3, int param_4)

{
    pass1_1008_3e76(param_3, 0x0, *(ushort *)(param_4 * 0x4 + 0x3e90), *(ushort *)(param_4 * 0x4 + 0x3e8e));
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_216e(ulong param_1, ulong param_2, ushort param_3)

{
    ushort uVar1;
    ushort uVar2;
    ushort uVar3;
    uint   uStack8;

    uStack8 = pass1_1008_adf2(param_2);
    uVar1   = pass1_1008_ae0c(param_2);
    for(; (int)uStack8 <= (int)uVar1; uStack8 = uStack8 + 0x1)
    {
        uVar2 = uVar1;
        pass1_1010_8018(_PTR_LOOP_1050_14cc, uStack8, (uchar *)param_3, 0x1010);
        uVar3                                            = (ushort)(param_1 >> 0x10);
        *(ushort *)((int)param_1 + uStack8 * 0x4 + 0x12) = uVar2;
        *(uchar **)((int)param_1 + uStack8 * 0x4 + 0x14) = (uchar *)param_3;
    }
    return;
}

ushort __cdecl16far pass1_1018_21f8(void)

{
    ushort *puVar1;

    pass1_1008_3e54(&USHORT_1048_4210, 0x0, 0x195, 0x1);
    pass1_1008_3e54(&USHORT_1050_65ca, 0x0, 0xe0, 0x1b1);
    pass1_1008_3e54(&USHORT_1050_65d0, 0x0, 0x17a, 0x72);
    pass1_1008_3e54(&USHORT_1050_65d6, 0x0, 0xde, 0x93);
    pass1_1008_3e54(&USHORT_1050_65dc, 0x0, 0x177, 0x1da);
    pass1_1008_3e54(&USHORT_1048_4216, 0x0, 0x195, 0x21c);
    pass1_1008_3e54(&USHORT_1048_421c, 0x0, 0x1b6, 0x22c);
    pass1_1008_3e54(&USHORT_1048_4222, 0x0, 0x109, 0x5);
    puVar1 = pass1_1008_3e54(&USHORT_1048_4228, 0x0, 0xfd, 0x1fd);
    return (ushort)puVar1;
}

void __stdcall16far pass1_1018_2504(uint param_1, uint param_2)

{
    ushort uVar1;

    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), 0x4000001);
    if((param_2 | param_1) != 0x0)
    {
        uVar1 = pass1_1028_d69e(**_PTR_LOOP_1050_5748);
        if(uVar1 == 0x0)
        {
            return;
        }
    }
    return;
}


void __stdcall16far pass1_1018_2548(ushort param_1, ushort param_2, ushort *param_3)

{
    pass1_1008_3f62(param_3, &USHORT_1048_4228);
    return;
}


ushort __stdcall16far pass1_1018_255e(ulong param_1)

{
    undefined4 uVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x26) != 0x0)
    {
        uVar1 = *(undefined4 *)((int)param_1 + 0x26);
        return *(ushort *)((int)uVar1 + 0xa);
    }
    return 0x0;
}


uchar *__stdcall16far pass1_1018_2580(ulong param_1, ushort param_2, ulong param_3, ushort param_4, ushort param_5, uchar param_6)

{
    uchar     *puVar1;
    int        iVar2;
    undefined2 uVar3;
    uchar      local_8[0x6];

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    if(*(long *)(iVar2 + 0x20) == 0x0)
    {
        return (uchar *)0x6ad;
    }
    pass1_1008_3e38((ushort *)CONCAT22(param_5, local_8));
    pass1_1018_161c(param_5, *(ulong *)(iVar2 + 0x20), (ushort *)CONCAT22(param_5, local_8), (int)param_3, (int)(param_3 >> 0x10));
    puVar1 = local_8;
    pass1_1018_17ce(*(ulong *)(iVar2 + 0x20), CONCAT22(puVar1, param_2), CONCAT22(param_4, param_5), param_5, param_6);
    return puVar1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ushort __stdcall16far pass1_1018_25d2(ulong param_1, ushort param_2, ulong param_3, int param_4, ushort param_5)

{
    ushort     uVar1;
    uchar     *puVar2;
    undefined2 uVar3;
    ushort    *puVar4;
    ushort    *puVar5;
    undefined  local_8[0x6];

    uVar3 = (undefined2)(param_1 >> 0x10);
    if(*(long *)((int)param_1 + 0x20) == 0x0)
    {
        return 0x0;
    }
    puVar4 = pass1_1008_3e38((ushort *)CONCAT22(param_5, local_8));
    puVar2 = (uchar *)((ulong)puVar4 >> 0x10);
    pass1_1018_161c(param_5, *(ulong *)((int)param_1 + 0x20), (ushort *)CONCAT22(param_5, local_8), (int)param_3, (int)(param_3 >> 0x10));
    puVar5 = (ushort *)CONCAT22(param_5, local_8);
    puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x32, param_5, puVar2, param_4);
    uVar1  = (ushort)puVar4;
    pass1_1010_71d6((ulong)puVar4, param_2, puVar5, uVar1, (uint)((ulong)puVar4 >> 0x10), param_5);
    return uVar1;
}


void __stdcall16far pass1_1018_262e(ulong param_1)

{
    undefined2 uVar1;

    uVar1                                = (undefined2)(param_1 >> 0x10);
    *(undefined2 *)((int)param_1 + 0x44) = 0x1;
    *(undefined4 *)((int)param_1 + 0x3e) = 0x0;
    return;
}


void __stdcall16far pass1_1018_2646(ushort param_1, ushort param_2, ushort *param_3)

{
    pass1_1008_3f62(param_3, &USHORT_1048_4222);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1018_265c(void)

{
    ulong uVar1;

    uVar1 = pass1_1030_8326();
    return uVar1;
}


ushort __stdcall16far pass1_1018_266a(ulong param_1)

{
    return *(ushort *)((int)param_1 + 0x44);
}


void __stdcall16far pass1_1018_2678(ushort param_1, ushort param_2, ushort *param_3)

{
    pass1_1008_3f62(param_3, &USHORT_1048_4216);
    return;
}


ulong __stdcall16far pass1_1018_268e(ulong param_1)

{
    astruct_287 *iVar1;
    int          iVar2;
    undefined2   uVar3;

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar1 = (astruct_287 *)param_1;
    if(iVar1->field_0x42 != 0x0)
    {
        *(undefined4 *)&iVar1->field_0x40 = 0x0;
        iVar1->field_0x44                 = 0x1;
    }
    iVar2 = iVar1->field_0x3e * 0x4;
    return CONCAT22(*(undefined2 *)(&iVar1[0x1].field_0x2 + iVar2), *(undefined2 *)(&iVar1[0x1].field_0x0 + iVar2));
}


void __stdcall16far pass1_1018_26c2(ushort param_1, ushort param_2, ushort *param_3)

{
    pass1_1008_3f62(param_3, &USHORT_1048_421c);
    return;
}


void __stdcall16far pass1_1018_26d8(ushort param_1, ushort param_2, int param_3, ushort *param_4)

{
    pass1_1008_3f62(param_4, (ushort *)CONCAT22(0x1050, (int)&USHORT_1050_65ca + param_3 * 0x6));
    return;
}


void __stdcall16far pass1_1018_26f8(ushort param_1, ushort param_2, ushort *param_3)

{
    pass1_1008_3f62(param_3, &USHORT_1048_4210);
    return;
}
void __stdcall16far pass1_1018_280c(ulong param_1)

{
    undefined4 uVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    if(*(int *)(iVar2 + 0x24) == 0x0)
    {
        return;
    }
    *(undefined2 *)(iVar2 + 0x24) = 0x0;
    if(*(long *)(iVar2 + 0x20) == 0x0)
    {
        *(undefined4 *)(iVar2 + 0x26) = 0x0;
    }
    else
    {
        uVar1                         = *(undefined4 *)(iVar2 + 0x20);
        *(undefined4 *)(iVar2 + 0x26) = *(undefined4 *)((int)uVar1 + 0x4c);
    }
    return;
}


void __stdcall16far pass1_1018_2862(ulong param_1)

{
    long         lVar1;
    astruct_654 *iVar2;
    undefined2   uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar2 = (astruct_654 *)param_1;
    if(iVar2->field_0x20 == 0x0)
    {
        iVar2->field_0x26 = 0x0;
    }
    else
    {
        lVar1             = iVar2->field_0x20;
        iVar2->field_0x26 = *(undefined4 *)((int)lVar1 + 0x4c);
    }
    return;
}


void __stdcall16far pass1_1018_289c(ulong param_1, int param_2, uint param_3, ushort param_4)

{
    uint uVar1;

    if(param_2 == 0x1)
    {
        *(undefined4 *)((int)param_1 + 0x4) = 0x0;
        return;
    }
    if(param_2 == 0x2)
    {
        pass1_1018_2922(param_1 & 0xffff0000 | (ulong)((int)param_1 - 0x1c));
    }
    else
    {
        if((((param_2 + -0x3 < 0x1) || (SBORROW2(param_2 + -0x3, 0x1))) || (0x1 < param_2 + -0x5)) || (*(long *)((int)param_1 + 0x4) == 0x0))
        {
            return;
        }
        uVar1 = (int)param_1 - 0x1c;
        pass1_1018_2862(param_1 & 0xffff0000 | (ulong)uVar1);
        if((param_3 | uVar1) != 0x0)
        {
            *(undefined2 *)((int)param_1 + 0x8) = 0x1;
        }
    }
    pass1_1010_1f62(param_4, param_1 & 0xffff0000 | (ulong)((int)param_1 - 0x1c), param_2);
    return;
}


void __stdcall16far pass1_1018_2922(ulong param_1)

{
    int       *piVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    if((*(int *)(iVar2 + 0x40) != 0x0) && (piVar1 = (int *)(iVar2 + 0x3e), *piVar1 = *piVar1 + 0x1, 0x9 < *(int *)(iVar2 + 0x3e)))
    {
        *(undefined2 *)(iVar2 + 0x3e) = 0x0;
        *(undefined2 *)(iVar2 + 0x42) = 0x1;
    }
    return;
}

void __cdecl16far pass1_1018_2aa3(void)

{
    pass1_1018_21f8();
    return;
}


void __stdcall16far pass1_1018_0ae8(ulong param_1, ushort param_2)

{
    *(ushort *)((int)param_1 + 0x5e) = param_2;
    return;
}


ushort __stdcall16far pass1_1018_0afa(ulong param_1)

{
    return *(ushort *)((int)param_1 + 0x5e);
}


ulong __stdcall16far pass1_1018_0b08(ulong param_1)

{
    undefined4 uVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar1 = *(undefined4 *)((int)param_1 + 0x7c);
    uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    return CONCAT22(*(undefined2 *)(iVar2 + 0x6), *(undefined2 *)(iVar2 + 0x4));
}


void __stdcall16far pass1_1018_0b1e(ushort *param_1, ushort *param_2, ushort param_3)

{
    int         iVar1;
    undefined4  uVar2;
    astruct_74 *iVar3;
    undefined2  uVar3;
    ushort      local_8;
    int         local_6;
    int         local_4;

    iVar3 = (astruct_74 *)param_1;
    iVar3 = (astruct_74 *)&iVar3->field_0x30;
    pass1_1008_3eb4((ushort *)((ulong)param_1 & 0xffff0000 | ZEXT24(iVar3)), (ushort *)CONCAT22(param_3, &local_8), (ushort *)CONCAT22(param_3, &local_6), (ushort *)CONCAT22(param_3, &local_4));
    if(local_4 + -0x3 < 0x1)
    {
        local_4 = 0x3;
    }
    if(local_6 + -0x3 < 0x1)
    {
        local_6 = 0x3;
    }
    uVar3 = (undefined2)((ulong)param_1 >> 0x10);
    uVar2 = iVar3->field_0x5a;
    iVar1 = *(int *)((int)uVar2 + 0x4);
    if(iVar1 <= local_4 + 0x2)
    {
        local_4 = iVar1 + -0x3;
    }
    uVar2 = iVar3->field_0x5a;
    iVar1 = *(int *)((int)uVar2 + 0x8);
    if(iVar1 <= local_6 + 0x2)
    {
        local_6 = iVar1 + -0x3;
    }
    pass1_1008_6cec((ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x40), local_8, CONCAT22(local_4 + 0x2, local_6 + 0x2), local_8, CONCAT22(local_4 + -0x3, local_6 + -0x3));
    pass1_1008_3f62(param_2, (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x40));
    pass1_1008_3f62((ushort *)((ulong)param_2 & 0xffff0000 | (ulong)((int)param_2 + 0x6)), (ushort *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar3->field_0x46));
    return;
}


void __stdcall16far pass1_1018_0bf4(ushort param_1, int param_2, ulong param_3, int param_4)

{
    undefined4 uVar1;
    uint       uVar2;
    ushort     uVar3;
    long       lVar4;
    uint       uVar5;
    undefined  local_14[0xc];
    ushort     local_8;
    undefined4 local_6;

    switch(param_4)
    {
    case 0x1:
        *(undefined4 *)((int)param_3 + 0xc)  = 0x0;
        *(undefined4 *)((int)param_3 + 0x7e) = 0x0;
        return;
    case 0x4:
        pass1_1008_3eb4((ushort *)(param_3 & 0xffff0000 | (ulong)((int)param_3 + 0x10)), (ushort *)CONCAT22(param_1, &local_8), (ushort *)CONCAT22(param_1, &local_6), (ushort *)CONCAT22(param_1, (int)&local_6 + 0x2));
        uVar1   = *(undefined4 *)((int)param_3 + 0xc);
        local_8 = *(ushort *)((int)uVar1 + 0x1e);
        pass1_1008_3e76((ushort *)(param_3 & 0xffff0000 | (ulong)((int)param_3 + 0x10)), local_8, (ushort)local_6, (ushort)((ulong)local_6 >> 0x10));
        pass1_1008_6c90((ushort *)CONCAT22(param_1, local_14));
        pass1_1018_0b1e((ushort *)(param_3 & 0xffff0000 | (ulong)((int)param_3 - 0x20)), (ushort *)CONCAT22(param_1, local_14), param_1);
        goto LAB_1018_0c71;
    case 0x5:
    case 0x6:
        uVar2 = (int)param_3 - 0x20;
        uVar5 = param_3._2_2_;
        pass1_1018_0dc6(param_3 & 0xffff0000 | (ulong)uVar2, param_1);
        pass1_1018_10c4(param_1, uVar5, param_3 & 0xffff0000 | (ulong)uVar2);
        pass1_1018_1346(param_1, uVar5, (astruct_93 *)(param_3 & 0xffff0000 | (ulong)uVar2));
    LAB_1018_0c71:
        *(undefined4 *)((int)param_3 + 0x2c) = 0x0;
        lVar4                                = *(long *)((int)param_3 + 0x1c);
        uVar3                                = *(ushort *)((int)param_3 + 0x1e);
        uVar1                                = *(undefined4 *)((int)param_3 + 0xc);
        if(*(long *)((int)uVar1 + 0x20) == lVar4)
        {
            pass1_1018_028c(*(ulong *)((int)param_3 + 0xc), *(ulong *)((int)param_3 + 0x1c), (ushort)lVar4, uVar3, param_1);
            *(undefined2 *)((int)param_3 + 0x2c) = (int)lVar4;
            *(ushort *)((int)param_3 + 0x2e)     = uVar3;
            pass1_1010_1f62(param_1, param_3 & 0xffff0000 | (ulong)((int)param_3 - 0x20), param_4);
            return;
        }
        break;
    case 0x14:
        uVar1 = *(undefined4 *)((int)param_3 + 0xc);
        if(*(long *)((int)uVar1 + 0x20) != *(long *)((int)param_3 + 0x1c))
        {
            post_win_msg_1020_291a(0x1020);
            return;
        }
        break;
    case 0x15:
        uVar3 = pass1_1010_65d0(param_1, *(ulong *)((int)param_3 + 0x7e), 0x88);
        if(uVar3 != 0x0)
        {
            *(undefined2 *)((int)param_3 + 0x88) = 0x1;
            return;
        }
    }
    return;
}


void __stdcall16far pass1_1018_0d76(ulong param_1)

{
    undefined4 uVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    uVar1 = *(undefined4 *)((int)param_1 + 0x2c);
    if(*(long *)((int)uVar1 + 0x20) == *(long *)((int)param_1 + 0x3c))
    {
        return;
    }
    return;
}


void __stdcall16far pass1_1018_0d9a(ulong param_1, ushort *param_2, ulong *param_3)

{
    undefined4 uVar1;
    undefined2 uVar2;

    uVar2    = (undefined2)(param_1 >> 0x10);
    uVar1    = *(undefined4 *)((int)param_1 + 0x7c);
    *param_3 = *(ulong *)((int)uVar1 + 0x16);
    uVar1    = *(undefined4 *)((int)param_1 + 0x7c);
    *param_2 = *(ushort *)((int)uVar1 + 0x1a);
    return;
}

void __stdcall16far pass1_1018_1054(ulong param_1, ushort *param_2, ulong *param_3, ushort param_4)

{
    int  iVar1;
    uint uVar2;

    uVar2 = (uint)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    if(*(long *)(iVar1 + 0x94) == 0x0)
    {
        pass1_1018_0dc6(param_1 & 0xffff | (ulong)uVar2 << 0x10, param_4);
    }
    *param_3 = *(ulong *)(iVar1 + 0x94);
    *param_2 = *(ushort *)(iVar1 + 0x92);
    return;
}


void __stdcall16far pass1_1018_108c(ulong param_1, ushort *param_2, ulong *param_3, ushort param_4)

{
    int  iVar1;
    uint uVar2;

    uVar2 = (uint)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    if(*(long *)(iVar1 + 0x9a) == 0x0)
    {
        pass1_1018_0dc6(param_1 & 0xffff | (ulong)uVar2 << 0x10, param_4);
    }
    *param_3 = *(ulong *)(iVar1 + 0x9a);
    *param_2 = *(ushort *)(iVar1 + 0x98);
    return;
}

void __stdcall16far pass1_1018_15f6(ulong param_1, ushort *param_2, ulong *param_3)

{
    undefined2 uVar1;

    uVar1    = (undefined2)(param_1 >> 0x10);
    *param_3 = *(ulong *)((int)param_1 + 0x8e);
    *param_2 = *(ushort *)((int)param_1 + 0x8c);
    return;
}


void __stdcall16far pass1_1018_161c(ushort param_1, ulong param_2, ushort *param_3, int param_4, int param_5)

{
    ushort uVar1;
    ushort uVar2;
    ulong  local_6;

    pass1_1008_3e94((ushort *)(param_2 & 0xffff0000 | (ulong)((int)param_2 + 0x36)), (ushort *)CONCAT22(param_1, &local_6), (ushort *)CONCAT22(param_1, (int)&local_6 + 0x2));
    uVar1   = local_6._2_2_ + param_5 + -0x3;
    uVar2   = (int)local_6 + param_4 + -0x3;
    local_6 = CONCAT22(uVar1, uVar2);
    pass1_1008_3e76(param_3, *(ushort *)((int)param_2 + 0x44), uVar2, uVar1);
    return;
}


void __stdcall16far pass1_1018_1662(ulong param_1, int param_2, int param_3, ushort param_4)

{
    int local_6;
    int local_4;

    pass1_1008_3e94((ushort *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x36)), (ushort *)CONCAT22(param_4, &local_6), (ushort *)CONCAT22(param_4, &local_4));
    pass1_1018_16b8(param_1, *(ushort *)((int)param_1 + 0x44), CONCAT22(local_4 + param_3, local_6 + param_2), param_4);
    return;
}


void __stdcall16far pass1_1018_169e(ulong param_1, ulong param_2, ushort param_3)

{
    uint uVar1;

    uVar1 = (uint)(param_1 >> 0x10);
    pass1_1018_16b8(param_1 & 0xffff | (ulong)uVar1 << 0x10, *(ushort *)((int)param_1 + 0x44), param_2, param_3);
    return;
}


// WARNING: Unable to use type for symbol uVar2

void __stdcall16far pass1_1018_16b8(ulong param_1, ushort param_2, ulong param_3, ushort param_4)

{
    int        iVar1;
    undefined4 uVar3;
    long       lVar4;
    ushort     uVar5;
    int        iVar6;
    ushort     uVar7;
    ushort     uVar8;
    undefined  local_6[0x2];
    undefined  local_4[0x2];
    undefined4 uVar2;

    if(param_3._2_2_ + -0x3 < 0x1)
    {
        param_3 = CONCAT22(0x3, (int)param_3);
    }
    if((int)param_3 + -0x3 < 0x1)
    {
        param_3 = CONCAT22(param_3._2_2_, 0x3);
    }
    uVar7 = (ushort)(param_1 >> 0x10);
    iVar6 = (int)param_1;
    uVar2 = *(undefined4 *)(iVar6 + 0x5a);
    iVar1 = *(int *)((int)uVar2 + 0x4);
    if(iVar1 <= param_3._2_2_ + 0x2)
    {
        param_3 = param_3 & 0xffff | (ulong)(iVar1 - 0x3) << 0x10;
    }
    uVar3 = *(undefined4 *)(iVar6 + 0x5a);
    iVar1 = *(int *)((int)uVar3 + 0x8);
    if(iVar1 <= (int)param_3 + 0x2)
    {
        param_3 = param_3 & 0xffff0000 | (ulong)(iVar1 - 0x3);
    }
    uVar8 = (ushort)(param_3 >> 0x10);
    pass1_1008_3e76((ushort *)(param_1 & 0xffff0000 | (ulong)(iVar6 + 0x30)), param_2, (ushort)param_3, uVar8);
    uVar5 = uVar7;
    pass1_1008_3e94((ushort *)(param_1 & 0xffff0000 | (ulong)(iVar6 + 0x36U)), (ushort *)CONCAT22(param_4, local_6), (ushort *)CONCAT22(param_4, local_4));
    pass1_1008_3e76((ushort *)(param_1 & 0xffff0000 | (ulong)(iVar6 + 0x36U)), 0x0, (ushort)param_3, uVar8);
    *(undefined4 *)(iVar6 + 0x4c) = 0x0;
    lVar4                         = *(long *)(iVar6 + 0x3c);
    uVar3                         = *(undefined4 *)(iVar6 + 0x2c);
    if(*(long *)((int)uVar3 + 0x20) == lVar4)
    {
        pass1_1018_028c(*(ulong *)(iVar6 + 0x2c), *(ulong *)(iVar6 + 0x3c), (ushort)lVar4, uVar5, param_4);
        *(undefined2 *)(iVar6 + 0x4c) = (int)lVar4;
        *(ushort *)(iVar6 + 0x4e)     = uVar5;
        pass1_1010_1f62(param_4, param_1, 0x4);
    }
    return;
}


void __stdcall16far pass1_1018_179e(ulong param_1, ulong param_2, ushort param_3, ushort param_4)

{
    ushort     local_8;
    undefined4 local_6;

    pass1_1008_3eb4((ushort *)param_2, (ushort *)CONCAT22(param_4, &local_8), (ushort *)CONCAT22(param_4, &local_6), (ushort *)CONCAT22(param_4, (int)&local_6 + 0x2));
    pass1_1018_16b8(param_1, local_8, local_6, param_4);
    return;
}


void __stdcall16far pass1_1018_17ce(ulong param_1, ulong param_2, ulong param_3, ushort param_4, uchar param_5)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    pass1_1018_0412(*(ulong *)((int)param_1 + 0x2c), (ushort)param_2, CONCAT22((int)param_3, (int)(param_2 >> 0x10)), (ushort)(param_3 >> 0x10), *(ulong *)((int)param_1 + 0x3c), param_4, param_5);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

int __stdcall16far pass1_1018_17f0(void)

{
    int iStack4;

    iStack4 = 0x0;
    while((iStack4 < 0x4 && (*(int *)(iStack4 * 0x2 + (int)_PTR_LOOP_1050_3962) != 0x0)))
    {
        iStack4 = iStack4 + 0x1;
    }
    return iStack4;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1018_181c(ulong param_1, char *param_2, uchar param_3, ushort param_4)

{
    undefined  in_AH;
    undefined2 uVar1;

    uVar1 = CONCAT11(in_AH, param_3);
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), *(ulong *)((int)param_1 + 0x3c));
    pass1_1030_5b6c(CONCAT22(param_4, uVar1), param_2, param_4);
    return;
}

void __stdcall16far pass1_1018_1b02(ushort param_1, ulong param_2, int param_3)

{
    int        *piVar1;
    ulong       uVar2;
    ulong       uVar3;
    astruct_96 *uVar4;
    astruct_95 *iVar5;
    undefined2  uVar5;
    int         iStack12;
    ushort      local_6;
    undefined   local_4[0x2];

    iStack12 = 0x0;
    while(true)
    {
        uVar5  = (undefined2)(param_2 >> 0x10);
        iVar5  = (astruct_95 *)param_2;
        piVar1 = &iVar5->field_0x44;
        if(*piVar1 == iStack12 || *piVar1 < iStack12)
            break;
        uVar2   = iVar5->field_0x40;
        uVar4   = (astruct_96 *)uVar2;
        uVar4   = uVar4 + iStack12;
        uVar2   = uVar2 & 0xffff0000;
        uVar3   = ZEXT24(uVar4);
        piVar1  = &uVar4->field_0x6;
        *piVar1 = *piVar1 + param_3 * 0x2 + -0x1;
        uVar5   = (undefined2)(uVar2 >> 0x10);
        if(0x23 < uVar4->field_0x6)
        {
            uVar4->field_0x6 = 0x0;
        }
        if(uVar4->field_0x6 < 0x0)
        {
            uVar4->field_0x6 = 0x23;
        }
        pass1_1008_3f62((ushort *)(uVar2 | (uint)&uVar4->field_0x10), (ushort *)(uVar2 | uVar3));
        uVar4->field_0x16 = uVar4->field_0xa;
        pass1_1008_3e94((ushort *)(uVar2 | uVar3), (ushort *)CONCAT22(param_1, &local_6), (ushort *)CONCAT22(param_1, local_4));
        pass1_1008_3e76((ushort *)(uVar2 | uVar3), 0x0, local_6, *(ushort *)((uVar4->field_0x8 * 0x24 + uVar4->field_0x6) * 0x2 + 0x3c20));
        uVar4->field_0xa = *(undefined2 *)(uVar4->field_0x6 * 0x2 + 0x3966);
        iStack12         = iStack12 + 0x1;
    }
    pass1_1010_1f62(param_1, param_2, 0xd);
    return;
}

void __stdcall16far pass1_1010_ebf8(ulong param_1, ushort param_2, ushort param_3, int param_4)

{
    undefined2 uVar1;

    uVar1                                            = (undefined2)(param_1 >> 0x10);
    *(ushort *)((int)param_1 + param_4 * 0x4 + 0x34) = param_2;
    *(ushort *)((int)param_1 + param_4 * 0x4 + 0x36) = param_3;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1010_ec18(ushort param_1, ushort param_2, ulong param_3, int param_4, ushort param_5)

{
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_3, (uint)(param_3 >> 0x10));
    return CONCAT22(*(undefined2 *)(param_4 + 0x12), *(undefined2 *)(param_4 + 0x10));
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1010_ec40(ushort param_1, ushort param_2, ulong param_3, int param_4, ushort param_5)

{
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)param_3, (uint)(param_3 >> 0x10));
    return CONCAT22(*(undefined2 *)(param_4 + 0x12), *(undefined2 *)(param_4 + 0x10));
}


void __stdcall16far pass1_1010_ec68(ulong param_1, ulong param_2, ushort param_3)

{
    uint uVar1;

    uVar1                           = (uint)(param_1 >> 0x10);
    *(ulong *)((int)param_1 + 0x28) = param_2;
    pass1_1010_1f62(param_3, param_1 & 0xffff | (ulong)uVar1 << 0x10, 0x13);
    return;
}

void __stdcall16far pass1_1010_ecc6(ulong param_1, ushort *param_2, long param_3, ushort param_4, uint param_5, ushort param_6)

{
    undefined4 uVar1;
    int        iVar2;
    undefined2 uVar3;

    pass1_1030_627e(param_6, param_4, param_5, _PTR_LOOP_1050_5740, param_2, param_3);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_4, param_5);
    uVar1 = *(undefined4 *)(param_4 + 0x2e);
    uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    if(*(long *)(iVar2 + 0x200) == 0x8000001)
    {
        pass1_1010_ed22(param_1, *(ulong *)(iVar2 + 0x4), param_6);
    }
    return;
}


void __stdcall16far pass1_1010_ed22(ulong param_1, ulong param_2, ushort param_3)

{
    uint uVar1;

    uVar1                           = (uint)(param_1 >> 0x10);
    *(ulong *)((int)param_1 + 0x24) = param_2;
    pass1_1010_1f62(param_3, param_1 & 0xffff | (ulong)uVar1 << 0x10, 0x12);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_ed3e(ulong param_1)

{
    undefined4 uVar1;

    uVar1 = *(undefined4 *)((int)param_1 + 0x16);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar1, (uint)((ulong)uVar1 >> 0x10));
    return;
}

void __stdcall16far pass1_1018_017c(ulong param_1, ushort param_2, ushort param_3)

{
    uint uVar1;

    uVar1                            = (uint)(param_1 >> 0x10);
    *(ushort *)((int)param_1 + 0x1e) = param_2;
    pass1_1010_1f62(param_3, param_1 & 0xffff | (ulong)uVar1 << 0x10, 0x4);
    return;
}

void __stdcall16far pass1_1018_03ea(ulong param_1, int param_2, ushort param_3)

{
    if(param_2 != 0x5)
    {
        return;
    }
    pass1_1010_1f62(param_3, param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xa), 0x5);
    return;
}

void __stdcall16far pass1_1018_04f2(ulong param_1)

{
    undefined4 *puVar1;
    uint        uVar2;
    code      **ppcVar3;
    int         iVar4;
    ushort      uVar5;

    uVar5  = (ushort)(param_1 >> 0x10);
    iVar4  = (int)param_1;
    puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x12);
    uVar2  = *(uint *)(iVar4 + 0x14);
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    *(undefined4 *)(iVar4 + 0x12) = 0x0;
    return;
}

void __stdcall16far pass1_1018_0902(ulong *param_1, ulong param_2)

{
    undefined4   uVar1;
    code       **ppcVar2;
    astruct_76 **ppaVar3;
    astruct_76 **ppaVar4;
    int          iVar5;
    undefined2   uVar6;
    ulong        uVar7;
    ulong       *puVar8;
    ulong       *puVar9;

    puVar9  = (ulong *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x28));
    ppaVar3 = (astruct_76 **)((int)param_1 + 0x24);
    puVar8  = (ulong *)((ulong)param_1 & 0xffff0000 | ZEXT24(ppaVar3));
    uVar6   = param_1._2_2_;
    ppaVar4 = ppaVar3;
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), param_2);
    pass1_1030_5a52(CONCAT22(uVar6, ppaVar4), puVar8, puVar9);
    uVar7                                                   = pass1_1008_4772(*ppaVar3);
    *(undefined2 *)((int)param_1 + 0x5a)                    = (int)uVar7;
    *(undefined2 *)((int)param_1 + 0x5c)                    = (int)(uVar7 >> 0x10);
    iVar5                                                   = pass1_1018_17f0();
    *(int *)((int)param_1 + 0x12)                           = iVar5 + 0x2;
    *(undefined2 *)(iVar5 * 0x2 + (int)_PTR_LOOP_1050_3962) = 0x1;
    ppcVar2                                                 = (code **)((int)*param_1 + 0x18);
    (**ppcVar2)();
    *(ulong *)((int)param_1 + 0x3c)      = param_2;
    uVar1                                = *(undefined4 *)((int)param_1 + 0x2c);
    uVar7                                = pass1_1010_ec18((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), param_2 & 0xffff0000 | (ulong) * (uint *)((int)param_1 + 0x3c), (int)param_2, param_2._2_2_);
    *(undefined2 *)((int)param_1 + 0x7c) = (int)uVar7;
    *(undefined2 *)((int)param_1 + 0x7e) = (int)(uVar7 >> 0x10);
    return;
}

ulong __stdcall16far pass1_1018_0a50(ulong param_1)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    iVar1 = (int)param_1;
    if(*(int *)(iVar1 + 0x84) == 0x2)
    {
        return CONCAT22(*(undefined2 *)(iVar1 + 0x2a), *(undefined2 *)(iVar1 + 0x28));
    }
    return CONCAT22(*(undefined2 *)(iVar1 + 0x26), *(undefined2 *)(iVar1 + 0x24));
}


void __stdcall16far pass1_1018_0a76(ulong param_1, ushort param_2)

{
    undefined2 uVar1;
    uint       uVar2;

    uVar2 = (uint)(param_1 >> 0x10);
    if(*(int *)((int)param_1 + 0x84) == 0x1)
    {
        uVar1 = 0x2;
    }
    else
    {
        uVar1 = 0x1;
    }
    *(undefined2 *)((int)param_1 + 0x84) = uVar1;
    pass1_1010_1f62(param_2, param_1 & 0xffff | (ulong)uVar2 << 0x10, 0x4);
    return;
}


void __stdcall16far pass1_1018_0aa0(ulong param_1, ushort param_2)

{
    int        iVar1;
    undefined2 uVar2;

    uVar2                     = (undefined2)(param_1 >> 0x10);
    iVar1                     = (int)param_1;
    *(ushort *)(iVar1 + 0x14) = param_2;
    pass1_1018_04de(*(ulong *)(iVar1 + 0x2c), *(ulong *)(iVar1 + 0x3c));
    return;
}


void __stdcall16far pass1_1018_0ac0(ulong param_1, ulong param_2)

{
    *(ulong *)((int)param_1 + 0x80) = param_2;
    return;
}


ulong __stdcall16far pass1_1018_0ad4(ulong param_1)

{
    undefined2 uVar1;

    uVar1 = (undefined2)(param_1 >> 0x10);
    return CONCAT22(*(undefined2 *)((int)param_1 + 0x82), *(undefined2 *)((int)param_1 + 0x80));
}

int __stdcall16far pass1_1010_e128(ushort param_1, ushort param_2, int param_3, int param_4, ulong param_5)

{
    int iStack6;
    int iStack4;

    iStack4 = 0x0;
    for(iStack6 = param_4; iStack6 <= param_3; iStack6 = iStack6 + 0x1)
    {
        if(*(int *)(iStack6 * 0x2 + (int)param_5) != 0x0)
        {
            iStack4 = iStack4 + 0x1;
        }
    }
    return iStack4;
}

void __stdcall16far pass1_1010_e15e(ulong param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    code      **ppcVar1;
    ushort      uVar2;
    uint        uVar3;
    ulong       uVar4;
    undefined2  extraout_DX;
    uint        extraout_DX_00;
    uint        uVar5;
    ulong       uStack18;
    ulong       uStack14;
    undefined4 *puStack10;

    pass1_1010_afde(param_1, 0xc);
    puStack10 = (undefined4 *)CONCAT22(param_3, param_2);
    ppcVar1   = (code **)((int)*puStack10 + 0x10);
    uVar2     = param_2;
    (**ppcVar1)(param_4, param_2, param_3);
    uStack14 = CONCAT22(extraout_DX, uVar2);
    for(uStack18 = 0x0; uStack18 < uStack14; uStack18 = uStack18 + 0x1)
    {
        ppcVar1 = (code **)((int)*puStack10 + 0x4);
        uVar4   = uStack14;
        (**ppcVar1)(param_4, param_2, param_3, (char)uStack18, (int)(uStack18 >> 0x10));
        uVar3 = (uint)uVar4;
        uVar5 = extraout_DX_00;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3, extraout_DX_00);
        param_4 = 0x1030;
        pass1_1030_7c28(CONCAT13((char)(uVar5 >> 0x8), CONCAT12((char)uVar5, uVar3)), 0x23, uVar3, uVar5, param_5);
    }
    if(puStack10 != (undefined4 *)0x0)
    {
        ppcVar1 = (code **)*puStack10;
        (**ppcVar1)(param_4, param_2, (char)param_3, 0x1);
    }
    return;
}

void __stdcall16far pass1_1010_e1f4(ulong param_1, ulong param_2, ushort param_3, ushort param_4)

{
    undefined2 uVar1;
    BOOL16     BVar2;
    char      *pcVar3;
    ushort     uVar4;
    undefined2 uVar5;
    int        iVar6;
    short      in_buf_len_5;
    ulong      uVar7;

    in_buf_len_5                  = (short)(param_1 >> 0x10);
    iVar6                         = (int)param_1;
    *(undefined *)(iVar6 + 0x13c) = 0x0;
    uVar7                         = struct_op_1030_73a8(param_2);
    uVar5                         = (undefined2)(uVar7 >> 0x10);
    uVar1                         = *(undefined2 *)((int)uVar7 + 0xc);
    BVar2                         = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0xc);
    if((((((((BVar2 == 0x0) && (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x14), BVar2 == 0x0)) && (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0xa), BVar2 == 0x0))
           && ((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x15), BVar2 == 0x0 && (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0xb), BVar2 == 0x0))))
          && (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x16), BVar2 == 0x0))
         && (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x17), BVar2 == 0x0 && (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x21), BVar2 == 0x0))
              && ((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x1c),
                   BVar2 == 0x0
                     && (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x1d), BVar2 == 0x0 && (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x18), BVar2 == 0x0))
                          && (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x19), BVar2 == 0x0))))))))
        && ((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x4), BVar2 == 0x0 && (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x3), BVar2 == 0x0))))
       && (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x1e),
             BVar2 == 0x0
               && (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x23), BVar2 == 0x0 && (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x1b), BVar2 == 0x0))
                    && ((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x1f),
                         BVar2 == 0x0
                           && (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x1), BVar2 == 0x0 && (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x2), BVar2 == 0x0))
                                && (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x13), BVar2 == 0x0))))))))
            && (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x20), BVar2 == 0x0 && (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0xe), BVar2 == 0x0))
                 && (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x10), BVar2 == 0x0))))))
    {
        pcVar3 = (char *)pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x12);
        if((pcVar3 == (char *)0x0) && (pcVar3 = (char *)pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x11), pcVar3 == (char *)0x0))
        {
            BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x6);
            if(BVar2 == 0x0)
            {
                BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x5);
                if(BVar2 == 0x0)
                {
                    pass1_1008_c6ae(_PTR_LOOP_1050_06e0, uVar1, 0x40);
                    goto LAB_1010_e241;
                }
                uVar4  = pass1_1030_7f98(param_2);
                pcVar3 = string_op_1020_c222(uVar4);
            }
            else
            {
                uVar4  = pass1_1030_7f5a(param_2);
                pcVar3 = string_op_1020_c2f8(uVar4);
            }
        }
        else
        {
            pass1_1010_e58a(param_1, uVar7, uVar5, param_3, param_4);
        }
        unk_str_op_1000_3d3e((char *)(param_1 & 0xffff0000 | (ulong)(iVar6 + 0x13c)), (char *)CONCAT22(uVar5, pcVar3));
    }
    else
    {
    LAB_1010_e241:
        load_string_1010_84e0(0x1008, (ushort)_PTR_LOOP_1050_14cc, (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, (char *)(iVar6 + 0x13c), in_buf_len_5);
    }
    return;
}

void __stdcall16far pass1_1010_e8d0(ushort param_1, ushort param_2, ushort *param_3, ushort *param_4, ulong param_5, ushort param_6)

{
    pass1_1030_7064(param_5);
    *param_4 = param_6;
    pass1_1030_70ac(param_5);
    *param_3 = param_6;
    return;
}

void __stdcall16far pass1_1010_c1ba(ushort param_1, ushort param_2, int param_3, uint param_4, ushort param_5)

{
    undefined *puVar1;
    int        iStack28;
    undefined  local_16[0x12];
    undefined2 uStack4;

    uStack4 = 0x0;
    pass1_1028_dc52((astruct_92 *)CONCAT22(param_5, local_16), 0x1, 0x0, 0x200);
    iStack28 = 0x1;
    while(true)
    {
        puVar1 = local_16;
        pass1_1028_e4ec(CONCAT22(param_5, puVar1));
        param_4 = param_4 | (uint)puVar1;
        if((param_4 == 0x0) || (iStack28 == param_3))
            break;
        iStack28 = iStack28 + 0x1;
    }
    return;
}


char *__stdcall16far pass1_1010_c234(uint param_1, uchar *param_2, int param_3, ushort param_4)

{
    char *pcVar1;

    pass1_1010_c28a(param_2, param_3, param_4);
    if(((uint)param_2 | param_1) == 0x0)
    {
        return (char *)0x0;
    }
    pcVar1 = pass1_1038_4d28(CONCAT22(param_2, param_1));
    return pcVar1;
}


void __stdcall16far pass1_1010_c25e(ushort param_1, ushort param_2, char *param_3, uint param_4, uchar *param_5, int param_6, ushort param_7)

{
    if(param_3 != (char *)0x0)
    {
        pass1_1010_c28a(param_5, param_6, param_7);
        if(((uint)param_5 | param_4) != 0x0)
        {
            pass1_1038_4d3c(CONCAT22(param_5, param_4), param_3, (uint)param_5 | param_4);
        }
    }
    return;
}
void __stdcall16far pass1_1010_c2d8(ushort param_1, ushort param_2, long param_3)

{
    undefined2 uVar1;
    uint       uStack6;

    if((param_3 != 0x0) && (uVar1 = (undefined2)((ulong)param_3 >> 0x10), uStack6 = (uint) * (undefined4 *)((int)param_3 + 0x2e), (*(uint *)((int)param_3 + 0x30) | uStack6) != 0x0))
    {
        return;
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1010_c312(void)

{
    return CONCAT22(*(undefined2 *)((int)_PTR_LOOP_1050_65e2 + 0x2), *_PTR_LOOP_1050_65e2);
}
void __stdcall16far pass1_1010_988c(ulong param_1, int param_2)

{
    code     **ppcVar1;
    undefined4 uVar2;
    int        iVar3;
    int        iVar4;
    undefined2 uVar5;
    int        iVar6;
    undefined2 uVar7;
    undefined2 unaff_SS;
    long       lVar8;
    undefined  local_a[0x8];

    uVar7 = (undefined2)(param_1 >> 0x10);
    iVar6 = (int)param_1;
    pass1_1008_5784((ulong *)CONCAT22(unaff_SS, local_a), *(ulong *)(iVar6 + 0xe));
    do
    {
        lVar8 = pass1_1008_5b12(local_a, unaff_SS);
        uVar5 = (undefined2)((ulong)lVar8 >> 0x10);
        iVar3 = (int)lVar8;
        if(lVar8 == 0x0)
        {
            return;
        }
    } while(*(int *)(iVar3 + 0x4) != param_2);
    iVar4                 = *(int *)(iVar3 + 0x6) + -0x1;
    *(int *)(iVar3 + 0x6) = iVar4;
    if((iVar4 < 0x1) && (ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar6 + 0xe) + 0xc), (**ppcVar1)(0x1008, *(undefined4 *)(iVar6 + 0xe), lVar8), uVar2 = *(undefined4 *)(iVar6 + 0xe), *(int *)((int)uVar2 + 0x8) == 0x0))
    {
        *(undefined2 *)(iVar6 + 0x16) = 0x1;
    }
    return;
}


void __stdcall16far pass1_1010_9f72(ulong param_1, int param_2, ushort param_3)

{
    ushort uVar1;

    uVar1 = (ushort)(param_1 >> 0x10);
    pass1_1010_9fa6((ushort)param_1, uVar1, *(ulong *)((ushort)param_1 + 0xe), param_2, param_3);
    return;
}


void __stdcall16far pass1_1010_9f8c(ulong param_1, int param_2, ushort param_3)

{
    ushort uVar1;

    uVar1 = (ushort)(param_1 >> 0x10);
    pass1_1010_9fa6((ushort)param_1, uVar1, *(ulong *)((ushort)param_1 + 0xa), param_2, param_3);
    return;
}


undefined2 __stdcall16far pass1_1010_9fa6(ushort param_1, ushort param_2, ulong param_3, int param_4, ushort param_5)

{
    undefined2 uVar1;
    long       lVar2;
    undefined  local_a[0x8];

    if(param_3 != 0x0)
    {
        pass1_1008_5784((ulong *)CONCAT22(param_5, local_a), param_3);
        while(true)
        {
            lVar2 = pass1_1008_5b12(local_a, param_5);
            uVar1 = (undefined2)((ulong)lVar2 >> 0x10);
            if(lVar2 == 0x0)
                break;
            if(*(int *)((int)lVar2 + 0x4) == param_4)
            {
                return *(undefined2 *)((int)lVar2 + 0x6);
            }
        }
    }
    return 0x0;
}

void __stdcall16far pass1_1010_a478(ushort *param_1, ushort param_2)

{
    undefined2  *puVar1;
    undefined2   uVar2;
    astruct_497 *uVar3;
    undefined2   uVar4;
    undefined2  *puStack6;

    uVar4            = (undefined2)((ulong)param_1 >> 0x10);
    uVar3            = (astruct_497 *)param_1;
    *param_1         = 0xe9cc;
    uVar3->field_0x2 = 0x1010;
    uVar3->field_0xa = 0xe9dc;
    uVar3->field_0xc = 0x1010;
    if(uVar3->field_0x138 != 0x0)
    {
        if(param_1 == (ushort *)0x0)
        {
            puVar1 = (undefined2 *)0x0;
            uVar2  = 0x0;
        }
        else
        {
            puVar1 = &uVar3->field_0xa;
            uVar2  = uVar4;
        }
        pass1_1010_1ea6(uVar3->field_0x138, CONCAT22(uVar2, puVar1), param_2);
    }
    uVar3->field_0x138 = 0x0;
    if(param_1 == (ushort *)0x0)
    {
        puVar1 = (undefined2 *)0x0;
        uVar4  = 0x0;
    }
    else
    {
        puVar1 = &uVar3->field_0xa;
    }
    puStack6    = (undefined2 *)CONCAT22(uVar4, puVar1);
    *puStack6   = 0x389a;
    puVar1[0x1] = 0x1008;
    pass1_1010_1d80(param_1, param_2);
    return;
}

void __stdcall16far pass1_1010_a50c(astruct_20 *param_1, ulong param_2, ulong param_3)

{
    int          iVar1;
    astruct_260 *iVar2;
    undefined4   local_8;
    int          iStack4;

    iVar1 = (int)param_1;
    pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0xa4)), (WNDCLASS16 *)0x0, 0x94);
    iVar2   = (astruct_260 *)(*(int *)((int)param_3 + 0xa) * 0x6 + iVar1);
    local_8 = iVar2->field_0xe;
    iStack4 = iVar2->field_0x12;
    (*(code *)local_8)(0x1000, iVar1 + iStack4, param_1._2_2_, param_2, param_3);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_a568(ushort param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), 0x8000001);
    pass1_1030_2622(CONCAT22(param_5, param_4), param_3);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_a58a(ushort param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), 0x8000001);
    pass1_1030_266c(param_4, CONCAT22(param_3, param_5));
    return;
}

void __stdcall16far pass1_1010_a5ca(ushort param_1, ushort param_2, ushort param_3, ushort param_4, ushort param_5)

{
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), 0x8000001);
    pass1_1030_2242(CONCAT22(param_5, param_4), param_3);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_a5ec(ushort param_1, ushort param_2, ushort param_3, ulong param_4, ushort param_5)

{
    code      **ppcVar1;
    uint        uVar2;
    ushort      uVar3;
    ushort      uVar4;
    ushort      uVar5;
    ushort      extraout_DX;
    undefined4 *puVar6;
    ulong       uStack6;

    uVar2 = param_4._2_2_ | (uint)param_4;
    if(param_4 != 0x0)
    {
        pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), 0x8000001);
        uStack6 = CONCAT22(param_5, uVar2);
        puVar6  = (undefined4 *)struct_op_1030_73a8(param_4);
        uVar5   = (ushort)((ulong)puVar6 >> 0x10);
        uVar4   = *(ushort *)((int)puVar6 + 0x20);
        if(uVar4 != param_3)
        {
            uVar3 = param_3;
            pass1_1010_a5ca(param_1, param_2, uVar4, param_3, uVar5);
            if((uVar4 != 0x70) && ((int)uVar3 < 0x0))
            {
                pass1_1030_25d8(CONCAT22(param_5, uVar2), uVar3 + 0x1, uVar4);
            }
            ppcVar1 = (code **)((int)*puVar6 + 0x8);
            uVar4   = param_3;
            (**ppcVar1)();
            if(param_3 != 0x70)
            {
                pass1_1010_a5ca(param_1, param_2, param_3, uVar4, extraout_DX);
                if((int)uVar4 < 0x0)
                {
                    pass1_1030_25d8(uStack6, uVar4 - 0x1, param_3);
                }
            }
        }
    }
    return;
}

ushort __stdcall16far pass1_1010_ac62(ushort param_1, ushort param_2, int param_3, ushort param_4, ushort param_5)

{
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748, (ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10), 0x8000001);
    return *(ushort *)(param_4 + 0x116 + param_3 * 0x2);
}

void __stdcall16far pass1_1010_acec(ulong param_1, int param_2, ushort param_3)

{
    if(param_2 == 0x1)
    {
        *(undefined4 *)((int)param_1 + 0x12e) = 0x0;
    }
    else
    {
        if(param_2 != 0x5)
        {
            return;
        }
    }
    pass1_1010_1f62(param_3, param_1 & 0xffff0000 | (ulong)((int)param_1 - 0xa), param_2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_ad22(ushort param_1, uint param_2, ushort param_3, ushort param_4, ushort param_5)

{
    undefined4 uVar1;
    ushort    *puVar2;

    uVar1  = *(undefined4 *)(param_1 + 0x138);
    puVar2 = &param_5;
    pass1_1030_627e(param_3, (uint)puVar2, param_2, _PTR_LOOP_1050_5740, (ushort *)CONCAT22(param_3, puVar2), *(long *)((int)uVar1 + 0x20));
    if((param_2 | (uint)puVar2) == 0x0)
    {
        return;
    }
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)puVar2, param_2);
    return;
}


void __stdcall16far pass1_1010_ad64(ushort param_1, ulong param_2, ulong param_3, ulong param_4, ushort param_5)

{
    if(param_3 != 0x0)
    {
        param_4 = *(ulong *)((int)param_3 + 0x2e);
        if(*(long *)((int)param_4 + 0x200) == 0x8000002)
        {
            return;
        }
    }
    pass1_1010_c58as(param_1, (ushort)param_2, (ushort)(param_2 >> 0x10), param_3, (ushort)param_4, param_5);
    return;
}

void __stdcall16far pass1_1010_af66(ulong param_1, uint param_2)

{
    undefined4 uVar1;
    ulong      uVar2;
    uint       uVar3;
    ushort     in_stack_00000008;

    uVar1 = *(undefined4 *)((int)param_1 + 0x138);
    uVar2 = *(ulong *)((int)uVar1 + 0x24);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar2, (uint)(uVar2 >> 0x10));
    uVar3 = param_2 | (uint)uVar2;
    if(uVar3 == 0x0)
    {
        return;
    }
    pass1_1038_5050(uVar2 & 0xffff | (ulong)param_2 << 0x10, in_stack_00000008, (uint)uVar2, uVar3);
    return;
}

void __stdcall16far pass1_1010_afa2(ulong param_1, uint param_2)

{
    undefined4 uVar1;
    ulong      uVar2;
    ushort     in_stack_00000008;

    uVar1 = *(undefined4 *)((int)param_1 + 0x138);
    uVar2 = *(ulong *)((int)uVar1 + 0x24);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar2, (uint)(uVar2 >> 0x10));
    if((param_2 | (uint)uVar2) == 0x0)
    {
        return;
    }
    pass1_1038_50e0(uVar2 & 0xffff | (ulong)param_2 << 0x10, in_stack_00000008, (uint)uVar2);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1010_afde(ulong param_1, int param_2)

{
    undefined4 uVar1;
    ulong      uVar2;
    uint       in_DX;
    ulong     *puVar3;

    uVar1 = *(undefined4 *)((int)param_1 + 0x138);
    uVar2 = *(ulong *)((int)uVar1 + 0x24);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (ushort)uVar2, (uint)(uVar2 >> 0x10));
    if((in_DX | (uint)uVar2) == 0x0)
    {
        return;
    }
    puVar3 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0, param_2);
    pass1_1038_4e78((uint)puVar3, (uchar *)((ulong)puVar3 >> 0x10), uVar2 & 0xffff | (ulong)in_DX << 0x10, puVar3);
    return;
}


ushort __stdcall16far pass1_1010_b028(ushort param_1, ushort param_2, ulong param_3)

{
    return *(ushort *)((int)param_3 + 0xc);
}

void __stdcall16far pass1_1010_bf1e(ulong param_1, int *param_2, int param_3, uchar *param_4, ushort param_5)

{
    ulong      uVar1;
    uint       uVar2;
    undefined *puVar3;
    int        iVar4;
    undefined2 uVar5;
    undefined4 uStack40;
    int        iStack36;
    uint       uStack32;
    ushort    *puStack26;
    undefined  local_16[0x12];
    int        iStack4;

    bad_1010_bf08((ushort)param_1, (ushort)(param_1 >> 0x10), 0x1000000);
    iStack4  = param_3 + -0x1;
    *param_2 = iStack4;
    uVar2    = iStack4 * 0x18;
    mem_op_1000_179c(uVar2, param_4, 0x1000);
    uStack40 = CONCAT22(param_4, uVar2);
    uStack32 = (uint)param_4 | uVar2;
    iVar4    = (int)param_2;
    uVar5    = (undefined2)((ulong)param_2 >> 0x10);
    if(uStack32 == 0x0)
    {
        *(undefined4 *)(iVar4 + 0x2) = 0x0;
    }
    else
    {
        pass1_1000_5586((uchar *)0x4092, 0x1020, iStack4, 0x18, uVar2, (ushort)param_4);
        *(undefined4 *)(iVar4 + 0x2) = uStack40;
    }
    pass1_1028_dc52((astruct_92 *)CONCAT22(param_5, local_16), 0x1, 0x0, 0x100);
    puStack26 = *(ushort **)(iVar4 + 0x2);
    iStack36  = 0x0;
    while(true)
    {
        puVar3 = local_16;
        pass1_1028_e4ec(CONCAT22(param_5, puVar3));
        if((uStack32 | (uint)puVar3) == 0x0)
            break;
        uVar1    = *(ulong *)(puVar3 + 0x10);
        uStack32 = uStack32 | (uint)puVar3;
        if(uVar1 != 0x0)
        {
            uStack32 = (uint)(uVar1 >> 0x10);
            pass1_1008_3f62(puStack26, (ushort *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0x4)));
            *(int *)((int)puStack26 + 0xc) = iStack36;
            iStack36                       = iStack36 + 0x1;
            puStack26                      = (ushort *)((ulong)puStack26 & 0xffff0000 | (ulong)((int)puStack26 + 0x18));
        }
    }
    return;
}

void __stdcall16far pass1_1010_8c78(ushort *param_1, ushort param_2)

{
    *param_1                            = 0x8ee2;
    *(undefined2 *)((int)param_1 + 0x2) = 0x1010;
    pass1_1010_1d80(param_1, param_2);
    return;
}


void __stdcall16far pass1_1010_8f78(ushort *param_1)

{
    undefined4  *puVar1;
    uint         uVar2;
    code       **ppcVar3;
    astruct_490 *iVar4;
    undefined2   uVar4;

    uVar4            = (undefined2)((ulong)param_1 >> 0x10);
    iVar4            = (astruct_490 *)param_1;
    *param_1         = 0x9254;
    iVar4->field_0x2 = 0x1010;
    puVar1           = iVar4->field_0x4;
    uVar2            = iVar4->field_0x6;
    if((uVar2 | (uint)puVar1) != 0x0)
    {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
    }
    *param_1         = 0x389a;
    iVar4->field_0x2 = 0x1008;
    return;
}

void __stdcall16far pass1_1010_8fba(ulong param_1, ushort param_2)

{
    code       **ppcVar1;
    ulong        uVar2;
    undefined2   extraout_DX;
    uint         extraout_DX_00;
    astruct_411 *iVar3;
    undefined2   uVar3;
    ulong        uStack14;
    ulong        uStack10;

    uVar3   = (undefined2)(param_1 >> 0x10);
    iVar3   = (astruct_411 *)param_1;
    ppcVar1 = (code **)((int)*iVar3->field_0x4 + 0x10);
    (**ppcVar1)();
    uStack10 = CONCAT22(extraout_DX, param_2);
    uStack14 = 0x0;
    while(true)
    {
        if(uStack10 <= uStack14)
        {
            return;
        }
        ppcVar1 = (code **)((int)*iVar3->field_0x4 + 0x4);
        uVar2   = uStack10;
        (**ppcVar1)();
        if((extraout_DX_00 | (uint)uVar2) != 0x0)
            break;
        uStack14 = uStack14 + 0x1;
    }
    ppcVar1 = (code **)((int)*iVar3->field_0x4 + 0x8);
    (**ppcVar1)();
    return;
}

void __stdcall16far pass1_1010_9130(ulong param_1, uchar *param_2, uint param_3, uint param_4, ushort param_5, uchar param_6)

{
    undefined4 uVar1;
    undefined2 uVar2;

    uVar2 = (undefined2)(param_1 >> 0x10);
    pass1_1030_1d58(*(ulong *)((int)param_1 + 0x4));
    if((uchar *)(param_4 | param_3) != (uchar *)0x0)
    {
        uVar1 = *(undefined4 *)((int)param_1 + 0x8);
        pass1_1010_c3c2((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), (ulong)param_2, CONCAT22(param_4, param_3), (uchar *)(param_4 | param_3), param_6, param_5);
        return;
    }
    *param_2 = '\0';
    return;
}

void __stdcall16far pass1_1010_91cc(ulong param_1)

{
    code     **ppcVar1;
    undefined2 uVar2;
    long       lVar3;

    uVar2   = (undefined2)(param_1 >> 0x10);
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0x10);
    lVar3   = (**ppcVar1)();
    if(lVar3 != 0x0)
    {
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0x8);
        (**ppcVar1)();
    }
    return;
}


void __stdcall16far pass1_1010_9210(ulong param_1)

{
    code **ppcVar1;

    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0xc);
    (**ppcVar1)();
    return;
}
