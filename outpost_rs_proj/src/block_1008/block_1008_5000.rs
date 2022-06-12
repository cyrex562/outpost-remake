
StructD * pass1_1008_507c(StructD *param_1,param_2: u8)

{
  pass1_1008_41bc((astruct_288 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1008_50c2(param_1: *mut astruct_110,mut param_2: u32,mut param_3: u32,param_4: *mut u16,param_5: *mut astruct_76)

{
  astruct_110 *pstruct110_1;
  mut param_1: let mut _seg: u16;

  param_1->field0_0x0 = *param_4;
  param_1_seg = ((u32)param_1 >> 0x10);
  pstruct110_1 = (astruct_110 *)param_1;
  pstruct110_1->field1_0x2 = ((int)param_4 + 0x2);
  pstruct110_1->field2_0x4 = param_3;
  pstruct110_1->field3_0x8 = param_2;
  pstruct110_1->field4_0xc = param_5;
  pstruct110_1->field5_0x10 = 0x0;
  pass1_1008_52fc((astruct_110 *)((u32)param_1 & 0xffff | (u32)param_1_seg << 0x10));
  return;
}
pub fn pass1_1008_5118(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x10) != 0x0) {
    call_fn_ptr_1000_0dc6(*(char **)((int)param_1 + 0x10));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_5134(mut param_1: u32)

{
  let mut puVar1: *mut u16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  i32 lVar4;
  let mut iVar5: i16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut iStack16: i16;
  i32 lStack14;
  let mut uStack10: u32;

  uVar7 = (param_1 >> 0x10);
  iVar6 = (int)param_1;
  lVar4 = *(i32 *)(iVar6 + 0x4) * *(i32 *)(iVar6 + 0x8);
  lVar4 = mem_op_1000_0a48(0x1,lVar4,(int)((u32)lVar4 >> 0x10),_PTR_LOOP_1050_5f2c);
  uVar3 = ((u32)lVar4 >> 0x10);
  (iVar6 + 0x10) = (int)lVar4;
  (iVar6 + 0x12) = uVar3;
  if ((uVar3 | (iVar6 + 0x10)) == 0x0) {
    return;
  }
  iVar5 = (iVar6 + 0x8);
  iVar2 = (iVar6 + 0xa);
  lVar4 = CONCAT22(iVar2 - (iVar5 == 0x0),iVar5 + -0x1) * *(i32 *)(iVar6 + 0x4);
  puVar1 = (u16 *)(iVar6 + 0x10);
  uVar3 = lVar4;
  uStack10 = CONCAT22(((int)((u32)lVar4 >> 0x10) + CARRY2(uVar3,*puVar1)) * 0x100 + (iVar6 + 0x12),
                      uVar3 + *puVar1);
  lStack14 = CONCAT22(iVar2,iVar5);
  iStack16 = (iVar6 + 0x2);
  while (lStack14 != 0x0) {
    iVar2 = iStack16 + 0x1;
    iVar5 = iStack16 >> 0xf;
    pass1_1008_4544(*(astruct_76 **)(iVar6 + 0xc));
    pass1_1000_48a8(uStack10,CONCAT22(iVar5,iStack16),(iVar6 + 0x4));
    iVar5 = (iVar6 + 0x4);
    uVar3 = -iVar5;
    uStack10 = CONCAT22((int)(uStack10 >> 0x10) +
                        (CARRY2(uStack10,uVar3) - ((iVar6 + 0x6) + (iVar5 != 0x0))) * 0x100,
                        uStack10 + uVar3);
    iStack16 = iVar2;
    lStack14 = lStack14 + -0x1;
  }
  return;
}
pub fn pass1_1008_5236(param_1: *mut astruct_109)

{
  let mut puVar1: *mut u16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  i32 lVar4;
  let mut iVar5: i16;
  astruct_109 *pstruct109_6;
  let mut uVar6: u16;
  let mut bVar7: bool;
  let mut iStack12: i16;
  i32 lStack10;
  let mut uStack6: u16;
  let mut iStack4: i16;

  uVar6 = ((u32)param_1 >> 0x10);
  pstruct109_6 = (astruct_109 *)param_1;
  iVar5 = pstruct109_6->field6_0x8;
  iVar2 = pstruct109_6->field7_0xa;
  lVar4 = CONCAT22(iVar2 - (iVar5 == 0x0),iVar5 + -0x1) * *(i32 *)&pstruct109_6->field_0x4;
  puVar1 = &pstruct109_6->field9_0x10;
  uVar3 = lVar4;
  uStack6 = uVar3 + *puVar1;
  iStack4 = ((int)((u32)lVar4 >> 0x10) + CARRY2(uVar3,*puVar1)) * 0x100 + pstruct109_6->field10_0x12;
  lStack10 = CONCAT22(iVar2,iVar5);
  iStack12 = pstruct109_6->field2_0x2;
  while (lStack10 != 0x0) {
    iVar2 = iStack12 + 0x1;
    iVar5 = iStack12 >> 0xf;
    pass1_1008_4544((astruct_76 *)pstruct109_6->field8_0xc);
    pass1_1000_48a8(CONCAT22(iVar5,iStack12),CONCAT22(iStack4,uStack6),&pstruct109_6->field_0x4);
    iVar5 = &pstruct109_6->field_0x4;
    uVar3 = -iVar5;
    bVar7 = CARRY2(uStack6,uVar3);
    uStack6 += uVar3;
    iStack4 += (bVar7 - (pstruct109_6->field5_0x6 + (iVar5 != 0x0))) * 0x100;
    iStack12 = iVar2;
    lStack10 = lStack10 + -0x1;
  }
  return;
}
pub fn pass1_1008_52fc(param_1: *mut astruct_110)

{
  u32 *puVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  i32 lVar4;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut iVar7: i16;
  astruct_110 *pstruct110_8;
  astruct_110 *uVar8;
  let mut uVar9: u32;
  let mut uStack14: u16;
  let mut iStack12: i16;

  uVar8 = (astruct_110 *)((u32)param_1 >> 0x10);
  pstruct110_8 = (astruct_110 *)param_1;
  uVar9 = pass1_1008_4772(pstruct110_8->field4_0xc);
  uVar5 = (uVar9 >> 0x10);
  iVar7 = (int)uVar9;
  iVar6 = (iVar7 + 0x4);
  uVar3 = iVar6 - 0x1;
  iVar6 = (iVar7 + 0x6) - (iVar6 == 0x0);
  lVar4 = *(i32 *)(iVar7 + 0x8) + -0x1;
  uVar2 = param_1->field0_0x0;
  puVar1 = &pstruct110_8->field2_0x4;
  iVar7 = ((int)uVar2 >> 0xf) + ((int)&pstruct110_8->field2_0x4 + 0x2) + CARRY2(uVar2,puVar1);
  if ((iVar6 <= iVar7) && ((iVar6 < iVar7 || (uVar3 < uVar2 + puVar1)))) {
    &pstruct110_8->field2_0x4 = uVar3 - uVar2;
    ((int)&pstruct110_8->field2_0x4 + 0x2) = (iVar6 - ((int)uVar2 >> 0xf)) - (uVar3 < uVar2);
  }
  uVar2 = pstruct110_8->field1_0x2;
  puVar1 = &pstruct110_8->field3_0x8;
  iVar6 = ((int)uVar2 >> 0xf) + ((int)&pstruct110_8->field3_0x8 + 0x2) + CARRY2(uVar2,puVar1);
  iStack12 = (int)((u32)lVar4 >> 0x10);
  if ((iStack12 <= iVar6) && ((uStack14 = lVar4, iStack12 < iVar6 || (uStack14 < uVar2 + puVar1)))) {
    &pstruct110_8->field3_0x8 = uStack14 - uVar2;
    ((int)&pstruct110_8->field3_0x8 + 0x2) = (iStack12 - ((int)uVar2 >> 0xf)) - (uStack14 < uVar2);
  }
  return;
}



u32 * pass1_1008_5394(u32 *param_1)

{
  *param_1 = 0x0;
  return param_1;
}
pub fn pass1_1008_53aa(void)

{
  return;
}
pub fn mci_send_command_1008_53ae(mut param_1: u32,mut param_2: u16 )

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

  local_1e = 0x0;
  uStack22 = 0x28c;
  uStack20 = &DAT_1050_1050;
  uStack18 = param_1;
  uStack14 = 0x0;
  uStack10 = 0x0;
  uStack8 = 0x4000;
  uStack6 = param_2;
  DVar1 = mciSendCommand16(CONCAT22(0x1050,&local_1e),0x30200,0x803,0x0);
  DVar1 = (DVar1 >> 0x10);
  uStack34 = DVar1;
  if (UStack26 != 0x0) {
    if ((DVar1 | uStack34) != 0x0) {
      mciGetErrorString16(0x400,(char *)CONCAT22(0x1050,&stack0xfbce),DVar1);
    }
    pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_2e),NULL,0xc);
    local_2e = param_2;
    uStack44 = 0x0;
    DVar2 = mciSendCommand16(CONCAT22(0x1050,&local_2e),0x2,0x806,UStack26);
    DVar2 = (DVar2 >> 0x10);
    uStack34 = DVar2;
    DVar1 = DVar2;
    if ((DVar2 | uStack34) != 0x0) {
      mciGetErrorString16(0x400,(char *)CONCAT22(0x1050,&stack0xfbce),DVar2);
    }
    local_32 = param_2;
    uStack48 = 0x0;
    w_error = mciSendCommand16(CONCAT22(0x1050,&local_32),0x1,0x804,UStack26);
    DVar1 = (w_error >> 0x10);
    uStack34 = w_error;
    if ((DVar1 | uStack34) != 0x0) {
      mciGetErrorString16(0x400,(char *)CONCAT22(0x1050,&stack0xfbce),w_error);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn init_op_1008_54aa(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,u8 *param_4,char *param_5,u8 *param_6,
                      u8 *param_7)

{
  code **ppcVar1;
  let mut uVar3: u16;
  let mut in_CX: u16;
  let mut in_DX: u16;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  let mut unaff_SI: u16;
  let mut unaff_DI: u16;
  let mut unaff_CS: u16;
  let mut uVar6: u32;
  let mut uVar7: u32;
  let mut in_stack_0000ffea: u16;
  let mut in_stack_0000ffec: u16;
  u32 *puStack12;
  let mut uVar2: u32;

  if (param_6 != NULL) {
    return;
  }
  dos3_call_op_1000_435c(unaff_CS,NULL,unaff_SI,unaff_DI,in_stack_0000ffea,in_stack_0000ffec);
  pass1_1000_4d0c(param_1);
  pass1_1000_1fea();
  _PTR_LOOP_1050_03a0 = mem_op_1000_1902(in_DX,0x0,0x32,0x0,0x12);
  _PTR_LOOP_1050_029c = mem_op_1000_1902((int)(_PTR_LOOP_1050_03a0 >> 0x10),0x0,0x64,0x0,0xc);
  _PTR_LOOP_1050_4fb8 = mem_op_1000_1902((int)(_PTR_LOOP_1050_029c >> 0x10),0x0,0x64,0x0,0x10);
  _PTR_LOOP_1050_68a2 = mem_op_1000_1902((int)(_PTR_LOOP_1050_4fb8 >> 0x10),0x0,0x64,0x0,0xe);
  _PTR_LOOP_1050_5744 = mem_op_1000_1902((int)(_PTR_LOOP_1050_68a2 >> 0x10),0x0,0x1f4,0x0,0x42);
  uVar6 = mem_op_1000_1902((int)(_PTR_LOOP_1050_5744 >> 0x10),0x0,0x32,0x0,0x6);
  PTR_LOOP_1050_576a = (uVar6 >> 0x10);
  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,PTR_LOOP_1050_576a);
  PTR_LOOP_1050_5768 = uVar6;
  HINSTANCE16_1050_038c = (HINSTANCE16)param_7;
  PTR_LOOP_1050_038e = param_6;
  PTR_LOOP_1050_0390 = param_4;
  uVar3 = str_op_1008_60e8(PTR_LOOP_1050_576a,param_5);
  _PTR_LOOP_1050_0392 = CONCAT22((int)paVar5,uVar3);
  mem_op_1000_179c(0xc,paVar5);
  extraout_DX = paVar5 | uVar3;
  if (extraout_DX == 0x0) {
    uVar3 = 0x0;
    extraout_DX = 0x0;
  }
  else {
    struct_op_1008_0000((u16 *)CONCAT22(paVar5,uVar3));
  }
  puStack12 = (u32 *)CONCAT22(extraout_DX,uVar3);
  uVar4 = extraout_DX;
  if (_PTR_LOOP_1050_0392 != 0x0) {
    ppcVar1 = (code **)((int)*puStack12 + 0x4);
    (**ppcVar1)(0x1000,uVar3,extraout_DX,_PTR_LOOP_1050_0392);
  }
  uVar7 = CONCAT22(extraout_DX,uVar3);
  uVar2 = *puStack12;
  ppcVar1 = (code **)uVar2 + 0x4;
  (**ppcVar1)();
  win_msg_op_1008_9498();
  if (puStack12 != NULL) {
    ppcVar1 = (code **)uVar2;
    (**ppcVar1)(0x1000,uVar3,extraout_DX,0x1,uVar7);
  }
  uVar6 = mem_op_1000_1b68(uVar4,_PTR_LOOP_1050_03a0,(_PTR_LOOP_1050_03a0 >> 0x10));
  uVar6 = mem_op_1000_1b68((uVar6 >> 0x10),_PTR_LOOP_1050_029c,(_PTR_LOOP_1050_029c >> 0x10));
  uVar6 = mem_op_1000_1b68((uVar6 >> 0x10),_PTR_LOOP_1050_4fb8,(_PTR_LOOP_1050_4fb8 >> 0x10));
  uVar6 = mem_op_1000_1b68((uVar6 >> 0x10),_PTR_LOOP_1050_68a2,(_PTR_LOOP_1050_68a2 >> 0x10));
  mem_op_1000_1b68((uVar6 >> 0x10),_PTR_LOOP_1050_5744,(_PTR_LOOP_1050_5744 >> 0x10));
  return;
}
pub fn def_win_proc_1008_5632(LPARAM param_1,WPARAM param_2,mut param_3: u16 ,HWND16 param_4)

{
  code **ppcVar1;
  u32 *puStack6;
  let mut uVar2: u16;

  uVar2 = &DAT_1050_1050;
  puStack6 = (u32 *)GetWindowLong16(0x0,param_4);
  if ((((u32)puStack6 >> 0x10) | puStack6) == 0x0) {
    if (param_3 != 0x1) {
      DefWindowProc16(param_1,param_2,param_3,param_4);
      return;
    }
    puStack6 = (u32*)param_1;
    SetWindowLong16((long)puStack6,0x0,param_4);
    pass1_1008_9628(puStack6,param_4);
  }
  ppcVar1 = (code **)((int)*puStack6 + 0x1c);
  (**ppcVar1)((int)s_tile2_bmp_1050_1538,(int)puStack6,(int)((u32)puStack6 >> 0x10),param_1,param_2,param_3,uVar2);
  return;
}



u16 * struct_op_1008_56b4(param_1: *mut astruct_76)

{
  astruct_76 *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_76 *)param_1;
  param_1->offset_0x0 = 0x389a;
  iVar1->base_0x2 = 0x1008;
  iVar1->field2_0x4 = 0x0;
  param_1->offset_0x0 = s__s__d_1050_573a;
  iVar1->base_0x2 = 0x1008;
  return &param_1->offset_0x0;
}



BOOL16 cleanup_palette_1008_56e2(mut param_1: u32,mut param_2: u32)

{
  HPALETTE16 hpalette_a;
  let mut u16_a: u16;

  u16_a = (param_1 >> 0x10);
  hpalette_a = SelectPalette16(0x0,*(HPALETTE16 *)((int)param_1 + 0x4),*(HDC16 *)param_2);
  *(HPALETTE16 *)((int)param_1 + 0x4) = hpalette_a;
  DeleteObject16(hpalette_a);
  return 0x1;
}



u16 * pass1_1008_570e(param_1: *mut u16,param_2: u8)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn set_struct_1008_574a(param_1: *mut astruct_57)

{
  StructD *iVar1;
  StructD *uVar1;

  uVar1 = (StructD *)((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  (u32)&iVar1->hfile_0x4 = 0x0;
  iVar1->field5_0x8 = 0x0;
  iVar1->field6_0xa = 0x1;
    // just 0x5bc4
  param_1->field0_0x0 = (int)s__s__s__1050_5bc0 + 0x4;
  iVar1->address_offset_field_0x2 = 0x1008;
  return;
}
pub fn pass1_1008_5784(char *param_1,mut param_2: u32)

{
  (u32)param_1 = param_2;
  (u32)((int)param_1 + 0x4) = 0x0;
  return;
}
pub fn pass1_1008_57a4(u32 *param_1,mut param_2: u32)

{
  *param_1 = param_2;
  (u32)((int)param_1 + 0x4) = 0x0;
  return;
}
pub fn pass1_1008_57c4(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = (int)s__s__s__1050_5bc0 + 0x4;
  ((int)param_1 + 0x2) = 0x1008;
  pass1_1008_5830((u32)param_1 & 0xffff | (u32)uVar1 << 0x10);
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  return;
}



i32 pass1_1008_57f0(mut param_1: u32,mut param_2: i16)

{
  let mut bVar1: bool;
  i32 lVar2;
  let mut iStack12: i16;
  u8 local_a [0x8];

  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),param_1);
  iStack12 = 0x0;
  do {
    lVar2 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    if (lVar2 == 0x0) {
      return 0x0;
    }
    bVar1 = iStack12 != param_2;
    iStack12 = iStack12 + 0x1;
  } while (bVar1);
  return lVar2;
}
pub fn pass1_1008_5830(mut param_1: u32)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut uVar4: u32;
  u32 *puVar5;
  let mut iVar6: i16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uVar9: u16;

  while( true ) {
    uVar8 = (param_1 >> 0x10);
    iVar6 = (int)param_1;
    if (*(i32 *)(iVar6 + 0x4) == 0x0) break;
    if ((iVar6 + 0xa) != 0x0) {
      uVar4 = (u32)(iVar6 + 0x4);
      uVar9 = ((u32)uVar4 >> 0x10);
      iVar7 = (int)uVar4;
      puVar1 = (u32 *)(iVar7 + 0x8);
      uVar2 = (iVar7 + 0xa);
      if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)();
      }
    }
    puVar5 = (u32 *)*(i32 *)(iVar6 + 0x4);
    (u32)(iVar6 + 0x4) = (u32)((int)puVar5 + 0x4);
    if (puVar5 != NULL) {
      ppcVar3 = (code **)*puVar5;
      (**ppcVar3)();
    }
  }
  (iVar6 + 0x8) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_58a6(mut param_1: u32,mut param_2: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  astruct_99 *paStack6;

  paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_029c);
  uVar3 = ((u32)paStack6 >> 0x10);
  uVar2 = paStack6;
  if ((uVar3 | uVar2) == 0x0) {
    paStack6 = NULL;
  }
  else {
    paStack6->field0_0x0 = 0x389a;
    (uVar2 + 0x2) = 0x1008;
    (u32)(uVar2 + 0x4) = 0x0;
    (u32)(uVar2 + 0x8) = 0x0;
    paStack6->field0_0x0 = s__s__s__1050_5bc0;
    (uVar2 + 0x2) = 0x1008;
  }
  if (paStack6 == NULL) {
    return;
  }
  uVar5 = ((u32)paStack6 >> 0x10);
  (u32)((int)paStack6 + 0x8) = param_2;
  uVar6 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  (u32)((int)paStack6 + 0x4) = (u32)(iVar4 + 0x4);
  *(astruct_99 **)(iVar4 + 0x4) = paStack6;
  piVar1 = (iVar4 + 0x8);
  *piVar1 = *piVar1 + 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_593c(u32 *param_1,mut param_2: u32)

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  astruct_99 *paStack6;

  uVar6 = ((u32)param_1 >> 0x10);
  iVar5 = (int)param_1;
  if ((iVar5 + 0x8) == 0x0) {
    ppcVar2 = (code **)((int)*param_1 + 0x4);
    (**ppcVar2)();
    return;
  }
  paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_029c);
  uVar4 = ((u32)paStack6 >> 0x10);
  uVar3 = paStack6;
  if ((uVar4 | uVar3) == 0x0) {
    paStack6 = NULL;
  }
  else {
    paStack6->field0_0x0 = 0x389a;
    (uVar3 + 0x2) = 0x1008;
    (u32)(uVar3 + 0x4) = 0x0;
    (u32)(uVar3 + 0x8) = 0x0;
    paStack6->field0_0x0 = s__s__s__1050_5bc0;
    (uVar3 + 0x2) = 0x1008;
  }
  if (paStack6 == NULL) {
    return;
  }
  (u32)((int)paStack6 + 0x8) = param_2;
  do {
    param_1 = (u32*)((int)param_1 + 0x4);
    uVar7 = ((u32)param_1 >> 0x10);
  } while (*(i32 *)((int)param_1 + 0x4) != 0x0);
  *(astruct_99 **)((int)param_1 + 0x4) = paStack6;
  piVar1 = (iVar5 + 0x8);
  *piVar1 = *piVar1 + 0x1;
  return;
}
pub fn pass1_1008_59f4(mut param_1: u32,i32 param_2)

{
  let mut piVar1: *mut i16;
  u32 *puVar2;
  let mut uVar3: u16;
  u32 *puVar4;
  code **ppcVar5;
  u32 *puVar6;
  let mut uVar7: u16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uStack10: u16;
  u32 *puStack6;

  puStack6 = NULL;
  uVar9 = (param_1 >> 0x10);
  puVar6 = puStack6;
  puVar4 = (u32 *)param_1;
  do {
    puStack6 = puVar6;
    uVar10 = ((u32)puVar4 >> 0x10);
    iVar8 = (int)puVar4;
    puVar4 = (u32 *)*(i32 *)(iVar8 + 0x4);
    uStack10 = puVar4;
    uVar11 = ((u32)puVar4 >> 0x10);
    if (((iVar8 + 0x6) | uStack10) == 0x0) break;
    puVar6 = puVar4;
  } while (*(i32 *)(uStack10 + 0x8) != param_2);
  if (puVar4 != NULL) {
    if (puStack6 == NULL) {
      uVar10 = (uStack10 + 0x4);
      uVar7 = (uStack10 + 0x6);
      puStack6 = (u32 *)param_1;
    }
    else {
      uVar10 = (uStack10 + 0x4);
      uVar7 = (uStack10 + 0x6);
    }
    uVar12 = ((u32)puStack6 >> 0x10);
    ((int)puStack6 + 0x4) = uVar10;
    ((int)puStack6 + 0x6) = uVar7;
    if (((int)param_1 + 0xa) != 0x0) {
      puVar2 = (u32 *)(uStack10 + 0x8);
      uVar3 = (uStack10 + 0xa);
      if ((uVar3 | puVar2) != 0x0) {
        ppcVar5 = (code **)*puVar2;
        (**ppcVar5)();
      }
    }
    if (puVar4 != NULL) {
      ppcVar5 = (code **)*puVar4;
      (**ppcVar5)();
    }
    piVar1 = ((int)param_1 + 0x8);
    *piVar1 = *piVar1 + -0x1;
    return;
  }
  return;
}
pub fn pass1_1008_5ab8(mut param_1: u32)

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  u32 *puVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(i32 *)(iVar4 + 0x4) == 0x0) {
    return;
  }
  puVar3 = (u32 *)(u32)(iVar4 + 0x4);
  uVar6 = ((u32)puVar3 >> 0x10);
  (u32)(iVar4 + 0x4) = (u32)(puVar3 + 0x4);
  if ((uVar6 | puVar3) != 0x0) {
    ppcVar2 = (code **)*puVar3;
    (**ppcVar2)();
  }
  piVar1 = (iVar4 + 0x8);
  *piVar1 = *piVar1 + -0x1;
  return;
}
pub fn pass1_1008_5b12(char *param_1)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u16;

  if ((*(i32 *)param_1 != 0x0) && (((int)(u32)param_1 + 0x8) != 0x0)) {
    uVar4 = ((u32)param_1 >> 0x10);
    iVar2 = (int)param_1;
    if (*(i32 *)(iVar2 + 0x4) == 0x0) {
      uVar5 = ((u32)(u32)param_1 >> 0x10);
      iVar3 = (int)(u32)param_1;
    }
    else {
      uVar1 = (u32)(iVar2 + 0x4);
      uVar5 = ((u32)uVar1 >> 0x10);
      iVar3 = (int)uVar1;
    }
    (u32)(iVar2 + 0x4) = (u32)(iVar3 + 0x4);
    if (*(i32 *)(iVar2 + 0x4) != 0x0) {
      return;
    }
  }
  return;
}



u16 * pass1_1008_5b6e(param_1: *mut u16,param_2: u8)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a((astruct_611 *)param_1);
  }
  return param_1;
}



StructD * pass1_1008_5b9a(StructD *param_1,param_2: u8)

{
  pass1_1008_57c4(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_5bdc(char *param_1)

{
  let mut in_EDX: u32;
  let mut uVar1: u16;
  astruct_19 *pstruct19_1;
  let mut unaff_BP: u16;
  astruct_19 *pstruct19_param_1;
  astruct_19 *paVar2;
  u32 *puVar3;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  uVar1 = ((u32)in_EDX >> 0x10);
  paVar2 = struct_op_1010_1d48((astruct_19 *)param_1,0x44);
  pstruct19_param_1 = (astruct_19 *)((u32)param_1 >> 0x10);
  pstruct19_1 = (astruct_19 *)param_1;
  pstruct19_1->horiz_res_0xa = 0x0;
  (u32)&pstruct19_1->ver_res_0xc = 0x0;
  pstruct19_1->field8_0x10 = 0x0;
  pstruct19_1->field9_0x12 = 0x0;
  param_1 = 0x5fc8;
  pstruct19_1->segment_0x2 = 0x1008;
  _u16_1050_02a0 = param_1;
  puVar3 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar1,(int)((u32)paVar2 >> 0x10)),_u16_1050_0ed0,
                           (u8 **)CONCAT22(unaff_BP,0x2),in_stack_0000fea6,in_stack_0000ffca,in_stack_0000ffd0,
                           in_stack_0000ffd4);
  pstruct19_1->ver_res_0xc = puVar3;
  &pstruct19_1->field_0xe = (int)((u32)puVar3 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1008_5c34(char *param_1)

{
  param_1 = 0x5fc8;
  ((int)param_1 + 0x2) = 0x1008;
  _u16_1050_02a0 = 0x0;
  pass1_1010_1d80((StructD *)param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_1008_5c5c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 )

{
  pass1_1010_84f8(_u16_1050_14cc,param_4);
  win_ui_op_1008_5cfe(param_3,CONCAT22(param_2,param_1),(WNDCLASS16 *)&DAT_1050_1050);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_1008_5c7c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  pass1_1010_85be(_u16_1050_14cc,(int)param_4,(int)(param_4 >> 0x10));
  win_ui_op_1008_5cfe(param_3,CONCAT22(param_2,param_1),(WNDCLASS16 *)&DAT_1050_1050);
  return;
}
pub fn win_1008_5c9e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,u32 *param_4)

{
  win_1008_5c7c(param_1,param_2,param_3,*param_4);
  return;
}
pub fn mci_send_command_1008_5cb6(param_1: *mut astruct_27,mut param_2: i16)

{
  astruct_27 *iVar1;
  let mut uVar1: u16;
  let mut iVar2: i16;

  mciSendCommand16(0x0,0x0,0x804,param_2);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_27 *)param_1;
  if ((&iVar1->field_0xa == 0x0) || (&iVar1->field_0xa != param_2)) {
    iVar1->field18_0x12 = 0x0;
    iVar2 = 0x11;
  }
  else {
    &iVar1->field_0x10 = 0x0;
    iVar2 = 0x10;
  }
  pass1_1010_1f62(param_1,iVar2);
  return;
}
pub fn win_ui_op_1008_5cfe(param_1: *mut astruct_27,char *param_2,WNDCLASS16 *in_wnd_class)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  astruct_27 *iVar3;
  let mut uVar3: u16;
  let mut DVar4: u32;
  let mut iVar5: i16;
  HWND16 message_1;
  let mut uStack298: u16;
  HWND16 window_handle_1;
  u8 local_11e [0x100];
  char *string_1;
  let mut iStack26: i16;
  let mut iStack24: i16;
  u8 local_16 [0x4];
  let mut offset_val: u16;
  char *pcStack14;
  char *pcStack10;

  pass1_1000_4906((StructD *)CONCAT22(0x1050,local_16),NULL,0x14);
  pcStack10 = param_2;
  uVar3 = ((u32)param_1 >> 0x10);
  iVar3 = (astruct_27 *)param_1;
  uVar1 = (u32)&iVar3->field_0xc;
  iStack24 = ((int)uVar1 + 0x72);
  iStack26 = 0x1;
  string_1 = s_waveaudio_1050_02a4;
  str_1000_4d58(param_2,NULL,0x0,0x0,(WNDCLASS16 *)CONCAT22(0x1050,local_11e));
  iVar2 = pass1_1000_475e(CONCAT22(0x1050,local_11e),(u32)s__mid_1050_02ae);
  if (iVar2 == 0x0) {
    uVar1 = (u32)&iVar3->field_0xc;
    iStack24 = ((int)uVar1 + 0x74);
    string_1 = s_sequencer_1050_02b3;
    iStack26 = 0x0;
  }
  if (iStack24 != 0x0) {
    if ((iStack26 != 0x0) && (&iVar3->field_0x10 != 0x0)) {
      return;
    }
    if ((iStack26 == 0x0) && (iVar3->field18_0x12 != 0x0)) {
      return;
    }
    pcStack14 = string_1;
    DVar4 = mciSendCommand16(CONCAT22(0x1050,local_16),0x2200,0x803,0x0);
    if (((DVar4 >> 0x10) | DVar4) == 0x0) {
      if (iStack26 == 0x0) {
        iVar3->field18_0x12 = 0x1;
      }
      else {
        &iVar3->field_0xa = offset_val;
        &iVar3->field_0x10 = 0x1;
      }
      window_handle_1 = create_window_1008_5e7e();
      if (window_handle_1 == 0x0) {
        mci_send_command_1008_5cb6(param_1,offset_val);
        return;
      }
      pass1_1000_4906((StructD *)CONCAT22(0x1050,&message_1),NULL,0xc);
      message_1 = window_handle_1;
      uStack298 = 0x0;
      mciSendCommand16(CONCAT22(0x1050,&message_1),0x1,0x806,offset_val);
      SetWindowWord16(offset_val,0x0,window_handle_1);
      return;
    }
  }
  if (iStack26 == 0x0) {
    iVar5 = 0x11;
  }
  else {
    iVar5 = 0x10;
  }
  pass1_1010_1f62(param_1,iVar5);
  return;
}



HWND16 create_window_1008_5e7e(void)

{
  u32 *puVar2;
  let mut BVar3: bool;
  ATOM AVar4;
  HWND16 window_handle_1;
  let mut iVar5: i16;
  char *string_1;
  u32 *puVar5;
  WNDCLASS16 wndclass_44;
  u32 local_12 [0x4];
  char *puVar1;

  puVar5 = local_12;
  string_1 = (char *)s_MciSoundWindow_1050_02bd;
  for (iVar5 = 0x3; iVar5 != 0x0; iVar5 += -0x1) {
    puVar2 = puVar5;
    puVar5 = puVar5 + 0x1;
    puVar1 = string_1;
    string_1 = (char *)((int)string_1 + 0x4);
    *puVar2 = (u32)puVar1;
  }
  puVar5 = string_1;
  *(u16 *)((int)puVar5 + 0x2) = *(u16 *)((int)string_1 + 0x2);
  wndclass_44.style = 0x2000;
  wndclass_44.lpfn_wnd_proc._0_2_ = SUB42(&DAT_1050_5f44,0x0);
  wndclass_44.lpfn_wnd_proc = 0x1008;
  wndclass_44.cb_wnd_extra = 0x2;
  wndclass_44.h_instance = HINSTANCE16_1050_038c;
  wndclass_44.h_icon = 0x0;
  wndclass_44.h_cursor = 0x0;
  wndclass_44.cb_cls_extra = 0x0;
  wndclass_44.hbr_background = GetStockObject16(WHITE_BRUSH);
  wndclass_44.lpsz_menu_name = 0x0;
  wndclass_44.lpsz_class_name._0_2_ = local_12;
  wndclass_44.lpsz_class_name = SUB42(&DAT_1050_1050,0x0);
  BVar3 = GetClassInfo16(&wndclass_44,(char *)CONCAT22((u32 *)wndclass_44.lpsz_class_name,0x1050),
                         (HINSTANCE16)&DAT_1050_1050);
  if (BVar3 == 0x0) {
    AVar4 = RegisterClass16(&wndclass_44);
    if (AVar4 == 0x0) {
      OutputDebugString16(s_MciSound_registerClass_failed_1050_02cc);
      return 0x0;
    }
  }
  window_handle_1 =
       CreateWindow16(0x0,(void *)(u32)HINSTANCE16_1050_038c,HWND16_1050_0396,0x1,0x1,-0x8000,-0x8000,0x0,0xcf,
                      s_MciSound_registerClass_failed_1050_02cc + 0x1e,(char *)CONCAT22(0x1050,local_12));
  return window_handle_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT make_def_win_proc_1008_5f44
                  (mut param_1: u16 ,mut param_2: u16 ,LPARAM param_3,WPARAM16 in_wparam_2,mut param_5: u16 ,HWND16 param_6)

{
  WORD WVar1;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar2;
  LRESULT LVar3;
  u32 *puVar4;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut in_stack_0000fff8: u16;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  if (param_5 == 0x2) {
    WVar1 = GetWindowWord16(0x0,param_6);
    mci_send_command_1008_5cb6(_u16_1050_02a0,WVar1);
    puVar4 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x37),in_stack_0000fea0,
                             in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
    pass1_1008_aa28(puVar4,(u32)puVar4);
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



pub fn pass1_1008_5fa2(mut param_1: u32,param_2: u8) -> u32

{
  pass1_1008_5c34((char *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u8 * pass1_1008_5fd8(u8 *param_1)

{
  let mut puVar1: *mut u16;
  u8 *puVar2;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  char *pcVar4;
  let mut in_stack_00000004: u16;
  let mut puStack6: *mut u16;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  puVar2 = &stack0x0006;
  puStack6 = (u16 *)CONCAT22(0x1050,puVar2);
  mem_op_1000_179c(0x1000,paVar3);
  pcVar4 = load_string_1010_847e(_u16_1050_14cc,in_stack_00000004);
  unk_str_op_1000_3d3e((char *)CONCAT22((int)paVar3,puVar2),pcVar4);
  while( true ) {
    puVar1 = puStack6;
    puStack6 = (u16 *)((u32)puStack6 & 0xffff0000 | (u32)((int)puStack6 + 0x2));
    if (*puVar1 == 0x0) break;
    pcVar4 = load_string_1010_847e(_u16_1050_14cc,*puVar1);
    pass1_1000_3cea(CONCAT22((int)paVar3,puVar2),pcVar4);
  }
  return puVar2;
}

