

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



void __stdcall16far ui_cleanup_op_1040_782c(astruct_18 *param_1,HGDIOBJ16 param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  HGDIOBJ16 menu;
  HMENU16 hwnd;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  param_1->field_0x0 = 0x840c;
  *(undefined2 *)(iVar4 + 0x2) = (int)&PTR_LOOP_1050_1040;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x70);
  uVar2 = *(uint *)(iVar4 + 0x72);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(param_2,puVar1,uVar2,0x1);
  }
  menu = param_2;
  if (*(int *)(iVar4 + 0x4) != 0x0) {
    menu = (HGDIOBJ16)s_tile2_bmp_1050_1538;
    DeleteObject16(param_2);
    *(undefined2 *)(iVar4 + 0x4) = 0x0;
  }
  hwnd = menu;
  if (*(int *)(iVar4 + 0x68) != 0x0) {
    hwnd = (HMENU16)s_tile2_bmp_1050_1538;
    DestroyMenu16(menu);
  }
  RemoveProp16(hwnd,(LPCSTR)s_thisLo_1050_5db1);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisHi_1050_5db8);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_procLo_1050_5dbf);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_procHi_1050_5dc6);
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)(iVar4 + 0x2) = 0x1008;
  return;
}



void __stdcall16far pass1_1040_78de(void)

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far dialog_ui_fn_1040_78e2(astruct_1 *in_struct_1,HINSTANCE16 in_instance_handle)

{
  code **ppcVar1;
  LPCSTR dlg_template;
  HWND16 dialog_handle;
  astruct_1 *local_struct_1;
  astruct_1 *local_string_1;
  undefined2 uVar2;
  long lVar3;
  undefined2 uVar4;
  undefined2 uVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  LPCSTR local_string_2;
  LPCSTR pCStack8;
  
  local_string_1 = (astruct_1 *)((ulong)in_struct_1 >> 0x10);
  local_struct_1 = (astruct_1 *)in_struct_1;
  if (*(long *)&local_struct_1->field_0xc == 0x0) {
    uVar2 = (undefined2)((ulong)_PTR_LOOP_1050_5bc8 >> 0x10);
    dlg_template = *(LPCSTR *)((int)_PTR_LOOP_1050_5bc8 + 0x4);
    dialog_handle = *(HWND16 *)((int)_PTR_LOOP_1050_5bc8 + 0x6);
  }
  else {
    dlg_template = *(LPCSTR *)&local_struct_1->field_0xc;
    dialog_handle = *(HWND16 *)&local_struct_1->field_0xe;
  }
  dialog_handle = CreateDialog16(in_instance_handle,dlg_template,dialog_handle,local_struct_1->lpvoid_field_0x8);
  *(HWND16 *)&local_struct_1->field_0x6 = dialog_handle;
  GetWindowText16((HWND16)s_tile2_bmp_1050_1538,0x50,(INT16)&local_struct_1->max_count_field_0x10);
  lVar3 = GetWindowLong16((HWND16)s_tile2_bmp_1050_1538,-0x4);
  SetWindowLong16((HWND16)s_tile2_bmp_1050_1538,(INT16)_PTR_LOOP_1050_5bcc,
                  CONCAT22(0xfffc,(int)((ulong)_PTR_LOOP_1050_5bcc >> 0x10)));
  uVar2 = *(undefined2 *)&local_struct_1->field_0x6;
  uVar10 = SUB42(&USHORT_1050_1050,0x0);
  SetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)local_struct_1,(HANDLE16)s_thisLo_1050_5dcd);
  uVar9 = *(undefined2 *)&local_struct_1->field_0x6;
  uVar8 = SUB42(&USHORT_1050_1050,0x0);
  SetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)local_string_1,(HANDLE16)s_thisHi_1050_5dd4);
  local_string_2 = (LPCSTR)lVar3;
  uVar7 = *(undefined2 *)&local_struct_1->field_0x6;
  uVar6 = SUB42(&USHORT_1050_1050,0x0);
  SetProp16((HWND16)s_tile2_bmp_1050_1538,local_string_2,(HANDLE16)s_procLo_1050_5ddb);
  pCStack8 = (LPCSTR)((ulong)lVar3 >> 0x10);
  uVar5 = *(undefined2 *)&local_struct_1->field_0x6;
  uVar4 = SUB42(&USHORT_1050_1050,0x0);
  SetProp16((HWND16)s_tile2_bmp_1050_1538,pCStack8,(HANDLE16)s_procHi_1050_5de2);
  ppcVar1 = (code **)((int)in_struct_1->field_0x0 + 0x50);
  (**ppcVar1)((int)s_tile2_bmp_1050_1538,in_struct_1,uVar4,uVar5,uVar6,uVar7,uVar8,uVar9,uVar10,uVar2);
  return;
}



ushort __stdcall16far pass1_1040_79c0(ulong *param_1,int *param_2,ushort param_3,ushort param_4,uint param_5)

{
  code **ppcVar1;
  char cVar2;
  ushort uVar3;
  undefined2 unaff_CS;
  
  if (param_5 == 0xa1) {
    ppcVar1 = (code **)((int)*param_1 + 0x38);
    uVar3 = (**ppcVar1)();
    return uVar3;
  }
  if (param_5 < 0xa2) {
    if (param_5 == 0x85) {
      ppcVar1 = (code **)((int)*param_1 + 0x1c);
      (**ppcVar1)();
      return 0x1;
    }
    if (param_5 < 0x86) {
      cVar2 = (char)param_5;
      if (cVar2 == '\x02') {
        ppcVar1 = (code **)((int)*param_1 + 0x24);
        (**ppcVar1)();
        return 0x1;
      }
      if (cVar2 == '\f') {
        ppcVar1 = (code **)((int)*param_1 + 0x18);
        (**ppcVar1)();
        return 0x1;
      }
      if (cVar2 == '\x0f') {
        ppcVar1 = (code **)((int)*param_1 + 0x60);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (cVar2 == '+') {
        if (*param_2 != 0x4) {
          return 0x1;
        }
        win_ui_get_prop_op_1040_9566((int *)CONCAT22(param_3,param_2),unaff_CS);
        return 0x1;
      }
    }
  }
  else {
    if (param_5 == 0x114) {
      ppcVar1 = (code **)((int)*param_1 + 0x58);
      uVar3 = (**ppcVar1)();
      return uVar3;
    }
    if (param_5 < 0x115) {
      if (param_5 == 0x104) {
        ppcVar1 = (code **)((int)*param_1 + 0x30);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (param_5 == 0x111) {
        ppcVar1 = (code **)((int)*param_1 + 0x10);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
    }
    else {
      if (param_5 == 0x115) {
        ppcVar1 = (code **)((int)*param_1 + 0x54);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (param_5 == 0x201) {
        ppcVar1 = (code **)((int)*param_1 + 0x44);
        (**ppcVar1)();
        return 0x1;
      }
      if (param_5 == 0x204) {
        ppcVar1 = (code **)((int)*param_1 + 0x28);
        (**ppcVar1)();
        return 0x1;
      }
    }
  }
  return 0x0;
}



BOOL16 __stdcall16far post_win_msg_1040_7b3c(ulong *param_1,ushort param_2,ushort param_3,int param_4,HWND16 param_5)

{
  code **ppcVar1;
  
  if ((param_4 == 0x1) || (param_4 == 0x2)) {
    ppcVar1 = (code **)((int)*param_1 + 0x14);
    (**ppcVar1)();
  }
  else {
    if (param_4 == 0x6f) {
      ppcVar1 = (code **)((int)*param_1 + 0x2c);
      (**ppcVar1)(param_5,param_1);
    }
    else {
      if (param_4 != 0x12e) {
        return 0x0;
      }
      PostMessage16(param_5,0x0,0x0,0x112f060);
    }
  }
  return 0x1;
}



void __stdcall16far destroy_win_1040_7b98(ULONG param_1,HWND16 param_2)

{
  if (*(int *)((int)param_1 + 0x74) == 0x0) {
    DestroyWindow16(param_2);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void __stdcall16far draw_op_1040_7bb2(astruct_14 *in_struct_1,HWND16 in_win_handle_2,UINT16 param_3)

{
  code **ppcVar1;
  BOOL16 BVar2;
  int y;
  int iVar3;
  COLORREF color;
  HPEN16 handle;
  HGDIOBJ16 handle_00;
  RECT16 *rect;
  HANDLE16 handle_01;
  LPCSTR str;
  astruct_14 *iVar4;
  char *count;
  char *str_00;
  ulong uVar4;
  DWORD DVar5;
  HBRUSH16 hbrush;
  undefined2 uVar6;
  undefined2 uVar7;
  HGDIOBJ16 local_obj_handle_42;
  RECT16 local_rect_12;
  int iStack14;
  int iStack12;
  HPALETTE16 HStack10;
  undefined4 uStack8;
  HDC16 local_hdc_4;
  
  str_00 = (char *)((ulong)in_struct_1 >> 0x10);
  iVar4 = (astruct_14 *)in_struct_1;
  uVar7 = iVar4->field_0x6;
  BVar2 = IsIconic16(in_win_handle_2);
  if (BVar2 == 0x0) {
    uVar6 = iVar4->field_0x6;
    local_hdc_4 = GetWindowDC16((HWND16)s_tile2_bmp_1050_1538);
    ppcVar1 = (code **)((int)*(undefined4 *)in_struct_1 + 0x68);
    uStack8 = (astruct_13 *)(**ppcVar1)((int)s_tile2_bmp_1050_1538,in_struct_1,iVar4->field_0x6e,uVar6,uVar7);
    if (uStack8 != (astruct_13 *)0x0) {
      HStack10 = palette_op_1008_4e08(uStack8,&local_hdc_4,(uint)((ulong)uStack8 >> 0x10) | (uint)uStack8,0x1008);
      GetWindowRect16(0x1008,&local_rect_12);
      y = (iStack14 - local_rect_12.x) + -0x1;
      iVar3 = (iStack12 - local_rect_12.y) + -0x1;
      color = (-(uint)(iVar4->field_0x60 == 0x0) & 0x1e) + 0x25;
      handle = CreatePen16((INT16)s_tile2_bmp_1050_1538,color,0x100);
      handle_00 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle);
      MoveTo16((HDC16)s_tile2_bmp_1050_1538,0x0,0x0);
      LineTo16((HDC16)s_tile2_bmp_1050_1538,0x0,y);
      LineTo16((HDC16)s_tile2_bmp_1050_1538,iVar3,y);
      LineTo16((HDC16)s_tile2_bmp_1050_1538,iVar3,0x0);
      LineTo16((HDC16)s_tile2_bmp_1050_1538,0x0,0x0);
      uVar4 = GetWindowLong16((HWND16)s_tile2_bmp_1050_1538,-0x10);
      if (((uVar4 & 0x800000) != 0x0) && ((uVar4 & 0x400000) != 0x0)) {
        iVar3 = iVar4->field_0x62 - iVar4->field_0x66;
        MoveTo16((HDC16)s_tile2_bmp_1050_1538,iVar3,0x0);
        LineTo16((HDC16)s_tile2_bmp_1050_1538,iVar3,y);
        iVar4->field_0x7a = iVar4->field_0x64;
        iVar4->field_0x7c = iVar4->field_0x66;
        iVar4->field_0x7e = y;
        iVar4->field_0x80 = iVar4->field_0x62 - iVar4->field_0x66;
        hbrush = 0x4;
        rect = (RECT16 *)GetStockObject16((INT16)s_tile2_bmp_1050_1538);
        FillRect16((HDC16)s_tile2_bmp_1050_1538,rect,hbrush);
        if (iVar4->field_0x76 != 0x0) {
          draw_op_1040_82ee((astruct_15 *)in_struct_1,(int)s_tile2_bmp_1050_1538);
        }
        count = &iVar4->field_0x10;
        if (*count != '\0') {
          local_obj_handle_42 = 0x0;
          handle_01 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)0x5de9);
          if (handle_01 != 0x0) {
            local_obj_handle_42 = SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle_01);
          }
          SetBkColor16((HDC16)s_tile2_bmp_1050_1538,0x0);
          SetTextColor16((HDC16)s_tile2_bmp_1050_1538,color);
          str = (LPCSTR)lstrlen16((LPCSTR)s_tile2_bmp_1050_1538);
          DVar5 = GetTextExtent16((HDC16)s_tile2_bmp_1050_1538,str,(INT16)count);
          TextOut16((HDC16)s_tile2_bmp_1050_1538,(INT16)str,(INT16)count,str_00,
                    (iVar4->field_0x80 - iVar4->field_0x7c) / 0x2 - (int)(DVar5 >> 0x10) / 0x2);
          if (handle_01 != 0x0) {
            SelectObject16((HDC16)s_tile2_bmp_1050_1538,local_obj_handle_42);
          }
        }
      }
      ppcVar1 = (code **)((int)*(undefined4 *)in_struct_1 + 0x64);
      (**ppcVar1)((int)s_tile2_bmp_1050_1538,in_struct_1,uStack8,local_hdc_4);
      HStack10 = SelectPalette16((HDC16)s_tile2_bmp_1050_1538,0x0,HStack10);
      DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
      SelectObject16((HDC16)s_tile2_bmp_1050_1538,handle_00);
      DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
    }
    ReleaseDC16((HWND16)s_tile2_bmp_1050_1538,local_hdc_4);
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

undefined4 __stdcall16far set_text_bk_color_1040_7e5e(ulong *param_1,ushort param_2,uint param_3,INT16 param_4)

{
  code **ppcVar1;
  int iVar2;
  HGDIOBJ16 HVar3;
  INT16 IVar4;
  HWND16 hwnd;
  HWND16 hdc;
  ulong uVar5;
  COLORREF color;
  undefined2 uVar6;
  
  uVar6 = 0x4;
  hwnd = (HWND16)s_tile2_bmp_1050_1538;
  HVar3 = GetStockObject16(param_4);
  if (_PTR_LOOP_1050_5df0 == 0x0) {
    ppcVar1 = (code **)((int)*param_1 + 0x68);
    uVar5 = (**ppcVar1)((int)s_tile2_bmp_1050_1538,param_1,*(undefined2 *)((int)param_1 + 0x6e),uVar6);
    if (uVar5 == 0x0) {
      return 0x0;
    }
    hwnd = 0x1008;
    uVar5 = pass1_1008_4d72(uVar5);
    uVar6 = (undefined2)(uVar5 >> 0x10);
    iVar2 = (int)uVar5;
    _PTR_LOOP_1050_5df0 =
         CONCAT22(CONCAT11(0x2,*(undefined *)(iVar2 + 0x94)),
                  CONCAT11(*(undefined *)(iVar2 + 0x95),*(undefined *)(iVar2 + 0x96)));
  }
  hdc = hwnd;
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return 0x0;
    }
    hdc = (HWND16)s_tile2_bmp_1050_1538;
    IVar4 = GetDlgCtrlID16(hwnd);
    if (IVar4 == 0x14c) {
      color = 0x0;
      goto LAB_1040_7f00;
    }
    if (IVar4 == 0x175) {
      color = 0x0;
      goto LAB_1040_7f00;
    }
  }
  color = (COLORREF)_PTR_LOOP_1050_5df0;
LAB_1040_7f00:
  SetTextColor16(hdc,color);
  SetBkColor16((HDC16)s_tile2_bmp_1050_1538,0x0);
  return CONCAT22(0x1050,HVar3);
}



BOOL16 __stdcall16far post_win_msg_1040_7f1c(ulong param_1,int param_2,HWND16 param_3)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(int *)(iVar1 + 0x78) != 0x0) {
    return 0x0;
  }
  if (*(int *)(iVar1 + 0x60) != param_2) {
    *(int *)(iVar1 + 0x60) = param_2;
    PostMessage16(param_3,0x0,0x0,0x850000);
  }
  return 0x1;
}



void __stdcall16far post_win_msg_1040_7f56(ulong param_1,char *param_2)

{
  unk_str_op_1000_3d3e((char *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x10)),param_2);
  PostMessage16(0x1000,0x0,0x0,0x850000);
  return;
}



void __stdcall16far menu_ui_op_1040_7f86(undefined4 param_1,HWND16 param_2,RECT16 *param_3)

{
  HMENU16 HVar1;
  int iVar2;
  undefined2 uVar3;
  HWND16 unaff_CS;
  POINT16 local_6;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((*(long *)(iVar2 + 0x6a) != 0x0) && (*(int *)(iVar2 + 0x68) == 0x0)) {
    HVar1 = LoadMenu16(unaff_CS,(LPCSTR)*(undefined4 *)(iVar2 + 0x6a));
    *(HMENU16 *)(iVar2 + 0x68) = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
    unaff_CS = (HWND16)s_tile2_bmp_1050_1538;
    HVar1 = GetSubMenu16((HMENU16)s_tile2_bmp_1050_1538,0x0);
    *(HMENU16 *)(iVar2 + 0x68) = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
  }
  local_6.x = (INT16)param_3;
  local_6.y = param_2;
  ClientToScreen16(unaff_CS,&local_6);
  TrackPopupMenu16((HMENU16)s_tile2_bmp_1050_1538,0x0,0x0,*(INT16 *)(iVar2 + 0x6),0x0,local_6.y,(RECT16 *)local_6.x);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_help_1040_800c(ulong param_1,ushort param_2)

{
  undefined2 uVar1;
  astruct_43 *paVar2;
  LPCSTR lp_help_file;
  UINT16 w_command;
  undefined2 uVar3;
  
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1f8,param_2);
  uVar1 = (undefined2)(param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x8a) == 0x0) {
    w_command = 0x0;
    uVar3 = 0x3;
    lp_help_file = (LPCSTR)0x0;
  }
  else {
    uVar3 = 0x1;
    lp_help_file = *(LPCSTR *)((int)param_1 + 0x8a);
    w_command = (int)lp_help_file >> 0xf;
  }
  WinHelp16(0x1010,lp_help_file,w_command,CONCAT22((int)paVar2,uVar3));
  return;
}



ushort __stdcall16far pass1_1040_8054(void)

{
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1040_805a(uchar *param_1)

{
  int unaff_DI;
  undefined2 uVar1;
  ushort unaff_SS;
  
  if (_PTR_LOOP_1050_4230 == 0x0) {
    mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x28,unaff_SS,param_1,unaff_DI);
  }
  uVar1 = (undefined2)((ulong)_PTR_LOOP_1050_4230 >> 0x10);
  return CONCAT22(*(undefined2 *)((int)_PTR_LOOP_1050_4230 + 0x10),*(undefined2 *)((int)_PTR_LOOP_1050_4230 + 0xe));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1040_807e(ulong param_1,ushort param_2,ushort param_3)

{
  uint uVar1;
  code **ppcVar2;
  undefined4 *puVar3;
  undefined4 *puVar4;
  uchar *in_DX;
  uint uVar5;
  uchar *extraout_DX;
  uchar *puVar6;
  uchar *extraout_DX_00;
  uchar *puVar7;
  astruct_395 *iVar9;
  undefined2 uVar8;
  astruct_43 *paVar9;
  ulong uStack10;
  astruct_393 *iVar8;
  
  if (param_2 == 0x1) {
    pass1_1040_805a(in_DX);
    return;
  }
  paVar9 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,param_2,param_3);
  uVar5 = (uint)((ulong)paVar9 >> 0x10);
  puVar3 = (undefined4 *)paVar9;
  if ((uVar5 | (uint)puVar3) != 0x0) {
    ppcVar2 = (code **)((int)*(undefined4 *)paVar9 + 0x14);
    puVar4 = puVar3;
    (**ppcVar2)(0x1010,puVar3,uVar5);
    uStack10 = CONCAT22(extraout_DX,puVar4);
    uVar8 = (undefined2)(param_1 >> 0x10);
    iVar9 = (astruct_395 *)param_1;
    puVar6 = extraout_DX;
    if (iVar9->field_0x70 != (astruct_90 *)0x0) {
      puVar4 = *(undefined4 **)&iVar9->field_0x70;
      uVar1 = *(uint *)((int)&iVar9->field_0x70 + 0x2);
      puVar6 = (uchar *)(uVar1 | (uint)puVar4);
      if (puVar6 != (uchar *)0x0) {
        ppcVar2 = (code **)*puVar4;
        (**ppcVar2)();
        puVar6 = extraout_DX_00;
      }
    }
    mem_op_1000_179c(0x14,puVar6,0x1000);
    puVar7 = (uchar *)((uint)puVar6 | (uint)puVar4);
    if (puVar7 == (uchar *)0x0) {
      puVar4 = (undefined4 *)0x0;
      puVar7 = (uchar *)0x0;
    }
    else {
      struct_1008_4c58((ushort *)CONCAT22(puVar6,puVar4));
    }
    *(undefined4 **)&iVar9->field_0x70 = puVar4;
    *(uchar **)((int)&iVar9->field_0x70 + 0x2) = puVar7;
    pass1_1008_4d84(iVar9->field_0x70,uStack10,puVar7);
    if (paVar9 != (astruct_43 *)0x0) {
      ppcVar2 = (code **)*(undefined4 *)paVar9;
      (**ppcVar2)(0x1008,puVar3,uVar5,0x1);
    }
    return;
  }
  return;
}



void __stdcall16far unk_win_ui_op_1040_8158(ulong *param_1,POINT16 param_2,int param_3,HWND16 param_4)

{
  code **ppcVar1;
  INT16 IVar2;
  BOOL16 BVar3;
  ulong uVar4;
  int iVar5;
  undefined2 uVar6;
  undefined2 uVar7;
  POINT16 local_6;
  
  if (param_3 == 0x2) {
    uVar6 = (undefined2)((ulong)param_1 >> 0x10);
    iVar5 = (int)param_1;
    if (*(int *)(iVar5 + 0x76) != 0x0) {
      local_6 = param_2;
      uVar6 = *(undefined2 *)(iVar5 + 0x6);
      ScreenToClient16(param_4,&local_6);
      uVar7 = 0x4;
      IVar2 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
      local_6 = (POINT16)((ulong)local_6 & 0xffff | (ulong)(uint)(local_6.y + IVar2) << 0x10);
      uVar4 = (ulong)param_1 & 0xffff0000 | (ulong)(iVar5 + 0x82);
      BVar3 = PtInRect16((RECT16 *)s_tile2_bmp_1050_1538,local_6);
      if (BVar3 != 0x0) {
        ppcVar1 = (code **)((int)*param_1 + 0x14);
        (**ppcVar1)((int)s_tile2_bmp_1050_1538,param_1,0x0,uVar4,uVar7,uVar6);
      }
    }
  }
  return;
}



void __stdcall16far check_dialog_msg_1040_81b6(ulong param_1,HWND16 param_2)

{
  BOOL16 BVar1;
  MSG16 local_14;
  
  *(undefined2 *)((int)param_1 + 0x78) = 0x1;
  while( true ) {
    BVar1 = IsWindow16(param_2);
    if (BVar1 == 0x0) {
      return;
    }
    BVar1 = GetMessage16((MSG16 *)s_tile2_bmp_1050_1538,0x0,0x0,0x0);
    if (BVar1 == 0x0) break;
    param_2 = (HWND16)s_tile2_bmp_1050_1538;
    IsDialogMessage16((HWND16)s_tile2_bmp_1050_1538,&local_14);
  }
  return;
}



void __stdcall16far win_ui_op_1040_81fe(HWND16 param_1)

{
  SetSysModalWindow(param_1);
  return;
}



void __stdcall16far destroy_win_1040_8212(ULONG param_1,HWND16 param_2)

{
  BOOL16 is_window;
  UINT16 uVar1;
  
  uVar1 = (UINT16)(param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x8c) != 0x0) {
    is_window = IsWindow16(param_2);
    if (is_window != 0x0) {
      DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
      *(undefined2 *)((int)param_1 + 0x8c) = 0x0;
    }
  }
  return;
}



ushort __stdcall16far pass1_1040_824a(ulong param_1,int param_2)

{
  if (*(int *)((int)param_1 + 0x6) != param_2) {
    return 0x1;
  }
  return 0x0;
}



void __stdcall16far move_win_1040_826c(astruct_1 *param_1,INT16 param_2,BOOL16 param_3)

{
  INT16 IVar1;
  HWND16 unaff_CS;
  RECT16 local_e;
  int iStack10;
  int iStack8;
  INT16 IStack6;
  BOOL16 BStack4;
  
  GetWindowRect16(unaff_CS,&local_e);
  if ((param_3 == 0xffff) || (param_2 == -0x1)) {
    IVar1 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    BStack4 = (IVar1 - (iStack10 - local_e.x)) / 0x2;
    IVar1 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
    param_2 = (IVar1 - (iStack8 - local_e.y)) / 0x2;
  }
  else {
    BStack4 = param_3;
  }
  IStack6 = param_2;
  MoveWindow16((HWND16)s_tile2_bmp_1050_1538,0x0,iStack8 - local_e.y,iStack10 - local_e.x,param_2,BStack4);
  return;
}



void __stdcall16far draw_op_1040_82ee(astruct_15 *param_1,COLORREF in_colorref_2)

{
  astruct_15 *iVar1;
  undefined2 uVar1;
  undefined4 local_1a;
  undefined4 uStack22;
  int local_12;
  int iStack16;
  int iStack14;
  int iStack12;
  RECT16 *l_brush_handle;
  int iStack8;
  int iStack6;
  int iStack4;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_15 *)param_1;
  iStack6 = (iVar1->field_0x80 - iVar1->field_0x7c) + -0x2;
  iStack8 = (-(uint)(iVar1->field_0x60 == 0x0) & 0x1e) + 0x25;
  iStack4 = iStack6;
  l_brush_handle = (RECT16 *)CreateSolidBrush16(in_colorref_2);
  if (iVar1->field_0x86 == 0x0) {
    local_1a = CONCAT22(iVar1->field_0x66 + 0x2,iVar1->field_0x64 + 0x2);
    uStack22 = CONCAT22(iStack4,iStack6);
    *(undefined4 *)&iVar1->field_0x82 = local_1a;
    *(undefined4 *)&iVar1->field_0x86 = uStack22;
  }
  local_12 = iVar1->field_0x82 + 0x2;
  iStack16 = (iVar1->field_0x88 - iVar1->field_0x84) / 0x2 + iVar1->field_0x84 + -0x2;
  iStack14 = iVar1->field_0x86 - 0x2;
  iStack12 = (iVar1->field_0x88 - iVar1->field_0x84) / 0x2 + iVar1->field_0x84 + 0x2;
  FrameRect16((HDC16)s_tile2_bmp_1050_1538,l_brush_handle,(HBRUSH16)&iVar1->field_0x82);
  FrameRect16((HDC16)s_tile2_bmp_1050_1538,l_brush_handle,(HBRUSH16)&local_12);
  DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
  iVar1->field_0x7a = iVar1->field_0x86 + 0x2;
  return;
}



astruct_18 * __stdcall16far pass1_1040_83e6(astruct_18 *param_1,byte param_2,ushort param_3)

{
  ui_cleanup_op_1040_782c(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_57 * __stdcall16far
pass1_1040_8478(astruct_57 *param_1,ushort param_2,char *param_3,char *param_4,ushort param_5,ushort param_6)

{
  ushort uVar1;
  astruct_712 *iVar2;
  undefined2 uVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfc3,param_5);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_712 *)param_1;
  iVar2->field_0x8e = 0x0;
  iVar2->field_0x98 = param_2;
  iVar2->field_0x9a = 0x0;
  iVar2->field_0xb2 = 0x0;
  *(undefined2 *)param_1 = 0x8ddc;
  iVar2->field_0x2 = (int)&PTR_LOOP_1050_1040;
  iVar2->field_0x9e = 0x0;
  iVar2->field_0xa2 = 0x12c;
  uVar1 = str_op_1008_60e8(param_4,param_6);
  iVar2->field_0x90 = uVar1;
  iVar2->field_0x92 = param_6;
  uVar1 = str_op_1008_60e8(param_3,param_6);
  iVar2->field_0x94 = uVar1;
  iVar2->field_0x96 = param_6;
  load_icon_1040_8b92((ulong)param_1,0x1008);
  PTR_LOOP_1050_5df8 = (undefined *)0x0;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

int __cdecl16far
string_1040_8520(astruct_57 *param_1,ushort param_2,ushort param_3,int param_4,ushort param_5,undefined2 param_6,
                uchar *param_7,ushort param_8)

{
  ULONG UVar1;
  uint uVar2;
  ushort uVar3;
  astruct_293 *iVar5;
  undefined2 uVar4;
  undefined2 uVar5;
  char *pcVar6;
  int iStack22;
  int iStack16;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfc3,param_2);
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (astruct_293 *)param_1;
  iVar5->field_0x8e = 0x0;
  iVar5->field_0x98 = param_3;
  iVar5->field_0x9a = 0x0;
  iVar5->field_0xb2 = 0x0;
  *(undefined2 *)param_1 = 0x8ddc;
  iVar5->field_0x2 = (int)&PTR_LOOP_1050_1040;
  iVar5->field_0x9e = 0x0;
  iVar5->field_0xa2 = 0x12c;
  iStack16 = param_4;
  if (param_4 != 0x0) {
    load_string_1010_84ac((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    iVar5->field_0x94 = param_5;
    iVar5->field_0x96 = param_7;
    iStack16 = param_4 + -0x1;
  }
  iStack22 = 0x0;
  while (iStack16 != 0x0) {
    pcVar6 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    param_7 = (uchar *)((ulong)pcVar6 >> 0x10);
    uVar2 = str_op_1000_3da4(pcVar6);
    iStack22 = iStack22 + uVar2;
    iStack16 = iStack16 + -0x1;
  }
  uVar3 = iStack22 + 0x1;
  uVar5 = 0x1000;
  mem_op_1000_179c(uVar3,param_7,0x1000);
  *(ushort *)&iVar5->field_0x90 = uVar3;
  *(uchar **)((int)&iVar5->field_0x90 + 0x2) = param_7;
  iStack16 = param_4 + -0x1;
  if (param_4 + -0x1 != 0x0) {
    UVar1 = iVar5->field_0x90;
    uVar5 = 0x1010;
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,(char *)UVar1,
               (short)(UVar1 >> 0x10));
    iStack16 = param_4 + -0x2;
  }
  while (iStack16 != 0x0) {
    pcVar6 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    uVar5 = 0x1000;
    pass1_1000_3cea(iVar5->field_0x90,(ULONG)pcVar6);
    iStack16 = iStack16 + -0x1;
  }
  load_icon_1040_8b92((ulong)param_1,uVar5);
  PTR_LOOP_1050_5df8 = (undefined *)0x0;
  return (int)iVar5;
}
