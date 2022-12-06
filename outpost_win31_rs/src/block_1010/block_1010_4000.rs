
pub fn pass1_1010_404a(mut param_1: i16,mut param_2: u32,mut param_3: u32)

{
  let mut iVar1: i16;
  let mut BVar2: bool;
  let mut local_4: u16;

  read_file_1008_7cfe(param_3,(param_3 >> 0x10),0x5);
  if (param_1 == 0) {
    u16_1050_0310 = 0x6d4;
  }
  else {
    iVar1 = param_2;
    BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(param_2 & 0xffff0000 | (iVar1 + 0x24)),0x2);
    if (BVar2 != 0) {
      BVar2 = read_file_1008_7dee((HFILE16 *)param_3,CONCAT22(0x1050,&local_4),0x2);
      if (BVar2 != 0) {
        BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(param_2 & 0xffff0000 | (iVar1 + 0x7e)),0x2);
        if (BVar2 != 0) {
          (iVar1 + 0x6a) = local_4;
          return;
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_40cc(mut param_1: i16,mut param_2: u16 ,mut param_3: u32) -> u32

{
  pass1_1030_8344(_u16_1050_5748,(param_3 + 0x6c));
  return CONCAT22((param_1 + 0xe),(param_1 + 0xc));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 pt_in_rect_1010_40f8(param_1: *mut astruct_57,mut param_2: u32,POINT16 *param_3,mut param_4: u16 )

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  let mut BVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut paVar8: *mut Struct57;
  let mut iVar10: i16;
  let mut uVar11: u16;
  let mut puVar12: *mut u32;
  let mut in_stack_0000fe94: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc2: u16;
  let mut puStack16: *mut u32;
  let mut iStack6: i16;
  let mut uStack4: u16;
  let mut uVar9: u32;

  iStack6 = 0;
  uStack4 = 0;
  loop {
    uVar11 = (param_2 >> 0x10);
    iVar10 = param_2;
    piVar1 = (iVar10 + 0x74);
    if (*piVar1 == iStack6 || *piVar1 < iStack6) {//
LAB_1010_413e:
      if ((uStack4 != 0) && (0x3 < PTR_LOOP_1050_3960)) {
        puVar12 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_4,iStack6 + 0xcU),
                                  in_stack_0000fe94,in_stack_0000ffb8,in_stack_0000ffbe,in_stack_0000ffc2);
        paVar8 = (astruct_57 *)(param_1 & 0xffff0000 | puVar12 >> 0x10);
        uVar4 = pass1_1018_0afa(puVar12);
        if (uVar4 == 0) {
          uVar11 = 0x1000;
          uVar5 = uVar4;
          mem_op_1000_179c(0xb4,paVar8);
          uVar6 = paVar8 | uVar5;
          uVar9 = paVar8 & 0xffff0000 | uVar6;
          if (uVar6 == 0) {
            iVar10 = 0;
            uVar7 = 0;
          }
          else {
            uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
            iVar10 = string_1040_8520(uVar9,(astruct_57 *)CONCAT22(paVar8,uVar5),HWND16_1050_0396,0x20030,
                                      0x6450643);
            uVar7 = uVar9;
          }
          puStack16 = CONCAT22(uVar7,iVar10);
          ppcVar2 = (code **)(*puStack16 + 0x74);
          (**ppcVar2)(uVar11,iVar10,uVar7);
          pass1_1010_209e(_u16_1050_0ed0,iStack6 + 0xcU);
          uStack4 = uVar4;
        }
      }
      if (uStack4 != 0) {
        return iStack6;
      }
      return -0x1;
    }
    param_1 = (astruct_57 *)(param_1 & 0xffff0000 | (iVar10 + 0x72));
    BVar3 = PtInRect16(*param_3,(RECT16 *)((iStack6 * 0x10 + (iVar10 + 0x24)) * 0x8 + (iVar10 + 0x70)));
    if (BVar3 != 0) {
      uStack4 = 0x1;
  // TODO: goto LAB_1010_413e;
    }
    iStack6 += 0x1;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_41d6(param_1: *mut astruct_57,param_2: *mut astruct_243,mut param_3: u32)

{
  let mut puVar1: *mut u16;
  let mut piVar2: *mut i16;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut puVar6: *mut u8;
  let mut uVar7: u16;
  let mut paVar8: *mut Struct57;
  let mut uVar9: u32;
  astruct_243 *iVar9;
  let mut iVar11: i16;
  astruct_244 *iVar10;
  let mut unaff_SI: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut puVar14: *mut u32;
  let mut in_stack_0000fe6e: u16;
  let mut in_stack_0000ff92: u16;
  let mut in_stack_0000ff98: u16;
  let mut in_stack_0000ff9c: u16;
  let mut iStack50: i16;
  let mut local_30: i16;
  let mut local_2e: *mut c_char;
  let mut iStack42: i16;
  let mut pcStack40: *mut c_char;
  let mut pcStack34: *mut c_char;
  let mut pcStack30: *mut c_char;
  let mut iStack26: i16;
  let mut uStack24: u16;
  let mut iStack22: i16;
  let mut pcStack20: *mut c_char;
  let mut uStack16: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut iStack6: i16;
  let mut uStack4: u16;

  uVar12 = (param_2 >> 0x10);
  iVar9 = (astruct_243 *)param_2;
  iVar9->field108_0x6c = param_3;
  puVar14 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fe6e,
                            in_stack_0000ff92,in_stack_0000ff98,in_stack_0000ff9c);
  uVar9 = param_1 & 0xffff0000;
  iStack6 = puVar14;
  uStack4 = (puVar14 >> 0x10);
  uStack10 = pass1_1010_ec40(iStack6,uStack4,iStack6,uStack4,iVar9->field108_0x6c);
  paVar8 = (astruct_57 *)(uVar9 & 0xffff0000 | uStack10 >> 0x10);
  iVar9->field112_0x74 = (uStack10 + 0x22);
  if (*(i32 *)&iVar9->field_0x70 != 0) {
    pcStack34 = *&iVar9->field_0x70;
    pcStack30 = pcStack34;
    fn_ptr_1000_17ce(pcStack34);
    &iVar9->field_0x70 = 0;
  }
  iVar4 = iVar9->field112_0x74 << 0x7;
  mem_op_1000_179c(iVar4,paVar8);
  &iVar9->field_0x70 = iVar4;
  iVar9->field111_0x72 = paVar8;
  pass1_1030_8344(_u16_1050_5748,iVar9->field108_0x6c);
  uStack14 = CONCAT22(paVar8,iVar4);
  uStack16 = (*(iVar4 + 0x10) == 0x9);
  iStack22 = (uStack10 + 0x22);
  uVar7 = iStack22 * 0x6;
  mem_op_1000_179c(uVar7,paVar8);
  uVar5 = paVar8;
  pcStack30 = CONCAT22(uVar5,uVar7);
  uVar9 = (uVar5 | uVar7);
  if ((uVar5 | uVar7) == 0) {
    pcStack20 = NULL;
  }
  else {
    pass1_1000_5586(0x3e38,0x1008,iStack22,0x6,uVar7,uVar5);
    pcStack20 = pcStack30;
  }
  uStack24 = 0;
  while( true ) {
    puVar6 = uVar9;
    uVar13 = (uStack10 >> 0x10);
    puVar1 = (uStack10 + 0x22);
    if (*puVar1 < uStack24 || *puVar1 == uStack24) break;
    uVar3 = (uStack10 + 0x24);
    uVar7 = uStack24;
    pass1_1028_e0a0(puVar6,_PTR_LOOP_1050_65e2,(uVar3 + uStack24 * 0x2) << 0x10);
    pcStack34 = CONCAT22(puVar6,uVar7);
    uVar9 = pcStack20 >> 0x10;
    pass1_1008_3f62((pcStack20 & 0xffff0000 | (uStack24 * 0x6 + pcStack20)),
                    CONCAT22(puVar6,uVar7 + 0x8));
    pcStack40 = pcStack34;
    pcStack30 = pcStack34;
    if (pcStack34 != NULL) {
      fn_ptr_1030_84d0(pcStack34);
      fn_ptr_1000_17ce(pcStack34);
    }
    uStack24 += 0x1;
  }
  for (iStack26 = 0; piVar2 = &iVar9->field112_0x74, *piVar2 != iStack26 && iStack26 <= *piVar2; iStack26 += 1) {
    pass1_1008_3e94((pcStack20 & 0xffff0000 | (iStack26 * 0x6 + pcStack20)),
                    CONCAT22(0x1050,&local_2e),CONCAT22(0x1050,&local_30));
    iStack42 = pass1_1000_49b2(local_2e);
    iStack42 /= 0x5;
    if (0xc < iStack42) {
      iStack42 = 0xc;
      iVar4 = pass1_1000_49b2(local_2e);
      local_2e = (local_2e & 0xffff0000 | ((local_2e / iVar4) * 0x3c));
    }
    iVar4 = pass1_1000_49b2(local_2e);
    uVar7 = iVar4 % 0x5;
    pcStack34 = (pcStack34 & 0xffff0000 | uVar7);
    if (local_2e < 0x0) {
      if (0x2 < uVar7) {
        uVar7 -= 0x5;
      }
      local_2e = (local_2e & 0xffff0000 | (local_2e + uVar7));
    }
    else if (uVar7 < 0x3) {
      local_2e = (local_2e & 0xffff0000 | (local_2e - uVar7));
    }
    else {
      local_2e = (local_2e & 0xffff0000 | (local_2e + (0x5 - uVar7)));
    }
    iStack50 = local_30 / 0x16;
    for (iVar4 = 0; iVar4 < 0x10; iVar4 += 1) {
      if (0xf < iStack50) {
        iStack50 = 0;
      }
      if (((uStack16 != 0) < iStack50) && (iStack50 < 0x8)) {
        iVar11 = ((iStack42 * 0x10 + iStack50) * 0x2 + 0x11e0);
        iVar10 = (astruct_244 *)((iStack26 * 0x10 + iVar4) * 0x8);
        uVar3 = &iVar9->field_0x70;
        (iVar10 + uVar3) = iVar11 + 0x49;
        uVar3 = &iVar9->field_0x70;
        (iVar10 + uVar3 + 0x2) = local_2e + 0x49;
        uVar3 = &iVar9->field_0x70;
        (iVar10 + uVar3 + 0x4) = iVar11 + 0x4e;
        uVar3 = &iVar9->field_0x70;
        (iVar10 + uVar3 + 0x6) = local_2e + 0x4e;
      }
      else {
        iVar11 = (iStack26 * 0x10 + iVar4) * 0x8;
        uVar3 = &iVar9->field_0x70;
        (iVar11 + uVar3) = 0;
        uVar3 = &iVar9->field_0x70;
        (uVar3 + iVar11 + 0x2) = 0;
        uVar3 = &iVar9->field_0x70;
        (uVar3 + iVar11 + 0x4) = 0x1;
        uVar3 = &iVar9->field_0x70;
        (uVar3 + iVar11 + 0x6) = 0x1;
      }
      iStack50 += 0x1;
    }
  }
  pcStack40 = pcStack20;
  local_2e = pcStack20;
  fn_ptr_1000_17ce(pcStack20);
  draw_1010_47ae(param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_451a(param_1: *mut u8,mut param_2: u32) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut puVar3: *mut u32;
  let mut uVar4: u32;
  let mut in_stack_0000fe9e: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000fff6: u16;

  puVar3 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000fff6,0x2f),in_stack_0000fe9e,in_stack_0000ffc2,
                           in_stack_0000ffc8,in_stack_0000ffcc);
  uVar1 = (puVar3 >> 0x10);
  uVar4 = pass1_1010_ec40(puVar3,uVar1,puVar3,uVar1,(param_2 + 0x6c));
  uVar2 = (uVar4 >> 0x10);
  return CONCAT22((uVar4 + 0x4),(uVar4 + 0x2));
}



pub fn pass1_1010_454a(mut param_1: u32) -> u32

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar2 = (iVar1 + 0x24) * 0x4;
  return CONCAT22((iVar1 + iVar2 + 0x28),(iVar1 + iVar2 + 0x26));
}
pub fn pass1_1010_4566(mut param_1: i16,mut param_2: u16 ,mut param_3: i16)

{
  if (param_3 != 0x2) {
    return;
  }
  pass1_1010_4956(CONCAT22(param_2,param_1 + -0x20));
  pass1_1010_1f62((astruct_27 *)CONCAT22(param_2,param_1 + -0x20),0x2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_459e(param_1: *mut astruct_27)

{
  let mut paVar1: *mut Struct57;
  let mut in_EDX: u32;
  let mut paVar2: *mut Struct57;

  if (param_1 == NULL) {
    paVar1 = NULL;
    paVar2 = (astruct_57 *)(in_EDX & 0xffff0000);
  }
  else {
    paVar2 = (astruct_57 *)(in_EDX & 0xffff0000 | param_1);
    paVar1 = (astruct_57 *)(param_1 + 0x20);
  }
  pass1_1008_9262(paVar1,paVar2,_PTR_LOOP_1050_0388,(_PTR_LOOP_1050_0388 >> 0x10),0x1f4,
                  CONCAT22(paVar2,paVar1));
  (param_1 + 0x7e) = 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_45d6(i32 param_1)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut unaff_CS: u16;
  let mut iStack4: i16;

  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  if ((iVar6 + 0x7e) != 0) {
    if (_PTR_LOOP_1050_0388 != 0) {
      if (param_1 == 0) {
        iVar4 = 0;
        uVar5 = 0;
      }
      else {
        iVar4 = iVar6 + 0x20;
        uVar5 = uVar7;
      }
      unaff_CS = 0x1008;
      pass1_1008_92b2(_PTR_LOOP_1050_0388,0x1f4,CONCAT22(uVar5,iVar4));
    }
    for (iStack4 = 0; iStack4 < 0x10; iStack4 += 1) {
      if ((iVar6 + 0x24) != iStack4) {
        puVar1 = (iVar6 + 0x26 + iStack4 * 0x4);
        uVar2 = (iVar6 + 0x26 + iStack4 * 0x4 + 2);
        if ((uVar2 | puVar1) != 0) {
          ppcVar3 = (code **)*puVar1;
          (**ppcVar3)(unaff_CS,puVar1,uVar2,1);
        }
        (iVar6 + iStack4 * 0x4 + 0x26) = 0;
      }
    }
    (iVar6 + 0x7e) = 0;
  }
  return;
}
pub fn pass1_1010_4674(param_1: *mut astruct_27,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 )

{
  let mut piVar1: *mut i16;
  astruct_27 *paVar2;
  let mut uVar2: u16;

  paVar2 = (astruct_27 *)param_1;
  uVar2 = (param_1 >> 0x10);
  if (param_2 == 1) {
    piVar1 = (&paVar2->field30_0x22 + 2);
    *piVar1 = *piVar1 + 1;
    if (0xf < (&paVar2->field30_0x22 + 0x2)) {
      (&paVar2->field30_0x22 + 0x2) = 0;
    }//
LAB_1010_469a:
    draw_op_1010_47d0(paVar2,uVar2,(&paVar2->field30_0x22 + 0x2));
  }
  else if (param_2 != 0x2) {
    if (param_2 != 0x3) {
      if ((paVar2[0x1].field19_0x14 != 0) && (paVar2[0x1].field19_0x14 != 0x4)) {
        pass1_1010_459e(param_1);
      }
  // TODO: goto LAB_1010_46e8;
    }
    piVar1 = (&paVar2->field30_0x22 + 2);
    *piVar1 = *piVar1 + -0x1;
    if (*piVar1 < 0x0) {
      (&paVar2->field30_0x22 + 0x2) = 0xf;
    }
// TODO: goto LAB_1010_469a;
  }
  pass1_1010_1f62(param_1,0x2);
  pass1_1010_45d6(param_1);//
LAB_1010_46e8:
  paVar2[0x1].field19_0x14 = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn get_sys_metrics_1010_46f6(mut param_1: u32,param_2: *mut astruct_57)

{
  let mut IVar1: i16;
  let mut IVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut puVar5: *mut u32;
  let mut uVar6: u32;
  let mut in_stack_0000fe7c: u16;
  let mut in_stack_0000ffa0: u16;
  let mut in_stack_0000ffa6: u16;
  let mut in_stack_0000ffaa: u16;
  let mut piVar7: *mut i16;
  let mut uVar8: u16;
  let mut pcVar9: *mut c_char;
  let mut local_6: i16;
  let mut local_4: i16;

  pcVar9 = CONCAT22(0x1050,&local_4);
  piVar7 = &local_6;
  uVar8 = SUB42(&DAT_1050_1050,0x0);
  puVar5 = mixed_1010_20ba(param_2,_u16_1050_0ed0,(u8 **)CONCAT22(piVar7,0x48),in_stack_0000fe7c,
                           in_stack_0000ffa0,in_stack_0000ffa6,in_stack_0000ffaa);
  pass1_1008_3e94((puVar5 & 0xffff0000 | (puVar5 + 0xe)),CONCAT22(uVar8,piVar7),pcVar9)
  ;
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  uVar6 = pass1_1008_4772(*(astruct_76 **)(iVar3 + 0x66));
  uVar8 = (uVar6 >> 0x10);
  (iVar3 + 0x18) = local_4 + 0x8;
  (iVar3 + 0x1a) = local_6 + 0x9;
  IVar1 = GetSystemMetrics16(SM_CXBORDER);
  (iVar3 + 0x1c) = IVar1 * 0x2 + (uVar6 + 0x4);
  IVar1 = GetSystemMetrics16(SM_CYCAPTION);
  IVar2 = GetSystemMetrics16(SM_CYBORDER);
  (iVar3 + 0x1e) = IVar2 + IVar1 + (uVar6 + 0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_4788(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,char *param_4)

{
  pass1_1030_8344(_u16_1050_5748,(param_3 + 0x6c));
  pass1_1030_301a(param_2,CONCAT22(param_2,param_1),param_4);
  return;
}
pub fn draw_1010_47ae(mut param_1: u32)

{
  let mut uStack4: u16;

  uStack4 = 0;
  loop {
    draw_op_1010_47d0((astruct_27 *)param_1,(param_1 >> 0x10),uStack4);
    uStack4 += 0x1;
  } while (uStack4 < 0x10);
  return;
}



// WARNING: Unable to use type for symbol uVar4
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn draw_op_1010_47d0(param_1: *mut astruct_27,mut param_2: u16 ,mut param_3: u16 )

{
  let mut piVar1: *mut i16;
  let mut puVar3: *mut u32;
  let mut iVar5: i16;
  HPALETTE16 hpalette16_var6;
  HGDIOBJ16 handle;
  HGDIOBJ16 hgdiobj16_00;
  astruct_797 *iVar4;
  let mut uVar5: u16;
  let mut extraout_DX: *mut u8;
  let mut puVar5: *mut u8;
  let mut iVar7: i16;
  astruct_739 *iVar8;
  let mut uVar11: u16;
  let mut uVar12: u32;
  let mut iStack32: i16;
  HDC16 hdc16_var_1;
  let mut devmodea_init_data: u16;
  let mut uStack16: u16;
  let mut local_e: u16;
  let mut uStack12: u16;
  let mut uStack10: u16;
  let mut uStack8: u16;
  HGDIOBJ16 stock_obj_handle;
  HPEN16 pen_handle;
  let mut uVar4: u32;
  let mut puVar2: *mut u32;
  let mut uVar13: u16;
  u8 uVar14;
  u8 uVar15;
  let mut uVar16: u16;
  let mut uVar17: u16;
  let mut offset_1: u16;
  let mut base_addr_1: u16;
  code **fn_ptr_1;

  pen_handle = CreatePen16(0x77d7fb,0x1,0x0);
  stock_obj_handle = GetStockObject16(HOLLOW_BRUSH);
  local_e = 0;
  uStack12 = 0;
  uStack10 = 0x1;
  uStack8 = 0x1;
  puVar3 = (&param_1->field_0x26 + param_3 * 0x4);
  puVar5 = (&param_1->field33_0x28)[param_3 * 0x2];
  if ((puVar5 | puVar3) != 0) {
    fn_ptr_1 = (code **)*puVar3;
    (**fn_ptr_1)(s_tile2_bmp_1050_1538,puVar3,puVar5,1);
    puVar5 = extraout_DX;
  }
  iVar5 = param_3 + 0x105;
  pass1_1010_8170(puVar5,_u16_1050_14cc,iVar5);
  iVar8 = (astruct_739 *)(param_3 * 0x4);
  (iVar8 + (&param_1->field_0x0 + 0x26)) = iVar5;
  (iVar8 + (&param_1->field_0x0 + 0x28)) = puVar5;
  base_addr_1 = &DAT_1050_1050;
  offset_1 = 0x1380;
  uVar16 = 0;
  uVar17 = 0;
  uVar13 = 0;
  uVar14 = '\0';
  uVar15 = '\0';
  uVar12 = pass1_1008_4772(*(astruct_76 **)(&param_1->field_0x26 + iVar8));
  uVar12 = (uVar12 >> 0x10);
  devmodea_init_data = uVar12;
  uStack16 = uVar12;
  hdc16_var_1 = CreateDC16((DEVMODEA *)(uVar12 & 0xffff | uVar12 << 0x10),
                           CONCAT13(uVar15,CONCAT12(uVar14,uVar13)),CONCAT22(uVar17,uVar16),
                           CONCAT22(base_addr_1,offset_1));
  hpalette16_var6 =
       palette_op_1008_4e08
                 ((HPALETTE16)&hdc16_var_1,uVar12,*(astruct_13 **)(_PTR_LOOP_1050_4230 + 0xe),
                  (HDC16 *)CONCAT22(0x1050,&hdc16_var_1));
  handle = SelectObject16(pen_handle,hdc16_var_1);
  hgdiobj16_00 = SelectObject16(stock_obj_handle,hdc16_var_1);
  iStack32 = 0;
  while( true ) {
    piVar1 = &param_1[0x1].field_0x1e;
    if (*piVar1 == iStack32 || *piVar1 < iStack32) break;
    iVar4 = (astruct_797 *)((iStack32 * 0x10 + param_3) * 0x8);
    uVar5 = pass1_1000_484c(CONCAT22(0x1050,&local_e),
                            CONCAT22(&param_1[0x1].field_0x1c,iVar4 + param_1[0x1].field23_0x1a),0x8);
    if (uVar5 != 0) {
      uVar4 = &param_1[0x1].field23_0x1a;
      uVar11 = (uVar4 >> 0x10);
      iVar7 = uVar4;
      Rectangle16(*(INT16 *)(iVar4 + iVar7 + 0x6),*(INT16 *)(iVar4 + iVar7 + 0x4),*(INT16 *)(iVar4 + iVar7 + 0x2),
                  *(INT16 *)(iVar4 + iVar7),hdc16_var_1);
    }
    iStack32 += 0x1;
  }
  hpalette16_var6 = SelectPalette16(0x0,hpalette16_var6,hdc16_var_1);
  DeleteObject16(hpalette16_var6);
  SelectObject16(handle,hdc16_var_1);
  SelectObject16(hgdiobj16_00,hdc16_var_1);
  DeleteDC16(hdc16_var_1);
  DeleteObject16(pen_handle);
  return;
}
pub fn pass1_1010_4956(mut param_1: u32)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  iVar2 = (iVar3 + 0x6a);
  if (iVar2 == 0) {
    piVar1 = (iVar3 + 0x24);
    *piVar1 = *piVar1 + 1;
    if (0xf < (iVar3 + 0x24)) {
      (iVar3 + 0x24) = 0;
      return;
    }
  }
  else {
    if (iVar2 != 0x4) {
      return;
    }
    piVar1 = (iVar3 + 0x24);
    *piVar1 = *piVar1 + -0x1;
    if (*piVar1 < 0x0) {
      (iVar3 + 0x24) = 0xf;
    }
  }
  return;
}



StructD * pass1_1010_4994(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  param_2 = (param_2 & 0xffff0000 | (param_2 - 0x20));
  pass1_1010_3f00(param_2);
  if ((param_3 & 1) != 0) {
    fn_ptr_1000_17ce(param_2);
  }
  return param_2;
}



pub fn pass1_1010_49a0(mut param_1: i16,mut param_2: u16 ) -> u32

{
  return CONCAT22(param_2,param_1 + 0xa);
}



pub fn pass1_1010_49b0(mut param_1: i16,mut param_2: u16 ) -> u32

{
  return CONCAT22(param_2,param_1 + 0x18);
}



u16 pass1_1010_49c0(mut param_1: u32)

{
  return (param_1 + 0x14);
}
pub fn pass1_1010_49ce(mut param_1: u32,mut param_2: u16 )

{
  (param_1 + 0x14) = param_2;
  return;
}



u16 pass1_1010_49e0(mut param_1: u32)

{
  return (param_1 + 0x16);
}
pub fn pass1_1010_49ee(mut param_1: u32,mut param_2: u16 )

{
  (param_1 + 0x16) = param_2;
  return;
}
pub fn pass1_1010_4a00(mut param_1: u32,mut param_2: u16 )

{
  (param_1 + 0x12) = param_2;
  return;
}



u16 pass1_1010_4a12(mut param_1: u32)

{
  return (param_1 + 0x12);
}



u16 * pass1_1010_4a20(param_1: *mut u16,param_2: u8)

{
  pass1_1010_3f00(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_4a8a(mut param_1: u32,param_2: *mut astruct_19,param_3: *mut astruct_19,mut param_4: u16 ,mut param_5: u16 ,
                    mut param_6: u16 ,mut param_7: u16 ,mut param_8: u16 ,mut param_9: u16 )

{
  let mut IVar1: i16;
  let mut uVar3: u16;
  let mut paVar2: *mut Struct57;
  let mut unaff_CS: u16;
  astruct_19 *paVar4;
  let mut puVar5: *mut u32;
  let mut uStack4: u16;

  uVar3 = (param_1 >> 0x10);
  paVar4 = struct_op_1010_1d48((astruct_19 *)CONCAT22(param_3,param_2),param_4);
  paVar2 = (astruct_57 *)CONCAT22(uVar3,(paVar4 >> 0x10));
  &param_2->field11_0x16 = 0;
  &param_2->field_0x1a = 0;
  param_2->field16_0x1e = 0;
  param_2->field17_0x20 = 0x1;
  param_2->field18_0x22 = 0;
  param_2->field19_0x24 = 0;
  param_2->field20_0x26 = NULL;
  param_2->field21_0x2a = 0;
  param_2->field22_0x2c = 0x1;
  param_2->field23_0x2e = 0;
  param_2->field24_0x30 = 0;
    // just 0x5024
  param_2->field25_0x32 = 0;
    // just 0x5024
  CONCAT22(param_3,param_2) = s_SCForceMorale__s_for_colony__08l_1050_5024 + 0x6;
  param_2->segment_0x2 = 0x1010;
  IVar1 = FUN_1010_830a(0x0,paVar2,unaff_CS,_u16_1050_14cc,0x1b3);
  param_2->field11_0x16 = IVar1;
  param_2->field12_0x18 = paVar2;
  puVar5 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(uStack4,0x3),param_6,param_7,param_8,param_9);
  &param_2->field20_0x26 = puVar5;
  (&param_2->field20_0x26 + 0x2) = (puVar5 >> 0x10);
  pass1_1008_4772(*(astruct_76 **)&param_2->field11_0x16);
  &param_2->field_0xe = 0x13c;
  param_2->horiz_res_0xa = 0;
  param_2->field8_0x10 = 0;
  param_2->ver_res_0xc = 0;
  return;
}
pub fn free_rsrc_1010_4b3e(StructD *param_1)

{
  let mut puVar3: *mut u32;
  let mut uVar5: u32;
  let mut BVar6: bool;
  StructD *pstructd_1;
  astruct_818 *iVar7;
  StructD *pstructd_1_hi;
  let mut uVar4: u16;
  let mut unaff_CS: u16;
  let mut iStack4: i16;
  let mut puVar2: *mut u32;
  let mut piVar1: *mut i16;
  let mut uVar1: u16;
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **fn_ptr_1;

  pstructd_1_hi = (param_1 >> 0x10);
  pstructd_1 = param_1;
    // really just 0x5024
  param_1->address_offset_field_0x0 = s_SCForceMorale__s_for_colony__08l_1050_5024 + 0x6;
  pstructd_1->address_offset_field_0x2 = 0x1010;
  if (&pstructd_1->field_0x2a != 0) {
    unaff_CS = SUB42(s_tile2_bmp_1050_1538,0x0);
    BVar6 = GlobalUnlock16(*(HGLOBAL16 *)&pstructd_1->field_0x2a);
    if (BVar6 == 0) {
      unaff_CS = SUB42(s_tile2_bmp_1050_1538,0x0);
      FreeResource16(*(HGLOBAL16 *)&pstructd_1->field_0x2a);
    }
  }
  &pstructd_1->field_0x2a = 0;
  if (**(i32 **)&pstructd_1->field11_0x12 != 0) {
    iStack4 = 0;
    while( true ) {
      puVar3 = &pstructd_1->field11_0x12;
      piVar1 = (puVar3 + 0x8);
      if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
      uVar4 = (*puVar3 >> 0x10);
      iVar7 = (astruct_818 *)*puVar3;
      puVar2 = (iVar7 + iStack4 * 0x4);
      uVar1 = (iVar7 + iStack4 * 0x4 + 2);
      if ((uVar1 | puVar2) != 0) {
        fn_ptr_1 = (code **)*puVar2;
        (**fn_ptr_1)(unaff_CS,puVar2,uVar1,1);
      }
      iStack4 += 0x1;
    }
  }
  uVar5 = &pstructd_1->field11_0x12;
  fn_ptr_1000_17ce(*(uVar5 + 0x4));
  fn_ptr_1000_17ce(*&pstructd_1->field11_0x12);
  puVar1 = (&pstructd_1->field12_0x14 + 2);
  uVar2 = pstructd_1->field13_0x18;
  if ((uVar2 | puVar1) != 0) {
    fn_ptr_1 = (code **)*puVar1;
    (**fn_ptr_1)(0x1000,puVar1,uVar2,1);
  }
  fn_ptr_1000_17ce(*&pstructd_1->field14_0x1a);
  pass1_1010_1d80(param_1);
  return;
}



pub fn pass1_1010_4c2c(mut param_1: u32) -> u32

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x18),(param_1 + 0x16));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_4c3e(mut param_1: u32,mut param_2: i16,mut param_3: i16,u8 *param_4,mut param_5: u16 )

{
  let mut piVar1: *mut i16;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut uVar4: u32;
  let mut iVar5: i16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut unaff_CS: u16;
  let mut uVar9: u32;
  let mut iStack14: i16;
  u8 local_c [0x6];
  let mut uStack6: u16;
  let mut iStack4: i16;

  uVar4 = CONCAT22(in_register_0000000a,param_4);
  uVar7 = (param_1 >> 0x10);
  iVar5 = param_1;
  pass1_1010_bffa(param_3,param_4,(iVar5 + 0x26));
  (iVar5 + 0x12) = param_3;
  (iVar5 + 0x14) = uVar4;
  if ((uVar4 | (iVar5 + 0x12)) != 0) {
    if (param_2 == 0) {
      uVar4 = (iVar5 + 0x12);
      (iVar5 + 0x30) = (uVar4 + 0x8);
    }
    else {
      (iVar5 + 0x2e) = 0x1;
      uVar2 = (iVar5 + 0x12);
      uVar2 = (uVar2 + 0x4);
      iVar6 = (uVar2 + 2);
      if ((iVar6 == 0x5) || (iVar6 == 0x6)) {
        (iVar5 + 0x30) = 0x1;
        (iVar5 + 0x20) = 0;
      }
      else {
        (iVar5 + 0x30) = 0x2;
        uVar2 = (iVar5 + 0x12);
        uVar2 = (uVar2 + 0x4);
        (iVar5 + 0x32) = uVar2;
        uVar3 = FUN_1010_830a(uVar2,uVar4,unaff_CS,_u16_1050_14cc,0x1bf);
        uVar2 = (iVar5 + 0x12);
        uVar8 = (uVar2 >> 0x10);
        iVar6 = uVar2;
        (iVar6 + 0x4) = uVar3;
        (iVar6 + 0x6) = uVar4;
      }
    }
    iStack4 = 0x14;
    pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_c));
    uStack6 = 0;
    iStack14 = 0;
    while( true ) {
      piVar1 = (iVar5 + 0x30);
      if (*piVar1 == iStack14 || *piVar1 < iStack14) break;
      uVar4 = (iVar5 + 0x12);
      uVar9 = pass1_1008_4772(*(astruct_76 **)(uVar4 + iStack14 * 0x4));
      iStack4 += (-(iStack14 == 0) & 0x5) + 0x14 + (uVar9 + 0x4);
      iStack14 += 0x1;
    }
    if ((iVar5 + 0xe) < iStack4) {
      (iVar5 + 0xe) = iStack4;
    }
  }
  return;
}



// WARNING: This is an inlined function
// WARNING: Unable to use type for symbol uVar2
// WARNING: Unable to use type for symbol uVar1
pub fn struct_1010_4d5c(param_1: *mut u8,param_2: *mut astruct_245,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 ,mut param_7: i16)

{
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  astruct_245 *iVar3;
  astruct_747 *iVar5;
  let mut uVar6: u16;
  let mut uVar2: u32;
  let mut uVar1: u32;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar6 = (param_2 >> 0x10);
  iVar3 = (astruct_245 *)param_2;
  if (*(i32 *)&iVar3->field_0x1a == 0) {
    iVar4 = iVar3->field47_0x30 << 0x3;
    mem_op_1000_179c(iVar4,paVar5);
    &iVar3->field_0x1a = iVar4;
    iVar3->field28_0x1c = paVar5;
  }
  uVar2 = &iVar3->field_0x1a;
  iVar5 = (astruct_747 *)(param_7 * 0x8);
  (iVar5 + uVar2) = param_6;
  uVar3 = &iVar3->field_0x1a;
  (iVar5 + uVar3 + 0x2) = param_5;
  uVar1 = &iVar3->field_0x1a;
  (iVar5 + uVar1 + 0x4) = param_4;
  uVar3 = &iVar3->field_0x1a;
  (iVar5 + uVar3 + 0x6) = param_3;
  return;
}



pub fn pass1_1010_4dc8(mut param_1: u32) -> u32

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((iVar1 + 0x20) == 0) {
    return 0x0;
  }
  return CONCAT22((iVar1 + 0x1c),(iVar1 + 0x20) * 0x8 + (iVar1 + 0x1a));
}
pub fn pass1_1010_4df0(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_2 >> 0x10);
  uVar1 = (param_2 + 0x26);
  pass1_1010_c1ba(param_1,uVar1,(uVar1 >> 0x10),(param_2 + 0x20));
  return;
}
pub fn pt_in_rect_1010_4e08(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 )

{
  let mut piVar1: *mut i16;
  let mut bVar2: bool;
  let mut BVar3: bool;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut iStack12: i16;
  let mut iStack10: i16;
  POINT16 PStack8;

  PStack8 = (POINT16)CONCAT22(param_2,param_3);
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  (iVar4 + 0x22) = (iVar4 + 0x20);
  bVar2 = false;
  (iVar4 + 0x24) = 0;
  iStack12 = 0;
  iStack10 = 0;
  loop {
    piVar1 = (iVar4 + 0x30);
    if (*piVar1 == iStack12 || *piVar1 < iStack12) {//
LAB_1010_4e67:
      if (iStack10 != 0) {
        (iVar4 + 0x20) = iStack10;
      }
      if (bVar2) {
        return;
      }
      return;
    }
    BVar3 = PtInRect16(PStack8,(RECT16 *)((iVar4 + 0x1a) + iStack12 * 0x8));
    if (BVar3 != 0) {
      iStack10 = iStack12;
      bVar2 = true;
  // TODO: goto LAB_1010_4e67;
    }
    iStack12 += 0x1;
  } while( true );
}
pub fn pass1_1010_4e8c(mut param_1: u32)

{
  pass1_1010_1f62((astruct_27 *)param_1,0xd);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2
pub fn find_n_load_rsrc_1010_4e9e(astruct_812 *struct_param_1)

{
  let mut BVar1: bool;
  HRSRC16 h_rsrc;
  let mut handle: HGLOBAL16;
  astruct_812 *struct_1;
  let mut uVar3: u16;
  let mut uVar1: u32;
  let mut uVar2: u32;

  uVar3 = (struct_param_1 >> 0x10);
  struct_1 = (astruct_812 *)struct_param_1;
  if (struct_1->field29_0x20 != 0) {
    if (struct_1->hglobal_0x2a != 0) {
      BVar1 = GlobalUnlock16(struct_1->hglobal_0x2a);
      if (BVar1 == 0) {
        FreeResource16(struct_1->hglobal_0x2a);
      }
    }
    uVar1 = struct_1->field18_0x12;
    uVar2 = (uVar1 + 0x4);
    h_rsrc = FindResource16(0xa,
                            ((uVar2 + struct_1->field29_0x20 * 0x2) * 0x2 + 0x1384)
                            ,HINSTANCE16_1050_038c);
    handle = LoadResource16(h_rsrc,HINSTANCE16_1050_038c);
    struct_1->hglobal_0x2a = handle;
    if (handle != 0) {
      WIN16_LockResource16(handle);
      return;
    }
  }
  return;
}



u16 pass1_1010_4f20(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16)

{
  return (param_3 * 0x2 + 0x139a);
}
pub fn pass1_1010_4f30(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16,param_4: *mut u16)

{
  *param_4 = 0xa;
  *param_3 = 0x73;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_4f48(param_1: *mut astruct_482)

{
  let mut uVar1: u16;
  code **ppcVar2;
  let mut puVar3: *mut u32;
  let mut puVar4: *mut u32;
  let mut in_EDX: u32;
  let mut uVar6: u16;
  let mut uVar5: u32;
  astruct_482 *iVar6;
  astruct_483 *iVar7;
  astruct_482 *uVar7;
  let mut uVar8: u16;
  let mut unaff_CS: u16;

  uVar6 = (in_EDX >> 0x10);
  uVar7 = (astruct_482 *)(param_1 >> 0x10);
  iVar6 = (astruct_482 *)param_1;
  puVar3 = iVar6->field18_0x12;
  iVar6->field39_0x30 = (puVar3 + 0x8);
  if (iVar6->field40_0x32 != 0) {
    uVar5 = *iVar6->field18_0x12;
    uVar8 = (uVar5 >> 0x10);
    iVar7 = (astruct_483 *)uVar5;
    puVar3 = iVar7->field4_0x4;
    iVar7->field4_0x4 = iVar6->field40_0x32;
    if (puVar3 != NULL) {
      ppcVar2 = (code **)*puVar3;
      (**ppcVar2)();
    }
    iVar6->field40_0x32 = 0;
  }
  puVar4 = iVar6->field19_0x16;
  uVar1 = iVar6->field20_0x18;
  uVar5 = CONCAT22(uVar6,uVar1);
  if ((uVar1 | puVar4) != 0) {
    ppcVar2 = (code **)*puVar4;
    (**ppcVar2)();
  }
  puVar4 = FUN_1010_830a(puVar4,uVar5,unaff_CS,_u16_1050_14cc,0x1b3);
  iVar6->field19_0x16 = puVar4;
  iVar6->field20_0x18 = uVar5;
  fn_ptr_1000_17ce(iVar6->field21_0x1a);
  iVar6->field21_0x1a = 0;
  iVar6->field38_0x2e = 0;
  return;
}
