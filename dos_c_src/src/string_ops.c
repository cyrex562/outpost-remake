
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
    pcVar4 = string_op_1010_ada6(0x1010, param_3, (ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), param_2, *(int *)((int)uVar2 + 0xa));
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
    if((*(long *)(iVar5 + 0x160) == 0x0) || (lVar3 = *(long *)(iVar5 + 0x164), plVar1 = (long *)(iVar5 + 0x164), *plVar1 = *plVar1 + -0x1, lVar3 == 0x0))
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

ushort __cdecl16far pass1_1020_bd80(ushort param_1)

{
    char  *pcVar1;
    ushort uStack6;

    switch(param_1)
    {
    case 0x1:
    case 0x6:
        break;
    case 0x2:
        break;
    case 0x3:
    case 0x7:
        break;
    case 0x4:
    case 0x8:
        break;
    case 0x5:
    case 0x9:
        break;
    case 0xa:
        break;
    case 0xb:
    case 0x37:
        break;
    case 0xc:
    case 0x35:
    case 0x36:
        break;
    case 0xd:
        break;
    case 0xe:
        break;
    case 0xf:
        break;
    case 0x10:
        break;
    case 0x11:
        break;
    case 0x12:
        break;
    case 0x13:
    case 0x14:
    case 0x15:
        break;
    case 0x16:
    case 0x19:
        break;
    case 0x17:
    case 0x1a:
        break;
    case 0x18:
        break;
    case 0x1b:
    case 0x1c:
    case 0x1d:
        break;
    case 0x1e:
    case 0x1f:
    case 0x20:
        break;
    case 0x21:
        break;
    case 0x22:
    case 0x23:
    case 0x24:
        break;
    case 0x25:
    case 0x26:
    case 0x27:
        break;
    case 0x28:
    case 0x29:
        break;
    case 0x2a:
    case 0x2b:
        break;
    case 0x2c:
        break;
    case 0x2d:
    case 0x2e:
        break;
    case 0x2f:
    case 0x30:
        break;
    case 0x31:
    case 0x32:
        break;
    case 0x33:
    case 0x34:
        break;
    case 0x38:
    case 0x39:
        break;
    case 0x3a:
    case 0x3b:
        break;
    case 0x3c:
    case 0x3d:
        break;
    case 0x3e:
        break;
    case 0x3f:
        break;
    case 0x40:
        break;
    case 0x41:
        break;
    case 0x42:
    case 0x46:
    case 0x6b:
        break;
    case 0x43:
        uStack6 = (ushort)s_bidLRoadConst_1050_4e7a;
        return uStack6;
    case 0x44:
        uStack6 = (ushort)s_bidRRoadConst_1050_4e88;
        return uStack6;
    case 0x45:
        uStack6 = (ushort)s_bidXRoadConst_1050_4e96;
        return uStack6;
    case 0x47:
        break;
    case 0x48:
    case 0x49:
    case 0x4a:
    case 0x70:
    case 0x71:
    case 0x72:
        break;
    case 0x4b:
        break;
    case 0x4c:
        break;
    case 0x4d:
        break;
    case 0x4e:
        break;
    case 0x4f:
    case 0x50:
    case 0x51:
        break;
    case 0x52:
    case 0x53:
        break;
    case 0x54:
    case 0x55:
    case 0x56:
        break;
    case 0x57:
    case 0x58:
    case 0x59:
        break;
    case 0x5a:
        break;
    case 0x5b:
    case 0x5c:
        break;
    case 0x5d:
    case 0x5e:
    case 0x5f:
        break;
    case 0x60:
    case 0x61:
        break;
    case 0x62:
    case 0x63:
        break;
    case 0x64:
        break;
    case 0x65:
        break;
    case 0x66:
    case 0x67:
        break;
    case 0x68:
    case 0x69:
        break;
    case 0x6a:
        break;
    case 0x6c:
    case 0x6d:
        break;
    case 0x6e:
        break;
    case 0x6f:
        break;
    case 0x73:
    case 0x77:
        break;
    case 0x74:
    case 0x78:
    case 0x79:
        break;
    case 0x75:
        break;
    case 0x76:
        break;
    case 0x7a:
        break;
    case 0x7b:
        break;
    case 0x7c:
        break;
    case 0x7d:
        break;
    case 0x7e:
        break;
    case 0x7f:
        break;
    case 0x80:
        break;
    case 0x81:
        break;
    case 0x82:
        break;
    case 0x83:
        break;
    case 0x84:
        break;
    case 0x85:
        break;
    case 0x86:
        break;
    case 0x87:
        break;
    case 0x88:
        break;
    case 0x89:
    }
    pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    return (ushort)pcVar1;
}

void __cdecl16far string_1020_c0ca(ushort param_1)

{
    string_1020_c0d8(param_1);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char *__cdecl16far string_1020_c0d8(ushort param_1)

{
    char *pcVar1;

    switch(param_1)
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
        break;
    case 0x8:
        break;
    case 0x9:
        break;
    case 0xa:
        break;
    case 0xb:
        break;
    case 0xc:
        break;
    case 0xd:
        break;
    case 0xe:
        break;
    case 0xf:
        break;
    case 0x10:
        break;
    case 0x11:
        break;
    case 0x12:
        break;
    case 0x13:
        break;
    case 0x14:
        break;
    case 0x15:
        break;
    case 0x16:
        break;
    case 0x17:
        break;
    case 0x18:
        break;
    case 0x19:
        break;
    case 0x1a:
        break;
    case 0x1b:
        break;
    case 0x1c:
        break;
    case 0x1d:
        break;
    case 0x1e:
        break;
    case 0x1f:
        break;
    case 0x21:
        break;
    case 0x23:
        break;
    case 0x24:
    }
    pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    return (char *)pcVar1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char *__cdecl16far string_op_1020_c222(ushort param_1)

{
    char *pcVar1;

    switch(param_1)
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
        break;
    case 0x8:
        break;
    case 0x9:
        break;
    case 0xa:
        break;
    case 0xb:
        break;
    case 0xc:
        break;
    case 0xd:
        break;
    case 0xe:
        break;
    case 0xf:
        break;
    case 0x10:
        break;
    case 0x11:
        break;
    case 0x12:
        break;
    case 0x13:
        break;
    case 0x14:
    }
    pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    return (char *)pcVar1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char *__cdecl16far string_op_1020_c2f8(ushort param_1)

{
    char *pcVar1;

    switch(param_1)
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
        break;
    case 0x8:
        break;
    case 0x9:
        break;
    case 0xa:
        break;
    case 0xb:
        break;
    case 0xc:
        break;
    case 0xd:
        break;
    case 0xe:
        break;
    case 0xf:
        break;
    case 0x10:
    }
    pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    return (char *)pcVar1;
}

void __stdcall16far pass1_1020_6e52(ushort param_1, uint param_2, uint param_3, int param_4, ushort param_5, int param_6)

{
    uint  uVar1;
    char *pcVar2;

    pass1_1018_2e5e(param_1, param_2, param_3, *(ulong *)(param_4 + 0xf2));
    uVar1 = param_3 | param_2;
    if(uVar1 == 0x0)
    {
        pcVar2 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    }
    else
    {
        pass1_1018_2d84(param_2, *(ulong *)(param_4 + 0xf2));
        pcVar2 = (char *)CONCAT22(uVar1, param_2);
    }
    string_1020_79b4(param_1, CONCAT22(param_5, param_4), param_6, pcVar2);
    return;
}

void __stdcall16far sprintf_op_1018_34b6(ulong param_1, uchar param_2)

{
    int        iVar1;
    undefined3 in_register_00000001;
    undefined2 in_DX;
    int        iVar2;
    WORD      *valist;
    LPSTR      buffer;
    ushort     unaff_SS;
    ulong      uVar3;
    long       lVar4;

    valist = (WORD *)(param_1 >> 0x10);
    iVar2  = (int)param_1;
    uVar3  = switch_1018_3b9e(param_1, *(ushort *)(iVar2 + 0x12e), (ushort)CONCAT31(in_register_00000001, param_2), in_DX, unaff_SS);
    iVar1  = *(int *)(iVar2 + 0x12e);
    if(iVar1 == 0x188)
    {
        lVar4  = pass1_1008_57f0(uVar3, *(int *)(iVar2 + 0x130), unaff_SS);
        buffer = (LPSTR)0x1020;
        string_1020_c0d8(*(ushort *)((int)lVar4 + 0xe));
    }
    else
    {
        if(iVar1 == 0x18b)
        {
            buffer = (LPSTR)0x1008;
            pass1_1008_57f0(uVar3, *(int *)(iVar2 + 0x130), unaff_SS);
        }
        else
        {
            if(iVar1 != 0x18c)
            {
                load_string_1010_84e0(0x1010, (ushort)_PTR_LOOP_1050_14cc, (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x100, (char *)(iVar2 + 0x22), (short)valist);
                return;
            }
            buffer = (LPSTR)0x1008;
            pass1_1008_57f0(uVar3, *(int *)(iVar2 + 0x130), unaff_SS);
        }
    }
    wsprintf16(buffer, (LPCSTR)(iVar2 + 0x22), valist);
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_str_op_1018_35b0(ulong param_1, ushort param_2, undefined2 param_3)

{
    uint       *puVar1;
    int        *piVar2;
    ushort      uVar3;
    uint        uVar4;
    code      **ppcVar5;
    undefined2  uVar6;
    undefined4 *puVar7;
    undefined2  uVar8;
    uint        uVar9;
    uint        extraout_DX;
    ushort      uVar10;
    WORD       *valist;
    bool        bVar11;
    ulong       uVar12;
    undefined4  uVar13;
    int         local_12;
    int         local_10;
    long        lStack14;
    undefined2  uStack10;
    undefined2  uStack8;
    uint        uStack6;
    uint        uStack4;

    uVar12  = pass1_1030_8326();
    uStack4 = (uint)(uVar12 >> 0x10);
    uStack6 = (uint)uVar12;
    valist  = (WORD *)(param_1 >> 0x10);
    uVar10  = (ushort)param_1;
    puVar1  = (uint *)(uVar10 + 0x140);
    bVar11  = *puVar1 < uStack4;
    if((bVar11) || ((bVar11 || *puVar1 == uStack4 && (*(uint *)(uVar10 + 0x13e) < uStack6))))
    {
        uVar3 = *(ushort *)(uVar10 + 0x13c);
        if(*(int *)(uVar10 + 0x13a) < (int)uVar3)
        {
            uVar13   = switch_1018_3b9e(param_1, *(ushort *)(uVar10 + 0x12e), uVar3, uStack4, param_2);
            uVar8    = (undefined2)((ulong)uVar13 >> 0x10);
            uVar6    = (undefined2)uVar13;
            uStack10 = uVar6;
            uStack8  = uVar8;
            pass1_1018_427c(param_1);
            lStack14 = CONCAT22(uVar8, uVar6);
            pass1_1018_3e8c(uVar10, (ushort)valist, (ushort *)CONCAT22(param_2, &local_12), (ushort *)CONCAT22(param_2, &local_10));
            if(lStack14 < local_12)
            {
                local_12 = (int)lStack14;
            }
            uVar4  = *(uint *)(uVar10 + 0x138);
            puVar7 = (undefined4 *)*(undefined4 *)(uVar10 + 0x136);
            uVar9  = uVar4 | (uint)puVar7;
            if(uVar9 != 0x0)
            {
                ppcVar5 = (code **)*puVar7;
                (**ppcVar5)(0x30, puVar7, uVar4, 0x1);
                uVar9 = extraout_DX;
            }
            pass1_1018_435e(param_1, lStack14, local_12, local_10, uVar9, param_2);
            *(undefined2 *)(uVar10 + 0x136) = puVar7;
            *(uint *)(uVar10 + 0x138)       = uVar9;
            piVar2                          = (int *)(uVar10 + 0x13a);
            *piVar2                         = *piVar2 + 0x1;
            wsprintf16((LPSTR)0x1030, (LPCSTR)(uVar10 + 0x22), valist);
            return;
        }
        *(uint *)(uVar10 + 0x13e)       = uStack6;
        *(uint *)(uVar10 + 0x140)       = uStack4;
        *(undefined2 *)(uVar10 + 0x13a) = 0x0;
        pass1_1008_612e(0x8, 0xc, uStack6);
        *(uint *)(uVar10 + 0x13c) = uStack6;
    }
    return;
}

BOOL16 __stdcall16far string_1018_39d8(ushort param_1, ulong param_2, ulong param_3, ulong param_4)

{
    int   iVar1;
    char *pcVar2;
    long  lVar3;
    ulong uVar4;

    uVar4  = param_3;
    pcVar2 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    iVar1  = pass1_1000_3d7a((ulong)pcVar2, uVar4);
    if(iVar1 != 0x0)
    {
        iVar1 = pass1_1000_3d7a(param_4, param_3);
        if(iVar1 != 0x0)
        {
            lVar3 = pass1_1018_4608(param_1, param_2, param_3, param_4);
            if((lVar3 != 0x0) && (*(int *)((int)lVar3 + 0xc) == 0x1))
            {
                return 0x1;
            }
        }
    }
    return 0x0;
}

ulong __stdcall16far pass1_1018_3a7a(ulong param_1, ulong param_2, uint param_3, uint param_4)

{
    undefined4 uVar1;
    ulong      uVar2;

    uVar1 = *(undefined4 *)((int)param_1 + 0x122);
    uVar2 = string_1008_e586((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), param_2, param_3, param_4);
    return uVar2;
}

void __stdcall16far pass1_1010_dc36(ushort param_1, ushort param_2, uint param_3, ulong param_4, uint *param_5, ushort param_6)

{
    undefined4 *puVar1;
    ushort      uVar2;
    ulong       uVar3;
    int         iVar4;
    uint        uVar5;
    undefined4 *puVar6;
    undefined2  uVar7;
    uint        uStack90;
    undefined  *local_54;
    undefined4  local_52[0x14];

    local_54 = PTR_s_New_failed_in_Op_Op_1050_0020_1050_393f;
    puVar6   = local_52;
    for(iVar4 = 0x13; iVar4 != 0x0; iVar4 = iVar4 + -0x1)
    {
        puVar1  = puVar6;
        puVar6  = puVar6 + 0x1;
        *puVar1 = 0x0;
    }
    *(undefined2 *)puVar6                           = 0x0;
    *(undefined *)(undefined2 *)((int)puVar6 + 0x2) = 0x0;
    uStack90                                        = param_3;
    while(true)
    {
        uVar7 = (undefined2)((ulong)param_5 >> 0x10);
        if(*param_5 < uStack90 || *param_5 == uStack90)
            break;
        uVar3                        = *(ulong *)((int)param_5 + 0x2);
        uVar2                        = *(ushort *)((int)param_5 + 0x4);
        uVar5                        = (int)uVar3 + uStack90 * 0xa;
        *(undefined2 *)(uVar5 + 0x4) = *(undefined2 *)(uStack90 * 0x2 + (int)param_4);
        uStack90                     = uStack90 + 0x1;
        string_1040_a626((ushort *)(uVar3 & 0xffff0000 | (ulong)uVar5), (char *)CONCAT22(param_6, &local_54), uVar2);
    }
    return;
}

void __stdcall16far load_str_1010_ddf6(ulong param_1, ulong param_2)

{
    short in_buf_len_5;
    ulong uVar1;

    in_buf_len_5                         = (short)(param_1 >> 0x10);
    *(undefined *)((int)param_1 + 0x13c) = 0x0;
    uVar1                                = struct_op_1030_73a8(param_2);
    switch(*(undefined2 *)((int)uVar1 + 0x12))
    {
    case 0x1:
    case 0x2:
    case 0x4:
    case 0x7:
    case 0x9:
        break;
    case 0x3:
    case 0x5:
        break;
    case 0x6:
        break;
    case 0x8:
        break;
    default:
        goto switchD_1010_de53_caseD_9;
    }
    load_string_1010_84e0(0x1010, (ushort)_PTR_LOOP_1050_14cc, (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, (char *)((int)param_1 + 0x13c), in_buf_len_5);
switchD_1010_de53_caseD_9:
    return;
}
void __stdcall16far pass1_1010_de78(ulong param_1, ulong param_2)

{
    short in_buf_len_5;

    in_buf_len_5                         = (short)(param_1 >> 0x10);
    *(undefined *)((int)param_1 + 0x13c) = 0x0;
    pass1_1030_809c(param_2);
    load_string_1010_84e0(0x1030, (ushort)_PTR_LOOP_1050_14cc, (ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x3ff, (char *)((int)param_1 + 0x13c), in_buf_len_5);
    return;
}

char *__stdcall16far load_string_1010_ac92(HINSTANCE16 param_1, ushort param_2, ushort param_3, int param_4)

{
    char *pcVar1;

    if((0x0 < param_4) && (param_4 < 0x43))
    {
        pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), param_1);
        return pcVar1;
    }
    return (char *)0x0;
}

char *__stdcall16far string_op_1010_ada6(HINSTANCE16 param_1, ushort param_2, ushort param_3, ushort param_4, int param_5, int param_6)

{
    char *pcVar1;
    char *pcStack6;

    pcStack6 = (char *)0x0;
    if(param_6 == 0x6)
    {
        if(param_5 == 0x0)
            goto LAB_1010_adee;
        pcVar1 = string_op_1020_c222(param_5);
    }
    else
    {
        if(param_6 != 0x7)
        {
            return (char *)0x0;
        }
        if(param_5 == 0x0)
            goto LAB_1010_adee;
        pcVar1 = string_op_1020_c2f8(param_5);
    }
    param_1  = 0x1020;
    pcStack6 = (char *)CONCAT22(param_2, pcVar1);
LAB_1010_adee:
    if(pcStack6 == (char *)0x0)
    {
        pcStack6 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), param_1);
    }
    return pcStack6;
}

ushort __stdcall16far pass1_1010_ae12(ushort param_1, ushort param_2, ulong param_3, int param_4, ushort param_5)

{
    char  *pcVar1;
    int    iVar2;
    ushort uStack4;

    if(param_4 == 0x6)
    {
        for(uStack4 = 0x0; (int)uStack4 < 0x15; uStack4 = uStack4 + 0x1)
        {
            pcVar1 = string_op_1020_c222(uStack4);
            iVar2  = pass1_1000_3d7a(param_3, CONCAT22(param_5, pcVar1));
            if(iVar2 == 0x0)
            {
                return uStack4;
            }
        }
    }
    else
    {
        if(param_4 == 0x7)
        {
            for(uStack4 = 0x0; (int)uStack4 < 0x11; uStack4 = uStack4 + 0x1)
            {
                pcVar1 = string_op_1020_c2f8(uStack4);
                iVar2  = pass1_1000_3d7a(param_3, CONCAT22(param_5, pcVar1));
                if(iVar2 == 0x0)
                {
                    return uStack4;
                }
            }
        }
    }
    return 0xffff;
}

char *__stdcall16far load_string_1010_9432(HINSTANCE16 param_1)

{
    char *pcVar1;

    pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), param_1);
    return pcVar1;
}

char *__stdcall16far load_string_1010_847e(int param_1, INT16 in_buf_len_2, HINSTANCE16 in_hinstsance_3)

{
    LoadString16(in_hinstsance_3, 0x3ff, (LPSTR)(param_1 + 0x682), in_buf_len_2);
    return (char *)CONCAT22(in_buf_len_2, (LPSTR)(param_1 + 0x682));
}


void __stdcall16far load_string_1010_84ac(int param_1, INT16 param_2, HINSTANCE16 param_3)

{
    ushort uVar1;

    uVar1 = param_2;
    LoadString16(param_3, 0x3ff, (LPSTR)(param_1 + 0x682), param_2);
    str_op_1008_60e8((char *)CONCAT22(param_2, (LPSTR)(param_1 + 0x682)), uVar1);
    return;
}


void __stdcall16far load_string_1010_84e0(HINSTANCE16 in_hinstance_5, ushort param_2, ushort param_3, ushort in_resc_id_3, char *in_buffer_4, short in_buf_len_5)

{
    LoadString16(in_hinstance_5, in_resc_id_3, in_buffer_4, in_buf_len_5);
    return;
}


void __stdcall16far pass1_1010_84f8(ulong param_1, int param_2, ushort param_3)

{
    undefined4 uVar1;
    undefined2 uStack780;
    char       local_308[0x100];
    undefined  local_208[0x100];
    undefined  local_108[0x104];
    int        iStack4;

    if(*(int *)(param_2 * 0x10 + 0x10) != 0x3)
    {
        return;
    }
    uVar1   = *(undefined4 *)((int)param_1 + 0xe88);
    iStack4 = *(int *)((int)uVar1 + 0x70);
    str_1000_4d58(*(char **)(param_2 * 0x10 + 0x12), (char *)0x0, 0x0, CONCAT22(param_3, local_208), (WNDCLASS16 *)CONCAT22(param_3, local_308));
    unk_str_op_1000_3d3e((char *)CONCAT22(param_3, local_108), (char *)CONCAT22(param_3, local_208));
    if(local_308[0] == '\0')
    {
        if(iStack4 == 0x0)
        {
            uStack780 = 0x14c0;
        }
        else
        {
            uStack780 = 0x14ba;
        }
        _uStack780 = CONCAT22(0x1050, uStack780);
    }
    else
    {
        _uStack780 = CONCAT22(param_3, local_308);
    }
    pass1_1000_3cea(CONCAT22(param_3, local_108), _uStack780);
    set_err_mode_1010_8b14(param_1, CONCAT22(param_3, local_108), param_3);
    return;
}

void __stdcall16far pass1_1010_85be(ulong param_1, int param_2, int param_3, ushort param_4)

{
    ulong     uVar1;
    undefined local_30a[0x100];
    undefined local_20a[0x100];
    undefined local_10a[0x108];

    if(param_2 == 0x2)
    {
        uVar1 = *(ulong *)(param_3 * 0x4 + 0x2e34);
        str_1000_4d58((char *)(uVar1 & 0xffff0000 | (ulong)((int)uVar1 + 0x3)), (char *)0x0, 0x0, CONCAT22(param_4, local_20a), (WNDCLASS16 *)CONCAT22(param_4, local_30a));
        unk_str_op_1000_3d3e((char *)CONCAT22(param_4, local_10a), s_male_1050_14c6);
        pass1_1000_3cea(CONCAT22(param_4, local_10a), CONCAT22(param_4, local_20a));
        pass1_1000_3cea(CONCAT22(param_4, local_10a), CONCAT22(param_4, local_30a));
        set_err_mode_1010_8b14(param_1, CONCAT22(param_4, local_10a), param_4);
        return;
    }
    set_err_mode_1010_8b14(param_1, *(ULONG *)(param_3 * 0x4 + 0x2e34), param_4);
    return;
}

void __stdcall16far pass1_1010_6034(ulong param_1, ushort param_2)

{
    uint      *puVar1;
    int        iVar2;
    undefined2 uVar3;

    uVar3                         = (undefined2)(param_1 >> 0x10);
    iVar2                         = (int)param_1;
    *(undefined2 *)(iVar2 + 0x1e) = 0x1;
    *(undefined2 *)(iVar2 + 0x20) = 0x1;
    *(undefined2 *)(iVar2 + 0x72) = 0x1;
    *(undefined2 *)(iVar2 + 0x74) = 0x1;
    pass1_1010_60a0(param_1);
    puVar1 = pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (ulong)(iVar2 + 0x22)), (WNDCLASS16 *)0x0, 0x40);
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1000);
    *(uint **)(iVar2 + 0x68)  = puVar1;
    *(ushort *)(iVar2 + 0x6a) = param_2;
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1000);
    *(uint **)(iVar2 + 0x6c)  = puVar1;
    *(ushort *)(iVar2 + 0x6e) = param_2;
    return;
}

char *__stdcall16far load_string_1008_ee56(void)

{
    char *pcVar1;

    pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    return pcVar1;
}

ushort __stdcall16far pass1_1008_e2a4(ulong param_1,ulong param_2,ulong param_3)

{
  int iVar1;
  int iVar2;
  ushort unaff_SS;
  char *pcVar3;
  long lVar4;
  ulong uVar5;
  
  uVar5 = param_2;
  pcVar3 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  iVar1 = pass1_1000_3d7a((ulong)pcVar3,uVar5);
  if ((iVar1 == 0x0) || (iVar1 = pass1_1000_3d7a(param_3,param_2), iVar1 == 0x0)) {
    return 0x0;
  }
  lVar4 = pass1_1008_e8cc(unaff_SS,param_1,param_2,param_3);
  if (lVar4 != 0x0) {
    iVar1 = *(int *)((int)lVar4 + 0xc);
    iVar2 = iVar1 + -0x1;
    if (iVar2 == 0x0) {
      return 0x2;
    }
    if (iVar2 < 0x1) {
      return 0x0;
    }
    if (SBORROW2(iVar2,0x1)) {
      return 0x0;
    }
    if (0x1 < iVar1 + -0x2) {
      return 0x0;
    }
  }
  return 0x1;
}


void __stdcall16far pass1_1008_e320(astruct_102 *param_1,ulong param_2,ulong param_3,ushort param_4)

{
  astruct_103 *paVar1;
  astruct_103 *uVar2;
  uint uVar3;
  uint uVar4;
  astruct_102 *iVar5;
  astruct_102 *uVar6;
  char *pcVar5;
  long lVar6;
  ulong uVar7;
  
  uVar6 = (astruct_102 *)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_102 *)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar5->field_0x1e,0x1000);
  *(undefined4 *)&iVar5->field_0x1e = 0x0;
  uVar7 = param_2;
  pcVar5 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  uVar4 = (uint)((ulong)pcVar5 >> 0x10);
  uVar2 = (astruct_103 *)pass1_1000_3d7a((ulong)pcVar5,uVar7);
  if ((uVar2 != (astruct_103 *)0x0) &&
     (uVar2 = (astruct_103 *)pass1_1000_3d7a(param_3,param_2), uVar2 != (astruct_103 *)0x0)) {
    lVar6 = pass1_1008_e8cc(param_4,(ulong)param_1,param_2,param_3);
    uVar3 = (uint)((ulong)lVar6 >> 0x10);
    uVar2 = (astruct_103 *)lVar6;
    uVar4 = uVar3 | (uint)uVar2;
    if ((uVar4 != 0x0) &&
       (((paVar1 = (astruct_103 *)uVar2->field_0xc, uVar2 = paVar1, paVar1 != (astruct_103 *)0x0 &&
         (uVar2 = (astruct_103 *)((int)&paVar1[-0x1].field_0xc + 0x1), uVar2 != (astruct_103 *)0x0)) &&
        (uVar2 = (astruct_103 *)&paVar1[-0x1].field_0xc, uVar2 != (astruct_103 *)0x0)))) {
      uVar2 = (astruct_103 *)&paVar1[-0x1].field_0xb;
    }
  }
  load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  iVar5->field_0x1e = (uint)uVar2;
  iVar5->field_0x20 = uVar4;
  return;
}

void __stdcall16far load_str_and_sprintf_1008_b69c(astruct_25 *param_1,WORD *param_2,uchar *param_3)

{
  code **ppcVar1;
  char *in_buffer_4;
  ushort uVar2;
  ushort uVar3;
  ushort uVar4;
  astruct_25 *iVar5;
  undefined2 uVar5;
  astruct_26 *paVar6;
  undefined4 uVar7;
  int iStack516;
  char local_202 [0x100];
  CHAR local_102 [0x100];
  
  in_buffer_4 = local_202;
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,in_buffer_4,
             (short)param_2);
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_25 *)param_1;
  if (iVar5->field_0xa == (undefined4 *)0x0) {
    mem_op_1000_179c(0xc,param_3,0x1000);
    if (((uint)param_3 | (uint)in_buffer_4) == 0x0) {
      paVar6 = (astruct_26 *)0x0;
    }
    else {
      paVar6 = (astruct_26 *)set_struct_1008_574a((astruct_21 *)CONCAT22(param_3,in_buffer_4));
    }
    *(int *)&iVar5->field_0xa = (int)paVar6;
    *(undefined2 *)((int)&iVar5->field_0xa + 0x2) = (int)((ulong)paVar6 >> 0x10);
    for (iStack516 = 0x1; iStack516 < 0x6; iStack516 = iStack516 + 0x1) {
      mem_op_1000_179c(0x12,(uchar *)((ulong)paVar6 >> 0x10),0x1000);
      if (paVar6 == (astruct_26 *)0x0) {
        uVar7 = 0x0;
      }
      else {
        uVar7 = set_stuct_1008_b0bc(paVar6);
      }
      uVar3 = (ushort)((ulong)uVar7 >> 0x10);
      uVar4 = uVar3;
      wsprintf16((LPSTR)&PTR_LOOP_1050_1000,local_102,param_2);
      uVar2 = str_op_1008_60e8((char *)CONCAT22(param_2,local_102),uVar4);
      *(ushort *)((int)uVar7 + 0x4) = uVar2;
      *(ushort *)((int)uVar7 + 0x6) = uVar4;
      ppcVar1 = (code **)((int)*iVar5->field_0xa + 0x8);
      paVar6 = (astruct_26 *)(**ppcVar1)();
    }
    iVar5->field_0x22 = 0x5;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far load_str_and_sprintf_1008_b78a(ULONG param_1,WORD *param_2,uchar *param_3,uint param_4)

{
  int *piVar1;
  code **ppcVar2;
  ushort uVar3;
  int iVar4;
  undefined2 uVar5;
  undefined4 uVar6;
  char local_206 [0x100];
  CHAR local_106 [0x100];
  int iStack6;
  undefined2 uStack4;
  
  mem_op_1000_179c(0x12,param_3,0x1000);
  if (((uint)param_3 | param_4) == 0x0) {
    uVar6 = 0x0;
  }
  else {
    uVar6 = set_stuct_1008_b0bc((astruct_26 *)CONCAT22(param_3,param_4));
  }
  uStack4 = (undefined2)((ulong)uVar6 >> 0x10);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,local_206,
             (short)param_2);
  iStack6 = (int)uVar6;
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  piVar1 = (int *)(iVar4 + 0x22);
  *piVar1 = *piVar1 + 0x1;
  wsprintf16((LPSTR)0x1010,local_106,param_2);
  iStack6 = (int)uVar6;
  uVar3 = str_op_1008_60e8((char *)CONCAT22(param_2,local_106),(ushort)((ulong)uVar6 >> 0x10));
  iStack6 = (int)uVar6;
  *(ushort *)(iStack6 + 0x4) = uVar3;
  *(undefined2 *)(iStack6 + 0x6) = (int)((ulong)uVar6 >> 0x10);
  ppcVar2 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar4 + 0xa) + 0x8);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,*(undefined4 *)(iVar4 + 0xa),iStack6,uStack4);
  return;
}


char * __stdcall16far load_string_1008_b1f0(void)

{
  char *pcVar1;
  
  pcVar1 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  return pcVar1;
}


void __stdcall16far pass1_1008_9c86(ulong param_1,char *param_2,int param_3)

{
  ushort uVar1;
  
  uVar1 = str_op_1000_3da4((char *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0xa)));
  if (param_3 < (int)uVar1) {
    uVar1 = param_3 - 0x1;
  }
  str_op_1000_3dbe(param_2,(char *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0xa)),uVar1);
  return;
}

ulong * __stdcall16far str_1008_6d8a(ulong *param_1,char *param_2,ushort param_3,ushort param_4,uchar param_5)

{
  ushort uVar1;
  ushort uVar2;
  
  uVar2 = (ushort)((ulong)param_1 >> 0x10);
  *param_1 = 0x0;
  *(undefined2 *)((int)param_1 + 0x4) = 0xffff;
  PTR_LOOP_1050_0312 = (undefined *)&DAT_1050_0004;
  sys_1000_3f9c((uchar *)0x65a0,(uchar *)&USHORT_1050_1050,(ushort)_PTR_s_SC_03d_1050_0314_1050_031c,
                (ushort)((ulong)_PTR_s_SC_03d_1050_0314_1050_031c >> 0x10),0x4,&stack0xfffe,uVar2,0x1000,param_4,param_5
               );
  uVar1 = str_op_1008_60e8(param_2,param_3);
  *(ushort *)param_1 = uVar1;
  *(ushort *)((int)param_1 + 0x2) = param_3;
  return param_1;
}


void __stdcall16far struct_op_1008_48fe(astruct_81 *param_1,ushort param_2,char *param_3,ushort param_4)

{
  ushort uVar1;
  astruct_81 *iVar2;
  ushort uVar3;
  
  uVar3 = (ushort)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_81 *)param_1;
  *(undefined2 *)param_1 = 0x389a;
  iVar2->field_0x2 = 0x1008;
  iVar2->field_0x4 = 0x0;
  *(undefined4 *)&iVar2->field_0x8 = 0x0;
  iVar2->field_0xc = 0xffff;
  iVar2->field_0xe = 0x0;
  iVar2->field_0x12 = 0x0;
  iVar2->field_0x16 = 0x0;
  iVar2->field_0x1a = 0x0;
  iVar2->field_0x1e = 0x0;
  iVar2->field_0x22 = param_2;
  *(int *)param_1 = (int)&PTR_LOOP_1050_4c4c;
  iVar2->field_0x2 = 0x1008;
  uVar1 = str_op_1008_60e8(param_3,param_4);
  iVar2->field_0x8 = uVar1;
  iVar2->field_0xa = param_4;
  return;
}


void __stdcall16far
pass1_1008_48de(ushort param_1,ulong param_2,int param_3,uint param_4,ushort *param_5,int param_6,int param_7,
               byte *param_8,ushort param_9,ushort param_10,char param_11,ushort param_12,uchar param_13)

{
  byte *pbVar1;
  undefined4 uVar2;
  byte bVar3;
  ushort uVar4;
  byte bVar5;
  ushort uVar6;
  undefined *puVar7;
  int iVar8;
  undefined2 uVar9;
  
  uVar6 = param_4 & 0xff | (uint)(byte)((char)(param_4 >> 0x8) + (char)param_4 + param_11) << 0x8;
  puVar7 = (undefined *)(param_6 + 0x1);
  pbVar1 = (byte *)((int)param_5 + param_7);
  bVar5 = (byte)(param_4 & 0xff);
  *pbVar1 = *pbVar1 | bVar5;
  bVar3 = in(0x46);
  pbVar1 = (byte *)((int)param_5 + param_7);
  *pbVar1 = *pbVar1 | bVar5;
  if (param_3 == 0x1) {
    pbVar1 = (byte *)((int)param_5 + param_7);
    *pbVar1 = *pbVar1 | bVar5;
    iVar8 = param_7 + 0x1;
    pbVar1 = (byte *)((int)param_5 + iVar8);
    bVar5 = (byte)param_12;
    *pbVar1 = *pbVar1 | bVar5;
    pbVar1 = (byte *)((int)param_5 + iVar8);
    *pbVar1 = *pbVar1 | bVar5;
    *param_8 = bVar3;
    pbVar1 = (byte *)((int)param_5 + iVar8);
    *pbVar1 = *pbVar1 | bVar5;
    uVar6 = param_12;
    if (*pbVar1 != 0x0) {
      pbVar1 = (byte *)((int)param_5 + iVar8);
      *pbVar1 = *pbVar1 | bVar5;
      puVar7 = (undefined *)((int)&param_12 + 0x1);
      param_5 = (ushort *)(param_2 >> 0x8);
      *(undefined2 *)CONCAT13(param_13,param_2._1_3_) = 0x389a;
      param_5[0x1] = 0x1008;
      param_9 = (ushort)(CONCAT13(param_13,param_2._1_3_) >> 0x10);
      *(undefined4 *)(param_5 + 0x2) = 0x0;
      *(undefined4 *)(param_5 + 0x4) = 0x0;
      param_5[0x6] = 0xffff;
      *(undefined4 *)(param_5 + 0x7) = 0x0;
      *(undefined4 *)(param_5 + 0x9) = 0x0;
      *(undefined4 *)(param_5 + 0xb) = 0x0;
      *(undefined4 *)(param_5 + 0xd) = 0x0;
      param_5[0xf] = 0x0;
    }
  }
  else {
    param_5[0x11] = bVar3 | 0x800;
  }
  param_5[0x11] = *(ushort *)(puVar7 + 0xa);
  *param_5 = (ushort)&PTR_LOOP_1050_4c4c;
  param_5[0x1] = 0x1008;
  uVar4 = str_op_1008_60e8(*(char **)(puVar7 + 0xc),uVar6);
  uVar2 = *(undefined4 *)(puVar7 + 0x6);
  uVar9 = (undefined2)((ulong)uVar2 >> 0x10);
  iVar8 = (int)uVar2;
  *(ushort *)(iVar8 + 0x8) = uVar4;
  *(ushort *)(iVar8 + 0xa) = uVar6;
  return;
}

void __stdcall16far pass1_1008_049c(ushort param_1,ushort param_2,char *param_3)

{
  uint uVar1;
  undefined *puVar2;
  
  if (param_3 != (char *)0x0) {
    uVar1 = str_op_1000_3da4(param_3);
    if (uVar1 != 0x0) {
      puVar2 = (undefined *)pass1_1000_545a((ulong)param_3 & 0xffff0000 | (ulong)((int)param_3 + 0x1),0x105000cc);
      if (puVar2 == (undefined *)0x0) {
        PTR_LOOP_1050_02ec = puVar2;
      }
    }
  }
  return;
}


void __cdecl16far str_1000_4d58(char *in_string_1,char *in_string_2,ulong param_3,ulong param_4,WNDCLASS16 *param_5)

{
  ushort uVar1;
  int iVar2;
  uint uVar3;
  undefined2 uVar4;
  uint uVar5;
  char *pcStack18;
  uint uStack12;
  uint uStack10;
  uint uStack8;
  uint uStack6;
  
  uStack10 = 0x0;
  uStack12 = 0x0;
  uVar4 = (undefined2)((ulong)in_string_1 >> 0x10);
  iVar2 = (int)in_string_1;
  if ((*in_string_1 == '\0') || (*(char *)(iVar2 + 0x1) != ':')) {
    if (in_string_2 != (char *)0x0) {
      *in_string_2 = '\0';
    }
  }
  else {
    if (in_string_2 != (char *)0x0) {
      *in_string_2 = *in_string_1;
      *(undefined *)((int)in_string_2 + 0x1) = *(undefined *)(iVar2 + 0x1);
      *(undefined *)((int)in_string_2 + 0x2) = 0x0;
    }
    in_string_1 = (char *)((ulong)in_string_1 & 0xffff0000 | (ulong)(iVar2 + 0x2));
  }
  uStack6 = 0x0;
  uStack8 = 0x0;
  pcStack18 = in_string_1;
  while( true ) {
    uVar5 = (uint)((ulong)pcStack18 >> 0x10);
    uVar3 = (uint)pcStack18;
    if (*pcStack18 == '\0') break;
    if ((*pcStack18 == '/') || (*pcStack18 == '\\')) {
      uStack8 = uVar3 + 0x1;
      uStack6 = uVar5;
    }
    else {
      if (*pcStack18 == '.') {
        uStack12 = uVar3;
        uStack10 = uVar5;
      }
    }
    pcStack18 = (char *)((ulong)pcStack18 & 0xffff0000 | (ulong)(uVar3 + 0x1));
  }
  if ((uStack6 | uStack8) == 0x0) {
    if (param_3 != 0x0) {
      *(undefined *)param_3 = 0x0;
    }
  }
  else {
    if (param_3 != 0x0) {
      uVar1 = uStack8 - (uint)in_string_1;
      if (0xff < (int)uVar1) {
        uVar1 = 0xff;
      }
      str_op_1000_3dbe((char *)(param_3 & 0xffff | (ulong)param_3._2_2_ << 0x10),in_string_1,uVar1);
      *(undefined *)((int)param_3 + uVar1) = 0x0;
    }
    in_string_1 = (char *)CONCAT22(uStack6,uStack8);
  }
  if (((uStack10 | uStack12) != 0x0) && ((uint)in_string_1 <= uStack12)) {
    if (param_4 != 0x0) {
      uVar1 = uStack12 - (uint)in_string_1;
      if (0xff < (int)uVar1) {
        uVar1 = 0xff;
      }
      str_op_1000_3dbe((char *)(param_4 & 0xffff | (ulong)param_4._2_2_ << 0x10),
                       (char *)((ulong)in_string_1 & 0xffff | (ulong)in_string_1._2_2_ << 0x10),uVar1);
      *(undefined *)((int)param_4 + uVar1) = 0x0;
    }
    if (param_5 == (WNDCLASS16 *)0x0) {
      return;
    }
    uVar1 = uVar3 - uStack12;
    if (0xff < (int)uVar1) {
      uVar1 = 0xff;
    }
    str_op_1000_3dbe((char *)((ulong)param_5 & 0xffff | (ulong)param_5._2_2_ << 0x10),
                     (char *)CONCAT22(uStack10,uStack12),uVar1);
    *(undefined *)((int)param_5 + uVar1) = 0x0;
    return;
  }
  if (param_4 != 0x0) {
    uVar1 = uVar3 - (uint)in_string_1;
    if (0xff < (int)uVar1) {
      uVar1 = 0xff;
    }
    str_op_1000_3dbe((char *)(param_4 & 0xffff | (ulong)param_4._2_2_ << 0x10),
                     (char *)((ulong)in_string_1 & 0xffff | (ulong)in_string_1._2_2_ << 0x10),uVar1);
    *(undefined *)((int)param_4 + uVar1) = 0x0;
  }
  if (param_5 != (WNDCLASS16 *)0x0) {
    *(undefined *)&param_5->style = 0x0;
  }
  return;
}

uint __cdecl16far str_op_1000_3da4(char *param_1)

{
  char *pcVar1;
  uint uVar2;
  char *pcVar3;
  bool bVar4;
  
  pcVar3 = (char *)param_1;
  bVar4 = true;
  uVar2 = 0xffff;
  do {
    if (uVar2 == 0x0) break;
    uVar2 = uVar2 - 0x1;
    pcVar1 = pcVar3;
    pcVar3 = pcVar3 + 0x1;
    bVar4 = *pcVar1 == '\0';
  } while (!bVar4);
  uVar2 = ~uVar2;
  if (bVar4) {
    uVar2 = uVar2 - 0x1;
  }
  return uVar2;
}



uchar __cdecl16far str_op_1000_3dbe(char *param_1,char *param_2,ushort param_3)

{
  char *pcVar1;
  char cVar2;
  char *pcVar3;
  char *pcVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  pcVar4 = (char *)param_1;
  pcVar3 = (char *)param_2;
  if (param_3 != 0x0) {
    do {
      pcVar1 = pcVar3;
      pcVar3 = pcVar3 + 0x1;
      cVar2 = *pcVar1;
      if (cVar2 == '\0') break;
      pcVar1 = pcVar4;
      pcVar4 = pcVar4 + 0x1;
      *pcVar1 = cVar2;
      param_3 = param_3 - 0x1;
    } while (param_3 != 0x0);
    for (; param_3 != 0x0; param_3 = param_3 - 0x1) {
      pcVar1 = pcVar4;
      pcVar4 = pcVar4 + 0x1;
      *pcVar1 = '\0';
    }
  }
  return (uchar)param_1;
}


int __cdecl16far pass1_1000_3ec0(uint param_1,uint param_2)

{
  uint uVar1;
  uint uVar2;
  uint16_t uVar3;
  uint16_t unaff_SI;
  uint16_t uVar4;
  undefined4 *puVar4;
  
  puVar4 = (undefined4 *)CONCAT22(PTR_LOOP_1050_5fc0,PTR_LOOP_1050_5fbe);
  if ((((uint)PTR_LOOP_1050_5fc0 | (uint)PTR_LOOP_1050_5fbe) != 0x0) && ((param_2 | param_1) != 0x0)) {
    uVar1 = str_op_1000_3da4((char *)CONCAT22(param_2,param_1));
    while( true ) {
      uVar4 = (uint16_t)((ulong)puVar4 >> 0x10);
      uVar3 = (uint16_t)puVar4;
      if ((*(uint *)(uVar3 + 0x2) | *(uint *)puVar4) == 0x0) break;
      uVar2 = str_op_1000_3da4((char *)CONCAT22(*(undefined2 *)(uVar3 + 0x2),*(uint *)puVar4));
      if (((uVar1 < uVar2) && (*(char *)((int)*puVar4 + uVar1) == '=')) &&
         (uVar2 = pass1_1000_3de8((char *)CONCAT22(*(undefined2 *)(uVar3 + 0x2),*(uint *)puVar4),
                                  (char *)CONCAT22(param_2,param_1),uVar1,unaff_SI,uVar3), uVar2 == 0x0)) {
        return *(uint *)puVar4 + uVar1 + 0x1;
      }
      puVar4 = (undefined4 *)((ulong)puVar4 & 0xffff0000 | (ulong)(uVar3 + 0x4));
    }
  }
  return 0x0;
}


PCHAR __stdcall16far poss_str_op_1000_28dc(int param_1)

{
  int *piVar1;
  PCHAR piVar2;
  int iVar2;
  PCHAR piVar3;
  
  piVar3 = (PCHAR)&PTR_LOOP_1050_63fe;
  do {
    piVar1 = (int *)piVar3;
    piVar3 = (PCHAR)((int)piVar3 + 0x2);
    iVar2 = *piVar1;
    piVar2 = piVar3;
    if ((iVar2 == param_1) || (piVar2 = (PCHAR)(iVar2 + 0x1), piVar2 == (PCHAR)0x0)) {
      return (PCHAR)(int *)piVar2;
    }
    iVar2 = -0x1;
    do {
      if (iVar2 == 0x0) break;
      iVar2 = iVar2 + -0x1;
      piVar1 = (int *)piVar3;
      piVar3 = (PCHAR)((int)piVar3 + 0x1);
    } while (*(char *)piVar1 != '\0');
  } while( true );
}



void __stdcall16far pass1_1000_2913(int param_1,uint16_t param_2,uint16_t param_3)

{
  char *pcVar1;
  char *pcVar2;
  int iVar3;
  
  if (PTR_LOOP_1050_61ec != (undefined *)0x0) {
    pcVar2 = poss_str_op_1000_28dc(param_1);
    if (pcVar2 != (PCHAR)0x0) {
      iVar3 = -0x1;
      do {
        if (iVar3 == 0x0) break;
        iVar3 = iVar3 + -0x1;
        pcVar1 = pcVar2;
        pcVar2 = pcVar2 + 0x1;
      } while (*pcVar1 != '\0');
      pass1_1000_55b1(0x2944,param_2,param_3);
    }
  }
  return;
}

