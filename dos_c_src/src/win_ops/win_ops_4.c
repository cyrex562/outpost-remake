
void __stdcall16far def_win_proc_1008_5632(ulong *param_1,WPARAM16 param_2,ushort param_3,int param_4,ushort param_5)

{
  code **ppcVar1;
  HWND16 unaff_CS;
  undefined2 unaff_SS;
  undefined2 uVar2;
  undefined4 *puStack6;
  
  uVar2 = SUB42(&USHORT_1050_1050,0x0);
  puStack6 = (undefined4 *)GetWindowLong16(unaff_CS,0x0);
  if (((uint)((ulong)puStack6 >> 0x10) | (uint)puStack6) == 0x0) {
    if (param_4 != 0x1) {
      DefWindowProc16((HWND16)s_tile2_bmp_1050_1538,(UINT16)param_1,param_2,CONCAT22(param_4,param_3));
      return;
    }
    puStack6 = (undefined4 *)*param_1;
    SetWindowLong16((HWND16)s_tile2_bmp_1050_1538,(INT16)puStack6,(ulong)puStack6 >> 0x10);
    pass1_1008_9628(puStack6,param_5);
  }
  ppcVar1 = (code **)((int)*puStack6 + 0x1c);
  (**ppcVar1)((int)s_tile2_bmp_1050_1538,(int)puStack6,(int)((ulong)puStack6 >> 0x10),param_1,param_2,param_3,param_4,
              uVar2);
  return;
}


void __stdcall16far
pass1_1008_3bd6(astruct_160 *param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5,ulong param_6,
               ulong param_7,ushort param_8,ushort param_9)

{
  mixed_struct_op_1040_8fb8
            ((ushort *)CONCAT22(param_2,param_1),param_3,(char *)0x0,param_5,(ushort)param_6,(ushort)(param_6 >> 0x10),
             (ushort)param_7,(ushort)(param_7 >> 0x10),param_8,(int)&PTR_LOOP_1050_1040,param_9);
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x3cfc;
  param_1->field_0x2 = 0x1008;
  param_1->field_0x36 = 0x0;
  param_1->field_0x26 = 0x0;
  pass1_1040_9252(CONCAT22(param_2,param_1),(ushort)&PTR_LOOP_1050_1040);
  create_window_1040_92dc(CONCAT22(param_2,param_1),(UINT16)&PTR_LOOP_1050_1040);
  mov_update_win_1040_93aa
            ((astruct_65 *)CONCAT22(param_2,param_1),(INT16)param_4,(ushort)(param_4 >> 0x10),(int)&PTR_LOOP_1050_1040);
  return;
}


void __stdcall16far post_msg_1008_3d20(ulong param_1,HWND16 param_2)

{
  PostMessage16(param_2,0x0,0x0,CONCAT22(0x111,*(undefined2 *)((int)param_1 + 0xcc)));
  return;
}



void __stdcall16far post_quit_msg_1008_3af4(short exit_code)

{
  PostQuitMessage16(exit_code);
  return;
}


undefined2 __stdcall16far unk_win_msg_op_1008_0a3c(ulong param_1,uint param_2,HWND16 param_3)

{
  BOOL16 BVar1;
  
  if ((param_2 & 0xfff0) == 0xf140) {
    return *(undefined2 *)((int)param_1 + 0xde);
  }
  if ((param_2 & 0xfff0) == 0xf060) {
    BVar1 = IsIconic16(param_3);
    if (BVar1 == 0x0) {
      PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1110067);
    }
    return 0x0;
  }
  return 0x1;
}


void __stdcall16far pass1_1008_0a92(ulong param_1,short param_2)

{
  code **ppcVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(long *)(iVar2 + 0xee) != 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar2 + 0xee) + 0x90);
    (**ppcVar1)(param_2,*(undefined4 *)(iVar2 + 0xee));
  }
  if (*(long *)(iVar2 + 0xe8) != 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar2 + 0xe8) + 0x90);
    (**ppcVar1)(param_2,*(undefined4 *)(iVar2 + 0xe8));
  }
  if (_PTR_LOOP_1050_0388 != (undefined4 *)0x0) {
    ppcVar1 = (code **)*_PTR_LOOP_1050_0388;
    (**ppcVar1)(param_2,(int)_PTR_LOOP_1050_0388,(int)((ulong)_PTR_LOOP_1050_0388 >> 0x10),0x1);
  }
  post_quit_msg_1008_3af4(param_2);
  return;
}



void __stdcall16far window_op_1008_0af8(astruct *param_1,uchar *param_2,undefined2 param_3)

{
  int *piVar1;
  undefined2 uVar2;
  code **ppcVar3;
  undefined *puVar4;
  undefined4 uVar5;
  uchar *puVar6;
  undefined2 extraout_DX;
  uchar *extraout_DX_00;
  uint uVar7;
  uchar *extraout_DX_01;
  int iVar8;
  uint uVar9;
  undefined2 uVar10;
  undefined2 unaff_SS;
  ushort *puVar11;
  undefined2 uVar12;
  undefined2 uVar13;
  undefined uVar14;
  astruct_20 *paStack6;
  
  create_window_ex_1008_9760(param_1,param_3);
  uVar9 = (uint)((ulong)param_1 >> 0x10);
  iVar8 = (int)param_1;
  puVar4 = (undefined *)*(undefined2 *)(iVar8 + 0x8);
  PTR_LOOP_1050_0396 = puVar4;
  mem_op_1000_179c(0x12,param_2,0x1000);
  puVar6 = (uchar *)((uint)param_2 | (uint)puVar4);
  if (puVar6 != (uchar *)0x0) {
    puVar11 = pass1_1008_91ba((ushort *)CONCAT22(param_2,puVar4),0x1000);
    puVar6 = (uchar *)((ulong)puVar11 >> 0x10);
    puVar4 = (undefined *)puVar11;
  }
  mem_op_1000_179c(0x6,puVar6,0x1000);
  if (((uint)puVar6 | (uint)puVar4) == 0x0) {
    *(undefined4 *)(iVar8 + 0xe0) = 0x0;
  }
  else {
    pass1_1008_392e((ushort *)CONCAT22(puVar6,puVar4),*(ushort *)(iVar8 + 0x8));
    *(undefined2 *)(iVar8 + 0xe0) = puVar4;
    *(undefined2 *)(iVar8 + 0xe2) = extraout_DX;
  }
  ppcVar3 = (code **)((int)*(undefined4 *)param_1 + 0x14);
  (**ppcVar3)(0x1000,param_1,0x0,0x15a,(int)&USHORT_1050_1050);
  uVar10 = 0x1000;
  puVar6 = extraout_DX_00;
  mem_op_1000_179c(0xec,extraout_DX_00,0x1000);
  paStack6 = (astruct_20 *)CONCAT22(puVar6,puVar4);
  uVar7 = (uint)puVar6 | (uint)puVar4;
  if (uVar7 == 0x0) {
    *(undefined4 *)(iVar8 + 0xe4) = 0x0;
  }
  else {
    piVar1 = (int *)(iVar8 + 0xcc);
    *piVar1 = *piVar1 + 0x1;
    uVar10 = 0x1020;
    pass1_1020_08b6(unaff_SS,paStack6,*(UINT16 *)(iVar8 + 0xcc),(ULONG)param_1);
    *(undefined2 *)(iVar8 + 0xe4) = puVar4;
    *(uint *)(iVar8 + 0xe6) = uVar7;
  }
  if (*(long *)(iVar8 + 0xce) != 0x0) {
    ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0xce) + 0x10);
    (**ppcVar3)();
  }
  *(undefined4 *)(iVar8 + 0xce) = *(undefined4 *)(iVar8 + 0xe4);
  uVar14 = 0x1;
  uVar5 = *(undefined4 *)(iVar8 + 0xe4);
  uVar12 = (undefined2)uVar5;
  uVar13 = (undefined2)((ulong)uVar5 >> 0x10);
  ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0xe4) + 0x10);
  (**ppcVar3)();
  uVar5 = *(undefined4 *)(iVar8 + 0xe4);
  uVar2 = *(undefined2 *)(iVar8 + 0xe6);
  *(undefined4 *)(iVar8 + 0xe8) = uVar5;
  ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0xe8) + 0x8);
  (**ppcVar3)(uVar10,*(undefined2 *)(iVar8 + 0xe8),uVar2,uVar12,uVar13,uVar14);
  uVar7 = (uint)uVar5;
  ppcVar3 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar8 + 0xe8) + 0xc);
  (**ppcVar3)();
  pass1_1008_6978((ulong)param_1 & 0xffff | (ulong)uVar9 << 0x10,0x0,*(ulong *)(iVar8 + 0xe8),uVar7,extraout_DX_01);
  return;
}

BOOL16 __stdcall16far
mixed_win_op_1008_0c60(astruct_72 **param_1,ushort param_2,BOOL16 param_3,HWND16 param_4,ushort param_5,ushort param_6)

{
  code **ppcVar1;
  HINSTANCE16 HVar2;
  BOOL16 BVar3;
  uchar *puVar4;
  uchar *extraout_DX;
  astruct_72 *struct_var5;
  int unaff_DI;
  HWND16 hwnd;
  uchar in_AF;
  undefined4 uVar5;
  LRESULT LVar6;
  char *pcVar7;
  ushort *puVar8;
  ushort uVar9;
  undefined2 uVar10;
  int iVar11;
  uint16_t uVar12;
  undefined2 uVar13;
  uchar local_64 [0x50];
  undefined4 uStack20;
  HCURSOR16 HStack16;
  HCURSOR16 HStack14;
  undefined4 uStack6;
  astruct_72 *struct_var15;
  
  uVar9 = (ushort)param_1;
  struct_var15 = (astruct_72 *)((ulong)param_1 >> 0x10);
  hwnd = 0x1008;
  switch(param_2) {
  case 0x64:
    BVar3 = pass1_1008_07d8(uVar9,param_3,param_6,param_5);
    win_ui_cursor_op_1008_2e9a(param_1,param_5);
    return BVar3;
  case 0x65:
    pass1_1008_3018((ulong)param_1,(uchar *)param_6,unaff_DI,param_5);
    return param_3;
  case 0x66:
    pass1_1008_30cc((ulong)param_1,param_3,param_6,unaff_DI,param_5);
    return param_3;
  case 0x67:
    iVar11 = win_ui_op_1008_2b54(param_3,param_6,param_5);
    if (iVar11 == 0x0) {
      return 0x0;
    }
  case 0xee:
    uVar13 = 0x0;
    uVar10 = 0x10;
    goto LAB_1008_0d18;
  case 0x68:
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),0x4000001);
    puVar4 = (uchar *)(param_6 | param_3);
    if (puVar4 == (uchar *)0x0) {
      return param_3;
    }
    if (PTR_LOOP_1050_4fe8 != (undefined *)0x0) {
      pcVar7 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)(_PTR_LOOP_1050_14cc >> 0x10),0x1010);
      BVar3 = MessageBox16(0x1010,(LPCSTR)&PTR_LOOP_1050_0010,(LPCSTR)pcVar7,(UINT16)((ulong)pcVar7 >> 0x10));
      return BVar3;
    }
    HStack14 = LoadCursor16(0x1030,(LPCSTR)0x7f02);
    HStack16 = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
    uStack20 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,param_5,puVar4,unaff_DI);
    pass1_1018_262e((ulong)uStack20);
    pass1_1030_838e(_PTR_LOOP_1050_5748,param_5,in_AF);
    uVar13 = (undefined2)((ulong)_PTR_LOOP_1050_5748 >> 0x10);
    *(undefined2 *)((int)_PTR_LOOP_1050_5748 + 0x8) = 0x1;
    pass1_1030_8326();
    pcVar7 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)(_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    sys_1000_3f9c(local_64,(uchar *)param_5,0x19c,(ushort)&USHORT_1050_1050,(ushort)pcVar7,&stack0xfffe,uVar13,0x1000,
                  param_5,in_AF);
    ppcVar1 = (code **)((int)*param_1 + 0x14);
    (**ppcVar1)(0x1000,param_1,0x0,0x9c,param_5);
    puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_5,extraout_DX,unaff_DI);
    pass1_1008_a9ec((ulong)puVar8);
    hwnd = (HWND16)s_tile2_bmp_1050_1538;
    SetCursor16(0x1010);
    uVar13 = 0xfc;
    uVar10 = 0x111;
    goto LAB_1008_0e3d;
  default:
    if ((*(uint *)(uVar9 + 0xea) | *(uint *)(uVar9 + 0xe8)) == 0x0) {
      return 0x0;
    }
    uVar5 = *(undefined4 *)(uVar9 + 0xe8);
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(uVar9 + 0xe8) + 0x40);
    BVar3 = (**ppcVar1)(0x8,(int)uVar5,(int)((ulong)uVar5 >> 0x10),param_2);
    return BVar3;
  case 0x6e:
    iVar11 = 0x2;
    goto LAB_1008_0cba;
  case 0x6f:
    uStack6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1f8,param_5);
    BVar3 = WinHelp16(0x1010,(LPCSTR)0x0,0x0,CONCAT22((int)uStack6,0x3));
    return BVar3;
  case 0x70:
    iVar11 = 0x1;
LAB_1008_0cba:
    uVar5 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(uVar9 + 0x8),iVar11,param_6,uVar9,
                            (ushort)&PTR_LOOP_1050_1038,param_5);
    return (BOOL16)uVar5;
  case 0x71:
    HVar2 = WinExec16((LPCSTR)0x1008,0x3);
    return HVar2;
  case 0x79:
    BVar3 = post_msg_1008_2d22((ulong)param_1);
    return BVar3;
  case 0x7a:
    uVar12 = 0xb;
    goto LAB_1008_0f3e;
  case 0x7b:
    uVar12 = 0x1e;
    goto LAB_1008_0f3e;
  case 0x7c:
    uVar12 = 0x1f;
    goto LAB_1008_0f3e;
  case 0x7d:
    uVar12 = 0x21;
    goto LAB_1008_0f3e;
  case 0x7e:
    uVar12 = 0x35;
    goto LAB_1008_0f3e;
  case 0x7f:
    uVar13 = 0x39;
    break;
  case 0x80:
    uVar12 = 0x22;
    goto LAB_1008_0f3e;
  case 0x81:
    uVar13 = 0x36;
    break;
  case 0x82:
    uVar13 = 0x37;
    break;
  case 0x83:
    uVar13 = 0x38;
    break;
  case 0x84:
    uVar13 = 0x3a;
    break;
  case 0x85:
    uVar13 = 0x3b;
    break;
  case 0x86:
    uVar13 = 0x3c;
    break;
  case 0x87:
    uVar13 = 0x3d;
    break;
  case 0x88:
    uVar13 = 0x3e;
    break;
  case 0x89:
    uVar13 = 0x3f;
    break;
  case 0x8a:
    uVar13 = 0x40;
    break;
  case 0x8b:
    uVar12 = 0xc;
    goto LAB_1008_0f3e;
  case 0x8c:
    uVar13 = 0x41;
    break;
  case 0x8d:
    uVar13 = 0x42;
    break;
  case 0x8e:
    uVar13 = 0x43;
    break;
  case 0x8f:
    uVar13 = 0x44;
    break;
  case 0x90:
    uVar13 = 0x45;
    break;
  case 0x91:
    uVar13 = 0x46;
    break;
  case 0x92:
    uVar13 = 0x47;
    break;
  case 0x93:
    uVar12 = 0x23;
    goto LAB_1008_0f3e;
  case 0x94:
    uVar12 = 0x24;
    goto LAB_1008_0f3e;
  case 0x95:
    uVar13 = 0x48;
    break;
  case 0x96:
    uVar13 = 0x49;
    break;
  case 0x97:
    uVar13 = 0x4a;
    break;
  case 0x98:
    uVar13 = 0x4b;
    break;
  case 0x99:
    uVar13 = 0x4c;
    break;
  case 0x9a:
    uVar12 = 0xd;
    goto LAB_1008_0f3e;
  case 0x9b:
    uVar13 = 0x4d;
    break;
  case 0x9c:
    uVar13 = 0x4e;
    break;
  case 0x9d:
    uVar13 = 0x4f;
    break;
  case 0x9e:
    uVar13 = 0x50;
    break;
  case 0x9f:
    uVar13 = 0x51;
    break;
  case 0xa0:
    uVar12 = 0xe;
    goto LAB_1008_0f3e;
  case 0xa1:
    uVar12 = 0xf;
    goto LAB_1008_0f3e;
  case 0xa2:
    uVar13 = 0x52;
    break;
  case 0xa3:
    uVar12 = 0x10;
    goto LAB_1008_0f3e;
  case 0xa4:
    uVar13 = 0x53;
    break;
  case 0xa5:
    uVar12 = 0x11;
    goto LAB_1008_0f3e;
  case 0xa6:
    uVar12 = 0x12;
    goto LAB_1008_0f3e;
  case 0xa7:
    uVar13 = 0x57;
    break;
  case 0xa8:
    uVar12 = 0x13;
    goto LAB_1008_0f3e;
  case 0xa9:
    uVar12 = 0x14;
    goto LAB_1008_0f3e;
  case 0xaa:
    uVar13 = 0x58;
    break;
  case 0xab:
    uVar13 = 0x63;
    break;
  case 0xac:
    uVar13 = 0x59;
    break;
  case 0xad:
    uVar13 = 0x5a;
    break;
  case 0xae:
    uVar13 = 0x5b;
    break;
  case 0xaf:
    uVar13 = 0x15;
    break;
  case 0xb0:
    uVar12 = 0x25;
    goto LAB_1008_0f3e;
  case 0xb1:
    uVar13 = 0x5c;
    break;
  case 0xb2:
    uVar13 = 0x16;
    break;
  case 0xb3:
    uVar13 = 0x5d;
    break;
  case 0xb4:
    uVar12 = 0x5e;
    goto LAB_1008_0f3e;
  case 0xb5:
    uVar13 = 0x5f;
    break;
  case 0xb6:
    uVar13 = 0x60;
    break;
  case 0xb7:
    uVar13 = 0x61;
    break;
  case 0xb8:
    uVar13 = 0x62;
    break;
  case 0xb9:
    uVar13 = 0x64;
    break;
  case 0xba:
    uVar13 = 0x65;
    break;
  case 0xbb:
    uVar13 = 0x66;
    break;
  case 0xbc:
    uVar13 = 0x67;
    break;
  case 0xbd:
    uVar13 = 0x68;
    break;
  case 0xbe:
    uVar13 = 0x69;
    break;
  case 0xbf:
    uVar12 = 0x17;
    goto LAB_1008_0f3e;
  case 0xc0:
    uVar12 = 0x18;
    goto LAB_1008_0f3e;
  case 0xc1:
    uVar12 = 0x19;
    goto LAB_1008_0f3e;
  case 0xc2:
    uVar12 = 0x1a;
    goto LAB_1008_0f3e;
  case 0xc3:
    uVar12 = 0x1b;
    goto LAB_1008_0f3e;
  case 0xc4:
    uVar12 = 0x1c;
    goto LAB_1008_0f3e;
  case 0xc5:
    uVar12 = 0x1d;
    goto LAB_1008_0f3e;
  case 0xc6:
    uVar12 = 0x4;
    goto LAB_1008_0f3e;
  case 0xc8:
    uVar12 = 0x3;
    goto LAB_1008_0f3e;
  case 0xc9:
    uVar12 = 0x1;
    goto LAB_1008_0f3e;
  case 0xca:
    uVar12 = 0x5;
    goto LAB_1008_0f3e;
  case 0xcb:
    pass1_1008_087e(param_3,(uchar *)param_6,param_5,in_AF);
    uVar12 = 0x6;
    goto LAB_1008_0f3e;
  case 0xcc:
    uVar12 = 0x7;
    goto LAB_1008_0f3e;
  case 0xcd:
    uVar12 = 0x8;
    goto LAB_1008_0f3e;
  case 0xce:
    uVar12 = 0x9;
    goto LAB_1008_0f3e;
  case 0xcf:
    uVar12 = 0xa;
    goto LAB_1008_0f3e;
  case 0xd0:
    uVar12 = 0x26;
    goto LAB_1008_0f3e;
  case 0xd1:
    uVar12 = 0x27;
    goto LAB_1008_0f3e;
  case 0xd2:
    uVar12 = 0x28;
    goto LAB_1008_0f3e;
  case 0xd3:
    uVar12 = 0x29;
    goto LAB_1008_0f3e;
  case 0xd4:
    uVar12 = 0x2a;
    goto LAB_1008_0f3e;
  case 0xd5:
    uVar12 = 0x2b;
    goto LAB_1008_0f3e;
  case 0xd6:
    uVar12 = 0x2c;
    goto LAB_1008_0f3e;
  case 0xd7:
    uVar12 = 0x2d;
    goto LAB_1008_0f3e;
  case 0xd8:
    uVar12 = 0x2e;
    goto LAB_1008_0f3e;
  case 0xd9:
    uVar12 = 0x2f;
    goto LAB_1008_0f3e;
  case 0xda:
    uVar12 = 0x30;
    goto LAB_1008_0f3e;
  case 0xdb:
    uVar12 = 0x31;
    goto LAB_1008_0f3e;
  case 0xdc:
    uVar12 = 0x32;
    goto LAB_1008_0f3e;
  case 0xdd:
    uVar12 = 0x33;
    goto LAB_1008_0f3e;
  case 0xde:
    uVar12 = 0x34;
LAB_1008_0f3e:
    cursor_op_1008_2dcc(uVar9,(uint16_t)struct_var15,uVar12,0x1008);
    return param_3;
  case 0xdf:
    uVar13 = 0x55;
    break;
  case 0xe0:
    uVar13 = 0x56;
    break;
  case 0x100:
    win_1008_5c5c((WNDCLASS16 *)param_5,param_3,param_6,_PTR_LOOP_1050_02a0,0x1dc);
    return param_3;
  case 0x12c:
    uVar13 = 0xf020;
    uVar10 = 0x112;
LAB_1008_0d18:
    LVar6 = SendMessage16(0x1008,0x0,0x0,CONCAT22(uVar10,uVar13));
    return (BOOL16)LVar6;
  case 0x12e:
    uVar13 = 0xf060;
    uVar10 = 0x112;
LAB_1008_0e3d:
    BVar3 = PostMessage16(hwnd,0x0,0x0,CONCAT22(uVar10,uVar13));
    return BVar3;
  }
  ui_op_1008_2c4e(uVar9,struct_var15,uVar13,0x1008);
  return param_3;
}
