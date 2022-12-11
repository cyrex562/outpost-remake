
pub unsafe fn pass1_1020_51c6(mut param_1: u32,mut param_2: u16 ,mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut in_DX: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (param_1 + 0xf4);
  uVar4 = param_3;
  if (iVar2 == 0x2) {
    win_ui_op_1020_5e76(param_1 & 0xffff | uVar3 << 0x10,param_2,uVar4);
    return;
  }
  iVar2 += -0x3;
  if (iVar2 != 0) {
    pt_in_rect_op_1020_58ce(in_DX,param_1 & 0xffff | uVar3 << 0x10,param_2,uVar4,(param_3 >> 0x10));
    if (iVar2 == 0) {
      ppcVar1 = ((param_1 + 0x4) + 0x5c);
      (**ppcVar1)();
    }
    return;
  }
  win_ui_op_1020_5de8(param_1 & 0xffff | uVar3 << 0x10,param_2,uVar4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn win_ui_cursor_op_1020_522e(mut param_1: u16 ,param_2: *mut astruct_52,mut param_3: u16 ,mut param_4: u16 )

{
  let mut iVar1: i16;
  let mut ppcVar2: *mut *mut code;
  let mut BVar3: bool;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut iVar4: *mut astruct_52;
  let mut uVar4: *mut astruct_52;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut uVar6: u8;
  let mut uVar7: u8;
  let mut uVar8: u16;

  paVar4 = CONCAT22(in_register_0000000a,param_1);
  uVar4 = (param_2 >> 0x10);
  iVar4 = param_2;
  iVar1 = iVar4.field242_0xf4;
  if (iVar1 == 0x2) {
    SetCursor16(iVar4.field237_0xee);
    iVar4.field237_0xee = 0;
    iVar4.field242_0xf4 = 0x1;
    (iVar4 + 1) = 0;
    ReleaseCapture16();
    return;
  }
  if (iVar1 == 0x3) {
    SetCursor16(iVar4.field237_0xee);
    iVar4.field237_0xee = 0;
    iVar4.field242_0xf4 = 0x1;
    (iVar4 + 1) = 0;
    ReleaseCapture16();
    uVar6 = 0;
    uVar7 = 0;
    uVar8 = 0;
    puVar5 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,0x47,in_stack_0000fea0,in_stack_0000ffc4,
                             in_stack_0000ffca,in_stack_0000ffce);
    pass1_1018_57e6(puVar5,CONCAT22(uVar8,CONCAT11(uVar7,uVar6)),puVar5,(puVar5 >> 0x10));
    return;
  }
  BVar3 = menu_ui_op_1020_5bf2(param_2,param_3,param_4);
  if (BVar3 == 0) {
    ppcVar2 = (*&iVar4.field_0x4 + 0x60);
    (**ppcVar2)();
  }
  return;
}
pub unsafe fn pass1_1020_52de(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;

  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  puVar1 = (iVar6 + 0xf6);
  uVar2 = (iVar6 + 0xf8);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  (iVar6 + 0xf6) = 0;
  if ((iVar6 + 0xfa) != 0) {
    if (param_1 == 0) {
      iVar4 = 0;
      uVar5 = 0;
    }
    else {
      iVar4 = iVar6 + 0xe2;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6((iVar6 + 0xfa),CONCAT22(uVar5,iVar4));
  }
  destroy_win_1008_628e(param_1);
  if ((iVar6 + 0xfa) != 0) {
    pass1_1010_1dda((iVar6 + 0xfa));
  }
  (iVar6 + 0xfa) = 0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn ui_op_1020_536e(mut param_1: u16 ,mut param_2: u32,mut param_3: u32,mut param_4: i16,mut param_5: i16)

{
  let mut piVar1: *mut i16;
  let mut UVar2: u16;
  let mut uVar3: u32;
  let mut ppcVar4: *mut *mut code;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut UVar7: u16;
  let mut uVar8: u16;
  let mut puVar9: *mut u8;
  let mut uVar10: u16;
  let mut in_register_0000000a: u16;
  let mut paVar11: *mut Struct57;
  let mut uVar12: u32;
  let mut paVar13: *mut Struct57;
  let mut paVar14: *mut Struct57;
  let mut puVar15: *mut u32;
  let mut unaff_SI: u16;
  let mut uVar16: u16;
  let mut puVar17: *mut u32;
  let pSVar18: *mut StructA;
  let mut paVar19: *mut astruct_27;
  let mut in_stack_0000fe88: u16;
  let mut in_stack_0000fe8a: u16;
  let mut in_stack_0000ff5c: u16;
  let mut in_stack_0000ff60: u16;
  let mut in_stack_0000ffac: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffb8: u16;
  let mut uVar20: u8;
  let mut uVar21: u8;
  let mut uVar22: u16;
  let mut uVar23: u16;
  let mut iVar24: i16;
  let mut puStack16: *mut u32;

  paVar11 = CONCAT22(in_register_0000000a,param_1);
  uVar8 = param_5 - 0x1;
  uVar16 = (param_2 >> 0x10);
  iVar24 = param_2;
  if (uVar8 == 0) {
    if ((iVar24 + 0xfe) == 0) {
      mem_op_1000_179c(0xfc,paVar11);
      uVar10 = paVar11 | uVar8;
      uVar12 = paVar11 & 0xffff0000 | uVar10;
      if (uVar10 == 0) {
        (iVar24 + 0xfe) = 0;
      }
      else {
        piVar1 = (iVar24 + 0xcc);
        *piVar1 = *piVar1 + 1;
        unk_win_ui_op_1020_67ce
                  (CONCAT13((paVar11 >> 0x8),CONCAT12(paVar11,uVar8)),
                   (iVar24 + 0xcc),param_2,uVar12);
        (iVar24 + 0xfe) = uVar8;
        (iVar24 + 0x100) = uVar12;
      }
      pass1_1008_6978(uVar8,uVar12,param_2,0x0,(iVar24 + 0xfe));
      uVar3 = (iVar24 + 0xfe);
      uVar22 = uVar3;
      uVar23 = (uVar3 >> 0x10);
      uVar3 = (iVar24 + 0xfe);
      uVar16 = (uVar3 >> 0x10);
      puVar15 = uVar3;
  // TODO: goto LAB_1020_53f3;
    }
  }
  else {
    if (param_5 == 0x2) {
      uVar5 = param_4 + 0xc;
      puVar17 = mixed_1010_20ba(paVar11,_u16_1050_0ed0,CONCAT22(unaff_SI,uVar5),in_stack_0000fe8a,
                                in_stack_0000ffae,in_stack_0000ffb4,in_stack_0000ffb8);
      paVar11 = (paVar11 & 0xffff0000 | puVar17 >> 0x10);
      uVar6 = pass1_1018_0afa(puVar17);
      if (uVar6 == 0) {
        piVar1 = (iVar24 + 0xcc);
        *piVar1 = *piVar1 + 1;
        UVar2 = (iVar24 + 0xcc);
        UVar7 = UVar2;
        mem_op_1000_179c(0xfe,paVar11);
        uVar8 = paVar11 | UVar7;
        paVar14 = (paVar11 & 0xffff0000);
        paVar13 = (paVar14 | uVar8);
        if (uVar8 == 0) {
          UVar7 = 0;
        }
        else {
          pass1_1020_289a(CONCAT13((paVar11 >> 0x8),CONCAT12(paVar11,UVar7)),UVar2,param_2)
          ;
          paVar14 = paVar13;
        }
        puVar9 = paVar14;
        puStack16 = CONCAT22(puVar9,UVar7);
        uVar20 = SUB41(paVar14,0x0);
        uVar21 = (paVar14 >> 0x8);
        pass1_1020_294a(puVar9,CONCAT13(uVar21,CONCAT12(uVar20,UVar7)),param_3,uVar5);
        uVar3 = *puStack16;
        ppcVar4 = (uVar3 + 0x8);
        uVar8 = (**ppcVar4)(0x1000,UVar7,puVar9);
        pass1_1008_3e0e(CONCAT13(uVar21,CONCAT12(uVar20,UVar7)));
        pass1_1008_6978(uVar8,paVar14,param_2,UVar2,CONCAT22(puVar9,UVar7));
        ppcVar4 = (uVar3 + 0xc);
        (**ppcVar4)(0x1008,UVar7,uVar20,1);
      }
      else {
        pSVar18 = pass1_1018_0ad4(puVar17);
        paVar14 = (paVar11 & 0xffff0000 | pSVar18 >> 0x10);
        pass1_1008_3e0e(pSVar18);
      }
      pass1_1018_1662(puVar17,0x0,0x0);
      uVar3 = (iVar24 + 0xce);
      BringWindowToTop16((uVar3 + 0x8));
      uVar5 = 0x1;
      iVar24 = 0x4;
      paVar19 =
                mixed_1010_20ba(paVar14,_u16_1050_0ed0,0x1002b,in_stack_0000fe88,in_stack_0000ffac,
                                in_stack_0000ffb2,in_stack_0000ffb6);
      pass1_1010_089e(paVar19,uVar5,iVar24);
      pass1_1010_089e(paVar19,0x1,0x3);
      return;
    }
    uVar8 = param_5 - 0x3;
    if ((uVar8 == 0) && ((iVar24 + 0x102) == 0)) {
      mem_op_1000_179c(0xfc,paVar11);
      uVar10 = paVar11 | uVar8;
      uVar12 = paVar11 & 0xffff0000 | uVar10;
      if (uVar10 == 0) {
        (iVar24 + 0x102) = 0;
      }
      else {
        piVar1 = (iVar24 + 0xcc);
        *piVar1 = *piVar1 + 1;
        pass1_1020_0dc4(CONCAT13((paVar11 >> 0x8),CONCAT12(paVar11,uVar8)),
                        (iVar24 + 0xcc),param_2,uVar12,in_stack_0000ff5c,in_stack_0000ff60);
        (iVar24 + 0x102) = uVar8;
        (iVar24 + 0x104) = uVar12;
      }
      pass1_1008_6978(uVar8,uVar12,param_2,0x0,(iVar24 + 0x102));
      uVar3 = (iVar24 + 0x102);
      uVar22 = uVar3;
      uVar23 = (uVar3 >> 0x10);
      uVar3 = (iVar24 + 0x102);
      uVar16 = (uVar3 >> 0x10);
      puVar15 = uVar3;//
// LAB_1020_53f3:
      ppcVar4 = (*puVar15 + 0xc);
      (**ppcVar4)(0x8,uVar22,uVar23,0x5);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn post_msg_1020_55b0(param_1: *mut StructD,mut param_2: u32,mut param_3: u16 ,mut param_4: i16) -> u16

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut paVar4: *mut Struct57;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut puVar7: *mut u16;
  let mut pcVar8: *mut c_char;
  let mut puVar6: *mut u32;
  let mut LVar9: LRESULT;
  let mut puVar10: *mut u32;
  let mut in_stack_0000fd80: u16;
  let mut in_stack_0000fd82: u16;
  let mut in_stack_0000fea4: u16;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000feaa: u16;
  let mut in_stack_0000feac: u16;
  let mut in_stack_0000feae: u16;
  let mut in_stack_0000feb0: u16;
  let mut paStack288: *mut Struct57;
  let mut puStack284: *mut u32;
  let mut local_114: WPARAM16;
  let mut local_112: [u8;0x2] = [0;0x2];
  let mut iStack272: i16;
  let mut local_10e: i16;
  let mut local_10c: [u8;0x100] = [0;0x100];
  let mut puStack12: *mut u32;
  let mut iStack8: i16;
  let mut puStack6: *mut u32;

  puStack6 = mixed_1010_20ba(param_1,_u16_1050_0ed0,CONCAT22(param_3,0x2),in_stack_0000fd80,
                             in_stack_0000fea4,in_stack_0000feaa,in_stack_0000feae);
  paVar4 = (param_1 & 0xffff0000 | puStack6 >> 0x10);
  iStack8 = (puStack6 + 0x20);
  puStack12 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,CONCAT22(param_3,0x37),in_stack_0000fd80,
                              in_stack_0000fea4,in_stack_0000feaa,in_stack_0000feae);
  uVar6 = (paVar4 >> 0x10);
  load_string_1010_84e0
            (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_10c,&DAT_1050_1050);
  puVar7 = pass1_1008_9436(CONCAT22(0x1050,local_112));
  pcVar8 = pass1_1008_a8f4((puVar7 >> 0x10),puStack12,
                                   CONCAT22(0x1050,&local_114),CONCAT22(0x1050,local_112),
                                   CONCAT22(0x1050,&local_10e));
  uVar2 = pcVar8;
  paVar4 = CONCAT22(uVar6,(pcVar8 >> 0x10) | uVar2);
  uVar5 = (param_2 >> 0x10);
  if ((pcVar8.is_null() == false) && (*pcVar8 != '\0')) {
    uVar6 = 0x1000;
    mem_op_1000_179c(0xb4,paVar4);
    paStack288 = CONCAT22(paVar4,uVar2);
    uVar2 = paVar4 | uVar2;
    paVar4 = (paVar4 & 0xffff0000);
    if (uVar2 == 0) {
      puVar6 = 0;
    }
    else {
      uVar6 = SUB42(&PTR_LOOP_1050_1040,0x0);
      puVar6 = pass1_1040_8478(uVar2,paStack288,0x0,CONCAT13(0x10,CONCAT12(0x50,local_10c)),pcVar8,
                                        (param_2 + 0x8));
      paVar4 = (paVar4 & 0xffff0000 | puVar6 >> 0x10);
      puVar6 = SUB42(puVar6,0x0);
    }
    uVar3 = SUB42(paVar4,0x0);
    puStack284 = CONCAT22(uVar3,puVar6);
    if (iStack272 == 0) {
      ppcVar1 = (*puStack284 + 0x74);
      (**ppcVar1)(uVar6,puVar6,uVar3);
    }
    else {
      ppcVar1 = (*puStack284 + 0x6c);
      (**ppcVar1)(uVar6,puVar6,uVar3,local_112,&DAT_1050_1050);
    }
    if ((iStack8 == 0) || (local_114 == 0)) {
      PostMessage16(0x0,0xfc,0x111,HWND16_1050_0396);
    }
  }
  if ((iStack8 != 0) && (local_114 != 0)) {
    LVar9 = SendMessage16(0x0,local_114,0x111,HWND16_1050_0396);
    (param_2 + 0x112) = 0x1;
    return (LVar9 >> 0x10);
  }
  if (local_10e == 0x6) {
    PostMessage16(0x0,0xb0,0x111,HWND16_1050_0396);
    puVar10 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,CONCAT22(param_4,0x2),in_stack_0000fd82,
                              in_stack_0000fea6,in_stack_0000feac,in_stack_0000feb0);
    paVar4 = (paVar4 & 0xffff0000 | puVar10 >> 0x10);
    param_4 = puVar10;
    (param_4 + 0x20) = 0x1;
  }
  if (local_10e == 0x15) {
    PostMessage16(0x0,0x97,0x111,HWND16_1050_0396);
    puVar10 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,CONCAT22(param_4,0x2),in_stack_0000fd82,
                              in_stack_0000fea6,in_stack_0000feac,in_stack_0000feb0);
    paVar4 = (puVar10 >> 0x10);
    (puVar10 + 0x20) = 0x1;
  }
  return paVar4;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn set_cursor_1020_5764(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut HVar3: HCURSOR16;
  let mut in_EDX: *mut Struct57;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffe6: u16;
  let mut local_e: i16;
  let mut local_c: [u8;0x2] = [0;0x2];
  let mut uStack10: u32;
  let mut puStack6: *mut u32;

  puStack6 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe6,0x2f),in_stack_0000fe8e,
                             in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  uVar5 = (puStack6 >> 0x10);
  uStack10 = (puStack6 + 0x20);
  uVar1 = (puStack6 + 0x22);
  if ((uVar1 | uStack10) != 0) {
    pass1_1030_8308(&local_e,uVar1,_u16_1050_5748,(_u16_1050_5748 >> 0x10),
                    CONCAT22(0x1050,&local_e),CONCAT22(0x1050,local_c),
                    CONCAT13((uVar1 >> 0x8),CONCAT12(uVar1,uStack10)));
    if (param_2 <= local_e) {
      uVar5 = (param_1 >> 0x10);
      iVar4 = param_1;
      if ((iVar4 + 0xf4) != 1) {
        SetCursor16((iVar4 + 0xee));
        (iVar4 + 0xee) = 0;
        (iVar4 + 0xf4) = 0x1;
        (iVar4 + 0x10c) = 0;
        ReleaseCapture16();
      }
      HVar3 = LoadCursor16(0x7f02,0x0);
      HVar3 = SetCursor16(HVar3);
      pass1_1018_017c(puStack6,param_2);
      uVar2 = (iVar4 + 0xf6);
      (uVar2 + 0x10) = 0x1;
      if ((iVar4 + 0xfe) != 0) {
        pass1_1020_68de((iVar4 + 0xfe));
        uVar2 = (iVar4 + 0xfe);
        PostMessage16(0x0,0xeb,0x111,(uVar2 + 0x8));
      }
      SetCursor16(HVar3);
    }
  }
  return;
}
pub unsafe fn pt_in_rect_1020_5856(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,param_4: *mut POINT16)

{
  let mut puVar1: *mut u32;
  let mut BVar2: bool;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut uStack10: u32;

  pass1_1018_2862(*(astruct_654 **)(param_3 + 0xfa));
  if ((param_2 | param_1) != 0) {
    uStack10 = 0;
    loop {
      puVar1 = (param_1 + 0xa);
      if (*puVar1 < uStack10 || *puVar1 == uStack10) break;
      uVar3 = uStack10;
      empty_1008_8fc4();
      if ((extraout_DX | uVar3) != 0) {
        BVar2 = PtInRect16(*param_4,(uVar3 + 0x14));
        if (BVar2 != 0) {
          return;
        }
      }
      uStack10 += 0x1;
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pt_in_rect_op_1020_58ce(mut param_1: u16 ,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,param_5: u8)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut ppcVar3: *mut *mut code;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut BVar6: bool;
  let mut puVar7: *mut u16;
  let mut uVar8: u16;
  let mut in_register_0000000a: u16;
  let mut paVar9: *mut Struct57;
  let mut iVar10: i16;
  let mut iVar11: i16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar14: u32;
  let mut puVar15: *mut u16;
  let mut puVar16: *mut u16;
  let mut puVar17: *mut u32;
  let mut in_stack_0000fe74: u16;
  let mut in_stack_0000ff98: u16;
  let mut in_stack_0000ff9e: u16;
  let mut in_stack_0000ffa2: u16;
  let mut wparam: WPARAM16;
  let mut in_stack_0000ffcc: u16;
  let mut uStack46: u16;
  let mut iStack26: i16;
  let mut local_18: [u16;0x2] = [0;0x2];
  let mut uStack20: u16;
  let mut uStack18: u32;
  pRStack14: *mut RECT16;
  let mut uStack12: u16;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut local_6: u16;
  let mut uStack4: u16;

  paVar9 = CONCAT22(in_register_0000000a,param_1);
  local_6 = param_4;
  uStack4 = param_3;
  uStack8 = param_5 & 0x8;
  uStack10 = param_5 & 0x4;
  uVar12 = (param_2 >> 0x10);
  iVar10 = param_2;
  uVar5 = pass1_1020_64d4((iVar10 + 0xf6),0x2);
  if (uVar5 == 0) {//
// LAB_1020_5942:
    uVar5 = pass1_1020_64d4((iVar10 + 0xf6),0x4);
    if (uVar5 == 0) {//
// LAB_1020_5a16:
      uVar5 = pass1_1020_64d4((iVar10 + 0xf6),1);
      if (uVar5 != 0) {
        uVar14 = pass1_1020_6498((iVar10 + 0xf6),1);
        paVar9 = (paVar9 & 0xffff0000 | uVar14 >> 0x10);
        for (iStack26 = 0; iStack26 < 0x4; iStack26 += 1) {
          paVar9 = (paVar9 & 0xffff0000 | uVar14 >> 0x10);
          BVar6 = PtInRect16(CONCAT22(uStack4,local_6),(iStack26 * 0x8 + uVar14));
          if (BVar6 != 0) {
            local_18[0] = 0;
            uStack20 = 0;
            if (iStack26 == 0) {
              uStack20 = (-(uStack10 == 0) & 0x4) - 0x5;
            }
            else if (iStack26 == 1) {
              uStack20 = (-(uStack10 == 0) & 0xfffc) + 0x5;
            }
            else if (iStack26 == 0x2) {
              local_18[0] = (-(uStack10 == 0) & 0x4) - 0x5;
            }
            else if (iStack26 == 0x3) {
              local_18[0] = (-(uStack10 == 0) & 0xfffc) + 0x5;
            }
            pass1_1020_2a94((iVar10 + 0xce),CONCAT22(local_18[0],uStack20));
            return;
          }
        }
      }
      uVar5 = pass1_1020_64d4((iVar10 + 0xf6),0x3);
      if (uVar5 != 0) {
        puVar7 = &local_6;
        pt_in_rect_1020_5856(puVar7,paVar9,param_2,CONCAT22(0x1050,puVar7));
        uVar8 = paVar9;
        uVar14 = (uVar8 | puVar7);
        if ((uVar8 | puVar7) != 0) {
          uVar5 = puVar7[0x17];
          if (((uStack8 == 0) || (uStack10 == 0)) && (uStack10 == 0)) {
            local_18[0] = 0x1;
          }
          else {
            local_18[0] = 0x2;
          }
          uStack20 = puVar7[0x6];
          uStack18 = CONCAT22(uStack18,puVar7[0x7]);
          if ((uVar5 == 0xb) || (uVar5 == 0x37)) {
            uVar4 = (iVar10 + 0xfa);
            uVar13 = (uVar4 >> 0x10);
            iVar11 = uVar4;
            uVar2 = (iVar11 + 0x20);
            uVar1 = (iVar11 + 0x22);
            uVar14 = paVar9 & 0xffff0000 | uVar1;
            uStack46 = uVar2;
            if ((uVar1 | uStack46) != 0) {
              puVar16 = pass1_1008_3e38(CONCAT22(0x1050,&stack0xffcc));
              paVar9 = (uVar14 & 0xffff0000 | puVar16 >> 0x10);
              pass1_1018_161c(uVar2,CONCAT22(0x1050,&stack0xffcc),uStack18,uStack20);
              puVar17 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,CONCAT22(in_stack_0000ffcc,0x2f),
                                        in_stack_0000fe74,in_stack_0000ff98,in_stack_0000ff9e,in_stack_0000ffa2);
              uVar14 = puVar17 >> 0x10;
              pass1_1010_ecc6(&stack0xffcc,(puVar17 >> 0x10),puVar17,
                              CONCAT22(0x1050,&stack0xffcc),(uStack46 + 0x3c));
            }
          }
          uVar13 = uVar14;
          uVar5 = pass1_1018_25d2((iVar10 + 0xfa),local_18[0],
                                  uStack18 & 0xffff | uStack20 << 0x10);
          if (uVar5 != 0) {
            return;
          }
          uVar5 = pass1_1020_5d56(uVar13,param_2,CONCAT22(uVar8,puVar7));
          if (uVar5 != 0) {
            return;
          }
        }
      }
      return;
    }
    uVar14 = pass1_1020_6498((iVar10 + 0xf6),0x4);
    paVar9 = (paVar9 & 0xffff0000 | uVar14 >> 0x10);
    pRStack14 = uVar14;
    uStack12 = (uVar14 >> 0x10);
    BVar6 = PtInRect16(CONCAT22(uStack4,local_6),pRStack14);
//    if (BVar6 == 0) goto LAB_1020_5a16;
    uStack18 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,CONCAT22(in_stack_0000ffcc,0x2),in_stack_0000fe74,
                               in_stack_0000ff98,in_stack_0000ff9e,in_stack_0000ffa2);
    if ((uStack18 + 0x72) != 0) {
      (iVar10 + 0x116) = 0x1;
      if (param_2 == 0) {
        iVar10 = 0;
        uVar12 = 0;
      }
      else {
        iVar10 += 0xe2;
      }
      ppcVar3 = (*_u16_1050_02a0 + 0x4);
      (**ppcVar3)(0x1010,_u16_1050_02a0,(_u16_1050_02a0 >> 0x10),0x10,iVar10,uVar12);
      puVar15 = pass1_1008_941a(CONCAT22(0x1050,local_18),0x1,0x86);
      puVar7 = local_18;
      win_1008_5c9e(puVar7,(puVar15 >> 0x10),_u16_1050_02a0,CONCAT22(0x1050,puVar7));
      if (puVar7.is_null() == false) {
        return;
      }
      wparam = 0xf6;
  // TODO: goto LAB_1020_5936;
    }
    wparam = 0xf6;
  }
  else {
    uVar14 = pass1_1020_6498((iVar10 + 0xf6),0x2);
    paVar9 = (paVar9 & 0xffff0000 | uVar14 >> 0x10);
    pRStack14 = uVar14;
    uStack12 = (uVar14 >> 0x10);
    BVar6 = PtInRect16(CONCAT22(uStack4,local_6),pRStack14);
//    if (BVar6 == 0) goto LAB_1020_5942;
    wparam = 0x68;
  }
  puVar7 = NULL;//
// LAB_1020_5936:
  PostMessage16(CONCAT22(puVar7,puVar7),wparam,0x111,HWND16_1050_0396);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn menu_ui_op_1020_5bf2(param_1: *mut astruct_52,param_2: INT16,param_3: INT16) -> BOOL16

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut BVar3: bool;
  INT16 *pIVar4;
//   HMENlet mut HVar5: u16;
let mut HVar5: HMENU16;
let mut in_DX: u16;
  let mut iVar5: *mut astruct_52;
  let mut uVar6: u16;
  let mut local_10: i16;
  let mut IStack14: i16;
  let mut iStack12: i16;
  let mut uStack10: u32;
  let mut local_6: i16;
  let mut IStack4: i16;

  local_6 = param_3;
  IStack4 = param_2;
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  uVar2 = pass1_1020_64d4(iVar5.field243_0xf6,0x2);
  if (uVar2 != 0) {
    uStack10 = pass1_1020_6498(iVar5.field243_0xf6,0x2);
    in_DX = (uStack10 >> 0x10);
    BVar3 = PtInRect16(CONCAT22(IStack4,local_6),uStack10);
    if (BVar3 != 0) {
      PostMessage16(0x0,0x131,0x111,HWND16_1050_0396);
      return 0x1;
    }
  }
  uVar2 = pass1_1020_64d4(iVar5.field243_0xf6,0x3);
  if (uVar2 == 0) {
    return 0x0;
  }
  pIVar4 = &local_6;
  pt_in_rect_1020_5856(pIVar4,in_DX,param_1,CONCAT22(0x1050,pIVar4));
  iVar5.field257_0x108 = pIVar4;
  iVar5.field258_0x10a = in_DX;
  if ((in_DX | iVar5.field257_0x108) == 0) {
    return 0x0;
  }
  if (iVar5.field256_0x106 == 0) {
    HVar5 = LoadMenu16(s_TILEMENU_1050_43f0,HINSTANCE16_1050_038c);
    iVar5.field256_0x106 = HVar5;
    if (HVar5 == 0) {
      return 0x1;
    }
    HVar5 = GetSubMenu16(0x0,iVar5.field256_0x106);
    iVar5.field256_0x106 = HVar5;
    if (HVar5 == 0) {
      return 0x1;
    }
  }
  uVar1 = &iVar5.field257_0x108;
  uStack10 = (uVar1 + 0x2e);
  iStack12 = 0;
  if (uStack10 == 0x42) {
    iStack12 = 0xc9;
  }
  else if ((s_VrMode_1050_42ca + 0x8 + uStack10 * 0x2) == 0) {
    iStack12 = 0xc8;
  }
  if (iStack12 != 0) {
    win_1008_5c7c(uStack10,in_DX,_u16_1050_02a0,CONCAT22(iStack12,1));
  }
  local_10 = param_3;
  IStack14 = param_2;
  ClientToScreen16(CONCAT22(0x1050,&local_10),iVar5.field8_0x8);
  TrackPopupMenu16(NULL,iVar5.field8_0x8,0x0,IStack14,local_10,0x0,iVar5.field256_0x106);
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1020_5d56(mut param_1: u16 ,param_2: *mut u32,mut param_3: u32) -> u16

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u16;
  local_12: i16[0x2];
  let mut local_e: i16;
  let mut local_c: i16;
  local_a: i16[0x2];
  let mut iStack6: i16;

  iStack6 = (param_3 + 0x2e);
  uVar2 = param_2;
  uVar3 = (param_2 >> 0x10);
  if (iStack6 == 0x47) {
    pass1_1020_61c4(uVar2,uVar3,CONCAT22(0x1050,&local_c),CONCAT22(0x1050,local_a));
//    if (local_a[0] == 0) goto LAB_1020_5d8b;
    if (local_c <= local_a[0]) {
      return 0x1;
    }
  }
  else {
    if (iStack6 != 0x6a) {
      return 0x0;
    }
    pass1_1020_61c4(uVar2,uVar3,CONCAT22(0x1050,&local_e),CONCAT22(0x1050,local_12));
    if (local_e <= local_12[0]) {//
// LAB_1020_5d8b:
      ppcVar1 = (*param_2 + 0x40);
      (**ppcVar1)();
      return 0x1;
    }
  }
  pass1_1038_af40(uVar2,param_1,_PTR_LOOP_1050_5b7c,(uVar2 + 0x8),0x9);
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn win_ui_op_1020_5de8(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 )

{
  let mut uVar1: u32;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_EDX: *mut Struct57;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut puVar7: *mut u32;
  let mut in_stack_0000fe8c: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffe4: u16;
  let mut uStack18: u16;
  let mut bStack15: u8;
  let mut local_6: u16;
  let mut uStack4: u16;

  ReleaseCapture16();
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  SetCursor16((iVar5 + 0xee));
  (iVar5 + 0xee) = 0;
  (iVar5 + 0xf4) = 0x1;
  local_6 = param_3;
  uStack4 = param_2;
  puVar7 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,CONCAT22(in_stack_0000ffe4,0x47),in_stack_0000fe8c,
                           in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
  uVar3 = (puVar7 >> 0x10);
  puVar2 = &local_6;
  pt_in_rect_1020_5856(puVar2,uVar3,param_1,CONCAT22(0x1050,puVar2));
  uVar4 = uVar3 | puVar2;
  if (uVar4 != 0) {
    uVar1 = (puVar2 + 0x21);
    uVar4 = puVar2[0x22];
    bStack15 = (uVar1 >> 0x18);
    puVar2 = bStack15;
    if (bStack15 == 0x5) {
      uStack18 = uVar1;
      uVar3 = uVar4;
  // TODO: goto LAB_1020_5e62;
    }
  }
  uStack18 = 0;
  uVar3 = 0;//
// LAB_1020_5e62:
  pass1_1018_57e6(puVar7,CONCAT22(uVar3,uStack18),puVar2,uVar4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn win_ui_op_1020_5e76(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut puVar2: *mut u16;
  let mut puVar3: *mut u8;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut in_EDX: u32;
  let mut iVar8: i16;
  let mut puVar9: *mut u32;
  let mut puVar10: *mut u32;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut puVar13: *mut u32;
  let mut uVar14: u8;
  let mut in_AF: u8;
  let mut paVar15: *mut Struct57;
  let mut puVar16: *mut u32;
  let mut in_stack_0000fc00: u16;
  let mut in_stack_0000fd24: u16;
  let mut in_stack_0000fd2a: u16;
  let mut in_stack_0000fd2e: u16;
  let mut pcVar17: *mut c_char;
  let mut uVar18: u16;
  let mut in_stack_0000fd58: u16;
  local_1aa: *mut u8 [0x80];
  let mut local_aa: [u8;0x80] = [0;0x80];
  let mut uStack42: u32;
  let mut uStack38: u32;
  let mut local_22: [u8;0x10] = [0;0x10];
  let mut puStack18: *mut u8;
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut uStack12: u16;
  let mut uStack10: u32;
  let mut local_6: u16;
  let mut uStack4: u16;
  let mut uVar7: u32;

  ReleaseCapture16();
  uVar11 = (param_1 >> 0x10);
  iVar8 = param_1;
  SetCursor16((iVar8 + 0xee));
  (iVar8 + 0xee) = 0;
  (iVar8 + 0xf4) = 0x1;
  local_6 = param_3;
  uStack4 = param_2;
  puVar2 = &local_6;
  pt_in_rect_1020_5856(puVar2,in_EDX,param_1,CONCAT22(0x1050,puVar2));
  uVar5 = in_EDX;
  uStack10 = CONCAT22(uVar5,puVar2);
  paVar15 = (in_EDX & 0xffff0000 | (uVar5 | puVar2));
//  if ((uVar5 | puVar2) == 0) goto LAB_1020_6176;
  uStack12 = puVar2[0x6];
  uStack14 = puVar2[0x7];
  uStack16 = 0;
  puVar3 = pass1_1018_2580(in_AF,(iVar8 + 0xfa),0x0,CONCAT22(uStack12,uStack14),(iVar8 + 0x10c));
//  if (puVar3 == 0x6b2) goto LAB_1020_6176;
  puStack18 = puVar3;
  if (puVar3 == 0x6b8) {
    mem_op_1000_179c(0xb4,paVar15);
    uStack42 = CONCAT22(paVar15,puVar3);
    uVar5 = paVar15 | puVar3;
    uVar7 = paVar15 & 0xffff0000 | uVar5;
    if (uVar5 == 0) {
      iVar4 = 0;
      uVar12 = 0;
    }
    else {
      iVar4 = string_1040_8520(uVar7,
                                     CONCAT13((paVar15 >> 0x8),CONCAT12(paVar15,puVar3)),
                               HWND16_1050_0396,0x20040,0x6ad06b8);
      uVar12 = uVar7;
    }
    uStack38 = CONCAT22(uVar12,iVar4);
    uVar18 = 0xa5;//
// LAB_1020_5f84:
    pass1_1008_941a(CONCAT13(0x10,CONCAT12(0x50,local_22)),0x1,uVar18);
    pcVar17 = local_22;
    uVar12 = (uStack38 >> 0x10);
    puVar9 = uStack38;
  }
  else {
    if (puVar3 == 0x6b4) {
      mem_op_1000_179c(0xb4,paVar15);
      uStack42 = CONCAT22(paVar15,puVar3);
      uVar5 = paVar15 | puVar3;
      uVar7 = paVar15 & 0xffff0000 | uVar5;
      if (uVar5 == 0) {
        iVar4 = 0;
        uVar12 = 0;
      }
      else {
        iVar4 = string_1040_8520(uVar7,
                                       CONCAT13((paVar15 >> 0x8),CONCAT12(paVar15,puVar3)),
                                 HWND16_1050_0396,0x20040,CONCAT22(puStack18,0x57b));
        uVar12 = uVar7;
      }
      uStack38 = CONCAT22(uVar12,iVar4);
      uVar18 = 0xab;
  // TODO: goto LAB_1020_5f84;
    }
    if (puVar3 == 0x6b6) {
      load_string_1010_84e0
                (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_aa,&DAT_1050_1050);
      load_string_1010_84e0
                (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_1aa,
                 &DAT_1050_1050);
      uVar5 = sys_1000_3f9c(CONCAT13(0x10,CONCAT12(0x50,&stack0xfd56)),CONCAT22(0x1050,local_1aa),
                            PTR_LOOP_1050_50cc);
      uVar14 = 0;
      mem_op_1000_179c(0xb4,paVar15);
      uVar6 = paVar15;
      uStack42 = CONCAT22(uVar6,uVar5);
      if ((uVar6 | uVar5) == 0) {
        puVar9 = NULL;
        paVar15 = NULL;
      }
      else {
        uVar14 = 0x40;
        paVar15 = pass1_1040_8478(uVar6 | uVar5,CONCAT22(uVar6,uVar5),0x40,
                                  CONCAT13(0x10,CONCAT12(0x50,local_aa)),CONCAT22(0x1050,&stack0xfd56),
                                  HWND16_1050_0396);
        puVar9 = paVar15;
      }
      uStack38 = paVar15 & 0xffff0000 | ZEXT24(puVar9);
      puVar10 = puVar9;
      puVar13 = ((paVar15 & 0xffff0000) >> 0x10);//
// LAB_1020_6027:
      ppcVar1 = (*puVar10 + 0x74);
      (**ppcVar1)(uVar14,puVar9);
  // TODO: goto LAB_1020_6176;
    }
    if (puVar3 < 0x6a7) {
      if (((iVar8 + 0x10c) == 0x78) || ((iVar8 + 0x10c) == 0x74)) {
        puVar16 = mixed_1010_20ba(paVar15,_u16_1050_0ed0,CONCAT22(in_stack_0000fd58,0x5),in_stack_0000fc00
                                  ,in_stack_0000fd24,in_stack_0000fd2a,in_stack_0000fd2e);
        paVar15 = (paVar15 & 0xffff0000 | puVar16 >> 0x10);
        in_stack_0000fd58 = (puVar16 >> 0x10);
        if ((puVar16 + 0xa) == 0) {
          return;
        }
      }
      if ((((((iVar8 + 0x10c) == 0x6c) || ((iVar8 + 0x10c) == 0x6d)) ||
           ((iVar8 + 0x10c) == 0x31)) || ((iVar8 + 0x10c) == 0x32)) &&
         (puVar16 = mixed_1010_20ba(paVar15,_u16_1050_0ed0,CONCAT22(in_stack_0000fd58,0x3a),
                                    in_stack_0000fc00,in_stack_0000fd24,in_stack_0000fd2a,in_stack_0000fd2e),
         (puVar16 + 0xa) == 0)) {
        return;
      }
      pass1_1020_68de((iVar8 + 0xfe));
  // TODO: goto LAB_1020_6176;
    }
    if (0x6b1 < puVar3) {
      uVar14 = 0;
      mem_op_1000_179c(0xb4,paVar15);
      uStack42 = CONCAT22(paVar15,puVar3);
      uVar5 = paVar15 | puVar3;
      uVar7 = paVar15 & 0xffff0000 | uVar5;
      if (uVar5 == 0) {
        puVar9 = NULL;
        puVar10 = NULL;
        puVar13 = puVar10;
      }
      else {
        uVar14 = 0x40;
        puVar9 =
                 string_1040_8520(uVar7,
                                        CONCAT13((paVar15 >> 0x8),CONCAT12(paVar15,puVar3)),
                                  HWND16_1050_0396,0x20040,CONCAT22(puStack18,0x57b));
        puVar10 = uVar7;
        puVar13 = puVar10;
      }
  // TODO: goto LAB_1020_6027;
    }
    mem_op_1000_179c(0xb4,paVar15);
    uStack42 = CONCAT22(paVar15,puVar3);
    uVar5 = paVar15 | puVar3;
    uVar7 = paVar15 & 0xffff0000 | uVar5;
    if (uVar5 == 0) {
      uVar12 = 0;
    }
    else {
      string_1040_8520(uVar7,CONCAT13((paVar15 >> 0x8),CONCAT12(paVar15,puVar3)),
                       HWND16_1050_0396,0x20040,CONCAT22(puStack18,0x57b));
      uVar12 = uVar7;
    }
    local_1aa[0] = puStack18 + -0x608;
    pass1_1008_941a(CONCAT13(0x10,CONCAT12(0x50,local_aa)),0x1,local_1aa[0]);
    pcVar17 = local_aa;
    puVar9 = &DAT_1050_1050;
  }
  ppcVar1 = (*puVar9 + 0x6c);
  (**ppcVar1)(0x1008,puVar9,uVar12,pcVar17);//
// LAB_1020_6176:
  (iVar8 + 0x10c) = 0;
  return;
}
