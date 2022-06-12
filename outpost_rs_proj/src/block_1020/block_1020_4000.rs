
StructD * pass1_1020_4064(StructD *param_1,param_2: u8)

{
  win_ui_palette_op_1020_3e84(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1020_4092(param_1: *mut u16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  pass1_1008_3e38((astruct_19 *)param_1);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x6) = 0x0;
  (iVar1 + 0x8) = 0x0;
  (iVar1 + 0xa) = 0x1;
  (iVar1 + 0xc) = 0x0;
  (iVar1 + 0xe) = 0x0;
  pass1_1008_3e38((astruct_19 *)(param_1 & 0xffff0000 | (iVar1 + 0x10)));
  return param_1;
}
pub fn draw_rect_1020_40ce(mut param_1: u32,mut param_2: i16,mut param_3: i16,HDC16 hdc16_param_4,mut param_5: u16 )

{
  HPEN16 pen_handle;
  HGDIOBJ16 brush_handle_1;
  let mut right: i16;
  let mut bottom: i16;
  let mut unaff_SS: u16;
  HDC16 hdc;
  let mut local_6: i16;
  let mut local_4: i16;
  HDC16 hdc16_var_fff2;
  let mut iVar1: i16;

  pass1_1008_3e94((u16 *)(param_1 & 0xffff0000 | (param_1 + 0x10)),(u16 *)CONCAT22(0x1050,&local_6),
                  CONCAT22(0x1050,&local_4));
  pass1_1008_3e94((u16 *)param_1,(u16 *)CONCAT22(0x1050,&local_6),CONCAT22(0x1050,&local_4));
  iVar1 = (param_1 + 0xa);
  Ellipse16(iVar1 + local_6 + param_2,iVar1 + local_4 + param_3,(local_6 - (param_1 + 0xa)) + param_2,
            (local_4 - (param_1 + 0xa)) + param_3,hdc16_param_4);
  if ((*(param_1 + 0xe) & 0x1) != 0x0) {
    brush_handle_1 = GetStockObject16(HOLLOW_BRUSH);
    SelectObject16(brush_handle_1,hdc16_var_fff2);
    hdc = hdc16_param_4;
    pen_handle = CreatePen16(0x10000f9,0x1,0x0);
    SelectObject16(pen_handle,hdc);
    right = local_4 + param_3 + 0x5;
    bottom = local_6 + param_2 + 0x5;
    Rectangle16(bottom,right,local_6 + param_2 + -0x5,local_4 + param_3 + -0x5,hdc16_param_4);
    brush_handle_1 = GetStockObject16(WHITE_BRUSH);
    SelectObject16(brush_handle_1,right);
    brush_handle_1 = GetStockObject16(WHITE_PEN);
    brush_handle_1 = SelectObject16(brush_handle_1,bottom);
    DeleteObject16(brush_handle_1);
  }
  return;
}


/*
Unable to decompile 'unk_draw_op_1020_41c8'
Cause:
Low-level Error: Symbol $$undef0000000d extends beyond the end of the address space
*/
pub fn destroy_cursor_1020_42f4(StructD *param_1)

{
  StructD *struct_1;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  struct_1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x623c;
  struct_1->address_offset_field_0x2 = 0x1020;
  &struct_1->field_0xe2 = 0x62d8;
  &struct_1->field_0xe4 = 0x1020;
  if (struct_1[0x1].field13_0x18 != 0x0) {
    DestroyMenu16(struct_1[0x1].field13_0x18);
  }
  DestroyCursor16(struct_1[0x1].address_offset_field_0x2);
  DestroyCursor16(struct_1[0x1].hfile_0x4);
  pass1_1020_808e(param_1);
  return;
}
pub fn pass1_1020_434c(mut param_1: u16 ,mut param_2: i16,mut param_3: u16 ,u32 *param_4,mut param_5: u16 ,mut param_6: u16 ,mut param_7: i16)

{
  if (param_7 == 0x1) {
    pass1_1020_6184(CONCAT22(param_3,param_2),param_6);
    return;
  }
  if (param_7 == 0x2) {
    ui_op_1020_536e(param_1,CONCAT22(param_3,param_2),CONCAT22(param_5,param_4),param_6,0x2);
    return;
  }
  pass1_1008_68ea(param_2,param_3,param_4,param_5,param_6,param_7);
  return;
}
pub fn post_msg_1020_4394(mut param_1: u32,mut param_2: u16 )

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  iVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  if (param_2 == 0x10) {
    if ((iVar2 + 0x34) != 0x0) {
      PostMessage16(0x0,0xf6,0x111,HWND16_1050_0396);
      return;
    }
  }
  else if (param_2 < 0x11) {
    if ((char)param_2 == '\x01') {
      (iVar2 + 0x18) = 0x0;
      return;
    }
    if ((char)param_2 == '\v') {
      uVar1 = (iVar2 + 0x2c);
      (uVar1 + 0xe) = (iVar2 + -0xda);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_1020_43f6(param_1: *mut astruct_57,param_2: *mut StructA,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,
                  mut param_6: u16 ,mut param_7: u16 )

{
  code **ppcVar1;
  astruct_160 *paVar2;
  let mut uVar3: u16;
  let mut paVar4: *mut Struct57;
  let mut puVar5: *mut u32;
  let mut uVar6: u16;
  let struct_a_1: *mut StructA;

  struct_a_1 = (StructA *)param_2;
  uVar6 = (param_2 >> 0x10);
  create_window_ex_1008_9760(param_2);
  get_dc_1018_4db0(*(astruct_126 **)&struct_a_1[0x1].field25_0x2e,struct_a_1->field4_0x8);
  puVar5 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_7,0x32),param_3,param_4,param_5,param_6);
  paVar4 = (astruct_57 *)(param_1 & 0xffff0000);
  &struct_a_1[0x1].field38_0x42 = puVar5;
  (&struct_a_1[0x1].field38_0x42 + 0x2) = (puVar5 >> 0x10);
  if (param_2 != NULL) {
    paVar4 = (astruct_57 *)(paVar4 | uVar6);
  }
  ppcVar1 = (code **)(struct_a_1[0x1].field38_0x42 + 0x4);
  paVar2 = (astruct_160 *)(**ppcVar1)();
  mem_op_1000_179c(0x30,paVar4);
  uVar3 = paVar4 | paVar2;
  if (uVar3 == 0x0) {
    &struct_a_1[0x1].field22_0x2a = 0x0;
  }
  else {
    pass1_1020_62e0(paVar2,paVar4,struct_a_1->field4_0x8);
    struct_a_1[0x1].field22_0x2a = paVar2;
    &struct_a_1[0x1].field_0x2c = uVar3;
  }
  ui_op_1020_536e(uVar3,param_2,0x0,-0x1,0x3);
  return;
}
pub fn pass1_1020_44b0(u32 *param_1)

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (*(i32 *)(iVar2 + 0xf6) != 0x0) {
    ppcVar1 = (code **)(*param_1 + 0x98);
    (**ppcVar1)();
    (iVar2 + 0x112) = 0x0;
    ppcVar1 = (code **)((iVar2 + 0xf6) + 0x8);
    (**ppcVar1)();
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn mixed_menu_op_1020_44ec
               (param_1: *mut astruct_850,mut param_2: u16 ,mut param_3: i16,HMENmut param_4: u16 ,mut param_5: u16 ,undefined1 param_6)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut UVar4: u16;
  let mut BVar5: bool;
  HMENlet mut HVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u32;
  u8 *puVar9;
  let mut uVar10: u16;
  let mut in_register_0000000a: u16;
  let mut paVar11: *mut Struct57;
  astruct_850 *iVar9;
  let mut iVar12: i16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut data: *mut c_char;
  let mut puVar15: *mut u32;
  let mut in_stack_0000fd70: u16;
  let mut in_stack_0000fe94: u16;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000fe9e: u16;
  let mut w_flags: u16;
  let mut in_stack_0000fec8: u16;
  HMENlet mut w_item_id: u16;
  let mut uStack300: u16;
  u8 bStack293;
  let mut uStack278: u16;
  let mut uStack268: u32;
  u32 local_108 [0x40];
  let mut uStack8: u16;
  let mut puStack6: *mut u32;

  paVar11 = (astruct_57 *)CONCAT22(in_register_0000000a,param_5);
  uVar13 = (param_1 >> 0x10);
  iVar9 = (astruct_850 *)param_1;
  if (iVar9->hmenu_0x106 != 0x0) {
    if (iVar9->hmenu_0x106 == param_4) {
      puStack6 = mixed_1010_20ba(paVar11,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fec8,0x3),in_stack_0000fd70,
                                 in_stack_0000fe94,in_stack_0000fe9a,in_stack_0000fe9e);
      uVar2 = iVar9->field257_0x108;
      uStack8 = (uVar2 + 0x2e);
      uVar2 = iVar9->field257_0x108;
      uVar14 = (uVar2 >> 0x10);
      iVar12 = uVar2;
      uVar1 = (iVar12 + 0x42);
      puVar9 = *(u8 **)(iVar12 + 0x44);
      bStack293 = (u8)(uVar1 >> 0x18);
      uVar7 = bStack293;
      if (bStack293 == 0x0) {
        uVar3 = pass1_1020_bd80(uStack8);
        unk_str_op_1000_3d3e(CONCAT22(0x1050,local_108),CONCAT22(puVar9,uVar3));
      }
      else {
        pass1_1030_8344(_u16_1050_5748,uVar1 & 0xffff | ZEXT24(puVar9) << 0x10);
        pass1_1010_c3c2(puVar9,puStack6,(puStack6 >> 0x10),CONCAT22(0x1050,local_108),
                        CONCAT22(puVar9,uVar7));
      }
      ModifyMenu16(CONCAT22(0x1050,local_108),0x76,0x0,0x76,iVar9->hmenu_0x106);
      UVar4 = GetMenuState16(0x0,0x13c,iVar9->hmenu_0x106);
      if (UVar4 != 0xffff) {
        DeleteMenu16(0x0,0x13c,iVar9->hmenu_0x106);
      }
      BVar5 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x20);
      if (BVar5 != 0x0) {
        data = load_string_1010_847e(_u16_1050_14cc,0x74b);
        InsertMenu16(data,0x13c,0x400,0xffff,iVar9->hmenu_0x106);
      }
      if ((s_VrMode_1050_42ca + 0x8 + uStack8 * 0x2) == 0x0) {
        HVar6 = iVar9->hmenu_0x106;
        w_flags = 0x1;
        UVar4 = 0x77;
        goto LAB_1020_464c;
      }
      HVar6 = iVar9->hmenu_0x106;
      UVar4 = 0x77;
    }
    else {
      HVar6 = GetSubMenu16(0x1,iVar9->hmenu_0x106);
      if (HVar6 != param_4) goto LAB_1020_479e;
      EnableMenuItem16(0x1,0x200,HVar6);
      uVar10 = paVar11;
      EnableMenuItem16(0x1,0x201,HVar6);
      EnableMenuItem16(0x1,0x202,HVar6);
      uVar2 = iVar9->field257_0x108;
      uVar8 = (uVar2 + 0x42);
      pass1_1030_8344(_u16_1050_5748,uVar8);
      uVar7 = uVar8;
      if ((uVar10 | uVar7) == 0x0) {
        return;
      }
      uVar2 = (uVar7 + 0x2e);
      uVar7 = (uVar7 + 0x30);
      uStack278 = uVar2;
      if ((uVar7 | uStack278) == 0x0) {
        return;
      }
      uStack268 = (uStack278 + 0x200);
      local_108[0] = struct_op_1030_73a8((astruct_419 *)(uVar8 & 0xffff | uVar10 << 0x10),uStack268,uVar7);
      uVar13 = (local_108[0] >> 0x10);
      puStack6 = (u32*)(local_108[0] + 0x1c);
      uVar7 = (local_108[0] + 0x1e);
      if ((uVar7 | puStack6) != 0x0) {
        uStack268 = puStack6 & 0xffff | uVar7 << 0x10;
      }
      uStack268 &= 0xff;
      if (uStack268 != 0x1) {
        return;
      }
      if ((uStack268 & 0xff0000) != 0x0) {
        return;
      }
      uVar3 = pass1_1030_6fa0(uVar8 & 0xffff | uVar10 << 0x10);
      BVar5 = pass1_1008_c6ae(_u16_1050_06e0,uVar3,0x3f);
      if (BVar5 != 0x0) {
        BVar5 = EnableMenuItem16(0x0,0x201,HVar6);
      }
      if (*(i32 *)((uVar8 & 0xffff) + 0x36) != 0x0) {
        BVar5 = EnableMenuItem16(0x0,0x202,HVar6);
      }
      pass1_1030_69cc(BVar5,uStack268,uVar8 & 0xffff | uVar10 << 0x10);
      if (BVar5 == 0x0) {
        return;
      }
      UVar4 = 0x200;
    }
    w_flags = 0x0;
    goto LAB_1020_464c;
  }//
LAB_1020_479e:
  iVar12 = param_3 + -0x1;
  if (iVar12 == 0x0) {
    pass1_1018_2504(0x0,paVar11);
    if (iVar12 == 0x0) {
      UVar4 = 0x0;
      EnableMenuItem16(0x401,0x0,param_4);
      HVar6 = 0x1;//
LAB_1020_47e3:
      w_flags = 0x401;
      goto LAB_1020_464c;
    }
    UVar4 = 0x0;
    EnableMenuItem16(0x400,0x0,param_4);
    HVar6 = 0x1;
  }
  else if (param_3 == 0x2) {
    uVar3 = pass1_1020_64d4(iVar9->field246_0xf6,0x2);
    if (uVar3 == 0x0) {
      EnableMenuItem16(0x401,0x0,param_4);
      UVar4 = 0x401;
    }
    else {
      EnableMenuItem16(0x400,0x0,param_4);
      UVar4 = 0x400;
    }
    HVar6 = 0x1;
    EnableMenuItem16(UVar4,0x1,param_4);
    if ((PTR_LOOP_1050_0010 != NULL) || (iVar9->field255_0x102 == 0x0)) goto LAB_1020_47e3;
  }
  else {
    if (param_3 == 0x3) {
      HVar6 = 0x0;
      puVar15 = mixed_1010_20ba(paVar11,_u16_1050_0ed0,(u8 **)0x2f,in_stack_0000fd70,in_stack_0000fe94,
                                in_stack_0000fe9a,in_stack_0000fe9e);
      uVar13 = (puVar15 >> 0x10);
      uVar1 = (puVar15 + 0x20);
      uVar7 = (puVar15 + 0x22);
      uStack300 = uVar1;
      if ((uVar7 | uStack300) != 0x0) {
        pass1_1030_8308(&stack0xfecc,uVar7,_u16_1050_5748,(_u16_1050_5748 >> 0x10),
                        (u16 *)CONCAT22(0x1050,&stack0xfecc),(u16 *)CONCAT22(0x1050,&stack0xfec8),
                        uVar1 & 0xffff | uVar7 << 0x10);
      }
      UVar4 = 0x0;
      do {
        CheckMenuItem16(0x400,UVar4,param_4);
        w_item_id = param_4;
        EnableMenuItem16(0x401,UVar4,param_4);
        UVar4 += 0x1;
      } while (UVar4 < 0x5);
      CheckMenuItem16(0x408,w_item_id,param_4);
      for (UVar4 = 0x0; UVar4 <= HVar6; UVar4 += 0x1) {
        HVar6 = param_4;
        EnableMenuItem16(0x400,UVar4,param_4);
      }
      return;
    }
    if (param_3 != 0x4) {
      return;
    }
    UVar4 = 0x2;
    HVar6 = param_4;
  }
  w_flags = 0x400;//
LAB_1020_464c:
  EnableMenuItem16(w_flags,UVar4,HVar6);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_sys_op_1020_493c
               (mut param_1: u16 ,StructD *param_2,StructD *param_3,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 ,
               u32 *param_7)

{
  code **ppcVar1;
  let mut uVar2: u32;
  i32 lVar3;
  HCURSOR16 HVar4;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut puVar7: *mut u16;
  let mut puVar8: *mut u32;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut paVar15: *mut Struct57;
  StructD *uVar9;
  let mut uVar16: u16;
  let mut uVar17: u32;
  let mut puVar18: *mut u32;
  astruct_97 *paVar19;
  let mut pcVar20: *mut c_char;
  let mut in_stack_0000fb4e: u16;
  let mut in_stack_0000fb50: u16;
  let mut in_stack_0000fb52: u16;
  let mut in_stack_0000fc72: u16;
  let mut in_stack_0000fc74: u16;
  let mut in_stack_0000fc76: u16;
  let mut in_stack_0000fc78: u16;
  let mut in_stack_0000fc7a: u16;
  let mut in_stack_0000fc7c: u16;
  let mut in_stack_0000fc7e: u16;
  let mut in_stack_0000fc80: u16;
  let mut uStack852: u16;
  let mut local_24e: u16;
  let mut uStack588: u16;
  let mut local_144: u32;
  let mut uStack320: u32;
  let mut local_13c: u32;
  let mut uStack42: u16;
  let mut uStack38: u32;
  let mut uStack34: u16;
  let mut uStack32: u16;
  let mut uStack30: u32;
  let mut uStack26: u32;
  let mut uStack22: u32;
  let mut puStack18: *mut u32;
  u8 *puStack14;
  u8 *puStack12;
  let mut uStack10: u16;
  let mut uStack6: u32;

  if (param_4 == 0xe9) {
    return;
  }
  uVar9 = (StructD *)param_3;
  uVar13 = (param_3 >> 0x10);
  if (param_4 < 0xea) {
    switch(param_4) {
    case 0x69:
      iVar6 = 0x0;
      break;
    case 0x6a:
      iVar6 = 0x1;
      break;
    case 0x6b:
      iVar6 = 0x2;
      break;
    case 0x6c:
      iVar6 = 0x3;
      break;
    case 0x6d:
      iVar6 = 0x4;
      break;
    default:
      return;
    case 0x77:
      if ((&uVar9[0x1].field_0x1c | uVar9[0x1].field14_0x1a) == 0x0) {
        return;
      }
      uVar2 = &uVar9[0x1].field14_0x1a;
      uVar11 = (s_VrMode_1050_42ca + 0x8 + (uVar2 + 0x2e) * 0x2);
      uStack26 = (uStack26 & 0xffff0000 | uVar11);
      if (uVar11 == 0x0) {
        return;
      }
      uVar16 = FUN_1010_830a(uVar11,param_2,0x1020,_u16_1050_14cc,0x1f8);
      puStack18 = CONCAT22(param_2,uVar16);
      param_7 = uVar9->field5_0x8;
      WinHelp16(CONCAT13((u8)(uStack26 >> 0xf),
                         CONCAT12((u8)(uStack26 >> 0xf),
                                  uStack26 & 0xff | (u8)((long)uStack26 >> 0x8) << 0x8))
                ,0x1,CONCAT22(param_2,uVar16),(HWND16)param_7);
      return;
    case 0x78:
      puVar18 = mixed_1010_20ba((astruct_57 *)param_2,_u16_1050_0ed0,(u8 **)CONCAT22(param_7,0x45),
                                in_stack_0000fb52,in_stack_0000fc76,in_stack_0000fc7c,in_stack_0000fc80);
      uStack588 = (puVar18 >> 0x10);
      local_24e = puVar18;
      enum_child_windows_1010_01be();
      return;
    }
    set_cursor_1020_5764(param_3,iVar6);
    return;
  }
  if (param_4 == 0x132) {
    uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8,0x3);
    if (uVar10 == 0x0) {
      return;
    }
    uVar16 = 0xffff;
    goto LAB_1020_4ef8;
  }
  if (param_4 < 0x133) {
    if (param_4 == 0x102) {
      uVar16 = 0x1000;
      mem_op_1000_179c(0xb4,(astruct_57 *)param_2);
      uStack32 = param_2;
      uVar17 = param_2 & 0xffff0000 | (uStack32 | param_4);
      uStack34 = param_4;
      if ((uStack32 | param_4) == 0x0) {
        iVar6 = 0x0;
        uVar12 = 0x0;
      }
      else {
        uVar16 = SUB42(&PTR_LOOP_1050_1040,0x0);
        iVar6 = string_1040_8520(uVar17,(astruct_57 *)CONCAT22(uStack32,param_4),HWND16_1050_0396,0x20031,0x62b057b);
        uVar12 = uVar17;
      }
      local_144 = CONCAT22(uVar12,iVar6);
      ppcVar1 = (code **)(*local_144 + 0x74);
      (**ppcVar1)(uVar16,iVar6,uVar12);
      uStack320 = CONCAT22(uStack320,iVar6);
      if (iVar6 != 0x1) {
        return;
      }
      pass1_1028_837e((astruct_97 *)CONCAT22(0x1050,&local_13c));//
LAB_1020_4b6c:
      fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,&local_13c));
      return;
    }
    if (param_4 < 0x103) {
      switch(param_4) {
      case 0xf0:
        ui_op_1020_536e(param_2,param_3,0x0,-0x1,0x1);
        return;
      default:
        return;
      case 0xf6:
        if (&uVar9[0x1].field_0x28 != 0x0) {
          if (param_3 == NULL) {
            param_7 = NULL;
            uStack852 = 0x0;
          }
          else {
            param_7 = &uVar9->field_0xe2;
            uStack852 = uVar13;
          }
          param_2 = (StructD *)uStack852;
          pass1_1010_1ea6(_u16_1050_02a0,(StructD *)CONCAT22(uStack852,param_7));
          &uVar9[0x1].field_0x28 = 0x0;
        }
        iVar6 = 0x12;
        break;
      case 0xf7:
        unk_win_op_1010_7300(param_2,*(astruct_57 **)&uVar9[0x1].field19_0x20,0x0,0x9,0x0);
        return;
      case 0xfb:
        local_144 = mixed_1010_20ba((astruct_57 *)param_2,_u16_1050_0ed0,(u8 **)CONCAT22(param_7,0x3),
                                    in_stack_0000fb52,in_stack_0000fc76,in_stack_0000fc7c,in_stack_0000fc80);
        uStack320 = mixed_1010_20ba((astruct_57 *)(param_2 & 0xffff0000 | local_144 >> 0x10),
                                    _u16_1050_0ed0,(u8 **)CONCAT22(param_7,0x30),in_stack_0000fb52,
                                    in_stack_0000fc76,in_stack_0000fc7c,in_stack_0000fc80);
        pcVar20 = pass1_1010_375e(uStack320);
        pass1_1010_c25e(pcVar20,(pcVar20 >> 0x10),local_144,
                        (local_144 >> 0x10),pcVar20);
        return;
      case 0xfc:
        post_msg_1020_55b0(param_2,param_3,param_5,param_6);
        return;
      case 0x101:
        uStack26 = mixed_1010_20ba((astruct_57 *)param_2,_u16_1050_0ed0,(u8 **)CONCAT22(param_7,0x2f),
                                   in_stack_0000fb52,in_stack_0000fc76,in_stack_0000fc7c,in_stack_0000fc80);
        uVar16 = (uStack26 >> 0x10);
        uStack22 = (uStack26 + 0x24);
        uVar11 = (uStack26 + 0x26);
        paVar15 = (astruct_57 *)(param_2 & 0xffff0000 | uVar11);
        uStack22 = uVar11 | uStack22;
        if (uStack22 == 0x0) {
          uVar16 = 0x1000;
          mem_op_1000_179c(0xb4,paVar15);
          uStack32 = paVar15;
          uVar17 = paVar15 & 0xffff0000 | (uStack32 | uStack22);
          uStack34 = uStack22;
          if ((uStack32 | uStack22) == 0x0) {
            puVar8 = NULL;
            uVar11 = 0x0;
          }
          else {
            uVar16 = SUB42(&PTR_LOOP_1050_1040,0x0);
            puVar8 =
                     string_1040_8520(uVar17,(astruct_57 *)CONCAT22(uStack32,uStack22),HWND16_1050_0396,0x20030,
                                      0x730057b);
            uVar11 = uVar17;
          }
          uStack30 = CONCAT22(uVar11,puVar8);//
LAB_1020_4c5f:
          ppcVar1 = (code **)(*puVar8 + 0x74);
          (**ppcVar1)(uVar16,puVar8,uVar11);
          return;
        }
        uVar17 = pass1_1038_af40(uVar9,uVar11,_PTR_LOOP_1050_5b7c,uVar9->field5_0x8,0xe);
        puStack18 = mixed_1010_20ba((astruct_57 *)(paVar15 & 0xffff0000 | uVar17 >> 0x10),_u16_1050_0ed0,
                                    (u8 **)CONCAT22(param_7,0x43),in_stack_0000fb52,in_stack_0000fc76,
                                    in_stack_0000fc7c,in_stack_0000fc80);
        uVar16 = (puStack18 >> 0x10);
        iVar6 = puStack18;
        puStack14 = (iVar6 + 0xa);
        uStack10 = (iVar6 + 0xc);
        uVar13 = (iVar6 + 0xe);
        uStack6 = CONCAT22(uStack6,uVar13);
        if ((iVar6 + 0x10) != 0x0) {
          return;
        }
        pass1_1028_84ca((astruct_97 *)CONCAT22(0x1050,&local_13c),uStack22,uVar13,uStack10,puStack14);
        goto LAB_1020_4b6c;
      }
    }
    else {
      if (param_4 != 0x106) {
        if (param_4 < 0x107) {
          if (param_4 == 0x103) {
            local_144 = mixed_1010_20ba((astruct_57 *)param_2,_u16_1050_0ed0,(u8 **)CONCAT22(param_7,0x2f),
                                        in_stack_0000fb52,in_stack_0000fc76,in_stack_0000fc7c,in_stack_0000fc80);
            uVar16 = (local_144 >> 0x10);
            uStack320 = *(char **)(local_144 + 0x24);
            uVar11 = (local_144 + 0x26);
            paVar15 = (astruct_57 *)(param_2 & 0xffff0000 | uVar11);
            uStack34 = uVar11 | uStack320;
            if (uStack34 != 0x0) {
              uVar17 = pass1_1038_af40(uVar9,uVar11,_PTR_LOOP_1050_5b7c,uVar9->field5_0x8,0xf);
              local_13c = (astruct_477 *)
                          mixed_1010_20ba((astruct_57 *)(paVar15 & 0xffff0000 | uVar17 >> 0x10),_u16_1050_0ed0,
                                          (u8 **)CONCAT22(param_7,0x42),in_stack_0000fb52,in_stack_0000fc76,
                                          in_stack_0000fc7c,in_stack_0000fc80);
              uStack42 = (local_13c + 0xa);
              if (uStack42 == 0x0) {
                return;
              }
              pass1_1030_e63e((astruct_97 *)CONCAT22(0x1050,&local_24e),uStack42);
              fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,&local_24e));
              return;
            }
            uVar16 = 0x1000;
            mem_op_1000_179c(0xb4,paVar15);
            uStack32 = paVar15;
            uVar17 = paVar15 & 0xffff0000 | (uStack32 | uStack34);
            if ((uStack32 | uStack34) == 0x0) {
              puVar8 = NULL;
              uVar11 = 0x0;
            }
            else {
              uVar16 = SUB42(&PTR_LOOP_1050_1040,0x0);
              puVar8 =
                       string_1040_8520(uVar17,(astruct_57 *)CONCAT22(uStack32,uStack34),HWND16_1050_0396,0x20030,
                                        0x730057b);
              uVar11 = uVar17;
            }
            uStack38 = CONCAT22(uVar11,puVar8);
          }
          else {
            if (param_4 != 0x104) {
              return;
            }
            uVar16 = 0x22;
            puVar18 = mixed_1010_20ba((astruct_57 *)param_2,_u16_1050_0ed0,(u8 **)0x220003,in_stack_0000fb50,
                                      in_stack_0000fc74,in_stack_0000fc7a,in_stack_0000fc7e);
            paVar15 = (astruct_57 *)(param_2 & 0xffff0000 | puVar18 >> 0x10);
            uStack34 = puVar18;
            uStack588 = (puVar18 >> 0x10);
            local_24e = uStack34;
            pass1_1010_af66(uStack588,puVar18,uVar16);
            local_144 = CONCAT22(local_144,uStack34);
            if (uStack34 != 0x0) {
              uVar16 = 0x1000;
              mem_op_1000_179c(0xb4,paVar15);
              uStack32 = paVar15;
              uVar17 = paVar15 & 0xffff0000 | (uStack32 | uStack34);
              if ((uStack32 | uStack34) == 0x0) {
                iVar6 = 0x0;
                uVar12 = 0x0;
              }
              else {
                uVar16 = SUB42(&PTR_LOOP_1050_1040,0x0);
                iVar6 = string_1040_8520(uVar17,(astruct_57 *)CONCAT22(uStack32,uStack34),HWND16_1050_0396,0x20031,
                                         0x62c057b);
                uVar12 = uVar17;
              }
              uStack320 = CONCAT22(uVar12,iVar6);
              ppcVar1 = (code **)(*uStack320 + 0x74);
              (**ppcVar1)(uVar16,iVar6,uVar12);
              local_13c = (astruct_477 *)CONCAT22(local_13c,iVar6);
              if (iVar6 != 0x1) {
                return;
              }
              paVar19 = pass1_1030_e79a((astruct_97 *)CONCAT22(0x1050,&param_7));
              uVar13 = (paVar19 >> 0x10);
              puVar7 = &param_7;
              fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,puVar7));
              win_1008_5c5c(puVar7,uVar13,_u16_1050_02a0,0x1e6);
              return;
            }
            uVar16 = 0x1000;
            mem_op_1000_179c(0xb4,paVar15);
            uStack32 = paVar15;
            uVar17 = paVar15 & 0xffff0000 | (uStack32 | uStack34);
            if ((uStack32 | uStack34) == 0x0) {
              puVar8 = NULL;
              uVar11 = 0x0;
              param_7 = puVar8;
              uStack852 = uVar11;
            }
            else {
              uVar16 = SUB42(&PTR_LOOP_1050_1040,0x0);
              puVar8 =
                       string_1040_8520(uVar17,(astruct_57 *)CONCAT22(uStack32,uStack34),HWND16_1050_0396,0x20030,
                                        0x731057b);
              uVar11 = uVar17;
              param_7 = puVar8;
              uStack852 = uVar11;
            }
          }
          goto LAB_1020_4c5f;
        }
        if (param_4 == 0x12f) {
          pass1_1020_61c4(uVar9,uVar13,CONCAT22(0x1050,&local_144),(u16 *)CONCAT22(0x1050,&local_24e));
          iVar6 = local_24e + 0x6a;
        }
        else {
          if (param_4 != 0x130) {
            if (param_4 != 0x131) {
              return;
            }
            uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8,0x2);
            if (uVar10 == 0x0) {
              return;
            }
            iVar6 = 0x7;
            goto LAB_1020_49b7;
          }
          pass1_1020_61c4(uVar9,uVar13,CONCAT22(0x1050,&local_144),(u16 *)CONCAT22(0x1050,&local_24e));
          iVar6 = local_24e + 0x68;
        }
        uStack320 = CONCAT22(uStack320,iVar6);
        if (0x6d < iVar6) {
          return;
        }
        if (iVar6 < 0x69) {
          return;
        }
        ppcVar1 = (code **)(param_3 + 0x40);
        (**ppcVar1)();
        return;
      }
      iVar6 = 0x13;
    }//
LAB_1020_49b7:
    pass1_1038_af40(uVar9,param_2,_PTR_LOOP_1050_5b7c,uVar9->field5_0x8,iVar6);
    return;
  }
  if (param_4 == 0x1c8) {
    lVar3 = uVar9[0x1].field12_0x14;
    SendMessage16(0x0,0x72,0x111,*(HWND16 *)(lVar3 + 0x8));
    return;
  }
  if (0x1c8 < param_4) {
    if (param_4 == 0x1ca) {
      local_144 = mixed_1010_20ba((astruct_57 *)param_2,_u16_1050_0ed0,(u8 **)CONCAT22(param_7,0x3),
                                  in_stack_0000fb52,in_stack_0000fc76,in_stack_0000fc7c,in_stack_0000fc80);
      uVar17 = param_2 & 0xffff0000;
      uStack320 = pass1_1010_c234(local_144,(local_144 >> 0x10));
      uVar11 = uStack320;
      uVar14 = (uStack320 >> 0x10);
      if ((uVar14 | uVar11) == 0x0) {
        return;
      }
      local_13c = (astruct_477 *)
                  mixed_1010_20ba((astruct_57 *)(uVar17 & 0xffff0000 | (uVar14 | uVar11)),_u16_1050_0ed0,
                                  (u8 **)CONCAT22(uVar11,0x30),in_stack_0000fb4e,in_stack_0000fc72,
                                  in_stack_0000fc78,in_stack_0000fc7c);
      param_2 = (StructD *)(local_13c >> 0x10);
      pass1_1010_3770((local_13c >> 0x10),local_13c,CONCAT22(uVar14,uVar11));
      iVar6 = 0x3;
    }
    else if (param_4 == 0x200) {
      uVar2 = &uVar9[0x1].field14_0x1a;
      uVar16 = (uVar2 >> 0x10);
      iVar6 = uVar2;
      uStack26 = (u32*)(iVar6 + 0x42);
      uVar11 = (iVar6 + 0x44);
      param_2 = (StructD *)uVar11;
      uStack26._3_1_ = (u8)(uStack26 >> 0x18);
      puStack14 = uStack26._3_1_;
      if (uStack26._3_1_ != 0x5) {
        return;
      }
      pass1_1030_8344(_u16_1050_5748,uStack26 & 0xffff | uVar11 << 0x10);
      PTR_LOOP_1050_5f0e = param_2;
      iVar6 = 0x25;
      PTR_LOOP_1050_5f0c = puStack14;
      puStack12 = PTR_LOOP_1050_5f0e;
    }
    else if (param_4 == 0x201) {
      uVar2 = &uVar9[0x1].field14_0x1a;
      uVar16 = (uVar2 >> 0x10);
      iVar6 = uVar2;
      uStack26 = (u32*)(iVar6 + 0x42);
      uVar11 = (iVar6 + 0x44);
      param_2 = (StructD *)uVar11;
      uStack26._3_1_ = (u8)(uStack26 >> 0x18);
      puStack14 = uStack26._3_1_;
      if (uStack26._3_1_ != 0x5) {
        return;
      }
      pass1_1030_8344(_u16_1050_5748,uStack26 & 0xffff | uVar11 << 0x10);
      PTR_LOOP_1050_5f18 = param_2;
      iVar6 = 0x26;
      PTR_LOOP_1050_5f16 = puStack14;
      puStack12 = PTR_LOOP_1050_5f18;
    }
    else {
      if (param_4 != 0x202) {
        if (param_4 != 0x203) {
          return;
        }
        if (&uVar9[0x1].field_0x6 != 0x1) {
          return;
        }
        HVar4 = SetCursor16(uVar9[0x1].hfile_0x4);
        (uVar9 + 0x1)->address_offset_field_0x0 = HVar4;
        &uVar9[0x1].field_0x6 = 0x3;
        param_7 = uVar9->field5_0x8;
        SetCapture16((HWND16)param_7);
        return;
      }
      uVar2 = &uVar9[0x1].field14_0x1a;
      uVar16 = (uVar2 >> 0x10);
      iVar6 = uVar2;
      uStack6 = (iVar6 + 0x42);
      uVar11 = (iVar6 + 0x44);
      param_2 = (StructD *)uVar11;
      uStack6._3_1_ = (u8)(uStack6 >> 0x18);
      uVar5 = uStack6._3_1_;
      if (uStack6._3_1_ != 0x5) {
        return;
      }
      pass1_1030_8344(_u16_1050_5748,uStack6 & 0xffff | uVar11 << 0x10);
      PTR_LOOP_1050_5a6a = param_2;
      uStack22 = CONCAT22(PTR_LOOP_1050_5a6a,uVar5);
      iVar6 = 0x27;
      u16_1050_5a68 = uVar5;
    }
    goto LAB_1020_49b7;
  }
  switch(param_4) {
  case 0x133:
    uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8,0x3);
    if (uVar10 == 0x0) {
      return;
    }
    uVar12 = 0xffff;
    uVar16 = 0x0;
    break;
  case 0x134:
    uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8,0x3);
    if (uVar10 == 0x0) {
      return;
    }
    uVar16 = 0x1;
    goto LAB_1020_4ef8;
  case 0x135:
    uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8,0x3);
    if (uVar10 == 0x0) {
      return;
    }
    uVar12 = 0x1;
    uVar16 = 0x0;
    break;
  case 0x136:
    uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8,0x3);
    if (uVar10 == 0x0) {
      return;
    }
    uVar16 = 0xfffb;
    goto LAB_1020_4ef8;
  case 0x137:
    uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8,0x3);
    if (uVar10 == 0x0) {
      return;
    }
    uVar12 = 0xfffb;
    uVar16 = 0x0;
    break;
  case 0x138:
    uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8,0x3);
    if (uVar10 == 0x0) {
      return;
    }
    uVar16 = 0x5;//
LAB_1020_4ef8:
    uVar12 = 0x0;
    break;
  case 0x139:
    uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8,0x3);
    if (uVar10 == 0x0) {
      return;
    }
    uVar12 = 0x5;
    uVar16 = 0x0;
    break;
  default:
    goto switchD_1020_518a_caseD_13a;
  case 0x13c:
    uVar10 = pass1_1020_64d4(&uVar9[0x1].field5_0x8,0x2);
    if (uVar10 != 0x0) {
      iVar6 = 0x1a;
      goto LAB_1020_49b7;
    }
    goto switchD_1020_518a_caseD_13a;
  }
  pass1_1020_2a94(&uVar9->field_0xce,CONCAT22(uVar16,uVar12));
switchD_1020_518a_caseD_13a:
  return;
}
