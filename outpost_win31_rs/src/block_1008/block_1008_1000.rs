
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn message_box_op_1008_12dc(param_1: *mut astruct_72,mut param_2: u32)

{
  let mut BVar1: bool;
  let mut uVar2: u16;
  let mut in_DX: u16;
  let mut HVar3: HWND16;
  let mut uVar4: u16;
  let mut pcVar5: *mut c_char;
  let mut uVar6: u16;
  let mut hwnd: HWND16;
  let mut uStack16: u32;
  u8 local_c [0x6];
  HCURSOR16 HStack6;
  HCURSOR16 HStack4;

  HStack4 = LoadCursor16(0x7f02,0x0);
  HStack6 = SetCursor16(HStack4);
  str_1008_6d8a(in_DX,CONCAT22(0x1050,local_c),param_2);
  BVar1 = file_fn_1008_6e02((astruct_802 *)CONCAT22(0x1050,local_c));
  uVar4 = (param_1 >> 0x10);
  if (BVar1 == 0) {
    SetCursor16(HStack6);
    pcVar5 = load_string_1010_847e(_u16_1050_14cc,u16_1050_0310);
    HVar3 = (HWND16)(pcVar5 >> 0x10);
    uVar2 = str_op_1008_60e8(HVar3,pcVar5);
    uStack16 = CONCAT22(HVar3,uVar2);
    pcVar5 = load_string_1010_847e(_u16_1050_14cc,0x57b);
    MessageBeep16(0x10);
    MessageBox16(0x10,pcVar5,CONCAT22(HVar3,uVar2),*(HWND16 *)(param_1 + 0x8));
  }
  else {
    (_u16_1050_5748 + 0x8) = 0;
    SetCursor16(HStack6);
    pcVar5 = load_string_1010_847e(_u16_1050_14cc,0x6d3);
    str_op_1008_60e8((pcVar5 >> 0x10),pcVar5);
    pcVar5 = load_string_1010_847e(_u16_1050_14cc,0x57b);
    uVar6 = 0;
    MessageBeep16(0x0);
    hwnd = *(HWND16 *)(param_1 + 0x8);
    HVar3 = hwnd;
    MessageBox16(0x40,pcVar5,CONCAT22(hwnd,uVar6),hwnd);
    uStack16 = CONCAT22(HVar3,hwnd);
  }
  fn_ptr_1000_17ce((uStack16 & 0xffff | HVar3 << 0x10));
  close_file_1008_6dd0(CONCAT22(0x1050,local_c));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1008_1414(mut param_1: u32,param_2: *mut astruct_20,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,
                        mut param_6: u16 )

{
  let mut uVar1: u32;
  code **ppcVar2;
  let mut BVar3: bool;
  let mut uVar4: u16;
  let mut iVar5: i16;
  let mut puVar5: *mut u32;
  let mut extraout_DX: u16;
  let mut uVar6: u32;
  let mut paVar7: *mut Struct57;
  let mut uVar8: u32;
  astruct_20 *iVar17;
  let mut unaff_SI: u16;
  let mut unaff_DI: u16;
  astruct_72 *uVar16;
  let mut puVar9: *mut u32;
  let mut pcVar10: *mut c_char;
  let mut puVar11: *mut u32;
  astruct_27 *paVar12;
  let mut in_stack_0000fe3c: u16;
  let mut in_stack_0000fe3e: u16;
  let mut in_stack_0000fe4e: u16;
  let mut in_stack_0000ff60: u16;
  let mut in_stack_0000ff62: u16;
  let mut in_stack_0000ff66: u16;
  let mut in_stack_0000ff68: u16;
  let mut in_stack_0000ff6a: u16;
  let mut in_stack_0000ff6c: u16;
  let mut in_stack_0000ff72: u16;
  let mut in_stack_0000ff78: u16;
  let mut in_stack_0000ff7c: u16;
  let mut uVar13: u8;
  let mut uVar14: u8;
  let mut uVar15: u16;
  let mut local_2a: u32;
  let mut uStack38: u16;
  let mut iStack36: i16;
  let mut uStack34: u16;
  let mut uStack32: u32;
  let mut uStack28: u32;
  let mut uStack24: u32;
  let mut uStack20: u32;
  let mut uStack16: u32;
  let mut puStack12: *mut u32;
  u8 local_8 [0x6];
  let mut uVar10: u16;

  puVar9 = str_1008_6d8a(param_1,CONCAT22(0x1050,local_8),param_3);
  paVar7 = (astruct_57 *)(param_1 & 0xffff0000 | puVar9 >> 0x10);
  BVar3 = read_file_1008_6e78((astruct_806 *)CONCAT22(0x1050,local_8),unaff_SI,unaff_DI,param_4);
  iVar17 = (astruct_20 *)param_2;
  uVar16 = (astruct_72 *)(param_2 >> 0x10);
  if (BVar3 == 0) {
    if (u16_1050_0310 == 0) {
      u16_1050_0310 = 0x6d4;
    }
    pcVar10 = load_string_1010_847e(_u16_1050_14cc,u16_1050_0310);
    uVar8 = paVar7 & 0xffff0000 | pcVar10 >> 0x10;
    uVar4 = str_op_1008_60e8((pcVar10 >> 0x10),pcVar10);
    uVar15 = uVar8;
    pcVar10 = load_string_1010_847e(_u16_1050_14cc,0x57b);
    paVar7 = (astruct_57 *)(uVar8 & 0xffff0000 | pcVar10 >> 0x10);
    MessageBeep16(0x10);
    MessageBox16(0x10,pcVar10,CONCAT22(uVar15,uVar4),iVar17.field3_0x8);
    fn_ptr_1000_17ce(CONCAT22(uVar15,uVar4));
    fn_ptr_op_1000_24cd(1);
  }
  cursor_op_1008_2dcc(paVar7,param_2,0x8,param_6,param_5);
  puStack12 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_DI,0x2f),in_stack_0000fe4e,
                              in_stack_0000ff72,in_stack_0000ff78,in_stack_0000ff7c);
  paVar7 = (astruct_57 *)(paVar7 & 0xffff0000 | puStack12 >> 0x10);
  uVar6 = (puStack12 + 0x20);
  uStack16 = uVar6;
  pass1_1030_8344(_u16_1050_5748,uVar6);
  uStack20 = uVar6 & 0xffff | paVar7 << 0x10;
  uStack24 = (uVar6 + 0x10);
  iVar5 = (uStack24 + 0x2) + -0x1;
  uVar1 = (&iVar17[0x1].field2_0x4 + 2);
  ppcVar2 = (code **)((&iVar17[0x1].field2_0x4 + 0x2) + 0x4);
  (**ppcVar2)(0x1030,uVar1,(uVar1 >> 0x10),uStack16,(uStack16 >> 0x10),iVar5,0x2);
  pass1_1030_8344(_u16_1050_5748,0x4000001);
  uStack28 = CONCAT22(paVar7,iVar5);
  uVar6 = (iVar5 + 0x10);
  uStack32 = uVar6;
  pass1_1030_8344(_u16_1050_5748,uVar6);
  iStack36 = uVar6;
  uStack34 = SUB42(paVar7,0x0);
  local_2a = (iStack36 + 0xc);
  uStack38 = (iStack36 + 0x10);
  puVar5 = pass1_1030_5b00(uStack20);
  uVar15 = SUB42(&DAT_1050_1050,0x0);
  uVar13 = SUB21(&local_2a,0x0);
  uVar14 = (&local_2a >> 0x8);
  puVar11 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT13(uVar14,CONCAT12(uVar13,puVar5)),
                            in_stack_0000fe3e,in_stack_0000ff62,in_stack_0000ff68,in_stack_0000ff6c);
  paVar7 = (astruct_57 *)(paVar7 & 0xffff0000 | puVar11 >> 0x10);
  pass1_1018_179e(puVar11,CONCAT22(uVar15,CONCAT11(uVar14,uVar13)));
  uVar13 = 0;
  uVar14 = 0x4;
  iVar5 = 0x1b;
  paVar12 = (astruct_27 *)
            mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fe3c,in_stack_0000ff60,
                            in_stack_0000ff66,in_stack_0000ff6a);
  pass1_1010_043a(paVar12,CONCAT13(uVar14,CONCAT12(uVar13,1)),iVar5);
  close_file_1008_6dd0(CONCAT22(0x1050,local_8));
  return;
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn big_switch_1008_15d4(param_1: *mut astruct_20,param_2: *mut astruct_72,i32 param_3)

{
  let mut var3: u16;
  let mut var2: *mut u8;
  let mut var4: u16;
  astruct_20 *var5;
  let mut puVar1: *mut u8;
  let mut uVar3: u16;
  let mut in_EDX: *mut Struct57;
  let mut uVar4: u32;
  let mut var6: u16;
  astruct_20 *paVar2;
  let mut in_stack_0000fe78: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffa6: u16;
  let mut uVar5: u16;
  astruct_20 *paStack32;
  u8 local_e [0x8];
  let mut uStack6: u32;
  let mut uVar2: u32;
  i32 *var_1;
  let mut piVar1: *mut i16;

  uStack6 = 0;
  var3 = param_2;
  var3 = var3 + 0xd2;
  pass1_1008_57a4(CONCAT22(0x1050,local_e),param_2 & 0xffff0000 | var3);
  loop {
    var2 = local_e;
    pass1_1008_5b12(CONCAT22(0x1050,var2));
    uVar3 = in_EDX;
    var5 = (astruct_20 *)(uVar3 | var2);
    uVar4 = in_EDX & 0xffff0000;
    in_EDX = (astruct_57 *)(uVar4 | ZEXT24(var5));
    if (var5 == NULL) goto LAB_1008_162a;
    uVar2 = (var2 + 0x4);
    uVar3 = (var2 + 0x6);
    in_EDX = (astruct_57 *)(uVar4 | uVar3);
    param_1 = (astruct_20 *)uVar2;
  } while (param_1.field164_0xde != param_3);
  uStack6 = uVar2 & 0xffff | uVar3 << 0x10;//
LAB_1008_162a:
  if (uStack6 != 0) {
    return;
  }
  uVar5 = (param_2 >> 0x10);
  switch(param_3 - 1) {
  case 0x0:
    mem_op_1000_179c(0xec,in_EDX);
    puVar1 = (in_EDX | param_1);
    uVar4 = ZEXT24(puVar1);
    if (puVar1 == NULL) {//
LAB_1008_169a:
      puVar1 = uVar4;
      uStack6 = 0;
  // TODO: goto LAB_1008_2b3a;
    }
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    pass1_1020_08b6((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    break;
  default:
    debug_print_1008_6048(in_EDX,s_OpWnd__getKid__Unknown_target_mo_1050_01a3);
    puVar1 = in_EDX;
    fn_ptr_op_1000_24cd(1);
// TODO: goto LAB_1008_2b3a;
  case 0x2:
    mem_op_1000_179c(0xfa,in_EDX);
    puVar1 = (in_EDX | param_1);
    uVar4 = ZEXT24(puVar1);
    if (puVar1 == NULL) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    pass1_1018_e91e(puVar1,(astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),var3,uVar5);
    break;
  case 0x3:
    mem_op_1000_179c(0xf6,in_EDX);
    puVar1 = (in_EDX | param_1);
    uVar4 = ZEXT24(puVar1);
    if (puVar1 == NULL) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    pass1_1018_e230(puVar1,(astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),var3,uVar5);
    break;
  case 0x4:
    mem_op_1000_179c(0xf6,in_EDX);
    puVar1 = (in_EDX | param_1);
    uVar4 = ZEXT24(puVar1);
    if (puVar1 == NULL) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    struct_1020_7554(puVar1,(astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),var3,uVar5);
    break;
  case 0x5:
    mem_op_1000_179c(0xf8,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = in_EDX & 0xffff0000 | uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    struct_1018_5840(uVar4,(astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),var3,uVar5,
                     in_stack_0000fe78,in_stack_0000ff9c,in_stack_0000ffa2,in_stack_0000ffa6);
    puVar1 = uVar4;
    break;
  case 0x6:
    mem_op_1000_179c(0xf6,in_EDX);
    puVar1 = (in_EDX | param_1);
    uVar4 = ZEXT24(puVar1);
    if (puVar1 == NULL) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    struct_1020_2524(puVar1,(astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),var3,uVar5);
    break;
  case 0x7:
    mem_op_1000_179c(0x118,in_EDX);
    puVar1 = (in_EDX | param_1);
    uVar4 = ZEXT24(puVar1);
    if (puVar1 == NULL) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    unk_draw_op_1020_41c8((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),var3,0x1020);
    break;
  case 0x8:
    mem_op_1000_179c(0xf6,in_EDX);
    puVar1 = (in_EDX | param_1);
    uVar4 = ZEXT24(puVar1);
    if (puVar1 == NULL) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    pass1_1018_e5dc(puVar1,(astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),var3,uVar5);
    break;
  case 0x9:
    mem_op_1000_179c(0xf6,in_EDX);
    puVar1 = (in_EDX | param_1);
    uVar4 = ZEXT24(puVar1);
    if (puVar1 == NULL) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    struct_1018_66cc(puVar1,(astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),var3,uVar5);
    break;
  case 0xa:
    win_1008_5c5c(param_1,in_EDX,_u16_1050_02a0,0x1d3);
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_6d02((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0xb:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_6d38((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0xc:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_6d6e((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0xd:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_6da4((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0xe:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_6dda((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0xf:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_6e10((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x10:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_6e46((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x11:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_6e7c((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x12:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_6eb2((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x13:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_6ee8((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x14:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_6f1e((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x15:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_6f54((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x16:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_6f8a((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x17:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_6fc0((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x18:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_6ff6((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x19:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_702c((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x1a:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7062((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x1b:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7098((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x1c:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_70ce((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x1d:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7104((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x1e:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_713a((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2,
                              &DAT_1050_1050);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x20:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7170((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x21:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_745e((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x22:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_71a6((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x23:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_71dc((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x24:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7212((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x25:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = pass1_1018_c958((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x26:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = pass1_1018_c9a6((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x27:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = pass1_1018_c9f4((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x28:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = pass1_1018_ca48((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x29:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = pass1_1018_ca96((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x2a:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = pass1_1018_caea((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x2b:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = pass1_1018_cb38((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x2c:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = pass1_1018_cb86((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x2d:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = pass1_1018_cbda((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x2e:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = pass1_1018_cc28((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x2f:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = pass1_1018_cc76((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x30:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = pass1_1018_ccc4((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x31:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = pass1_1018_cd12((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x32:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = pass1_1018_cd60((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x33:
    mem_op_1000_179c(0x114,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = pass1_1018_cf74((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x34:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_73c2((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x35:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7494((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x36:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_74ca((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x37:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7500((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x38:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_73f8((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x39:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7536((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x3a:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_756c((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x3b:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = pass1_1018_75a2((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x3c:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = pass1_1018_75d8((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x3d:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_760e((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x3e:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7644((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x3f:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_767a((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x40:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_76b0((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x41:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_76e6((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x42:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_771c((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x43:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7752((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x44:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7788((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x45:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_77be((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x46:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_77f4((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x47:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_782a((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x48:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7860((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x49:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7896((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x4a:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_78cc((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x4b:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7902((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x4c:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7938((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x4d:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_796e((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x4e:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_79a4((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x4f:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_79da((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x50:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7a10((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x51:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7a46((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x52:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7a7c((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x54:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7ab2((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x55:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7ae8((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x56:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7b1e((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x57:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7b54((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x58:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7b8a((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x59:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7bc0((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x5a:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7bf6((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x5b:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7c2c((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x5c:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7c62((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x5d:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7c98((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x5e:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7cce((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x5f:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7d04((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x60:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7d3a((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x61:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7d70((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x62:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7248((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x63:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_727e((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x64:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_72b4((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x65:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_72ea((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x66:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7320((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x67:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    piVar1 = (var3 + 0xcc);
    *piVar1 = *piVar1 + 1;
    paVar2 = struct_1018_7356((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
    break;
  case 0x68:
    mem_op_1000_179c(0xf2,in_EDX);
    uVar3 = in_EDX | param_1;
    uVar4 = uVar3;
    if (uVar3 == 0) goto LAB_1008_169a;
    var_1 = (i32 *)(var3 + 0xcc);
    var_1 = var_1 + 1;
    paVar2 = struct_1018_738c((astruct_20 *)CONCAT22(in_EDX,param_1),(var3 + 0xcc),param_2);
    puVar1 = (paVar2 >> 0x10);
    param_1 = (astruct_20 *)paVar2;
  }
  uStack6 = CONCAT22(puVar1,param_1);//
LAB_1008_2b3a:
  pass1_1008_6978(param_1,puVar1,param_2,0x0,uStack6);
  return;
}
