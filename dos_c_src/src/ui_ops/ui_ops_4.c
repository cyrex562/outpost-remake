
void __stdcall16far win_dlg_op_1038_bea4(ulong param_1, WORD *param_2)

{
    undefined4 uVar1;
    HWND16     HVar2;
    uchar     *in_DX;
    uchar     *puVar3;
    uint       uVar4;
    WPARAM16   wparam;
    int        iVar5;
    int        unaff_DI;
    undefined2 uVar6;
    ushort    *puVar7;
    ulong      uVar8;
    char      *pcVar9;
    LRESULT    LVar10;
    ulong     *local_116;
    ulong     *local_112;
    CHAR       local_10e[0x82];
    undefined  local_8c[0x82];
    undefined4 uStack10;
    ushort    *puStack6;

    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2, (ushort)param_2, in_DX, unaff_DI);
    puVar3   = (uchar *)((ulong)puStack6 >> 0x10);
    uStack10 = *(undefined4 *)((int)puStack6 + 0x68);
    uVar6    = (undefined2)(param_1 >> 0x10);
    iVar5    = (int)param_1;
    GetWindowText16(0x1010, 0x80, (INT16)local_8c);
    wsprintf16((LPSTR)s_tile2_bmp_1050_1538, local_10e, param_2);
    SetWindowText16((HWND16)s_tile2_bmp_1050_1538, (SEGPTR)local_10e);
    HVar2                     = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x179);
    *(HWND16 *)(iVar5 + 0x92) = HVar2;
    pass1_1008_e3ec(*(ulong *)(iVar5 + 0x8e),
                    (ulong *)CONCAT22(param_2, &local_116),
                    (ulong *)CONCAT22(param_2, &local_112),
                    (ushort)param_2);
    send_msg_1038_c374(param_1, local_112, *(uint *)(iVar5 + 0x92), 0x1008);
    puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0, 0x2f, (ushort)param_2, puVar3, unaff_DI);
    uVar4  = (uint)((ulong)puVar7 >> 0x10);
    uVar8  = *(ulong *)((int)puVar7 + 0x24);
    uVar1  = *(undefined4 *)(iVar5 + 0x8e);
    uVar8  = string_1008_e586((ushort)uVar1, (ushort)((ulong)uVar1 >> 0x10), uVar8, (uint)uVar8, uVar4);
    SendMessage16(0x1008, (UINT16)uVar8, (WPARAM16)(uVar8 >> 0x10), 0x40dffff);
    HVar2                     = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x17a);
    *(HWND16 *)(iVar5 + 0x94) = HVar2;
    send_msg_1038_c374(param_1, local_116, HVar2, (int)s_tile2_bmp_1050_1538);
    pcVar9 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc, (INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    wparam = (WPARAM16)((ulong)pcVar9 >> 0x10);
    LVar10 = SendMessage16(0x1010, (UINT16)pcVar9, wparam, 0x4030000);
    *(undefined2 *)(iVar5 + 0x9c) = (int)LVar10;
    SendMessage16((HWND16)s_tile2_bmp_1050_1538, (UINT16)pcVar9, wparam, 0x40dffff);
    HVar2                     = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x178);
    *(HWND16 *)(iVar5 + 0x96) = HVar2;
    HVar2                     = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x177);
    *(HWND16 *)(iVar5 + 0x98) = HVar2;
    HVar2                     = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538, 0x184);
    *(HWND16 *)(iVar5 + 0x9a) = HVar2;
    return;
}
