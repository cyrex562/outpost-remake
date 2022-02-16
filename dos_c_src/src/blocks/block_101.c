


void __stdcall16far win_dlg_op_1038_c95e(ulong param_1,int param_2)

{
  undefined4 uVar1;
  UINT16 UVar2;
  int iVar3;
  undefined2 uVar4;
  HWND16 unaff_CS;
  
  iVar3 = (int)param_1;
  uVar4 = (undefined2)(param_1 >> 0x10);
  if (param_2 == 0x0) {
    uVar1 = *(undefined4 *)(iVar3 + 0x8e);
    *(undefined2 *)((int)uVar1 + 0xa) = 0x0;
  }
  else {
    UVar2 = IsDlgButtonChecked(unaff_CS,0xfac);
    if (UVar2 == 0x0) {
      unaff_CS = (HWND16)s_tile2_bmp_1050_1538;
      UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,0xfad);
      if (UVar2 == 0x0) {
        unaff_CS = (HWND16)s_tile2_bmp_1050_1538;
        UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,0xfae);
        if (UVar2 == 0x0) {
          unaff_CS = (HWND16)s_tile2_bmp_1050_1538;
          UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,0xfaf);
          if (UVar2 == 0x0) {
            unaff_CS = (HWND16)s_tile2_bmp_1050_1538;
            UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,0xfb0);
            if (UVar2 != 0x0) {
              uVar1 = *(undefined4 *)(iVar3 + 0x8e);
              *(undefined2 *)((int)uVar1 + 0xa) = 0x5;
            }
          }
          else {
            uVar1 = *(undefined4 *)(iVar3 + 0x8e);
            *(undefined2 *)((int)uVar1 + 0xa) = 0x4;
          }
        }
        else {
          uVar1 = *(undefined4 *)(iVar3 + 0x8e);
          *(undefined2 *)((int)uVar1 + 0xa) = 0x3;
        }
      }
      else {
        uVar1 = *(undefined4 *)(iVar3 + 0x8e);
        *(undefined2 *)((int)uVar1 + 0xa) = 0x2;
      }
    }
    else {
      uVar1 = *(undefined4 *)(iVar3 + 0x8e);
      *(undefined2 *)((int)uVar1 + 0xa) = 0x1;
      unaff_CS = (HWND16)s_tile2_bmp_1050_1538;
    }
  }
  DestroyWindow16(unaff_CS);
  PTR_LOOP_1050_5b80 = (undefined *)0x0;
  return;
}



astruct_18 * __stdcall16far pass1_1038_ca46(astruct_18 *param_1,byte param_2)

{
  pass1_1038_c80a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * __stdcall16far
pass1_1038_cad8(astruct_57 *param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5)

{
  astruct_709 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x1cb,param_2);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_709 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  *(undefined2 *)param_1 = 0xcc9a;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2c,param_5,param_3,param_4);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  iVar1->field_0x74 = 0x0;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_cb30(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xcc9a;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far show_win_1038_cb5c(astruct_1 *param_1)

{
  undefined4 uVar1;
  ushort uVar2;
  ushort in_DX;
  uchar *puVar3;
  uint uVar4;
  undefined2 uVar5;
  WNDCLASS16 *unaff_SS;
  undefined2 *puVar6;
  undefined2 *puVar7;
  int iStack10;
  
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  uVar2 = pass1_1008_eb6e();
  for (iStack10 = 0x0; iStack10 < (int)uVar2; iStack10 = iStack10 + 0x1) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x8e);
    puVar6 = (undefined2 *)pass1_1008_eb5c((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),iStack10);
    puVar3 = (uchar *)((ulong)puVar6 >> 0x10);
    puVar7 = puVar6;
    mem_op_1000_179c(0x42,puVar3,0x1000);
    uVar4 = (uint)((ulong)puVar7 >> 0x10);
    in_DX = uVar4 | (uint)(astruct_160 *)puVar7;
    if (puVar7 != (undefined2 *)0x0) {
      pass1_1008_3bd6((astruct_160 *)puVar7,uVar4,0x0,CONCAT22(*puVar6,*(undefined2 *)((int)puVar6 + 0x2)),0x101,
                      0xff0100,CONCAT22(*(undefined2 *)((int)param_1 + 0x6),*(undefined2 *)((int)puVar6 + 0x4)),in_DX,
                      (ushort)unaff_SS);
    }
  }
  win_1008_5c7c(_PTR_LOOP_1050_02a0,0x90001,unaff_SS,uVar2,in_DX);
  ShowWindow16(0x1008,0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far destroy_window_1038_cc00(int param_1,UINT16 param_2,UINT16 param_3,ULONG param_4)

{
  ushort uVar1;
  uchar *in_DX;
  int unaff_DI;
  WNDCLASS16 *unaff_SS;
  int iVar2;
  
  uVar1 = param_4._2_2_ - 0x1cd;
  if (uVar1 == 0x0) {
    iVar2 = 0x1;
  }
  else {
    uVar1 = param_4._2_2_ - 0x1ce;
    if (uVar1 == 0x0) {
      iVar2 = 0x2;
    }
    else {
      uVar1 = param_4._2_2_ - 0x1cf;
      if (uVar1 == 0x0) {
        iVar2 = 0x3;
      }
      else {
        uVar1 = param_4._2_2_ - 0x1d0;
        if (uVar1 == 0x0) {
          iVar2 = 0x4;
        }
        else {
          uVar1 = param_4._2_2_ - 0x1d1;
          if (uVar1 != 0x0) {
            post_win_msg_1040_7b3c
                      ((ulong *)CONCAT22(param_2,param_1),param_3,(ushort)param_4,param_4._2_2_,(int)&PTR_LOOP_1050_1040
                      );
            return;
          }
          iVar2 = 0x5;
        }
      }
    }
  }
  pass1_1008_eb74(*(undefined4 *)(param_1 + 0x8e),iVar2,in_DX,unaff_DI,(ushort)unaff_SS);
  if (uVar1 != 0x0) {
    win_1008_5c7c(_PTR_LOOP_1050_02a0,CONCAT22(uVar1,0x1),unaff_SS,uVar1,(ushort)in_DX);
    DestroyWindow16(0x1008);
  }
  return;
}



astruct_18 * __stdcall16far pass1_1038_cc74(astruct_18 *param_1,byte param_2)

{
  pass1_1038_cb30(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_cd06(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               int param_7,ushort param_8)

{
  astruct_710 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfcc,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_710 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  *(undefined2 *)param_1 = 0xcf00;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x42,param_8,param_6,param_7);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_cd5c(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xcf00;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



void __stdcall16far destroy_window_1038_cd88(astruct_1 *param_1)

{
  undefined2 unaff_SS;
  
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16((HWND16)&PTR_LOOP_1050_1040,0x5);
  *(undefined2 *)((int)param_1 + 0x92) = 0x1;
  unk_win_msg_op_1008_9510((int *)((ulong)param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x92)),0x1008,unaff_SS);
  DestroyWindow16(0x1008);
  return;
}



void __stdcall16far check_dlg_btn_checked_1038_cdd6(ulong param_1,int param_2,HWND16 param_3)

{
  undefined4 uVar1;
  UINT16 UVar2;
  astruct_61 *iVar3;
  undefined2 uVar3;
  
  iVar3 = (astruct_61 *)param_1;
  uVar3 = (undefined2)(param_1 >> 0x10);
  if (param_2 == 0x0) {
    uVar1 = iVar3->field_0x8e;
    *(undefined2 *)((int)uVar1 + 0xa) = 0x0;
  }
  else {
    UVar2 = IsDlgButtonChecked(param_3,0x182e);
    if (UVar2 == 0x0) {
      UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,0x182f);
      if (UVar2 == 0x0) {
        UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,0x1829);
        if (UVar2 == 0x0) {
          UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,0x182a);
          if (UVar2 == 0x0) {
            UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,0x182c);
            if (UVar2 == 0x0) {
              UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,0x182d);
              if (UVar2 != 0x0) {
                uVar1 = iVar3->field_0x8e;
                *(undefined2 *)((int)uVar1 + 0xa) = 0x7;
              }
            }
            else {
              uVar1 = iVar3->field_0x8e;
              *(undefined2 *)((int)uVar1 + 0xa) = 0x6;
            }
          }
          else {
            uVar1 = iVar3->field_0x8e;
            *(undefined2 *)((int)uVar1 + 0xa) = 0x4;
          }
        }
        else {
          uVar1 = iVar3->field_0x8e;
          *(undefined2 *)((int)uVar1 + 0xa) = 0x3;
        }
      }
      else {
        uVar1 = iVar3->field_0x8e;
        *(undefined2 *)((int)uVar1 + 0xa) = 0x2;
      }
    }
    else {
      uVar1 = iVar3->field_0x8e;
      *(undefined2 *)((int)uVar1 + 0xa) = 0x1;
    }
  }
  iVar3->field_0x92 = 0x0;
  return;
}



astruct_18 * __stdcall16far pass1_1038_ceda(astruct_18 *param_1,byte param_2)

{
  pass1_1038_cd5c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far make_proc_inst_1038_cf6c(ushort *param_1,uchar *param_2,LPVOID param_3)

{
  LPVOID pvVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar2 + 0x2) = 0x1008;
  *(undefined4 *)(iVar2 + 0x4) = 0x0;
  *(undefined2 *)(iVar2 + 0x8) = 0x0;
  *param_1 = 0xd23e;
  *(undefined2 *)(iVar2 + 0x2) = (int)&PTR_LOOP_1050_1038;
  _PTR_LOOP_1050_5bc8 = param_1;
  pvVar1 = MakeProcInstance16(param_3,(HANDLE16)PTR_LOOP_1050_038c);
  *(LPVOID *)(iVar2 + 0x4) = pvVar1;
  *(uchar **)(iVar2 + 0x6) = param_2;
  PTR_LOOP_1050_5bcc = (undefined *)MakeProcInstance16((LPVOID)s_tile2_bmp_1050_1538,(HANDLE16)PTR_LOOP_1050_038c);
  PTR_LOOP_1050_5bce = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far free_proc_inst_1038_cfda(ushort *param_1,LPVOID param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0xd23e;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  FreeProcInstance16(param_2);
  FreeProcInstance16((LPVOID)s_tile2_bmp_1050_1538);
  *(undefined4 *)(iVar1 + 0x4) = 0x0;
  *param_1 = 0x389a;
  *(undefined2 *)(iVar1 + 0x2) = 0x1008;
  return;
}



// WARNING: Unable to use type for symbol var8

long __stdcall16far
call_win_proc_1038_d020(HWND16 win_handle_1,u32 param_2,LPARAM l_param,u16 param_4,HWND16 win_handle_2)

{
  HANDLE16 handle_1;
  HANDLE16 handle_2;
  uint var1;
  LRESULT lresult;
  i32 var5;
  u32 *var6;
  long var7;
  undefined2 var8;
  code **fn_ptr_1;
  undefined2 var2;
  undefined2 var3;
  undefined2 var4;
  WPARAM16 w_param;
  
  var4 = SUB42(&USHORT_1050_1050,0x0);
  var3 = l_param._2_2_;
  handle_1 = GetProp16(win_handle_2,(LPCSTR)s_procHi_1050_5bd7);
  var2 = l_param._2_2_;
  handle_2 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_procLo_1050_5bd0);
  var7 = CONCAT22(handle_1,handle_2);
  var8 = l_param._2_2_;
  handle_1 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisHi_1050_5be5);
  handle_2 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisLo_1050_5bde);
  var6 = (u32 *)CONCAT22(handle_1,handle_2);
  w_param = (WPARAM16)(param_2 >> 0x10);
  if ((handle_1 | handle_2) != 0x0) {
    var5 = 0x0;
    if ((int)l_param == 0x19) {
      fn_ptr_1 = (code **)((int)*var6 + 0x34);
      var5 = (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,(char)handle_2,handle_1,win_handle_1,param_2,l_param._2_2_,var8,
                          var2,var3,var4);
    }
    else {
      if ((int)l_param == 0x86) {
        fn_ptr_1 = (code **)((int)*var6 + 0x20);
        var1 = (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,handle_2,handle_1,w_param);
        goto LAB_1038_d10e;
      }
      if (((int)l_param == 0x112) && ((w_param & 0xfff0) == 0xf140)) {
        lresult = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x112f140);
        var1 = (uint)((int)lresult == 0x0);
        goto LAB_1038_d10e;
      }
    }
    if (var5 != 0x0) {
      return var5;
    }
  }
  if (var7 != 0x0) {
    lresult = CallWindowProc16((LPVOID)s_tile2_bmp_1050_1538,win_handle_1,(UINT16)param_2,w_param,l_param);
    return lresult;
  }
  var1 = 0x0;
LAB_1038_d10e:
  return (long)(int)var1;
}



void __stdcall16far win_prop_op_1038_d118(ulong param_1,ulong param_2,ushort param_3,ushort param_4,HWND16 param_5)

{
  code **ppcVar1;
  undefined4 uVar2;
  char cVar3;
  HANDLE16 HVar4;
  HANDLE16 HVar5;
  ushort uVar6;
  ushort uVar7;
  undefined2 uVar8;
  undefined4 *puStack6;
  
  uVar8 = SUB42(&USHORT_1050_1050,0x0);
  uVar7 = param_3;
  HVar4 = GetProp16(param_5,(LPCSTR)s_thisHi_1050_5bf3);
  uVar6 = param_3;
  HVar5 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisLo_1050_5bec);
  puStack6 = (undefined4 *)CONCAT22(HVar4,HVar5);
  if (param_2._2_2_ == 0x30) {
    if ((LPCSTR)param_2 == (LPCSTR)0x0) {
      return;
    }
    SetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)param_2,0x5c06);
    return;
  }
  if (param_2 < 0x310000) {
    cVar3 = (char)(param_2 >> 0x10);
    if (cVar3 == '\x02') {
      if ((HVar4 | HVar5) != 0x0) {
        uVar2 = *puStack6;
        ppcVar1 = (code **)uVar2 + 0x6;
        (**ppcVar1)((int)s_tile2_bmp_1050_1538,HVar5,HVar4,param_1,param_2,uVar6,uVar7,uVar8);
        if (puStack6 != (undefined4 *)0x0) {
          ppcVar1 = (code **)uVar2;
          (**ppcVar1)((int)s_tile2_bmp_1050_1538,HVar5,HVar4,0x1);
        }
      }
      HVar4 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)0x5bfa);
      if (HVar4 == 0x0) {
        return;
      }
      DeleteObject16((HGDIOBJ16)s_tile2_bmp_1050_1538);
      RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)0x5c00);
      return;
    }
    if (cVar3 == '\x06') {
      if (((LPCSTR)param_2 != (LPCSTR)((int)&PTR_LOOP_1050_0000 + 0x1)) &&
         ((LPCSTR)param_2 != (LPCSTR)&PTR_LOOP_1050_0002)) {
        uVar2 = *(undefined4 *)&PTR_LOOP_1050_5bc8;
        *(undefined2 *)((int)uVar2 + 0x8) = 0x0;
        return;
      }
      uVar2 = *(undefined4 *)&PTR_LOOP_1050_5bc8;
      *(ushort *)((int)uVar2 + 0x8) = param_3;
      return;
    }
  }
  if ((HVar4 | HVar5) != 0x0) {
    ppcVar1 = (code **)((int)*puStack6 + 0xc);
    (**ppcVar1)((int)s_tile2_bmp_1050_1538,HVar5,HVar4,param_1,param_2);
  }
  return;
}



astruct_18 * __stdcall16far pass1_1038_d218(astruct_18 *param_1,byte param_2,ushort param_3)

{
  free_proc_inst_1038_cfda(&param_1->field_0x0,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_57 * __stdcall16far pass1_1038_d242(astruct_57 *param_1,ushort param_2)

{
  undefined2 uVar1;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x13e,param_2);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined2 *)param_1 = 0xd6ea;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  *(undefined2 *)((int)param_1 + 0x74) = 0x1;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_d276(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xd6ea;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1038_d2a2(astruct_1 *param_1)

{
  astruct_160 *rect;
  int iVar1;
  BOOL16 BVar2;
  uchar *in_DX;
  uchar *puVar3;
  ushort uVar4;
  int unaff_DI;
  undefined2 uVar5;
  HWND16 hwnd;
  HWND16 hwnd_00;
  WNDCLASS16 *unaff_SS;
  char *pcVar6;
  LRESULT LVar7;
  WPARAM16 w_param;
  undefined2 uVar8;
  undefined2 uVar9;
  ushort uVar10;
  undefined2 local_16;
  undefined2 uStack20;
  undefined2 uStack18;
  undefined2 uStack16;
  undefined4 uStack14;
  int iStack10;
  undefined4 uStack8;
  int iStack4;
  
  hwnd = (HWND16)&PTR_LOOP_1050_1040;
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  iStack4 = 0x7;
  for (iStack10 = 0x0; uVar5 = (undefined2)((ulong)param_1 >> 0x10), iStack10 < iStack4; iStack10 = iStack10 + 0x1) {
    unaff_DI = iStack10 * 0xc;
    local_16 = *(undefined2 *)(unaff_DI + 0x5c0c);
    uStack20 = *(undefined2 *)(unaff_DI + 0x5c0e);
    uStack18 = 0x1;
    uStack16 = 0x1;
    rect = (astruct_160 *)&local_16;
    MapDialogRect16(hwnd,(RECT16 *)rect);
    hwnd_00 = 0x1000;
    mem_op_1000_179c(0x42,in_DX,0x1000);
    puVar3 = (uchar *)((uint)in_DX | (uint)rect);
    if (puVar3 == (uchar *)0x0) {
      rect = (astruct_160 *)0x0;
      in_DX = (uchar *)0x0;
    }
    else {
      hwnd_00 = 0x1008;
      pass1_1008_3bd6(rect,(ushort)in_DX,0x1,CONCAT22(local_16,uStack20),0x104,0x1020103,
                      CONCAT22(*(undefined2 *)((int)param_1 + 0x6),*(undefined2 *)(unaff_DI + 0x5c10)),(ushort)puVar3,
                      (ushort)unaff_SS);
      in_DX = puVar3;
    }
    uStack8 = CONCAT22(in_DX,rect);
    hwnd = hwnd_00;
    if (*(int *)(iStack10 * 0xc + 0x5c12) == 0x0) {
      hwnd = (HWND16)s_tile2_bmp_1050_1538;
      EnableWindow16(hwnd_00,0x0);
    }
  }
  uVar10 = 0x86;
  uStack14 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x9,(ushort)unaff_SS,in_DX,unaff_DI);
  uVar4 = (ushort)((ulong)uStack14 >> 0x10);
  iVar1 = pass1_1010_659a((ulong)uStack14,uVar10,(ushort)unaff_SS);
  if (iVar1 == 0x0) {
    GetDlgItem16(0x1010,0x14a);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
    uVar8 = 0xc;
    uVar9 = 0x144;
    w_param = 0x0;
    pcVar6 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    LVar7 = SendDlgItemMessage16(0x1010,(INT16)pcVar6,(UINT16)((ulong)pcVar6 >> 0x10),w_param,CONCAT22(uVar9,uVar8));
    uVar4 = (ushort)((ulong)LVar7 >> 0x10);
  }
  move_win_1040_826c(param_1,-0x1,0xffff);
  BVar2 = ShowWindow16((HWND16)&PTR_LOOP_1050_1040,0x5);
  win_1008_5c7c(_PTR_LOOP_1050_02a0,0x9a0001,unaff_SS,BVar2,uVar4);
  *(BOOL16 *)((int)param_1 + 0x8c) = BVar2;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far unk_win_ui_op_1038_d400(UCHAR param_1,UINT16 param_2,UINT16 param_3,ULONG param_4)

{
  int iVar1;
  ushort uVar2;
  BOOL16 BVar3;
  ushort in_DX;
  uchar *puVar4;
  int unaff_DI;
  HWND16 hwnd;
  HWND16 hwnd_00;
  WNDCLASS16 *unaff_SS;
  ushort *puVar5;
  LRESULT LVar6;
  char *pcVar7;
  undefined in_stack_00000005;
  WPARAM16 WVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  ushort uVar11;
  undefined local_c [0x4];
  int iStack8;
  undefined4 uStack6;
  
  uStack6 = 0x0;
  iStack8 = 0x0;
  switch(param_4._2_2_) {
  case 0x145:
    GetDlgItem16((HWND16)&PTR_LOOP_1050_1038,0x146);
    uVar2 = EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    uStack6 = 0x13f0647;
    uVar11 = 0x1f1;
    goto LAB_1038_d490;
  case 0x146:
    uStack6 = 0x1400648;
    puVar5 = pass1_1008_941a((ushort *)CONCAT22(unaff_SS,local_c),0x1,0xc4);
    puVar4 = (uchar *)((ulong)puVar5 >> 0x10);
    win_1008_5c9e(_PTR_LOOP_1050_02a0,(ulong *)CONCAT22(unaff_SS,local_c),local_c,puVar4,unaff_SS);
    uVar2 = 0x86;
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x9,(ushort)unaff_SS,puVar4,unaff_DI);
    pass1_1010_6604((ulong)puVar5,uVar2,(ushort)unaff_SS);
    GetDlgItem16(0x1010,0x145);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
    uVar9 = 0xc;
    uVar10 = 0x13f;
    WVar8 = 0x0;
    pcVar7 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    LVar6 = SendDlgItemMessage16(0x1010,(INT16)pcVar7,(UINT16)((ulong)pcVar7 >> 0x10),WVar8,CONCAT22(uVar10,uVar9));
    puVar4 = (uchar *)((ulong)LVar6 >> 0x10);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x146);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
    iVar1 = pass1_1010_659a((ulong)puVar5,0x86,(ushort)unaff_SS);
    if (iVar1 == 0x0) {
      GetDlgItem16(0x1010,0x14a);
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
      uVar9 = 0xc;
      uVar10 = 0x144;
      WVar8 = 0x0;
      pcVar7 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
      LVar6 = SendDlgItemMessage16(0x1010,(INT16)pcVar7,(UINT16)((ulong)pcVar7 >> 0x10),WVar8,CONCAT22(uVar10,uVar9));
      puVar4 = (uchar *)((ulong)LVar6 >> 0x10);
    }
    hwnd = 0x1010;
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,(ushort)unaff_SS,puVar4,unaff_DI);
    if (*(int *)((int)puVar5 + 0x20) != 0x0) {
      hwnd = (HWND16)s_tile2_bmp_1050_1538;
      PostMessage16(0x1010,0x0,0x0,0x11100af);
    }
    break;
  case 0x147:
    GetDlgItem16((HWND16)&PTR_LOOP_1050_1038,0x148);
    uVar2 = EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    uStack6 = 0x1410647;
    uVar11 = 0x1f5;
    goto LAB_1038_d490;
  case 0x148:
    GetDlgItem16((HWND16)&PTR_LOOP_1050_1038,0x149);
    uVar2 = EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    uStack6 = 0x1420647;
    uVar11 = 0x1f2;
LAB_1038_d490:
    hwnd = 0x1008;
    win_1008_5c5c(unaff_SS,uVar2,in_DX,_PTR_LOOP_1050_02a0,uVar11);
    break;
  case 0x149:
    uStack6 = 0x1430648;
    PostMessage16((HWND16)&PTR_LOOP_1050_1038,0x0,0x0,0x11100b8);
    hwnd = (HWND16)s_tile2_bmp_1050_1538;
    DestroyWindow16((HWND16)s_tile2_bmp_1050_1538);
    break;
  case 0x14a:
    GetDlgItem16((HWND16)&PTR_LOOP_1050_1038,0x145);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    uVar9 = 0xc;
    uVar10 = 0x140;
    WVar8 = 0x0;
    pcVar7 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    hwnd = (HWND16)s_tile2_bmp_1050_1538;
    SendDlgItemMessage16(0x1010,(INT16)pcVar7,(UINT16)((ulong)pcVar7 >> 0x10),WVar8,CONCAT22(uVar10,uVar9));
    break;
  case 0x14b:
    GetDlgItem16((HWND16)&PTR_LOOP_1050_1038,0x147);
    hwnd = (HWND16)s_tile2_bmp_1050_1538;
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    break;
  default:
    post_win_msg_1040_7b3c
              ((ulong *)CONCAT22(param_2,CONCAT11(in_stack_00000005,param_1)),param_3,(ushort)param_4,param_4._2_2_,
               (int)&PTR_LOOP_1050_1040);
    return;
  }
  hwnd_00 = hwnd;
  if ((uStack6._2_2_ != 0x0) && ((int)uStack6 != 0x0)) {
    hwnd_00 = (HWND16)s_tile2_bmp_1050_1538;
    BVar3 = IsWindow16(hwnd);
    if (BVar3 != 0x0) {
      WVar8 = 0x0;
      uVar9 = 0xc;
      pcVar7 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
      hwnd_00 = (HWND16)s_tile2_bmp_1050_1538;
      SendDlgItemMessage16(0x1010,(INT16)pcVar7,(UINT16)((ulong)pcVar7 >> 0x10),WVar8,CONCAT22(uStack6._2_2_,uVar9));
    }
  }
  if (iStack8 != 0x0) {
    PostMessage16(hwnd_00,0x0,0x0,CONCAT22(0x111,iStack8));
  }
  return;
}



astruct_18 * __stdcall16far pass1_1038_d6c4(astruct_18 *param_1,byte param_2)

{
  pass1_1038_d276(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * __stdcall16far
pass1_1038_d756(astruct_57 *param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5)

{
  code **ppcVar1;
  astruct_711 *iVar2;
  undefined2 uVar2;
  ushort *puVar3;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x11b,param_2);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_711 *)param_1;
  iVar2->field_0x8e = 0x0;
  iVar2->field_0x90 = 0x0;
  iVar2->field_0x92 = (undefined4 *)0x0;
  iVar2->field_0x96 = 0x0;
  *(undefined2 *)param_1 = 0xe0d4;
  iVar2->field_0x2 = (int)&PTR_LOOP_1050_1038;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,param_3,param_4);
  *(int *)&iVar2->field_0x92 = (int)puVar3;
  *(undefined2 *)((int)&iVar2->field_0x92 + 0x2) = (int)((ulong)puVar3 >> 0x10);
  ppcVar1 = (code **)((int)*iVar2->field_0x92 + 0x4);
  (**ppcVar1)();
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_d7d0(astruct_18 *param_1,ushort param_2)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  param_1->field_0x0 = 0xe0d4;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  if (*(int *)(iVar1 + 0x90) != 0x0) {
    pass1_1010_1ea6(_PTR_LOOP_1050_02a0,(long)param_1,param_2);
  }
  if (*(long *)(iVar1 + 0x92) != 0x0) {
    pass1_1010_1ea6(*(ulong *)(iVar1 + 0x92),(long)param_1,param_2);
  }
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)(iVar1 + 0x6));
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x96),0x1000);
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



void __stdcall16far post_win_msg_1038_d840(astruct_70 *param_1,uint param_2,HWND16 param_3)

{
  astruct_70 *iVar1;
  astruct_70 *uVar1;
  
  iVar1 = (astruct_70 *)param_1;
  uVar1 = (astruct_70 *)((ulong)param_1 >> 0x10);
  if (param_2 == 0x10) {
    if (iVar1->field_0x8e != 0x0) {
      PostMessage16(param_3,0x0,0x0,CONCAT22(0x111,iVar1->field_0x8e));
      iVar1->field_0x8e = 0x0;
      return;
    }
  }
  else {
    if (param_2 < 0x11) {
      if ((char)param_2 == '\x01') {
        iVar1->field_0x90 = 0x0;
        iVar1->field_0x92 = 0x0;
        return;
      }
      if ((char)param_2 == '\x03') {
        pass1_1038_e03e((ulong)param_1);
        return;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far destroy_crsor_1038_d8b2(int param_1,HINSTANCE16 param_2,ushort param_3)

{
  int *piVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  HCURSOR16 HVar4;
  ushort uVar5;
  astruct_160 *rect;
  uchar *in_DX;
  uchar *puVar6;
  int iVar7;
  int iVar8;
  int unaff_DI;
  undefined2 uVar9;
  undefined2 *puVar10;
  ushort *puVar11;
  
  HVar4 = LoadCursor16(param_2,(LPCSTR)0x7f02);
  *(HCURSOR16 *)(param_1 + -0x2) = HVar4;
  HVar4 = SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
  *(HCURSOR16 *)(param_1 + -0x4) = HVar4;
  dialog_ui_fn_1040_78e2(*(astruct_1 **)(param_1 + 0x6),(int)&PTR_LOOP_1050_1040);
  uVar5 = pass1_1010_0886();
  *(ushort *)(param_1 + -0x6) = uVar5;
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)in_DX,0x1000);
    PTR_LOOP_1050_5f2e = in_DX;
  }
  else {
  }
  *(uchar **)(param_1 + -0x1c) = PTR_LOOP_1050_5f2c;
  *(uchar **)(param_1 + -0x1a) = PTR_LOOP_1050_5f2e;
  uVar5 = fn_ptr_op_1000_1708((*(int *)(param_1 + -0x6) + 0x2) * 0x4,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,
                              (uint)PTR_LOOP_1050_5f2e,0x1000);
  uVar2 = *(undefined4 *)(param_1 + 0x6);
  uVar9 = (undefined2)((ulong)uVar2 >> 0x10);
  iVar7 = (int)uVar2;
  *(ushort *)(iVar7 + 0x96) = uVar5;
  *(uchar **)(iVar7 + 0x98) = PTR_LOOP_1050_5f2e;
  *(undefined2 *)(param_1 + -0x8) = 0x1;
  while (iVar7 = *(int *)(param_1 + -0x6), piVar1 = (int *)(param_1 + -0x8), *piVar1 == iVar7 || *piVar1 < iVar7) {
    uVar2 = *(undefined4 *)(param_1 + 0x6);
    uVar2 = *(undefined4 *)((int)uVar2 + 0x92);
    puVar10 = (undefined2 *)pass1_1010_08e2((ushort)uVar2,(ushort)((ulong)uVar2 >> 0x10),*(int *)(param_1 + -0x8));
    puVar6 = (uchar *)((ulong)puVar10 >> 0x10);
    *(int *)(param_1 + -0x1c) = (int)puVar10;
    *(uchar **)(param_1 + -0x1a) = puVar6;
    *(undefined2 *)(param_1 + -0x24) = *puVar10;
    *(undefined2 *)(param_1 + -0x22) = *(undefined2 *)((int)puVar10 + 0x2);
    *(undefined2 *)(param_1 + -0x20) = 0x1;
    *(undefined2 *)(param_1 + -0x1e) = 0x1;
    rect = (astruct_160 *)(param_1 + -0x24);
    MapDialogRect16(0x1010,(RECT16 *)rect);
    mem_op_1000_179c(0x42,puVar6,0x1000);
    *(astruct_160 **)(param_1 + -0x28) = rect;
    *(uchar **)(param_1 + -0x26) = puVar6;
    PTR_LOOP_1050_5f2e = (undefined *)((uint)puVar6 | (uint)rect);
    if (PTR_LOOP_1050_5f2e == (uchar *)0x0) {
      uVar2 = *(undefined4 *)(param_1 + 0x6);
      uVar2 = *(undefined4 *)((int)uVar2 + 0x96);
      *(undefined4 *)((int)uVar2 + *(int *)(param_1 + -0x8) * 0x4) = 0x0;
    }
    else {
      uVar2 = *(undefined4 *)(param_1 + 0x6);
      uVar3 = *(undefined4 *)(param_1 + -0x1c);
      pass1_1008_3bd6(rect,*(ushort *)(param_1 + -0x26),0x0,
                      CONCAT22(*(undefined2 *)(param_1 + -0x24),*(undefined2 *)(param_1 + -0x22)),0x101,0xff0100,
                      CONCAT22(*(undefined2 *)((int)uVar2 + 0x6),*(undefined2 *)((int)uVar3 + 0x4)),
                      (ushort)PTR_LOOP_1050_5f2e,param_3);
      uVar2 = *(undefined4 *)(param_1 + 0x6);
      uVar2 = *(undefined4 *)((int)uVar2 + 0x96);
      uVar9 = (undefined2)((ulong)uVar2 >> 0x10);
      iVar7 = (int)uVar2;
      iVar8 = *(int *)(param_1 + -0x8) * 0x4;
      *(astruct_160 **)(iVar7 + iVar8) = rect;
      *(uchar **)(iVar7 + iVar8 + 0x2) = PTR_LOOP_1050_5f2e;
    }
    uVar2 = *(undefined4 *)(param_1 + 0x6);
    uVar2 = *(undefined4 *)((int)uVar2 + 0x96);
    uVar9 = (undefined2)((ulong)uVar2 >> 0x10);
    iVar7 = (int)uVar2;
    iVar8 = *(int *)(param_1 + -0x8) * 0x4;
    if (*(long *)(iVar7 + iVar8) != 0x0) {
      uVar2 = *(undefined4 *)(iVar7 + iVar8);
      *(undefined2 *)((int)uVar2 + 0x3e) = 0x1;
      uVar2 = *(undefined4 *)(param_1 + -0x1c);
      uVar3 = *(undefined4 *)(param_1 + 0x6);
      uVar3 = *(undefined4 *)((int)uVar3 + 0x96);
      enable_win_1040_9234
                (*(ulong *)((int)uVar3 + *(int *)(param_1 + -0x8) * 0x4),*(BOOL16 *)((int)uVar2 + 0x6),
                 (int)&PTR_LOOP_1050_1040);
    }
    piVar1 = (int *)(param_1 + -0x8);
    *piVar1 = *piVar1 + 0x1;
  }
  puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_3,PTR_LOOP_1050_5f2e,unaff_DI);
  *(undefined2 *)(param_1 + -0xc) = (int)puVar11;
  *(undefined2 *)(param_1 + -0xa) = (int)((ulong)puVar11 >> 0x10);
  uVar2 = *(undefined4 *)(param_1 + -0xc);
  SetWindowText16(0x1010,(SEGPTR)*(undefined4 *)((int)uVar2 + 0x68));
  ShowWindow16((HWND16)s_tile2_bmp_1050_1538,0x5);
  SetCursor16((HCURSOR16)s_tile2_bmp_1050_1538);
  return;
}

