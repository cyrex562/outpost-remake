
fn show_win_1040_0766(astruct_1 *param_1,param_2: u16)
{
  let in_DX: *mut u8
  let puVar1: *mut u8
  let unaff_DI: i16;
  let unaff_SS: u16;
  let puVar2: *mut u16;
  let piVar3: *mut i16;
  let piVar4: *mut i16;
  let uVar5: u16;
  let local_a: i16;
  let local_8: i16;
  let uStack6: u32;
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  uStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,unaff_SS,in_DX,unaff_DI);
  puVar1 = (uStack6 >> 0x10);
  pass1_1010_6118(uStack6);
  piVar4 = &local_8;
  piVar3 = &local_a;
  uVar5 = unaff_SS;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,puVar1,unaff_DI);
  pass1_1008_3e94((u16 *)(puVar2 & 0xffff0000 | (puVar2 + 0xe)),
                  CONCAT22(unaff_SS,piVar3),CONCAT22(uVar5,piVar4));
  move_win_1040_826c(param_1,local_a + 0x8c,local_8 + 0xb9);
  ShowWindow16(0x1008,0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
win_ui_op_1040_07dc(param_1: u16,param_2: u16,param_3: u16,param_4: u16,
                   param_5: u16,param_6: u16,HWND16 param_7,param_8: u16)

{
  code **ppcVar1;
  let IVar2: i16;
  let BVar3: bool;
  let puVar4: *mut u8
  let puVar5: *mut u8
  let unaff_DI: i16;
  let uVar6: u16;
  let puVar7: *mut u16;
  let puVar8: u32;
  let uVar9: u8;
  let uVar10: u8;
  let uStack2060: u32;
  char local_806 [0x400];
  let local_406: [u32;0x100];
  let uStack6: u32;
  
  uStack6 = 0x0;
  if (param_5 == 0x73) {
    enable_window_1040_0acc(param_1,param_2,0x0,param_7);
    puVar4 = pass1_1008_5fd8(param_8,param_6);
    uStack2060 = CONCAT22(param_6,puVar4);
    puVar5 = param_6;
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_806,param_8);
    IVar2 = MessageBox16(0x1010,(LPCSTR)(s_New_failed_in_Op__Op_1050_0020 + 0x14),
                         local_806,param_8);
    local_406[0] = uStack2060;
    uVar6 = 0x1000;
    fn_ptr_1000_17ce((astruct_18 *)CONCAT22(param_6,puVar4),0x1000);
    if (IVar2 == 0x6) {
      uVar6 = SUB42(s_tile2_bmp_1050_1538,0x0);
      PostMessage16(0x1000,0x0,0x0,0x11100cb);
      BVar3 = post_win_msg_1040_7b3c
                        (CONCAT22(param_2,param_1),param_3,param_4,0x1,
                         s_tile2_bmp_1050_1538);
      uStack6 = CONCAT22(puVar5,BVar3);
    }
  }
  else {
    uVar9 = (u8)(param_2 >> 0x8);
    if (param_5 < 0x74) {
      if (param_5 == 0x6e) {
        (_PTR_LOOP_1050_5b7c + 0xae) = 0x99;
        puVar8 = 
                 pass1_1038_af40(_PTR_LOOP_1050_5b7c,(param_1 + 0x6),0x2,
                                 param_6,param_1,&PTR_LOOP_1050_1038,param_8);
        ppcVar1 = (code **)(*puVar8 + 0x3c);
        (**ppcVar1)(&PTR_LOOP_1050_1038,puVar8,(puVar8 >> 0x10));
        SetFocus16((HWND16)&PTR_LOOP_1050_1038);
        return;
      }
      if (0x6e < param_5) {
LAB_1040_09f9:
        post_win_msg_1040_7b3c
                  (CONCAT13(uVar9,CONCAT12((char)param_2,param_1)),param_3,
                   param_4,param_5,param_7);
        return;
      }
      if ((char)param_5 == '\x02') {
LAB_1040_09b4:
        post_win_msg_1040_7b3c
                  (CONCAT13(uVar9,CONCAT12((char)param_2,param_1)),0x0,0x0,0x2,
                   param_7);
        PostMessage16(param_7,0x0,0x0,0x11100ee);
        return;
      }
      if ((char)param_5 != 'd') goto LAB_1040_09f9;
      uVar9 = 0x0;
      uVar10 = 0x0;
      uVar6 = SUB42(s_tile2_bmp_1050_1538,0x0);
      PostMessage16(param_7,0x0,0x0,0x1110064);
      goto LAB_1040_0821;
    }
    if (param_5 != 0x74) {
      if (param_5 == 0xee) goto LAB_1040_09b4;
      if (param_5 == 0x13d) {
        enable_window_1040_0acc(param_1,param_2,0x1,param_7);
        return;
      }
      goto LAB_1040_09f9;
    }
    enable_window_1040_0acc(param_1,param_2,0x0,param_7);
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_406,
               param_8);
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_806,param_8);
    uVar6 = SUB42(s_tile2_bmp_1050_1538,0x0);
    IVar2 = MessageBox16(0x1010,(LPCSTR)(s_New_failed_in_Op__Op_1050_0020 + 0x14),
                         (LPCSTR)local_406,param_8);
    if (IVar2 == 0x6) {
      PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x111007a);
      BVar3 = post_win_msg_1040_7b3c
                        (CONCAT22(param_2,param_1),param_3,param_4,0x1,
                         s_tile2_bmp_1050_1538);
      uStack6 = CONCAT22(param_6,BVar3);
      puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_8,param_6,unaff_DI);
      uVar6 = 0x1010;
      pass1_1010_60fa(puVar7);
    }
  }
  uVar9 = 0x1;
  uVar10 = 0x0;
LAB_1040_0821:
  enable_window_1040_0acc(param_1,param_2,CONCAT11(uVar10,uVar9),uVar6);
  return;
}


fn enable_window_1040_0acc(param_1: u16,param_2: u16,bool param_3,HWND16 param_4)
{
  let BVar1: bool;
  
  BVar1 = IsWindow16(param_4);
  if (BVar1 != 0x0) {
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x64);
    BVar1 = IsWindow16((HWND16)s_tile2_bmp_1050_1538);
    if (BVar1 != 0x0) {
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,param_3);
      GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x74);
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,param_3);
      GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x73);
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,param_3);
      GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x6e);
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,param_3);
      GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0xee);
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,param_3);
    }
  }
  return;
}


fn show_win_1040_0c7c(astruct_1 *param_1,param_2: u16,param_3: u16)
{
  let uVar1: u32;
  let local_6: u32;
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  uVar1 = (param_1 + 0x8e);
  pass1_1010_4f30(uVar1,(uVar1 >> 0x10),
                  CONCAT22(param_3,&local_6),
                  CONCAT22(param_3,&local_6 + 0x2));
  move_win_1040_826c(param_1,local_6,(bool)(local_6 >> 0x10));
  ShowWindow16(0x1010,0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn set_text_bk_color_1040_0cc0(param_1: *mut u32,param_2: u16,param_3: u16,INT16 param_4) -> u32

{
  code **ppcVar1;
  let iVar2: i16;
  HDC16 obj;
  HDC16 hdc;
  let uVar3: u32;
  let uVar4: u16;
  HGDIOBJ16 HStack4;
  
  uVar4 = 0x4;
  obj = (HDC16)s_tile2_bmp_1050_1538;
  HStack4 = GetStockObject16(param_4);
  if (_PTR_LOOP_1050_5cd0 == 0x0) {
    ppcVar1 = (code **)(*param_1 + 0x68);
    uVar3 = (**ppcVar1)(s_tile2_bmp_1050_1538,param_1,
                        (param_1 + 0x6e),uVar4);
    obj = 0x1008;
    uVar3 = pass1_1008_4d72(uVar3);
    uVar4 = (uVar3 >> 0x10);
    iVar2 = uVar3;
    _PTR_LOOP_1050_5cd0 =
         CONCAT22(CONCAT11(0x2,*(iVar2 + 0x94)),
                  CONCAT11(*(iVar2 + 0x95),*(iVar2 + 0x96)));
  }
  hdc = obj;
  if (0x3 < param_3) {
    if (param_3 == 0x4) {
      hdc = (HDC16)s_tile2_bmp_1050_1538;
      HStack4 = GetStockObject16(obj);
    }
    else {
      if ((param_3 == 0x4) || (0x1 < param_3 - 0x5)) {
        return 0x0;
      }
    }
  }
  SetTextColor16(hdc,(COLORREF)_PTR_LOOP_1050_5cd0);
  SetBkColor16((HDC16)s_tile2_bmp_1050_1538,0x0);
  return CONCAT22(0x1050,HStack4);
}



fn post_win_msg_1040_0d5e(param_1: u16,param_2: u16,param_3: i16,HWND16 param_4)
{
  if (param_3 != 0x0) {
    PostMessage16(param_4,0x0,0x0,0x1110001);
  }
  return;
}


fn set_win_pos_1040_0f10(HWND16 param_1,param_2: u16,param_3: i16)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let iVar3: i16;
  let unaff_DI: i16;
  let uVar4: u16;
  let uVar5: u32;
  let puVar6: *mut u16;
  let check: u16;
  
  dialog_ui_fn_1040_78e2(*(astruct_1 **)(param_3 + 0x6),param_1);
  uVar2 = (param_3 + 0x6);
  uVar4 = (uVar2 >> 0x10);
  iVar3 = uVar2;
  if ((iVar3 + 0x98) == 0x0) {
    GetWindowRect16(param_1,(RECT16 *)(param_3 + -0x24));
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1830);
    GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)(param_3 + -0x2c));
    piVar1 = (i16 *)(param_3 + -0x20);
    *piVar1 = *piVar1 - (param_3 + -0x24);
    iVar3 = ((param_3 + -0x2a) - (param_3 + -0x22)) + -0x2;
    (param_3 + -0x1e) = iVar3;
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x6,iVar3,*(INT16 *)(param_3 + -0x20),0x0
                   ,0x0,0x0);
    CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,0x1,0x1c1);
    uVar2 = (param_3 + 0x6);
    uVar2 = (uVar2 + 0x8e);
    (uVar2 + 0xa) = 0x2;
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1830);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  }
  else {
    uVar2 = (iVar3 + 0x92);
    uVar5 = struct_op_1030_73a8((uVar2 + 0x6));
    (param_3 + -0x32) = uVar5;
    (param_3 + -0x30) = (uVar5 >> 0x10);
    uVar2 = (param_3 + -0x32);
    if ((uVar2 + 0x20) == 0x2) {
      check = 0x1c1;
    }
    else {
      check = 0x1c2;
    }
    CheckDlgButton16(0x1030,0x1,check);
  }
  GetCursorPos16((POINT16 *)s_tile2_bmp_1050_1538);
  GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,(RECT16 *)(param_3 + -0xc));
  iVar3 = (param_3 + -0x8) - (param_3 + -0xc);
  (param_3 + -0x12) = iVar3;
  (param_3 + -0xe) = -(iVar3 / 0x2 - (param_3 + -0x4));
  iVar3 = (param_3 + -0x6) - (param_3 + -0xa);
  (param_3 + -0x14) = iVar3;
  (param_3 + -0x10) = -(iVar3 / 0x2 - (param_3 + -0x2));
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_2,(iVar3 >> 0xf),
                           unaff_DI);
  uVar4 = (puVar6 >> 0x10);
  iVar3 = puVar6;
  (param_3 + -0x1c) = iVar3;
  (param_3 + -0x1a) = uVar4;
  (param_3 + -0x16) = (iVar3 + 0xa);
  (param_3 + -0x18) = (iVar3 + 0xc);
  if ((param_3 + -0x16) < (param_3 + -0x12) + (param_3 + -0xe)) {
    (param_3 + -0xe) = (param_3 + -0x16) - (param_3 + -0x12);
  }
  if ((param_3 + -0x18) < (param_3 + -0x14) + (param_3 + -0x10)) {
    (param_3 + -0x10) = (param_3 + -0x18) - (param_3 + -0x14);
  }
  uVar2 = (param_3 + -0x10);
  SetWindowPos16(0x1010,0x45,0x0,0x0,uVar2,(uVar2 >> 0x10),0x0);
  return;
}


fn win_ui_op_1040_12bc(astruct_1 *param_1,param_2: u16,uchar *param_3)
{
  let uVar1: u32;
  WPARAM16 wparam;
  HWND16 HVar2;
  let uVar3: u16;
  let in_AF: u8;
  char *pcVar4;
  uchar local_54 [0x52];
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  uVar1 = (param_1 + 0x8e);
  uVar3 = (uVar1 >> 0x10);
  sys_1000_3f9c(local_54,param_3,0x5cd4,&USHORT_1050_1050,
                (uVar1 + 0xa),&stack0xfffe,uVar3,0x1000,param_3,in_AF);
  GetDlgItem16(0x1000,0xfd2);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,local_54,(WPARAM16)param_3,0xc0000);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0xffff,0x4010000);
  move_win_1040_826c(param_1,-0x1,0xffff);
  pcVar4 = load_string_1010_847e
                     (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  wparam = (WPARAM16)(pcVar4 >> 0x10);
  HVar2 = GetDlgItem16(0x1010,(s_vrpal_bmp_1050_183a + 0x5));
  send_msg_1040_1696(param_1,HVar2,param_3,s_tile2_bmp_1050_1538);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,pcVar4,wparam,0x40dffff);
  HVar2 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(s_vrpal_bmp_1050_183a + 0x4))
  ;
  send_msg_1040_1696(param_1,HVar2,param_3,s_tile2_bmp_1050_1538);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,pcVar4,wparam,0x40dffff);
  ShowWindow16((HWND16)s_tile2_bmp_1050_1538,0x5);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn win_msg_op_1040_13b2(param_1: u32,param_2: i16,HWND16 param_3,param_4: u16)
{
  code **ppcVar1;
  let uVar2: u32;
  let uVar3: u16;
  let iVar4: i16;
  let puVar5: *mut u8;
  let puVar6: *mut u8
  let puVar7: *mut u8
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  let in_AF: u8;
  LRESULT LVar11;
  let puStack562: u32;
  let local_22e: [u8;118];
  let uStack278: u32;
  let uStack274: u32;
  let puStack270: *mut u8;
  let puStack268: *mut u8
  let uStack266: u32;
  let uStack262: u16;
  char *pcStack260;
  let local_100: [u8;52];
  let iStack174: i16;
  HWND16 HStack172;
  uchar local_aa [0x52];
  let uStack88: u16;
  HWND16 HStack86;
  let local_54: [u8;52];
  
  iVar4 = param_1;
  uVar9 = (param_1 >> 0x10);
  if (param_2 != 0x0) {
    HStack86 = GetDlgItem16(param_3,0xfd2);
    SendMessage16((HWND16)s_tile2_bmp_1050_1538,local_54,param_4,0xd0051);
    uStack88 = pass1_1000_3e2c(CONCAT22(param_4,local_54));
    HStack172 = GetDlgItem16(0x1000,(s_vrpal_bmp_1050_183a + 0x4));
    LVar11 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4070000);
    iStack174 = LVar11;
    if (iStack174 != -0x1) {
      SendMessage16((HWND16)s_tile2_bmp_1050_1538,local_aa,param_4,
                    CONCAT22(0x408,iStack174));
    }
    HStack172 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,
                             (s_vrpal_bmp_1050_183a + 0x5));
    LVar11 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4070000);
    iStack174 = LVar11;
    if (iStack174 != -0x1) {
      SendMessage16((HWND16)s_tile2_bmp_1050_1538,local_100,param_4,
                    CONCAT22(0x408,iStack174));
    }
    pcStack260 = load_string_1010_847e
                           (_PTR_LOOP_1050_14cc,
                            (_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    puVar6 = local_aa;
    uVar3 = pass1_1000_3d7a(CONCAT22(param_4,puVar6),CONCAT22(param_4,local_100));
    if (uVar3 != 0x0) {
      uVar3 = pass1_1000_3d7a(CONCAT22(param_4,local_aa),pcStack260);
      if (uVar3 != 0x0) {
        uVar3 = pass1_1000_3d7a(CONCAT22(param_4,local_100),pcStack260);
        if (uVar3 != 0x0) {
          pass1_1010_531c((iVar4 + 0x8e),CONCAT22(param_4,local_aa),
                          local_aa,puVar6,param_4);
          puVar5 = local_100;
          pass1_1010_52fc((iVar4 + 0x8e),CONCAT22(param_4,puVar5),puVar5
                          ,puVar6,param_4);
          pass1_1010_5120((iVar4 + 0x8e),uStack88,puVar5,puVar6,
                          param_4);
          uStack266 = CONCAT22(uStack266._2_2_,puVar5);
          if (puVar5 == 0x0) {
            mem_op_1000_179c(0xb4,puVar6,0x1000);
            puVar7 = (puVar6 | puVar5);
            puStack270 = puVar5;
            puStack268 = puVar6;
            if (puVar7 == 0x0) {
              iVar4 = 0x0;
              puVar7 = 0x0;
            }
            else {
              iVar4 = string_1040_8520((astruct_57 *)CONCAT22(puVar6,puVar5),
                                       PTR_LOOP_1050_0396,0x30,0x2,0x57b,0x7d2,
                                       puVar7,param_4);
            }
            puStack562 = CONCAT22(puVar7,iVar4);
            ppcVar1 = (code **)(*puStack562 + 0x74);
            (**ppcVar1)(0x1000,iVar4,puVar7);
            return;
          }
          uVar2 = (iVar4 + 0x8e);
          uStack274 = (uVar2 + 0x12);
          uVar2 = (iVar4 + 0x8e);
          uVar10 = (uVar2 >> 0x10);
          iVar8 = uVar2;
          uStack278 = (iVar8 + 0x16);
          uVar2 = (iVar4 + 0x8e);
          uStack262 = (uVar2 + 0xa);
          pass1_1028_8d9e((astruct_100 *)CONCAT22(param_4,local_22e),uStack262,
                          uStack274,
                          uStack278 & 0xffff | (iVar8 + 0x18) << 0x10,
                          param_4,in_AF);
          fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_4,local_22e));
          param_3 = &USHORT_1050_1028;
          pass1_1028_8dec((u16 *)CONCAT22(param_4,local_22e));
          goto LAB_1040_1619;
        }
      }
    }
    param_3 = 0x1000;
    mem_op_1000_179c(0xb4,puVar6,0x1000);
    puVar7 = (puVar6 | uVar3);
    puStack270 = uVar3;
    puStack268 = puVar6;
    if (puVar7 == 0x0) {
      iVar4 = 0x0;
      puVar7 = 0x0;
    }
    else {
      iVar4 = string_1040_8520((astruct_57 *)CONCAT22(puVar6,uVar3),
                               PTR_LOOP_1050_0396,0x30,0x2,0x57b,0x755,puVar7,
                               param_4);
    }
    uStack266 = CONCAT22(puVar7,iVar4);
    ppcVar1 = (code **)(*uStack266 + 0x74);
    (**ppcVar1)(0x1000,iVar4,puVar7);
  }
LAB_1040_1619:
  DestroyWindow16(param_3);
  return;
}



u32 
set_win_pos_1040_162a
          (param_1: u16,param_2: u32,param_3: u32,param_4: u16,HWND16 param_5)

{
  let uVar1: u16;
  let BVar2: bool;
  RECT16 local_a;
  let iStack6: i16;
  
  if ((param_3._2_2_ != (s_vrpal_bmp_1050_183a + 0x5)) &&
     (param_3._2_2_ != (s_vrpal_bmp_1050_183a + 0x4))) {
    BVar2 = post_win_msg_1040_7b3c
                      (CONCAT22(param_2,param_1),param_2._2_2_,
                       param_3,param_3._2_2_,param_5);
    return CONCAT22(param_4,BVar2);
  }
  if (param_3 == 0x7) {
    GetWindowRect16(param_5,&local_a);
    iStack6 -= local_a.x;
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x2,0x50,iStack6,0x0,0x0,0x0);
  }
  else {
    if ((param_3 != 0x9) && (param_3 != 0xa)) {
      uVar1 = 0x0;
      goto LAB_1040_164d;
    }
  }
  uVar1 = 0x1;
LAB_1040_164d:
  return uVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn send_msg_1040_1696(param_1: u32,param_2: u16,param_3: u16,HWND16 param_4)
{
  let uVar1: u32;
  let uVar2: u32;
  let puVar3: *mut u16;
  let puVar4: *mut u8
  let puVar5: *mut u8
  let uVar6: u16;
  LRESULT LVar7;
  astruct_18 *paVar8;
  char *pcVar9;
  let uVar10: u16;
  let uVar11: u16;
  let uStack18: u16;
  let local_4: u16;
  
  SendMessage16(param_4,0x0,0x0,0x40b0000);
  LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xb0000);
  puVar4 = (LVar7 >> 0x10);
  local_4 = 0x0;
  puVar3 = &local_4;
  uVar6 = (param_1 >> 0x10);
  pass1_1010_519a((param_1 + 0x8e),(i16 *)CONCAT22(param_3,puVar3),puVar4,
                  param_3);
  puVar5 = puVar4;
  for (uStack18 = 0x0; uStack18 < local_4; uStack18 += 0x1) {
    uVar1 = (puVar3 + uStack18 * 0x2);
    uVar10 = 0x0;
    uVar11 = 0x403;
    uVar2 = (param_1 + 0x8e);
    paVar8 = (astruct_18 *)
             string_1010_5286(uVar2,(uVar2 >> 0x10),uVar1,
                              uVar1,puVar5);
    LVar7 = SendMessage16(0x1010,paVar8,(WPARAM16)(paVar8 >> 0x10),
                          CONCAT22(uVar11,uVar10));
    puVar5 = (LVar7 >> 0x10);
    fn_ptr_1000_17ce(paVar8,0x1000);
  }
  uVar6 = 0x0;
  uVar10 = 0x40a;
  pcVar9 = load_string_1010_847e
                     (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  SendMessage16(0x1010,pcVar9,(WPARAM16)(pcVar9 >> 0x10),
                CONCAT22(uVar10,uVar6));
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xb0001);
  return;
}


fn show_win_1040_18a2(astruct_1 *param_1,HWND16 param_2,param_3: *mut u16)
{
  let uVar1: u32;
  CHAR local_304 [0x100];
  char local_204 [0x100];
  char local_104 [0x100];
  let uStack4: u16;
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  check_dialog_btn_1040_1afe(param_1);
  if (PTR_LOOP_1050_13ae != 0x0) {
    if (PTR_LOOP_1050_13ae == &PTR_LOOP_1050_0002) {
      uStack4 = 0x621;
    }
    else {
      if (PTR_LOOP_1050_13ae == (&PTR_LOOP_1050_0002 + 0x1)) {
        uStack4 = 0x622;
      }
      else {
        if (PTR_LOOP_1050_13ae == &DAT_1050_0004) {
          uStack4 = 0x623;
        }
        else {
          uStack4 = 0x620;
        }
      }
    }
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,(short)param_3
              );
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_204,(short)param_3
              );
    wsprintf16((LPSTR)0x1010,local_304,param_3);
    SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,local_304,(SEGPTR)param_3);
    uVar1 = (param_1 + 0x8e);
    if ((uVar1 + 0x82) == 0x0) {
      uStack4 = 0x627;
    }
    else {
      uStack4 = 0x626;
    }
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,(short)param_3
              );
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_204,(short)param_3
              );
    wsprintf16((LPSTR)0x1010,local_304,param_3);
    param_2 = s_tile2_bmp_1050_1538;
    SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,local_304,(SEGPTR)param_3);
  }
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(param_2,0x5);
  return;
}



fn unk_win_ui_op_1040_19ea(astruct_32 *param_1,param_2: i16,HWND16 param_3)
{
  let uVar1: u32;
  let UVar2: u16;
  let in_DX: *mut u8
  astruct_32 *iVar4;
  let unaff_DI: i16;
  astruct_32 *uVar3;
  let unaff_SS: u16;
  
  iVar4 = (astruct_32 *)param_1;
  uVar3 = (astruct_32 *)(param_1 >> 0x10);
  if (param_2 != 0x0) {
    UVar2 = IsDlgButtonChecked(param_3,0xfdb);
    pass1_1010_5d9c(iVar4->field_0x8e,UVar2,in_DX,unaff_DI,unaff_SS);
    UVar2 = IsDlgButtonChecked(0x1010,0xfdc);
    uVar1 = iVar4->field_0x8e;
    (uVar1 + 0x20) = UVar2;
    UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,0xfdd);
    uVar1 = iVar4->field_0x8e;
    (uVar1 + 0x74) = UVar2;
    param_3 = s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked((HWND16)s_tile2_bmp_1050_1538,0xfde);
    uVar1 = iVar4->field_0x8e;
    (uVar1 + 0x72) = UVar2;
    if (iVar4->field_0x92 != 0x0) {
      uVar1 = iVar4->field_0x8e;
      param_3 = 0x1000;
      pass1_1000_4906((astruct_20 *)(uVar1 & 0xffff0000 | (uVar1 + 0x22)),
                      (WNDCLASS16 *)0x0,0x40);
    }
    if (iVar4->field_0x94 != 0x0) {
      param_3 = 0x1010;
      pass1_1010_60a0(iVar4->field_0x8e);
    }
  }
  DestroyWindow16(param_3);
  return;
}


fn check_dialog_btn_1040_1afe(param_1: u32)
{
  let id: i16;
  let id_00: i16;
  let id_01: i16;
  let uVar1: u32;
  let uVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  HWND16 unaff_CS;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  uVar1 = (iVar3 + 0x8e);
  uVar2 = (iVar3 + 0x8e);
  id = *(INT16 *)(uVar2 + 0x20);
  uVar2 = (iVar3 + 0x8e);
  id_00 = *(INT16 *)(uVar2 + 0x74);
  uVar2 = (iVar3 + 0x8e);
  id_01 = *(INT16 *)(uVar2 + 0x72);
  CheckDlgButton16(unaff_CS,*(INT16 *)(uVar1 + 0x1e),0xfdb);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id_00,0xfdd);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id_01,0xfde);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id,0xfdc);
  return;
}



fn check_dialog_btn_1040_1b8a(void)
{
  let id: u16;
  let id_00: u16;
  let id_01: u16;
  let id_02: u16;
  
  id = pass1_1010_60b4();
  id_00 = pass1_1010_60c6();
  id_01 = pass1_1010_60c0();
  id_02 = pass1_1010_60ba();
  CheckDlgButton16(0x1010,id,0xfdb);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id_01,0xfdd);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id_02,0xfde);
  CheckDlgButton16((HWND16)s_tile2_bmp_1050_1538,id_00,0xfdc);
  return;
}




fn show_win_1040_1d50(astruct_1 *param_1,HWND16 param_2)
{
  dialog_ui_fn_1040_78e2(param_1,param_2);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(param_2,0x5);
  return;
}



fn unk_win_ui_op_1040_1d7a(astruct_33 *param_1,param_2: i16,HWND16 param_3)
{
  let uVar1: u32;
  let UVar2: u16;
  astruct_33 *iVar3;
  astruct_33 *uVar3;
  HWND16 HVar3;
  HWND16 HVar4;
  let unaff_SS: u16;
  
  iVar3 = (astruct_33 *)param_1;
  uVar3 = (astruct_33 *)(param_1 >> 0x10);
  if ((param_2 != 0x0) && (uVar1 = iVar3->field_0x8e, (uVar1 + 0x72) != 0x0))
  {
    HVar3 = s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked(param_3,0xe1);
    if (UVar2 != 0x0) {
      HVar3 = 0x1008;
      pass1_1008_a930(iVar3->field_0x92,0x1d5,unaff_SS);
    }
    HVar4 = s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked(HVar3,0xe2);
    if (UVar2 != 0x0) {
      HVar4 = 0x1008;
      pass1_1008_a930(iVar3->field_0x92,0x1d6,unaff_SS);
    }
    HVar3 = s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked(HVar4,0xe3);
    if (UVar2 != 0x0) {
      HVar3 = 0x1008;
      pass1_1008_a930(iVar3->field_0x92,0x1d7,unaff_SS);
    }
    HVar4 = s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked(HVar3,0xe5);
    if (UVar2 != 0x0) {
      HVar4 = 0x1008;
      pass1_1008_a930(iVar3->field_0x92,0x1d8,unaff_SS);
    }
    HVar3 = s_tile2_bmp_1050_1538;
    UVar2 = IsDlgButtonChecked(HVar4,0xe6);
    if (UVar2 != 0x0) {
      HVar3 = 0x1008;
      pass1_1008_a930(iVar3->field_0x92,0x1e2,unaff_SS);
    }
    UVar2 = IsDlgButtonChecked(HVar3,0xe7);
    if (UVar2 != 0x0) {
      pass1_1008_a930(iVar3->field_0x92,0x1dc,unaff_SS);
    }
    return;
  }
  DestroyWindow16(param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn create_win_1040_20d4(param_1: u16,param_2: u16,param_3: u16,astruct_1 *param_4)
{
  let y: i16;
  let unaff_DI: i16;
  let uVar1: u16;
  let puVar2: *mut u16;
  RECT16 local_1e;
  let iStack26: i16;
  let iStack24: i16;
  let uStack22: u32;
  let uStack18: u32;
  let iStack14: i16;
  let uStack12: u16;
  let uStack10: i16;
  let iStack8: i16;
  let uStack6: u16;
  let iStack4: i16;
  
  dialog_ui_fn_1040_78e2(param_4,param_2);
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_1,param_3,unaff_DI);
  uStack12 = (puVar2 >> 0x10);
  iStack14 = puVar2;
  iStack8 = (iStack14 + 0xa);
  iStack10 = (iStack14 + 0xc);
  uVar1 = (param_4 >> 0x10);
  uStack18 = pass1_1008_4772(*(astruct_76 **)(param_4 + 0x8e));
  y = (uStack18 + 0x4);
  iStack4 = (iStack8 - y) / 0x2;
  uStack6 = 0x5;
  SetWindowPos16(0x1008,0x6,0x1d6,y,0x5,iStack4,0x0);
  GetClientRect16((HWND16)s_tile2_bmp_1050_1538,&local_1e);
  load_string_1010_847e
            (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  uStack22 = 0x50010001;
  CreateWindow16((LPCSTR)0x1010,(LPCSTR)0x0,ZEXT24(PTR_LOOP_1050_038c) << 0x10,0x1,
                 *(INT16 *)(param_4 + 0x6),0x19,0x58,iStack24 - 0x28,
                 (iStack26 + -0x58) / 0x2,0x1,(LPVOID)(s_Rebel_1050_4ffc + 0x5));
  SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x45,iStack10 + -0xa,
                 *(INT16 *)(uStack18 + 0x4),0x5,iStack4,0x0);
  return;
}


fn show_win_1040_2490(astruct_1 *param_1,HWND16 param_2)
{
  code **ppcVar1;
  let uVar2: u16;
  astruct_1 *iVar4;
  let uVar3: u16;
  let piVar4: *mut i16;
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  uVar3 = (param_1 >> 0x10);
  iVar4 = (astruct_1 *)param_1;
  GetDlgItem16(param_2,0xfb1);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
                    // WARNING: Load size is inaccurate
  ppcVar1 = (code **)(*iVar4->field_0x8e + 0x10);
  piVar4 = (i16 *)(**ppcVar1)(s_tile2_bmp_1050_1538,&iVar4->field_0x8e
                             );
  uVar2 = (piVar4 >> 0x10);
  move_win_1040_826c(param_1,(piVar4 + 0x2) + -0x2,
                     (piVar4 + 0x4) + *piVar4 + 0x3);
  ShowWindow16((HWND16)s_tile2_bmp_1050_1538,0x5);
  pass1_1018_1c9a(&iVar4->field_0x8e,0x1a0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 
win_ui_op_1040_2512(param_1: *mut u32,param_2: u32,param_3: u16,HWND16 param_4,
                   WNDCLASS16 *param_5,uchar *param_6)

{
  let piVar1: *mut i16;
  code **ppcVar2;
  let uVar3: u16;
  let Bvar4: bool;
  let iVar5: i16;
  let iVar6: i16;
  let UVar7: u16;
  let puVar8: *mut u8
  let unaff_DI: i16;
  let uVar9: u16;
  let uVar10: u16;
  HWND16 hwnd;
  let in_AF: u8;
  let uVar11: u32;
  let local_1e: [u8;4];
  let uStack26: u16;
  let puStack24: *mut u8
  u16 *local_16 [0x2];
  let uStack12: u16;
  let puStack10: u32;
  let BStack6: bool;
  let puStack4: *mut u8
  
  BStack6 = 0x0;
  puStack4 = 0x0;
  if (param_3 == 0x2) {
LAB_1040_266d:
    BStack6 = 0x1;
    puStack4 = 0x0;
  }
  else {
    uVar9 = (param_1 >> 0x10);
    if (0x19d < param_3 - 0x2) {
      iVar5 = param_1;
      if (param_3 - 0x1a0 < 0x14 || param_3 == 0x1b4) {
        UVar7 = IsDlgButtonChecked(param_4,param_3);
        if (UVar7 == 0x0) {
          piVar1 = (i16 *)(iVar5 + 0x92);
          *piVar1 = *piVar1 + 0x1;
          if (0x0 < (iVar5 + 0x92)) {
            (iVar5 + 0x94) = 0x0;
          }
          uVar11 = (iVar5 + 0x8e);
          if ((uVar11 + 0x28) == (iVar5 + 0x92)) {
            GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0xfb1);
            EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
          }
        }
        else {
          piVar1 = (i16 *)(iVar5 + 0x92);
          *piVar1 = *piVar1 + -0x1;
          GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0xfb1);
          BVar4 = IsWindowEnabled16((HWND16)s_tile2_bmp_1050_1538);
          if (BVar4 == 0x0) {
            GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0xfb1);
            EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
          }
          if ((iVar5 + 0x92) < 0x1) {
            (iVar5 + 0x94) = 0x1;
          }
          pass1_1018_1c9a((iVar5 + 0x8e),param_3);
          puStack10 = pass1_1018_1e78((iVar5 + 0x8e),-0x1);
          uVar3 = (puStack10 >> 0x10);
          if (puStack10 == 0x0) {
            uStack12 = 0x0;
          }
          else {
            uStack12 = (puStack10 + 0x1c);
          }
          win_1008_5c7c(_PTR_LOOP_1050_02a0,CONCAT22(uStack12,0x1),param_5,uStack12,
                        uVar3 | puStack10);
        }
        if ((-0x1 < (iVar5 + 0x92)) &&
           (uVar11 = (iVar5 + 0x8e),
           (iVar5 + 0x92) <= (uVar11 + 0x28))) {
          sys_1000_3f9c((uchar *)local_16,param_5,0x5cf4,
                        &USHORT_1050_1050,(iVar5 + 0x92),&stack0xfffe,
                        uVar9,0x1000,param_5,in_AF);
          SetDlgItemText16(0x1000,local_16,(SEGPTR)param_5);
        }
        goto LAB_1040_266d;
      }
      uVar3 = param_3 - 0xfb1;
      if (uVar3 == 0x0) {
        if ((iVar5 + 0x92) < 0x0) {
          mem_op_1000_179c(0xb4,param_6,0x1000);
          puVar8 = (param_6 | uVar3);
          uStack26 = uVar3;
          puStack24 = param_6;
          if (puVar8 == 0x0) {
            iVar5 = 0x0;
            puVar8 = 0x0;
          }
          else {
            iVar5 = string_1040_8520((astruct_57 *)CONCAT22(param_6,uVar3),
                                     PTR_LOOP_1050_0396,0x30,0x2,0x57b,0x57c,
                                     puVar8,param_5);
          }
          puStack10 = CONCAT22(puVar8,iVar5);
          ppcVar2 = (code **)(*puStack10 + 0x74);
          (**ppcVar2)(0x1000,iVar5,puVar8);
          goto LAB_1040_27c0;
        }
        if (0x0 < (iVar5 + 0x92)) {
          mem_op_1000_179c(0xb4,param_6,0x1000);
          puVar8 = (param_6 | uVar3);
          uStack26 = uVar3;
          puStack24 = param_6;
          if (puVar8 == 0x0) {
            iVar6 = 0x0;
            puVar8 = 0x0;
          }
          else {
            iVar6 = string_1040_8520((astruct_57 *)CONCAT22(param_6,uVar3),
                                     PTR_LOOP_1050_0396,0x21,0x2,0x57b,0x57d,
                                     puVar8,param_5);
          }
          puStack10 = CONCAT22(puVar8,iVar6);
          pass1_1008_941a((u16 *)CONCAT22(param_5,local_1e),0x1,0xc2);
          ppcVar2 = (code **)(*puStack10 + 0x6c);
          uVar11 = (**ppcVar2)(0x1008,puStack10,(puStack10 >> 0x10),
                               local_1e,param_5);
          param_6 = (uVar11 >> 0x10);
          if (uVar11 == 0x2) goto LAB_1040_27c0;
        }
        local_16[0] = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x6,param_5,param_6,
                                      unaff_DI);
        param_6 = (local_16[0] >> 0x10);
        uStack12 = 0x1a0;
        hwnd = 0x1010;
        do {
          UVar7 = IsDlgButtonChecked(hwnd,uStack12);
          if (UVar7 == 0x1) {
            uVar10 = (local_16[0] >> 0x10);
            iVar6 = local_16[0];
            (iVar6 + (iVar6 + 0x56) * 0x2 + 0x4e) = uStack12;
            piVar1 = (i16 *)(iVar6 + 0x56);
            *piVar1 = *piVar1 + 0x1;
          }
          uStack12 += 0x1;
          hwnd = s_tile2_bmp_1050_1538;
        } while (uStack12 < 0x1b5);
        uVar3 = (iVar5 + 0x92);
        puStack10 = (puStack10 & 0xffff0000 | uVar3);
        uVar11 = (iVar5 + 0x8e);
        (uVar11 + 0x28) = uVar3;
        param_4 = s_tile2_bmp_1050_1538;
        PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x11100c8);
        param_3 = 0x1;
      }
    }
    BStack6 = post_win_msg_1040_7b3c
                        (param_1,param_2,(param_2 >> 0x10),param_3,param_4
                        );
    puStack4 = param_6;
  }
LAB_1040_27c0:
  return CONCAT22(puStack4,BStack6);
}


fn dlg_ui_op_1040_2a64(astruct_1 *param_1,param_2: u16,param_3: u16)
{
  let uVar1: u32;
  astruct_160 *paVar2;
  let uVar3: u16;
  let puVar4: *mut u8
  let puVar5: *mut u8
  let iVar6: i16;
  let uVar7: u16;
  HWND16 hwnd;
  HWND16 hwnd_00;
  let iVar8: i16;
  RECT16 local_16;
  let uStack18: u16;
  let uStack16: u16;
  let iStack14: i16;
  let uStack12: u32;
  let uStack8: u32;
  let iStack4: i16;
  
  unk_win_ui_op_1040_b230(param_1,param_2,param_3);
  iStack4 = 0x5;
  iVar8 = 0x0;
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  uVar1 = (iVar6 + 0x90);
  uStack12 = struct_op_1030_73a8((uVar1 + 0x6));
  puVar4 = (uStack12 >> 0x10);
  hwnd = &USHORT_1050_1028;
  PTR_LOOP_1050_5d04 = pass1_1028_4a9a(uStack12,iVar8);
  for (iStack14 = 0x0; iStack14 < iStack4; iStack14 += 0x1) {
    if (iStack14 != 0x0) {
      (&PTR_LOOP_1050_5d04 + iStack14 * 0xc) = 0x0;
    }
    iVar8 = iStack14 * 0xc;
    local_16.x = *(INT16 *)(iVar8 + 0x5cfc);
    local_16.y = *(INT16 *)(iVar8 + 0x5cfe);
    paVar2 = (astruct_160 *)(&PTR_LOOP_1050_0000 + 0x1);
    uStack18 = 0x1;
    uStack16 = 0x1;
    MapDialogRect16(hwnd,&local_16);
    hwnd_00 = 0x1000;
    mem_op_1000_179c(0x42,puVar4,0x1000);
    puVar5 = (puVar4 | paVar2);
    if (puVar5 == 0x0) {
      paVar2 = (astruct_160 *)0x0;
      puVar4 = 0x0;
    }
    else {
      hwnd_00 = 0x1008;
      pass1_1008_3bd6(paVar2,puVar4,0x1,CONCAT22(local_16.x,local_16.y),0x101,
                      0xff0100,CONCAT22((iVar6 + 0x6),
                                        (iVar8 + 0x5d00)),puVar5,
                      param_3);
      puVar4 = puVar5;
    }
    uStack8 = CONCAT22(puVar4,paVar2);
    if (PTR_LOOP_1050_5d04 == 0x0) {
      hwnd = hwnd_00;
      if ((iStack14 != 0x0) && ((puVar4 | paVar2) != 0x0)) {
        hwnd = s_tile2_bmp_1050_1538;
        EnableWindow16(hwnd_00,0x0);
      }
    }
    else {
      iVar8 = iStack14 * 0xc;
      uVar3 = pass1_1028_4a9a(uStack12,(iVar8 + 0x5d02));
      hwnd = &USHORT_1050_1028;
      if (uVar3 != 0x0) {
        (&PTR_LOOP_1050_5d04 + iVar8) = 0x1;
        uVar1 = (iVar6 + 0x98);
        SetDlgItemText16((HWND16)&USHORT_1050_1028,uVar1,
                         (SEGPTR)(uVar1 >> 0x10));
        hwnd = s_tile2_bmp_1050_1538;
      }
    }
  }
  return;
}



void 
win_ui_op_1040_2bb2(param_1: i16,param_2: u16,param_3: u16,param_4: u32,HWND16 param_5
                   )

{
  let uVar1: u16;
  let in_DX: *mut u8
  let unaff_SS: u16;
  let uVar2: u32;
  let iStack8: i16;
  let iStack4: i16;
  
  if (param_4._2_2_ == 0x158) {
    PTR_LOOP_1050_5d04 = (PTR_LOOP_1050_5d04 == 0x0);
    if (PTR_LOOP_1050_5d04 == 0x0) {
      for (iStack8 = 0x1; iStack8 < 0x5; iStack8 += 0x1) {
        GetDlgItem16(param_5,*(INT16 *)(iStack8 * 0xc + 0x5d00));
        EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
        (&PTR_LOOP_1050_5d04 + iStack8 * 0xc) = 0x0;
        uVar2 = (param_1 + 0x94);
        param_5 = s_tile2_bmp_1050_1538;
        SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,uVar2,
                         (SEGPTR)(uVar2 >> 0x10));
      }
      uVar2 = (param_1 + 0x94);
      goto LAB_1040_2ccc;
    }
    for (iStack8 = 0x1; iStack8 < 0x5; iStack8 += 0x1) {
      GetDlgItem16(param_5,*(INT16 *)(iStack8 * 0xc + 0x5d00));
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
      (&PTR_LOOP_1050_5d04 + iStack8 * 0xc) = 0x0;
      uVar2 = (param_1 + 0x94);
      param_5 = s_tile2_bmp_1050_1538;
      SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,uVar2,
                       (SEGPTR)(uVar2 >> 0x10));
    }
  }
  else {
    if (param_4._2_2_ == 0x159) {
      iStack4 = 0x1;
    }
    else {
      if (param_4._2_2_ == 0x15a) {
        iStack4 = 0x2;
      }
      else {
        if (param_4._2_2_ == 0x15b) {
          iStack4 = 0x3;
        }
        else {
          if (param_4._2_2_ != 0x15c) {
            pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,param_5,unaff_SS);
            return;
          }
          iStack4 = 0x4;
        }
      }
    }
    if (iStack4 == 0x0) {
      return;
    }
    uVar1 = ((&PTR_LOOP_1050_5d04 + iStack4 * 0xc) == 0x0);
    (&PTR_LOOP_1050_5d04 + iStack4 * 0xc) = uVar1;
    if (uVar1 == 0x0) {
      uVar2 = (param_1 + 0x94);
      goto LAB_1040_2ccc;
    }
  }
  uVar2 = (param_1 + 0x98);
LAB_1040_2ccc:
  SetDlgItemText16(param_5,uVar2,(SEGPTR)(uVar2 >> 0x10));
  return;
}



fn win_dlg_item_1040_2d48(param_1: u32,HWND16 param_2,bool param_3)
{
  let UVar1: u16;
  let value: u16;
  let local_4: bool;
  
  pass1_1040_b45e(param_1,param_2);
  UVar1 = GetDlgItemInt16(param_2,0x1,&local_4,param_3);
  value = GetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x1,&local_4,param_3);
  if (UVar1 != 0x0) {
    value /= UVar1;
  }
  SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x1,value,0x165);
  return;
}


fn show_win_1040_2f5a(astruct_1 *param_1,HWND16 param_2)
{
  dialog_ui_fn_1040_78e2(param_1,param_2);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(param_2,0x5);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn win_dlg_op_1040_2f90(param_1: u32,param_2: *mut u16)
{
  HWND16 HVar1;
  let in_DX: *mut u8
  let puVar2: *mut u8
  let uVar3: u16;
  let msg: u16;
  let iVar4: i16;
  let unaff_DI: i16;
  let uVar5: u16;
  let puVar6: *mut u16;
  let uVar7: u32;
  char *pcVar8;
  let local_116: u32;
  let local_112: u32;
  CHAR local_10e [0x82];
  let local_8c: [u8;82];
  let uStack10: u32;
  let puStack6: *mut u16;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_2,in_DX,unaff_DI);
  puVar2 = (puStack6 >> 0x10);
  uStack10 = (puStack6 + 0x68);
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  GetWindowText16(0x1010,0x80,local_8c);
  wsprintf16((LPSTR)s_tile2_bmp_1050_1538,local_10e,param_2);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_10e);
  HVar1 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x182);
  *(HWND16 *)(iVar4 + 0x92) = HVar1;
  pass1_1018_3a94((iVar4 + 0x96),CONCAT22(param_2,&local_116),
                  CONCAT22(param_2,&local_112),param_2);
  send_msg_1040_3374(param_1,local_112,(iVar4 + 0x92),0x1018);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_2,puVar2,unaff_DI);
  uVar3 = (puVar6 >> 0x10);
  uVar7 = (puVar6 + 0x24);
  uVar7 = pass1_1018_3a7a((iVar4 + 0x96),uVar7,uVar7,uVar3);
  SendMessage16(0x1018,uVar7,(WPARAM16)(uVar7 >> 0x10),0x40dffff);
  HVar1 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x183);
  *(HWND16 *)(iVar4 + 0x94) = HVar1;
  send_msg_1040_3374(param_1,local_116,HVar1,s_tile2_bmp_1050_1538);
  pcVar8 = load_string_1010_847e
                     (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  msg = (pcVar8 >> 0x10);
  SendDlgItemMessage16(0x1010,pcVar8,msg,0x0,0x1830403);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,pcVar8,msg,0xffff,0x183040d);
  HVar1 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x181);
  *(HWND16 *)(iVar4 + 0x8e) = HVar1;
  HVar1 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x184);
  *(HWND16 *)(iVar4 + 0x90) = HVar1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn win_ui_op_1040_311a(param_1: i16,param_2: u16,param_3: u16,param_4: u32)
{
  let iVar1: i16;
  let uVar2: u32;
  let puVar3: *mut u8;
  let unaff_CS: u16;
  let unaff_SS: u16;
  LRESULT LVar4;
  let puVar5: *mut u16;
  let iVar6: i16;
  
  send_msg_1040_323c(CONCAT22(param_2,param_1),unaff_CS);
  load_string_1010_847e
            (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  if (param_4._2_2_ == 0x181) {
    iVar1 = param_1 + 0x9a;
    puVar3 = param_2;
    iVar6 = iVar1;
    pass1_1018_3cda(*(u32 **)(param_1 + 0x96),CONCAT22(param_2,param_1 + 0x19a),
                    CONCAT22(param_2,iVar1));
    pass1_1018_3424((param_1 + 0x96),iVar6,puVar3,unaff_SS);
    if (iVar6 == 0x0) {
      iVar6 = 0x21;
    }
    else {
      pass1_1018_3a42((param_1 + 0x96),CONCAT22(param_2,iVar1),puVar3,unaff_SS);
      pass1_1030_8344(_PTR_LOOP_1050_5748,
                      (_PTR_LOOP_1050_5748 >> 0x10),CONCAT22(puVar3,iVar6))
      ;
      uVar2 = (iVar6 + 0x10);
      pass1_1030_8344(_PTR_LOOP_1050_5748,
                      (_PTR_LOOP_1050_5748 >> 0x10),uVar2);
      PTR_LOOP_1050_5f0c = uVar2;
      PTR_LOOP_1050_5f10 = (&PTR_LOOP_1050_0000 + 0x1);
      iVar6 = 0x25;
      PTR_LOOP_1050_5f0e = puVar3;
    }
    pass1_1038_af40(_PTR_LOOP_1050_5b7c,(param_1 + 0x8),iVar6,puVar3,
                    param_1,&PTR_LOOP_1050_1038,unaff_SS);
    LVar4 = SendMessage16((HWND16)&PTR_LOOP_1050_1038,0x0,0x0,0x1110002);
    iVar6 = 0x1;
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,unaff_SS,
                             (LVar4 >> 0x10),param_2);
    pass1_1010_038e(puVar5,iVar6,unaff_SS);
  }
  else {
    if ((param_4._2_2_ == 0x181) || (0x1 < param_4._2_2_ - 0x182U)) {
      post_win_msg_1040_7b3c
                (CONCAT22(param_2,param_1),param_3,param_4,param_4._2_2_,
                 0x1010);
      return;
    }
    set_win_pos_1040_331a(CONCAT22(param_2,param_1),param_3,param_4,0x1010);
  }
  return;
}



LRESULT  send_msg_1040_323c(param_1: u32,HWND16 param_2)

{
  WPARAM16 wparam;
  LRESULT LVar1;
  LRESULT LVar2;
  
  wparam = (WPARAM16)(param_1 >> 0x10);
  LVar1 = SendMessage16(param_2,0x0,0x0,0x4070000);
  LVar2 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4070000);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,param_1 + 0x9a,wparam,
                CONCAT22(0x408,LVar1));
  LVar1 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,param_1 + 0x19a,wparam,
                        CONCAT22(0x408,LVar2));
  return LVar1;
}



fn enable_win_1040_32a8(param_1: u32)
{
  SEGPTR lp_string;
  let BVar1: bool;
  let unaff_SS: u16;
  let uStack12: u32;
  
  lp_string = param_1 + 0x19a;
  uStack12 = param_1 & 0xffff0000 | lp_string;
  pass1_1018_3a5c((param_1 + 0x96),
                  param_1 & 0xffff0000 | (param_1 + 0x9aU),
                  param_1 & 0xffff0000 | lp_string,unaff_SS);
  SetWindowText16(0x1018,lp_string);
  BVar1 = string_1018_39d8(unaff_SS,(param_1 + 0x96),
                           param_1 & 0xffff0000 | (param_1 + 0x9aU),uStack12);
  EnableWindow16(0x1018,BVar1 & 0x1);
  return;
}



fn set_win_pos_1040_331a(param_1: u32,param_2: u16,param_3: i16,HWND16 param_4) -> bool

{
  RECT16 local_e;
  let uStack10: i16;
  let uStack6: u16;
  let iStack4: i16;
  
  iStack4 = param_3;
  uStack6 = param_2;
  if (param_3 == 0x1) {
    enable_win_1040_32a8(param_1);
  }
  else {
    if (param_3 != 0x7) {
      return 0x0;
    }
    GetWindowRect16(param_4,&local_e);
    iStack10 -= local_e.x;
    SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x2,0x50,iStack10,0x0,0x0,0x0);
  }
  return 0x1;
}



fn send_msg_1040_3374(param_1: u32,param_2: *mut u32,param_3: u16,HWND16 param_4)
{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u32;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar4: u16;
  let uVar5: u16;
  LRESULT LVar6;
  astruct_18 *paVar7;
  let uVar8: u16;
  let uStack10: u32;
  let uStack6: u32;
  
  uVar5 = SUB42(s_tile2_bmp_1050_1538,0x0);
  uVar8 = param_3;
  LVar6 = SendMessage16(param_4,0x0,0x0,0x40b0000);
  uVar2 = LVar6;
  uVar4 = param_2;
  ppcVar1 = (code **)(*param_2 + 0x10);
  (**ppcVar1)(s_tile2_bmp_1050_1538,param_2,uVar8);
  uStack6 = CONCAT22(extraout_DX,uVar2);
  uStack10 = 0x0;
  while( true ) {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar1 = (code **)(*param_2 + 0x4);
    uVar3 = uStack6;
    (**ppcVar1)(uVar5,param_2,uStack10,(uStack10 >> 0x10),uVar4);
    paVar7 = (astruct_18 *)
             pass1_1018_3a7a((param_1 + 0x96),
                             uVar3 & 0xffff | extraout_DX_00 << 0x10,uVar3,
                             extraout_DX_00);
    uVar4 = param_3;
    LVar6 = SendMessage16(0x1018,paVar7,(WPARAM16)(paVar7 >> 0x10),
                          0x4030000);
    uVar5 = 0x1000;
    fn_ptr_1000_17ce(paVar7,0x1000);
    if (LVar6 == -0x1) break;
    if (LVar6 == -0x2) {
      return;
    }
    uStack10 += 0x1;
  }
  return;
}


fn show_win_1040_355a(astruct_1 *param_1,HWND16 param_2)
{
  dialog_ui_fn_1040_78e2(param_1,param_2);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(param_2,0x5);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn set_win_text_1040_3590(param_1: u32,param_2: *mut u16)
{
  HWND16 HVar1;
  SEGPTR lp_string;
  let lp_string_00: u16;
  let in_DX: *mut u8
  let uVar2: u16;
  let iVar3: i16;
  let unaff_DI: i16;
  let uVar4: u16;
  let local_59a: [u8;4];
  let local_596: [u8;4];
  let BStack1426: bool;
  let uStack1424: u16;
  CHAR local_58e [0x82];
  CHAR local_50c [0x100];
  let uStack1036: u32;
  let puStack1032: *mut u16;
  char local_404 [0x402];
  
  puStack1032 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_2,in_DX,unaff_DI);
  uVar2 = (puStack1032 >> 0x10);
  uStack1036 = (puStack1032 + 0x68);
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  GetWindowText16(0x1010,0x80,local_50c);
  wsprintf16((LPSTR)s_tile2_bmp_1050_1538,local_58e,param_2);
  BStack1426 = SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_58e);
  sprintf_op_1018_34b6((iVar3 + 0x8e),(uchar)BStack1426);
  uStack1424 = uVar2;
  pass1_1018_3d44((iVar3 + 0x8e),CONCAT22(param_2,local_59a),
                  CONCAT22(param_2,local_596));
  HVar1 = GetDlgItem16(0x1018,0x193);
  *(HWND16 *)(iVar3 + 0x98) = HVar1;
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_404,(short)param_2);
  wsprintf16((LPSTR)0x1010,local_50c,param_2);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x195);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_50c);
  lp_string = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x196);
  sprintf_op_1018_34b6((iVar3 + 0x8e),(uchar)lp_string);
  SetWindowText16(0x1018,lp_string);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x197);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_404,(short)param_2);
  SetWindowText16(0x1010,(SEGPTR)local_404);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_404,(short)param_2);
  wsprintf16((LPSTR)0x1010,local_50c,param_2);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x198);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_50c);
  lp_string_00 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x199);
  unk_str_op_1018_35b0((iVar3 + 0x8e),param_2,lp_string_00);
  if ((uVar2 | lp_string_00) == 0x0) {
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_404,(short)param_2
              );
    SetWindowText16(0x1010,(SEGPTR)local_404);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x19a);
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_404,(short)param_2
              );
    SetWindowText16(0x1010,(SEGPTR)local_404);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
    return;
  }
  SetWindowText16(0x1018,lp_string_00);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
message_box_op_1040_37f0
          (param_1: i16,param_2: u16,param_3: u16,param_4: u32,param_5: u16,
          param_6: u16)

{
  let uVar1: u16;
  let in_DX: *mut u8
  let uVar2: u16;
  let unaff_DI: i16;
  LRESULT LVar3;
  let iVar4: i16;
  char local_40c [0x402];
  let uStack10: u32;
  let puStack6: *mut u16;
  
  if (param_4._2_2_ == 0x193) {
    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_6,in_DX,unaff_DI);
    uVar2 = (puStack6 >> 0x10);
    uStack10 = (puStack6 + 0x68);
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_40c,param_6);
    uVar1 = MessageBox16(0x1010,(LPCSTR)(s_New_failed_in_Op__Op_1050_0020 + 0x10),
                         (LPCSTR)uStack10,(uStack10 >> 0x10));
    pass1_1018_3710(*(u32 **)(param_1 + 0x8e),param_6,uVar1,uVar2);
    PostMessage16(0x1018,0x0,0x0,0x1110002);
  }
  else {
    if (param_4._2_2_ != 0x194) {
      post_win_msg_1040_7b3c
                (CONCAT22(param_2,param_1),param_3,param_4,param_4._2_2_,
                 param_5);
      return;
    }
    pass1_1038_af40(_PTR_LOOP_1050_5b7c,(param_1 + 0x8),0x21,in_DX,
                    param_1,&PTR_LOOP_1050_1038,param_6);
    LVar3 = SendMessage16((HWND16)&PTR_LOOP_1050_1038,0x0,0x0,0x1110002);
    iVar4 = 0x1;
    puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_6,
                               (LVar3 >> 0x10),unaff_DI);
    pass1_1010_038e(puStack6,iVar4,param_6);
  }
  return;
}


u16 
enable_win_1040_3a36
          (param_1: u32,param_2: u16,param_3: u16,param_4: i16,HWND16 param_5)

{
  let piVar1: *mut i16;
  let bVar2: bool;
  let iVar3: i16;
  let uVar4: u16;
  HWND16 hwnd;
  HWND16 hwnd_00;
  
  bVar2 = false;
  iVar3 = param_1;
  uVar4 = (param_1 >> 0x10);
  if (param_4 == 0x0) {
    if ((iVar3 + 0x9e) <= (iVar3 + 0x9c)) goto LAB_1040_3a79;
    piVar1 = (i16 *)(iVar3 + 0x9c);
    *piVar1 = *piVar1 + 0x1;
  }
  else {
    if (param_4 != 0x1) goto LAB_1040_3a79;
    if ((iVar3 + 0x9c) == 0x0) goto LAB_1040_3a79;
    piVar1 = (i16 *)(iVar3 + 0x9c);
    *piVar1 = *piVar1 + -0x1;
  }
  bVar2 = true;
LAB_1040_3a79:
  hwnd = param_5;
  if (bVar2) {
    hwnd = s_tile2_bmp_1050_1538;
    SetDlgItemInt16(param_5,0x0,(iVar3 + 0x9c),0x18e);
  }
  hwnd_00 = hwnd;
  if (((iVar3 + 0x9c) != 0x0) && ((iVar3 + 0xa2) == 0x0)) {
    (iVar3 + 0xa2) = 0x1;
    hwnd_00 = s_tile2_bmp_1050_1538;
    EnableWindow16(hwnd,0x1);
  }
  if (((iVar3 + 0x9c) == 0x0) && ((iVar3 + 0xa2) != 0x0)) {
    (iVar3 + 0xa2) = 0x0;
    EnableWindow16(hwnd_00,0x0);
  }
  return 0x0;
}



fn show_win_1040_3ae8(astruct_1 *param_1,param_2: u16)
{
  dialog_ui_fn_1040_78e2(param_1,param_2);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(param_2,0x5);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn win_ui_op_1040_3b1e(astruct_2 *param_1,param_2: *mut u16)
{
  let BVar1: bool;
  HWND16 HVar2;
  let in_DX: *mut u8
  let uVar3: u16;
  let uVar4: u16;
  let unaff_DI: i16;
  let uVar5: u16;
  let uVar6: u32;
  CHAR local_10e [0x82];
  CHAR local_8c [0x82];
  let uStack10: u32;
  let puStack6: *mut u16;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_2,in_DX,unaff_DI);
  uStack10 = (puStack6 + 0x68);
  uVar5 = (param_1 >> 0x10);
  uVar4 = param_1;
  GetWindowText16(0x1010,0x80,local_8c);
  wsprintf16((LPSTR)s_tile2_bmp_1050_1538,local_10e,param_2);
  SetWindowText16((HWND16)s_tile2_bmp_1050_1538,(SEGPTR)local_10e);
  uVar3 = uVar5;
  pass1_1018_3d44((uVar4 + 0x8e),
                  (param_1 & 0xffff0000 | (uVar4 + 0x92)),
                  (param_1 & 0xffff0000 | (uVar4 + 0x96)));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x80,local_10e,(short)param_2);
  wsprintf16((LPSTR)0x1010,local_8c,param_2);
  SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,local_8c,(SEGPTR)param_2);
  BVar1 = CheckRadioButton16((HWND16)s_tile2_bmp_1050_1538,0x188,0x18d,0x188);
  (uVar4 + 0xa0) = 0x188;
  uVar6 = switch_1018_3b9e((uVar4 + 0x8e),(uVar4 + 0xa0),BVar1,uVar3,
                           param_2);
  send_dlg_item_msg_1040_3f12(uVar4,uVar5,uVar6,0x1018,param_2);
  dialog_item_ui_op_1040_3e08(param_1,0x1018);
  HVar2 = GetDlgItem16(0x1018,0x186);
  *(HWND16 *)(uVar4 + 0x9a) = HVar2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
unk_win_ui_op_1040_3c64
          (param_1: i16,param_2: u16,param_3: u16,param_4: u32,param_5: u16)

{
  let UVar1: u16;
  let in_DX: u16;
  let uVar2: u16;
  let unaff_DI: i16;
  let unaff_SS: u16;
  let uVar3: u32;
  LRESULT LVar4;
  let puVar5: *mut u16;
  let iVar6: i16;
  
  if (param_4._2_2_ == 0x186) {
    LVar4 = SendDlgItemMessage16(param_5,0x0,0x0,0x0,0x1900409);
    uVar2 = (LVar4 >> 0x10);
    UVar1 = GetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x0,(bool *)0x0,0x0);
    pass1_1018_36e6((param_1 + 0x8e),UVar1,LVar4,
                    (param_1 + 0xa0));
    pass1_1038_af40(_PTR_LOOP_1050_5b7c,(param_1 + 0x8),0x22,uVar2,param_1,
                    &PTR_LOOP_1050_1038,unaff_SS);
    LVar4 = SendMessage16((HWND16)&PTR_LOOP_1050_1038,0x0,0x0,0x1110002);
    iVar6 = 0x1;
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,unaff_SS,
                             (LVar4 >> 0x10),unaff_DI);
    pass1_1010_038e(puVar5,iVar6,unaff_SS);
  }
  else {
    if (param_4._2_2_ - 0x186 < 0x2) {
LAB_1040_3c7f:
      post_win_msg_1040_7b3c
                (CONCAT22(param_2,param_1),param_3,param_4,param_4._2_2_,
                 param_5);
      return;
    }
    if (param_4._2_2_ - 0x188 < 0x5 || param_4._2_2_ == 0x18d) {
      (param_1 + 0xa0) = param_4._2_2_;
      param_5 = 0x1018;
      uVar3 = switch_1018_3b9e((param_1 + 0x8e),param_4._2_2_,param_4._2_2_,
                               in_DX,unaff_SS);
      send_dlg_item_msg_1040_3f12(param_1,param_2,uVar3,0x1018,unaff_SS);
    }
    else {
      if (param_4._2_2_ - 0x188 != 0x8) goto LAB_1040_3c7f;
      if (param_4 != 0x1) {
        return;
      }
    }
    dialog_item_ui_op_1040_3e08((astruct_2 *)CONCAT22(param_2,param_1),param_5);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn get_dc_op_1040_3d5e(param_1: u32,HWND16 param_2,param_3: u16) -> u16

{
  code **ppcVar1;
  let iVar2: i16;
  let uVar3: u16;
  astruct_43 *paVar4;
  let uVar5: u16;
  HDC16 local_4;
  
  uVar3 = (param_1 >> 0x10);
  uVar5 = (param_1 + 0x6);
  local_4 = GetDC16(param_2);
  paVar4 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,(param_1 + 0xa4),
                               param_3);
  iVar2 = paVar4;
  ppcVar1 = (code **)(iVar2 + 0x8);
  (**ppcVar1)(0x1010,paVar4,(paVar4 >> 0x10),&local_4,param_3,uVar5);
  ppcVar1 = (code **)(iVar2 + 0x4);
  (**ppcVar1)(0x1010,paVar4,0x50078,&local_4,param_3);
  ppcVar1 = (code **)(iVar2 + 0xc);
  (**ppcVar1)(0x1010,paVar4,&local_4,param_3);
  ReleaseDC16(0x1010,local_4);
  return 0x0;
}



fn invalidate_rect_1040_3ddc(astruct_2 *in_struct_1,HWND16 in_win_handle_2)
{
  let local_b_erase: u32;
  let uStack6: u32;
  
  local_b_erase = 0x780005;
  uStack6 = 0xdc0069;
  InvalidateRect16(in_win_handle_2,(RECT16 *)0x0,(bool)&local_b_erase);
  return;
}



fn dialog_item_ui_op_1040_3e08(astruct_2 *in_struct_1,param_2: u16)
{
  let UVar1: u16;
  let uVar2: u16;
  astruct_2 *local_struct_1;
  let var3: u16;
  HWND16 HVar3;
  let unaff_SS: u16;
  LRESULT LVar4;
  
  var3 = (in_struct_1 >> 0x10);
  local_struct_1 = (astruct_2 *)in_struct_1;
  CheckRadioButton16(param_2,local_struct_1->field_0xa0,0x18d,0x188);
  local_struct_1->field_0x9c = 0x0;
  local_struct_1->field_0x9e = 0x0;
  HVar3 = s_tile2_bmp_1050_1538;
  LVar4 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1900409);
  if ((LVar4 != -0x1) || (false)) {
    HVar3 = 0x1018;
    uVar2 = pass1_1018_3ab2(local_struct_1->field_0x8e,LVar4,
                            local_struct_1->field_0xa0,unaff_SS);
    local_struct_1->field_0x9e = uVar2;
  }
  SetDlgItemInt16(HVar3,0x0,local_struct_1->field_0x9c,0x18e);
  HVar3 = s_tile2_bmp_1050_1538;
  SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x0,local_struct_1->field_0x9e,0x191);
  UVar1 = local_struct_1->field_0xa0;
  if (UVar1 - 0x188 < 0x6) {
    HVar3 = &PTR_LOOP_1050_1040;
    switch(UVar1) {
    case 0x188:
      local_struct_1->field_0xa4 = 0x5;
      break;
    case 0x189:
      local_struct_1->field_0xa4 = 0x6;
      break;
    case 0x18a:
      local_struct_1->field_0xa4 = 0x7;
      break;
    case 0x18b:
      local_struct_1->field_0xa4 = 0x8;
      break;
    case 0x18c:
      local_struct_1->field_0xa4 = 0x9;
      break;
    case 0x18d:
      local_struct_1->field_0xa4 = 0xa;
    }
  }
  invalidate_rect_1040_3ddc(in_struct_1,HVar3);
  return;
}



void 
send_dlg_item_msg_1040_3f12
          (param_1: u16,param_2: u16,param_3: u32,HWND16 param_4,param_5: u16)

{
  let uVar1: u32;
  let puVar2: *mut u8;
  let extraout_DX: u16;
  let iVar3: i16;
  HWND16 hwnd;
  LRESULT LVar4;
  let local_a: [u8;8];
  
  SendDlgItemMessage16(param_4,0x0,0x0,0x0,0x190000b);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1900405);
  pass1_1008_5784(CONCAT22(param_5,local_a),param_3);
  while( true ) {
    puVar2 = local_a;
    hwnd = 0x1008;
    pass1_1008_5b12(puVar2,param_5);
    if ((extraout_DX | puVar2) == 0x0) break;
    uVar1 = (puVar2 + 0x4);
    hwnd = s_tile2_bmp_1050_1538;
    LVar4 = SendDlgItemMessage16
                      (0x1008,uVar1,(uVar1 >> 0x10),0x0,0x1900401);
    iVar3 = (LVar4 >> 0x10);
    if (((LVar4 == -0x1) && (iVar3 == -0x1)) ||
       ((LVar4 == -0x2 && (iVar3 == -0x1)))) break;
  }
  SendDlgItemMessage16(hwnd,0x0,0x0,0x0,0x1900407);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x190000b);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn win_ui_op_1040_410e(astruct_1 *param_1,param_2: u16,uchar *param_3)
{
  let uVar1: u32;
  let puVar2: *mut u8
  let iVar3: i16;
  RECT16 *unaff_DI;
  let uVar4: u16;
  let uVar5: u16;
  HWND16 hwnd;
  let in_AF: u8;
  let puVar6: *mut u16;
  let piVar7: *mut i16;
  let piVar8: *mut i16;
  let puVar9: *mut u8
  let local_36: i16;
  let local_34: i16;
  let local_32: i16;
  let local_30: [u8;6];
  i16 local_2a [0x4];
  let uStack34: u32;
  let local_1e: u32;
  let uStack26: u32;
  RECT16 local_16;
  let iStack18: i16;
  let iStack16: i16;
  HWND16 HStack14;
  uchar local_c [0xa];
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  pass1_1000_4906((astruct_20 *)CONCAT22(param_3,local_c),(WNDCLASS16 *)0x0,0xa);
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  uVar1 = (iVar3 + 0x8e);
  uVar5 = (uVar1 >> 0x10);
  sys_1000_3f9c(local_c,param_3,0x5d38,&USHORT_1050_1050,
                (uVar1 + 0x76),&stack0xfffe,uVar5,0x1000,
                param_3,in_AF);
  HStack14 = GetDlgItem16(0x1000,0xfb5);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,local_c,(WPARAM16)param_3,0xc0000);
  SetFocus16((HWND16)s_tile2_bmp_1050_1538);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0xffff,0x4010000);
  GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,&local_16);
  pass1_1000_4906((astruct_20 *)CONCAT22(param_3,&local_1e),(WNDCLASS16 *)0x0,0x8);
  uVar1 = (iVar3 + 0x8e);
  hwnd = 0x1010;
  uStack34 = 
             pass1_1010_5f7a(uVar1,(uVar1 >> 0x10),0x0,0x7);
  if (uStack34 != 0x0) {
    local_1e = *uStack34;
    unaff_DI = &local_16;
    uStack26 = (uStack34 + 0x4);
  }
  if ((local_1e._2_2_ == 0x0) && ((bool)local_1e == 0x0)) {
    puVar6 = pass1_1008_3e38((u16 *)CONCAT22(param_3,local_30));
    puVar2 = (puVar6 >> 0x10);
    uVar1 = (iVar3 + 0x96);
    pass1_1018_2678(uVar1,(uVar1 >> 0x10),
                    CONCAT22(param_3,local_30));
    pass1_1008_3e94((u16 *)CONCAT22(param_3,local_30),
                    CONCAT22(param_3,&local_32),
                    CONCAT22(param_3,local_2a));
    piVar8 = &local_34;
    piVar7 = &local_36;
    puVar9 = param_3;
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_3,puVar2,unaff_DI
                            );
    hwnd = 0x1008;
    pass1_1008_3e94((u16 *)(puVar6 & 0xffff0000 | (puVar6 + 0xe)),
                    CONCAT22(param_3,piVar7),CONCAT22(puVar9,piVar8));
    uStack26 = CONCAT22(iStack16 - local_16.y,iStack18 - local_16.x);
    local_1e = CONCAT22((((puVar6 + 0xc) * -0x14) / 0x258 -
                        (iStack16 - local_16.y)) + local_36 + local_32,
                        local_34 + local_2a[0]);
  }
  move_win_1040_826c(param_1,local_1e._2_2_,(bool)local_1e);
  ShowWindow16(hwnd,0x5);
  return;
}



fn win_ui_op_1040_42b2(param_1: u32,param_2: i16,HWND16 param_3,param_4: *mut u16)
{
  let uVar1: u32;
  let uVar2: u16;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  LRESULT LVar6;
  CHAR local_54 [0x52];
  
  iVar4 = param_1;
  uVar5 = (param_1 >> 0x10);
  if (param_2 == 0x0) {
    (iVar4 + 0x9a) = 0x1;
    DestroyWindow16(param_3);
    return;
  }
  pass1_1000_4906((astruct_20 *)CONCAT22(param_4,local_54),(WNDCLASS16 *)0x0,0x51);
  GetDlgItem16(0x1000,0xfb5);
  LVar6 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,local_54,(WPARAM16)param_4,
                        0xd0051);
  uVar3 = (LVar6 >> 0x10);
  uVar2 = pass1_1000_3e2c(CONCAT22(param_4,local_54));
  if ((uVar3 | uVar2) != 0x0) {
    (iVar4 + 0x92) = uVar2;
    (iVar4 + 0x94) = uVar3;
  }
  if (uVar3 < 0x0) {
    wsprintf16((LPSTR)&PTR_LOOP_1050_1000,local_54,param_4);
    SendMessage16((HWND16)s_tile2_bmp_1050_1538,local_54,(WPARAM16)param_4,0xc0000
                 );
    SetFocus16((HWND16)s_tile2_bmp_1050_1538);
    SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0xffff,0x4010000);
    return;
  }
  GetDlgItem16(0x1000,0x1);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  uVar1 = (iVar4 + 0x8e);
  (uVar1 + 0x76) = (iVar4 + 0x92);
  uVar1 = (iVar4 + 0x92);
  PostMessage16((HWND16)s_tile2_bmp_1050_1538,uVar1,
                (WPARAM16)(uVar1 >> 0x10),0x4000000);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  return;
}



fn get_win_rect_1040_43ea(param_1: i16,HWND16 param_2,param_3: u16,param_4: u16)
{
  let uVar1: u32;
  RECT16 local_a;
  let iStack6: i16;
  let iStack4: i16;
  
  GetWindowRect16(param_2,&local_a);
  iStack6 -= local_a.x;
  iStack4 -= local_a.y;
  pass1_1010_5fb0((param_1 + 0x8e),0x0,&local_a,param_3,0x7);
  uVar1 = (param_1 + 0x8e);
  (uVar1 + 0x7a) = ((param_1 + 0x9a) == 0x0);
  return;
}


LRESULT  send_win_msg_1040_4a0a(astruct_48 **param_1,HWND16 param_2)

{
  let piVar1: *mut i16;
  code **ppcVar2;
  let uVar3: u32;
  let uVar4: u32;
  let uVar5: u16;
  astruct_48 *iVar5;
  let uVar6: u16;
  LRESULT LVar7;
  char *pcVar8;
  let uVar9: u16;
  let uVar10: u16;
  let uStack10: i16;
  
  uVar6 = (param_1 >> 0x10);
  iVar5 = (astruct_48 *)param_1;
  ppcVar2 = (code **)(*param_1 + 0x74);
  (**ppcVar2)(param_2,param_1,0x5d6a,&USHORT_1050_1050);
  GetDlgItem16(param_2,0x1770);
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x40b0000);
  LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xb0000);
  uVar5 = (LVar7 >> 0x10);
  for (iStack10 = 0x0; uVar3 = iVar5->field_0x90, piVar1 = (i16 *)(uVar3 + 0x10),
      *piVar1 != iStack10 && iStack10 <= *piVar1; iStack10 += 0x1) {
    uVar10 = 0x0;
    uVar9 = 0x403;
    uVar3 = iVar5->field_0x90;
    uVar3 = (uVar3 + 0xc);
    pcVar8 = pass1_1040_4dcc(param_1,(uVar3 + iStack10 * 0x2),uVar5);
    LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,pcVar8,
                          (WPARAM16)(pcVar8 >> 0x10),CONCAT22(uVar9,uVar10));
    uVar5 = (LVar7 >> 0x10);
  }
  pass1_1040_4d7e(param_1);
  if (iStack10 == 0x0) {
    uVar10 = 0x40a;
    uVar4 = iVar5->field_0x90;
    uVar3 = iVar5->field_0x94;
    pcVar8 = string_op_1010_ada6(0x1010,uVar5,uVar3,(uVar3 >> 0x10)
                                 ,0x0,(uVar4 + 0xa));
    SendMessage16(0x1010,pcVar8,(WPARAM16)(pcVar8 >> 0x10),
                  CONCAT22(uVar10,iStack10));
  }
  LVar7 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xb0001);
  return LVar7;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
set_win_pos_1040_4ae4
          (param_1: i16,param_2: u16,param_3: u16,param_4: u32,param_5: u16)

{
  code **ppcVar1;
  let uVar2: u32;
  let uVar3: u16;
  astruct_18 *paVar4;
  let in_DX: *mut u8
  let puVar5: *mut u8
  let puVar6: *mut u8
  let iVar7: i16;
  let unaff_DI: i16;
  let uVar8: u16;
  let unaff_SS: u16;
  RECT16 local_24;
  let iStack32: i16;
  astruct_18 *paStack20;
  astruct_18 *paStack16;
  let iStack12: i16;
  astruct_18 *paStack10;
  astruct_20 *paStack6;
  
  if (param_4._2_2_ == 0xeb) {
    paStack6 = (astruct_20 *)
               mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,unaff_SS,in_DX,unaff_DI);
    puVar5 = (paStack6 >> 0x10);
    paVar4 = *(astruct_18 **)(param_1 + 0x90);
    if (paVar4 != (astruct_18 *)0x0) {
      paStack10 = paVar4;
      mem_op_1000_179c(0x18,puVar5,0x1000);
      uVar3 = paVar4;
      paStack16 = (astruct_18 *)(paVar4 & 0xffff | ZEXT24(puVar5) << 0x10);
      puVar6 = (puVar5 | uVar3);
      if (puVar6 == 0x0) {
        uVar3 = 0x0;
        puVar6 = 0x0;
      }
      else {
        struct_1040_a598((u16 *)(paVar4 & 0xffff | ZEXT24(puVar5) << 0x10));
      }
      (param_1 + 0x90) = uVar3;
      *(uchar **)(param_1 + 0x92) = puVar6;
      (param_1 + 0x90) = 0x7;
      iStack12 = **(i16 **)(param_1 + 0x90);
      uVar3 = iStack12 * 0xa + 0x2;
      mem_op_1000_179c(uVar3,puVar6,0x1000);
      paStack16 = (astruct_18 *)CONCAT22(puVar6,uVar3);
      if ((puVar6 | uVar3) == 0x0) {
        uVar2 = (param_1 + 0x90);
        (uVar2 + 0x2) = 0x0;
      }
      else {
        paStack16 = iStack12;
        pass1_1000_5586((uchar *)0xa564,&PTR_LOOP_1050_1040,iStack12,0xa,
                        uVar3 + 0x2,puVar6);
        uVar2 = (param_1 + 0x90);
        uVar8 = (uVar2 >> 0x10);
        iVar7 = uVar2;
        (iVar7 + 0x2) = uVar3 + 0x2;
        *(uchar **)(iVar7 + 0x4) = puVar6;
      }
      uVar8 = (paStack10 >> 0x10);
      iVar7 = paStack10;
      uVar2 = (param_1 + 0x90);
      (uVar2 + 0x6) = (iVar7 + 0x6);
      uVar2 = (param_1 + 0x90);
      (uVar2 + 0xa) = (iVar7 + 0xa);
      uVar2 = (param_1 + 0x90);
      (uVar2 + 0x12) = (iVar7 + 0x12);
      pass1_1010_a50c(paStack6,0x10505d6a,(param_1 + 0x90));
      paStack20 = paStack10;
      paStack16 = paStack10;
      if (paStack10 != (astruct_18 *)0x0) {
        pass1_1040_a5d0(paStack10);
        fn_ptr_1000_17ce(paStack10,0x1000);
      }
      ppcVar1 = (code **)(CONCAT22(param_2,param_1) + 0x70);
      (**ppcVar1)();
    }
  }
  else {
    if (param_4._2_2_ != 0x1770) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,param_5,unaff_SS);
      return;
    }
    if (param_4 == 0x7) {
      GetWindowRect16(param_5,&local_24);
      iStack32 -= local_24.x;
      SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x2,0x50,iStack32,0x0,0x0,0x0);
    }
  }
  return;
}



LRESULT  send_msg_1040_4cb2(param_1: u32,HWND16 param_2)

{
  ulet uVar1: u8;
  HWND16 HVar1;
  let in_DX: u16;
  let uVar2: u32;
  LRESULT LVar2;
  let uVar3: u16;
  let uVar4: u16;
  
  pass1_1040_b45e(param_1,param_2);
  HVar1 = GetDlgItem16(param_2,0x1770);
  uVar3 = 0xffff;
  uVar4 = 0x40d;
  pass1_1040_4d7e(param_1);
  uVar2 = pass1_1040_4dcc(param_1,HVar1,in_DX);
  LVar2 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,uVar2,
                        (WPARAM16)(uVar2 >> 0x10),CONCAT22(uVar4,uVar3));
  return LVar2;
}


fn set_win_pos_1040_4f96(astruct_1 *param_1,param_2: u16,param_3: u16,uchar *param_4)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u16;
  let uVar4: u32;
  astruct_160 *paVar5;
  let uVar6: u16;
  let uVar7: u16;
  let iVar8: i16;
  let extraout_DX: *mut u8
  let puVar9: *mut u8
  let puVar10: *mut u8
  let uVar11: u16;
  let uVar12: u16;
  astruct_443 *iVar11;
  let unaff_DI: i16;
  let uVar13: u16;
  let uVar14: u16;
  let puVar15: *mut u16;
  let BVar16: bool;
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  puVar15 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x41,param_3,param_4,unaff_DI);
  uVar14 = (puVar15 >> 0x10);
  paVar5 = (astruct_160 *)puVar15;
  uVar13 = (param_1 >> 0x10);
  iVar11 = (astruct_443 *)param_1;
  *(astruct_160 **)&iVar11->field_0x98 = paVar5;
  (&iVar11->field_0x98 + 0x2) = uVar14;
  ppcVar2 = (code **)(*iVar11->field_0x98 + 0x10);
  (**ppcVar2)(0x1010,&iVar11->field_0x98,uVar14);
  puVar10 = extraout_DX;
  mem_op_1000_179c(0xa,extraout_DX,0x1000);
  puVar9 = (puVar10 | paVar5);
  if (puVar9 == 0x0) {
    iVar11->field_0x94 = 0x0;
  }
  else {
    puVar15 = struct_1040_bf3e((u16 *)CONCAT22(puVar10,paVar5),iVar11->field_0x6);
    puVar9 = (puVar15 >> 0x10);
    paVar5 = (astruct_160 *)puVar15;
    *(astruct_160 **)&iVar11->field_0x94 = paVar5;
    *(uchar **)(&iVar11->field_0x94 + 0x2) = puVar9;
  }
  pass1_1040_bfde(iVar11->field_0x94,iVar11->field_0x98,param_3);
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar10 = (puVar9 | paVar5);
  if (puVar10 != 0x0) {
    pass1_1008_3bd6(paVar5,puVar9,0x1,0xa000a,0x0,0x800081,
                    CONCAT22(iVar11->field_0x6,0x10a),puVar10,param_3);
  }
  mem_op_1000_179c(0x42,puVar10,0x1000);
  puVar9 = (puVar10 | paVar5);
  if (puVar9 != 0x0) {
    pass1_1008_3bd6(paVar5,puVar10,0x1,0xa0028,0x0,0x840085,
                    CONCAT22(iVar11->field_0x6,0x10b),puVar9,param_3);
  }
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar10 = (puVar9 | paVar5);
  if (puVar10 != 0x0) {
    pass1_1008_3bd6(paVar5,puVar9,0x1,0xa0046,0x0,0x860087,
                    CONCAT22(iVar11->field_0x6,0x10d),puVar10,param_3);
  }
  mem_op_1000_179c(0x42,puVar10,0x1000);
  puVar9 = (puVar10 | paVar5);
  if (puVar9 != 0x0) {
    pass1_1008_3bd6(paVar5,puVar10,0x1,0xa0064,0x0,0x880089,
                    CONCAT22(iVar11->field_0x6,0x10e),puVar9,param_3);
  }
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar10 = (puVar9 | paVar5);
  if (puVar10 != 0x0) {
    pass1_1008_3bd6(paVar5,puVar9,0x1,0xa0082,0x0,0x820083,
                    CONCAT22(iVar11->field_0x6,0x10c),puVar10,param_3);
  }
  mem_op_1000_179c(0x42,puVar10,0x1000);
  puVar9 = (puVar10 | paVar5);
  if (puVar9 != 0x0) {
    pass1_1008_3bd6(paVar5,puVar10,0x1,0xa00d2,0x0,0x8a008b,
                    CONCAT22(iVar11->field_0x6,0xbbb),puVar9,param_3);
  }
  BVar16 = 0x42;
  uVar14 = 0x1000;
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar10 = (puVar9 | paVar5);
  if (puVar10 == 0x0) {
    paVar5 = (astruct_160 *)0x0;
    puVar10 = 0x0;
  }
  else {
    uVar14 = 0x1008;
    pass1_1008_3bd6(paVar5,puVar9,0x1,0xa00a0,0x8e,0x8c008d,
                    CONCAT22(iVar11->field_0x6,0xbbc),puVar10,param_3);
  }
  puVar9 = puVar10;
  enable_win_1040_9234(CONCAT22(puVar10,paVar5),BVar16,uVar14);
  puVar15 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_3,puVar9,unaff_DI);
  uVar11 = (puVar15 >> 0x10);
  uVar3 = puVar15;
  uVar12 = uVar11;
  uVar6 = pass1_1010_a5ac(uVar3,uVar11,iVar11->field_0xb0);
  uVar7 = pass1_1010_ac62(uVar3,uVar11,0x1e,uVar6,uVar12);
  if (uVar7 != 0x0) {
    pass1_1010_a5ca(uVar3,uVar11,uVar6,uVar7,uVar12);
    if (0x0 < uVar7) {
      pass1_1010_a58a(uVar3,uVar11,uVar6,uVar7,uVar12);
      if (uVar7 == 0x0) {
        enable_win_1040_9234(CONCAT22(puVar10,paVar5),0x1,0x1010);
      }
    }
  }
  puVar1 = iVar11->field_0x98;
  iVar8 = puVar1;
  uVar4 = puVar1 & 0xffff0000;
  uVar14 = (uVar4 >> 0x10);
  SetWindowPos16(0x1010,0x40,*(INT16 *)(iVar8 + 0x10),*(INT16 *)(iVar8 + 0xe),
                 *(INT16 *)(iVar8 + 0xc),*(INT16 *)(uVar4 | iVar8 + 0xa),0x0);
  return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
win_ui_op_1040_52c0(param_1: i16,param_2: u16,param_3: u16,param_4: u32,HWND16 param_5
                   ,param_6: u16)

{
  code **ppcVar1;
  let BVar2: bool;
  let iVar3: i16;
  let uVar4: u16;
  let in_DX: *mut u8
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let unaff_DI: i16;
  let puVar8: *mut u16;
  let puVar9: *mut u16;
  let uVar10: u32;
  let uVar11: u16;
  let uVar12: u16;
  let uVar13: u16;
  let uStack30: u16;
  RECT16 local_a;
  let iStack6: i16;
  let iStack4: i16;
  
  if (param_4._2_2_ != 0x10c) {
    if (param_4._2_2_ < 0x10d) {
      if (param_4._2_2_ == 0xfa) {
        uVar10 = (param_1 + 0x98);
        ppcVar1 = (code **)((param_1 + 0x98) + 0x18);
        (**ppcVar1)(param_5,uVar10,(uVar10 >> 0x10),0x0,
                    _PTR_LOOP_1050_5efe,(_PTR_LOOP_1050_5efe >> 0x10));
        return;
      }
      if (param_4._2_2_ == 0x10a) {
        GetClientRect16(param_5,&local_a);
        uVar10 = (param_1 + 0x98);
        local_a.y += 0x3;
        local_a.x = (uVar10 + 0x1a) + -0x9;
        iStack6 += -0x3;
        iStack4 += -0x3;
        InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,
                         (RECT16 *)(&PTR_LOOP_1050_0000 + 0x1),(bool)&local_a);
        unk_destroy_win_op_1010_2fa0(*(ULONG *)(param_1 + 0x98),0x1010);
        pass1_1010_32c0((param_1 + 0x98),0x0);
        pass1_1010_2ee2(*(u32 **)(param_1 + 0x98),param_6,0x1010);
        return;
      }
      if (param_4._2_2_ != 0x10b) goto LAB_1040_5560;
      uVar10 = (param_1 + 0x98);
      uVar11 = (uVar10 + 0x12);
      uVar6 = uVar11;
      puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,in_DX,unaff_DI);
      uVar5 = (puVar8 >> 0x10);
      puVar9 = puVar8;
      pass1_1010_a5ca(puVar8,uVar5,uVar6,puVar8,uVar5);
      uVar6 = (puVar9 >> 0x10);
      if ((uVar11 != 0x70) && (puVar9 == 0x0)) {
        return;
      }
      uVar10 = (param_1 + 0xb0);
      uVar12 = uVar10;
      uVar13 = (uVar10 >> 0x10);
      uVar10 = (param_1 + 0x98);
      uVar11 = (uVar10 + 0x12);
    }
    else {
      if (param_4._2_2_ != 0x10d) {
        if (param_4._2_2_ == 0x10e) {
          puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_6,in_DX,unaff_DI);
          iVar3 = puVar8;
          ui_op_1010_79aa(puVar8,0xfc6,*(long *)(param_1 + 0xb0),param_6);
          if (iVar3 != 0x0) {
            return;
          }
          unk_win_op_1010_7300(puVar8,0x0,0x13,(param_1 + 0xb0));
          return;
        }
        if (param_4._2_2_ == 0xbbb) {
          if ((param_1 + 0xb6) != 0x0) {
            BVar2 = IsWindow16(param_5);
            param_5 = s_tile2_bmp_1050_1538;
            if (BVar2 != 0x0) goto LAB_1040_5417;
          }
          uVar10 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,(param_1 + 0x6),0x1b,
                                   in_DX,param_1,&PTR_LOOP_1050_1038,
                                   param_6);
          (param_1 + 0xb6) = (uVar10 + 0x6);
          set_win_pos_1038_abdc(&PTR_LOOP_1050_1038);
          ShowWindow16((HWND16)&PTR_LOOP_1050_1038,0x1);
          return;
        }
        if (param_4._2_2_ == 0xbbc) {
          puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,in_DX,unaff_DI);
          uVar7 = (puVar8 >> 0x10);
          uVar6 = puVar8;
          uVar5 = uVar7;
          uVar4 = pass1_1010_a5ac(uVar6,uVar7,(param_1 + 0xb0));
          uVar11 = uVar4;
          pass1_1010_a58a(uVar6,uVar7,uVar4,uVar4,uVar5);
          if (uVar11 == 0x0) {
            pass1_1010_a568(uVar6,uVar7,uVar4,0x0,uVar5);
          }
          GetDlgItem16(0x1010,0xbbc);
          EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
          return;
        }
LAB_1040_5560:
        pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,param_5,param_6);
        return;
      }
      puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,in_DX,unaff_DI);
      uVar6 = (puVar8 >> 0x10);
      uVar10 = (param_1 + 0xb0);
      uVar12 = uVar10;
      uVar13 = (uVar10 >> 0x10);
      uVar11 = 0x71;
      uVar5 = uVar6;
    }
    uStack30 = puVar8;
    param_5 = 0x1010;
    pass1_1010_a5ec(uStack30,uVar5,uVar11,CONCAT22(uVar13,uVar12),uVar6);
    if ((param_1 + 0xb4) != 0x0) {
      param_5 = s_tile2_bmp_1050_1538;
      BVar2 = IsWindow16(0x1010);
      if (BVar2 != 0x0) {
        param_5 = s_tile2_bmp_1050_1538;
        SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x11100eb);
      }
    }
  }
LAB_1040_5417:
  DestroyWindow16(param_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn enable_win_1040_5780(param_1: *mut u32)
{
  code **ppcVar1;
  let uVar2: u32;
  let uVar3: u16;
  let extraout_DX: *mut u8
  let unaff_DI: i16;
  let unaff_SS: u16;
  let puVar4: *mut u16;
  
  ppcVar1 = (code **)(*param_1 + 0x74);
  (**ppcVar1)();
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,unaff_SS,extraout_DX,unaff_DI);
  uVar2 = (param_1 + 0x90);
  uVar3 = pass1_1010_acc0(puVar4,(puVar4 >> 0x10),
                          (uVar2 + 0x6));
  if (uVar3 != 0x0) {
    GetDlgItem16(0x1010,0x1790);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
win_ui_op_1040_5800(param_1: i16,param_2: u16,param_3: u16,param_4: u32,param_5: u16
                   )

{
  code **ppcVar1;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  astruct_18 *paVar5;
  let in_DX: *mut u8
  let puVar6: *mut u8
  let puVar7: *mut u8
  let extraout_DX: *mut u8
  let iVar8: i16;
  let unaff_DI: *mut u8
  let uVar9: u16;
  HWND16 hwnd;
  let unaff_SS: u16;
  let piStack24: *mut i16;
  RECT16 local_14 [0x2];
  let iStack12: i16;
  astruct_18 *paStack10;
  astruct_20 *paStack6;
  
  if (param_4._2_2_ == 0xeb) {
    paStack6 = (astruct_20 *)
               mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,unaff_SS,in_DX,unaff_DI);
    puVar6 = (paStack6 >> 0x10);
    paVar5 = *(astruct_18 **)(param_1 + 0x90);
    if (paVar5 != (astruct_18 *)0x0) {
      paStack10 = paVar5;
      mem_op_1000_179c(0x18,puVar6,0x1000);
      uVar3 = paVar5;
      puVar7 = (puVar6 | uVar3);
      if (puVar7 == 0x0) {
        uVar3 = 0x0;
        puVar7 = 0x0;
      }
      else {
        struct_1040_a598((u16 *)(paVar5 & 0xffff | ZEXT24(puVar6) << 0x10));
      }
      (param_1 + 0x90) = uVar3;
      *(uchar **)(param_1 + 0x92) = puVar7;
      (param_1 + 0x90) = 0x6;
      iStack12 = **(i16 **)(param_1 + 0x90);
      uVar3 = iStack12 * 0xa + 0x2;
      mem_op_1000_179c(uVar3,puVar7,0x1000);
      piStack24 = (i16 *)CONCAT22(puVar7,uVar3);
      if ((puVar7 | uVar3) == 0x0) {
        uVar2 = (param_1 + 0x90);
        (uVar2 + 0x2) = 0x0;
      }
      else {
        *piStack24 = iStack12;
        pass1_1000_5586((uchar *)0xa564,&PTR_LOOP_1050_1040,iStack12,0xa,
                        uVar3 + 0x2,puVar7);
        uVar2 = (param_1 + 0x90);
        uVar9 = (uVar2 >> 0x10);
        iVar8 = uVar2;
        (iVar8 + 0x2) = uVar3 + 0x2;
        *(uchar **)(iVar8 + 0x4) = puVar7;
        unaff_DI = puVar7;
      }
      uVar2 = (param_1 + 0x90);
      (uVar2 + 0x6) = (paStack10 + 0x6);
      uVar2 = (param_1 + 0x90);
      (uVar2 + 0xa) = 0x4;
      uVar2 = (param_1 + 0x90);
      (uVar2 + 0x12) = (param_1 + 0xa);
      hwnd = 0x1010;
      pass1_1010_a50c(paStack6,0x10505d78,(param_1 + 0x90));
      if (paStack10 != (astruct_18 *)0x0) {
        pass1_1040_a5d0(paStack10);
        hwnd = 0x1000;
        fn_ptr_1000_17ce(paStack10,0x1000);
      }
      ppcVar1 = (code **)(CONCAT22(param_2,param_1) + 0x70);
      (**ppcVar1)();
      puVar6 = extraout_DX;
      uVar4 = pass1_1040_5cd6(CONCAT22(param_2,param_1));
      if (uVar4 != 0x0) {
        pass1_1040_5eaa(CONCAT22(param_2,param_1));
        (param_1 + 0x94) = 0x0;
      }
      pass1_1040_5dc4(CONCAT22(param_2,param_1),puVar6,unaff_DI,unaff_SS);
      GetWindowRect16(hwnd,local_14);
      InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,*(RECT16 **)(param_1 + 0x9c),0x0);
      if ((param_1 + 0x9c) != 0x0) {
        (param_1 + 0x9c) = 0x0;
      }
    }
  }
  else {
    if (param_4._2_2_ != 0x13b) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,param_5,unaff_SS);
      return;
    }
    GetDlgItem16(param_5,0x1790);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  }
  return;
}


fn msg_box_ui_op_1040_64ca(param_1: u32,char *param_2,uchar *param_3,param_4: u16)
{
  char local_206 [0x102];
  char local_104 [0x102];
  
  mem_op_1000_179c(0x1000,param_3,0x1000);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x100,local_206,param_4);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,param_2,(short)param_3);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,(LPCSTR)0x0,local_206,param_4);
  fn_ptr_1000_17ce((astruct_18 *)CONCAT22(param_3,param_2),0x1000);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn show_win_1040_65ba(astruct_1 *param_1,param_2: u16)
{
  let uVar1: u32;
  let uVar2: u16;
  astruct_160 *rect;
  let in_DX: *mut u8
  let puVar3: *mut u8
  let iVar4: i16;
  let iVar5: i16;
  let unaff_DI: i16;
  let uVar6: u16;
  let uVar7: u16;
  HWND16 hwnd;
  let unaff_SS: u16;
  let local_22: u16;
  let uStack32: u16;
  let uStack30: u16;
  let uStack28: u16;
  let puStack26: *mut u16;
  let uStack10: i16;
  let uStack8: u16;
  let puStack6: *mut u16;
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,unaff_SS,in_DX,unaff_DI);
  PTR_LOOP_1050_5f2e = (puStack6 >> 0x10);
  uStack8 = pass1_1010_0898();
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
  }
  puStack26 = CONCAT22(PTR_LOOP_1050_5f2e,PTR_LOOP_1050_5f2c);
  hwnd = 0x1000;
  uVar2 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4,0x0,0x1,PTR_LOOP_1050_5f2c,
                              PTR_LOOP_1050_5f2e,0x1000);
  uVar6 = (param_1 >> 0x10);
  iVar4 = param_1;
  (iVar4 + 0x8e) = uVar2;
  *(uchar **)(iVar4 + 0x90) = PTR_LOOP_1050_5f2e;
  for (iStack10 = 0x1; iStack10 <= uStack8; iStack10 += 0x1) {
    puStack26 =
                pass1_1010_0946(puStack6,(puStack6 >> 0x10),iStack10,
                                PTR_LOOP_1050_5f2e,unaff_DI,unaff_SS);
    puVar3 = (puStack26 >> 0x10);
    local_22 = *puStack26;
    uStack32 = (puStack26 + 0x2);
    uStack30 = 0x1;
    uStack28 = 0x1;
    rect = (astruct_160 *)&local_22;
    MapDialogRect16(0x1010,(RECT16 *)rect);
    hwnd = 0x1000;
    mem_op_1000_179c(0x42,puVar3,0x1000);
    PTR_LOOP_1050_5f2e = (puVar3 | rect);
    if (PTR_LOOP_1050_5f2e == 0x0) {
      uVar1 = (iVar4 + 0x8e);
      (uVar1 + iStack10 * 0x4) = 0x0;
    }
    else {
      hwnd = 0x1008;
      pass1_1008_3bd6(rect,puVar3,0x0,CONCAT22(local_22,uStack32),0x101,0xff0100,
                      CONCAT22((iVar4 + 0x6),
                               (puStack26 + 0x4)),
                      PTR_LOOP_1050_5f2e,unaff_SS);
      uVar1 = (iVar4 + 0x8e);
      uVar7 = (uVar1 >> 0x10);
      iVar5 = uVar1;
      *(astruct_160 **)(iVar5 + iStack10 * 0x4) = rect;
      *(uchar **)(iVar5 + iStack10 * 0x4 + 0x2) = PTR_LOOP_1050_5f2e;
    }
    uVar1 = (iVar4 + 0x8e);
    uVar7 = (uVar1 >> 0x10);
    iVar5 = uVar1;
    if (*(long *)(iVar5 + iStack10 * 0x4) != 0x0) {
      unaff_DI = puStack26;
      enable_win_1040_9234
                ((iVar5 + iStack10 * 0x4),*(bool *)(unaff_DI + 0x6),hwnd);
    }
  }
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(hwnd,0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
post_win_msg_1040_672e
          (param_1: i16,param_2: u16,param_3: u16,param_4: u32,param_5: u16,
          param_6: u16)

{
  let unaff_CS: u16;
  let iVar1: i16;
  
  if (param_4._2_2_ == (s_vrpal_bmp_1050_183a + 0x7)) {
    msg_box_ui_op_1040_64ca(CONCAT22(param_2,param_1),0x0,param_5,param_6);
  }
  else {
    if (param_4._2_2_ == 0x1851) {
      iVar1 = 0x2a;
    }
    else {
      if (param_4._2_2_ != 0x1852) {
        post_win_msg_1040_7b3c
                  (CONCAT22(param_2,param_1),param_3,param_4,
                   param_4._2_2_,unaff_CS);
        return;
      }
      iVar1 = 0x29;
    }
    pass1_1038_af40(_PTR_LOOP_1050_5b7c,(param_1 + 0x8),iVar1,param_5,param_1,
                    &PTR_LOOP_1050_1038,param_6);
    PostMessage16((HWND16)&PTR_LOOP_1050_1038,0x0,0x0,0x1110002);
  }
  return;
}


fn enable_win_1040_6880(param_1: u32,param_2: i16,HWND16 param_3)
{
  let uVar1: u32;
  let uVar2: u16;
  
  if (param_2 == 0x8) {
    uVar2 = (param_1 >> 0x10);
    GetDlgItem16(param_3,0x107);
    uVar1 = (param_1 + 0x94);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,*(bool *)(uVar1 + 0x24));
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x108);
    uVar1 = (param_1 + 0x94);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,*(bool *)(uVar1 + 0x26));
  }
  return;
}


fn mixed_win_ui_op_1040_6942(astruct_1 *param_1,param_2: u16,param_3: u16)
{
  let uVar1: u32;
  let uVar2: u32;
  code **ppcVar3;
  astruct_160 *paVar4;
  LPCSTR pCVar5;
  let puVar6: u32;
  let iVar7: i16;
  let in_DX: *mut u8
  let extraout_DX: *mut u8
  let puVar8: *mut u8
  let puVar9: *mut u8
  let uVar10: u16;
  let iVar11: i16;
  let unaff_DI: i16;
  let uVar12: u16;
  let uVar13: u16;
  let uVar14: u16;
  HWND16 hwnd;
  let puVar15: *mut u16;
  let DVar16: u32;
  char *pcVar17;
  let BVar18: bool;
  let local_64: u32;
  let uStack96: u32;
  let IStack92: i16;
  let IStack90: i16;
  char local_58 [0x50];
  HDC16 HStack8;
  astruct_160 *paStack6;
  let puStack4: *mut u8
  
  dialog_ui_fn_1040_78e2(param_1,param_3);
  puVar15 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x33,param_2,in_DX,unaff_DI);
  uVar14 = (puVar15 >> 0x10);
  paVar4 = (astruct_160 *)puVar15;
  uVar12 = (param_1 >> 0x10);
  iVar11 = param_1;
  *(astruct_160 **)(iVar11 + 0x94) = paVar4;
  (iVar11 + 0x96) = uVar14;
  ppcVar3 = (code **)((iVar11 + 0x94) + 0x4);
  (**ppcVar3)(0x1010,(iVar11 + 0x94),uVar14,0x0,param_1);
  puVar9 = extraout_DX;
  mem_op_1000_179c(0xa,extraout_DX,0x1000);
  puVar8 = (puVar9 | paVar4);
  if (puVar8 == 0x0) {
    (iVar11 + 0x98) = 0x0;
  }
  else {
    puVar15 = struct_1040_bf3e((u16 *)CONCAT22(puVar9,paVar4),(iVar11 + 0x6)
                              );
    puVar8 = (puVar15 >> 0x10);
    paVar4 = (astruct_160 *)puVar15;
    *(astruct_160 **)(iVar11 + 0x98) = paVar4;
    *(uchar **)(iVar11 + 0x9a) = puVar8;
  }
  pass1_1040_bfde((iVar11 + 0x98),*(u32 **)(iVar11 + 0x94),param_2);
  mem_op_1000_179c(0x42,puVar8,0x1000);
  puVar9 = (puVar8 | paVar4);
  if (puVar9 != 0x0) {
    pass1_1008_3bd6(paVar4,puVar8,0x1,0xa000a,0x0,0x800081,
                    CONCAT22((iVar11 + 0x6),0x10a),puVar9,param_2);
  }
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar8 = (puVar9 | paVar4);
  if (puVar8 != 0x0) {
    pass1_1008_3bd6(paVar4,puVar9,0x1,0xa0028,0x0,0x820083,
                    CONCAT22((iVar11 + 0x6),0x10c),puVar8,param_2);
  }
  BVar18 = 0x42;
  uVar14 = 0x1000;
  mem_op_1000_179c(0x42,puVar8,0x1000);
  puVar9 = (puVar8 | paVar4);
  if (puVar9 == 0x0) {
    paVar4 = (astruct_160 *)0x0;
    puVar9 = 0x0;
  }
  else {
    uVar14 = 0x1008;
    pass1_1008_3bd6(paVar4,puVar8,0x1,0xa00aa,0x101,0xff0100,
                    CONCAT22((iVar11 + 0x6),0x107),puVar9,param_2);
  }
  paStack6 = paVar4;
  puStack4 = puVar9;
  enable_win_1040_9234(CONCAT22(puVar9,paVar4),BVar18,uVar14);
  BVar18 = 0x42;
  hwnd = 0x1000;
  mem_op_1000_179c(0x42,puVar9,0x1000);
  uVar10 = puVar9 | paVar4;
  if (uVar10 == 0x0) {
    paVar4 = (astruct_160 *)0x0;
    uVar10 = 0x0;
  }
  else {
    hwnd = 0x1008;
    pass1_1008_3bd6(paVar4,puVar9,0x1,0xa00c2,0x101,0xff0100,
                    CONCAT22((iVar11 + 0x6),0x108),uVar10,param_2);
  }
  paStack6 = paVar4;
  puStack4 = uVar10;
  enable_win_1040_9234(CONCAT22(uVar10,paVar4),BVar18,hwnd);
  HStack8 = GetDC16(hwnd);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x50,local_58,param_2);
  pcVar17 = local_58;
  pCVar5 = (LPCSTR)str_op_1000_3da4(CONCAT22(param_2,pcVar17));
  DVar16 = GetTextExtent16(0x1000,pCVar5,pcVar17);
  IStack90 = (DVar16 >> 0x10);
  IStack92 = DVar16;
  CreateWindow16((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)0x0,
                 ZEXT24(PTR_LOOP_1050_038c) << 0x10,0x7cd,*(INT16 *)(iVar11 + 0x6),
                 IStack90,IStack92,0xad,0x22,0x0,(LPVOID)(s_Rebel_1050_4ffc + 0x4));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x50,local_58,param_2);
  pcVar17 = local_58;
  pCVar5 = (LPCSTR)str_op_1000_3da4(CONCAT22(param_2,pcVar17));
  DVar16 = GetTextExtent16(0x1000,pCVar5,pcVar17);
  IStack90 = (DVar16 >> 0x10);
  IStack92 = DVar16;
  ReleaseDC16((HWND16)s_tile2_bmp_1050_1538,HStack8);
  CreateWindow16((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)0x0,
                 ZEXT24(PTR_LOOP_1050_038c) << 0x10,0x7ce,*(INT16 *)(iVar11 + 0x6),
                 IStack90,IStack92,0xc5,0x22,0x0,(LPVOID)(s_Rebel_1050_4ffc + 0x4));
  local_64 = 0x5a000a;
  uStack96 = 0x140050;
  puVar6 = &local_64;
  create_window_1040_6eae();
  local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
  create_window_1040_6eae();
  local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
  create_window_1040_6eae();
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4010001);
  uVar1 = (iVar11 + 0x94);
  iVar7 = uVar1;
  uVar1 &= 0xffff0000;
  uVar14 = (iVar11 + 0x6);
  uVar13 = (uVar1 >> 0x10);
  SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x40,*(INT16 *)(iVar7 + 0x10),
                 *(INT16 *)(iVar7 + 0xe),*(INT16 *)(iVar7 + 0xc),
                 *(INT16 *)(uVar1 | iVar7 + 0xa),0x0);
  DAT_1050_0ecc = 0x0;
  uVar2 = (iVar11 + 0x94);
  ppcVar3 = (code **)((iVar11 + 0x94) + 0x10);
  (**ppcVar3)(s_tile2_bmp_1050_1538,uVar2,(uVar2 >> 0x10),uVar14,
              puVar6);
  pass1_1010_2ee2(*(u32 **)(iVar11 + 0x94),param_2,0x1010);
  PostMessage16(0x1010,0x0,0x0,0x111010a);
  return;
}


fn win_ui_op_1040_6d1a(param_1: i16,param_2: u16,param_3: u16,param_4: u32)
{
  code **ppcVar1;
  let uVar2: u32;
  let in_DX: *mut u8
  let unaff_CS: u16;
  let unaff_SS: u16;
  let iVar3: i16;
  RECT16 local_a;
  let iStack6: i16;
  let iStack4: i16;
  
  if (false) {
switchD_1040_6e7b_caseD_fb:
    pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,unaff_CS,unaff_SS);
    return;
  }
  unaff_CS = &PTR_LOOP_1050_1040;
  switch(param_4._2_2_) {
  case 0xfa:
    ppcVar1 = (code **)((param_1 + 0x94) + 0x18);
    (**ppcVar1)();
    break;
  default:
    goto switchD_1040_6e7b_caseD_fb;
  case 0xfd:
    if (DAT_1050_0ecc == 0x0) {
      return;
    }
    DAT_1050_0ecc = 0x0;
    goto LAB_1040_6deb;
  case 0xfe:
    if (DAT_1050_0ecc == 0x1) {
      return;
    }
    DAT_1050_0ecc = 0x1;
    goto LAB_1040_6deb;
  case 0xff:
    if (DAT_1050_0ecc == 0x2) {
      return;
    }
    DAT_1050_0ecc = 0x2;
LAB_1040_6deb:
    uVar2 = (param_1 + 0x94);
    ppcVar1 = (code **)((param_1 + 0x94) + 0x10);
    (**ppcVar1)(&PTR_LOOP_1050_1040,uVar2,(uVar2 >> 0x10));
    pass1_1010_2ee2(*(u32 **)(param_1 + 0x94),unaff_SS,0x1010);
    PostMessage16(0x1010,0x0,0x0,0x111010a);
    break;
  case 0x107:
    iVar3 = 0x0;
    goto LAB_1040_6e48;
  case 0x108:
    iVar3 = 0x1;
LAB_1040_6e48:
    win_ui_op_1010_3202((param_1 + 0x94),iVar3,0x1010);
    break;
  case 0x10a:
    GetClientRect16((HWND16)&PTR_LOOP_1050_1040,&local_a);
    uVar2 = (param_1 + 0x94);
    local_a.y += 0x3;
    local_a.x = (uVar2 + 0x1a) + -0x9;
    iStack6 += -0x3;
    iStack4 += -0x3;
    InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,
                     (RECT16 *)(&PTR_LOOP_1050_0000 + 0x1),(bool)&local_a);
    unk_destroy_win_op_1010_2fa0(*(ULONG *)(param_1 + 0x94),0x1010);
    pass1_1010_32c0((param_1 + 0x94),0x0);
    pass1_1010_2ee2(*(u32 **)(param_1 + 0x94),unaff_SS,0x1010);
    break;
  case 0x10c:
    DestroyWindow16((HWND16)&PTR_LOOP_1050_1040);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
create_window_1040_6eae
          (param_1: u32,param_2: i16,HMENU16 *in_menu_handle,param_4: u16,
          INT16 param_5)

{
  let iVar1: i16;
  let uVar2: u16;
  HINSTANCE16 h_instance;
  
  load_string_1010_847e
            (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  _h_instance = 0x50000009;
  if (param_2 != 0x0) {
    _h_instance = 0x50020009;
  }
  uVar2 = (in_menu_handle >> 0x10);
  iVar1 = in_menu_handle;
  CreateWindow16((LPCSTR)0x1010,(LPCSTR)0x0,ZEXT24(PTR_LOOP_1050_038c) << 0x10,param_5,
                 *(INT16 *)(param_1 + 0x6),*(INT16 *)(iVar1 + 0x6),
                 *(INT16 *)(iVar1 + 0x4),*(HWND16 *)(iVar1 + 0x2),*in_menu_handle,
                 (HINSTANCE16)_h_instance,(LPVOID)(_h_instance >> 0x10));
  return;
}


fn enable_win_1040_6ff2(param_1: u32,param_2: i16,HWND16 param_3)
{
  let uVar1: u32;
  let uVar2: u16;
  
  if (param_2 == 0x8) {
    uVar2 = (param_1 >> 0x10);
    GetDlgItem16(param_3,0x107);
    uVar1 = (param_1 + 0x94);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,*(bool *)(uVar1 + 0x24));
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x108);
    uVar1 = (param_1 + 0x94);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,*(bool *)(uVar1 + 0x26));
  }
  return;
}




// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn mixed_win_ui_op_1040_70b4(astruct_1 *param_1,param_2: u16,param_3: u16)
{
  let uVar1: u32;
  let uVar2: u32;
  code **ppcVar3;
  astruct_160 *paVar4;
  LPCSTR pCVar5;
  let puVar6: u32;
  let iVar7: i16;
  let in_DX: *mut u8
  let extraout_DX: *mut u8
  let puVar8: *mut u8
  let puVar9: *mut u8
  let uVar10: u16;
  let iVar11: i16;
  let unaff_DI: i16;
  let uVar12: u16;
  let uVar13: u16;
  let uVar14: u16;
  HWND16 hwnd;
  let puVar15: *mut u16;
  let DVar16: u32;
  char *pcVar17;
  let BVar18: bool;
  let local_64: u32;
  let uStack96: u32;
  let IStack92: i16;
  let IStack90: i16;
  char local_58 [0x50];
  HDC16 HStack8;
  astruct_160 *paStack6;
  let puStack4: *mut u8
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  puVar15 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x34,param_3,in_DX,unaff_DI);
  uVar14 = (puVar15 >> 0x10);
  paVar4 = (astruct_160 *)puVar15;
  uVar12 = (param_1 >> 0x10);
  iVar11 = param_1;
  *(astruct_160 **)(iVar11 + 0x94) = paVar4;
  (iVar11 + 0x96) = uVar14;
  ppcVar3 = (code **)((iVar11 + 0x94) + 0x4);
  (**ppcVar3)(0x1010,(iVar11 + 0x94),uVar14,0x0,param_1);
  puVar9 = extraout_DX;
  mem_op_1000_179c(0xa,extraout_DX,0x1000);
  puVar8 = (puVar9 | paVar4);
  if (puVar8 == 0x0) {
    (iVar11 + 0x98) = 0x0;
  }
  else {
    puVar15 = struct_1040_bf3e((u16 *)CONCAT22(puVar9,paVar4),(iVar11 + 0x6)
                              );
    puVar8 = (puVar15 >> 0x10);
    paVar4 = (astruct_160 *)puVar15;
    *(astruct_160 **)(iVar11 + 0x98) = paVar4;
    *(uchar **)(iVar11 + 0x9a) = puVar8;
  }
  pass1_1040_bfde((iVar11 + 0x98),*(u32 **)(iVar11 + 0x94),param_3);
  mem_op_1000_179c(0x42,puVar8,0x1000);
  puVar9 = (puVar8 | paVar4);
  if (puVar9 != 0x0) {
    pass1_1008_3bd6(paVar4,puVar8,0x1,0xa000a,0x0,0x800081,
                    CONCAT22((iVar11 + 0x6),0x10a),puVar9,param_3);
  }
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar8 = (puVar9 | paVar4);
  if (puVar8 != 0x0) {
    pass1_1008_3bd6(paVar4,puVar9,0x1,0xa0028,0x0,0x820083,
                    CONCAT22((iVar11 + 0x6),0x10c),puVar8,param_3);
  }
  BVar18 = 0x42;
  uVar14 = 0x1000;
  mem_op_1000_179c(0x42,puVar8,0x1000);
  puVar9 = (puVar8 | paVar4);
  if (puVar9 == 0x0) {
    paVar4 = (astruct_160 *)0x0;
    puVar9 = 0x0;
  }
  else {
    uVar14 = 0x1008;
    pass1_1008_3bd6(paVar4,puVar8,0x1,0xa00aa,0x101,0xff0100,
                    CONCAT22((iVar11 + 0x6),0x107),puVar9,param_3);
  }
  paStack6 = paVar4;
  puStack4 = puVar9;
  enable_win_1040_9234(CONCAT22(puVar9,paVar4),BVar18,uVar14);
  BVar18 = 0x42;
  hwnd = 0x1000;
  mem_op_1000_179c(0x42,puVar9,0x1000);
  uVar10 = puVar9 | paVar4;
  if (uVar10 == 0x0) {
    paVar4 = (astruct_160 *)0x0;
    uVar10 = 0x0;
  }
  else {
    hwnd = 0x1008;
    pass1_1008_3bd6(paVar4,puVar9,0x1,0xa00c2,0x101,0xff0100,
                    CONCAT22((iVar11 + 0x6),0x108),uVar10,param_3);
  }
  paStack6 = paVar4;
  puStack4 = uVar10;
  enable_win_1040_9234(CONCAT22(uVar10,paVar4),BVar18,hwnd);
  HStack8 = GetDC16(hwnd);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x50,local_58,param_3);
  pcVar17 = local_58;
  pCVar5 = (LPCSTR)str_op_1000_3da4(CONCAT22(param_3,pcVar17));
  DVar16 = GetTextExtent16(0x1000,pCVar5,pcVar17);
  IStack90 = (DVar16 >> 0x10);
  IStack92 = DVar16;
  CreateWindow16((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)0x0,
                 ZEXT24(PTR_LOOP_1050_038c) << 0x10,0x7cd,*(INT16 *)(iVar11 + 0x6),
                 IStack90,IStack92,0xad,0x22,0x0,(LPVOID)(s_Rebel_1050_4ffc + 0x4));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x50,local_58,param_3);
  pcVar17 = local_58;
  pCVar5 = (LPCSTR)str_op_1000_3da4(CONCAT22(param_3,pcVar17));
  DVar16 = GetTextExtent16(0x1000,pCVar5,pcVar17);
  IStack90 = (DVar16 >> 0x10);
  IStack92 = DVar16;
  ReleaseDC16((HWND16)s_tile2_bmp_1050_1538,HStack8);
  CreateWindow16((LPCSTR)s_tile2_bmp_1050_1538,(LPCSTR)0x0,
                 ZEXT24(PTR_LOOP_1050_038c) << 0x10,0x7ce,*(INT16 *)(iVar11 + 0x6),
                 IStack90,IStack92,0xc5,0x22,0x0,(LPVOID)(s_Rebel_1050_4ffc + 0x4));
  local_64 = 0x5a000a;
  uStack96 = 0x140050;
  puVar6 = &local_64;
  create_window_1040_7620();
  local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
  create_window_1040_7620();
  local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
  create_window_1040_7620();
  SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4010001);
  uVar1 = (iVar11 + 0x94);
  iVar7 = uVar1;
  uVar1 &= 0xffff0000;
  uVar14 = (iVar11 + 0x6);
  uVar13 = (uVar1 >> 0x10);
  SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x40,*(INT16 *)(iVar7 + 0x10),
                 *(INT16 *)(iVar7 + 0xe),*(INT16 *)(iVar7 + 0xc),
                 *(INT16 *)(uVar1 | iVar7 + 0xa),0x0);
  DAT_1050_0ecc = 0x0;
  uVar2 = (iVar11 + 0x94);
  ppcVar3 = (code **)((iVar11 + 0x94) + 0x10);
  (**ppcVar3)(s_tile2_bmp_1050_1538,uVar2,(uVar2 >> 0x10),uVar14,
              puVar6);
  pass1_1010_2ee2(*(u32 **)(iVar11 + 0x94),param_3,0x1010);
  PostMessage16(0x1010,0x0,0x0,0x111010a);
  return;
}


fn win_cleanup_op_1040_748c(param_1: i16,param_2: u16,param_3: u16,param_4: u32)
{
  code **ppcVar1;
  let uVar2: u32;
  let in_DX: *mut u8
  let unaff_CS: u16;
  let unaff_SS: u16;
  let iVar3: i16;
  RECT16 local_a;
  let iStack6: i16;
  let iStack4: i16;
  
  if (false) {
switchD_1040_75ed_caseD_fb:
    pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,unaff_CS,unaff_SS);
    return;
  }
  unaff_CS = &PTR_LOOP_1050_1040;
  switch(param_4._2_2_) {
  case 0xfa:
    ppcVar1 = (code **)((param_1 + 0x94) + 0x18);
    (**ppcVar1)();
    break;
  default:
    goto switchD_1040_75ed_caseD_fb;
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
    uVar2 = (param_1 + 0x94);
    ppcVar1 = (code **)((param_1 + 0x94) + 0x10);
    (**ppcVar1)(&PTR_LOOP_1050_1040,uVar2,(uVar2 >> 0x10));
    pass1_1010_2ee2(*(u32 **)(param_1 + 0x94),unaff_SS,0x1010);
    PostMessage16(0x1010,0x0,0x0,0x111010a);
    break;
  case 0x107:
    iVar3 = 0x0;
    goto LAB_1040_75ba;
  case 0x108:
    iVar3 = 0x1;
LAB_1040_75ba:
    win_ui_op_1010_3202((param_1 + 0x94),iVar3,0x1010);
    break;
  case 0x10a:
    GetClientRect16((HWND16)&PTR_LOOP_1050_1040,&local_a);
    uVar2 = (param_1 + 0x94);
    local_a.y += 0x3;
    local_a.x = (uVar2 + 0x1a) + -0x9;
    iStack6 += -0x3;
    iStack4 += -0x3;
    InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,
                     (RECT16 *)(&PTR_LOOP_1050_0000 + 0x1),(bool)&local_a);
    unk_destroy_win_op_1010_2fa0(*(ULONG *)(param_1 + 0x94),0x1010);
    pass1_1010_32c0((param_1 + 0x94),0x0);
    pass1_1010_2ee2(*(u32 **)(param_1 + 0x94),unaff_SS,0x1010);
    break;
  case 0x10c:
    DestroyWindow16((HWND16)&PTR_LOOP_1050_1040);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
create_window_1040_7620
          (param_1: u32,param_2: i16,HMENU16 *in_menu_handle,param_4: u16,
          INT16 param_5)

{
  let iVar1: i16;
  let uVar2: u16;
  HINSTANCE16 h_instance;
  
  load_string_1010_847e
            (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  _h_instance = 0x50000009;
  if (param_2 != 0x0) {
    _h_instance = 0x50020009;
  }
  uVar2 = (in_menu_handle >> 0x10);
  iVar1 = in_menu_handle;
  CreateWindow16((LPCSTR)0x1010,(LPCSTR)0x0,ZEXT24(PTR_LOOP_1050_038c) << 0x10,param_5,
                 *(INT16 *)(param_1 + 0x6),*(INT16 *)(iVar1 + 0x6),
                 *(INT16 *)(iVar1 + 0x4),*(HWND16 *)(iVar1 + 0x2),*in_menu_handle,
                 (HINSTANCE16)_h_instance,(LPVOID)(_h_instance >> 0x10));
  return;
}


fn ui_cleanup_op_1040_782c(astruct_18 *param_1,HGDIOBJ16 param_2)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  HGDIOBJ16 menu;
  HMENU16 hwnd;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  param_1->field_0x0 = 0x840c;
  (iVar4 + 0x2) = &PTR_LOOP_1050_1040;
  puVar1 = (iVar4 + 0x70);
  uVar2 = (iVar4 + 0x72);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(param_2,puVar1,uVar2,0x1);
  }
  menu = param_2;
  if ((iVar4 + 0x4) != 0x0) {
    menu = (HGDIOBJ16)s_tile2_bmp_1050_1538;
    DeleteObject16(param_2);
    (iVar4 + 0x4) = 0x0;
  }
  hwnd = menu;
  if ((iVar4 + 0x68) != 0x0) {
    hwnd = (HMENU16)s_tile2_bmp_1050_1538;
    DestroyMenu16(menu);
  }
  RemoveProp16(hwnd,(LPCSTR)s_thisLo_1050_5db1);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisHi_1050_5db8);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_procLo_1050_5dbf);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_procHi_1050_5dc6);
  param_1->field_0x0 = 0x389a;
  (iVar4 + 0x2) = 0x1008;
  return;
}


fn dialog_ui_fn_1040_78e2(astruct_1 *in_struct_1,HINSTANCE16 in_instance_handle)
{
  code **ppcVar1;
  LPCSTR dlg_template;
  HWND16 dialog_handle;
  astruct_1 *local_struct_1;
  astruct_1 *local_string_1;
  let uVar2: u16;
  let lVar3: i32;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  LPCSTR local_string_2;
  LPCSTR pCStack8;
  
  local_string_1 = (astruct_1 *)(in_struct_1 >> 0x10);
  local_struct_1 = (astruct_1 *)in_struct_1;
  if (*(long *)&local_struct_1->field_0xc == 0x0) {
    uVar2 = (_PTR_LOOP_1050_5bc8 >> 0x10);
    dlg_template = *(LPCSTR *)(_PTR_LOOP_1050_5bc8 + 0x4);
    dialog_handle = *(HWND16 *)(_PTR_LOOP_1050_5bc8 + 0x6);
  }
  else {
    dlg_template = *(LPCSTR *)&local_struct_1->field_0xc;
    dialog_handle = *(HWND16 *)&local_struct_1->field_0xe;
  }
  dialog_handle =
       CreateDialog16(in_instance_handle,dlg_template,dialog_handle,
                      local_struct_1->lpvoid_field_0x8);
  *(HWND16 *)&local_struct_1->field_0x6 = dialog_handle;
  GetWindowText16((HWND16)s_tile2_bmp_1050_1538,0x50,
                  &local_struct_1->max_count_field_0x10);
  lVar3 = GetWindowLong16((HWND16)s_tile2_bmp_1050_1538,-0x4);
  SetWindowLong16((HWND16)s_tile2_bmp_1050_1538,_PTR_LOOP_1050_5bcc,
                  CONCAT22(0xfffc,(_PTR_LOOP_1050_5bcc >> 0x10)));
  uVar2 = &local_struct_1->field_0x6;
  uVar10 = SUB42(&USHORT_1050_1050,0x0);
  SetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)local_struct_1,
            (HANDLE16)s_thisLo_1050_5dcd);
  uVar9 = &local_struct_1->field_0x6;
  uVar8 = SUB42(&USHORT_1050_1050,0x0);
  SetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)local_string_1,
            (HANDLE16)s_thisHi_1050_5dd4);
  local_string_2 = (LPCSTR)lVar3;
  uVar7 = &local_struct_1->field_0x6;
  uVar6 = SUB42(&USHORT_1050_1050,0x0);
  SetProp16((HWND16)s_tile2_bmp_1050_1538,local_string_2,(HANDLE16)s_procLo_1050_5ddb);
  pCStack8 = (LPCSTR)(lVar3 >> 0x10);
  uVar5 = &local_struct_1->field_0x6;
  uVar4 = SUB42(&USHORT_1050_1050,0x0);
  SetProp16((HWND16)s_tile2_bmp_1050_1538,pCStack8,(HANDLE16)s_procHi_1050_5de2);
  ppcVar1 = (code **)(in_struct_1->field_0x0 + 0x50);
  (**ppcVar1)(s_tile2_bmp_1050_1538,in_struct_1,uVar4,uVar5,uVar6,uVar7,uVar8,uVar9,
              uVar10,uVar2);
  return;
}


bool 
post_win_msg_1040_7b3c
          (param_1: *mut u32,param_2: u16,param_3: u16,param_4: i16,HWND16 param_5)

{
  code **ppcVar1;
  
  if ((param_4 == 0x1) || (param_4 == 0x2)) {
    ppcVar1 = (code **)(*param_1 + 0x14);
    (**ppcVar1)();
  }
  else {
    if (param_4 == 0x6f) {
      ppcVar1 = (code **)(*param_1 + 0x2c);
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


fn post_win_msg_1040_7f1c(param_1: u32,param_2: i16,HWND16 param_3) -> bool

{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((iVar1 + 0x78) != 0x0) {
    return 0x0;
  }
  if ((iVar1 + 0x60) != param_2) {
    (iVar1 + 0x60) = param_2;
    PostMessage16(param_3,0x0,0x0,0x850000);
  }
  return 0x1;
}



fn post_win_msg_1040_7f56(param_1: u32,char *param_2)
{
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | (param_1 + 0x10)),param_2);
  PostMessage16(0x1000,0x0,0x0,0x850000);
  return;
}



fn menu_ui_op_1040_7f86(param_1: u32,HWND16 param_2,RECT16 *param_3)
{
  HMENU16 HVar1;
  let iVar2: i16;
  let uVar3: u16;
  HWND16 unaff_CS;
  POlet local_6: i16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((*(long *)(iVar2 + 0x6a) != 0x0) && ((iVar2 + 0x68) == 0x0)) {
    HVar1 = LoadMenu16(unaff_CS,(LPCSTR)(iVar2 + 0x6a));
    *(HMENU16 *)(iVar2 + 0x68) = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
    unaff_CS = s_tile2_bmp_1050_1538;
    HVar1 = GetSubMenu16((HMENU16)s_tile2_bmp_1050_1538,0x0);
    *(HMENU16 *)(iVar2 + 0x68) = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
  }
  local_6.x = param_3;
  local_6.y = param_2;
  ClientToScreen16(unaff_CS,&local_6);
  TrackPopupMenu16((HMENU16)s_tile2_bmp_1050_1538,0x0,0x0,*(INT16 *)(iVar2 + 0x6),0x0,
                   local_6.y,(RECT16 *)local_6.x);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn win_help_1040_800c(param_1: u32,param_2: u16)
{
  let uVar1: u16;
  astruct_43 *paVar2;
  LPCSTR lp_help_file;
  let w_command: u16;
  let uVar3: u16;
  
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1f8,param_2);
  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x8a) == 0x0) {
    w_command = 0x0;
    uVar3 = 0x3;
    lp_help_file = (LPCSTR)0x0;
  }
  else {
    uVar3 = 0x1;
    lp_help_file = *(LPCSTR *)(param_1 + 0x8a);
    w_command = lp_help_file >> 0xf;
  }
  WinHelp16(0x1010,lp_help_file,w_command,CONCAT22(paVar2,uVar3));
  return;
}


fn unk_win_ui_op_1040_8158(param_1: *mut u32,POINT16 param_2,param_3: i16,HWND16 param_4)
{
  code **ppcVar1;
  let IVar2: i16;
  let BVar3: bool;
  let uVar4: u32;
  let iVar5: i16;
  let uVar6: u16;
  let uVar7: u16;
  POlet local_6: i16;
  
  if (param_3 == 0x2) {
    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    if ((iVar5 + 0x76) != 0x0) {
      local_6 = param_2;
      uVar6 = (iVar5 + 0x6);
      ScreenToClient16(param_4,&local_6);
      uVar7 = 0x4;
      IVar2 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
      local_6 = (POINT16)(local_6 & 0xffff |
                         (local_6.y + IVar2) << 0x10);
      uVar4 = param_1 & 0xffff0000 | (iVar5 + 0x82);
      BVar3 = PtInRect16((RECT16 *)s_tile2_bmp_1050_1538,local_6);
      if (BVar3 != 0x0) {
        ppcVar1 = (code **)(*param_1 + 0x14);
        (**ppcVar1)(s_tile2_bmp_1050_1538,param_1,0x0,uVar4,uVar7,uVar6);
      }
    }
  }
  return;
}


fn check_dialog_msg_1040_81b6(param_1: u32,HWND16 param_2)
{
  let BVar1: bool;
  MSG16 local_14;
  
  (param_1 + 0x78) = 0x1;
  while( true ) {
    BVar1 = IsWindow16(param_2);
    if (BVar1 == 0x0) {
      return;
    }
    BVar1 = GetMessage16((MSG16 *)s_tile2_bmp_1050_1538,0x0,0x0,0x0);
    if (BVar1 == 0x0) break;
    param_2 = s_tile2_bmp_1050_1538;
    IsDialogMessage16((HWND16)s_tile2_bmp_1050_1538,&local_14);
  }
  return;
}



fn win_ui_op_1040_81fe(HWND16 param_1)
{
  SetSysModalWindow(param_1);
  return;
}


fn move_win_1040_826c(astruct_1 *param_1,INT16 param_2,bool param_3)
{
  let IVar1: i16;
  HWND16 unaff_CS;
  RECT16 local_e;
  let uStack10: i16;
  let iStack8: i16;
  let IStack6: i16;
  let BStack4: bool;
  
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
  MoveWindow16((HWND16)s_tile2_bmp_1050_1538,0x0,iStack8 - local_e.y,iStack10 - local_e.x,
               param_2,BStack4);
  return;
}


fn enable_win_1040_86dc(HWND16 param_1)
{
  HWND16 HVar1;
  
  HVar1 = GetDlgItem16(param_1,0x1);
  if (HVar1 != 0x0) {
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    HVar1 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x2);
    if (HVar1 != 0x0) {
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uchar *  win_ui_op_1040_8718(astruct_37 *param_1,param_2: u16)

{
  let piVar1: *mut i16;
  let iVar2: i16;
  let uVar3: u16;
  let extraout_DX: *mut u8
  let puVar4: *mut u8
  let unaff_DI: i16;
  let uVar5: u16;
  let puVar6: *mut u16;
  let uVar7: u16;
  let uVar9: u16;
  let UVar10: u32;
  i16 local_104 [0x80];
  let uStack4: u16;
  let uVar8: u16;
  
  uVar5 = 0x1008;
  unk_win_msg_op_1008_9510((i16 *)&PTR_LOOP_1050_5df4,0x1008,param_2);
  UVar10 = param_1;
  uVar8 = (param_1 >> 0x10);
  dialog_ui_fn_1040_78e2((astruct_1 *)param_1,0x1008);
  PTR_LOOP_1050_5df6 = (UVar10 + 0x6);
  if (*(long *)(UVar10 + 0x94) != 0x0) {
    uVar5 = 0x1000;
    unk_str_op_1000_3d3e
              ((param_1 & 0xffff0000 | (UVar10 + 0x10)),
               *(char **)(UVar10 + 0x94));
  }
  get_sys_metrics_1040_8c66(param_1,uVar5);
  uStack4 = *(byte *)(UVar10 + 0x98) & 0xf;
  if (uStack4 == 0x1) {
    (UVar10 + 0xae) = ((UVar10 + 0xaa) + -0xc4) / 0x2;
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0xff,local_104,param_2
              );
    create_window_1040_8bea(UVar10,uVar8,0x1,0x1,(UVar10 + 0xae));
    piVar1 = (i16 *)(UVar10 + 0xae);
    *piVar1 = *piVar1 + 0x6c;
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0xff,local_104,param_2
              );
    uVar9 = (UVar10 + 0xae);
    uVar7 = 0x2;
  }
  else {
    if (uStack4 != 0x4) {
      (UVar10 + 0xae) = ((UVar10 + 0xaa) + -0x58) / 0x2;
      load_string_1010_84e0
                (0x1010,_PTR_LOOP_1050_14cc,
                 (_PTR_LOOP_1050_14cc >> 0x10),0xff,local_104,
                 param_2);
      uVar9 = (UVar10 + 0xae);
      uVar5 = 0x1;
      uVar7 = 0x1;
      goto LAB_1040_88a5;
    }
    (UVar10 + 0xae) = ((UVar10 + 0xaa) + -0xc4) / 0x2;
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0xff,local_104,param_2
              );
    create_window_1040_8bea(UVar10,uVar8,0x1,0x6,(UVar10 + 0xae));
    piVar1 = (i16 *)(UVar10 + 0xae);
    *piVar1 = *piVar1 + 0x6c;
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0xff,local_104,param_2
              );
    uVar9 = (UVar10 + 0xae);
    uVar7 = 0x7;
  }
  uVar5 = 0x0;
LAB_1040_88a5:
  create_window_1040_8bea(UVar10,uVar8,uVar5,uVar7,uVar9);
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_2,extraout_DX,unaff_DI);
  uVar5 = (puVar6 >> 0x10);
  local_104[0] = (puVar6 + 0xa);
  uStack4 = (puVar6 + 0xc);
  iVar2 = uStack4 - (UVar10 + 0xac);
  puVar4 = (iVar2 >> 0xf);
  SetWindowPos16(0x1010,0x40,*(INT16 *)(UVar10 + 0xac),*(INT16 *)(UVar10 + 0xaa),
                 iVar2 / 0x2,(local_104[0] - (UVar10 + 0xaa)) / 0x2,0x0);
  PTR_LOOP_1050_5df4 = (&PTR_LOOP_1050_0000 + 0x1);
  unk_win_msg_op_1008_9510((i16 *)&PTR_LOOP_1050_5df4,0x1008,param_2);
  destroy_win_1040_8b7e(0x1008);
  PTR_LOOP_1050_5df6 = 0x0;
  if ((UVar10 + 0xb2) != 0x0) {
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_2,puVar4,unaff_DI);
    uVar3 = pass1_1008_ab54(puVar6);
    if (uVar3 != 0x0) {
      PostMessage16(0x1008,0x0,0x0,0x11100fc);
    }
  }
  return PTR_LOOP_1050_5df8;
}



HANDLE16 
create_window_1040_8bea
          (param_1: u32,param_2: u16,param_3: i16,INT16 param_4,HMENparam_5: u16)

{
  HANDLE16 HVar1;
  LPCSTR unaff_CS;
  LRESULT LVar2;
  HWND16 in_stack_0000000e;
  let uStack6: u32;
  
  uStack6 = 0x50010000;
  if (param_3 != 0x0) {
    uStack6 = 0x50010001;
  }
  if ((param_1 + 0x74) != 0x0) {
    uStack6 |= 0x8000000;
  }
  CreateWindow16(unaff_CS,(LPCSTR)0x0,ZEXT24(PTR_LOOP_1050_038c) << 0x10,param_4,
                 *(INT16 *)(param_1 + 0x6),0x17,0x58,in_stack_0000000e,param_5,
                 (HINSTANCE16)uStack6,(LPVOID)(uStack6 >> 0x10));
  HVar1 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)0x5e09);
  if (HVar1 != 0x0) {
    LVar2 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x1,0x0,CONCAT22(0x30,HVar1));
    HVar1 = (HANDLE16)LVar2;
  }
  return HVar1;
}


void 
enable_window_1040_8ea0
          (param_1: u16,param_2: u16,param_3: u16,param_4: u32,param_5: u16)

{
  HWND16 enable;
  let in_DX: *mut u8
  let unaff_SS: u16;
  
  if (param_4._2_2_ == 0xf8) {
    GetDlgItem16(param_5,0x17d8);
    enable = 0x1;
  }
  else {
    if (param_4._2_2_ != 0x17d8) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,param_5,unaff_SS);
      return;
    }
    SetWindowPos16(param_5,0x6,0xf6,0x269,0x0,0x0,0x0);
    enable = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x17d8);
  }
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,enable);
  return;
}


fn mix_win_ui_op_1040_911e(param_1: *mut u16)
{
  let puVar1: u32;
  let uVar2: u16;
  let uVar3: u32;
  code **ppcVar4;
  let iVar5: i16;
  let uVar6: u16;
  
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  *param_1 = 0x9800;
  (iVar5 + 0x2) = &PTR_LOOP_1050_1040;
  if ((iVar5 + 0x38) != 0x0) {
    puVar1 = (iVar5 + 0x8);
    uVar2 = (iVar5 + 0xa);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar4 = (code **)*puVar1;
      (**ppcVar4)();
    }
    puVar1 = (iVar5 + 0xc);
    uVar2 = (iVar5 + 0xe);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar4 = (code **)*puVar1;
      (**ppcVar4)();
    }
    puVar1 = (iVar5 + 0x10);
    uVar2 = (iVar5 + 0x12);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar4 = (code **)*puVar1;
      (**ppcVar4)();
    }
  }
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar5 + 0x4),0x1000);
  uVar3 = (iVar5 + 0x14);
  SetWindowLong16(0x1000,uVar3,CONCAT22(0xfffc,(uVar3 >> 0x10)));
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisLo_1050_5e1c);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisHi_1050_5e23);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_procLo_1050_5e2a);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_procHi_1050_5e31);
  RemoveProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)0x5e38);
  PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + -0x1;
  if (PTR_LOOP_1050_5e16 == 0x0) {
    FreeProcInstance16((LPVOID)s_tile2_bmp_1050_1538);
    _PTR_LOOP_1050_5e18 = 0x0;
  }
  *param_1 = 0x389a;
  (iVar5 + 0x2) = 0x1008;
  return;
}



fn enable_win_1040_9234(param_1: u32,bool param_2,HWND16 param_3)
{
  if ((param_1 + 0x18) != 0x0) {
    EnableWindow16(param_3,param_2);
  }
  return;
}


fn create_window_1040_92dc(param_1: u32,param_2: u16)
{
  HWND16 HVar1;
  LPCSTR str;
  LPCSTR str_00;
  LPCSTR str_01;
  let lVar2: i32;
  
  str_01 = (LPCSTR)(param_1 >> 0x10);
  str_00 = (LPCSTR)param_1;
  if ((str_00 + 0x18) == 0x0) {
    HVar1 = CreateWindow16((LPCSTR)param_2,(LPCSTR)0x0,ZEXT24(PTR_LOOP_1050_038c) << 0x10,
                           *(INT16 *)(str_00 + 0x1c),*(INT16 *)(str_00 + 0x1a),0x0,0x0,
                           *(HWND16 *)(str_00 + 0x20),*(HMENU16 *)(str_00 + 0x1e),0xb,
                           (LPVOID)0x4000);
    *(HWND16 *)(str_00 + 0x18) = HVar1;
    lVar2 = SetWindowLong16((HWND16)s_tile2_bmp_1050_1538,_PTR_LOOP_1050_5e18,
                            CONCAT22(0xfffc,(_PTR_LOOP_1050_5e18 >> 0x10)));
    str = (LPCSTR)(lVar2 >> 0x10);
    (str_00 + 0x14) = lVar2;
    *(LPCSTR *)(str_00 + 0x16) = str;
    SetProp16((HWND16)s_tile2_bmp_1050_1538,str,(HANDLE16)s_procHi_1050_5e46);
    SetProp16((HWND16)s_tile2_bmp_1050_1538,*(LPCSTR *)(str_00 + 0x14),
              (HANDLE16)s_procLo_1050_5e4d);
    SetProp16((HWND16)s_tile2_bmp_1050_1538,str_01,(HANDLE16)s_thisHi_1050_5e54);
    SetProp16((HWND16)s_tile2_bmp_1050_1538,str_00,(HANDLE16)s_thisLo_1050_5e5b);
    if ((str_00 + 0x40) != 0x0) {
      SetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)(&PTR_LOOP_1050_0000 + 0x1),
                0x5e62);
    }
    ShowWindow16((HWND16)s_tile2_bmp_1050_1538,0x5);
  }
  return;
}



fn mov_update_win_1040_93aa(astruct_65 *param_1,INT16 param_2,param_3: u16,HWND16 param_4)
{
  astruct_65 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_65 *)param_1;
  iVar1->field_0x1e = param_3;
  iVar1->field_0x20 = param_2;
  MoveWindow16(param_4,0x1,iVar1->field_0x24,iVar1->field_0x22,param_2,iVar1->field_0x1e);
  UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
  return;
}


LRESULT  send_msg_1040_9404(param_1: u32,HWND16 param_2)

{
  LRESULT LVar1;
  
  LVar1 = SendMessage16(param_2,0x0,0x0,
                        CONCAT22(0x111,(param_1 + 0x1c)));
  return LVar1;
}


fn win_ui_get_prop_op_1040_9566(i16 *param_1,HWND16 param_2)
{
  let uVar1: u16;
  let iVar2: i16;
  code **ppcVar3;
  HANDLE16 HVar4;
  HANDLE16 HVar5;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let puStack12: u32;
  
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  if (*param_1 == 0x4) {
    uVar1 = (iVar6 + 0xc);
    uVar9 = (iVar6 + 0xa);
    HVar4 = GetProp16(param_2,(LPCSTR)s_thisHi_1050_5e6f);
    uVar8 = (iVar6 + 0xa);
    HVar5 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisLo_1050_5e68);
    puStack12 = CONCAT22(HVar4,HVar5);
    if ((HVar4 | HVar5) != 0x0) {
      iVar2 = (iVar6 + 0x6);
      if (iVar2 == 0x1) {
        ppcVar3 = (code **)(*puStack12 + 0xc);
        (**ppcVar3)(s_tile2_bmp_1050_1538,HVar5,HVar4,(iVar6 + 0x8),
                    uVar1,uVar8,uVar9);
        return;
      }
      if (iVar2 == 0x2) {
        ppcVar3 = (code **)(*puStack12 + 0x10);
        (**ppcVar3)(s_tile2_bmp_1050_1538,HVar5,HVar4,(iVar6 + 0x8),
                    uVar1);
        return;
      }
      if (iVar2 == 0x4) {
        ppcVar3 = (code **)(*puStack12 + 0x18);
        (**ppcVar3)(s_tile2_bmp_1050_1538,HVar5,HVar4,*(byte *)(iVar6 + 0x8) & 0x10,
                    uVar1);
        return;
      }
    }
  }
  return;
}



void 
call_win_proc_1040_9684
          (HWND16 param_1,param_2: u16,WPARAM16 param_3,LPARAM param_4,HWND16 param_5,
          param_6: u16)

{
  code **ppcVar1;
  HANDLE16 HVar2;
  HANDLE16 HVar3;
  let Bvar4: bool;
  RECT16 *pRVar5;
  let uVar6: u32;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  RECT16 local_1a [0x2];
  let puStack18: u32;
  let puStack14: u32;
  let puStack10: u32;
  let lStack6: i32;
  
  uVar11 = SUB42(&USHORT_1050_1050,0x0);
  uVar10 = param_4._2_2_;
  HVar2 = GetProp16(param_5,(LPCSTR)s_procHi_1050_5e7d);
  uVar9 = param_4._2_2_;
  HVar3 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_procLo_1050_5e76);
  lStack6 = CONCAT22(HVar2,HVar3);
  uVar8 = param_4._2_2_;
  HVar2 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisHi_1050_5e8b);
  uVar7 = param_4._2_2_;
  HVar3 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)s_thisLo_1050_5e84);
  puStack10 = CONCAT22(HVar2,HVar3);
  if ((HVar2 | HVar3) != 0x0) {
    if (param_4 == 0x2) {
      puStack18 = puStack10;
      puStack14 = puStack10;
      if (puStack10 != 0x0) {
        ppcVar1 = (code **)*puStack10;
        (**ppcVar1)(s_tile2_bmp_1050_1538,HVar3,HVar2,0x1,uVar7,uVar8,uVar9,uVar10,
                    uVar11);
      }
    }
    else {
      if (param_4 == 0x201) {
        HVar2 = GetProp16((HWND16)s_tile2_bmp_1050_1538,(LPCSTR)0x5e92);
        if (HVar2 == 0x0) {
          uVar7 = (puStack10 + 0x18);
          GetClientRect16((HWND16)s_tile2_bmp_1050_1538,local_1a);
          pRVar5 = local_1a;
          uVar6 = CONCAT22(uVar7,param_6);
          BVar4 = PtInRect16((RECT16 *)s_tile2_bmp_1050_1538,
                             (POINT16)CONCAT13((char)(param_2 >> 0x8),
                                               CONCAT12((char)param_2,param_1)));
          if (BVar4 == 0x0) {
            return;
          }
          debug_print_1008_6048(CONCAT22(param_6,0x5e98),0x1008,param_6);
          ppcVar1 = (code **)(*puStack10 + 0x1c);
          (**ppcVar1)(0x1008,puStack10,(puStack10 >> 0x10),param_2,
                      param_1,param_3,pRVar5,uVar6,param_4._2_2_);
          return;
        }
      }
      else {
        if (param_4 == 0x204) {
          uVar7 = (HVar3 + 0x18);
          GetClientRect16((HWND16)s_tile2_bmp_1050_1538,local_1a);
          uVar6 = CONCAT22(param_6,local_1a);
          BVar4 = PtInRect16((RECT16 *)s_tile2_bmp_1050_1538,
                             (POINT16)CONCAT22(param_2,param_1));
          if (BVar4 == 0x0) {
            return;
          }
          debug_print_1008_6048(CONCAT22(param_6,0x5eab),0x1008,param_6);
          ppcVar1 = (code **)(*puStack10 + 0x20);
          (**ppcVar1)(0x1008,puStack10,(puStack10 >> 0x10),param_2,
                      param_1,param_3,uVar6,uVar7);
          return;
        }
      }
    }
  }
  if (lStack6 != 0x0) {
    CallWindowProc16((LPVOID)s_tile2_bmp_1050_1538,param_1,param_2,param_3,param_4);
  }
  return;
}


fn unk_win_ui_op_1040_9854(param_1: *mut u16,param_2: u16) -> u16

{
  HCURSOR16 HVar1;
  HGDIOBJ16 HVar2;
  let iVar3: i16;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  *param_1 = 0x389a;
  (iVar3 + 0x2) = 0x1008;
  *param_1 = 0xa230;
  (iVar3 + 0x2) = &PTR_LOOP_1050_1040;
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | (iVar3 + 0x4)),
             s_OPButton_1050_5ece);
  (iVar3 + 0x54) = 0x3;
  HVar1 = LoadCursor16(0x1000,(LPCSTR)0x7f00);
  *(HCURSOR16 *)(iVar3 + 0x58) = HVar1;
  HVar2 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
  *(HGDIOBJ16 *)(iVar3 + 0x56) = HVar2;
  reg_class_1040_98c0(param_1 & 0xffff | uVar4 << 0x10,
                      s_tile2_bmp_1050_1538,param_2);
  return param_1;
}


void 
win_op_1040_9cde(param_1: u16,WPARAM16 param_2,param_3: i16,param_4: u32,HWND16 param_5,
                param_6: u16)

{
  byte *pbVar1;
  let iVar2: i16;
  let uVar3: u16;
  let uVar4: u16;
  let iVar5: i16;
  let IVar6: i16;
  let BVar7: bool;
  let offset: u16;
  let puVar8: *mut u8
  let uVar9: u16;
  let uVar10: u16;
  HWND16 HVar11;
  astruct_18 *paVar12;
  INT16 *pIVar13;
  LRESULT LVar14;
  let uVar15: u32;
  let bVar16: u8;
  let uStack30: u16;
  RECT16 local_a [0x2];
  
  HVar11 = s_tile2_bmp_1050_1538;
  paVar12 = (astruct_18 *)GetWindowLong16(param_5,0x0);
  puVar8 = (paVar12 >> 0x10);
  iVar5 = paVar12;
  uVar10 = ((paVar12 & 0xffff0000) >> 0x10);
  if (param_4 == 0x30) {
    (iVar5 + 0x5a) = param_3;
  }
  else {
    bVar16 = (byte)param_4;
    if (param_4 < 0x31) {
      if (param_4 == 0x1f) {
        (iVar5 + 0x4) = 0x0;
        ReleaseCapture16();
        return;
      }
      if (0x1f < param_4) goto LAB_1040_a1ae;
      if (bVar16 == 0x8) {
        pbVar1 = (byte *)(iVar5 + 0x4);
        *pbVar1 = *pbVar1 & 0xf7;
        uStack30 = 0x0;
        BVar7 = IsWindow16((HWND16)s_tile2_bmp_1050_1538);
        if (BVar7 != 0x0) {
          uVar15 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x870000);
          uStack30 = ((uVar15 & 0x20) == 0x0);
        }
        (iVar5 + 0x56) = 0x0;
        SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,
                      CONCAT22(0x401,(iVar5 + 0x5c)));
        if (((iVar5 + 0x5c) != 0x0) &&
           ((iVar5 + 0x5c) != paVar12->field_0x0)) {
          uVar3 = (iVar5 + 0x5c);
          SendDlgItemMessage16
                    ((HWND16)s_tile2_bmp_1050_1538,uStack30,0x0,0x1,
                     CONCAT13((char)(uVar3 >> 0x8),CONCAT12((char)uVar3,0x404)));
        }
        HVar11 = s_tile2_bmp_1050_1538;
        (iVar5 + 0x5c) = 0x0;
      }
      else {
        if (bVar16 < 0x9) {
          if (bVar16 == 0x1) {
            pIVar13 = (INT16 *)GetWindowLong16((HWND16)s_tile2_bmp_1050_1538,0x0);
            iVar5 = pIVar13;
            uVar10 = ((pIVar13 & 0xffff0000) >> 0x10);
            (iVar5 + 0x2) = (param_1 + 0x8);
            IVar6 = GetDlgCtrlID16((HWND16)s_tile2_bmp_1050_1538);
            *pIVar13 = IVar6;
            (iVar5 + 0x56) = (param_1 + 0x12);
            unk_str_op_1000_3d3e
                      ((pIVar13 & 0xffff0000 | (iVar5 + 0x6)),
                       *(char **)(param_1 + 0x16));
            if ((*(byte *)(param_1 + 0x12) & 0x1) != 0x0) {
              SendMessage16(0x1000,0x0,0x0,CONCAT22(0x401,*pIVar13));
            }
            if (((param_1 + 0x14) & 0x800) == 0x0) {
              return;
            }
            pbVar1 = (byte *)(iVar5 + 0x4);
            *pbVar1 = *pbVar1 | 0x4;
            return;
          }
          if (bVar16 == 0x2) {
            fn_ptr_1000_17ce(paVar12,0x1000);
            SetWindowLong16(0x1000,0x0,0x0);
            return;
          }
          if (bVar16 != 0x7) goto LAB_1040_a1ae;
          pbVar1 = (byte *)(iVar5 + 0x4);
          *pbVar1 = *pbVar1 | 0x8;
          LVar14 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4000000);
          uVar4 = LVar14;
          if (((LVar14 >> 0x10) == 0x534b) &&
             ((iVar5 + 0x5c) = uVar4, uVar4 != paVar12->field_0x0)) {
            SendDlgItemMessage16
                      ((HWND16)s_tile2_bmp_1050_1538,0x1,0x0,0x0,CONCAT22(uVar4,0x404));
          }
          HVar11 = s_tile2_bmp_1050_1538;
          SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,
                        CONCAT22(0x401,paVar12->field_0x0));
          (iVar5 + 0x56) = 0x1;
        }
        else {
          if (bVar16 == 0xa) {
            pbVar1 = (byte *)(iVar5 + 0x4);
            *pbVar1 = *pbVar1 & 0xfb;
            if (param_3 == 0x0) {
              pbVar1 = (byte *)(iVar5 + 0x4);
              *pbVar1 = *pbVar1 | 0x4;
            }
          }
          else {
            if (bVar16 != 0xc) {
              if (bVar16 == 0xf) {
                draw_op_1040_9948(param_4._2_2_,paVar12,s_tile2_bmp_1050_1538,
                                  param_6);
                return;
              }
              goto LAB_1040_a1ae;
            }
            if (CONCAT22(param_2,param_1) != 0x0) {
              HVar11 = 0x1000;
              unk_str_op_1000_3d3e
                        ((paVar12 & 0xffff0000 | (iVar5 + 0x6)),
                         CONCAT22(param_2,param_1));
            }
          }
        }
      }
      goto LAB_1040_9e20;
    }
    if (param_4 == 0x200) {
      if ((*(byte *)(iVar5 + 0x4) & 0x1) == 0x0) {
        return;
      }
      GetClientRect16((HWND16)s_tile2_bmp_1050_1538,local_a);
      iVar2 = (iVar5 + 0x4);
      BVar7 = PtInRect16((RECT16 *)s_tile2_bmp_1050_1538,
                         (POINT16)CONCAT22(param_2,param_1));
      if (BVar7 == 0x0) {
        pbVar1 = (byte *)(iVar5 + 0x4);
        *pbVar1 = *pbVar1 & 0xfd;
      }
      else {
        pbVar1 = (byte *)(iVar5 + 0x4);
        *pbVar1 = *pbVar1 | 0x2;
      }
      param_1 = (iVar5 + 0x4) - iVar2;
    }
    else {
      if (param_4 < 0x201) {
        offset = param_4 - 0x81;
        if (offset == 0x0) {
          uVar10 = 0x5e;
          mem_op_1000_179c(0x5e,puVar8,0x1000);
          uVar9 = puVar8 | offset;
          if (uVar9 == 0x0) {
            offset = 0x0;
            uVar9 = 0x0;
          }
          else {
            pass1_1040_9824(CONCAT22(puVar8,offset));
          }
          SetWindowLong16(0x1000,offset,CONCAT22(uVar10,uVar9));
          return;
        }
        if (param_4 == 0x87) {
          return;
        }
        if (param_4 == 0x100) {
          if ((param_3 == 0x26) || (param_3 == 0x25)) {
            HVar11 = 0x1;
          }
          else {
            if ((param_3 != 0x28) && (param_3 != 0x27)) {
              if (((param_3 == 0x20) || (param_3 == 0xd)) &&
                 (&PTR_LOOP_1050_5ed8 == 0x0)) {
                &PTR_LOOP_1050_5ed8 = 0x1;
                pbVar1 = (byte *)(iVar5 + 0x4);
                *pbVar1 = *pbVar1 | 0x2;
                goto LAB_1040_9e20;
              }
              goto LAB_1040_a1ae;
            }
            HVar11 = 0x0;
          }
          GetNextDlgTabItem16((HWND16)s_tile2_bmp_1050_1538,HVar11,param_4._2_2_);
          SetFocus16((HWND16)s_tile2_bmp_1050_1538);
          return;
        }
        if ((param_4 == 0x101) && (&PTR_LOOP_1050_5ed8 != 0x0)) {
          &PTR_LOOP_1050_5ed8 = 0x0;
          pbVar1 = (byte *)(iVar5 + 0x4);
          *pbVar1 = *pbVar1 & 0xfd;
          InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,
                           (RECT16 *)(&PTR_LOOP_1050_0000 + 0x1),0x0);
          UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
          SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,
                        CONCAT22(0x111,paVar12->field_0x0));
          return;
        }
LAB_1040_a1ae:
        DefWindowProc16((HWND16)s_tile2_bmp_1050_1538,param_1,param_2,
                        CONCAT13((char)(param_4 >> 0x8),CONCAT12(bVar16,param_3)));
        return;
      }
      if (param_4 == 0x201) {
LAB_1040_9e74:
        SetFocus16((HWND16)s_tile2_bmp_1050_1538);
        pbVar1 = (byte *)(iVar5 + 0x4);
        *pbVar1 = *pbVar1 | 0x3;
        InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,
                         (RECT16 *)(&PTR_LOOP_1050_0000 + 0x1),0x0);
        UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
        SetCapture16((HWND16)s_tile2_bmp_1050_1538);
        return;
      }
      if (param_4 == 0x202) {
        ReleaseCapture16();
        GetClientRect16((HWND16)s_tile2_bmp_1050_1538,local_a);
        if ((*(byte *)(iVar5 + 0x4) & 0x1) == 0x0) {
          return;
        }
        pbVar1 = (byte *)(iVar5 + 0x4);
        *pbVar1 = *pbVar1 & 0xfc;
        InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,
                         (RECT16 *)(&PTR_LOOP_1050_0000 + 0x1),0x0);
        UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
        BVar7 = PtInRect16((RECT16 *)s_tile2_bmp_1050_1538,
                           (POINT16)CONCAT22(param_2,param_1));
        if (BVar7 == 0x0) {
          return;
        }
        PostMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,
                      CONCAT22(0x111,paVar12->field_0x0));
        return;
      }
      if (param_4 == 0x203) goto LAB_1040_9e74;
      if (param_4 != 0x404) goto LAB_1040_a1ae;
      if (param_3 == 0x1) {
        (iVar5 + 0x56) = 0x1;
      }
      else {
        (iVar5 + 0x56) = 0x0;
      }
    }
  }
  HVar11 = s_tile2_bmp_1050_1538;
  if (param_1 == 0x0) {
    return;
  }
LAB_1040_9e20:
  InvalidateRect16(HVar11,(RECT16 *)(&PTR_LOOP_1050_0000 + 0x1),0x0);
  UpdateWindow16((HWND16)s_tile2_bmp_1050_1538);
  return;
}


fn win_msg_1040_a308(param_1: u32,param_2: i16,HWND16 param_3,param_4: u16) -> u32

{
  let piVar1: *mut i16;
  let uVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  HWND16 hwnd;
  LRESULT LVar5;
  let puVar6: *mut u16;
  char *pcVar7;
  let uVar8: u16;
  let uVar9: u16;
  let iStack12: i16;
  
  SendMessage16(param_3,0x0,0x0,0x4050000);
  LVar5 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xb0000);
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  uVar2 = (iVar3 + 0x90);
  if ((uVar2 + 0x10) == 0x0) {
    uVar4 = 0x0;
    uVar8 = 0x401;
    pcVar7 = load_string_1010_847e
                       (_PTR_LOOP_1050_14cc,
                        (_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    hwnd = s_tile2_bmp_1050_1538;
    SendMessage16(0x1010,pcVar7,(WPARAM16)(pcVar7 >> 0x10),
                  CONCAT22(uVar8,uVar4));
  }
  else {
    hwnd = 0x1010;
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_4,
                             (LVar5 >> 0x10),param_2);
    for (iStack12 = 0x0; uVar2 = (iVar3 + 0x90),
        piVar1 = (i16 *)(uVar2 + 0x10), *piVar1 != iStack12 && iStack12 <= *piVar1;
        iStack12 += 0x1) {
      uVar8 = 0x0;
      uVar9 = 0x401;
      uVar2 = (iVar3 + 0x90);
      uVar2 = (uVar2 + 0xc);
      pcVar7 = load_string_1010_ac92
                         (0x1010,puVar6,(puVar6 >> 0x10),
                          (uVar2 + iStack12 * 0x2));
      hwnd = s_tile2_bmp_1050_1538;
      SendMessage16(0x1010,pcVar7,(WPARAM16)(pcVar7 >> 0x10),
                    CONCAT22(uVar9,uVar8));
    }
  }
  LVar5 = SendMessage16(hwnd,0x0,0x0,0xb0001);
  return CONCAT22((LVar5 >> 0x10),0x1);
}



fn get_dlg_item_1040_a3d0(param_1: u32,HWND16 param_2)
{
  let lVar1: i32;
  astruct_49 *iVar3;
  let unaff_DI: i16;
  let uVar2: u16;
  let unaff_SS: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar3 = (astruct_49 *)param_1;
  if (iVar3->field_0x90 != 0x0) {
    lVar1 = iVar3->field_0x90;
    (lVar1 + 0x14) = iVar3->field_0x6;
    GetDlgItem16(param_2,0x1826);
    win_msg_1040_a308(param_1,unaff_DI,s_tile2_bmp_1050_1538,unaff_SS);
  }
  return;
}


void 
win_ui_op_1040_a784(param_1: i16,param_2: i16,param_3: u16,param_4: u32,param_5: u16,
                   param_6: u16,param_7: u16)

{
  let iVar1: i16;
  
  if (param_4._2_2_ != 0xeb) {
    if (param_4._2_2_ == 0x1f4) {
      msg_box_op_1040_a85a(CONCAT22(param_2,param_1),0x0,param_5,param_7);
      return;
    }
    if (param_4._2_2_ == 0x1f7) {
      _PTR_LOOP_1050_5ef0 = (param_1 + 0x94);
      pass1_1038_af40(_PTR_LOOP_1050_5b7c,(param_1 + 0x8),0x23,param_5,param_1,
                      &PTR_LOOP_1050_1038,param_7);
      PostMessage16((HWND16)&PTR_LOOP_1050_1038,0x0,0x0,0x1110002);
      return;
    }
    if (param_4._2_2_ != 0x17d8) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4,param_5,param_6,param_7);
      return;
    }
    iVar1 = (param_1 + 0x6);
    SetWindowPos16(param_6,0x6,0xed,0x237,0x0,0x0,0x0);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x17d8);
    EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
    (param_1 + 0x98) = 0x1;
    param_1 = param_2;
    param_2 = iVar1;
  }
  win_ui_dlg_op_1040_a94a(CONCAT22(param_2,param_1),param_7);
  return;
}


fn msg_box_op_1040_a85a(param_1: u32,char *param_2,uchar *param_3,param_4: u16)
{
  char local_206 [0x102];
  char local_104 [0x102];
  
  mem_op_1000_179c(0x1000,param_3,0x1000);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x100,local_206,param_4);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,param_2,(short)param_3);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,(LPCSTR)0x0,local_206,param_4);
  fn_ptr_1000_17ce((astruct_18 *)CONCAT22(param_3,param_2),0x1000);
  return;
}


fn win_ui_dlg_op_1040_a94a(param_1: u32,param_2: u16)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let uVar3: u32;
  let uVar4: u16;
  let value: *mut u8;
  char *pcVar5;
  let uVar6: u16;
  let in_DX: *mut u8
  let puVar8: *mut u8
  let puVar9: *mut u8
  let lp_string: u16;
  let iVar10: i16;
  let iVar11: i16;
  let unaff_DI: i16;
  let uVar12: u16;
  let uVar13: u16;
  HWND16 HVar14;
  ulet in_AF: u8;
  let bVar15: bool;
  let puVar16: *mut u16;
  let lVar17: i32;
  let uStack288: u16;
  let uStack286: u16;
  let BStack278: bool;
  let iStack276: i16;
  let local_102: [u8;100];
  let uVar7: u32;
  
  puVar16 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_2,in_DX,unaff_DI);
  puVar8 = (puVar16 >> 0x10);
  uVar4 = puVar16;
  uVar12 = (param_1 >> 0x10);
  iVar10 = param_1;
  puVar9 = puVar8;
  pass1_1010_c3c2(uVar4,puVar8,CONCAT22(param_2,local_102),
                  (iVar10 + 0x94),puVar8,in_AF,param_2);
  SetDlgItemText16(0x1010,local_102,param_2);
  pass1_1010_c320(uVar4,puVar8,CONCAT22(param_2,local_102),
                  (iVar10 + 0x94),puVar9);
  SetDlgItemText16(0x1010,local_102,param_2);
  string_op_1010_c446(param_2,in_AF,puVar9,puVar16,CONCAT22(param_2,local_102),
                      (iVar10 + 0x94));
  value = local_102;
  SetDlgItemText16(0x1010,value,param_2);
  pass1_1030_6ddc((iVar10 + 0x94));
  SetDlgItemInt16(0x1030,0x0,value,0x1f5);
  pass1_1030_6e14((iVar10 + 0x94));
  SetDlgItemInt16(0x1030,0x0,value,0x1f6);
  if ((iVar10 + 0x98) != 0x0) {
    HVar14 = 0x1010;
    struct_1010_dd5e(uVar4,puVar8,(iVar10 + 0x94));
    if ((puVar9 | value) != 0x0) {
      uVar3 = (iVar10 + 0x94);
      uVar13 = (uVar3 >> 0x10);
      iVar11 = uVar3;
      uVar2 = (iVar11 + 0x26);
      lp_string = (iVar11 + 0x28);
      iStack276 = 0x1798;
      BStack278 = 0x17c3;
      (iVar10 + 0xea) = 0x0;
      uVar7 = uVar2;
      for (uStack288 = 0x1; uStack288 < 0x25; uStack288 += 0x1) {
        if (uVar2 == 0x0) {
          lVar17 = 0x0;
        }
        else {
          HVar14 = 0x1020;
          lVar17 = pass1_1020_bae6(uVar2,CONCAT22(uStack288,(uVar2 >> 0x10)),
                                   uVar7,lp_string,param_2);
        }
        uVar6 = (lVar17 >> 0x10);
        bVar15 = *(long *)(value + uStack288 * 0x4) != 0x0;
        lp_string = uVar6;
        if (bVar15) {
          pcVar5 = string_1020_c0d8(uStack288);
          SetDlgItemText16(0x1020,pcVar5,lp_string);
          HVar14 = s_tile2_bmp_1050_1538;
          SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x0,
                          (value + uStack288 * 0x4),BStack278);
        }
        uStack286 = lVar17;
        uVar6 |= uStack286;
        if (lVar17 != 0x0) {
          if (!bVar15) {
            pcVar5 = string_1020_c0d8(uStack288);
            HVar14 = s_tile2_bmp_1050_1538;
            SetDlgItemText16(0x1020,pcVar5,lp_string);
          }
          SetDlgItemInt16(HVar14,0x0,uStack286,BStack278);
          iVar11 = (iVar10 + 0xea);
          HVar14 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,iStack276);
          *(HWND16 *)(iVar10 + iVar11 * 0x2 + 0x9a) = HVar14;
          piVar1 = (i16 *)(iVar10 + 0xea);
          *piVar1 = *piVar1 + 0x1;
          iVar11 = (iVar10 + 0xea);
          HVar14 = s_tile2_bmp_1050_1538;
          uVar6 = GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,BStack278);
          *(HWND16 *)(iVar10 + iVar11 * 0x2 + 0x9a) = uVar6;
          piVar1 = (i16 *)(iVar10 + 0xea);
          *piVar1 = *piVar1 + 0x1;
          bVar15 = true;
        }
        uVar7 = uVar6;
        if (bVar15) {
          iStack276 += 0x1;
          BStack278 += 0x1;
        }
      }
    }
  }
  return;
}


fn msg_box_ui_op_1040_ad66(param_1: u32,char *param_2,uchar *param_3,param_4: u16)
{
  char local_206 [0x102];
  char local_104 [0x102];
  
  mem_op_1000_179c(0x1000,param_3,0x1000);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x100,local_206,param_4);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,param_2,(short)param_3);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,(LPCSTR)0x0,local_206,param_4);
  fn_ptr_1000_17ce((astruct_18 *)CONCAT22(param_3,param_2),0x1000);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn win_ui_op_1040_ae04(param_1: u32,param_2: *mut u16)
{
  let bVar1: bool;
  let iVar2: i16;
  char *id;
  let in_DX: *mut u8
  let uVar3: u16;
  SEGPTR lp_string;
  let iVar4: i16;
  let uVar5: u16;
  let unaff_DI: i16;
  let uVar6: u16;
  let uVar7: u16;
  ulet in_AF: u8;
  let puVar8: *mut u16;
  let uVar9: u32;
  let uVar10: u32;
  char *pcVar11;
  let iStack280: i16;
  CHAR local_102 [0x100];
  
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_2,in_DX,unaff_DI);
  uVar3 = (puVar8 >> 0x10);
  uVar6 = (param_1 >> 0x10);
  iVar4 = param_1;
  pass1_1010_c3c2(puVar8,uVar3,CONCAT22(param_2,local_102),
                  (iVar4 + 0x94),uVar3,in_AF,param_2);
  SetDlgItemText16(0x1010,local_102,(SEGPTR)param_2);
  uVar9 = struct_op_1030_73a8((iVar4 + 0x94));
  iVar2 = uVar9 + 0x20;
  uVar10 = pass1_1030_8326();
  lp_string = (SEGPTR)(uVar10 >> 0x10);
  bVar1 = false;
  for (iStack280 = 0x0; iStack280 < 0xa; iStack280 += 0x1) {
    uVar7 = ((uVar9 & 0xffff0000) >> 0x10);
    if (((iStack280 * 0xc + iVar2 + 0x2) | (iStack280 * 0xc + iVar2)) !=
        0x0) {
      uVar5 = iStack280 * 0xc + iVar2;
      id = string_op_1020_c222((uVar5 + 0x4));
      SetDlgItemText16(0x1020,id,lp_string);
      wsprintf16((LPSTR)s_tile2_bmp_1050_1538,local_102,param_2);
      SetDlgItemText16((HWND16)s_tile2_bmp_1050_1538,local_102,(SEGPTR)param_2);
      uVar10 = unk_load_str_op_1010_8c96
                         ((iVar4 + 0x98),CONCAT22(param_2,local_102),
                          uVar9 & 0xffff0000 | uVar5,0x1010,param_2);
      lp_string = (SEGPTR)uVar10;
      SetDlgItemText16(0x1010,local_102,(SEGPTR)param_2);
      bVar1 = true;
    }
  }
  if (!bVar1) {
    pcVar11 = load_string_1010_847e
                        (_PTR_LOOP_1050_14cc,
                         (_PTR_LOOP_1050_14cc >> 0x10),0x1010);
    SetDlgItemText16(0x1010,pcVar11,(SEGPTR)(pcVar11 >> 0x10));
  }
  return;
}


fn unk_win_ui_op_1040_b230(astruct_1 *param_1,param_2: u16,param_3: u16)
{
  code **ppcVar1;
  let IVar2: i16;
  let in_DX: *mut u8
  let unaff_DI: i16;
  let uVar3: u16;
  let puVar4: *mut u16;
  let puVar5: *mut u16;
  let uVar7: u16;
  let uVar6: u32;
  RECT16 local_1a;
  let iStack22: i16;
  let iStack20: i16;
  let iStack18: i16;
  let iStack16: i16;
  let iStack14: i16;
  let iStack12: i16;
  let puStack10: *mut u16;
  let local_6: i16;
  let local_4: i16;
  
  dialog_ui_fn_1040_78e2(param_1,param_2);
  if (PTR_LOOP_1050_5ef8 == (&DAT_1050_0004 + 0x1)) {
    PTR_LOOP_1050_5ef8 = 0x0;
  }
  puVar5 = CONCAT22(param_3,&local_4);
  puVar4 = CONCAT22(param_3,&local_6);
  puStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_3,in_DX,unaff_DI);
  pass1_1008_3e94((u16 *)
                  (puStack10 & 0xffff0000 | (puStack10 + 0xe)),puVar4,
                  puVar5);
  uVar3 = (puStack10 >> 0x10);
  iStack12 = (puStack10 + 0xa);
  iStack14 = (puStack10 + 0xc);
  uVar7 = 0x4;
  IVar2 = GetSystemMetrics16(0x1008);
  iStack16 = IVar2 * PTR_LOOP_1050_5ef8 + 0xa;
  PTR_LOOP_1050_5ef8 = PTR_LOOP_1050_5ef8 + 0x1;
  iStack18 = iStack16 + local_6;
  iStack16 += local_4;
  uVar3 = (param_1 >> 0x10);
  uVar6 = CONCAT22(uVar7,(param_1 + 0x6));
  GetWindowRect16((HWND16)s_tile2_bmp_1050_1538,&local_1a);
  if (iStack14 < (iStack20 - local_1a.y) + iStack18) {
    iStack18 = -0x2 - ((iStack20 - local_1a.y) - iStack14);
  }
  if (iStack12 < (iStack22 - local_1a.x) + iStack16) {
    iStack16 = -0x2 - ((iStack22 - local_1a.x) - iStack12);
  }
  uVar3 = (param_1 + 0x6);
  SetWindowPos16((HWND16)s_tile2_bmp_1050_1538,0x1,0x0,0x0,iStack18,iStack16,0x0);
  ppcVar1 = (code **)(param_1->field_0x0 + 0x6c);
  (**ppcVar1)(s_tile2_bmp_1050_1538,param_1,uVar3,uVar6);
  return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn win_ui_op_1040_b372(Uparam_1: i32,param_2: u16,param_3: u16,COLORREF in_colorref_4)
{
  let uVar1: u16;
  let iVar2: i16;
  HBRUSH16 local_brush_handle;
  let IVar3: i16;
  Ulet uVar4: i32;
  let extraout_DX: u16;
  let uVar5: u16;
  HWND16 local_win_handle;
  let uVar6: u32;
  COLORREF local_colorref;
  
  uVar5 = (param_1 >> 0x10);
  local_colorref = in_colorref_4;
  if ((param_1 + 0x8e) == 0x0) {
    local_colorref = (COLORREF)s_tile2_bmp_1050_1538;
    local_brush_handle = CreateSolidBrush16(in_colorref_4);
    *(HBRUSH16 *)(param_1 + 0x8e) = local_brush_handle;
  }
  if (_PTR_LOOP_1050_5efa == 0x0) {
    local_colorref = 0x1008;
    uVar6 = pass1_1008_4d72((_PTR_LOOP_1050_4230 + 0xe));
    uVar1 = (uVar6 >> 0x10);
    iVar2 = uVar6;
    _PTR_LOOP_1050_5efa =
         CONCAT12(*(iVar2 + 0x94),
                         CONCAT11(*(iVar2 + 0x95),
                                  *(iVar2 + 0x96)));
  }
  if (param_3 < 0x4) {
LAB_1040_b3ea:
    local_win_handle = s_tile2_bmp_1050_1538;
    IVar3 = GetDlgCtrlID16(local_colorref);
    if (IVar3 == 0x14c) {
      local_colorref = 0x0;
      goto LAB_1040_b41a;
    }
    if (IVar3 == 0x175) {
      local_colorref = 0x0;
      goto LAB_1040_b41a;
    }
  }
  else {
    local_win_handle = local_colorref;
    if (param_3 != 0x4) {
      if ((param_3 == 0x4) || (0x1 < param_3 - 0x5)) {
        return;
      }
      goto LAB_1040_b3ea;
    }
  }
  local_colorref = (COLORREF)_PTR_LOOP_1050_5efa;
LAB_1040_b41a:
  SetTextColor16(local_win_handle,local_colorref);
  SetBkColor16((HDC16)s_tile2_bmp_1050_1538,0x0);
  return;
}



fn show_win_1040_b43c(param_1: *mut u32,HWND16 param_2)
{
  code **ppcVar1;
  
  ppcVar1 = (code **)(*param_1 + 0x70);
  (**ppcVar1)(param_2,param_1);
  ShowWindow16(param_2,0x5);
  return;
}


fn win_ui_1040_b8d2(astruct_1 *param_1,param_2: u16,param_3: u16,param_4: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  let uVar3: u32;
  astruct_160 *paVar4;
  let uVar5: u16;
  let uVar6: u16;
  let iVar7: i16;
  let puVar8: *mut u8
  let puVar9: *mut u8
  let uVar10: u16;
  let uVar11: u16;
  astruct_167 *iVar10;
  let unaff_DI: i16;
  let uVar12: u16;
  let puVar13: *mut u16;
  
  dialog_ui_fn_1040_78e2(param_1,param_3);
  puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x31,param_4,param_2,unaff_DI);
  puVar9 = (puVar13 >> 0x10);
  paVar4 = (astruct_160 *)puVar13;
  uVar12 = (param_1 >> 0x10);
  iVar10 = (astruct_167 *)param_1;
  *(astruct_160 **)&iVar10->field_0x98 = paVar4;
  *(uchar **)(&iVar10->field_0x98 + 0x2) = puVar9;
  mem_op_1000_179c(0xa,puVar9,0x1000);
  puVar8 = (puVar9 | paVar4);
  if (puVar8 == 0x0) {
    iVar10->field_0x94 = 0x0;
  }
  else {
    puVar13 = struct_1040_bf3e((u16 *)CONCAT22(puVar9,paVar4),iVar10->field_0x6);
    puVar8 = (puVar13 >> 0x10);
    paVar4 = (astruct_160 *)puVar13;
    *(astruct_160 **)&iVar10->field_0x94 = paVar4;
    *(uchar **)(&iVar10->field_0x94 + 0x2) = puVar8;
  }
  pass1_1040_bfde(iVar10->field_0x94,iVar10->field_0x98,param_4);
  mem_op_1000_179c(0x42,puVar8,0x1000);
  puVar9 = (puVar8 | paVar4);
  if (puVar9 != 0x0) {
    pass1_1008_3bd6(paVar4,puVar8,0x1,0xa000a,0x0,0x800081,
                    CONCAT22(iVar10->field_0x6,0x10a),puVar9,param_4);
  }
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar8 = (puVar9 | paVar4);
  if (puVar8 != 0x0) {
    pass1_1008_3bd6(paVar4,puVar9,0x1,0xa0028,0x0,0x840085,
                    CONCAT22(iVar10->field_0x6,0x10b),puVar8,param_4);
  }
  mem_op_1000_179c(0x42,puVar8,0x1000);
  puVar9 = (puVar8 | paVar4);
  if (puVar9 != 0x0) {
    pass1_1008_3bd6(paVar4,puVar8,0x1,0xa0046,0x0,0x860087,
                    CONCAT22(iVar10->field_0x6,0x10d),puVar9,param_4);
  }
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar8 = (puVar9 | paVar4);
  if (puVar8 != 0x0) {
    pass1_1008_3bd6(paVar4,puVar9,0x1,0xa0064,0x0,0x880089,
                    CONCAT22(iVar10->field_0x6,0x10e),puVar8,param_4);
  }
  mem_op_1000_179c(0x42,puVar8,0x1000);
  puVar9 = (puVar8 | paVar4);
  if (puVar9 != 0x0) {
    pass1_1008_3bd6(paVar4,puVar8,0x1,0xa0082,0x0,0x820083,
                    CONCAT22(iVar10->field_0x6,0x10c),puVar9,param_4);
  }
  mem_op_1000_179c(0x42,puVar9,0x1000);
  puVar8 = (puVar9 | paVar4);
  if (puVar8 != 0x0) {
    pass1_1008_3bd6(paVar4,puVar9,0x1,0xa00d2,0x0,0x8a008b,
                    CONCAT22(iVar10->field_0x6,0xbbb),puVar8,param_4);
  }
  mem_op_1000_179c(0x42,puVar8,0x1000);
  puVar9 = (puVar8 | paVar4);
  if (puVar9 == 0x0) {
    paVar4 = (astruct_160 *)0x0;
    puVar9 = 0x0;
  }
  else {
    pass1_1008_3bd6(paVar4,puVar8,0x1,0xa00a0,0x8e,0x8c008d,
                    CONCAT22(iVar10->field_0x6,0xbbc),puVar9,param_4);
  }
  puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_4,puVar9,unaff_DI);
  uVar10 = (puVar13 >> 0x10);
  uVar2 = puVar13;
  uVar11 = uVar10;
  uVar5 = pass1_1010_a5ac(uVar2,uVar10,iVar10->field_0xb0);
  uVar6 = pass1_1010_ac62(uVar2,uVar10,0x1e,uVar5,uVar11);
  if (uVar6 != 0x0) {
    pass1_1010_a5ca(uVar2,uVar10,uVar5,uVar6,uVar11);
    if (0x0 < uVar6) {
      pass1_1010_a58a(uVar2,uVar10,uVar5,uVar6,uVar11);
      if (uVar6 == 0x0) goto LAB_1040_bb26;
    }
  }
  enable_win_1040_9234(CONCAT22(puVar9,paVar4),0x0,0x1010);
LAB_1040_bb26:
  puVar1 = iVar10->field_0x98;
  iVar7 = puVar1;
  uVar3 = puVar1 & 0xffff0000;
  uVar12 = (uVar3 >> 0x10);
  SetWindowPos16(0x1010,0x40,*(INT16 *)(iVar7 + 0x10),*(INT16 *)(iVar7 + 0xe),
                 *(INT16 *)(iVar7 + 0xc),*(INT16 *)(uVar3 | iVar7 + 0xa),0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
win_ui_op_1040_bbe2(param_1: i16,param_2: u16,param_3: u16,param_4: u32,HWND16 param_5
                   ,param_6: u16)

{
  code **ppcVar1;
  let BVar2: bool;
  let iVar3: i16;
  let uVar4: u16;
  let in_DX: *mut u8
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let unaff_DI: i16;
  let puVar8: *mut u16;
  let puVar9: *mut u16;
  let uVar10: u32;
  let uVar11: u16;
  let uVar12: u16;
  let uVar13: u16;
  let uStack30: u16;
  RECT16 local_a;
  let iStack6: i16;
  let iStack4: i16;
  
  if (param_4._2_2_ != 0x10c) {
    if (param_4._2_2_ < 0x10d) {
      if (param_4._2_2_ == 0xfa) {
        uVar10 = (param_1 + 0x98);
        ppcVar1 = (code **)((param_1 + 0x98) + 0x18);
        (**ppcVar1)(param_5,uVar10,(uVar10 >> 0x10),0x0,
                    _PTR_LOOP_1050_5efe,(_PTR_LOOP_1050_5efe >> 0x10));
        return;
      }
      if (param_4._2_2_ == 0x10a) {
        GetClientRect16(param_5,&local_a);
        uVar10 = (param_1 + 0x98);
        local_a.y += 0x3;
        local_a.x = (uVar10 + 0x1a) + -0x9;
        iStack6 += -0x3;
        iStack4 += -0x3;
        InvalidateRect16((HWND16)s_tile2_bmp_1050_1538,
                         (RECT16 *)(&PTR_LOOP_1050_0000 + 0x1),(bool)&local_a);
        unk_destroy_win_op_1010_2fa0(*(ULONG *)(param_1 + 0x98),0x1010);
        pass1_1010_32c0((param_1 + 0x98),0x0);
        pass1_1010_2ee2(*(u32 **)(param_1 + 0x98),param_6,0x1010);
        return;
      }
      if (param_4._2_2_ != 0x10b) goto LAB_1040_be78;
      uVar10 = (param_1 + 0x98);
      uVar11 = (uVar10 + 0x12);
      uVar6 = uVar11;
      puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,in_DX,unaff_DI);
      uVar5 = (puVar8 >> 0x10);
      puVar9 = puVar8;
      pass1_1010_a5ca(puVar8,uVar5,uVar6,puVar8,uVar5);
      uVar6 = (puVar9 >> 0x10);
      if ((uVar11 != 0x70) && (puVar9 == 0x0)) {
        return;
      }
      uVar10 = (param_1 + 0xb0);
      uVar12 = uVar10;
      uVar13 = (uVar10 >> 0x10);
      uVar10 = (param_1 + 0x98);
      uVar11 = (uVar10 + 0x12);
    }
    else {
      if (param_4._2_2_ != 0x10d) {
        if (param_4._2_2_ == 0x10e) {
          puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_6,in_DX,unaff_DI);
          iVar3 = puVar8;
          ui_op_1010_79aa(puVar8,0xfc6,*(long *)(param_1 + 0xb0),param_6);
          if (iVar3 != 0x0) {
            return;
          }
          unk_win_op_1010_7300(puVar8,0x0,0x13,(param_1 + 0xb0));
          return;
        }
        if (param_4._2_2_ == 0xbbb) {
          if ((param_1 + 0xb6) != 0x0) {
            BVar2 = IsWindow16(param_5);
            param_5 = s_tile2_bmp_1050_1538;
            if (BVar2 != 0x0) goto LAB_1040_bd39;
          }
          uVar10 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,(param_1 + 0x6),0x1b,
                                   in_DX,param_1,&PTR_LOOP_1050_1038,
                                   param_6);
          (param_1 + 0xb6) = (uVar10 + 0x6);
          ShowWindow16((HWND16)&PTR_LOOP_1050_1038,0x1);
          return;
        }
        if (param_4._2_2_ == 0xbbc) {
          puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,in_DX,unaff_DI);
          uVar7 = (puVar8 >> 0x10);
          uVar6 = puVar8;
          uVar5 = uVar7;
          uVar4 = pass1_1010_a5ac(uVar6,uVar7,(param_1 + 0xb0));
          uVar11 = uVar4;
          pass1_1010_a58a(uVar6,uVar7,uVar4,uVar4,uVar5);
          if (uVar11 == 0x0) {
            pass1_1010_a568(uVar6,uVar7,uVar4,0x0,uVar5);
          }
          GetDlgItem16(0x1010,0xbbc);
          EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
          return;
        }
LAB_1040_be78:
        pass1_1040_b54a(param_1,param_2,param_3,param_4,in_DX,param_5,param_6);
        return;
      }
      puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,in_DX,unaff_DI);
      uVar6 = (puVar8 >> 0x10);
      uVar10 = (param_1 + 0xb0);
      uVar12 = uVar10;
      uVar13 = (uVar10 >> 0x10);
      uVar11 = 0x71;
      uVar5 = uVar6;
    }
    uStack30 = puVar8;
    param_5 = 0x1010;
    pass1_1010_a5ec(uStack30,uVar5,uVar11,CONCAT22(uVar13,uVar12),uVar6);
    if ((param_1 + 0xb4) != 0x0) {
      param_5 = s_tile2_bmp_1050_1538;
      BVar2 = IsWindow16(0x1010);
      if (BVar2 != 0x0) {
        param_5 = s_tile2_bmp_1050_1538;
        SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x11100eb);
      }
    }
  }
LAB_1040_bd39:
  DestroyWindow16(param_5);
  return;
}


fn send_msg_1040_c85a(param_1: u32,HWND16 param_2)
{
  _PTR_LOOP_1050_5efe = param_1;
  SendMessage16(param_2,0x0,0x0,0x11100fa);
  return;
}


fn win_ui_op_1040_cace(param_1: u32,HWND16 param_2,param_3: u16)
{
  let uVar1: u32;
  let bVar2: bool;
  let iVar3: i16;
  let IVar4: i16;
  let in_DX: u16;
  let uVar5: u16;
  let uVar6: u16;
  let bVar7: bool;
  let uVar8: u16;
  char local_208 [0x100];
  char local_108 [0x100];
  let UStack8: u16;
  let uStack6: u16;
  let local_4: bool;
  
  uVar6 = (param_1 >> 0x10);
  uVar5 = param_1;
  uStack6 = GetDlgItemInt16(param_2,0x0,&local_4,param_3);
  UStack8 = GetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x0,&local_4,param_3);
  if (uStack6 == 0x0) {
    return;
  }
  pass1_1018_50ea((uVar5 + 0x98),uStack6,(uVar5 + 0x94));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_208,param_3);
  uVar1 = (uVar5 + 0x94);
  uVar8 = (_PTR_LOOP_1050_14cc >> 0x10);
  if (*(long *)(uVar1 + 0x36) == 0x0) {
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,uVar8,0x3ff,local_108,param_3);
    iVar3 = MessageBox16(0x1010,(LPCSTR)(s_New_failed_in_Op__Op_1050_0020 + 0x14),
                         local_208,param_3);
  }
  else {
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,uVar8,0x3ff,local_108,param_3);
    iVar3 = MessageBox16(0x1010,(LPCSTR)(s_New_failed_in_Op__Op_1050_0020 + 0x14),
                         local_208,param_3);
  }
  bVar2 = iVar3 == 0x6;
  bVar7 = false;
  if ((!bVar2) &&
     (uVar1 = (uVar5 + 0x94), (uVar1 + 0x34) < 0x1)) {
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_108,param_3);
    IVar4 = MessageBox16(0x1010,(LPCSTR)(s_New_failed_in_Op__Op_1050_0020 + 0x14),
                         local_208,param_3);
    bVar7 = IVar4 == 0x6;
    bVar2 = false;
  }
  if (bVar2) {
    _PTR_LOOP_1050_5f16 = (uVar5 + 0x94);
    iVar3 = 0x26;
  }
  else {
    if (!bVar7) {
      return;
    }
    _PTR_LOOP_1050_5a68 = (uVar5 + 0x94);
    iVar3 = 0x27;
  }
  pass1_1038_af40(_PTR_LOOP_1050_5b7c,(uVar5 + 0x8),iVar3,in_DX,uVar5,
                  &PTR_LOOP_1050_1038,param_3);
  return;
}


fn msg_box_op_1040_cce4(param_1: u32,char *param_2,uchar *param_3,param_4: u16)
{
  let uStack522: u32;
  char local_206 [0x102];
  char local_104 [0x102];
  let uVar2: u16;
  let uVar3: u16;
  
  mem_op_1000_179c(0x1000,param_3,0x1000);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x100,local_206,param_4);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,param_2,(short)param_3);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,(LPCSTR)0x0,local_206,param_4);
  fn_ptr_1000_17ce((astruct_18 *)CONCAT22(param_3,param_2),0x1000);
  return;
}



void 
send_dlg_item_msg_1040_ce12
          (param_1: u16,param_2: u16,param_3: u32,param_4: u16,param_5: *mut u16)

{
  let lVar1: i32;
  CHAR local_10a [0x100];
  let local_a: [u8;8];
  
  pass1_1008_5784(CONCAT22(param_5,local_a),param_3);
  while( true ) {
    lVar1 = pass1_1008_5b12(local_a,param_5);
    if (lVar1 == 0x0) break;
    wsprintf16((LPSTR)0x1008,local_10a,param_5);
    SendDlgItemMessage16
              ((HWND16)s_tile2_bmp_1050_1538,local_10a,param_5,0x0,
               CONCAT22(param_4,0x401));
  }
  return;
}



fn send_dlg_item_1040_ce76(param_1: u32,HWND16 param_2,param_3: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  LRESULT LVar3;
  let uVar4: u32;
  let local_106: [u8;100];
  WPARAM16 WStack6;
  let iStack4: i16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  LVar3 = SendDlgItemMessage16(param_2,0x0,0x0,0x0,0x18420409);
  WStack6 = (WPARAM16)LVar3;
  iStack4 = WStack6 >> 0xf;
  if ((WStack6 != 0xffff) || (false)) {
    SendDlgItemMessage16
              ((HWND16)s_tile2_bmp_1050_1538,local_106,param_3,WStack6,0x1842040a);
    uVar4 = pass1_1018_5206((iVar1 + 0x98),CONCAT22(param_3,local_106),param_3);
    if (uVar4 != 0x0) {
      (iVar1 + 0x9c) = (uVar4 + 0x8);
      (iVar1 + 0x9e) = 0x0;
      SetDlgItemInt16(0x1018,0x0,0x0,0x18e);
      SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x0,(iVar1 + 0x9c),0x191);
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT  send_dlg_msg_1040_cf1c(param_1: u32,param_2: u16)

{
  let in_DX: *mut u8
  let uVar1: u16;
  let unaff_DI: i16;
  let uVar2: u16;
  ulet in_AF: u8;
  LRESULT LVar3;
  let enable: bool;
  let uVar4: u16;
  char local_50c [0x402];
  let uStack266: u32;
  let local_106: [u8;100];
  let puStack6: *mut u16;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_2,in_DX,unaff_DI);
  uVar4 = (puStack6 >> 0x10);
  uVar2 = (param_1 >> 0x10);
  uVar1 = param_1;
  pass1_1010_c3c2(puStack6,uVar4,CONCAT22(param_2,local_106),
                  (uVar1 + 0x94),uVar4,in_AF,param_2);
  SendDlgItemMessage16(0x1010,local_106,param_2,0x0,0x1846000c);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1842000b);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18420405);
  uVar4 = (s_vrpal_bmp_1050_183a + 0x8);
  uStack266 = pass1_1018_526a((uVar1 + 0x98),(uVar1 + 0x94),param_2);
  send_dlg_item_msg_1040_ce12(uVar1,uVar2,uStack266,uVar4,param_2);
  LVar3 = SendDlgItemMessage16(0x1018,0x0,0x0,0x0,0x1842040c);
  if (((LVar3 >> 0x10) != 0x0 && -0x1 < LVar3) ||
     ((-0x1 < LVar3 && (LVar3 != 0x0)))) {
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1);
    enable = 0x1;
  }
  else {
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_50c,param_2);
    SendDlgItemMessage16(0x1010,local_50c,param_2,0x0,0x18420401);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1);
    enable = 0x0;
  }
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,enable);
  LVar3 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x1842000b);
  return LVar3;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn send_dlg_item_msg_1040_d20c(param_1: u32,param_2: i32,param_3: u16,param_4: u16, in_AF: u8, unaff_DI: i16, in_DX: *mut u8, in_AX: u16)
{
  let uVar2: u16;
  let puVar3: *mut u16;
  let puVar4: *mut u8;
  let uVar5: u16;
  let local_106: [u8;104];
  
  if (param_2 == 0x0) {
    enable_win_1040_d60e(param_1,param_3);
    return;
  }
  uVar2 = (param_1 >> 0x10);
  if ((param_1 + 0xa0) != 0x0) {
    pass1_1010_9210((param_1 + 0x9c));
    enable_win_1040_d60e(param_1,0x1010);
    pass1_1030_8344(_PTR_LOOP_1050_5748,
                    (_PTR_LOOP_1050_5748 >> 0x10),param_2);
    puVar4 = local_106;
    uVar5 = param_4;
    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_4,in_DX,unaff_DI);
    uVar1 = (puVar3 >> 0x10);
    pass1_1010_c3c2(puVar3,uVar1,CONCAT22(uVar5,puVar4),CONCAT22(in_DX,in_AX),
                    uVar1,in_AF,param_4);
    SendDlgItemMessage16(0x1010,local_106,param_4,0x0,0x18470401);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
win_ui_op_1040_d2ac(param_1: i16,param_2: u16,param_3: u16,param_4: u32,param_5: u16
                   ,param_6: u16,param_7: u16)

{
  LRESULT LVar1;
  
  if (param_4._2_2_ == (s_dibtext_bmp_1050_1844 + 0x4)) {
    SendDlgItemMessage16(param_6,0x0,0x0,0x0,0x18470405);
    struct_1010_9172((param_1 + 0x9c));
  }
  else {
    if ((s_dibtext_bmp_1050_1844 + 0x4) < param_4._2_2_) {
      if (param_4._2_2_ == (s_dibtext_bmp_1050_1844 + 0x5)) {
        LVar1 = SendDlgItemMessage16(param_6,0x0,0x0,0x0,0x1847040c);
        if ((LVar1 != -0x1) || ((LVar1 >> 0x10) != -0x1)) {
          SendDlgItemMessage16
                    ((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,LVar1 - 0x1,0x18470403);
          pass1_1010_91cc((param_1 + 0x9c));
        }
      }
      else {
        if (param_4._2_2_ == (s_dibtext_bmp_1050_1844 + 0x6)) {
          enable_win_1040_d6be(CONCAT22(param_2,param_1),param_6);
          pass1_1018_57d2((param_1 + 0x94),CONCAT22(param_2,param_1));
          PostMessage16(0x1018,0x0,0x0,0x1110203);
        }
        else {
          if (param_4._2_2_ != (s_dibtext_bmp_1050_1844 + 0x7)) goto LAB_1040_d3b3;
          _PTR_LOOP_1050_5a68 = (param_1 + 0x98);
          pass1_1038_af40(_PTR_LOOP_1050_5b7c,(param_1 + 0x6),0x27,param_5,
                          param_1,&PTR_LOOP_1050_1038,param_7);
        }
      }
    }
    else {
      if (param_4._2_2_ == 0xeb) {
        send_ldg_item_msg_1040_d79c(CONCAT22(param_2,param_1),param_7);
      }
      else {
        if (param_4._2_2_ != (s_vrpal_bmp_1050_183a + 0x7)) {
LAB_1040_d3b3:
          pass1_1040_b54a(param_1,param_2,param_3,param_4,param_5,param_6,param_7
                         );
          return;
        }
        msg_box_op_1040_d3d0(CONCAT22(param_2,param_1),0x0,param_5,param_7);
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn msg_box_op_1040_d3d0(param_1: u32,char *param_2,uchar *param_3,param_4: u16)
{
  char local_206 [0x102];
  char local_104 [0x102];
  
  mem_op_1000_179c(0x1000,param_3,0x1000);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x100,local_206,param_4);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,param_2,(short)param_3);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,(LPCSTR)0x0,local_206,param_4);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,param_2,(short)param_3);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,(LPCSTR)0x0,local_206,param_4);
  fn_ptr_1000_17ce((astruct_18 *)CONCAT22(param_3,param_2),0x1000);
  return;
}



fn enable_win_1040_d60e(param_1: u32,HWND16 param_2)
{
  GetDlgItem16(param_2,0x1);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x2);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(s_vrpal_bmp_1050_183a + 0x7));
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(s_dibtext_bmp_1050_1844 + 0x4));
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(s_dibtext_bmp_1050_1844 + 0x5));
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(s_dibtext_bmp_1050_1844 + 0x6));
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(s_dibtext_bmp_1050_1844 + 0x7));
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
  (param_1 + 0xa0) = 0x0;
  return;
}



fn enable_win_1040_d6be(param_1: u32,HWND16 param_2)
{
  GetDlgItem16(param_2,0x1);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x2);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(s_vrpal_bmp_1050_183a + 0x7));
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(s_dibtext_bmp_1050_1844 + 0x4));
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(s_dibtext_bmp_1050_1844 + 0x5));
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(s_dibtext_bmp_1050_1844 + 0x6));
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,(s_dibtext_bmp_1050_1844 + 0x7));
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  (param_1 + 0xa0) = 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn send_ldg_item_msg_1040_d79c(param_1: u32,param_2: u16)
{
  let uVar1: u16;
  let in_DX: *mut u8
  let uVar2: u16;
  let uVar3: u16;
  let iVar4: i16;
  let unaff_DI: i16;
  let uVar5: u16;
  HWND16 hwnd;
  ulet in_AF: u8;
  LRESULT LVar6;
  let uStack270: u32;
  let uStack266: u32;
  char local_106 [0x100];
  let puStack6: *mut u16;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_2,in_DX,unaff_DI);
  uVar2 = (puStack6 >> 0x10);
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  pass1_1010_c3c2(puStack6,uVar2,CONCAT22(param_2,local_106),
                  (iVar4 + 0x98),uVar2,in_AF,param_2);
  SendDlgItemMessage16(0x1010,local_106,param_2,0x0,0x1846000c);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1847000b);
  LVar6 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18470405);
  uVar3 = (LVar6 >> 0x10);
  uVar1 = LVar6;
  hwnd = 0x1010;
  pass1_1010_9044((iVar4 + 0x9c));
  uStack266 = CONCAT22(uVar3,uVar1);
  for (uStack270 = 0x0; uStack270 < uStack266; uStack270 += 0x1) {
    hwnd = 0x1010;
    pass1_1010_9130((iVar4 + 0x9c),CONCAT22(param_2,local_106),
                    local_106,uVar3,param_2,in_AF);
    if (local_106[0] != '\0') {
      hwnd = s_tile2_bmp_1050_1538;
      LVar6 = SendDlgItemMessage16(0x1010,local_106,param_2,0x0,0x18470401);
      uVar3 = (LVar6 >> 0x10);
    }
  }
  SendDlgItemMessage16(hwnd,0x0,0x0,0x1,0x1847000b);
  return;
}
