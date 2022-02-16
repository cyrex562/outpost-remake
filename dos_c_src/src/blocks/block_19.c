





// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_9466(ushort *param_1)

{
  *param_1 = 0x52a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  fn_ptr_1000_17ce(_PTR_LOOP_1050_0392,0x1000);
  _PTR_LOOP_1050_0392 = (astruct_18 *)0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

WPARAM16 __cdecl16far win_msg_op_1008_9498(MSG *in_msg_1,MSG16 *in_msg_2)

{
  BOOL16 BVar1;
  INT16 IVar2;
  MSG16 local_msg_1;
  
LAB_1008_949c:
  BVar1 = GetMessage16((MSG16 *)in_msg_1,0x0,0x0,0x0);
  if (BVar1 == 0x0) {
    return local_msg_1.wparam;
  }
  if (*(int *)((int)_PTR_LOOP_1050_5bc8 + 0x8) != 0x0) goto code_r0x100894cd;
  goto LAB_1008_94dc;
code_r0x100894cd:
  in_msg_1 = (MSG *)s_tile2_bmp_1050_1538;
  BVar1 = IsDialogMessage16((HWND16)s_tile2_bmp_1050_1538,&local_msg_1);
  if (BVar1 == 0x0) {
LAB_1008_94dc:
    if (PTR_LOOP_1050_0398 != (undefined *)0x0) {
      in_msg_1 = (MSG *)s_tile2_bmp_1050_1538;
      IVar2 = TranslateAccelerator16((HWND16)s_tile2_bmp_1050_1538,(HACCEL16)&local_msg_1,in_msg_2);
      if (IVar2 != 0x0) goto LAB_1008_949c;
    }
    TranslateMessage16((MSG16 *)s_tile2_bmp_1050_1538);
    in_msg_1 = (MSG *)s_tile2_bmp_1050_1538;
    DispatchMessage16((MSG16 *)s_tile2_bmp_1050_1538);
  }
  goto LAB_1008_949c;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl16far unk_win_msg_op_1008_9510(int *param_1,MSG16 *param_2,MSG16 *param_3)

{
  BOOL16 has_message;
  INT16 IVar1;
  MSG16 local_14;
  
LAB_1008_9578:
  if (*param_1 != 0x0) {
    has_message = GetMessage16(param_2,0x0,0x0,0x0);
    if (has_message != 0x0) {
      if (*(int *)((int)_PTR_LOOP_1050_5bc8 + 0x8) != 0x0) goto code_r0x10089538;
      goto LAB_1008_9547;
    }
  }
  return;
code_r0x10089538:
  param_2 = (MSG16 *)s_tile2_bmp_1050_1538;
  has_message = IsDialogMessage16((HWND16)s_tile2_bmp_1050_1538,&local_14);
  if (has_message == 0x0) {
LAB_1008_9547:
    if (PTR_LOOP_1050_0398 != (undefined *)0x0) {
      param_2 = (MSG16 *)s_tile2_bmp_1050_1538;
      IVar1 = TranslateAccelerator16((HWND16)s_tile2_bmp_1050_1538,(HACCEL16)&local_14,param_3);
      if (IVar1 != 0x0) goto LAB_1008_9578;
    }
    TranslateMessage16((MSG16 *)s_tile2_bmp_1050_1538);
    param_2 = (MSG16 *)s_tile2_bmp_1050_1538;
    DispatchMessage16((MSG16 *)s_tile2_bmp_1050_1538);
  }
  goto LAB_1008_9578;
}



void __stdcall16far set_struct_op_1008_9584(astruct_20 *param_1,ULONG param_2)

{
  astruct_20 *iVar1;
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_20 *)param_1;
  param_1->field_0x0 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = param_2;
  param_1->field_0x0 = 0x9d2e;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x8 = 0x0;
  iVar1->field_0xac = 0x2000000;
  iVar1->field_0xb0 = 0x0;
  iVar1->field_0xb4 = 0x8000;
  iVar1->field_0xb6 = 0x8000;
  iVar1->field_0xb8 = 0x8000;
  iVar1->field_0xba = 0x8000;
  iVar1->field_0xbc = 0x0;
  iVar1->field_0xbe = 0x0;
  iVar1->field_0xc2 = 0x0;
  iVar1->hcursor_field_0xc4 = 0x0;
  iVar1->hgdiobj_field_0xc6 = 0x0;
  iVar1->field_0xc8 = 0x2008;
  iVar1->field_0xca = 0x0;
  param_1->field_0x0 = 0x380a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x5b = '\0';
  *(undefined *)&iVar1->field_0xa = 0x0;
  return;
}



void __stdcall16far pass1_1008_9628(undefined4 param_1,undefined2 param_2)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x8) == 0x0) {
    *(undefined2 *)((int)param_1 + 0x8) = param_2;
  }
  return;
}



void __stdcall16far send_msg_1008_9640(ulong param_1,ushort param_2,HWND16 param_3)

{
  if (*(int *)((int)param_1 + 0x8) != 0x0) {
    SendMessage16(param_3,0x0,0x0,CONCAT22(0x86,param_2));
  }
  return;
}



void __stdcall16far set_win_text_1008_9664(ulong param_1,undefined2 param_2,char *param_3)

{
  unk_str_op_1000_3d3e((char *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0xaU)),param_3);
  SetWindowText16(0x1000,(int)param_1 + 0xaU);
  return;
}



void __stdcall16far destroy_win_1008_9698(HWND16 param_1)

{
  DestroyWindow16(param_1);
  return;
}



BOOL16 __stdcall16far show_win_1008_96ae(ulong param_1,INT16 param_2,HWND16 param_3)

{
  BOOL16 BVar1;
  
  if (*(int *)((int)param_1 + 0x8) != 0x0) {
    BVar1 = ShowWindow16(param_3,param_2);
    return BVar1;
  }
  return 0x0;
}



void __stdcall16far win_ui_reg_class_1008_96d2(astruct_20 *param_1,HINSTANCE16 in_h_inst_2,WNDCLASS16 *in_wnd_class_3)

{
  BOOL16 BVar1;
  ATOM AVar2;
  undefined2 name_1c;
  undefined2 uStack26;
  undefined2 uStack24;
  undefined4 uStack22;
  undefined *puStack18;
  undefined2 uStack16;
  undefined2 uStack14;
  undefined2 uStack12;
  undefined4 uStack10;
  int iStack6;
  undefined2 uStack4;
  
  iStack6 = (int)param_1 + 0x5b;
  BVar1 = GetClassInfo16(in_h_inst_2,(SEGPTR)&name_1c,in_wnd_class_3);
  if (BVar1 == 0x0) {
    name_1c = *(undefined2 *)((int)param_1 + 0xc8);
    uStack26 = 0x5632;
    uStack24 = 0x1008;
    uStack22 = 0x40000;
    puStack18 = PTR_LOOP_1050_038c;
    uStack16 = *(undefined2 *)((int)param_1 + 0xc2);
    uStack14 = *(undefined2 *)((int)param_1 + 0xc4);
    uStack12 = *(undefined2 *)((int)param_1 + 0xc6);
    uStack10 = 0x0;
    uStack4 = param_1._2_2_;
    AVar2 = RegisterClass16((WNDCLASS16 *)s_tile2_bmp_1050_1538);
    if (AVar2 == 0x0) {
      fn_ptr_op_1000_24cd(0x0,&stack0xfffe);
    }
  }
  return;
}



void __stdcall16far create_window_ex_1008_9760(astruct *in_struct_1,undefined2 param_2)

{
  undefined4 uVar1;
  HWND16 window_handle;
  astruct *struct_1;
  LPCSTR class_name;
  
  class_name = (LPCSTR)((ulong)in_struct_1 >> 0x10);
  struct_1 = (astruct *)in_struct_1;
  if (struct_1->field_0x8 == 0x0) {
    uVar1 = struct_1->field_0xac;
    window_handle =
         CreateWIndowEx16(CONCAT22(struct_1,param_2),class_name,PTR_LOOP_1050_038c,
                          CONCAT22(struct_1->field_0xbc,struct_1->field_0xca),struct_1->field_0xba,struct_1->field_0xb8,
                          struct_1->field_0xb6,struct_1->field_0xb4,(HWND16)uVar1,(HMENU16)((ulong)uVar1 >> 0x10),0x39e,
                          (LPVOID)&USHORT_1050_1050);
    struct_1->field_0x8 = window_handle;
  }
  if (struct_1->field_0x8 == 0x0) {
    fn_ptr_op_1000_24cd(0x0,&stack0xfffe);
  }
  return;
}



void __stdcall16far begin_end_paint_1008_97c8(HWND16 param_1)

{
  PAINTSTRUCT16 local_22;
  
  BeginPaint16(param_1,&local_22);
  EndPaint16((HWND16)s_tile2_bmp_1050_1538,&local_22);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far
unk_win_op_1008_97f2(ulong *param_1,int *param_2,WPARAM16 param_3,uchar *param_4,uint param_5,HWND16 param_6)

{
  code **ppcVar1;
  BOOL16 BVar2;
  undefined2 uVar3;
  int iVar4;
  uint uVar5;
  UINT16 msg;
  UINT16 wparam;
  ushort unaff_SS;
  ulong uVar6;
  undefined uVar7;
  undefined uVar8;
  char cVar9;
  
  msg = (UINT16)param_1;
  wparam = (UINT16)((ulong)param_1 >> 0x10);
  if (param_5 == 0x2b) {
    if (*param_2 == 0x4) {
      win_ui_get_prop_op_1040_9566((int *)CONCAT22(param_3,param_2),(int)&PTR_LOOP_1050_1040);
    }
    else {
      ppcVar1 = (code **)((int)*param_1 + 0x70);
      (**ppcVar1)();
    }
    uVar5 = 0x1;
    goto LAB_1008_9a95;
  }
  uVar8 = (undefined)((ulong)param_1 >> 0x10);
  uVar7 = SUB41(param_1,0x0);
  if (param_5 < 0x2c) {
    param_6 = 0x1008;
    switch(param_5) {
    case 0x1:
      break;
    case 0x2:
      ppcVar1 = (code **)((int)*param_1 + 0x3c);
      (**ppcVar1)(0x1008,param_1);
      SetWindowLong16(0x1008,0x0,0x0);
      BVar2 = IsWindow16((HWND16)s_tile2_bmp_1050_1538);
      if (BVar2 != 0x0) {
        PostMessage16((HWND16)s_tile2_bmp_1050_1538,msg,wparam,0x11100c7);
      }
      break;
    case 0x3:
      ppcVar1 = (code **)((int)*param_1 + 0x54);
      (**ppcVar1)(0x8,uVar7,wparam,param_3,param_2);
      break;
    default:
      goto switchD_1008_9b30_caseD_4;
    case 0x5:
      ppcVar1 = (code **)((int)*param_1 + 0x58);
      (**ppcVar1)(0x8,uVar7,uVar8,param_3,param_2,param_4);
      break;
    case 0x7:
      ppcVar1 = (code **)((int)*param_1 + 0x50);
      (**ppcVar1)(0x8,param_1,param_4);
      break;
    case 0x8:
      ppcVar1 = (code **)((int)*param_1 + 0x74);
      (**ppcVar1)(0x8,param_1,param_4);
      break;
    case 0xd:
      ppcVar1 = (code **)((int)*param_1 + 0x84);
      iVar4 = (**ppcVar1)(0x8,uVar7,uVar8,param_2,CONCAT12(param_4._0_1_,param_3));
      goto LAB_1008_9ada;
    case 0xf:
      ppcVar1 = (code **)((int)*param_1 + 0x34);
      (**ppcVar1)(0x1008,param_1);
      break;
    case 0x10:
      ppcVar1 = (code **)((int)*param_1 + 0x38);
      uVar6 = (**ppcVar1)(0x1008,param_1);
      return uVar6;
    case 0x19:
      ppcVar1 = (code **)((int)*param_1 + 0x78);
      uVar3 = (**ppcVar1)(0x8,uVar7,uVar8,param_2,CONCAT12(param_4._0_1_,param_3));
      return CONCAT22(0x1050,uVar3);
    case 0x1c:
      ppcVar1 = (code **)((int)*param_1 + 0x30);
      (**ppcVar1)(0x8,param_1,param_4);
    }
  }
  else {
    cVar9 = (char)param_5;
    if (param_5 == 0x112) {
      if ((PTR_LOOP_1050_039a == (undefined *)0x0) &&
         (ppcVar1 = (code **)((int)*param_1 + 0x48), iVar4 = (**ppcVar1)(), iVar4 != 0x0)) {
        make_def_wnd_proc_1008_9ce6(msg,wparam,(UINT16)param_2,param_3,CONCAT13(0x1,CONCAT12(cVar9,param_4)),param_6);
      }
    }
    else {
      if (param_5 < 0x113) {
        if (param_5 == 0x86) {
          ppcVar1 = (code **)((int)*param_1 + 0x80);
          uVar6 = (**ppcVar1)();
          return uVar6;
        }
        if (param_5 < 0x87) {
          if (param_5 == 0x85) {
            ppcVar1 = (code **)((int)*param_1 + 0x7c);
            uVar6 = (**ppcVar1)();
            return uVar6;
          }
          if (param_5 < 0x86) {
            if (cVar9 == '7') {
              return (ulong)*(uint *)(msg + 0xc2);
            }
            if (cVar9 == 'A') {
              ppcVar1 = (code **)((int)*param_1 + 0x2c);
              (**ppcVar1)(param_6,param_1);
              goto switchD_1008_9b30_caseD_1;
            }
          }
switchD_1008_9b30_caseD_4:
          if ((param_5 < 0x400) || (0x7ffe < param_5)) {
            uVar6 = make_def_wnd_proc_1008_9ce6(msg,wparam,(UINT16)param_2,param_3,CONCAT22(param_5,param_4),param_6);
            return uVar6;
          }
          ppcVar1 = (code **)((int)*param_1 + 0x28);
          (**ppcVar1)((char)param_6,uVar7,uVar8,(char)param_2,param_3,CONCAT22(param_5,param_4));
        }
        else {
          if (param_5 == 0x100) {
            if (PTR_LOOP_1050_039a == (undefined *)0x0) {
              ppcVar1 = (code **)((int)*param_1 + 0x6c);
              (**ppcVar1)();
            }
          }
          else {
            if (param_5 == 0x102) {
              if (PTR_LOOP_1050_039a == (undefined *)0x0) {
                ppcVar1 = (code **)((int)*param_1 + 0x68);
                (**ppcVar1)();
              }
            }
            else {
              if (param_5 != 0x111) goto switchD_1008_9b30_caseD_4;
              if ((param_4 != PTR_LOOP_1050_039c) && (PTR_LOOP_1050_039a == (undefined *)0x0)) {
                if (param_2 == (int *)0x0) {
                  ppcVar1 = (code **)((int)*param_1 + 0x40);
                  (**ppcVar1)();
                }
                else {
                  ppcVar1 = (code **)((int)*param_1 + 0x44);
                  (**ppcVar1)();
                }
              }
            }
          }
        }
      }
      else {
        if (param_5 == 0x204) {
          if (PTR_LOOP_1050_039a == (undefined *)0x0) {
            ppcVar1 = (code **)((int)*param_1 + 0x60);
            (**ppcVar1)();
          }
        }
        else {
          if (param_5 < 0x205) {
            if (param_5 == 0x113) {
              if (_PTR_LOOP_1050_0388 != 0x0) {
                pass1_1008_932a(_PTR_LOOP_1050_0388,unaff_SS);
              }
            }
            else {
              if (param_5 == 0x117) {
                if (param_3 == 0x0) {
                  ppcVar1 = (code **)((int)*param_1 + 0x4c);
                  (**ppcVar1)();
                }
                else {
                  ppcVar1 = (code **)((int)*param_1 + 0x20);
                  (**ppcVar1)();
                }
              }
              else {
                if (param_5 != 0x201) goto switchD_1008_9b30_caseD_4;
                if (PTR_LOOP_1050_039a == (undefined *)0x0) {
                  ppcVar1 = (code **)((int)*param_1 + 0x5c);
                  (**ppcVar1)();
                }
              }
            }
          }
          else {
            if (param_5 == 0x210) {
              ppcVar1 = (code **)((int)*param_1 + 0x64);
              (**ppcVar1)();
            }
            else {
              if (param_5 == 0x30f) {
LAB_1008_9af8:
                ppcVar1 = (code **)((int)*param_1 + 0x8c);
                iVar4 = (**ppcVar1)(param_6,param_1);
LAB_1008_9ada:
                return (ulong)(long)iVar4;
              }
              if (param_5 == 0x311) {
                ppcVar1 = (code **)((int)*param_1 + 0x88);
                iVar4 = (**ppcVar1)();
                if (iVar4 != 0x0) goto LAB_1008_9af8;
              }
              else {
                if (param_5 != 0x3b9) goto switchD_1008_9b30_caseD_4;
                ppcVar1 = (code **)((int)*param_1 + 0x24);
                (**ppcVar1)();
              }
            }
          }
        }
      }
    }
  }
switchD_1008_9b30_caseD_1:
  uVar5 = 0x0;
LAB_1008_9a95:
  return (ulong)uVar5;
}



LRESULT __stdcall16far pass1_1008_9c16(UINT16 param_1,ulong param_2,ulong param_3,HWND16 param_4)

{
  LRESULT LVar1;
  
  LVar1 = make_def_wnd_proc_1008_9ce6
                    (param_1,(UINT16)param_2,(UINT16)(param_2 >> 0x10),(WPARAM16)param_3,
                     CONCAT22(0x85,(int)(param_3 >> 0x10)),param_4);
  return LVar1;
}



LRESULT __stdcall16far pass1_1008_9c30(UINT16 param_1,ulong param_2,ulong param_3,HWND16 param_4)

{
  LRESULT LVar1;
  
  LVar1 = make_def_wnd_proc_1008_9ce6
                    (param_1,(UINT16)param_2,(UINT16)(param_2 >> 0x10),(WPARAM16)param_3,
                     CONCAT22(0x86,(int)(param_3 >> 0x10)),param_4);
  return LVar1;
}



void __stdcall16far pass1_1008_9c4a(void)

{
  return;
}



void __stdcall16far pass1_1008_9c4e(void)

{
  return;
}



void __stdcall16far pass1_1008_9c52(void)

{
  return;
}



void __stdcall16far get_stock_obj_1008_9c56(INT16 param_1)

{
  GetStockObject16(param_1);
  return;
}



void __stdcall16far pass1_1008_9c60(ushort param_1,ushort param_2,ulong *param_3,int param_4)

{
  code **ppcVar1;
  
  if ((param_4 == 0xc7) && (param_3 != (ulong *)0x0)) {
    ppcVar1 = (code **)*param_3;
    (**ppcVar1)();
  }
  return;
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



BOOL16 __stdcall16far pass1_1008_9cc4(ulong param_1,int param_2)

{
  if (*(int *)((int)param_1 + 0x8) != param_2) {
    return 0x1;
  }
  return 0x0;
}



ushort __stdcall16far pass1_1008_9ce0(void)

{
  return 0x0;
}



LRESULT __stdcall16far
make_def_wnd_proc_1008_9ce6
          (UINT16 param_1,UINT16 param_2,UINT16 in_msg_3,WPARAM16 param_4,LPARAM param_5,HWND16 in_hwnd_5)

{
  LRESULT LVar1;
  
  LVar1 = DefWindowProc16(in_hwnd_5,in_msg_3,param_4,param_5);
  return LVar1;
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_9d36(uchar *param_1,uchar *param_2,ushort param_3,ushort param_4)

{
  uchar *puVar1;
  undefined2 uVar2;
  ushort *puVar3;
  astruct_43 *paVar4;
  ulong uVar5;
  int iStack4;
  
  struct_op_1018_4cda((int)param_1,(ushort)param_2,param_3);
  *(undefined2 *)(param_1 + 0x1c) = 0x389a;
  *(undefined2 *)(param_1 + 0x1e) = 0x1008;
  *(undefined2 *)(param_1 + 0x1c) = 0x3aa8;
  *(undefined2 *)(param_1 + 0x1e) = 0x1008;
  *(undefined2 *)(param_1 + 0x20) = 0x0;
  puVar3 = pass1_1008_3e38((ushort *)CONCAT22(param_2,param_1 + 0x52));
  puVar1 = (uchar *)((ulong)puVar3 >> 0x10);
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x9fb2;
  *(undefined2 *)(param_1 + 0x2) = 0x1008;
  *(undefined2 *)(param_1 + 0x1c) = 0x9fca;
  *(undefined2 *)(param_1 + 0x1e) = 0x1008;
  PTR_LOOP_1050_4230 = param_1;
  PTR_LOOP_1050_4232 = param_2;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,param_1 + 0x22),(WNDCLASS16 *)0x0,0x30);
  pass1_1018_4dce((ulong *)CONCAT22(param_2,param_1),0x1c0,puVar1,param_4);
  iStack4 = 0x0;
  do {
    paVar4 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,iStack4 + 0x1c0,param_4);
    *(int *)(param_1 + iStack4 * 0x4 + 0x22) = (int)paVar4;
    *(int *)(param_1 + iStack4 * 0x4 + 0x24) = (int)((ulong)paVar4 >> 0x10);
    iStack4 = iStack4 + 0x1;
  } while (iStack4 < 0xc);
  uVar5 = pass1_1008_4772(*(astruct_76 **)(param_1 + 0x22));
  uVar2 = (undefined2)(uVar5 >> 0x10);
  pass1_1008_3e76((ushort *)CONCAT22(param_2,param_1 + 0x52),0x0,(0x1e0 - *(int *)((int)uVar5 + 0x8)) / 0x2 - 0x32,
                  (0x280 - *(int *)((int)uVar5 + 0x4)) / 0x2);
  if (CONCAT22(param_2,param_1) == 0x0) {
    puVar1 = (uchar *)0x0;
    param_2 = (uchar *)0x0;
  }
  else {
    puVar1 = param_1 + 0x1c;
  }
  pass1_1008_9262((int)_PTR_LOOP_1050_0388,(ushort)((ulong)_PTR_LOOP_1050_0388 >> 0x10),0x50,CONCAT22(param_2,puVar1),
                  (uint)puVar1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1008_9e5a(astruct_11 *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  undefined2 *puVar4;
  undefined2 uVar6;
  astruct_464 *uVar5;
  undefined2 uVar7;
  undefined2 *puStack8;
  int iStack4;
  
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  uVar5 = (astruct_464 *)param_1;
  *(undefined2 *)param_1 = 0x9fb2;
  uVar5->field_0x2 = 0x1008;
  uVar5->field_0x1c = 0x9fca;
  uVar5->field_0x1e = 0x1008;
  if (_PTR_LOOP_1050_0388 != 0x0) {
    if (param_1 == (astruct_11 *)0x0) {
      puVar4 = (undefined2 *)0x0;
      uVar6 = 0x0;
    }
    else {
      puVar4 = &uVar5->field_0x1c;
      uVar6 = uVar7;
    }
    pass1_1008_92b2(_PTR_LOOP_1050_0388,0x50,CONCAT22(uVar6,puVar4));
  }
  iStack4 = 0x0;
  do {
    puVar1 = (undefined4 *)*(uint *)(&uVar5[0x1].field_0x0 + iStack4 * 0x4);
    uVar2 = (&uVar5[0x1].field_0x2)[iStack4 * 0x2];
    if ((uVar2 | (uint)puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    iStack4 = iStack4 + 0x1;
  } while (iStack4 < 0xc);
  if (param_1 == (astruct_11 *)0x0) {
    puVar4 = (undefined2 *)0x0;
    uVar7 = 0x0;
  }
  else {
    puVar4 = &uVar5->field_0x1c;
  }
  puStack8 = (undefined2 *)CONCAT22(uVar7,puVar4);
  *puStack8 = 0x389a;
  puVar4[0x1] = 0x1008;
  clenaup_win_ui_1018_4d22(param_1,0x1018);
  return;
}



void __stdcall16far pass1_1008_9f18(int param_1,ushort param_2,int param_3,ushort param_4)

{
  if (param_3 == 0x2) {
    pass1_1008_9f64(CONCAT22(param_2,param_1 + -0x1c));
    pass1_1010_1f62(param_4,CONCAT22(param_2,param_1 + -0x1c),0x2);
  }
  return;
}



ulong __stdcall16far pass1_1008_9f48(ulong param_1)

{
  astruct_134 *iVar1;
  int iVar2;
  ushort uVar3;
  
  uVar3 = (ushort)(param_1 >> 0x10);
  iVar1 = (astruct_134 *)param_1;
  iVar2 = iVar1->field_0x20 * 0x4;
  return CONCAT22(*(undefined2 *)(&iVar1[0x1].field_0x2 + iVar2),*(undefined2 *)(&iVar1[0x1].field_0x0 + iVar2));
}



void __stdcall16far pass1_1008_9f64(ulong param_1)

{
  int *piVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  piVar1 = (int *)(iVar2 + 0x20);
  *piVar1 = *piVar1 + 0x1;
  if (0xb < *(int *)(iVar2 + 0x20)) {
    *(undefined2 *)(iVar2 + 0x20) = 0x0;
  }
  return;
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



void pass1_1008_9fb2(ushort param_1,int param_2,ushort param_3,ushort param_4,uint param_5,byte param_6,byte param_7,
                    int param_8,int param_9,byte param_10)

{
  char *pcVar1;
  byte *pbVar2;
  byte bVar3;
  uint uVar4;
  code *pcVar5;
  byte bVar6;
  uint uVar7;
  uint uVar8;
  byte extraout_DL;
  uchar *puVar9;
  uchar *puVar10;
  uchar *extraout_DX;
  undefined2 extraout_DX_00;
  undefined2 uVar11;
  byte bVar12;
  bool bVar13;
  bool bVar14;
  astruct_79 *paVar15;
  
  *(undefined2 *)(param_8 + 0x1008) = (int)&USHORT_1050_1050;
  uVar8 = (uint)param_10;
  uVar4 = param_5 + 0xeff0;
  bVar12 = param_5 < 0x1010 || uVar4 < uVar8;
  uVar7 = uVar4 - uVar8;
  pcVar5 = (code *)swi(0x4);
  if (SBORROW2(param_5,0x1010) != SBORROW2(uVar4,uVar8)) {
    (*pcVar5)();
    param_7 = extraout_DL;
  }
  bVar6 = (byte)((char)(uVar7 + 0xeff0) - bVar12) % 0x1d;
  pcVar1 = (char *)(param_8 + param_9);
  *pcVar1 = *pcVar1 + param_7 + (uVar7 < 0x1010 || uVar7 + 0xeff0 < (uint)bVar12);
  pbVar2 = (byte *)(param_8 + param_9);
  bVar13 = *pbVar2 < param_7 || (byte)(*pbVar2 - param_7) < (0xb1 < bVar6);
  *pbVar2 = (*pbVar2 - param_7) - (0xb1 < bVar6);
  pbVar2 = (byte *)(param_8 + 0x18);
  bVar14 = *pbVar2 < param_6 || (byte)(*pbVar2 - param_6) < bVar13;
  *pbVar2 = (*pbVar2 - param_6) - bVar13;
  pbVar2 = (byte *)(param_8 + param_9 + 0x89f);
  bVar12 = *pbVar2;
  bVar3 = *pbVar2 + bVar6 + 0x4e;
  *pbVar2 = bVar3 + bVar14;
  pcVar1 = (char *)(param_8 + param_9);
  *pcVar1 = *pcVar1 + (char)param_8 + (CARRY1(bVar12,bVar6 + 0x4e) || CARRY1(bVar3,bVar14));
  pbVar2 = (byte *)(param_8 + param_9);
  *pbVar2 = *pbVar2 | param_7;
  paVar15 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_3,param_2),param_4);
  puVar9 = (uchar *)((ulong)paVar15 >> 0x10);
  uVar8 = 0x0;
  *(undefined4 *)(param_2 + 0xa) = 0x0;
  *(undefined4 *)(param_2 + 0x410) = 0x0;
  *(undefined2 *)(param_2 + 0x414) = 0x0;
  *(undefined2 *)(param_2 + 0x416) = 0x0;
  *(undefined2 *)(param_2 + 0x418) = 0x0;
  *(undefined2 *)(param_2 + 0x41a) = 0x0;
  *(undefined2 *)(param_2 + 0x41c) = 0x0;
  *(undefined2 *)(param_2 + 0x41e) = 0x0;
  *(undefined2 *)CONCAT22(param_3,param_2) = 0xad92;
  *(undefined2 *)(param_2 + 0x2) = 0x1008;
  mem_op_1000_179c(0xc,puVar9,0x1000);
  puVar10 = (uchar *)((uint)puVar9 | uVar8);
  if (puVar10 == (uchar *)0x0) {
    *(undefined4 *)(param_2 + 0xa) = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar9,uVar8));
    *(uint *)(param_2 + 0xa) = uVar8;
    *(uchar **)(param_2 + 0xc) = extraout_DX;
    puVar10 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar10,0x1000);
  if (((uint)puVar10 | uVar8) == 0x0) {
    uVar8 = 0x0;
    uVar11 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar10,uVar8));
    uVar11 = extraout_DX_00;
  }
  *(uint *)(param_2 + 0x410) = uVar8;
  *(undefined2 *)(param_2 + 0x412) = uVar11;
  return;
}



void __stdcall16far struct_1008_9fd2(astruct_79 *param_1,astruct_79 *param_2,ushort param_3)

{
  uint uVar1;
  uchar *puVar2;
  uchar *puVar3;
  uchar *extraout_DX;
  ushort extraout_DX_00;
  ushort uVar4;
  astruct_79 *paVar5;
  
  paVar5 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  puVar2 = (uchar *)((ulong)paVar5 >> 0x10);
  uVar1 = 0x0;
  *(undefined4 *)(param_1 + 0x1) = 0x0;
  *(undefined4 *)(param_1 + 0x68) = 0x0;
  *(undefined2 *)&param_1[0x68].field_0x4 = 0x0;
  *(undefined2 *)((int)&param_1[0x68].field_0x4 + 0x2) = 0x0;
  param_1[0x68].field_0x8 = 0x0;
  ((astruct_79 *)(param_1 + 0x69))->field_0x0 = 0x0;
  param_1[0x69].field_0x2 = 0x0;
  *(undefined2 *)&param_1[0x69].field_0x4 = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0xad92;
  param_1->field_0x2 = 0x1008;
  mem_op_1000_179c(0xc,puVar2,0x1000);
  puVar3 = (uchar *)((uint)puVar2 | uVar1);
  if (puVar3 == (uchar *)0x0) {
    *(undefined4 *)(param_1 + 0x1) = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar2,uVar1));
    ((astruct_79 *)(param_1 + 0x1))->field_0x0 = uVar1;
    param_1[0x1].field_0x2 = (ushort)extraout_DX;
    puVar3 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar3,0x1000);
  if (((uint)puVar3 | uVar1) == 0x0) {
    uVar1 = 0x0;
    uVar4 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar3,uVar1));
    uVar4 = extraout_DX_00;
  }
  ((astruct_79 *)(param_1 + 0x68))->field_0x0 = uVar1;
  param_1[0x68].field_0x2 = uVar4;
  return;
}



void __stdcall16far pass1_1008_a086(ushort *param_1,ushort param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  astruct_465 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_465 *)param_1;
  *param_1 = 0xad92;
  iVar4->field_0x2 = 0x1008;
  puVar1 = iVar4->field_0xa;
  uVar2 = iVar4->field_0xc;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar4->field_0x410;
  uVar2 = iVar4->field_0x412;
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
post_win_msg_1008_a0e4
          (astruct_67 *param_1,long param_2,int param_3,ushort param_4,ulong param_5,int param_6,HWND16 param_7,
          ushort param_8)

{
  undefined4 *puVar1;
  code **ppcVar2;
  uint uVar3;
  bool bVar4;
  astruct_68 *puVar4;
  astruct_66 *uVar5;
  uint extraout_DX;
  uint uVar7;
  astruct_67 *iVar7;
  astruct_67 *uVar6;
  astruct_99 *paStack14;
  undefined local_a [0x8];
  
  uVar6 = (astruct_67 *)((ulong)param_1 >> 0x10);
  iVar7 = (astruct_67 *)param_1;
  pass1_1008_5784((ulong *)CONCAT22(param_8,local_a),(ulong)iVar7->field_0xa);
  bVar4 = false;
  do {
    puVar4 = (astruct_68 *)local_a;
    pass1_1008_5b12(puVar4,param_8);
    if ((extraout_DX | (uint)puVar4) == 0x0) goto LAB_1008_a146;
  } while (puVar4->field_0x4 != param_6);
  puVar4->field_0xc = puVar4->field_0xc + param_3;
  puVar4->field_0xe = puVar4->field_0xe + param_2;
  bVar4 = true;
LAB_1008_a146:
  if (!bVar4) {
    param_7 = 0x1000;
    paStack14 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_03a0);
    uVar7 = (uint)((ulong)paStack14 >> 0x10);
    uVar3 = (uint)paStack14;
    if ((uVar7 | uVar3) == 0x0) {
      paStack14 = (astruct_99 *)0x0;
    }
    else {
      paStack14->field_0x0 = 0x389a;
      *(undefined2 *)(uVar3 + 0x2) = 0x1008;
      *(int *)(uVar3 + 0x4) = param_6;
      *(ulong *)(uVar3 + 0x6) = param_5;
      *(ushort *)(uVar3 + 0xa) = param_4;
      *(int *)(uVar3 + 0xc) = param_3;
      *(long *)(uVar3 + 0xe) = param_2;
      paStack14->field_0x0 = 0xad8e;
      *(undefined2 *)(uVar3 + 0x2) = 0x1008;
    }
    puVar1 = iVar7->field_0xa;
    ppcVar2 = (code **)((int)*iVar7->field_0xa + 0x8);
    (**ppcVar2)(0x1000,(char)puVar1,(int)((ulong)puVar1 >> 0x10),(int)paStack14,(int)((ulong)paStack14 >> 0x10));
  }
  if (param_6 == 0x14) {
    PostMessage16(param_7,0x0,0x0,0x11100fc);
  }
  return;
}
