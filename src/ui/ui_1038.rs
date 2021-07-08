
LRESULT  send_dlg_item_msg_1038_7eac(param_1: u32)

{
  let in_DX: *mut u8
  let unaff_DI: i16;
  let unaff_SS: u16;
  let puVar1: *mut u16;
  let mut pcVar2: String; 
  LRESULT LVar3;
  
  puVar1 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x30,unaff_SS,in_DX,unaff_DI);
  pcVar2 = pass1_1010_375e(puVar1);
  pass1_1008_b1a6((param_1 + 0x94),pcVar2);
  SendDlgItemMessage16(0x1008,0x0,0x0,0x0,0x1854000b);
  LVar3 = SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18540409);
  if ((LVar3 != -0x1) || (false)) {
    SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,(WPARAM16)LVar3,0x18540403)
    ;
    SendDlgItemMessage16
              (ctx.s_tile2_bmp_1050_1538,pcVar2,(pcVar2 >> 0x10)
               ,0x0,0x18540401);
    SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0xffff,0x18540407);
    SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18550405);
    SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18560405);
    enable_win_1038_806a(param_1,s_tile2_bmp_1050_1538);
  }
  LVar3 = SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x1854000b);
  return LVar3;
}



pub fn send_dlg_item_msg_1038_7fae(param_1: u32)
{
  let in_AX: u16;
  let in_DX: u16;
  let iVar1: i16;
  let uVar2: u16;
  let unaff_SS: u16;
  LRESULT LVar3;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  pass1_1008_b146((iVar1 + 0x94),in_AX,in_DX);
  SendDlgItemMessage16(0x1008,0x0,0x0,0xffff,0x18550407);
  LVar3 = SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0xffff,0x18560407);
  pass1_1008_b61a((iVar1 + 0x94),0x0,LVar3,(LVar3 >> 0x10),
                  unaff_SS);
  pass1_1008_b63a((iVar1 + 0x94),0x0);
  return;
}


pub fn enable_win_1038_806a(param_1: u32,param_2: HWND16)
{
  let BVar1: bool;
  let in_DX: u16;
  let iVar2: i16;
  let uVar3: u16;
  let hwnd_dlg: HWND16;
  let uVar4: u32;
  let uVar5: u32;
  let uVar6: u32;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  GetDlgItem16(param_2,0x1);
  EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x0);
  GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x1858);
  EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x0);
  GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x1859);
  BVar1 = EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x0);
  uVar4 = pass1_1008_b820((iVar2 + 0x94),BVar1,in_DX);
  if (uVar4 != 0x0) {
    uVar4 = pass1_1008_b340((iVar2 + 0x94));
    uVar5 = pass1_1008_b366((iVar2 + 0x94));
    hwnd_dlg = 0x1008;
    uVar6 = pass1_1008_b47a((iVar2 + 0x94));
    if (((uVar4 != 0x0) && (uVar5 != 0x0)) && (uVar6 != 0x0)) {
      GetDlgItem16(0x1008,0x1);
      EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x1);
      GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x1858);
      hwnd_dlg = ctx.s_tile2_bmp_1050_1538;
      EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x1);
    }
    if (uVar4 != 0x0) {
      GetDlgItem16(hwnd_dlg,0x1859);
      EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x1);
    }
  }
  return;
}



u16 
send_dlg_item_msg_1038_8164
          (param_1: u16,param_2: u16,uchar *param_3,param_4: u16,param_5: HWND16)

{
  LRESULT LVar1;
  
  *param_3 = '\0';
  LVar1 = SendDlgItemMessage16(param_5,0x0,0x0,0x0,CONCAT22(param_4,0x409));
  if ((LVar1 != -0x1) || (false)) {
    LVar1 = SendDlgItemMessage16
                      (ctx.s_tile2_bmp_1050_1538,param_3,
                       (param_3 >> 0x10),(WPARAM16)LVar1,
                       CONCAT22(param_4,0x40a));
    if ((LVar1 != -0x1) || (false)) {
      return 0x1;
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn msg_box_op_1038_81be(param_1: u32,char *param_2,uchar *param_3,param_4: u16)
{
  char local_206 [0x102];
  char local_104 [0x102];
  
  mem_op_1000_179c(0x1000,param_3,0x1000);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_206,param_4);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,param_2,(short)param_3);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,0x0,local_206,param_4);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,param_2,(short)param_3);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,0x0,local_206,param_4);
  fn_ptr_1000_17ce(CONCAT22(param_3,param_2),0x1000);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn set_win_text_1038_8358(param_1: u32,param_2: HWND16,param_3: u16)
{
  let mut lp_string: String; 
  let uVar2: u16;
  let in_DX: u16;
  let uVar4: u16;
  let uVar3: u16;
  let hwnd: HWND16;
  char local_30a [0x102];
  CHAR local_208 [0x100];
  let local_108: [u8;100];
  let uStack8: u32;
  let HStack4: HWND16;
  let uVar1: u32;
  
  uVar3 = (param_1 >> 0x10);
  uVar4 = param_1;
  HStack4 = GetDlgItem16(param_2,0x1857);
  uStack8 = pass1_1008_b820((uVar4 + 0x94),HStack4,in_DX);
  if (uStack8 == 0x0) {
    hwnd = 0x1010;
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (ctx.PTR__LOOP_1050_14cc >> 0x10),0x100,local_30a,param_3);
    lp_string = local_30a;
  }
  else {
    uVar2 = send_dlg_item_msg_1038_8164
                      (uVar4,uVar3,CONCAT22(param_3,local_108),0x1854,0x1008);
    if (uVar2 == 0x0) {
      hwnd = 0x1010;
      load_string_1010_84e0
                (0x1010,_PTR_LOOP_1050_14cc,
                 (ctx.PTR__LOOP_1050_14cc >> 0x10),0x100,local_208,param_3);
    }
    else {
      hwnd = 0x1008;
      load_string_1008_b65a
                (*(ULONG *)(uVar4 + 0x94),local_208,CONCAT22(local_108,param_3),param_3);
    }
    lp_string = local_208;
  }
  SetWindowText16(hwnd,(SEGPTR)lp_string);
  return;
}



void 
send_dlg_item_msg_1038_8400
          (param_1: u16,param_2: u16,param_3: u32,param_4: u16,param_5: u16)

{
  let uVar1: u32;
  let lVar2: i32;
  let local_a: [u8;8];
  
  pass1_1008_5784(CONCAT22(param_5,local_a),param_3);
  while( true ) {
    lVar2 = pass1_1008_5b12(local_a,param_5);
    if (lVar2 == 0x0) break;
    uVar1 = (lVar2 + 0x4);
    SendDlgItemMessage16
              (0x1008,uVar1,(uVar1 >> 0x10),0x0,
               CONCAT22(param_4,0x401));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn send_dlg_item_msg_1038_844a(param_1: u32,param_2: HWND16,param_3: u16) -> LRESULT
{
  let BVar1: bool;
  let uVar2: u16;
  let uVar3: u16;
  LRESULT LVar4;
  char local_108 [0x102];
  let uStack6: u32;
  
  uVar3 = (param_1 >> 0x10);
  SendDlgItemMessage16(param_2,0x0,0x0,0x0,0x1854000b);
  SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1855000b);
  SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1856000b);
  SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18540405);
  SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18550405);
  LVar4 = SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18560405);
  uStack6 = pass1_1008_b820((param_1 + 0x94),LVar4,
                            (LVar4 >> 0x10));
  if (uStack6 == 0x0) {
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (ctx.PTR__LOOP_1050_14cc >> 0x10),0x100,local_108,param_3);
    SendDlgItemMessage16(0x1010,local_108,param_3,0x0,0x18540401);
    SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x1854000b);
    SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x1855000b);
    LVar4 = SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x1856000b);
    uVar2 = (LVar4 >> 0x10);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x1857);
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (ctx.PTR__LOOP_1050_14cc >> 0x10),0x100,local_108,param_3);
    BVar1 = SetWindowText16(0x1010,(SEGPTR)local_108);
    return CONCAT22(uVar2,BVar1);
  }
  send_dlg_item_msg_1038_8400(param_1,uVar3,uStack6,0x1854,param_3);
  set_win_text_1038_8358(param_1,0x1008,param_3);
  SendDlgItemMessage16(0x1008,0x0,0x0,0x1,0x1854000b);
  SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x1855000b);
  LVar4 = SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x1856000b);
  return LVar4;
}



// WARNING: Could not reconcile some variable overlaps

pub fn send_dlg_item_msg_1038_8618(param_1: u32,param_2: u16) -> u16

{
  let in_AX: i16;
  let uVar1: u16;
  let puVar2: *mut u8;
  let in_DX: u16;
  let puVar3: *mut u8
  let msg: u16;
  let uVar4: u16;
  let uVar5: u16;
  let hwnd: HWND16;
  LRESULT LVar6;
  let uVar7: u32;
  let uVar8: u32;
  let local_106: [u8;100];
  let uStack6: u32;
  
  uVar5 = (param_1 >> 0x10);
  uVar4 = param_1;
  uStack6 = pass1_1008_b820((uVar4 + 0x94),in_AX,in_DX);
  uVar1 = uStack6;
  if (uStack6 != 0x0) {
    uVar1 = send_dlg_item_msg_1038_8164
                      (uVar4,uVar5,CONCAT22(param_2,local_106),0x1854,0x1008);
    if (uVar1 != 0x0) {
      SendDlgItemMessage16(0x1008,0x0,0x0,0x0,0x1855000b);
      SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1856000b);
      SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18550405);
      LVar6 = SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18560405);
      puVar3 = (LVar6 >> 0x10);
      puVar2 = local_106;
      pass1_1008_b4a0((uVar4 + 0x94),CONCAT22(param_2,puVar2),puVar2,
                      puVar3,param_2);
      pass1_1008_b200((uVar4 + 0x94),param_2);
      uVar8 = CONCAT22(puVar3 | puVar2,puVar2);
      if ((puVar3 | puVar2) != 0x0) {
        send_dlg_item_msg_1038_8400(uVar4,uVar5,CONCAT22(puVar3,puVar2),0x1855,param_2);
        uVar7 = pass1_1008_b366((uVar4 + 0x94));
        msg = (uVar7 >> 0x10);
        uVar8 = uVar7 & 0xffff | (msg | uVar7) << 0x10;
        if (uVar7 != 0x0) {
          uVar8 = SendDlgItemMessage16(0x1008,uVar7,msg,0xffff,0x1855040d);
        }
      }
      hwnd = 0x1008;
      uVar8 = pass1_1008_b38c((uVar4 + 0x94),uVar8,
                              (uVar8 >> 0x10));
      if (uVar8 != 0x0) {
        send_dlg_item_msg_1038_8400(uVar4,uVar5,uVar8,0x1856,param_2);
        hwnd = 0x1008;
        uVar8 = pass1_1008_b47a((uVar4 + 0x94));
        if (uVar8 != 0x0) {
          hwnd = ctx.s_tile2_bmp_1050_1538;
          SendDlgItemMessage16
                    (0x1008,uVar8,(uVar8 >> 0x10),0xffff,0x1856040d);
        }
      }
      SendDlgItemMessage16(hwnd,0x0,0x0,0x1,0x1855000b);
      LVar6 = SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x1856000b);
      uVar1 = LVar6;
    }
  }
  return uVar1;
}



pub fn send_dlg_item_msg_1038_87b2(param_1: u32,param_2: u16,param_3: u16) -> u16

{
  let uVar1: u16;
  let uVar2: u16;
  let in_DX: u16;
  let uVar3: u32;
  LRESULT LVar4;
  let uVar5: u16;
  let local_102: [u8;100];
  
  uVar5 = param_1;
  uVar1 = (param_1 >> 0x10);
  uVar2 = send_dlg_item_msg_1038_8164
                    (uVar5,uVar1,CONCAT22(param_3,local_102),0x1855,param_2);
  if (uVar2 != 0x0) {
    pass1_1008_b61a((uVar5 + 0x94),CONCAT22(param_3,local_102),local_102
                    ,in_DX,param_3);
    uVar3 = (uVar5 + 0x94);
    uVar3 = load_string_1008_b1f0(uVar3,(uVar3 >> 0x10));
    LVar4 = SendDlgItemMessage16
                      (0x1008,uVar3,(uVar3 >> 0x10),0xffff,
                       0x1856040d);
    uVar2 = LVar4;
  }
  return uVar2;
}


pub fn msg_box_ui_op_1038_8a3a(param_1: u32,char *param_2,uchar *param_3,param_4: u16)
{
  char local_20a [0x102];
  let mut pcStack264: String; 
  let puStack262: *mut u8
  char local_104 [0x102];
  
  mem_op_1000_179c(0x1000,param_3,0x1000);
  pcStack264 = param_2;
  puStack262 = param_3;
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,param_2,(short)param_3);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(puStack262,pcStack264),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(puStack262,pcStack264),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x101,local_20a,param_4);
  MessageBox16(0x1010,0x0,local_20a,param_4);
  fn_ptr_1000_17ce(CONCAT22(puStack262,pcStack264),0x1000);
  return;
}



pub fn unk_win_ui_op_1038_8afe(param_1: &mut Struct50,param_2: HWND16,bool param_3)
{
  let uVar1: u32;
  let dlg_item: u16;
  let in_DX: u16;
  let iVar4: &mut Struct50;
  let uVar4: &mut Struct50;
  let local_4: bool;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  dlg_item = GetDlgItemInt16(param_2,0x0,&local_4,param_3);
  pass1_1030_6c1a(iVar4.field_0x94,dlg_item);
  uVar1 = iVar4.field_0x94;
  pass1_1038_387e((uVar1 + 0x2e),dlg_item,iVar4.field_0x9c,
                  iVar4.field_0x94,in_DX);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn send_dlg_item_msg_1038_8b58(param_1: u32,param_2: u16)
{
  let uVar1: u32;
  let uVar2: u32;
  let in_DX: *mut u8
  let uVar3: u16;
  let uVar4: u16;
  let iVar5: i16;
  let unaff_DI: i16;
  let uVar6: u16;
  ulet in_AF: u8;
  LRESULT LVar7;
  let local_106: [u8;100];
  let puStack6: *mut u16;
  
  puStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x3,param_2,in_DX,unaff_DI);
  uVar3 = (puStack6 >> 0x10);
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  pass1_1010_c3c2(puStack6,uVar3,CONCAT22(param_2,local_106),
                  (iVar5 + 0x94),uVar3,in_AF,param_2);
  LVar7 = SendDlgItemMessage16(0x1010,local_106,param_2,0x0,0x1846000c);
  uVar4 = (LVar7 >> 0x10);
  uVar1 = (iVar5 + 0x94);
  (iVar5 + 0x9c) = (uVar1 + 0x32);
  (iVar5 + 0x9a) = (iVar5 + 0x9c);
  SetDlgItemInt16(ctx.s_tile2_bmp_1050_1538,0x0,(iVar5 + 0x9c),
                  (s_dibtext_bmp_1050_1844 + 0x9));
  uVar1 = (iVar5 + 0x94);
  uVar2 = (uVar1 + 0x2e);
  pass1_1038_3aa6(uVar2,uVar2,uVar4);
  (iVar5 + 0x98) = uVar2;
  SetDlgItemInt16(ctx.s_tile2_bmp_1050_1538,0x0,uVar2,
                  (s_dibtext_bmp_1050_1844 + 0xb));
  return;
}


pub fn send_dlg_item_msg_1038_8d22(param_1: u32,param_2: HWND16,param_3: u16)
{
  let unaff_DI: u16;
  let in_AF: u8;
  LRESULT LVar1;
  let local_106: [u8;100];
  WPARAM16 WStack6;
  let iStack4: i16;
  
  LVar1 = SendDlgItemMessage16(param_2,0x0,0x0,0x0,0x185b0409);
  WStack6 = (WPARAM16)LVar1;
  iStack4 = WStack6 >> 0xf;
  if ((WStack6 != 0xffff) || (false)) {
    SendDlgItemMessage16
              (ctx.s_tile2_bmp_1050_1538,local_106,param_3,WStack6,0x185b040a);
    pass1_1008_c79a((param_1 + 0x94),CONCAT22(param_3,local_106),unaff_DI,
                    param_3,in_AF);
  }
  return;
}


pub fn msg_box_op_1038_8dda(param_1: u32,char *param_2,uchar *param_3,param_4: u16)
{
  char local_206 [0x102];
  char local_104 [0x102];
  
  mem_op_1000_179c(0x1000,param_3,0x1000);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x100,local_206,param_4);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,param_2,(short)param_3);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,0x0,local_206,param_4);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,param_2,(short)param_3);
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,0x0,local_206,param_4);
  fn_ptr_1000_17ce(CONCAT22(param_3,param_2),0x1000);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn send_dlg_item_msg_1038_8f74(param_1: u32,param_2: HWND16,param_3: *mut u16) -> LRESULT
{
  let iVar1: i16;
  let uVar2: u16;
  let lVar3: i32;
  LRESULT LVar4;
  let enable: bool;
  CHAR local_50c [0x100];
  let local_40c: [u8;8];
  let local_404: u32;
  
  uVar2 = (param_1 >> 0x10);
  SendDlgItemMessage16(param_2,0x0,0x0,0x0,0x185b000b);
  SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x185b0405);
  iVar1 = pass1_1008_c83a((param_1 + 0x94));
  if (iVar1 == 0x0) {
    local_404 = pass1_1008_c85e((param_1 + 0x94),param_3);
    pass1_1008_5784(CONCAT22(param_3,local_40c),local_404);
    while( true ) {
      lVar3 = pass1_1008_5b12(local_40c,param_3);
      if (lVar3 == 0x0) break;
      wsprintf16(0x1008,local_50c,param_3);
      SendDlgItemMessage16
                (ctx.s_tile2_bmp_1050_1538,local_50c,param_3,0x0,
                 0x185b0401);
    }
    GetDlgItem16(0x1008,0x1);
    enable = 0x1;
  }
  else {
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,&local_404,
               (short)param_3);
    SendDlgItemMessage16(0x1010,&local_404,param_3,0x0,0x185b0401);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x1);
    enable = 0x0;
  }
  EnableWindow16(ctx.s_tile2_bmp_1050_1538,enable);
  LVar4 = SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x185b000b);
  return LVar4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_dlg_op_1038_9294(param_1: &mut Struct1,param_2: u16)
{
  let UVar1: u16;
  let uVar2: u16;
  let in_DX: u16;
  let uVar3: u16;
  WNDCLASS16 *unaff_SS;
  let local_6: bool;
  let local_4: bool;
  
  unk_win_ui_op_1040_b230(param_1,&ctx.PTR_LOOP_1050_1040,unaff_SS);
  uVar3 = (param_1 >> 0x10);
  UVar1 = GetDlgItemInt16(&ctx.PTR_LOOP_1050_1040,0x1,&local_4,unaff_SS);
  (param_1 + 0x94) = UVar1;
  uVar2 = GetDlgItemInt16(ctx.s_tile2_bmp_1050_1538,0x1,&local_6,unaff_SS);
  (param_1 + 0x96) = uVar2;
  win_ui_dlg_op_1038_98b4
            ((param_1 & 0xffff | uVar3 << 0x10),
             ctx.s_tile2_bmp_1050_1538,unaff_SS);
  win_1008_5c7c(ctx.PTR__LOOP_1050_02a0,0x950001,unaff_SS,uVar2,in_DX);
  return;
}


bool 
send_dlg_item_int_1038_94da
          (param_1: i16,param_2: u16,param_3: u16,param_4: u16,param_5: i16,
          param_6: HWND16,bool param_7)

{
  let pUVar1: *mut u16;
  let iVar2: i16;
  let lVar3: i32;
  let local_c: bool;
  let uStack10: u16;
  let iStack8: i16;
  let UStack: u16;
  let iStack4: i16;
  
  iStack4 = 0x1;
  iStack8 = pass1_1038_993a(param_1,param_2,param_3);
  if ((-0x1 < iStack8) &&
     (UStack6 = GetDlgItemInt16(param_6,0x1,&local_c,param_7), local_c != 0x0)) {
    if (param_5 == 0x0) {
      UStack6 += 0x1;
    }
    else {
      iStack4 = -0x1;
      UStack6 -= 0x1;
    }
    uStack10 = (UStack6 <= (iStack8 * 0xe + 0x5a7a));
    pUVar1 = (iStack8 * 0xe + 0x5a78);
    if (*pUVar1 != UStack6 && UStack6 <= *pUVar1) {
      uStack10 = 0x0;
    }
    iVar2 = iStack8 * 0xe;
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538,*(INT16 *)(iVar2 + 0x5a72));
    SetFocus16(ctx.s_tile2_bmp_1050_1538);
    if ((uStack10 != 0x0) &&
       (lVar3 = unk_win_ui_op_1038_9820
                          (CONCAT22(param_2,param_1),0x1,iStack4,
                           ctx.s_tile2_bmp_1050_1538,param_7), lVar3 != 0x0)) {
      SetDlgItemInt16(ctx.s_tile2_bmp_1050_1538,0x1,UStack6,
                      *(bool *)(iVar2 + 0x5a72));
      SetDlgItemInt16(ctx.s_tile2_bmp_1050_1538,0x1,(param_1 + 0x94),0xfa9)
      ;
      SetDlgItemInt16(ctx.s_tile2_bmp_1050_1538,0x1,(param_1 + 0x96),0xfa8)
      ;
    }
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_msg_op_1038_95fc(param_1: u32,param_2: u16)
{
  let ppcVar1: u32;
  let uVar2: u16;
  let UVar3: u16;
  let UVar4: u16;
  let in_DX: *mut u8
  let extraout_DX: *mut u8
  let puVar5: *mut u8
  let extraout_DX_00: *mut u8
  let iVar6: i16;
  let unaff_DI: i16;
  let uVar7: u16;
  let hwnd: HWND16;
  let HVar8: HWND16;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  let uVar12: u16;
  let puStack30: *mut u16;
  let puStack24: *mut u16;
  let iStack20: i16;
  let local_10: bool;
  let puStack14: u32;
  let puStack10: *mut u16;
  let puStack6: *mut u16;
  
  puStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x8,param_2,in_DX,unaff_DI);
  puStack10 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x9,param_2,
                              (puStack6 >> 0x10),unaff_DI);
  puVar5 = (puStack10 >> 0x10);
  uVar2 = puStack10;
  hwnd = 0x1000;
  mem_op_1000_179c(0xc,puVar5,0x1000);
  if ((puVar5 | uVar2) == 0x0) {
    uVar2 = 0x0;
    puVar5 = 0x0;
  }
  else {
    hwnd = 0x1008;
    set_struct_1008_574a(CONCAT22(puVar5,uVar2));
    puVar5 = extraout_DX;
  }
  puStack14 = CONCAT22(puVar5,uVar2);
  for (iStack20 = 0x0; iStack20 < 0xf; iStack20 += 0x1) {
    uVar12 = (param_1 + 0x6);
    HVar8 = ctx.s_tile2_bmp_1050_1538;
    UVar3 = GetDlgItemInt16(hwnd,0x1,&local_10,param_2);
    if (UVar3 != 0x0) {
      if ((iStack20 * 0xe + 0x5a7c) < 0x83) {
        uVar11 = 0x8;
        HVar8 = 0x1000;
        UVar4 = UVar3;
        mem_op_1000_179c(0x8,puVar5,0x1000);
        puStack24 = CONCAT22(puVar5,UVar4);
        if ((puVar5 | UVar4) == 0x0) {
          puStack30 = 0x0;
        }
        else {
          *puStack24 = 0x389a;
          (UVar4 + 0x2) = 0x1008;
          *puStack24 = 0xa1c4;
          (UVar4 + 0x2) = 0x1010;
          puStack30 = puStack24;
        }
        uVar7 = (puStack30 >> 0x10);
        iVar6 = puStack30;
        (iVar6 + 0x6) = UVar3;
        (iVar6 + 0x4) = (iStack20 * 0xe + 0x5a7c);
        ppcVar1 = (*puStack14 + 0x4);
        (**ppcVar1)(0x1000,puStack14,(puStack14 >> 0x10),iVar6,uVar7,
                    uVar11,uVar12);
        puVar5 = extraout_DX_00;
      }
      else {
        if ((iStack20 * 0xe + 0x5a7c) == 0x89) {
          uVar10 = (iStack20 * 0xe + 0x5a7c);
          uVar9 = UVar3;
        }
        else {
          uVar10 = (iStack20 * 0xe + 0x5a7c);
          uVar9 = 0x0;
        }
        HVar8 = 0x1010;
        pass1_1010_6566(puStack10,uVar9,UVar3,uVar10,param_2);
      }
    }
    hwnd = HVar8;
  }
  (puStack6 + 0xa) = puStack14;
  PostMessage16(hwnd,0x0,0x0,0x11100ed);
  return;
}



void 
win_ui_op_1038_977a(param_1: i16,param_2: u16,param_3: i16,uchar *param_4,param_5: HWND16,
                   param_6: u16)

{
  let ppcVar1: u32;
  let uVar2: u16;
  let iVar3: i16;
  let puVar4: *mut u8
  let uVar5: u16;
  let uVar6: u16;
  let local_10: [u8;4];
  let puStack12: u32;
  let iStack8: i16;
  let UStack: u16;
  let local_4: bool;
  
  iStack8 = 0x0;
  uVar6 = (param_1 + 0x6);
  uVar2 = GetDlgItemInt16(param_5,0x1,&local_4,param_6);
  UStack6 = uVar2;
  if (uVar2 != 0x0) {
    uVar5 = 0xb4;
    mem_op_1000_179c(0xb4,param_4,0x1000);
    puVar4 = (param_4 | uVar2);
    if (puVar4 == 0x0) {
      iVar3 = 0x0;
      puVar4 = 0x0;
    }
    else {
      iVar3 = string_1040_8520(CONCAT22(param_4,uVar2),
                               (param_1 + 0x6),0x41,0x2,0x5db,0x5da,puVar4,
                               param_6);
    }
    puStack12 = CONCAT22(puVar4,iVar3);
    pass1_1008_941a(CONCAT22(param_6,local_10),0x1,0xc3);
    ppcVar1 = (*puStack12 + 0x6c);
    iStack8 = (**ppcVar1)(0x1008,puStack12,(puStack12 >> 0x10),local_10,
                          param_6,uVar5,uVar6,uVar2);
  }
  if ((iStack8 == 0x1) || (UStack6 == 0x0)) {
    destroy_window_1040_b726
              ((ULONG *)CONCAT22(param_2,param_1),param_3,&ctx.PTR_LOOP_1050_1040);
  }
  return;
}



long 
unk_win_ui_op_1038_9820
          (param_1: &mut Struct51,param_2: i16,param_3: i16,param_4: HWND16,bool param_5)

{
  let piVar1: *mut i16;
  let lVar2: i32;
  let UVar3: u16;
  let iVar4: i16;
  let iVar5: i16;
  let uVar6: u16;
  let iVar7: &mut Struct51;
  let uVar7: &mut Struct51;
  let local_6: bool;
  let local_4: bool;
  
  uVar7 = (param_1 >> 0x10);
  iVar7 = param_1;
  UVar3 = GetDlgItemInt16(param_4,0x1,&local_4,param_5);
  iVar4 = UVar3 * param_2 * param_3;
  UVar3 = GetDlgItemInt16(ctx.s_tile2_bmp_1050_1538,0x1,&local_6,param_5);
  lVar2 = (UVar3 * param_2) * param_3;
  uVar6 = (lVar2 >> 0x10);
  iVar5 = lVar2;
  if ((iVar4 - iVar7.field_0x94 < 0x1) && (-0x1 < iVar7.field_0x96 - iVar5)) {
    piVar1 = &iVar7.field_0x94;
    *piVar1 = *piVar1 - iVar4;
    piVar1 = &iVar7.field_0x96;
    *piVar1 = *piVar1 - iVar5;
    return CONCAT22(uVar6,0x1);
  }
  return (uVar6 << 0x10);
}



pub fn win_ui_dlg_op_1038_98b4(param_1: &mut Struct51,param_2: HWND16,bool param_3)
{
  let UVar1: u16;
  let uVar2: u16;
  let iVar3: i16;
  let iStack8: i16;
  let local_4: bool;
  
  local_4 = 0x0;
  for (iStack8 = 0x0; iVar3 = param_1, uVar2 = (param_1 >> 0x10),
      iStack8 < 0xf; iStack8 += 0x1) {
    iVar3 = (iVar3 + 0x6);
    UVar1 = GetDlgItemInt16(param_2,0x1,&local_4,param_3);
    unk_win_ui_op_1038_9820(param_1,UVar1,iVar3,s_tile2_bmp_1050_1538,param_3);
    param_2 = ctx.s_tile2_bmp_1050_1538;
  }
  SetDlgItemInt16(param_2,0x1,(iVar3 + 0x94),0xfa9);
  SetDlgItemInt16(ctx.s_tile2_bmp_1050_1538,0x1,(iVar3 + 0x96),0xfa8);
  return;
}


void 
enable_win_1038_9a66
          (param_1: u16,param_2: u16,u16 in_b_enable_3,param_4: u32,
          in_hwnd_5: HWND16)

{
  let in_DX: *mut u8
  let unaff_SS: u16;
  
  if (param_4._2_2_ == 0xf8) {
    GetDlgItem16(in_hwnd_5,0x17d9);
    in_b_enable_3 = 0x1;
  }
  else {
    if (param_4._2_2_ != 0x17d9) {
      pass1_1040_b54a(param_1,param_2,in_b_enable_3,param_4,in_DX,
                      &ctx.PTR_LOOP_1050_1040,unaff_SS);
      return;
    }
    SetWindowPos16(in_hwnd_5,0x6,0x1a0,0x12c,0x0,0x0,0x0);
  }
  EnableWindow16(ctx.s_tile2_bmp_1050_1538,in_b_enable_3);
  return;
}


pub fn unk_win_ui_op_1038_9bc8(param_1: &mut Struct1)
{
  let piVar1: *mut i16;
  let ppcVar2: u32;
  let iVar3: i16;
  let IVar4: i16;
  HDC16 hdc;
  let iVar5: i16;
  let HVar6: HWND16;
  let in_DX: *mut u8
  let puVar7: *mut u8
  let iVar8: i16;
  let unaff_DI: i16;
  let uVar9: u16;
  let unaff_SS: u16;
  let puVar10: *mut u16;
  let uVar12: u16;
  let uVar11: u32;
  let puVar13: *mut u16;
  let iStack36: i16;
  RECT16 local_16;
  let iStack16: i16;
  let iStack14: i16;
  let iStack12: i16;
  let uStack10: u32;
  let local_6: i16;
  let local_4: i16;
  
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  if (ctx.PTR_LOOP_1050_5ef8 == (&DAT_1050_0004 + 0x1)) {
    ctx.PTR_LOOP_1050_5ef8 = 0x0;
  }
  puVar13 = CONCAT22(unaff_SS,&local_4);
  puVar10 = CONCAT22(unaff_SS,&local_6);
  uStack10 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x48,unaff_SS,in_DX,unaff_DI);
  pass1_1008_3e94((uStack10 & 0xffff0000 | (uStack10 + 0xe)),
                  puVar10,puVar13);
  IVar4 = GetSystemMetrics16(0x1008);
  puVar7 = ((IVar4 * PTR_LOOP_1050_5ef8) >> 0x10);
  iStack12 = (IVar4 * PTR_LOOP_1050_5ef8) + 0xa;
  ctx.PTR_LOOP_1050_5ef8 = ctx.PTR_LOOP_1050_5ef8 + 0x1;
  iStack14 = iStack12 + local_6;
  iStack12 += local_4;
  uVar9 = (param_1 >> 0x10);
  iVar8 = param_1;
  GetWindowRect16(ctx.s_tile2_bmp_1050_1538,&local_16);
  uVar12 = 0x0;
  hdc = GetDC16(ctx.s_tile2_bmp_1050_1538);
  IVar4 = GetDeviceCaps16(ctx.s_tile2_bmp_1050_1538,0xa);
  ReleaseDC16(ctx.s_tile2_bmp_1050_1538,hdc);
  if (IVar4 < iStack16) {
    iStack14 = (local_16.y - (iStack16 - IVar4)) + 0x1;
  }
  uVar11 = CONCAT22(uVar12,(iVar8 + 0x6));
  SetWindowPos16(ctx.s_tile2_bmp_1050_1538,0x1,0x0,0x0,iStack14,iStack12,0x0);
  puVar10 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x3,unaff_SS,puVar7,unaff_DI);
  iVar5 = puVar10 + 0xa4;
  uVar12 = (puVar10 >> 0x10);
  iStack36 = 0x0;
  HVar6 = 0x1010;
  while (iVar3 = iStack36 * 0x2, (iVar3 + iVar5) != 0x0) {
    HVar6 = GetDlgItem16(HVar6,*(INT16 *)(iVar3 + iVar5));
    *(HWND16 *)(iVar8 + iVar3 + 0x94) = HVar6;
    iStack36 += 0x1;
    piVar1 = (iVar8 + 0x128);
    *piVar1 = *piVar1 + 0x1;
    HVar6 = ctx.s_tile2_bmp_1050_1538;
  }
  ppcVar2 = (param_1.field_0x0 + 0x6c);
  (**ppcVar2)(HVar6,iVar8,uVar9,uVar11);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
enable_window_1038_9cec
          (param_1: i16,param_2: u16,param_3: u16,param_4: u16,param_5: i16,
          param_6: HWND16)

{
  let piVar1: *mut i16;
  let iVar2: i16;
  let uVar3: u16;
  let iVar4: i16;
  let HVar5: HWND16;
  let in_DX: *mut u8
  let unaff_DI: i16;
  let unaff_SS: u16;
  let puVar6: *mut u16;
  let iStack12: i16;
  
  if (param_5 == 0xeb) {
    pass1_1040_b54a(param_1,param_2,param_3,CONCAT22(0xeb,param_4),in_DX,
                    &ctx.PTR_LOOP_1050_1040,unaff_SS);
    puVar6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x3,unaff_SS,in_DX,unaff_DI);
    iVar4 = puVar6 + 0xa4;
    uVar3 = (puVar6 >> 0x10);
    iStack12 = 0x0;
    HVar5 = 0x1010;
    while (iVar2 = iStack12 * 0x2, (iVar2 + iVar4) != 0x0) {
      HVar5 = GetDlgItem16(HVar5,*(INT16 *)(iVar2 + iVar4));
      *(HWND16 *)(param_1 + iVar2 + 0x94) = HVar5;
      iStack12 += 0x1;
      piVar1 = (param_1 + 0x128);
      *piVar1 = *piVar1 + 0x1;
      HVar5 = ctx.s_tile2_bmp_1050_1538;
    }
  }
  else {
    if (param_5 == 0xf8) {
      GetDlgItem16(param_6,0x17d8);
      HVar5 = 0x1;
    }
    else {
      if (param_5 != 0x17d8) {
        pass1_1040_b54a(param_1,param_2,param_3,CONCAT22(param_5,param_4),in_DX,
                        &ctx.PTR_LOOP_1050_1040,unaff_SS);
        return;
      }
      SetWindowPos16(param_6,0x6,0xed,0x237,0x0,0x0,0x0);
      HVar5 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x17d8);
    }
    EnableWindow16(ctx.s_tile2_bmp_1050_1538,HVar5);
  }
  return;
}


pub fn show_win_1038_9fd0(param_1: &mut Struct1)
{
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(&ctx.PTR_LOOP_1050_1040,0x5);
  return;
}


pub fn unk_win_ui_op_1038_a18c(param_1: &mut Struct1,param_2: u16)
{
  let ppcVar1: u32;
  let IVar2: i16;
  let in_DX: *mut u8
  let unaff_DI: i16;
  let uVar3: u16;
  let piVar4: *mut i16;
  let puVar5: *mut u8;
  let uVar6: u16;
  let uVar7: u16;
  RECT16 local_2c;
  let iStack40: i16;
  let puStack36: *mut u16;
  let iStack32: i16;
  let uStack30: u16;
  let local_1c: i16;
  let local_1a: [u8;2];
  let uStack24: u32;
  let paStack20: &mut Struct76;
  let local_10: i16;
  let local_e: bool;
  let local_c: [u8;6];
  let puStack6: *mut u16;
  
  puStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x27,param_2,in_DX,unaff_DI);
  pass1_1008_3e38(CONCAT22(param_2,local_c));
  pass1_1008_3f62(CONCAT22(param_2,local_c),
                  (puStack6 & 0xffff0000 | (puStack6 + 0x52))
                 );
  pass1_1008_3e94(CONCAT22(param_2,local_c),
                  CONCAT22(param_2,&local_10),
                  CONCAT22(param_2,&local_e));
  paStack20 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc,0x1c0,param_2);
  uStack24 = pass1_1008_4772(paStack20);
  puVar5 = local_1a;
  piVar4 = &local_1c;
  uVar7 = param_2;
  puStack36 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x48,param_2,(uStack24 >> 0x10)
                              ,unaff_DI);
  pass1_1008_3e94(
                  (puStack36 & 0xffff0000 | (puStack36 + 0xe)),
                  CONCAT22(param_2,piVar4),CONCAT22(uVar7,puVar5));
  uVar3 = (puStack36 >> 0x10);
  uStack30 = (puStack36 + 0xa);
  iStack32 = (puStack36 + 0xc);
  local_10 += (iStack32 * 0xa) / 0x258 + (uStack24 + 0x8) + local_1c;
  uVar3 = (param_1 + 0x6);
  GetWindowRect16(0x1008,&local_2c);
  uVar6 = 0x0;
  IVar2 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  local_e = (IVar2 - (iStack40 - local_2c.x)) / 0x2;
  move_win_1040_826c(param_1,local_10,local_e);
  if (paStack20 != 0x0) {
    ppcVar1 = paStack20;
    (**ppcVar1)(&ctx.PTR_LOOP_1050_1040,paStack20,(paStack20 >> 0x10),
                0x1,uVar6,uVar3,paStack20,paStack20);
  }
  return;
}


pub fn show_win_1038_a396(param_1: &mut Struct1,param_2: u16,param_3: u16)
{
  let in_AX: u16;
  let in_DX: u16;
  
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  unk_win_ui_op_1038_a18c(param_1,param_3);
  win_1008_5c7c(ctx.PTR__LOOP_1050_02a0,0x10001,param_3,in_AX,in_DX);
  (param_1 + 0x8c) = in_AX;
  ShowWindow16(0x1008,0x5);
  return;
}


pub fn win_ui_op_1038_a4ee(param_1: &mut Struct1,param_2: u16)
{
  let uVar1: u32;
  let in_DX: *mut u8
  let unaff_DI: i16;
  WNDCLASS16 *unaff_SS;
  let puVar2: *mut u16;
  
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  win_1008_5c7c(ctx.PTR__LOOP_1050_02a0,0x20001,unaff_SS,param_2,in_DX);
  (param_1 + 0x8c) = param_2;
  puVar2 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2,unaff_SS,in_DX,unaff_DI);
  uVar1 = (puVar2 + 0x6c);
  GetDlgItem16(0x1010,0x114);
  SetWindowText16(ctx.s_tile2_bmp_1050_1538,(SEGPTR)uVar1);
  SetFocus16(ctx.s_tile2_bmp_1050_1538);
  SendMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0xffff,0x4010000);
  unk_win_ui_op_1038_a18c(param_1,unaff_SS);
  ShowWindow16(ctx.s_tile2_bmp_1050_1538,0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_op_1038_a584(param_1: u16,param_2: i16,param_3: HWND16,param_4: u16)
{
  let uVar1: u16;
  let in_DX: *mut u8
  let unaff_DI: i16;
  let puVar2: *mut u16;
  let in_stack_00000006: u16;
  let puVar3: *mut u8;
  let local_52: [u8;50];
  
  if (param_2 != 0x0) {
    GetDlgItem16(param_3,0x114);
    GetWindowText16(ctx.s_tile2_bmp_1050_1538,0x50,local_52);
    uVar1 = str_op_1000_3da4(CONCAT22(param_4,local_52));
    if (uVar1 != 0x0) {
      puVar3 = local_52;
      puVar2 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2,param_4,in_DX,unaff_DI);
      pass1_1010_6006(puVar2,CONCAT22(param_4,puVar3),
                      (puVar2 >> 0x10));
      GetWindowWord16(0x1010,-0x8);
      PostMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x1110105);
      destroy_win_1040_7b98(CONCAT22(in_stack_00000006,param_1),&ctx.PTR_LOOP_1050_1040);
    }
  }
  return;
}


pub fn win_ui_op_1038_a6f4(param_1: &mut Struct1)
{
  let uVar1: u32;
  let uVar2: u16;
  let in_DX: *mut u8
  let uVar3: u16;
  let unaff_DI: i16;
  WNDCLASS16 *unaff_SS;
  let puVar4: *mut u16;
  LRESULT LVar5;
  
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  puVar4 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2,unaff_SS,in_DX,unaff_DI);
  uVar1 = (puVar4 + 0x68);
  GetDlgItem16(0x1010,0x115);
  SetWindowText16(ctx.s_tile2_bmp_1050_1538,(SEGPTR)uVar1);
  SetFocus16(ctx.s_tile2_bmp_1050_1538);
  LVar5 = SendMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0xffff,0x4010000);
  uVar3 = (LVar5 >> 0x10);
  uVar2 = LVar5;
  unk_win_ui_op_1038_a18c(param_1,unaff_SS);
  win_1008_5c7c(ctx.PTR__LOOP_1050_02a0,0x30001,unaff_SS,uVar2,uVar3);
  (param_1 + 0x8c) = uVar2;
  ShowWindow16(0x1008,0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_op_1038_a788(param_1: u32,param_2: i16,param_3: HWND16,param_4: u16)
{
  let uVar1: u16;
  let in_DX: *mut u8
  let unaff_DI: i16;
  let pUVar2: *mut u16;
  let puVar2: *mut u8;
  let local_52: [u8;50];
  let puVar3: *mut u8
  
  if (param_2 != 0x0) {
    GetDlgItem16(param_3,0x115);
    GetWindowText16(ctx.s_tile2_bmp_1050_1538,0x50,local_52);
    uVar1 = str_op_1000_3da4(CONCAT22(param_4,local_52));
    if (uVar1 != 0x0) {
      puVar2 = local_52;
      pUVar2 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2,param_4,in_DX,unaff_DI);
      pass1_1010_5fd8(pUVar2,CONCAT22(param_4,puVar2),
                      (pUVar2 >> 0x10));
      GetWindowWord16(0x1010,-0x8);
      PostMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x1110105);
      destroy_win_1040_7b98(param_1,&ctx.PTR_LOOP_1050_1040);
    }
  }
  return;
}


void 
enable_win_1038_a8f8
          (param_1: u16,param_2: u16,param_3: u16,TwoWords param_4,in_hwnd_5: HWND16)

{
  let enable: bool;
  
  if (param_4.b_0x2 == 0x116) {
    SendDlgItemMessage16(in_hwnd_5,0x0,0x0,0x1,0x11a0401);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x11a);
    enable = 0x0;
  }
  else {
    if ((param_4.b_0x2 == 0x116) || (0x2 < param_4.b_0x2 - 0x117)) {
      post_win_msg_1040_7b3c
                (CONCAT22(param_2,param_1),param_3,param_4,param_4.b_0x2,
                 &ctx.PTR_LOOP_1050_1040);
      return;
    }
    GetDlgItem16(in_hwnd_5,0x11a);
    enable = 0x1;
  }
  EnableWindow16(ctx.s_tile2_bmp_1050_1538,enable);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_op_1038_a972(param_1: &mut Struct1)
{
  let BVar1: bool;
  let uVar2: u16;
  WNDCLASS16 *unaff_SS;
  LRESULT LVar3;
  
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  SendDlgItemMessage16(&ctx.PTR_LOOP_1050_1040,0x0,0x0,0x1,0x1160401);
  LVar3 = SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x11a0401);
  uVar2 = (LVar3 >> 0x10);
  GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x11a);
  BVar1 = EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x0);
  win_1008_5c7c(ctx.PTR__LOOP_1050_02a0,0x40001,unaff_SS,BVar1,uVar2);
  *(bool *)(param_1 + 0x8c) = BVar1;
  unk_win_ui_op_1038_a18c(param_1,unaff_SS);
  ShowWindow16(0x1008,0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_sys_op_1038_a9fa(Uparam_1: i32,param_2: i16)
{
  let in_DX: *mut u8
  let unaff_DI: i16;
  let unaff_SS: u16;
  let puVar1: *mut u16;
  LRESULT LVar2;
  
  if (param_2 != 0x0) {
    puVar1 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2,unaff_SS,in_DX,unaff_DI);
    LVar2 = SendDlgItemMessage16(0x1010,0x0,0x0,0x0,0x1160400);
    if (LVar2 == 0x0) {
      LVar2 = SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1170400);
      if (LVar2 == 0x0) {
        LVar2 = SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1180400);
        if (LVar2 == 0x0) {
          LVar2 = SendDlgItemMessage16
                            (ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1190400);
          if (LVar2 != 0x0) {
            ctx.PTR_LOOP_1050_13ae = &DAT_1050_0004;
          }
        }
        else {
          ctx.PTR_LOOP_1050_13ae = (&ctx.PTR_LOOP_1050_0002 + 0x1);
        }
      }
      else {
        ctx.PTR_LOOP_1050_13ae = &ctx.PTR_LOOP_1050_0002;
      }
    }
    else {
      ctx.PTR_LOOP_1050_13ae = (&ctx.PTR_LOOP_1050_0000 + 0x1);
    }
    LVar2 = SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x11a0400);
    (puVar1 + 0x82) = LVar2;
    GetWindowWord16(ctx.s_tile2_bmp_1050_1538,-0x8);
    PostMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x1110105);
    destroy_win_1040_7b98(param_1,&ctx.PTR_LOOP_1050_1040);
  }
  return;
}


pub fn set_win_pos_1038_abdc(param_1: HWND16)
{
  RECT16 local_12 [0x2];
  RECT16 local_a;
  let iStack6: i16;
  let iStack4: i16;
  
  GetWindowRect16(param_1,&local_a);
  GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0xfd7);
  GetWindowRect16(ctx.s_tile2_bmp_1050_1538,local_12);
  iStack6 -= local_a.x;
  iStack4 = (local_12[0].y - local_a.y) + -0x2;
  SetWindowPos16(ctx.s_tile2_bmp_1050_1538,0x6,iStack4,iStack6,0x0,0x0,0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn unk_win_ui_op_1038_ac38(INT16 param_1,param_2: u16)
{
  let uVar1: u16;
  let iVar2: i16;
  let IVar3: i16;
  Ulet uVar3: i32;
  let extraout_DX: u16;
  let hwnd: HWND16;
  let hdc: HWND16;
  let uVar5: u32;
  COLORREF color;
  Ulet uVar4: u8;
  let iVar1: &mut Struct46;
  
  hwnd = ctx.s_tile2_bmp_1050_1538;
  GetStockObject16(param_1);
  if (ctx.PTR__LOOP_1050_5b78 == 0x0) {
    hwnd = 0x1008;
    uVar5 = pass1_1008_4d72((ctx.PTR__LOOP_1050_4230 + 0xe));
    uVar1 = (uVar5 >> 0x10);
    iVar2 = uVar5;
    ctx._PTR_LOOP_1050_5b6c =
         CONCAT12(*(iVar2 + 0x3ec),
                         CONCAT11(*(iVar2 + 0x3ed),
                                  *(iVar2 + 0x3ee)));
    ctx._PTR_LOOP_1050_5b70 =
         CONCAT12(*(iVar2 + 0x3e4),
                         CONCAT11(*(iVar2 + 0x3e5),
                                  *(iVar2 + 0x3e6)));
    ctx._PTR_LOOP_1050_5b74 =
         CONCAT12(*(iVar2 + 0x3f8),
                         CONCAT11(*(iVar2 + 0x3f9),
                                  *(iVar2 + 0x3fa)));
    ctx._PTR_LOOP_1050_5b78 =
         CONCAT12(*(iVar2 + 0x94),
                         CONCAT11(*(iVar2 + 0x95),
                                  *(iVar2 + 0x96)));
  }
  if (param_2 < 0x4) {
//LAB_1038_acf0:
    hdc = ctx.s_tile2_bmp_1050_1538;
    IVar3 = GetDlgCtrlID16(hwnd);
    if (IVar3 == 0xfd4) {
      color = (COLORREF)_PTR_LOOP_1050_5b70;
//       TODO: goto LAB_1038_ad0e;
    }
    if (IVar3 != 0xfd5) {
      if (IVar3 == 0xfd6) {
        color = (COLORREF)_PTR_LOOP_1050_5b6c;
//         TODO: goto LAB_1038_ad0e;
      }
      if (IVar3 == 0xfd7) {
        color = (COLORREF)_PTR_LOOP_1050_5b74;
//         TODO: goto LAB_1038_ad0e;
      }
    }
  }
  else {
    hdc = hwnd;
    if (param_2 != 0x4) {
      if ((param_2 == 0x4) || (0x1 < param_2 - 0x5)) {
        return;
      }
//       TODO: goto LAB_1038_acf0;
    }
  }
  color = (COLORREF)_PTR_LOOP_1050_5b78;
//LAB_1038_ad0e:
  SetTextColor16(hdc,color);
  SetBkColor16(ctx.s_tile2_bmp_1050_1538,0x0);
  return;
}


pub fn show_win_1038_b634(param_1: u32,param_2: HWND16)
{
  let iVar1: i16;
  let uVar2: u16;
  let HVar3: HWND16;
  let uStack4: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((iVar1 + 0xac) == 0x0) {
    (iVar1 + 0xac) = 0x1;
    for (uStack4 = 0x1; uStack4 < 0x2b; uStack4 += 0x1) {
      HVar3 = param_2;
      if (((uStack4 * 0x4 + iVar1 + 0x2) | (uStack4 * 0x4 + iVar1)) !=
          0x0) {
        HVar3 = ctx.s_tile2_bmp_1050_1538;
        ShowWindow16(param_2,0x0);
      }
      param_2 = HVar3;
    }
  }
  return;
}



pub fn show_win_1038_b68a(param_1: u32,param_2: HWND16)
{
  let iVar1: i16;
  let uVar2: u16;
  let HVar3: HWND16;
  let uStack4: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((iVar1 + 0xac) != 0x0) {
    (iVar1 + 0xac) = 0x0;
    for (uStack4 = 0x1; uStack4 < 0x2b; uStack4 += 0x1) {
      HVar3 = param_2;
      if (((uStack4 * 0x4 + iVar1 + 0x2) | (uStack4 * 0x4 + iVar1)) !=
          0x0) {
        HVar3 = ctx.s_tile2_bmp_1050_1538;
        ShowWindow16(param_2,0x1);
      }
      param_2 = HVar3;
    }
  }
  return;
}


pub fn bring_win_to_top_1038_b72e(param_1: u32,param_2: i16,in_win_handle_3: HWND16) -> bool

{
  if ((param_2 * 0x4 + param_1) != 0x0) {
    SetFocus16(in_win_handle_3);
    BringWindowToTop16(ctx.s_tile2_bmp_1050_1538);
    return 0x1;
  }
  return 0x0;
}



pub fn win_ui_op_1038_b81c(param_1: &mut Struct1)
{
  let uVar1: u32;
  let uVar2: u32;
  let ppcVar3: u32;
  let uVar4: u16;
  let win_enabled: bool;
  let in_DX: *mut u8
  let extraout_DX: u16;
  let iVar6: i16;
  let unaff_DI: i16;
  let uVar7: u16;
  let HVar8: HWND16;
  let hwnd_dlg: HWND16;
  let unaff_SS: u16;
  let puVar9: *mut u16;
  let piStack16: *mut i16;
  let UStack12: u16;
  let uStack10: i16;
  let iVar7: &mut Struct1;
  let piVar5: *mut i16;
  
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  puVar9 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x6,unaff_SS,in_DX,unaff_DI);
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  (iVar6 + 0x92) = puVar9;
  (iVar6 + 0x94) = (puVar9 >> 0x10);
  uVar1 = (iVar6 + 0x92);
  uVar4 = uVar1 + 0x4e;
  uVar1 &= 0xffff0000;
  piVar5 = (uVar1 | uVar4);
  iStack10 = 0x0;
  hwnd_dlg = 0x1010;
  for (UStack12 = 0x1a0; UStack12 < 0x1b5; UStack12 += 0x1) {
    if ((iStack10 * 0x2 + uVar4) == UStack12) {
      iStack10 += 0x1;
      HVar8 = hwnd_dlg;
    }
    else {
      HVar8 = ctx.s_tile2_bmp_1050_1538;
      CheckDlgButton16(hwnd_dlg,0x2,UStack12);
    }
    hwnd_dlg = HVar8;
  }
  GetDlgItem16(hwnd_dlg,0xfb1);
  win_enabled = EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x0);
  uVar2 = (iVar6 + 0x92);
  ppcVar3 = ((iVar6 + 0x92) + 0x10);
  (**ppcVar3)(ctx.s_tile2_bmp_1050_1538,uVar2,(uVar2 >> 0x10));
  piStack16 = CONCAT22(extraout_DX,win_enabled);
  move_win_1040_826c(param_1,(win_enabled + 0x2) + -0x2,
                     (win_enabled + 0x4) + *piStack16 + 0x3);
  ShowWindow16(&ctx.PTR_LOOP_1050_1040,0x5);
  pass1_1018_1c9a((iVar6 + 0x92),*piVar5);
  GetDlgItem16(0x1018,*piVar5);
  SetFocus16(ctx.s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 
win_ui_op_1038_b922(param_1: *mut u32,param_2: u32,param_3: u16,param_4: u16,
                   param_5: HWND16,WNDCLASS16 *param_6)

{
  let piVar1: *mut i16;
  let ppcVar2: u32;
  let UVar3: u16;
  let Bvar4: bool;
  let uVar5: u16;
  let puVar6: *mut u8
  let iVar7: i16;
  let unaff_DI: i16;
  let uVar8: u16;
  let uVar9: u16;
  LRESULT LVar10;
  let mut pcVar11: String; 
  let paVar12: &mut Struct57;
  let uVar13: u32;
  let mut pCVar14: String; 
  WNDCLASS16 *pWVar15;
  let puVar16: *mut u8;
  let uStack1132: u16;
  char local_464 [0x50];
  CHAR local_414 [0x400];
  let uStack20: u32;
  let puStack16: *mut u8;
  let puStack14: *mut u16;
  let uStack10: i16;
  let HStack8: HWND16;
  let BStack6: bool;
  let uStack4: u16;
  
  uVar13 = CONCAT22(param_4,HStack8);
  BStack6 = 0x0;
  uStack4 = 0x0;
  iVar7 = param_1;
  uVar8 = (param_1 >> 0x10);
  if (param_3 < 0x1b5) {
    if (param_3 < 0x1a0) {
      uVar13 = CONCAT22(param_4,HStack8);
      if (param_3 != 0x2) goto LAB_1038_bbbf;
    }
    else {
      HStack8 = GetDlgItem16(param_5,param_3);
      LVar10 = SendMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x4000000);
      iStack10 = LVar10;
      if (iStack10 == 0x2) {
        BStack6 = 0x0;
        uStack4 = 0x0;
//         TODO: goto LAB_1038_bc26;
      }
      SendMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,
                    CONCAT13(0x4,CONCAT12(0x1,(iStack10 == 0x0))));
      UVar3 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538,param_3);
      if (UVar3 == 0x0) {
        piVar1 = (iVar7 + 0x96);
        *piVar1 = *piVar1 + 0x1;
        if ((iVar7 + 0x96) == 0x1) {
          GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0xfb1);
          EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x0);
        }
      }
      else {
        piVar1 = (iVar7 + 0x96);
        *piVar1 = *piVar1 + -0x1;
        GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0xfb1);
        BVar4 = IsWindowEnabled16(ctx.s_tile2_bmp_1050_1538);
        if (BVar4 == 0x0) {
          GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0xfb1);
          EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x1);
        }
        if ((iVar7 + 0x96) < 0x0) {
          CheckDlgButton16(ctx.s_tile2_bmp_1050_1538,0x0,(iVar7 + 0x98));
          (iVar7 + 0x96) = 0x0;
        }
        (iVar7 + 0x98) = param_3;
        pass1_1018_1c9a((iVar7 + 0x92),param_3);
        puStack14 = pass1_1018_1e78((iVar7 + 0x92),-0x1);
        uVar5 = (puStack14 >> 0x10);
        if (puStack14 == 0x0) {
          puStack16 = 0x0;
        }
        else {
          puStack16 = (puStack14 + 0x1c);
        }
        win_1008_5c7c(ctx.PTR__LOOP_1050_02a0,CONCAT22(puStack16,0x1),param_6,
                      puStack16,uVar5 | puStack14);
      }
    }
    BStack6 = 0x1;
    uStack4 = 0x0;
  }
  else {
    if (param_3 == 0xfb1) {
      for (uStack1132 = 0x1a0; uVar13 = CONCAT22(param_4,HStack8), uStack1132 < 0x1b5;
          uStack1132 += 0x1) {
        UVar3 = IsDlgButtonChecked(param_5,uStack1132);
        if (UVar3 == 0x1) {
          pass1_1008_d818((iVar7 + 0x8e),uStack1132);
          uVar13 = CONCAT22(param_4,HStack8);
//           TODO: goto LAB_1038_bba2;
        }
        param_5 = ctx.s_tile2_bmp_1050_1538;
      }
    }
    else {
      if (param_3 != 0xfbe) goto LAB_1038_bbbf;
      puStack14 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2,param_6,param_4
                                  ,unaff_DI);
      puStack16 = ctx.PTR_LOOP_1050_13ae;
      if (ctx.PTR_LOOP_1050_13ae == (&ctx.PTR_LOOP_1050_0000 + 0x1)) {
        puStack16 = &ctx.PTR_LOOP_1050_0002;
      }
      iStack10 = (puStack16 * 0xc + 0x5b84) + -0x1;
      pass1_1008_612e(0x0,iStack10,iStack10);
      uStack20 = pass1_1018_1e78((iVar7 + 0x92),
                                 ((puStack16 * 0x6 + iStack10) * 0x2 + 0x5b86
                                         ));
      load_string_1010_84e0
                (0x1010,_PTR_LOOP_1050_14cc,
                 (ctx.PTR__LOOP_1050_14cc >> 0x10),0x50,local_464,
                 (short)param_6);
      pcVar11 = load_string_1010_847e
                          (ctx.PTR__LOOP_1050_14cc,
                           (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
      puVar6 = (pcVar11 >> 0x10);
      uVar5 = wsprintf16(0x1010,local_414,param_6);
      uVar9 = 0x1000;
      mem_op_1000_179c(0xb4,puVar6,0x1000);
      if ((puVar6 | uVar5) == 0x0) {
        paVar12 = 0x0;
      }
      else {
        pCVar14 = local_414;
        pWVar15 = param_6;
        puVar16 = ctx.PTR_LOOP_1050_0396;
        pcVar11 = load_string_1010_847e
                            (ctx.PTR__LOOP_1050_14cc,
                             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
        uVar9 = SUB42(&ctx.PTR_LOOP_1050_1040,0x0);
        paVar12 = pass1_1040_8478(CONCAT22(puVar6,puVar16),0x41,pcVar11,
                                  CONCAT22(pWVar15,pCVar14),puVar16,
                                  (pcVar11 >> 0x10));
      }
      ppcVar2 = (paVar12 + 0x74);
      uVar13 = (**ppcVar2)(uVar9,paVar12);
      if (uVar13 != 0x1) goto LAB_1038_bc26;
      pass1_1008_d818((iVar7 + 0x8e),(uStack20 + 0x1a));
      HStack8 = uVar13;
//LAB_1038_bba2:
      param_5 = 0x1008;
      win_ui_cursor_op_1038_bc30(param_1,0x1008,param_6);
      HStack8 = uVar13;
    }
    PostMessage16(param_5,0x0,0x0,0x11100ce);
    HStack8 = uVar13;
    param_3 = 0x1;
//LAB_1038_bbbf:
    BStack6 = post_win_msg_1040_7b3c
                        (param_1,param_2,(param_2 >> 0x10),param_3,
                         &ctx.PTR_LOOP_1050_1040);
    uStack4 = (uVar13 >> 0x10);
    HStack8 = uVar13;
  }
//LAB_1038_bc26:
  return CONCAT22(uStack4,BStack6);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_cursor_op_1038_bc30(param_1: u32,HINSTANCE16 param_2,param_3: u16)
{
  let uVar1: u32;
  ulet in_AF: u8;
  let local_112: u16;
  let uStack272: u16;
  HCURSOR16 HStack6;
  HCURSOR16 HStack4;
  
  HStack4 = LoadCursor16(param_2,0x7f02);
  HStack6 = SetCursor16(ctx.s_tile2_bmp_1050_1538);
  uVar1 = (param_1 + 0x8e);
  pass1_1030_532e(CONCAT22(param_3,&local_112),
                  (uVar1 + 0xe) + 0x1000000,param_3,in_AF);
  fn_ptr_1030_835a(ctx.PTR__LOOP_1050_5748,CONCAT22(param_3,&local_112));
  pass1_1030_838e(ctx.PTR__LOOP_1050_5748,param_3,in_AF);
  local_112 = 0x389a;
  uStack272 = 0x1008;
  pass1_1030_8334(ctx.PTR__LOOP_1050_5748);
  SetCursor16(0x1030);
  return;
}


pub fn win_dlg_op_1038_bea4(param_1: u32,param_2: *mut u16)
{
  let uVar1: u32;
  let HVar2: HWND16;
  let in_DX: *mut u8
  let puVar3: *mut u8
  let uVar4: u16;
  WPARAM16 wparam;
  let iVar5: i16;
  let unaff_DI: i16;
  let uVar6: u16;
  let puVar7: *mut u16;
  let uVar8: u32;
  let mut pcVar9: String; 
  LRESULT LVar10;
  let local_116: u32;
  let local_112: u32;
  CHAR local_10e [0x82];
  let local_8c: [u8;82];
  let uStack10: u32;
  let puStack6: *mut u16;
  
  puStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2,param_2,in_DX,unaff_DI);
  puVar3 = (puStack6 >> 0x10);
  uStack10 = (puStack6 + 0x68);
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  GetWindowText16(0x1010,0x80,local_8c);
  wsprintf16(s_tile2_bmp_1050_1538,local_10e,param_2);
  SetWindowText16(ctx.s_tile2_bmp_1050_1538,(SEGPTR)local_10e);
  HVar2 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x179);
  *(HWND16 *)(iVar5 + 0x92) = HVar2;
  pass1_1008_e3ec((iVar5 + 0x8e),CONCAT22(param_2,&local_116),
                  CONCAT22(param_2,&local_112),param_2);
  send_msg_1038_c374(param_1,local_112,(iVar5 + 0x92),0x1008);
  puVar7 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2f,param_2,puVar3,unaff_DI);
  uVar4 = (puVar7 >> 0x10);
  uVar8 = (puVar7 + 0x24);
  uVar1 = (iVar5 + 0x8e);
  uVar8 = string_1008_e586(uVar1,(uVar1 >> 0x10),uVar8,uVar8,
                           uVar4);
  SendMessage16(0x1008,uVar8,(WPARAM16)(uVar8 >> 0x10),0x40dffff);
  HVar2 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x17a);
  *(HWND16 *)(iVar5 + 0x94) = HVar2;
  send_msg_1038_c374(param_1,local_116,HVar2,s_tile2_bmp_1050_1538);
  pcVar9 = load_string_1010_847e
                     (ctx.PTR__LOOP_1050_14cc,(ctx.PTR__LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  wparam = (WPARAM16)(pcVar9 >> 0x10);
  LVar10 = SendMessage16(0x1010,pcVar9,wparam,0x4030000);
  (iVar5 + 0x9c) = LVar10;
  SendMessage16(ctx.s_tile2_bmp_1050_1538,pcVar9,wparam,0x40dffff);
  HVar2 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x178);
  *(HWND16 *)(iVar5 + 0x96) = HVar2;
  HVar2 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x177);
  *(HWND16 *)(iVar5 + 0x98) = HVar2;
  HVar2 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x184);
  *(HWND16 *)(iVar5 + 0x9a) = HVar2;
  return;
}



pub fn show_win_1038_c044(param_1: &mut Struct1)
{
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(&ctx.PTR_LOOP_1050_1040,0x5);
  SetFocus16(ctx.s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
msg_box_op_1038_c07a
          (param_1: i16,param_2: u16,param_3: u16,param_4: u32,param_5: u16)

{
  let iVar1: i16;
  let uVar2: u16;
  let unaff_CS: u16;
  let hwnd: HWND16;
  ulet in_AF: u8;
  uchar local_70c [0x200];
  char local_50c [0x100];
  char local_40c [0x402];
  let uStack10: u32;
  let uStack6: u32;
  
  send_msg_1038_c228(CONCAT22(param_2,param_1),unaff_CS);
  uStack6 = load_string_1010_847e
                      (ctx.PTR__LOOP_1050_14cc,
                       (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
  if (param_4._2_2_ == 0x177) {
    pass1_1008_e05e((param_1 + 0x8e),0x2,CONCAT22(param_2,param_1 + 0x19e),
                    CONCAT22(param_2,param_1 + 0x9e),param_5,in_AF);
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (ctx.PTR__LOOP_1050_14cc >> 0x10),0x200,local_40c,param_5);
    sys_1000_3f9c(local_70c,param_5,local_40c,param_5,param_1 + 0x19e,
                  &stack0xfffe,param_2,0x1000,param_5,in_AF);
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (ctx.PTR__LOOP_1050_14cc >> 0x10),0x100,local_50c,param_5);
    hwnd = ctx.s_tile2_bmp_1050_1538;
    MessageBox16(0x1010,(s_New_failed_in_Op__Op_1050_0020 + 0x10),local_50c,
                 param_5);
  }
  else {
    if (param_4._2_2_ != 0x178) {
      if ((param_4._2_2_ != 0x178) && (param_4._2_2_ - 0x179 < 0x2)) {
        set_win_pos_1038_c31a(CONCAT22(param_2,param_1),param_3,param_4,0x1010);
        return;
      }
      post_win_msg_1040_7b3c
                (CONCAT22(param_2,param_1),param_3,param_4,param_4._2_2_,
                 &ctx.PTR_LOOP_1050_1040);
      return;
    }
    uStack10 = CONCAT22(param_2,param_1 + 0x9e);
    uVar2 = param_2;
    iVar1 = pass1_1008_e10c((param_1 + 0x8e),CONCAT22(param_2,param_1 + 0x19e),
                            CONCAT22(param_2,param_1 + 0x9e),param_2,param_5);
    if (iVar1 == 0x0) {
      load_string_1010_84e0
                (0x1010,_PTR_LOOP_1050_14cc,
                 (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_40c,param_5);
      load_string_1010_84e0
                (0x1010,_PTR_LOOP_1050_14cc,
                 (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,local_50c,param_5);
      MessageBox16(0x1010,(s_New_failed_in_Op__Op_1050_0020 + 0x10),local_50c,
                   param_5);
      return;
    }
    hwnd = 0x1008;
    pass1_1008_e01c((param_1 + 0x8e),CONCAT22(param_2,param_1 + 0x19e),uStack10)
    ;
    pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c,(param_1 + 0x8),0x1f,uVar2,param_1,
                    0x1008,param_5);
  }
  PostMessage16(hwnd,0x0,0x0,0x1110002);
  return;
}



LRESULT  send_msg_1038_c228(param_1: u32,param_2: HWND16)

{
  WPARAM16 wparam;
  LRESULT LVar1;
  LRESULT LVar2;
  
  wparam = (WPARAM16)(param_1 >> 0x10);
  LVar1 = SendMessage16(param_2,0x0,0x0,0x4070000);
  LVar2 = SendMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x4070000);
  SendMessage16(ctx.s_tile2_bmp_1050_1538,param_1 + 0x9e,wparam,
                CONCAT22(0x408,LVar1));
  LVar1 = SendMessage16(ctx.s_tile2_bmp_1050_1538,param_1 + 0x19e,wparam,
                        CONCAT22(0x408,LVar2));
  return LVar1;
}



pub fn enable_win_1038_c294(param_1: u32)
{
  SEGPTR lp_string;
  let uVar1: u16;
  let unaff_SS: u16;
  let uStack12: u32;
  
  lp_string = param_1 + 0x9e;
  uStack12 = param_1 & 0xffff0000 | lp_string;
  pass1_1008_e320(*(astruct_102 **)(param_1 + 0x8e),
                  param_1 & 0xffff0000 | (param_1 + 0x19e),
                  param_1 & 0xffff0000 | lp_string,unaff_SS);
  SetWindowText16(0x1008,lp_string);
  uVar1 = pass1_1008_e2a4((param_1 + 0x8e),
                          param_1 & 0xffff0000 | (param_1 + 0x19e),uStack12);
  EnableWindow16(0x1008,uVar1 & 0x1);
  EnableWindow16(ctx.s_tile2_bmp_1050_1538,uVar1 & 0x2);
  return;
}



pub fn set_win_pos_1038_c31a(param_1: u32,param_2: u16,param_3: i16,param_4: HWND16) -> bool

{
  RECT16 local_e;
  let uStack10: i16;
  let uStack6: u16;
  let iStack4: i16;
  
  iStack4 = param_3;
  uStack6 = param_2;
  if (param_3 == 0x1) {
    enable_win_1038_c294(param_1);
  }
  else {
    if (param_3 != 0x7) {
      return 0x0;
    }
    GetWindowRect16(param_4,&local_e);
    iStack10 -= local_e.x;
    SetWindowPos16(ctx.s_tile2_bmp_1050_1538,0x2,0x50,iStack10,0x0,0x0,0x0);
  }
  return 0x1;
}



pub fn send_msg_1038_c374(param_1: u32,param_2: *mut u32,param_3: u16,param_4: HWND16)
{
  let uVar1: u32;
  let ppcVar2: u32;
  let uVar3: u16;
  let uVar4: u32;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar5: u16;
  let uVar6: u16;
  LRESULT LVar7;
  let paVar8: &mut Struct18;
  let uVar9: u16;
  let uStack10: u32;
  let uStack6: u32;
  
  uVar6 = SUB42(ctx.s_tile2_bmp_1050_1538,0x0);
  uVar9 = param_3;
  LVar7 = SendMessage16(param_4,0x0,0x0,0x40b0000);
  uVar3 = LVar7;
  uVar5 = param_2;
  ppcVar2 = (*param_2 + 0x10);
  (**ppcVar2)(ctx.s_tile2_bmp_1050_1538,param_2,uVar9);
  uStack6 = CONCAT22(extraout_DX,uVar3);
  uStack10 = 0x0;
  while( true ) {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar2 = (*param_2 + 0x4);
    uVar4 = uStack6;
    (**ppcVar2)(uVar6,param_2,uStack10,(uStack10 >> 0x10),uVar5);
    uVar1 = (param_1 + 0x8e);
    paVar8 = 
             string_1008_e586(uVar1,(uVar1 >> 0x10),
                              uVar4 & 0xffff | extraout_DX_00 << 0x10,uVar4,
                              extraout_DX_00);
    uVar5 = param_3;
    LVar7 = SendMessage16(0x1008,paVar8,(WPARAM16)(paVar8 >> 0x10),
                          0x4030000);
    uVar6 = 0x1000;
    fn_ptr_1000_17ce(paVar8,0x1000);
    if (LVar7 == -0x1) break;
    if (LVar7 == -0x2) {
      return;
    }
    uStack10 += 0x1;
  }
  return;
}


pub fn show_win_1038_c558(param_1: &mut Struct1)
{
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(&ctx.PTR_LOOP_1050_1040,0x5);
  SetFocus16(ctx.s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_dlg_op_1038_c58e(param_1: u32,param_2: *mut u16)
{
  let in_DX: *mut u8
  let iVar1: i16;
  let unaff_DI: i16;
  CHAR local_80e [0x402];
  CHAR local_40c [0x402];
  let uStack10: u32;
  let puStack6: *mut u16;
  
  puStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2,param_2,in_DX,unaff_DI);
  uStack10 = (puStack6 + 0x68);
  iVar1 = param_1;
  GetWindowText16(0x1010,0x80,local_40c);
  wsprintf16(s_tile2_bmp_1050_1538,local_80e,param_2);
  SetWindowText16(ctx.s_tile2_bmp_1050_1538,(SEGPTR)local_80e);
  pass1_1008_e038((iVar1 + 0x8e),
                  (param_1 & 0xffff0000 | (iVar1 + 0x92)),
                  (param_1 & 0xffff0000 | (iVar1 + 0x96)));
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (ctx.PTR__LOOP_1050_14cc >> 0x10),0x400,local_80e,(short)param_2);
  wsprintf16(0x1010,local_40c,param_2);
  SetDlgItemText16(ctx.s_tile2_bmp_1050_1538,local_40c,(SEGPTR)param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
message_box_op_1038_c672
          (param_1: i16,param_2: u16,param_3: u16,param_4: u32,short param_5)

{
  let uVar1: u32;
  let hwnd: HWND16;
  ulet in_AF: u8;
  let uVar2: u16;
  char local_404 [0x402];
  
  uVar2 = (ctx.PTR__LOOP_1050_14cc >> 0x10);
  if (param_4._2_2_ == 0x17d) {
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,uVar2,0x3ff,local_404,param_5);
    uVar1 = (param_1 + 0x92);
    hwnd = ctx.s_tile2_bmp_1050_1538;
    MessageBox16(0x1010,(s_New_failed_in_Op__Op_1050_0020 + 0x10),uVar1,
                 (uVar1 >> 0x10));
  }
  else {
    if (param_4._2_2_ != 0x17e) {
      post_win_msg_1040_7b3c
                (CONCAT22(param_2,param_1),param_3,param_4,param_4._2_2_,
                 &ctx.PTR_LOOP_1050_1040);
      return;
    }
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,uVar2,0x3ff,local_404,param_5);
    uVar1 = (param_1 + 0x92);
    MessageBox16(0x1010,(s_New_failed_in_Op__Op_1050_0020 + 0x10),uVar1,
                 (uVar1 >> 0x10));
    hwnd = 0x1008;
    pass1_1008_e164((param_1 + 0x8e),param_5,in_AF);
  }
  PostMessage16(hwnd,0x0,0x0,0x1110002);
  return;
}


pub fn destroy_window_1038_c836(param_1: i16,Uparam_2: i32,Uparam_3: i32,param_4: u16)
{
  let uVar1: u32;
  let puVar2: *mut u16;
  let local_6: [u8;4];
  
  if (param_3._2_2_ == 0xfce) {
    puVar2 = pass1_1008_941a(CONCAT22(param_4,local_6),0x1,0xac);
    win_1008_5c9e(ctx.PTR__LOOP_1050_02a0,CONCAT22(param_4,local_6),local_6,
                  (puVar2 >> 0x10),param_4);
    uVar1 = (param_1 + 0x8e);
    (uVar1 + 0xa) = 0x6;
    DestroyWindow16(0x1008);
    ctx.PTR_LOOP_1050_5b80 = 0x0;
    return;
  }
  post_win_msg_1040_7b3c
            (CONCAT22(param_2,param_1),(param_2 >> 0x10),
             param_3,param_3._2_2_,&ctx.PTR_LOOP_1050_1040);
  return;
}



pub fn win_ui_op_1038_c89c(param_1: &mut Struct1)
{
  let iVar1: i16;
  let uVar2: u32;
  let HVar3: HWND16;
  let uVar4: u16;
  let enable: bool;
  
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  uVar4 = (param_1 >> 0x10);
  CheckRadioButton16(&ctx.PTR_LOOP_1050_1040,0xfac,0xfad,0xfac);
  uVar2 = (param_1 + 0x8e);
  (uVar2 + 0xa) = 0x1;
  uVar2 = (param_1 + 0x8e);
  iVar1 = (uVar2 + 0x12);
  if (iVar1 == 0x4) {
//LAB_1038_c8da:
    HVar3 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0xfce);
    if (HVar3 != 0x0) {
      EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x1);
    }
    HVar3 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x1);
    if (HVar3 == 0x0) goto LAB_1038_c93c;
    enable = 0x0;
  }
  else {
    if ((iVar1 + -0x5 < 0x1) || (SBORROW2(iVar1 + -0x5,0x1))) goto LAB_1038_c93c;
    if (iVar1 != 0x8 && 0x0 < iVar1 + -0x7) {
      if (iVar1 != 0x9) goto LAB_1038_c93c;
//       TODO: goto LAB_1038_c8da;
    }
    HVar3 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0xfce);
    if (HVar3 == 0x0) goto LAB_1038_c93c;
    enable = 0x1;
  }
  EnableWindow16(ctx.s_tile2_bmp_1050_1538,enable);
//LAB_1038_c93c:
  move_win_1040_826c(param_1,0xc8,0x0);
  ShowWindow16(&ctx.PTR_LOOP_1050_1040,0x5);
  return;
}



pub fn win_dlg_op_1038_c95e(param_1: u32,param_2: i16)
{
  let uVar1: u32;
  let UVar2: u16;
  let iVar3: i16;
  let uVar4: u16;
  let unaff_CS: HWND16;
  
  iVar3 = param_1;
  uVar4 = (param_1 >> 0x10);
  if (param_2 == 0x0) {
    uVar1 = (iVar3 + 0x8e);
    (uVar1 + 0xa) = 0x0;
  }
  else {
    UVar2 = IsDlgButtonChecked(unaff_CS,0xfac);
    if (UVar2 == 0x0) {
      unaff_CS = ctx.s_tile2_bmp_1050_1538;
      UVar2 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538,0xfad);
      if (UVar2 == 0x0) {
        unaff_CS = ctx.s_tile2_bmp_1050_1538;
        UVar2 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538,0xfae);
        if (UVar2 == 0x0) {
          unaff_CS = ctx.s_tile2_bmp_1050_1538;
          UVar2 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538,0xfaf);
          if (UVar2 == 0x0) {
            unaff_CS = ctx.s_tile2_bmp_1050_1538;
            UVar2 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538,0xfb0);
            if (UVar2 != 0x0) {
              uVar1 = (iVar3 + 0x8e);
              (uVar1 + 0xa) = 0x5;
            }
          }
          else {
            uVar1 = (iVar3 + 0x8e);
            (uVar1 + 0xa) = 0x4;
          }
        }
        else {
          uVar1 = (iVar3 + 0x8e);
          (uVar1 + 0xa) = 0x3;
        }
      }
      else {
        uVar1 = (iVar3 + 0x8e);
        (uVar1 + 0xa) = 0x2;
      }
    }
    else {
      uVar1 = (iVar3 + 0x8e);
      (uVar1 + 0xa) = 0x1;
      unaff_CS = ctx.s_tile2_bmp_1050_1538;
    }
  }
  DestroyWindow16(unaff_CS);
  ctx.PTR_LOOP_1050_5b80 = 0x0;
  return;
}


pub fn show_win_1038_cb5c(param_1: &mut Struct1)
{
  let uVar1: u32;
  let uVar2: u16;
  let in_DX: u16;
  let puVar3: *mut u8
  let uVar4: u16;
  let uVar5: u16;
  WNDCLASS16 *unaff_SS;
  let puVar6: *mut u16;
  let puVar7: *mut u16;
  let uStack10: i16;
  
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  uVar5 = (param_1 >> 0x10);
  uVar2 = pass1_1008_eb6e();
  for (iStack10 = 0x0; iStack10 < uVar2; iStack10 += 0x1) {
    uVar1 = (param_1 + 0x8e);
    puVar6 =
             pass1_1008_eb5c(uVar1,(uVar1 >> 0x10),iStack10);
    puVar3 = (puVar6 >> 0x10);
    puVar7 = puVar6;
    mem_op_1000_179c(0x42,puVar3,0x1000);
    uVar4 = (puVar7 >> 0x10);
    in_DX = uVar4 | puVar7;
    if (puVar7 != 0x0) {
      pass1_1008_3bd6(puVar7,uVar4,0x0,
                      CONCAT22(*puVar6,(puVar6 + 0x2)),0x101,0xff0100,
                      CONCAT22((param_1 + 0x6),
                               (puVar6 + 0x4)),in_DX,unaff_SS)
      ;
    }
  }
  win_1008_5c7c(ctx.PTR__LOOP_1050_02a0,0x90001,unaff_SS,uVar2,in_DX);
  ShowWindow16(0x1008,0x5);
  return;
}


pub fn check_dlg_btn_checked_1038_cdd6(param_1: u32,param_2: i16,param_3: HWND16)
{
  let uVar1: u32;
  let UVar2: u16;
  let iVar3: &mut Struct61;
  let uVar3: u16;
  
  iVar3 = param_1;
  uVar3 = (param_1 >> 0x10);
  if (param_2 == 0x0) {
    uVar1 = iVar3.field_0x8e;
    (uVar1 + 0xa) = 0x0;
  }
  else {
    UVar2 = IsDlgButtonChecked(param_3,0x182e);
    if (UVar2 == 0x0) {
      UVar2 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538,0x182f);
      if (UVar2 == 0x0) {
        UVar2 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538,0x1829);
        if (UVar2 == 0x0) {
          UVar2 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538,0x182a);
          if (UVar2 == 0x0) {
            UVar2 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538,0x182c);
            if (UVar2 == 0x0) {
              UVar2 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538,0x182d);
              if (UVar2 != 0x0) {
                uVar1 = iVar3.field_0x8e;
                (uVar1 + 0xa) = 0x7;
              }
            }
            else {
              uVar1 = iVar3.field_0x8e;
              (uVar1 + 0xa) = 0x6;
            }
          }
          else {
            uVar1 = iVar3.field_0x8e;
            (uVar1 + 0xa) = 0x4;
          }
        }
        else {
          uVar1 = iVar3.field_0x8e;
          (uVar1 + 0xa) = 0x3;
        }
      }
      else {
        uVar1 = iVar3.field_0x8e;
        (uVar1 + 0xa) = 0x2;
      }
    }
    else {
      uVar1 = iVar3.field_0x8e;
      (uVar1 + 0xa) = 0x1;
    }
  }
  iVar3.field_0x92 = 0x0;
  return;
}


pub fn win_ui_op_1038_d2a2(param_1: &mut Struct1)
{
  let rect: &mut Struct160;
  let iVar1: i16;
  let BVar2: bool;
  let in_DX: *mut u8
  let puVar3: *mut u8
  let uVar4: u16;
  let unaff_DI: i16;
  let uVar5: u16;
  let hwnd: HWND16;
  let hwnd_00: HWND16;
  WNDCLASS16 *unaff_SS;
  let mut pcVar6: String; 
  LRESULT LVar7;
  WPARAM16 w_param;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let local_16: u16;
  let uStack20: u16;
  let uStack18: u16;
  let uStack16: u16;
  let uStack14: u32;
  let uStack10: i16;
  let uStack8: u32;
  let iStack4: i16;
  
  hwnd = &ctx.PTR_LOOP_1050_1040;
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  iStack4 = 0x7;
  for (iStack10 = 0x0; uVar5 = (param_1 >> 0x10), iStack10 < iStack4;
      iStack10 += 0x1) {
    unaff_DI = iStack10 * 0xc;
    local_16 = (unaff_DI + 0x5c0c);
    uStack20 = (unaff_DI + 0x5c0e);
    uStack18 = 0x1;
    uStack16 = 0x1;
    rect = &local_16;
    MapDialogRect16(hwnd,(RECT16 *)rect);
    hwnd_00 = 0x1000;
    mem_op_1000_179c(0x42,in_DX,0x1000);
    puVar3 = (in_DX | rect);
    if (puVar3 == 0x0) {
      rect = 0x0;
      in_DX = 0x0;
    }
    else {
      hwnd_00 = 0x1008;
      pass1_1008_3bd6(rect,in_DX,0x1,CONCAT22(local_16,uStack20),0x104,0x1020103,
                      CONCAT22((param_1 + 0x6),
                               (unaff_DI + 0x5c10)),puVar3,
                      unaff_SS);
      in_DX = puVar3;
    }
    uStack8 = CONCAT22(in_DX,rect);
    hwnd = hwnd_00;
    if ((iStack10 * 0xc + 0x5c12) == 0x0) {
      hwnd = ctx.s_tile2_bmp_1050_1538;
      EnableWindow16(hwnd_00,0x0);
    }
  }
  uVar10 = 0x86;
  uStack14 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x9,unaff_SS,in_DX,unaff_DI);
  uVar4 = (uStack14 >> 0x10);
  iVar1 = pass1_1010_659a(uStack14,uVar10,unaff_SS);
  if (iVar1 == 0x0) {
    GetDlgItem16(0x1010,0x14a);
    EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x0);
    uVar8 = 0xc;
    uVar9 = 0x144;
    w_param = 0x0;
    pcVar6 = load_string_1010_847e
                       (ctx.PTR__LOOP_1050_14cc,
                        (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
    LVar7 = SendDlgItemMessage16
                      (0x1010,pcVar6,(pcVar6 >> 0x10),w_param,
                       CONCAT22(uVar9,uVar8));
    uVar4 = (LVar7 >> 0x10);
  }
  move_win_1040_826c(param_1,-0x1,0xffff);
  BVar2 = ShowWindow16(&ctx.PTR_LOOP_1050_1040,0x5);
  win_1008_5c7c(ctx.PTR__LOOP_1050_02a0,0x9a0001,unaff_SS,BVar2,uVar4);
  *(bool *)(param_1 + 0x8c) = BVar2;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn unk_win_ui_op_1038_d400(param_1: u8,param_2: u16,param_3: u16,Uparam_4: i32)
{
  let iVar1: i16;
  let uVar2: u16;
  let BVar3: bool;
  let in_DX: u16;
  let puVar4: *mut u8
  let unaff_DI: i16;
  let hwnd: HWND16;
  let hwnd_00: HWND16;
  WNDCLASS16 *unaff_SS;
  let puVar5: *mut u16;
  LRESULT LVar6;
  let mut pcVar7: String; 
  let in_stack_00000005: u8;
  WPARAM16 WVar8;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  let local_c: [u8;4];
  let iStack8: i16;
  let uStack6: u32;
  
  uStack6 = 0x0;
  iStack8 = 0x0;
  switch(param_4._2_2_) {
  case 0x145:
    GetDlgItem16(&ctx.PTR_LOOP_1050_1038,0x146);
    uVar2 = EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x1);
    uStack6 = 0x13f0647;
    uVar11 = 0x1f1;
//     TODO: goto LAB_1038_d490;
  case 0x146:
    uStack6 = 0x1400648;
    puVar5 = pass1_1008_941a(CONCAT22(unaff_SS,local_c),0x1,0xc4);
    puVar4 = (puVar5 >> 0x10);
    win_1008_5c9e(ctx.PTR__LOOP_1050_02a0,CONCAT22(unaff_SS,local_c),local_c,puVar4,
                  unaff_SS);
    uVar2 = 0x86;
    puVar5 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x9,unaff_SS,puVar4,unaff_DI);
    pass1_1010_6604(puVar5,uVar2,unaff_SS);
    GetDlgItem16(0x1010,0x145);
    EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x0);
    uVar9 = 0xc;
    uVar10 = 0x13f;
    WVar8 = 0x0;
    pcVar7 = load_string_1010_847e
                       (ctx.PTR__LOOP_1050_14cc,
                        (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
    LVar6 = SendDlgItemMessage16
                      (0x1010,pcVar7,(pcVar7 >> 0x10),WVar8,
                       CONCAT22(uVar10,uVar9));
    puVar4 = (LVar6 >> 0x10);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538,0x146);
    EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x0);
    iVar1 = pass1_1010_659a(puVar5,0x86,unaff_SS);
    if (iVar1 == 0x0) {
      GetDlgItem16(0x1010,0x14a);
      EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x0);
      uVar9 = 0xc;
      uVar10 = 0x144;
      WVar8 = 0x0;
      pcVar7 = load_string_1010_847e
                         (ctx.PTR__LOOP_1050_14cc,
                          (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
      LVar6 = SendDlgItemMessage16
                        (0x1010,pcVar7,(pcVar7 >> 0x10),WVar8,
                         CONCAT22(uVar10,uVar9));
      puVar4 = (LVar6 >> 0x10);
    }
    hwnd = 0x1010;
    puVar5 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2,unaff_SS,puVar4,unaff_DI);
    if ((puVar5 + 0x20) != 0x0) {
      hwnd = ctx.s_tile2_bmp_1050_1538;
      PostMessage16(0x1010,0x0,0x0,0x11100af);
    }
    break;
  case 0x147:
    GetDlgItem16(&ctx.PTR_LOOP_1050_1038,0x148);
    uVar2 = EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x1);
    uStack6 = 0x1410647;
    uVar11 = 0x1f5;
//     TODO: goto LAB_1038_d490;
  case 0x148:
    GetDlgItem16(&ctx.PTR_LOOP_1050_1038,0x149);
    uVar2 = EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x1);
    uStack6 = 0x1420647;
    uVar11 = 0x1f2;
//LAB_1038_d490:
    hwnd = 0x1008;
    win_1008_5c5c(unaff_SS,uVar2,in_DX,_PTR_LOOP_1050_02a0,uVar11);
    break;
  case 0x149:
    uStack6 = 0x1430648;
    PostMessage16(&ctx.PTR_LOOP_1050_1038,0x0,0x0,0x11100b8);
    hwnd = ctx.s_tile2_bmp_1050_1538;
    DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
    break;
  case 0x14a:
    GetDlgItem16(&ctx.PTR_LOOP_1050_1038,0x145);
    EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x1);
    uVar9 = 0xc;
    uVar10 = 0x140;
    WVar8 = 0x0;
    pcVar7 = load_string_1010_847e
                       (ctx.PTR__LOOP_1050_14cc,
                        (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
    hwnd = ctx.s_tile2_bmp_1050_1538;
    SendDlgItemMessage16
              (0x1010,pcVar7,(pcVar7 >> 0x10),WVar8,
               CONCAT22(uVar10,uVar9));
    break;
  case 0x14b:
    GetDlgItem16(&ctx.PTR_LOOP_1050_1038,0x147);
    hwnd = ctx.s_tile2_bmp_1050_1538;
    EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x1);
    break;
  default:
    post_win_msg_1040_7b3c
              (CONCAT22(param_2,CONCAT11(in_stack_00000005,param_1)),param_3,
               param_4,param_4._2_2_,&ctx.PTR_LOOP_1050_1040);
    return;
  }
  hwnd_00 = hwnd;
  if ((uStack6._2_2_ != 0x0) && (uStack6 != 0x0)) {
    hwnd_00 = ctx.s_tile2_bmp_1050_1538;
    BVar3 = IsWindow16(hwnd);
    if (BVar3 != 0x0) {
      WVar8 = 0x0;
      uVar9 = 0xc;
      pcVar7 = load_string_1010_847e
                         (ctx.PTR__LOOP_1050_14cc,
                          (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
      hwnd_00 = ctx.s_tile2_bmp_1050_1538;
      SendDlgItemMessage16
                (0x1010,pcVar7,(pcVar7 >> 0x10),WVar8,
                 CONCAT22(uStack6._2_2_,uVar9));
    }
  }
  if (iStack8 != 0x0) {
    PostMessage16(hwnd_00,0x0,0x0,CONCAT22(0x111,iStack8));
  }
  return;
}


pub fn post_win_msg_1038_d840(param_1: &mut Struct70,param_2: u16,param_3: HWND16)
{
  let iVar1: &mut Struct70;
  let uVar1: &mut Struct70;
  
  iVar1 = param_1;
  uVar1 = (param_1 >> 0x10);
  if (param_2 == 0x10) {
    if (iVar1.field_0x8e != 0x0) {
      PostMessage16(param_3,0x0,0x0,CONCAT22(0x111,iVar1.field_0x8e));
      iVar1.field_0x8e = 0x0;
      return;
    }
  }
  else {
    if (param_2 < 0x11) {
      if (param_2 == '\x01') {
        iVar1.field_0x90 = 0x0;
        iVar1.field_0x92 = 0x0;
        return;
      }
      if (param_2 == '\x03') {
        pass1_1038_e03e(param_1);
        return;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn destroy_crsor_1038_d8b2(param_1: i16,HINSTANCE16 param_2,param_3: u16)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let uVar3: u32;
  HCURSOR16 HVar4;
  let uVar5: u16;
  let rect: &mut Struct160;
  let in_DX: *mut u8
  let puVar6: *mut u8
  let iVar7: i16;
  let iVar8: i16;
  let unaff_DI: i16;
  let uVar9: u16;
  let puVar10: *mut u16;
  let puVar11: *mut u16;
  
  HVar4 = LoadCursor16(param_2,0x7f02);
  *(HCURSOR16 *)(param_1 + -0x2) = HVar4;
  HVar4 = SetCursor16(ctx.s_tile2_bmp_1050_1538);
  *(HCURSOR16 *)(param_1 + -0x4) = HVar4;
  dialog_ui_fn_1040_78e2(*(astruct_1 **)(param_1 + 0x6),&ctx.PTR_LOOP_1050_1040);
  uVar5 = pass1_1010_0886();
  (param_1 + -0x6) = uVar5;
  if (ctx.PTR__LOOP_1050_5f2c == 0x0) {
    ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(in_DX,0x1000);
    ctx.PTR_LOOP_1050_5f2e = in_DX;
  }
  else {
  }
  *(uchar **)(param_1 + -0x1c) = ctx.PTR_LOOP_1050_5f2c;
  *(uchar **)(param_1 + -0x1a) = ctx.PTR_LOOP_1050_5f2e;
  uVar5 = fn_ptr_op_1000_1708(((param_1 + -0x6) + 0x2) * 0x4,0x0,0x1,
                              ctx.PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e,0x1000);
  uVar2 = (param_1 + 0x6);
  uVar9 = (uVar2 >> 0x10);
  iVar7 = uVar2;
  (iVar7 + 0x96) = uVar5;
  *(uchar **)(iVar7 + 0x98) = ctx.PTR_LOOP_1050_5f2e;
  (param_1 + -0x8) = 0x1;
  while (iVar7 = (param_1 + -0x6), piVar1 = (param_1 + -0x8),
        *piVar1 == iVar7 || *piVar1 < iVar7) {
    uVar2 = (param_1 + 0x6);
    uVar2 = (uVar2 + 0x92);
    puVar10 =
              pass1_1010_08e2(uVar2,(uVar2 >> 0x10),
                              (param_1 + -0x8));
    puVar6 = (puVar10 >> 0x10);
    (param_1 + -0x1c) = puVar10;
    *(uchar **)(param_1 + -0x1a) = puVar6;
    (param_1 + -0x24) = *puVar10;
    (param_1 + -0x22) = (puVar10 + 0x2);
    (param_1 + -0x20) = 0x1;
    (param_1 + -0x1e) = 0x1;
    rect = (param_1 + -0x24);
    MapDialogRect16(0x1010,(RECT16 *)rect);
    mem_op_1000_179c(0x42,puVar6,0x1000);
    *(astruct_160 **)(param_1 + -0x28) = rect;
    *(uchar **)(param_1 + -0x26) = puVar6;
    ctx.PTR_LOOP_1050_5f2e = (puVar6 | rect);
    if (ctx.PTR_LOOP_1050_5f2e == 0x0) {
      uVar2 = (param_1 + 0x6);
      uVar2 = (uVar2 + 0x96);
      (uVar2 + (param_1 + -0x8) * 0x4) = 0x0;
    }
    else {
      uVar2 = (param_1 + 0x6);
      uVar3 = (param_1 + -0x1c);
      pass1_1008_3bd6(rect,(param_1 + -0x26),0x0,
                      CONCAT22((param_1 + -0x24),
                               (param_1 + -0x22)),0x101,0xff0100,
                      CONCAT22((uVar2 + 0x6),
                               (uVar3 + 0x4)),
                      ctx.PTR_LOOP_1050_5f2e,param_3);
      uVar2 = (param_1 + 0x6);
      uVar2 = (uVar2 + 0x96);
      uVar9 = (uVar2 >> 0x10);
      iVar7 = uVar2;
      iVar8 = (param_1 + -0x8) * 0x4;
      *(astruct_160 **)(iVar7 + iVar8) = rect;
      *(uchar **)(iVar7 + iVar8 + 0x2) = ctx.PTR_LOOP_1050_5f2e;
    }
    uVar2 = (param_1 + 0x6);
    uVar2 = (uVar2 + 0x96);
    uVar9 = (uVar2 >> 0x10);
    iVar7 = uVar2;
    iVar8 = (param_1 + -0x8) * 0x4;
    if ((iVar7 + iVar8) != 0x0) {
      uVar2 = (iVar7 + iVar8);
      (uVar2 + 0x3e) = 0x1;
      uVar2 = (param_1 + -0x1c);
      uVar3 = (param_1 + 0x6);
      uVar3 = (uVar3 + 0x96);
      enable_win_1040_9234
                ((uVar3 + (param_1 + -0x8) * 0x4),
                 *(bool *)(uVar2 + 0x6),&ctx.PTR_LOOP_1050_1040);
    }
    piVar1 = (param_1 + -0x8);
    *piVar1 = *piVar1 + 0x1;
  }
  puVar11 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2,param_3,PTR_LOOP_1050_5f2e,unaff_DI);
  (param_1 + -0xc) = puVar11;
  (param_1 + -0xa) = (puVar11 >> 0x10);
  uVar2 = (param_1 + -0xc);
  SetWindowText16(0x1010,(SEGPTR)(uVar2 + 0x68));
  ShowWindow16(ctx.s_tile2_bmp_1050_1538,0x5);
  SetCursor16(ctx.s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
unk_win_sys_op_1038_da68
          (param_1: i16,param_2: u16,param_3: u16,param_4: u32,WNDCLASS16 *param_5,
          uchar *param_6)

{
  let ppcVar1: u32;
  let uVar2: u16;
  let puVar3: *mut u8
  let extraout_DX: *mut u8
  let in_BX: u16;
  let unaff_DI: i16;
  let unaff_CS: u16;
  let uVar4: u16;
  let uVar5: u32;
  let uVar6: u32;
  let iVar7: i16;
  let local_16: [u8;4];
  let uStack18: u16;
  let puStack16: *mut u8
  let paStack14: &mut Struct43;
  let uStack10: u16;
  let uStack8: u16;
  let iStack6: i16;
  let iStack4: i16;
  
  if (param_3 == 0x204) {
    pass1_1038_de20(CONCAT22(param_2,param_1),0x204,param_4,param_4._2_2_,param_6,
                    in_BX,param_5);
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
        if ((param_1 + 0x8e) == 0x0) {
          pass1_1010_1ea6(ctx.PTR__LOOP_1050_02a0,CONCAT22(param_2,param_1),
                          param_5);
          (param_1 + 0x90) = 0x0;
        }
        iStack4 = 0x72c;
        uStack8 = 0x48;
      }
      else {
        if (param_4._2_2_ - 0x11c == 0x0) {
          param_4._2_2_ = param_4._2_2_ - 0x11c;
          pass1_1038_df86(CONCAT22(param_2,param_1),param_6,unaff_DI,param_5);
        }
        else {
          if (param_4._2_2_ != 0x11d) {
            if (param_4._2_2_ == 0x11e) {
              iVar7 = 0x1d;
            }
            else {
              if (param_4._2_2_ != 0x120) {
//LAB_1038_dc20:
                post_win_msg_1040_7b3c
                          (CONCAT22(param_2,param_1),param_3,param_4,
                           param_4._2_2_,&ctx.PTR_LOOP_1050_1040);
                return;
              }
              iVar7 = 0x1c;
            }
//             TODO: goto LAB_1038_db1c;
          }
          uVar5 = pass1_1038_df5c(CONCAT22(param_2,param_1),param_6,param_5);
          param_6 = (uVar5 >> 0x10);
          param_4._2_2_ = uVar5;
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
            ppcVar1 = (*_PTR_LOOP_1050_02a0 + 0x4);
            param_4._2_2_ = param_4._2_2_ - 0x125;
            (**ppcVar1)();
            (param_1 + 0x90) = 0x1;
            param_6 = extraout_DX;
            win_1008_5c5c(param_5,param_4._2_2_,extraout_DX,
                          ctx._PTR_LOOP_1050_02a0,0x1db);
            (param_1 + 0x8e) = 0x100;
          }
          else {
            if (param_4._2_2_ == 0x126) {
              (param_1 + 0x8e) = 0x0;
              win_1008_5c7c(ctx.PTR__LOOP_1050_02a0,0xcb0001,param_5,0x0,
                            param_6);
              paStack14 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc,0x1f8,param_5);
              param_6 = (paStack14 >> 0x10);
              param_4._2_2_ =
                   WinHelp16(0x1010,0x0,0x0,CONCAT22(paStack14,0x3));
            }
            else {
              if (param_4._2_2_ - 0x127 != 0x0) goto LAB_1038_dc20;
              param_4._2_2_ = param_4._2_2_ - 0x127;
              post_win_msg_1038_dcb0(CONCAT22(param_2,param_1),0x0,param_6,param_5);
            }
          }
//           TODO: goto LAB_1038_dac3;
        }
        iVar7 = 0x28;
      }
//LAB_1038_db1c:
      uVar6 = pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c,(param_1 + 0x8),iVar7,
                              param_6,param_1,unaff_CS,param_5);
      param_6 = (uVar6 >> 0x10);
      param_4._2_2_ = uVar6;
    }
  }
//LAB_1038_dac3:
  if (iStack4 == 0x0) {
    return;
  }
  if (iStack6 == 0x0) {
    mem_op_1000_179c(0xb4,param_6,0x1000);
    puVar3 = (param_6 | param_4._2_2_);
    uStack18 = param_4._2_2_;
    puStack16 = param_6;
    if (puVar3 != 0x0) {
      uVar4 = SUB42(&ctx.PTR_LOOP_1050_1040,0x0);
      iVar7 = string_1040_8520(
                               CONCAT13((param_6 >> 0x8),
                                        CONCAT12(param_6,param_4._2_2_)),
                               (param_1 + 0x6),0x0,0x2,0x634,iStack4,puVar3,
                               param_5);
//       TODO: goto LAB_1038_dc37;
    }
  }
  else {
    mem_op_1000_179c(0xb4,param_6,0x1000);
    puVar3 = (param_6 | param_4._2_2_);
    uStack18 = param_4._2_2_;
    puStack16 = param_6;
    if (puVar3 != 0x0) {
      uVar4 = SUB42(&ctx.PTR_LOOP_1050_1040,0x0);
      iVar7 = string_1040_8520(CONCAT22(param_6,param_4._2_2_),
                               (param_1 + 0x6),0x0,0x3,0x634,iStack4,puVar3,
                               param_5);
//       TODO: goto LAB_1038_dc37;
    }
  }
  uVar4 = 0x1000;
  iVar7 = 0x0;
  puVar3 = 0x0;
//LAB_1038_dc37:
  paStack14 = CONCAT22(puVar3,iVar7);
  if (uStack8 == 0x0) {
    ppcVar1 = (paStack14 + 0x74);
    (**ppcVar1)(uVar4,iVar7,puVar3);
  }
  else {
    pass1_1008_941a(CONCAT22(param_5,local_16),0x1,uStack8);
    ppcVar1 = (paStack14 + 0x6c);
    (**ppcVar1)(0x1008,paStack14,(paStack14 >> 0x10),local_16,param_5);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn post_win_msg_1038_dcb0(param_1: u32,param_2: u16,uchar *param_3,param_4: u16)
{
  let ppcVar1: u32;
  let iVar2: i16;
  let iVar3: i16;
  let puVar4: *mut u8
  let extraout_DX: *mut u8
  let unaff_DI: i16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u8;
  let uVar10: u8;
  let local_18: u32;
  let local_14: [u8;4];
  let uStack16: u32;
  let iStack12: i16;
  let local_a: [u8;4];
  let puStack6: u32;
  
  mem_op_1000_179c(0xb4,param_3,0x1000);
  puVar4 = (param_3 | param_2);
  iVar3 = param_1;
  uVar5 = (param_1 >> 0x10);
  uStack16._0_2_ = param_2;
  uStack16._2_2_ = param_3;
  if (puVar4 == 0x0) {
    iVar2 = 0x0;
    puVar4 = 0x0;
  }
  else {
    iVar2 = string_1040_8520(CONCAT22(param_3,param_2),
                             (iVar3 + 0x6),0x4,0x3,0x634,0x726,puVar4,param_4);
  }
  puStack6 = CONCAT22(puVar4,iVar2);
  pass1_1008_941a(CONCAT22(param_4,local_a),0x1,0x49);
  ppcVar1 = (*puStack6 + 0x6c);
  uStack16 = 
             (**ppcVar1)(0x1008,puStack6,(puStack6 >> 0x10),local_a,
                         param_4);
  puVar4 = (uStack16 >> 0x10);
  iStack12 = uStack16;
  if (iStack12 == 0x6) {
    mem_op_1000_179c(0xb4,puVar4,0x1000);
    puVar4 = ((uStack16 >> 0x10) | uStack16);
    if (uStack16 == 0x0) {
      iVar3 = 0x0;
      puVar4 = 0x0;
    }
    else {
      iVar3 = string_1040_8520(uStack16,(iVar3 + 0x6),0x0,0x2,0x634,0x728,
                               puVar4,param_4);
    }
    puStack6 = CONCAT22(puVar4,iVar3);
    pass1_1008_941a(CONCAT22(param_4,local_14),0x1,0x4a);
    ppcVar1 = (*puStack6 + 0x6c);
    (**ppcVar1)(0x1008,puStack6,(puStack6 >> 0x10),local_14);
    uVar9 = 0x0;
    uVar10 = 0x0;
    iVar2 = 0x15;
    uVar7 = 0x1;
    uVar8 = 0x0;
    uVar6 = 0x0;
    iVar3 = 0x0;
    uVar5 = 0x0;
    local_18 = 
               mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x37,param_4,extraout_DX,unaff_DI);
    post_win_msg_1008_a0e4
              (local_18,CONCAT22(uVar6,uVar5),iVar3,uVar7,
               CONCAT13(uVar10,CONCAT12(uVar9,uVar8)),iVar2,0x1008,param_4);
    PostMessage16(0x1008,0x0,0x0,0x11100fc);
    return;
  }
  mem_op_1000_179c(0xb4,puVar4,0x1000);
  puVar4 = ((uStack16 >> 0x10) | uStack16);
  if (uStack16 == 0x0) {
    iVar3 = 0x0;
    puVar4 = 0x0;
  }
  else {
    iVar3 = string_1040_8520(uStack16,(iVar3 + 0x6),0x0,0x2,0x634,0x729,puVar4,
                             param_4);
  }
  puStack6 = CONCAT22(puVar4,iVar3);
  pass1_1008_941a(CONCAT22(param_4,&local_18),0x1,0x4b);
  ppcVar1 = (*puStack6 + 0x6c);
  (**ppcVar1)(0x1008,puStack6,(puStack6 >> 0x10),&local_18);
  return;
}


pub fn check_radio_btn_show_win_1038_e19a(param_1: &mut Struct1)
{
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  CheckRadioButton16(&ctx.PTR_LOOP_1050_1040,0x1807,0x1807,0x1807);
  move_win_1040_826c(param_1,0xc8,0xc8);
  ShowWindow16(&ctx.PTR_LOOP_1050_1040,0x5);
  return;
}


pub fn win_ui_op_1038_e348(param_1: &mut Struct1)
{
  let uVar1: u32;
  let uVar2: u16;
  let rect: &mut Struct160;
  let in_DX: *mut u8
  let puVar3: *mut u8
  let uVar4: u16;
  let iVar5: i16;
  let iVar6: i16;
  let unaff_DI: i16;
  let uVar7: u16;
  let uVar8: u16;
  let unaff_SS: u16;
  let local_22: u16;
  let uStack32: u16;
  let uStack30: u16;
  let uStack28: u16;
  let puStack26: *mut u16;
  let uStack10: i16;
  let uStack8: u16;
  let puStack6: *mut u16;
  
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  puStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2b,unaff_SS,in_DX,unaff_DI);
  ctx.PTR_LOOP_1050_5f2e = (puStack6 >> 0x10);
  uStack8 = pass1_1010_088c();
  if (ctx.PTR__LOOP_1050_5f2c == 0x0) {
    ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
  }
  puStack26 = CONCAT22(ctx.PTR_LOOP_1050_5f2e,PTR_LOOP_1050_5f2c);
  uVar2 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4,0x0,0x1,PTR_LOOP_1050_5f2c,
                              ctx.PTR_LOOP_1050_5f2e,0x1000);
  uVar7 = (param_1 >> 0x10);
  iVar5 = param_1;
  (iVar5 + 0x8e) = uVar2;
  (iVar5 + 0x90) = ctx.PTR_LOOP_1050_5f2e;
  for (iStack10 = 0x1; iStack10 <= uStack8; iStack10 += 0x1) {
    puStack26 =
                pass1_1010_091e(puStack6,(puStack6 >> 0x10),
                                iStack10);
    puVar3 = (puStack26 >> 0x10);
    local_22 = *puStack26;
    uStack32 = (puStack26 + 0x2);
    uStack30 = 0x1;
    uStack28 = 0x1;
    rect = &local_22;
    MapDialogRect16(0x1010,(RECT16 *)rect);
    mem_op_1000_179c(0x42,puVar3,0x1000);
    uVar4 = puVar3 | rect;
    if (uVar4 == 0x0) {
      uVar1 = (iVar5 + 0x8e);
      (uVar1 + iStack10 * 0x4) = 0x0;
    }
    else {
      pass1_1008_3bd6(rect,puVar3,0x0,CONCAT22(local_22,uStack32),0x101,0xff0100,
                      CONCAT22((iVar5 + 0x6),
                               (puStack26 + 0x4)),uVar4,unaff_SS);
      uVar1 = (iVar5 + 0x8e);
      uVar8 = (uVar1 >> 0x10);
      iVar6 = uVar1;
      *(astruct_160 **)(iVar6 + iStack10 * 0x4) = rect;
      (iVar6 + iStack10 * 0x4 + 0x2) = uVar4;
    }
    uVar1 = (iVar5 + 0x8e);
    uVar8 = (uVar1 >> 0x10);
    iVar6 = uVar1;
    if ((iVar6 + iStack10 * 0x4) != 0x0) {
      enable_win_1040_9234
                ((iVar6 + iStack10 * 0x4),*(bool *)(puStack26 + 0x6),
                 &ctx.PTR_LOOP_1050_1040);
    }
  }
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(&ctx.PTR_LOOP_1050_1040,0x5);
  return;
}


pub fn unk_win_ui_op_1038_e71c(param_1: &mut Struct1,param_2: u16)
{
  let extraout_DX: u16;
  let iVar1: i16;
  let uVar2: u16;
  let unaff_SS: u16;
  let paStack6: &mut Struct18;
  
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  unk_load_str_op_1010_2c34((iVar1 + 0x8e));
  paStack6 = CONCAT22(extraout_DX,param_2);
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | (iVar1 + 0x10)),
             CONCAT22(extraout_DX,param_2));
  fn_ptr_1000_17ce(paStack6,0x1000);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(&ctx.PTR_LOOP_1050_1040,0x5);
  (iVar1 + 0x92) = 0x1;
  unk_win_msg_op_1008_9510
            ((param_1 & 0xffff0000 | (iVar1 + 0x92)),0x1008,unaff_SS)
  ;
  DestroyWindow16(0x1008);
  return;
}



pub fn chk_is_dlg_btn_checked_1038_e7a0(param_1: u32,param_2: i16)
{
  let uVar1: u32;
  let UVar2: u16;
  let iVar3: &mut Struct62;
  let uVar3: u16;
  let unaff_CS: HWND16;
  
  iVar3 = param_1;
  uVar3 = (param_1 >> 0x10);
  if (param_2 == 0x0) {
    uVar1 = iVar3.field_0x8e;
    (uVar1 + 0x10) = 0x1;
    uVar1 = iVar3.field_0x8e;
    (uVar1 + 0xa) = 0x0;
    uVar1 = iVar3.field_0x8e;
    (uVar1 + 0xc) = 0x0;
    uVar1 = iVar3.field_0x8e;
    (uVar1 + 0xe) = 0x0;
  }
  else {
    UVar2 = IsDlgButtonChecked(unaff_CS,0x1827);
    if (UVar2 == 0x0) {
      UVar2 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538,0x1828);
      if (UVar2 == 0x0) {
        uVar1 = iVar3.field_0x8e;
        (uVar1 + 0xa) = 0x0;
      }
      else {
        uVar1 = iVar3.field_0x8e;
        (uVar1 + 0xa) = 0x2;
      }
    }
    else {
      uVar1 = iVar3.field_0x8e;
      (uVar1 + 0xa) = 0x1;
    }
    UVar2 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538,s_vrpal_bmp_1050_183a
                              );
    if (UVar2 == 0x0) {
      UVar2 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538,
                                 (s_vrpal_bmp_1050_183a + 0x1));
      if (UVar2 == 0x0) {
        uVar1 = iVar3.field_0x8e;
        (uVar1 + 0xc) = 0x0;
      }
      else {
        uVar1 = iVar3.field_0x8e;
        (uVar1 + 0xc) = 0x2;
      }
    }
    else {
      uVar1 = iVar3.field_0x8e;
      (uVar1 + 0xc) = 0x1;
    }
    UVar2 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538,
                               (s_vrpal_bmp_1050_183a + 0x2));
    if (UVar2 == 0x0) {
      UVar2 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538,
                                 (s_vrpal_bmp_1050_183a + 0x3));
      if (UVar2 == 0x0) {
        uVar1 = iVar3.field_0x8e;
        (uVar1 + 0xe) = 0x0;
      }
      else {
        uVar1 = iVar3.field_0x8e;
        (uVar1 + 0xe) = 0x2;
      }
    }
    else {
      uVar1 = iVar3.field_0x8e;
      (uVar1 + 0xe) = 0x1;
    }
    uVar1 = iVar3.field_0x8e;
    (uVar1 + 0x10) = 0x0;
  }
  iVar3.field_0x92 = 0x0;
  return;
}


pub fn win_ui_op_1038_ea18(param_1: &mut Struct1)
{
  let IVar1: i16;
  let BVar2: bool;
  RECT16 local_10 [0x2];
  let HStack8: HWND16;
  let uStack6: u32;
  
  dialog_ui_fn_1040_78e2(param_1,&ctx.PTR_LOOP_1050_1040);
  uStack6 = pass1_1010_375e((param_1 + 0x8e));
  HStack8 = GetDlgItem16(0x1010,0xfa5);
  SendMessage16(ctx.s_tile2_bmp_1050_1538,uStack6,(WPARAM16)(uStack6 >> 0x10),
                0xc0000);
  GetWindowRect16(ctx.s_tile2_bmp_1050_1538,local_10);
  BVar2 = 0x4;
  IVar1 = GetSystemMetrics16((INT16)s_tile2_bmp_1050_1538);
  move_win_1040_826c(param_1,IVar1 + local_10[0].y + 0x5,BVar2);
  ShowWindow16(&ctx.PTR_LOOP_1050_1040,0x5);
  return;
}



pub fn win_ui_op_1038_eaa2(param_1: u32,param_2: i16,param_3: HWND16,WPARAM16 param_4)
{
  LRESULT LVar1;
  let local_54: [u8;52];
  
  if (param_2 != 0x0) {
    GetDlgItem16(param_3,0xfa5);
    LVar1 = SendMessage16(ctx.s_tile2_bmp_1050_1538,local_54,param_4,0xd0050);
    pass1_1010_3770((param_1 + 0x8e),CONCAT22(param_4,local_54),
                    (LVar1 >> 0x10));
    param_3 = ctx.s_tile2_bmp_1050_1538;
    PostMessage16(0x1010,0x0,0x0,0x11100fb);
  }
  DestroyWindow16(param_3);
  return;
}


pub fn win_ui_op_1038_ec1a(param_1: u16,param_2: i16)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let uVar3: u32;
  let uVar4: u16;
  let rect: &mut Struct160;
  let in_DX: *mut u8
  let puVar5: *mut u8
  let uVar6: u16;
  let iVar7: i16;
  let iVar8: i16;
  let unaff_DI: i16;
  let uVar9: u16;
  let puVar10: *mut u16;
  let puVar11: *mut u16;
  
  dialog_ui_fn_1040_78e2(*(astruct_1 **)(param_2 + 0x6),&ctx.PTR_LOOP_1050_1040);
  puVar10 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2b,param_1,in_DX,unaff_DI);
  ctx.PTR_LOOP_1050_5f2e = (puVar10 >> 0x10);
  (param_2 + -0x4) = puVar10;
  (param_2 + -0x2) = ctx.PTR_LOOP_1050_5f2e;
  uVar4 = pass1_1010_0892();
  (param_2 + -0x6) = uVar4;
  if (ctx.PTR__LOOP_1050_5f2c == 0x0) {
    ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
  }
  *(uchar **)(param_2 + -0x18) = ctx.PTR_LOOP_1050_5f2c;
  (param_2 + -0x16) = ctx.PTR_LOOP_1050_5f2e;
  uVar4 = fn_ptr_op_1000_1708(((param_2 + -0x6) + 0x2) * 0x4,0x0,0x1,
                              ctx.PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e,0x1000);
  uVar2 = (param_2 + 0x6);
  uVar9 = (uVar2 >> 0x10);
  iVar7 = uVar2;
  (iVar7 + 0x8e) = uVar4;
  (iVar7 + 0x90) = ctx.PTR_LOOP_1050_5f2e;
  (param_2 + -0x8) = 0x1;
  while (iVar7 = (param_2 + -0x6), piVar1 = (param_2 + -0x8),
        *piVar1 == iVar7 || *piVar1 < iVar7) {
    uVar2 = (param_2 + -0x4);
    puVar11 =
              pass1_1010_0932(uVar2,(uVar2 >> 0x10),
                              (param_2 + -0x8));
    puVar5 = (puVar11 >> 0x10);
    (param_2 + -0x18) = puVar11;
    *(uchar **)(param_2 + -0x16) = puVar5;
    (param_2 + -0x20) = *puVar11;
    (param_2 + -0x1e) = (puVar11 + 0x2);
    (param_2 + -0x1c) = 0x1;
    (param_2 + -0x1a) = 0x1;
    rect = (param_2 + -0x20);
    MapDialogRect16(0x1010,(RECT16 *)rect);
    mem_op_1000_179c(0x42,puVar5,0x1000);
    *(astruct_160 **)(param_2 + -0x24) = rect;
    *(uchar **)(param_2 + -0x22) = puVar5;
    uVar6 = puVar5 | rect;
    if (uVar6 == 0x0) {
      uVar2 = (param_2 + 0x6);
      uVar2 = (uVar2 + 0x8e);
      (uVar2 + (param_2 + -0x8) * 0x4) = 0x0;
    }
    else {
      uVar2 = (param_2 + 0x6);
      uVar3 = (param_2 + -0x18);
      pass1_1008_3bd6(rect,(param_2 + -0x22),0x0,
                      CONCAT22((param_2 + -0x20),
                               (param_2 + -0x1e)),0x101,0xff0100,
                      CONCAT22((uVar2 + 0x6),
                               (uVar3 + 0x4)),uVar6,param_1);
      uVar2 = (param_2 + 0x6);
      uVar2 = (uVar2 + 0x8e);
      uVar9 = (uVar2 >> 0x10);
      iVar7 = uVar2;
      iVar8 = (param_2 + -0x8) * 0x4;
      *(astruct_160 **)(iVar7 + iVar8) = rect;
      (iVar7 + iVar8 + 0x2) = uVar6;
    }
    uVar2 = (param_2 + 0x6);
    uVar2 = (uVar2 + 0x8e);
    uVar9 = (uVar2 >> 0x10);
    iVar7 = uVar2;
    iVar8 = (param_2 + -0x8) * 0x4;
    if ((iVar7 + iVar8) != 0x0) {
      uVar2 = (param_2 + -0x18);
      enable_win_1040_9234
                ((iVar7 + iVar8),*(bool *)(uVar2 + 0x6),
                 &ctx.PTR_LOOP_1050_1040);
    }
    piVar1 = (param_2 + -0x8);
    *piVar1 = *piVar1 + 0x1;
  }
  move_win_1040_826c(*(astruct_1 **)(param_2 + 0x6),-0x1,0xffff);
  ShowWindow16(&ctx.PTR_LOOP_1050_1040,0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn send_msg_1038_ed8a(param_1: u16,param_2: u32,param_3: u32,param_4: HWND16)
{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let in_DX: *mut u8
  let uVar5: u16;
  let puVar6: *mut u8
  let unaff_DI: i16;
  let unaff_SS: u16;
  let puVar7: *mut u16;
  let uVar8: u32;
  
  if (param_3._2_2_ != 0x1c8) {
    if (param_3._2_2_ == 0x1c9) {
      puVar7 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2f,unaff_SS,in_DX,unaff_DI);
      uVar2 = (puVar7 >> 0x10);
      uVar1 = (puVar7 + 0x20);
      uVar5 = (puVar7 + 0x22);
      uVar3 = uVar5 | uVar1;
      if (uVar3 == 0x0) {
        return;
      }
      pass1_1028_e1ec(ctx.PTR__LOOP_1050_65e2,uVar1,uVar5);
      puVar6 = (uVar5 | uVar3);
      if (puVar6 == 0x0) {
        return;
      }
      uVar4 = pass1_1030_5b00(CONCAT22(uVar5,uVar3));
      puVar7 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,uVar4,unaff_SS,puVar6,unaff_DI);
      if (puVar7 == 0x0) {
        return;
      }
      param_4 = 0x1018;
      uVar8 = pass1_1018_0ad4(puVar7);
      if (uVar8 == 0x0) {
        return;
      }
      param_3._2_2_ = 0x72;
    }
    else {
      if (param_3._2_2_ != 0x1ca) {
        post_win_msg_1040_7b3c
                  (CONCAT22(param_2,param_1),(param_2 >> 0x10),
                   param_3,param_3._2_2_,&ctx.PTR_LOOP_1050_1040);
        return;
      }
    }
  }
  SendMessage16(param_4,0x0,0x0,CONCAT22(0x111,param_3._2_2_));
  return;
}

pub fn win_ui_op_1040_0000(param_1: &mut Struct1,uchar *param_2,param_3: HWND16)
{
  let rect: &mut Struct160;
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let unaff_DI: i16;
  let uVar6: u16;
  WNDCLASS16 *unaff_SS;
  LRESULT LVar7;
  let uVar8: u32;
  let local_24: u16;
  let uStack34: u16;
  let uStack32: u16;
  let uStack30: u16;
  let iStack28: i16;
  RECT16 local_1a;
  let iStack22: i16;
  let uStack18: u32;
  let uStack14: u16;
  let iStack12: i16;
  let uStack10: i16;
  let paStack8: &mut Struct160;
  let uStack6: u16;
  let iStack4: i16;
  
                    // Segment:    9
                    // Offset:     0006f820
                    // Length:     d974
                    // Min Alloc:  d974
                    // Flags:      0d50
                    //     Code
                    //     Moveable
                    //     Preload
                    //     Impure (Non-shareable)
                    // 
  dialog_ui_fn_1040_78e2(param_1,param_3);
  iStack4 = 0x8;
  for (iStack10 = 0x0; uVar5 = param_1,
      uVar6 = (param_1 >> 0x10), iStack10 < iStack4; iStack10 += 0x1) {
    unaff_DI = iStack10 * 0xe;
    local_24 = (unaff_DI + 0x5c60);
    uStack34 = (unaff_DI + 0x5c62);
    uStack32 = 0x1;
    uStack30 = 0x1;
    rect = &local_24;
    MapDialogRect16(param_3,(RECT16 *)rect);
    param_3 = 0x1000;
    mem_op_1000_179c(0x42,param_2,0x1000);
    uVar1 = param_2 | rect;
    if (uVar1 == 0x0) {
      rect = 0x0;
      uVar1 = 0x0;
    }
    else {
      param_3 = 0x1008;
      pass1_1008_3bd6(rect,param_2,0x1,CONCAT22(local_24,uStack34),0x104,0x1020103
                      ,CONCAT22((uVar5 + 0x6),
                                (unaff_DI + 0x5c64)),uVar1,unaff_SS
                     );
    }
    paStack8 = rect;
    uStack6 = uVar1;
    LVar7 = win_ui_op_1040_0558(param_1,iStack10,param_3);
    param_2 = (LVar7 >> 0x10);
  }
  move_win_1040_826c(param_1,-0x1,0xffff);
  uStack18 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x48,unaff_SS,param_2,unaff_DI);
  uVar2 = (uStack18 >> 0x10);
  iStack12 = (uStack18 + 0xa);
  uStack14 = (uStack18 + 0xc);
  GetWindowRect16(0x1010,&local_1a);
  uVar3 = iStack12 >> 0xf;
  iStack28 = iStack22 - local_1a.x;
  local_1a.x = (iStack12 / 0x2 - iStack28) + -0x3;
  if (local_1a.x < 0x0) {
    local_1a.x = 0x0;
  }
  SetWindowPos16(ctx.s_tile2_bmp_1050_1538,0x41,0x0,0x0,local_1a.y,local_1a.x,0x0);
  uVar8 = pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c,(uVar5 + 0x6),0x17,uVar3,uVar5,
                          &ctx.PTR_LOOP_1050_1038,unaff_SS);
  uVar4 = (uVar8 >> 0x10);
  uVar3 = uVar8;
  (uVar5 + 0x96) = uVar3;
  (uVar5 + 0x98) = uVar4;
  win_1008_5c7c(ctx.PTR__LOOP_1050_02a0,0x9e0001,unaff_SS,uVar3,uVar4);
  (uVar5 + 0x8c) = uVar3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
win_ui_op_1040_0170(param_1: i16,param_2: u16,param_3: u16,param_4: u32,param_5: u16
                   ,WNDCLASS16 *param_6)

{
  let uVar1: u16;
  let BVar2: bool;
  let iVar3: i16;
  let in_DX: *mut u8
  let iVar4: i16;
  let extraout_DX: *mut u8
  let unaff_DI: i16;
  let uVar5: u16;
  ulet in_AF: u8;
  let mut pcVar6: String; 
  LRESULT LVar7;
  WPARAM16 w_param;
  let uVar8: u16;
  HCURSOR16 *pHVar9;
  WNDCLASS16 *pWVar10;
  HCURSOR16 *pHVar11;
  WNDCLASS16 *pWVar12;
  u32 *local_12a [0x43];
  let puStack30: *mut u16;
  let uStack26: u16;
  HCURSOR16 local_18;
  HCURSOR16 local_16;
  let uStack20: u32;
  let iStack16: i16;
  let iStack14: i16;
  let puStack12: *mut u16;
  let iStack8: i16;
  let iStack6: i16;
  let iStack4: i16;
  
  iStack4 = 0x8;
  iStack6 = 0x0;
  switch(param_4._2_2_) {
  case 0x167:
    enable_win_1040_060e
              (CONCAT22(param_2,param_1),0x3,&ctx.PTR_LOOP_1050_1040,param_6);
    GetDlgItem16(&ctx.PTR_LOOP_1050_1040,0x16b);
    EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x1);
    iStack4 = 0x0;
    break;
  case 0x168:
    enable_win_1040_060e
              (CONCAT22(param_2,param_1),0x3,&ctx.PTR_LOOP_1050_1040,param_6);
    GetDlgItem16(&ctx.PTR_LOOP_1050_1040,0x16b);
    EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x1);
    iStack4 = 0x1;
    break;
  case 0x169:
    enable_win_1040_060e
              (CONCAT22(param_2,param_1),0x3,&ctx.PTR_LOOP_1050_1040,param_6);
    GetDlgItem16(&ctx.PTR_LOOP_1050_1040,0x16b);
    EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x1);
    iStack4 = 0x2;
    break;
  case 0x16a:
    enable_win_1040_060e
              (CONCAT22(param_2,param_1),0x3,&ctx.PTR_LOOP_1050_1040,param_6);
    GetDlgItem16(&ctx.PTR_LOOP_1050_1040,0x16b);
    EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x1);
    iStack4 = 0x3;
    break;
  case 0x16b:
    GetDlgItem16(&ctx.PTR_LOOP_1050_1040,0x16b);
    uVar5 = SUB42(ctx.s_tile2_bmp_1050_1538,0x0);
    BVar2 = EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x0);
    if ((param_1 + 0x92) != 0x3) {
      uVar5 = 0x1008;
      win_1008_5c5c(param_6,BVar2,in_DX,_PTR_LOOP_1050_02a0,0x1de);
    }
    if ((param_1 + 0x92) != 0x8) {
      iVar3 = (param_1 + 0x92) * 0xe;
      iStack6 = (iVar3 + 0x5c6c);
      uVar5 = 0x1010;
      pass1_1010_6604((param_1 + 0x8e),(iVar3 + 0x5c66),
                      param_6);
      (param_1 + 0x92) = 0x8;
    }
    for (iStack8 = 0x0; iStack8 < 0x4; iStack8 += 0x1) {
      LVar7 = win_ui_op_1040_0558(CONCAT22(param_2,param_1),iStack8,uVar5);
      in_DX = (LVar7 >> 0x10);
    }
//     TODO: goto LAB_1040_04da;
  case 0x16c:
    GetDlgItem16(&ctx.PTR_LOOP_1050_1040,0x16d);
    EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x1);
    iStack4 = 0x5;
    (param_1 + 0x94) = 0x5;
//     TODO: goto LAB_1040_04da;
  case 0x16d:
    GetDlgItem16(&ctx.PTR_LOOP_1050_1040,0x16d);
    BVar2 = EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x0);
    uVar5 = 0x1008;
    win_1008_5c5c(param_6,BVar2,in_DX,_PTR_LOOP_1050_02a0,0x1de);
    if ((param_1 + 0x94) != 0x8) {
      iVar3 = (param_1 + 0x94) * 0xe;
      iStack6 = (iVar3 + 0x5c6c);
      uVar5 = 0x1010;
      pass1_1010_6604((param_1 + 0x8e),(iVar3 + 0x5c66),
                      param_6);
      (param_1 + 0x94) = 0x8;
    }
    LVar7 = win_ui_op_1040_0558(CONCAT22(param_2,param_1),0x5,uVar5);
    puStack12 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x39,param_6,
                                (LVar7 >> 0x10),unaff_DI);
    iVar3 = (puStack12 + 0x20);
    pHVar11 = &local_16;
    pHVar9 = &local_18;
    iVar4 = (iVar3 >> 0xf) + 0x200;
    pWVar10 = param_6;
    pWVar12 = param_6;
    iStack16 = iVar3;
    iStack14 = iVar4;
    iStack8 = iVar3;
    pass1_1030_8344(ctx.PTR__LOOP_1050_5748,
                    (ctx.PTR__LOOP_1050_5748 >> 0x10),CONCAT22(iVar4,iVar3));
    uStack20 = CONCAT22(iVar4,iVar3);
    pass1_1030_2f1a(CONCAT22(iVar4,iVar3),CONCAT22(pWVar10,pHVar9),
                    CONCAT22(pWVar12,pHVar11));
    in_DX = ((local_18 - local_16) >> 0xf);
    local_16 += (local_18 - local_16) / 0x2;
    uStack26 = pass1_1030_2fac(uStack20);
    set_window_text_1018_6086(*(ULONG *)(param_1 + 0x96),0x1018,param_6);
//     TODO: goto LAB_1040_04da;
  case 0x16e:
    puStack30 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x39,param_6,in_DX,unaff_DI);
    uStack26 = (puStack30 + 0x20);
    local_18 = LoadCursor16(0x1010,0x7f02);
    local_16 = SetCursor16(ctx.s_tile2_bmp_1050_1538);
    pass1_1030_532e(CONCAT22(param_6,local_12a),
                    uStack26 + 0x2000000,param_6,in_AF);
    fn_ptr_1030_835a(ctx.PTR__LOOP_1050_5748,CONCAT22(param_6,local_12a));
    pass1_1030_838e(ctx.PTR__LOOP_1050_5748,param_6,in_AF);
    pass1_1030_8334(ctx.PTR__LOOP_1050_5748,(ctx.PTR__LOOP_1050_5748 >> 0x10));
    in_DX = extraout_DX;
    SetCursor16(0x1030);
    PostMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x111007e);
    DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
    local_12a[0] = &ULONG_1008_389a;
//     TODO: goto LAB_1040_04da;
  default:
    post_win_msg_1040_7b3c
              (CONCAT22(param_2,param_1),param_3,param_4,param_4._2_2_,
               param_5);
    return;
  }
  (param_1 + 0x92) = iStack4;
//LAB_1040_04da:
  if (iStack4 != 0x8) {
    uVar5 = (iStack4 * 0xe + 0x5c68);
    w_param = 0x0;
    uVar8 = 0xc;
    pcVar6 = load_string_1010_847e
                       (ctx.PTR__LOOP_1050_14cc,
                        (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
    LVar7 = SendDlgItemMessage16
                      (0x1010,pcVar6,(pcVar6 >> 0x10),w_param,
                       CONCAT22(uVar5,uVar8));
    in_DX = (LVar7 >> 0x10);
  }
  if (iStack6 != 0x0) {
    local_12a[0] = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2,param_6,in_DX,
                                            unaff_DI);
    uVar1 = (local_12a[0] + 0x20);
    puStack30 = (puStack30 & 0xffff0000 | uVar1);
    if (uVar1 != 0x0) {
      PostMessage16(0x1010,0x0,0x0,CONCAT22(0x111,iStack6));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT  win_ui_op_1040_0558(param_1: u32,param_2: i16,param_3: HWND16)

{
  let iVar1: i16;
  let iVar2: i16;
  let unaff_SS: u16;
  let mut pcVar3: String; 
  LRESULT LVar4;
  WPARAM16 w_param;
  let uVar5: u16;
  let uVar6: u16;
  
  iVar1 = param_2 * 0xe;
  GetDlgItem16(param_3,*(INT16 *)(iVar1 + 0x5c64));
  iVar2 = pass1_1010_659a((param_1 + 0x8e),(iVar1 + 0x5c66),
                          unaff_SS);
  if ((iVar2 == 0x0) && ((iVar1 + 0x5c66) != 0xa)) {
    EnableWindow16(0x1010,0x0);
    uVar6 = (param_2 * 0xe + 0x5c68);
  }
  else {
    EnableWindow16(0x1010,0x1);
    uVar6 = (param_2 * 0xe + 0x5c68);
  }
  uVar5 = 0xc;
  w_param = 0x0;
  pcVar3 = load_string_1010_847e
                     (ctx.PTR__LOOP_1050_14cc,(ctx.PTR__LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  LVar4 = SendDlgItemMessage16
                    (0x1010,pcVar3,(pcVar3 >> 0x10),w_param,
                     CONCAT22(uVar6,uVar5));
  return LVar4;
}



pub fn enable_win_1040_060e(param_1: u32,param_2: i16,param_3: HWND16,param_4: u16)
{
  INT16 *pIVar1;
  let HStack16: HWND16;
  let uStack10: i16;
  let iStack8: i16;
  
  _iStack8 = (INT16 *)CONCAT22(param_4,&stack0x000a);
  iStack10 = param_2;
  while( true ) {
    pIVar1 = _iStack8;
    if (iStack10 == 0x0) break;
    _iStack8 = (INT16 *)(_iStack8 & 0xffff0000 | (iStack8 + 0x2));
    HStack16 = (param_1 + 0x6);
    iStack10 = iStack10 + -0x1;
    HStack16 = GetDlgItem16(param_3,*pIVar1);
    param_3 = ctx.s_tile2_bmp_1050_1538;
    EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x0);
  }
  return;
}


