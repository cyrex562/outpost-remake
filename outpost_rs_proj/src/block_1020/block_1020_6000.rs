
pub fn pass1_1020_6184(mut param_1: u32,mut param_2: u16 )

{
  HCURSOR16 HVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0xf4) == 0x1) {
    HVar1 = SetCursor16(*(HCURSOR16 *)(iVar2 + 0xf0));
    *(HCURSOR16 *)(iVar2 + 0xee) = HVar1;
    (iVar2 + 0x10c) = param_2;
    SetCapture16(*(HWND16 *)(iVar2 + 0x8));
    (iVar2 + 0xf4) = 0x2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_61c4(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,param_4: *mut u16)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut in_EDX: *mut Struct57;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000fff2: u16;

  puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff2,0x2f),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  uVar2 = (puVar3 >> 0x10);
  uVar1 = (puVar3 + 0x20);
  pass1_1030_8308(uVar1,uVar2,_u16_1050_5748,(_u16_1050_5748 >> 0x10),(u16 *)param_3,
                  param_4,uVar1);
  *param_4 = (puVar3 + 0x1e);
  return;
}



StructD * pass1_1020_6208(StructD *param_1,param_2: u8)

{
  param_1 = (StructD *)(param_1 & 0xffff0000 | (param_1 - 0xe2));
  destroy_cursor_1020_42f4(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



StructD * FUN_1020_6216(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  destroy_cursor_1020_42f4(param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_2);
  }
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_62e0(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 )

{
  let mut puVar1: *mut u32;
  code **ppcVar2;
  let mut puVar3: *mut u16;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut in_EDX: *mut Struct57;
  let mut paVar8: *mut Struct57;
  let mut paVar9: *mut Struct57;
  let mut puVar10: *mut u32;
  let mut in_stack_0000fe3e: u16;
  let mut in_stack_0000fe84: u16;
  let mut in_stack_0000fe96: u16;
  let mut in_stack_0000ff62: u16;
  let mut in_stack_0000ff68: u16;
  let mut in_stack_0000ff6c: u16;
  let mut in_stack_0000ffa8: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc6: u32;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut iVar14: i16;
  let mut uVar15: u16;
  let mut in_stack_0000ffee: u16;
  let mut paVar7: *mut Struct57;

  set_struct_op_1020_921c(in_EDX,(StructA *)CONCAT22(param_2,param_1),param_3,in_stack_0000ffc6);
  (param_1 + 0x14) = 0x0;
  (param_1 + 0x2c) = 0x0;
  CONCAT22(param_2,param_1) = 0x67c2;
  (param_1 + 0x2) = 0x1020;
  puVar3 = pass1_1000_4906((StructD *)CONCAT22(param_2,param_1 + 0x18),NULL,0x14);
  mem_op_1000_179c(0x3c,in_EDX);
  uVar5 = in_EDX | puVar3;
  paVar7 = (astruct_57 *)(in_EDX & 0xffff0000 | uVar5);
  if (uVar5 == 0x0) {
    (param_1 + 0x1c) = 0x0;
  }
  else {
    pass1_1020_87c2((astruct_20 *)CONCAT22(in_EDX,puVar3));
    (u16*)(param_1 + 0x1c) = puVar3;
    (param_1 + 0x1e) = paVar7;
  }
  mem_op_1000_179c(0x26,paVar7);
  uVar5 = paVar7 | puVar3;
  paVar9 = (astruct_57 *)(paVar7 & 0xffff0000);
  paVar8 = (astruct_57 *)(paVar9 | uVar5);
  if (uVar5 == 0x0) {
    puVar3 = NULL;
  }
  else {
    pass1_1020_8a9c((astruct_20 *)CONCAT22(paVar7,puVar3),paVar8,in_stack_0000ff68,in_stack_0000ff6c,
                    in_stack_0000ff62,in_stack_0000fe3e);
    paVar9 = paVar8;
  }
  (u16*)(param_1 + 0x20) = puVar3;
  (param_1 + 0x22) = paVar9;
  mem_op_1000_179c(0xbe,paVar9);
  uVar5 = paVar9 | puVar3;
  paVar7 = (astruct_57 *)(paVar9 & 0xffff0000);
  paVar8 = (astruct_57 *)(paVar7 | uVar5);
  if (uVar5 == 0x0) {
    puVar3 = NULL;
  }
  else {
    pass1_1020_8eaa((astruct_20 *)CONCAT22(paVar9,puVar3),paVar8);
    paVar7 = paVar8;
  }
  (u16*)(param_1 + 0x24) = puVar3;
  (param_1 + 0x26) = paVar7;
  mem_op_1000_179c(0x20,paVar7);
  uVar5 = paVar7 | puVar3;
  paVar9 = (astruct_57 *)(paVar7 & 0xffff0000);
  paVar8 = (astruct_57 *)(paVar9 | uVar5);
  if (uVar5 == 0x0) {
    puVar3 = NULL;
  }
  else {
    pass1_1020_8360((astruct_20 *)CONCAT22(paVar7,puVar3));
    paVar9 = paVar8;
  }
  (u16*)(param_1 + 0x28) = puVar3;
  (param_1 + 0x2a) = paVar9;
  pass1_1020_6746(CONCAT22(param_2,param_1),0x1,0x4);
  puVar10 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffee,0x29),in_stack_0000fe96,
                            in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
  paVar7 = (astruct_57 *)(paVar9 & 0xffff0000 | puVar10 >> 0x10);
  (param_1 + 0x14) = puVar10;
  uVar6 = (puVar10 >> 0x10);
  (param_1 + 0x16) = uVar6;
  uVar13 = 0x0;
  uVar12 = (param_1 + 0x14);
  ppcVar2 = (code **)((param_1 + 0x14) + 0x4);
  iVar14 = param_1;
  uVar15 = param_2;
  (**ppcVar2)();
  (param_1 + 0x6) = (param_1 + 0x14);
  uVar4 = (param_1 + 0x14);
  puVar1 = (uVar4 + 0xa);
  uVar4 = CONCAT22(param_2,param_1 + 0xa);
  uVar11 = SUB42(puVar1,0x0);
  ppcVar2 = (code **)(*puVar1 + 0x8);
  (**ppcVar2)(0x1010,uVar11,(puVar1 >> 0x10),uVar4,uVar12,uVar6,uVar13,iVar14,uVar15);
  (param_1 + 0x12) = uVar4;
  (param_1 + 0x10) = 0x1;
  puVar10 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(uVar11,0x2),in_stack_0000fe84,in_stack_0000ffa8
                            ,in_stack_0000ffae,in_stack_0000ffb2);
  (param_1 + 0x2c) = puVar10;
  (param_1 + 0x2e) = (puVar10 >> 0x10);
  return;
}
pub fn pass1_1020_6466(StructD *param_1)

{
  StructD *iVar1;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x67c2;
  iVar1->address_offset_field_0x2 = 0x1020;
  if (iVar1->field12_0x14 != 0x0) {
    pass1_1010_1ea6(iVar1->field12_0x14,(StructD *)(param_1 & 0xffff | uVar1 << 0x10));
  }
  palette_op_1020_92c4(param_1);
  return;
}



pub fn pass1_1020_6498(mut param_1: u32,mut param_2: i16) -> u32

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  if (*(i32 *)(param_1 + 0x18 + param_2 * 0x4) != 0x0) {
    uVar1 = (param_1 + 0x18 + param_2 * 0x4);
    uVar3 = (uVar1 >> 0x10);
    iVar2 = uVar1;
    return CONCAT22((iVar2 + 0xa),(iVar2 + 0x8));
  }
  return 0x0;
}



u16 pass1_1020_64d4(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)(param_1 + 0x18 + param_2 * 0x4) != 0x0) {
    uVar1 = (param_1 + 0x18 + param_2 * 0x4);
    return (uVar1 + 0x4);
  }
  return 0x0;
}
pub fn mix_draw_op_1020_650c(param_1: *mut StructA)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let struct_1: *mut StructA;
  let mut uVar3: u16;
  u8 local_28 [0x20];
  let mut iStack8: i16;
  let mut puStack6: *mut u32;

  uVar3 = (param_1 >> 0x10);
  struct_1 = (StructA *)param_1;
  uVar2 = &struct_1->field10_0x14;
  puStack6 = (uVar2 + 0xa);
  if ((struct_1->field8_0x10 != 0x0) ||
     (uVar2 = &struct_1->field10_0x14, (uVar2 + 0x24) != 0x0)) {
    draw_op_1020_9364(param_1);
    if (*(i32 *)&struct_1->field19_0x24 != 0x0) {
      uVar2 = &struct_1->field19_0x24;
      ppcVar1 = (code **)(*(u32*)&struct_1->field19_0x24 + 0x14);
      (**ppcVar1)(0x1020,uVar2,(uVar2 >> 0x10));
    }
  }
  iStack8 = 0x0;
  do {
    if (*(i32 *)(&struct_1->field12_0x18 + iStack8 * 0x2) != 0x0) {
      uVar2 = (&struct_1->field12_0x18 + iStack8 * 0x2);
      ppcVar1 = (code **)(*(u32*)(&struct_1->field12_0x18 + iStack8 * 0x2) + 0x8);
      (**ppcVar1)(0x1020,(char)uVar2,(uVar2 >> 0x10),puStack6,(puStack6 >> 0x10));
    }
    iStack8 += 0x1;
  } while (iStack8 < 0x5);
  BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_28),struct_1->field2_0x4);
  ppcVar1 = (code **)(*puStack6 + 0x4);
  (**ppcVar1)(s_tile2_bmp_1050_1538,puStack6,(char)(puStack6 >> 0x10),0x0,0x0,&struct_1->field5_0xa,
              uVar3);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,local_28),struct_1->field2_0x4);
  return;
}
pub fn unk_win_op_1020_65cc(param_1: *mut astruct_60,mut param_2: i16)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut BVar3: bool;
  let mut uVar4: u16;
  astruct_59 *iVar4;
  astruct_60 *iVar5;
  let mut iVar6: i16;
  astruct_60 *uVar7;
  let mut iStack4: i16;

  iVar5 = (astruct_60 *)param_1;
  uVar7 = (astruct_60 *)(param_1 >> 0x10);
  if (param_2 == 0x1) {
    iVar5->field20_0x14 = 0x0;
    return;
  }
  if (param_2 == 0x2) {
    for (iStack4 = 0x0; iStack4 < 0x5; iStack4 += 0x1) {
      iVar6 = iStack4 * 0x4;
      if (((&iVar5->field_0x1a + iVar6) | (&iVar5->field_0x18 + iVar6)) != 0x0) {
        ppcVar1 = (code **)(*(u32*)(&iVar5->field_0x18 + iVar6) + 0x4);
        (**ppcVar1)();
      }
    }
  }
  else if (((0x0 < param_2 + -0x3) && (!SBORROW2(param_2 + -0x3,0x1))) && (param_2 + -0x4 < 0x4)) {
    BVar3 = IsIconic16(HWND16_1050_0396);
    if (BVar3 == 0x0) {
      BVar3 = IsIconic16(*(HWND16 *)&iVar5->field_0x4);
      if ((BVar3 == 0x0) && (uVar2 = iVar5->field20_0x14, (uVar2 + 0x24) != 0x0)) {
        InvalidateRect16(0x0,NULL,0x0);
        uVar4 = pass1_1020_64d4(param_1,0x2);
        if (uVar4 == 0x0) {
          pass1_1020_6746(param_1,0x1,0x2);
        }
        uVar4 = pass1_1020_64d4(param_1,0x3);
        if (uVar4 == 0x0) {
          pass1_1020_6746(param_1,0x1,0x3);
        }
        uVar4 = pass1_1018_255e(iVar5->field20_0x14);
        if (uVar4 == 0x0) {
          SendMessage16(0x0,0x69,0x111,*(HWND16 *)&iVar5->field_0x4);
        }
        else {
          uVar4 = pass1_1020_64d4(param_1,0x1);
          if (uVar4 == 0x0) {
            pass1_1020_6746(param_1,0x1,0x1);
          }
        }
        SendMessage16(0x0,0xf0,0x111,*(HWND16 *)&iVar5->field_0x4);
        uVar2 = iVar5->field41_0x2c;
        if ((uVar2 + 0x7a) != 0x0) {
          uVar2 = iVar5->field41_0x2c;
          (uVar2 + 0x7a) = 0x0;
          SendMessage16(0x0,0x131,0x111,*(HWND16 *)&iVar5->field_0x4);
          return;
        }
      }
    }
  }
  return;
}
pub fn pass1_1020_6746(mut param_1: u32,mut param_2: i16,mut param_3: i16)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut uVar4: u16;

  if (param_3 != 0x0) {
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if (*(i32 *)(iVar3 + 0x18 + param_3 * 0x4) != 0x0) {
      uVar2 = (iVar3 + 0x18 + param_3 * 0x4);
      (uVar2 + 0x4) = param_2;
      (iVar3 + 0x10) = 0x1;
      if (param_2 == 0x0) {
        ppcVar1 = (code **)((iVar3 + 0x18 + param_3 * 0x4) + 0x14);
        (**ppcVar1)();
      }
    }
  }
  return;
}



StructD * pass1_1020_679c(StructD *param_1,param_2: u8)

{
  pass1_1020_6466(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn unk_win_ui_op_1020_67ce(StructA *in_struct_1,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  HGDIOBJ16 HVar1;
  HCURSOR16 hcursor_2;
  let struct_a_1_lo: *mut StructA;
  let struct_a_1_hi: *mut StructA;
  let mut in_stack_0000fe10: u16;
  let mut in_stack_0000fe14: u16;
  let mut in_stack_0000fe66: u16;
  let mut in_stack_0000ff3a: u16;
  let mut in_stack_0000ff3e: u16;
  let mut in_stack_0000ff42: u16;
  let mut in_stack_0000ff82: u16;
  let mut in_stack_0000ff8a: u16;
  let mut in_stack_0000ff90: u16;
  let mut in_stack_0000ff94: u16;

  struct_1020_790e(&in_struct_1->field0_0x0,s_TPPOPMENU_1050_43fa,param_2,param_3);
  struct_a_1_hi = (StructA *)(in_struct_1 >> 0x10);
  struct_a_1_lo = (StructA *)in_struct_1;
  &struct_a_1_lo[0x1].field20_0x26 = 0x0;
  &struct_a_1_lo[0x1].field22_0x2a = 0x0;
  in_struct_1->field0_0x0 = 0x70e6;
  struct_a_1_lo->field1_0x2 = 0x1020;
  unk_str_op_1000_3d3e
            ((in_struct_1 & 0xffff0000 | ZEXT24(&struct_a_1_lo->field60_0x5b)),s_VrMode2_1050_4404);
  HVar1 = GetStockObject16(HOLLOW_BRUSH);
  struct_a_1_lo->field157_0xc6 = HVar1;
  hcursor_2 = LoadCursor16(0x7f00,0x0);
  struct_a_1_lo->field156_0xc4 = hcursor_2;
  struct_a_1_lo->field140_0xac = 0x44c00000;
  struct_a_1_lo->field158_0xc8 = 0x2020;
  struct_a_1_lo->field149_0xbc = (param_3 + 0x8);
  struct_a_1_lo->field159_0xca = param_2;
  win_ui_reg_class_1008_96d2(in_struct_1);
  window_op_1020_6c3a(param_4,in_struct_1,in_stack_0000fe66,in_stack_0000ff82,in_stack_0000ff8a,in_stack_0000ff90,
                      in_stack_0000ff94,in_stack_0000fe10,in_stack_0000fe14,in_stack_0000ff3a,in_stack_0000ff3e,
                      in_stack_0000ff42);
  return;
}
pub fn pass1_1020_687c(param_1: *mut astruct_868)

{
  u8 unaff_BP;

  get_win_ui_info_op_1020_7a50(param_1);
  destroy_icon_1020_6bd2(param_1,unaff_BP);
  return;
}



// WARNING: Unable to use type for symbol uVar2
pub fn realize_palette_1020_6896(mut param_1: u32,mut param_2: i16)

{
  code **ppcVar1;
  let mut uVar3: u32;
  let mut puVar4: u32;
  astruct_801 *iVar4;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar2: u32;

  if (param_2 != 0x0) {
    uVar4 = (param_1 >> 0x10);
    uVar2 = (param_1 + 0xf2);
    uVar5 = (uVar2 >> 0x10);
    iVar4 = (astruct_801 *)uVar2;
    puVar4 = iVar4->field36_0x24;
    ppcVar1 = (code **)(puVar4 + 0x18);
    (**ppcVar1)();
    UnrealizeObject16((HGDIOBJ16)puVar4);
    uVar3 = (param_1 + 0xf2);
    RealizePalette16(*(HDC16 *)(uVar3 + 0x178));
  }
  return;
}
pub fn pass1_1020_68de(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if (*(i32 *)(param_1 + 0xf6) != 0x0) {
    invalidate_rect_1020_735a((param_1 + 0xf6));
  }
  return;
}
pub fn pt_in_rect_1020_68fc(u32 *param_1,mut param_2: u16 ,mut param_3: u16 )

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut uVar4: u16;
  POINT16 PStack6;

  PStack6 = (POINT16)CONCAT22(param_2,param_3);
  uVar4 = (param_1 >> 0x10);
  uVar2 = pass1_1018_31d0(*(astruct_126 **)(param_1 + 0xf2));
  if (uVar2 != 0x0) {
    BVar3 = PtInRect16(PStack6,(RECT16 *)((param_1 + 0xf2) + 0x16c));
    if (BVar3 != 0x0) {
      ppcVar1 = (code **)(*param_1 + 0x40);
      (**ppcVar1)(s_tile2_bmp_1050_1538,param_1,0xef);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 unk_destroy_win_op_1020_694c(mut param_1: u16 ,param_2: *mut StructA,mut param_3: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut HVar4: HWND16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut uVar6: u32;
  let iVar5: *mut StructA;
  let uVar4: *mut StructA;
  let mut unaff_CS: u16;
  let mut wparam: WPARAM16;

  uVar6 = CONCAT22(in_register_0000000a,param_1);
  uVar1 = param_3;
  if (param_3 != 0x12b) {
    iVar5 = (StructA *)param_2;
    uVar4 = (StructA *)(param_2 >> 0x10);
    if (param_3 < 0x12c) {
      if (param_3 == 0x6f) {
        uVar2 = FUN_1010_830a(0x0,uVar6,unaff_CS,_u16_1050_14cc,0x1f8);
        BVar3 = WinHelp16(0x29,0x1,CONCAT22(uVar6,uVar2),iVar5->field4_0x8);
        return BVar3;
      }
      if (param_3 == 0xeb) {
        uVar1 = GetDlgItem16(0x1797,iVar5->field4_0x8);
        uVar5 = uVar6;
        if (uVar1 != 0x0) {//
LAB_1020_6a6f:
          win_ui_fn_1020_6e98(uVar5,param_2);
          return uVar1;
        }
      }
      else {
        uVar1 = param_3 - 0xef;
        if (uVar1 == 0x0) {
          pass1_1018_2e28(*(astruct_126 **)&iVar5[0x1].field20_0x26);
          pass1_1008_3e0e(param_2);
        }
        else {
          uVar1 = param_3 - 0x129;
          if ((uVar1 != 0x0) && (uVar1 = param_3 - 0x12a, uVar1 == 0x0)) {
            HVar4 = iVar5->field4_0x8;
            wparam = 0xf012;//
LAB_1020_69c3:
            BVar3 = PostMessage16(0x0,wparam,0x112,HVar4);
            return BVar3;
          }
        }
      }
    }
    else if (param_3 == 0xbb8) {
      HVar4 = GetDlgItem16(0x1797,iVar5->field4_0x8);
      if (HVar4 != 0x0) {
        DestroyWindow16(HVar4);
      }
      uVar1 = pass1_1018_31d0(*(astruct_126 **)&iVar5[0x1].field20_0x26);
      if (uVar1 != 0x0) {
        uVar1 = pass1_1018_2d9a(*(astruct_126 **)&iVar5[0x1].field20_0x26);//
LAB_1020_6a0b:
        invalidate_rect_1020_735a(&iVar5[0x1].field22_0x2a);
        return uVar1;
      }
    }
    else if (param_3 < 0xbb9) {
      if (param_3 == 0x12c) {
        HVar4 = iVar5->field4_0x8;
        wparam = 0xf020;
        goto LAB_1020_69c3;
      }
      uVar1 = param_3 - 0x12d;
      if (param_3 != 0x12c) {
        uVar1 = param_3 - 0x12e;
      }
    }
    else if (param_3 == 0xbb9) {
      HVar4 = GetDlgItem16(0x1797,iVar5->field4_0x8);
      if (HVar4 != 0x0) {
        DestroyWindow16(HVar4);
      }
      uVar1 = pass1_1018_31d0(*(astruct_126 **)&iVar5[0x1].field20_0x26);
      if (uVar1 != 0x0) {
        uVar1 = pass1_1018_2dde(*(astruct_126 **)&iVar5[0x1].field20_0x26);
        goto LAB_1020_6a0b;
      }
    }
    else {
      uVar1 = param_3 - 0xbba;
      if (uVar1 == 0x0) {
        uVar1 = GetDlgItem16(0x1797,iVar5->field4_0x8);
        uVar5 = uVar6;
        if (uVar1 != 0x0) {
          BVar3 = DestroyWindow16(uVar1);
          return BVar3;
        }
        goto LAB_1020_6a6f;
      }
    }
  }
  return uVar1;
}
pub fn win_ui_op_1020_6ae6(param_1: *mut astruct_877,mut param_2: u16 ,mut param_3: i16,mut param_4: i16,u16_t param_5,u16_t param_6)

{
  code **ppcVar1;
  let mut hwnd: HWND16;
  u8 *puVar2;
  astruct_877 *iVar3;
  let mut uVar3: u16;
  let mut LVar4: LRESULT;

  if (param_4 == 0x1797) {
    uVar3 = (param_1 >> 0x10);
    iVar3 = (astruct_877 *)param_1;
    hwnd = GetDlgItem16(0x1797,iVar3->field8_0x8);
    if (hwnd != 0x0) {
      if (param_3 == 0x2) {
        LVar4 = SendMessage16(0x0,0x0,0x409,hwnd);
        if (LVar4 != -0x1) {
          LVar4 = SendMessage16(CONCAT13(0x10,CONCAT12(0x50,&stack0xffa8)),(WPARAM16)LVar4,0x40a,hwnd);
          puVar2 = &stack0xffa8;
          pass1_1018_30ca((LVar4 >> 0x10),(astruct_504 *)iVar3->field241_0xf2,
                          CONCAT22(0x1050,puVar2));
          pass1_1018_2fe8((astruct_126 *)iVar3->field241_0xf2,param_5,param_6);
          if (puVar2 != NULL) {
            invalidate_rect_1020_735a(iVar3->field242_0xf6);
            ppcVar1 = (code **)(param_1 + 0x40);
            (**ppcVar1)(0x1018,iVar3);
          }
        }
      }
      else if (param_3 != 0x3) {
        return;
      }
      DestroyWindow16(hwnd);
    }
  }
  return;
}
pub fn enable_menu_item_1020_6b9a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16,HMENmut param_5: u16 )

{
  if (param_4 != 0x0) {
    return;
  }
  EnableMenuItem16(0x400,0x0,param_5);
  return;
}
pub fn pass1_1020_6bbc(mut param_1: u32)

{
  let mut in_DX: u16;

  win_ui_op_1020_737a(in_DX,*(astruct_788 **)(param_1 + 0xf6));
  return;
}
pub fn destroy_icon_1020_6bd2(param_1: *mut astruct_868,param_2: u8)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  astruct_868 *struct_1;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  struct_1 = (astruct_868 *)param_1;
  DestroyIcon16(struct_1->hicon_0xc2);
  struct_1->hicon_0xc2 = 0x0;
  struct_1->field8_0x8 = 0x0;
  puVar1 = struct_1->field241_0xf6;
  uVar2 = struct_1->field242_0xf8;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(s_tile2_bmp_1050_1538,puVar1,uVar2,0x1);
  }
  &struct_1->field241_0xf6 = 0x0;
  pass1_1010_1dda(struct_1->field240_0xf2);
  struct_1->field240_0xf2 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn window_op_1020_6c3a(param_1: *mut astruct_57,StructA *struct_param_1,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,
                        mut param_6: u16 ,mut param_7: u16 ,mut param_8: u16 ,mut param_9: u16 ,mut param_10: u16 ,
                        mut param_11: u16 ,mut param_12: u16 )

{
  let mut uVar1: u32;
  code **ppcVar2;
  HICON16 HVar3;
  let mut paVar4: *mut Struct57;
  INT16 *pIVar5;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut paVar9: *mut Struct57;
  let mut unaff_SI: u16;
  let mut puVar11: *mut u32;
  u8 uVar12;
  let mut local_6: u32;
  let struct_a_1: *mut StructA;
  let mut paVar10: *mut Struct57;
  let mut struct_a_1_hi: u16;

  struct_a_1 = (StructA *)struct_param_1;
  struct_a_1_hi = (struct_param_1 >> 0x10);
  create_window_ex_1008_9760(struct_param_1);
  puVar11 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x4),param_3,param_5,param_6,param_7)
  ;
  paVar9 = (astruct_57 *)(param_1 & 0xffff0000 | puVar11 >> 0x10);
  struct_a_1[0x1].field20_0x26 = (astruct_243 *)puVar11;
  uVar7 = (puVar11 >> 0x10);
  struct_a_1[0x1].field21_0x28 = uVar7;
  struct_a_1[0x1].field10_0x14 = (i16)struct_a_1[0x1].field20_0x26;
  struct_a_1[0x1].field11_0x16 = uVar7;
  HVar3 = LoadIcon16(s_TILEICON_1050_440c,HINSTANCE16_1050_038c);
  *(HICON16 *)&struct_a_1->field_0xc2 = HVar3;
  uVar6 = &struct_a_1[0x1].field20_0x26;
  ppcVar2 = (code **)(*(u32*)&struct_a_1[0x1].field20_0x26 + 0x30);
  (**ppcVar2)(s_tile2_bmp_1050_1538,uVar6,(uVar6 >> 0x10),HVar3);
  paVar4 = (astruct_57 *)(&local_6 + 0x2);
  pass1_1018_2d22(&struct_a_1[0x1].field20_0x26,CONCAT13(0x10,CONCAT12(0x50,&local_6)),
                  (u16 *)CONCAT13(0x10,CONCAT12(0x50,paVar4)),0xbb8);
  mem_op_1000_179c(0x42,paVar9);
  uVar8 = (astruct_57 *)paVar9 | paVar4;
  paVar10 = (astruct_57 *)(paVar9 & 0xffff0000 | uVar8);
  if (uVar8 != 0x0) {
    pass1_1008_3bd6(paVar10,paVar4,(astruct_57 *)paVar9,0x0,local_6,0x0,0x7c007d,
                    CONCAT22(struct_a_1->field4_0x8,0xbb8),param_4,param_8,param_9,param_10,param_11,param_12);
  }
  paVar4 = (astruct_57 *)(&local_6 + 0x2);
  pass1_1018_2d22(&struct_a_1[0x1].field20_0x26,CONCAT13(0x10,CONCAT12(0x50,&local_6)),
                  (u16 *)CONCAT13(0x10,CONCAT12(0x50,paVar4)),0xbb9);
  mem_op_1000_179c(0x42,paVar10);
  uVar8 = (astruct_57 *)paVar10 | paVar4;
  paVar9 = (astruct_57 *)(paVar10 & 0xffff0000 | uVar8);
  if (uVar8 != 0x0) {
    pass1_1008_3bd6(paVar9,paVar4,(astruct_57 *)paVar10,0x0,local_6,0x0,0x7e007f,
                    CONCAT22(struct_a_1->field4_0x8,0xbb9),param_4,param_8,param_9,param_10,param_11,param_12);
  }
  paVar4 = (astruct_57 *)(&local_6 + 0x2);
  pass1_1018_2d22(&struct_a_1[0x1].field20_0x26,CONCAT13(0x10,CONCAT12(0x50,&local_6)),
                  (u16 *)CONCAT13(0x10,CONCAT12(0x50,paVar4)),0xbba);
  mem_op_1000_179c(0x42,paVar9);
  uVar8 = (astruct_57 *)paVar9 | paVar4;
  paVar10 = (astruct_57 *)(paVar9 & 0xffff0000 | uVar8);
  if (uVar8 != 0x0) {
    pass1_1008_3bd6(paVar10,paVar4,(astruct_57 *)paVar9,0x0,local_6,0x1b2,0x1b001b1,
                    CONCAT22(struct_a_1->field4_0x8,0xbba),param_4,param_8,param_9,param_10,param_11,param_12);
  }
  mem_op_1000_179c(0x22,paVar10);
  uVar8 = paVar10 | paVar4;
  if (uVar8 == 0x0) {
    &struct_a_1[0x1].field22_0x2a = 0x0;
  }
  else {
    unk_win_ui_op_1020_717e(uVar8,param_5,(astruct_40 *)CONCAT22(paVar10,paVar4),struct_param_1);
    struct_a_1[0x1].field22_0x2a = (astruct_160 *)paVar4;
    &struct_a_1[0x1].field_0x2c = uVar8;
  }
  uVar6 = &struct_a_1[0x1].field22_0x2a;
  &struct_a_1[0x1].field14_0x1c = uVar6;
  uVar1 = &struct_a_1[0x1].field20_0x26;
  ppcVar2 = (code **)(*(u32*)&struct_a_1[0x1].field20_0x26 + 0x10);
  (**ppcVar2)(0x1000,(char)uVar1,(uVar1 >> 0x10));
  pIVar5 = (INT16 *)uVar6;
  MoveWindow16(0x1,pIVar5[0x3],pIVar5[0x2],pIVar5[0x1],*pIVar5,struct_a_1->field4_0x8);
  uVar12 = (u8)(struct_param_1 >> 0x10);
  uVar6 = struct_param_1;
  ppcVar2 = (code **)(uVar6 + 0x94);
  (**ppcVar2)(s_tile2_bmp_1050_1538,struct_a_1,uVar12,0x0);
  ppcVar2 = (code **)(uVar6 + 0x10);
  (**ppcVar2)(s_tile2_bmp_1050_1538,struct_a_1,uVar12,0x1);
  UpdateWindow16(struct_a_1->field4_0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_6e52(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,mut param_4: u16 ,mut param_5: i16)

{
  let mut uVar1: u16;
  let mut pcVar2: *mut c_char;

  pass1_1018_2e5e(param_1,param_2,*(astruct_126 **)(param_3 + 0xf2));
  uVar1 = param_2 | param_1;
  if (uVar1 == 0x0) {
    pcVar2 = load_string_1010_847e(_u16_1050_14cc,0x5a1);
  }
  else {
    pass1_1018_2d84(param_1,*(astruct_126 **)(param_3 + 0xf2));
    pcVar2 = CONCAT22(uVar1,param_1);
  }
  string_1020_79b4(CONCAT22(param_4,param_3),param_5,pcVar2);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
