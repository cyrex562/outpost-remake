use crate::windef::{HWND16, LPARAM, LRESULT, WPARAM16};


pub unsafe fn pass1_1008_507c(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  pass1_1008_41bc(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub unsafe fn pass1_1008_50c2(param_1: *mut astruct_110, mut param_2: u32, mut param_3: u32, param_4: *mut u16, param_5: *mut astruct_76)
{
  let mut pstruct110_1: *mut astruct_110;
  let mut _seg: u16;
  param_1.field0_0x0 = *param_4;
  param_1_seg = (param_1 >> 0x10);
  pstruct110_1 = param_1;
  pstruct110_1.field1_0x2 = (param_4 + 2);
  pstruct110_1.field2_0x4 = param_3;
  pstruct110_1.field3_0x8 = param_2;
  pstruct110_1.field4_0xc = param_5;
  pstruct110_1.field5_0x10 = 0;
  pass1_1008_52fc((param_1 & 0xffff | param_1_seg << 0x10));
  return;
}


pub unsafe fn pass1_1008_5118(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x10) != 0) {
    call_fn_ptr_1000_0dc6(*(param_1 + 0x10));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_5134(mut param_1: u32)

{
  let mut puVar1: *mut u16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut lVar4: i32;
  let mut iVar5: i16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut iStack16: i16;
  let mut lStack14: i32;
  let mut uStack10: u32;

  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  lVar4 = (iVar6 + 0x4) * (iVar6 + 0x8);
  lVar4 = mem_op_1000_0a48(0x1,lVar4,(lVar4 >> 0x10),_PTR_LOOP_1050_5f2c);
  uVar3 = (lVar4 >> 0x10);
  (iVar6 + 0x10) = lVar4;
  (iVar6 + 0x12) = uVar3;
  if ((uVar3 | (iVar6 + 0x10)) == 0) {
    return;
  }
  iVar5 = (iVar6 + 0x8);
  iVar2 = (iVar6 + 0xa);
  lVar4 = CONCAT22(iVar2 - (iVar5 == 0),iVar5 -1) * (iVar6 + 0x4);
  puVar1 = (iVar6 + 0x10);
  uVar3 = lVar4;
  uStack10 = CONCAT22(((lVar4 >> 0x10) + CARRY2(uVar3,*puVar1)) * 0x100 + (iVar6 + 0x12),
                      uVar3 + *puVar1);
  lStack14 = CONCAT22(iVar2,iVar5);
  iStack16 = (iVar6 + 2);
  while (lStack14 != 0) {
    iVar2 = iStack16 + 1;
    iVar5 = iStack16 >> 0xf;
    pass1_1008_4544((iVar6 + 0xc));
    pass1_1000_48a8(uStack10,CONCAT22(iVar5,iStack16),(iVar6 + 0x4));
    iVar5 = (iVar6 + 0x4);
    uVar3 = -iVar5;
    uStack10 = CONCAT22((uStack10 >> 0x10) +
                        (CARRY2(uStack10,uVar3) - ((iVar6 + 0x6) + (iVar5 != 0))) * 0x100,
                        uStack10 + uVar3);
    iStack16 = iVar2;
    lStack14 = lStack14 -0x1;
  }
  return;
}

pub unsafe fn pass1_1008_5236(param_1: *mut astruct_109)

{
  let mut puVar1: *mut u16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut lVar4: i32;
  let mut iVar5: i16;
  let mut pstruct109_6: *mut astruct_109;
  let mut uVar6: u16;
  let mut bVar7: bool;
  let mut iStack12: i16;
  let mut lStack10: i32;
  let mut uStack6: u16;
  let mut iStack4: i16;

  uVar6 = (param_1 >> 0x10);
  pstruct109_6 = param_1;
  iVar5 = pstruct109_6.field6_0x8;
  iVar2 = pstruct109_6.field7_0xa;
  lVar4 = CONCAT22(iVar2 - (iVar5 == 0),iVar5 -1) * &pstruct109_6.field_0x4;
  puVar1 = &pstruct109_6.field9_0x10;
  uVar3 = lVar4;
  uStack6 = uVar3 + *puVar1;
  iStack4 = ((lVar4 >> 0x10) + CARRY2(uVar3,*puVar1)) * 0x100 + pstruct109_6.field10_0x12;
  lStack10 = CONCAT22(iVar2,iVar5);
  iStack12 = pstruct109_6.field2_0x2;
  while (lStack10 != 0) {
    iVar2 = iStack12 + 1;
    iVar5 = iStack12 >> 0xf;
    pass1_1008_4544(pstruct109_6.field8_0xc);
    pass1_1000_48a8(CONCAT22(iVar5,iStack12),CONCAT22(iStack4,uStack6),&pstruct109_6.field_0x4);
    iVar5 = &pstruct109_6.field_0x4;
    uVar3 = -iVar5;
    bVar7 = CARRY2(uStack6,uVar3);
    uStack6 += uVar3;
    iStack4 += (bVar7 - (pstruct109_6.field5_0x6 + (iVar5 != 0))) * 0x100;
    iStack12 = iVar2;
    lStack10 = lStack10 -0x1;
  }
  return;
}
pub unsafe fn pass1_1008_52fc(param_1: *mut astruct_110)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut lVar4: i32;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut iVar7: i16;
  let mut pstruct110_8: *mut astruct_110;
  let mut uVar8: *mut astruct_110;
  let mut uVar9: u32;
  let mut uStack14: u16;
  let mut iStack12: i16;

  uVar8 = (param_1 >> 0x10);
  pstruct110_8 = param_1;
  uVar9 = pass1_1008_4772(pstruct110_8.field4_0xc);
  uVar5 = (uVar9 >> 0x10);
  iVar7 = uVar9;
  iVar6 = (iVar7 + 0x4);
  uVar3 = iVar6 - 0x1;
  iVar6 = (iVar7 + 0x6) - (iVar6 == 0);
  lVar4 = (iVar7 + 0x8) -0x1;
  uVar2 = param_1.field0_0x0;
  puVar1 = &pstruct110_8.field2_0x4;
  iVar7 = (uVar2 >> 0xf) + (&pstruct110_8.field2_0x4 + 0x2) + CARRY2(uVar2,puVar1);
  if ((iVar6 <= iVar7) && ((iVar6 < iVar7 || (uVar3 < uVar2 + puVar1)))) {
    pstruct110_8.field2_0x4 = uVar3 - uVar2;
    (&pstruct110_8.field2_0x4 + 0x2) = (iVar6 - (uVar2 >> 0xf)) - (uVar3 < uVar2);
  }
  uVar2 = pstruct110_8.field1_0x2;
  puVar1 = &pstruct110_8.field3_0x8;
  iVar6 = (uVar2 >> 0xf) + (&pstruct110_8.field3_0x8 + 0x2) + CARRY2(uVar2,puVar1);
  iStack12 = (lVar4 >> 0x10);
  if ((iStack12 <= iVar6) && ((uStack14 = lVar4, iStack12 < iVar6 || (uStack14 < uVar2 + puVar1)))) {
    pstruct110_8.field3_0x8 = uStack14 - uVar2;
    (&pstruct110_8.field3_0x8 + 0x2) = (iStack12 - (uVar2 >> 0xf)) - (uStack14 < uVar2);
  }
  return;
}



pub unsafe fn pass1_1008_5394(param_1: u32) -> *mut u32

{
  *param_1 = 0;
  return param_1;
}
pub unsafe fn pass1_1008_53aa()

{
  return;
}
pub unsafe fn mci_send_command_1008_53ae(mut param_1: u32,mut param_2: u16 )

{
  let mut DVar1: u32;
  let mut DVar2: u32;
  let mut w_error: u32;
  let mut local_32: u16;
  let mut uStack48: u16;
  let mut local_2e: u16;
  let mut uStack44: u16;
  let mut uStack34: u16;
  let mut uStack32: u16;
  let mut local_1e: u32;
  let mut UStack26: u16;
  let mut uStack22: u16;
  let mut uStack20: u16;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut uStack6: u16;

  local_1e = 0;
  uStack22 = 0x28c;
  uStack20 = &DAT_1050_1050;
  uStack18 = param_1;
  uStack14 = 0;
  uStack10 = 0;
  uStack8 = 0x4000;
  uStack6 = param_2;
  DVar1 = mciSendCommand16(CONCAT22(0x1050,&local_1e),0x30200,0x803,0x0);
  DVar1 = (DVar1 >> 0x10);
  uStack34 = DVar1;
  if (UStack26 != 0) {
    if ((DVar1 | uStack34) != 0) {
      mciGetErrorString16(0x400,CONCAT22(0x1050,&stack0xfbce),DVar1);
    }
    pass1_1000_4906(CONCAT22(0x1050,&local_2e),NULL,0xc);
    local_2e = param_2;
    uStack44 = 0;
    DVar2 = mciSendCommand16(CONCAT22(0x1050,&local_2e),0x2,0x806,UStack26);
    DVar2 = (DVar2 >> 0x10);
    uStack34 = DVar2;
    DVar1 = DVar2;
    if ((DVar2 | uStack34) != 0) {
      mciGetErrorString16(0x400,CONCAT22(0x1050,&stack0xfbce),DVar2);
    }
    local_32 = param_2;
    uStack48 = 0;
    w_error = mciSendCommand16(CONCAT22(0x1050,&local_32),0x1,0x804,UStack26);
    DVar1 = (w_error >> 0x10);
    uStack34 = w_error;
    if ((DVar1 | uStack34) != 0) {
      mciGetErrorString16(0x400,CONCAT22(0x1050,&stack0xfbce),w_error);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn init_op_1008_54aa(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut u8,param_5: *mut c_char,param_6: *mut u8,
                      param_7: *mut u8)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar3: u16;
  let mut in_CX: u16;
  let mut in_DX: u16;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  let mut unaff_SI: u16;
  let mut unaff_DI: u16;
  let mut unaff_CS: u16;
  let mut uVar6: u32;
  let mut uVar7: u32;
  let mut in_stack_0000ffea: u16;
  let mut in_stack_0000ffec: u16;
  let mut puStack12: *mut u32;
  let mut uVar2: u32;

  if (param_6.is_null() == false) {
    return;
  }
  dos3_call_op_1000_435c(unaff_CS,NULL,unaff_SI,unaff_DI,in_stack_0000ffea,in_stack_0000ffec);
  pass1_1000_4d0c(param_1);
  pass1_1000_1fea();
  _PTR_LOOP_1050_03a0 = mem_op_1000_1902(in_DX,0x0,0x32,0x0,0x12);
  _PTR_LOOP_1050_029c = mem_op_1000_1902((_PTR_LOOP_1050_03a0 >> 0x10),0x0,0x64,0x0,0xc);
  _PTR_LOOP_1050_4fb8 = mem_op_1000_1902((_PTR_LOOP_1050_029c >> 0x10),0x0,0x64,0x0,0x10);
  _PTR_LOOP_1050_68a2 = mem_op_1000_1902((_PTR_LOOP_1050_4fb8 >> 0x10),0x0,0x64,0x0,0xe);
  _PTR_LOOP_1050_5744 = mem_op_1000_1902((_PTR_LOOP_1050_68a2 >> 0x10),0x0,0x1f4,0x0,0x42);
  uVar6 = mem_op_1000_1902((_PTR_LOOP_1050_5744 >> 0x10),0x0,0x32,0x0,0x6);
  PTR_LOOP_1050_576a = (uVar6 >> 0x10);
  paVar5 = CONCAT22(in_register_0000000a,PTR_LOOP_1050_576a);
  PTR_LOOP_1050_5768 = uVar6;
  HINSTANCE16_1050_038c = param_7;
  PTR_LOOP_1050_038e = param_6;
  PTR_LOOP_1050_0390 = param_4;
  uVar3 = str_op_1008_60e8(PTR_LOOP_1050_576a,param_5);
  _PTR_LOOP_1050_0392 = CONCAT22(paVar5,uVar3);
  mem_op_1000_179c(0xc,paVar5);
  extraout_DX = paVar5 | uVar3;
  if (extraout_DX == 0) {
    uVar3 = 0;
    extraout_DX = 0;
  }
  else {
    struct_op_1008_0000(CONCAT22(paVar5,uVar3));
  }
  puStack12 = CONCAT22(extraout_DX,uVar3);
  uVar4 = extraout_DX;
  if (_PTR_LOOP_1050_0392 != 0) {
    ppcVar1 = (*puStack12 + 0x4);
    (**ppcVar1)(0x1000,uVar3,extraout_DX,_PTR_LOOP_1050_0392);
  }
  uVar7 = CONCAT22(extraout_DX,uVar3);
  uVar2 = *puStack12;
  ppcVar1 = uVar2 + 0x4;
  (**ppcVar1)();
  win_msg_op_1008_9498();
  if (puStack12.is_null() == false) {
    ppcVar1 = uVar2;
    (**ppcVar1)(0x1000,uVar3,extraout_DX,0x1,uVar7);
  }
  uVar6 = mem_op_1000_1b68(uVar4,_PTR_LOOP_1050_03a0,(_PTR_LOOP_1050_03a0 >> 0x10));
  uVar6 = mem_op_1000_1b68((uVar6 >> 0x10),_PTR_LOOP_1050_029c,(_PTR_LOOP_1050_029c >> 0x10));
  uVar6 = mem_op_1000_1b68((uVar6 >> 0x10),_PTR_LOOP_1050_4fb8,(_PTR_LOOP_1050_4fb8 >> 0x10));
  uVar6 = mem_op_1000_1b68((uVar6 >> 0x10),_PTR_LOOP_1050_68a2,(_PTR_LOOP_1050_68a2 >> 0x10));
  mem_op_1000_1b68((uVar6 >> 0x10),_PTR_LOOP_1050_5744,(_PTR_LOOP_1050_5744 >> 0x10));
  return;
}


pub unsafe fn def_win_proc_1008_5632(param_1: LPARAM,param_2: WPARAM,mut param_3: u16 ,param_4: HWND16)

{
  let mut ppcVar1: *mut *mut code;
  let mut puStack6: *mut u32;
  let mut uVar2: u16;

  uVar2 = &DAT_1050_1050;
  puStack6 = GetWindowLong16(0x0,param_4);
  if (((puStack6 >> 0x10) | puStack6) == 0) {
    if (param_3 != 1) {
      DefWindowProc16(param_1,param_2,param_3,param_4);
      return;
    }
    puStack6 = param_1;
    SetWindowLong16(puStack6,0x0,param_4);
    pass1_1008_9628(puStack6,param_4);
  }
  ppcVar1 = (*puStack6 + 0x1c);
  (**ppcVar1)(s_tile2_bmp_1050_1538,puStack6,(puStack6 >> 0x10),param_1,param_2,param_3,uVar2);
  return;
}



pub unsafe fn struct_op_1008_56b4(param_1: *mut astruct_76) -> *mut u16

{
  let mut iVar1: *mut astruct_76;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.offset_0x0 = 0x389a;
  iVar1.base_0x2 = 0x1008;
  iVar1.field2_0x4 = 0;
  param_1.offset_0x0 = s__s__d_1050_573a;
  iVar1.base_0x2 = 0x1008;
  return &param_1.offset_0x0;
}



pub unsafe fn cleanup_palette_1008_56e2(mut param_1: u32,mut param_2: u32) -> BOOL16

{
  let mut hpalette_a: HPALETTE16;
  let mut u16_a: u16;

  u16_a = (param_1 >> 0x10);
  hpalette_a = SelectPalette16(0x0,(param_1 + 0x4),*param_2);
  (param_1 + 0x4) = hpalette_a;
  DeleteObject16(hpalette_a);
  return 0x1;
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
pub unsafe fn set_struct_1008_574a(param_1: *mut Struct57)

{
  let mut iVar1: *mut StructD;
  let mut uVar1: *mut StructD;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1.field0_0x0 = 0x389a;
  iVar1.address_offset_field_0x2 = 0x1008;
  iVar1.hfile_0x4 = 0;
  iVar1.field5_0x8 = 0;
  iVar1.field6_0xa = 0x1;
    // just 0x5bc4
  param_1.field0_0x0 = s__s__s__1050_5bc0 + 0x4;
  iVar1.address_offset_field_0x2 = 0x1008;
  return;
}
pub unsafe fn pass1_1008_5784(param_1: *mut c_char,mut param_2: u32)

{
  param_1 = param_2;
  (param_1 + 0x4) = 0;
  return;
}
pub unsafe fn pass1_1008_57a4(param_1: u32,mut param_2: u32)

{
  *param_1 = param_2;
  (param_1 + 0x4) = 0;
  return;
}
pub unsafe fn pass1_1008_57c4(param_1: *mut StructD)

{
  let mut uVar1: u16;
  uVar1 = (param_1 >> 0x10);
  param_1.address_offset_field_0x0 = s__s__s__1050_5bc0 + 0x4;
  (param_1 + 0x2) = 0x1008;
  pass1_1008_5830(param_1 & 0xffff | uVar1 << 0x10);
  param_1.address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  return;
}



pub unsafe fn pass1_1008_57f0(mut param_1: u32,mut param_2: i16) -> i32

{
  let mut bVar1: bool;
  let mut lVar2: i32;
  let mut iStack12: i16;
  let mut local_a: [u8;0x8] = [0;0x8];

  pass1_1008_5784(CONCAT22(0x1050,local_a),param_1);
  iStack12 = 0;
  loop {
    lVar2 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
    if (lVar2 == 0) {
      return 0x0;
    }
    bVar1 = iStack12 != param_2;
    iStack12 = iStack12 + 1;
    if bVar1 == false{break;}
  }
  return lVar2;
}
pub unsafe fn pass1_1008_5830(mut param_1: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut uVar4: u32;
  let mut puVar5: *mut u32;
  let mut iVar6: i16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uVar9: u16;

  loop {
    uVar8 = (param_1 >> 0x10);
    iVar6 = param_1;
    if ((iVar6 + 0x4) == 0) {break;}
    if ((iVar6 + 0xa) != 0) {
      uVar4 = (iVar6 + 0x4);
      uVar9 = (uVar4 >> 0x10);
      iVar7 = uVar4;
      puVar1 = (iVar7 + 0x8);
      uVar2 = (iVar7 + 0xa);
      if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
      }
    }
    puVar5 = (iVar6 + 0x4);
    (iVar6 + 0x4) = (puVar5 + 0x4);
    if (puVar5.is_null() == false) {
      ppcVar3 = *puVar5;
      (**ppcVar3)();
    }
  }
  (iVar6 + 0x8) = 0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
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
pub unsafe fn pass1_1008_5b12(param_1: *mut c_char)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u16;

  if ((param_1 != 0) && ((param_1 + 0x8) != 0)) {
    uVar4 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0x4) == 0) {
      uVar5 = (param_1 >> 0x10);
      iVar3 = param_1;
    }
    else {
      uVar1 = (iVar2 + 0x4);
      uVar5 = (uVar1 >> 0x10);
      iVar3 = uVar1;
    }
    (iVar2 + 0x4) = (iVar3 + 0x4);
    if ((iVar2 + 0x4) != 0) {
      return;
    }
  }
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_5bdc(param_1: *mut c_char)

{
  let mut in_EDX: u32;
  let mut uVar1: u16;
  let mut pstruct19_1: *mut Struct19;
  let mut unaff_BP: u16;
  let mut pstruct19_param_1: *mut Struct19;
  let mut paVar2: *mut Struct19;
  let mut puVar3: *mut u32;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  uVar1 = (in_EDX >> 0x10);
  paVar2 = struct_op_1010_1d48(param_1,0x44);
  pstruct19_param_1 = (param_1 >> 0x10);
  pstruct19_1 = param_1;
  pstruct19_1.horiz_res_0xa = 0;
  pstruct19_1.ver_res_0xc = 0;
  pstruct19_1.field8_0x10 = 0;
  pstruct19_1.field9_0x12 = 0;
  param_1 = 0x5fc8;
  pstruct19_1.segment_0x2 = 0x1008;
  _u16_1050_02a0 = param_1;
  puVar3 = mixed_1010_20ba(CONCAT22(uVar1,(paVar2 >> 0x10)),_u16_1050_0ed0,
                           CONCAT22(unaff_BP,0x2),in_stack_0000fea6,in_stack_0000ffca,in_stack_0000ffd0,
                           in_stack_0000ffd4);
  pstruct19_1.ver_res_0xc = puVar3;
  pstruct19_1.field_0xe = (puVar3 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1008_5c34(param_1: *mut c_char)

{
  param_1 = 0x5fc8;
  (param_1 + 0x2) = 0x1008;
  _u16_1050_02a0 = 0;
  pass1_1010_1d80(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn win_1008_5c5c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 )

{
  pass1_1010_84f8(_u16_1050_14cc,param_4);
  win_ui_op_1008_5cfe(param_3,CONCAT22(param_2,param_1),&DAT_1050_1050);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn win_1008_5c7c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  pass1_1010_85be(_u16_1050_14cc,param_4,(param_4 >> 0x10));
  win_ui_op_1008_5cfe(param_3,CONCAT22(param_2,param_1),&DAT_1050_1050);
  return;
}
pub unsafe fn win_1008_5c9e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,param_4: *mut u32)

{
  win_1008_5c7c(param_1,param_2,param_3,*param_4);
  return;
}
pub unsafe fn mci_send_command_1008_5cb6(param_1: *mut Struct27,mut param_2: i16)

{
  let mut iVar1: *mut Struct27;
  let mut uVar1: u16;
  let mut iVar2: i16;

  mciSendCommand16(0x0,0x0,0x804,param_2);
  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((&iVar1.field_0xa == 0) || (&iVar1.field_0xa != param_2)) {
    iVar1.field18_0x12 = 0;
    iVar2 = 0x11;
  }
  else {
    iVar1.field_0x10 = 0;
    iVar2 = 0x10;
  }
  pass1_1010_1f62(param_1,iVar2);
  return;
}
pub unsafe fn win_ui_op_1008_5cfe(param_1: *mut Struct27,param_2: *mut c_char,in_wnd_class: *mut WNDCLASS16 )

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut iVar3: *mut Struct27;
  let mut uVar3: u16;
  let mut DVar4: u32;
  let mut iVar5: i16;
  let mut message_1: HWND16;
  let mut uStack298: u16;
  let mut window_handle_1: HWND16;
  let mut local_11e: [u8;0x100] = [0;0x100];
  let mut string_1: *mut c_char;
  let mut iStack26: i16;
  let mut iStack24: i16;
  let mut local_16: [u8;0x4] = [0;0x4];
  let mut offset_val: u16;
  let mut pcStack14: *mut c_char;
  let mut pcStack10: *mut c_char;

  pass1_1000_4906(CONCAT22(0x1050,local_16),NULL,0x14);
  pcStack10 = param_2;
  uVar3 = (param_1 >> 0x10);
  iVar3 = param_1;
  uVar1 = &iVar3.field_0xc;
  iStack24 = (uVar1 + 0x72);
  iStack26 = 0x1;
  string_1 = s_waveaudio_1050_02a4;
  str_1000_4d58(param_2,NULL,0x0,0x0,CONCAT22(0x1050,local_11e));
  iVar2 = pass1_1000_475e(CONCAT22(0x1050,local_11e),s__mid_1050_02ae);
  if (iVar2 == 0) {
    uVar1 = &iVar3.field_0xc;
    iStack24 = (uVar1 + 0x74);
    string_1 = s_sequencer_1050_02b3;
    iStack26 = 0;
  }
  if (iStack24 != 0) {
    if ((iStack26 != 0) && (&iVar3.field_0x10 != 0)) {
      return;
    }
    if ((iStack26 == 0) && (iVar3.field18_0x12 != 0)) {
      return;
    }
    pcStack14 = string_1;
    DVar4 = mciSendCommand16(CONCAT22(0x1050,local_16),0x2200,0x803,0x0);
    if (((DVar4 >> 0x10) | DVar4) == 0) {
      if (iStack26 == 0) {
        iVar3.field18_0x12 = 0x1;
      }
      else {
        iVar3.field_0xa = offset_val;
        iVar3.field_0x10 = 0x1;
      }
      window_handle_1 = create_window_1008_5e7e();
      if (window_handle_1 == 0) {
        mci_send_command_1008_5cb6(param_1,offset_val);
        return;
      }
      pass1_1000_4906(CONCAT22(0x1050,&message_1),NULL,0xc);
      message_1 = window_handle_1;
      uStack298 = 0;
      mciSendCommand16(CONCAT22(0x1050,&message_1),0x1,0x806,offset_val);
      SetWindowWord16(offset_val,0x0,window_handle_1);
      return;
    }
  }
  if (iStack26 == 0) {
    iVar5 = 0x11;
  }
  else {
    iVar5 = 0x10;
  }
  pass1_1010_1f62(param_1,iVar5);
  return;
}






// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn win_proc_1008_5f44(mut param_1: u16, mut param_2: u16, param_3: LPARAM, in_wparam_2: WPARAM16, mut param_5: u16, param_6: HWND16) -> LRESULT

{
  let mut WVar1: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut LVar3: LRESULT;
  let mut puVar4: *mut u32;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut in_stack_0000fff8: u16;

  paVar2 = CONCAT22(in_register_0000000a,param_1);
  if (param_5 == 0x2) {
    WVar1 = GetWindowWord16(0x0,param_6);
    mci_send_command_1008_5cb6(_u16_1050_02a0,WVar1);
    puVar4 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,CONCAT22(param_2,0x37),in_stack_0000fea0,
                             in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
    pass1_1008_aa28(puVar4,puVar4);
  }
  else {
    if (param_5 != 0x3b9) {
      LVar3 = DefWindowProc16(param_3,in_wparam_2,param_5,param_6);
      return LVar3;
    }
    DestroyWindow16(param_6);
  }
  return 0x0;
}



pub unsafe fn pass1_1008_5fa2(mut param_1: u32,param_2: u8) -> u32

{
  pass1_1008_5c34(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_5fd8(param_1: *mut u8) -> *mut u8

{
  let mut puVar1: *mut u16;
  let mut puVar2: *mut u8;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  let mut pcVar4: *mut c_char;
  let mut in_stack_00000004: u16;
  let mut puStack6: *mut u16;

  paVar3 = CONCAT22(in_register_0000000a,param_1);
  puVar2 = &stack0x0006;
  puStack6 = CONCAT22(0x1050,puVar2);
  mem_op_1000_179c(0x1000,paVar3);
  pcVar4 = load_string_1010_847e(_u16_1050_14cc,in_stack_00000004);
  unk_str_op_1000_3d3e(CONCAT22(paVar3,puVar2),pcVar4);
  loop {
    puVar1 = puStack6;
    puStack6 = (puStack6 & 0xffff0000 | (puStack6 + 0x2));
    if (*puVar1 == 0) {break;}
    pcVar4 = load_string_1010_847e(_u16_1050_14cc,*puVar1);
    pass1_1000_3cea(CONCAT22(paVar3,puVar2),pcVar4);
  }
  return puVar2;
}
