
pub fn pass1_1008_4016(param_1: *mut astruct_76)

{
  astruct_76 *iVar1;
  let mut uVar1: u16;

  struct_op_1008_56b4(param_1);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_76 *)param_1;
  &iVar1.field3_0x6 = 0x0;
  &iVar1.field5_0xa = 0x0;
  iVar1.field7_0xe = 0x0;
  iVar1.field8_0x10 = 0x0;
  &iVar1.field9_0x14 = 0x0;
  &iVar1.field11_0x18 = 0x0;
  iVar1.field13_0x1c = 0x0;
    // just 0x48de
  param_1.offset_0x0 = &PTR_LOOP_1050_48de;
  iVar1.base_0x2 = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_405c(param_1: *mut astruct_76,mut param_2: u32,mut param_3: i16,mut param_4: i16)

{
  let mut uVar1: u32;
  sqword sVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  i32 lVar5;
  let mut puVar6: *mut u8,
  astruct_76 *iVar4;
  let mut uVar7: u16;
  let mut uStack10: u32;

  struct_op_1008_56b4(param_1);
  uVar7 = (param_1 >> 0x10);
  iVar4 = (astruct_76 *)param_1;
  &iVar4.field3_0x6 = 0x0;
  &iVar4.field5_0xa = 0x0;
  iVar4.field7_0xe = 0x0;
  iVar4.field8_0x10 = 0x0;
  &iVar4.field9_0x14 = 0x0;
  &iVar4.field11_0x18 = 0x0;
  iVar4.field13_0x1c = 0x0;
  param_1.offset_0x0 = &PTR_LOOP_1050_48de;
  iVar4.base_0x2 = 0x1008;
  iVar3 = param_4 * 0x8 + 0x1f;
  uVar4 = ((iVar3 + (iVar3 >> 0xf & 0x1fU)) >> 0x5) << 0x2;
  uStack10 = param_3;
  lVar5 = (long)uVar4 * (long)param_3 + 0x436;
  lVar5 = mem_op_1000_0a48(0x1,lVar5,(lVar5 >> 0x10),_PTR_LOOP_1050_5f2c);
  iVar4.field3_0x6 = lVar5;
  iVar4.field4_0x8 = (lVar5 >> 0x10);
  pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | uVar7 << 0x10));
  iVar4.field11_0x18 = uVar4;
  iVar4.field12_0x1a = uVar4 >> 0xf;
  iVar4.field8_0x10 = 0x28;
  uVar1 = iVar4.field8_0x10;
  *(i32 *)(uVar1 + 0x4) = (long)param_4;
  uVar1 = iVar4.field8_0x10;
  (uVar1 + 0x8) = uStack10;
  uVar1 = iVar4.field8_0x10;
  (uVar1 + 0xc) = 0x1;
  uVar1 = iVar4.field8_0x10;
  (uVar1 + 0xe) = 0x8;
  uVar1 = iVar4.field8_0x10;
  (uVar1 + 0x10) = 0x0;
  sVar2 = (qword)&iVar4.field11_0x18 * (qword)uStack10;
  puVar6 = ((qword)sVar2 >> 0x20);
  uVar1 = iVar4.field8_0x10;
  (uVar1 + 0x14) = (long)sVar2;
  uVar1 = iVar4.field8_0x10;
  (uVar1 + 0x20) = 0x100;
  uVar1 = iVar4.field8_0x10;
  (uVar1 + 0x24) = 0x100;
  pass1_1008_4834(param_1);
  pass1_1008_4d84(puVar6,*(astruct_90 **)&iVar4.field5_0xa,param_2);
  return;
}
pub fn pass1_1008_41bc(param_1: *mut astruct_288)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  astruct_288 *iVar5;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar5 = (astruct_288 *)param_1;
  param_1 = &PTR_LOOP_1050_48de;
  iVar5.field2_0x2 = 0x1008;
  puVar1 = iVar5.field6_0xa;
  uVar2 = iVar5.field7_0xc;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  if (iVar5.field5_0x6 != 0x0) {
    call_fn_ptr_1000_0dc6(iVar5.field5_0x6);
  }
  param_1 = 0x389a;
  iVar5.field2_0x2 = 0x1008;
  return;
}
pub fn struct_op_1008_4214(param_1: *mut astruct_76,param_2: *mut astruct_81)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  astruct_81 *iVar4;
  astruct_81 *uVar4;

  uVar4 = (astruct_81 *)(param_2 >> 0x10);
  iVar4 = (astruct_81 *)param_2;
  (param_1 + 0x6) = iVar4.buffer_0x1a;
  iVar4.buffer_0x1a = 0x0;
  puVar1 = (u32*)&iVar4.field2_0x4;
  uVar2 = (&iVar4.field2_0x4 + 0x2);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  iVar4.field2_0x4 = 0x0;
  iVar4.field6_0xe = 0x0;
  iVar4.field7_0x12 = 0x0;
  iVar4.field8_0x16 = 0x0;
  iVar4.field10_0x1e = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn memcpy_op_1008_4274(mut param_1: u16 ,param_2: *mut astruct_826)

{
  let mut uVar1: u16;
  let mut in_EDX: u32;
  let mut uVar5: u16;
  let mut uVar2: u32;
  astruct_826 *iVar3;
  astruct_827 *uVar3;
  let mut uVar4: u16;
  let mut count: u32;
pub fn *dst;
  astruct_76 *paStack14;
  let mut paVar2: *mut Struct57;

  uVar5 = (in_EDX >> 0x10);
  uVar4 = (param_2 >> 0x10);
  iVar3 = (astruct_826 *)param_2;
  if (iVar3.pvoid32_0x6 != NULL) {
    count = pass1_1000_1284(iVar3.pvoid32_0x6);
    dst = (void *)mem_op_1000_0a48(0x1,count,(count >> 0x10),_PTR_LOOP_1050_5f2c);
    uVar3 = (astruct_827 *)dst;
    uVar1 = (dst >> 0x10) | uVar3;
    paVar2 = (astruct_57 *)CONCAT22(uVar5,uVar1);
    if (uVar1 != 0x0) {
      hmemcpy16(count,iVar3.pvoid32_0x6,dst);
      mem_op_1000_179c(0x1e,paVar2);
      uVar2 = paVar2 | uVar3;
      if (uVar2 == 0x0) {
        uVar3 = NULL;
        uVar2 = 0x0;
      }
      else {
        pass1_1008_4016((astruct_76 *)CONCAT22(paVar2,uVar3));
      }
      paStack14 = (astruct_76 *)CONCAT22(uVar2,uVar3);
      uVar3.field6_0x6 = dst;
      pass1_1008_47cc((astruct_76 *)CONCAT22(uVar2,uVar3));
      pass1_1008_4834(paStack14);
      uVar3.field25_0x1c = 0x1;
      return;
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1008_431c(param_1: *mut astruct_76,param_2: u8)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u32;
  let mut bVar3: bool;
  let mut uVar4: u32;
  astruct_76 *iVar5;
  astruct_76 *uVar5;
  let mut uStack6: u32;

  uVar5 = (astruct_76 *)(param_1 >> 0x10);
  iVar5 = (astruct_76 *)param_1;
  if (*(i32 *)&iVar5.field3_0x6 == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | ZEXT24(uVar5) << 0x10));
  }
  if ((iVar5.field4_0x8 | iVar5.field3_0x6) == 0x0) {
    bVar3 = false;
  }
  else {
    if ((iVar5.field6_0xc | iVar5.field5_0xa) == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | ZEXT24(uVar5) << 0x10));
    }
    bVar3 = true;
  }
  if (bVar3) {
    if ((iVar5.field10_0x16 | iVar5.field9_0x14) == 0x0) {
      return;
    }
    uStack6 = 0x0;
    while( true ) {
      uVar2 = iVar5.field8_0x10;
      puVar1 = (uVar2 + 0x8);
      if (*puVar1 == uStack6 || (long)*puVar1 < (long)uStack6) break;
      uVar4 = uStack6;
      pass1_1008_4544(param_1);
      uVar2 = iVar5.field8_0x10;
      pass1_1000_4906((StructD *)(uVar4 & 0xffff | uStack6 << 0x10),(WNDCLASS16 *)param_2,
                      (uVar2 + 0x4));
      uStack6 += 0x1;
    }
  }
  return;
}



pub fn pass1_1008_43cc(param_1: *mut astruct_76) -> u32

{
  let mut bVar1: bool;
  astruct_76 *iVar2;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_76 *)param_1;
  if (*(i32 *)&iVar2.field3_0x6 == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | uVar2 << 0x10));
  }
  if (*(i32 *)&iVar2.field3_0x6 == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(i32 *)&iVar2.field5_0xa == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | uVar2 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return 0x0;
  }
  return CONCAT22(iVar2.field10_0x16,iVar2.field9_0x14);
}



pub fn pass1_1008_4426(param_1: *mut astruct_76) -> u32

{
  let mut bVar1: bool;
  astruct_76 *iVar2;
  astruct_76 *uVar2;

  uVar2 = (astruct_76 *)(param_1 >> 0x10);
  iVar2 = (astruct_76 *)param_1;
  if (*(i32 *)&iVar2.field3_0x6 == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | ZEXT24(uVar2) << 0x10));
  }
  if (*(i32 *)&iVar2.field3_0x6 == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(i32 *)&iVar2.field5_0xa == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | ZEXT24(uVar2) << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return 0x0;
  }
  return CONCAT22(iVar2.field6_0xc,iVar2.field5_0xa);
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1008_4480(param_1: *mut astruct_76,param_2: *mut u16,param_3: *mut astruct_76)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut iStack26: i16;
  let mut pcStack24: *mut c_char;
  let mut pcStack20: *mut c_char;
  let mut iStack16: i16;
  let mut local_6: i16;
  char local_4 [0x2];

  pass1_1008_3e94(param_2,CONCAT22(0x1050,&local_6),CONCAT22(0x1050,local_4));
  uVar7 = pass1_1008_4772(param_3);
  uVar5 = (uVar7 >> 0x10);
  iVar1 = (uVar7 + 0x4);
  iVar2 = (uVar7 + 0x8);
  for (iStack16 = 0x0; iStack16 < iVar2; iStack16 += 0x1) {
    uVar6 = local_6 >> 0xf;
    iVar3 = local_6;
    local_6 = local_6 + 0x1;
    pass1_1008_4544(param_1);
    pcStack20 = CONCAT22(uVar6,iVar3);
    uVar4 = iStack16;
    pass1_1008_4544(param_3);
    pcStack24 = (uVar4 & 0xffff | uVar6 << 0x10);
    iStack26 = iVar1;
    while (iStack26 != 0x0) {
      if (*pcStack24 != -0x1) {
        *pcStack20 = *pcStack24;
      }
      pcStack24 = CONCAT22((pcStack24 >> 0x10) + (-(0xfffe < pcStack24) & 0x6c),
                                   pcStack24 + 0x1);
      pcStack20 = CONCAT22((pcStack20 >> 0x10) + (-(0xfffe < pcStack20) & 0x6c),
                                   pcStack20 + 0x1);
      iStack26 = iStack26 + -0x1;
    }
  }
  return;
}
pub fn pass1_1008_4544(param_1: *mut astruct_76)

{
  let mut bVar1: bool;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if (*(i32 *)(iVar2 + 0x6) == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | uVar3 << 0x10));
  }
  if (*(i32 *)(iVar2 + 0x6) == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(i32 *)(iVar2 + 0xa) == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | uVar3 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return;
  }
  return;
}



// WARNING: Unable to use type for symbol uVar2
pub fn set_di_bits_to_device_1008_45d6(param_1: *mut astruct_76,INT16 param_2,HDC16 param_3)

{
  let mut bVar1: bool;
  astruct_76 *iVar2;
  BITMAPINFO *info;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut startscan: u16;
  let mut uVar1: u32;
  let mut uVar2: u32;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (astruct_76 *)param_1;
  if (*(i32 *)&iVar2.field3_0x6 == 0x0) {
    pass1_1008_47cc(param_1);
  }
  if ((iVar2.field4_0x8 | iVar2.field3_0x6) == 0x0) {
    bVar1 = false;
  }
  else {
    if ((iVar2.field6_0xc | iVar2.field5_0xa) == 0x0) {
      pass1_1008_4834(param_1);
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return;
  }
  uVar1 = iVar2.field8_0x10;
  uVar4 = (uVar1 >> 0x10);
  info = (BITMAPINFO *)uVar1;
  startscan = &(info.bim_header).biHeight;
  uVar2 = &iVar2.field9_0x14;
  SetDIBitsToDevice(0x0,info,(void *)CONCAT22(uVar2,uVar4),(uVar2 >> 0x10),startscan,0x0,0x0,0x0,
                    startscan,*(INT16 *)&(info.bim_header).biWidth,param_2,param_3);
  return;
}



// WARNING: Unable to use type for symbol uVar1
pub fn stretch_di_bits_1008_465a(param_1: *mut astruct_76,HDC16 hdc_param_2)

{
  INT16 x_src;
  INT16 y_src;
  let mut uVar2: u32;
  let mut bVar3: bool;
  astruct_76 *iVar3;
  BITMAPINFO *info;
  let mut uVar4: u16;
  PVOID bits;
  let mut uVar1: u32;

  uVar4 = (param_1 >> 0x10);
  iVar3 = (astruct_76 *)param_1;
  if (*(i32 *)&iVar3.field3_0x6 == 0x0) {
    pass1_1008_47cc(param_1);
  }
  if ((iVar3.field4_0x8 | iVar3.field3_0x6) == 0x0) {
    bVar3 = false;
  }
  else {
    if ((iVar3.field6_0xc | iVar3.field5_0xa) == 0x0) {
      pass1_1008_4834(param_1);
    }
    bVar3 = true;
  }
  if (!bVar3) {
    return;
  }
  uVar1 = iVar3.field8_0x10;
  bits = (PVOID)(uVar1 >> 0x10);
  info = (BITMAPINFO *)uVar1;
  x_src = *(INT16 *)&(info.bim_header).biWidth;
  y_src = *(INT16 *)&(info.bim_header).biHeight;
  uVar2 = &iVar3.field9_0x14;
  StretchDIBits16(0xcc0020,0x0,info,bits,(INT16)uVar2,(INT16)(uVar2 >> 0x10),y_src,x_src,0x0,0x0,y_src,x_src,
                  hdc_param_2);
  return;
}



u16 palette_op_1008_46e4(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_76,HDC16 *param_4)

{
  let mut bVar1: bool;
  let mut uVar2: u16;
  HPALETTE16 HVar2;
  let mut uVar5: u16;
  astruct_76 *struct_var3;
  let mut uVar4: u16;
  let mut uVar3: u32;

  uVar4 = (param_3 >> 0x10);
  struct_var3 = (astruct_76 *)param_3;
  if (*(i32 *)&struct_var3.field3_0x6 == 0x0) {
    uVar5 = param_2;
    pass1_1008_47cc((astruct_76 *)(param_3 & 0xffff | uVar4 << 0x10));
    param_2 = uVar5;
  }
  uVar3 = CONCAT22(param_2,param_1);
  if (*(i32 *)&struct_var3.field3_0x6 == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(i32 *)&struct_var3.field5_0xa == 0x0) {
      uVar3 = pass1_1008_4834((astruct_76 *)(param_3 & 0xffff | uVar4 << 0x10));
    }
    bVar1 = true;
  }
  uVar2 = uVar3;
  if (!bVar1) {
    return 0x0;
  }
  create_palette_1008_4e38(*(astruct_13 **)&struct_var3.field5_0xa,(uVar3 >> 0x10));
  struct_var3.field7_0xe = uVar2;
  HVar2 = SelectPalette16(0x0,struct_var3.field7_0xe,*param_4);
  struct_var3.field2_0x4 = HVar2;
  RealizePalette16(*param_4);
  return struct_var3.field2_0x4;
}



pub fn pass1_1008_4772(param_1: *mut astruct_76) -> u32

{
  let mut bVar1: bool;
  astruct_76 *iVar2;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_76 *)param_1;
  if (*(i32 *)&iVar2.field3_0x6 == 0x0) {
    pass1_1008_47cc((astruct_76 *)(param_1 & 0xffff | uVar2 << 0x10));
  }
  if (*(i32 *)&iVar2.field3_0x6 == 0x0) {
    bVar1 = false;
  }
  else {
    if (*(i32 *)&iVar2.field5_0xa == 0x0) {
      pass1_1008_4834((astruct_76 *)(param_1 & 0xffff | uVar2 << 0x10));
    }
    bVar1 = true;
  }
  if (!bVar1) {
    return 0x0;
  }
  return CONCAT22((&iVar2.field8_0x10 + 0x2),&iVar2.field8_0x10);
}
pub fn pass1_1008_47cc(param_1: *mut astruct_76)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut uVar4: u16;
  astruct_76 *iVar5;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uStack14: u32;

  uVar7 = (param_1 >> 0x10);
  iVar5 = (astruct_76 *)param_1;
  if (*(i32 *)&iVar5.field3_0x6 != 0x0) {
    uVar2 = &iVar5.field3_0x6;
    uVar1 = iVar5.field4_0x8;
    iVar6 = uVar2;
    uVar4 = iVar6 + 0xe;
    iVar5.field8_0x10 = uVar2 & 0xffff0000 | uVar4;
    iVar5.field9_0x14 = iVar6 + 0x436;
    iVar5.field10_0x16 = uVar1 + (-(0xfbd7 < uVar4) & 0x6c);
    uVar3 = iVar5->field8_0x10;
    uVar8 = (uVar3 >> 0x10);
    iVar6 = uVar3;
    uStack14 = (iVar6 + 0xe);
    *(i32 *)&iVar5->field11_0x18 = (long)(uStack14 * *(i32 *)(iVar6 + 0x4) + 0x1f) / 0x20 << 0x2;
  }
  return;
}
pub fn pass1_1008_4834(param_1: *mut astruct_76)

{
  code **ppcVar1;
  let mut puVar2: *mut u32;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut in_EDX: u32;
  let mut paVar5: *mut Struct57;
  astruct_76 *struct_var5_1;
  astruct_76 *struct_var5;
  astruct_76 *paStack10;

  struct_var5 = (astruct_76 *)(param_1 >> 0x10);
  struct_var5_1 = (astruct_76 *)param_1;
  puVar2 = struct_var5_1->field5_0xa;
  uVar4 = struct_var5_1->field6_0xc;
  paVar5 = (astruct_57 *)(in_EDX & 0xffff0000 | uVar4);
  if ((uVar4 | puVar2) != 0x0) {
    ppcVar1 = (code **)*puVar2;
    (**ppcVar1)();
  }
  mem_op_1000_179c(0x14,paVar5);
  paStack10 = (astruct_76 *)CONCAT22(paVar5,puVar2);
  uVar4 = paVar5 | puVar2;
  if (uVar4 != 0x0) {
    uVar3 = struct_var5_1->field8_0x10;
    uVar3 = uVar3 & 0xffff0000 | (uVar3 + 0x28);
    struct_op_1008_4c98(paStack10,uVar3,0x100);
    struct_var5_1->field5_0xa = uVar3;
    struct_var5_1->field6_0xc = uVar4;
    return;
  }
  &struct_var5_1->field5_0xa = 0x0;
  return;
}



u16 pass1_1008_48aa(mut param_1: u32)

{
  return (param_1 + 0xe);
}



StructD * pass1_1008_48b8(StructD *param_1,param_2: u8)

{
  pass1_1008_41bc((astruct_288 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Instruction at (ram,0x10084942) overlaps instruction at (ram,0x10084941)
//
pub fn pass1_1008_48de(param_1: *mut u16,mut param_2: i16,char param_3,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u32,mut param_7: u16 ,
                    uchar param_8)

{
  let mut pbVar1: *mut u8,
  let mut uVar2: u32;
  u8 bVar3;
  let mut uVar4: u16;
  u8 bVar5;
  let mut uVar6: u16;
  let mut unaff_BP: i16;
  let mut puVar7: *mut u8,
  let mut unaff_SI: i16;
  let mut iVar8: i16;
  let mut unaff_DI: *mut u8,
  let mut unaff_ES: u16;
  let mut uVar9: u16;

  uVar6 = param_4 & 0xff | (u8)((char)(param_4 >> 0x8) + (char)param_4 + param_3) << 0x8;
  puVar7 = (unaff_BP + 0x1);
  pbVar1 = (param_1 + unaff_SI);
  bVar5 = (u8)(param_4 & 0xff);
  *pbVar1 = *pbVar1 | bVar5;
  bVar3 = in(0x46);
  pbVar1 = (param_1 + unaff_SI);
  *pbVar1 = *pbVar1 | bVar5;
  if (param_2 == 0x1) {
    pbVar1 = (param_1 + unaff_SI);
    *pbVar1 = *pbVar1 | bVar5;
    iVar8 = unaff_SI + 0x1;
    pbVar1 = (param_1 + iVar8);
    bVar5 = (u8)param_7;
    *pbVar1 = *pbVar1 | bVar5;
    pbVar1 = (param_1 + iVar8);
    *pbVar1 = *pbVar1 | bVar5;
    *unaff_DI = bVar3;
    pbVar1 = (param_1 + iVar8);
    *pbVar1 = *pbVar1 | bVar5;
    uVar6 = param_7;
    if (*pbVar1 != 0x0) {
      pbVar1 = (param_1 + iVar8);
      *pbVar1 = *pbVar1 | bVar5;
      puVar7 = (&param_7 + 0x1);
      param_1 = (param_6 >> 0x8);
      CONCAT13(param_8,param_6._1_3_) = 0x389a;
      param_1[0x1] = 0x1008;
      unaff_ES = (CONCAT13(param_8,param_6._1_3_) >> 0x10);
      (param_1 + 0x2) = 0x0;
      (param_1 + 0x4) = 0x0;
      param_1[0x6] = 0xffff;
      (param_1 + 0x7) = 0x0;
      (param_1 + 0x9) = 0x0;
      (param_1 + 0xb) = 0x0;
      (param_1 + 0xd) = 0x0;
      param_1[0xf] = 0x0;
    }
  }
  else {
    param_1[0x11] = bVar3 | 0x800;
  }
  param_1[0x11] = (puVar7 + 0xa);
  *param_1 = &u16_1050_4c4c;
  param_1[0x1] = 0x1008;
  uVar4 = str_op_1008_60e8(uVar6,*(char **)(puVar7 + 0xc));
  uVar2 = (puVar7 + 0x6);
  uVar9 = (uVar2 >> 0x10);
  iVar8 = uVar2;
  (iVar8 + 0x8) = uVar4;
  (iVar8 + 0xa) = uVar6;
  return;
}
pub fn struct_op_1008_48fe(param_1: *mut astruct_57,param_2: *mut astruct_81,mut param_3: u16 ,char *param_4)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  astruct_81 *pstruct81_2;
  let mut uVar3: u16;

  uVar2 = param_1;
  uVar3 = (param_2 >> 0x10);
  pstruct81_2 = (astruct_81 *)param_2;
  param_2->field0_0x0 = 0x389a;
  pstruct81_2->field1_0x2 = 0x1008;
  pstruct81_2->field2_0x4 = 0x0;
  &pstruct81_2->field3_0x8 = 0x0;
  pstruct81_2->hfile_0xc = 0xffff;
  pstruct81_2->field6_0xe = 0x0;
  pstruct81_2->field7_0x12 = 0x0;
  pstruct81_2->field8_0x16 = 0x0;
  pstruct81_2->buffer_0x1a = 0x0;
  pstruct81_2->field10_0x1e = 0x0;
  pstruct81_2->field13_0x22 = param_3;
    // just 0x4c4c
  param_2->field0_0x0 = &u16_1050_4c4c;
  pstruct81_2->field1_0x2 = 0x1008;
  uVar1 = str_op_1008_60e8(uVar2,param_4);
  pstruct81_2->field3_0x8 = uVar1;
  pstruct81_2->field4_0xa = uVar2;
  return;
}
pub fn close_file_1008_496c(param_1: *mut astruct_803)

{
  code **ppcVar1;
  astruct_803 *iVar5;
  let mut uVar2: u16;
  let mut puVar1: *mut u32;
  let mut uVar1: u16;
  i32 lVar1;

  uVar2 = (param_1 >> 0x10);
  iVar5 = (astruct_803 *)param_1;
  param_1->offset_0x0 = &u16_1050_4c4c;
  iVar5->base_0x2 = 0x1008;
  puVar1 = iVar5->field2_0x4;
  uVar1 = iVar5->field3_0x6;
  if ((uVar1 | puVar1) != 0x0) {
    ppcVar1 = (code **)*puVar1;
    (**ppcVar1)();
  }
  fn_ptr_1000_17ce(iVar5->field4_0x8);
  if (iVar5->field18_0x1a != 0x0) {
    call_fn_ptr_1000_0dc6(iVar5->field18_0x1a);
  }
  if (iVar5->field5_0xc != 0xffff) {
    _lclose16(iVar5->field5_0xc);
  }
  param_1->offset_0x0 = 0x389a;
  iVar5->base_0x2 = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 read_file_1008_49e8(HFILE16 param_1,mut param_2: u16 ,astruct_81 *struct_param_1,mut param_4: u32)

{
  let mut uVar4: u32;
  HFILE16 hfile_1;
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar1: u32;
  let mut lVar5: i16;
  let mut puVar5: *mut u8,
  let mut extraout_DX: *mut u8,
  let mut uVar8: u16;
  let mut paVar7: *mut Struct57;
  astruct_81 *struct_1;
  let mut unaff_DI: i16;
  let mut uVar5: u16;
  let mut unaff_SS: u16;
  i32 iVar9;
  i32 lVar10;
  let mut uStack20: u16;
  let mut uStack10: u16;
  let mut uStack8: u16;
  i32 iStack6;
  let mut in_stack_0000ffe8: i16;
  let mut uVar3: u32;
  let mut in_stack_0000ffea: u16;
  let mut in_stack_0000ffe8_00: u16;
  let mut uVar6: u32;

  uVar8 = (param_4 >> 0x10);
  uVar5 = (struct_param_1 >> 0x10);
  struct_1 = (astruct_81 *)struct_param_1;
  if (*(i32 *)&struct_1->field3_0x8 != 0x0) {
    if (struct_1->field10_0x1e != 0x0) {
      return 0x1;
    }
    if (struct_1->hfile_0xc == -0x1) {
      hfile_1 = _lopen16(0x0,*(char **)&struct_1->field3_0x8);
      struct_1->hfile_0xc = hfile_1;
      if (hfile_1 == 0xffff) {
        return 0x0;
      }
    }
    iStack6 = 0x0;
    iVar9 = WIN16_hread(0xe,(void *)CONCAT22(0x1050,&param_1),struct_1->hfile_0xc);
    if (((iVar9 == 0xe) && ((iVar9 >> 0x10) == 0x0)) &&
       (iStack6 = CONCAT22(uStack20,param_2), param_1 == &PTR_LOOP_1050_4d42)) {
      _llseek16(0x0,0x0,struct_1->hfile_0xc);
      lVar10 = mem_op_1000_0a48(0x1,iStack6,(iStack6 >> 0x10),_PTR_LOOP_1050_5f2c);
      lVar5 = (i16)(lVar10 >> 0x10);
      &struct_1->buffer_0x1a = lVar10;
      (&struct_1->buffer_0x1a + 0x2) = lVar5;
      if ((lVar5 | &struct_1->buffer_0x1a) != 0x0) {
        iVar9 = WIN16_hread(iStack6,(void *)struct_1->buffer_0x1a,struct_1->hfile_0xc);
        uStack8 = (iVar9 >> 0x10);
        paVar7 = (astruct_57 *)CONCAT22(uVar8,uStack8);
        uStack10 = iVar9;
        param_1 = struct_1->hfile_0xc;
        _lclose16(param_1);
        struct_1->hfile_0xc = 0xffff;
        struct_1->field10_0x1e = 0x1;
        struct_1->field6_0xe = struct_1->buffer_0x1a;
        uVar3 = struct_1->buffer_0x1a;
        iVar1 = (i16)uVar3;
        iVar1 = iVar1 + 0xe;
        struct_1->field7_0x12 = uVar3 & 0xffff0000 | iVar1;
        uVar1 = iVar1 + 0x436;
        uVar1 = uVar3 & 0xffff0000 | uVar1;
        struct_1->field8_0x16 = uVar1;
        param_2 = 0x14;
        param_1 = (HFILE16)s_tile2_bmp_1050_1538;
        mem_op_1000_179c(0x14,paVar7);
        puVar5 = (paVar7 | uVar1);
        extraout_DX = puVar5;
        if (puVar5 == NULL) {
          struct_1->field2_0x4 = 0x0;
        }
        else {
          param_2 = 0x100;
          uVar4 = struct_1->field7_0x12;
          uVar2 = uVar4;
          uVar2 = uVar2 + 0x28;
          uVar4 &= 0xffff0000;
          uVar6 = uVar4 | uVar2;
          param_1 = (HFILE16)(uVar4 >> 0x10);
          struct_op_1008_4c98((astruct_76 *)(uVar1 & 0xffff | uVar2 << 0x10),uVar6,0x100);
          &struct_1->field2_0x4 = uVar6;
          *(u8 **)(&struct_1->field2_0x4 + 0x2) = extraout_DX;
        }
        if (struct_1->field13_0x22 == 0x0) {
          return 0x1;
        }
        _param_1 = struct_param_1;
        pass1_1008_4b8e(extraout_DX,(astruct_807 *)struct_param_1);
        return 0x1;
      }
    }
    else {
      _lclose16(struct_1->hfile_0xc);
      struct_1->hfile_0xc = 0xffff;
    }
  }
  return 0x0;
}



pub fn pass1_1008_4b5e(u32 *param_1) -> u32

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if ((iVar3 + 0x1e) == 0x0) {
    ppcVar1 = (code **)(*param_1 + 0x8);
    iVar2 = (**ppcVar1)();
    if (iVar2 == 0x0) {
      return 0x0;
    }
  }
  return CONCAT22((iVar3 + 0x6),(iVar3 + 0x4));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_4b8e(u8 *param_1,param_2: *mut astruct_807)

{
  let mut uVar1: u32;
  let mut in_register_0000000a: u16;
  let mut uVar2: u16;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffe4: u32;
  let mut iStack18: i16;
  let mut iStack16: i16;
  let mut iStack10: i16;

  puVar3 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22((in_stack_0000ffe4 >> 0x10),0x48),in_stack_0000fe8e,
                           in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  uVar2 = (puVar3 >> 0x10);
  uVar1 = (puVar3 + 0x18);
  iStack18 = (puVar3 + 0x16) / 0x2;
  for (iStack16 = 0x0; iStack10 = uVar1, uVar2 = (param_2 >> 0x10), iStack16 < iStack18;
      iStack16 += 0x1) {
    pass1_1008_4d26(*(astruct_650 **)(param_2 + 0x4),
                    (uVar1 & 0xffff0000 | (iStack16 * 0x4 + iStack10)),iStack16);
  }
  for (iStack18 = 0x100 - iStack18; iStack18 < 0x100; iStack18 += 0x1) {
    pass1_1008_4d26(*(astruct_650 **)(param_2 + 0x4),
                    (uVar1 & 0xffff0000 | (iStack16 * 0x4 + iStack10)),iStack18);
    iStack16 += 0x1;
  }
  return;
}



astruct_803 * file_1008_4c26(param_1: *mut astruct_803,param_2: u8)

{
  close_file_1008_496c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_1008_4c58(param_1: *mut astruct_394)

{
  astruct_394 *iVar1;
  let mut in_stack_00000006: u16;

  *_param_1 = 0x389a;
  param_1->field2_0x2 = 0x1008;
  param_1->field3_0x4 = 0x0;
  param_1->field8_0xc = 0x0;
  param_1->field9_0xe = 0x0;
  param_1->field10_0x12 = 0x1;
  *_param_1 = 0x4f1c;
  param_1->field2_0x2 = 0x1008;
  return;
}
pub fn struct_op_1008_4c98(param_1: *mut astruct_76,mut param_2: u32,mut param_3: u16 )

{
  astruct_76 *iVar1;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_76 *)param_1;
  param_1->offset_0x0 = 0x389a;
  iVar1->base_0x2 = 0x1008;
  &iVar1->field2_0x4 = param_2;
  iVar1->field6_0xc = param_3;
  &iVar1->field7_0xe = 0x0;
  (&iVar1->field8_0x10 + 0x2) = 0x0;
  param_1->offset_0x0 = 0x4f1c;
  iVar1->base_0x2 = 0x1008;
  return;
}
pub fn pass1_1008_4cdc(param_1: *mut astruct_454)

{
  astruct_454 *iVar2;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar2 = (astruct_454 *)param_1;
  param_1 = 0x4f1c;
  iVar2->field2_0x2 = 0x1008;
  fn_ptr_1000_17ce(iVar2->field10_0xe);
  if (iVar2->field11_0x12 != 0x0) {
    fn_ptr_1000_17ce(iVar2->field3_0x4);
  }
  param_1 = 0x389a;
  iVar2->field2_0x2 = 0x1008;
  return;
}



u16 pass1_1008_4d26(param_1: *mut astruct_650,param_2: *mut u16,mut param_3: i16)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  i32 lVar3;
  astruct_650 *iVar5;
  astruct_649 *iVar4;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar5 = (astruct_650 *)param_1;
  if (((iVar5->field4_0x4 != 0x0) && (-0x1 < param_3)) &&
     (piVar1 = &iVar5->field9_0xc, *piVar1 != param_3 && param_3 <= *piVar1)) {
    uVar2 = (param_2 + 0x2);
    lVar3 = iVar5->field4_0x4;
    uVar4 = (lVar3 >> 0x10);
    iVar4 = (astruct_649 *)lVar3;
    (iVar4 + param_3 * 0x4) = *param_2;
    (iVar4 + param_3 * 0x4 + 0x2) = uVar2;
    return 0x1;
  }
  return 0x0;
}



pub fn pass1_1008_4d72(mut param_1: u32) -> u32

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x6),(param_1 + 0x4));
}
pub fn pass1_1008_4d84(u8 *param_1,param_2: *mut astruct_90,mut param_3: u32)

{
  let mut iVar1: i16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  astruct_90 *iVar3;
  let mut uVar3: u16;
  let mut uVar4: u16;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar3 = (param_2 >> 0x10);
  iVar3 = (astruct_90 *)param_2;
  uVar4 = (param_3 >> 0x10);
  if (iVar3->field14_0x12 != 0x0) {
    iVar3->field9_0xc = (param_3 + 0xc);
    fn_ptr_1000_17ce(iVar3->field4_0x4);
    iVar3->field4_0x4 = 0x0;
    iVar1 = iVar3->field9_0xc << 0x2;
    mem_op_1000_179c(iVar1,paVar2);
    &iVar3->field4_0x4 = iVar1;
    (&iVar3->field4_0x4 + 0x2) = paVar2;
  }
  if (iVar3->field9_0xc != 0x100) {
    return;
  }
  pass1_1000_48a8(iVar3->field4_0x4,(param_3 + 0x4),0x400);
  return;
}



HPALETTE16 palette_op_1008_4e08(HPALETTE16 hpal_param_2,mut param_2: u16 ,param_3: *mut astruct_13,HDC16 *phdc_param_2)

{
  HDC16 hdc_1;

  hdc_1 = *phdc_param_2;
  create_palette_1008_4e38(param_3,param_2);
  SelectPalette16(0x0,hpal_param_2,hdc_1);
  hdc_1 = *phdc_param_2;
  RealizePalette16(hdc_1);
  return hdc_1;
}



// WARNING: Unable to use type for symbol uVar3
pub fn create_palette_1008_4e38(astruct_13 *in_struct_1,mut param_2: u16 )

{
  let mut piVar1: *mut i16;
  LOGPALETTE *pLVar2;
  let mut iVar3: i16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  astruct_13 *local_struct_1;
  let mut iVar5: i16;
  astruct_13 *uVar8;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut iStack14: i16;
  UCHAR *puStack12;
  UCHAR *puStack8;
  LOGPALETTE *uVar3;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar8 = (astruct_13 *)(in_struct_1 >> 0x10);
  local_struct_1 = (astruct_13 *)in_struct_1;
  iVar3 = (local_struct_1->field9_0xc + 0x2) * 0x4;
  if (local_struct_1->field10_0xe == NULL) {
    mem_op_1000_179c(iVar3,paVar4);
    &local_struct_1->field10_0xe = iVar3;
    (&local_struct_1->field10_0xe + 0x2) = paVar4;
    local_struct_1->field10_0xe->pal_version = 0x300;
    uVar3 = local_struct_1->field10_0xe;
    (uVar3 + 0x2) = local_struct_1->field9_0xc;
    pLVar2 = local_struct_1->field10_0xe;
    puStack8 = (UCHAR *)(pLVar2 & 0xffff0000 | (pLVar2 + 0x4));
    puStack12 = local_struct_1->field4_0x4;
    iStack14 = 0x0;
    while( true ) {
      piVar1 = &local_struct_1->field9_0xc;
      if (*piVar1 == iStack14 || *piVar1 < iStack14) break;
      uVar9 = (puStack12 >> 0x10);
      iVar3 = puStack12;
      *puStack8 = *(UCHAR *)(iVar3 + 0x2);
      uVar10 = (puStack8 >> 0x10);
      iVar5 = puStack8;
      *(iVar5 + 0x1) = *(iVar3 + 0x1);
      *(UCHAR *)(iVar5 + 0x2) = *puStack12;
      *(iVar5 + 0x3) = 0x0;
      iStack14 += 0x1;
      puStack8 = (UCHAR *)(puStack8 & 0xffff0000 | (iVar5 + 0x4));
      puStack12 = (UCHAR *)(puStack12 & 0xffff0000 | (iVar3 + 0x4));
    }
  }
  CreatePalette16((LOGPALETTE *)local_struct_1->field10_0xe);
  return;
}



StructD * pass1_1008_4ef6(StructD *param_1,param_2: u8)

{
  pass1_1008_4cdc((astruct_454 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn file_and_draw_op_1008_4f20
               (param_1: *mut astruct_57,param_2: *mut astruct_76,mut param_3: u32,mut param_4: u16 ,char *param_5,mut param_6: u16 ,
               mut param_7: u16 )

{
  let mut uVar1: u16;
  HDC16 hdc;
  let mut uVar2: u16;
  let mut count: i16;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  HPALETTE16 hpalette_a;
  astruct_76 *struct_1;
  let mut uVar6: u16;
  let mut unaff_CS: u16;
  let mut uVar3: u32;
  COLORREF color;
  astruct_81 struct81_26;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut offset: u16;
  let mut segment: u16;
  let mut hdc_b: u16;
  u8 uVar14;
  u8 uVar15;
  struct uVar17;
  HDC16 hdc_a;

  pass1_1008_4016(param_2);
  uVar6 = (param_2 >> 0x10);
  struct_1 = (astruct_76 *)param_2;
  struct_1->lpcstr_field13_0x1e = param_5;
  struct_1->field15_0x22 = param_4;
  struct_1->field16_0x24 = param_3;
    // 0x50a2
  param_2->offset_0x0 = s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x9;
  struct_1->base_0x2 = 0x1008;
  uVar1 = FUN_1010_830a(param_3,param_1,unaff_CS,_u16_1050_14cc,0x2);
  struct_op_1008_48fe(param_1,(astruct_81 *)CONCAT22(0x1050,&struct81_26),0x1,CONCAT22(param_1,uVar1));
  read_file_1008_49e8(param_6,param_7,(astruct_81 *)CONCAT22(0x1050,&struct81_26),param_1);
  pass1_1008_5068(param_2,(astruct_81 *)CONCAT22(0x1050,&struct81_26));
  pass1_1008_47cc(param_2);
  pass1_1008_4834(param_2);
  segment = &DAT_1050_1050;
  offset = 0x27e;
  uVar10 = 0x0;
  uVar11 = 0x0;
  uVar8 = 0x0;
  uVar9 = 0x0;
  uVar3 = pass1_1008_4772(param_2);
  uVar4 = (uVar3 >> 0x10);
  hdc = CreateDC16((DEVMODEA *)(uVar3 & 0xffff | uVar4 << 0x10),CONCAT22(uVar9,uVar8),
                   CONCAT22(uVar11,uVar10),CONCAT22(segment,offset));
  uVar2 = palette_op_1008_46e4(&stack0xffd4,uVar4,param_2,(HDC16 *)CONCAT22(0x1050,&stack0xffd4));
  color = SetBkColor16(0xffffff,hdc);
  SetTextColor16(CONCAT22(0x100,struct_1->field15_0x22),hdc);
  count = str_op_1000_3da4(struct_1->lpcstr_field13_0x1e);
  TextOut16(count,struct_1->lpcstr_field13_0x1e,0x0,0x0,hdc);
  uVar1 = (color >> 0x10);
  hdc_a = hdc;
  SetBkColor16(color,hdc);
  SetTextColor16(CONCAT22(hdc,uVar1),hdc_a);
  hpalette_a = SelectPalette16(0x0,uVar2,hdc_a);
  DeleteObject16(hpalette_a);
  DeleteDC16(hdc_a);
  close_file_1008_496c((astruct_803 *)CONCAT22(0x1050,&struct81_26));
  return;
}
pub fn pass1_1008_5068(param_1: *mut astruct_76,param_2: *mut astruct_81)

{
  struct_op_1008_4214(param_1,param_2);
  return;
}
