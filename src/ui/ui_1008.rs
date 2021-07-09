
pub fn win_ui_cursor_op_1008_06c0(param_1: *mut u32,param_2: u32,param_3: u16,param_4: i16)
{
  let ppcVar1: u32;
  let in_AX: u16;
  let in_DX: u16;
  let puVar2: *mut u8;
  let extraout_DX: *mut u8;
  let unaff_DI: i16;
  let uVar3: u16;
  let unaff_SS: *mut u8;
  let in_AF: u8;
  let mut pcVar4: String; 
  let puVar5: *mut u16;
  uchar local_5a [0x50];
  let uStack10: u32;
  let HStack6: HCURSOR16;
  let HStack4: HCURSOR16;
  
  if (param_4 == 0x400) {
    pass1_1030_8344(ctx.PTR__LOOP_1050_5748,
                    (ctx.PTR__LOOP_1050_5748 >> 0x10),0x4000001);
    puVar2 = (in_DX | in_AX);
    if (puVar2 != 0x0) {
      if (ctx.PTR_LOOP_1050_4fe8 != 0x0) {
        pcVar4 = load_string_1010_847e
                           (ctx.PTR__LOOP_1050_14cc,
                            (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
        MessageBox16(0x1010,&ctx.PTR_LOOP_1050_0010,pcVar4,
                     (pcVar4 >> 0x10));
        return;
      }
      HStack4 = LoadCursor16(0x1030,0x7f02);
      HStack6 = SetCursor16(ctx.s_tile2_bmp_1050_1538);
      pass1_1030_83ba(ctx.PTR__LOOP_1050_5748,param_2,unaff_SS,in_AF);
      uVar3 = (ctx.PTR__LOOP_1050_5748 >> 0x10);
      (ctx.PTR__LOOP_1050_5748 + 0x8) = 0x1;
      uStack10 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x29,unaff_SS,puVar2,unaff_DI
                                );
      pass1_1018_262e(uStack10);
      pass1_1030_8326();
      pcVar4 = load_string_1010_847e
                         (ctx.PTR__LOOP_1050_14cc,
                          (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
      sys_1000_3f9c(local_5a,unaff_SS,0x109,ctx.data_seg,pcVar4,
                    &stack0xfffe,uVar3,0x1000,unaff_SS,in_AF);
      ppcVar1 = (*param_1 + 0x14);
      (**ppcVar1)(0x1000,param_1,(param_1 >> 0x10),0x0,local_5a);
      puVar5 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x37,unaff_SS,extraout_DX,
                               unaff_DI);
      pass1_1008_a9ec(puVar5);
      SetCursor16(0x1010);
      PostMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x11100fc);
    }
  }
  return;
}


pub fn menu_ui_op_1008_09ba(param_1: u32,param_2: HWND16,param_3: &RECT16,param_4: HWND16)
{
  let HVar1: HMENU16;
  let iVar2: i16;
  let uVar3: u16;
  POlet local_6: i16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0xec) == 0x0) {
    HVar1 = LoadMenu16(param_4,s_OPPOPMENU_1050_0150);
    *(HMENU16 *)(iVar2 + 0xec) = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
    param_4 = ctx.s_tile2_bmp_1050_1538;
    HVar1 = GetSubMenu16(ctx.s_tile2_bmp_1050_1538,0x0);
    *(HMENU16 *)(iVar2 + 0xec) = HVar1;
    if (HVar1 == 0x0) {
      return;
    }
  }
  local_6.x = param_3;
  local_6.y = param_2;
  ClientToScreen16(param_4,&local_6);
  TrackPopupMenu16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,PTR_LOOP_1050_0396,0x0,
                   local_6.y,local_6.x);
  return;
}



pub fn unk_win_msg_op_1008_0a3c(param_1: u32,param_2: u16,param_3: HWND16) -> u16

{
  let BVar1: bool;
  
  if ((param_2 & 0xfff0) == 0xf140) {
    return (param_1 + 0xde);
  }
  if ((param_2 & 0xfff0) == 0xf060) {
    BVar1 = IsIconic16(param_3);
    if (BVar1 == 0x0) {
      PostMessage16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,0x1110067);
    }
    return 0x0;
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn message_box_op_1008_12dc(param_1: u32,param_2: u32,HINSTANCE16 param_3,param_4: u16)
{
  let BVar1: bool;
  let uVar2: u16;
  let in_DX: u16;
  let uVar3: u16;
  ulet in_AF: u8;
  let mut pcVar4: String; 
  let uStack36: u32;
  let uStack16: u32;
  let local_c: [u8;6];
  let HStack6: HCURSOR16;
  let HStack4: HCURSOR16;
  
  HStack4 = LoadCursor16(param_3,0x7f02);
  HStack6 = SetCursor16(ctx.s_tile2_bmp_1050_1538);
  str_1008_6d8a(CONCAT22(param_4,local_c),param_2,in_DX,param_4,in_AF);
  BVar1 = file_fn_1008_6e02((uint32_t *)CONCAT22(param_4,local_c),
                            ctx.s_tile2_bmp_1050_1538,param_4);
  if (BVar1 == 0x0) {
    SetCursor16(ctx.s_tile2_bmp_1050_1538);
    pcVar4 = load_string_1010_847e
                       (ctx.PTR__LOOP_1050_14cc,
                        (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
    uVar3 = (pcVar4 >> 0x10);
    uVar2 = str_op_1008_60e8(pcVar4,uVar3);
    uStack16 = CONCAT22(uVar3,uVar2);
    pcVar4 = load_string_1010_847e
                       (ctx.PTR__LOOP_1050_14cc,
                        (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
    MessageBeep16(0x1010);
    MessageBox16(ctx.s_tile2_bmp_1050_1538,&ctx.PTR_LOOP_1050_0010,pcVar4,
                 (pcVar4 >> 0x10));
  }
  else {
    (ctx.PTR__LOOP_1050_5748 + 0x8) = 0x0;
    SetCursor16(ctx.s_tile2_bmp_1050_1538);
    pcVar4 = load_string_1010_847e
                       (ctx.PTR__LOOP_1050_14cc,
                        (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
    uVar3 = (pcVar4 >> 0x10);
    uVar2 = str_op_1008_60e8(pcVar4,uVar3);
    uStack36 = CONCAT22(uVar3,uVar2);
    pcVar4 = load_string_1010_847e
                       (ctx.PTR__LOOP_1050_14cc,
                        (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
    MessageBeep16(0x1010);
    MessageBox16(ctx.s_tile2_bmp_1050_1538,0x40,pcVar4,
                 (pcVar4 >> 0x10));
    uStack16 = uStack36;
  }
  fn_ptr_1000_17ce((uStack16 & 0xffff | uVar3 << 0x10),0x1000);
  close_file_1008_6dd0(CONCAT22(param_4,local_c),0x1000);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
win_ui_op_1008_1414(astruct_72 **param_1,param_2: u32,LPCSTR param_3,param_4: u16,
                   param_5: u8,param_6: u16)

{
  let ppcVar1: u32;
  let BVar2: bool;
  let uVar3: u16;
  let iVar4: i16;
  let puVar5: u32;
  let uVar5: u32;
  let puVar6: *mut u8;
  let uVar7: u16;
  let type: *mut u8;
  let uVar8: u16;
  let extraout_DX: u16;
  let unaff_DI: i16;
  let uVar9: u16;
  let puVar10: u32;
  let mut pcVar11: String; 
  let puVar12: *mut u16;
  let uVar13: u8;
  let uVar14: u8;
  let iVar15: i16;
  let local_2a: u32;
  let uStack38: u16;
  let iStack36: i16;
  let puStack34: *mut u8;
  let uStack32: u32;
  let uStack28: u32;
  let uStack24: u32;
  let uStack20: u32;
  let uStack16: u32;
  let puStack12: *mut u16;
  let local_8: [u8;6];
  let uVar10: u16;
  
  puVar10 = str_1008_6d8a(CONCAT22(param_4,local_8),param_2,param_6,
                          param_4,param_5);
  puVar6 = (puVar10 >> 0x10);
  BVar2 = read_file_1008_6e78((uint32_t)local_8,param_4,param_3,param_4);
  iVar15 = param_1;
  uVar9 = (param_1 >> 0x10);
  if (BVar2 == 0x0) {
    if (ctx.PTR_LOOP_1050_0310 == 0x0) {
      ctx.PTR_LOOP_1050_0310 = 0x6d4;
    }
    pcVar11 = load_string_1010_847e
                        (ctx.PTR__LOOP_1050_14cc,
                         (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
    uVar7 = (pcVar11 >> 0x10);
    uVar3 = str_op_1008_60e8(pcVar11,uVar7);
    pcVar11 = load_string_1010_847e
                        (ctx.PTR__LOOP_1050_14cc,
                         (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
    type = (pcVar11 >> 0x10);
    puVar6 = type;
    MessageBeep16(0x1010);
    MessageBox16(ctx.s_tile2_bmp_1050_1538,&ctx.PTR_LOOP_1050_0010,pcVar11
                 ,type);
    fn_ptr_1000_17ce(CONCAT22(uVar7,uVar3),0x1000);
    param_3 = &ctx.PTR_LOOP_1050_1000;
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  cursor_op_1008_2dcc(iVar15,uVar9,0x8,param_3);
  puStack12 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2f,param_4,puVar6,unaff_DI);
  uVar8 = (puStack12 >> 0x10);
  uVar5 = (puStack12 + 0x20);
  uStack16 = uVar5;
  pass1_1030_8344(ctx.PTR__LOOP_1050_5748,(ctx.PTR__LOOP_1050_5748 >> 0x10)
                  ,uVar5);
  uStack20 = uVar5 & 0xffff | uVar8 << 0x10;
  uStack24 = (uVar5 + 0x10);
  iVar4 = (uStack24 + 0x2) + -0x1;
  ppcVar1 = ((iVar15 + 0xe8) + 0x4);
  (**ppcVar1)(0x1030,(iVar15 + 0xe8),uStack16,(uStack16 >> 0x10),
              iVar4,0x2);
  puVar6 = extraout_DX;
  pass1_1030_8344(ctx.PTR__LOOP_1050_5748,(ctx.PTR__LOOP_1050_5748 >> 0x10)
                  ,0x4000001);
  uStack28 = CONCAT22(puVar6,iVar4);
  uVar5 = (iVar4 + 0x10);
  uStack32 = uVar5;
  pass1_1030_8344(ctx.PTR__LOOP_1050_5748,(ctx.PTR__LOOP_1050_5748 >> 0x10)
                  ,uVar5);
  iStack36 = uVar5;
  local_2a = (iStack36 + 0xc);
  uStack38 = (iStack36 + 0x10);
  puStack34 = puVar6;
  puVar5 = pass1_1030_5b00(uStack20);
  uVar13 = SUB21(&local_2a,0x0);
  uVar14 = (&local_2a >> 0x8);
  uVar3 = param_4;
  puVar12 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,puVar5,param_4,puVar6,
                            &iStack36);
  puVar6 = (puVar12 >> 0x10);
  pass1_1018_179e(puVar12,CONCAT22(uVar3,CONCAT11(uVar14,uVar13)),0x1018,param_4);
  uVar13 = 0x0;
  uVar14 = 0x4;
  iVar15 = 0x1b;
  uVar10 = 0x1;
  puVar12 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2b,param_4,puVar6,&iStack36);
  pass1_1010_043a(puVar12,CONCAT13(uVar14,CONCAT12(uVar13,uVar10)),iVar15,param_4);
  close_file_1008_6dd0(CONCAT22(param_4,local_8),0x1010);
  return;
}


pub fn post_msg_1008_2d22(param_1: u32)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let ppcVar3: u32;
  let iVar4: i16;
  let uVar5: u16;
  let unaff_SS: u16;
  let uVar6: u32;
  let uVar7: u16;
  let uVar8: u32;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (((iVar4 + 0xee) != 0x0) &&
     (piVar1 = (iVar4 + 0xf2), *piVar1 = *piVar1 + -0x1,
     (iVar4 + 0xf2) < 0x1)) {
    uVar8 = (iVar4 + 0xee);
    ppcVar3 = ((iVar4 + 0xee) + 0x90);
    (**ppcVar3)();
    (iVar4 + 0xee) = 0x0;
    (iVar4 + 0xf2) = 0x0;
    if ((iVar4 + 0xe8) != 0x0) {
      uVar7 = 0x3;
      uVar6 = (iVar4 + 0xe8);
      ppcVar3 = ((iVar4 + 0xe8) + 0xc);
      (**ppcVar3)();
      show_win_1038_b68a(ctx.PTR__LOOP_1050_5b7c,&ctx.PTR_LOOP_1050_1038);
      show_window_1010_7ace((iVar4 + 0xf4),unaff_SS);
      uVar2 = (iVar4 + 0xe8);
      ppcVar3 = ((iVar4 + 0xe8) + 0x98);
      (**ppcVar3)(0x1010,uVar2,(uVar2 >> 0x10),0x1,uVar6,uVar7,uVar8);
      PostMessage16(0x1010,0x0,0x0,0x11100fc);
    }
  }
  return;
}



void 
cursor_op_1008_2dcc(param_1: i16,param_2: u16,param_3: u16,HINSTANCE16 in_hinstance
                   )

{
  let uVar1: u32;
  let ppcVar2: u32;
  let cursor_handle: HCURSOR16;
  let in_DX: u16;
  let extraout_DX: u16;
  let iVar3: i16;
  let unaff_SS: u16;
  let uVar4: u32;
  
  uVar4 = 0x0;
  cursor_handle = LoadCursor16(in_hinstance,0x7f02);
  uVar4 = uVar4 & 0xffff0000 | cursor_handle;
  cursor_handle = SetCursor16(ctx.s_tile2_bmp_1050_1538);
  iVar3 = param_1;
  if ((param_1 + 0xe8) != 0x0) {
    uVar1 = (param_1 + 0xe8);
    iVar3 = (param_1 + 0xe8);
    ppcVar2 = (iVar3 + 0x90);
    (**ppcVar2)(ctx.s_tile2_bmp_1050_1538,uVar1,(uVar1 >> 0x10),uVar4);
    in_DX = extraout_DX;
  }
  big_switch_1008_15d4
            (iVar3,s_tile2_bmp_1050_1538,unaff_SS,CONCAT22(param_2,param_1),
             param_3);
  *(HCURSOR16 *)(param_1 + 0xe8) = cursor_handle;
  (param_1 + 0xea) = in_DX;
  uVar1 = (param_1 + 0xe8);
  if ((uVar1 + 0xe0) == 0x0) {
    uVar1 = (param_1 + 0xe8);
    ppcVar2 = ((param_1 + 0xe8) + 0x8);
    (**ppcVar2)(ctx.s_tile2_bmp_1050_1538,uVar1,(uVar1 >> 0x10));
    ppcVar2 = ((param_1 + 0xe8) + 0xc);
    (**ppcVar2)(ctx.s_tile2_bmp_1050_1538,(param_1 + 0xe8),0x3);
    (param_1 + 0xce) = (param_1 + 0xe8);
  }
  else {
    (param_1 + 0xe8) = 0x0;
    ui_op_1008_2c4e(param_1,param_2,param_3,s_tile2_bmp_1050_1538);
    (param_1 + 0xce) = 0x0;
  }
  SetCursor16(ctx.s_tile2_bmp_1050_1538);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_cursor_op_1008_2e9a(astruct_72 **param_1,param_2: u16)
{
  let uVar1: u16;
  let iVar2: i16;
  let in_DX: *mut u8;
  let uVar3: u16;
  let uVar4: u16;
  let unaff_DI: i16;
  let in_AF: u8;
  char local_22e [0xa];
  let local_224: [u8;108];
  let uStack284: u16;
  let mut pcStack282: String; 
  let HStack274: HCURSOR16;
  let HStack272: HCURSOR16;
  let uStack270: u32;
  Ulet UStack266: i32;
  let uStack262: u32;
  char local_102 [0x100];
  
  local_102[0] = '\0';
  uStack262 = 
              mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2,param_2,in_DX,unaff_DI);
  uVar1 = (uStack262 >> 0x10);
  iVar2 = uStack262;
  UStack266 = *(ULONG *)(iVar2 + 0x16);
  uVar3 = (iVar2 + 0x18);
  UStack266._0_2_ = uVar3 | UStack266;
  if (UStack266 == 0x0) {
    save_file_1008_3178(param_1,0x1,param_2);
    UStack266 = CONCAT22(uVar3,UStack266);
    uVar4 = uVar3 | UStack266;
    if (uVar4 == 0x0) {
      PostMessage16(0x1010,0x0,0x0,0x111013d);
      return;
    }
    unk_str_op_1000_3d3e
              (CONCAT22(param_2,local_102),CONCAT22(uVar3,UStack266)
              );
    str_1000_4d58(CONCAT22(param_2,local_102),0x0,0x0,
                  CONCAT22(param_2,local_224),CONCAT22(param_2,local_22e));
    uVar3 = uVar4;
    if (local_22e[0] != '\0') {
      pass1_1000_3cea(CONCAT22(param_2,local_224),CONCAT22(param_2,local_22e));
      uVar3 = uVar4;
    }
    struct_1010_5f1e(uStack262,CONCAT22(param_2,local_224),uVar3);
  }
  else {
    pcStack282 = (iVar2 + 0x1a);
    unk_str_op_1000_3d3e(CONCAT22(param_2,local_102),pcStack282);
    uStack284 = str_op_1000_3da4(CONCAT22(param_2,local_102));
    if (local_102[uStack284 - 0x1] != '\\') {
      local_102[uStack284] = '\\';
      local_102[uStack284 + 0x1] = '\0';
    }
    pass1_1000_3cea(CONCAT22(param_2,local_102),UStack266);
  }
  if (local_102[0] != '\0') {
    uStack270 = (param_1 + 0xe8);
    send_msg_1020_097e(uStack270,0x1020);
    UpdateWindow16(0x1020);
    HStack272 = LoadCursor16(s_tile2_bmp_1050_1538,0x7f02);
    HStack274 = SetCursor16(ctx.s_tile2_bmp_1050_1538);
    win_ui_op_1008_1414(param_1,CONCAT22(param_2,local_102),s_tile2_bmp_1050_1538,
                        param_2,in_AF,uVar3);
    SetCursor16(ctx.s_tile2_bmp_1050_1538);
  }
  return;
}


pub fn save_file_1008_3178(param_1: u32,param_2: i16,param_3: u16)
{
  let uVar1: u32;
  let iVar2: i16;
  let puVar3: *mut u16;
  let uVar4: u16;
  let BVar5: bool;
  let in_DX: *mut u8;
  let extraout_DX: u16;
  let uVar6: u16;
  let unaff_DI: i16;
  let uVar7: u16;
  let mut pcVar8: String; 
  let mut pcVar9: String; 
  let in_buf_len_2: i16;
  let uVar10: u16;
  char local_782 [0x104];
  let local_67e: [u16;0x4];
  let paStack1654: &mut Struct18;
  let mut pCStack1650: String;
  let UStack1648: u16;
  let paStack1646: &mut Struct18;
  let local_666: [u8;100];
  let mut pcStack1382: String; 
  let local_562: u32;
  let uStack1374: u16;
  let mut pcStack1370: String; 
  let uStack1350: u32;
  let puStack1346: *mut u8;
  let uStack1342: u32;
  let mut pcStack1338: String; 
  let puStack1334: *mut u8;
  let uStack1330: u32;
  let uStack1326: u16;
  let mut pcStack1322: String; 
  let cStack1306: u8;
  char acStack1305 [0x101];
  let uStack1048: u16;
  char local_416 [0x8];
  let uStack1038: u16;
  let local_40c: [u8;102];
  let uStack778: u32;
  let puStack774: *mut u16;
  let local_302: [u8;100];
  u8 local_202 [0xff];
  char acStack259 [0x101];
  
  acStack259[1] = 0x0;
  local_302[0] = 0x0;
  local_202[0] = 0x0;
  puStack774 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2,param_3,in_DX,unaff_DI);
  uVar7 = (puStack774 >> 0x10);
  iVar2 = puStack774;
  uStack778 = (iVar2 + 0x1a);
  uVar10 = (iVar2 + 0x1c);
  if ((uVar10 | uStack778) == 0x0) {
    paStack1646 = (iVar2 + 0x64);
    uVar10 = (iVar2 + 0x66);
    if ((uVar10 | paStack1646) != 0x0) {
      pass1_1008_5784(CONCAT22(param_3,local_67e),
                      paStack1646 & 0xffff | uVar10 << 0x10);
      puVar3 = local_67e;
      pass1_1008_5b12(puVar3,param_3);
      paStack1654 = CONCAT22(extraout_DX,puVar3);
      if ((extraout_DX | puVar3) != 0x0) {
        uVar1 = (puVar3 + 0x2);
        uStack778._0_2_ = uVar1;
        uVar10 = (uVar1 >> 0x10);
//         TODO: goto LAB_1008_3206;
      }
    }
  }
  else {
//LAB_1008_3206:
    unk_str_op_1000_3d3e
              (CONCAT22(param_3,acStack259 + 0x1),
               CONCAT22(uVar10,uStack778));
  }
  pass1_1000_5008(local_40c,param_3,0x100,&stack0xfffe);
  uStack1038 = str_op_1000_3da4(CONCAT22(param_3,local_40c));
  if (local_40c[uStack1038 - 0x1] == '\\') {
    local_40c[uStack1038 - 0x1] = 0x0;
  }
  uStack1038 = str_op_1000_3da4(CONCAT22(param_3,acStack259 + 0x1));
  if (acStack259[uStack1038] == '\\') {
    acStack259[uStack1038] = '\0';
  }
  pass1_1000_4f2e(&stack0xfffe);
  uVar7 = (puStack774 >> 0x10);
  uStack778 = (puStack774 + 0x12);
  uVar10 = (puStack774 + 0x14);
  if ((uVar10 | uStack778) != 0x0) {
    unk_str_op_1000_3d3e
              (CONCAT22(param_3,local_202),
               (uStack778 & 0xffff | uVar10 << 0x10));
  }
  local_416[0] = '\0';
  pcVar8 = load_string_1010_847e
                     (ctx.PTR__LOOP_1050_14cc,(ctx.PTR__LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  unk_str_op_1000_3d3e(CONCAT22(param_3,local_416),pcVar8);
  uStack1048 = str_op_1000_3da4(CONCAT22(param_3,local_416));
  uStack1038 = uStack1048;
  for (; -0x1 < uStack1048; uStack1048 -= 0x1) {
    if (local_416[uStack1048] == '.') {
      unk_str_op_1000_3d3e
                (CONCAT22(param_3,local_67e),
                 CONCAT22(param_3,local_416 + uStack1048 + 0x1));
      unk_str_op_1000_3d3e
                (CONCAT22(param_3,local_416),CONCAT22(param_3,local_67e));
    }
  }
  acStack1305[1] = 0x0;
  pcVar8 = load_string_1010_847e
                     (ctx.PTR__LOOP_1050_14cc,(ctx.PTR__LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  uVar4 = (pcVar8 >> 0x10);
  unk_str_op_1000_3d3e(CONCAT22(param_3,acStack1305 + 0x1),pcVar8);
  uStack1038 = str_op_1000_3da4(CONCAT22(param_3,acStack1305 + 0x1));
  cStack1306 = acStack1305[uStack1038];
  uStack1048 = 0x0;
  while (acStack1305[uStack1048 + 0x1] != '\0') {
    if (acStack1305[uStack1048 + 0x1] == cStack1306) {
      acStack1305[uStack1048 + 0x1] = '\0';
    }
    uStack1048 += 0x1;
  }
  pass1_1000_4906(CONCAT22(param_3,&local_562),0x0,0x48);
  local_562 = 0x48;
  uStack1374 = (param_1 + 0x8);
  pcStack1370 = acStack1305 + 0x1;
  pcVar8 = CONCAT22(param_3,local_202);
  puStack1346 = local_302;
  uStack1350 = 0x100;
  uStack1342 = 0x100;
  pcStack1338 = acStack259 + 0x1;
  pcStack1322 = local_416;
  pcStack1382 = 0x0;
  local_666[0] = 0x0;
  in_buf_len_2 = (ctx.PTR__LOOP_1050_14cc >> 0x10);
  if (param_2 == 0x1) {
    uStack1330 = 0x1804;
    pcVar9 = load_string_1010_847e(ctx.PTR__LOOP_1050_14cc,in_buf_len_2,0x1010);
    uVar4 = (pcVar9 >> 0x10);
    unk_str_op_1000_3d3e(CONCAT22(param_3,local_666),pcVar9);
    puStack1334 = local_666;
    BVar5 = GetOpenFileName16(0x1000);
  }
  else {
    if (param_2 != 0x2) {
      debug_print_1008_6048
                (s_Unsupported_FileStructType_in_Op_1050_01ca,0x1000,param_3);
//       TODO: goto LAB_1008_3461;
    }
    uStack1330 = 0x6;
    pcVar9 = load_string_1010_847e(ctx.PTR__LOOP_1050_14cc,in_buf_len_2,0x1010);
    uVar4 = (pcVar9 >> 0x10);
    unk_str_op_1000_3d3e(CONCAT22(param_3,local_666),pcVar9);
    puStack1334 = local_666;
    BVar5 = GetSaveFileName16(0x1000);
  }
  if (BVar5 != 0x0) {
    pcStack1382 = pcVar8;
  }
//LAB_1008_3461:
  if (pcStack1382 != 0x0) {
    local_67e[0] = uStack1326;
    if (uStack1326 < 0x0) {
      paStack1654 = 
                    load_string_1010_847e
                              (ctx.PTR__LOOP_1050_14cc,
                               (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
      uVar6 = (paStack1654 >> 0x10);
      uVar4 = str_op_1008_60e8(paStack1654,uVar6);
      paStack1654 = CONCAT22(uVar6,uVar4);
      pcVar8 = load_string_1010_847e
                         (ctx.PTR__LOOP_1050_14cc,
                          (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
      UStack1648 = (pcVar8 >> 0x10);
      pCStack1650 = pcVar8;
      MessageBox16(0x1010,&ctx.PTR_LOOP_1050_0010,pCStack1650,UStack1648);
      pcStack1382 = 0x0;
      paStack1646 = paStack1654;
      fn_ptr_1000_17ce(paStack1654,0x1000);
    }
    else {
      str_op_1000_3dbe(CONCAT13((param_3 >> 0x8),
                                        CONCAT12(param_3,local_782)),pcVar8,
                       uStack1326);
      local_782[uStack1326] = '\0';
      if (local_782[0] != '\0') {
        pass1_1010_60cc(puStack774,CONCAT22(param_3,local_782),uVar4);
      }
    }
  }
  pass1_1000_4f2e(&stack0xfffe);
  return;
}



// WARNING: Could not reconcile some variable overlaps

pub fn set_sys_color_1008_357e(param_1: u32,param_2: i16,in_index_3: i16,param_4: u16)
{
  let uVar1: u16;
  COLORREF colorref_var2;
  let iVar2: i16;
  let iVar3: &mut Struct53;
  let iVar4: i16;
  let uVar6: &mut Struct53;
  let uVar5: u16;
  let count: i16;
  let uVar7: u32;
  let iStack132: i16;
  let local_80: u32;
  let uStack124: u16;
  let uStack122: u16;
  let uStack120: u16;
  let uStack118: u16;
  let uStack116: u16;
  let uStack114: u16;
  let uStack112: u32;
  let uStack108: u32;
  let uStack104: u16;
  let uStack102: u16;
  let uStack100: u16;
  let uStack98: u16;
  let uStack96: u16;
  let uStack94: u16;
  let uStack92: u16;
  let uStack90: u16;
  let uStack88: u32;
  let uStack84: u32;
  let uStack80: u16;
  let uStack78: u16;
  let uStack76: u32;
  let uStack72: u32;
  let uStack68: u32;
  let uStack64: u32;
  let uStack60: u32;
  let uStack56: u32;
  let uStack52: u32;
  let uStack48: u32;
  let local_2c: u32;
  let uStack40: u32;
  let uStack36: u32;
  let uStack32: u32;
  let uStack28: u32;
  let uStack24: u32;
  let uStack20: u32;
  let uStack16: u32;
  let uStack12: u32;
  let uStack8: u32;
  let uStack4: u16;
  
  local_2c = 0x70004;
  uStack40 = 0xf0000;
  uStack36 = 0x100014;
  uStack32 = 0xd0012;
  uStack28 = 0x6000e;
  uStack24 = 0x80005;
  uStack20 = 0x10011;
  uStack16 = 0x30002;
  uStack12 = 0xa0009;
  uStack8 = 0xc000b;
  uStack4 = 0x13;
  local_80 = 0x0;
  uStack108 = 0x808080;
  iVar2 = 0x100;
  uStack116 = 0x0;
  uStack114 = 0x100;
  uStack100 = 0x0;
  uStack98 = 0x100;
  uStack96 = 0xffff;
  uStack94 = 0x0;
  uStack124 = 0x2;
  uStack122 = 0x100;
  uStack120 = 0x2;
  uStack118 = 0x100;
  uStack104 = 0x2;
  uStack102 = 0x100;
  uStack92 = 0x2;
  uStack90 = 0x100;
  uStack88 = 0x0;
  uStack80 = 0xc0c0;
  uStack78 = 0x0;
  uStack76 = 0x0;
  uStack72 = 0x0;
  uStack68 = 0x0;
  uStack52 = 0x0;
  uVar1 = 0x8000;
  uStack112 = 0x8000;
  uStack84 = 0x8000;
  uStack64 = 0x8000;
  uStack60 = 0x8000;
  uStack56 = 0x8000;
  uStack48 = 0x8000;
  uVar6 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (&iVar3.field_0xf8 == 0x0) {
    mem_op_1000_179c(0x54,(s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),
                     0x1000);
    iVar3.field_0xf8 = uVar1;
    iVar3.field_0xfa = iVar2;
    in_index_3 = 0x1000;
    for (iStack132 = 0x0; iStack132 < 0x15; iStack132 += 0x1) {
      colorref_var2 = GetSysColor16(in_index_3);
      uVar7 = &iVar3.field_0xf8;
      uVar5 = (uVar7 >> 0x10);
      iVar4 = uVar7;
      *(COLORREF *)(iVar4 + iStack132 * 0x4) = colorref_var2;
      (iVar4 + iStack132 * 0x4 + 0x2) = iVar2;
      in_index_3 = ctx.s_tile2_bmp_1050_1538;
    }
  }
  count = in_index_3;
  if (param_2 != 0x0) {
    count = ctx.s_tile2_bmp_1050_1538;
    colorref_var2 = GetSysColor16(in_index_3);
    if ((colorref_var2 == local_80) && (iVar2 == local_80._2_2_)) {
      return;
    }
  }
  if (ctx.PTR_LOOP_1050_0010 == 0x0) {
    uStack112 = 0xc0c0c0;
  }
  if (param_2 == 0x0) {
    uVar7 = &iVar3.field_0xf8;
  }
  else {
    uVar7 = CONCAT22(param_4,&local_80);
  }
  SetSysColors16(count,(INT16 *)uVar7,(COLORREF *)(uVar7 >> 0x10));
  return;
}


pub fn fill_rect_1008_39ac(in_win_handle_1: HWND16)
{
  RECT16 local_brush_handle [0x2];
  let local_brush_handle_2: *mut RECT16;
  let HStack36: HDC16;
  let local_paint_struct: PAINTSTRUCT16;
  
  HStack36 = BeginPaint16(in_win_handle_1,&local_paint_struct);
  local_brush_handle_2 = CreateSolidBrush16(s_tile2_bmp_1050_1538);
  GetClientRect16(ctx.s_tile2_bmp_1050_1538,local_brush_handle);
  FillRect16(ctx.s_tile2_bmp_1050_1538,local_brush_handle_2,
             local_brush_handle);
  EndPaint16(ctx.s_tile2_bmp_1050_1538,&local_paint_struct);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  return;
}


pub fn post_quit_msg_1008_3af4(short exit_code)
{
  PostQuitMessage16(exit_code);
  return;
}


void 
win_ui_op_1008_3c34(param_1: u32,param_2: u8,param_3: u16,HDC16 param_4,
                   param_5: u16)

{
  let uVar1: u16;
  let ppcVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let HStack12: HPALETTE16;
  let uStack10: u16;
  let puStack6: u32;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (((iVar3 + 0xa) | (iVar3 + 0x8)) != 0x0) {
    puStack6 = (iVar3 + 0x8);
    if (((iVar3 + 0xc) != 0x0) && ((param_2 & 0x1) != 0x0)) {
      puStack6 = (iVar3 + 0xc);
    }
    if (((iVar3 + 0x10) != 0x0) && ((param_2 & 0x4) != 0x0)) {
      puStack6 = (iVar3 + 0x10);
    }
    uVar5 = (ctx.PTR__LOOP_1050_4230 >> 0x10);
    uStack10 = (ctx.PTR__LOOP_1050_4230 + 0xe);
    uVar1 = (ctx.PTR__LOOP_1050_4230 + 0x10);
    HStack12 = palette_op_1008_4e08
                         (CONCAT22(uVar1,uStack10),&param_3,uVar1,param_4);
    ppcVar2 = (*puStack6 + 0x4);
    (**ppcVar2)(param_4,puStack6,(puStack6 >> 0x10),
                (iVar3 + 0x28),(iVar3 + 0x26),&param_3,
                param_5);
    SelectPalette16(param_4,0x0,HStack12);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  }
  return;
}



pub fn post_msg_1008_3d20(param_1: u32,param_2: HWND16)
{
  PostMessage16(param_2,0x0,0x0,CONCAT22(0x111,(param_1 + 0xcc)));
  return;
}


pub fn set_di_bits_to_device_1008_45d6(param_1: u32,HDC16 param_2)
{
  let info: i16;
  let uVar1: u32;
  let bVar2: bool;
  let iVar3: i16;
  let y_dest: i16;
  let uVar4: u16;
  let cx: i16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if ((iVar3 + 0x6) == 0x0) {
    pass1_1008_47cc(param_1);
  }
  if (((iVar3 + 0x8) | (iVar3 + 0x6)) == 0x0) {
    bVar2 = false;
  }
  else {
    if (((iVar3 + 0xc) | (iVar3 + 0xa)) == 0x0) {
      pass1_1008_4834(param_1);
    }
    bVar2 = true;
  }
  if (!bVar2) {
    return;
  }
  uVar1 = (iVar3 + 0x10);
  cx = (uVar1 >> 0x10);
  y_dest = uVar1;
  info = (y_dest + 0x8);
  uVar1 = (iVar3 + 0x14);
  SetDIBitsToDevice(param_2,0x0,y_dest,cx,uVar1,(uVar1 >> 0x10),info,
                    0x0,0x0,0x0,(BITMAPINFO *)info,(y_dest + 0x4));
  return;
}



pub fn stretch_di_bits_1008_465a(param_1: u32,HDC16 param_2)
{
  PVOID bits;
  let height_src: i16;
  let uVar1: u32;
  let bVar2: bool;
  let iVar3: i16;
  let height_dst: i16;
  let uVar4: u16;
  let x_src: i16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if ((iVar3 + 0x6) == 0x0) {
    pass1_1008_47cc(param_1);
  }
  if (((iVar3 + 0x8) | (iVar3 + 0x6)) == 0x0) {
    bVar2 = false;
  }
  else {
    if (((iVar3 + 0xc) | (iVar3 + 0xa)) == 0x0) {
      pass1_1008_4834(param_1);
    }
    bVar2 = true;
  }
  if (!bVar2) {
    return;
  }
  uVar1 = (iVar3 + 0x10);
  x_src = (uVar1 >> 0x10);
  height_dst = uVar1;
  bits = *(PVOID *)(height_dst + 0x4);
  height_src = (height_dst + 0x8);
  uVar1 = (iVar3 + 0x14);
  StretchDIBits16(param_2,0x20,0xcc,0x0,height_dst,x_src,uVar1,
                  (uVar1 >> 0x10),height_src,bits,(BITMAPINFO *)0x0,0x0,
                  CONCAT22(bits,height_src));
  return;
}



pub fn palette_op_1008_46e4(param_1: u32,param_2: u16,param_3: u16,HDC16 param_4) -> u16

{
  let bVar1: bool;
  let uVar2: u16;
  let HVar2: HPALETTE16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u32;
  let uVar6: u32;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if ((iVar3 + 0x6) == 0x0) {
    uVar5._0_2_ = param_2;
    uVar5._2_2_ = param_3;
    pass1_1008_47cc((param_1 & 0xffff | uVar4 << 0x10));
    param_2 = uVar5;
    param_3 = uVar5._2_2_;
  }
  uVar6 = CONCAT22(param_3,param_2);
  if ((iVar3 + 0x6) == 0x0) {
    bVar1 = false;
  }
  else {
    if ((iVar3 + 0xa) == 0x0) {
      uVar6 = pass1_1008_4834((param_1 & 0xffff | uVar4 << 0x10));
    }
    bVar1 = true;
  }
  uVar2 = uVar6;
  if (!bVar1) {
    return 0x0;
  }
  create_palette_1008_4e38
            ((iVar3 + 0xa),param_4,(uVar6 >> 0x10));
  (iVar3 + 0xe) = uVar2;
  HVar2 = SelectPalette16(param_4,0x0,*(bool *)(iVar3 + 0xe));
  *(HPALETTE16 *)(iVar3 + 0x4) = HVar2;
  RealizePalette16(ctx.s_tile2_bmp_1050_1538);
  return (iVar3 + 0x4);
}



HPALETTE16 
palette_op_1008_4e08(param_1: &mut Struct13,bool param_2,param_3: u16,HDC16 param_4)

{
  let HVar1: HPALETTE16;
  
  create_palette_1008_4e38(param_1,param_4,param_3);
  HVar1 = SelectPalette16(param_4,0x0,param_2);
  RealizePalette16(ctx.s_tile2_bmp_1050_1538);
  return HVar1;
}



// WARNING: Unable to use type for symbol uVar3

void 
create_palette_1008_4e38
          (in_struct_1: &mut Struct13,LOGPALETTE *in_log_palette_2,param_3: *mut u8)

{
  let piVar1: *mut i16;
  let puVar2: *mut u16;
  let uVar4: u16;
  let local_struct_1: &mut Struct13;
  let iVar5: i16;
  let iVar6: i16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let iStack14: i16;
  let puStack12: *mut u8;
  let puStack8: *mut u8;
  let uVar3: *mut u16;
  
  uVar8 = (in_struct_1 >> 0x10);
  local_struct_1 = in_struct_1;
  uVar4 = (local_struct_1.field_0xc + 0x2) * 0x4;
  if (local_struct_1.field_0xe == 0x0) {
    in_log_palette_2 = (LOGPALETTE *)&ctx.PTR_LOOP_1050_1000;
    mem_op_1000_179c(uVar4,param_3,0x1000);
    &local_struct_1.field_0xe = uVar4;
    (&local_struct_1.field_0xe + 0x2) = param_3;
    *local_struct_1.field_0xe = 0x300;
    uVar3 = local_struct_1.field_0xe;
    (uVar3 + 0x2) = local_struct_1.field_0xc;
    puVar2 = local_struct_1.field_0xe;
    puStack8 = (puVar2 & 0xffff0000 | (puVar2 + 0x4));
    puStack12 = local_struct_1.field_0x4;
    iStack14 = 0x0;
    while( true ) {
      piVar1 = &local_struct_1.field_0xc;
      if (*piVar1 == iStack14 || *piVar1 < iStack14) break;
      uVar9 = (puStack12 >> 0x10);
      iVar5 = puStack12;
      *puStack8 = *(iVar5 + 0x2);
      uVar10 = (puStack8 >> 0x10);
      iVar6 = puStack8;
      *(iVar6 + 0x1) = *(iVar5 + 0x1);
      *(iVar6 + 0x2) = *puStack12;
      *(iVar6 + 0x3) = 0x0;
      iStack14 += 0x1;
      puStack8 = (puStack8 & 0xffff0000 | (iVar6 + 0x4));
      puStack12 = (puStack12 & 0xffff0000 | (iVar5 + 0x4));
    }
  }
  CreatePalette16(in_log_palette_2);
  return;
}


void 
file_and_draw_op_1008_4f20
          (param_1: *mut u16,param_2: u32,param_3: u16,param_4: u32,param_5: u16)

{
  let uVar1: u32;
  let b_force_background: u16;
  COLORREF color;
  COLORREF color_00;
  let x: u16;
  let uVar2: u16;
  let mut output: String;
  let iVar3: i16;
  let uVar4: u16;
  let paVar5: &mut Struct43;
  let uVar6: u32;
  init_data: DEVMODEA;
  let local_2c: HDC16;
  let mut pCStack42: String;
  let mut pCStack40: String;
  let local_26: [u8;24];
  
  pass1_1008_4016(param_1);
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  (iVar3 + 0x1e) = param_4;
  (iVar3 + 0x22) = param_3;
  (iVar3 + 0x24) = param_2;
  *param_1 = (s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x9);
  (iVar3 + 0x2) = 0x1008;
  paVar5 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc,0x2,param_5);
  uVar2 = (paVar5 >> 0x10);
  struct_op_1008_48fe(CONCAT22(param_5,local_26),0x1,paVar5,uVar2);
  read_file_1008_49e8(CONCAT22(param_5,local_26),0x1010,uVar2);
  pass1_1008_5068(param_1,CONCAT22(param_5,local_26));
  pass1_1008_47cc(param_1);
  pass1_1008_4834(param_1);
  init_data = 0x0;
  uVar6 = pass1_1008_4772(param_1);
  output = (uVar6 >> 0x10);
  pCStack42 = uVar6;
  pCStack40 = output;
  local_2c = CreateDC16(0x1010,pCStack42,output,init_data);
  b_force_background =
       palette_op_1008_46e4
                 (param_1,&local_2c,output,
                  ctx.s_tile2_bmp_1050_1538);
  color = SetBkColor16(ctx.s_tile2_bmp_1050_1538,0xffff);
  color_00 = SetTextColor16(ctx.s_tile2_bmp_1050_1538,*(COLORREF *)(iVar3 + 0x22));
  x = str_op_1000_3da4((iVar3 + 0x1e));
  uVar1 = (iVar3 + 0x1e);
  TextOut16(0x1000,x,uVar1,(uVar1 >> 0x10),0x0);
  SetBkColor16(ctx.s_tile2_bmp_1050_1538,color);
  SetTextColor16(ctx.s_tile2_bmp_1050_1538,color_00);
  SelectPalette16(ctx.s_tile2_bmp_1050_1538,0x0,b_force_background);
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  DeleteDC16(ctx.s_tile2_bmp_1050_1538);
  close_file_1008_496c(local_26,param_5);
  return;
}



pub fn cleanup_palette_1008_56e2(Uparam_1: i32,HDC16 param_2) -> bool

{
  let HVar1: HPALETTE16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  HVar1 = SelectPalette16(param_2,0x0,*(bool *)(param_1 + 0x4));
  *(HPALETTE16 *)(param_1 + 0x4) = HVar1;
  DeleteObject16(ctx.s_tile2_bmp_1050_1538);
  return 0x1;
}


void 
win_1008_5c5c(WNDCLASS16 *param_1,param_2: u16,param_3: u16,param_4: u32,
             param_5: u16)

{
  pass1_1010_84f8(ctx.PTR__LOOP_1050_14cc,param_5,param_1);
  win_ui_op_1008_5cfe(param_4,CONCAT22(param_3,param_2),param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
win_1008_5c7c(param_1: u32,param_2: u32,WNDCLASS16 *param_3,param_4: u16,
             param_5: u16)

{
  pass1_1010_85be(ctx.PTR__LOOP_1050_14cc,param_2,(param_2 >> 0x10),param_3)
  ;
  win_ui_op_1008_5cfe(param_1,CONCAT22(param_5,param_4),param_3);
  return;
}



void 
win_1008_5c9e(param_1: u32,param_2: *mut u32,param_3: u16,param_4: u16,
             WNDCLASS16 *param_5)

{
  win_1008_5c7c(param_1,*param_2,param_5,param_3,param_4);
  return;
}


pub fn win_ui_op_1008_5cfe(param_1: u32,char *param_2,WNDCLASS16 *in_wnd_class)
{
  let uVar1: u32;
  let iVar2: i16;
  let iVar3: i16;
  let uVar4: u16;
  let DVar5: u32;
  let message_1: HWND16;
  let uStack298: u16;
  let window_handle_1: HWND16;
  let local_11e: [u8;100];
  let mut string_1: String; 
  let iStack26: i16;
  let iStack24: i16;
  let local_16: [u8;4];
  let offset_val: i16;
  let mut pcStack14: String; 
  let mut pcStack10: String; 
  
  pass1_1000_4906(CONCAT22(in_wnd_class,local_16),0x0,0x14);
  pcStack10 = param_2;
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  uVar1 = (iVar3 + 0xc);
  iStack24 = (uVar1 + 0x72);
  iStack26 = 0x1;
  string_1 = s_waveaudio_1050_02a4;
  str_1000_4d58(param_2,0x0,0x0,0x0,CONCAT22(in_wnd_class,local_11e)
               );
  iVar2 = pass1_1000_475e(CONCAT22(in_wnd_class,local_11e),0x105002ae);
  if (iVar2 == 0x0) {
    uVar1 = (iVar3 + 0xc);
    iStack24 = (uVar1 + 0x74);
    string_1 = s_sequencer_1050_02b3;
    iStack26 = 0x0;
  }
  if (iStack24 != 0x0) {
    if ((iStack26 != 0x0) && ((iVar3 + 0x10) != 0x0)) {
      return;
    }
    if ((iStack26 == 0x0) && ((iVar3 + 0x12) != 0x0)) {
      return;
    }
    pcStack14 = string_1;
    DVar5 = mciSendCommand16(0x1000,local_16,CONCAT13(0x22,ZEXT23(in_wnd_class)),
                             0x8030000);
    if (((DVar5 >> 0x10) | DVar5) == 0x0) {
      if (iStack26 == 0x0) {
        (iVar3 + 0x12) = 0x1;
      }
      else {
        (iVar3 + 0xa) = offset_val;
        (iVar3 + 0x10) = 0x1;
      }
      window_handle_1 =
           create_window_1008_5e7e(ctx.s_tile2_bmp_1050_1538,in_wnd_class);
      if (window_handle_1 == 0x0) {
        mci_send_command_1008_5cb6(param_1,offset_val,s_tile2_bmp_1050_1538);
        return;
      }
      pass1_1000_4906(CONCAT22(in_wnd_class,&message_1),0x0,
                      0xc);
      message_1 = window_handle_1;
      uStack298 = 0x0;
      mciSendCommand16(0x1000,&message_1,CONCAT12(0x1,in_wnd_class),
                       0x8060000);
      SetWindowWord16(ctx.s_tile2_bmp_1050_1538,offset_val,0x0);
      return;
    }
  }
  if (iStack26 == 0x0) {
    iVar2 = 0x11;
  }
  else {
    iVar2 = 0x10;
  }
  pass1_1010_1f62(in_wnd_class,param_1,iVar2);
  return;
}



pub fn create_window_1008_5e7e(u16 in_stock_obj_id,WNDCLASS16 *in_wnd_class) -> HWND16
{
  let puVar1: u32;
  let puVar2: u32;
  let BVar3: bool;
  ATOM AVar4;
  let window_handle_1: HWND16;
  let iVar5: i16;
  let mut string_1: String;
  let puVar6: u32;
  let name: u16;
  let uStack42: u16;
  let uStack40: u16;
  let uStack38: u16;
  let uStack36: u16;
  let puStack34: *mut u8;
  let uStack32: u16;
  let uStack30: u16;
  let HStack28: HGDIOBJ16;
  let uStack26: u32;
  let puStack22: u32;
  let local_12: [u32;0x4];
  
  puVar6 = local_12;
  string_1 = s_MciSoundWindow_1050_02bd;
  for (iVar5 = 0x3; iVar5 != 0x0; iVar5 += -0x1) {
    puVar2 = puVar6;
    puVar6 = puVar6 + 0x1;
    puVar1 = string_1;
    string_1 = (string_1 + 0x4);
    *puVar2 = *puVar1;
  }
  puVar6 = string_1;
  *(puVar6 + 0x2) =
       *(string_1 + 0x2);
  name = 0x2000;
  uStack42 = SUB42(&DAT_1050_5f44,0x0);
  uStack40 = 0x1008;
  uStack36 = 0x2;
  puStack34 = ctx.PTR_LOOP_1050_038c;
  uStack32 = 0x0;
  uStack30 = 0x0;
  uStack38 = 0x0;
  HStack28 = GetStockObject16(in_stock_obj_id);
  uStack26 = 0x0;
  puStack22 = local_12;
  BVar3 = GetClassInfo16(s_tile2_bmp_1050_1538,(SEGPTR)&name,in_wnd_class);
  if (BVar3 == 0x0) {
    AVar4 = RegisterClass16(s_tile2_bmp_1050_1538);
    if (AVar4 == 0x0) {
      OutputDebugString16(ctx.s_tile2_bmp_1050_1538);
      return 0x0;
    }
  }
  window_handle_1 =
       CreateWindow16(ctx.s_tile2_bmp_1050_1538,0x0,
                      ZEXT24(ctx.PTR_LOOP_1050_038c) << 0x10,0x0,PTR_LOOP_1050_0396,0x1
                      ,0x1,0x8000,0x8000,0x0,(LPVOID)0xcf);
  return window_handle_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT 
make_def_win_proc_1008_5f44
          (param_1: u16,WPARAM16 in_wparam_2,LPARAM param_3,in_hwnd_4: HWND16)

{
  let WVar1: u16;
  let in_DX: *mut u8;
  let unaff_DI: i16;
  WNDCLASS16 *unaff_SS;
  LRESULT LVar2;
  let puVar3: *mut u16;
  
  if (param_3._2_2_ == 0x2) {
    WVar1 = GetWindowWord16(in_hwnd_4,0x0);
    mci_send_command_1008_5cb6(ctx.PTR__LOOP_1050_02a0,WVar1,s_tile2_bmp_1050_1538);
    puVar3 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x37,unaff_SS,in_DX,unaff_DI);
    pass1_1008_aa28(puVar3,puVar3,unaff_SS);
  }
  else {
    if (param_3._2_2_ != 0x3b9) {
      LVar2 = DefWindowProc16(in_hwnd_4,param_1,in_wparam_2,param_3);
      return LVar2;
    }
    DestroyWindow16(in_hwnd_4);
  }
  return 0x0;
}



u16 
win_ui_op_1008_8214(param_1: u16,param_2: i16,param_3: u16,param_4: u32,param_5: u16,
                   param_6: *mut u8,param_7: HWND16)

{
  let IVar1: i16;
  let puVar2: u32;
  let puVar3: *mut u16;
  let uVar4: u16;
  
  if (param_2 == 0x81) {
    uVar4 = 0x6;
    mem_op_1000_179c(0x6,param_6,0x1000);
    if ((param_6 | param_5) == 0x0) {
      puVar2 = 0x0;
    }
    else {
      puVar2 = pass1_1008_80d2(CONCAT22(param_6,param_5));
    }
    param_7 = ctx.s_tile2_bmp_1050_1538;
    SetWindowLong16(0x1000,puVar2,CONCAT22(uVar4,(puVar2 >> 0x10)));
  }
  if (param_2 == 0x1) {
    puVar3 = GetWindowLong16(param_7,0x0);
    *puVar3 = (param_4 + 0x8);
    IVar1 = GetDlgCtrlID16(ctx.s_tile2_bmp_1050_1538);
    (puVar3 + 0x2) = IVar1;
  }
  return 0x1;
}



pub fn send_msg_1008_84ba(param_1: u16,param_2: u32,param_3: HWND16)
{
  let iVar1: i16;
  let uVar2: u16;
  let uStack4: u16;
  
  uVar2 = (param_2 >> 0x10);
  iVar1 = param_2;
  if (((iVar1 + 0x4) & 0x4) == 0x0) {
    if (((iVar1 + 0x4) & 0x8) == 0x0) {
      return;
    }
    uStack4 = 0x1;
  }
  else {
    uStack4 = 0x0;
  }
  SendMessage16(param_3,(iVar1 + 0x2),0x0,CONCAT22(0x115,uStack4));
  return;
}



pub fn win_sys_op_1008_84f2(param_1: u16,param_2: u16,param_3: i16,param_4: u32,param_5: HWND16)
{
  let pbVar1: *mut u8;
  let iVar2: i16;
  let iVar3: i16;
  let Bvar4: bool;
  let uVar5: u16;
  let uVar6: u16;
  let paVar7: &mut Struct18;
  let cVar8: u8;
  let local_a: RECT16;
  let iStack4: i16;
  
  paVar7 = GetWindowLong16(param_5,0x0);
  uVar6 = (paVar7 >> 0x10);
  iVar3 = paVar7;
  if (param_4 == 0x1f) {
    (iVar3 + 0x4) = 0x0;
    KillTimer16(ctx.s_tile2_bmp_1050_1538,0xfa6);
    KillTimer16(ctx.s_tile2_bmp_1050_1538,0xfa7);
    ReleaseCapture16();
  }
  else {
    cVar8 = param_4;
    if (param_4 < 0x20) {
      if (param_4 != 0x14) {
        if (0x14 < param_4) goto LAB_1008_8771;
        uVar5 = param_4 & 0xff00 | (cVar8 - 0x1);
        if ((cVar8 - 0x1) == 0x0) {
//LAB_1008_8560:
          win_ui_op_1008_8214(param_4._2_2_,param_4,param_3,
                              CONCAT22(param_2,param_1),uVar5,uVar6,
                              ctx.s_tile2_bmp_1050_1538);
          return;
        }
        if (cVar8 == '\x02') {
          fn_ptr_1000_17ce(paVar7,0x1000);
        }
        else {
          if (cVar8 != '\f') {
            if (cVar8 != '\x0f') goto LAB_1008_8771;
            draw_op_1008_8288(param_4._2_2_,paVar7,s_tile2_bmp_1050_1538);
          }
        }
      }
    }
    else {
      if (param_4 == 0x200) {
        if (((iVar3 + 0x4) & 0x1) != 0x0) {
          GetClientRect16(ctx.s_tile2_bmp_1050_1538,&local_a);
          iVar2 = (iVar3 + 0x4);
          pbVar1 = (iVar3 + 0x4);
          *pbVar1 = *pbVar1 & 0xf3;
          BVar4 = PtInRect16(s_tile2_bmp_1050_1538,
                             (POINT16)CONCAT22(param_2,param_1));
          if (BVar4 == 0x0) {
            pbVar1 = (iVar3 + 0x4);
            *pbVar1 = *pbVar1 | 0x2;
          }
          else {
            if (param_2 < iStack4 >> 0x1) {
              pbVar1 = (iVar3 + 0x4);
              *pbVar1 = *pbVar1 | 0x4;
            }
            else {
              pbVar1 = (iVar3 + 0x4);
              *pbVar1 = *pbVar1 | 0x8;
            }
            pbVar1 = (iVar3 + 0x4);
            *pbVar1 = *pbVar1 & 0xfd;
          }
          if ((iVar3 + 0x4) != iVar2) {
            InvalidateRect16(ctx.s_tile2_bmp_1050_1538,
                             (&ctx.PTR_LOOP_1050_0000 + 0x1),0x0);
            UpdateWindow16(ctx.s_tile2_bmp_1050_1538);
          }
        }
      }
      else {
        if (param_4 < 0x201) {
          uVar5 = param_4 - 0x81;
          if (uVar5 == 0x0) goto LAB_1008_8560;
          if (param_4 != 0x113) {
//LAB_1008_8771:
            DefWindowProc16(ctx.s_tile2_bmp_1050_1538,param_1,param_2,
                            CONCAT13((param_4 >> 0x8),CONCAT12(cVar8,param_3)));
            return;
          }
          if (param_3 == 0xfa6) {
            KillTimer16(ctx.s_tile2_bmp_1050_1538,0xfa6);
            SetTimer16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,
                       (LPVOID)(&ctx.PTR_LOOP_1050_0000 + 0x1));
          }
          if (((iVar3 + 0x4) & 0x2) == 0x0) {
            send_msg_1008_84ba(param_4._2_2_,paVar7,s_tile2_bmp_1050_1538);
          }
        }
        else {
          if (param_4 != 0x201) {
            if (param_4 == 0x202) {
              KillTimer16(ctx.s_tile2_bmp_1050_1538,0xfa6);
              KillTimer16(ctx.s_tile2_bmp_1050_1538,0xfa7);
              ReleaseCapture16();
              uVar5 = (iVar3 + 0x4);
              if (((uVar5 & 0x1) != 0x0) && ((uVar5 & 0xfffd) != 0x0)) {
                pbVar1 = (iVar3 + 0x4);
                *pbVar1 = *pbVar1 & 0xf2;
                InvalidateRect16(ctx.s_tile2_bmp_1050_1538,
                                 (&ctx.PTR_LOOP_1050_0000 + 0x1),0x0);
                UpdateWindow16(ctx.s_tile2_bmp_1050_1538);
              }
              SendMessage16(ctx.s_tile2_bmp_1050_1538,(iVar3 + 0x2),0x0,
                            0x11100f9);
              return;
            }
            if (param_4 != 0x203) goto LAB_1008_8771;
          }
          pbVar1 = (iVar3 + 0x4);
          *pbVar1 = *pbVar1 | 0x1;
          GetClientRect16(ctx.s_tile2_bmp_1050_1538,&local_a);
          if (param_2 < (iStack4 >> 0x1)) {
            pbVar1 = (iVar3 + 0x4);
            *pbVar1 = *pbVar1 | 0x4;
          }
          else {
            pbVar1 = (iVar3 + 0x4);
            *pbVar1 = *pbVar1 | 0x8;
          }
          send_msg_1008_84ba(param_4._2_2_,paVar7,s_tile2_bmp_1050_1538);
          SetTimer16(ctx.s_tile2_bmp_1050_1538,0x0,0x0,
                     (LPVOID)(s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x1c));
          InvalidateRect16(ctx.s_tile2_bmp_1050_1538,
                           (&ctx.PTR_LOOP_1050_0000 + 0x1),0x0);
          UpdateWindow16(ctx.s_tile2_bmp_1050_1538);
          SetCapture16(ctx.s_tile2_bmp_1050_1538);
        }
      }
    }
  }
  return;
}


pub fn send_msg_1008_9640(param_1: u32,param_2: u16,param_3: HWND16)
{
  if ((param_1 + 0x8) != 0x0) {
    SendMessage16(param_3,0x0,0x0,CONCAT22(0x86,param_2));
  }
  return;
}



pub fn set_win_text_1008_9664(param_1: u32,param_2: u16,char *param_3)
{
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | (param_1 + 0xa)),param_3);
  SetWindowText16(0x1000,param_1 + 0xa);
  return;
}


pub fn show_win_1008_96ae(param_1: u32,param_2: i16,param_3: HWND16) -> bool

{
  let BVar1: bool;
  
  if ((param_1 + 0x8) != 0x0) {
    BVar1 = ShowWindow16(param_3,param_2);
    return BVar1;
  }
  return 0x0;
}



void 
win_ui_reg_class_1008_96d2
          (param_1: &mut Struct20,HINSTANCE16 in_h_inst_2,WNDCLASS16 *in_wnd_class_3)

{
  let BVar1: bool;
  ATOM AVar2;
  let name_1c: u16;
  let uStack26: u16;
  let uStack24: u16;
  let uStack22: u32;
  let puStack18: *mut u8;
  let uStack16: u16;
  let uStack14: u16;
  let uStack12: u16;
  let uStack10: u32;
  let iStack6: i16;
  let uStack4: u16;
  
  iStack6 = param_1 + 0x5b;
  BVar1 = GetClassInfo16(in_h_inst_2,(SEGPTR)&name_1c,in_wnd_class_3);
  if (BVar1 == 0x0) {
    name_1c = (param_1 + 0xc8);
    uStack26 = 0x5632;
    uStack24 = 0x1008;
    uStack22 = 0x40000;
    puStack18 = ctx.PTR_LOOP_1050_038c;
    uStack16 = (param_1 + 0xc2);
    uStack14 = (param_1 + 0xc4);
    uStack12 = (param_1 + 0xc6);
    uStack10 = 0x0;
    uStack4 = param_1._2_2_;
    AVar2 = RegisterClass16(s_tile2_bmp_1050_1538);
    if (AVar2 == 0x0) {
      fn_ptr_op_1000_24cd(0x0,&stack0xfffe);
    }
  }
  return;
}



pub fn create_window_ex_1008_9760(astruct *in_struct_1,param_2: u16)
{
  let uVar1: u32;
  let window_handle: HWND16;
  astruct *struct_1;
  let mut class_name: String;
  
  class_name = (in_struct_1 >> 0x10);
  struct_1 = (astruct *)in_struct_1;
  if (struct_1.field_0x8 == 0x0) {
    uVar1 = struct_1.field_0xac;
    window_handle =
         CreateWIndowEx16(CONCAT22(struct_1,param_2),class_name,PTR_LOOP_1050_038c,
                          CONCAT22(struct_1.field_0xbc,struct_1.field_0xca),
                          struct_1.field_0xba,struct_1.field_0xb8,struct_1.field_0xb6,
                          struct_1.field_0xb4,uVar1,
                          (uVar1 >> 0x10),0x39e,(LPVOID)ctx.data_seg)
    ;
    struct_1.field_0x8 = window_handle;
  }
  if (struct_1.field_0x8 == 0x0) {
    fn_ptr_op_1000_24cd(0x0,&stack0xfffe);
  }
  return;
}



pub fn begin_end_paint_1008_97c8(param_1: HWND16)
{
  let local_22: PAINTSTRUCT16;
  
  BeginPaint16(param_1,&local_22);
  EndPaint16(ctx.s_tile2_bmp_1050_1538,&local_22);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 
unk_win_op_1008_97f2
          (param_1: *mut u32,param_2: &mut i16,WPARAM16 param_3,param_4: *mut u8,param_5: u16,
          param_6: HWND16)

{
  let ppcVar1: u32;
  let BVar2: bool;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let msg: u16;
  let wparam: u16;
  let unaff_SS: u16;
  let uVar6: u32;
  let uVar7: u8;
  let uVar8: u8;
  let cVar9: u8;
  
  msg = param_1;
  wparam = (param_1 >> 0x10);
  if (param_5 == 0x2b) {
    if (*param_2 == 0x4) {
      win_ui_get_prop_op_1040_9566
                (CONCAT22(param_3,param_2),&ctx.PTR_LOOP_1050_1040);
    }
    else {
      ppcVar1 = (*param_1 + 0x70);
      (**ppcVar1)();
    }
    uVar5 = 0x1;
//     TODO: goto LAB_1008_9a95;
  }
  uVar8 = (param_1 >> 0x10);
  uVar7 = SUB41(param_1,0x0);
  if (0x2b < param_5) {
    cVar9 = param_5;
    if (param_5 == 0x112) {
      if ((ctx.PTR_LOOP_1050_039a == 0x0) &&
         (ppcVar1 = (*param_1 + 0x48), iVar4 = (**ppcVar1)(), iVar4 != 0x0))
      {
        make_def_wnd_proc_1008_9ce6
                  (msg,wparam,param_2,param_3,
                   CONCAT13(0x1,CONCAT12(cVar9,param_4)),param_6);
      }
    }
    else {
      if (param_5 < 0x113) {
        if (param_5 == 0x86) {
          ppcVar1 = (*param_1 + 0x80);
          uVar6 = (**ppcVar1)();
          return uVar6;
        }
        if (param_5 < 0x87) {
          if (param_5 == 0x85) {
            ppcVar1 = (*param_1 + 0x7c);
            uVar6 = (**ppcVar1)();
            return uVar6;
          }
          if (param_5 < 0x86) {
            if (cVar9 == '7') {
              return (msg + 0xc2);
            }
            if (cVar9 == 'A') {
              ppcVar1 = (*param_1 + 0x2c);
              (**ppcVar1)(param_6,param_1);
//               TODO: goto switchD_1008_9b30_caseD_1;
            }
          }
//           TODO: goto switchD_1008_9b30_caseD_4;
        }
        if (param_5 == 0x100) {
          if (ctx.PTR_LOOP_1050_039a == 0x0) {
            ppcVar1 = (*param_1 + 0x6c);
            (**ppcVar1)();
          }
        }
        else {
          if (param_5 == 0x102) {
            if (ctx.PTR_LOOP_1050_039a == 0x0) {
              ppcVar1 = (*param_1 + 0x68);
              (**ppcVar1)();
            }
          }
          else {
            if (param_5 != 0x111) goto switchD_1008_9b30_caseD_4;
            if ((param_4 != ctx.PTR_LOOP_1050_039c) &&
               (ctx.PTR_LOOP_1050_039a == 0x0)) {
              if (param_2 == 0x0) {
                ppcVar1 = (*param_1 + 0x40);
                (**ppcVar1)();
              }
              else {
                ppcVar1 = (*param_1 + 0x44);
                (**ppcVar1)();
              }
            }
          }
        }
      }
      else {
        if (param_5 == 0x204) {
          if (ctx.PTR_LOOP_1050_039a == 0x0) {
            ppcVar1 = (*param_1 + 0x60);
            (**ppcVar1)();
          }
        }
        else {
          if (param_5 < 0x205) {
            if (param_5 == 0x113) {
              if (ctx.PTR__LOOP_1050_0388 != 0x0) {
                pass1_1008_932a(ctx.PTR__LOOP_1050_0388,unaff_SS);
              }
            }
            else {
              if (param_5 == 0x117) {
                if (param_3 == 0x0) {
                  ppcVar1 = (*param_1 + 0x4c);
                  (**ppcVar1)();
                }
                else {
                  ppcVar1 = (*param_1 + 0x20);
                  (**ppcVar1)();
                }
              }
              else {
                if (param_5 != 0x201) goto switchD_1008_9b30_caseD_4;
                if (ctx.PTR_LOOP_1050_039a == 0x0) {
                  ppcVar1 = (*param_1 + 0x5c);
                  (**ppcVar1)();
                }
              }
            }
          }
          else {
            if (param_5 == 0x210) {
              ppcVar1 = (*param_1 + 0x64);
              (**ppcVar1)();
            }
            else {
              if (param_5 == 0x30f) {
//LAB_1008_9af8:
                ppcVar1 = (*param_1 + 0x8c);
                iVar4 = (**ppcVar1)(param_6,param_1);
//LAB_1008_9ada:
                return iVar4;
              }
              if (param_5 == 0x311) {
                ppcVar1 = (*param_1 + 0x88);
                iVar4 = (**ppcVar1)();
                if (iVar4 != 0x0) goto LAB_1008_9af8;
              }
              else {
                if (param_5 != 0x3b9) goto switchD_1008_9b30_caseD_4;
                ppcVar1 = (*param_1 + 0x24);
                (**ppcVar1)();
              }
            }
          }
        }
      }
    }
//     TODO: goto switchD_1008_9b30_caseD_1;
  }
  if (false) {
switchD_1008_9b30_caseD_4:
    if ((param_5 < 0x400) || (0x7ffe < param_5)) {
      uVar6 = make_def_wnd_proc_1008_9ce6
                        (msg,wparam,param_2,param_3,CONCAT22(param_5,param_4),
                         param_6);
      return uVar6;
    }
    ppcVar1 = (*param_1 + 0x28);
    (**ppcVar1)(param_6,uVar7,uVar8,param_2,param_3,CONCAT22(param_5,param_4))
    ;
  }
  else {
    param_6 = 0x1008;
    switch(param_5) {
    case 0x1:
      break;
    case 0x2:
      ppcVar1 = (*param_1 + 0x3c);
      (**ppcVar1)(0x1008,param_1);
      SetWindowLong16(0x1008,0x0,0x0);
      BVar2 = IsWindow16(ctx.s_tile2_bmp_1050_1538);
      if (BVar2 != 0x0) {
        PostMessage16(ctx.s_tile2_bmp_1050_1538,msg,wparam,0x11100c7);
      }
      break;
    case 0x3:
      ppcVar1 = (*param_1 + 0x54);
      (**ppcVar1)(0x8,uVar7,wparam,param_3,param_2);
      break;
    default:
//       TODO: goto switchD_1008_9b30_caseD_4;
    case 0x5:
      ppcVar1 = (*param_1 + 0x58);
      (**ppcVar1)(0x8,uVar7,uVar8,param_3,param_2,param_4);
      break;
    case 0x7:
      ppcVar1 = (*param_1 + 0x50);
      (**ppcVar1)(0x8,param_1,param_4);
      break;
    case 0x8:
      ppcVar1 = (*param_1 + 0x74);
      (**ppcVar1)(0x8,param_1,param_4);
      break;
    case 0xd:
      ppcVar1 = (*param_1 + 0x84);
      iVar4 = (**ppcVar1)(0x8,uVar7,uVar8,param_2,CONCAT12(param_4._0_1_,param_3));
//       TODO: goto LAB_1008_9ada;
    case 0xf:
      ppcVar1 = (*param_1 + 0x34);
      (**ppcVar1)(0x1008,param_1);
      break;
    case 0x10:
      ppcVar1 = (*param_1 + 0x38);
      uVar6 = (**ppcVar1)(0x1008,param_1);
      return uVar6;
    case 0x19:
      ppcVar1 = (*param_1 + 0x78);
      uVar3 = (**ppcVar1)(0x8,uVar7,uVar8,param_2,CONCAT12(param_4._0_1_,param_3));
      return CONCAT22(0x1050,uVar3);
    case 0x1c:
      ppcVar1 = (*param_1 + 0x30);
      (**ppcVar1)(0x8,param_1,param_4);
    }
  }
switchD_1008_9b30_caseD_1:
  uVar5 = 0x0;
//LAB_1008_9a95:
  return uVar5;
}


pub fn get_stock_obj_1008_9c56(param_1: i16)
{
  GetStockObject16(param_1);
  return;
}


LRESULT 
make_def_wnd_proc_1008_9ce6
          (param_1: u16,param_2: u16,u16 in_msg_3,WPARAM16 param_4,LPARAM param_5,
          in_hwnd_5: HWND16)

{
  LRESULT LVar1;
  
  LVar1 = DefWindowProc16(in_hwnd_5,in_msg_3,param_4,param_5);
  return LVar1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
post_win_msg_1008_a0e4
          (param_1: &mut Struct67,param_2: i32,param_3: i16,param_4: u16,param_5: u32,
          param_6: i16,param_7: HWND16,param_8: u16)

{
  let puVar1: u32;
  let ppcVar2: u32;
  let uVar3: u16;
  let bVar4: bool;
  let puVar4: &mut Struct68;
  let uVar5: &mut Struct66;
  let extraout_DX: u16;
  let uVar7: u16;
  let iVar7: &mut Struct67;
  let uVar6: &mut Struct67;
  let paStack14: &mut Struct99;
  let local_a: [u8;8];
  
  uVar6 = (param_1 >> 0x10);
  iVar7 = param_1;
  pass1_1008_5784(CONCAT22(param_8,local_a),iVar7.field_0xa);
  bVar4 = false;
  do {
    puVar4 = local_a;
    pass1_1008_5b12(puVar4,param_8);
    if ((extraout_DX | puVar4) == 0x0) goto LAB_1008_a146;
  } while (puVar4.field_0x4 != param_6);
  puVar4.field_0xc = puVar4.field_0xc + param_3;
  puVar4.field_0xe = puVar4.field_0xe + param_2;
  bVar4 = true;
//LAB_1008_a146:
  if (!bVar4) {
    param_7 = 0x1000;
    paStack14 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_03a0);
    uVar7 = (paStack14 >> 0x10);
    uVar3 = paStack14;
    if ((uVar7 | uVar3) == 0x0) {
      paStack14 = 0x0;
    }
    else {
      paStack14.field_0x0 = 0x389a;
      (uVar3 + 0x2) = 0x1008;
      (uVar3 + 0x4) = param_6;
      (uVar3 + 0x6) = param_5;
      (uVar3 + 0xa) = param_4;
      (uVar3 + 0xc) = param_3;
      (uVar3 + 0xe) = param_2;
      paStack14.field_0x0 = 0xad8e;
      (uVar3 + 0x2) = 0x1008;
    }
    puVar1 = iVar7.field_0xa;
    ppcVar2 = (*iVar7.field_0xa + 0x8);
    (**ppcVar2)(0x1000,puVar1,(puVar1 >> 0x10),paStack14,
                (paStack14 >> 0x10));
  }
  if (param_6 == 0x14) {
    PostMessage16(param_7,0x0,0x0,0x11100fc);
  }
  return;
}


void 
pass1_1008_a1f0(param_1: u16,param_2: u16,param_3: u8,param_4: u32,param_5: *mut u16,
               param_6: *mut u16,param_7: *mut u16,param_8: *mut u16)

{
  let uVar1: u32;
  let ppcVar2: u32;
  let uVar3: u32;
  let uVar4: u16;
  let uVar5: u16;
  let extraout_DX: u16;
  let uVar6: u16;
  let puVar7: *mut u8;
  let uVar8: u16;
  let iVar9: i16;
  let in_buf_len_5: *mut u8;
  let uVar10: u16;
  let puVar11: *mut u16;
  let mut pcVar12: String; 
  let uVar13: u16;
  let uVar14: u8;
  let uVar15: u8;
  char local_106 [0x100];
  let puStack6: u32;
  
  uVar4 = 0x0;
  *param_8 = 0x0;
  *param_7 = 0x0;
  *param_6 = 0x0;
  *param_5 = 0x0;
  in_buf_len_5 = (param_4 >> 0x10);
  uVar8 = param_4;
  *(uVar8 + 0xe) = 0x0;
  ppcVar2 = ((uVar8 + 0xa) + 0x10);
  (**ppcVar2)(param_1,(uVar8 + 0xa));
  puStack6 = CONCAT22(extraout_DX,uVar4);
  uVar6 = extraout_DX | uVar4;
  if (uVar6 == 0x0) {
    return;
  }
  *param_8 = (uVar4 + 0x4);
  *param_6 = (uVar4 + 0xa);
  uVar5 = pass1_1008_ab80(uVar8,in_buf_len_5,*param_8);
  *param_5 = uVar5;
  uVar10 = (puStack6 >> 0x10);
  iVar9 = puStack6;
  if (false) goto switchD_1008_a835_caseD_3;
  param_1 = 0x1008;
  uVar13 = ctx._PTR_LOOP_1050_14cc;
  uVar5 = (ctx.PTR__LOOP_1050_14cc >> 0x10);
  switch((iVar9 + 0x4)) {
  case 0x1:
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd1;
//     TODO: goto LAB_1008_a2b1;
  case 0x2:
    uVar1 = (iVar9 + 0x6);
    pass1_1028_e1ec(ctx.PTR__LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
    load_string_1010_84e0
              (0x1010,_PTR_LOOP_1050_14cc,
               (ctx.PTR__LOOP_1050_14cc >> 0x10),0x100,local_106,param_2);
    uVar3 = puStack6 >> 0x10;
    pcVar12 = pass1_1038_4d28(CONCAT13((uVar6 >> 0x8),CONCAT12(uVar6,iVar9)));
    param_1 = 0x1000;
    sys_1000_3f9c((uVar8 + 0xe),in_buf_len_5,local_106,param_2,
                  pcVar12,&stack0xfffe,uVar3,0x1000,param_2,param_3);
    break;
  case 0x5:
//     TODO: goto LAB_1008_a277;
  case 0x6:
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd4;
//LAB_1008_a2b1:
    param_1 = 0x1010;
    *param_6 = 0x1;
    break;
  case 0x7:
//LAB_1008_a277:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    break;
  case 0x9:
    if ((uVar8 + 0x416) == 0x0) {
      (uVar8 + 0x416) = 0x1;
      break;
    }
//     TODO: goto LAB_1008_a35a;
  case 0xb:
    if ((uVar8 + 0x41a) == 0x0) {
      (uVar8 + 0x41a) = 0x1;
      break;
    }
//     TODO: goto LAB_1008_a35a;
  case 0xe:
    if ((uVar8 + 0x41c) == 0x0) {
      (uVar8 + 0x41c) = 0x1;
      break;
    }
//     TODO: goto LAB_1008_a35a;
  case 0x14:
    if ((uVar8 + 0x418) == 0x0) {
      (uVar8 + 0x418) = 0x1;
      load_string_1010_84e0
                (0x1010,_PTR_LOOP_1050_14cc,
                 (ctx.PTR__LOOP_1050_14cc >> 0x10),0x3ff,(uVar8 + 0xe),
                 (short)in_buf_len_5);
      pcVar12 = load_string_1010_847e
                          (ctx.PTR__LOOP_1050_14cc,
                           (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
      puVar7 = (pcVar12 >> 0x10);
      pass1_1000_3cea(param_4 & 0xffff0000 | ZEXT24((uVar8 + 0xe)),(ULONG)pcVar12)
      ;
      *param_7 = 0x4c;
      uVar14 = 0x1;
      uVar15 = 0x0;
      iVar9 = 0xa;
      puVar11 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2b,param_2,puVar7,in_buf_len_5)
      ;
      param_1 = 0x1010;
      pass1_1010_089e(param_2,puVar11,CONCAT11(uVar15,uVar14),iVar9);
      break;
    }
//     TODO: goto LAB_1008_a35a;
  case 0x16:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x28;
    break;
  case 0x17:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2c;
    break;
  case 0x18:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2e;
    break;
  case 0x1b:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x30;
    break;
  case 0x1c:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x32;
    break;
  case 0x1f:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x34;
    break;
  case 0x21:
    if ((uVar8 + 0x41e) == 0x0) {
      (uVar8 + 0x41e) = 0x1;
      break;
    }
//LAB_1008_a35a:
    *param_5 = 0x0;
    break;
  case 0x24:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2a;
    break;
  case 0x31:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x27;
    break;
  case 0x32:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x29;
    break;
  case 0x33:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2b;
    break;
  case 0x34:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2d;
    break;
  case 0x35:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x2f;
    break;
  case 0x36:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x31;
    break;
  case 0x37:
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    pcVar12 = load_string_1010_847e
                        (ctx.PTR__LOOP_1050_14cc,
                         (ctx.PTR__LOOP_1050_14cc >> 0x10),0x1010);
    param_1 = 0x1000;
    pass1_1000_3cea(param_4 & 0xffff0000 | ZEXT24((uVar8 + 0xe)),(ULONG)pcVar12);
    *param_7 = 0x33;
    break;
  case 0x38:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x35;
    break;
  case 0x39:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x36;
    break;
  case 0x3a:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x37;
    break;
  case 0x3b:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x38;
    break;
  case 0x3c:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0x39;
    break;
  case 0x3d:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xce;
    break;
  case 0x3e:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xcf;
    break;
  case 0x3f:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd0;
    break;
  case 0x40:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd1;
    break;
  case 0x41:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd2;
    break;
  case 0x42:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd3;
    break;
  case 0x43:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd5;
    break;
  case 0x44:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd6;
    break;
  case 0x45:
    param_1 = 0x1010;
    load_string_1010_84e0
              (0x1010,uVar13,uVar5,0x3ff,(uVar8 + 0xe),(short)in_buf_len_5);
    *param_7 = 0xd7;
  }
switchD_1008_a835_caseD_3:
  if (puStack6 != 0x0) {
    ppcVar2 = *puStack6;
    (**ppcVar2)(param_1,puStack6,(puStack6 >> 0x10),0x1);
  }
  return;
}



u32 
pass1_1008_a8f4(param_1: u32,param_2: *mut u16,param_3: *mut u16,param_4: *mut u16,
               param_5: u16,param_6: u16,param_7: u16,param_8: u8)

{
  let iVar1: i16;
  let local_6: u32;
  
  iVar1 = &local_6 + 0x2;
  pass1_1008_a1f0(param_6,param_7,param_8,param_1,param_2,
                  CONCAT22(param_7,&local_6),CONCAT22(param_7,iVar1),
                  param_4);
  pass1_1008_944e(param_3,local_6,(local_6 >> 0x10));
  return CONCAT22(param_5,iVar1);
}



pub fn pass1_1008_a930(param_1: u32,param_2: i16,param_3: u16)
{
  let uVar1: u32;
  let ppcVar2: u32;
  let puVar3: *mut u8;
  let extraout_DX: u16;
  let uVar4: u16;
  let iVar5: i16;
  let uVar6: u16;
  let puStack24: *mut u16;
  let puStack18: *mut u16;
  let local_a: [u8;8];
  
  if (param_2 == 0x0) {
    return;
  }
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  pass1_1008_5784(CONCAT22(param_3,local_a),(iVar5 + 0x410));
  do {
    puVar3 = local_a;
    pass1_1008_5b12(puVar3,param_3);
    uVar4 = extraout_DX | puVar3;
    if (uVar4 == 0x0) {
      mem_op_1000_179c(0x6,0x0,0x1000);
      puStack24 = CONCAT22(uVar4,puVar3);
      if ((uVar4 | puVar3) == 0x0) {
        puStack18 = 0x0;
      }
      else {
        *puStack24 = 0x389a;
        (puVar3 + 0x2) = 0x1008;
        (puVar3 + 0x4) = param_2;
        *puStack24 = 0xad8a;
        (puVar3 + 0x2) = 0x1008;
        puStack18 = puStack24;
      }
      uVar1 = (iVar5 + 0x410);
      ppcVar2 = ((iVar5 + 0x410) + 0x8);
      (**ppcVar2)(0x1000,uVar1,(uVar1 >> 0x10),puStack18,
                  (puStack18 >> 0x10));
      return;
    }
  } while ((puVar3 + 0x4) != param_2);
  return;
}



pub fn pass1_1008_a9ec(param_1: u32) -> u16

{
  let uVar1: u32;
  let in_AX: u16;
  let iVar2: i16;
  let uVar3: u16;
  WNDCLASS16 *unaff_SS;
  let uStack4: u16;
  
  uStack4 = 0x0;
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (((iVar2 + 0x414) == 0x0) &&
     (uVar1 = (iVar2 + 0x410), (uVar1 + 0x8) != 0x0)) {
    (iVar2 + 0x414) = 0x1;
    pass1_1008_aa28(param_1 & 0xffff | uVar3 << 0x10,in_AX,unaff_SS);
    uStack4 = in_AX;
  }
  return uStack4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_aa28(param_1: u32,param_2: u16,WNDCLASS16 *param_3)
{
  let ppcVar1: u32;
  let uVar2: u32;
  let extraout_DX: u16;
  let iVar3: i16;
  let uVar4: u16;
  let puStack6: u32;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if ((iVar3 + 0x414) != 0x0) {
    uVar2 = (iVar3 + 0x410);
    if ((uVar2 + 0x8) == 0x0) {
      (iVar3 + 0x414) = 0x0;
      return;
    }
    ppcVar1 = ((iVar3 + 0x410) + 0x10);
    (**ppcVar1)();
    puStack6 = CONCAT22(extraout_DX,param_2);
    if ((extraout_DX | param_2) != 0x0) {
      win_1008_5c5c(param_3,param_2,extraout_DX | param_2,_PTR_LOOP_1050_02a0,
                    (param_2 + 0x4));
      if (puStack6 != 0x0) {
        ppcVar1 = *puStack6;
        (**ppcVar1)();
      }
      return;
    }
  }
  return;
}



pub fn pass1_1008_aaa8(param_1: u16,param_2: u16,param_3: u16) -> u16

{
  let uStack4: u16;
  
  uStack4 = 0x0;
  switch(param_3) {
  case 0x1:
    uStack4 = 0x24;
    break;
  case 0x2:
    uStack4 = 0x16;
    break;
  case 0x3:
    uStack4 = 0x17;
    break;
  case 0x4:
    uStack4 = 0x18;
    break;
  case 0x5:
    uStack4 = 0x1b;
    break;
  case 0x6:
    uStack4 = 0x1c;
    break;
  case 0x7:
    uStack4 = 0x1f;
  }
  return uStack4;
}



pub fn pass1_1008_ab12(param_1: u16,param_2: u16,param_3: u16) -> u16

{
  if (param_3 == 0x37) {
    return 0x22;
  }
  if (param_3 < 0x38) {
    if (param_3 == '\r') {
      return 0xf;
    }
    if (param_3 == '*') {
      return 0x2b;
    }
  }
  return 0x0;
}



pub fn pass1_1008_ab54(param_1: u32) -> u16

{
  let uVar1: u32;
  let uVar2: u16;
  let uStack4: u16;
  
  uStack4 = 0x0;
  uVar2 = (param_1 >> 0x10);
  if (((param_1 + 0xa) != 0x0) &&
     (uVar1 = (param_1 + 0xa), (uVar1 + 0x8) != 0x0)) {
    uStack4 = 0x1;
  }
  return uStack4;
}



pub fn pass1_1008_ab80(param_1: u16,param_2: u16,param_3: u16) -> u16

{
  let uStack4: u16;
  
  uStack4 = 0x0;
  if (true) {
    switch(param_3) {
    case 0x8:
      uStack4 = 0x82;
      break;
    case 0x9:
      uStack4 = 0x7f;
      break;
    case 0xa:
      uStack4 = 0x80;
      break;
    case 0xb:
      uStack4 = 0x84;
      break;
    case 0xc:
      uStack4 = 0x89;
      break;
    case 0xd:
      uStack4 = 0x8a;
      break;
    case 0xe:
      uStack4 = 0x8c;
      break;
    case 0xf:
      uStack4 = 0x8e;
      break;
    case 0x10:
      uStack4 = 0x8f;
      break;
    case 0x11:
      uStack4 = 0x90;
      break;
    case 0x12:
      uStack4 = 0x91;
      break;
    case 0x13:
      uStack4 = 0x95;
      break;
    case 0x14:
      uStack4 = 0x96;
      break;
    case 0x16:
      uStack4 = 0x9b;
      break;
    case 0x17:
      uStack4 = 0x9f;
      break;
    case 0x18:
      uStack4 = 0xa2;
      break;
    case 0x19:
      uStack4 = 0xa4;
      break;
    case 0x1b:
    case 0x1c:
      uStack4 = 0xa7;
      break;
    case 0x1d:
      uStack4 = 0xaa;
      break;
    case 0x1e:
      uStack4 = 0xac;
      break;
    case 0x1f:
      uStack4 = 0xad;
      break;
    case 0x20:
      uStack4 = 0xae;
      break;
    case 0x21:
      uStack4 = 0xb1;
      break;
    case 0x22:
      uStack4 = 0xb3;
      break;
    case 0x23:
      uStack4 = 0xb4;
      break;
    case 0x24:
      uStack4 = 0xb5;
      break;
    case 0x25:
      uStack4 = 0xb6;
      break;
    case 0x26:
      uStack4 = 0xb7;
      break;
    case 0x27:
      uStack4 = 0xab;
      break;
    case 0x28:
      uStack4 = 0xb9;
      break;
    case 0x29:
      uStack4 = 0xba;
      break;
    case 0x2a:
      uStack4 = 0xbc;
      break;
    case 0x2b:
      uStack4 = 0xbe;
      break;
    case 0x2c:
      uStack4 = 0xdf;
      break;
    case 0x2d:
      uStack4 = 0xe0;
    }
  }
  return uStack4;
}



pub fn pass1_1008_ad0c(param_1: *mut u16,param_2: u8) -> u16

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x389a;
  (param_1)[0x1] = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a(param_1,uVar1,0x1000);
  }
  return param_1;
}



pub fn pass1_1008_ad38(param_1: *mut u16,param_2: u8) -> u16

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



pub fn pass1_1008_ad64(param_1: u32,param_2: u8) -> u32

{
  let unaff_SS: u16;
  
  pass1_1008_a086(param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



pub fn pass1_1008_ada2(param_1: *mut u16,param_2: i16) -> u16

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x0;
  (param_1 + 0x2) = 0x0;
  (param_1 + 0x4) = param_2;
  *param_1 = (param_2 * 0x6 + 0x3a4);
  return param_1;
}



pub fn pass1_1008_add2(param_1: *mut u16)
{
  *param_1 = ((param_1 + 0x4) * 0x6 + 0x3a4);
  return;
}



pub fn pass1_1008_adf2(param_1: u32) -> u16

{
  return ((param_1 + 0x4) * 0x6 + 0x3a4);
}



pub fn pass1_1008_ae0c(param_1: u32) -> u16

{
  return ((param_1 + 0x4) * 0x6 + 0x3a6);
}



pub fn pass1_1008_ae26(param_1: &mut i16)
{
  let piVar1: *mut i16;
  let iVar2: i16;
  let iVar3: i16;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  iVar2 = ((iVar3 + 0x4) * 0x6 + 0x3a8);
  if (iVar2 == 0x2) {
    if ((iVar3 + 0x2) == 0x1) {
      *param_1 = *param_1 + -0x1;
      iVar2 = (iVar3 + 0x4) * 0x6;
      piVar1 = (iVar2 + 0x3a4);
      if (*piVar1 != *param_1 && *param_1 <= *piVar1) {
        *param_1 = (iVar2 + 0x3a4) + 0x1;
        (iVar3 + 0x2) = 0x0;
        return;
      }
    }
    else {
      *param_1 = *param_1 + 0x1;
      iVar2 = (iVar3 + 0x4) * 0x6;
      if ((iVar2 + 0x3a6) < *param_1) {
        *param_1 = (iVar2 + 0x3a6) + -0x1;
        (iVar3 + 0x2) = 0x1;
        return;
      }
    }
  }
  else {
    if ((iVar2 != 0x3) && (iVar2 != 0x4)) {
      *param_1 = *param_1 + 0x1;
      iVar2 = (iVar3 + 0x4) * 0x6;
      if ((iVar2 + 0x3a6) < *param_1) {
        *param_1 = (iVar2 + 0x3a4);
      }
    }
  }
  return;
}



pub fn pass1_1008_aed8(param_1: u32) -> bool

{
  if (((param_1 + 0x4) * 0x6 + 0x3a4) != 0x0) {
    return 0x1;
  }
  return 0x0;
}



u32 
pass1_1008_aefe(param_1: *mut u8,param_2: *mut u8,param_3: u16,param_4: *mut u8,param_5: u16
               )

{
  struct_op_1018_4cda(param_1,param_2,param_3);
  CONCAT22(param_2,param_1) = 0xaf7c;
  (param_1 + 0x2) = 0x1008;
  ctx.PTR_LOOP_1050_4230 = param_1;
  ctx.PTR_LOOP_1050_4232 = param_2;
  pass1_1018_4dce(CONCAT22(param_2,param_1),0x1b3,param_4,param_5);
  return CONCAT22(param_2,param_1);
}



pub fn pass1_1008_af38(Struct11 *param_1)
{
  param_1 = 0xaf7c;
  (param_1 + 0x2) = 0x1008;
  clenaup_win_ui_1018_4d22(param_1,0x1018);
  return;
}



pub fn pass1_1008_af56(param_1: u32,param_2: u8) -> u32

{
  pass1_1008_af38(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



pub fn pass1_1008_af94(param_1: &mut Struct643,param_2: u16,param_3: u16)
{
  struct_op_1010_1d48(CONCAT22(param_2,param_1),param_3);
  param_1.field_0xa = 0x0;
  param_1.field_0xe = 0x0;
  param_1.field_0x12 = 0x0;
  param_1.field_0x16 = 0x0;
  param_1.field_0x1a = 0x0;
  param_1.field_0x1e = 0x0;
  param_1.field_0x22 = 0x0;
  CONCAT22(param_2,param_1) = 0xbdcc;
  param_1.field_0x2 = 0x1008;
  return;
}



pub fn pass1_1008_afde(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  let ppcVar3: u32;
  let iVar4: &mut Struct468;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0xbdcc;
  iVar4.field_0x2 = 0x1008;
  puVar1 = iVar4.field_0xa;
  uVar2 = iVar4.field_0xc;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar4.field_0xe;
  uVar2 = iVar4.field_0x10;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar4.field_0x12;
  uVar2 = iVar4.field_0x14;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}



pub fn pass1_1008_b05a(param_1: *mut u16) -> u16

{
  let iVar1: &mut Struct193;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0x389a;
  iVar1.field_0x2 = 0x1008;
  iVar1.field_0x4 = 0x0;
  *param_1 = 0xbdc8;
  iVar1.field_0x2 = 0x1008;
  return param_1;
}



pub fn pass1_1008_b08c(param_1: *mut u16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0xbdc8;
  (iVar1 + 0x2) = 0x1008;
  fn_ptr_1000_17ce((iVar1 + 0x4),0x1000);
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  return;
}



pub fn set_stuct_1008_b0bc(param_1: &mut Struct26)
{
  let iVar1: &mut Struct26;
  let uVar1: u16;
  
  pass1_1008_b05a(param_1);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field_0x8 = 0x0;
  iVar1.field_0xa = 0x0;
  iVar1.field_0xe = 0x0;
  iVar1.field_0x10 = 0x0;
  param_1 = 0xbdc4;
  iVar1.field_0x2 = 0x1008;
  return;
}



pub fn pass1_1008_b0f2(param_1: *mut u16) -> u16

{
  let uVar1: u16;
  
  pass1_1008_b05a(param_1);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x8) = 0x0;
  *param_1 = 0xbdc0;
  (param_1 + 0x2) = 0x1008;
  return param_1;
}



pub fn pass1_1008_b11e(param_1: *mut u16) -> u16

{
  let uVar1: u16;
  
  pass1_1008_b05a(param_1);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x8) = 0x0;
  *param_1 = 0xbddc;
  (param_1 + 0x2) = 0x1008;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_b146(param_1: u32,param_2: u16,param_3: u16)
{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0x16) != 0x0) {
    uVar1 = (iVar2 + 0x16);
    pass1_1030_8344(ctx.PTR__LOOP_1050_5748,
                    (ctx.PTR__LOOP_1050_5748 >> 0x10),
                    (uVar1 + 0xa));
    pass1_1038_3608(CONCAT22(param_3,param_2));
    uVar1 = (iVar2 + 0x16);
    (uVar1 + 0x8) = 0x0;
    uVar1 = (iVar2 + 0x16);
    (uVar1 + 0xa) = 0x0;
    uVar1 = (iVar2 + 0x16);
    (uVar1 + 0xe) = 0x0;
    uVar1 = (iVar2 + 0x16);
    (uVar1 + 0x10) = 0x0;
  }
  return;
}



pub fn pass1_1008_b1a6(param_1: u32,char *param_2)
{
  let lVar1: i32;
  let uVar2: u16;
  let in_DX: u16;
  let iVar3: &mut Struct467;
  let iVar4: &mut Struct466;
  let uVar3: u16;
  let uVar4: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (iVar3.field_0x16 != 0x0) {
    lVar1 = iVar3.field_0x16;
    fn_ptr_1000_17ce((lVar1 + 0x4),0x1000);
    uVar2 = str_op_1008_60e8(param_2,in_DX);
    lVar1 = iVar3.field_0x16;
    uVar4 = (lVar1 >> 0x10);
    iVar4 = lVar1;
    iVar4.field_0x4 = uVar2;
    iVar4.field_0x6 = in_DX;
    iVar3.field_0x16 = 0x0;
  }
  return;
}
