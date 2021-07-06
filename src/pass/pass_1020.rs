
astruct_18 *  pass1_1020_0d82(astruct_18 *param_1,param_2: u8)

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1.field_0x0 = 0x3ab0;
  (param_1 + 0x2) = 0x1008;
  param_1.field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1020_0dc4(param_1: *mut u16,param_2: u16,param_3: u32,param_4: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  struct_1020_790e(param_1,s_PCPOPMENU_1050_4256,param_2,param_3,param_4);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0xf2) = 0x0;
  (iVar1 + 0xf6) = 0x0;
  (iVar1 + 0xfa) = 0x0;
  *param_1 = 0x1384;
  (iVar1 + 0x2) = 0x1020;
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | (iVar1 + 0x5b)),
             s_VrMode_1050_4260);
  (iVar1 + 0xac) = 0x44c00000;
  window_op_1020_10a0(param_1);
  return;
}



fn pass1_1020_0e2c(param_1: u32,param_2: u16)
{
  get_win_ui_info_op_1020_7a50(param_1,param_2);
  cleanup_ui_op_1020_1038(param_1);
  return;
}


void 
pass1_1020_0e8e(param_1: i16,param_2: u16,param_3: i16,param_4: i16,param_5: i16,
               param_6: u16,param_7: u16)

{
  code **ppcVar1;
  
  win_ui_cursor_op_1020_1294(CONCAT22(param_2,param_1),param_3,param_4,param_6,param_7);
  if (param_5 == 0x0) {
    ppcVar1 = ((param_1 + 0x4) + 0x5c);
    (**ppcVar1)();
  }
  return;
}


fn pass1_1020_1022(param_1: u32,param_2: u16)
{
  draw_op_1020_15de((param_1 + 0xf6),param_2);
  return;
}


astruct_3 *  pass1_1020_135e(astruct_3 *param_1,param_2: u8,param_3: u16)

{
  cleanup_menu_ui_op_1020_795c(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


astruct_18 * 
pass1_1020_170a(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  win_ui_op_1020_150e(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1020_1780(param_1: *mut u32)
{
  code **ppcVar1;
  
  ppcVar1 = (*param_1 + 0x6c);
  (**ppcVar1)();
  destroy_win_1040_8212((ULONG)param_1,&ctx.PTR_LOOP_1050_1040);
  return;
}


fn pass1_1020_1b68(param_1: u32,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0x92);
  uVar2 = (iVar4 + 0x94);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  (iVar4 + 0x92) = 0x0;
  pass1_1010_4f48((iVar4 + 0x8e),param_2);
  (iVar4 + 0x8e) = 0x0;
  return;
}



fn pass1_1020_1bb6(param_1: u32) -> u16

{
  code **ppcVar1;
  
  ppcVar1 = ((param_1 + 0x92) + 0x8);
  (**ppcVar1)();
  return 0x0;
}


fn pass1_1020_1d8e(param_1: u32,param_2: u32)
{
  pt_in_rect_1010_4e08
            ((param_1 + 0x8e),param_2,(param_2 >> 0x10),
             0x1010);
  return;
}



fn pass1_1020_1da8(param_1: u32,param_2: i16,param_3: u16,param_4: u16) -> u16

{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  uVar1 = (iVar2 + 0x8e);
  if ((uVar1 + 0x30) == 0x1) {
    return 0x1;
  }
  uVar1 = (iVar2 + 0x8e);
  if (((uVar1 + 0x30) < 0x3) &&
     (pass1_1010_4df0((iVar2 + 0x8e),param_3,param_4), param_2 == 0x0)) {
    return 0x1;
  }
  return 0x0;
}


astruct_18 *  pass1_1020_1e54(astruct_18 *param_1,param_2: u8)

{
  ui_cleanup_op_1040_782c(param_1,&ctx.PTR_LOOP_1050_1040);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_1eea(param_1: *mut u16,param_2: u32,param_3: u16,uchar *param_4,param_5: i16,
               param_6: u16)

{
  code **ppcVar1;
  let uVar2: u16;
  astruct_663 *iVar3;
  let uVar3: u16;
  let puVar4: *mut u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_663 *)param_1;
  *param_1 = 0x389a;
  iVar3.field_0x2 = 0x1008;
  *param_1 = 0x3aa8;
  iVar3.field_0x2 = 0x1008;
  iVar3.field_0x4 = param_3;
  *param_1 = 0x3ab0;
  iVar3.field_0x2 = 0x1008;
  iVar3.field_0x6 = 0x0;
  iVar3.field_0xa = param_2;
  *param_1 = 0x2518;
  iVar3.field_0x2 = 0x1020;
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x39,param_6,param_4,param_5);
  uVar2 = (puVar4 >> 0x10);
  &iVar3.field_0x6 = puVar4;
  (&iVar3.field_0x6 + 0x2) = uVar2;
  ppcVar1 = (*iVar3.field_0x6 + 0x4);
  (**ppcVar1)(0x1010,&iVar3.field_0x6,uVar2,0x0,param_1);
  return;
}



fn pass1_1020_1f74(param_1: *mut u16,param_2: u16)
{
  astruct_582 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_582 *)param_1;
  *param_1 = 0x2518;
  iVar1.field_0x2 = 0x1020;
  pass1_1010_1ea6(iVar1.field_0x6,param_1 & 0xffff | uVar1 << 0x10,param_2)
  ;
  *param_1 = 0x3ab0;
  iVar1.field_0x2 = 0x1008;
  *param_1 = 0x389a;
  iVar1.field_0x2 = 0x1008;
  return;
}


fn pass1_1020_2286(param_1: u16,param_2: u16,i16 *param_3,param_4: i16)
{
  *param_3 = 0x64 - param_4 >> 0x1;
  return;
}


fn pass1_1020_239c(param_1: u32,i16 *param_2,param_3: u16)
{
  let puVar1: *mut u16;
  let uVar2: u32;
  let uVar3: u16;
  let local_a: [u8;6];
  let uStack4: u16;
  
  if (param_2 != 0x0) {
    uStack4 = ((param_2 + 0x4) - *param_2 >> 0x1) + *param_2;
    puVar1 = pass1_1008_3e54(CONCAT22(param_3,local_a),0x0,0x57,uStack4);
    uVar3 = (param_1 >> 0x10);
    uVar2 = pass1_1020_23f2(param_1,uVar3,CONCAT22(param_3,local_a),
                            (puVar1 >> 0x10),param_3);
    draw_polygon_1020_2474(param_1,uVar3,CONCAT22(uVar2,0x3),0x1008);
  }
  return;
}



u32 
pass1_1020_23f2(param_1: u16,param_2: u16,param_3: *mut u16,uchar *param_4,
               param_5: u16)

{
  let piVar1: *mut i16;
  let iStack18: i16;
  let local_6: i16;
  let local_4: i16;
  
  piVar1 = &local_6;
  pass1_1008_3e94(param_3,CONCAT22(param_5,piVar1),
                  CONCAT22(param_5,&local_4));
  mem_op_1000_179c(0xc,param_4,0x1000);
  for (iStack18 = 0x0; iStack18 < 0x3; iStack18 += 0x1) {
    piVar1[iStack18 * 0x2] = (iStack18 * 0x4 + 0x4270) + local_4;
    piVar1[iStack18 * 0x2 + 0x1] = (iStack18 * 0x4 + 0x4272) + local_6;
  }
  return CONCAT22(param_4,piVar1);
}


fn pass1_1020_2488(param_1: u32,param_2: u16,param_3: u16)
{
  let uVar1: u32;
  let in_dlg_id_5: u16;
  let uVar2: u16;
  let iVar3: i16;
  let uVar4: u16;
  let iStack12: i16;
  SEGPTR SStack10;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  find_n_load_rsrc_1010_4e9e((iVar3 + 0x6),0x1010);
  if ((param_3 | param_2) != 0x0) {
    SStack10 = param_2;
    for (iStack12 = 0x0; iStack12 < 0x9; iStack12 += 0x1) {
      uVar1 = (iVar3 + 0x6);
      in_dlg_id_5 = pass1_1010_4f20(uVar1,(uVar1 >> 0x10),iStack12)
      ;
      uVar1 = (iVar3 + 0xa);
      set_win_tet_1020_1d2a
                (uVar1,(uVar1 >> 0x10),SStack10,param_3,in_dlg_id_5
                 ,0x1010);
      uVar2 = str_op_1000_3da4(CONCAT22(param_3,SStack10));
      SStack10 += uVar2 + 0x1;
    }
  }
  return;
}



astruct_18 * 
pass1_1020_24f2(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  pass1_1020_1f74(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1020_2594(param_1: *mut u16)
{
  astruct_583 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_583 *)param_1;
  *param_1 = 0x270c;
  iVar1.field_0x2 = 0x1020;
  iVar1.field_0xe2 = 0x27a8;
  iVar1.field_0xe4 = 0x1020;
  pass1_1020_808e(param_1);
  return;
}



fn pass1_1020_25c0(param_1: u32,param_2: u16,param_3: u16)
{
  let piVar1: *mut i16;
  code **ppcVar2;
  let uVar3: u16;
  let uVar4: u16;
  astruct_277 *iVar3;
  let uVar5: u16;
  astruct_57 *paVar6;
  let puStack6: u32;
  
  paVar6 = (astruct_57 *)CONCAT22(param_3,param_2);
  uVar5 = (param_1 >> 0x10);
  iVar3 = (astruct_277 *)param_1;
  if (iVar3.field_0xee != 0x0) {
    ppcVar2 = (*iVar3.field_0xee + 0x8);
    paVar6 = (astruct_57 *)(**ppcVar2)();
  }
  if (iVar3.field_0xea == 0x0) {
    iVar3.field_0xea = 0x1;
    mem_op_1000_179c(0x98,(paVar6 >> 0x10),0x1000);
    uVar3 = paVar6;
    uVar4 = (paVar6 >> 0x10) | uVar3;
    if (paVar6 == (astruct_57 *)0x0) {
      puStack6 = 0x0;
    }
    else {
      piVar1 = &iVar3.field_0xcc;
      *piVar1 = *piVar1 + 0x1;
      struct_1020_1738(paVar6,iVar3.field_0xcc,param_1);
      puStack6 = CONCAT22(uVar4,uVar3);
    }
    ppcVar2 = (*puStack6 + 0x8);
    (**ppcVar2)(0x1000,puStack6,(puStack6 >> 0x10));
  }
  return;
}


fn pass1_1020_26a6(Uparam_1: i32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  puVar1 = (param_1 + 0xee);
  uVar2 = (param_1 + 0xf0);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  destroy_win_1008_628e(param_1,0x1008);
  return;
}



astruct_18 *  pass1_1020_26d8(astruct_18 *param_1,param_2: u8)

{
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
  pass1_1020_2594(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_27b0(astruct_664 *param_1,param_2: u16,param_3: u16,param_4: i16,
               param_5: u16)

{
  code **ppcVar1;
  let uVar2: u32;
  let iVar3: i16;
  let extraout_DX: *mut u8
  let uVar4: u16;
  let puVar5: *mut u16;
  
  set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2,param_1),param_3);
  &param_1.field_0x14 = 0x0;
  CONCAT22(param_2,param_1) = 0x288e;
  param_1.field_0x2 = 0x1020;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2a,param_5,extraout_DX,param_4);
  uVar4 = (puVar5 >> 0x10);
  param_1.field_0x14 = puVar5;
  param_1.field_0x16 = uVar4;
  param_1.field_0x6 = param_1.field_0x14;
  param_1.field_0x8 = uVar4;
  uVar2 = &param_1.field_0x14;
  iVar3 = &param_1.field_0xa;
  ppcVar1 = ((uVar2 + 0xa) + 0x8);
  (**ppcVar1)();
  param_1.field_0x12 = iVar3;
  draw_op_1020_9364((astruct_7 *)CONCAT22(param_2,param_1),0x1010,param_5);
  return;
}



fn pass1_1020_2838(param_1: *mut u16,param_2: u16)
{
  astruct_584 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_584 *)param_1;
  *param_1 = 0x288e;
  iVar1.field_0x2 = 0x1020;
  if (iVar1.field_0x14 != 0x0) {
    param_2 = 0x1010;
    pass1_1010_1dda(iVar1.field_0x14);
  }
  palette_op_1020_92c4(param_1,param_2);
  return;
}



astruct_18 * 
pass1_1020_2868(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  pass1_1020_2838(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1020_289a(param_1: *mut u16,param_2: u16,param_3: u32,param_4: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  struct_1020_790e(param_1,s_SCPOPMENU_1050_427c,param_2,param_3,param_4);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0xf2) = 0x0;
  (iVar1 + 0xf6) = 0x0;
  (iVar1 + 0xfa) = 0x0;
  (iVar1 + 0xfc) = 0x0;
  *param_1 = 0x2e4a;
  (iVar1 + 0x2) = 0x1020;
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | (iVar1 + 0x5b)),
             s_VrMode_1050_4286);
  (iVar1 + 0xac) = 0x44c00000;
  return;
}



fn pass1_1020_28fc(astruct_3 *param_1,param_2: u16)
{
  param_1.address_offset_field_0x0 = 0x2e4a;
  (param_1 + 0x2) = 0x1020;
  cleanup_menu_ui_op_1020_795c(param_1,param_2);
  return;
}


fn pass1_1020_2936(void)
{
  pass1_1020_79ae();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_294a(param_1: u32,param_2: u32,param_3: u16,uchar *param_4,param_5: i16,
               param_6: u16)

{
  let uVar1: u16;
  astruct_665 *iVar3;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar3 = (astruct_665 *)param_1;
  iVar3.field_0xfc = param_3;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,param_3,param_6,param_4,param_5);
  uVar1 = (puVar3 >> 0x10);
  iVar3.field_0xf2 = puVar3;
  iVar3.field_0xf4 = uVar1;
  iVar3.field_0xe0 = iVar3.field_0xf2;
  iVar3.field_0xe2 = uVar1;
  pass1_1018_0902(*(u32 **)&iVar3.field_0xf2,param_2);
  return;
}


fn pass1_1020_2a46(param_1: u16,param_2: u16,param_3: i16)
{
  pass1_1018_0ae8((param_1 + 0xf2),0x1);
  pass1_1008_68c6(param_1,param_2,param_3,0x1008);
  return;
}



fn pass1_1020_2a6a(param_1: u32,param_2: u16)
{
  get_win_ui_info_op_1020_7a50(param_1,param_2);
  pass1_1018_0ae8((param_1 + 0xf2),0x0);
  destroy_icon_1020_2c88(param_1,0x1018);
  return;
}



fn pass1_1020_2a94(param_1: u32,param_2: u32,param_3: u16)
{
  pass1_1018_1662((param_1 + 0xf2),param_2,(param_2 >> 0x10),
                  param_3);
  return;
}


fn pass1_1020_2c72(param_1: u32,param_2: u16,param_3: u16)
{
  draw_op_1020_30be((param_1 + 0xf6),param_2,param_3);
  return;
}


astruct_3 *  pass1_1020_2e24(astruct_3 *param_1,param_2: u8)

{
  let unaff_CS: u16;
  
  pass1_1020_28fc(param_1,unaff_CS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


void 
pass1_1020_3540(param_1: u16,param_2: u16,param_3: i16,param_4: *mut u16,uchar *param_5,
               param_6: u16)

{
  let uVar1: u16;
  astruct_279 *iVar2;
  let iStack18: i16;
  let iStack12: i16;
  let uStack10: i16;
  let local_6: i16;
  let local_4: i16;
  
  pass1_1008_3e94(param_4,CONCAT22(param_6,&local_6),
                  CONCAT22(param_6,&local_4));
  if (param_3 == 0x0) {
    iStack12 = 0x3;
    iStack10 = 0x42a6;
  }
  else {
    if (param_3 == 0x1) {
      iStack12 = 0x4;
      iStack10 = (s_SITEICON_1050_428d + 0x9);
    }
    else {
      if (param_3 != 0x2) {
        return;
      }
      iStack12 = 0x4;
      iStack10 = 0x42b2;
    }
  }
  uVar1 = iStack12 << 0x2;
  mem_op_1000_179c(uVar1,param_5,0x1000);
  for (iStack18 = 0x0; iStack18 < iStack12; iStack18 += 0x1) {
    iVar2 = (astruct_279 *)(iStack18 * 0x4);
    (iVar2 + uVar1) = (iVar2 + iStack10) + local_4;
    (iVar2 + uVar1 + 0x2) = (iVar2 + iStack10 + 0x2) + local_6;
  }
  return;
}


astruct_18 * 
pass1_1020_3616(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  cleanup_win_ui_1020_2fea((astruct_12 *)param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1020_3898(astruct_30 *param_1,param_2: u16)
{
  destroy_window_1020_3b3e(param_1,param_2);
  return;
}


fn pass1_1020_3bd6(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u32;
  
  uVar3 = (param_1 >> 0x10);
  uVar2 = param_1;
  mixed_draw_op_1020_3fa0((uVar2 + 0xf6),param_3,param_4);
  if ((uVar2 + 0x100) == 0x0) {
    (uVar2 + 0x100) = 0x1;
    uVar4 = (uVar2 + 0xfa);
    if ((uVar4 + 0x56) == 0x0) {
      iVar1 = 0x5;
    }
    else {
      iVar1 = 0x8;
    }
    uVar4 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,(uVar2 + 0x8),iVar1,param_2,
                            uVar2,&ctx.PTR_LOOP_1050_1038,param_4);
    (uVar2 + 0x10e) = uVar4;
    (uVar2 + 0x110) = (uVar4 >> 0x10);
  }
  return;
}



fn pass1_1020_3c32(param_1: i16,param_2: u16,param_3: u16,param_4: u16)
{
  let cVar1: u8;
  let iVar2: i16;
  
  if (param_3 == 0xf5) {
    iVar2 = 0x1;
//LAB_1020_3c52:
    pass1_1018_1b02(param_4,(param_1 + 0xfa),iVar2);
    return;
  }
  if ((param_3 < 0xf6) && (cVar1 = param_3, cVar1 != '\0')) {
    if (cVar1 == '\x01' || cVar1 == '\x02') {
      return;
    }
    if (cVar1 == -0xc) {
      iVar2 = 0x0;
      goto LAB_1020_3c52;
    }
  }
  pass1_1020_3c32(param_1,param_2,param_3,param_4);
  return;
}



fn pass1_1020_3c74(param_1: u16,param_2: u32,param_3: u16,param_4: u16)
{
  pass1_1020_3c8c(CONCAT22(param_2,param_1),CONCAT22(param_3,(param_2 >> 0x10)),
                  param_4);
  return;
}



fn pass1_1020_3c8c(param_1: u32,param_2: u32,param_3: u16)
{
  pt_in_rect_1018_1bda
            ((param_1 + 0xfa),param_2,(param_2 >> 0x10),
             param_3);
  return;
}



astruct_3 * 
pass1_1020_3ca6(astruct_3 *param_1,param_2: u8,param_3: u16)

{
  let uVar1: u32;
  let puStack10: *mut u16;
  
  uVar1 = param_1 & 0xffff0000;
  param_1 = (astruct_3 *)(uVar1 | param_1 - 0xf2);
  param_1._2_2_ = (uVar1 >> 0x10);
  if (param_1 == (astruct_3 *)0x0) {
    param_1._0_2_ = 0x0;
    param_1._2_2_ = 0x0;
  }
  puStack10 = CONCAT22(param_1._2_2_,param_1);
  *puStack10 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  cleanup_menu_ui_op_1020_795c(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Instruction at (ram,0x10203dab) overlaps instruction at (ram,0x10203da8)
// 
// WARNING: Variable defined which should be unmapped: param_16
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_3d08(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16
               ,param_6: u16,param_7: u16,param_8: u16,param_9: u16,param_10: u8
               ,param_11: u8,param_12: u8,param_13: u8,param_14: u8,param_1: u325
               ,param_16: u16,param_17: u16,param_18: u16,param_19: u16)

{
  char *pcVar1;
  byte *pbVar2;
  let bVar3: bool;
  let bVar4: bool;
  code **ppcVar5;
  let puVar6: *mut u16;
  let uVar7: u32;
  let puVar8: u32;
  let uVar9: u32;
  let bVar10: u8;
  let bVar12: u8;
  let uVar13: u16;
  let bVar18: u8;
  let cVar19: u8;
  HDC16 HVar14;
  int16_t iVar15;
  HGDIOBJ16 HVar16;
  let puVar17: *mut u8;
  let uVar20: u16;
  let uVar21: u16;
  let bVar22: u8;
  let bVar23: u8;
  let puVar24: *mut u8
  let bVar25: u8;
  let iVar26: i16;
  char *pcVar27;
  char *pcVar28;
  let uVar29: u16;
  let uVar30: u16;
  let bVar31: bool;
  let bVar32: bool;
  let puVar33: *mut u16;
  let uStack8: u16;
  let uStack6: u16;
  code *pcStack4;
  let bVar11: u8;
  
  uVar9 = CONCAT22(param_19,param_18);
  bVar22 = param_2 + (param_1 >> 0x8) + param_10;
  *param_6 = 0x3c;
  puVar24 = CONCAT11((param_2 >> 0x8) + '<' +
                              ((param_3 + param_5) < 0x20),bVar22);
  pcStack4 = switchD_1008:1091::caseD_a7;
  uVar13 = 0x203d;
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar22;
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 & bVar22;
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar22;
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar22;
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar22;
  bVar10 = (param_6 + 0x2);
  bVar12 = 0x9 < (bVar10 & 0xf) | param_11;
  bVar11 = bVar10 + bVar12 * '\x06' & 0xf;
  pbVar2 = (param_3 + 0x203d);
  *pbVar2 = *pbVar2 | bVar22;
  bVar10 = 0x9 < bVar11 | bVar12;
  uVar20 = CONCAT11((param_6 + 0x2 >> 0x8) + bVar12 + bVar10,
                    bVar11 + bVar10 * '\x06') & 0xff0f;
  pcVar28 = CONCAT11(0x79,param_5 + -0x37);
  do {
    pcVar27 = pcVar28;
    pbVar2 = (param_3 + uVar13);
    bVar23 = puVar24;
    *pbVar2 = *pbVar2 | bVar23;
    bVar12 = (uVar20 - 0x1);
    bVar3 = 0x9 < (bVar12 & 0xf);
    bVar22 = bVar3 | bVar10;
    pbVar2 = (param_3 + uVar13);
    *pbVar2 = *pbVar2 | bVar23;
    bVar4 = 0x9 < (bVar12 + bVar22 * '\x06' & 0xf);
    bVar18 = (uVar20 - 0x1 >> 0x8) + bVar22 + (bVar4 | bVar22);
    pbVar2 = (param_3 + uVar13);
    *pbVar2 = *pbVar2 | bVar23;
    uVar20 = 0x0;
    bVar31 = &pcStack4 < (param_3 + uVar13);
    pbVar2 = (param_3 + uVar13 + 0x896);
    bVar25 = param_3;
    bVar32 = CARRY1(*pbVar2,bVar25) || CARRY1(*pbVar2 + bVar25,bVar31);
    *pbVar2 = *pbVar2 + bVar25 + bVar31;
    pbVar2 = (param_3 + uVar13 + 0x2038);
    bVar12 = *pbVar2;
    bVar11 = *pbVar2;
    *pbVar2 = bVar11 + bVar25 + bVar32;
    pcVar1 = (param_4 + uVar13);
    *pcVar1 = *pcVar1 + (puVar24 >> 0x8) +
              (CARRY1(bVar12,bVar25) || CARRY1(bVar11 + bVar25,bVar32));
    pcVar1 = (param_3 + uVar13 + -0x64);
    *pcVar1 = *pcVar1 + bVar18 + '\x01';
    pbVar2 = (param_3 + uVar13);
    *pbVar2 = *pbVar2 | bVar23;
    pcVar1 = pcVar27;
    pcVar28 = pcVar27 + 0x1;
    bVar31 = *pcVar1 != '\0';
    if (-*pcVar1 < '\0') {
      pcVar1 = (param_4 + 0x37);
      *pcVar1 = *pcVar1 + bVar25 + bVar31;
      pbVar2 = (param_3 + uVar13);
      *pbVar2 = *pbVar2 | bVar23;
      bVar31 = SBORROW2(uVar13,0x1);
      uVar13 -= 0x1;
      uStack6 = (param_14 & 0x1) * 0x4000 | bVar31 * 0x800 |
                (param_13 & 0x1) * 0x200 | (param_12 & 0x1) * 0x100 |
                (uVar13 < 0x0) * 0x80 | (uVar13 == 0x0) * 0x40 |
                (bVar4 | bVar3 | bVar10 & 0x1) * 0x10 |
                ((POPCOUNT(uVar13 & 0xff) & 0x1) == 0x0) * 0x4;
      pbVar2 = (param_3 + uVar13);
      *pbVar2 = *pbVar2 | bVar23;
      pbVar2 = (param_3 + uVar13);
      *pbVar2 = *pbVar2 | bVar23;
      bVar31 = SBORROW2(puVar24,0x1);
      puVar24 = puVar24 + -0x1;
      uStack8 = (param_14 & 0x1) * 0x4000 | bVar31 * 0x800 |
                (param_13 & 0x1) * 0x200 | (param_12 & 0x1) * 0x100 |
                (puVar24 < 0x0) * 0x80 | (puVar24 == 0x0) * 0x40
                | (bVar4 | bVar3 | bVar10 & 0x1) * 0x10 |
                ((POPCOUNT(puVar24 & 0xff) & 0x1) == 0x0) * 0x4;
      pbVar2 = (param_3 + uVar13);
      bVar12 = puVar24;
      *pbVar2 = *pbVar2 | bVar12;
      if (*pbVar2 == 0x0) {
        pbVar2 = (param_3 + uVar13);
        *pbVar2 = *pbVar2 & bVar12;
code_r0x10203d96:
        pbVar2 = (param_3 + uVar13);
        *pbVar2 = *pbVar2 | puVar24;
        pbVar2 = (param_3 + uVar13);
        *pbVar2 = *pbVar2 & puVar24;
        pcVar28 = pcVar27 + 0x2;
        uVar21 = puVar24 & 0xff;
        puVar24 = (uVar21 | ((puVar24 >> 0x8) * '\x02' +
                                                 (uVar20 < 0x20)) << 0x8);
        pbVar2 = (param_3 + uVar13 + 0x1);
        *pbVar2 = *pbVar2 & uVar21;
        param_4 = &stack0xfff6;
        param_16 = pcStack4;
        param_17 = (pcStack4 >> 0x10);
        uVar9 = param_15;
code_r0x10203db1:
        get_sys_metrics_1020_7c1a(CONCAT22(param_17,param_16),uVar9,param_8);
        puVar6 = (param_4 + 0x6);
        uVar29 = (puVar6 >> 0x10);
        (puVar6 + 0x14) = 0x0;
        *puVar6 = 0x408a;
        (puVar6 + 0x2) = 0x1020;
        puVar33 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x6,param_9,puVar24,pcVar28);
        uVar29 = (puVar33 >> 0x10);
        uVar7 = (param_4 + 0x6);
        uVar30 = (uVar7 >> 0x10);
        iVar26 = uVar7;
        (iVar26 + 0x14) = puVar33;
        (iVar26 + 0x16) = uVar29;
        ppcVar5 = ((iVar26 + 0x14) + 0x4);
        (**ppcVar5)(0x1010,(iVar26 + 0x14),uVar29,0x0,iVar26,uVar30);
        HVar14 = GetDC16(0x1010);
        *(HDC16 *)(param_4 + -0x2) = HVar14;
        iVar15 = SetMapMode16(s_tile2_bmp_1050_1538,0x1);
        uVar7 = (param_4 + 0x6);
        *(int16_t *)(uVar7 + 0x1e) = iVar15;
        uVar29 = (param_4 + -0x2);
        HVar16 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
        HVar16 = SelectObject16(s_tile2_bmp_1050_1538,HVar16);
        uVar7 = (param_4 + 0x6);
        *(HGDIOBJ16 *)(uVar7 + 0x18) = HVar16;
        uVar30 = (param_4 + -0x2);
        HVar16 = GetStockObject16((INT16)s_tile2_bmp_1050_1538);
        HVar16 = SelectObject16(s_tile2_bmp_1050_1538,HVar16);
        uVar7 = (param_4 + 0x6);
        *(HGDIOBJ16 *)(uVar7 + 0x1a) = HVar16;
        uVar7 = (param_4 + 0x6);
        uVar7 = (uVar7 + 0x14);
        (param_4 + -0x6) = (uVar7 + 0x24);
        puVar17 = (param_4 + -0x2);
        puVar8 = (param_4 + -0x6);
        ppcVar5 = (*puVar8 + 0x8);
        (**ppcVar5)(s_tile2_bmp_1050_1538,puVar8,(puVar8 >> 0x10),
                    puVar17,param_9,uVar30,uVar29);
        uVar7 = (param_4 + 0x6);
        uVar30 = (uVar7 >> 0x10);
        iVar26 = uVar7;
        (iVar26 + 0x1c) = puVar17;
        uVar29 = (param_4 + -0x2);
        (param_4 + -0x14) = uVar29;
        uVar7 = (iVar26 + 0x14);
        (uVar7 + 0x4c) = uVar29;
        return;
      }
      pbVar2 = (param_3 + uVar13);
      *pbVar2 = *pbVar2 & bVar12;
      pcVar1 = (param_4 + uVar13 + 0x20);
      bVar11 = param_1 & 0x1f;
      cVar19 = *pcVar1;
      *pcVar1 = *pcVar1 >> bVar11;
      pcVar1 = (param_4 + uVar13 + 0x6a);
      *pcVar1 = *pcVar1 + param_1 +
                ((param_1 & 0x1f) != 0x0 && (cVar19 >> bVar11 - 0x1 & 0x1) != 0x0);
      pbVar2 = (param_3 + uVar13);
      *pbVar2 = *pbVar2 | bVar12;
      param_8 = 0x1020;
      uVar20 = (param_3 + uVar13) * 0x10;
      pbVar2 = (pcVar28 + param_4 + 0x8);
      bVar12 = (uVar20 >> 0x8);
      uVar21 = uVar20 & 0xff | (bVar12 + *pbVar2) << 0x8;
      pcVar1 = (param_4 + uVar13 + 0x37);
      *pcVar1 = *pcVar1 + (param_3 >> 0x8) + CARRY1(bVar12,*pbVar2);
    }
    else {
      pcVar1 = (param_4 + uVar13);
      *pcVar1 = *pcVar1 + bVar31;
      uVar21 = (param_3 + uVar13) * 0x10;
      if ((POPCOUNT(*pcVar1) & 0x1) == 0x0) goto code_r0x10203db1;
    }
    pbVar2 = (param_3 + uVar13);
    bVar11 = puVar24;
    *pbVar2 = *pbVar2 | bVar11;
    pbVar2 = (param_3 + uVar13);
    *pbVar2 = *pbVar2 | bVar11;
    param_16 = (param_14 & 0x1) * 0x4000 | (param_13 & 0x1) * 0x200 |
               (param_12 & 0x1) * 0x100 | (*pbVar2 < '\0') * 0x80 |
               (*pbVar2 == 0x0) * 0x40 |
               (bVar4 | bVar3 | bVar10 & 0x1) * 0x10 |
               ((POPCOUNT(*pbVar2) & 0x1) == 0x0) * 0x4;
    pbVar2 = (param_3 + uVar13);
    *pbVar2 = *pbVar2 | bVar11;
    bVar12 = in(0x79);
    pbVar2 = (param_3 + uVar13);
    *pbVar2 = *pbVar2 & bVar11;
    if (-0x1 < *pbVar2) {
      uVar9 = CONCAT22(param_19,param_18);
      if ((bVar18 | (param_4 - 0x1)) == 0x0) {
        param_16 = param_7;
        uVar9 = CONCAT22(param_19,param_18);
      }
      goto code_r0x10203db1;
    }
    pbVar2 = (param_4 + 0x89c);
    bVar31 = CARRY1(*pbVar2,bVar12);
    *pbVar2 = *pbVar2 + bVar12;
    bVar23 = bVar18 + bVar12;
    cVar19 = bVar23 + bVar31;
    uVar20 = CONCAT11(cVar19,bVar12);
    pcStack4._0_3_ =
         CONCAT21((param_14 & 0x1) * 0x4000 |
                  (SCARRY1(bVar18,bVar12) != SCARRY1(bVar23,bVar31)) * 0x800 |
                  (param_13 & 0x1) * 0x200 | (param_12 & 0x1) * 0x100 |
                  (cVar19 < '\0') * 0x80 | (cVar19 == '\0') * 0x40 |
                  (bVar4 | bVar3 | bVar10 & 0x1) * 0x10 |
                  ((POPCOUNT(cVar19) & 0x1) == 0x0) * 0x4 |
                  (CARRY1(bVar18,bVar12) || CARRY1(bVar23,bVar31)),pcStack4._0_1_);
    pcStack4 = (pcStack4 & 0xff000000 | pcStack4);
    pbVar2 = (param_3 + uVar13);
    *pbVar2 = *pbVar2 | bVar11;
    param_1 = uVar21 - 0x1;
    bVar10 = bVar4 | bVar22;
    if (param_1 == 0x0 || *pbVar2 == 0x0) goto code_r0x10203d96;
  } while( true );
}


fn pass1_1020_4092(param_1: *mut u16) -> u16

{
  let iVar1: i16;
  let uVar2: u16;
  
  pass1_1008_3e38(param_1);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x6) = 0x0;
  (iVar1 + 0x8) = 0x0;
  (iVar1 + 0xa) = 0x1;
  (iVar1 + 0xc) = 0x0;
  (iVar1 + 0xe) = 0x0;
  pass1_1008_3e38((param_1 & 0xffff0000 | (iVar1 + 0x10)));
  return param_1;
}


void 
pass1_1020_434c(param_1: i16,param_2: u16,param_3: *mut u32,param_4: u16,param_5: u16,
               param_6: i16,param_7: u16,param_8: u16)

{
  if (param_6 == 0x1) {
    pass1_1020_6184(CONCAT22(param_2,param_1),param_5,param_8);
    return;
  }
  if (param_6 == 0x2) {
    ui_op_1020_536e(CONCAT22(param_2,param_1),CONCAT22(param_4,param_3),param_5,0x2,
                    param_7);
    return;
  }
  pass1_1008_68ea(param_1,param_2,param_3,param_4,param_5,param_6);
  return;
}


fn pass1_1020_44b0(param_1: *mut u32)
{
  code **ppcVar1;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0xf6) != 0x0) {
    ppcVar1 = (*param_1 + 0x98);
    (**ppcVar1)();
    (iVar2 + 0x112) = 0x0;
    ppcVar1 = ((iVar2 + 0xf6) + 0x8);
    (**ppcVar1)();
  }
  return;
}


fn pass1_1020_51c6(param_1: u32,param_2: u16,param_3: u32,param_4: u16,param_5: u16)
{
  code **ppcVar1;
  let iVar2: i16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  iVar2 = (iVar3 + 0xf4);
  uVar5 = param_3;
  if (iVar2 == 0x2) {
    win_ui_op_1020_5e76(param_1 & 0xffff | uVar4 << 0x10,param_2,uVar5);
    return;
  }
  iVar2 += -0x3;
  if (iVar2 != 0x0) {
    pt_in_rect_op_1020_58ce
              (param_1 & 0xffff | uVar4 << 0x10,param_2,uVar5,
               (param_3 >> 0x10),param_4,param_5);
    if (iVar2 == 0x0) {
      ppcVar1 = ((iVar3 + 0x4) + 0x5c);
      (**ppcVar1)(param_4,(iVar3 + 0x4),param_2,param_3);
    }
    return;
  }
  win_ui_op_1020_5de8(param_1 & 0xffff | uVar4 << 0x10,param_2,uVar5,param_5);
  return;
}


fn pass1_1020_52de(Uparam_1: i32,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  let iVar6: i16;
  let uVar7: u16;
  
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  puVar1 = (iVar6 + 0xf6);
  uVar2 = (iVar6 + 0xf8);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  (iVar6 + 0xf6) = 0x0;
  if ((iVar6 + 0xfa) != 0x0) {
    if (param_1 == 0x0) {
      iVar4 = 0x0;
      uVar5 = 0x0;
    }
    else {
      iVar4 = iVar6 + 0xe2;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6((iVar6 + 0xfa),CONCAT22(uVar5,iVar4),param_2);
  }
  destroy_win_1008_628e(param_1,0x1008);
  if ((iVar6 + 0xfa) != 0x0) {
    pass1_1010_1dda((iVar6 + 0xfa));
  }
  (iVar6 + 0xfa) = 0x0;
  return;
}

G: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_5d56(param_1: *mut u32,param_2: u32,uchar *param_3,param_4: i16,param_5: u16) -> u16

{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u16;
  i16 local_12 [0x2];
  let local_e: i16;
  let local_c: i16;
  i16 local_a [0x2];
  let iStack6: i16;
  
  iStack6 = (param_2 + 0x2e);
  uVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  if (iStack6 == 0x47) {
    pass1_1020_61c4(uVar2,uVar3,CONCAT22(param_5,&local_c),
                    CONCAT22(param_5,local_a),param_3,param_4,param_5);
    if (local_a[0] == 0x0) goto LAB_1020_5d8b;
    if (local_c <= local_a[0]) {
      return 0x1;
    }
  }
  else {
    if (iStack6 != 0x6a) {
      return 0x0;
    }
    pass1_1020_61c4(uVar2,uVar3,CONCAT22(param_5,&local_e),
                    CONCAT22(param_5,local_12),param_3,param_4,param_5);
    if (local_e <= local_12[0]) {
//LAB_1020_5d8b:
      ppcVar1 = (*param_1 + 0x40);
      (**ppcVar1)();
      return 0x1;
    }
  }
  pass1_1038_af40(_PTR_LOOP_1050_5b7c,(uVar2 + 0x8),0x9,param_3,uVar2,
                  &ctx.PTR_LOOP_1050_1038,param_5);
  return 0x1;
}


fn pass1_1020_6184(param_1: u32,param_2: u16,param_3: u16)
{
  HCURSOR16 HVar1;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0xf4) == 0x1) {
    HVar1 = SetCursor16(param_3);
    *(HCURSOR16 *)(iVar2 + 0xee) = HVar1;
    (iVar2 + 0x10c) = param_2;
    SetCapture16((HWND16)s_tile2_bmp_1050_1538);
    (iVar2 + 0xf4) = 0x2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_61c4(param_1: u16,param_2: u16,param_3: u32,param_4: *mut u16,uchar *param_5
               ,param_6: i16,param_7: u16)

{
  let uVar1: u32;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_7,param_5,param_6);
  uVar2 = (puVar3 >> 0x10);
  uVar1 = (puVar3 + 0x20);
  pass1_1030_8308(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,param_3,param_4,uVar1,uVar1,uVar2);
  *param_4 = (puVar3 + 0x1e);
  return;
}



astruct_18 * 
pass1_1020_6208(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
  destroy_cursor_1020_42f4(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_62e0(param_1: i16,param_2: u16,param_3: u16,param_4: u16)
{
  let puVar1: u32;
  code **ppcVar2;
  let puVar3: *mut u16;
  let uVar4: u32;
  let extraout_DX: *mut u8
  let puVar5: *mut u8
  let puVar6: *mut u8
  let uVar7: u16;
  let extraout_DX_00: *mut u8
  let unaff_DI: i16;
  let puVar8: *mut u16;
  let uVar9: u16;
  let uVar10: u16;
  let iVar11: i16;
  let uVar12: u16;
  
  set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2,param_1),param_3);
  (param_1 + 0x14) = 0x0;
  (param_1 + 0x2c) = 0x0;
  CONCAT22(param_2,param_1) = 0x67c2;
  (param_1 + 0x2) = 0x1020;
  puVar6 = extraout_DX;
  puVar3 = pass1_1000_4906((astruct_20 *)CONCAT22(param_2,param_1 + 0x18),
                           (WNDCLASS16 *)0x0,0x14);
  mem_op_1000_179c(0x3c,puVar6,0x1000);
  puVar5 = (puVar6 | puVar3);
  if (puVar5 == 0x0) {
    (param_1 + 0x1c) = 0x0;
  }
  else {
    pass1_1020_87c2(CONCAT22(puVar6,puVar3),param_4,unaff_DI);
    (param_1 + 0x1c) = puVar3;
    *(uchar **)(param_1 + 0x1e) = puVar5;
  }
  mem_op_1000_179c(0x26,puVar5,0x1000);
  puVar6 = (puVar5 | puVar3);
  if (puVar6 == 0x0) {
    puVar3 = 0x0;
    puVar6 = 0x0;
  }
  else {
    pass1_1020_8a9c(CONCAT22(puVar5,puVar3));
  }
  (param_1 + 0x20) = puVar3;
  *(uchar **)(param_1 + 0x22) = puVar6;
  mem_op_1000_179c(0xbe,puVar6,0x1000);
  puVar5 = (puVar6 | puVar3);
  if (puVar5 == 0x0) {
    puVar3 = 0x0;
    puVar5 = 0x0;
  }
  else {
    pass1_1020_8eaa(CONCAT22(puVar6,puVar3),param_4);
  }
  (param_1 + 0x24) = puVar3;
  *(uchar **)(param_1 + 0x26) = puVar5;
  mem_op_1000_179c(0x20,puVar5,0x1000);
  puVar6 = (puVar5 | puVar3);
  if (puVar6 == 0x0) {
    puVar3 = 0x0;
    puVar6 = 0x0;
  }
  else {
    pass1_1020_8360(CONCAT22(puVar5,puVar3),param_4);
  }
  (param_1 + 0x28) = puVar3;
  *(uchar **)(param_1 + 0x2a) = puVar6;
  pass1_1020_6746(CONCAT22(param_2,param_1),0x1,0x4);
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,param_4,puVar6,unaff_DI);
  uVar7 = (puVar8 >> 0x10);
  (param_1 + 0x14) = puVar8;
  (param_1 + 0x16) = uVar7;
  uVar10 = 0x0;
  uVar9 = (param_1 + 0x14);
  ppcVar2 = ((param_1 + 0x14) + 0x4);
  iVar11 = param_1;
  uVar12 = param_2;
  (**ppcVar2)();
  (param_1 + 0x6) = (param_1 + 0x14);
  uVar4 = (param_1 + 0x14);
  puVar1 = (uVar4 + 0xa);
  uVar4 = CONCAT22(param_2,param_1 + 0xa);
  ppcVar2 = (*puVar1 + 0x8);
  (**ppcVar2)(0x1010,puVar1,(puVar1 >> 0x10),uVar4,uVar9,uVar7,uVar10,
              iVar11,uVar12);
  (param_1 + 0x12) = uVar4;
  (param_1 + 0x10) = 0x1;
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,extraout_DX_00,unaff_DI);
  (param_1 + 0x2c) = puVar8;
  (param_1 + 0x2e) = (puVar8 >> 0x10);
  return;
}



fn pass1_1020_6466(param_1: *mut u16,param_2: u16,param_3: u16)
{
  astruct_585 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_585 *)param_1;
  *param_1 = 0x67c2;
  iVar1.field_0x2 = 0x1020;
  if (iVar1.field_0x14 != 0x0) {
    param_2 = 0x1010;
    pass1_1010_1ea6(iVar1.field_0x14,param_1 & 0xffff | uVar1 << 0x10,
                    param_3);
  }
  palette_op_1020_92c4(param_1,param_2);
  return;
}



fn pass1_1020_6498(param_1: u32,param_2: i16) -> u32

{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  if ((param_1 + 0x18 + param_2 * 0x4) != 0x0) {
    uVar1 = (param_1 + 0x18 + param_2 * 0x4);
    uVar3 = (uVar1 >> 0x10);
    iVar2 = uVar1;
    return CONCAT22((iVar2 + 0xa),(iVar2 + 0x8));
  }
  return 0x0;
}



fn pass1_1020_64d4(param_1: u32,param_2: i16) -> u16

{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if ((param_1 + 0x18 + param_2 * 0x4) != 0x0) {
    uVar1 = (param_1 + 0x18 + param_2 * 0x4);
    return (uVar1 + 0x4);
  }
  return 0x0;
}


fn pass1_1020_6746(param_1: u32,param_2: i16,param_3: i16)
{
  code **ppcVar1;
  let uVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  
  if (param_3 != 0x0) {
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x18 + param_3 * 0x4) != 0x0) {
      uVar2 = (iVar3 + 0x18 + param_3 * 0x4);
      (uVar2 + 0x4) = param_2;
      (iVar3 + 0x10) = 0x1;
      if (param_2 == 0x0) {
        ppcVar1 = (
                                  (iVar3 + 0x18 + param_3 * 0x4) + 0x14);
        (**ppcVar1)();
      }
    }
  }
  return;
}



astruct_18 * 
pass1_1020_679c(astruct_18 *param_1,param_2: u8,param_3: u16,param_4: u16)

{
  pass1_1020_6466(param_1,param_3,param_4);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1020_687c(param_1: u32,param_2: u16,param_3: u16)
{
  ulet uVar1: u8;
  
  uVar1 = (uchar)param_2;
  get_win_ui_info_op_1020_7a50(param_1,param_3);
  destroy_icon_1020_6bd2(param_1,uVar1,param_3);
  return;
}


fn pass1_1020_68de(param_1: u32,param_2: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0xf6) != 0x0) {
    invalidate_rect_1020_735a((param_1 + 0xf6),param_2);
  }
  return;
}


fn pass1_1020_6bbc(param_1: u32,param_2: u16,param_3: u16)
{
  win_ui_op_1020_737a(*(ULONG *)(param_1 + 0xf6),param_2,param_3);
  return;
}


void 
pass1_1020_6e52(param_1: u16,param_2: u16,param_3: u16,param_4: i16,param_5: u16,
               param_6: i16)

{
  let uVar1: u16;
  char *pcVar2;
  
  pass1_1018_2e5e(param_1,param_2,param_3,(param_4 + 0xf2));
  uVar1 = param_3 | param_2;
  if (uVar1 == 0x0) {
    pcVar2 = load_string_1010_847e
                       (_PTR_LOOP_1050_14cc,
                        (_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  }
  else {
    pass1_1018_2d84(param_2,(param_4 + 0xf2));
    pcVar2 = CONCAT22(uVar1,param_2);
  }
  string_1020_79b4(param_1,CONCAT22(param_5,param_4),param_6,pcVar2);
  return;
}


astruct_3 *  pass1_1020_70c0(astruct_3 *param_1,param_2: u8,param_3: u16)

{
  cleanup_menu_ui_op_1020_795c(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


astruct_18 * 
pass1_1020_7526(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  palette_op_1020_7270(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1020_75c4(param_1: *mut u16)
{
  astruct_586 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_586 *)param_1;
  *param_1 = 0x7780;
  iVar1.field_0x2 = 0x1020;
  iVar1.field_0xe2 = 0x781c;
  iVar1.field_0xe4 = 0x1020;
  pass1_1020_808e(param_1);
  return;
}


fn pass1_1020_770e(param_1: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0xee);
  uVar2 = (iVar4 + 0xf0);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  (iVar4 + 0xee) = 0x0;
  destroy_win_1008_628e(param_1 & 0xffff | uVar5 << 0x10,0x1008);
  return;
}



astruct_18 *  pass1_1020_774c(astruct_18 *param_1,param_2: u8)

{
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
  pass1_1020_75c4(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_7824(astruct_666 *param_1,param_2: u16,param_3: u16,param_4: i16,
               param_5: u16)

{
  code **ppcVar1;
  let uVar2: u32;
  let iVar3: i16;
  let extraout_DX: *mut u8
  let uVar4: u16;
  let puVar5: *mut u16;
  
  set_struct_op_1020_921c((astruct_42 *)CONCAT22(param_2,param_1),param_3);
  &param_1.field_0x14 = 0x0;
  CONCAT22(param_2,param_1) = 0x7902;
  param_1.field_0x2 = 0x1020;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x25,param_5,extraout_DX,param_4);
  uVar4 = (puVar5 >> 0x10);
  param_1.field_0x14 = puVar5;
  param_1.field_0x16 = uVar4;
  param_1.field_0x6 = param_1.field_0x14;
  param_1.field_0x8 = uVar4;
  uVar2 = &param_1.field_0x14;
  iVar3 = &param_1.field_0xa;
  ppcVar1 = ((uVar2 + 0xa) + 0x8);
  (**ppcVar1)();
  param_1.field_0x12 = iVar3;
  draw_op_1020_9364((astruct_7 *)CONCAT22(param_2,param_1),0x1010,param_5);
  return;
}



fn pass1_1020_78ac(param_1: *mut u16,param_2: u16)
{
  astruct_587 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_587 *)param_1;
  *param_1 = 0x7902;
  iVar1.field_0x2 = 0x1020;
  if (iVar1.field_0x14 != 0x0) {
    param_2 = 0x1010;
    pass1_1010_1dda(iVar1.field_0x14);
  }
  palette_op_1020_92c4(param_1,param_2);
  return;
}



astruct_18 * 
pass1_1020_78dc(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  pass1_1020_78ac(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1020_79ae(void) -> u16

{
  return 0x1;
}


fn pass1_1020_79e4(param_1: u32,param_2: u16,param_3: u16)
{
  draw_op_1020_7cc8(*(ULONG *)(param_1 + 0xe8),param_2,param_3);
  return;
}


astruct_3 *  pass1_1020_7b60(astruct_3 *param_1,param_2: u8,param_3: u16)

{
  cleanup_menu_ui_op_1020_795c(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


astruct_18 *  pass1_1020_7f38(astruct_18 *param_1,param_2: u8)

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1.field_0x0 = 0x3ab0;
  (param_1 + 0x2) = 0x1008;
  param_1.field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1020_808e(param_1: *mut u16)
{
  let puVar1: *mut u16;
  let uVar2: u16;
  astruct_574 *iVar3;
  let uVar3: u16;
  let puStack6: *mut u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_574 *)param_1;
  *param_1 = 0x82bc;
  iVar3.field_0x2 = 0x1020;
  iVar3.field_0xe2 = 0x8358;
  iVar3.field_0xe4 = 0x1020;
  if (param_1 == 0x0) {
    puVar1 = 0x0;
    uVar2 = 0x0;
  }
  else {
    puVar1 = &iVar3.field_0xe2;
    uVar2 = uVar3;
  }
  puStack6 = CONCAT22(uVar2,puVar1);
  *puStack6 = 0x389a;
  puVar1[0x1] = 0x1008;
  pass1_1008_57c4(
                  (param_1 & 0xffff0000 | &iVar3.field_0xd2));
  *param_1 = 0x380a;
  iVar3.field_0x2 = 0x1008;
  *param_1 = 0x389a;
  iVar3.field_0x2 = 0x1008;
  return;
}



fn pass1_1020_8106(param_1: u32)
{
  code **ppcVar1;
  
  ppcVar1 = ((param_1 + 0x4) + 0x60);
  (**ppcVar1)();
  return;
}


astruct_18 *  pass1_1020_8288(astruct_18 *param_1,param_2: u8)

{
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
  pass1_1020_808e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_8360(param_1: *mut u16,param_2: u16)
{
  let uVar1: u32;
  let uVar2: u16;
  let puVar3: *mut u16;
  let uVar4: u16;
  astruct_667 *iVar4;
  
  iVar4 = (astruct_667 *)param_1;
  uVar4 = (param_1 >> 0x10);
  struct_1020_847a(param_1,0x1,param_2);
  puVar3 = pass1_1008_3e38(
                           (param_1 & 0xffff0000 | &iVar4.field_0x16)
                          );
  &iVar4.field_0x1c = 0x0;
  *param_1 = 0x8462;
  iVar4.field_0x2 = 0x1020;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,param_2,
                           (puVar3 >> 0x10),uVar4);
  uVar2 = (puVar3 >> 0x10);
  iVar4.field_0x1c = puVar3;
  iVar4.field_0x1e = uVar2;
  pass1_1018_26f8(iVar4.field_0x1c,uVar2,

                  (param_1 & 0xffff0000 | &iVar4.field_0x16));
  uVar1 = &iVar4.field_0x1c;
  pass1_1020_8712(param_1 & 0xffff | uVar4 << 0x10,iVar4.field_0x8,
                  *(astruct_76 **)(uVar1 + 0x2a),

                  (param_1 & 0xffff0000 | &iVar4.field_0x16));
  return;
}



fn pass1_1020_83f8(param_1: u32,param_2: u16)
{
  let uVar1: u32;
  let uVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if ((iVar3 + 0x4) != 0x0) {
    uVar1 = (iVar3 + 0x1c);
    uVar2 = (iVar3 + 0x1c);
    pass1_1008_4480((uVar1 + 0xa),
                    (param_1 & 0xffff0000 | (iVar3 + 0x16)),
                    *(astruct_76 **)(uVar2 + 0x2a),param_2);
  }
  return;
}



astruct_18 *  pass1_1020_843c(astruct_18 *param_1,param_2: u8)

{
  pass1_1020_8556(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1020_8556(param_1: *mut u16)
{
  let piVar1: *mut i16;
  let uVar2: u16;
  astruct_18 *paVar3;
  astruct_588 *iVar5;
  astruct_589 *iVar4;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u16;
  let iStack12: i16;
  
  uVar7 = (param_1 >> 0x10);
  iVar5 = (astruct_588 *)param_1;
  *param_1 = 0x87aa;
  iVar5.field_0x2 = 0x1020;
  fn_ptr_1000_17ce(iVar5.field_0x8,0x1000);
  if (((&iVar5.field_0xc + 0x2) | &iVar5.field_0xc) != 0x0) {
    iStack12 = 0x0;
    while( true ) {
      piVar1 = &iVar5.field_0x6;
      if (*piVar1 == iStack12 || *piVar1 < iStack12) break;
      iVar6 = iStack12 * 0x4;
      paVar3 = iVar5.field_0xc;
      uVar8 = (paVar3 >> 0x10);
      iVar4 = (astruct_589 *)paVar3;
      if ((iVar4 + iVar6) != 0x0) {
        paVar3 = (iVar4 + iVar6);
        uVar2 = (iVar4 + iVar6 + 0x2);
        if ((uVar2 | paVar3) != 0x0) {
          pass1_1008_5118(paVar3 & 0xffff | uVar2 << 0x10);
          fn_ptr_1000_17ce(paVar3,0x1000);
        }
      }
      iStack12 += 0x1;
    }
    fn_ptr_1000_17ce(iVar5.field_0xc,0x1000);
  }
  *param_1 = 0x389a;
  iVar5.field_0x2 = 0x1008;
  return;
}



fn pass1_1020_85f6(param_1: u32)
{
  let piVar1: *mut i16;
  let uVar2: u16;
  astruct_18 *paVar3;
  let uVar4: u32;
  let iVar5: i16;
  astruct_590 *iVar6;
  let uVar6: u16;
  let uVar7: u16;
  let iStack4: i16;
  
  iStack4 = 0x0;
  while( true ) {
    uVar7 = (param_1 >> 0x10);
    iVar6 = (astruct_590 *)param_1;
    piVar1 = &iVar6.field_0x6;
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    uVar4 = iVar6.field_0xc;
    uVar6 = (uVar4 >> 0x10);
    iVar5 = uVar4;
    paVar3 = (iVar5 + iStack4 * 0x4);
    uVar2 = (iVar5 + iStack4 * 0x4 + 0x2);
    if ((uVar2 | paVar3) != 0x0) {
      pass1_1008_5118(paVar3 & 0xffff | uVar2 << 0x10);
      fn_ptr_1000_17ce(paVar3,0x1000);
    }
    uVar4 = iVar6.field_0xc;
    (uVar4 + iStack4 * 0x4) = 0x0;
    iStack4 += 0x1;
  }
  return;
}



fn pass1_1020_865a(param_1: u32)
{
  let piVar1: *mut i16;
  let uVar2: u16;
  astruct_18 *paVar3;
  let uVar4: u32;
  let iVar5: i16;
  astruct_592 *iVar7;
  astruct_591 *iVar6;
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  let iStack4: i16;
  
  iStack4 = 0x0;
  while( true ) {
    uVar9 = (param_1 >> 0x10);
    iVar5 = param_1;
    piVar1 = (iVar5 + 0x6);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    iVar8 = iStack4 * 0x4;
    uVar4 = (iVar5 + 0xc);
    uVar10 = (uVar4 >> 0x10);
    iVar7 = (astruct_592 *)uVar4;
    if ((iVar7 + iVar8) != 0x0) {
      pass1_1008_5236((iVar7 + iVar8));
      uVar4 = (iVar5 + 0xc);
      uVar10 = (uVar4 >> 0x10);
      iVar6 = (astruct_591 *)uVar4;
      paVar3 = (iVar6 + iVar8);
      uVar2 = (iVar6 + iVar8 + 0x2);
      if ((uVar2 | paVar3) != 0x0) {
        pass1_1008_5118(paVar3 & 0xffff | uVar2 << 0x10);
        fn_ptr_1000_17ce(paVar3,0x1000);
      }
      uVar4 = (iVar5 + 0xc);
      (uVar4 + iStack4 * 0x4) = 0x0;
    }
    iStack4 += 0x1;
  }
  return;
}



fn pass1_1020_86d8(param_1: u32)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  let iStack4: i16;
  
  iStack4 = 0x0;
  while( true ) {
    uVar4 = (param_1 >> 0x10);
    piVar1 = (param_1 + 0x6);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    uVar2 = (param_1 + 0xc);
    uVar4 = (uVar2 >> 0x10);
    iVar3 = uVar2;
    if ((iVar3 + iStack4 * 0x4) != 0x0) {
      pass1_1008_5236((iVar3 + iStack4 * 0x4));
    }
    iStack4 += 0x1;
  }
  return;
}



fn pass1_1020_8712(param_1: u32,i16 *param_2,astruct_76 *param_3,param_4: *mut u16)
{
  let uVar1: u16;
  let uVar2: u32;
  
  pass1_1008_3f32(param_4,
                  (param_1 & 0xffff0000 | (param_1 + 0x10)));
  uVar2 = pass1_1008_4772(param_3);
  uVar1 = (uVar2 >> 0x10);
  pass1_1008_3e94(param_4,
                          (param_2 & 0xffff0000 |
                          ZEXT24((param_2 + 0x2))),
                  (param_2 & 0xffff | param_2._2_2_ << 0x10));
  (param_2 + 0x4) = (uVar2 + 0x4) + *param_2;
  (param_2 + 0x6) = (uVar2 + 0x8) + (param_2 + 0x2)
  ;
  return;
}



astruct_18 *  pass1_1020_8784(astruct_18 *param_1,param_2: u8)

{
  pass1_1020_8556(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_87c2(param_1: *mut u16,param_2: u16,param_3: i16)
{
  let uVar1: u32;
  astruct_281 *iVar2;
  let uVar2: u16;
  let puVar3: *mut u16;
  let local_12: [u8;8];
  let uStack10: i16;
  let puStack8: *mut u16;
  let iStack4: i16;
  
  struct_1020_847a(param_1,0x4,param_2);
  iStack4 = 0x4;
  iVar2 = (astruct_281 *)param_1;
  iVar2 = (astruct_281 *)&iVar2.field_0x16;
  puStack8 = (param_1 & 0xffff0000 | ZEXT24(iVar2));
  do {
    pass1_1008_3e38(puStack8);
    puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0x6));
    iStack4 += -0x1;
  } while (iStack4 != 0x0);
  uVar2 = (param_1 >> 0x10);
  &iVar2.field_0x2e = 0x0;
  puVar3 = pass1_1008_3e38(
                           (param_1 & 0xffff0000 | &iVar2.field_0x32)
                          );
  iVar2.field_0x38 = 0x0;
  *param_1 = 0x8a84;
  iVar2.field_0x2 = 0x1020;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,param_2,
                           (puVar3 >> 0x10),param_3);
  iVar2.field_0x2e = puVar3;
  iVar2.field_0x30 = (puVar3 >> 0x10);
  iStack10 = 0x0;
  do {
    uVar1 = &iVar2.field_0x2e;
    pass1_1018_26d8(uVar1,(uVar1 >> 0x10),iStack10,

                    (param_1 & 0xffff0000 |
                    (&iVar2.field_0x16 + iStack10 * 0x6)));
    uVar1 = &iVar2.field_0x2e;
    pass1_1020_8712(param_1 & 0xffff | uVar2 << 0x10,
                    CONCAT22(iVar2.field_0xa,iVar2.field_0x8 + iStack10 * 0x8),
                    *(astruct_76 **)(uVar1 + 0x2e + iStack10 * 0x4),

                    (param_1 & 0xffff0000 |
                    (&iVar2.field_0x16 + iStack10 * 0x6)));
    iStack10 += 0x1;
  } while (iStack10 < 0x4);
  uVar1 = &iVar2.field_0x2e;
  pass1_1018_2548(uVar1,(uVar1 >> 0x10),

                  (param_1 & 0xffff0000 | &iVar2.field_0x32));
  uVar1 = &iVar2.field_0x2e;
  iVar2.field_0x38 = (uVar1 + 0x6e);
  pass1_1020_8712(param_1 & 0xffff | uVar2 << 0x10,
                  CONCAT22(param_2,local_12),(astruct_76 *)iVar2.field_0x38,

                  (param_1 & 0xffff0000 | &iVar2.field_0x32));
  return;
}



fn pass1_1020_8908(param_1: u32,param_2: u32,param_3: u16)
{
  astruct_76 *paVar1;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let puVar5: *mut u8
  let puVar6: *mut u8
  let uVar7: u16;
  astruct_284 *iVar8;
  let iVar9: i16;
  let iVar10: i16;
  let uVar11: u16;
  let uVar12: u16;
  let uVar13: u32;
  Struct110 *paStack28;
  let iStack4: i16;
  
  for (iStack4 = 0x0; iVar8 = (astruct_284 *)param_1,
      uVar11 = (param_1 >> 0x10), iStack4 < 0x4; iStack4 += 0x1) {
    if (iVar8.field_0x4 == 0x0) {
      uVar2 = iVar8.field_0xc;
      uVar11 = (uVar2 >> 0x10);
      iVar10 = uVar2;
      iVar9 = iStack4 * 0x4;
      if (((iVar10 + iVar9 + 0x2) | (iVar10 + iVar9)) != 0x0) {
        pass1_1008_5236((iVar10 + iVar9));
      }
    }
    else {
      uVar2 = iVar8.field_0x2e;
      paVar1 = *(astruct_76 **)(uVar2 + 0x2e + iStack4 * 0x4);
      uVar13 = pass1_1008_4772(paVar1);
      puVar5 = (uVar13 >> 0x10);
      uVar3 = uVar13;
      uVar2 = iVar8.field_0xc;
      iVar10 = iStack4 * 0x4;
      if ((uVar2 + iVar10) == 0x0) {
        puVar6 = puVar5;
        uVar4 = uVar3;
        mem_op_1000_179c(0x14,puVar5,0x1000);
        paStack28 = (Struct110 *)CONCAT22(puVar6,uVar4);
        if ((puVar6 | uVar4) == 0x0) {
          uVar2 = iVar8.field_0xc;
          (uVar2 + iStack4 * 0x4) = 0x0;
        }
        else {
          uVar4 = &iVar8.field_0x16 + iStack4 * 0x6;
          uVar7 = uVar11;
          pass1_1008_50c2(paStack28,(uVar3 + 0x8),(uVar3 + 0x4),
                          (param_1 & 0xffff0000 | uVar4),param_2);
          uVar2 = iVar8.field_0xc;
          uVar12 = (uVar2 >> 0x10);
          iVar9 = uVar2;
          (iVar9 + iVar10) = uVar4;
          (iVar9 + iVar10 + 0x2) = uVar7;
        }
        uVar2 = iVar8.field_0xc;
        pass1_1008_5134((uVar2 + iStack4 * 0x4));
      }
      uVar2 = iVar8.field_0xc;
      pass1_1008_5236((uVar2 + iStack4 * 0x4));
      pass1_1008_4480(param_2,
                              (param_1 & 0xffff0000 |
                              (&iVar8.field_0x16 + iStack4 * 0x6)),paVar1,
                      param_3);
    }
  }
  if (iVar8.field_0x4 != 0x0) {
    pass1_1008_4480(param_2,
                            (param_1 & 0xffff0000 | &iVar8.field_0x32),
                    iVar8.field_0x38,param_3);
  }
  return;
}



astruct_18 *  pass1_1020_8a5e(astruct_18 *param_1,param_2: u8)

{
  pass1_1020_8556(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_8a9c(param_1: *mut u16)
{
  let uVar1: u32;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let unaff_SS: u16;
  let puVar5: *mut u16;
  astruct_76 *paVar6;
  astruct_43 *paVar7;
  let iVar8: i16;
  let uVar9: u16;
  let puStack76: *mut u16;
  u8 local_48 [0x1e];
  let local_2a: [u8;24];
  let uStack6: u16;
  let uStack4: u16;
  
  iVar8 = param_1;
  uVar9 = (param_1 >> 0x10);
  struct_1020_847a(param_1,0x2,unaff_SS);
  uVar3 = iVar8 + 0x16;
  pass1_1008_3e38((param_1 & 0xffff0000 | uVar3));
  puStack76 = (param_1 & 0xffff0000 | (iVar8 + 0x1c));
  puVar5 = pass1_1008_3e38(
                           (param_1 & 0xffff0000 | (iVar8 + 0x1c)));
  (iVar8 + 0x22) = 0x0;
  *param_1 = 0x8e92;
  (iVar8 + 0x2) = 0x1020;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,unaff_SS,
                           (puVar5 >> 0x10),uVar9);
  uVar4 = (puVar5 >> 0x10);
  (iVar8 + 0x22) = puVar5;
  (iVar8 + 0x24) = uVar4;
  pass1_1018_2678((iVar8 + 0x22),uVar4,
                  (param_1 & 0xffff0000 | uVar3));
  paVar6 = (astruct_76 *)pass1_1018_268e((iVar8 + 0x22));
  uStack4 = (paVar6 >> 0x10);
  uStack6 = SUB42(paVar6,0x0);
  pass1_1020_8712(param_1 & 0xffff | uVar9 << 0x10,(iVar8 + 0x8),
                  paVar6,(param_1 & 0xffff0000 | uVar3));
  uVar1 = (iVar8 + 0x22);
  pass1_1018_26c2(uVar1,(uVar1 >> 0x10),puStack76);
  paVar7 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x2,unaff_SS);
  struct_op_1008_48fe((astruct_81 *)
                      CONCAT13((unaff_SS >> 0x8),CONCAT12(unaff_SS,local_2a)),
                      0x1,paVar7,(paVar7 >> 0x10));
  struct_op_1008_3f92((astruct_76 *)CONCAT22(unaff_SS,local_48),
                      (astruct_83 *)CONCAT22(unaff_SS,local_2a));
  uVar2 = (iVar8 + 0x8);
  pass1_1020_8712(param_1 & 0xffff | uVar9 << 0x10,
                  (uVar2 & 0xffff0000 | (uVar2 + 0x8)),
                  (astruct_76 *)CONCAT22(unaff_SS,local_48),puStack76);
  pass1_1008_41bc(CONCAT22(unaff_SS,local_48));
  close_file_1008_496c(local_2a,unaff_SS);
  return;
}



fn pass1_1020_8bae(param_1: *mut u16)
{
  *param_1 = 0x8e92;
  (param_1 + 0x2) = 0x1020;
  pass1_1020_8556(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_8bcc(param_1: u32,param_2: u16)
{
  let uVar1: u32;
  let puVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let puVar5: *mut u16;
  let puVar6: *mut u8
  let uVar7: u16;
  let extraout_DX: u16;
  astruct_285 *iVar9;
  astruct_286 *iVar10;
  let uVar8: u16;
  let uVar9: u16;
  astruct_43 *paVar10;
  u8 local_58 [0x1e];
  let local_3a: [u8;26];
  let uStack20: u32;
  let uStack12: u16;
  astruct_76 *paStack10;
  let uStack6: u32;
  
  uVar8 = (param_1 >> 0x10);
  iVar9 = (astruct_285 *)param_1;
  if (iVar9.field_0x4 != 0x0) {
    uVar1 = iVar9.field_0x22;
    uStack6 = (uVar1 + 0xa);
    paStack10 = (astruct_76 *)pass1_1018_268e(iVar9.field_0x22);
    uVar9 = (paStack10 >> 0x10);
    uVar1 = iVar9.field_0x22;
    uStack12 = (uVar1 + 0x16);
    if (*iVar9.field_0xc == 0x0) {
      uStack20 = pass1_1008_4772(paStack10);
      puVar6 = (uStack20 >> 0x10);
      uVar3 = uStack20;
      mem_op_1000_179c(0x14,puVar6,0x1000);
      uVar7 = puVar6 | uVar3;
      if (uVar7 == 0x0) {
        *iVar9.field_0xc = 0x0;
      }
      else {
        puVar5 = (param_1 & 0xffff0000 | &iVar9.field_0x16);
        uVar9 = (uStack20 >> 0x10);
        pass1_1008_50c2((Struct110 *)CONCAT22(puVar6,uVar3),
                        (uStack20 + 0x8),(uStack20 + 0x4),
                        puVar5,uStack6);
        puVar2 = iVar9.field_0xc;
        puVar2 = puVar5;
        (puVar2 + 0x2) = uVar7;
      }
      pass1_1008_5134(*iVar9.field_0xc);
      paVar10 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x2,param_2);
      struct_op_1008_48fe((astruct_81 *)CONCAT22(param_2,local_3a),0x1,paVar10,
                          (paVar10 >> 0x10));
      struct_op_1008_3f92((astruct_76 *)CONCAT22(param_2,local_58),
                          (astruct_83 *)CONCAT22(param_2,local_3a));
      uStack20 = pass1_1008_4772((astruct_76 *)CONCAT22(param_2,local_58));
      puVar6 = (uStack20 >> 0x10);
      uVar3 = uStack20;
      mem_op_1000_179c(0x14,puVar6,0x1000);
      uVar7 = puVar6 | uVar3;
      if (uVar7 == 0x0) {
        puVar2 = iVar9.field_0xc;
        (puVar2 + 0x4) = 0x0;
      }
      else {
        uVar4 = &iVar9.field_0x16;
        uVar9 = (uStack20 >> 0x10);
        pass1_1008_50c2((Struct110 *)CONCAT22(puVar6,uVar3),
                        (uStack20 + 0x8),(uStack20 + 0x4),
                        (param_1 & 0xffff0000 | uVar4),uStack6);
        puVar2 = iVar9.field_0xc;
        uVar9 = (puVar2 >> 0x10);
        iVar10 = (astruct_286 *)puVar2;
        iVar10.field_0x4 = uVar4;
        iVar10.field_0x6 = uVar7;
      }
      puVar2 = iVar9.field_0xc;
      pass1_1008_5134((puVar2 + 0x4));
      pass1_1008_41bc(CONCAT22(param_2,local_58));
      close_file_1008_496c(local_3a,param_2);
      uVar9 = extraout_DX;
    }
    puVar2 = iVar9.field_0xc;
    pass1_1008_5236((puVar2 + 0x4));
    pass1_1008_5236(*iVar9.field_0xc);
    uVar3 = &iVar9.field_0x16;
    pass1_1008_4480(uStack6,(param_1 & 0xffff0000 | uVar3),paStack10,
                    param_2);
    invalidate_rect_1020_8d90(param_1,uStack12,uStack6,uVar3,uVar9,param_2);
  }
  return;
}


astruct_18 *  pass1_1020_8e6c(astruct_18 *param_1,param_2: u8)

{
  pass1_1020_8bae(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_8eaa(param_1: *mut u16,param_2: u16)
{
  let uVar1: u16;
  let uVar2: u16;
  let puVar3: *mut u8
  astruct_668 *iVar4;
  let uVar4: u16;
  let puVar5: *mut u16;
  astruct_43 *paVar6;
  let local_a: [u8;8];
  
  struct_1020_847a(param_1,0x25,param_2);
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_668 *)param_1;
  &iVar4.field_0x16 = 0x0;
  iVar4.field_0xaa = 0x0;
  uVar1 = &iVar4.field_0xae;
  puVar5 = pass1_1008_3e38((param_1 & 0xffff0000 | uVar1));
  &iVar4.field_0xb4 = 0x0;
  iVar4.field_0xb8 = 0xffff;
  &iVar4.field_0xba = 0x0;
  *param_1 = 0x9204;
  iVar4.field_0x2 = 0x1020;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,param_2,
                           (puVar5 >> 0x10),uVar4);
  uVar2 = (puVar5 >> 0x10);
  iVar4.field_0x16 = puVar5;
  iVar4.field_0x18 = uVar2;
  pass1_1018_2646(iVar4.field_0x16,uVar2,
                  (param_1 & 0xffff0000 | uVar1));
  paVar6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1ce,param_2);
  puVar3 = (paVar6 >> 0x10);
  iVar4.field_0xb4 = paVar6;
  iVar4.field_0xb6 = puVar3;
  pass1_1020_8712(param_1 & 0xffff | uVar4 << 0x10,
                  CONCAT22(param_2,local_a),
                  (astruct_76 *)(paVar6 & 0xffff0000 | iVar4.field_0xb4),
                  (param_1 & 0xffff0000 | uVar1));
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_2,puVar3,uVar4);
  iVar4.field_0xba = puVar5;
  iVar4.field_0xbc = (puVar5 >> 0x10);
  return;
}



fn pass1_1020_8f74(param_1: *mut u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_593 *iVar4;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_593 *)param_1;
  *param_1 = 0x9204;
  iVar4.field_0x2 = 0x1020;
  puVar1 = iVar4.field_0xb4;
  uVar2 = iVar4.field_0xb6;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  pass1_1020_8556(param_1);
  return;
}


fn pass1_1020_9068(param_1: *mut u32,uchar *param_2,param_3: i16,param_4: u16)
{
  let iVar1: i16;
  let uVar2: u32;
  code **ppcVar3;
  let uVar4: u32;
  let uVar5: u16;
  let uVar6: u32;
  let extraout_DX: u16;
  let uVar7: u16;
  let iVar8: i16;
  let iVar9: i16;
  let uVar10: u16;
  let uVar11: u16;
  let uStack10: i16;
  
  uVar10 = (param_1 >> 0x10);
  iVar8 = param_1;
  uVar4 = (iVar8 + 0x16);
  uVar2 = (uVar4 + 0xa);
  uVar6 = uVar2;
  pass1_1018_280c((iVar8 + 0x16));
  (iVar8 + 0xaa) = uVar6;
  *(uchar **)(iVar8 + 0xac) = param_2;
  uVar5 = param_2 | (iVar8 + 0xaa);
  if (uVar5 == 0x0) {
    pass1_1018_2862((iVar8 + 0x16));
    (iVar8 + 0xaa) = uVar5;
    *(uchar **)(iVar8 + 0xac) = param_2;
  }
  if (((iVar8 + 0xac) | (iVar8 + 0xaa)) != 0x0) {
    pass1_1020_915a(param_1 & 0xffff | uVar10 << 0x10,param_2,param_3,
                    param_4);
    pass1_1008_4480(uVar2,(param_1 & 0xffff0000 | (iVar8 + 0xae)),
                    *(astruct_76 **)(iVar8 + 0xb4),param_4);
    ppcVar3 = (*param_1 + 0x10);
    (**ppcVar3)();
    uVar4 = (iVar8 + 0xaa);
    iVar1 = (uVar4 + 0xa);
    for (iStack10 = 0x0; iStack10 < iVar1; iStack10 += 0x1) {
      uVar6 = SEXT24(iStack10);
      empty_1008_8fc4((iVar8 + 0xaa),uVar6);
      uVar5 = uVar6;
      uVar7 = extraout_DX | uVar5;
      if (uVar7 != 0x0) {
        pass1_1008_8c4e(uVar6 & 0xffff | extraout_DX << 0x10,uVar2,param_4);
        uVar4 = (iVar8 + 0xc);
        uVar11 = (uVar4 >> 0x10);
        iVar9 = uVar4;
        (iVar9 + iStack10 * 0x4) = uVar5;
        (iVar9 + iStack10 * 0x4 + 0x2) = uVar7;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_915a(param_1: u32,uchar *param_2,param_3: i16,param_4: u16)
{
  let iVar1: i16;
  astruct_669 *iVar2;
  let uVar2: u16;
  let puVar3: *mut u16;
  astruct_43 *paVar4;
  let uStack12: u16;
  
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_4,param_2,param_3);
  iVar1 = (puVar3 + 0x1e);
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_669 *)param_1;
  if (iVar2.field_0xb8 != iVar1) {
    uStack12 = 0x1ce;
    if (iVar1 == 0x1) {
      uStack12 = 0x1cf;
    }
    else {
      if (iVar1 == 0x2) {
        uStack12 = 0x1d0;
      }
      else {
        if (iVar1 == 0x3) {
          uStack12 = 0x1d1;
        }
        else {
          if (iVar1 == 0x4) {
            uStack12 = 0x1d2;
          }
        }
      }
    }
    paVar4 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,uStack12,param_4);
    iVar2.field_0xb4 = paVar4;
    iVar2.field_0xb6 = (paVar4 >> 0x10);
    iVar2.field_0xb8 = iVar1;
  }
  return;
}



astruct_18 *  pass1_1020_91de(astruct_18 *param_1,param_2: u8)

{
  pass1_1020_8f74(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


astruct_18 * 
pass1_1020_96a2(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  palette_op_1020_92c4(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1020_a426(void) -> u16

{
  let puVar1: *mut u16;
  
  pass1_1008_3e38(&ctx.PTR_LOOP_1048_4230);
  puVar1 = pass1_1008_3e38(0x10484236);
  return puVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_a43e(param_1: u16,uchar *param_2,param_3: *mut u16) -> u16

{
  let unaff_DI: i16;
  
  *param_3 = 0xba36;
  (param_3 + 0x2) = 0x1020;
  if (_PTR_LOOP_1050_4e74 != 0x0) {
    return param_3;
  }
  mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_1,param_2,unaff_DI);
  if ((0x0 < ctx.PTR_LOOP_1050_13ae) && (!SBORROW2(ctx.PTR_LOOP_1050_13ae,0x1))) {
    if (ctx.PTR_LOOP_1050_13ae == &ctx.PTR_LOOP_1050_0002 ||
        (ctx.PTR_LOOP_1050_13ae + -0x1) < 0x1) {
      ctx.PTR_LOOP_1050_4e74 = 0x44b4;
      goto LAB_1020_a482;
    }
    if (ctx.PTR_LOOP_1050_13ae == &DAT_1050_0004) {
      ctx.PTR_LOOP_1050_4e74 = 0x4b2c;
      goto LAB_1020_a482;
    }
  }
  ctx.PTR_LOOP_1050_4e74 = 0x47f0;
//LAB_1020_a482:
  _PTR_LOOP_1050_4e74 = CONCAT22(0x1050,PTR_LOOP_1050_4e74);
  return param_3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_a49a(param_1: u16,param_2: u8,uchar *param_3,param_4: u32,i16 *param_5,
               param_6: u16)

{
  let uVar1: u32;
  let unaff_DI: i16;
  let uVar2: u16;
  let uVar3: u16;
  let local_136: [u8;128];
  let uStack14: u16;
  let uStack12: u16;
  let uStack10: u32;
  let puStack6: *mut u16;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_1,param_3,unaff_DI);
  uStack12 = (puStack6 >> 0x10);
  uVar1 = (puStack6 + 0x20);
  uStack10 = uVar1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
  uStack14 = uVar1;
  if (param_5 != 0x0) {
    uVar2 = (param_5 >> 0x10);
    if ((param_5 + 0x1) == 0x0) {
      uVar3 = SUB42(&ctx.PTR_LOOP_1050_4230,0x0);
    }
    else {
      uVar3 = 0x4236;
    }
    pass1_1008_3f32(param_5,CONCAT22(0x1048,uVar3));
    struct_op_1028_87f0(param_1,param_2,(astruct_97 *)CONCAT22(param_1,local_136),0x0,0x0,
                        param_6,param_5,uVar2,
                        (_PTR_LOOP_1050_4e70 + 0x4),uStack10);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_1,local_136));
    return;
  }
  pass1_1020_abc0(param_1,param_2,param_4,param_6,uVar1 & 0xffff | uStack12 << 0x10
                 );
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_a54c(param_1: u16,param_2: u8,uchar *param_3,param_4: u16,param_5: u16,
               param_6: i16)

{
  let uVar1: u32;
  let unaff_DI: i16;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let local_140: [u8;124];
  let puStack28: u32;
  let local_18: i16;
  let local_16: u16;
  let local_14: u32;
  let puStack16: *mut u8;
  let uStack14: u16;
  let uStack12: u16;
  let uStack10: u32;
  let puStack6: *mut u16;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_1,param_3,unaff_DI);
  uStack12 = (puStack6 >> 0x10);
  uVar1 = (puStack6 + 0x20);
  uStack10 = uVar1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
  uStack14 = uVar1;
  local_14 = _PTR_LOOP_1048_4230;
  puStack16 = ctx.PTR_LOOP_1048_4234;
  puStack28 = &local_14;
  pass1_1008_3e94(CONCAT22(param_1,&local_14),
                  CONCAT22(param_1,&local_18),
                  CONCAT22(param_1,&local_16));
  if ((param_6 < 0x0) || (0x5 < param_6)) {
    pass1_1008_3e76(CONCAT22(param_1,&local_14),0x0,local_18 - 0x9,local_16);
    uVar5 = uStack10;
    uVar6 = (uStack10 >> 0x10);
    uVar1 = (_PTR_LOOP_1050_4e70 + 0x4);
    uVar3 = uVar1;
    uVar4 = (uVar1 >> 0x10);
    uVar2 = 0x14;
  }
  else {
    pass1_1008_3e76(CONCAT22(param_1,&local_14),0x0,(local_18 - param_6) - 0x3,
                    local_16);
    uVar5 = uStack10;
    uVar6 = (uStack10 >> 0x10);
    uVar1 = (_PTR_LOOP_1050_4e70 + 0x4);
    uVar3 = uVar1;
    uVar4 = (uVar1 >> 0x10);
    uVar2 = 0x7b;
  }
  struct_op_1028_87f0(param_1,param_2,(astruct_97 *)CONCAT22(param_1,local_140),0x0,0x0,
                      uVar2,&local_14,param_1,CONCAT22(uVar4,uVar3),CONCAT22(uVar6,uVar5))
  ;
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_1,local_140));
  return;
}



fn pass1_1020_a644(param_1: u16,param_2: u16,param_3: u32,param_4: u16) -> bool

{
  let BVar1: bool;
  
  BVar1 = write_to_file_1008_7cac(param_3,param_4);
  if (BVar1 != 0x0) {
    BVar1 = 0x1;
  }
  return BVar1;
}



void 
pass1_1020_a6ee(param_1: u32,param_2: u16,param_3: u16,param_4: u16,param_5: i16,
               param_6: u16,param_7: u8)

{
  let uVar1: u32;
  let uVar2: u16;
  let puVar3: *mut u16;
  let uVar4: u16;
  let local_13e: [u8;120];
  let uStack30: u32;
  let BStack26: bool;
  let local_18: u32;
  let uStack20: u16;
  let iStack18: i16;
  let uStack16: u16;
  let uStack14: u32;
  let puStack10: *mut u16;
  let uStack6: u32;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x2,0x400);
  uStack6 = CONCAT22(param_4,param_3);
  if (((uchar *)(param_4 | param_3) == 0x0) ||
     ((param_3 + 0x200) == 0x8000002)) {
    puStack10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,
                                (param_4 | param_3),param_5);
    uStack16 = (puStack10 >> 0x10);
    uVar1 = (puStack10 + 0x20);
    uStack14 = uVar1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
    iStack18 = uVar1;
    puVar3 = pass1_1008_3e38(CONCAT22(param_6,&local_18));
    uVar2 = (puVar3 >> 0x10);
    BStack26 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,param_2,0x28);
    if (BStack26 != 0x0) {
      uStack20 = 0x1;
    }
    uVar4 = (param_1 >> 0x10);
    pass1_1020_b2da(param_6,param_1,uVar4,(BStack26 != 0x0),
                    CONCAT22(param_6,&local_18),CONCAT22(uStack16,iStack18));
    struct_op_1028_87f0(param_6,param_7,(astruct_97 *)CONCAT22(param_6,local_13e),0x0,0x0,
                        param_2,&local_18,param_6,
                        (_PTR_LOOP_1050_4e70 + 0x4),
                        (iStack18 + 0x4));
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_6,local_13e));
    if (BStack26 != 0x0) {
      pass1_1020_ad90(param_6,uVar2,param_1,uVar4,
                      CONCAT22(param_6,&local_18),(iStack18 + 0x4));
    }
    (uStack30 + 0x1c) = 0x8000001;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_a80e(param_1: u16,param_2: u16,param_3: i16,param_4: u16,param_5: u16,
               param_6: u16,param_7: u8,param_8: i16)

{
  let uVar1: u16;
  let uVar2: u32;
  let uVar3: u16;
  let puVar4: *mut u16;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x2,0x400);
  if (((uchar *)(param_5 | param_4) == 0x0) ||
     ((param_4 + 0x200) == 0x8000002)) {
    puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,(param_5 | param_4)
                             ,param_8);
    uVar3 = (puVar4 >> 0x10);
    uVar2 = (puVar4 + 0x20);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,(uVar2 >> 0x10));
    uVar1 = uVar2;
    if (param_3 == 0xa) {
      pass1_1020_b872(param_6,param_7,CONCAT22(param_2,param_1),
                      uVar2 & 0xffff | uVar3 << 0x10);
      return;
    }
    pass1_1020_b0aa(param_1,param_2,param_3,uVar3);
    if (uVar1 != 0x0) {
      pass1_1020_abc0(param_6,param_7,CONCAT22(param_2,param_1),uVar1,
                      uVar2 & 0xffff | uVar3 << 0x10);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_a89e(param_1: u16,param_2: u32,param_3: *mut u32,param_4: u16)
{
  let piVar1: *mut i16;
  let puVar2: *mut u8;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u32;
  let in_DX: *mut u8
  let uVar6: u16;
  let puVar7: u32;
  let extraout_DX: u16;
  let unaff_DI: i16;
  ulet in_AF: u8;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u8;
  let uVar11: u8;
  let local_5ee: u16;
  let uStack1516: u16;
  let puStack1218: u32;
  let iStack1214: i16;
  let uStack1212: u32;
  let local_4b8: [u8;8];
  let uStack1200: u32;
  let puStack1196: *mut u16;
  let local_4a8: [u8;124];
  let local_384: [u8;124];
  let local_260: [u8;124];
  let local_13c: [u8;124];
  let local_18: u16;
  let local_16: u16;
  let local_14: u32;
  let uStack16: u16;
  let uStack14: u32;
  let uStack10: u32;
  let puStack6: *mut u16;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_1,in_DX,unaff_DI);
  uVar6 = (puStack6 >> 0x10);
  uVar5 = (puStack6 + 0x20);
  uStack10 = uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5,(uVar5 >> 0x10));
  uStack14 = uVar5 & 0xffff | uVar6 << 0x10;
  local_14 = *param_3;
  uStack16 = (param_3 + 0x1);
  puStack1218 = &local_14;
  puVar7 = &local_14;
  pass1_1008_3e94(CONCAT22(param_1,puVar7),CONCAT22(param_1,&local_18)
                  ,CONCAT22(param_1,&local_16));
  uVar10 = (u8)param_1;
  uVar11 = (u8)(param_1 >> 0x8);
  pass1_1008_3e76(CONCAT13(uVar11,CONCAT12(uVar10,&local_14)),0x0,local_18,
                  local_16 + 0x2);
  struct_op_1028_8888(param_1,in_AF,(astruct_100 *)CONCAT22(param_1,local_13c),0x0,0x7a,
                      &local_14,param_1,0x8000002,0x4000002,uStack10);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_1,local_13c));
  pass1_1008_3e76(CONCAT13(uVar11,CONCAT12(uVar10,&local_14)),0x0,local_18 - 0x2
                  ,local_16);
  struct_op_1028_8888(param_1,in_AF,(astruct_100 *)CONCAT22(param_1,local_260),0x0,0x47,
                      &local_14,param_1,0x8000002,0x4000002,uStack10);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_1,local_260));
  pass1_1008_3e76(CONCAT13(uVar11,CONCAT12(uVar10,&local_14)),0x1,local_18 - 0x2
                  ,local_16);
  struct_op_1028_8888(param_1,in_AF,(astruct_100 *)CONCAT22(param_1,local_384),0x0,0x6a,
                      &local_14,param_1,0x8000002,0x4000002,uStack10);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_1,local_384));
  uVar8 = param_2;
  uVar9 = (param_2 >> 0x10);
  pass1_1020_ad90(param_1,puVar7,uVar8,uVar9,CONCAT22(param_1,&local_14)
                  ,uStack10);
  pass1_1008_3e76(CONCAT13(uVar11,CONCAT12(uVar10,&local_14)),0x1,local_18 - 0x2
                  ,local_16 + 0x1);
  struct_op_1028_8888(param_1,in_AF,(astruct_100 *)CONCAT22(param_1,local_4a8),0x0,0x7f,
                      &local_14,param_1,0x8000002,0x4000002,uStack10);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_1,local_4a8));
  pass1_1020_ad90(param_1,puVar7,uVar8,uVar9,CONCAT22(param_1,&local_14)
                  ,uStack10);
  puStack1196 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_1,puVar7,
                                &uStack14);
  uStack1200 = (puStack1196 + 0x12);
  pass1_1008_5784(CONCAT22(param_1,local_4b8),uStack1200);
  iStack1214 = 0x0;
  do {
    do {
      puVar2 = local_4b8;
      pass1_1008_5b12(puVar2,param_1);
      uStack1212 = CONCAT22(extraout_DX,puVar2);
      if ((extraout_DX | puVar2) == 0x0) {
        pass1_1010_9674(puStack1196);
        return;
      }
    } while (((puVar2 + 0x4) != 0x3e) && ((puVar2 + 0x4) != 0x41));
    while (0x0 < (uStack1212 + 0x6)) {
      if (iStack1214 == 0x0) {
        uVar4 = local_16 - 0x2;
//LAB_1020_ab4a:
        uVar3 = local_18 - 0x2;
//LAB_1020_ab51:
        iStack1214 = iStack1214 + 0x1;
        pass1_1008_3e76(CONCAT13(uVar11,CONCAT12(uVar10,&local_14)),0x0,uVar3,
                        uVar4);
      }
      else {
        if (iStack1214 == 0x1) {
          uVar4 = local_16 + 0x2;
          goto LAB_1020_ab4a;
        }
        if (iStack1214 == 0x2) {
          uVar4 = local_16 + 0x2;
//LAB_1020_ab6e:
          uVar3 = local_18 + 0x2;
          goto LAB_1020_ab51;
        }
        if (iStack1214 == 0x3) {
          uVar4 = local_16 - 0x2;
          goto LAB_1020_ab6e;
        }
        iStack1214 = iStack1214 + 0x1;
        pass1_1020_b2da(param_1,uVar8,uVar9,0x0,CONCAT22(param_1,&local_14),
                        uStack14);
      }
      struct_op_1028_8888(param_1,in_AF,(astruct_100 *)CONCAT22(param_1,&local_5ee),0x0,
                          (uStack1212 + 0x4),&local_14,param_1,0x8000002,
                          0x4000002,uStack10);
      fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_1,&local_5ee));
      piVar1 = (uStack1212 + 0x6);
      *piVar1 = *piVar1 + -0x1;
      local_5ee = 0x389a;
      uStack1516 = 0x1008;
    }
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_abc0(param_1: u16,param_2: u8,param_3: u32,param_4: u16,param_5: u32)
{
  let uVar1: u16;
  let uVar2: u16;
  let puVar3: *mut u16;
  let uVar4: u16;
  let local_12e: [u8;124];
  let BStack10: bool;
  let local_8: u32;
  let uStack4: u16;
  
  puVar3 = pass1_1008_3e38(CONCAT22(param_1,&local_8));
  uVar1 = (puVar3 >> 0x10);
  BStack10 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,param_4,0x28);
  if (BStack10 != 0x0) {
    uStack4 = 0x1;
  }
  uVar4 = (param_3 >> 0x10);
  pass1_1020_b2da(param_1,param_3,uVar4,(BStack10 != 0x0),
                  CONCAT22(param_1,&local_8),param_5);
  uVar2 = (param_5 >> 0x10);
  struct_op_1028_87f0(param_1,param_2,(astruct_97 *)CONCAT22(param_1,local_12e),0x0,0x0,
                      param_4,&local_8,param_1,(_PTR_LOOP_1050_4e70 + 0x4),
                      (param_5 + 0x4));
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_1,local_12e));
  if (BStack10 != 0x0) {
    pass1_1020_ad90(param_1,uVar1,param_3,uVar4,
                    CONCAT22(param_1,&local_8),(param_5 + 0x4));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_ac6e(param_1: u16,param_2: u8,param_3: u32,param_4: i16,param_5: i16,
               param_6: i16)

{
  let uVar1: u16;
  let puVar2: u32;
  let uVar3: u32;
  let uVar4: u16;
  let unaff_DI: i16;
  let puVar5: *mut u16;
  let uVar6: u16;
  u8 local_146 [0x12c];
  let iStack26: i16;
  let uStack24: u16;
  let uStack22: u32;
  let puStack18: *mut u16;
  let local_e: u32;
  let local_8: u16;
  let local_6: u32;
  
  if (param_4 == 0x0) {
    uVar6 = SUB42(&ctx.PTR_LOOP_1050_4230,0x0);
  }
  else {
    uVar6 = 0x4236;
  }
  pass1_1008_3eb4(CONCAT22(0x1048,uVar6),CONCAT22(param_1,&local_8),
                  CONCAT22(param_1,&local_6),
                  CONCAT22(param_1,&local_6 + 0x2));
  if (param_6 == 0x0) {
    local_6 = local_6 & 0xffff | (local_6._2_2_ + param_5) << 0x10;
  }
  else {
    if (param_6 == 0x1) {
      local_6 = local_6 & 0xffff0000 | (local_6 + param_5);
    }
    else {
      if (param_6 == 0x2) {
        local_6 = local_6 & 0xffff | (local_6._2_2_ - param_5) << 0x10;
      }
    }
  }
  puVar5 = pass1_1008_3e54(CONCAT22(param_1,&local_e),local_8,local_6,
                           (local_6 >> 0x10));
  puStack18 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_1,
                              (puVar5 >> 0x10),unaff_DI);
  uVar4 = (puStack18 >> 0x10);
  uVar3 = (puStack18 + 0x20);
  uStack22 = uVar3;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,(uVar3 >> 0x10));
  iStack26 = uVar3;
  uStack24 = uVar4;
  uVar1 = pass1_1020_b1ae(&local_e,uVar4,param_1,param_3,
                          (param_3 >> 0x10),CONCAT22(param_1,&local_e),
                          (iStack26 + 0x4));
  if (uVar1 != 0x0) {
    puVar2 = &local_e;
    pass1_1020_b240(param_1,uVar4,param_3,CONCAT22(param_1,puVar2),
                    CONCAT22(uStack24,iStack26));
    if (puVar2 != 0x0) {
      struct_op_1028_87f0(param_1,param_2,(astruct_97 *)CONCAT22(param_1,local_146),0x0,
                          0x0,(-(param_4 == 0x0) & 0xfffb) + 0x7f,&local_e,param_1,
                          (_PTR_LOOP_1050_4e70 + 0x4),uStack22);
      fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_1,local_146));
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_ad90(param_1: u16,param_2: u16,param_3: u16,param_4: u16,
               param_5: *mut u16,param_6: u32)

{
  code **ppcVar1;
  let puVar2: *mut u16;
  let puVar3: *mut u8;
  let iVar4: i16;
  let uVar5: u32;
  let uVar6: u16;
  let extraout_DX: u16;
  ulet in_AF: u8;
  let puVar7: u32;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let local_17e: u16;
  let uStack380: u16;
  let iStack90: i16;
  let puStack78: u32;
  let uStack70: u16;
  let iStack68: i16;
  let uStack66: u32;
  let puStack62: u32;
  u8 local_3a [0xc];
  let local_2e: u32;
  let uStack42: u16;
  let iStack40: i16;
  let uStack38: u16;
  let local_24: i16;
  let local_22: i16;
  let uStack32: u32;
  let uStack28: u32;
  let uStack24: u32;
  let puStack20: *mut u16;
  let uStack18: u16;
  let iStack16: i16;
  let iStack14: i16;
  let uStack12: u32;
  let local_8: u16;
  let local_6: i16;
  let local_4: i16;
  
  puVar2 = &local_8;
  pass1_1008_3eb4(param_5,CONCAT22(param_1,puVar2),
                  CONCAT22(param_1,&local_6),
                  CONCAT22(param_1,&local_4));
  pass1_1030_627e(param_1,puVar2,param_2,_PTR_LOOP_1050_5740,param_5,param_6)
  ;
  puStack20 = puVar2;
  uStack18 = param_2;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,puVar2,param_2);
  uStack24 = CONCAT22(param_2,puVar2);
  uStack28 = (puVar2 + 0x17);
  uVar5 = (uStack28 + 0x4);
  uStack32 = uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_6,(param_6 >> 0x10));
  iStack40 = uVar5;
  uStack38 = param_2;
  puVar7 = pass1_1030_5b5c(iStack40,param_2);
  uVar6 = (puVar7 >> 0x10);
  local_2e = *puVar7;
  uStack42 = (puVar7 + 0x4);
  puStack78 = &local_2e;
  pass1_1008_3e94(CONCAT22(param_1,&local_2e),
                  CONCAT22(param_1,&local_24),
                  CONCAT22(param_1,&local_22));
  iStack14 = local_4 + 0x1;
  uStack12 = CONCAT22(local_4 + -0x1,local_6 - 0x1);
  iStack16 = local_6 + 0x1;
  if (local_4 + -0x1 < 0x0) {
    uStack12 = (local_6 - 0x1);
  }
  if (local_22 <= iStack14) {
    iStack14 = local_22 + -0x1;
  }
  if (uStack12 < 0x0) {
    uStack12 &= 0xffff0000;
  }
  if (local_24 <= iStack16) {
    iStack16 = local_24 + -0x1;
  }
  pass1_1008_6c90(CONCAT22(param_1,local_3a));
  pass1_1008_6cec(CONCAT22(param_1,local_3a),local_8,CONCAT22(iStack14,iStack16)
                  ,local_8,uStack12);
  puVar3 = local_3a;
  pass1_1030_6522(_PTR_LOOP_1050_5740,CONCAT22(param_1,puVar3),param_6,param_1);
  puStack62 = CONCAT22(uVar6,puVar3);
  if ((uVar6 | puVar3) != 0x0) {
    uStack66 = 0x0;
    iStack68 = 0x0;
    for (uStack70 = uStack12; uStack70 <= iStack16; uStack70 += 0x1) {
      for (puStack78 = uStack12._2_2_; puStack78 <= iStack14;
          puStack78 = (puStack78 + 0x1)) {
        ppcVar1 = (*puStack62 + 0x4);
        iVar4 = iStack68;
        iStack68 = iStack68 + 0x1;
        (**ppcVar1)(0x1030,puStack62,(puStack62 >> 0x10));
        uStack66 = CONCAT22(extraout_DX,iVar4);
        uStack66._3_1_ = (extraout_DX >> 0x8);
        if (uStack66._3_1_ == '\0') {
          iStack90 = iVar4;
          if (iVar4 == 0x7) {
            pass1_1008_3e76(param_5,local_8,uStack70,puStack78);
            uVar9 = uStack32;
            uVar10 = (uStack32 >> 0x10);
            uVar8 = 0x6;
          }
          else {
            if (iVar4 == 0x8) {
              pass1_1008_3e76(param_5,local_8,uStack70,puStack78);
              uVar9 = uStack32;
              uVar10 = (uStack32 >> 0x10);
              uVar8 = 0x7;
            }
            else {
              if (iVar4 != 0x9) goto LAB_1020_af1c;
              pass1_1008_3e76(param_5,local_8,uStack70,puStack78);
              uVar9 = uStack32;
              uVar10 = (uStack32 >> 0x10);
              uVar8 = 0x8;
            }
          }
          struct_op_1028_87f0(param_1,in_AF,(astruct_97 *)CONCAT22(param_1,&local_17e),0x0
                              ,0x0,uVar8,param_5,(param_5 >> 0x10)
                              ,CONCAT22(uVar10,uVar9),param_6);
          fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_1,&local_17e));
          local_17e = 0x389a;
          uStack380 = 0x1008;
        }
//LAB_1020_af1c:
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_afc4(param_1: u16,param_2: u16,param_3: u16,param_4: u16,
               param_5: *mut u16,param_6: i32)

{
  let puVar1: u32;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u32;
  let bStack27: u8;
  let local_a: u32;
  let uStack6: u32;
  
  puVar1 = &local_a;
  pass1_1030_64ce(param_1,puVar1,param_2,_PTR_LOOP_1050_5740,param_5,param_6,
                  CONCAT22(param_1,puVar1));
  uStack6 = *puVar1;
  uVar3 = (puVar1 + 0x2);
  bStack27 = (uStack6 >> 0x18);
  uVar2 = bStack27;
  if (bStack27 == 0x0) {
    return;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6,uVar3);
  uVar4 = struct_op_1030_73a8(CONCAT22(uVar3,uVar2));
  uVar3 = (uVar4 >> 0x10);
  if ((uVar3 | uVar4) != 0x0) {
    switch((uVar4 + 0xc)) {
    case 0x1:
      break;
    case 0x2:
      break;
    case 0x3:
      break;
    case 0x4:
      break;
    case 0x5:
      break;
    case 0x6:
      break;
    case 0x7:
      return;
    case 0x8:
      return;
    case 0x9:
      return;
    }
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_b0aa(param_1: u16,param_2: u16,param_3: i16,param_4: u16)
{
  let puVar1: u32;
  code **ppcVar2;
  let iVar3: i16;
  let puVar4: u32;
  let extraout_DX: u16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u32;
  let uStack20: u32;
  
  uVar7 = (_PTR_LOOP_1050_4e74 >> 0x10);
  if ((_PTR_LOOP_1050_4e74 + param_3 * 0x6 + 0x4) == 0x0) {
    return;
  }
  if ((_PTR_LOOP_1050_4e74 + param_3 * 0x6 + 0x4) != -0x1) {
    if (ctx.PTR_LOOP_1050_4e78 == 0x0) {
      iVar3 = param_3;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x2,0x400);
      puVar1 = (iVar3 + 0xc);
      ppcVar2 = (*puVar1 + 0x10);
      puVar4 = puVar1;
      (**ppcVar2)();
      uVar6 = extraout_DX;
      for (uStack20 = 0x0;
          uStack20 < (puVar4 & 0xffff | extraout_DX << 0x10);
          uStack20 += 0x1) {
        uVar8 = pass1_1030_1d7c((puVar4 & 0xffff),uVar6,puVar1);
        uVar5 = (uVar8 >> 0x10);
        uVar6 = uVar5 | uVar8;
        if ((uVar6 != 0x0) &&
           ((iVar3 = (uVar8 + 0xc), iVar3 == 0x2a || (iVar3 == 0x2b)))) {
          ctx.PTR_LOOP_1050_4e78 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
          break;
        }
      }
      if (ctx.PTR_LOOP_1050_4e78 == 0x0) {
        ctx.PTR_LOOP_1050_4e78 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
        return;
      }
    }
    iVar3 = (_PTR_LOOP_1050_4e74 + param_3 * 0x6 + 0x4) + -0x1;
    pass1_1008_612e(0x0,iVar3,iVar3);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 
pass1_1020_b1ae(param_1: i16,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
               param_6: *mut u16,param_7: u32)

{
  let puVar1: u32;
  let local_14: i16;
  let local_12: i16;
  let local_10: i16;
  let local_e: i16;
  let local_c: u32;
  let uStack8: u16;
  let iStack6: i16;
  let uStack4: u16;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_7,(param_7 >> 0x10));
  iStack6 = param_1;
  uStack4 = param_2;
  puVar1 = pass1_1030_5b5c(param_1,param_2);
  local_c = *puVar1;
  uStack8 = (puVar1 + 0x4);
  pass1_1008_3e94(param_6,CONCAT22(param_3,&local_10),
                  CONCAT22(param_3,&local_e));
  pass1_1008_3e94(CONCAT22(param_3,&local_c),
                  CONCAT22(param_3,&local_14),
                  CONCAT22(param_3,&local_12));
  if ((((0xb < local_e) && (0xb < local_10)) && (local_e < local_12 + -0xb)) &&
     (local_10 < local_14 + -0xb)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_b240(param_1: u16,param_2: u16,param_3: u32,param_4: u32,param_5: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u32;
  let bStack31: u8;
  let local_a: u32;
  let uStack6: u32;
  
  puVar1 = &local_a;
  uVar6 = (param_5 >> 0x10);
  pass1_1030_64ce(param_1,puVar1,param_2,_PTR_LOOP_1050_5740,param_4,
                  (param_5 + 0x4),CONCAT22(param_1,puVar1));
  uStack6 = *puVar1;
  uVar5 = (puVar1 + 0x2);
  bStack31 = (uStack6 >> 0x18);
  uVar2 = bStack31;
  if (bStack31 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6,uVar5);
    uVar7 = struct_op_1030_73a8(CONCAT22(uVar5,uVar2));
    uVar4 = (uVar7 >> 0x10);
    uVar2 = uVar7;
    uVar5 = uVar4 | uVar2;
    if ((uVar5 != 0x0) && (uVar2 = (uVar2 + 0xc), 0x9 < uVar2)) {
      return;
    }
  }
  uVar3 = pass1_1020_b1ae(uVar2,uVar5,param_1,param_3,(param_3 >> 0x10),
                          param_4,(param_5 + 0x4));
  if (uVar3 == 0x0) {
    return;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void 
pass1_1020_b2da(param_1: u16,param_2: u16,param_3: u16,param_4: i16,param_5: *mut u16,
               param_6: u32)

{
  let iVar1: i16;
  let uVar2: u16;
  let uVar3: u16;
  let puVar4: *mut u8;
  let uVar5: u16;
  ulet in_AF: u8;
  let puVar6: *mut u16;
  u8 **ppuVar7;
  let iStack28: i16;
  let local_1a: [u8;6];
  let uStack20: u16;
  let uStack18: u16;
  let piStack16: *mut i16;
  let piStack12: *mut i16;
  let local_8: u16;
  let local_6: u32;
  
  if (param_4 == 0x0) {
    uVar2 = 0x4e6a;
  }
  else {
    uVar2 = 0x4e6e;
  }
  piStack12 = CONCAT22(0x1050,uVar2);
  if (param_4 == 0x0) {
    uStack20 = 0x4e68;
  }
  else {
    uStack20 = 0x4e6c;
  }
  uStack18 = SUB42(ctx.data_seg,0x0);
  piStack16 = CONCAT22(0x1050,uStack20);
  do {
    if (param_4 == 0x0) {
      ppuVar7 = &ctx.PTR_LOOP_1048_4230;
    }
    else {
      ppuVar7 = (u8 **)0x10484236;
    }
    pass1_1008_3eb4(ppuVar7,CONCAT22(param_1,&local_8),
                    CONCAT22(param_1,&local_6),
                    CONCAT22(param_1,&local_6 + 0x2));
    iVar1 = *piStack12;
    if (iVar1 == 0x0) {
      local_6 = CONCAT22(local_6._2_2_ + *piStack16,local_6 + -0x1);
    }
    else {
      if (iVar1 == 0x1) {
        local_6 = CONCAT22(local_6._2_2_ + -0x1,local_6 + *piStack16);
      }
      else {
        if (iVar1 == 0x2) {
          local_6 = CONCAT22(local_6._2_2_ - *piStack16,local_6 + -0x1);
        }
      }
    }
    puVar6 = pass1_1008_3e54(CONCAT22(param_1,local_1a),local_8,local_6,
                             (local_6 >> 0x10));
    uVar5 = (puVar6 >> 0x10);
    uVar2 = (param_6 >> 0x10);
    uVar3 = pass1_1020_b1ae(local_1a,uVar5,param_1,param_2,param_3,
                            CONCAT22(param_1,local_1a),
                            (param_6 + 0x4));
    if (uVar3 != 0x0) {
      puVar4 = local_1a;
      pass1_1020_b240(param_1,uVar5,CONCAT22(param_3,param_2),CONCAT22(param_1,puVar4),
                      param_6);
      if (puVar4 != 0x0) {
//LAB_1020_b46e:
        pass1_1008_3e76(param_5,local_8,local_6,(local_6 >> 0x10));
        return;
      }
    }
    iVar1 = *piStack12;
    if (iVar1 == 0x0) {
//LAB_1020_b45e:
      local_6 = local_6 & 0xffff0000 | (local_6 + 0x2);
    }
    else {
      if (iVar1 == 0x1) {
        local_6 = local_6 & 0xffff | (local_6._2_2_ + 0x2) << 0x10;
      }
      else {
        if (iVar1 == 0x2) goto LAB_1020_b45e;
      }
    }
    pass1_1008_3e76(CONCAT22(param_1,local_1a),local_8,local_6,
                    (local_6 >> 0x10));
    uVar3 = pass1_1020_b1ae(local_1a,uVar5,param_1,param_2,param_3,
                            CONCAT22(param_1,local_1a),
                            (param_6 + 0x4));
    if (uVar3 != 0x0) {
      puVar4 = local_1a;
      pass1_1020_b240(param_1,uVar5,CONCAT22(param_3,param_2),CONCAT22(param_1,puVar4),
                      param_6);
      if (puVar4 != 0x0) goto LAB_1020_b46e;
    }
    iStack28 = *piStack12 + 0x1;
    if (0x2 < iStack28) {
      iStack28 = 0x0;
      *piStack16 = *piStack16 + 0x1;
    }
    *piStack12 = iStack28;
    pass1_1020_ac6e(param_1,in_AF,CONCAT22(param_3,param_2),param_4,*piStack16,iStack28);
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_b482(param_1: u16,param_2: u32,param_3: *mut u32,param_4: u32)
{
  let puVar1: *mut u8;
  let puVar2: u32;
  let uVar3: u32;
  let uVar4: u16;
  let uVar5: u16;
  let puVar6: u32;
  let uVar7: u32;
  let uVar8: u16;
  let uVar9: u16;
  let puVar10: u32;
  let iStack46: i16;
  let local_2a: u32;
  let local_26: u16;
  let local_24: u32;
  let uStack32: u16;
  let lStack30: i32;
  let uStack26: u32;
  let local_16: [u8;12];
  let local_4: [u8;2];
  
  uVar7 = pass1_1030_bcae(local_4,param_1);
  uVar4 = (uVar7 >> 0x10);
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_1,local_16),0x1,0x0,0x400);
  while( true ) {
    puVar1 = local_16;
    pass1_1028_e4ec(CONCAT22(param_1,puVar1));
    uStack26 = CONCAT22(uVar4,puVar1);
    uVar5 = uVar4 | puVar1;
    if (uVar5 == 0x0) {
      pass1_1020_b240(param_1,0x0,param_2,param_3,param_4);
      if (puVar1 != 0x0) {
        lStack30 = (param_4 + 0x4);
        local_24 = *param_3;
        uStack32 = (param_3 + 0x4);
        puVar6 = &local_2a;
        pass1_1008_3eb4(CONCAT22(param_1,&local_24),
                        CONCAT22(param_1,puVar6),
                        CONCAT22(param_1,&local_2a + 0x2),
                        CONCAT22(param_1,&local_26));
        pass1_1008_3e76(CONCAT22(param_1,&local_24),local_2a,
                        local_2a._2_2_ - 0x1,local_26 - 0x1);
        puVar2 = &local_24;
        uVar8 = param_2;
        uVar9 = (param_2 >> 0x10);
        pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                        CONCAT22(param_1,puVar2),lStack30);
        if (puVar2 != 0x0) {
          pass1_1008_3e76(CONCAT22(param_1,&local_24),local_2a,
                          (local_2a >> 0x10),local_26 - 0x1);
          puVar2 = &local_24;
          pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                          CONCAT22(param_1,puVar2),lStack30);
          if (puVar2 != 0x0) {
            pass1_1008_3e76(CONCAT22(param_1,&local_24),local_2a,
                            local_2a._2_2_ + 0x1,local_26 - 0x1);
            puVar2 = &local_24;
            pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                            CONCAT22(param_1,puVar2),lStack30);
            if (puVar2 != 0x0) {
              pass1_1008_3e76(CONCAT22(param_1,&local_24),local_2a,
                              local_2a._2_2_ - 0x1,local_26);
              puVar2 = &local_24;
              pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                              CONCAT22(param_1,puVar2),lStack30);
              if (puVar2 != 0x0) {
                pass1_1008_3e76(CONCAT22(param_1,&local_24),local_2a,
                                local_2a._2_2_ + 0x1,local_26);
                puVar2 = &local_24;
                pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                CONCAT22(param_1,puVar2),lStack30);
                if (puVar2 != 0x0) {
                  pass1_1008_3e76(CONCAT22(param_1,&local_24),local_2a,
                                  local_2a._2_2_ + 0x1,local_26 + 0x1);
                  puVar2 = &local_24;
                  pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                  CONCAT22(param_1,puVar2),lStack30);
                  if (puVar2 != 0x0) {
                    pass1_1008_3e76(CONCAT22(param_1,&local_24),local_2a
                                    ,(local_2a >> 0x10),local_26 + 0x1);
                    puVar2 = &local_24;
                    pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                    CONCAT22(param_1,puVar2),lStack30);
                    if (puVar2 != 0x0) {
                      pass1_1008_3e76(CONCAT22(param_1,&local_24),
                                      local_2a,local_2a._2_2_ - 0x1,local_26 + 0x1
                                     );
                      puVar2 = &local_24;
                      pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                      CONCAT22(param_1,puVar2),lStack30);
                      if (puVar2 != 0x0) {
                        pass1_1008_3e76(CONCAT22(param_1,&local_24),
                                        local_2a,local_2a._2_2_ - 0x2,
                                        local_26 - 0x2);
                        puVar2 = &local_24;
                        pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                        CONCAT22(param_1,puVar2),lStack30);
                        if (puVar2 != 0x0) {
                          pass1_1008_3e76(CONCAT22(param_1,&local_24),
                                          local_2a,local_2a._2_2_ + 0x2,
                                          local_26 - 0x2);
                          puVar2 = &local_24;
                          pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                          CONCAT22(param_1,puVar2),lStack30);
                          if (puVar2 != 0x0) {
                            pass1_1008_3e76(CONCAT22(param_1,&local_24),
                                            local_2a,local_2a._2_2_ - 0x2,
                                            local_26 + 0x2);
                            puVar2 = &local_24;
                            pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                            CONCAT22(param_1,puVar2),lStack30);
                            if (puVar2 != 0x0) {
                              pass1_1008_3e76(CONCAT22(param_1,&local_24),
                                              local_2a,local_2a._2_2_ + 0x2,
                                              local_26 + 0x2);
                              puVar2 = &local_24;
                              pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                              CONCAT22(param_1,puVar2),lStack30)
                              ;
                              if (puVar2 != 0x0) {
                                pass1_1008_3e76(CONCAT22(param_1,&local_24),
                                                local_2a,local_2a._2_2_ - 0x1,
                                                local_26 + 0x2);
                                puVar2 = &local_24;
                                pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                                CONCAT22(param_1,puVar2),
                                                lStack30);
                                if (puVar2 != 0x0) {
                                  pass1_1008_3e76(CONCAT22(param_1,&local_24),
                                                  local_2a,local_2a._2_2_ - 0x1,
                                                  local_26 + 0x3);
                                  puVar2 = &local_24;
                                  pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                                  CONCAT22(param_1,puVar2),
                                                  lStack30);
                                  if (puVar2 != 0x0) {
                                    iStack46 = 0x3;
                                    while( true ) {
                                      if (0x9 < iStack46) {
                                        return;
                                      }
                                      pass1_1008_3e76(
                                                      CONCAT22(param_1,&local_24),0x0,
                                                      local_2a._2_2_ - iStack46,local_26);
                                      puVar2 = &local_24;
                                      pass1_1020_afc4(param_1,puVar6,uVar8,uVar9,
                                                      CONCAT22(param_1,puVar2),
                                                      lStack30);
                                      if (puVar2 == 0x0) break;
                                      iStack46 += 0x1;
                                    }
                                    return;
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      return;
    }
    uVar3 = (puVar1 + 0x10);
    puVar10 = param_3;
    uVar7 = param_4;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,(uVar3 >> 0x10));
    puVar1 = local_4;
    pass1_1030_bcbc(param_1,puVar1,CONCAT22(uVar3,param_1),
                    CONCAT22(puVar10,uVar5),(puVar10 >> 0x10),uVar7);
    if (puVar1 < 0x0) break;
    uVar4 = uVar5;
    if (puVar1 < 0x65) {
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_b872(param_1: u16,param_2: u8,param_3: u32,param_4: u32)
{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let puVar4: u32;
  let puVar5: *mut u8;
  let puVar6: u32;
  let puVar7: *mut u16;
  let uVar8: u16;
  let local_136: [u8;124];
  let local_12: u32;
  let local_c: i16;
  let local_a: i16;
  let local_8: u32;
  let uStack4: u16;
  
  uVar8 = (param_4 >> 0x10);
  puVar6 = pass1_1030_5b5c(param_4,uVar8);
  local_8 = *puVar6;
  uStack4 = (puVar6 + 0x4);
  pass1_1008_3e94(CONCAT22(param_1,&local_8),
                  CONCAT22(param_1,&local_c),
                  CONCAT22(param_1,&local_a));
  uVar1 = local_a - 0xa;
  pass1_1008_612e(0xa,uVar1,uVar1);
  uVar2 = local_c - 0xa;
  pass1_1008_612e(0xa,uVar2,uVar2);
  puVar7 = pass1_1008_3e54(CONCAT22(param_1,&local_12),0x0,uVar2,uVar1);
  uVar1 = (puVar7 >> 0x10);
  while( true ) {
    puVar4 = &local_12;
    pass1_1020_b482(param_1,param_3,CONCAT22(param_1,puVar4),param_4);
    if (puVar4 != 0x0) break;
    uVar2 = local_a - 0xa;
    pass1_1008_612e(0xa,uVar2,uVar2);
    uVar3 = local_c - 0xa;
    pass1_1008_612e(0xa,uVar3,uVar3);
    pass1_1008_3e76(CONCAT22(param_1,&local_12),0x0,uVar3,uVar2);
  }
  struct_op_1028_8888(param_1,param_2,(astruct_100 *)CONCAT22(param_1,local_136),0x0,0xa,
                      &local_12,param_1,0x8000002,0x0,(param_4 + 0x4));
  puVar5 = local_136;
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_1,puVar5));
  pass1_1020_b97e(param_1,puVar5,uVar1,param_3,(param_3 >> 0x10),0x1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_b97e(param_1: u16,param_2: i16,param_3: u16,param_4: u16,param_5: u16,
               param_6: i16)

{
  let uVar1: u32;
  let local_e: i16;
  let local_c: u16;
  let uStack10: i16;
  let uStack8: u16;
  let uStack6: u32;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x2,0x400);
  _PTR_LOOP_1050_4e70 = CONCAT22(param_3,param_2);
  uVar1 = (param_2 + 0x10);
  uStack6 = uVar1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
  iStack10 = uVar1;
  uStack8 = param_3;
  pass1_1008_3f62(&ctx.PTR_LOOP_1048_4230,CONCAT22(param_3,iStack10 + 0xc)
                 );
  pass1_1008_3e94(&ctx.PTR_LOOP_1048_4230,CONCAT22(param_1,&local_e),
                  CONCAT22(param_1,&local_c));
  if (param_6 == 0x0) {
    pass1_1008_3e76(&ctx.PTR_LOOP_1048_4230,0x0,local_e + 0x1,local_c - 0x1);
    pass1_1008_3e94(&ctx.PTR_LOOP_1048_4230,CONCAT22(param_1,&local_e),
                    CONCAT22(param_1,&local_c));
  }
  pass1_1008_3e76(0x10484236,0x1,local_e - 0x2,local_c);
  return;
}



fn pass1_1020_ba2b(void)
{
  init_globals_1020_96d4();
  pass1_1020_a426();
  return;
}



fn pass1_1020_ba3e(long *param_1,param_2: u16,param_3: i16,param_4: u16,param_5: u16)
{
  astruct_172 *iVar1;
  let uVar1: u16;
  let unaff_SS: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_172 *)param_1;
  *param_1 = 0x0;
  iVar1.field_0x4 = 0x0;
  iVar1.field_0x6 = param_3;
  iVar1.field_0x8 = param_2;
  if (iVar1.field_0x6 == 0x0) {
    iVar1.field_0x6 = 0x5;
  }
  pass1_1020_bcc4(param_1,param_4,unaff_SS);
  return;
}


fn pass1_1020_ba94(long *param_1)
{
  let puVar1: *mut u16;
  let uStack8: u16;
  
  if (*param_1 == 0x0) {
    return;
  }
  uStack8 = 0x0;
  while( true ) {
    puVar1 = (param_1 + 0x4);
    if (*puVar1 < uStack8 || *puVar1 == uStack8) break;
    uStack8 += 0x1;
  }
  return;
}



fn pass1_1020_bae6(param_1: u16,param_2: u32,param_3: u16,param_4: u16,param_5: u16) -> u32

{
  let puStack6: *mut u16;
  
  pass1_1020_bc92(CONCAT22(param_2,param_1),(param_2 >> 0x10),param_5
                 );
  puStack6 = CONCAT22(param_4,param_3);
  if ((param_4 | param_3) != 0x0) {
    return CONCAT22((param_3 + 0x2),*puStack6);
  }
  return 0x0;
}



fn pass1_1020_bb16(param_1: *mut u32,param_2: *mut u32,param_3: *mut u16,param_4: u16)
{
  if ((param_1 + 0x4) < param_4) {
    *param_3 = 0x0;
    *param_2 = 0x0;
    return;
  }
  *param_3 = (param_4 * 0x6 + *param_1 + 0x4);
  *param_2 = (*param_1 + param_4 * 0x6);
  return;
}



void 
pass1_1020_bb70(long *param_1,param_2: u16,param_3: u32,param_4: u16,
               param_5: u16,param_6: u16)

{
  pass1_1020_bba4(param_1,0x1,param_2,param_3,(param_3 >> 0x10),
                  param_4,param_6);
  return;
}



void 
pass1_1020_bb8a(long *param_1,param_2: u16,param_3: u32,param_4: u16,
               param_5: u16)

{
  pass1_1020_bba4(param_1,0x0,param_2,param_3,(param_3 >> 0x10),
                  param_4,param_5);
  return;
}



bool 
pass1_1020_bba4(long *param_1,param_2: i16,param_3: u16,param_4: i16,param_5: u16,
               param_6: u16,param_7: u16)

{
  let in_AX: *mut u16;
  let in_DX: u16;
  let uVar1: u16;
  let uVar2: u16;
  let bVar3: bool;
  let puStack6: *mut u16;
  
  pass1_1020_bc92(param_1,param_5,param_7);
  puStack6 = CONCAT22(in_DX,in_AX);
  uVar1 = in_DX | in_AX;
  if (uVar1 == 0x0) {
    pass1_1020_bc92(param_1,0x0,param_7);
    uVar2 = uVar1 | in_AX;
    if (uVar2 == 0x0) {
      pass1_1020_bcc4(param_1,param_6,param_7);
      pass1_1020_bc92(param_1,0x0,param_7);
      if ((uVar2 | in_AX) == 0x0) {
        return 0x0;
      }
      in_AX[0x2] = param_5;
      uVar1 = uVar2;
    }
    else {
      in_AX[0x2] = param_5;
    }
    if (param_2 != 0x0) {
      uVar2 = *in_AX;
      bVar3 = CARRY2(uVar2,param_3);
      param_3 = uVar2 + param_3;
      param_4 = in_AX[0x1] + param_4 + bVar3;
    }
    *in_AX = param_3;
    in_AX[0x1] = param_4;
    pass1_1020_bc72(param_1,param_6,param_7);
  }
  else {
    if (param_2 != 0x0) {
      bVar3 = CARRY2(*puStack6,param_3);
      param_3 = *puStack6 + param_3;
      param_4 = in_AX[0x1] + param_4 + bVar3;
    }
    *in_AX = param_3;
    in_AX[0x1] = param_4;
  }
  return 0x1;
}



fn pass1_1020_bc72(param_1: *mut u16,param_2: u16,param_3: u16)
{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  uVar1 = (param_1 + 0x2);
  pass1_1000_4aea(*param_1,uVar1,(uVar1 >> 0x10),0x6,0xbd6c,
                  &stack0xfffe,param_2,uVar2,0x1000,param_3);
  return;
}



fn pass1_1020_bc92(param_1: *mut u16,param_2: u16,param_3: u16)
{
  let uVar1: u32;
  let local_c: [u8;4];
  let uStack8: u16;
  
  uStack8 = param_2;
  uVar1 = (param_1 + 0x2);
  pass1_1000_49c6(local_c,param_3,*param_1,uVar1,
                  (uVar1 >> 0x10),0x6,0xbd6c,&stack0xfffe);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_bcc4(long *param_1,param_2: u16,param_3: u16)
{
  let puVar1: *mut u16;
  let iVar2: i16;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let lVar6: i32;
  let lStack12: i32;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if ((iVar4 + 0x4) == 0x0) {
    ctx.PTR_LOOP_1050_5f2e = 0x0;
    iVar2 = (iVar4 + 0x6);
  }
  else {
    uVar3 = (iVar4 + 0x4);
    puVar1 = (iVar4 + 0x8);
    iVar2 = uVar3 + *puVar1;
    ctx.PTR_LOOP_1050_5f2e = CARRY2(uVar3,*puVar1);
  }
  if ((false) || (ctx.PTR_LOOP_1050_5f2e == 0x0)) {
    if (*param_1 == 0x0) {
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e,0x1000);
      }
      else {
      }
      uVar3 = fn_ptr_op_1000_1708(iVar2 * 0x6,0x0,0x1,PTR_LOOP_1050_5f2c,
                                  ctx.PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
      lVar6 = pass1_1000_0ed4(0x1000,param_3,0x1,iVar2 * 0x6,0x0,*param_1,
                              (*param_1 >> 0x10));
      ctx.PTR_LOOP_1050_5f2e = (lVar6 >> 0x10);
      uVar3 = lVar6;
    }
    lStack12 = CONCAT22(ctx.PTR_LOOP_1050_5f2e,uVar3);
    if ((ctx.PTR_LOOP_1050_5f2e | uVar3) != 0x0) {
      (iVar4 + 0x4) = iVar2;
      *param_1 = lStack12;
      pass1_1020_bc72((param_1 & 0xffff | uVar5 << 0x10),param_2,
                      param_3);
    }
  }
  return;
}



i16  pass1_1020_bd6c(param_1: u32,param_2: u32)

{
  return (param_1 + 0x4) - (param_2 + 0x4);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_bd80(param_1: u16) -> u16

{
  char *pcVar1;
  let uStack6: u16;
  
  switch(param_1) {
  case 0x1:
  case 0x6:
    break;
  case 0x2:
    break;
  case 0x3:
  case 0x7:
    break;
  case 0x4:
  case 0x8:
    break;
  case 0x5:
  case 0x9:
    break;
  case 0xa:
    break;
  case 0xb:
  case 0x37:
    break;
  case 0xc:
  case 0x35:
  case 0x36:
    break;
  case 0xd:
    break;
  case 0xe:
    break;
  case 0xf:
    break;
  case 0x10:
    break;
  case 0x11:
    break;
  case 0x12:
    break;
  case 0x13:
  case 0x14:
  case 0x15:
    break;
  case 0x16:
  case 0x19:
    break;
  case 0x17:
  case 0x1a:
    break;
  case 0x18:
    break;
  case 0x1b:
  case 0x1c:
  case 0x1d:
    break;
  case 0x1e:
  case 0x1f:
  case 0x20:
    break;
  case 0x21:
    break;
  case 0x22:
  case 0x23:
  case 0x24:
    break;
  case 0x25:
  case 0x26:
  case 0x27:
    break;
  case 0x28:
  case 0x29:
    break;
  case 0x2a:
  case 0x2b:
    break;
  case 0x2c:
    break;
  case 0x2d:
  case 0x2e:
    break;
  case 0x2f:
  case 0x30:
    break;
  case 0x31:
  case 0x32:
    break;
  case 0x33:
  case 0x34:
    break;
  case 0x38:
  case 0x39:
    break;
  case 0x3a:
  case 0x3b:
    break;
  case 0x3c:
  case 0x3d:
    break;
  case 0x3e:
    break;
  case 0x3f:
    break;
  case 0x40:
    break;
  case 0x41:
    break;
  case 0x42:
  case 0x46:
  case 0x6b:
    break;
  case 0x43:
    uStack6 = s_bidLRoadConst_1050_4e7a;
    return uStack6;
  case 0x44:
    uStack6 = s_bidRRoadConst_1050_4e88;
    return uStack6;
  case 0x45:
    uStack6 = s_bidXRoadConst_1050_4e96;
    return uStack6;
  case 0x47:
    break;
  case 0x48:
  case 0x49:
  case 0x4a:
  case 0x70:
  case 0x71:
  case 0x72:
    break;
  case 0x4b:
    break;
  case 0x4c:
    break;
  case 0x4d:
    break;
  case 0x4e:
    break;
  case 0x4f:
  case 0x50:
  case 0x51:
    break;
  case 0x52:
  case 0x53:
    break;
  case 0x54:
  case 0x55:
  case 0x56:
    break;
  case 0x57:
  case 0x58:
  case 0x59:
    break;
  case 0x5a:
    break;
  case 0x5b:
  case 0x5c:
    break;
  case 0x5d:
  case 0x5e:
  case 0x5f:
    break;
  case 0x60:
  case 0x61:
    break;
  case 0x62:
  case 0x63:
    break;
  case 0x64:
    break;
  case 0x65:
    break;
  case 0x66:
  case 0x67:
    break;
  case 0x68:
  case 0x69:
    break;
  case 0x6a:
    break;
  case 0x6c:
  case 0x6d:
    break;
  case 0x6e:
    break;
  case 0x6f:
    break;
  case 0x73:
  case 0x77:
    break;
  case 0x74:
  case 0x78:
  case 0x79:
    break;
  case 0x75:
    break;
  case 0x76:
    break;
  case 0x7a:
    break;
  case 0x7b:
    break;
  case 0x7c:
    break;
  case 0x7d:
    break;
  case 0x7e:
    break;
  case 0x7f:
    break;
  case 0x80:
    break;
  case 0x81:
    break;
  case 0x82:
    break;
  case 0x83:
    break;
  case 0x84:
    break;
  case 0x85:
    break;
  case 0x86:
    break;
  case 0x87:
    break;
  case 0x88:
    break;
  case 0x89:
  }
  pcVar1 = load_string_1010_847e
                     (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  return pcVar1;
}



fn string_1020_c0ca(param_1: u16)
{
  string_1020_c0d8(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn string_1020_c0d8(param_1: u16) -> *mut u8

{
  char *pcVar1;
  
  if (true) {
    switch(param_1) {
    case 0x1:
      break;
    case 0x2:
      break;
    case 0x3:
      break;
    case 0x4:
      break;
    case 0x5:
      break;
    case 0x6:
      break;
    case 0x7:
      break;
    case 0x8:
      break;
    case 0x9:
      break;
    case 0xa:
      break;
    case 0xb:
      break;
    case 0xc:
      break;
    case 0xd:
      break;
    case 0xe:
      break;
    case 0xf:
      break;
    case 0x10:
      break;
    case 0x11:
      break;
    case 0x12:
      break;
    case 0x13:
      break;
    case 0x14:
      break;
    case 0x15:
      break;
    case 0x16:
      break;
    case 0x17:
      break;
    case 0x18:
      break;
    case 0x19:
      break;
    case 0x1a:
      break;
    case 0x1b:
      break;
    case 0x1c:
      break;
    case 0x1d:
      break;
    case 0x1e:
      break;
    case 0x1f:
      break;
    case 0x21:
      break;
    case 0x23:
      break;
    case 0x24:
    }
  }
  pcVar1 = load_string_1010_847e
                     (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  return pcVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn string_op_1020_c222(param_1: u16) -> *mut u8

{
  char *pcVar1;
  
  switch(param_1) {
  case 0x1:
    break;
  case 0x2:
    break;
  case 0x3:
    break;
  case 0x4:
    break;
  case 0x5:
    break;
  case 0x6:
    break;
  case 0x7:
    break;
  case 0x8:
    break;
  case 0x9:
    break;
  case 0xa:
    break;
  case 0xb:
    break;
  case 0xc:
    break;
  case 0xd:
    break;
  case 0xe:
    break;
  case 0xf:
    break;
  case 0x10:
    break;
  case 0x11:
    break;
  case 0x12:
    break;
  case 0x13:
    break;
  case 0x14:
  }
  pcVar1 = load_string_1010_847e
                     (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  return pcVar1;
}


fn pass1_1020_c3ae(void) -> u16

{
  return 0x1;
}


fn pass1_1020_c42e(param_1: i16) -> u16

{
  let uVar1: u16;
  
  if (param_1 == 0xf) {
    uVar1 = 0x1;
  }
  else {
    uVar1 = 0x3;
  }
  return uVar1;
}


fn pass1_1020_c47a(param_1: *mut u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0xc834;
  (param_1 + 0x2) = 0x1020;
  fn_ptr_1000_17ce((param_1 + 0x18),0x1000);
  pass1_1030_1d28(param_1);
  return;
}



void 
pass1_1020_c4a8(param_1: u32,param_2: *mut u16,param_3: *mut u32,param_4: i16,param_5: u16,
               param_6: u16)

{
  let uVar1: u32;
  let puVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  
  uVar3 = (param_1 >> 0x10);
  if ((param_1 + 0x1c) != 0x0) {
    pass1_1020_c6a4(param_1 & 0xffff | uVar3 << 0x10,param_5,param_6);
  }
  uVar1 = (param_1 + 0x18);
  uVar4 = (uVar1 >> 0x10);
  puVar2 = (uVar1 + param_4 * 0x6);
  *param_3 = *puVar2;
  *param_2 = (puVar2 + 0x1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_c4f4(param_1: u32,param_2: u16,param_3: u16,param_4: u32,
               astruct_361 *param_5,param_6: u16)

{
  astruct_361 *paVar1;
  let uVar2: u16;
  let uVar3: u16;
  
  pass1_1020_c6de(param_1,param_4);
  uVar3 = param_6 | param_5;
  if (uVar3 != 0x0) {
    paVar1 = param_5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4,(param_4 >> 0x10));
    uVar2 = pass1_1030_6fa0(CONCAT22(uVar3,paVar1));
    param_5.field_0x4 = (uVar2 * 0x2 + 0x4ea4);
  }
  return;
}



fn pass1_1020_c538(param_1: u32) -> u32

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x12),
                  (param_1 + 0x10));
}



fn pass1_1020_c54a(param_1: u32,param_2: i16,param_3: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x1c) != 0x0) {
    pass1_1020_c6a4(param_1 & 0xffff | uVar1 << 0x10,param_2,param_3);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_c5b8(param_1: u16,param_2: u16,param_3: i16,param_4: u16)
{
  long *plVar1;
  let uVar2: u32;
  code **ppcVar3;
  let puVar4: u32;
  let uVar5: u16;
  let extraout_DX: u16;
  let uVar6: u16;
  let iVar7: i16;
  let uVar8: u16;
  
  uVar2 = (param_3 + 0xa);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,(uVar2 >> 0x10));
  uVar5 = pass1_1030_6fa0(CONCAT22(param_2,param_1));
  (param_3 + -0x6) = uVar5;
  pass1_1020_c6de((param_3 + 0x6),0x0);
  (param_3 + -0xa) = uVar5;
  (param_3 + -0x8) = param_2;
  if ((param_2 | uVar5) == 0x0) {
    ppcVar3 = ((param_3 + 0x6) + 0x20);
    (**ppcVar3)();
    uVar6 = extraout_DX;
    pass1_1020_c6de((param_3 + 0x6),0x0);
    (param_3 + -0xa) = uVar5;
    (param_3 + -0x8) = uVar6;
    if ((uVar6 | uVar5) == 0x0) {
      return;
    }
  }
  uVar2 = (param_3 + 0x6);
  uVar8 = (uVar2 >> 0x10);
  iVar7 = uVar2;
  (iVar7 + 0x1c) = 0x1;
  plVar1 = (long *)(iVar7 + 0x8);
  *plVar1 = *plVar1 + 0x1;
  puVar4 = (param_3 + -0xa);
  *puVar4 = (param_3 + 0xa);
  (puVar4 + 0x4) =
       ((param_3 + -0x6) * 0x2 + 0x4ea4);
  return;
}



fn pass1_1020_c644(param_1: *mut u32,param_2: u16,param_3: u32)
{
  long *plVar1;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let iVar5: i16;
  let uVar6: u16;
  let puStack6: u32;
  
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  if ((iVar5 + 0x18) == 0x0) {
    ppcVar3 = (*param_1 + 0x20);
    (**ppcVar3)();
  }
  iVar4 = (iVar5 + 0x8) * 0x6 + (iVar5 + 0x18);
  uVar2 = (iVar5 + 0x1a);
  puStack6 = CONCAT22(uVar2,iVar4);
  plVar1 = (long *)(iVar5 + 0x8);
  *plVar1 = *plVar1 + 0x1;
  *puStack6 = param_3;
  (iVar4 + 0x4) = param_2;
  return;
}



fn pass1_1020_c694(param_1: u32,param_2: i16,param_3: u16)
{
  pass1_1020_c6a4(param_1,param_2,param_3);
  return;
}



fn pass1_1020_c6a4(param_1: u32,param_2: i16,param_3: u16)
{
  let lVar1: i32;
  astruct_359 *iVar2;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_359 *)param_1;
  if ((iVar2.field_0x18 != 0x0) && (iVar2.field_0x8 != 0x0)) {
    lVar1 = iVar2.field_0x18;
    pass1_1000_4aea(lVar1,(lVar1 >> 0x10),iVar2.field_0x10,0x6,
                    0xc7fa,&stack0xfffe,param_2,uVar2,0x1000,param_3);
    iVar2.field_0x1c = 0x0;
  }
  return;
}



fn pass1_1020_c6de(param_1: u32,param_2: i32)
{
  let puVar1: u32;
  let uVar2: u32;
  astruct_360 *iVar3;
  let unaff_DI: i16;
  let uVar3: u16;
  let unaff_SS: u16;
  let uStack6: u32;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_360 *)param_1;
  if (iVar3.field_0x1c != 0x0) {
    pass1_1020_c6a4(param_1 & 0xffff | uVar3 << 0x10,unaff_DI,unaff_SS);
  }
  uStack6 = 0x0;
  while( true ) {
    puVar1 = &iVar3.field_0x10;
    if (*puVar1 < uStack6 || *puVar1 == uStack6) {
      return;
    }
    uVar2 = iVar3.field_0x18;
    if ((uVar2 + uStack6 * 0x6) == param_2) break;
    uStack6 += 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_c73a(param_1: u32,param_2: u16)
{
  let puVar1: *mut u16;
  let uVar2: u16;
  let uVar3: u32;
  let uVar4: u16;
  let iVar5: i16;
  let uVar6: u16;
  let lVar7: i32;
  let uStack10: u32;
  let uStack6: u32;
  
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  if ((iVar5 + 0x10) == 0x0) {
    uVar4 = (iVar5 + 0xc);
    ctx.PTR_LOOP_1050_5f2e = (iVar5 + 0xe);
  }
  else {
    uVar2 = (iVar5 + 0x10);
    puVar1 = (iVar5 + 0x14);
    uVar4 = uVar2 + *puVar1;
    ctx.PTR_LOOP_1050_5f2e =
         
         ((iVar5 + 0x12) + (iVar5 + 0x16) + CARRY2(uVar2,*puVar1));
  }
  uStack6 = CONCAT22(ctx.PTR_LOOP_1050_5f2e,uVar4);
  if ((iVar5 + 0x18) == 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
    }
    uVar4 = fn_ptr_op_1000_1708(uVar4 * 0x6,0x0,0x1,PTR_LOOP_1050_5f2c,
                                ctx.PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
    uVar3 = (iVar5 + 0x18);
    lVar7 = pass1_1000_0ed4(0x1000,param_2,0x1,uVar4 * 0x6,
                            (ctx.PTR_LOOP_1050_5f2e * 0x3 + CARRY2(uVar4,uVar4) +
                            CARRY2(uVar4 * 0x2,uVar4)) * 0x2 +
                            CARRY2(uVar4 * 0x3,uVar4 * 0x3),uVar3,
                            (uVar3 >> 0x10));
    ctx.PTR_LOOP_1050_5f2e = (lVar7 >> 0x10);
    uVar4 = lVar7;
  }
  uStack10 = CONCAT22(ctx.PTR_LOOP_1050_5f2e,uVar4);
  if ((ctx.PTR_LOOP_1050_5f2e | uVar4) != 0x0) {
    (iVar5 + 0x10) = uStack6;
    (iVar5 + 0x18) = uStack10;
  }
  (iVar5 + 0x1c) = 0x1;
  return;
}



i16  pass1_1020_c7fa(param_1: u32,param_2: u32)

{
  return (param_1 + 0x4) - (param_2 + 0x4);
}



astruct_18 *  pass1_1020_c80e(astruct_18 *param_1,param_2: u8)

{
  pass1_1020_c47a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1020_c860(param_1: u32) -> u32

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x6),(param_1 + 0x4))
  ;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_c872(param_1: u32,param_2: u32,param_3: u32)
{
  let puVar1: *mut u16;
  let puVar2: u32;
  let piVar3: *mut i16;
  astruct_98 *uVar4;
  let uVar6: u16;
  let iVar7: i16;
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  let bVar12: bool;
  let uStack14: u32;
  let uStack10: u32;
  astruct_99 *puStack6;
  astruct_99 *uVar5;
  
  puStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_4fb8);
  uVar6 = (puStack6 >> 0x10);
  uVar5 = puStack6;
  if ((uVar6 | uVar5) == 0x0) {
    puStack6 = 0x0;
  }
  else {
    puStack6.field_0x0 = 0x389a;
    uVar5.field_0x2 = 0x1008;
    uVar5.field_0x4 = 0x0;
    uVar5.field_0x8 = 0x0;
    puStack6.field_0x0 = 0x5bc0;
    uVar5.field_0x2 = 0x1008;
    uVar5.field_0xe = 0x0;
    uVar5.field_0xc = 0x0;
    puStack6.field_0x0 = 0xc9e6;
    uVar5.field_0x2 = 0x1020;
  }
  if (puStack6 == 0x0) {
    return;
  }
  uVar9 = (puStack6 >> 0x10);
  iVar7 = puStack6;
  (iVar7 + 0x8) = param_3;
  (iVar7 + 0xc) = param_2;
  uVar10 = (param_1 >> 0x10);
  iVar8 = param_1;
  uStack14 = (iVar8 + 0x4);
  uVar11 = (iVar8 + 0x6);
  if ((iVar8 + 0x8) == 0x0) {
//LAB_1020_c92d:
    (iVar7 + 0x4) = (iVar8 + 0x4);
  }
  else {
    puVar1 = (uStack14 + 0xe);
    bVar12 = *puVar1 < param_2._2_2_;
    if ((bVar12 || *puVar1 == param_2._2_2_) &&
       ((bVar12 ||
        (puVar1 = (uStack14 + 0xc),
        *puVar1 < param_2 || *puVar1 == param_2)))) goto LAB_1020_c92d;
    bVar12 = false;
    while( true ) {
      if (uStack14 == 0x0) break;
      uVar11 = (uStack14 >> 0x10);
      puVar2 = (uStack14 + 0xc);
      if (*puVar2 < param_2 || *puVar2 == param_2) {
        bVar12 = true;
        (iVar7 + 0x4) = uStack14;
        *(astruct_99 **)(uStack10 + 0x4) = puStack6;
        break;
      }
      uStack10 = uStack14;
      uStack14 = (uStack14 + 0x4);
    }
    param_1 = uStack10;
    if (bVar12) goto LAB_1020_c9ab;
  }
  uVar11 = (param_1 >> 0x10);
  (param_1 + 0x4) = iVar7;
  (param_1 + 0x6) = uVar9;
//LAB_1020_c9ab:
  piVar3 = (iVar8 + 0x8);
  *piVar3 = *piVar3 + 0x1;
  return;
}



fn pass1_1020_c9ba(param_1: *mut u16,param_2: u8) -> u16

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x389a;
  (param_1)[0x1] = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a(param_1,uVar1,0x1000);
  }
  return param_1;
}



u16 * 
pass1_1020_ca0c(astruct_179 *param_1,param_2: u16,param_3: i16,param_4: u32,
               param_5: u16)

{
  pass1_1028_0982(param_1,param_2,param_3,param_4,param_5);
  CONCAT22(param_2,param_1) = 0xcc7c;
  param_1.field_0x2 = 0x1020;
  return CONCAT22(param_2,param_1);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_ca36(param_1: i16,param_2: u16,param_3: u16,param_4: i16,param_5: u16)
{
  let puVar1: *mut u8
  let uVar2: u32;
  let puVar3: *mut u16;
  
  pass1_1028_09b8(CONCAT22(param_2,param_1));
  uVar2 = pass1_1028_b4f2(CONCAT22(param_2,param_1));
  puVar1 = (uVar2 >> 0x10);
  if ((uVar2 + 0x200) != 0x8000002) {
    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_5,puVar1,param_4);
    pass1_1010_988c(puVar3,(param_1 + 0xc));
  }
  return;
}



fn pass1_1020_ca82(param_1: *mut u32,param_2: u16,param_3: u16,param_4: u16)
{
  let puVar1: *mut u8
  let uVar2: u16;
  let uVar3: u32;
  
  pass1_1028_be9e(param_1,param_2,param_3,&USHORT_1050_1028,param_4);
  uVar3 = pass1_1028_b4f2(param_1);
  puVar1 = (uVar3 >> 0x10);
  if ((uVar3 + 0x200) != 0x8000002) {
    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 0x5) {
      pass1_1020_cac2(param_1 & 0xffff | uVar2 << 0x10,puVar1,param_2,
                      param_3,param_4);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_cac2(param_1: u32,uchar *param_2,param_3: u16,param_4: u16,param_5: u16)
{
  let piVar1: *mut i16;
  code **ppcVar2;
  let puVar3: *mut u8;
  let puVar4: *mut u8;
  let uVar5: u16;
  let iVar6: i16;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar7: u16;
  let puVar8: *mut u16;
  let iStack52: i16;
  let puStack36: *mut u8;
  let local_1c: [u8;4];
  let uStack24: u32;
  let puStack20: u32;
  let puStack16: u32;
  let puStack12: *mut u16;
  let puStack8: *mut u8;
  let uStack6: u16;
  let puStack4: *mut u8
  
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_5,param_2,param_4);
  puStack4 = (puVar8 >> 0x10);
  uStack6 = SUB42(puVar8,0x0);
  puStack8 = ctx.PTR_LOOP_1050_13ae;
  if (ctx.PTR_LOOP_1050_13ae == (&ctx.PTR_LOOP_1050_0000 + 0x1)) {
    puStack8 = &ctx.PTR_LOOP_1050_0002;
  }
  puStack12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_5,puStack4,param_4);
  uVar7 = (puStack12 >> 0x10);
  puStack16 = (puStack12 + 0xa);
  puStack20 = (puStack12 + 0xe);
  pass1_1008_5784(CONCAT22(param_5,local_1c),puStack16);
  do {
    do {
      while( true ) {
        do {
          puVar3 = local_1c;
          pass1_1008_5b12(puVar3,param_5);
          if ((extraout_DX | puVar3) == 0x0) {
            return;
          }
          iVar6 = (puVar3 + 0x4);
        } while ((iVar6 < 0x12) || (SBORROW2(iVar6,0x12)));
        if (iVar6 != 0x13 && 0x0 < iVar6 + -0x12) break;
        iStack52 = 0x0;
        if (puStack8 == (&ctx.PTR_LOOP_1050_0002 + 0x1)) {
          iStack52 = (puVar3 + 0x6) / 0x2;
        }
        else {
          if (puStack8 == &DAT_1050_0004) {
            iVar6 = (puVar3 + 0x6) * 0x3;
            iStack52 = (iVar6 + (iVar6 >> 0xf & 0x3)) >> 0x2;
          }
        }
        piVar1 = (puVar3 + 0x6);
        *piVar1 = *piVar1 - iStack52;
        uVar7 = (puStack16 >> 0x10);
        (puStack16 + 0xa) = 0x0;
        ppcVar2 = (*puStack16 + 0xc);
        (**ppcVar2)(0x1008,puStack16,uVar7,puVar3,extraout_DX);
        (puStack16 + 0xa) = 0x1;
        uStack24 = 0x0;
        ppcVar2 = (*puStack20 + 0x4);
        (**ppcVar2)(0x1008,puStack20,(puStack20 >> 0x10),puVar3,
                    extraout_DX);
      }
    } while (iVar6 != 0x81);
    puStack36 = 0x0;
    if (puStack8 == &ctx.PTR_LOOP_1050_0002) {
      iVar6 = (puVar3 + 0x6);
//LAB_1020_cba7:
      puVar4 = ((iVar6 + (iVar6 >> 0xf & 0x3)) >> 0x2);
      puStack36 = puVar4;
    }
    else {
      if (puStack8 == (&ctx.PTR_LOOP_1050_0002 + 0x1)) {
        puVar4 = ((puVar3 + 0x6) / 0x2);
        puStack36 = puVar4;
      }
      else {
        puVar4 = puStack8 + -0x4;
        if (puVar4 == 0x0) {
          iVar6 = (puVar3 + 0x6) * 0x3;
          goto LAB_1020_cba7;
        }
      }
    }
    pass1_1028_b58e(param_1);
    uVar5 = (puVar3 + 0x6) - puStack36;
    pass1_1030_7ddc(CONCAT13((extraout_DX_00 >> 0x8),
                             CONCAT12(extraout_DX_00,puVar4)),(long)uVar5,0x1e,
                    uVar5,(uVar5 >> 0xf),param_3,param_4,param_5);
    ppcVar2 = (*puStack16 + 0xc);
    (**ppcVar2)(0x1030,puStack16,(puStack16 >> 0x10),puVar3,extraout_DX)
    ;
    uStack24 = 0x0;
  } while( true );
}



astruct_18 *  pass1_1020_cc56(astruct_18 *param_1,param_2: u8)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1020_cd06(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u16

{
  pass1_1028_b39e(CONCAT22(param_2,param_1),param_3,param_4,param_5);
  CONCAT22(param_2,param_1) = 0xcd7e;
  (param_1 + 0x2) = 0x1020;
  return CONCAT22(param_2,param_1);
}



fn pass1_1020_cd30(param_1: u32) -> u16

{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((((iVar1 + 0x12) == 0x6) || ((iVar1 + 0x12) == 0x5)) &&
     (((iVar1 + 0x1a) & 0x2) != 0x0)) {
    return 0x1;
  }
  return 0x0;
}



astruct_18 *  pass1_1020_cd58(astruct_18 *param_1,param_2: u8)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


u16 * 
pass1_1020_ce08(astruct_179 *param_1,param_2: u16,param_3: i16,param_4: u32,
               param_5: u16)

{
  pass1_1028_0982(param_1,param_2,param_3,param_4,param_5);
  CONCAT22(param_2,param_1) = 0xd004;
  param_1.field_0x2 = 0x1020;
  return CONCAT22(param_2,param_1);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_ce32(param_1: i16,param_2: u16,param_3: i16,param_4: u16)
{
  let puVar1: *mut u8
  let uVar2: u32;
  let puVar3: *mut u16;
  astruct_67 *paVar4;
  let uVar5: u16;
  let uVar6: u16;
  let iVar7: i16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let iVar11: i16;
  
  pass1_1028_09b8(CONCAT22(param_2,param_1));
  uVar2 = pass1_1028_b4f2(CONCAT22(param_2,param_1));
  puVar1 = (uVar2 >> 0x10);
  if ((uVar2 + 0x200) != 0x8000002) {
    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_4,puVar1,param_3);
    puVar1 = (puVar3 >> 0x10);
    pass1_1010_988c(puVar3,(param_1 + 0xc));
    uVar10 = 0x0;
    iVar11 = 0x9;
    uVar8 = 0x1;
    uVar9 = 0x0;
    uVar6 = 0x0;
    iVar7 = 0x0;
    uVar5 = 0x0;
    paVar4 = (astruct_67 *)
             mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_4,puVar1,param_3);
    post_win_msg_1008_a0e4
              (paVar4,CONCAT22(uVar6,uVar5),iVar7,uVar8,CONCAT22(uVar10,uVar9),iVar11,
               0x1008,param_4);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_ce9e(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  let puVar1: *mut u8
  let uVar2: u32;
  let puVar3: *mut u16;
  
  pass1_1028_be9e(param_1,param_4,param_2,&USHORT_1050_1028,param_3);
  if ((param_1 + 0x12) == 0x5) {
    pass1_1020_cefc(param_1,param_2,param_3);
    uVar2 = pass1_1028_b4f2(param_1);
    puVar1 = (uVar2 >> 0x10);
    if ((uVar2 + 0x200) != 0x8000002) {
      puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_3,puVar1,param_2);
      pass1_1010_043a(puVar3,(uVar2 + 0x4),0x5,param_3);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_cefc(param_1: u32,param_2: i16,param_3: u16)
{
  let puVar1: *mut u8
  let uVar2: u32;
  let puVar3: *mut u16;
  let uStack12: u16;
  
  uVar2 = pass1_1028_b4f2(param_1);
  puVar1 = (uVar2 >> 0x10);
  if ((uVar2 + 0x200) == 0x8000002) {
    uStack12 = 0x32;
  }
  else {
    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_3,puVar1,param_2);
    uStack12 = pass1_1010_96c2(puVar3);
    if (0x32 < uStack12) {
      uStack12 = 0x32;
    }
    pass1_1010_96a8(puVar3,uStack12);
  }
  pass1_1020_cf6c(param_1,(param_1 >> 0x10),uStack12,uVar2);
  return;
}



fn pass1_1020_cf6c(param_1: u16,param_2: u16,param_3: i16,param_4: u32)
{
  let puVar1: *mut u16;
  let piVar2: *mut i16;
  let uVar3: u16;
  let uVar4: u32;
  let uVar5: u16;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u16;
  let iVar9: i16;
  let uVar10: u16;
  
  uVar10 = (param_4 >> 0x10);
  uVar4 = (param_4 + 0x1f6);
  iVar6 = uVar4;
  uVar5 = (uVar4 >> 0x10);
  uVar7 = param_3 / 0x5;
  uVar8 = uVar7 * -0x4 + param_3;
  puVar1 = (iVar6 + 0x50);
  uVar3 = *puVar1;
  *puVar1 = *puVar1 + uVar8;
  piVar2 = (iVar6 + 0x52);
  *piVar2 = *piVar2 + (uVar8 >> 0xf) + CARRY2(uVar3,uVar8);
  iVar9 = uVar7 >> 0xf;
  puVar1 = (iVar6 + 0x78);
  uVar3 = *puVar1;
  *puVar1 = *puVar1 + uVar7;
  piVar2 = (iVar6 + 0x7a);
  *piVar2 = *piVar2 + iVar9 + CARRY2(uVar3,uVar7);
  puVar1 = (iVar6 + 0xa0);
  uVar3 = *puVar1;
  *puVar1 = *puVar1 + uVar7;
  piVar2 = (iVar6 + 0xa2);
  *piVar2 = *piVar2 + iVar9 + CARRY2(uVar3,uVar7);
  puVar1 = (iVar6 + 0xc8);
  uVar3 = *puVar1;
  *puVar1 = *puVar1 + uVar7;
  piVar2 = (iVar6 + 0xca);
  *piVar2 = *piVar2 + iVar9 + CARRY2(uVar3,uVar7);
  puVar1 = (iVar6 + 0xf0);
  uVar3 = *puVar1;
  *puVar1 = *puVar1 + uVar7;
  piVar2 = (iVar6 + 0xf2);
  *piVar2 = *piVar2 + iVar9 + CARRY2(uVar3,uVar7);
  (param_4 + 0x1fe) = 0x1;
  return;
}



astruct_18 *  pass1_1020_cfde(astruct_18 *param_1,param_2: u8)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1020_d08e(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u16

{
  pass1_1028_b39e(CONCAT22(param_2,param_1),param_3,param_4,param_5);
  CONCAT22(param_2,param_1) = 0xd314;
  (param_1 + 0x2) = 0x1020;
  return CONCAT22(param_2,param_1);
}



fn pass1_1020_d0b8(param_1: *mut u32,param_2: u16,param_3: u16,param_4: u16)
{
  let uVar1: u32;
  let iVar2: i16;
  
  if ((param_1 + 0x12) != 0x6) {
    return;
  }
  uVar1 = pass1_1028_b4f2(param_1);
  iVar2 = uVar1;
  if ((iVar2 + 0x200) != 0x8000002) {
    pass1_1028_cb04(param_1,param_2,param_3,param_4);
    if ((iVar2 == 0x0) || (pass1_1020_d194(param_1,param_3,param_4), iVar2 == 0x0))
    {
      iVar2 = 0x6;
      goto LAB_1020_d10b;
    }
    pass1_1028_c952(param_1,param_2,param_3,param_4);
  }
  iVar2 = 0x5;
//LAB_1020_d10b:
  pass1_1028_bdac(param_1,iVar2,&USHORT_1050_1028);
  return;
}



u16 
pass1_1020_d118(param_1: u32,param_2: *mut u16,param_3: u32,param_4: u32,param_5: i16,
               param_6: u16,param_7: u16)

{
  let BVar1: bool;
  let uVar2: u16;
  let uVar3: u16;
  
  uVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  pass1_1028_c7b6(param_7,param_6,uVar2,uVar3,param_2,param_4);
  if (param_5 == 0x5) {
    pass1_1028_c23e(uVar2,uVar3,param_2,param_3,param_4,0x5,param_6,param_7);
    if (param_5 != 0x0) {
      pass1_1028_c3aa(uVar2,uVar3,param_2,param_3,param_4,param_7);
      if (param_5 != 0x0) {
        BVar1 = pass1_1028_c314(param_7,param_5,param_6,uVar2,uVar3,param_2,
                                param_3,(param_3 >> 0x10),param_4);
        if (BVar1 != 0x0) {
          return 0x1;
        }
      }
    }
  }
  else {
    ctx.PTR_LOOP_1050_50ca = 0x6a8;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_d194(param_1: u32,param_2: i16,param_3: u16)
{
  code **ppcVar1;
  let uVar2: u16;
  let puVar3: *mut u8;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u32;
  let uVar7: u16;
  let extraout_DX: *mut u8
  let puVar8: *mut u8
  let puVar9: *mut u8
  let extraout_DX_00: u16;
  let uVar10: u16;
  let uVar11: u16;
  let uVar12: u32;
  let puVar13: *mut u16;
  let puVar14: u32;
  let uVar15: u8;
  let uVar16: u8;
  let puVar17: *mut u8
  let uVar18: u16;
  let uVar19: u16;
  let uStack42: u32;
  let uStack38: u32;
  let puStack34: u32;
  let local_4: [u8;2];
  
  pass1_1030_bcae(local_4,param_3);
  uVar12 = pass1_1028_b4f2(param_1);
  uVar7 = (uVar12 >> 0x10);
  uVar6 = (uVar12 + 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar6,(uVar6 >> 0x10));
  uVar2 = uVar6;
  pass1_1028_b58e(param_1);
  puVar3 = local_4;
  puVar8 = extraout_DX;
  pass1_1030_bd74(puVar3,param_3,uVar6 & 0xffff | uVar7 << 0x10,
                  CONCAT22(extraout_DX,uVar2),param_3);
  if (puVar3 < 0x0) {
    return;
  }
  if (0x1e < puVar3) {
    uVar4 = 0x87;
    puVar13 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x9,param_3,puVar8,param_2);
    uVar4 = pass1_1010_65d0(param_3,puVar13,uVar4);
    if (uVar4 == 0x0) {
      puVar14 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x15);
      puVar9 = (puVar14 >> 0x10);
      uVar7 = puVar14;
      uVar11 = SUB42(&ctx.PTR_LOOP_1050_1038,0x0);
      pass1_1038_4e78(uVar7,puVar9,uVar12,puVar14);
      puStack34 = CONCAT22(puVar9,uVar7);
      ppcVar1 = (*puStack34 + 0x10);
      uVar10 = uVar7;
      uVar18 = uVar7;
      puVar8 = puVar9;
      (**ppcVar1)(&ctx.PTR_LOOP_1050_1038,uVar7,puVar9);
      uStack38 = CONCAT22(extraout_DX_00,uVar10);
      uStack42 = 0x0;
      uVar10 = extraout_DX_00;
      while( true ) {
        if (uStack38 <= uStack42) {
          if (puStack34 == 0x0) {
            return;
          }
          ppcVar1 = *puStack34;
          (**ppcVar1)(uVar11,uVar7,puVar9,0x1,uVar18,puVar8,puStack34,puStack34);
          return;
        }
        uVar15 = (u8)uVar2;
        uVar16 = (u8)(uVar2 >> 0x8);
        uVar6 = uStack38;
        puVar17 = extraout_DX;
        pass1_1030_1d58(puStack34);
        uVar5 = uVar6;
        puVar3 = local_4;
        uVar11 = 0x1030;
        uVar19 = uVar10;
        pass1_1030_bd74(puVar3,param_3,uVar6 & 0xffff | uVar10 << 0x10,
                        CONCAT22(puVar17,CONCAT11(uVar16,uVar15)),param_3);
        if ((0x0 < puVar3) && (puVar3 < 0x1f)) break;
        uStack42 += 0x1;
      }
      if (puStack34 == 0x0) {
        return;
      }
      ppcVar1 = *puStack34;
      (**ppcVar1)(0x1030,uVar7,puVar9,0x1,uVar18,puVar8,puStack34,puStack34,uVar5,
                  uVar19);
      return;
    }
  }
  return;
}



astruct_18 *  pass1_1020_d2ee(astruct_18 *param_1,param_2: u8)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1020_d3a4(param_1: *mut u16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u16

{
  let uVar1: u16;
  
  pass1_1028_b39e(param_1,param_3,param_4,param_5);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x20) = param_2;
  *param_1 = 0xd53e;
  (param_1 + 0x2) = 0x1020;
  return param_1;
}


fn pass1_1020_d41a(param_1: u32,param_2: u32,bool param_3,uchar *param_4,param_5: u16) -> bool

{
  let BVar1: bool;
  let local_4: u16;
  
  file_1028_b81a(param_1,param_2,param_3,param_5,param_4);
  if (param_3 != 0x0) {
    BVar1 = read_file_1008_7dee(param_2,(param_2 >> 0x10),&local_4
                                ,0x0,param_5,0x2,0x1008);
    if (BVar1 == 0x0) {
      ctx.PTR_LOOP_1050_0310 = 0x6d2;
      return BVar1;
    }
    (param_1 + 0x20) = local_4;
    param_3 = 0x1;
  }
  return param_3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 
pass1_1020_d460(param_1: *mut u32,param_2: *mut u16,param_3: u32,param_4: u32,param_5: i16,
               param_6: u16,param_7: i16,undefined8 param_8)

{
  let uVar1: u16;
  let puVar2: *mut u8
  let unaff_SS: u16;
  let uVar3: u32;
  let puVar4: *mut u16;
  
  uVar1 = pass1_1028_bc90(param_1,param_2,param_3,param_4,param_5,param_6,unaff_SS);
  if (uVar1 != 0x0) {
    uVar3 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,
                            (_PTR_LOOP_1050_4230 + 0x16),0x11,param_6,
                            _PTR_LOOP_1050_4230,&ctx.PTR_LOOP_1050_1038,
                            unaff_SS);
    puVar2 = (uVar3 >> 0x10);
    ctx.PTR_LOOP_1050_5b80 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
    unk_win_msg_op_1008_9510(&ctx.PTR_LOOP_1050_5b80,0x1008,unaff_SS);
    puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3a,unaff_SS,puVar2,param_7);
    (param_1 + 0x20) = (puVar4 + 0xa);
    uVar1 = 0x1;
  }
  return uVar1;
}



fn pass1_1020_d4ca(param_1: u32,param_2: i16)
{
  let BVar1: bool;
  let uVar2: u32;
  let extraout_DX: u16;
  let uVar3: u16;
  let iVar4: i16;
  
  if ((param_1 + 0x20) == 0x2) {
    return;
  }
  pass1_1028_b58e(param_1);
  uVar2 = (param_2 + 0x2e);
  iVar4 = 0x63;
  uVar3 = extraout_DX;
  pass1_1038_3fb0(uVar2);
  BVar1 = pass1_1030_25b2(uVar2 & 0xffff | uVar3 << 0x10,iVar4);
  if (BVar1 != 0x0) {
    return;
  }
  return;
}



astruct_18 *  pass1_1020_d518(astruct_18 *param_1,param_2: u8)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1020_d5c8(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u16

{
  pass1_1028_b39e(CONCAT22(param_2,param_1),param_3,param_4,param_5);
  CONCAT22(param_2,param_1) = 0xd7fe;
  (param_1 + 0x2) = 0x1020;
  return CONCAT22(param_2,param_1);
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_d5f2(param_1: u32,param_2: u32,param_3: i16,param_4: u16)
{
  code **ppcVar1;
  let uVar2: u16;
  let puVar3: u32;
  let extraout_DX: u16;
  let uVar4: u16;
  let uVar5: u32;
  let puVar6: u32;
  let puVar7: u32;
  let bStack55: u8;
  u8 local_32 [0xa];
  let uStack40: u32;
  let uStack36: u32;
  let puStack28: u32;
  let local_1a: u32;
  let iStack22: i16;
  let uStack20: u16;
  let iStack18: i16;
  let uStack16: u16;
  let iStack14: i16;
  let local_c: u32;
  let iStack8: i16;
  let iStack6: i16;
  let uStack4: u16;
  
  pass1_1028_b58e(param_1);
  local_c = (param_3 + 0xc);
  iStack18 = (param_3 + 0x10);
  puStack28 = &local_c;
  uStack16 = extraout_DX;
  iStack14 = iStack18;
  iStack8 = iStack18;
  iStack6 = param_3;
  uStack4 = extraout_DX;
  pass1_1028_bab6(param_1,iStack18,extraout_DX);
  uVar2 = pass1_1030_2fac(CONCAT22(uStack16,iStack18));
  local_1a = local_c;
  iStack22 = iStack8;
  uStack36 = CONCAT22(uStack36._2_2_,&local_1a);
  iStack14 += 0x1;
  uStack20 = uVar2;
  if (iStack14 < uVar2) {
    puVar7 = CONCAT22(param_4,local_32);
    iStack22 = iStack14;
    uVar5 = pass1_1028_bb24(param_1);
    uVar4 = (uVar5 >> 0x10);
    puVar3 = &local_1a;
    pass1_1030_64ce(param_4,puVar3,uVar4,_PTR_LOOP_1050_5740,
                    CONCAT22(param_4,puVar3),
                    uVar5 & 0xffff | uVar4 << 0x10,puVar7);
    uStack40 = *puVar3;
    uVar4 = (puVar3 + 0x2);
    bStack55 = (uStack40 >> 0x18);
    uVar2 = bStack55;
    uStack36 = uStack40;
    if (bStack55 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack40,uVar4);
      puVar6 = struct_op_1030_73a8(CONCAT22(uVar4,uVar2));
      uVar2 = puVar6;
      ppcVar1 = (*puVar6 + 0x58);
      (**ppcVar1)();
    }
  }
  pass1_1028_b46e(param_1,param_2,uVar2);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_d6e6(param_1: u32,param_2: i16,param_3: u16)
{
  code **ppcVar1;
  let puVar2: u32;
  let uVar3: u16;
  let extraout_DX: u16;
  let uVar4: u16;
  let uVar5: u32;
  let puVar6: u32;
  let puVar7: u32;
  let bStack55: u8;
  u8 local_32 [0xa];
  let uStack40: u32;
  let uStack36: u32;
  let puStack28: u32;
  let local_1a: u32;
  let iStack22: i16;
  let uStack20: u16;
  let iStack18: i16;
  let uStack16: u16;
  let iStack14: i16;
  let local_c: u32;
  let iStack8: i16;
  let iStack6: i16;
  let uStack4: u16;
  
  pass1_1028_b514(param_1);
  pass1_1028_b58e(param_1);
  local_c = (param_2 + 0xc);
  iStack18 = (param_2 + 0x10);
  puStack28 = &local_c;
  uStack16 = extraout_DX;
  iStack14 = iStack18;
  iStack8 = iStack18;
  iStack6 = param_2;
  uStack4 = extraout_DX;
  pass1_1028_bab6(param_1,iStack18,extraout_DX);
  uStack20 = pass1_1030_2fac(CONCAT22(uStack16,iStack18));
  local_1a = local_c;
  uStack36 = CONCAT22(uStack36._2_2_,&local_1a);
  iStack22 = iStack14 + 0x1;
  if (iStack22 < uStack20) {
    puVar7 = CONCAT22(param_3,local_32);
    iStack14 = iStack22;
    uVar5 = pass1_1028_bb24(param_1);
    uVar4 = (uVar5 >> 0x10);
    puVar2 = &local_1a;
    pass1_1030_64ce(param_3,puVar2,uVar4,_PTR_LOOP_1050_5740,
                    CONCAT22(param_3,puVar2),
                    uVar5 & 0xffff | uVar4 << 0x10,puVar7);
    uStack40 = *puVar2;
    uVar4 = (puVar2 + 0x2);
    bStack55 = (uStack40 >> 0x18);
    uVar3 = bStack55;
    if (bStack55 != 0x0) {
      uStack36 = uStack40;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack40,uVar4);
      puVar6 = struct_op_1030_73a8(CONCAT22(uVar4,uVar3));
      if ((puVar6 + 0xc) == 0x6a) {
        ppcVar1 = (*puVar6 + 0x24);
        (**ppcVar1)();
      }
    }
  }
  return;
}



astruct_18 *  pass1_1020_d7d8(astruct_18 *param_1,param_2: u8)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1020_d888(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u16

{
  pass1_1028_b39e(CONCAT22(param_2,param_1),param_3,param_4,param_5);
  CONCAT22(param_2,param_1) = 0xd8ec;
  (param_1 + 0x2) = 0x1020;
  return CONCAT22(param_2,param_1);
}



fn pass1_1020_d8ca(param_1: i16,param_2: u16) -> u32

{
  pass1_1028_b418((param_1 + 0x6));
  if (((param_1 + 0xa) & 0x1) != 0x0) {
    fn_ptr_1000_17ce((param_1 + 0x6),0x1000);
  }
  return CONCAT22((param_1 + 0x8),(param_1 + 0x6));
}


fn pass1_1020_d9fa(param_1: u32,param_2: u16)
{
  let extraout_DX: u16;
  
  if ((param_1 + 0xc) != 0x79) {
    pass1_1030_df0c(param_1,param_2);
    pass1_1028_b58e(param_1);
    pass1_1038_57dc((param_2 + 0x2e),0x1,0x2);
  }
  return;
}



fn pass1_1020_da3c(param_1: *mut u32)
{
  pass1_1028_bdac(param_1,0x2,&USHORT_1050_1028);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_da4e(param_1: *mut u32,param_2: *mut u16,param_3: u32,param_4: u32,param_5: u16,
               param_6: i16,param_7: u16)

{
  code **ppcVar1;
  let puVar2: u32;
  let uVar3: u16;
  let Bvar4: bool;
  let extraout_DX: *mut u8
  let puVar5: *mut u8
  let extraout_DX_00: *mut u8
  let uVar6: u16;
  let uVar7: u32;
  let uVar8: u32;
  let uVar9: u16;
  let uVar11: u16;
  let puVar10: *mut u16;
  let uVar12: u32;
  let bStack31: u8;
  let local_e: u32;
  let uStack10: u16;
  let uStack8: u16;
  let uStack6: u32;
  
  puVar2 = &local_e;
  pass1_1030_64ce(param_7,puVar2,param_5,_PTR_LOOP_1050_5740,param_2,param_4,
                  CONCAT22(param_7,puVar2));
  uStack6 = *puVar2;
  uVar6 = (puVar2 + 0x2);
  bStack31 = (uStack6 >> 0x18);
  uVar3 = bStack31;
  if (bStack31 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6,uVar6);
    uVar7 = struct_op_1030_73a8(CONCAT22(uVar6,uVar3));
    uVar6 = (uVar7 >> 0x10);
    uVar3 = uVar7;
    if ((uVar3 + 0xc) == 0x10) {
      ctx.PTR_LOOP_1050_50ca = 0x6a9;
      return;
    }
  }
  uVar9 = param_1;
  uVar11 = (param_1 >> 0x10);
  pass1_1028_c7b6(param_7,uVar6,uVar9,uVar11,param_2,param_4);
  uVar8 = param_1 & 0xffff | uVar11 << 0x10;
  ppcVar1 = (*param_1 + 0x60);
  puVar10 = param_2;
  uVar7 = param_3;
  uVar12 = param_4;
  uStack8 = uVar3;
  (**ppcVar1)();
  if (((uVar3 != 0x0) &&
      (puVar5 = extraout_DX,
      pass1_1028_c23e(uVar9,uVar11,param_2,param_3,param_4,uVar3,extraout_DX,param_7
                     ), uVar3 != 0x0)) &&
     (BVar4 = pass1_1028_c314(param_7,uVar3,puVar5,uVar9,uVar11,param_2,
                              param_3,(param_3 >> 0x10),param_4),
     BVar4 != 0x0)) {
    uVar6 = (param_2 >> 0x10);
    if ((((param_2 + 0x4) == 0x0) && (uStack8 != 0x4)) &&
       (ppcVar1 = (*param_1 + 0x5c),
       (**ppcVar1)(&USHORT_1050_1028,param_1,param_2,param_3,param_4,uVar8,puVar10,
                   uVar7,uVar12), puVar5 = extraout_DX_00, BVar4 == 0x0)) {
      return;
    }
    uStack10 = (param_2 + 0x4);
    if (uStack10 != 0x0) {
      pass1_1020_df10(param_1,
                      (param_2 & 0xffff | uVar6 << 0x10),param_4,
                      uStack10,puVar5,param_6,param_7);
      return;
    }
    pass1_1020_deac(param_1,
                    (param_2 & 0xffff | uVar6 << 0x10),param_4,0x0
                    ,puVar5,param_6,param_7);
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_db86(param_1: u16,param_2: u16,param_3: *mut u16,param_4: u32,param_5: i32,
               param_6: u16)

{
  let iVar1: i16;
  let puVar2: *mut u8;
  let uVar3: u16;
  let uVar4: u32;
  let puVar5: *mut u16;
  let local_4: [u8;2];
  
  uVar4 = pass1_1030_bcae(local_4,param_6);
  uVar3 = (uVar4 >> 0x10);
  iVar1 = uVar4;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4,(param_4 >> 0x10));
  uVar4 = (iVar1 + 0x10);
  puVar5 = param_3;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4,(uVar4 >> 0x10));
  puVar2 = local_4;
  pass1_1030_bcde(param_6,puVar2,param_6,uVar4 & 0xffff | uVar3 << 0x10,
                  puVar5,param_5);
  if (puVar2 < 0x0) {
    ctx.PTR_LOOP_1050_50ca = 0x6af;
  }
  else {
    if ((puVar2 < 0x1f) || ((param_3 + 0x4) < 0x1)) {
      return;
    }
    ctx.PTR_LOOP_1050_50ca = 0x6b6;
    ctx.PTR_LOOP_1050_50cc = puVar2 + -0x1e;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_dc1c(param_1: u32,param_2: *mut u16,param_3: u16)
{
  let iVar1: i16;
  code **ppcVar2;
  let puVar3: u32;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u32;
  let puVar7: u32;
  let puVar8: u32;
  let bStack27: u8;
  let local_a: [u8;4];
  let uStack6: u32;
  
  puVar8 = CONCAT22(param_3,local_a);
  uVar6 = pass1_1028_bb24(param_1);
  uVar5 = (uVar6 >> 0x10);
  puVar3 = uVar6;
  pass1_1030_64ce(param_3,puVar3,uVar5,_PTR_LOOP_1050_5740,param_2,
                  uVar6 & 0xffff | uVar5 << 0x10,puVar8);
  uStack6 = *puVar3;
  uVar5 = (puVar3 + 0x2);
  bStack27 = (uStack6 >> 0x18);
  uVar4 = bStack27;
  if (bStack27 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6,uVar5);
    puVar7 = struct_op_1030_73a8(CONCAT22(uVar5,uVar4));
    iVar1 = (puVar7 + 0xc);
    if (((iVar1 < 0x1) || (SBORROW2(iVar1,0x1))) ||
       ((iVar1 != 0x9 && 0x7 < iVar1 + -0x1 &&
        ((iVar1 + -0x9 < 0x6a || (0x6 < iVar1 + -0x73)))))) {
      ppcVar2 = (*puVar7 + 0x24);
      (**ppcVar2)();
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1020_dca8(param_1: u32,param_2: u16,param_3: u16)
{
  let uVar1: u16;
  let uVar2: u16;
  u8 local_2e [0xe];
  let puStack32: u32;
  let uStack30: u16;
  let uStack28: u16;
  let uStack26: u16;
  let uStack24: u16;
  let uStack22: u16;
  let uStack20: u16;
  let uStack18: u16;
  let local_10: u32;
  let uStack12: u16;
  let uStack10: u32;
  let local_6: [u8;2];
  let local_4: i16;
  
  pass1_1028_c1f8(param_3,local_6,param_2,param_1,CONCAT22(param_3,local_6)
                  ,CONCAT22(param_3,&local_4));
  uStack10 = pass1_1028_b58e(param_1);
  uVar1 = (uStack10 >> 0x10);
  local_10 = (uStack10 + 0xc);
  uStack12 = (uStack10 + 0x10);
  puStack32 = &local_10;
  uStack18 = local_10;
  uStack20 = (local_10 >> 0x10);
  uStack24 = local_10 - 0x1;
  if (uStack24 < 0x0) {
    uStack24 = 0x0;
  }
  uVar2 = local_4 - 0x1;
  uStack26 = local_10 + 0x1;
  if (uVar2 < (local_10 + 0x1)) {
    uStack26 = uVar2;
  }
  uStack28 = uStack20 - 0x1;
  if (uStack28 < 0x0) {
    uStack28 = 0x0;
  }
  uStack30 = uStack20 + 0x1;
  if (uVar2 < (uStack20 + 0x1)) {
    uStack30 = uVar2;
  }
  uStack22 = uStack12;
  pass1_1008_3e54(CONCAT22(param_3,local_2e),uStack12,uStack28,uStack24);
  pass1_1020_dc1c(param_1,CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54(CONCAT22(param_3,local_2e),uStack22,uStack28,uStack18);
  pass1_1020_dc1c(param_1,CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54(CONCAT22(param_3,local_2e),uStack22,uStack28,uStack26);
  pass1_1020_dc1c(param_1,CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54(CONCAT22(param_3,local_2e),uStack22,uStack20,uStack24);
  pass1_1020_dc1c(param_1,CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54(CONCAT22(param_3,local_2e),uStack22,uStack20,uStack26);
  pass1_1020_dc1c(param_1,CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54(CONCAT22(param_3,local_2e),uStack22,uStack30,uStack24);
  pass1_1020_dc1c(param_1,CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54(CONCAT22(param_3,local_2e),uStack22,uStack30,uStack18);
  pass1_1020_dc1c(param_1,CONCAT22(param_3,local_2e),param_3);
  pass1_1008_3e54(CONCAT22(param_3,local_2e),uStack22,uStack30,uStack26);
  pass1_1020_dc1c(param_1,CONCAT22(param_3,local_2e),param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_de32(param_1: u32,param_2: u16,uchar *param_3,param_4: i16,param_5: u16)
{
  let BVar1: bool;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let puVar5: *mut u16;
  
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x5,param_5,param_3,param_4);
  uVar2 = (puVar5 >> 0x10);
  (puVar5 + 0x12) = param_2;
  uVar3 = uVar2;
  BVar1 = bring_win_to_top_1038_b72e(_PTR_LOOP_1050_5b7c,0x4,&ctx.PTR_LOOP_1050_1038);
  if (BVar1 == 0x0) {
    pass1_1038_af40(_PTR_LOOP_1050_5b7c,(_PTR_LOOP_1050_4230 + 0x16),
                    0x4,uVar3,_PTR_LOOP_1050_4230,&ctx.PTR_LOOP_1050_1038,
                    param_5);
  }
  ctx.PTR_LOOP_1050_5b80 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
  unk_win_msg_op_1008_9510(&ctx.PTR_LOOP_1050_5b80,0x1008,param_5);
  uVar4 = (param_1 >> 0x10);
  (param_1 + 0x24) = (puVar5 + 0xa);
  if ((param_1 + 0x24) == 0x0) {
    ctx.PTR_LOOP_1050_50ca = 0x6b2;
  }
  return;
}



bool 
pass1_1020_deac(param_1: u32,param_2: *mut u16,param_3: i32,param_4: i16,uchar *param_5,
               param_6: i16,param_7: u16)

{
  let uVar1: u16;
  let uVar2: u16;
  
  uVar1 = param_1;
  uVar2 = (param_1 >> 0x10);
  pass1_1028_c7b6(param_7,param_5,uVar1,uVar2,param_2,param_3);
  if (param_4 < 0x1) {
    return 0x0;
  }
  if (SBORROW2(param_4,0x1)) {
    return 0x0;
  }
  if (param_4 != 0x3 && 0x0 < param_4 + -0x2) {
    if (param_4 == 0x4) {
      pass1_1020_de32(param_1,0x4,param_5,param_6,param_7);
      if ((uVar1 + 0x24) == 0x6) {
        return 0x1;
      }
      return 0x0;
    }
    if (param_4 != 0x5) {
      return 0x0;
    }
  }
  (uVar1 + 0x24) = 0x1;
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_df10(param_1: u32,param_2: *mut u16,param_3: i32,param_4: u16,uchar *param_5,
               param_6: i16,param_7: u16)

{
  let puVar1: u32;
  let uVar2: u16;
  let BVar3: bool;
  let uVar4: u16;
  let uVar5: u32;
  let uVar6: u16;
  let uVar7: u16;
  let bStack31: u8;
  let local_e: u32;
  let uStack10: u32;
  let uStack6: u16;
  let uStack4: u16;
  
  uStack4 = 0x0;
  uVar6 = param_1;
  uVar7 = (param_1 >> 0x10);
  pass1_1028_c7b6(param_7,param_5,uVar6,uVar7,param_2,param_3);
  uStack6 = param_4;
  if (param_4 == 0x0) {
    puVar1 = &local_e;
    pass1_1030_64ce(param_7,puVar1,param_5,_PTR_LOOP_1050_5740,param_2,param_3,
                    CONCAT22(param_7,puVar1));
    uStack10 = *puVar1;
    uVar4 = (puVar1 + 0x2);
    bStack31 = (uStack10 >> 0x18);
    uVar2 = bStack31;
    if (bStack31 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack10,uVar4);
      uVar5 = struct_op_1030_73a8(CONCAT22(uVar4,uVar2));
      if ((uVar5 + 0xc) == 0x6a) {
        BVar3 = pass1_1020_e044(param_1);
        if (BVar3 == 0x0) {
          (uVar6 + 0x24) = 0x1;
        }
        else {
          ctx.PTR_LOOP_1050_50ca = 0x6ac;
        }
      }
    }
  }
  else {
    if (((0x5 < param_4) && (!SBORROW2(param_4,0x6))) && ((param_4 - 0x6) < 0x4)
       ) {
      pass1_1020_de32(param_1,param_4,param_5,param_6,param_7);
      if (true) {
        switch((uVar6 + 0x24)) {
        case 0x1:
          BVar3 = pass1_1020_e044(param_1);
          if (BVar3 != 0x0) {
            ctx.PTR_LOOP_1050_50ca = 0x6ac;
          }
          break;
        case 0x2:
        case 0x3:
        case 0x4:
        case 0x5:
          pass1_1020_e652(param_1,param_2,(param_2 >> 0x10),
                          param_3,param_7);
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_e044(param_1: u32) -> bool

{
  let uVar1: u32;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u32;
  
  uVar3 = (param_1 >> 0x10);
  uVar4 = pass1_1018_04b8((param_1 + 0x28));
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4,(uVar4 >> 0x10));
  uVar2 = pass1_1030_2fac(uVar4);
  uVar1 = (param_1 + 0x28);
  if (uVar2 <= (uVar1 + 0x1e)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_e08e(param_1: u32,param_2: u16,param_3: u16,param_4: u8)
{
  let iVar1: i16;
  let uVar2: u32;
  let iVar3: i16;
  let iVar4: i16;
  let uVar5: u32;
  let extraout_DX: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let piVar9: *mut i16;
  let puVar10: *mut u16;
  let uVar11: u16;
  let uVar12: u16;
  let local_158: u16;
  let uStack342: u16;
  let uStack50: u32;
  let puStack42: u32;
  let local_22: i16;
  let local_20: [u8;2];
  let local_1e: [u8;2];
  let uStack28: u16;
  let piStack26: *mut i16;
  let local_18: i16;
  let local_16: u16;
  let uStack20: u32;
  let local_10: u32;
  let iStack12: i16;
  let uStack10: u32;
  let uStack6: u32;
  
  uVar8 = (param_1 >> 0x10);
  uVar7 = param_1;
  iVar3 = (uVar7 + 0xc);
  if (iVar3 == 0x74) {
    iVar1 = (uVar7 + 0x24);
    iVar3 = iVar1 + -0x1;
    if (iVar3 == 0x0) goto LAB_1020_e0ae;
    iVar3 = iVar1 + -0x6;
    if (iVar3 != 0x0) goto LAB_1020_e0b9;
    uVar11 = 0x1;
  }
  else {
    if (iVar3 == 0x78) {
      iVar1 = (uVar7 + 0x24);
      iVar4 = iVar1 + -0x1;
      if (iVar4 != 0x0) {
        iVar3 = iVar1 + -0x2;
        if ((0x0 < iVar4) && (!SBORROW2(iVar4,0x1))) {
          if (iVar1 + -0x5 == 0x0 || iVar3 < 0x3) {
            iVar3 = iVar1 + -0x5;
            pass1_1020_e49a(param_1,param_3,param_4);
          }
          else {
            iVar3 = iVar1 + -0x6;
            if (iVar3 == 0x0) {
              pass1_1020_e39c(param_1,0x6,0x0,param_3,param_4);
              pass1_1020_dca8(param_1,param_2,param_3);
            }
          }
        }
        goto LAB_1020_e0b9;
      }
      uVar11 = 0x6a;
      iVar3 = iVar4;
    }
    else {
      iVar3 += -0x79;
      if (iVar3 == 0x0) {
        pass1_1020_e49a(param_1,param_3,param_4);
        return;
      }
//LAB_1020_e0ae:
      uVar11 = 0x47;
    }
  }
  pass1_1020_e39c(param_1,uVar11,iVar3,param_3,param_4);
//LAB_1020_e0b9:
  pass1_1028_b58e(param_1);
  uStack6 = CONCAT22(extraout_DX,iVar3);
  uVar5 = (iVar3 + 0x2e);
  uVar6 = (iVar3 + 0x30);
  uStack10 = uVar5;
  if ((uVar7 + 0xc) != 0x79) {
    pass1_1038_5804(uVar5 & 0xffff | uVar6 << 0x10,0x1,0x2);
  }
  if ((uVar7 + 0x24) == 0x6) {
    pass1_1038_43cc(uStack10,(uStack10 >> 0x10),0x1,0x2,uVar5,uVar6);
  }
  local_10 = (uStack6 + 0xc);
  iStack12 = (uStack6 + 0x10);
  puStack42 = &local_10;
  if (((uVar7 + 0x24) == 0x6) && (iStack12 == 0x0)) {
    return;
  }
  uVar2 = (uVar7 + 0x28);
  uVar5 = (uVar2 + 0x20);
  puVar10 = &local_16;
  piStack26 = &local_18;
  piVar9 = piStack26;
  uVar11 = param_3;
  uVar12 = param_3;
  uStack20 = uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5,(uVar5 >> 0x10));
  uStack28 = uVar5;
  pass1_1030_5b1c(uVar5 & 0xffff | ZEXT24(piStack26) << 0x10,
                  CONCAT22(uVar11,piVar9),CONCAT22(uVar12,puVar10));
  pass1_1028_c8ee(param_3,uVar7,uVar8,(uVar7 + 0x24),
                  CONCAT22(param_3,&local_10));
  pass1_1008_3eb4(CONCAT22(param_3,&local_10),
                  CONCAT22(param_3,&local_22),
                  CONCAT22(param_3,local_20),
                  CONCAT22(param_3,local_1e));
  if ((uVar7 + 0x24) == 0x1) {
    if (local_18 < local_22) {
      pass1_1030_5b3e(CONCAT22(piStack26,uStack28),local_22,local_16);
      pass1_1030_5b1c(CONCAT22(piStack26,uStack28),CONCAT22(param_3,&local_18),
                      CONCAT22(param_3,&local_16));
    }
    uStack50 = (uStack10 + 0x4);
    struct_op_1028_87f0(param_3,param_4,(astruct_97 *)CONCAT22(param_3,&local_158),0x0,0x0
                        ,0x6a,&local_10,param_3,uStack50,uStack20);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_3,&local_158));
    local_158 = 0x389a;
    uStack342 = 0x1008;
  }
  pass1_1028_ccd0(param_4,param_3,param_1,CONCAT22(param_3,&local_10));
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_e294(param_1: u32,param_2: u16,param_3: u8)
{
  let uVar1: u32;
  let puVar2: u32;
  let uVar3: u32;
  let extraout_DX: u16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let cStack347: u8;
  u8 local_150 [0xc];
  let puStack324: u32;
  let local_140: [u8;124];
  let uStack28: u32;
  let uStack24: u32;
  let uStack20: u32;
  let local_10: u32;
  let uStack12: u16;
  let uStack10: i16;
  let uStack8: u16;
  let uStack6: u32;
  
  uVar6 = (param_1 >> 0x10);
  uVar5 = param_1;
  if ((0x1 < (uVar5 + 0x24)) && ((uVar5 + 0x24) < 0x6)) {
    uVar1 = (uVar5 + 0x28);
    uVar3 = (uVar1 + 0x20);
    uStack6 = uVar3;
    pass1_1028_b58e(param_1);
    iStack10 = uVar3;
    local_10 = (iStack10 + 0xc);
    uStack12 = (iStack10 + 0x10);
    puStack324 = &local_10;
    uVar4 = extraout_DX;
    uStack8 = extraout_DX;
    pass1_1028_c8ee(param_2,uVar5,uVar6,(uVar5 + 0x24),
                    CONCAT22(param_2,puStack324));
    puVar2 = &local_10;
    pass1_1028_c89c(param_1,CONCAT22(param_2,puVar2),
                    CONCAT22(param_2,local_150),puVar2,param_2);
    uStack20 = *puVar2;
    cStack347 = (uStack20 >> 0x18);
    if ((cStack347 == '\0') && (uStack20 == 0x9)) {
      (uVar5 + 0x14) = 0x1;
    }
    uStack24 = (iStack10 + 0x2e);
    uStack28 = (uStack24 + 0x4);
    struct_op_1028_87f0(param_2,param_3,(astruct_97 *)CONCAT22(param_2,local_140),0x0,
                        (uVar5 + 0x14) + 0x1,0x79,&local_10,param_2,uStack28,
                        uStack6);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_2,local_140));
  }
  (uVar5 + 0x26) = 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_e39c(param_1: u32,param_2: u16,param_3: i16,param_4: u16,param_5: u8)
{
  let uVar1: u32;
  let uVar2: u16;
  let extraout_DX: u16;
  let local_13c: [u8;124];
  let uStack24: u32;
  let uStack20: u32;
  let uStack16: u32;
  let local_c: u32;
  let iStack8: i16;
  let uStack6: u32;
  
  pass1_1028_b58e(param_1);
  uStack6 = CONCAT22(extraout_DX,param_3);
  local_c = (param_3 + 0xc);
  iStack8 = (param_3 + 0x10);
  if (iStack8 < 0x1) {
    uVar2 = 0x5;
  }
  else {
    uVar2 = 0x6;
  }
  (param_3 + 0x14) = uVar2;
  uVar1 = (param_1 + 0x28);
  uStack16 = (uVar1 + 0x20);
  uStack20 = (param_3 + 0x2e);
  uStack24 = (uStack20 + 0x4);
  struct_op_1028_87f0(param_4,param_5,(astruct_97 *)CONCAT22(param_4,local_13c),0x0,0x1,
                      param_2,&local_c,param_4,uStack24,uStack16);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_4,local_13c));
  return;
}



fn pass1_1020_e44c(param_1: u32,param_2: u16,param_3: u16,param_4: u8)
{
  let piVar1: *mut i16;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  if ((iVar2 + 0x12) == 0x2) {
    piVar1 = (iVar2 + 0x14);
    *piVar1 = *piVar1 + -0x1;
    if (((iVar2 + 0x26) == 0x0) && ((iVar2 + 0xc) == 0x78)) {
      pass1_1020_e294(param_1 & 0xffff | uVar3 << 0x10,param_3,param_4);
    }
    if ((iVar2 + 0x14) == 0x0) {
      pass1_1020_e08e(param_1 & 0xffff | uVar3 << 0x10,param_2,param_3,param_4);
      return;
    }
    if ((iVar2 + 0x24) == 0x6) {
      (iVar2 + 0xe) = 0x49;
    }
  }
  return;
}



fn pass1_1020_e49a(param_1: u32,param_2: u16,param_3: u8)
{
  let iVar1: i16;
  let iVar2: i16;
  let uVar3: u32;
  let uStack10: u16;
  
  uVar3 = pass1_1028_b58e(param_1);
  iVar1 = (uVar3 + 0x14);
  uStack10 = 0x0;
  iVar2 = iVar1 + -0x6;
  if (iVar2 == 0x0) {
    uStack10 = 0x9;
  }
  else {
    iVar2 = iVar1 + -0x7;
    if (iVar2 == 0x0) {
      uStack10 = 0x6;
    }
    else {
      iVar2 = iVar1 + -0x8;
      if (iVar2 == 0x0) {
        uStack10 = 0x7;
      }
      else {
        iVar2 = iVar1 + -0x9;
        if (iVar2 == 0x0) {
          uStack10 = 0x8;
        }
      }
    }
  }
  pass1_1020_e39c(param_1,uStack10,iVar2,param_2,param_3);
  return;
}



i16  pass1_1020_e4fa(param_1: u32,param_2: u16)

{
  let uVar1: u32;
  let iStack4: i16;
  
  if (false) {
switchD_1020_e53c_caseD_4:
    uVar1 = pass1_1028_b58e(param_1);
    iStack4 = (uVar1 + 0x14) + 0x2;
  }
  else {
    switch(param_2) {
    default:
      iStack4 = 0x4;
      break;
    case 0x3:
    case 0x8:
      iStack4 = 0x5;
      break;
    default:
      goto switchD_1020_e53c_caseD_4;
    }
  }
  return iStack4;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_e558(param_1: u32,param_2: i16,param_3: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  let iVar3: i16;
  let extraout_DX: u16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let bStack45: u8;
  u8 local_24 [0xc];
  let uStack24: u32;
  let uStack20: u32;
  let local_10: u32;
  let uStack12: u16;
  let uStack10: i16;
  let uStack8: u16;
  let iStack6: i16;
  let uStack4: u16;
  
  uVar7 = (param_1 >> 0x10);
  uVar6 = param_1;
  if ((uVar6 + 0xc) == 0x79) {
    param_2 = (uVar6 + 0x24);
    (uVar6 + 0x14) = param_2;
    (uVar6 + 0x24) = 0x0;
  }
  if ((uVar6 + 0x24) != 0x6) {
    pass1_1028_b58e(param_1);
    uStack8 = (param_2 + 0x14);
    iStack6 = param_2;
    uStack4 = extraout_DX;
    iStack10 = pass1_1020_e4fa(param_1,uStack8);
    local_10 = (iStack6 + 0xc);
    uStack12 = (iStack6 + 0x10);
    uStack24 = CONCAT22(uStack24._2_2_,&local_10);
    uVar4 = uStack4;
    pass1_1028_c8ee(param_3,uVar6,uVar7,(uVar6 + 0x24),
                    CONCAT22(param_3,&local_10));
    puVar1 = &local_10;
    pass1_1028_c89c(param_1,CONCAT22(param_3,puVar1),
                    CONCAT22(param_3,local_24),puVar1,param_3);
    uStack24 = *puVar1;
    uVar5 = (puVar1 + 0x2);
    bStack45 = (uStack24 >> 0x18);
    uVar2 = bStack45;
    uStack20._0_2_ = uStack24;
    uStack20 = uStack24;
    if (bStack45 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack20,uVar5);
      uStack20._0_2_ = (uVar2 + 0x14);
    }
    uStack8 = uStack20;
    iVar3 = pass1_1020_e4fa(param_1,uStack20);
    (uVar6 + 0x14) = iStack10 + iVar3;
    return;
  }
  (uVar6 + 0x14) = 0x1;
  return;
}



fn pass1_1020_e652(param_1: u32,param_2: *mut u32,param_3: u16,param_4: i32,param_5: u16) -> u32

{
  let puVar1: u32;
  let uVar2: u16;
  let uVar3: u16;
  let local_8: u32;
  let uStack4: u16;
  
  local_8 = *param_2;
  uStack4 = (param_2 + 0x1);
  uVar3 = (param_1 >> 0x10);
  uVar2 = param_1;
  pass1_1028_c8ee(param_5,uVar2,uVar3,(uVar2 + 0x24),
                  CONCAT22(param_5,&local_8));
  puVar1 = &local_8;
  pass1_1028_c7b6(param_5,param_3,uVar2,uVar3,CONCAT22(param_5,puVar1),param_4);
  if (puVar1 != 0x0) {
    puVar1 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
  }
  return puVar1;
}


fn pass1_1020_e70e(param_1: u32,param_2: u32,param_3: i16,uchar *param_4,param_5: u16)
{
  let uVar1: u16;
  let BVar2: bool;
  let uVar3: u16;
  
  pass1_1030_dec4(param_1,param_2,param_3,param_4,param_5);
  if (param_3 != 0x0) {
    uVar1 = (param_1 >> 0x10);
    uVar3 = (param_2 >> 0x10);
    BVar2 = read_file_1008_7dee(param_2,uVar3,param_1 + 0x24,0x0,uVar1,0x2,
                                0x1008);
    if (BVar2 != 0x0) {
      BVar2 = read_file_1008_7dee(param_2,uVar3,param_1 + 0x26,0x0,uVar1,0x2,
                                  0x1008);
      if (BVar2 != 0x0) {
        return;
      }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
  }
  return;
}



astruct_18 *  pass1_1020_e76c(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  pass1_1030_dcf4(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1020_e81c(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u16

{
  pass1_1028_b39e(CONCAT22(param_2,param_1),param_3,param_4,param_5);
  CONCAT22(param_2,param_1) = 0xe88e;
  (param_1 + 0x2) = 0x1020;
  return CONCAT22(param_2,param_1);
}



fn pass1_1020_e846(param_1: *mut u16)
{
  *param_1 = 0xe88e;
  (param_1 + 0x2) = 0x1020;
  pass1_1028_b418(param_1);
  return;
}



astruct_18 *  pass1_1020_e868(astruct_18 *param_1,param_2: u8)

{
  pass1_1020_e846(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1020_e91e(param_1: i16,param_2: u16,param_3: i16,param_4: u32,param_5: u16) -> u16

{
  pass1_1030_dcc2(param_1,param_2,param_3,param_4,param_5);
  (param_1 + 0x24) = 0x0;
  CONCAT22(param_2,param_1) = 0xeef6;
  (param_1 + 0x2) = 0x1020;
  return CONCAT22(param_2,param_1);
}



fn pass1_1020_e94e(param_1: u32,param_2: u32,param_3: u16) -> bool

{
  let in_AX: bool;
  let BVar1: bool;
  let local_c: [u16;0x5];
  
  pass1_1030_de7c(param_1,param_2,param_3);
  if (in_AX != 0x0) {
    local_c[0] = (param_1 + 0x24);
    BVar1 = write_to_file_1008_7e1c
                      (param_2,(param_2 >> 0x10),local_c,param_3,
                       0x2,0x1008);
    if (BVar1 == 0x0) {
      ctx.PTR_LOOP_1050_0310 = 0x6d0;
      return BVar1;
    }
    in_AX = 0x1;
  }
  return in_AX;
}



fn pass1_1020_e994(param_1: u32,param_2: u32,param_3: i16,uchar *param_4,param_5: u16)
{
  let BVar1: bool;
  
  pass1_1030_dec4(param_1,param_2,param_3,param_4,param_5);
  if ((param_3 != 0x0) &&
     (BVar1 = read_file_1008_7dee(param_2,(param_2 >> 0x10),
                                  param_1 + 0x24,0x0,(param_1 >> 0x10),0x2,
                                  0x1008), BVar1 == 0x0)) {
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
    return;
  }
  return;
}



fn pass1_1020_e9d4(param_1: u16,param_2: u16,param_3: u16)
{
  let extraout_DX: u16;
  
  pass1_1030_df0c(CONCAT22(param_2,param_1),param_3);
  pass1_1028_b58e(CONCAT22(param_2,param_1));
  pass1_1038_57dc((param_3 + 0x2e),0x1,0x1);
  return;
}



fn pass1_1020_ea0e(param_1: *mut u32)
{
  pass1_1028_bdac(param_1,0x1,&USHORT_1050_1028);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1020_ea20(param_1: u32,param_2: *mut u16,param_3: u32,param_4: u32,param_5: u16,
               param_6: u16,param_7: u8,param_8: u16)

{
  let uVar1: u16;
  code **ppcVar2;
  let uVar3: u16;
  let cVar4: u8;
  let puVar5: u32;
  let uVar6: u16;
  let puVar7: *mut u8
  let extraout_DX: *mut u8
  let unaff_DI: i16;
  let uVar8: u16;
  let uVar9: u16;
  u8 local_146 [0x10c];
  let uStack58: u16;
  let puStack56: *mut u8
  let uStack50: u32;
  let puStack46: *mut u16;
  let puStack42: u32;
  let uStack38: u32;
  let uStack34: u32;
  let uStack28: u32;
  let local_12: u32;
  let iStack14: i16;
  let puStack12: u32;
  let uStack8: u32;
  let BStack4: bool;
  
  uVar9 = param_1;
  uVar3 = (param_1 >> 0x10);
  pass1_1028_c3aa(uVar9,uVar3,param_2,param_3,param_4,param_6);
  if (param_5 == 0x0) {
    return;
  }
  pass1_1028_c23e(uVar9,uVar3,param_2,param_3,param_4,param_5,param_8,param_6);
  if (param_5 == 0x0) {
    return;
  }
  BStack4 = pass1_1028_c314(param_6,param_5,param_8,uVar9,uVar3,param_2,param_3,
                            (param_3 >> 0x10),param_4);
  if (BStack4 == 0x0) {
    return;
  }
  pass1_1028_c7b6(param_6,param_8,uVar9,uVar3,param_2,param_4);
  if ((((BStack4 == 0x5) || (BStack4 == 0x4)) || (BStack4 == 0x6)) || (BStack4 == 0x9)) {
    ctx.PTR_LOOP_1050_50ca = 0x6a8;
    return;
  }
  if (BStack4 != 0x0) {
    return;
  }
  puVar5 = &local_12;
  pass1_1030_64ce(param_6,puVar5,param_8,_PTR_LOOP_1050_5740,param_2,param_4,
                  CONCAT22(param_6,puVar5));
  uStack38 = *puVar5;
  puStack56 = *(uchar **)(puVar5 + 0x2);
  uStack38._3_1_ = (uStack38 >> 0x18);
  uStack58 = uStack38._3_1_;
  uStack28 = uStack38;
  uStack8 = uStack38;
  if (uStack38._3_1_ == 0x0) goto LAB_1020_eb4e;
  uStack8._0_2_ = uStack38;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack8,puStack56);
  uStack38 = CONCAT22(puStack56,uStack58);
  uStack34 = (uStack58 + 0x2e);
  if ((uStack34 + 0x4) != param_3) {
    ctx.PTR_LOOP_1050_50ca = 0x6b7;
    return;
  }
  uStack28 = struct_op_1030_73a8(CONCAT22(puStack56,uStack58));
  puStack56 = (uStack28 >> 0x10);
  uVar1 = (uStack28 + 0xc);
  uStack58 = uVar1;
  if (uVar1 != 0x41) {
    if (0x41 < uVar1) {
      if (uVar1 == 0x6b) {
        ctx.PTR_LOOP_1050_50ca = 0x6b1;
        return;
      }
      if (uVar1 < 0x6c) {
        if (uVar1 == 0x42) {
          ctx.PTR_LOOP_1050_50ca = 0x6b1;
          return;
        }
        uStack58 = uVar1 - 0x4b;
        if (uStack58 == 0x0) {
          ctx.PTR_LOOP_1050_50ca = 0x6b1;
          return;
        }
      }
      else {
        if (uVar1 == 0x6e) {
          return;
        }
        uStack58 = uVar1 - 0x73;
        if ((0x4 < (uVar1 - 0x6e)) &&
           (uStack58 = uVar1 - 0x79, uStack58 == 0x0 || (uVar1 - 0x73) < 0x6)) {
          ctx.PTR_LOOP_1050_50ca = 0x6b0;
          return;
        }
      }
      goto LAB_1020_eb4e;
    }
    if (uVar1 != 0x3e) {
      if (uVar1 < 0x3f) {
        cVar4 = uVar1;
        if (cVar4 != '\v') {
          if (cVar4 == '\x10') {
            return;
          }
          uStack58 = uVar1 & 0xff00 | (cVar4 - 0x37);
          if ((cVar4 - 0x37) != 0x0) goto LAB_1020_eb4e;
        }
        ctx.PTR_LOOP_1050_50ca = 0x6b4;
        return;
      }
      goto LAB_1020_eb4e;
    }
  }
  if ((uStack28 + 0x12) == 0x4) {
    ctx.PTR_LOOP_1050_50ca = 0x6b1;
    return;
  }
//LAB_1020_eb4e:
  uVar8 = 0x1000;
  mem_op_1000_179c(0xb4,puStack56,0x1000);
  puVar7 = (puStack56 | uStack58);
  if (puVar7 == 0x0) {
    iStack14 = 0x0;
    puVar7 = 0x0;
  }
  else {
    uVar8 = SUB42(&ctx.PTR_LOOP_1050_1040,0x0);
    iStack14 = string_1040_8520((astruct_57 *)
                                CONCAT13((puStack56 >> 0x8),
                                         CONCAT12(puStack56,uStack58)),
                                ctx.PTR_LOOP_1050_0396,0x24,0x2,0x57b,0x5e8,puVar7,
                                param_6);
  }
  puStack12 = CONCAT22(puVar7,iStack14);
  ppcVar2 = (*puStack12 + 0x74);
  (**ppcVar2)(uVar8,iStack14,puVar7);
  if (iStack14 != 0x7) {
    puStack46 = uStack8;
    uStack34 = uStack8;
    uStack34._3_1_ = (uStack8 >> 0x18);
    uVar6 = uStack34._3_1_;
    if (uStack34._3_1_ != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack8,uStack8._2_2_);
      uStack50 = CONCAT22(uStack8._2_2_,uVar6);
      fn_ptr_1030_7296(CONCAT22(uStack8._2_2_,uVar6));
      pass1_1030_730a(uStack50,uVar6,0x1030,param_6);
      puStack46 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,uStack8._2_2_,unaff_DI)
      ;
      pass1_1010_ec68(puStack46,uStack50,param_6);
      puStack42 = struct_op_1030_73a8(uStack50);
      puVar5 = puStack42;
      ppcVar2 = (*puStack42 + 0x24);
      (**ppcVar2)(0x1030,puStack42,(puStack42 >> 0x10));
      uVar6 = pass1_1028_bc4a(puStack42,puVar5,extraout_DX,param_6);
      (uVar9 + 0x24) = uVar6;
      struct_1030_e4fa((astruct_100 *)CONCAT22(param_6,local_146),
                       (uStack50 + 0x16),param_6,param_7);
      fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_6,local_146));
    }
    return;
  }
  ctx.PTR_LOOP_1050_50ca = (&ctx.PTR_LOOP_1050_0000 + 0x1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_ecb0(param_1: u32,param_2: i16,param_3: u16)
{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  let unaff_SS: u16;
  let uStack8: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  uVar1 = (iVar2 + 0x8);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
  if ((iVar2 + 0x12) != 0x1) {
    pass1_1028_bf22(param_1 & 0xffff | uVar3 << 0x10,param_3,unaff_SS);
    return;
  }
  if (false) {
switchD_1020_ed0e_caseD_4:
    uStack8 = (param_2 + 0x14);
  }
  else {
    switch((param_2 + 0x14)) {
    default:
      uStack8 = 0x2;
      break;
    case 0x3:
    case 0x8:
      uStack8 = 0x3;
      break;
    default:
      goto switchD_1020_ed0e_caseD_4;
    case 0x5:
    case 0x6:
      uStack8 = 0x1;
    }
  }
  (iVar2 + 0x14) = uStack8;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_ed3c(param_1: u32,param_2: i16,param_3: u16,param_4: u8)
{
  let piVar1: *mut i16;
  let uVar2: u16;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let iVar3: i16;
  let uVar4: u16;
  let local_138: [u8;112];
  let uStack38: u32;
  let puStack30: u32;
  let uStack28: u32;
  let uStack24: u32;
  let uStack20: u16;
  let local_12: i16;
  let local_10: [u8;2];
  let local_e: [u8;2];
  let local_c: u32;
  let uStack8: u16;
  let iStack6: i16;
  let uStack4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  piVar1 = (iVar3 + 0x14);
  *piVar1 = *piVar1 + -0x1;
  if (*piVar1 == 0x0) {
    (iVar3 + 0x12) = 0x0;
    pass1_1028_b58e(param_1);
    local_c = (param_2 + 0xc);
    uStack8 = (param_2 + 0x10);
    puStack30 = &local_c;
    iStack6 = param_2;
    uStack4 = extraout_DX;
    pass1_1008_3eb4(CONCAT22(param_3,&local_c),
                    CONCAT22(param_3,&local_12),
                    CONCAT22(param_3,local_10),
                    CONCAT22(param_3,local_e));
    if (local_12 < 0x1) {
      puStack30 = 0x5;
    }
    else {
      puStack30 = 0x6;
    }
    (iStack6 + 0x14) = puStack30;
    if (local_12 < 0x1) {
      uVar2 = 0x5;
    }
    else {
      uVar2 = 0x9;
    }
    uStack20 = uVar2;
    pass1_1020_ee3a(param_1,uVar2,uVar2,param_3,param_4);
    pass1_1028_b58e(param_1);
    uStack24 = CONCAT22(extraout_DX_00,uVar2);
    uStack28 = (uVar2 + 0x2e);
    pass1_1038_5804(uStack28,0x1,0x1);
    if (0x0 < (iVar3 + 0x24)) {
      uStack38 = (uStack28 + 0x4);
      pass1_1028_68de((astruct_100 *)CONCAT22(param_3,local_138),(iVar3 + 0x24)
                      ,uStack38,param_4,param_3);
      fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_3,local_138));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1020_ee3a(param_1: u32,param_2: u16,param_3: i16,param_4: u16,param_5: u8)
{
  let extraout_DX: u16;
  let local_13c: [u8;124];
  let uStack24: u32;
  let uStack20: u32;
  let uStack16: u32;
  let local_c: u32;
  let uStack8: u16;
  let iStack6: i16;
  let uStack4: u16;
  
  pass1_1028_b58e(param_1);
  local_c = (param_3 + 0xc);
  uStack8 = (param_3 + 0x10);
  iStack6 = param_3;
  uStack4 = extraout_DX;
  uStack16 = pass1_1028_bb24(param_1);
  uStack20 = (iStack6 + 0x2e);
  uStack24 = (uStack20 + 0x4);
  struct_op_1028_87f0(param_4,param_5,(astruct_97 *)CONCAT22(param_4,local_13c),0x0,0x1,
                      param_2,&local_c,param_4,uStack24,uStack16);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_4,local_13c));
  return;
}



astruct_18 *  pass1_1020_eed0(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  pass1_1030_dcf4(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1020_ef5e(param_1: *mut u16)
{
  *param_1 = 0x0;
  (param_1 + 0x2) = &USHORT_1050_1028;
  pass1_1028_b418(param_1);
  return;
}



astruct_18 *  pass1_1020_ef94(astruct_18 *param_1,param_2: u8)

{
  pass1_1020_ef5e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}
