

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
unk_win_sys_op_1038_da68(int param_1,ushort param_2,ushort param_3,ulong param_4,WNDCLASS16 *param_5,uchar *param_6)

{
  code **ppcVar1;
  uint uVar2;
  uchar *puVar3;
  uchar *extraout_DX;
  undefined2 in_BX;
  int unaff_DI;
  ushort unaff_CS;
  undefined2 uVar4;
  ulong uVar5;
  undefined4 uVar6;
  int iVar7;
  undefined local_16 [0x4];
  uint uStack18;
  uchar *puStack16;
  astruct_43 *paStack14;
  undefined2 uStack10;
  ushort uStack8;
  int iStack6;
  int iStack4;
  
  if (param_3 == 0x204) {
    pass1_1038_de20(CONCAT22(param_2,param_1),0x204,(ushort)param_4,param_4._2_2_,param_6,in_BX,param_5);
    return;
  }
  iStack6 = 0x0;
  iStack4 = 0x0;
  uStack8 = 0x0;
  if (param_4._2_2_ == 0x121) {
    uStack10 = 0x1;
    iStack6 = 0x0;
    iStack4 = 0x6ec;
    uStack8 = 0x15;
  }
  else {
    if (param_4 < 0x1220000) {
      uVar2 = param_4._2_2_ - 0x100;
      if (uVar2 == 0x0) {
        param_4._2_2_ = uVar2;
        if (*(int *)(param_1 + 0x8e) == 0x0) {
          pass1_1010_1ea6((ulong)_PTR_LOOP_1050_02a0,CONCAT22(param_2,param_1),(ushort)param_5);
          *(undefined2 *)(param_1 + 0x90) = 0x0;
        }
        iStack4 = 0x72c;
        uStack8 = 0x48;
      }
      else {
        if (param_4._2_2_ - 0x11c == 0x0) {
          param_4._2_2_ = param_4._2_2_ - 0x11c;
          pass1_1038_df86(CONCAT22(param_2,param_1),param_6,unaff_DI,(ushort)param_5);
        }
        else {
          if (param_4._2_2_ != 0x11d) {
            if (param_4._2_2_ == 0x11e) {
              iVar7 = 0x1d;
            }
            else {
              if (param_4._2_2_ != 0x120) {
LAB_1038_dc20:
                post_win_msg_1040_7b3c
                          ((ulong *)CONCAT22(param_2,param_1),param_3,(ushort)param_4,param_4._2_2_,
                           (int)&PTR_LOOP_1050_1040);
                return;
              }
              iVar7 = 0x1c;
            }
            goto LAB_1038_db1c;
          }
          uVar5 = pass1_1038_df5c(CONCAT22(param_2,param_1),param_6,param_5);
          param_6 = (uchar *)(uVar5 >> 0x10);
          param_4._2_2_ = (uint)uVar5;
        }
      }
    }
    else {
      if (param_4._2_2_ == 0x122) {
        iVar7 = 0x14;
      }
      else {
        if (param_4._2_2_ != 0x123) {
          if (param_4._2_2_ - 0x125 == 0x0) {
            ppcVar1 = (code **)((int)*_PTR_LOOP_1050_02a0 + 0x4);
            param_4._2_2_ = param_4._2_2_ - 0x125;
            (**ppcVar1)();
            *(undefined2 *)(param_1 + 0x90) = 0x1;
            param_6 = extraout_DX;
            win_1008_5c5c(param_5,param_4._2_2_,(ushort)extraout_DX,(ulong)_PTR_LOOP_1050_02a0,0x1db);
            *(undefined2 *)(param_1 + 0x8e) = 0x100;
          }
          else {
            if (param_4._2_2_ == 0x126) {
              *(undefined2 *)(param_1 + 0x8e) = 0x0;
              win_1008_5c7c((ulong)_PTR_LOOP_1050_02a0,0xcb0001,param_5,0x0,(ushort)param_6);
              paStack14 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1f8,(ushort)param_5);
              param_6 = (uchar *)((ulong)paStack14 >> 0x10);
              param_4._2_2_ = WinHelp16(0x1010,(LPCSTR)0x0,0x0,CONCAT22((int)paStack14,0x3));
            }
            else {
              if (param_4._2_2_ - 0x127 != 0x0) goto LAB_1038_dc20;
              param_4._2_2_ = param_4._2_2_ - 0x127;
              post_win_msg_1038_dcb0(CONCAT22(param_2,param_1),0x0,param_6,param_5);
            }
          }
          goto LAB_1038_dac3;
        }
        iVar7 = 0x28;
      }
LAB_1038_db1c:
      uVar6 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(param_1 + 0x8),iVar7,(ushort)param_6,param_1,unaff_CS,
                              (ushort)param_5);
      param_6 = (uchar *)((ulong)uVar6 >> 0x10);
      param_4._2_2_ = (uint)uVar6;
    }
  }
LAB_1038_dac3:
  if (iStack4 == 0x0) {
    return;
  }
  if (iStack6 == 0x0) {
    mem_op_1000_179c(0xb4,param_6,0x1000);
    puVar3 = (uchar *)((uint)param_6 | param_4._2_2_);
    uStack18 = param_4._2_2_;
    puStack16 = param_6;
    if (puVar3 != (uchar *)0x0) {
      uVar4 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar7 = string_1040_8520((astruct_57 *)
                               CONCAT13((char)((uint)param_6 >> 0x8),CONCAT12((char)param_6,param_4._2_2_)),
                               *(ushort *)(param_1 + 0x6),0x0,0x2,0x634,iStack4,puVar3,(ushort)param_5);
      goto LAB_1038_dc37;
    }
  }
  else {
    mem_op_1000_179c(0xb4,param_6,0x1000);
    puVar3 = (uchar *)((uint)param_6 | param_4._2_2_);
    uStack18 = param_4._2_2_;
    puStack16 = param_6;
    if (puVar3 != (uchar *)0x0) {
      uVar4 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar7 = string_1040_8520((astruct_57 *)CONCAT22(param_6,param_4._2_2_),*(ushort *)(param_1 + 0x6),0x0,0x3,0x634,
                               iStack4,puVar3,(ushort)param_5);
      goto LAB_1038_dc37;
    }
  }
  uVar4 = 0x1000;
  iVar7 = 0x0;
  puVar3 = (uchar *)0x0;
LAB_1038_dc37:
  paStack14 = (astruct_43 *)CONCAT22(puVar3,iVar7);
  if (uStack8 == 0x0) {
    ppcVar1 = (code **)((int)*(undefined4 *)paStack14 + 0x74);
    (**ppcVar1)(uVar4,iVar7,puVar3);
  }
  else {
    pass1_1008_941a((ushort *)CONCAT22(param_5,local_16),0x1,uStack8);
    ppcVar1 = (code **)((int)*(undefined4 *)paStack14 + 0x6c);
    (**ppcVar1)(0x1008,(char)paStack14,(int)((ulong)paStack14 >> 0x10),local_16,param_5);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far post_win_msg_1038_dcb0(ulong param_1,uint param_2,uchar *param_3,ushort param_4)

{
  code **ppcVar1;
  int iVar2;
  int iVar3;
  uchar *puVar4;
  uchar *extraout_DX;
  int unaff_DI;
  undefined2 uVar5;
  undefined2 uVar6;
  ushort uVar7;
  undefined2 uVar8;
  undefined uVar9;
  undefined uVar10;
  undefined4 local_18;
  undefined local_14 [0x4];
  undefined4 uStack16;
  int iStack12;
  undefined local_a [0x4];
  undefined4 *puStack6;
  
  mem_op_1000_179c(0xb4,param_3,0x1000);
  puVar4 = (uchar *)((uint)param_3 | param_2);
  iVar3 = (int)param_1;
  uVar5 = (undefined2)(param_1 >> 0x10);
  uStack16._0_2_ = param_2;
  uStack16._2_2_ = param_3;
  if (puVar4 == (uchar *)0x0) {
    iVar2 = 0x0;
    puVar4 = (uchar *)0x0;
  }
  else {
    iVar2 = string_1040_8520((astruct_57 *)CONCAT22(param_3,param_2),*(ushort *)(iVar3 + 0x6),0x4,0x3,0x634,0x726,puVar4
                             ,param_4);
  }
  puStack6 = (undefined4 *)CONCAT22(puVar4,iVar2);
  pass1_1008_941a((ushort *)CONCAT22(param_4,local_a),0x1,0x49);
  ppcVar1 = (code **)((int)*puStack6 + 0x6c);
  uStack16 = (astruct_57 *)(**ppcVar1)(0x1008,(int)puStack6,(int)((ulong)puStack6 >> 0x10),local_a,param_4);
  puVar4 = (uchar *)((ulong)uStack16 >> 0x10);
  iStack12 = (int)uStack16;
  if (iStack12 == 0x6) {
    mem_op_1000_179c(0xb4,puVar4,0x1000);
    puVar4 = (uchar *)((uint)((ulong)uStack16 >> 0x10) | (uint)uStack16);
    if (uStack16 == (astruct_57 *)0x0) {
      iVar3 = 0x0;
      puVar4 = (uchar *)0x0;
    }
    else {
      iVar3 = string_1040_8520(uStack16,*(ushort *)(iVar3 + 0x6),0x0,0x2,0x634,0x728,puVar4,param_4);
    }
    puStack6 = (undefined4 *)CONCAT22(puVar4,iVar3);
    pass1_1008_941a((ushort *)CONCAT22(param_4,local_14),0x1,0x4a);
    ppcVar1 = (code **)((int)*puStack6 + 0x6c);
    (**ppcVar1)(0x1008,(int)puStack6,(int)((ulong)puStack6 >> 0x10),local_14);
    uVar9 = 0x0;
    uVar10 = 0x0;
    iVar2 = 0x15;
    uVar7 = 0x1;
    uVar8 = 0x0;
    uVar6 = 0x0;
    iVar3 = 0x0;
    uVar5 = 0x0;
    local_18 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_4,extraout_DX,unaff_DI);
    post_win_msg_1008_a0e4
              (local_18,CONCAT22(uVar6,uVar5),iVar3,uVar7,CONCAT13(uVar10,CONCAT12(uVar9,uVar8)),iVar2,0x1008,param_4);
    PostMessage16(0x1008,0x0,0x0,0x11100fc);
    return;
  }
  mem_op_1000_179c(0xb4,puVar4,0x1000);
  puVar4 = (uchar *)((uint)((ulong)uStack16 >> 0x10) | (uint)uStack16);
  if (uStack16 == (astruct_57 *)0x0) {
    iVar3 = 0x0;
    puVar4 = (uchar *)0x0;
  }
  else {
    iVar3 = string_1040_8520(uStack16,*(ushort *)(iVar3 + 0x6),0x0,0x2,0x634,0x729,puVar4,param_4);
  }
  puStack6 = (undefined4 *)CONCAT22(puVar4,iVar3);
  pass1_1008_941a((ushort *)CONCAT22(param_4,&local_18),0x1,0x4b);
  ppcVar1 = (code **)((int)*puStack6 + 0x6c);
  (**ppcVar1)(0x1008,(int)puStack6,(int)((ulong)puStack6 >> 0x10),&local_18);
  return;
}



void __stdcall16far
pass1_1038_de20(ulong param_1,ushort param_2,ushort param_3,int param_4,uchar *param_5,uint param_6,ushort param_7)

{
  code **ppcVar1;
  int iVar2;
  uchar *puVar3;
  undefined2 uVar4;
  undefined local_12 [0x4];
  uint uStack14;
  uchar *puStack12;
  undefined4 *puStack10;
  ushort uStack6;
  int iStack4;
  
  iStack4 = 0x644;
  uStack6 = 0x0;
  switch(param_4 + -0x11c) {
  case 0x0:
    iStack4 = 0x635;
    uStack6 = 0x3a;
    break;
  case 0x1:
    iStack4 = 0x636;
    uStack6 = 0x3b;
    break;
  case 0x2:
    iStack4 = 0x637;
    uStack6 = 0x3c;
    break;
  case 0x4:
    iStack4 = 0x639;
    uStack6 = 0x3e;
    break;
  case 0x5:
    iStack4 = 0x63a;
    uStack6 = 0x3f;
    break;
  case 0x6:
    iStack4 = 0x63b;
    uStack6 = 0x40;
    break;
  case 0x7:
    iStack4 = 0x640;
    uStack6 = 0x45;
    break;
  case 0x9:
    iStack4 = 0x642;
    uStack6 = 0x47;
    break;
  case 0xa:
    iStack4 = 0x641;
    uStack6 = 0x46;
    break;
  case 0xb:
    iStack4 = 0x63f;
    uStack6 = 0x44;
  }
  if (iStack4 != 0x0) {
    uVar4 = 0x1000;
    mem_op_1000_179c(0xb4,param_5,0x1000);
    puVar3 = (uchar *)((uint)param_5 | param_6);
    uStack14 = param_6;
    puStack12 = param_5;
    if (puVar3 == (uchar *)0x0) {
      iVar2 = 0x0;
      puVar3 = (uchar *)0x0;
    }
    else {
      uVar4 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar2 = string_1040_8520((astruct_57 *)CONCAT22(param_5,param_6),*(ushort *)((int)param_1 + 0x6),0x0,0x2,0x634,
                               iStack4,puVar3,param_7);
    }
    puStack10 = (undefined4 *)CONCAT22(puVar3,iVar2);
    if (uStack6 == 0x0) {
      ppcVar1 = (code **)((int)*puStack10 + 0x74);
      (**ppcVar1)(uVar4,iVar2,puVar3);
    }
    else {
      pass1_1008_941a((ushort *)CONCAT22(param_7,local_12),0x1,uStack6);
      ppcVar1 = (code **)((int)*puStack10 + 0x6c);
      (**ppcVar1)(0x1008,(char)puStack10,(int)((ulong)puStack10 >> 0x10),local_12,param_7);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1038_df5c(ulong param_1,ushort param_2,ushort param_3)

{
  ushort uVar1;
  undefined2 uVar2;
  ulong uVar3;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  uVar1 = (ushort)param_1;
  pass1_1010_038e(*(ulong *)(uVar1 + 0x92),0x1,param_3);
  uVar3 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(uVar1 + 0x8),0x20,param_2,uVar1,0x1010,param_3);
  return uVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_df86(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  char *pcVar1;
  code **ppcVar2;
  BOOL16 BVar3;
  uint uVar4;
  ushort uVar5;
  uchar *puVar6;
  undefined2 uVar7;
  ushort uVar8;
  undefined2 uVar9;
  undefined uVar10;
  ushort *puVar11;
  char *pcVar12;
  astruct_57 *paVar13;
  undefined4 *puStack22;
  
  puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,param_2,param_3);
  uVar5 = (ushort)((ulong)puVar11 >> 0x10);
  pcVar1 = *(char **)((int)puVar11 + 0x68);
  uVar9 = (undefined2)(param_1 >> 0x10);
  uVar8 = (ushort)param_1;
  BVar3 = pass1_1010_041a();
  if (BVar3 != 0x0) {
    pass1_1010_038e(*(ulong *)(uVar8 + 0x92),0x1,param_4);
    pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(uVar8 + 0x8),0x1e,uVar5,uVar8,0x1010,param_4);
    return;
  }
  pcVar12 = load_string_1010_847e((int)_PTR_LOOP_1050_14cc,(INT16)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  puVar6 = (uchar *)((ulong)pcVar12 >> 0x10);
  uVar4 = (uint)pcVar12;
  uVar10 = 0x0;
  mem_op_1000_179c(0xb4,puVar6,0x1000);
  if (((uint)puVar6 | uVar4) == 0x0) {
    uVar9 = 0x0;
    uVar7 = 0x0;
  }
  else {
    uVar10 = 0x40;
    paVar13 = pass1_1040_8478((astruct_57 *)CONCAT22(puVar6,uVar4),0x20,pcVar1,pcVar12,*(ushort *)(uVar8 + 0x6),
                              (uint)puVar6 | uVar4);
    uVar7 = (undefined2)((ulong)paVar13 >> 0x10);
    uVar9 = SUB42(paVar13,0x0);
  }
  puStack22 = (undefined4 *)CONCAT22(uVar7,uVar9);
  ppcVar2 = (code **)((int)*puStack22 + 0x74);
  (**ppcVar2)(uVar10,uVar9,uVar7);
  return;
}



void __stdcall16far pass1_1038_e03e(ulong param_1)

{
  undefined4 uVar1;
  ushort uVar2;
  int iVar3;
  undefined2 uVar4;
  undefined2 uVar5;
  ulong uVar6;
  int iStack6;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  uVar2 = pass1_1010_0886();
  for (iStack6 = 0x1; iStack6 <= (int)uVar2; iStack6 = iStack6 + 0x1) {
    uVar1 = *(undefined4 *)((int)param_1 + 0x92);
    uVar6 = pass1_1010_08e2((ushort)uVar1,(ushort)((ulong)uVar1 >> 0x10),iStack6);
    uVar1 = *(undefined4 *)((int)param_1 + 0x96);
    uVar5 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar3 = (int)uVar1;
    if (*(long *)(iVar3 + iStack6 * 0x4) != 0x0) {
      enable_win_1040_9234(*(ulong *)(iVar3 + iStack6 * 0x4),*(BOOL16 *)((int)uVar6 + 0x6),(int)&PTR_LOOP_1050_1040);
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1038_e0ae(astruct_18 *param_1,byte param_2,ushort param_3)

{
  pass1_1038_d7d0(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_57 * __stdcall16far
pass1_1038_e140(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5)

{
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfc2,param_5);
  *(undefined2 *)param_1 = 0xe264;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_e16e(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xe264;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



void __stdcall16far check_radio_btn_show_win_1038_e19a(astruct_1 *param_1)

{
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  CheckRadioButton16((HWND16)&PTR_LOOP_1050_1040,0x1807,0x1807,0x1807);
  move_win_1040_826c(param_1,0xc8,0xc8);
  ShowWindow16((HWND16)&PTR_LOOP_1050_1040,0x5);
  return;
}



void __stdcall16far destroy_win_1038_e1dc(UINT16 param_1,UINT16 param_2,int param_3,HWND16 param_4)

{
  UINT16 UVar1;
  LPARAM lparam;
  
  if (param_3 != 0x0) {
    UVar1 = IsDlgButtonChecked(param_4,0x1807);
    if (UVar1 == 0x0) {
      param_4 = (HWND16)s_tile2_bmp_1050_1538;
      UVar1 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,0x1806);
      if (UVar1 == 0x0) goto LAB_1038_e229;
      lparam = 0x1110130;
    }
    else {
      lparam = 0x111012f;
    }
    param_4 = (HWND16)s_tile2_bmp_1050_1538;
    SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,lparam);
  }
LAB_1038_e229:
  DestroyWindow16(param_4);
  return;
}



astruct_18 * __stdcall16far pass1_1038_e23e(astruct_18 *param_1,byte param_2)

{
  pass1_1038_e16e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_57 * __stdcall16far pass1_1038_e2d0(astruct_57 *param_1,ushort param_2)

{
  undefined2 uVar1;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x1c3,param_2);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined4 *)((int)param_1 + 0x8e) = 0x0;
  *(undefined2 *)param_1 = 0xe62e;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_e308(astruct_18 *param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  param_1->field_0x0 = 0xe62e;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)(iVar1 + 0x6));
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x8e),0x1000);
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far win_ui_op_1038_e348(astruct_1 *param_1)

{
  undefined4 uVar1;
  ushort uVar2;
  astruct_160 *rect;
  uchar *in_DX;
  uchar *puVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  int unaff_DI;
  undefined2 uVar7;
  undefined2 uVar8;
  ushort unaff_SS;
  undefined2 local_22;
  undefined2 uStack32;
  undefined2 uStack30;
  undefined2 uStack28;
  undefined2 *puStack26;
  int iStack10;
  ushort uStack8;
  ushort *puStack6;
  
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,unaff_SS,in_DX,unaff_DI);
  PTR_LOOP_1050_5f2e = (undefined *)((ulong)puStack6 >> 0x10);
  uStack8 = pass1_1010_088c();
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a((ushort)PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
  }
  puStack26 = (undefined2 *)CONCAT22(PTR_LOOP_1050_5f2e,PTR_LOOP_1050_5f2c);
  uVar2 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4,0x0,0x1,(uint)PTR_LOOP_1050_5f2c,(uint)PTR_LOOP_1050_5f2e,0x1000);
  uVar7 = (undefined2)((ulong)param_1 >> 0x10);
  iVar5 = (int)param_1;
  *(ushort *)(iVar5 + 0x8e) = uVar2;
  *(undefined2 *)(iVar5 + 0x90) = PTR_LOOP_1050_5f2e;
  for (iStack10 = 0x1; iStack10 <= (int)uStack8; iStack10 = iStack10 + 0x1) {
    puStack26 = (undefined2 *)pass1_1010_091e((ushort)puStack6,(ushort)((ulong)puStack6 >> 0x10),iStack10);
    puVar3 = (uchar *)((ulong)puStack26 >> 0x10);
    local_22 = *puStack26;
    uStack32 = *(undefined2 *)((int)puStack26 + 0x2);
    uStack30 = 0x1;
    uStack28 = 0x1;
    rect = (astruct_160 *)&local_22;
    MapDialogRect16(0x1010,(RECT16 *)rect);
    mem_op_1000_179c(0x42,puVar3,0x1000);
    uVar4 = (uint)puVar3 | (uint)rect;
    if (uVar4 == 0x0) {
      uVar1 = *(undefined4 *)(iVar5 + 0x8e);
      *(undefined4 *)((int)uVar1 + iStack10 * 0x4) = 0x0;
    }
    else {
      pass1_1008_3bd6(rect,(ushort)puVar3,0x0,CONCAT22(local_22,uStack32),0x101,0xff0100,
                      CONCAT22(*(undefined2 *)(iVar5 + 0x6),*(undefined2 *)((int)puStack26 + 0x4)),uVar4,unaff_SS);
      uVar1 = *(undefined4 *)(iVar5 + 0x8e);
      uVar8 = (undefined2)((ulong)uVar1 >> 0x10);
      iVar6 = (int)uVar1;
      *(astruct_160 **)(iVar6 + iStack10 * 0x4) = rect;
      *(uint *)(iVar6 + iStack10 * 0x4 + 0x2) = uVar4;
    }
    uVar1 = *(undefined4 *)(iVar5 + 0x8e);
    uVar8 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar6 = (int)uVar1;
    if (*(long *)(iVar6 + iStack10 * 0x4) != 0x0) {
      enable_win_1040_9234
                (*(ulong *)(iVar6 + iStack10 * 0x4),*(BOOL16 *)((int)puStack26 + 0x6),(int)&PTR_LOOP_1050_1040);
    }
  }
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16((HWND16)&PTR_LOOP_1050_1040,0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_e4bc(ushort param_1,ulong param_2,ulong param_3,uchar *param_4,int param_5,ushort param_6)

{
  code **ppcVar1;
  undefined4 uVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  uchar *puVar6;
  uint extraout_DX;
  uchar *extraout_DX_00;
  uchar *puVar7;
  code **ppcVar8;
  ulong *puVar9;
  ushort *puVar10;
  undefined2 uVar11;
  undefined uVar12;
  undefined uVar13;
  undefined2 uVar14;
  undefined2 uVar15;
  undefined2 uVar16;
  undefined4 *puStack22;
  
  if (param_3._2_2_ == 0x1c4) {
    puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,param_4,param_5);
    uVar14 = (undefined2)((ulong)puVar10 >> 0x10);
    uVar4 = *(uint *)((int)puVar10 + 0x24);
    uVar5 = *(uint *)((int)puVar10 + 0x26);
    uVar3 = uVar5 | uVar4;
    if (uVar3 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4,uVar5);
      if ((uVar5 | uVar3) != 0x0) {
        puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x20);
        puVar6 = (uchar *)((ulong)puVar9 >> 0x10);
        uVar4 = (uint)puVar9;
        pass1_1038_4e78(uVar4,puVar6,CONCAT22(uVar5,uVar3),puVar9);
        puStack22 = (undefined4 *)CONCAT22(puVar6,uVar4);
        uVar2 = *puStack22;
        ppcVar8 = (code **)uVar2;
        ppcVar1 = ppcVar8 + 0x8;
        uVar5 = uVar4;
        (**ppcVar1)(0x1008,uVar4,puVar6);
        if ((extraout_DX | uVar5) == 0x0) {
          if (puStack22 != (undefined4 *)0x0) {
            ppcVar1 = ppcVar8;
            (**ppcVar1)(0x1008,uVar4,(char)puVar6,0x1);
          }
        }
        else {
          ppcVar1 = (code **)((int)*puStack22 + 0x4);
          (**ppcVar1)(0x8,uVar4,puVar6,0x0,0x0);
          puVar7 = extraout_DX_00;
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5,(uint)extraout_DX_00);
          puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_6,puVar7,(int)((ulong)uVar2 >> 0x10));
          pass1_1010_71d6((ulong)puVar10,0x1,
                          (ushort *)((ZEXT24(puVar7) & 0xff00) << 0x10 | (ulong)CONCAT12((char)puVar7,uVar5 + 0xc)),
                          uVar5 + 0xc,(uint)((ulong)puVar10 >> 0x10),param_6);
          if (puStack22 != (undefined4 *)0x0) {
            ppcVar1 = (code **)*puStack22;
            (**ppcVar1)(0x1010,uVar4,(char)puVar6,0x1);
          }
        }
      }
    }
  }
  else {
    if (param_3._2_2_ == 0x1c5) {
      uVar14 = 0xe;
    }
    else {
      if (param_3._2_2_ != 0x1c6) {
        post_win_msg_1040_7b3c
                  ((ulong *)CONCAT13((char)(param_2 >> 0x8),CONCAT12((char)param_2,param_1)),(ushort)(param_2 >> 0x10),
                   (ushort)param_3,param_3._2_2_,(int)&PTR_LOOP_1050_1040);
        return;
      }
      uVar14 = 0xd;
    }
    uVar16 = 0x0;
    uVar15 = 0x0;
    uVar11 = 0x0;
    uVar12 = 0x0;
    uVar13 = 0x0;
    puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_6,param_4,param_5);
    unk_win_op_1010_7300((ulong)puVar10,CONCAT13(uVar13,CONCAT12(uVar12,uVar11)),uVar14,CONCAT22(uVar16,uVar15));
  }
  return;
}



astruct_18 * __stdcall16far pass1_1038_e608(astruct_18 *param_1,byte param_2)

{
  pass1_1038_e308(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_e69a(astruct_57 *param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar *param_6,
               int param_7,ushort param_8)

{
  astruct_713 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfcb,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_713 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  *(undefined2 *)param_1 = 0xe92e;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x43,param_8,param_6,param_7);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_e6f0(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xe92e;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



void __stdcall16far unk_win_ui_op_1038_e71c(astruct_1 *param_1,UINT16 param_2)

{
  undefined2 extraout_DX;
  int iVar1;
  undefined2 uVar2;
  undefined2 unaff_SS;
  astruct_18 *paStack6;
  
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  unk_load_str_op_1010_2c34(*(undefined4 *)(iVar1 + 0x8e));
  paStack6 = (astruct_18 *)CONCAT22(extraout_DX,param_2);
  unk_str_op_1000_3d3e
            ((char *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x10)),(char *)CONCAT22(extraout_DX,param_2));
  fn_ptr_1000_17ce(paStack6,0x1000);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16((HWND16)&PTR_LOOP_1050_1040,0x5);
  *(undefined2 *)(iVar1 + 0x92) = 0x1;
  unk_win_msg_op_1008_9510((int *)((ulong)param_1 & 0xffff0000 | (ulong)(iVar1 + 0x92)),0x1008,unaff_SS);
  DestroyWindow16(0x1008);
  return;
}



void __stdcall16far chk_is_dlg_btn_checked_1038_e7a0(ulong param_1,int param_2)

{
  undefined4 uVar1;
  UINT16 UVar2;
  astruct_62 *iVar3;
  undefined2 uVar3;
  HWND16 unaff_CS;
  
  iVar3 = (astruct_62 *)param_1;
  uVar3 = (undefined2)(param_1 >> 0x10);
  if (param_2 == 0x0) {
    uVar1 = iVar3->field_0x8e;
    *(undefined2 *)((int)uVar1 + 0x10) = 0x1;
    uVar1 = iVar3->field_0x8e;
    *(undefined2 *)((int)uVar1 + 0xa) = 0x0;
    uVar1 = iVar3->field_0x8e;
    *(undefined2 *)((int)uVar1 + 0xc) = 0x0;
    uVar1 = iVar3->field_0x8e;
    *(undefined2 *)((int)uVar1 + 0xe) = 0x0;
  }
  else {
    UVar2 = IsDlgButtonChecked(unaff_CS,0x1827);
    if (UVar2 == 0x0) {
      UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,0x1828);
      if (UVar2 == 0x0) {
        uVar1 = iVar3->field_0x8e;
        *(undefined2 *)((int)uVar1 + 0xa) = 0x0;
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
    UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,(UINT16)s_vrpal_bmp_1050_183a);
    if (UVar2 == 0x0) {
      UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,(int)s_vrpal_bmp_1050_183a + 0x1);
      if (UVar2 == 0x0) {
        uVar1 = iVar3->field_0x8e;
        *(undefined2 *)((int)uVar1 + 0xc) = 0x0;
      }
      else {
        uVar1 = iVar3->field_0x8e;
        *(undefined2 *)((int)uVar1 + 0xc) = 0x2;
      }
    }
    else {
      uVar1 = iVar3->field_0x8e;
      *(undefined2 *)((int)uVar1 + 0xc) = 0x1;
    }
    UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,(int)s_vrpal_bmp_1050_183a + 0x2);
    if (UVar2 == 0x0) {
      UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,(int)s_vrpal_bmp_1050_183a + 0x3);
      if (UVar2 == 0x0) {
        uVar1 = iVar3->field_0x8e;
        *(undefined2 *)((int)uVar1 + 0xe) = 0x0;
      }
      else {
        uVar1 = iVar3->field_0x8e;
        *(undefined2 *)((int)uVar1 + 0xe) = 0x2;
      }
    }
    else {
      uVar1 = iVar3->field_0x8e;
      *(undefined2 *)((int)uVar1 + 0xe) = 0x1;
    }
    uVar1 = iVar3->field_0x8e;
    *(undefined2 *)((int)uVar1 + 0x10) = 0x0;
  }
  iVar3->field_0x92 = 0x0;
  return;
}



astruct_18 * __stdcall16far pass1_1038_e908(astruct_18 *param_1,byte param_2)

{
  pass1_1038_e6f0(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * __stdcall16far
pass1_1038_e99a(astruct_57 *param_1,ulong param_2,undefined2 param_3,undefined2 param_4,ushort param_5,uchar *param_6,
               ushort param_7)

{
  astruct_434 *iVar1;
  int unaff_DI;
  undefined2 uVar1;
  ushort *puVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfb9,param_5);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_434 *)param_1;
  *(undefined4 *)&iVar1->field_0x8e = 0x0;
  *(undefined2 *)param_1 = 0xeb32;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x30,param_7,param_6,unaff_DI);
  iVar1->field_0x8e = (int)puVar2;
  iVar1->field_0x90 = (int)((ulong)puVar2 >> 0x10);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_e9ec(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0xeb32;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,(int)&PTR_LOOP_1050_1040);
  return;
}



void __stdcall16far win_ui_op_1038_ea18(astruct_1 *param_1)

{
  INT16 IVar1;
  BOOL16 BVar2;
  RECT16 local_10 [0x2];
  HWND16 HStack8;
  ulong uStack6;
  
  dialog_ui_fn_1040_78e2(param_1,(int)&PTR_LOOP_1050_1040);
  uStack6 = pass1_1010_375e(*(ulong *)((int)param_1 + 0x8e));
  HStack8 = GetDlgItem16(0x1010,0xfa5);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)uStack6,(WPARAM16)(uStack6 >> 0x10),0xc0000);
  GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,local_10);
  BVar2 = 0x4;
  IVar1 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  move_win_1040_826c(param_1,IVar1 + local_10[0].y + 0x5,BVar2);
  ShowWindow16((HWND16)&PTR_LOOP_1050_1040,0x5);
  return;
}



void __stdcall16far win_ui_op_1038_eaa2(ulong param_1,int param_2,HWND16 param_3,WPARAM16 param_4)

{
  LRESULT LVar1;
  undefined local_54 [0x52];
  
  if (param_2 != 0x0) {
    GetDlgItem16(param_3,0xfa5);
    LVar1 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,(UINT16)local_54,param_4,0xd0050);
    pass1_1010_3770(*(ulong *)((int)param_1 + 0x8e),(char *)CONCAT22(param_4,local_54),(ushort)((ulong)LVar1 >> 0x10));
    param_3 = (HWND16)s_tile2_bmp_1050_1538;
    PostMessage16(0x1010,0x0,0x0,0x11100fb);
  }
  DestroyWindow16(param_3);
  return;
}



astruct_18 * __stdcall16far pass1_1038_eb0c(astruct_18 *param_1,byte param_2)

{
  pass1_1038_e9ec(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_57 * __stdcall16far pass1_1038_eb9e(astruct_57 *param_1,ushort param_2)

{
  undefined2 uVar1;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x1c7,param_2);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined4 *)((int)param_1 + 0x8e) = 0x0;
  *(undefined2 *)param_1 = 0xee6e;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  return param_1;
}
