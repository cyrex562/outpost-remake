//
// Created by cyrex on 2022-06-07.
//

#include "win_ops.h"
#include "string_defs.h"
#include "utils.h"
#include "globals.h"
#include "sys_api.h"
#include "structs_2.h"
#include "mci_ops.h"

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
//  wndclass_44.lpfn_wnd_proc._0_2_ = SUB42(&DAT_1050_5f44,0x0);
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
  u16 in_register_0000000a;
  Struct57* paVar2;
  LRESULT LVar3;
  u32 *puVar4;
  u16 in_stack_0000fea0;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffce;
  u16 in_stack_0000fff8;

  paVar2 = (Struct57*)CONCAT22(in_register_0000000a,param_1);
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


