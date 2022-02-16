


void __stdcall16far send_dlg_item_msg_1040_d20c(ulong param_1,long param_2,ushort param_3,ushort param_4)

{
  undefined2 in_AX;
  uchar *in_DX;
  ushort uVar1;
  int unaff_DI;
  undefined2 uVar2;
  uchar in_AF;
  ushort *puVar3;
  undefined *puVar4;
  ushort uVar5;
  undefined local_106 [0x104];
  
  if (param_2 == 0x0) {
    enable_win_1040_d60e(param_1,param_3);
    return;
  }
  uVar2 = (undefined2)(param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0xa0) != 0x0) {
    pass1_1010_9210(*(ulong *)((int)param_1 + 0x9c));
    enable_win_1040_d60e(param_1,0x1010);
    pass1_1030_8344((ushort)_PTR_LOOP_1050_5748,(ushort)((ulong)_PTR_LOOP_1050_5748 >> 0x10),param_2);
    puVar4 = local_106;
    uVar5 = param_4;
    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_4,in_DX,unaff_DI);
    uVar1 = (ushort)((ulong)puVar3 >> 0x10);
    pass1_1010_c3c2((ushort)puVar3,uVar1,CONCAT22(uVar5,puVar4),CONCAT22(in_DX,in_AX),(uchar *)uVar1,in_AF,param_4);
    SendDlgItemMessage16(0x1010,(INT16)local_106,param_4,0x0,0x18470401);
  }
  return;
}




void __stdcall16far
win_ui_op_1040_d2ac(int param_1,ushort param_2,ushort param_3,ulong param_4,ushort param_5,ushort param_6,ushort param_7
                   )

{
  LRESULT LVar1;
  
  if (param_4._2_2_ == (int)s_dibtext_bmp_1050_1844 + 0x4U) {
    SendDlgItemMessage16(param_6,0x0,0x0,0x0,0x18470405);
    struct_1010_9172(*(ulong *)(param_1 + 0x9c));
  }
  else {
    if ((int)s_dibtext_bmp_1050_1844 + 0x4U < param_4._2_2_) {
      if (param_4._2_2_ == (int)s_dibtext_bmp_1050_1844 + 0x5U) {
        LVar1 = SendDlgItemMessage16(param_6,0x0,0x0,0x0,0x1847040c);
        if (((int)LVar1 != -0x1) || ((int)((ulong)LVar1 >> 0x10) != -0x1)) {
          SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,(int)LVar1 - 0x1,0x18470403);
          pass1_1010_91cc(*(ulong *)(param_1 + 0x9c));
        }
      }
      else {
        if (param_4._2_2_ == (int)s_dibtext_bmp_1050_1844 + 0x6U) {
          enable_win_1040_d6be(CONCAT22(param_2,param_1),param_6);
          pass1_1018_57d2(*(ulong *)(param_1 + 0x94),CONCAT22(param_2,param_1));
          PostMessage16(0x1018,0x0,0x0,0x1110203);
        }
        else {
          if (param_4._2_2_ != (int)s_dibtext_bmp_1050_1844 + 0x7U) goto LAB_1040_d3b3;
          _PTR_LOOP_1050_5a68 = *(undefined4 *)(param_1 + 0x98);
          pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(param_1 + 0x6),0x27,param_5,param_1,
                          (ushort)&PTR_LOOP_1050_1038,param_7);
        }
      }
    }
    else {
      if (param_4._2_2_ == 0xeb) {
        send_ldg_item_msg_1040_d79c(CONCAT22(param_2,param_1),param_7);
      }
      else {
        if (param_4._2_2_ != (int)s_vrpal_bmp_1050_183a + 0x7U) {
LAB_1040_d3b3:
          pass1_1040_b54a(param_1,param_2,param_3,param_4,(uchar *)param_5,param_6,param_7);
          return;
        }
        msg_box_op_1040_d3d0(CONCAT22(param_2,param_1),0x0,param_5,param_7);
      }
    }
  }
  return;
}




void __stdcall16far msg_box_op_1040_d3d0(undefined4 param_1,char *param_2,uchar *param_3,UINT16 param_4)

{
  char local_206 [0x102];
  char local_104 [0x102];
  
  mem_op_1000_179c(0x1000,param_3,0x1000);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,local_206,param_4);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,param_2,
             (short)param_3);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,(LPCSTR)0x0,local_206,param_4);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,param_2,
             (short)param_3);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,(LPCSTR)0x0,local_206,param_4);
  fn_ptr_1000_17ce((astruct_18 *)CONCAT22(param_3,param_2),0x1000);
  return;
}



void __stdcall16far enable_win_1040_d60e(ulong param_1,HWND16 param_2)

{
  GetDlgItem16(param_2,0x1);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x2);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(int)s_vrpal_bmp_1050_183a + 0x7);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(int)s_dibtext_bmp_1050_1844 + 0x4);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(int)s_dibtext_bmp_1050_1844 + 0x5);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(int)s_dibtext_bmp_1050_1844 + 0x6);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(int)s_dibtext_bmp_1050_1844 + 0x7);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  *(undefined2 *)((int)param_1 + 0xa0) = 0x0;
  return;
}



void __stdcall16far enable_win_1040_d6be(ulong param_1,HWND16 param_2)

{
  GetDlgItem16(param_2,0x1);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x2);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(int)s_vrpal_bmp_1050_183a + 0x7);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(int)s_dibtext_bmp_1050_1844 + 0x4);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(int)s_dibtext_bmp_1050_1844 + 0x5);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(int)s_dibtext_bmp_1050_1844 + 0x6);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(int)s_dibtext_bmp_1050_1844 + 0x7);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  *(undefined2 *)((int)param_1 + 0xa0) = 0x1;
  return;
}




void __stdcall16far send_ldg_item_msg_1040_d79c(ulong param_1,ushort param_2)

{
  undefined2 uVar1;
  uchar *in_DX;
  ushort uVar2;
  uint uVar3;
  int iVar4;
  int unaff_DI;
  undefined2 uVar5;
  HWND16 hwnd;
  uchar in_AF;
  LRESULT LVar6;
  ulong uStack270;
  ulong uStack266;
  char local_106 [0x100];
  ushort *puStack6;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_2,in_DX,unaff_DI);
  uVar2 = (ushort)((ulong)puStack6 >> 0x10);
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  pass1_1010_c3c2((ushort)puStack6,uVar2,CONCAT22(param_2,local_106),*(ulong *)(iVar4 + 0x98),(uchar *)uVar2,in_AF,
                  param_2);
  SendDlgItemMessage16(0x1010,(INT16)local_106,param_2,0x0,0x1846000c);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1847000b);
  LVar6 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18470405);
  uVar3 = (uint)((ulong)LVar6 >> 0x10);
  uVar1 = (undefined2)LVar6;
  hwnd = 0x1010;
  pass1_1010_9044(*(ulong *)(iVar4 + 0x9c));
  uStack266 = CONCAT22(uVar3,uVar1);
  for (uStack270 = 0x0; uStack270 < uStack266; uStack270 = uStack270 + 0x1) {
    hwnd = 0x1010;
    pass1_1010_9130(*(ulong *)(iVar4 + 0x9c),(uchar *)CONCAT22(param_2,local_106),(uint)local_106,uVar3,param_2,in_AF);
    if (local_106[0] != '\0') {
      hwnd = (HWND16)s_tile2_bmp_1050_1538;
      LVar6 = SendDlgItemMessage16(0x1010,(INT16)local_106,param_2,0x0,0x18470401);
      uVar3 = (uint)((ulong)LVar6 >> 0x10);
    }
  }
  SendDlgItemMessage16(hwnd,0x0,0x0,0x1,0x1847000b);
  return;
}

void __stdcall16far pass1_1040_d29c(ulong param_1,ushort param_2)

{
  send_ldg_item_msg_1040_d79c(param_1,param_2);
  return;
}
