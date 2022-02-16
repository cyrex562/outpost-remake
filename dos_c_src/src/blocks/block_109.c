

void __stdcall16far pass1_1040_741e(ulong param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  uint uVar5;
  
  uVar5 = (uint)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  pass1_1010_1ea6(*(ulong *)(iVar4 + 0x94),param_1 & 0xffff | (ulong)uVar5 << 0x10,param_2);
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x98);
  uVar2 = *(uint *)(iVar4 + 0x9a);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1010,puVar1,uVar2,0x1);
  }
  *(undefined4 *)(iVar4 + 0x98) = 0x0;
  *(undefined4 *)(iVar4 + 0x94) = 0x0;
  return;
}



ushort __stdcall16far pass1_1040_746c(ulong param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x98) + 0x8);
  (**ppcVar1)();
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_cleanup_op_1040_748c(int param_1,ushort param_2,ushort param_3,ulong param_4)

{
  code **ppcVar1;
  undefined4 uVar2;
  uchar *in_DX;
  ushort unaff_SS;
  int iVar3;
  RECT16 local_a;
  int iStack6;
  int iStack4;
  
  switch(param_4._2_2_) {
  case 0xfa:
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0x94) + 0x18);
    (**ppcVar1)();
    break;
  default:
    pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,(ushort)&PTR_LOOP_1050_1040,unaff_SS);
    return;
  case 0xfd:
    if (DAT_1050_0ecc == 0x0) {
      return;
    }
    DAT_1050_0ecc = 0x0;
    goto LAB_1040_755d;
  case 0xfe:
    if (DAT_1050_0ecc == 0x1) {
      return;
    }
    DAT_1050_0ecc = 0x1;
    goto LAB_1040_755d;
  case 0xff:
    if (DAT_1050_0ecc == 0x2) {
      return;
    }
    DAT_1050_0ecc = 0x2;
LAB_1040_755d:
    uVar2 = *(undefined4 *)(param_1 + 0x94);
    ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(param_1 + 0x94) + 0x10);
    (**ppcVar1)((int)&PTR_LOOP_1050_1040,(int)uVar2,(int)((ulong)uVar2 >> 0x10));
    pass1_1010_2ee2(*(ulong **)(param_1 + 0x94),unaff_SS,0x1010);
    PostMessage16(0x1010,0x0,0x0,0x111010a);
    break;
  case 0x107:
    iVar3 = 0x0;
    goto LAB_1040_75ba;
  case 0x108:
    iVar3 = 0x1;
LAB_1040_75ba:
    win_ui_op_1010_3202(*(ulong *)(param_1 + 0x94),iVar3,0x1010);
    break;
  case 0x10a:
    GetClientRect16((HWND16)&PTR_LOOP_1050_1040,&local_a);
    uVar2 = *(undefined4 *)(param_1 + 0x94);
    local_a.y = local_a.y + 0x3;
    local_a.x = *(int *)((int)uVar2 + 0x1a) + -0x9;
    iStack6 = iStack6 + -0x3;
    iStack4 = iStack4 + -0x3;
    InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)((int)&PTR_LOOP_1050_0000 + 0x1),(BOOL16)&local_a);
    unk_destroy_win_op_1010_2fa0(*(ULONG *)(param_1 + 0x94),0x1010);
    pass1_1010_32c0(*(ulong *)(param_1 + 0x94),0x0);
    pass1_1010_2ee2(*(ulong **)(param_1 + 0x94),unaff_SS,0x1010);
    break;
  case 0x10c:
    DestroyWindow16((HWND16)&PTR_LOOP_1050_1040);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
create_window_1040_7620(undefined4 param_1,int param_2,HMENU16 *in_menu_handle,undefined2 param_4,INT16 param_5)

{
  int iVar1;
  undefined2 uVar2;
  HINSTANCE16 h_instance;
  
  load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  _h_instance = 0x50000009;
  if (param_2 != 0x0) {
    _h_instance = 0x50020009;
  }
  uVar2 = (undefined2)((ulong)in_menu_handle >> 0x10);
  iVar1 = (int)in_menu_handle;
  CreateWindow16((LPCSTR)0x1010,(LPCSTR)0x0,ZEXT24(PTR_LOOP_1050_038c) << 0x10,param_5,*(INT16 *)((int)param_1 + 0x6),
                 *(INT16 *)(iVar1 + 0x6),*(INT16 *)(iVar1 + 0x4),*(HWND16 *)(iVar1 + 0x2),*in_menu_handle,
                 (HINSTANCE16)_h_instance,(LPVOID)((ulong)_h_instance >> 0x10));
  return;
}



astruct_18 * __stdcall16far pass1_1040_767e(astruct_18 *param_1,byte param_2)

{
  unk_draw_op_1040_b0f8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far
get_sys_metrics_1040_7728(astruct_57 *param_1,ushort param_2,ulong param_3,ushort param_4,ushort param_5)

{
  INT16 IVar1;
  astruct_57 *iVar2;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_57 *)param_1;
  *(undefined2 *)param_1 = 0x389a;
  iVar2->field_0x2 = 0x1008;
  *(undefined2 *)param_1 = 0x3aa8;
  iVar2->field_0x2 = 0x1008;
  iVar2->field_0x4 = 0x0;
  iVar2->field_0x6 = 0x0;
  iVar2->field_0x8 = param_5;
  iVar2->field_0xa = param_4;
  iVar2->field_0xc = 0x0;
  iVar2->field_0x60 = 0x0;
  iVar2->field_0x62 = 0x0;
  iVar2->field_0x64 = 0x0;
  iVar2->field_0x66 = 0x0;
  iVar2->field_0x68 = 0x0;
  iVar2->field_0x6a = param_3;
  iVar2->field_0x6e = param_2;
  iVar2->field_0x70 = 0x0;
  iVar2->field_0x74 = 0x0;
  iVar2->field_0x76 = 0x0;
  iVar2->field_0x78 = 0x0;
  iVar2->field_0x8a = 0x0;
  iVar2->field_0x8c = 0x0;
  *(undefined2 *)param_1 = 0x840c;
  iVar2->field_0x2 = (int)&PTR_LOOP_1050_1040;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x10),(char *)0x10505db0);
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x7a),(WNDCLASS16 *)0x0,0x8);
  pass1_1000_4906((astruct_20 *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x82),(WNDCLASS16 *)0x0,0x8);
  IVar1 = GetSystemMetrics16(0x1000);
  iVar2->field_0x62 = IVar1;
  IVar1 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  iVar2->field_0x64 = IVar1;
  IVar1 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  iVar2->field_0x66 = IVar1;
  return;
}






// WARNING: Globals starting with '_' overlap smaller symbols at the same address







// WARNING: Could not reconcile some variable overlaps



// WARNING: Globals starting with '_' overlap smaller symbols at the same address





// WARNING: Globals starting with '_' overlap smaller symbols at the same address



// WARNING: Globals starting with '_' overlap smaller symbols at the same address


// WARNING: Globals starting with '_' overlap smaller symbols at the same address






// WARNING: Globals starting with '_' overlap smaller symbols at the same address
