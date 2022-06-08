//
// Created by cyrex on 2022-06-07.
//

#include "ops_4.h"
#include "func_ptrs.h"
#include "ops_6.h"
#include "win_ops.h"
#include "string_defs.h"
#include "utils.h"
#include "globals.h"
#include "sys_api.h"
#include "structs_2.h"
#include "mci_ops.h"
#include "ops_5.h"

HWND16 create_window_1008_5e7e(void)

{
  u32 *puVar2;
  BOOL16 BVar3;
  ATOM AVar4;
  HWND16 window_handle_1;
  i16 iVar5;
  char *string_1;
  u32 *puVar5;
  WNDCLASS16 wndclass_44;
  u32 local_12 [0x4];
  char *puVar1;

  puVar5 = local_12;
  string_1 = (char*)s_MciSoundWindow_1050_02bd;
  for (iVar5 = 0x3; iVar5 != 0x0; iVar5 += -0x1) {
    puVar2 = puVar5;
    puVar5 = puVar5 + 0x1;
    puVar1 = string_1;
    string_1 = (char *)((int)string_1 + 0x4);
    *puVar2 = (u32)puVar1;
  }
  puVar5 = string_1;
  *(u8 *)(u16 *)((int)puVar5 + 0x2) = *(u8 *)(u16 *)((int)string_1 + 0x2);
  wndclass_44.style = 0x2000;
//  wndclass_44.lpfn_wnd_proc = SUB42(&DAT_1050_5f44,0x0);
//  wndclass_44.lpfn_wnd_proc = 0x10085f44;
  wndclass_44.lpfn_wnd_proc = make_def_win_proc_1008_5f44;
  wndclass_44.cb_wnd_extra = 0x2;
  wndclass_44.h_instance = HINSTANCE16_1050_038c;
  wndclass_44.h_icon = 0x0;
  wndclass_44.h_cursor = 0x0;
  wndclass_44.cb_cls_extra = 0x0;
  wndclass_44.hbr_background = GetStockObject16(WHITE_BRUSH);
  wndclass_44.lpsz_menu_name = 0x0;
  wndclass_44.lpsz_class_name = local_12;
//  wndclass_44.lpsz_class_name = SUB42(0x1050,0x0);
  BVar3 = GetClassInfo16(&wndclass_44,wndclass_44.lpsz_class_name, 0);
  if (BVar3 == 0x0) {
    AVar4 = RegisterClass16(&wndclass_44);
    if (AVar4 == 0x0) {
      OutputDebugString16(s_MciSound_registerClass_failed_1050_02cc);
      return 0x0;
    }
  }
  window_handle_1 =
       CreateWindow16(0x0,(void *)HINSTANCE16_1050_038c,HWND16_1050_0396,0x1,0x1,-0x8000,-0x8000,0x0,0xcf,
                      s_MciSound_registerClass_failed_1050_02cc + 0x1e,local_12);
  return window_handle_1;
}

LRESULT make_def_win_proc_1008_5f44
                  (u16 param_1,u16 param_2,LPARAM param_3,WPARAM16 in_wparam_2,u16 param_5,HWND16 param_6)

{
  WORD WVar1;
//  u16 in_register_0000000a;
  Struct57* paVar2;
  LRESULT LVar3;
  u32 *puVar4;
  u16 in_stack_0000fea0 = 0;
  u16 in_stack_0000ffc4 = 0;
  u16 in_stack_0000ffca = 0;
  u16 in_stack_0000ffce = 0;
  u16 in_stack_0000fff8 = 0;

  paVar2 = (Struct57*)CONCAT22(AX_REG,param_1);
  if (param_5 == 0x2) {
    WVar1 = GetWindowWord16(0x0,param_6);
    mci_send_command_1008_5cb6(u16_1050_02a0,WVar1);
    puVar4 = mixed_1010_20ba(paVar2,u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x37),in_stack_0000fea0,
                             in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
    pass1_1008_aa28(puVar4,(u32)puVar4);
  }
  else {
    if (param_5 != 0x3b9) {
      LVar3 = DefWindowProc16(param_3,in_wparam_2,param_5,param_6);
      return LVar3;
    }
    DestroyWindow16(param_6);
  }
  return 0x0;
}


void win_ui_fn_1020_6e98(u16 param_1,StructA *param_2)

{
  Struct878 **ppaVar1;
  u32 uVar2;
  HWND16 window_handle;
  u16 uVar3;
  u16 uVar4;
  StructA *iVar4;
  u16 uVar5;
  LRESULT LVar6;
  char *pcVar7;
  WPARAM16 WVar8;
  u16 UVar9;
  HWND16 HVar10;
  u32 win_style;
  RECT16 rectangle;
  HWND16 hwnd_stack_5 = 0;
  i16 i16_stack_6 = 0;
  Struct878 *iVar9;

//  uVar5 = ((u32)param_2 >> 0x10);
  iVar4 = (StructA *)param_2;
  GetClientRect16(&rectangle,(HWND16)0x1050);
  win_style = 0x0;
  window_handle = GetDlgItem16(0x1797,iVar4->field4_0x8);
  if (window_handle != 0x0) {
    DestroyWindow16(window_handle);
  }
  pass1_1018_30fc(param_1,(u32)&iVar4[0x1].field20_0x26,(u16 **)CONCAT22(0x1050,&win_style));
  if (win_style != 0x0) {
    window_handle = CreateWindow16(win_style,
                                   (void *)CONCAT22(0x1797,HINSTANCE16_1050_038c),
                                   iVar4->field4_0x8,
                                   i16_stack_6 - 0x19,
                                   hwnd_stack_5,
                                   0x0,
                                   0x0,
                                   0x103,
                                   0x40a0,
                                   s__1050_4415,
                                   s_listbox_1050_4416);
    uVar2 = win_style;
    if (window_handle == 0x0) {
      if (win_style != 0x0) {
        pass1_1018_2afa((u32 *)win_style);
        fn_ptr_1000_17ce((char *)uVar2);
        return;
      }
    }
    else {
      LVar6 = SendMessage16(0x0,0x0,0xb,window_handle);
      uVar4 = ((u32)LVar6 >> 0x10);
      if (((int)win_style + 0x4) == 0x0) {
        WVar8 = 0x0;
        UVar9 = 0x401;
        HVar10 = window_handle;
        pcVar7 = load_string_1010_847e(_u16_1050_14cc,0x531);
        SendMessage16((LPARAM)pcVar7,WVar8,UVar9,HVar10);
      }
      else {
        iVar9 = NULL;
        while( true ) {
          ppaVar1 = (Struct878 **)((int)win_style + 0x4);
          if (*ppaVar1 == iVar9 || (int)*ppaVar1 < (int)iVar9) break;
          WVar8 = 0x0;
          UVar9 = 0x401;
          HVar10 = window_handle;
          uVar3 = pass1_1020_bd80(((int)(u32)win_style + (int)iVar9 * 0x2));
          LVar6 = SendMessage16(CONCAT22(uVar4,uVar3),WVar8,UVar9,HVar10);
          uVar4 = ((u32)LVar6 >> 0x10);
          iVar9 = (Struct878 *)(UVar9 + 0x1);
        }
      }
      LVar6 = SendMessage16(0x0,0x1,0xb,window_handle);
      uVar4 = ((u32)LVar6 >> 0x10);
      uVar3 = LVar6;
      WVar8 = 0xffff;
      UVar9 = 0x40d;
      HVar10 = window_handle;
      pass1_1018_2d84(uVar3,*(astruct_126 **)&iVar4[0x1].field20_0x26);
      LVar6 = SendMessage16(CONCAT22(uVar4,uVar3),WVar8,UVar9,HVar10);
      WVar8 = (WPARAM16)LVar6;
      if ((WVar8 != 0xffff) || ((int)((u32)LVar6 >> 0x10) != -0x1)) {
        SendMessage16(0x0,WVar8,0x407,window_handle);
        SendMessage16(0x0,WVar8,0x418,window_handle);
      }
      if (win_style != 0x0) {
//        pcVar7 = win_style;
        pass1_1018_2afa(win_style);
        fn_ptr_1000_17ce(win_style);
      }
      ShowWindow16(0x1,window_handle);
      SetFocus16(window_handle);
    }
  }
}


void pass1_1018_2afa(u32 param_1)

{
  fn_ptr_1000_17ce(param_1);
}


void create_win_1040_20d4(u32 param_1,StructB *struct_b_param_2,u16 param_3)

{
  i16 cx;
  StructB *struct_b_1;
  u16 uVar1;
  u32 *puVar2;
  char *window_name;
  u16 in_stack_0000fe72;
  u16 in_stack_0000ff96;
  u16 in_stack_0000ff9c;
  u16 in_stack_0000ffa0;
  RECT16 local_1e;
  i16 iStack26;
  Struct_858 *iStack24;
  u32 uStack22;
  u32 uStack18;
  i16 iStack14;
  u16 uStack12;
  i16 iStack10;
  i16 iStack8;
  u16 uStack6;
  i16 iStack4;

  dialog_ui_fn_1040_78e2(struct_b_param_2);
  puVar2 = mixed_1010_20ba((Struct57*)param_1,u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x48),in_stack_0000fe72,
                           in_stack_0000ff96,in_stack_0000ff9c,in_stack_0000ffa0);
  uStack12 = ((u32)puVar2 >> 0x10);
  iStack14 = (int)puVar2;
  iStack8 = (iStack14 + 0xa);
  iStack10 = (iStack14 + 0xc);
  uVar1 = ((u32)struct_b_param_2 >> 0x10);
  struct_b_1 = (StructB *)struct_b_param_2;
  uStack18 = pass1_1008_4772(*(Struct76 **)&struct_b_1[0x7].field1_0x2);
  cx = ((int)uStack18 + 0x4);
  iStack4 = (iStack8 - cx) / 0x2;
  uStack6 = 0x5;
  SetWindowPos16(0x6,0x1d6,cx,0x5,iStack4,0x0,(HWND16)struct_b_1->lpvoid_field_0x8);
  GetClientRect16(&local_1e,(HWND16)0x1050);
  window_name = load_string_1010_847e(_u16_1050_14cc,0x592);
  uStack22 = 0x50010001;
  CreateWindow16(0x0,(void *)CONCAT22(0x1,HINSTANCE16_1050_038c),(HINSTANCE16)struct_b_1->lpvoid_field_0x8,0x19,0x58,
                 (INT16)(iStack24 + -0x28),(iStack26 + -0x58) / 0x2,0x1,(int)s_Rebel_1050_4ffc + 0x5,window_name,
                 s_OPButton_1050_5ce4);
  SetWindowPos16(0x45,iStack10 + -0xa,*(INT16 *)((int)uStack18 + 0x4),0x5,iStack4,0x0,
                 (HWND16)struct_b_1->lpvoid_field_0x8);
  return;
}


void dialog_ui_fn_1040_78e2(StructB *in_struct_1)

{
  u8 *puVar1;
  LPVOID dialog_handle;
  u16 uVar2;
  StructB *struct_b_1;
  StructB *local_string_1;
  u16 uVar3;
  i32 lVar4;
  HANDLE16 local_string_2;
  HANDLE16 HStack8;
  void *pvStack6;
  code **fn_ptr_1;

  local_string_1 = (StructB *)((u32)in_struct_1 >> 0x10);
  struct_b_1 = (StructB *)in_struct_1;
  if (*(i32 *)&struct_b_1->field6_0xc == 0x0) {
    uVar3 = ((u32)_u16_1050_5bc8 >> 0x10);
    puVar1 = *(u8 **)((int)_u16_1050_5bc8 + 0x4);
    uVar2 = ((int)_u16_1050_5bc8 + 0x6);
  }
  else {
    puVar1 = struct_b_1->field6_0xc;
    uVar2 = struct_b_1->field7_0xe;
  }
  pvStack6 = (void *)CONCAT22(uVar2,puVar1);
  dialog_handle =
       (LPVOID)CreateDialog16(pvStack6,struct_b_1->max_count_field_0x10,(char *)ZEXT24(struct_b_1->field5_0xa),
                              HINSTANCE16_1050_038c);
  struct_b_1->lpvoid_field_0x8 = dialog_handle;
  GetWindowText16(0x50,(u32)in_struct_1 & 0xffff0000 | ZEXT24(&struct_b_1->field8_0x10),(HWND16)dialog_handle);
  lVar4 = GetWindowLong16(-0x4,(HWND16)struct_b_1->lpvoid_field_0x8);
  SetWindowLong16(_u16_1050_5bcc,-0x4,(HWND16)struct_b_1->lpvoid_field_0x8);
  SetProp16((HANDLE16)struct_b_1,s_thisLo_1050_5dcd,(HWND16)struct_b_1->lpvoid_field_0x8);
  SetProp16((HANDLE16)local_string_1,s_thisHi_1050_5dd4,(HWND16)struct_b_1->lpvoid_field_0x8);
  local_string_2 = (HANDLE16)lVar4;
  SetProp16(local_string_2,s_procLo_1050_5ddb,(HWND16)struct_b_1->lpvoid_field_0x8);
  HStack8 = (HANDLE16)((u32)lVar4 >> 0x10);
  SetProp16(HStack8,s_procHi_1050_5de2,(HWND16)struct_b_1->lpvoid_field_0x8);
  fn_ptr_1 = (code **)((int)(u32)in_struct_1 + 0x50);
  (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,in_struct_1);
  return;
}

void win_ui_op_1008_5cfe(Struct27 *param_1,char *param_2,WNDCLASS16 *in_wnd_class)

{
  u32 uVar1;
  i16 iVar2;
  Struct27 *p_struct7_1;
  u16 uVar3;
  DWORD DVar4;
  i16 iVar5;
  HWND16 message_1;
  u16 uStack298;
  HWND16 window_handle_1;
  u8 local_11e [0x100];
  char *string_1;
  i16 iStack26;
  i16 iStack24;
  u8 local_16 [0x4];
  u16 offset_val;
  char *pcStack14;
  char *pcStack10;

  pass1_1000_4906((StructD *)CONCAT22(0x1050,local_16),NULL,0x14);
  pcStack10 = param_2;
//  uVar3 = ((u32)param_1 >> 0x10);
  p_struct7_1 = (Struct27 *)param_1;
  uVar1 = (u32)&p_struct7_1->field_0xc;
  iStack24 = ((int)uVar1 + 0x72);
  iStack26 = 0x1;
  string_1 = s_waveaudio_1050_02a4;
  str_1000_4d58(param_2,NULL,0x0,0x0,(WNDCLASS16 *)CONCAT22(0x1050,local_11e));
  iVar2 = pass1_1000_475e(CONCAT22(0x1050,local_11e),(u32)s__mid_1050_02ae);
  if (iVar2 == 0x0) {
    uVar1 = (u32)&p_struct7_1->field_0xc;
    iStack24 = ((int)uVar1 + 0x74);
    string_1 = s_sequencer_1050_02b3;
    iStack26 = 0x0;
  }
  if (iStack24 != 0x0) {
    if ((iStack26 != 0x0) && (&p_struct7_1->field_0x10 != 0x0)) {
      return;
    }
    if ((iStack26 == 0x0) && (p_struct7_1->field18_0x12 != 0x0)) {
      return;
    }
    pcStack14 = string_1;
    DVar4 = mciSendCommand16(CONCAT22(0x1050,local_16),0x2200,0x803,0x0);
    if (((DVar4 >> 0x10) | DVar4) == 0x0) {
      if (iStack26 == 0x0) {
          p_struct7_1->field18_0x12 = 0x1;
      }
      else {
        &p_struct7_1->field_0xa = offset_val;
        &p_struct7_1->field_0x10 = 0x1;
      }
      window_handle_1 = create_window_1008_5e7e();
      if (window_handle_1 == 0x0) {
        mci_send_command_1008_5cb6(param_1,offset_val);
        return;
      }
      pass1_1000_4906((StructD *)CONCAT22(0x1050,&message_1),NULL,0xc);
      message_1 = window_handle_1;
      uStack298 = 0x0;
      mciSendCommand16(CONCAT22(0x1050,&message_1),0x1,0x806,offset_val);
      SetWindowWord16(offset_val,0x0,window_handle_1);
      return;
    }
  }
  if (iStack26 == 0x0) {
    iVar5 = 0x11;
  }
  else {
    iVar5 = 0x10;
  }
  pass1_1010_1f62(param_1,iVar5);
  return;
}
