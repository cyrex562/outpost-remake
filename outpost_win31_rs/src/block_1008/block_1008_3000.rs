
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_3018(param_1: *mut u8,mut param_2: u32)

{
  let mut pcVar1: *mut c_char;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut unaff_SI: u16;
  let mut in_stack_0000fd8a: u16;
  let mut in_stack_0000feae: u16;
  let mut in_stack_0000feb4: u16;
  let mut in_stack_0000feb8: u16;
  let mut uStack266: u16;
  let mut uStack262: u32;
  char local_102 [0x100];

  local_102[0] = '\0';
  uStack262 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                              (u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000fd8a,in_stack_0000feae,in_stack_0000feb4
                              ,in_stack_0000feb8);
  uVar2 = (uStack262 >> 0x10);
  iVar3 = uStack262;
  pcVar1 = *(iVar3 + 0x12);
  uVar4 = (iVar3 + 0x14);
  uStack266 = pcVar1;
  if ((uVar4 | uStack266) == 0) {
    pass1_1008_30cc(0x0,uVar4,(astruct_72 *)param_2);
  }
  else {
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_102),*(iVar3 + 0x1a));
    uVar4 = str_op_1000_3da4(CONCAT22(0x1050,local_102));
    if (local_102[uVar4 - 0x1] != '\\') {
      local_102[uVar4] = '\\';
      local_102[uVar4 + 0x1] = '\0';
    }
    pass1_1000_3cea(CONCAT22(0x1050,local_102),pcVar1);
    if (local_102[0] != '\0') {
      message_box_op_1008_12dc((astruct_72 *)param_2,CONCAT22(0x1050,local_102));
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_30cc(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_72)

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  astruct_484 *paVar2;
  let mut in_stack_0000fc90: u16;
  let mut in_stack_0000fdb4: u16;
  let mut in_stack_0000fdba: u16;
  let mut in_stack_0000fdbe: u16;
  let mut puVar3: *mut u8;
  let mut uVar4: u16;
  char local_210 [0xa];
  u8 local_206 [0x100];
  let mut uStack262: u16;
  let mut uStack260: u16;
  char local_102 [0x100];

  local_102[0] = '\0';
  save_file_1008_3178(param_2,param_3,0x2);
  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2 | param_1);
  if ((param_2 | param_1) != 0) {
    uStack262 = param_1;
    uStack260 = param_2;
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_102),CONCAT22(param_2,param_1));
    str_1000_4d58(CONCAT22(0x1050,local_102),NULL,0x0,CONCAT22(0x1050,local_206),
                  (WNDCLASS16 *)CONCAT22(0x1050,local_210));
    if (local_210[0] != '\0') {
      pass1_1000_3cea(CONCAT22(0x1050,local_206),CONCAT22(0x1050,local_210));
    }
    puVar3 = local_206;
    uVar4 = SUB42(&DAT_1050_1050,0x0);
    paVar2 = (astruct_484 *)
             mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(puVar3,0x2),in_stack_0000fc90,
                             in_stack_0000fdb4,in_stack_0000fdba,in_stack_0000fdbe);
    pass1_1010_5f4c((paVar2 >> 0x10),paVar2,CONCAT22(uVar4,puVar3));
    if (local_102[0] != '\0') {
      message_box_op_1008_12dc(param_3,CONCAT22(0x1050,local_102));
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn save_file_1008_3178(mut param_1: u16 ,param_2: *mut astruct_72,mut param_3: i16)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut puVar3: *mut u16;
  let mut uVar4: u16;
  let mut success: bool;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  let mut unaff_SI: u16;
  let mut uVar8: u16;
  let mut pcVar9: *mut c_char;
  let mut in_stack_0000f720: u16;
  let mut in_stack_0000f844: u16;
  let mut in_stack_0000f84a: u16;
  let mut in_stack_0000f84e: u16;
  char local_782 [0x104];
  u16 local_67e [0x4];
  let mut pcStack1654: *mut c_char;
  let mut type: u16;
  let mut text: u16;
  let mut pcStack1646: *mut c_char;
  u8 local_666 [0x100];
  let mut pcStack1382: *mut c_char;
  SEGPTR ofn;
  let mut uStack1374: u16;
  let mut pcStack1370: *mut c_char;
  let mut uStack1368: u16;
  let mut pcStack1354: *mut c_char;
  let mut uStack1350: u32;
  let mut puStack1346: *mut u8;
  let mut uStack1344: u16;
  let mut uStack1342: u32;
  let mut pcStack1338: *mut c_char;
  let mut uStack1336: u16;
  let mut puStack1334: *mut u8;
  let mut uStack1332: u16;
  let mut uStack1330: u32;
  let mut uStack1326: u16;
  let mut pcStack1322: *mut c_char;
  let mut uStack1320: u16;
  let mut cStack1306: u8;
  char acStack1305 [0x101];
  let mut uStack1048: u16;
  char local_416 [0x8];
  let mut uStack1038: u16;
  u8 local_40c [0x102];
  let mut uStack778: u32;
  astruct_487 *paStack774;
  u8 local_302 [0x100];
  u8 local_202 [0xff];
  char acStack259 [0x101];

  acStack259[1] = 0;
  local_302[0] = 0;
  local_202[0] = 0;
  paStack774 = (astruct_487 *)
               mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                               (u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000f720,in_stack_0000f844,
                               in_stack_0000f84a,in_stack_0000f84e);
  uVar8 = (paStack774 >> 0x10);
  iVar2 = paStack774;
  uStack778 = (iVar2 + 0x1a);
  uVar5 = (iVar2 + 0x1c);
  if ((uVar5 | uStack778) == 0) {
    pcStack1646 = *(iVar2 + 0x64);
    uVar5 = (iVar2 + 0x66);
    if ((uVar5 | pcStack1646) != 0) {
      pass1_1008_5784(CONCAT22(0x1050,local_67e),pcStack1646 & 0xffff | uVar5 << 0x10);
      puVar3 = local_67e;
      pass1_1008_5b12(CONCAT22(0x1050,puVar3));
      pcStack1654 = CONCAT22(uVar5,puVar3);
      if ((uVar5 | puVar3) != 0) {
        uVar1 = (puVar3 + 2);
        uStack778 = uVar1;
        uVar5 = (uVar1 >> 0x10);
    // TODO: goto LAB_1008_3206;
      }
    }
  }
  else {//
LAB_1008_3206:
    unk_str_op_1000_3d3e(CONCAT22(0x1050,acStack259 + 1),CONCAT22(uVar5,uStack778));
  }
  pass1_1000_5008(local_40c,&DAT_1050_1050,0x100);
  uStack1038 = str_op_1000_3da4(CONCAT22(0x1050,local_40c));
  if (local_40c[uStack1038 - 0x1] == '\\') {
    local_40c[uStack1038 - 0x1] = 0;
  }
  uStack1038 = str_op_1000_3da4(CONCAT22(0x1050,acStack259 + 1));
  if (acStack259[uStack1038] == '\\') {
    acStack259[uStack1038] = '\0';
  }
  pass1_1000_4f2e();
  uVar8 = (paStack774 >> 0x10);
  uStack778 = (paStack774 + 0x12);
  uVar5 = (paStack774 + 0x14);
  if ((uVar5 | uStack778) != 0) {
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_202),(uStack778 & 0xffff | uVar5 << 0x10));
  }
  local_416[0] = '\0';
  pcVar9 = load_string_1010_847e(_u16_1050_14cc,0x579);
  unk_str_op_1000_3d3e(CONCAT22(0x1050,local_416),pcVar9);
  uStack1048 = str_op_1000_3da4(CONCAT22(0x1050,local_416));
  uStack1038 = uStack1048;
  for (; -0x1 < uStack1048; uStack1048 -= 1) {
    if (local_416[uStack1048] == '.') {
      unk_str_op_1000_3d3e(CONCAT22(0x1050,local_67e),CONCAT22(0x1050,local_416 + uStack1048 + 1));
      unk_str_op_1000_3d3e(CONCAT22(0x1050,local_416),CONCAT22(0x1050,local_67e));
    }
  }
  acStack1305[1] = 0;
  pcVar9 = load_string_1010_847e(_u16_1050_14cc,0x74c);
  uVar7 = (pcVar9 >> 0x10);
  unk_str_op_1000_3d3e(CONCAT22(0x1050,acStack1305 + 1),pcVar9);
  uStack1038 = str_op_1000_3da4(CONCAT22(0x1050,acStack1305 + 1));
  cStack1306 = acStack1305[uStack1038];
  uStack1048 = 0;
  while (acStack1305[uStack1048 + 0x1] != '\0') {
    if (acStack1305[uStack1048 + 0x1] == cStack1306) {
      acStack1305[uStack1048 + 0x1] = '\0';
    }
    uStack1048 += 0x1;
  }
  pass1_1000_4906(CONCAT22(0x1050,&ofn),NULL,0x48);
  _ofn = 0x48;
  uVar8 = (param_2 >> 0x10);
  uStack1374 = (param_2 + 0x8);
  pcStack1370 = acStack1305 + 1;
  uStack1368 = SUB42(&DAT_1050_1050,0x0);
  pcStack1354 = CONCAT22(0x1050,local_202);
  puStack1346 = local_302;
  uStack1344 = SUB42(&DAT_1050_1050,0x0);
  uStack1350 = 0x100;
  uStack1342 = 0x100;
  pcStack1338 = acStack259 + 1;
  uStack1336 = SUB42(&DAT_1050_1050,0x0);
  pcStack1322 = local_416;
  uStack1320 = SUB42(&DAT_1050_1050,0x0);
  pcStack1382 = NULL;
  local_666[0] = 0;
  if (param_3 == 1) {
    uStack1330 = 0x1804;
    pcVar9 = load_string_1010_847e(_u16_1050_14cc,0x74d);
    uVar7 = (pcVar9 >> 0x10);
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_666),pcVar9);
    puStack1334 = local_666;
    uStack1332 = SUB42(&DAT_1050_1050,0x0);
    success = GetOpenFileName16(CONCAT22(0x1050,&ofn));
  }
  else {
    if (param_3 != 0x2) {
      debug_print_1008_6048(uVar7,s_Unsupported_FileStructType_in_Op_1050_01ca);
  // TODO: goto LAB_1008_3461;
    }
    uStack1330 = 0x6;
    pcVar9 = load_string_1010_847e(_u16_1050_14cc,0x74e);
    uVar7 = (pcVar9 >> 0x10);
    unk_str_op_1000_3d3e(CONCAT22(0x1050,local_666),pcVar9);
    puStack1334 = local_666;
    uStack1332 = SUB42(&DAT_1050_1050,0x0);
    success = GetSaveFileName16(CONCAT22(0x1050,&ofn));
  }
  if (success != 0) {
    pcStack1382 = pcStack1354;
  }//
LAB_1008_3461:
  if (pcStack1382 != NULL) {
    local_67e[0] = uStack1326;
    if (uStack1326 < 0x0) {
      pcStack1654 = load_string_1010_847e(_u16_1050_14cc,0x3fd);
      uVar6 = (pcStack1654 >> 0x10);
      uVar4 = str_op_1008_60e8(uVar6,pcStack1654);
      pcStack1654 = CONCAT22(uVar6,uVar4);
      pcVar9 = load_string_1010_847e(_u16_1050_14cc,0x57b);
      text = (pcVar9 >> 0x10);
      type = SUB42(pcVar9,0x0);
      MessageBox16(0x10,pcVar9,pcStack1654,*(HWND16 *)(param_2 + 0x8));
      pcStack1382 = NULL;
      pcStack1646 = pcStack1654;
      fn_ptr_1000_17ce(pcStack1654);
    }
    else {
      str_op_1000_3dbe(CONCAT13(0x10,CONCAT12(0x50,local_782)),pcStack1354,uStack1326);
      local_782[uStack1326] = '\0';
      if (local_782[0] != '\0') {
        pass1_1010_60cc(uVar7,paStack774,CONCAT22(0x1050,local_782));
      }
    }
  }
  pass1_1000_4f2e();
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn set_sys_color_1008_357e(param_1: *mut astruct_53,mut param_2: i16,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut paVar2: *mut Struct57;
  astruct_53 *iVar3;
  let mut iVar4: i16;
  astruct_53 *uVar6;
  let mut uVar5: u16;
  COLORREF CVar6;
  let mut uVar7: u32;
  let mut iStack132: i16;
  let mut local_80: u32;
  let mut uStack124: u16;
  let mut uStack122: u16;
  let mut uStack120: u16;
  let mut uStack118: u16;
  let mut uStack116: u16;
  let mut uStack114: u16;
  let mut uStack112: u32;
  let mut uStack108: u32;
  let mut uStack104: u16;
  let mut uStack102: u16;
  let mut uStack100: u16;
  let mut uStack98: u16;
  let mut uStack96: u16;
  let mut uStack94: u16;
  let mut uStack92: u16;
  let mut uStack90: u16;
  let mut uStack88: u32;
  let mut uStack84: u32;
  let mut uStack80: u16;
  let mut uStack78: u16;
  let mut uStack76: u32;
  let mut uStack72: u32;
  let mut uStack68: u32;
  let mut uStack64: u32;
  let mut uStack60: u32;
  let mut uStack56: u32;
  let mut uStack52: u32;
  let mut uStack48: u32;
  let mut local_2c: u32;
  let mut uStack40: u32;
  let mut uStack36: u32;
  let mut uStack32: u32;
  let mut uStack28: u32;
  let mut uStack24: u32;
  let mut uStack20: u32;
  let mut uStack16: u32;
  let mut uStack12: u32;
  let mut uStack8: u32;
  let mut uStack4: u16;

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
  local_80 = 0;
  uStack108 = 0x808080;
  paVar2 = (astruct_57 *)CONCAT22((param_3 >> 0x10),0x100);
  uStack116 = 0;
  uStack114 = 0x100;
  uStack100 = 0;
  uStack98 = 0x100;
  uStack96 = 0xffff;
  uStack94 = 0;
  uStack124 = 0x2;
  uStack122 = 0x100;
  uStack120 = 0x2;
  uStack118 = 0x100;
  uStack104 = 0x2;
  uStack102 = 0x100;
  uStack92 = 0x2;
  uStack90 = 0x100;
  uStack88 = 0;
  uStack80 = 0xc0c0;
  uStack78 = 0;
  uStack76 = 0;
  uStack72 = 0;
  uStack68 = 0;
  uStack52 = 0;
  uVar1 = 0x8000;
  uStack112 = 0x8000;
  uStack84 = 0x8000;
  uStack64 = 0x8000;
  uStack60 = 0x8000;
  uStack56 = 0x8000;
  uStack48 = 0x8000;
  uVar6 = (astruct_53 *)(param_1 >> 0x10);
  iVar3 = (astruct_53 *)param_1;
  if (*(i32 *)&iVar3.field248_0xf8 == 0) {
    mem_op_1000_179c(0x54,paVar2);
    iVar3.field248_0xf8 = uVar1;
    iVar3.field249_0xfa = paVar2;
    for (iStack132 = 0; iStack132 < 0x15; iStack132 += 1) {
      CVar6 = GetSysColor16(*(INT16 *)(&local_2c + iStack132 * 0x2));
      uVar7 = &iVar3.field248_0xf8;
      uVar5 = (uVar7 >> 0x10);
      iVar4 = uVar7;
      (iVar4 + iStack132 * 0x4) = CVar6;
      (iVar4 + iStack132 * 0x4 + 0x2) = (CVar6 >> 0x10);
    }
  }
  if (param_2 != 0) {
    CVar6 = GetSysColor16((INT16)local_2c);
    if ((CVar6 == local_80) && ((CVar6 >> 0x10) == local_80)) {
      return;
    }
  }
  if (PTR_LOOP_1050_0010 == NULL) {
    uStack112 = 0xc0c0c0;
  }
  if (param_2 == 0) {
    uVar7 = &iVar3.field248_0xf8;
  }
  else {
    uVar7 = CONCAT22(0x1050,&local_80);
  }
  SetSysColors16((COLORREF *)uVar7,(INT16 *)(uVar7 >> 0x10),(INT16)&local_2c);
  return;
}
pub fn pass1_1008_3714(param_1: *mut StructA)

{
  pass1_1008_3e0e(param_1);
  return;
}



pub fn pass1_1008_372c(mut param_1: i16,mut param_2: u16 ) -> u32

{
  return CONCAT22(param_2,param_1 + 0xa);
}
pub fn pass1_1008_373c()

{
  return;
}
pub fn pass1_1008_3740()

{
  return;
}
pub fn pass1_1008_3744()

{
  return;
}
pub fn pass1_1008_3748()

{
  return;
}
pub fn pass1_1008_374c()

{
  return;
}
pub fn pass1_1008_3750()

{
  return;
}
pub fn pass1_1008_3754()

{
  return;
}



u16 pass1_1008_3758()

{
  return 0x1;
}
pub fn pass1_1008_375e()

{
  return;
}
pub fn pass1_1008_3762()

{
  return;
}
pub fn pass1_1008_3766()

{
  return;
}
pub fn FUN_1008_376a()

{
  return;
}
pub fn FUN_1008_376e()

{
  return;
}
pub fn FUN_1008_3772()

{
  return;
}
pub fn FUN_1008_3776()

{
  return;
}
pub fn pass1_1008_377a()

{
  return;
}



u16 * pass1_1008_377e(param_1: *mut u16,param_2: u8)

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_450 * pass1_1008_37aa(param_1: *mut astruct_450,param_2: u8)

{
  astruct_450 *uVar1;
  astruct_450 *uVar2;

  uVar2 = (astruct_450 *)(param_1 >> 0x10);
  uVar1 = (astruct_450 *)param_1;
  param_1.field0_0x0 = 0x380a;
  uVar1.field1_0x2 = 0x1008;
  param_1.field0_0x0 = 0x389a;
  uVar1.field1_0x2 = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub fn pass1_1008_37e4(mut param_1: u32,param_2: u8) -> u32

{
  cleanup_ui_op_1008_0618((astruct_53 *)param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1008_392e(param_1: *mut u16,mut param_2: u16 )

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x3aa8;
  (iVar1 + 0x2) = 0x1008;
  (iVar1 + 0x4) = param_2;
  *param_1 = 0x3ab0;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x3aa0;
  (iVar1 + 0x2) = 0x1008;
  return param_1;
}
pub fn pass1_1008_397a(param_1: *mut astruct_452)

{
  astruct_452 *iVar1;
  astruct_452 *uVar1;

  uVar1 = (astruct_452 *)(param_1 >> 0x10);
  iVar1 = (astruct_452 *)param_1;
  param_1.field0_0x0 = 0x3aa0;
  iVar1.field1_0x2 = 0x1008;
  param_1.field0_0x0 = 0x3ab0;
  iVar1.field1_0x2 = 0x1008;
  param_1.field0_0x0 = 0x389a;
  iVar1.field1_0x2 = 0x1008;
  return;
}



// l
pub fn fill_rect_1008_39ac(astruct_930 *in_win_handle_1,mut param_2: u16 )

{
  HBRUSH16 hbrush;
  u8 local_paint_struct [0x20];

  BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_paint_struct),in_win_handle_1.field4_0x4);
  hbrush = CreateSolidBrush16(0x210070b);
  GetClientRect16((RECT16 *)&stack0xffd2,(HWND16)&DAT_1050_1050);
  FillRect16(hbrush,(RECT16 *)&stack0xffd2,(HDC16)&DAT_1050_1050);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_paint_struct),in_win_handle_1.field4_0x4);
  DeleteObject16(hbrush);
  return;
}
pub fn pass1_1008_3a10()

{
  return;
}



u16 * pass1_1008_3a14(param_1: *mut u16,param_2: u8)

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1008_3a40(param_1: *mut astruct_451,param_2: u8)

{
  astruct_451 *uVar1;
  astruct_451 *uVar2;

  uVar2 = (astruct_451 *)(param_1 >> 0x10);
  uVar1 = (astruct_451 *)param_1;
  param_1 = 0x3ab0;
  uVar1.field2_0x2 = 0x1008;
  param_1 = 0x389a;
  uVar1.field2_0x2 = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub fn pass1_1008_3a7a(mut param_1: u32,param_2: u8) -> u32

{
  pass1_1008_397a((astruct_452 *)param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_20 * pass1_1008_3ab8(param_1: *mut astruct_20)

{
  astruct_20 *iVar1;
  let mut uVar1: u16;

  set_struct_1008_687a(param_1,0x0);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_20 *)param_1;
  iVar1.field164_0xde = 0;
  param_1.offset_0x0 = 0x3b46;
  iVar1.base_0x2 = 0x1008;
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | ZEXT24(&iVar1.field60_0x5b)),s_SOLDefaultWindowClass_1050_01fe);
  return param_1;
}
pub fn post_quit_msg_1008_3af4()

{
  PostQuitMessage16(0x0);
  return;
}
pub fn pass1_1008_3afe(param_1: *mut u16,param_2: u8)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  iVar1 = param_1;
  pass1_1008_57c4((param_1 & 0xffff0000 | (iVar1 + 0xd2)));
  uVar2 = (param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return;
}
pub fn pass1_1008_3bd6(mut param_1: u32,param_2: *mut astruct_57,param_3: *mut astruct_57,mut param_4: u16 ,mut param_5: u32,mut param_6: u16 ,
                    mut param_7: u32,mut param_8: u32,mut param_9: u16 ,mut param_10: u16 ,mut param_11: u16 ,
                    mut param_12: u16 ,mut param_13: u16 ,mut param_14: u16 )

{
  mixed_struct_op_1040_8fb8
            (param_1,(astruct_65 *)CONCAT22(param_3,param_2),param_4,NULL,param_6,param_7,(param_7 >> 0x10),
             param_8,(param_8 >> 0x10),param_9);
  CONCAT22(param_3,param_2) = 0x3cfc;
  param_2.field1_0x2 = 0x1008;
  &param_2.field_0x36 = 0;
  &param_2.field21_0x26 = 0;
  pass1_1040_9252((astruct_65 *)CONCAT22(param_3,param_2));
  create_window_1040_92dc((astruct_65 *)CONCAT22(param_3,param_2));
  mov_update_win_1040_93aa((astruct_65 *)CONCAT22(param_3,param_2),(INT16)param_5,(param_5 >> 0x10));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1008_3c34(mut param_1: u32,param_2: u8,HDC16 hdc_param_3)

{
  let mut uVar1: u16;
  code **ppcVar2;
  HPALETTE16 HVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut puStack6: *mut u32;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (((iVar4 + 0xa) | (iVar4 + 0x8)) != 0) {
    puStack6 = (iVar4 + 0x8);
    if ((*(i32 *)(iVar4 + 0xc) != 0) && ((param_2 & 1) != 0)) {
      puStack6 = (iVar4 + 0xc);
    }
    if ((*(i32 *)(iVar4 + 0x10) != 0) && ((param_2 & 0x4) != 0)) {
      puStack6 = (iVar4 + 0x10);
    }
    uVar6 = (_PTR_LOOP_1050_4230 >> 0x10);
    uVar1 = (_PTR_LOOP_1050_4230 + 0x10);
    HVar3 = palette_op_1008_4e08
                      ((HPALETTE16)&hdc_param_3,uVar1,
                       (astruct_13 *)CONCAT22(uVar1,(_PTR_LOOP_1050_4230 + 0xe)),
                       (HDC16 *)CONCAT22(0x1050,&hdc_param_3));
    ppcVar2 = (code **)(*puStack6 + 0x4);
    (**ppcVar2)();
    HVar3 = SelectPalette16(0x0,HVar3,hdc_param_3);
    DeleteObject16(HVar3);
  }
  return;
}
pub fn FUN_1008_3cd2()

{
  return;
}



StructD * pass1_1008_3cd6(StructD *param_1,param_2: u8)

{
  mix_win_ui_op_1040_911e(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn post_msg_1008_3d20(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  PostMessage16(0x0,*(WPARAM16 *)(param_1 + 0xcc),0x111,*(HWND16 *)(param_1 + 0xbc));
  return;
}
pub fn FUN_1008_3d40()

{
  return;
}



u16 * pass_1008_3d44(param_1: *mut astruct_453,param_2: u8)

{
  astruct_453 *uVar1;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  uVar1 = (astruct_453 *)param_1;
  param_1.field0_0x0 = 0x380a;
  uVar1.field1_0x2 = 0x1008;
  param_1.field0_0x0 = 0x389a;
  uVar1.field1_0x2 = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return &param_1.field0_0x0;
}
pub fn pass1_1008_3e0e(param_1: *mut StructA)

{
  code **ppcVar1;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)(param_1 + 0x4) != 0) {
    ppcVar1 = (code **)((param_1 + 0x4) + 0x4);
    (**ppcVar1)();
  }
  return;
}



u16 * pass1_1008_3e38(param_1: *mut astruct_19)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1.offset_0x0 = 0;
  (param_1 + 0x2) = 0;
  (param_1 + 0x4) = 0;
  return &param_1.offset_0x0;
}



u16 * pass1_1008_3e54(param_1: *mut u16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 )

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *param_1 = param_4;
  (param_1 + 0x2) = param_3;
  (param_1 + 0x4) = param_2;
  return param_1;
}
pub fn pass1_1008_3e76(param_1: *mut u16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 )

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *param_1 = param_4;
  (param_1 + 0x2) = param_3;
  (param_1 + 0x4) = param_2;
  return;
}
pub fn pass1_1008_3e94(param_1: *mut u16,param_2: *mut u16,char *param_3)

{
  param_3 = *param_1;
  *param_2 = (param_1 + 2);
  return;
}
pub fn pass1_1008_3eb4(param_1: *mut astruct_615,param_2: *mut u16,param_3: *mut u16,param_4: *mut u16)

{
  let mut uVar1: u16;

  *param_4 = param_1;
  uVar1 = (param_1 >> 0x10);
  *param_3 = (param_1 + 2);
  *param_2 = (param_1 + 0x4);
  return;
}
pub fn pass1_1008_3ee2(param_1: *mut i16,param_2: *mut i16)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u16;

  iVar1 = *param_2 - *param_1;
  if (iVar1 < 0x0) {
    iVar1 = -iVar1;
  }
  *param_1 = iVar1 + 1;
  uVar3 = (param_2 >> 0x10);
  uVar4 = (param_1 >> 0x10);
  iVar2 = param_1;
  iVar1 = (param_2 + 0x2) - (iVar2 + 2);
  if (iVar1 < 0x0) {
    iVar1 = -iVar1;
  }
  (iVar2 + 0x2) = iVar1 + 1;
  iVar1 = (param_2 + 0x4) - (iVar2 + 0x4);
  if (iVar1 < 0x0) {
    iVar1 = -iVar1;
  }
  (iVar2 + 0x4) = iVar1 + 1;
  return;
}
pub fn pass1_1008_3f32(param_1: *mut i16,param_2: *mut i16)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut uVar3: u16;

  *param_1 = *param_1 + *param_2;
  uVar2 = (param_2 >> 0x10);
  uVar3 = (param_1 >> 0x10);
  piVar1 = (param_1 + 2);
  *piVar1 = *piVar1 + (param_2 + 2);
  piVar1 = (param_1 + 0x4);
  *piVar1 = *piVar1 + (param_2 + 0x4);
  return;
}
pub fn pass1_1008_3f62(param_1: *mut u16,param_2: *mut u16)

{
  let mut uVar1: u16;
  let mut uVar2: u16;

  *param_1 = *param_2;
  uVar1 = (param_2 >> 0x10);
  uVar2 = (param_1 >> 0x10);
  (param_1 + 0x2) = (param_2 + 2);
  (param_1 + 0x4) = (param_2 + 0x4);
  return;
}
pub fn struct_op_1008_3f92(param_1: *mut astruct_76,param_2: *mut c_char)

{
  code **ppcVar1;
  astruct_76 *iVar2;
  let mut uVar2: u16;

  struct_op_1008_56b4(param_1);
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_76 *)param_1;
  &iVar2.field3_0x6 = 0;
  &iVar2.field5_0xa = 0;
  iVar2.field7_0xe = 0;
  iVar2.field8_0x10 = 0;
  &iVar2.field9_0x14 = 0;
  &iVar2.field11_0x18 = 0;
  iVar2.field13_0x1c = 0;
  param_1.offset_0x0 = &PTR_LOOP_1050_48de;
  iVar2.base_0x2 = 0x1008;
  if (param_2 == NULL) {
    return;
  }
  ppcVar1 = (code **)(param_2 + 0x8);
  (**ppcVar1)();
  struct_op_1008_4214(param_1,(astruct_81 *)param_2);
  pass1_1008_47cc(param_1);
  pass1_1008_4834(param_1);
  return;
}
