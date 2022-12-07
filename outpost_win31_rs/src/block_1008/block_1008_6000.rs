

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn debug_print_1008_6048(mut param_1: u16 ,param_2: *mut c_char)

{
  let mut uVar1: u16;
  let mut local_106: [u8;0x100] = [0;0x100];

  if (PTR_LOOP_1050_02ec.is_null() == false) {
    if (DAT_1050_02ee == 0xffff) {
      uVar1 = pass1_1000_3ec0(0x2f4,&DAT_1050_1050);
      DAT_1050_02ee = ((param_1 | uVar1) != 0);
    }
    if (DAT_1050_02ee != 0) {
      wvsprintf16(&stack0x0008,CONCAT22(param_2,0x1050),
                  CONCAT22(local_106,(param_2 >> 0x10)));
      OutputDebugString16(CONCAT22(0x1050,local_106));
      OutputDebugString16(s__1050_02fa);
      if (_PTR_LOOP_1050_02f0 != 0) {
        pass1_1000_2b5c(_PTR_LOOP_1050_02f0,(_PTR_LOOP_1050_02f0 >> 0x10),0x2fd,
                        &DAT_1050_1050);
        pass1_1000_2f48(_PTR_LOOP_1050_02f0);
      }
    }
  }
  return;
}



pub unsafe fn str_op_1008_60e8(mut param_1: u16 ,param_2: *mut c_char) -> u16

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  if (param_2.is_null() == false) {
    uVar1 = str_op_1000_3da4(param_2);
    uVar1 += 0x1;
    mem_op_1000_179c(uVar1,paVar2);
    if ((paVar2 | uVar1) != 0) {
      unk_str_op_1000_3d3e(CONCAT22(paVar2,uVar1),param_2);
      return uVar1;
    }
  }
  return 0x0;
}
pub fn pass1_1008_612e(mut param_1: u16 ,mut param_2: i16,mut param_3: i16)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut lVar3: i32;
  let mut iVar4: i16;
  let mut iStack18: i16;
  let mut iStack16: i16;

  uVar1 = pass1_1000_4d24();
  uVar2 = (param_3 - param_2) + 1;
  if ((uVar2 >> 0xf | uVar2) == 0) {
    return;
  }
  iStack16 = 0x1;
  iStack18 = param_2;
  loop {
    if (param_3 < iStack18) {
      return;
    }
    lVar3 = iStack16 * (0x7fff / uVar2);
    iVar4 = (lVar3 >> 0x10);
    if (uVar1 >> 0xf <= iVar4) {
      if (uVar1 >> 0xf < iVar4) {
        return;
      }
      if (uVar1 <= lVar3) {
        return;
      }
    }
    iStack18 += 0x1;
    iStack16 += 0x1;
  }
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn unk_draw_op_1008_61b2(mut param_1: u16 ,param_2: *mut StructA,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32) -> *mut astruct_20

{
  let mut l_hgdiobj_1: HGDIOBJ16;
  let mut l_hcursor_1: HCURSOR16;
  let mut in_EDX: *mut Struct57;
  let mut iVar2: *mut astruct_20;
  let mut uVar2: u16;
  let mut l_struct_2: *mut u16;
  let mut in_stack_0000fe90: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffbe: u16;
  let mut uVar5: *mut astruct_20;
  let mut in_stack_0000ffe8: u16;
  let mut iVar1: *mut astruct_20;
  let mut iVar4: *mut astruct_20;
  let mut uVar1: *mut u16;

  set_struct_1008_687a(param_2,param_5);
  uVar2 = (param_2 >> 0x10);
  iVar2 = param_2;
  iVar2.field164_0xde = param_3;
  iVar2.field165_0xe0 = 0;
  param_2.field0_0x0 = 0x6378;
  iVar2.base_0x2 = 0x1008;
  unk_str_op_1000_3d3e((param_2 & 0xffff0000 | ZEXT24(&iVar2.field60_0x5b)),s_DanBrotherton_1050_0302);
  l_hgdiobj_1 = GetStockObject16(BLACK_BRUSH);
  iVar2.hgdiobj_field_0xc6 = l_hgdiobj_1;
  l_hcursor_1 = LoadCursor16(0x7f00,0x0);
  iVar2.hcursor_field_0xc4 = l_hcursor_1;
  iVar2.field150_0xc8 = 0x200b;
  iVar2.field139_0xac = 0x45000000;
  iVar2.field145_0xbc = (param_5 + 0x8);
  l_struct_2 =
               mixed_1010_20ba(in_EDX,_u16_1050_0ed0,CONCAT22(param_1,0x48),in_stack_0000fe90,
                               in_stack_0000ffb4,in_stack_0000ffba,in_stack_0000ffbe);
  uVar1 = (l_struct_2 >> 0x10);
  iVar2.field141_0xb4 = 0;
  iVar2.field142_0xb6 = 0;
  iVar2.field143_0xb8 = (l_struct_2 + 0xa);
  iVar2.field144_0xba = (l_struct_2 + 0xc);
  iVar2.field151_0xca = param_4;
  win_ui_reg_class_1008_96d2(param_2);
  return param_2;
}

pub unsafe fn destroy_win_1008_628e(mut param_1: u32)

{
  let mut fn_ptr_1: *mut *mut code;

  fn_ptr_1 = ((param_1 + 0xd2) + 0x14);
  (**fn_ptr_1)();
  DestroyWindow16((param_1 + 0x8));
  (param_1 + 0x8) = 0;
  return;
}
pub fn fill_rect_1008_62c0(param_1: *mut astruct_838,mut param_2: u16 )

{
//   RECT16 rect_2e [0x2];
let mut rect_2e: [RECt16;2] = [RECT16::default();2];
let mut hbrush_var38: HBRUSH16;
  let mut hbrush_var36: HDC16;
  let mut paintstruct_22: [u8;0x20] = [0;0x20];

  hbrush_var36 = BeginPaint16(CONCAT22(0x1050,paintstruct_22),param_1.field8_0x8);
  hbrush_var38 = CreateSolidBrush16(0x210070b);
  GetClientRect16(rect_2e,&DAT_1050_1050);
  FillRect16(hbrush_var38,rect_2e,(HDC16)&DAT_1050_1050);
  EndPaint16(CONCAT22(0x1050,paintstruct_22),param_1.field8_0x8);
  DeleteObject16(hbrush_var38);
  return;
}
pub fn FUN_1008_6324()

{
  return;
}
pub fn FUN_1008_6328()

{
  return;
}
pub fn FUN_1008_632c()

{
  return;
}
pub fn pass1_1008_6330(param_1: *mut astruct_456,param_2: u8)

{
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
pub unsafe fn file_1008_6414(param_1: *mut astruct_57,param_2: *mut u32,param_3: *mut c_char)

{
  let mut ppcVar1: *mut *mut code;
  let mut paVar2: *mut astruct_76;
  let mut pstruct81_3: *mut astruct_81;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut string_26: astruct_81;

  uVar5 = (param_2 >> 0x10);
  iVar4 = param_2;
  *param_2 = 0;
  (iVar4 + 0x4) = 0;
  pstruct81_3 = &string_26;
  struct_op_1008_48fe(param_1,CONCAT13(0x10,CONCAT12(0x50,pstruct81_3)),0x1,param_3);
  mem_op_1000_179c(0x1e,param_1);
  uVar3 = param_1 | pstruct81_3;
  if (uVar3 == 0) {
    *param_2 = 0;
  }
  else {
    paVar2 = CONCAT22(param_1,pstruct81_3);
    pstruct81_3 = &string_26;
    struct_op_1008_3f92(paVar2,CONCAT22(0x1050,&string_26));
    param_2 = pstruct81_3;
    (iVar4 + 0x2) = uVar3;
  }
  ppcVar1 = (*param_2 + 0x14);
  (**ppcVar1)(0x1000,*param_2);
  (iVar4 + 0x4) = pstruct81_3;
  (iVar4 + 0x6) = uVar3;
  close_file_1008_496c(CONCAT22(0x1050,&string_26));
  return;
}

pub unsafe fn pass1_1008_64a2(param_1: *mut u16)

{
  let mut uVar1: u16;
  let mut ppcVar2: *mut *mut code;

  uVar1 = (param_1 + 2);
  if ((uVar1 | *param_1) != 0) {
    ppcVar2 = *param_1;
    (**ppcVar2)();
  }
  return;
}

pub unsafe fn pass1_1008_64c8(mut param_1: u16 ,param_2: *mut u8,param_3: *mut u32,mut param_4: u32,mut param_5: i16)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut iVar5: i16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut in_stack_0000ffe6: u16;
  let mut iStack8: i16;
  let mut paStack6: *mut astruct_76;

  paVar6 = CONCAT22(in_register_0000000a,param_2);
  if (*param_3 == 0) {
    return;
  }
  mem_op_1000_179c(0x1e,paVar6);
  uVar3 = paVar6 | param_1;
  if (uVar3 == 0) {
    param_1 = 0;
    uVar3 = 0;
  }
  else {
    struct_op_1008_6604(CONCAT22(paVar6,param_1),param_4,
                        CONCAT22(in_stack_0000ffe6,(param_4 >> 0x10)));
  }
  paStack6 = CONCAT22(uVar3,param_1);
  iStack8 = 0;
  while (param_4 = param_4 & 0xffff0000 | (param_4 - 1), param_4 != 0) {
    iVar1 = param_5 + 1;
    iVar4 = param_5 >> 0xf;
    pass1_1008_4544(*param_3);
    iVar2 = iStack8 + 1;
    iVar5 = iStack8 >> 0xf;
    pass1_1008_4544(paStack6);
    pass1_1000_48a8(CONCAT22(iVar5,iStack8),CONCAT22(iVar4,param_5),param_4);
    param_5 = iVar1;
    iStack8 = iVar2;
  }
  return;
}

pub fn pass1_1008_6562(param_1: *mut astruct_57,param_2: *mut astruct_76,mut param_3: u32,mut param_4: i16)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut pstruct57_hi: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut iVar5: i16;
  let mut paVar6: *mut Struct57;
  let mut iStack8: i16;
  let mut paStack6: *mut astruct_76;

  pstruct57_hi = (param_1 >> 0x10);
  paVar6 = (param_1 & 0xffff0000 | param_1 & 0xffff);
  if (param_2 == 0) {
    return;
  }
  mem_op_1000_179c(0x1e,paVar6);
  uVar3 = paVar6 | pstruct57_hi;
  if (uVar3 == 0) {
    pstruct57_hi = 0;
    uVar3 = 0;
  }
  else {
    pass1_1008_405c(CONCAT22(paVar6,pstruct57_hi),(param_2 + 0x4),param_3,
                    (param_3 >> 0x10));
  }
  paStack6 = CONCAT22(uVar3,pstruct57_hi);
  iStack8 = 0;
  while (param_3 = param_3 & 0xffff0000 | (param_3 - 1), param_3 != 0) {
    iVar1 = param_4 + 1;
    iVar4 = param_4 >> 0xf;
    pass1_1008_4544(param_2);
    iVar2 = iStack8 + 1;
    iVar5 = iStack8 >> 0xf;
    pass1_1008_4544(paStack6);
    pass1_1000_48a8(CONCAT22(iVar5,iStack8),CONCAT22(iVar4,param_4),param_3);
    param_4 = iVar1;
    iStack8 = iVar2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn struct_op_1008_6604(param_1: *mut astruct_76,mut param_2: i16,param_3: i32)

{
  let mut uVar1: u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut pstruct76_4: *mut astruct_76;
  let mut iVar2: *mut astruct_84;
  let mut uVar5: *mut astruct_76;
  let mut uVar6: u16;
  let mut lVar7: i32;

  pass1_1008_4016(param_1);
  uVar5 = (param_1 >> 0x10);
  pstruct76_4 = param_1;
  param_1.offset_0x0 = 0x685a;
  pstruct76_4.base_0x2 = 0x1008;
  lVar7 = mem_op_1000_0a48(0x1,0x28,0x0,_PTR_LOOP_1050_5f2c);
  pstruct76_4.field8_0x10 = lVar7;
  (pstruct76_4.field8_0x10 + 0x2) = (lVar7 >> 0x10);
  iVar3 = param_3 * 0x8 + 0x1f;
  uVar4 = ((iVar3 + (iVar3 >> 0xf & 0x1f)) >> 0x5) << 0x2;
  pstruct76_4.field11_0x18 = uVar4;
  pstruct76_4.field12_0x1a = uVar4 >> 0xf;
  lVar7 = mem_op_1000_0a48(0x1,(uVar4 * param_2),
                           ((uVar4 * param_2) >> 0x10),_PTR_LOOP_1050_5f2c);
  uVar4 = (lVar7 >> 0x10);
  pstruct76_4.field3_0x6 = lVar7;
  pstruct76_4.field4_0x8 = uVar4;
  pstruct76_4.field9_0x14 = pstruct76_4.field3_0x6;
  pstruct76_4.field10_0x16 = uVar4;
  pstruct76_4.field8_0x10 = 0x28;
  uVar1 = pstruct76_4.field8_0x10;
  (uVar1 + 0x4) = param_3;
  uVar1 = pstruct76_4.field8_0x10;
  uVar6 = (uVar1 >> 0x10);
  iVar2 = uVar1;
  iVar2.field8_0x8 = param_2;
  iVar2.field9_0xa = param_2 >> 0xf;
  uVar1 = pstruct76_4.field8_0x10;
  (uVar1 + 0xc) = 0x1;
  uVar1 = pstruct76_4.field8_0x10;
  (uVar1 + 0xe) = 0x8;
  uVar1 = pstruct76_4.field8_0x10;
  (uVar1 + 0x10) = 0;
  uVar1 = pstruct76_4.field8_0x10;
  (uVar1 + 0x14) = &pstruct76_4.field11_0x18 * param_2;
  uVar1 = pstruct76_4.field8_0x10;
  (uVar1 + 0x20) = 0x100;
  uVar1 = pstruct76_4.field8_0x10;
  (uVar1 + 0x24) = 0x100;
  return;
}

pub fn pass1_1008_6732(param_1: *mut astruct_288)

{
  let mut iVar2: *mut astruct_288;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar2 = param_1;
  param_1 = 0x685a;
  iVar2.field2_0x2 = 0x1008;
  if (&iVar2[0x1].field2_0x2 != 0) {
    call_fn_ptr_1000_0dc6(*&iVar2[0x1].field2_0x2);
  }
  iVar2[0x1].field2_0x2 = 0;
  pass1_1008_41bc(param_1);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2
pub unsafe fn memcpy_op_1008_676e(param_1: *mut astruct_830,mut param_2: u16 ,param_3: *mut astruct_828)

{
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  let mut iVar1: *mut astruct_828;
  let mut iVar2: *mut astruct_829;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_stack_0000fff2: u16;
  let mut uVar1: u32;
  let mut uVar2: u32;

  paVar1 = CONCAT22(in_register_0000000a,param_2);
  uVar4 = (param_3 >> 0x10);
  iVar1 = param_3;
  if (iVar1.field6_0x6.is_null()) {
    return;
  }
  mem_op_1000_179c(0x1e,paVar1);
  uVar3 = paVar1 | param_1;
  if (uVar3 == 0) {
    param_1 = NULL;
    uVar3 = 0;
  }
  else {
    uVar1 = iVar1.field13_0x10;
    uVar5 = (uVar1 >> 0x10);
    iVar2 = uVar1;
    struct_op_1008_6604(CONCAT22(paVar1,param_1),iVar2.field7_0x8,
                        CONCAT22(in_stack_0000fff2,iVar2.field4_0x4));
  }
  pass1_1000_48a8(param_1.field13_0x10,iVar1.field13_0x10,0x28);
  uVar2 = param_1.field13_0x10;
  hmemcpy16((uVar2 + 0x8) * iVar1.field18_0x18,iVar1.field6_0x6,param_1.field6_0x6);
  param_1.field22_0x1c = 0x1;
  return;
}
pub fn FUN_1008_6814()

{
  return;
}
pub fn FUN_1008_681a()

{
  return;
}
pub fn FUN_1008_681e()

{
  return;
}



pub unsafe fn FUN_1008_6822() -> u16

{
  return 0x0;
}
pub fn FUN_1008_6824()

{
  return;
}
pub fn FUN_1008_6828()

{
  return;
}



pub unsafe fn FUN_1008_682e() -> u16

{
  return 0x0;
}



pub fn pass1_1008_6834(mut param_1: u32,param_2: u8) -> u32

{
  pass1_1008_6732(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn set_struct_1008_687a(param_1: *mut astruct_20,mut param_2: u32)

{
  let mut iVar1: *mut astruct_20;
  let mut uVar1: *mut astruct_20;

  set_struct_op_1008_9584(param_1,param_2);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field152_0xcc = 0;
  iVar1.field153_0xce = 0;
  set_struct_1008_574a((param_1 & 0xffff0000 | ZEXT24(&iVar1.field154_0xd2)));
  param_1.offset_0x0 = 0x6bfc;
  iVar1.base_0x2 = 0x1008;
  iVar1.field163_0xdc = 0;
  return;
}



pub unsafe fn pass1_1008_68c6(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16) -> BOOL16

{
  let mut BVar1: bool;

  BVar1 = show_win_1008_96ae(CONCAT22(param_3,param_2),param_4);
  pass1_1008_6a04(param_1,CONCAT22(param_3,param_2));
  return BVar1;
}
pub unsafe fn pass1_1008_68ea(mut param_1: i16,mut param_2: u16 ,param_3: *mut u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: i16)

{
  let mut ppcVar1: *mut *mut code;

  if (param_6 == 0) {
    if ((param_1 + 0xce) != CONCAT22(param_4,param_3)) {
      if ((param_1 + 0xce) != 0) {
        ppcVar1 = ((param_1 + 0xce) + 0x10);
        (**ppcVar1)();
      }
      (param_1 + 0xce) = CONCAT22(param_4,param_3);
      ppcVar1 = (*param_3 + 0x10);
      (**ppcVar1)();
      ppcVar1 = ((param_1 + 0xce) + 0xc);
      (**ppcVar1)();
      return;
    }
  }
  else {
    pass1_1008_3e0e(CONCAT13((param_2 >> 0x8),CONCAT12(param_2,param_1)));
  }
  return;
}
pub unsafe fn pass1_1008_6978(mut param_1: u16 ,param_2: *mut u8,mut param_3: u32,mut param_4: i16,mut param_5: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  let mut puStack10: *mut u16;
  let mut puStack6: *mut u16;

  paVar3 = CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0xa,paVar3);
  uVar2 = paVar3;
  puStack10 = CONCAT22(uVar2,param_1);
  if ((uVar2 | param_1) == 0) {
    puStack6 = NULL;
  }
  else {
    if (param_4 == 0) {
      param_4 = (param_3 + 0xcc);
    }
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    (param_1 + 0x4) = param_5;
    (param_1 + 0x8) = param_4;
    *puStack10 = 0x6c8c;
    (param_1 + 0x2) = 0x1008;
    puStack6 = puStack10;
  }
  ppcVar1 = ((param_3 + 0xd2) + 0x4);
  (**ppcVar1)(0x1000,(param_3 + 0xd2),param_3,puStack6);
  return;
}
pub unsafe fn pass1_1008_6a04(mut param_1: u16 ,mut param_2: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut puVar2: *mut u8;
  let mut extraout_DX: u16;
  let mut local_a: [u8;0x8] = [0;0x8];

  pass1_1008_57a4(CONCAT22(0x1050,local_a),param_2 & 0xffff0000 | (param_2 + 0xd2));
  loop {
    puVar2 = local_a;
    pass1_1008_5b12(CONCAT22(0x1050,puVar2));
    if ((extraout_DX | puVar2) == 0) {break;}
    ppcVar1 = (*(puVar2 + 0x4) + 0xc);
    (**ppcVar1)();
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub unsafe fn pass1_1008_6a4a(mut param_1: u32,mut param_2: i16,mut param_3: u16 ,mut param_4: i16)

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut puVar3: *mut u8;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut local_e: [u8;0x4] = [0;0x4];
  let mut uStack10: u32;
  let mut uStack6: u32;

  if (param_4 == 0x2) {
    iVar2 = param_1;
    pass1_1008_57a4(CONCAT22(0x1050,local_e),param_1 & 0xffff0000 | (iVar2 + 0xd2));
    loop {
      puVar3 = local_e;
      pass1_1008_5b12(CONCAT22(0x1050,puVar3));
      uStack6 = CONCAT22(extraout_DX,puVar3);
      if ((extraout_DX | puVar3) == 0) {break;}
      if !((puVar3 + 0x8) != param_2){break;}
    }
    if (uStack6 != 0) {
      ppcVar1 = ((iVar2 + 0xd2) + 0xc);
      (**ppcVar1)();
      uStack10 = 0;
      uStack6 = local_e;
      pass1_1008_5b12(CONCAT22(0x1050,uStack6));
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
pub unsafe fn pass1_1008_6b02(mut param_1: u32)

{
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
pub unsafe fn pass1_1008_6b2e(mut param_1: u32)

{
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



pub unsafe fn pass1_1008_6b5a(param_1: *mut astruct_458,param_2: u8) -> *mut u16

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut uVar4: *mut astruct_458;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  uVar4 = param_1;
  param_1 = 0x6c8c;
  uVar4.field2_0x2 = 0x1008;
  puVar1 = uVar4.field3_0x4;
  uVar2 = uVar4.field4_0x6;
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  param_1 = 0x389a;
  uVar4.field2_0x2 = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn pass1_1008_6bb4(param_1: *mut astruct_459,param_2: u8)

{
  let mut uVar1: *mut astruct_459;
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
pub fn pass1_1008_6c90(param_1: *mut u16)

{
  pass1_1008_3e38(param_1);
  pass1_1008_3e38((param_1 & 0xffff0000 | (param_1 + 0x6)));
  return;
}



pub unsafe fn pass1_1008_6cb4(param_1: *mut astruct_362,param_2: *mut u32,mut param_3: u16 ,param_4: *mut u32,mut param_5: u16 ) -> *mut u32

{
  let mut iVar1: *mut astruct_362;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1 = *param_4;
  iVar1.field4_0x4 = (param_4 + 1);
  iVar1.field5_0x6 = *param_2;
  iVar1.field6_0xa = (param_2 + 1);
  return param_1;
}
pub fn pass1_1008_6cec(param_1: *mut u16,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 ,mut param_5: u32)

{
  pass1_1008_3e76(param_1,param_4,param_5,(param_5 >> 0x10));
  pass1_1008_3e76((param_1 & 0xffff0000 | (param_1 + 0x6)),param_2,param_3,
                  (param_3 >> 0x10));
  return;
}
pub fn pass1_1008_6d18(param_1: *mut u16,param_2: *mut u16,param_3: *mut u16)

{
  pass1_1008_3f62(param_1,param_3);
  pass1_1008_3f62((param_1 & 0xffff0000 | (param_1 + 0x6)),param_2);
  return;
}
pub fn pass1_1008_6d3e(param_1: *mut u16,param_2: *mut u16,param_3: *mut u16)

{
  pass1_1008_3f62(param_3,param_1);
  pass1_1008_3f62(param_2,(param_1 & 0xffff0000 | (param_1 + 0x6)));
  return;
}
pub fn pass1_1008_6d64(param_1: *mut u16,param_2: *mut u16)

{
  pass1_1008_3f62(param_2,param_1);
  pass1_1008_3ee2(param_2,(param_1 & 0xffff0000 | (param_1 + 0x6)));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

 pub unsafe fn str_1008_6d8a(mut param_1: u16 ,param_2: *mut u32,param_3: *mut c_char) -> *mut u32

{
  let mut uVar1: u16;
  let mut uVar2: u16;

  uVar2 = (param_2 >> 0x10);
  *param_2 = 0;
  (param_2 + 0x4) = 0xffff;
  u16_1050_0312 = 0x4;
  sys_1000_3f9c(s__1050_65a0,_PTR_s_SC_03d_1050_0314_1050_031c,0x4);
  uVar1 = str_op_1008_60e8(param_1,param_3);
  param_2 = uVar1;
  (param_2 + 0x2) = param_1;
  return param_2;
}

pub unsafe fn close_file_1008_6dd0(param_1: *mut StructD)

{
  let mut struct_1: *mut StructD;
  let mut struct_1_hi: u16;

  struct_1_hi = (param_1 >> 0x10);
  struct_1 = param_1;
  if (struct_1.hfile_0x4 != 0xffff) {
    _lclose16(struct_1.hfile_0x4);
    struct_1.hfile_0x4 = 0xffff;
  }
  fn_ptr_1000_17ce(*param_1);
  return;
}



pub unsafe fn file_fn_1008_6e02(param_1: *mut astruct_802) -> BOOL16

{
  let mut var1: u16;
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut var2: bool;
  let mut extraout_DX: *mut u8;
  let mut struct_1: *mut astruct_802;
  let mut unaff_DI: i16;
  let mut uVar1: u16;
  let mut puVar1: *mut u16;
  let mut local_4: [u8;0x2] = [0;0x2];

  u16_1050_0310 = 0;
  var1 = write_to_file_1008_70a6(param_1);
  if (var1 != 0) {
    uVar1 = (param_1 >> 0x10);
    struct_1 = param_1;
    puVar1 = pass1_1008_72a8(CONCAT22(0x1050,local_4),struct_1.hfile_0x4);
    extraout_DX = (puVar1 >> 0x10);
    iVar1 = pass1_1008_7006(extraout_DX,struct_1,uVar1,CONCAT22(0x1050,local_4),&DAT_1050_1050);
    if ((iVar1 != 0) && (iVar2 = file_fn_1008_6eee(struct_1,uVar1,CONCAT22(0x1050,local_4)), iVar2 != 0)) {
      var2 = file_fn_1008_726c(struct_1,uVar1);
      if (var2 == 0) {
        return 0x0;
      }
      return 0x1;
    }
    _lclose16(struct_1.hfile_0x4);
  }
  return 0x0;
}



pub unsafe fn read_file_1008_6e78(param_1: *mut astruct_806,mut param_2: u16 ,in_string: u16,mut param_4: u16 ) -> BOOL16

{
  let mut b_var1: bool;
  let mut i_var2: i16;
  let mut var3: *mut u8;
  let mut puVar1: *mut u8;
  let mut paVar2: *mut astruct_802;
  let mut unaff_BP: u16;
  let mut uVar3: u16;
  let mut unaff_CS: u16;
  let mut puVar4: *mut u16;
  let mut in_stack_00000000: u16;
  let mut in_stack_0000fffc: u16;

  u16_1050_0310 = 0;
  b_var1 = read_file_1008_7146(unaff_CS,param_1,in_stack_0000fffc,unaff_BP,in_stack_00000000);
  if (b_var1 != 0) {
    uVar3 = (param_1 >> 0x10);
    paVar2 = param_1;
    puVar4 = pass1_1008_72a8(CONCAT22(0x1050,&stack0xfffc),paVar2.hfile_0x4);
    puVar1 = (puVar4 >> 0x10);
    i_var2 = pass1_1008_7056(puVar1,paVar2,uVar3,CONCAT22(0x1050,&stack0xfffc));
    if (i_var2 != 0) {
      var3 = &stack0xfffc;
      read_file_1008_6f7a(paVar2,uVar3,CONCAT22(0x1050,var3),puVar1);
      if (var3.is_null() == false) {
        b_var1 = file_fn_1008_726c(paVar2,uVar3);
        if (b_var1 == 0) {
          return 0x0;
        }
        return 0x1;
      }
    }
    _lclose16(paVar2.hfile_0x4);
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn file_fn_1008_6eee(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut BVar1: bool;
  let mut uVar2: u16;
  let mut in_DX: *mut u8;
  let mut uVar3: u16;
  let mut puVar4: *mut u16;
  let mut uVar5: u32;
  let mut local_e: [u8;0x4] = [0;0x4];
  let mut uStack10: u32;
  let mut puStack6: *mut u32;

  puStack6 = *_u16_1050_5748;
  uStack10 = *puStack6;
  puVar4 = pass1_1020_a43e(in_DX,CONCAT22(0x1050,local_e));
  uVar3 = (puVar4 >> 0x10);
  BVar1 = pass1_1028_d7a0(uStack10,(uStack10 >> 0x10),param_3);
  if (BVar1 != 0) {
    BVar1 = pass1_1030_5c1a(_PTR_LOOP_1050_5736,param_3);
    if (BVar1 != 0) {
      uVar5 = write_to_file_1028_dce2(uVar3,_PTR_LOOP_1050_65e2,param_3);
      if ((uVar5 >> 0x10) != 0) {
        uVar2 = pass1_1038_7b20(_PTR_LOOP_1050_5a64,param_3);
        if (uVar2 != 0) {
          BVar1 = pass1_1020_a644(local_e,&DAT_1050_1050,param_3);
          if (BVar1 != 0) {
            return;
          }
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn read_file_1008_6f7a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,param_4: *mut u8)

{
  let mut var5: u16;
  let mut i_var3: i16;
  let mut b_var4: bool;
  let mut uVar1: u16;
  let mut puVar2: *mut u16;
  let mut local_e: [u8;0x4] = [0;0x4];
  let mut uStack10: u32;
  let mut puStack6: *mut u32;

  puStack6 = *_u16_1050_5748;
  uStack10 = *puStack6;
  puVar2 = pass1_1020_a43e(param_4,CONCAT22(0x1050,local_e));
  uVar1 = (puVar2 >> 0x10);
  var5 = read_file_1028_d7ba(puVar2,uStack10,(uStack10 >> 0x10),param_3);
  if (var5 != 0) {
    var5 = read_file_1030_5c52(var5,_PTR_LOOP_1050_5736,param_3);
    if (var5 != 0) {
      read_file_1028_def2(var5,_PTR_LOOP_1050_65e2,param_3);
      if (var5 != 0) {
        i_var3 = read_file_1038_7c02(var5,uVar1,_PTR_LOOP_1050_5a64,param_3);
        if (i_var3 != 0) {
          b_var4 = read_file_1020_a65e(local_e,CONCAT22(0x1050,local_e),param_3);
          if (b_var4 != 0) {
            return;
          }
        }
      }
    }
  }
  return;
}
