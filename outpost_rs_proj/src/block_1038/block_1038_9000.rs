
StructD * pass1_1038_90a2(StructD *param_1,param_2: u8)

{
  pass1_1038_8cf6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_9144(mut param_1: u16 ,param_2: *mut u16,mut param_3: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut iVar6: *mut Struct57;
  let mut iVar7: i16;
  let mut unaff_SI: u16;
  let mut uVar8: *mut Struct57;
  let mut uVar9: u16;
  let mut puVar10: *mut u32;
  let mut in_stack_0000fe9c: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffca: u16;
  let mut piStack8: *mut i16;
  let mut paVar5: *mut Struct57;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  struct_1040_b082((astruct_57 *)param_2,CONCAT22(param_3,0xfaa));
  uVar8 = (astruct_57 *)(param_2 >> 0x10);
  iVar6 = (astruct_57 *)param_2;
  iVar6[0x1].field3_0x6 = 0x0;
  iVar6[0x1].field4_0x8 = 0x0;
  &iVar6[0x1].field5_0xa = 0x0;
  *param_2 = 0x99a2;
  iVar6->field1_0x2 = &u16_1050_1038;
  iVar6->field105_0x8a = 0x27;
  puVar10 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x28),in_stack_0000fe9c,
                            in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
  paVar4 = (astruct_57 *)(paVar4 & 0xffff0000 | puVar10 >> 0x10);
  uVar2 = puVar10;
  iVar6[0x1].field5_0xa = uVar2;
  iVar6[0x1].field6_0xc = (puVar10 >> 0x10);
  mem_op_1000_179c(0x18,paVar4);
  uVar3 = paVar4 | uVar2;
  paVar5 = (astruct_57 *)(paVar4 & 0xffff0000 | uVar3);
  if (uVar3 == 0x0) {
    &iVar6[0x1].field1_0x2 = 0x0;
  }
  else {
    struct_1040_a598((astruct_259 *)CONCAT22(paVar4,uVar2));
    iVar6[0x1].field1_0x2 = uVar2;
    iVar6[0x1].field2_0x4 = paVar5;
  }
  *(u16*)&iVar6[0x1].field1_0x2 = 0x11;
  iVar7 = **(i16 **)&iVar6[0x1].field1_0x2;
  uVar2 = iVar7 * 0xa + 0x2;
  mem_op_1000_179c(uVar2,paVar5);
  uVar3 = paVar5;
  piStack8 = CONCAT22(uVar3,uVar2);
  if ((uVar3 | uVar2) == 0x0) {
    uVar1 = &iVar6[0x1].field1_0x2;
    (uVar1 + 0x2) = 0x0;
  }
  else {
    *piStack8 = iVar7;
    pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,iVar7,0xa,uVar2 + 0x2,uVar3);
    uVar1 = &iVar6[0x1].field1_0x2;
    uVar9 = (uVar1 >> 0x10);
    iVar7 = uVar1;
    (iVar7 + 0x2) = uVar2 + 0x2;
    (iVar7 + 0x4) = uVar3;
  }
  uVar1 = &iVar6[0x1].field1_0x2;
  (uVar1 + 0xa) = 0x18;
  uVar1 = &iVar6[0x1].field1_0x2;
  (uVar1 + 0x12) = iVar6->field5_0xa;
  return;
}
pub fn pass1_1038_927c(u32 *param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)(*param_1 + 0x74);
  (**ppcVar1)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_dlg_op_1038_9294(mut param_1: u16 ,StructB *param_2)

{
  let mut UVar1: u16;
  let mut uVar1: u16;
  StructB *struct_b_1_hi;
  let mut local_6: bool;
  let mut local_4: bool;

  unk_win_ui_op_1040_b230(param_1,param_2);
  struct_b_1_hi = (StructB *)(param_2 >> 0x10);
  UVar1 = GetDlgItemInt16(0x1,&local_4,(INT16)&DAT_1050_1050,0xfa9);
  (param_2 + 0x94) = UVar1;
  uVar1 = GetDlgItemInt16(0x1,&local_6,(INT16)&DAT_1050_1050,0xfa8);
  (param_2 + 0x96) = uVar1;
  win_ui_dlg_op_1038_98b4((StructB *)(param_2 & 0xffff | ZEXT24(struct_b_1_hi) << 0x10));
  win_1008_5c7c(uVar1,param_1,_u16_1050_02a0,0x950001);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn draw_op_1038_92f6(u8 *param_1,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut iVar4: i16;
  StructD *pSVar5;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  let mut paVar7: *mut Struct57;
  let mut paVar9: *mut Struct57;
  let mut unaff_SI: u16;
  let mut uVar10: u16;
  let mut in_stack_0000fe88: u16;
  let mut in_stack_0000ffac: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb6: u16;
  HFILE16 *hfile_param;
  BOOL16 local_1a [0x2];
  let mut UStack22: u16;
  StructD *pSStack20;
  StructD *pSStack16;
  let mut iStack12: i16;
  StructD *pSStack10;
  astruct_20 *paStack6;
  let mut paVar8: *mut Struct57;

  paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  if (param_5 == 0xeb) {
    paStack6 = (astruct_20 *)
               mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x3),in_stack_0000fe88,
                               in_stack_0000ffac,in_stack_0000ffb2,in_stack_0000ffb6);
    paVar7 = (astruct_57 *)(paVar7 & 0xffff0000 | paStack6 >> 0x10);
    pSVar5 = *(StructD **)(param_2 + 0x90);
    if (pSVar5 != NULL) {
      pSStack10 = pSVar5;
      mem_op_1000_179c(0x18,paVar7);
      uVar3 = pSVar5;
      pSStack16 = (StructD *)(pSVar5 & 0xffff | (long)paVar7 << 0x10);
      uVar6 = paVar7 | uVar3;
      paVar9 = (astruct_57 *)(paVar7 & 0xffff0000);
      paVar8 = (astruct_57 *)(paVar9 | uVar6);
      if (uVar6 == 0x0) {
        uVar3 = 0x0;
      }
      else {
        struct_1040_a598((astruct_259 *)(pSVar5 & 0xffff | (long)paVar7 << 0x10));
        paVar9 = paVar8;
      }
      (param_2 + 0x90) = uVar3;
      (param_2 + 0x92) = paVar9;
      (param_2 + 0x90) = 0x11;
      iStack12 = **(i16 **)(param_2 + 0x90);
      uVar3 = iStack12 * 0xa + 0x2;
      mem_op_1000_179c(uVar3,paVar9);
      uVar6 = paVar9;
      pSStack16 = (StructD *)CONCAT22(uVar6,uVar3);
      if ((uVar6 | uVar3) == 0x0) {
        uVar2 = (param_2 + 0x90);
        (uVar2 + 0x2) = 0x0;
      }
      else {
        pSStack16 = iStack12;
        pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,iStack12,0xa,uVar3 + 0x2,uVar6);
        uVar2 = (param_2 + 0x90);
        uVar10 = (uVar2 >> 0x10);
        iVar4 = uVar2;
        (iVar4 + 0x2) = uVar3 + 0x2;
        (iVar4 + 0x4) = uVar6;
      }
      uVar10 = (pSStack10 >> 0x10);
      uVar2 = (param_2 + 0x90);
      (uVar2 + 0x6) = (pSStack10 + 0x6);
      uVar2 = (param_2 + 0x90);
      (uVar2 + 0xa) = (pSStack10 + 0xa);
      uVar2 = (param_2 + 0x90);
      (uVar2 + 0x12) = (param_2 + 0xa);
      uVar10 = 0x1010;
      pass1_1010_a50c(paStack6,(u8 **)&u32_1050_5b42,*(StructD **)(param_2 + 0x90));
      pSStack20 = pSStack10;
      pSStack16 = pSStack10;
      if (pSStack10 != NULL) {
        pass1_1040_a5d0(pSStack10);
        uVar10 = 0x1000;
        fn_ptr_1000_17ce(pSStack10);
      }
      ppcVar1 = (code **)(CONCAT22(param_3,param_2) + 0x70);
      (**ppcVar1)(uVar10,param_2,param_3);
    }
  }
  else {
    if (param_5 != 0xf9) {
      pass1_1040_b54a(param_1,(astruct_903 *)CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,param_2)),param_4,
                      param_5);
      return;
    }
    iVar4 = pass1_1038_993a(param_2,param_3,param_4);
    if (-0x1 < iVar4) {
      hfile_param = *(HFILE16 **)(param_2 + 0x6);
      UStack22 = GetDlgItemInt16(0x1,local_1a,(INT16)&DAT_1050_1050,*(HWND16 *)(iVar4 * 0xe + 0x5a72));
      if (local_1a[0] != 0x0) {
        FUN_1010_2a32(*(u8 **)(param_2 + 0x98),CONCAT22((iVar4 * 0xe + 0x5a72),UStack22),hfile_param,
                      unaff_SI);
      }
    }
  }
  return;
}



BOOL16 send_dlg_item_int_1038_94da(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16)

{
  let mut pUVar1: *mut u16;
  let mut iVar2: i16;
  let mut hwnd: HWND16;
  i32 lVar3;
  let mut local_c: bool;
  let mut uStack10: u16;
  let mut iStack8: i16;
  let mut UStack6: u16;
  let mut iStack4: i16;

  iStack4 = 0x1;
  iStack8 = pass1_1038_993a(param_1,param_2,param_3);
  if ((-0x1 < iStack8) &&
     (UStack6 = GetDlgItemInt16(0x1,&local_c,(INT16)&DAT_1050_1050,*(HWND16 *)(iStack8 * 0xe + 0x5a72)), local_c != 0x0)
     ) {
    if (param_5 == 0x0) {
      UStack6 += 0x1;
    }
    else {
      iStack4 = -0x1;
      UStack6 -= 0x1;
    }
    uStack10 = (UStack6 <= (iStack8 * 0xe + 0x5a7a));
    pUVar1 = (u16 *)(iStack8 * 0xe + 0x5a78);
    if (*pUVar1 != UStack6 && UStack6 <= *pUVar1) {
      uStack10 = 0x0;
    }
    iVar2 = iStack8 * 0xe;
    hwnd = GetDlgItem16(*(INT16 *)(iVar2 + 0x5a72),*(HWND16 *)(param_1 + 0x6));
    SetFocus16(hwnd);
    if ((uStack10 != 0x0) &&
       (lVar3 = unk_win_ui_op_1038_9820((StructB *)CONCAT22(param_2,param_1),0x1,iStack4,iStack8), lVar3 != 0x0)) {
      SetDlgItemInt16(0x1,UStack6,*(INT16 *)(iVar2 + 0x5a72),*(HWND16 *)(param_1 + 0x6));
      SetDlgItemInt16(0x1,(param_1 + 0x94),0xfa9,*(HWND16 *)(param_1 + 0x6));
      SetDlgItemInt16(0x1,(param_1 + 0x96),0xfa8,*(HWND16 *)(param_1 + 0x6));
    }
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_msg_op_1038_95fc(mut param_1: u16 ,mut param_2: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut UVar3: u16;
  let mut UVar4: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut paVar7: *mut Struct57;
  let mut iVar9: i16;
  let mut unaff_SI: u16;
  let mut uVar10: u16;
  let mut in_stack_0000fe80: u16;
  let mut in_stack_0000ffa4: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffae: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut puStack30: *mut u16;
  let mut puStack24: *mut u16;
  let mut iStack20: i16;
  let mut local_10: bool;
  let mut puStack14: *mut u32;
  let mut puStack10: *mut u32;
  let mut puStack6: *mut u32;
  let mut paVar8: *mut Struct57;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  puStack6 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x8),in_stack_0000fe80,
                             in_stack_0000ffa4,in_stack_0000ffaa,in_stack_0000ffae);
  paVar6 = (astruct_57 *)(paVar6 & 0xffff0000 | puStack6 >> 0x10);
  puStack10 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x9),in_stack_0000fe80,
                              in_stack_0000ffa4,in_stack_0000ffaa,in_stack_0000ffae);
  paVar7 = (astruct_57 *)(paVar6 & 0xffff0000 | puStack10 >> 0x10);
  uVar2 = puStack10;
  mem_op_1000_179c(0xc,paVar7);
  uVar5 = paVar7 | uVar2;
  paVar6 = (astruct_57 *)(paVar7 & 0xffff0000);
  paVar8 = (astruct_57 *)(paVar6 | uVar5);
  if (uVar5 == 0x0) {
    uVar2 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar7,uVar2));
    paVar6 = paVar8;
  }
  puStack14 = CONCAT22(paVar6,uVar2);
  for (iStack20 = 0x0; iStack20 < 0xf; iStack20 += 0x1) {
    uVar13 = (param_2 + 0x6);
    UVar3 = GetDlgItemInt16(0x1,&local_10,(INT16)&DAT_1050_1050,*(HWND16 *)(iStack20 * 0xe + 0x5a72));
    if (UVar3 != 0x0) {
      if ((iStack20 * 0xe + 0x5a7c) < 0x83) {
        UVar4 = UVar3;
        mem_op_1000_179c(0x8,paVar6);
        uVar2 = paVar6;
        puStack24 = (u16 *)CONCAT22(uVar2,UVar4);
        paVar6 = (astruct_57 *)(paVar6 & 0xffff0000 | (uVar2 | UVar4));
        if ((uVar2 | UVar4) == 0x0) {
          puStack30 = NULL;
        }
        else {
          *puStack24 = 0x389a;
          (UVar4 + 0x2) = 0x1008;
          *puStack24 = 0xa1c4;
          (UVar4 + 0x2) = 0x1010;
          puStack30 = puStack24;
        }
        uVar10 = (puStack30 >> 0x10);
        iVar9 = puStack30;
        (iVar9 + 0x6) = UVar3;
        (iVar9 + 0x4) = (iStack20 * 0xe + 0x5a7c);
        ppcVar1 = (code **)(*puStack14 + 0x4);
        (**ppcVar1)(0x1000,puStack14,(puStack14 >> 0x10),iVar9,uVar10,uVar13);
      }
      else {
        if ((iStack20 * 0xe + 0x5a7c) == 0x89) {
          uVar12 = (iStack20 * 0xe + 0x5a7c);
          uVar11 = UVar3;
        }
        else {
          uVar12 = (iStack20 * 0xe + 0x5a7c);
          uVar11 = 0x0;
        }
        pass1_1010_6566(puStack10,uVar11,UVar3,uVar12);
      }
    }
  }
  (puStack6 + 0xa) = puStack14;
  PostMessage16(0x0,0xed,0x111,HWND16_1050_0396);
  return;
}
pub fn win_ui_op_1038_977a(param_1: *mut astruct_57,mut param_2: i16,mut param_3: u16 ,mut param_4: i16)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar6: u16;
  u8 local_10 [0x4];
  let mut puStack12: *mut u32;
  let mut iStack8: i16;
  let mut uStack6: u16;
  let mut local_4: bool;
  let mut uVar5: u32;

  iStack8 = 0x0;
  uVar6 = (param_2 + 0x6);
  uVar2 = GetDlgItemInt16(0x1,&local_4,(INT16)&DAT_1050_1050,0xfa8);
  uStack6 = uVar2;
  if (uVar2 != 0x0) {
    mem_op_1000_179c(0xb4,param_1);
    uVar3 = param_1 | uVar2;
    uVar5 = param_1 & 0xffff0000 | uVar3;
    if (uVar3 == 0x0) {
      iVar2 = 0x0;
      uVar4 = 0x0;
    }
    else {
      iVar2 = string_1040_8520(uVar5,(astruct_57 *)CONCAT22(param_1,uVar2),(param_2 + 0x6),0x20041,
                               0x5da05db);
      uVar4 = uVar5;
    }
    puStack12 = CONCAT22(uVar4,iVar2);
    pass1_1008_941a((u16 *)CONCAT22(0x1050,local_10),0x1,0xc3);
    ppcVar1 = (code **)(*puStack12 + 0x6c);
    iStack8 = (**ppcVar1)(0x1008,puStack12,(puStack12 >> 0x10),local_10,&DAT_1050_1050,uVar6,uVar2
                         );
  }
  if ((iStack8 == 0x1) || (uStack6 == 0x0)) {
    destroy_window_1040_b726(CONCAT22(param_3,param_2),param_4);
  }
  return;
}



i32 unk_win_ui_op_1038_9820(StructB *param_1,mut param_2: i16,mut param_3: i16,mut param_4: i16)

{
  let mut puVar1: *mut u16;
  u32 **ppuVar2;
  i32 lVar3;
  let mut UVar4: u16;
  let mut iVar5: i16;
  let mut iVar6: i16;
  let mut uVar8: u16;
  StructB *iVar7;
  StructB *uVar7;
  let mut local_6: bool;
  let mut local_4: bool;

  uVar7 = (StructB *)(param_1 >> 0x10);
  iVar7 = (StructB *)param_1;
  UVar4 = GetDlgItemInt16(0x1,&local_4,(INT16)&DAT_1050_1050,*(HWND16 *)(param_4 * 0xe + 0x5a74));
  iVar5 = UVar4 * param_2 * param_3;
  UVar4 = GetDlgItemInt16(0x1,&local_6,(INT16)&DAT_1050_1050,*(HWND16 *)(param_4 * 0xe + 0x5a76));
  lVar3 = (long)(UVar4 * param_2) * (long)param_3;
  uVar8 = (lVar3 >> 0x10);
  iVar6 = lVar3;
  if (((iVar5 - iVar7[0x7].max_count_field_0x10) < 0x1) && (-0x1 < iVar7[0x7].field5_0xa - iVar6)) {
    puVar1 = &iVar7[0x7].max_count_field_0x10;
    *puVar1 = *puVar1 - iVar5;
    ppuVar2 = &iVar7[0x7].field5_0xa;
    *ppuVar2 = (*ppuVar2 - iVar6);
    return CONCAT22(uVar8,0x1);
  }
  return uVar8 << 0x10;
}
pub fn win_ui_dlg_op_1038_98b4(StructB *param_1)

{
  let mut UVar1: u16;
  StructB *iVar3;
  StructB *uVar2;
  LPVOID pvVar2;
  let mut iVar4: i16;
  let mut iStack8: i16;
  let mut local_4: bool;

  local_4 = 0x0;
  for (iStack8 = 0x0; iVar3 = (StructB *)param_1, uVar2 = (StructB *)(param_1 >> 0x10), iStack8 < 0xf;
      iStack8 += 0x1) {
    iVar4 = 0x1;
    pvVar2 = iVar3->lpvoid_field_0x8;
    UVar1 = GetDlgItemInt16(0x1,&local_4,(INT16)&DAT_1050_1050,*(HWND16 *)(iStack8 * 0xe + 0x5a72));
    unk_win_ui_op_1038_9820(param_1,UVar1,pvVar2,iVar4);
  }
  SetDlgItemInt16(0x1,iVar3[0x7].max_count_field_0x10,0xfa9,(HWND16)iVar3->lpvoid_field_0x8);
  SetDlgItemInt16(0x1,iVar3[0x7].field5_0xa,0xfa8,(HWND16)iVar3->lpvoid_field_0x8);
  return;
}



i16 pass1_1038_993a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16)

{
  let mut iStack6: i16;

  iStack6 = 0x0;
  while( true ) {
    if (0xe < iStack6) {
      return -0x1;
    }
    if ((iStack6 * 0xe + 0x5a70) == param_3) break;
    iStack6 += 0x1;
  }
  return iStack6;
}



StructD * pass1_1038_997c(StructD *param_1,param_2: u8)

{
  let mut in_stack_0000ffda: u16;

  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1038_9a1e(param_1: *mut astruct_57,param_2: *mut astruct_57,mut param_3: u16 ,mut param_4: u32)

{
  pass1_1040_b040((astruct_57 *)CONCAT22(param_2,param_1),CONCAT22(param_4,param_3),(param_4 >> 0x10));
  CONCAT22(param_2,param_1) = 0x9af6;
  param_1->field1_0x2 = &u16_1050_1038;
  return (u16 *)CONCAT22(param_2,param_1);
}
pub fn pass1_1038_9a48(StructD *param_1)

{
  let mut in_stack_0000ffde: u16;

  param_1->address_offset_field_0x0 = 0x9af6;
  (param_1 + 0x2) = &u16_1050_1038;
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}
pub fn enable_win_1038_9a66(u8 *param_1,astruct_903 *pstruct903_param_2,u16 in_b_enable_3,mut param_4: u32)

{
  let mut enable: bool;
  let mut hwnd: HWND16;

  if (param_4 == 0xf8) {
    hwnd = GetDlgItem16(0x17d9,*(HWND16 *)(pstruct903_param_2 + 0x6));
    enable = 0x1;
  }
  else {
    if (param_4 != 0x17d9) {
      pass1_1040_b54a(param_1,pstruct903_param_2,in_b_enable_3,param_4);
      return;
    }
    enable = 0x0;
    SetWindowPos16(0x6,0x1a0,0x12c,0x0,0x0,0x0,*(HWND16 *)(pstruct903_param_2 + 0x6));
    hwnd = 0x0;
  }
  EnableWindow16(enable,hwnd);
  return;
}



StructD * pass1_1038_9ad0(StructD *param_1,param_2: u8)

{
  pass1_1038_9a48(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



pub fn pass1_1038_9b72(param_1: *mut astruct_57,param_2: *mut astruct_57,mut param_3: u16 ,mut param_4: u32) -> u32

{
  let mut iStack4: i16;

  pass1_1040_b040((astruct_57 *)CONCAT22(param_2,param_1),CONCAT22(param_4,param_3),(param_4 >> 0x10));
  param_1[0x2].field6_0xc = 0x0;
  CONCAT22(param_2,param_1) = 0x9efa;
  param_1->field1_0x2 = &u16_1050_1038;
  iStack4 = 0x0;
  do {
    (&param_1[0x1].field3_0x6)[iStack4] = 0x0;
    iStack4 += 0x1;
  } while (iStack4 < 0x4a);
  return CONCAT22(param_2,param_1);
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_win_ui_op_1038_9bc8(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,StructB *param_4)

{
  let mut IVar2: i16;
  let mut iVar2: i16;
  HDC16 hdc;
  INT16 IVar1;
  let mut HVar2: HWND16;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  StructB *struct_b_7;
  let mut uVar4: u16;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe60: u16;
  let mut in_stack_0000fe70: u16;
  let mut in_stack_0000ff84: u16;
  let mut in_stack_0000ff8a: u16;
  let mut in_stack_0000ff8e: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ff9e: u16;
  let mut piVar6: *mut i16;
  let mut uVar7: u16;
  let mut piVar8: *mut i16;
  let mut uVar9: u16;
  let mut iStack36: i16;
  u8 local_16 [0x2];
  let mut iStack20: i16;
  let mut iStack16: i16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut local_6: i16;
  let mut local_4: i16;
  astruct_778 *iVar3;
  let mut piVar1: *mut i16;
  let mut in_stack_0000ffc6: u32;
  let mut uVar16: u16;
  code **fn_ptr_1;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(param_4);
  if (PTR_LOOP_1050_5ef8 == (&u32_1050_0004 + 0x1)) {
    PTR_LOOP_1050_5ef8 = NULL;
  }
  piVar8 = &local_4;
  uVar9 = SUB42(&DAT_1050_1050,0x0);
  piVar6 = &local_6;
  uVar7 = SUB42(&DAT_1050_1050,0x0);
  puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(piVar6,0x48),in_stack_0000fe60,in_stack_0000ff84
                           ,in_stack_0000ff8a,in_stack_0000ff8e);
  uVar4 = (paVar3 >> 0x10);
  uStack10 = puVar5;
  uStack10 = (puVar5 >> 0x10);
  pass1_1008_3e94((u16 *)(puVar5 & 0xffff0000 | (uStack10 + 0xe)),(u16 *)CONCAT22(uVar7,piVar6),
                  CONCAT22(uVar9,piVar8));
  IVar2 = GetSystemMetrics16(SM_CYCAPTION);
  paVar3 = (astruct_57 *)CONCAT22(uVar4,(((long)IVar2 * (long)PTR_LOOP_1050_5ef8) >> 0x10));
  iVar2 = ((long)IVar2 * (long)PTR_LOOP_1050_5ef8) + 0xa;
  PTR_LOOP_1050_5ef8 = PTR_LOOP_1050_5ef8 + 0x1;
  uStack14 = CONCAT22(iVar2 + local_4,iVar2 + local_6);
  uVar4 = (param_4 >> 0x10);
  struct_b_7 = (StructB *)param_4;
  GetWindowRect16((RECT16 *)CONCAT22(0x1050,local_16),(HWND16)struct_b_7->lpvoid_field_0x8);
  hdc = GetDC16(0x0);
  IVar1 = GetDeviceCaps16(VERTRES,hdc);
  ReleaseDC16(hdc,0x0);
  if (IVar1 < iStack16) {
    uStack14 = uStack14 & 0xffff0000 | ((iStack20 - (iStack16 - IVar1)) + 0x1);
  }
  SetWindowPos16(0x1,0x0,0x0,(INT16)uStack14,(INT16)(uStack14 >> 0x10),0x0,(HWND16)struct_b_7->lpvoid_field_0x8);
  _param_3 = (u8 **)CONCAT22(param_2,0x3);
  uVar9 = 0x1010;
  puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,_param_3,in_stack_0000fe70,in_stack_0000ff94,in_stack_0000ff9a,
                           in_stack_0000ff9e);
  uVar7 = (puVar5 >> 0x10);
  iStack36 = 0x0;
  while (iVar3 = (astruct_778 *)(iStack36 * 0x2), (&iVar3[0x52].field_0x0 + puVar5) != 0x0) {
    _param_3 = (u8 **)(_param_3 & 0xffff0000);
    uVar9 = SUB42(s_tile2_bmp_1050_1538,0x0);
    HVar2 = GetDlgItem16(*(INT16 *)(&iVar3[0x52].field_0x0 + puVar5),(HWND16)struct_b_7->lpvoid_field_0x8);
    *(HWND16 *)(&iVar3[0x4a].field_0x0 + &struct_b_7->field0_0x0) = HVar2;
    iStack36 += 0x1;
    piVar1 = &struct_b_7[0xe].field8_0x10;
    *piVar1 = *piVar1 + 0x1;
  }
  fn_ptr_1 = (code **)(param_4 + 0x6c);
  (**fn_ptr_1)(uVar9,param_4,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn enable_window_1038_9cec(u8 *param_1,param_2: *mut astruct_903,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut unaff_SI: u16;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe90: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffbe: u16;
  let mut enable: bool;
  let mut HVar6: HWND16;
  let mut iStack12: i16;
  astruct_905 *iVar2;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  if (param_5 == 0xeb) {
    pass1_1040_b54a(param_1,param_2,param_3,CONCAT22(0xeb,param_4));
    puVar5 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x3),in_stack_0000fe90,
                             in_stack_0000ffb4,in_stack_0000ffba,in_stack_0000ffbe);
    iVar3 = puVar5 + 0xa4;
    uVar2 = (puVar5 >> 0x10);
    iStack12 = 0x0;
    while (iVar2 = (astruct_905 *)(iStack12 * 0x2), (iVar2 + iVar3) != 0x0) {
      HVar6 = GetDlgItem16(*(INT16 *)(iVar2 + iVar3),*(HWND16 *)(param_2 + 0x6));
      *(HWND16 *)(iVar2 + param_2 + 0x94) = HVar6;
      iStack12 += 0x1;
      piVar1 = (param_2 + 0x128);
      *piVar1 = *piVar1 + 0x1;
    }
  }
  else {
    if (param_5 == 0xf8) {
      HVar6 = GetDlgItem16(0x17d8,*(HWND16 *)(param_2 + 0x6));
      enable = 0x1;
    }
    else {
      if (param_5 != 0x17d8) {
        pass1_1040_b54a(param_1,param_2,param_3,CONCAT22(param_5,param_4));
        return;
      }
      SetWindowPos16(0x6,0xed,0x237,0x0,0x0,0x0,*(HWND16 *)(param_2 + 0x6));
      enable = (BOOL16)s_tile2_bmp_1050_1538;
      GetDlgItem16(0x17d8,*(HWND16 *)(param_2 + 0x6));
      HVar6 = 0x0;
    }
    EnableWindow16(enable,HVar6);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn draw_op_1038_9dcc(astruct_10 *in_struct_1,mut param_2: i16,mut param_3: u16 ,u16 in_hdc_param_4,mut param_5: u16 )

{
  let mut bVar1: bool;
  HBRUSH16 local_brush_handle;
  astruct_10 *struct10_5;
  astruct_10 *struct10_5_hi;
  let mut uVar3: u32;
  let mut uStack14: u16;
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  astruct_749 *iVar2;

  struct10_5_hi = (astruct_10 *)(in_struct_1 >> 0x10);
  struct10_5 = (astruct_10 *)in_struct_1;
  if (struct10_5->brush_handle_field_0x8e == 0x0) {
    local_brush_handle = CreateSolidBrush16(WHITE_BRUSH);
    struct10_5->brush_handle_field_0x8e = local_brush_handle;
  }
  if (_u16_1050_5b64 == 0x0) {
    uVar3 = pass1_1008_4d72((_PTR_LOOP_1050_4230 + 0xe));
    uVar2 = (uVar3 >> 0x10);
    iVar2 = (astruct_749 *)uVar3;
    _u16_1050_5b64 = CONCAT12(iVar2->field_0x94,CONCAT11(iVar2->field_0x95,iVar2->field_0x96));
    u16_1050_5b68 = CONCAT11(iVar2->field_0x3e5,iVar2->field_0x3e6);
    u16_1050_5b6a = iVar2->field996_0x3e4;
  }
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return;
    }
    bVar1 = false;
    for (uStack14 = 0x0; puVar1 = &struct10_5->field295_0x128, uStack14 <= *puVar1 && *puVar1 != uStack14;
        uStack14 += 0x1) {
      if ((&struct10_5->field_0x94 + uStack14 * 0x2) == param_2) {
        bVar1 = true;
        break;
      }
    }
    if (bVar1) {
      u16_1050_5b64 = u16_1050_5b68;
      u16_1050_5b66 = u16_1050_5b6a;
    }
  }
  SetTextColor16(CONCAT22(u16_1050_5b66,u16_1050_5b64),in_hdc_param_4);
  SetBkColor16(0x1000000,in_hdc_param_4);
  return;
}



StructD * pass1_1038_9ed4(StructD *param_1,param_2: u8)

{
  let mut in_stack_0000ffda: u16;

  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_57 * pass1_1038_9f76(param_1: *mut astruct_57,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfba,param_5);
  param_1->field0_0x0 = 0xa0b6;
  (param_1 + 0x2) = &u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_9fa4(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xa0b6;
  (param_1 + 0x2) = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn show_win_1038_9fd0(StructB *param_1)

{
  dialog_ui_fn_1040_78e2(param_1);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(0x5,*(HWND16 *)(param_1 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 call_fn_ptr_1038_9ffa(mut param_1: u32,astruct_733 *pstruct_param_2,mut param_3: u16 )

{
  code **ppcVar1;
  astruct_43 *struct_3;
  astruct_43 *struct_2;
  let mut puStack8: *mut u32;
  HDC16 hdc;
  let mut var_5: u16;

  hdc = GetDC16(pstruct_param_2->hwnd_0x6);
  struct_2 = FUN_1010_830a(hdc,param_1,s_tile2_bmp_1050_1538,_u16_1050_14cc,0x3);
  puStack8 = CONCAT22(param_1,struct_2);
  struct_3 = (astruct_43 *)*puStack8;
  ppcVar1 = (code **)&struct_3->fn_ptr_field_0x8;
  (**ppcVar1)(0x1010,struct_2,param_1,&hdc,&DAT_1050_1050);
  ppcVar1 = (code **)&struct_3->fn_ptr_field_0x4;
  (**ppcVar1)(0x1010,puStack8,0x50005,&hdc,&DAT_1050_1050);
  ppcVar1 = (code **)&struct_3->fn_ptr_field_0xc;
  (**ppcVar1)(0x1010,puStack8,&hdc,&DAT_1050_1050);
  ReleaseDC16(hdc,pstruct_param_2->hwnd_0x6);
  return 0x0;
}
