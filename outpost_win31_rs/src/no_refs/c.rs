
pub unsafe fn caseD_b3() {
    let mut in_DX: u16;
    let mut unaff_BP: i16;
    let mut in_stack_0000ffee: u16;

    ui_op_1008_2c4e(in_DX, in_stack_0000ffee, (unaff_BP + 0x6), 0x5d);
    return;
}

pub unsafe fn draw_op_1008_1230(mut param_1: u32) {
    let mut uVar1: u32;

    uVar1 = (param_1 + 0xe0);
    fill_rect_1008_39ac(uVar1, (uVar1 >> 0x10));
    return;
}

pub unsafe fn pass1_1008_1246(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0xe8) != 0) {
        ppcVar1 = ((param_1 + 0xe8) + 0x4c);
        (**ppcVar1)();
    }
    return;
}


pub unsafe fn pass1_1008_1272(mut param_1: u32, mut param_2: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0xe8) != 0) {
        ppcVar1 = ((param_1 + 0xe8) + 0x88);
        (**ppcVar1)();
        return;
    }
    pass1_1008_9cc4(param_1 & 0xffff | uVar2 << 0x10, param_2);
    return;
}


pub unsafe fn pass1_1008_12aa(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0xe8) != 0) {
        ppcVar1 = ((param_1 + 0xe8) + 0x8c);
        (**ppcVar1)();
        return;
    }
    pass1_1008_9ce0();
    return;
}

pub unsafe fn pass1_1008_3714(param_1: *mut StructA) {
    pass1_1008_3e0e(param_1);
    return;
}

pub unsafe fn pass1_1008_372c(mut param_1: i16, mut param_2: u16) -> u32 {
    return CONCAT22(param_2, param_1 + 0xa);
}
pub unsafe fn pass1_1008_373c() {
    return;
}
pub unsafe fn pass1_1008_3740() {
    return;
}
pub unsafe fn pass1_1008_3744() {
    return;
}
pub unsafe fn pass1_1008_3748() {
    return;
}
pub unsafe fn pass1_1008_374c() {
    return;
}
pub unsafe fn pass1_1008_3750() {
    return;
}
pub unsafe fn pass1_1008_3754() {
    return;
}

pub unsafe fn pass1_1008_3758() -> u16 {
    return 0x1;
}
pub unsafe fn pass1_1008_375e() {
    return;
}
pub unsafe fn pass1_1008_3762() {
    return;
}
pub unsafe fn pass1_1008_3766() {
    return;
}
pub unsafe fn FUN_1008_376a() {
    return;
}
pub unsafe fn FUN_1008_376e() {
    return;
}
pub unsafe fn FUN_1008_3772() {
    return;
}
pub unsafe fn FUN_1008_3776() {
    return;
}
pub unsafe fn pass1_1008_377a() {
    return;
}


pub unsafe fn pass1_1008_37aa(param_1: *mut astruct_450, param_2: u8) -> *mut astruct_450 {
    let mut uVar1: *mut astruct_450;
    let mut uVar2: *mut astruct_450;

    uVar2 = (param_1 >> 0x10);
    uVar1 = param_1;
    param_1.field0_0x0 = 0x380a;
    uVar1.field1_0x2 = 0x1008;
    param_1.field0_0x0 = 0x389a;
    uVar1.field1_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_3a10() {
    return;
}

pub unsafe fn pass1_1008_3a14(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1008_3a40(param_1: *mut astruct_451, param_2: u8) -> *mut u16 {
    let mut uVar1: *mut astruct_451;
    let mut uVar2: *mut astruct_451;

    uVar2 = (param_1 >> 0x10);
    uVar1 = param_1;
    param_1 = 0x3ab0;
    uVar1.field2_0x2 = 0x1008;
    param_1 = 0x389a;
    uVar1.field2_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_3a7a(mut param_1: u32, param_2: u8) -> u32 {
    pass1_1008_397a(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1008_3afe(param_1: *mut u16, param_2: u8) {
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

pub unsafe fn FUN_1008_3cd2() {
    return;
}


pub unsafe fn pass1_1008_3cd6(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    mix_win_ui_op_1040_911e(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1008_3d40() {
    return;
}


pub unsafe fn pass_1008_3d44(param_1: *mut astruct_453, param_2: u8) -> *mut u16 {
    let mut uVar1: *mut astruct_453;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    uVar1 = param_1;
    param_1.field0_0x0 = 0x380a;
    uVar1.field1_0x2 = 0x1008;
    param_1.field0_0x0 = 0x389a;
    uVar1.field1_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return &param_1.field0_0x0;
}


pub unsafe fn pass1_1008_4426(param_1: *mut astruct_76) -> u32 {
    let mut bVar1: bool;
    let mut iVar2: *mut astruct_76;
    let mut uVar2: *mut astruct_76;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (&iVar2.field3_0x6 == 0) {
        pass1_1008_47cc((param_1 & 0xffff | ZEXT24(uVar2) << 0x10));
    }
    if (&iVar2.field3_0x6 == 0) {
        bVar1 = false;
    } else {
        if (&iVar2.field5_0xa == 0) {
            pass1_1008_4834((param_1 & 0xffff | ZEXT24(uVar2) << 0x10));
        }
        bVar1 = true;
    }
    if (!bVar1) {
        return 0x0;
    }
    return CONCAT22(iVar2.field6_0xc, iVar2.field5_0xa);
}



pub unsafe fn set_di_bits_to_device_1008_45d6(
    param_1: *mut astruct_76,
    param_2: INT16,
    param_3: HDC16,
) {
    let mut bVar1: bool;
    let mut iVar2: *mut astruct_76;
    let mut info: BITMAPINFO;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut startscan: u16;
    let mut uVar1: u32;
    let mut uVar2: u32;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (&iVar2.field3_0x6 == 0) {
        pass1_1008_47cc(param_1);
    }
    if ((iVar2.field4_0x8 | iVar2.field3_0x6) == 0) {
        bVar1 = false;
    } else {
        if ((iVar2.field6_0xc | iVar2.field5_0xa) == 0) {
            pass1_1008_4834(param_1);
        }
        bVar1 = true;
    }
    if (!bVar1) {
        return;
    }
    uVar1 = iVar2.field8_0x10;
    uVar4 = (uVar1 >> 0x10);
    info = uVar1;
    startscan = &(info.bim_header).biHeight;
    uVar2 = &iVar2.field9_0x14;
    SetDIBitsToDevice(
        0x0,
        info,
        CONCAT22(uVar2, uVar4),
        (uVar2 >> 0x10),
        startscan,
        0x0,
        0x0,
        0x0,
        startscan,
        &(info.bim_header).biWidth,
        param_2,
        param_3,
    );
    return;
}



pub unsafe fn stretch_di_bits_1008_465a(param_1: *mut astruct_76, hdc_param_2: HDC16) {
    let mut x_src: i16;
    let mut y_src: i16;
    let mut uVar2: u32;
    let mut bVar3: bool;
    let mut iVar3: *mut astruct_76;
    let mut info: BITMAPINFO;
    let mut uVar4: u16;
    let mut bits: PVOID;
    let mut uVar1: u32;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if (&iVar3.field3_0x6 == 0) {
        pass1_1008_47cc(param_1);
    }
    if ((iVar3.field4_0x8 | iVar3.field3_0x6) == 0) {
        bVar3 = false;
    } else {
        if ((iVar3.field6_0xc | iVar3.field5_0xa) == 0) {
            pass1_1008_4834(param_1);
        }
        bVar3 = true;
    }
    if (!bVar3) {
        return;
    }
    uVar1 = iVar3.field8_0x10;
    bits = (PVOID)(uVar1 >> 0x10);
    info = uVar1;
    x_src = &(info.bim_header).biWidth;
    y_src = &(info.bim_header).biHeight;
    uVar2 = &iVar3.field9_0x14;
    StretchDIBits16(
        0xcc0020,
        0x0,
        info,
        bits,
        uVar2,
        (uVar2 >> 0x10),
        y_src,
        x_src,
        0x0,
        0x0,
        y_src,
        x_src,
        hdc_param_2,
    );
    return;
}


pub unsafe fn pass1_1008_48aa(mut param_1: u32) -> u16 {
    return (param_1 + 0xe);
}


pub unsafe fn pass1_1008_48b8(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1008_41bc(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_48de(
    param_1: *mut u16,
    mut param_2: i16,
    param_3: u8,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u32,
    mut param_7: u16,
    param_8: u8,
) {
    let mut pbVar1: *mut u8;
    let mut uVar2: u32;
    let mut bVar3: u8;
    let mut uVar4: u16;
    let mut bVar5: u8;
    let mut uVar6: u16;
    let mut unaff_BP: i16;
    let mut puVar7: *mut u8;
    let mut unaff_SI: i16;
    let mut iVar8: i16;
    let mut unaff_DI: *mut u8;
    let mut unaff_ES: u16;
    let mut uVar9: u16;

    uVar6 = param_4 & 0xff | ((param_4 >> 0x8) + param_4 + param_3) << 0x8;
    puVar7 = (unaff_BP + 1);
    pbVar1 = (param_1 + unaff_SI);
    bVar5 = (param_4 & 0xff);
    *pbVar1 = *pbVar1 | bVar5;
    // TODO bVar3 = in(0x46);
    pbVar1 = (param_1 + unaff_SI);
    *pbVar1 = *pbVar1 | bVar5;
    if (param_2 == 1) {
        pbVar1 = (param_1 + unaff_SI);
        *pbVar1 = *pbVar1 | bVar5;
        iVar8 = unaff_SI + 1;
        pbVar1 = (param_1 + iVar8);
        bVar5 = param_7;
        *pbVar1 = *pbVar1 | bVar5;
        pbVar1 = (param_1 + iVar8);
        *pbVar1 = *pbVar1 | bVar5;
        *unaff_DI = bVar3;
        pbVar1 = (param_1 + iVar8);
        *pbVar1 = *pbVar1 | bVar5;
        uVar6 = param_7;
        if (*pbVar1 != 0) {
            pbVar1 = (param_1 + iVar8);
            *pbVar1 = *pbVar1 | bVar5;
            puVar7 = (&param_7 + 1);
            param_1 = (param_6 >> 0x8);
            CONCAT13(param_8, param_6._1_3_) = 0x389a;
            param_1[0x1] = 0x1008;
            unaff_ES = (CONCAT13(param_8, param_6._1_3_) >> 0x10);
            (param_1 + 0x2) = 0;
            (param_1 + 0x4) = 0;
            param_1[0x6] = 0xffff;
            (param_1 + 0x7) = 0;
            (param_1 + 0x9) = 0;
            (param_1 + 0xb) = 0;
            (param_1 + 0xd) = 0;
            param_1[0xf] = 0;
        }
    } else {
        param_1[0x11] = bVar3 | 0x800;
    }
    param_1[0x11] = (puVar7 + 0xa);
    *param_1 = &u16_1050_4c4c;
    param_1[0x1] = 0x1008;
    uVar4 = str_op_1008_60e8(uVar6, *(puVar7 + 0xc));
    uVar2 = (puVar7 + 0x6);
    uVar9 = (uVar2 >> 0x10);
    iVar8 = uVar2;
    (iVar8 + 0x8) = uVar4;
    (iVar8 + 0xa) = uVar6;
    return;
}


pub unsafe fn pass1_1008_4b5e(param_1: u32) -> u32 {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut iVar3: i16;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x1e) == 0) {
        ppcVar1 = (*param_1 + 0x8);
        iVar2 = (**ppcVar1)();
        if (iVar2 == 0) {
            return 0x0;
        }
    }
    return CONCAT22((iVar3 + 0x6), (iVar3 + 0x4));
}


pub unsafe fn pass1_1008_4ef6(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1008_4cdc(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1008_507c(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1008_41bc(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub unsafe fn pass1_1008_570e(param_1: *mut u16,param_2: u8) -> *mut u16

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

pub unsafe fn pass1_1008_58a6(mut param_1: u32,mut param_2: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut paStack6: *mut astruct_99;

  paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_029c);
  uVar3 = (paStack6 >> 0x10);
  uVar2 = paStack6;
  if ((uVar3 | uVar2) == 0) {
    paStack6 = null_mut();
  }
  else {
    paStack6.field0_0x0 = 0x389a;
    (uVar2 + 0x2) = 0x1008;
    (uVar2 + 0x4) = 0;
    (uVar2 + 0x8) = 0;
    paStack6.field0_0x0 = s__s__s__1050_5bc0;
    (uVar2 + 0x2) = 0x1008;
  }
  if (paStack6.is_null()) {
    return;
  }
  uVar5 = (paStack6 >> 0x10);
  (paStack6 + 0x8) = param_2;
  uVar6 = (param_1 >> 0x10);
  iVar4 = param_1;
  (paStack6 + 0x4) = (iVar4 + 0x4);
  (iVar4 + 0x4) = paStack6;
  piVar1 = (iVar4 + 0x8);
  *piVar1 = *piVar1 + 1;
  return;
}

pub unsafe fn pass1_1008_593c(param_1: u32,mut param_2: u32)

{
  let mut piVar1: *mut i16;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut paStack6: *mut astruct_99;

  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  if ((iVar5 + 0x8) == 0) {
    ppcVar2 = (*param_1 + 0x4);
    (**ppcVar2)();
    return;
  }
  paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_029c);
  uVar4 = (paStack6 >> 0x10);
  uVar3 = paStack6;
  if ((uVar4 | uVar3) == 0) {
    paStack6 = null_mut();
  }
  else {
    paStack6.field0_0x0 = 0x389a;
    (uVar3 + 0x2) = 0x1008;
    (uVar3 + 0x4) = 0;
    (uVar3 + 0x8) = 0;
    paStack6.field0_0x0 = s__s__s__1050_5bc0;
    (uVar3 + 0x2) = 0x1008;
  }
  if (paStack6.is_null()) {
    return;
  }
  (paStack6 + 0x8) = param_2;
  loop {
    param_1 = (param_1 + 0x4);
    uVar7 = (param_1 >> 0x10);
    if param_1 + 0x4 == 0 {break;}
  }
  (param_1 + 0x4) = paStack6;
  piVar1 = (iVar5 + 0x8);
  *piVar1 = *piVar1 + 1;
  return;
}


pub unsafe fn pass1_1008_59f4(mut param_1: u32,param_2: i32)

{
  let mut piVar1: *mut i16;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  let mut puVar4: *mut u32;
  let mut ppcVar5: *mut *mut code;
  let mut puVar6: *mut u32;
  let mut uVar7: u16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uStack10: u16;
  let mut puStack6: *mut u32;

  puStack6 = null_mut();
  uVar9 = (param_1 >> 0x10);
  puVar6 = puStack6;
  puVar4 = param_1;
  loop {
    puStack6 = puVar6;
    uVar10 = (puVar4 >> 0x10);
    iVar8 = puVar4;
    puVar4 = (iVar8 + 0x4);
    uStack10 = puVar4;
    uVar11 = (puVar4 >> 0x10);
    if (((iVar8 + 0x6) | uStack10) == 0) {break;}
    puVar6 = puVar4;
    if !((uStack10 + 0x8) != param_2){break;}
  }
  if (puVar4.is_null() == false) {
    if (puStack6.is_null()) {
      uVar10 = (uStack10 + 0x4);
      uVar7 = (uStack10 + 0x6);
      puStack6 = param_1;
    }
    else {
      uVar10 = (uStack10 + 0x4);
      uVar7 = (uStack10 + 0x6);
    }
    uVar12 = (puStack6 >> 0x10);
    (puStack6 + 0x4) = uVar10;
    (puStack6 + 0x6) = uVar7;
    if ((param_1 + 0xa) != 0) {
      puVar2 = (uStack10 + 0x8);
      uVar3 = (uStack10 + 0xa);
      if ((uVar3 | puVar2) != 0) {
        ppcVar5 = *puVar2;
        (**ppcVar5)();
      }
    }
    if (puVar4.is_null() == false) {
      ppcVar5 = *puVar4;
      (**ppcVar5)();
    }
    piVar1 = (param_1 + 0x8);
    *piVar1 = *piVar1 -0x1;
    return;
  }
  return;
}


pub unsafe fn pass1_1008_5ab8(mut param_1: u32)

{
  let mut piVar1: *mut i16;
  let mut ppcVar2: *mut *mut code;
  let mut puVar3: *mut u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if ((iVar4 + 0x4) == 0) {
    return;
  }
  puVar3 = (iVar4 + 0x4);
  uVar6 = (puVar3 >> 0x10);
  (iVar4 + 0x4) = (puVar3 + 0x4);
  if ((uVar6 | puVar3) != 0) {
    ppcVar2 = *puVar3;
    (**ppcVar2)();
  }
  piVar1 = (iVar4 + 0x8);
  *piVar1 = *piVar1 -0x1;
  return;
}


pub unsafe fn pass1_1008_5b6e(param_1: *mut u16,param_2: u8) -> *mut u16

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    pass1_1000_093a(param_1);
  }
  return param_1;
}


pub unsafe fn pass1_1008_5b9a(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1008_57c4(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub unsafe fn pass1_1008_5fa2(mut param_1: u32,param_2: u8) -> u32

{
  pass1_1008_5c34(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

pub unsafe fn FUN_1008_6324() {
    return;
}
pub unsafe fn FUN_1008_6328() {
    return;
}
pub unsafe fn FUN_1008_632c() {
    return;
}


pub unsafe fn pass1_1008_6330(param_1: *mut astruct_456, param_2: u8) {
    let mut uVar1: *mut astruct_456;
    let mut uVar2: u16;

    uVar1 = param_1;
    uVar1 = uVar1 + 1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(uVar1)));
    uVar2 = (param_1 >> 0x10);
    param_1 = 0x380a;
    uVar1.field2_0x2 = 0x1008;
    param_1 = 0x389a;
    uVar1.field2_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn FUN_1008_6814() {
    return;
}
pub unsafe fn FUN_1008_681a() {
    return;
}
pub unsafe fn FUN_1008_681e() {
    return;
}

pub unsafe fn FUN_1008_6822() -> u16 {
    return 0x0;
}
pub unsafe fn FUN_1008_6824() {
    return;
}
pub unsafe fn FUN_1008_6828() {
    return;
}

pub unsafe fn FUN_1008_682e() -> u16 {
    return 0x0;
}


pub unsafe fn pass1_1008_6834(mut param_1: u32, param_2: u8) -> u32 {
    pass1_1008_6732(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_6a4a(
    mut param_1: u32,
    mut param_2: i16,
    mut param_3: u16,
    mut param_4: i16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut puVar3: *mut u8;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut local_e: [u8; 0x4] = [0; 0x4];
    let mut uStack10: u32;
    let mut uStack6: u32;

    if (param_4 == 0x2) {
        iVar2 = param_1;
        pass1_1008_57a4(
            CONCAT22(0x1050, local_e),
            param_1 & 0xffff0000 | (iVar2 + 0xd2),
        );
        loop {
            puVar3 = local_e;
            pass1_1008_5b12(CONCAT22(0x1050, puVar3));
            uStack6 = CONCAT22(extraout_DX, puVar3);
            if ((extraout_DX | puVar3) == 0) {
                break;
            }
            if !((puVar3 + 0x8) != param_2) {
                break;
            }
        }
        if (uStack6 != 0) {
            ppcVar1 = ((iVar2 + 0xd2) + 0xc);
            (**ppcVar1)();
            uStack10 = 0;
            uStack6 = local_e;
            pass1_1008_5b12(CONCAT22(0x1050, uStack6));
            if ((extraout_DX_00 | uStack6) != 0) {
                ppcVar1 = (*(uStack6 + 0x4) + 0x10);
                uStack6 = extraout_DX_00;
                (**ppcVar1)();
                (iVar2 + 0xce) = (uStack6 + 0x4);
                return;
            }
            (iVar2 + 0xce) = 0;
        }
    }
    return;
}


pub unsafe fn pass1_1008_6b02(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (((iVar2 + 0xd0) | (iVar2 + 0xce)) != 0) {
        ppcVar1 = ((iVar2 + 0xce) + 0x6c);
        (**ppcVar1)();
    }
    return;
}
pub unsafe fn pass1_1008_6b2e(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (((iVar2 + 0xd0) | (iVar2 + 0xce)) != 0) {
        ppcVar1 = ((iVar2 + 0xce) + 0x68);
        (**ppcVar1)();
    }
    return;
}
