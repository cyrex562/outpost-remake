
pub fn window_op_1008_0af8(astruct *param_1,param_2: U32Ptr,param_3: u16)
{
  let pi_var1: U32Ptr;
  let u_var2: u16;
  let ppc_var3: u32;
  let puVar4: U32Ptr;
  let u_var5: u32;
  let puVar6: U32Ptr;
  let extraout_dx: u16;
  let extraout_DX_00: U32Ptr;
  let uVar7: u16;
  let extraout_DX_01: U32Ptr;
  let i_var8: i16;
  let uVar9: u16;
  let u_var10: u16;
  let unaff_SS: u16;
  let puVar11: U32Ptr;
  let uVar12: u16;
  let uVar13: u16;
  let uVar14: u8;
  let paStack6: &mut Struct20;
  
  create_window_ex_1008_9760(param_1,param_3);
 // uVar9 = (param_1 >> 0x10);
  i_var8 = param_1;
  puVar4 = (i_var8 + 0x8);
  ctx.PTR_LOOP_1050_0396 = puVar4;
  mem_op_1000_179c(0x12,param_2,0x1000);
  puVar6 = (param_2 | puVar4);
  if (puVar6 != 0x0) {
    puVar11 = pass1_1008_91ba(CONCAT22(param_2,puVar4),0x1000);
   // puVar6 = (puVar11 >> 0x10);
    puVar4 = puVar11;
  }
  mem_op_1000_179c(0x6,puVar6,0x1000);
  if ((puVar6 | puVar4) == 0x0) {
    (i_var8 + 0xe0) = 0x0;
  }
  else {
    pass1_1008_392e(CONCAT22(puVar6,puVar4),(i_var8 + 0x8));
    (i_var8 + 0xe0) = puVar4;
    (i_var8 + 0xe2) = extraout_dx;
  }
  ppc_var3 = (param_1 + 0x14);
  (**ppc_var3)(0x1000,param_1,0x0,0x15a,ctx.data_seg);
  u_var10 = 0x1000;
  puVar6 = extraout_DX_00;
  mem_op_1000_179c(0xec,extraout_DX_00,0x1000);
  paStack6 = CONCAT22(puVar6,puVar4);
  uVar7 = puVar6 | puVar4;
  if (uVar7 == 0x0) {
    (i_var8 + 0xe4) = 0x0;
  }
  else {
    pi_var1 = (i_var8 + 0xcc);
    *pi_var1 = *pi_var1 + 0x1;
    u_var10 = 0x1020;
    pass1_1020_08b6(unaff_SS,paStack6,(i_var8 + 0xcc),param_1);
    (i_var8 + 0xe4) = puVar4;
    (i_var8 + 0xe6) = uVar7;
  }
  if ((i_var8 + 0xce) != 0x0) {
    ppc_var3 = ((i_var8 + 0xce) + 0x10);
    (**ppc_var3)();
  }
  (i_var8 + 0xce) = (i_var8 + 0xe4);
  uVar14 = 0x1;
  u_var5 = (i_var8 + 0xe4);
  uVar12 = u_var5;
 // uVar13 = (u_var5 >> 0x10);
  ppc_var3 = ((i_var8 + 0xe4) + 0x10);
  (**ppc_var3)();
  u_var5 = (i_var8 + 0xe4);
  u_var2 = (i_var8 + 0xe6);
  (i_var8 + 0xe8) = u_var5;
  ppc_var3 = ((i_var8 + 0xe8) + 0x8);
  (**ppc_var3)(u_var10,(i_var8 + 0xe8),u_var2,uVar12,uVar13,uVar14);
  uVar7 = u_var5;
  ppc_var3 = ((i_var8 + 0xe8) + 0xc);
  (**ppc_var3)();
  pass1_1008_6978(param_1 & 0xffff | uVar9 << 0x10,0x0,
                  (i_var8 + 0xe8),uVar7,extraout_DX_01);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

bool 
mixed_win_op_1008_0c60
          (astruct_72 **param_1,param_2: u16,param_3: bool,param_4: HWND16,
          param_5: u16,param_6: u16)

{
  let ppcVar1: u32;
  let HVar2: HINSTANCE16;
  let BVar3: bool;
  let puVar4: U32Ptr;
  let extraout_dx: U32Ptr;
  let struct_var5: &mut Struct72;
  let unaff_DI: i16;
  let in_AF: u8;
  let u_var5: u32;
  let LVar6: LRESULT;
  let mut pcVar7: String; 
  let puVar8: U32Ptr;
  let uVar9: u16;
  let u_var10: u16;
  let iVar11: i16;
  let uVar12: u16;
  let uVar13: u16;
  ulocal_64: u8 [0x50];
  let uStack20: u32;
  let HStack16: HCURSOR16;
  let HStack14: HCURSOR16;
  let uStack6: u32;
  let struct_var15: &mut Struct72;
  
  uVar9 = param_1;
 // struct_var15 = (param_1 >> 0x10);
  if (false) {
switchD_1008_1091_caseD_69:
    if (((uVar9 + 0xea) | (uVar9 + 0xe8)) == 0x0) {
      return 0x0;
    }
    u_var5 = (uVar9 + 0xe8);
    ppcVar1 = ((uVar9 + 0xe8) + 0x40);
    BVar3 = (**ppcVar1)(param_4,u_var5,(u_var5 >> 0x10),param_2);
    return BVar3;
  }
  param_4 = 0x1008;
  switch(param_2) {
  0x64 =>
    BVar3 = pass1_1008_07d8(uVar9,param_3,param_6,param_5);
    win_ui_cursor_op_1008_2e9a(param_1,param_5);
    return BVar3;
  0x65 =>
    pass1_1008_3018(param_1,param_6,unaff_DI,param_5);
    return param_3;
  0x66 =>
    pass1_1008_30cc(param_1,param_3,param_6,unaff_DI,param_5);
    return param_3;
  0x67 =>
    iVar11 = win_ui_op_1008_2b54(param_3,param_6,param_5);
    if (iVar11 == 0x0) {
      return 0x0;
    }
  0xee =>
    uVar13 = 0x0;
    u_var10 = 0x10;
//     TODO: goto LAB_1008_0d18;
  0x68 =>
    pass1_1030_8344(ctx.PTR_LOOP_1050_5748,
                    (ctx.PTR_LOOP_1050_5748 >> 0x10),0x4000001);
    puVar4 = (param_6 | param_3);
    if (puVar4 == 0x0) {
      return param_3;
    }
    if (ctx.PTR_LOOP_1050_4fe8 != 0x0) {
      pcVar7 = load_string_1010_847e
                         (ctx.PTR_LOOP_1050_14cc,(ctx.PTR_LOOP_1050_14cc >> 0x10),
                          0x1010);
      BVar3 = MessageBox16(0x1010,&ctx.PTR_LOOP_1050_0010,pcVar7,
                           (pcVar7 >> 0x10));
      return BVar3;
    }
    HStack14 = LoadCursor16(0x1030,0x7f02);
    HStack16 = SetCursor16(ctx.s_tile2_bmp_1050_1538);
    uStack20 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x29,param_5,puVar4,unaff_DI);
    pass1_1018_262e(uStack20);
    pass1_1030_838e(ctx.PTR_LOOP_1050_5748,param_5,in_AF);
    uVar13 = (ctx.PTR_LOOP_1050_5748 >> 0x10);
    (ctx.PTR_LOOP_1050_5748 + 0x8) = 0x1;
    pass1_1030_8326();
    pcVar7 = load_string_1010_847e
                       (ctx.PTR_LOOP_1050_14cc,(ctx.PTR_LOOP_1050_14cc >> 0x10),
                        0x1010);
    sys_1000_3f9c(local_64,param_5,0x19c,ctx.data_seg,pcVar7
                  ,&stack0xfffe,uVar13,0x1000,param_5,in_AF);
    ppcVar1 = (*param_1 + 0x14);
    (**ppcVar1)(0x1000,param_1,0x0,0x9c,param_5);
    puVar8 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0,0x37,param_5,extraout_dx,unaff_DI);
    pass1_1008_a9ec(puVar8);
    param_4 = ctx.s_tile2_bmp_1050_1538;
    SetCursor16(0x1010);
    uVar13 = 0xfc;
    u_var10 = 0x111;
//     TODO: goto LAB_1008_0e3d;
  default:
//     TODO: goto switchD_1008_1091_caseD_69;
  0x6e =>
    iVar11 = 0x2;
//     TODO: goto LAB_1008_0cba;
  0x6f =>
    uStack6 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc,0x1f8,param_5);
    BVar3 = WinHelp16(0x1010,0x0,0x0,CONCAT22(uStack6,0x3));
    return BVar3;
  0x70 =>
    iVar11 = 0x1;
//LAB_1008_0cba:
    u_var5 = pass1_1038_af40(ctx.PTR_LOOP_1050_5b7c,(uVar9 + 0x8),iVar11,param_6,
                            uVar9,&ctx.PTR_LOOP_1050_1038,param_5);
    return u_var5;
  0x71 =>
    HVar2 = WinExec16(0x1008,0x3);
    return HVar2;
  0x79 =>
    BVar3 = post_msg_1008_2d22(param_1);
    return BVar3;
  0x7a =>
    uVar12 = 0xb;
//     TODO: goto LAB_1008_0f3e;
  0x7b =>
    uVar12 = 0x1e;
//     TODO: goto LAB_1008_0f3e;
  0x7c =>
    uVar12 = 0x1f;
//     TODO: goto LAB_1008_0f3e;
  0x7d =>
    uVar12 = 0x21;
//     TODO: goto LAB_1008_0f3e;
  0x7e =>
    uVar12 = 0x35;
//     TODO: goto LAB_1008_0f3e;
  0x7f =>
    uVar13 = 0x39;
    break;
  0x80 =>
    uVar12 = 0x22;
//     TODO: goto LAB_1008_0f3e;
  0x81 =>
    uVar13 = 0x36;
    break;
  0x82 =>
    uVar13 = 0x37;
    break;
  0x83 =>
    uVar13 = 0x38;
    break;
  0x84 =>
    uVar13 = 0x3a;
    break;
  0x85 =>
    uVar13 = 0x3b;
    break;
  0x86 =>
    uVar13 = 0x3c;
    break;
  0x87 =>
    uVar13 = 0x3d;
    break;
  0x88 =>
    uVar13 = 0x3e;
    break;
  0x89 =>
    uVar13 = 0x3f;
    break;
  0x8a =>
    uVar13 = 0x40;
    break;
  0x8b =>
    uVar12 = 0xc;
//     TODO: goto LAB_1008_0f3e;
  0x8c =>
    uVar13 = 0x41;
    break;
  0x8d =>
    uVar13 = 0x42;
    break;
  0x8e =>
    uVar13 = 0x43;
    break;
  0x8f =>
    uVar13 = 0x44;
    break;
  0x90 =>
    uVar13 = 0x45;
    break;
  0x91 =>
    uVar13 = 0x46;
    break;
  0x92 =>
    uVar13 = 0x47;
    break;
  0x93 =>
    uVar12 = 0x23;
//     TODO: goto LAB_1008_0f3e;
  0x94 =>
    uVar12 = 0x24;
//     TODO: goto LAB_1008_0f3e;
  0x95 =>
    uVar13 = 0x48;
    break;
  0x96 =>
    uVar13 = 0x49;
    break;
  0x97 =>
    uVar13 = 0x4a;
    break;
  0x98 =>
    uVar13 = 0x4b;
    break;
  0x99 =>
    uVar13 = 0x4c;
    break;
  0x9a =>
    uVar12 = 0xd;
//     TODO: goto LAB_1008_0f3e;
  0x9b =>
    uVar13 = 0x4d;
    break;
  0x9c =>
    uVar13 = 0x4e;
    break;
  0x9d =>
    uVar13 = 0x4f;
    break;
  0x9e =>
    uVar13 = 0x50;
    break;
  0x9f =>
    uVar13 = 0x51;
    break;
  0xa0 =>
    uVar12 = 0xe;
//     TODO: goto LAB_1008_0f3e;
  0xa1 =>
    uVar12 = 0xf;
//     TODO: goto LAB_1008_0f3e;
  0xa2 =>
    uVar13 = 0x52;
    break;
  0xa3 =>
    uVar12 = 0x10;
//     TODO: goto LAB_1008_0f3e;
  0xa4 =>
    uVar13 = 0x53;
    break;
  0xa5 =>
    uVar12 = 0x11;
//     TODO: goto LAB_1008_0f3e;
  0xa6 =>
    uVar12 = 0x12;
//     TODO: goto LAB_1008_0f3e;
  0xa7 =>
    uVar13 = 0x57;
    break;
  0xa8 =>
    uVar12 = 0x13;
//     TODO: goto LAB_1008_0f3e;
  0xa9 =>
    uVar12 = 0x14;
//     TODO: goto LAB_1008_0f3e;
  0xaa =>
    uVar13 = 0x58;
    break;
  0xab =>
    uVar13 = 0x63;
    break;
  0xac =>
    uVar13 = 0x59;
    break;
  0xad =>
    uVar13 = 0x5a;
    break;
  0xae =>
    uVar13 = 0x5b;
    break;
  0xaf =>
    uVar13 = 0x15;
    break;
  0xb0 =>
    uVar12 = 0x25;
//     TODO: goto LAB_1008_0f3e;
  0xb1 =>
    uVar13 = 0x5c;
    break;
  0xb2 =>
    uVar13 = 0x16;
    break;
  0xb3 =>
    uVar13 = 0x5d;
    break;
  0xb4 =>
    uVar12 = 0x5e;
//     TODO: goto LAB_1008_0f3e;
  0xb5 =>
    uVar13 = 0x5f;
    break;
  0xb6 =>
    uVar13 = 0x60;
    break;
  0xb7 =>
    uVar13 = 0x61;
    break;
  0xb8 =>
    uVar13 = 0x62;
    break;
  0xb9 =>
    uVar13 = 0x64;
    break;
  0xba =>
    uVar13 = 0x65;
    break;
  0xbb =>
    uVar13 = 0x66;
    break;
  0xbc =>
    uVar13 = 0x67;
    break;
  0xbd =>
    uVar13 = 0x68;
    break;
  0xbe =>
    uVar13 = 0x69;
    break;
  0xbf =>
    uVar12 = 0x17;
//     TODO: goto LAB_1008_0f3e;
  0xc0 =>
    uVar12 = 0x18;
//     TODO: goto LAB_1008_0f3e;
  0xc1 =>
    uVar12 = 0x19;
//     TODO: goto LAB_1008_0f3e;
  0xc2 =>
    uVar12 = 0x1a;
//     TODO: goto LAB_1008_0f3e;
  0xc3 =>
    uVar12 = 0x1b;
//     TODO: goto LAB_1008_0f3e;
  0xc4 =>
    uVar12 = 0x1c;
//     TODO: goto LAB_1008_0f3e;
  0xc5 =>
    uVar12 = 0x1d;
//     TODO: goto LAB_1008_0f3e;
  0xc6 =>
    uVar12 = 0x4;
//     TODO: goto LAB_1008_0f3e;
  0xc8 =>
    uVar12 = 0x3;
//     TODO: goto LAB_1008_0f3e;
  0xc9 =>
    uVar12 = 0x1;
//     TODO: goto LAB_1008_0f3e;
  0xca =>
    uVar12 = 0x5;
//     TODO: goto LAB_1008_0f3e;
  0xcb =>
    pass1_1008_087e(param_3,param_6,param_5,in_AF);
    uVar12 = 0x6;
//     TODO: goto LAB_1008_0f3e;
  0xcc =>
    uVar12 = 0x7;
//     TODO: goto LAB_1008_0f3e;
  0xcd =>
    uVar12 = 0x8;
//     TODO: goto LAB_1008_0f3e;
  0xce =>
    uVar12 = 0x9;
//     TODO: goto LAB_1008_0f3e;
  0xcf =>
    uVar12 = 0xa;
//     TODO: goto LAB_1008_0f3e;
  0xd0 =>
    uVar12 = 0x26;
//     TODO: goto LAB_1008_0f3e;
  0xd1 =>
    uVar12 = 0x27;
//     TODO: goto LAB_1008_0f3e;
  0xd2 =>
    uVar12 = 0x28;
//     TODO: goto LAB_1008_0f3e;
  0xd3 =>
    uVar12 = 0x29;
//     TODO: goto LAB_1008_0f3e;
  0xd4 =>
    uVar12 = 0x2a;
//     TODO: goto LAB_1008_0f3e;
  0xd5 =>
    uVar12 = 0x2b;
//     TODO: goto LAB_1008_0f3e;
  0xd6 =>
    uVar12 = 0x2c;
//     TODO: goto LAB_1008_0f3e;
  0xd7 =>
    uVar12 = 0x2d;
//     TODO: goto LAB_1008_0f3e;
  0xd8 =>
    uVar12 = 0x2e;
//     TODO: goto LAB_1008_0f3e;
  0xd9 =>
    uVar12 = 0x2f;
//     TODO: goto LAB_1008_0f3e;
  0xda =>
    uVar12 = 0x30;
//     TODO: goto LAB_1008_0f3e;
  0xdb =>
    uVar12 = 0x31;
//     TODO: goto LAB_1008_0f3e;
  0xdc =>
    uVar12 = 0x32;
//     TODO: goto LAB_1008_0f3e;
  0xdd =>
    uVar12 = 0x33;
//     TODO: goto LAB_1008_0f3e;
  0xde =>
    uVar12 = 0x34;
//LAB_1008_0f3e:
    cursor_op_1008_2dcc(uVar9,struct_var15,uVar12,0x1008);
    return param_3;
  0xdf =>
    uVar13 = 0x55;
    break;
  0xe0 =>
    uVar13 = 0x56;
    break;
  0x100 =>
    win_1008_5c5c(param_5,param_3,param_6,_PTR_LOOP_1050_02a0,0x1dc);
    return param_3;
  0x12c =>
    uVar13 = 0xf020;
    u_var10 = 0x112;
//LAB_1008_0d18:
    LVar6 = SendMessage16(0x1008,0x0,0x0,CONCAT22(u_var10,uVar13));
    return LVar6;
  0x12e =>
    uVar13 = 0xf060;
    u_var10 = 0x112;
//LAB_1008_0e3d:
    BVar3 = PostMessage16(param_4,0x0,0x0,CONCAT22(u_var10,uVar13));
    return BVar3;
  }
  ui_op_1008_2c4e(uVar9,struct_var15,uVar13,0x1008);
  return param_3;
}


pub fn  switchD_1008:1091::caseD_a7()

{
  let u_var1: u32;
  let unaff_BP: i16;
  let unaff_CS: HINSTANCE16;
  let unaff_SS: u16;
  
  u_var1 = (unaff_BP + 0x6);
  ui_op_1008_2c4e(u_var1,(u_var1 >> 0x10),0x57,unaff_CS);
  return;
}



pub fn  switchD_1008:1091::caseD_aa()

{
  let u_var1: u32;
  let unaff_BP: i16;
  let unaff_CS: HINSTANCE16;
  let unaff_SS: u16;
  
  u_var1 = (unaff_BP + 0x6);
  ui_op_1008_2c4e(u_var1,(u_var1 >> 0x10),0x58,unaff_CS);
  return;
}



pub fn  switchD_1008:1091::caseD_ac()

{
  let u_var1: u32;
  let unaff_BP: i16;
  let unaff_CS: HINSTANCE16;
  let unaff_SS: u16;
  
  u_var1 = (unaff_BP + 0x6);
  ui_op_1008_2c4e(u_var1,(u_var1 >> 0x10),0x59,unaff_CS);
  return;
}



pub fn  switchD_1008:1091::caseD_ad()

{
  let u_var1: u32;
  let unaff_BP: i16;
  let unaff_CS: HINSTANCE16;
  let unaff_SS: u16;
  
  u_var1 = (unaff_BP + 0x6);
  ui_op_1008_2c4e(u_var1,(u_var1 >> 0x10),0x5a,unaff_CS);
  return;
}



pub fn  switchD_1008:1091::caseD_ae()

{
  let u_var1: u32;
  let unaff_BP: i16;
  let unaff_CS: HINSTANCE16;
  let unaff_SS: u16;
  
  u_var1 = (unaff_BP + 0x6);
  ui_op_1008_2c4e(u_var1,(u_var1 >> 0x10),0x5b,unaff_CS);
  return;
}



pub fn  switchD_1008:1091::caseD_b1()

{
  let u_var1: u32;
  let unaff_BP: i16;
  let unaff_CS: HINSTANCE16;
  let unaff_SS: u16;
  
  u_var1 = (unaff_BP + 0x6);
  ui_op_1008_2c4e(u_var1,(u_var1 >> 0x10),0x5c,unaff_CS);
  return;
}



pub fn  switchD_1008:1091::caseD_b3()

{
  let u_var1: u32;
  let unaff_BP: i16;
  let unaff_CS: HINSTANCE16;
  let unaff_SS: u16;
  
  u_var1 = (unaff_BP + 0x6);
  ui_op_1008_2c4e(u_var1,(u_var1 >> 0x10),0x5d,unaff_CS);
  return;
}


pub fn
big_switch_1008_15d4
          (param_1: u16,param_2: u16,param_3: &WNDCLASS16,param_4: u32,param_5: i16)

{
  let pi_var1: U32Ptr;
  let u_var2: u32;
  let u_var3: u16;
  let u_var4: u16;
  let pu_var5: U32Ptr;
  let extraout_dx: u16;
  let puVar6: U32Ptr;
  let uVar7: u16;
  let paVar8: &mut Struct20;
  let paStack32: &mut Struct20;
  let local_e: [u8;8];
  let uStack6: u32;
  
  uStack6 = 0x0;
  u_var4 = param_4;
  pass1_1008_57a4(CONCAT22(param_3,local_e),
                  param_4 & 0xffff0000 | (u_var4 + 0xd2));
  loop {
    pu_var5 = local_e;
    pass1_1008_5b12(pu_var5,param_3);
    puVar6 = (extraout_dx | pu_var5);
    if (puVar6 == 0x0) {
        // goto
        // LAB_1008_162a;
    }
    u_var2 = (pu_var5 + 0x4);
    puVar6 = (pu_var5 + 0x6);
    param_1 = u_var2;
    if ((param_1 + 0xde) != param_5) == false { break; }
  }
  uStack6 = u_var2 & 0xffff | ZEXT24(puVar6) << 0x10;
//LAB_1008_162a:
  if (uStack6 != 0x0) {
    return;
  }
  u_var3 = param_5 - 0x1;
  if (false) {
switchD_1008_2a63_caseD_2:
    param_1 = u_var3;
    debug_print_1008_6048
              (s_OpWnd__getKid__Unknown_target_mo_1050_01a3,param_2,param_3);
    fn_ptr_op_1000_24cd(0x1,&stack0xfffe);
  }
  else {
    param_2 = 0x1008;
   // uVar7 = (param_4 >> 0x10);
    u_var3 = param_1;
    switch(param_5 - 0x1) {
    0x0 =>
      mem_op_1000_179c(0xec,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        pass1_1020_08b6(param_3,paStack32,(u_var4 + 0xcc),param_4);
//LAB_1008_2a35:
        uStack6 = CONCAT22(puVar6,param_1);
//         TODO: goto LAB_1008_2b3a;
      }
      break;
    default:
//       TODO: goto switchD_1008_2a63_caseD_2;
    0x2 =>
      mem_op_1000_179c(0xfa,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        pass1_1018_e91e(paStack32,(u_var4 + 0xcc),u_var4,param_3);
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x3 =>
      mem_op_1000_179c(0xf6,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        pass1_1018_e230(param_3,paStack32,(u_var4 + 0xcc),u_var4);
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x4 =>
      mem_op_1000_179c(0xf6,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        struct_1020_7554(param_3,paStack32,(u_var4 + 0xcc),u_var4);
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x5 =>
      mem_op_1000_179c(0xf8,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        struct_1018_5840(paStack32,(u_var4 + 0xcc),u_var4,param_3);
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x6 =>
      mem_op_1000_179c(0xf6,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        struct_1020_2524(paStack32,(u_var4 + 0xcc),u_var4,param_3);
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x7 =>
      mem_op_1000_179c(0x118,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        unk_draw_op_1020_41c8(paStack32,(u_var4 + 0xcc),u_var4,0x1020);
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x8 =>
      mem_op_1000_179c(0xf6,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        pass1_1018_e5dc(param_3,paStack32,(u_var4 + 0xcc),u_var4);
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x9 =>
      mem_op_1000_179c(0xf6,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        struct_1018_66cc(paStack32,(u_var4 + 0xcc),u_var4,param_3);
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0xa =>
      win_1008_5c5c(param_3,param_1,puVar6,_PTR_LOOP_1050_02a0,0x1d3);
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_6d02(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0xb =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_6d38(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0xc =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_6d6e(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0xd =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_6da4(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0xe =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_6dda(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0xf =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_6e10(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x10 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_6e46(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x11 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_6e7c(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x12 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_6eb2(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x13 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_6ee8(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x14 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_6f1e(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x15 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_6f54(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x16 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_6f8a(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x17 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_6fc0(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x18 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_6ff6(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x19 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_702c(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x1a =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7062(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x1b =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7098(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x1c =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_70ce(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x1d =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7104(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x1e =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_713a(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x20 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7170(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x21 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_745e(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x22 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_71a6(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x23 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_71dc(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x24 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7212(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x25 =>
      mem_op_1000_179c(0x114,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = pass1_1018_c958(paStack32,(u_var4 + 0xcc),param_4,
                                 param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x26 =>
      mem_op_1000_179c(0x114,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = pass1_1018_c9a6(paStack32,(u_var4 + 0xcc),param_4,
                                 param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x27 =>
      mem_op_1000_179c(0x114,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = pass1_1018_c9f4(paStack32,(u_var4 + 0xcc),param_4,
                                 param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x28 =>
      mem_op_1000_179c(0x114,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = pass1_1018_ca48(paStack32,(u_var4 + 0xcc),param_4,
                                 param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x29 =>
      mem_op_1000_179c(0x114,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = pass1_1018_ca96(paStack32,(u_var4 + 0xcc),param_4,
                                 param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x2a =>
      mem_op_1000_179c(0x114,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = pass1_1018_caea(paStack32,(u_var4 + 0xcc),param_4,
                                 param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x2b =>
      mem_op_1000_179c(0x114,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = pass1_1018_cb38(paStack32,(u_var4 + 0xcc),param_4,
                                 param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x2c =>
      mem_op_1000_179c(0x114,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = pass1_1018_cb86(paStack32,(u_var4 + 0xcc),param_4,
                                 param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x2d =>
      mem_op_1000_179c(0x114,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = pass1_1018_cbda(paStack32,(u_var4 + 0xcc),param_4,
                                 param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x2e =>
      mem_op_1000_179c(0x114,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = pass1_1018_cc28(paStack32,(u_var4 + 0xcc),param_4,
                                 param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x2f =>
      mem_op_1000_179c(0x114,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = pass1_1018_cc76(paStack32,(u_var4 + 0xcc),param_4,
                                 param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x30 =>
      mem_op_1000_179c(0x114,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = pass1_1018_ccc4(paStack32,(u_var4 + 0xcc),param_4,
                                 param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x31 =>
      mem_op_1000_179c(0x114,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = pass1_1018_cd12(paStack32,(u_var4 + 0xcc),param_4,
                                 param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x32 =>
      mem_op_1000_179c(0x114,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = pass1_1018_cd60(paStack32,(u_var4 + 0xcc),param_4,
                                 param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x33 =>
      mem_op_1000_179c(0x114,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = pass1_1018_cf74(paStack32,(u_var4 + 0xcc),param_4,
                                 param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x34 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_73c2(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x35 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7494(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x36 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_74ca(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x37 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7500(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x38 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_73f8(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x39 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7536(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x3a =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_756c(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x3b =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = pass1_1018_75a2(paStack32,(u_var4 + 0xcc),param_4,
                                 param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x3c =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = pass1_1018_75d8(paStack32,(u_var4 + 0xcc),param_4,
                                 param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x3d =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_760e(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x3e =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7644(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x3f =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_767a(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x40 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_76b0(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x41 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_76e6(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x42 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_771c(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x43 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7752(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x44 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7788(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x45 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_77be(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x46 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_77f4(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x47 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_782a(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x48 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7860(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x49 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7896(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x4a =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_78cc(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x4b =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7902(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x4c =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7938(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x4d =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_796e(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x4e =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_79a4(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x4f =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_79da(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x50 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7a10(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x51 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7a46(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x52 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7a7c(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x54 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7ab2(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x55 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7ae8(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x56 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7b1e(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x57 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7b54(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x58 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7b8a(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x59 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7bc0(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x5a =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7bf6(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x5b =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7c2c(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x5c =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7c62(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x5d =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7c98(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x5e =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7cce(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x5f =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7d04(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x60 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7d3a(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x61 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7d70(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x62 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7248(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x63 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_727e(paStack32,(u_var4 + 0xcc),param_4,
                                  param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x64 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_72b4(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x65 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_72ea(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x66 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7320(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x67 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_7356(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
      break;
    0x68 =>
      mem_op_1000_179c(0xf2,puVar6,0x1000);
      paStack32 = CONCAT22(puVar6,param_1);
      puVar6 = (puVar6 | param_1);
      if (puVar6 != 0x0) {
        pi_var1 = (u_var4 + 0xcc);
        *pi_var1 = *pi_var1 + 0x1;
        paVar8 = struct_1018_738c(paStack32,(u_var4 + 0xcc),param_4,param_3);
       // puVar6 = (paVar8 >> 0x10);
        param_1 = paVar8;
//         TODO: goto LAB_1008_2a35;
      }
    }
    uStack6 = 0x0;
  }
//LAB_1008_2b3a:
  pass1_1008_6978(param_4,0x0,uStack6,param_1,puVar6);
  return;
}


/*
Unable to decompile 'FUN_1008_1df2'
Cause: 
Low-level Error: Symbol $$undef00000009 extends beyond the end of the address space
*/


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16  win_ui_op_1008_2b54(param_1: u16,param_2: U32Ptr,param_3: u16)

{
  let u_var1: u16;
  let ppcVar2: u32;
  let i_var3: i16;
  let puVar4: U32Ptr;
  let hwnd: HWND16;
  let mut pcVar5: String; 
  let u_var6: u16;
  u32 *local_a6 [0x14];
  let local_56: [u8;50];
  let i_stack6: i16;
  let i_stack4: i16;
  
  i_stack4 = 0x0;
  if (ctx.PTR_LOOP_1050_4230 == 0x0) {
    pcVar5 = load_string_1010_847e
                       (ctx.PTR_LOOP_1050_14cc,
                        (ctx.PTR_LOOP_1050_14cc >> 0x10),0x1010);
    unk_str_op_1000_3d3e(CONCAT22(param_3,local_56),pcVar5);
    pcVar5 = load_string_1010_847e
                       (ctx.PTR_LOOP_1050_14cc,
                        (ctx.PTR_LOOP_1050_14cc >> 0x10),0x1010);
    unk_str_op_1000_3d3e(CONCAT22(param_3,local_a6),pcVar5);
    hwnd = ctx.s_tile2_bmp_1050_1538;
    i_stack4 = MessageBox16(0x1000,(s_New_failed_in_Op__Op_1050_0020 + 0x1),
                           local_a6,param_3);
  }
  else {
    u_var6 = 0xb4;
    hwnd = 0x1000;
    mem_op_1000_179c(0xb4,param_2,0x1000);
    puVar4 = (param_2 | param_1);
    if (puVar4 == 0x0) {
      i_var3 = 0x0;
      puVar4 = 0x0;
    }
    else {
      hwnd = &ctx.PTR_LOOP_1050_1040;
      i_var3 = string_1040_8520(CONCAT22(param_2,param_1),
                               ctx.PTR_LOOP_1050_0396,0x21,0x2,0x57b,0x5f2,puVar4,
                               param_3);
    }
    local_a6[0] = CONCAT22(puVar4,i_var3);
    ppcVar2 = (*local_a6[0] + 0x74);
    i_stack4 = (**ppcVar2)(hwnd,i_var3,puVar4,u_var6,param_1);
  }
  i_stack6 = i_stack4;
  if (i_stack4 != 0x1) {
    i_stack6 = 0x0;
  }
  if (((i_stack6 != 0x0) && (ctx.PTR_LOOP_1050_5748 != 0x0)) &&
     (u_var1 = (ctx.PTR_LOOP_1050_5748 + 0x8),
     local_a6[0] = (local_a6[0] & 0xffff0000 | u_var1),
     u_var1 != 0x0)) {
    PostMessage16(hwnd,0x0,0x0,0x11100b4);
    i_stack6 = 0x0;
  }
  return i_stack6;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn ui_op_1008_2c4e(param_1: i16,param_2: u16,param_3: i16,in_h_instance_4: HINSTANCE16)
{
  let pi_var1: U32Ptr;
  let ppcVar2: u32;
  let HVar3: HCURSOR16;
  let in_DX: u16;
  let u_var4: u16;
  let iVar5: i16;
  let unaff_SS: u16;
  let u_var6: u32;
  let uVar7: u32;
  
  uVar7 = 0x0;
  HVar3 = LoadCursor16(in_h_instance_4,0x7f02);
  uVar7 = uVar7 & 0xffff0000 | HVar3;
  HVar3 = SetCursor16(ctx.s_tile2_bmp_1050_1538);
  u_var6 = CONCAT22(in_DX,HVar3);
  pi_var1 = (param_1 + 0xf2);
  *pi_var1 = *pi_var1 + 0x1;
  iVar5 = param_1;
  if ((param_1 + 0xee) != 0x0) {
    u_var6 = (param_1 + 0xee);
    iVar5 = (param_1 + 0xee);
    ppcVar2 = (iVar5 + 0x90);
    u_var6 = (**ppcVar2)(ctx.s_tile2_bmp_1050_1538,u_var6,(u_var6 >> 0x10),
                        uVar7);
  }
  big_switch_1008_15d4
            (iVar5,s_tile2_bmp_1050_1538,unaff_SS,CONCAT22(param_2,param_1),
             param_3);
 // u_var4 = (u_var6 >> 0x10);
  (param_1 + 0xee) = u_var6;
  (param_1 + 0xf0) = u_var4;
  ppcVar2 = ((param_1 + 0xee) + 0x8);
  (**ppcVar2)(ctx.s_tile2_bmp_1050_1538,(param_1 + 0xee),u_var4);
  if ((param_1 + 0xe8) != 0x0) {
    u_var6 = (param_1 + 0xe8);
    ppcVar2 = ((param_1 + 0xe8) + 0xc);
    (**ppcVar2)(ctx.s_tile2_bmp_1050_1538,u_var6,(u_var6 >> 0x10),0x0);
  }
  show_win_1038_b634(ctx.PTR_LOOP_1050_5b7c,&ctx.PTR_LOOP_1050_1038);
  show_win_1010_7a76((param_1 + 0xf4),0x1010);
  u_var6 = (param_1 + 0xee);
  ppcVar2 = ((param_1 + 0xee) + 0xc);
  (**ppcVar2)(0x1010,u_var6,(u_var6 >> 0x10),0x5);
  BringWindowToTop16(0x1010);
  SetCursor16(ctx.s_tile2_bmp_1050_1538);
  return;
}


pub fn switch_1008_72bc(param_1: u16,param_2: u16,param_3: u16) -> u16

{
  if (ctx.PTR_LOOP_1050_0312 < 0x2) {
    switch(param_3) {
    0x1 =>
      param_3 = 0x1;
      break;
    0x2 =>
      param_3 = 0x2;
      break;
    0x3 =>
      param_3 = 0x3;
      break;
    0x4 =>
      param_3 = 0x5;
      break;
    0x5 =>
      param_3 = 0x4;
      break;
    0x6 =>
      param_3 = 0x6;
      break;
    0x7 =>
      param_3 = 0x7;
      break;
    0x8 =>
      param_3 = 0x8;
      break;
    0x9 =>
      param_3 = 0x9;
      break;
    0xa =>
      param_3 = 0xa;
      break;
    0xb =>
      param_3 = 0xb;
      break;
    0xc =>
      param_3 = 0xc;
      break;
    0xd =>
      param_3 = 0xd;
      break;
    0xe =>
      param_3 = 0xe;
      break;
    0xf =>
      param_3 = 0xf;
      break;
    0x10 =>
      return 0x10;
    0x11 =>
      return 0x11;
    0x12 =>
      return 0x12;
    0x13 =>
      return 0x13;
    default:
      return 0x0;
    }
  }
  return param_3;
}


pub fn pass1_1008_738c(param_1: u16,param_2: u16,param_3: u16) -> u16

{
  let u_var1: u16;
  
  switch(param_3) {
  0x1 =>
    u_var1 = 0x3;
    break;
  0x2 =>
    u_var1 = 0x4;
    break;
  0x3 =>
    return 0x5;
  0x4 =>
    return 0x6;
  0x5 =>
    return 0x8;
  0x6 =>
    return 0x9;
  0x7 =>
    return 0xa;
  default:
    u_var1 = 0x0;
  }
  return u_var1;
}



pub fn  switch_1008_73ea(param_1: u16,param_2: u16,param_3: i16) -> i16

{
  let i_var1: i16;
  let i_stack4: i16;
  
  i_stack4 = 0x0;
  i_var1 = param_3;
  if ((ctx.PTR_LOOP_1050_0312 < 0x2) && (i_var1 = i_stack4, true)) {
    i_var1 = param_3;
    switch(param_3) {
    0x18 =>
    0x19 =>
    0x1a =>
    0x1b =>
    0x1c =>
    0x1d =>
    0x1e =>
    0x1f =>
    0x20 =>
    0x21 =>
    0x22 =>
    0x23 =>
    0x24 =>
    0x25 =>
    0x26 =>
    0x27 =>
    0x28 =>
    0x29 =>
    0x2a =>
    0x2b =>
    0x2c =>
    0x2d =>
    0x2e =>
    0x2f =>
    0x30 =>
    0x31 =>
    0x32 =>
    0x33 =>
    0x34 =>
    0x35 =>
    0x36 =>
    0x37 =>
    0x38 =>
    0x39 =>
    0x3a =>
    0x3b =>
    0x3c =>
      i_var1 = param_3 + 0x3;
      break;
    0x3d =>
    0x3e =>
      i_var1 = param_3 + 0x4;
      break;
    0x3f =>
    0x40 =>
    0x41 =>
    0x42 =>
    0x43 =>
    0x44 =>
    0x45 =>
    0x46 =>
    0x47 =>
    0x48 =>
    0x49 =>
    0x4a =>
    0x4b =>
    0x4c =>
    0x4d =>
    0x4e =>
    0x4f =>
    0x50 =>
    0x51 =>
    0x52 =>
    0x53 =>
    0x54 =>
    0x55 =>
    0x56 =>
    0x57 =>
    0x58 =>
    0x59 =>
    0x5a =>
    0x5b =>
    0x5c =>
    0x5d =>
    0x5e =>
    0x5f =>
    0x60 =>
    0x61 =>
    0x62 =>
    0x63 =>
    0x64 =>
    0x65 =>
    0x66 =>
      i_var1 = param_3 + 0x8;
      break;
    0x67 =>
    0x68 =>
    0x69 =>
    0x6a =>
    0x6b =>
    0x6c =>
    0x6d =>
    0x6e =>
    0x6f =>
    0x70 =>
    0x71 =>
    0x72 =>
    0x73 =>
    0x74 =>
    0x75 =>
    0x76 =>
    0x77 =>
    0x78 =>
    0x79 =>
    0x7a =>
    0x7b =>
    0x7c =>
    0x7d =>
    0x7e =>
    0x7f =>
    0x80 =>
      i_var1 = param_3 + 0x9;
    }
  }
  i_stack4 = i_var1;
  return i_stack4;
}

