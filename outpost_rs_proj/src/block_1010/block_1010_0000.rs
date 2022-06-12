
astruct_19 * pass1_1010_0000(param_1: *mut astruct_19,mut param_2: u16 )

{
  let mut in_EDX: u32;
  let mut uVar1: u16;
  astruct_19 *paVar2;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fe86: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb4: u16;
  let mut iVar4: i16;
  let mut iVar5: i16;
  let mut uVar6: u16;

    // Segment:    3
    // Offset:     00015420
    // Length:     ee9f
    // Min Alloc:  ee9f
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
  uVar1 = (in_EDX >> 0x10);
  paVar2 = struct_op_1010_1d48(param_1,param_2);
  (param_1 + 0xa) = 0x0;
  (param_1 + 0xc) = 0x0;
  param_1.offset_0x0 = 0x2c8;
  (param_1 + 0x2) = 0x1010;
  iVar5 = param_1 + 0xa;
  iVar4 = param_1 + 0xc;
  uVar6 = param_1;
  puVar3 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar1,(paVar2 >> 0x10)),_u16_1050_0ed0,
                           (u8 **)CONCAT22(iVar4,0x48),in_stack_0000fe86,in_stack_0000ffaa,in_stack_0000ffb0,
                           in_stack_0000ffb4);
  pass1_1008_3e94((puVar3 & 0xffff0000 | (puVar3 + 0xe)),CONCAT22(param_1,iVar4),
                  CONCAT22(uVar6,iVar5));
  return param_1;
}
pub fn pass1_1010_0052(param_1: *mut u16,mut param_2: u16 )

{
  *param_1 = 0x2c8;
  (param_1 + 0x2) = 0x1010;
  pass1_1010_1d80((StructD *)param_1);
  return;
}
pub fn set_window_placement_1010_0070(mut param_1: u32,mut param_2: i16,mut param_3: u16 )

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  i32 lVar4;
  let mut uVar5: u16;
  u8 local_18 [0x6];
  INT16 IStack18;
  let mut iStack16: i16;
  INT16 IStack14;
  INT16 IStack12;
  INT16 IStack10;
  INT16 IStack8;
  let mut uStack6: u16;
  let mut uStack4: u16;

  local_18 = 0x16;
  local_18._2_4_ = 0x0;
  IStack18 = 0x0;
  iStack16 = 0x0;
  IStack14 = 0x0;
  IStack12 = 0x0;
  IStack10 = 0x0;
  IStack8 = 0x0;
  uStack6 = 0x0;
  uStack4 = 0x0;
  uVar5 = param_3;
  GetWindowPlacement16((WINDOWPLACEMENT16 *)local_18,(HWND16)&DAT_1050_1050);
  if ((iStack16 == -0x1) || (param_2 != 0x0)) {
    local_18._2_4_ = 0x50001;
    lVar4 = GetWindowLong16(0x0,param_3);
    uVar2 = (lVar4 >> 0x10);
    puVar3 = (lVar4 + 0xe0);
    ppcVar1 = (code **)(*puVar3 + 0x38);
    (**ppcVar1)(s_tile2_bmp_1050_1538,puVar3,(lVar4 + 0xe2),uVar5);
    pass1_1010_01f8(param_1,CONCAT22(0x1050,local_18),puVar3);
    SetWindowPlacement16((WINDOWPLACEMENT16 *)local_18,(HWND16)&DAT_1050_1050);
  }
  return;
}
pub fn set_win_placement_1010_010e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 )

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut piVar3: *mut i16;
  let mut uVar4: u16;
  let mut puVar5: *mut u32;
  let mut extraout_DX: u16;
  i32 lVar6;
  let mut uVar7: u16;
  WINDOWPLACEMENT16 local_18;
  let mut iStack6: i16;
  let mut iStack4: i16;

  local_18.length = 0x16;
  local_18.flags = 0x0;
  local_18.show_cmd = 0x0;
  local_18.pt_min_position.x = 0x0;
  local_18.pt_min_position.y = 0x0;
  local_18.pt_max_position.x = 0x0;
  local_18.pt_max_position.y = 0x0;
  local_18.rc_normal_position.x = 0x0;
  local_18.rc_normal_position.y = 0x0;
  iStack6 = 0x0;
  iStack4 = 0x0;
  uVar7 = param_3;
  GetWindowPlacement16(&local_18,(HWND16)&DAT_1050_1050);
  if (local_18.rc_normal_position.x == -0x1) {
    lVar6 = GetWindowLong16(0x0,param_3);
    uVar4 = (lVar6 >> 0x10);
    puVar5 = (lVar6 + 0xe0);
    ppcVar1 = (code **)(*puVar5 + 0x1c);
    (**ppcVar1)(s_tile2_bmp_1050_1538,puVar5,(lVar6 + 0xe2),uVar7);
    iVar2 = puVar5;
    piVar3 = (puVar5 & 0xffff | extraout_DX << 0x10);
    local_18.show_cmd = 0x9;
    local_18.rc_normal_position.x = *piVar3;
    local_18.rc_normal_position.y = *(INT16 *)(iVar2 + 0x2);
    iStack6 = (iVar2 + 0x4) + *piVar3;
    iStack4 = (iVar2 + 0x2) + (iVar2 + 0x6);
    SetWindowPlacement16(&local_18,(HWND16)&DAT_1050_1050);
  }
  return;
}
pub fn enum_child_windows_1010_01be(void)

{
pub fn *func;

  if (PTR_LOOP_1050_0010 == NULL) {
    func = MakeProcInstance16(HINSTANCE16_1050_038c,win_ui_op_1010_0240);
    EnumChildWindows1(0x0,func,(HWND16)func);
    FreeProcInstance16(func);
  }
  return;
}
pub fn pass1_1010_01f8(mut param_1: u32,mut param_2: u32,mut param_3: i16)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u16;

  iVar2 = (param_3 * 0x4 + 0xe02) * 0x4;
  iVar1 = (iVar2 + 0xdfc);
  uVar3 = (param_1 >> 0x10);
  uVar4 = (param_2 >> 0x10);
  (param_2 + 0x6) =
       (param_3 * 0x4 + 0xe04) * 0x28 + (iVar2 + 0xdfa) + (param_1 + 0xa);
  (param_2 + 0x8) = (param_1 + 0xc) + iVar1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 win_ui_op_1010_0240(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 )

{
  code **ppcVar1;
  let mut BVar2: bool;
  WORD WVar3;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fea2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000fff8: u32;
  let mut uVar6: u16;

  uVar6 = (in_stack_0000fff8 >> 0x10);
  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  BVar2 = IsWindow16(param_4);
  if (BVar2 != 0x0) {
    WVar3 = GetWindowWord16(-0x6,param_4);
    if (WVar3 == HINSTANCE16_1050_038c) {
      BVar2 = IsIconic16(param_4);
      if (BVar2 != 0x0) {
        puVar5 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(uVar6,0x45),in_stack_0000fea2,
                                 in_stack_0000ffc6,in_stack_0000ffcc,in_stack_0000ffd0);
        ppcVar1 = (code **)((puVar5 & 0xffff0000 | param_4) + 0x10);
        (**ppcVar1)(s_tile2_bmp_1050_1538,puVar5,(puVar5 >> 0x10),0x1);
      }
    }
  }
  return 0x1;
}



u16 * pass1_1010_02a2(param_1: *mut u16,param_2: u8)

{
  pass1_1010_0052(param_1,&DAT_1050_1050);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_1010_02e0(param_1: *mut astruct_19,mut param_2: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_EDX: u32;
  let mut uVar4: u16;
  let mut paVar3: *mut Struct57;
  astruct_19 *paVar5;

  uVar4 = (in_EDX >> 0x10);
  paVar5 = struct_op_1010_1d48(param_1,param_2);
  paVar3 = (astruct_57 *)CONCAT22(uVar4,(paVar5 >> 0x10));
  uVar1 = 0x0;
  (param_1 + 0xa) = 0x0;
  (param_1 + 0xe) = 0x0;
  (param_1 + 0x10) = 0x0;
  (param_1 + 0x18) = 0x0;
  param_1.offset_0x0 = 0xe98;
  (param_1 + 0x2) = 0x1010;
  mem_op_1000_179c(0xc,paVar3);
  uVar2 = paVar3 | uVar1;
  if (uVar2 == 0x0) {
    (param_1 + 0xa) = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar3,uVar1));
    (param_1 + 0xa) = uVar1;
    (param_1 + 0xc) = uVar2;
  }
  return;
}
pub fn pass1_1010_0350(param_1: *mut astruct_455)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  code **ppcVar3;
  astruct_455 *iVar4;
  astruct_455 *uVar4;

  uVar4 = (astruct_455 *)(param_1 >> 0x10);
  iVar4 = (astruct_455 *)param_1;
  param_1.field0_0x0 = 0xe98;
  iVar4.field1_0x2 = 0x1010;
  puVar1 = iVar4[0x1].field1_0x2;
  puVar2 = iVar4[0x1].field2_0x4;
  if ((puVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80((StructD *)param_1);
  return;
}
pub fn pass1_1010_038e(param_1: *mut astruct_27,mut param_2: i16)

{
  let mut bVar1: bool;
  astruct_27 *iVar2;
  let mut uVar2: u16;

  bVar1 = false;
  iVar2 = (astruct_27 *)param_1;
  uVar2 = (param_1 >> 0x10);
  if (param_2 != 0x0) {
    if (&iVar2.field_0x18 == 0x0) {
      iVar2.field18_0x12 = DAT_1050_0e28;
      iVar2.field19_0x14 = PTR_LOOP_1050_0e30;
      iVar2.field20_0x16 = PTR_LOOP_1050_0ea8;
      DAT_1050_0e28 = 0x0;
      PTR_LOOP_1050_0e30 = NULL;
      PTR_LOOP_1050_0ea8 = NULL;
      &iVar2.field_0x18 = 0x1;
      bVar1 = true;
      goto LAB_1010_0404;
    }
  }
  if (param_2 == 0x0) {
    if (&iVar2.field_0x18 != 0x0) {
      DAT_1050_0e28 = iVar2.field18_0x12;
      PTR_LOOP_1050_0e30 = iVar2.field19_0x14;
      PTR_LOOP_1050_0ea8 = iVar2.field20_0x16;
      &iVar2.field_0x18 = 0x0;
      bVar1 = true;
    }
  }//
LAB_1010_0404:
  if (bVar1) {
    pass1_1010_1f62(param_1,0x3);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 pass1_1010_041a(void)

{
  let mut uVar1: u32;

  uVar1 = pass1_1030_8326();
  if (((uVar1 >> 0x10) == 0x0) && (uVar1 < 0x64)) {
    return 0x0;
  }
  return 0x1;
}
pub fn pass1_1010_043a(param_1: *mut astruct_27,i32 param_2,mut param_3: i16)

{
  code **ppcVar1;
  let mut pchar_1: *mut c_char;
  let mut uVar2: u16;
  let mut in_EDX: *mut Struct57;
  astruct_27 *iVar4;
  astruct_227 *iVar5;
  astruct_27 *uVar3;
  let mut uVar4: u16;
  let mut puStack18: *mut u16;
  let mut puStack14: *mut u16;
  char local_a [0x8];

  iVar4 = (astruct_27 *)param_1;
  uVar3 = (astruct_27 *)(param_1 >> 0x10);
  if (param_3 == 0xc) {
    if (&iVar4.field_0xe != 0x0) {
      return;
    }
    &iVar4.field_0xe = 0x1;
  }
  else if (param_3 == 0xd) {
    if (&iVar4.field_0x10 != 0x0) {
      return;
    }
    &iVar4.field_0x10 = 0x1;
  }
  else if (param_3 == 0x12) {
    return;
  }
  pass1_1010_089e(param_1,0x1,0x6);
  pass1_1008_5784(CONCAT22(0x1050,local_a),&iVar4.field_0xa);
  do {
    pchar_1 = local_a;
    pass1_1008_5b12(CONCAT22(0x1050,pchar_1));
    uVar2 = in_EDX;
    in_EDX = (astruct_57 *)(in_EDX & 0xffff0000 | (uVar2 | pchar_1));
    if ((uVar2 | pchar_1) == 0x0) {
      mem_op_1000_179c(0xa,in_EDX);
      uVar2 = in_EDX;
      puStack18 = CONCAT22(uVar2,pchar_1);
      if ((uVar2 | pchar_1) == 0x0) {
        puStack14 = NULL;
      }
      else {
        *puStack18 = 0x389a;
        (pchar_1 + 0x2) = 0x1008;
        *puStack18 = 0xea8;
        (pchar_1 + 0x2) = 0x1010;
        puStack14 = puStack18;
      }
      uVar4 = (puStack14 >> 0x10);
      iVar5 = (astruct_227 *)puStack14;
      iVar5.field4_0x4 = param_3;
      iVar5.field5_0x6 = param_2;
      ppcVar1 = (code **)(*(u32*)&iVar4.field_0xa + 0x8);
      (**ppcVar1)(0x1000,&iVar4.field_0xa,iVar5,uVar4);
      return;
    }
  } while (((pchar_1 + 0x4) != param_3) || (*(i32 *)(pchar_1 + 0x6) != param_2));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_0538(param_1: *mut astruct_27,char **param_2,char **param_3)

{
  let mut iVar1: i16;
  let mut uVar2: u32;
  code **ppcVar3;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut pcVar7: *mut c_char;
  let mut puVar8: *mut u8,
  let mut extraout_DX: u16;
  let mut puVar9: *mut u8,
  let mut extraout_DX_00: *mut u8,
  astruct_27 *uVar10;
  astruct_27 *uVar11;
  let mut uVar12: u16;
  let mut puStack6: *mut u32;

  uVar5 = 0x0;
  *param_3 = NULL;
  *param_2 = NULL;
  uVar11 = (astruct_27 *)(param_1 >> 0x10);
  uVar10 = (astruct_27 *)param_1;
  ppcVar3 = (code **)(*(u32*)&uVar10.field_0xa + 0x10);
  (**ppcVar3)();
  puStack6 = CONCAT22(extraout_DX,uVar5);
  puVar9 = (extraout_DX | uVar5);
  if (puVar9 == NULL) {
    return;
  }
  iVar1 = (uVar5 + 0x4);
  uVar2 = (uVar5 + 0x6);
  if ((extraout_DX | uVar5) != 0x0) {
    ppcVar3 = (code **)*puStack6;
    (**ppcVar3)();
    puVar9 = extraout_DX_00;
  }
  uVar4 = &uVar10.field_0xa;
  if ((uVar4 + 0x8) == 0x0) {
    pass1_1010_089e(param_1,0x0,0x6);
    pass1_1010_1f62(param_1,0x3);
  }
  uVar6 = iVar1 + 0x757;
  load_string_1010_84ac(_u16_1050_14cc,(INT16)(_u16_1050_14cc >> 0x10),uVar6);
  param_3 = uVar6;
  *(u8 **)(param_3 + 0x2) = puVar9;
  while (pcVar7 = pass1_1000_472c(*param_3,'%'), (puVar9 | pcVar7) != 0x0) {
    pass1_1010_09b4(puVar9,uVar10,uVar11,CONCAT22(puVar9,pcVar7),param_3,uVar2);
  }
  if (0x1e < iVar1 - 0x1U) goto LAB_1010_0850;
  uVar12 = (param_2 >> 0x10);
  switch(iVar1) {
  case 0x1:
    goto LAB_1010_0619;
  case 0x2:
    goto LAB_1010_0619;
  case 0x3:
    break;
  case 0x4:
    goto LAB_1010_0619;
  case 0x5:
    goto LAB_1010_0619;
  case 0x6:
    break;
  case 0x7:
    goto LAB_1010_0619;
  case 0x8:
    goto LAB_1010_0619;
  case 0x9:
    break;
  case 0xa:
    goto LAB_1010_0619;
  case 0xb:
    goto LAB_1010_0619;
  case 0xc:
    break;
  case 0xd:
    goto LAB_1010_0619;
  case 0xe:
    break;
  case 0xf:
    goto LAB_1010_0619;
  case 0x10:
    break;
  case 0x11:
    break;
  case 0x12:
    break;
  case 0x13:
    break;
  case 0x14:
    break;
  case 0x15:
    break;
  case 0x16://
LAB_1010_0619:
    puVar8 = pass1_1008_5fd8(puVar9);
    goto LAB_1010_0621;
  case 0x17:
    break;
  case 0x18:
    break;
  case 0x19:
  case 0x1f://
LAB_1010_0785:
    puVar8 = pass1_1008_5fd8(puVar9);
    goto LAB_1010_0621;
  case 0x1a:
    goto LAB_1010_0785;
  case 0x1b:
    goto LAB_1010_0785;
  case 0x1c:
    break;
  case 0x1d:
    break;
  case 0x1e:
    puVar8 = pass1_1008_5fd8(puVar9);
    *(u8 **)param_2 = puVar8;
    *(u8 **)(param_2 + 0x2) = puVar9;
    goto LAB_1010_0785;
  }
  puVar8 = pass1_1008_5fd8(puVar9);//
LAB_1010_0621:
  *(u8 **)param_2 = puVar8;
  *(u8 **)(param_2 + 0x2) = puVar9;//
LAB_1010_0850:
  while (pcVar7 = pass1_1000_472c(*param_2,'%'), (puVar9 | pcVar7) != 0x0) {
    pass1_1010_09b4(puVar9,uVar10,uVar11,CONCAT22(puVar9,pcVar7),param_2,uVar2);
  }
  return;
}



u16 pass1_1010_0886(void)

{
  return 0xa;
}



u16 pass1_1010_088c(void)

{
  return 0x3;
}



u16 pass1_1010_0892(void)

{
  return 0x3;
}



u16 pass1_1010_0898(void)

{
  return 0x3;
}
pub fn pass1_1010_089e(param_1: *mut astruct_27,mut param_2: u16 ,mut param_3: i16)

{
  ((param_3 + -0x1) * 0x8 + 0xe28) = param_2;
  pass1_1010_1f62(param_1,0x3);
  return;
}
pub fn pass1_1010_08c0(mut param_1: u32,mut param_2: u16 ,mut param_3: i16)

{
  ((param_3 + -0x1) * 0x8 + 0xea8) = param_2;
  pass1_1010_1f62((astruct_27 *)param_1,0x3);
  return;
}



pub fn pass1_1010_08e2(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16) -> u32

{
  if (PTR_LOOP_1050_4fe8 != NULL) {
    DAT_1050_0e28 = 0x0;
    PTR_LOOP_1050_0e30 = NULL;
    PTR_LOOP_1050_0e38 = NULL;
    PTR_LOOP_1050_0e40 = NULL;
    PTR_LOOP_1050_0e48 = NULL;
    DAT_1050_0e58 = 0x0;
    DAT_1050_0e60 = 0x0;
    PTR_LOOP_1050_0e70 = NULL;
  }
  return CONCAT22(0x1050,(param_3 + -0x1) * 0x8 + 0xe22);
}



pub fn pass1_1010_091e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16) -> u32

{
  return CONCAT22(0x1050,(param_3 + -0x1) * 0x8 + 0xe72);
}



pub fn pass1_1010_0932(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16) -> u32

{
  return CONCAT22(0x1050,(param_3 + -0x1) * 0x8 + 0xe8a);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_0946(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,u8 *param_4,mut param_5: i16,mut param_6: u16 ) -> u32

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fe8c: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffba: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  i32 lVar6;

  PTR_LOOP_1050_0ea8 = NULL;
  lVar6 = 0x4000001;
  uVar4 = 0x2;
  uVar5 = 0x400;
  puVar3 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_4),_u16_1050_0ed0,(u8 **)0x2003b,
                           in_stack_0000fe8c,in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
  uVar2 = (puVar3 >> 0x10);
  iVar1 = puVar3;
  pass1_1008_dfa6(puVar3,CONCAT22(uVar5,uVar4),lVar6);
  if (iVar1 != 0x0) {
    pass1_1030_8344(_u16_1050_5748,0x4000002);
    if (*(i32 *)(iVar1 + 0x200) == 0x8000002) {
      PTR_LOOP_1050_0ea8 = (&PTR_LOOP_1050_0000 + 0x1);
    }
  }
  return CONCAT22(0x1050,(param_3 + -0x1) * 0x8 + 0xea2);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_09b4(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,u8 *param_4,char **param_5,mut param_6: u32)

{
  u8 bVar1;
  let mut bVar2: bool;
  let mut bVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  let mut paVar8: *mut Struct57;
  let mut puVar9: *mut u32;
  let mut pcVar10: *mut c_char;
  let mut in_stack_0000fe86: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffde: u16;
  let mut pcStack18: *mut c_char;
  let mut uStack10: u16;
  let mut uStack8: u16;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  bVar3 = false;
  bVar2 = false;
  bVar1 = *(param_4 + 0x1);
  if (bVar1 == 0x70) {//
LAB_1010_0a06:
    bVar3 = false;
    bVar2 = true;
  }
  else if (bVar1 < 0x71) {
    if (bVar1 != 0x43) {
      if (bVar1 == 0x50) goto LAB_1010_0a06;
      if (bVar1 != 0x63) goto LAB_1010_09db;
    }
    bVar3 = true;
    bVar2 = false;
  }//
LAB_1010_09db:
  uVar7 = 0x0;
  uStack10 = 0x0;
  uStack8 = 0x0;
  if (bVar2) {
    puVar9 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffde,0x2),in_stack_0000fe86,
                             in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
    uVar7 = (puVar9 >> 0x10);
    uStack10 = (puVar9 + 0x6c);
    paVar8 = (astruct_57 *)(paVar8 & 0xffff0000 | (puVar9 + 0x6e));
  }
  else {
    if (!bVar3) goto LAB_1010_0a36;
    pass1_1030_8344(_u16_1050_5748,param_6);
    pcVar10 = pass1_1038_4d28(CONCAT22(paVar8,uVar7));
    paVar8 = (astruct_57 *)(paVar8 & 0xffff0000 | pcVar10 >> 0x10);
    uStack10 = pcVar10;
  }
  uStack8 = paVar8;//
LAB_1010_0a36:
  if ((uStack8 | uStack10) != 0x0) {
    uVar4 = str_op_1000_3da4(*param_5);
    uVar5 = str_op_1000_3da4(CONCAT22(uStack8,uStack10));
    iVar6 = uVar5 + 0xa + uVar4;
    mem_op_1000_179c(iVar6,paVar8);
    uVar7 = SUB42(paVar8,0x0);
    pcStack18 = CONCAT22(uVar7,iVar6);
    *param_4 = '\0';
    unk_str_op_1000_3d3e(CONCAT22(uVar7,iVar6),*param_5);
    pass1_1000_3cea(CONCAT22(uVar7,iVar6),CONCAT22(uStack8,uStack10));
    pass1_1000_3cea(CONCAT22(uVar7,iVar6),(param_4 & 0xffff0000 | (param_4 + 0x2)));
    fn_ptr_1000_17ce(*param_5);
    *param_5 = pcStack18;
  }
  return;
}
pub fn pass1_1010_0ad2(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut BVar2: bool;
  let mut puVar3: *mut u8,
  let mut extraout_DX: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  HFILE16 in_stack_0000ffbc;
  u32 local_2a [0x2];
  u16 local_22 [0x2];
  u16 local_1e [0x3];
  u16 local_18 [0x3];
  let mut uStack18: u32;
  u8 local_e [0x8];
  let mut uStack6: u16;
  let mut iStack4: i16;

  BVar2 = write_to_file_1008_7cac(param_2);
  if (BVar2 == 0x0) {
    return;
  }
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (*(i32 *)(iVar4 + 0xa) == 0x0) {
    uStack6 = 0x0;
  }
  else {
    uVar1 = (iVar4 + 0xa);
    uStack6 = (uVar1 + 0x8);
  }
  local_1e[0] = uStack6;
  BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_1e),0x2,in_stack_0000ffbc);
  if (BVar2 != 0x0) {
    pass1_1008_5784(CONCAT22(0x1050,local_e),(iVar4 + 0xa));
    do {
      puVar3 = local_e;
      pass1_1008_5b12(CONCAT22(0x1050,puVar3));
      uStack18 = CONCAT22(extraout_DX,puVar3);
      if ((extraout_DX | puVar3) == 0x0) {
        local_22[0] = (iVar4 + 0xe);
        BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_22),0x2,in_stack_0000ffbc);
        if (BVar2 == 0x0) {
          u16_1050_0310 = 0x6d0;
          return;
        }
        local_22[0] = (iVar4 + 0x10);
        BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_22),0x2,in_stack_0000ffbc);
        if (BVar2 == 0x0) {
          u16_1050_0310 = 0x6d0;
          return;
        }
        if ((iVar4 + 0x18) != 0x0) {
          DAT_1050_0e28 = (iVar4 + 0x12);
          PTR_LOOP_1050_0e30 = (iVar4 + 0x14);
          PTR_LOOP_1050_0ea8 = (iVar4 + 0x16);
        }
        iStack4 = 0x0;
        while( true ) {
          if (0x9 < iStack4) {
            iStack4 = 0x0;
            while( true ) {
              if (0x2 < iStack4) {
                if ((iVar4 + 0x18) != 0x0) {
                  DAT_1050_0e28 = 0x0;
                  PTR_LOOP_1050_0e30 = NULL;
                  PTR_LOOP_1050_0ea8 = NULL;
                }
                return;
              }
              local_1e[0] = (iStack4 * 0x8 + 0xea8);
              BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_1e),0x2,in_stack_0000ffbc);
              if (BVar2 == 0x0) break;
              iStack4 += 0x1;
            }
            u16_1050_0310 = 0x6d0;
            return;
          }
          local_1e[0] = (iStack4 * 0x8 + 0xe28);
          BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_1e),0x2,in_stack_0000ffbc);
          if (BVar2 == 0x0) break;
          iStack4 += 0x1;
        }
        u16_1050_0310 = 0x6d0;
        return;
      }
      local_18[0] = (puVar3 + 0x4);
      BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_18),0x2,in_stack_0000ffbc);
      if (BVar2 == 0x0) {
        u16_1050_0310 = 0x6d0;
        return;
      }
      local_2a[0] = (uStack18 + 0x6);
      BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_2a),0x4,in_stack_0000ffbc);
    } while (BVar2 != 0x0);
  }
  u16_1050_0310 = 0x6d0;
  return;
}
pub fn file_1010_0c7c(mut param_1: i16,u8 *param_2,param_3: *mut astruct_228,mut param_4: u32)

{
  code **ppcVar1;
  let mut BVar2: bool;
  astruct_229 *uVar4;
  let mut uVar3: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  astruct_228 *iVar6;
  let mut uVar7: u16;
  u16 local_2a [0x2];
  let mut uStack38: u16;
  let mut puStack26: *mut u32;
  let mut puStack22: *mut u32;
  u16 local_12 [0x5];
  astruct_229 *paStack8;
  astruct_229 *local_6;
  let mut uStack4: u16;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar7 = (param_4 >> 0x10);
  read_file_1008_7cfe(param_4,uVar7,0x6);
  if (param_1 == 0x0) {
    u16_1050_0310 = 0x6d4;
  }
  else {
    BVar2 = read_file_1008_7dee((HFILE16 *)param_4,CONCAT22(0x1050,&local_6),0x2);
    if (BVar2 != 0x0) {
      for (paStack8 = NULL; iVar6 = (astruct_228 *)param_3, paStack8 < local_6;
          paStack8 = (astruct_229 *)&paStack8.field_0x1) {
        uVar4 = local_6;
        mem_op_1000_179c(0xa,paVar6);
        uVar5 = paVar6;
        puStack26 = CONCAT22(uVar5,uVar4);
        paVar6 = (astruct_57 *)(paVar6 & 0xffff0000 | (uVar5 | uVar4));
        if ((uVar5 | uVar4) == 0x0) {
          puStack22 = NULL;
        }
        else {
          puStack26 = 0x389a;
          uVar4.field2_0x2 = 0x1008;
          puStack26 = 0xea8;
          uVar4.field2_0x2 = 0x1010;
          puStack22 = puStack26;
        }
        BVar2 = read_file_1008_7dee((HFILE16 *)param_4,CONCAT22(0x1050,local_12),0x2);
        if ((BVar2 == 0x0) ||
           (BVar2 = read_file_1008_7dee((HFILE16 *)param_4,
                                        (puStack22 & 0xffff0000 | (puStack22 + 0x6)),0x4),
           BVar2 == 0x0)) {
          puStack26 = puStack22;
          if (puStack22 != NULL) {
            ppcVar1 = (code **)*puStack22;
            (**ppcVar1)(0x1008,puStack22,(puStack22 >> 0x10),0x1);
          }
          goto LAB_1010_0cb1;
        }
        (puStack22 + 0x4) = local_12[0];
        ppcVar1 = (code **)(*iVar6.field10_0xa + 0x8);
        (**ppcVar1)();
      }
      BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar6.field_0xe)),0x2
                                 );
      if ((BVar2 != 0x0) &&
         (BVar2 = read_file_1008_7dee((HFILE16 *)param_4,
                                      (param_3 & 0xffff0000 | ZEXT24(&iVar6.field_0x10)),0x2),
         BVar2 != 0x0)) {
        for (uStack4 = 0x0; uStack4 < 0xa; uStack4 += 0x1) {
          BVar2 = read_file_1008_7dee((HFILE16 *)param_4,CONCAT22(0x1050,local_2a),0x2);
          if (BVar2 == 0x0) goto LAB_1010_0cb1;
          uVar3 = uStack4;
          if (u16_1050_0312 < 0x2) {
            uVar3 = pass1_1008_738c(param_4,uVar7,uStack4);
          }
          (uVar3 * 0x8 + 0xe28) = local_2a[0];
          uStack38 = uVar3;
        }
        if (0x2 < u16_1050_0312) {
          uStack4 = 0x0;
          do {
            BVar2 = read_file_1008_7dee((HFILE16 *)param_4,CONCAT22(0x1050,local_2a),0x2);
            if (BVar2 == 0x0) goto LAB_1010_0cb1;
            (uStack4 * 0x8 + 0xea8) = local_2a[0];
            uStack4 += 0x1;
          } while (uStack4 < 0x3);
        }
        return;
      }
    }//
LAB_1010_0cb1:
    u16_1050_0310 = 0x6d2;
  }
  return;
}



u16 * pass1_1010_0e46(param_1: *mut u16,param_2: u8,mut param_3: u16 )

{
  pass1_1010_0350((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1010_0e6c(param_1: *mut u16,param_2: u8)

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_19 * pass1_1010_0eac(u8 *param_1,param_2: *mut astruct_19,mut param_3: u16 )

{
  struct_op_1018_4cda(param_2,param_3);
  param_2.offset_0x0 = 0xf0c;
  (param_2 + 0x2) = 0x1010;
  _PTR_LOOP_1050_4230 = param_2;
  pass1_1018_4dce(param_1,param_2,0xff);
  return param_2;
}



StructD * pass1_1010_0ee6(StructD *param_1,param_2: u8)

{
  clenaup_win_ui_1018_4d22(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_0f24(u8 *param_1,param_2: *mut astruct_19,mut param_3: u16 )

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut unaff_BP: u16;
  let mut puVar2: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  struct_1010_2cd2(param_2,param_3);
  (param_2 + 0x60) = 0x0;
  (param_2 + 0x64) = 0x0;
  (param_2 + 0x68) = 0x0;
  (param_2 + 0x6a) = 0x0;
    // 0x191a
  param_2.offset_0x0 = s_648_bmp_1050_1919 + 0x1;
  (param_2 + 0x2) = 0x1010;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (param_2 + 0x6a) = puVar2;
  (param_2 + 0x6c) = (puVar2 >> 0x10);
  return;
}
pub fn pass1_1010_0f76(param_1: *mut astruct_455)

{
  astruct_455 *uVar1;

  uVar1 = (astruct_455 *)(param_1 >> 0x10);
  param_1.field0_0x0 = s_648_bmp_1050_1919 + 0x1;
  (param_1 + 0x2) = 0x1010;
  pass1_1010_17c0((astruct_455 *)(param_1 & 0xffff | ZEXT24(uVar1) << 0x10));
  pass1_1010_2db2(param_1);
  return;
}
pub fn struct_1010_0f9c(mut param_1: u16 ,param_2: *mut astruct_232,param_3: *mut astruct_57)

{
  let mut puVar1: *mut u8,
  code **ppcVar2;
  let mut iVar3: i16;
  astruct_92 *paVar4;
  astruct_92 *paVar5;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut uVar8: u16;
  astruct_232 *iVar8;
  astruct_231 *iVar9;
  astruct_230 *iVar10;
  astruct_233 *iVar11;
  astruct_232 *uVar9;
  let mut uVar10: u16;
  astruct_232 *paVar11;
  u8 uVar12;
  let mut uStack36: u32;
  let mut iStack32: i16;
  let mut uStack30: u16;
  astruct_92 **ppaStack28;
  astruct_15 *paStack24;
  astruct_92 local_14;

  uVar9 = (astruct_232 *)(param_2 >> 0x10);
  iVar8 = (astruct_232 *)param_2;
  ppcVar2 = (code **)(param_2 + 0x38);
  (**ppcVar2)();
  iVar8.field100_0x68 = param_1;
  if ((*(i32 *)&iVar8.field96_0x60 != 0x0) && (iVar8.field100_0x68 == 0x1)) {
    return;
  }
  if (iVar8.field100_0x68 == 0x0) {
    return;
  }
  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x700);
  iVar3 = iVar8.field100_0x68 * 0x18;
  mem_op_1000_179c(iVar3,param_3);
  iVar8.field96_0x60 = iVar3;
  iVar8.field97_0x62 = param_3;
  ppaStack28 = (astruct_92 **)CONCAT22(param_3,iVar8.field96_0x60);
  uStack30 = iVar8.field100_0x68;
  do {
    do {
      paVar4 = &local_14;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar4));
      uVar8 = param_3;
      paStack24 = (astruct_15 *)CONCAT22(uVar8,paVar4);
      param_3 = (astruct_57 *)(param_3 & 0xffff0000 | (uVar8 | paVar4));
      if ((uVar8 | paVar4) == 0x0) goto LAB_1010_10ca;
      iVar9 = (astruct_231 *)param_2;
      ppcVar2 = (code **)&iVar9.field58_0x40;
      paVar5 = paVar4;
      (**ppcVar2)(0x1028,param_2);
    } while (paVar5 == NULL);
    pass1_1028_b58e(paStack24);
    uStack36 = CONCAT22(param_3,paVar5);
    ppcVar2 = (code **)&iVar9.field44_0x2c;
    (**ppcVar2)(0x1028,param_2);
    uVar10 = (ppaStack28 >> 0x10);
    iVar10 = (astruct_230 *)ppaStack28;
    *ppaStack28 = paVar5;
    iVar10.field2_0x2 = param_3;
    uVar12 = SUB21(paVar4,0x0);
    ppcVar2 = (code **)&iVar9.field46_0x30;
    paVar11 = param_2;
    (**ppcVar2)();
    iVar10.field7_0x8 = paVar5;
    iVar10.field8_0xa = param_3;
    iVar10.field9_0xc = uStack36;
    ppcVar2 = (code **)&iVar9.field56_0x3c;
    uVar7 = uStack36;
    (**ppcVar2)(0x1028,param_2,paStack24,paVar11,uVar12,uVar8);
    iVar10.field10_0x10 = uVar7;
    iVar10.field11_0x12 = param_3;
    iVar10.field12_0x14 = uStack36;
    ppaStack28 = (astruct_92 **)(ppaStack28 & 0xffff0000 | ZEXT24(iVar10 + 0x1));
    uStack30 -= 0x1;
  } while (uStack30 != 0x0);//
LAB_1010_10ca:
  uVar6 = iVar8.field100_0x68 << 0x2;
  mem_op_1000_179c(uVar6,param_3);
  iVar8.field98_0x64 = uVar6;
  iVar8.field99_0x66 = param_3;
  iStack32 = 0x0;
  uStack30 = 0x0;
  while( true ) {
    if ((iVar8.field100_0x68 * 0x3) <= uStack30) break;
    puVar1 = iVar8.field97_0x62;
    uVar7 = &iVar8.field98_0x64;
    uVar10 = (uVar7 >> 0x10);
    iVar11 = (astruct_233 *)uVar7;
    (iVar11 + iStack32 * 0x4) = iVar8.field96_0x60 + uStack30 * 0x8;
    *(u8 **)(iVar11 + iStack32 * 0x4 + 0x2) = puVar1;
    uStack30 += 0x3;
    iStack32 += 0x1;
  }
  return;
}
