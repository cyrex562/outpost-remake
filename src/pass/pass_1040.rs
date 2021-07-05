
fn pass1_1040_0a1a(param_1: u32)
{
  let uVar1: u16;
  let puVar2: u32;
  code **ppcVar3;
  let uVar4: u32;
  let puVar5: u32;
  let extraout_DX: *mut u8
  let puVar6: *mut u8
  let extraout_DX_00: *mut u8
  let puVar7: *mut u8
  let iVar8: i16;
  let iVar9: i16;
  let uVar10: u16;
  let uVar11: u16;
  let uStack10: u32;
  let uStack6: u16;
  
  uVar10 = (param_1 >> 0x10);
  iVar8 = param_1;
  uVar4 = (iVar8 + 0x8e);
  uVar11 = (uVar4 >> 0x10);
  iVar9 = uVar4;
  puVar2 = (iVar9 + 0xa);
  uStack6 = puVar2;
  puVar5 = ((iVar9 + 0xc) | uStack6);
  if (puVar5 == 0x0) {
    return;
  }
  ppcVar3 = (code **)(*puVar2 + 0x14);
  (**ppcVar3)();
  uStack10 = CONCAT22(extraout_DX,puVar5);
  puVar6 = extraout_DX;
  if (*(long *)(iVar8 + 0x70) != 0x0) {
    puVar5 = (iVar8 + 0x70);
    uVar1 = (iVar8 + 0x72);
    puVar6 = (uVar1 | puVar5);
    if (puVar6 != 0x0) {
      ppcVar3 = (code **)*puVar5;
      (**ppcVar3)();
      puVar6 = extraout_DX_00;
    }
  }
  mem_op_1000_179c(0x14,puVar6,0x1000);
  puVar7 = (puVar6 | puVar5);
  if (puVar7 == 0x0) {
    puVar5 = 0x0;
    puVar7 = 0x0;
  }
  else {
    struct_1008_4c58((u16 *)CONCAT22(puVar6,puVar5));
  }
  (iVar8 + 0x70) = puVar5;
  *(uchar **)(iVar8 + 0x72) = puVar7;
  pass1_1008_4d84(*(astruct_90 **)(iVar8 + 0x70),uStack10,puVar7);
  return;
}


astruct_18 *  pass1_1040_0b6a(astruct_18 *param_1,param_2: u8)

{
  pass1_1040_073a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * 
pass1_1040_0bfc(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16,uchar *param_6,param_7: i16,param_8: u16)

{
  astruct_715 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfcd,param_5);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_715 *)param_1;
  &iVar1->field_0x8e = 0x0;
  param_1 = 0xdb0;
  iVar1->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x39,param_8,param_6,param_7);
  iVar1->field_0x8e = puVar2;
  iVar1->field_0x90 = (puVar2 >> 0x10);
  iVar1->field_0x74 = 0x1;
  return param_1;
}



fn pass1_1040_0c54(astruct_18 *param_1,param_2: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xdb0;
  (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  (param_1 + 0x8e) = 0x0;
  ui_cleanup_op_1040_782c(param_1,param_2);
  return;
}


fn pass1_1040_0d80(void) -> u16

{
  return 0x1;
}



astruct_18 * 
pass1_1040_0d8a(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  pass1_1040_0c54(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1040_0e1c(astruct_57 *param_1,param_2: u16,param_3: u32,param_4: u16,
               uchar *param_5,param_6: i16,param_7: u16)

{
  astruct_716 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x1c0,param_4);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_716 *)param_1;
  &iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = param_3;
  iVar1->field_0x96 = 0x0;
  iVar1->field_0x98 = param_2;
  param_1 = (s_overflow_on_node__d_1050_11ca + 0x8);
  iVar1->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3a,param_7,param_5,param_6);
  iVar1->field_0x8e = puVar2;
  iVar1->field_0x90 = (puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_0e86(astruct_18 *param_1,param_2: u16)
{
  let uVar1: u16;
  astruct_18 *paVar2;
  let puVar3: *mut u8
  let iVar4: i16;
  let unaff_DI: i16;
  let uVar5: u16;
  let uVar6: u16;
  let puVar7: *mut u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  param_1->field_0x0 = (s_overflow_on_node__d_1050_11ca + 0x8);
  (iVar4 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  paVar2 = *(astruct_18 **)(iVar4 + 0x92);
  uVar1 = (iVar4 + 0x94);
  puVar3 = (uVar1 | paVar2);
  if (puVar3 != 0x0) {
    pass1_1040_a5d0(paVar2 & 0xffff | uVar1 << 0x10);
    fn_ptr_1000_17ce(paVar2,0x1000);
  }
  ctx.PTR_LOOP_1050_5b82 = (iVar4 + 0x96);
  if (*(long *)(iVar4 + 0x92) == 0x0) {
    uVar6 = SUB42(&ctx.PTR_LOOP_1050_1038,0x0);
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(iVar4 + 0x6));
  }
  else {
    puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_2,puVar3,unaff_DI);
    uVar6 = 0x1010;
    pass1_1010_7b8c(puVar7,(iVar4 + 0x6),param_2);
  }
  ui_cleanup_op_1040_782c(param_1,uVar6);
  return;
}


void 
pass1_1040_109c(param_1: i16,param_2: u16,param_3: u16,param_4: u32,uchar *param_5,
               param_6: i16,param_7: u16,param_8: u16)

{
  let uVar1: u32;
  let bVar2: bool;
  let iVar3: i16;
  let puVar4: *mut u16;
  
  bVar2 = false;
  if (param_4._2_2_ == 0x1c1) {
    (param_1 + 0x96) = 0x2;
    bVar2 = true;
  }
  else {
    if (param_4._2_2_ == 0x1c2) {
      (param_1 + 0x96) = 0x1;
      bVar2 = true;
    }
    else {
      if (param_4._2_2_ != 0x1830) {
        post_win_msg_1040_7b3c
                  (CONCAT22(param_2,param_1),param_3,param_4,
                   param_4._2_2_,param_7);
        return;
      }
      puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_8,param_5,param_6);
      iVar3 = puVar4;
      uVar1 = (param_1 + 0x92);
      ui_op_1010_79aa(puVar4,0xfb6,*(long *)(uVar1 + 0x6),param_8);
      if (iVar3 == 0x0) {
        uVar1 = (param_1 + 0x92);
        unk_win_op_1010_7300(puVar4,0x0,0xc,(uVar1 + 0x6));
      }
    }
  }
  if (bVar2) {
    uVar1 = (param_1 + 0x8e);
    (uVar1 + 0xa) = (param_1 + 0x96);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1040_1152(param_1: i16,param_2: u16,uchar *param_3,param_4: i16,param_5: u16,
               param_6: u16)

{
  let uVar1: u16;
  let uVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  let puVar5: *mut u16;
  
  if (*(long *)(param_1 + 0x92) != 0x0) {
    uVar2 = (param_1 + 0x8e);
    uVar1 = (uVar2 + 0xa);
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_3,param_4);
    uVar2 = (param_1 + 0x92);
    uVar4 = (uVar2 >> 0x10);
    iVar3 = uVar2;
    param_5 = 0x1010;
    pass1_1010_ae92(puVar5,uVar1,(iVar3 + 0xa),(iVar3 + 0x6),
                    param_4,param_6);
  }
  destroy_win_1040_7b98(CONCAT22(param_2,param_1),param_5);
  ctx.PTR_LOOP_1050_5b80 = 0x0;
  return;
}



astruct_18 * 
pass1_1040_11ac(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  pass1_1040_0e86(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * 
pass1_1040_123e(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16,uchar *param_6,param_7: i16,param_8: u16)

{
  astruct_717 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfd1,param_5);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_717 *)param_1;
  &iVar1->field_0x8e = 0x0;
  param_1 = 0x17b0;
  iVar1->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x46,param_8,param_6,param_7);
  iVar1->field_0x8e = puVar2;
  iVar1->field_0x90 = (puVar2 >> 0x10);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_1290(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x17b0;
  (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&ctx.PTR_LOOP_1050_1038);
  return;
}


astruct_18 *  pass1_1040_178a(astruct_18 *param_1,param_2: u8)

{
  pass1_1040_1290(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1040_181c(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16,uchar *param_6,param_7: u16)

{
  astruct_433 *iVar1;
  let unaff_DI: i16;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfbb,param_5);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_433 *)param_1;
  &iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x94 = 0x0;
  param_1 = 0x1c48;
  iVar1->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_7,param_6,unaff_DI);
  iVar1->field_0x8e = puVar2;
  iVar1->field_0x90 = (puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_1876(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x1c48;
  (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&ctx.PTR_LOOP_1050_1038);
  return;
}


u32 
pass1_1040_1ab0(param_1: i16,param_2: u16,param_3: u16,param_4: u32,param_5: u16,
               param_6: u16)

{
  let BStack6: bool;
  let uStack4: u16;
  
  BStack6 = 0x0;
  uStack4 = 0x0;
  if (param_4._2_2_ == 0x1831) {
    (param_1 + 0x92) = 0x1;
    (param_1 + 0x94) = 0x1;
    check_dialog_btn_1040_1b8a(param_1,param_2);
  }
  else {
    BStack6 = post_win_msg_1040_7b3c
                        (CONCAT13((param_2 >> 0x8),
                                           CONCAT12(param_2,param_1)),param_3,
                         param_4,param_4._2_2_,param_6);
    uStack4 = param_5;
  }
  return CONCAT22(uStack4,BStack6);
}


astruct_18 *  pass1_1040_1c22(astruct_18 *param_1,param_2: u8)

{
  pass1_1040_1876(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1040_1cb4(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16,uchar *param_6,param_7: i16,param_8: u16)

{
  let puVar1: *mut u8
  astruct_718 *iVar2;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xe8,param_5);
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_718 *)param_1;
  &iVar2->field_0x8e = 0x0;
  &iVar2->field_0x92 = 0x0;
  param_1 = 0x1eee;
  iVar2->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_8,param_6,param_7);
  puVar1 = (puVar3 >> 0x10);
  iVar2->field_0x8e = puVar3;
  iVar2->field_0x90 = puVar1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_8,puVar1,param_7);
  iVar2->field_0x92 = puVar3;
  iVar2->field_0x94 = (puVar3 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_1d24(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x1eee;
  (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&ctx.PTR_LOOP_1050_1038);
  return;
}



u32 
pass1_1040_1e80(param_1: i16,param_2: u16,param_3: u16,param_4: u32,param_5: u16,
               param_6: u16)

{
  let BStack6: bool;
  let uStack4: u16;
  
  BStack6 = 0x0;
  uStack4 = 0x0;
  if (param_4._2_2_ == 0xe4) {
    pass1_1008_a9ec((param_1 + 0x92));
  }
  else {
    BStack6 = post_win_msg_1040_7b3c
                        (CONCAT22(param_2,param_1),param_3,param_4,
                         param_4._2_2_,param_6);
    uStack4 = param_5;
  }
  return CONCAT22(uStack4,BStack6);
}



astruct_18 *  pass1_1040_1ec8(astruct_18 *param_1,param_2: u8)

{
  pass1_1040_1d24(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_1f5a(astruct_57 *param_1,param_2: u16,param_3: i16,param_4: u16)
{
  let piVar1: *mut i16;
  let puVar2: *mut u8
  astruct_719 *iVar3;
  astruct_43 *paVar3;
  let uVar4: u32;
  let puVar5: *mut u16;
  let iVar6: i16;
  let uVar7: u16;
  let iVar8: i16;
  let iVar9: i16;
  let uVar10: u16;
  let local_16: u32;
  let uStack18: u32;
  
  iVar6 = param_1;
  uVar7 = (param_1 >> 0x10);
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfcf,param_2);
  (iVar6 + 0x8e) = 0x0;
  (iVar6 + 0xa2) = 0x0;
  (iVar6 + 0xa6) = 0x0;
  param_1 = 0x237e;
  (iVar6 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  paVar3 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1cc,param_4);
  (iVar6 + 0x8e) = paVar3;
  (iVar6 + 0x90) = (paVar3 >> 0x10);
  uVar4 = pass1_1008_4772((astruct_76 *)
                          (paVar3 & 0xffff0000 | (iVar6 + 0x8e)));
  puVar2 = (uVar4 >> 0x10);
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,puVar2,param_3);
  local_16 = CONCAT22((uVar4 + 0x8) + 0xa,0xa);
  uStack18 = CONCAT22(0x1d6,(uVar4 + 0x4) + -0xa);
  (iVar6 + 0x92) = local_16;
  (iVar6 + 0x96) = uStack18;
  (iVar6 + 0x9a) = local_16;
  (iVar6 + 0x9e) = uStack18;
  piVar1 = (iVar6 + 0x9c);
  *piVar1 = *piVar1 + 0x14;
  iVar9 = iVar6 + 0xa2;
  iVar8 = iVar6 + 0xa6;
  uVar10 = uVar7;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_4,
                           (puVar5 >> 0x10),iVar6 + 0xa2);
  pass1_1010_0538(puVar5,CONCAT22(uVar7,iVar8),
                  CONCAT22(uVar10,iVar9),0x1010,param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_205e(astruct_18 *param_1)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_624 *iVar4;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_624 *)param_1;
  param_1->field_0x0 = 0x237e;
  iVar4->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  puVar1 = iVar4->field_0x8e;
  uVar2 = iVar4->field_0x90;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce(iVar4->field_0xa2,0x1000);
  fn_ptr_1000_17ce(iVar4->field_0xa6,0x1000);
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,iVar4->field_0x6);
  ui_cleanup_op_1040_782c(param_1,&ctx.PTR_LOOP_1050_1038);
  return;
}


astruct_18 *  pass1_1040_2358(astruct_18 *param_1,param_2: u8)

{
  pass1_1040_205e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1040_23ea(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16,param_6: u16,param_7: u16)

{
  let uVar1: u32;
  astruct_436 *iVar2;
  let unaff_DI: i16;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x9a,param_2,0xfbd,param_5);
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_436 *)param_1;
  &iVar2->field_0x8e = 0x0;
  iVar2->field_0x92 = 0x0;
  iVar2->field_0x94 = 0x0;
  param_1 = 0x2956;
  iVar2->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  iVar2->field_0x8a = 0x26;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x6,param_6,param_7,unaff_DI);
  iVar2->field_0x8e = puVar3;
  iVar2->field_0x90 = (puVar3 >> 0x10);
  uVar1 = &iVar2->field_0x8e;
  iVar2->field_0x92 = (uVar1 + 0x28);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_2464(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x2956;
  (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&ctx.PTR_LOOP_1050_1038);
  return;
}


fn pass1_1040_288e(param_1: u32)
{
  let uVar1: u16;
  code **ppcVar2;
  let uVar3: u32;
  let puVar4: u32;
  let puVar5: u32;
  let extraout_DX: *mut u8
  let puVar6: *mut u8
  let extraout_DX_00: *mut u8
  let puVar7: *mut u8
  let iVar8: i16;
  let uVar9: u16;
  
  uVar9 = (param_1 >> 0x10);
  iVar8 = param_1;
  uVar3 = (iVar8 + 0x8e);
  puVar5 = (uVar3 + 0x24);
  ppcVar2 = (code **)(*puVar5 + 0x14);
  (**ppcVar2)();
  puVar4 = puVar5;
  puVar6 = extraout_DX;
  if (*(long *)(iVar8 + 0x70) != 0x0) {
    puVar4 = (iVar8 + 0x70);
    uVar1 = (iVar8 + 0x72);
    puVar6 = (uVar1 | puVar4);
    if (puVar6 != 0x0) {
      ppcVar2 = (code **)*puVar4;
      (**ppcVar2)();
      puVar6 = extraout_DX_00;
    }
  }
  mem_op_1000_179c(0x14,puVar6,0x1000);
  puVar7 = (puVar6 | puVar4);
  if (puVar7 == 0x0) {
    puVar4 = 0x0;
    puVar7 = 0x0;
  }
  else {
    struct_1008_4c58((u16 *)CONCAT22(puVar6,puVar4));
  }
  (iVar8 + 0x70) = puVar4;
  *(uchar **)(iVar8 + 0x72) = puVar7;
  pass1_1008_4d84(*(astruct_90 **)(iVar8 + 0x70),
                  puVar5 & 0xffff | ZEXT24(extraout_DX) << 0x10,puVar7);
  return;
}



astruct_18 *  pass1_1040_2930(astruct_18 *param_1,param_2: u8)

{
  pass1_1040_2464(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * 
pas1_1040_29c2(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
              param_5: u16)

{
  let iVar1: i16;
  let uVar2: u16;
  
  pass1_1040_b0bc(param_1,param_2,CONCAT22(param_3,0x157));
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1 = 0x2e26;
  (iVar1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  load_string_1010_84ac
            (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  (iVar1 + 0x94) = param_4;
  (iVar1 + 0x96) = param_5;
  load_string_1010_84ac
            (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  (iVar1 + 0x98) = param_4;
  (iVar1 + 0x9a) = param_5;
  return param_1;
}



fn pass1_1040_2a22(astruct_18 *param_1)
{
  astruct_625 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_625 *)param_1;
  param_1->field_0x0 = 0x2e26;
  iVar1->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  fn_ptr_1000_17ce(iVar1->field_0x94,0x1000);
  fn_ptr_1000_17ce(iVar1->field_0x98,0x1000);
  unk_draw_op_1040_b0f8(param_1);
  return;
}


fn pass1_1040_2dac(param_1: u32)
{
  let uVar1: u32;
  let uVar2: u32;
  let uStack10: i16;
  
  uVar1 = (param_1 + 0x90);
  uVar2 = struct_op_1030_73a8((uVar1 + 0x6));
  for (iStack10 = 0x0; iStack10 < 0x5; iStack10 += 0x1) {
    pass1_1028_4ab2(uVar2,(&ctx.PTR_LOOP_1050_5d04 + iStack10 * 0xc),
                    (iStack10 * 0xc + 0x5d02));
  }
  return;
}



astruct_18 *  pass1_1040_2e00(astruct_18 *param_1,param_2: u8)

{
  pass1_1040_2a22(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1040_2ea2(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16,uchar *param_6,param_7: i16,param_8: u16)

{
  astruct_720 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0x180,param_5);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_720 *)param_1;
  iVar1->field_0x8e = 0x0;
  iVar1->field_0x90 = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x94 = 0x0;
  &iVar1->field_0x96 = 0x0;
  param_1 = 0x3436;
  iVar1->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3c,param_8,param_6,param_7);
  iVar1->field_0x96 = puVar2;
  iVar1->field_0x98 = (puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_2f06(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x3436;
  (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&ctx.PTR_LOOP_1050_1038);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1040_2f32(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16
               )

{
  let puVar1: *mut u16;
  let iVar2: i16;
  
  iVar2 = 0x0;
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,param_3,param_4);
  pass1_1010_038e(puVar1,iVar2,param_5);
  destroy_win_1040_7b98(CONCAT22(param_2,param_1),0x1010);
  return;
}


astruct_18 *  pass1_1040_3410(astruct_18 *param_1,param_2: u8)

{
  pass1_1040_2f06(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1040_34a2(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16,uchar *param_6,param_7: i16,param_8: u16)

{
  astruct_721 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0x192,param_5);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_721 *)param_1;
  &iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x94 = 0x0;
  iVar1->field_0x96 = 0x0;
  iVar1->field_0x98 = 0x0;
  param_1 = (s_Null_Ptr_1050_38f3 + 0x7);
  iVar1->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3c,param_8,param_6,param_7);
  iVar1->field_0x8e = puVar2;
  iVar1->field_0x90 = (puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_3506(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = (s_Null_Ptr_1050_38f3 + 0x7);
  (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&ctx.PTR_LOOP_1050_1038);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_3532(param_1: u16,param_2: u16,uchar *param_3,param_4: i16,param_5: u16)
{
  let puVar1: *mut u16;
  let iVar2: i16;
  
  iVar2 = 0x0;
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,param_3,param_4);
  pass1_1010_038e(puVar1,iVar2,param_5);
  destroy_win_1040_7b98(CONCAT22(param_2,param_1),0x1010);
  return;
}


astruct_18 *  pass1_1040_38d4(astruct_18 *param_1,param_2: u8)

{
  pass1_1040_3506(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1040_3966(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16,uchar *param_6,param_7: i16,param_8: u16)

{
  astruct_722 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0x185,param_5);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_722 *)param_1;
  &iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x96 = 0x0;
  iVar1->field_0x9a = 0x0;
  iVar1->field_0x9c = 0x0;
  iVar1->field_0x9e = 0x0;
  iVar1->field_0xa0 = 0x0;
  iVar1->field_0xa2 = 0x0;
  iVar1->field_0xa4 = 0x5;
  param_1 = 0x3ffc;
  iVar1->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3c,param_8,param_6,param_7);
  iVar1->field_0x8e = puVar2;
  iVar1->field_0x90 = (puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_39e2(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x3ffc;
  (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&ctx.PTR_LOOP_1050_1038);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_3a0e(param_1: u16,param_2: u16,uchar *param_3,param_4: i16,param_5: u16)
{
  let puVar1: *mut u16;
  let iVar2: i16;
  
  iVar2 = 0x0;
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,param_3,param_4);
  pass1_1010_038e(puVar1,iVar2,param_5);
  destroy_win_1040_7b98(CONCAT22(param_2,param_1),0x1010);
  return;
}



astruct_18 *  pass1_1040_3fd6(astruct_18 *param_1,param_2: u8)

{
  pass1_1040_39e2(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1040_4068(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16,uchar *param_6,param_7: i16,param_8: u16)

{
  let puVar1: *mut u8
  astruct_723 *iVar2;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfb7,param_5);
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_723 *)param_1;
  &iVar2->field_0x8e = 0x0;
  iVar2->field_0x92 = 0x0;
  iVar2->field_0x9a = 0x0;
  param_1 = 0x4466;
  iVar2->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  iVar2->field_0x76 = 0x1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_8,param_6,param_7);
  puVar1 = (puVar3 >> 0x10);
  iVar2->field_0x8e = puVar3;
  iVar2->field_0x90 = puVar1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x29,param_8,puVar1,param_7);
  iVar2->field_0x96 = puVar3;
  iVar2->field_0x98 = (puVar3 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_40e2(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x4466;
  (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&ctx.PTR_LOOP_1050_1038);
  return;
}


astruct_18 *  pass1_1040_4440(astruct_18 *param_1,param_2: u8)

{
  pass1_1040_40e2(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void 
pass1_1040_44d2(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               uchar *param_5)

{
  let uVar1: u32;
  let uVar2: u16;
  let puVar3: *mut u8
  let iVar4: i16;
  let uVar5: u16;
  let iVar6: i16;
  let uVar7: u16;
  let piStack8: *mut i16;
  
  iVar6 = param_1;
  uVar7 = (param_1 >> 0x10);
  struct_1040_b082(param_1,CONCAT22(param_3,0xfa2));
  param_1 = 0x4824;
  (iVar6 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  mem_op_1000_179c(0x18,param_5,0x1000);
  puVar3 = (param_5 | param_4);
  if (puVar3 == 0x0) {
    (iVar6 + 0x90) = 0x0;
  }
  else {
    struct_1040_a598((u16 *)CONCAT22(param_5,param_4));
    (iVar6 + 0x90) = param_4;
    *(uchar **)(iVar6 + 0x92) = puVar3;
  }
  (iVar6 + 0x90) = 0x14;
  iVar4 = *(iVar6 + 0x90);
  uVar2 = iVar4 * 0xa + 0x2;
  mem_op_1000_179c(uVar2,puVar3,0x1000);
  piStack8 = CONCAT22(puVar3,uVar2);
  if ((puVar3 | uVar2) == 0x0) {
    uVar1 = (iVar6 + 0x90);
    (uVar1 + 0x2) = 0x0;
  }
  else {
    *piStack8 = iVar4;
    pass1_1000_5586((uchar *)0xa564,&ctx.PTR_LOOP_1050_1040,iVar4,0xa,uVar2 + 0x2,
                    puVar3);
    uVar1 = (iVar6 + 0x90);
    uVar5 = (uVar1 >> 0x10);
    iVar4 = uVar1;
    (iVar4 + 0x2) = uVar2 + 0x2;
    *(uchar **)(iVar4 + 0x4) = puVar3;
  }
  uVar1 = (iVar6 + 0x90);
  (uVar1 + 0x6) = param_2;
  uVar1 = (iVar6 + 0x90);
  (uVar1 + 0xa) = 0x1;
  uVar1 = (iVar6 + 0x90);
  (uVar1 + 0x12) = (iVar6 + 0xa);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1040_45e8(param_1: i16,param_2: u16,param_3: u16,param_4: u32,uchar *param_5,
               param_6: u16,param_7: u16)

{
  astruct_18 *paVar1;
  code **ppcVar2;
  let uVar3: u32;
  let uVar4: u16;
  astruct_18 *paVar5;
  let puVar6: *mut u8
  let puVar7: *mut u8
  let iVar8: i16;
  let unaff_DI: i16;
  let uVar9: u16;
  astruct_20 *paVar10;
  let piStack16: *mut i16;
  
  if (param_4._2_2_ != 0xeb) {
    pass1_1040_b54a(param_1,param_2,param_3,param_4,param_5,param_6,param_7);
    return;
  }
  paVar10 = (astruct_20 *)
            mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_7,param_5,unaff_DI);
  puVar6 = (paVar10 >> 0x10);
  paVar1 = *(astruct_18 **)(param_1 + 0x90);
  if (paVar1 != (astruct_18 *)0x0) {
    paVar5 = paVar1;
    mem_op_1000_179c(0x18,puVar6,0x1000);
    uVar4 = paVar5;
    puVar7 = (puVar6 | uVar4);
    if (puVar7 == 0x0) {
      uVar4 = 0x0;
      puVar7 = 0x0;
    }
    else {
      struct_1040_a598((u16 *)(paVar5 & 0xffff | ZEXT24(puVar6) << 0x10));
    }
    (param_1 + 0x90) = uVar4;
    *(uchar **)(param_1 + 0x92) = puVar7;
    (param_1 + 0x90) = 0x14;
    iVar8 = *(param_1 + 0x90);
    uVar4 = iVar8 * 0xa + 0x2;
    mem_op_1000_179c(uVar4,puVar7,0x1000);
    piStack16 = CONCAT22(puVar7,uVar4);
    if ((puVar7 | uVar4) == 0x0) {
      uVar3 = (param_1 + 0x90);
      (uVar3 + 0x2) = 0x0;
    }
    else {
      *piStack16 = iVar8;
      pass1_1000_5586((uchar *)0xa564,&ctx.PTR_LOOP_1050_1040,iVar8,0xa,uVar4 + 0x2,
                      puVar7);
      uVar3 = (param_1 + 0x90);
      uVar9 = (uVar3 >> 0x10);
      iVar8 = uVar3;
      (iVar8 + 0x2) = uVar4 + 0x2;
      *(uchar **)(iVar8 + 0x4) = puVar7;
    }
    uVar3 = (param_1 + 0x90);
    (uVar3 + 0x6) = (paVar1 + 0x6);
    uVar3 = (param_1 + 0x90);
    (uVar3 + 0xa) = 0x1;
    uVar3 = (param_1 + 0x90);
    (uVar3 + 0x12) = (param_1 + 0xa);
    pass1_1010_a50c(paVar10,0x10505d40,(param_1 + 0x90));
    if (paVar1 != (astruct_18 *)0x0) {
      pass1_1040_a5d0(paVar1);
      fn_ptr_1000_17ce(paVar1,0x1000);
    }
    ppcVar2 = (code **)(CONCAT22(param_2,param_1) + 0x70);
    (**ppcVar2)();
  }
  return;
}



fn pass1_1040_4766(param_1: *mut u32)
{
  code **ppcVar1;
  
  ppcVar1 = (code **)(*param_1 + 0x74);
  (**ppcVar1)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_477e(astruct_1 *param_1,uchar *param_2,param_3: u16,param_4: u16)
{
  let puVar1: *mut u8
  let pUVar2: *mut u16;
  let puVar3: *mut u8
  let puVar4: *mut u8
  let unaff_DI: i16;
  let puVar5: *mut u16;
  let uVar6: u16;
  let uVar7: u16;
  
  unk_win_ui_op_1040_b230(param_1,param_3,param_4);
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_4,param_2,unaff_DI);
  puVar3 = (puVar5 >> 0x10);
  uVar7 = SUB42(ctx.data_seg,0x0);
  uVar6 = 0x5d68;
  puVar1 = pass1_1008_5fd8(param_4,puVar3);
  puVar4 = puVar3;
  pUVar2 = pass1_1000_3cea(CONCAT22(puVar3,puVar1),CONCAT22(uVar7,uVar6));
  pass1_1010_e964(puVar4,param_4,unaff_DI);
  pass1_1000_3cea(CONCAT22(puVar3,puVar1),CONCAT22(puVar4,pUVar2));
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | (param_1 + 0x10)),
             CONCAT22(puVar3,puVar1));
  fn_ptr_1000_17ce((astruct_18 *)CONCAT22(puVar3,puVar1),0x1000);
  return;
}



astruct_18 *  pass1_1040_47fe(astruct_18 *param_1,param_2: u8)

{
  unk_draw_op_1040_b0f8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1040_48a0(astruct_57 *param_1,param_2: u16,param_3: u32,param_4: u16,
               uchar *param_5,param_6: u16)

{
  let iVar1: i16;
  let piVar2: *mut i16;
  let uVar3: u16;
  let puVar4: *mut u8
  let puVar5: *mut u8
  astruct_444 *iVar5;
  astruct_445 *iVar6;
  let unaff_DI: i16;
  let uVar6: u16;
  let uVar7: u16;
  let puVar8: *mut u16;
  let piStack8: *mut i16;
  
  struct_1040_b082(param_1,CONCAT22(param_4,0xfa1));
  uVar6 = (param_1 >> 0x10);
  iVar5 = (astruct_444 *)param_1;
  iVar5->field_0x94 = 0x0;
  param_1 = &ctx.PTR_LOOP_1050_4e18;
  iVar5->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_5,unaff_DI);
  puVar4 = (puVar8 >> 0x10);
  uVar3 = puVar8;
  &iVar5->field_0x94 = uVar3;
  *(uchar **)(&iVar5->field_0x94 + 0x2) = puVar4;
  mem_op_1000_179c(0x18,puVar4,0x1000);
  puVar5 = (puVar4 | uVar3);
  if (puVar5 == 0x0) {
    iVar5->field_0x90 = 0x0;
  }
  else {
    struct_1040_a598((u16 *)CONCAT22(puVar4,uVar3));
    &iVar5->field_0x90 = uVar3;
    *(uchar **)(&iVar5->field_0x90 + 0x2) = puVar5;
  }
  *iVar5->field_0x90 = 0x7;
  iVar1 = *iVar5->field_0x90;
  uVar3 = iVar1 * 0xa + 0x2;
  mem_op_1000_179c(uVar3,puVar5,0x1000);
  piStack8 = CONCAT22(puVar5,uVar3);
  if ((puVar5 | uVar3) == 0x0) {
    piVar2 = iVar5->field_0x90;
    (piVar2 + 0x2) = 0x0;
  }
  else {
    *piStack8 = iVar1;
    pass1_1000_5586((uchar *)0xa564,&ctx.PTR_LOOP_1050_1040,iVar1,0xa,uVar3 + 0x2,
                    puVar5);
    piVar2 = iVar5->field_0x90;
    uVar7 = (piVar2 >> 0x10);
    iVar6 = (astruct_445 *)piVar2;
    iVar6->field_0x2 = uVar3 + 0x2;
    iVar6->field_0x4 = puVar5;
  }
  piVar2 = iVar5->field_0x90;
  (piVar2 + 0x6) = param_3;
  piVar2 = iVar5->field_0x90;
  (piVar2 + 0xa) = param_2;
  piVar2 = iVar5->field_0x90;
  (piVar2 + 0x12) = iVar5->field_0xa;
  iVar1 = &iVar5->field_0x90;
  uVar7 = (&iVar5->field_0x90 + 0x2);
  pass1_1010_debe(iVar5->field_0x94,(iVar1 + 0xa),
                  CONCAT22(uVar7,iVar1 + 0x10),
                  CONCAT22(uVar7,iVar1 + 0xc),param_3,param_6);
  return;
}


fn pass1_1040_4cf4(param_1: u32,HWND16 param_2,param_3: u16)
{
  let uVar1: u32;
  let uVar2: u32;
  let uVar3: u16;
  let iVar4: i16;
  let iVar5: i16;
  let unaff_DI: i16;
  let uVar6: u16;
  let uVar7: u16;
  LRESULT LVar8;
  let local_52: [u8;50];
  
  uVar6 = (param_1 >> 0x10);
  iVar4 = param_1;
  GetDlgItem16(param_2,0x1770);
  LVar8 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x4070000);
  uVar3 = (LVar8 >> 0x10);
  if (LVar8 != -0x1) {
    LVar8 = SendMessage16((HWND16)s_tile2_bmp_1050_1538,local_52,param_3,
                          CONCAT22(0x408,LVar8));
    uVar3 = (LVar8 >> 0x10);
  }
  uVar2 = (iVar4 + 0x90);
  uVar1 = (iVar4 + 0x94);
  uVar3 = pass1_1010_ae12(uVar1,(uVar1 >> 0x10),
                          CONCAT22(param_3,local_52),(uVar2 + 0xa),uVar3);
  if (uVar3 != 0xffff) {
    uVar1 = (iVar4 + 0x90);
    uVar7 = (uVar1 >> 0x10);
    iVar5 = uVar1;
    pass1_1010_ae92((iVar4 + 0x94),uVar3,(iVar5 + 0xa),
                    (iVar5 + 0x6),unaff_DI,param_3);
  }
  return;
}



fn pass1_1040_4d7e(param_1: u32)
{
  let uVar1: u32;
  let piVar2: *mut i16;
  let uVar3: u16;
  let iStack8: i16;
  let puStack6: u32;
  
  uVar3 = (param_1 >> 0x10);
  uVar1 = (param_1 + 0x90);
  puStack6 = *(u32 **)(uVar1 + 0x2);
  iStack8 = 0x0;
  while ((piVar2 = (param_1 + 0x90),
         *piVar2 != iStack8 && iStack8 <= *piVar2 &&
         ((puStack6 + 0x4) != 0x1770))) {
    iStack8 += 0x1;
    puStack6 = (puStack6 & 0xffff0000 | (puStack6 + 0xa));
  }
  pass1_1000_3e2c(*puStack6);
  return;
}



fn pass1_1040_4dcc(param_1: u32,param_2: i16,param_3: u16) -> *mut u8

{
  let uVar1: u32;
  let uVar2: u32;
  let uVar3: u16;
  char *pcVar4;
  
  uVar3 = (param_1 >> 0x10);
  uVar2 = (param_1 + 0x90);
  uVar1 = (param_1 + 0x94);
  pcVar4 = string_op_1010_ada6(0x1010,param_3,uVar1,(uVar1 >> 0x10)
                               ,param_2,(uVar2 + 0xa));
  return pcVar4;
}



astruct_18 *  pass1_1040_4df2(astruct_18 *param_1,param_2: u8)

{
  unk_draw_op_1040_b0f8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1040_4e94(astruct_57 *param_1,param_2: i32,param_3: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  let uVar3: u16;
  
  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_3,0xfab));
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x94) = 0x0;
  (iVar1 + 0x98) = 0x0;
  (iVar1 + 0xb0) = 0x0;
  (iVar1 + 0xb4) = 0x0;
  (iVar1 + 0xb6) = 0x0;
  param_1 = 0x55a2;
  (iVar1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  if (param_2 != 0x0) {
    uVar3 = (param_2 >> 0x10);
    (iVar1 + 0xb0) = (param_2 + 0x6);
    (iVar1 + 0xb4) = (param_2 + 0x14);
  }
  return;
}



fn pass1_1040_4f0a(astruct_18 *param_1)
{
  param_1->field_0x0 = 0x55a2;
  (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  unk_draw_op_1040_b0f8(param_1);
  return;
}



u16 
pass1_1040_4f28(param_1: *mut u32,i16 *param_2,param_3: u16,param_4: u16,param_5: i16,
               param_6: u16)

{
  code **ppcVar1;
  let uVar2: u16;
  
  if (param_5 == 0x2b) {
    if (*param_2 == 0x4) {
      win_ui_get_prop_op_1040_9566(CONCAT22(param_3,param_2),param_6);
    }
  }
  else {
    if (param_5 != 0x111) {
      uVar2 = pass1_1040_b316(param_1,param_2,param_3,param_4,param_5);
      return uVar2;
    }
    ppcVar1 = (code **)(*param_1 + 0x7c);
    (**ppcVar1)(param_6,param_1,param_2,CONCAT22(param_4,param_3));
  }
  return 0x1;
}



fn pass1_1040_4f82(param_1: *mut u32)
{
  code **ppcVar1;
  
  ppcVar1 = (code **)(*param_1 + 0x80);
  (**ppcVar1)();
  return;
}


fn pass1_1040_5238(param_1: u32) -> u16

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((param_1 + 0x94) + 0x8);
  (**ppcVar1)();
  return 0x0;
}


astruct_18 *  pass1_1040_557c(astruct_18 *param_1,param_2: u8)

{
  pass1_1040_4f0a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1040_5626(astruct_57 *param_1,param_2: u32,param_3: u16,uchar *param_4)
{
  let piVar1: *mut i16;
  let uVar2: u16;
  let puVar3: *mut u8
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u32;
  let uVar7: u16;
  let piStack12: *mut i16;
  astruct_441 *iVar8;
  astruct_396 *iVar7;
  astruct_439 *iVar6;
  
  iVar8 = (astruct_441 *)param_1;
  uVar7 = (param_1 >> 0x10);
  struct_1040_b082(param_1,CONCAT22(param_3,0xfa3));
  uVar2 = 0x0;
  iVar8->field_0x94 = 0x0;
  iVar8->field_0x96 = 0x0;
  iVar8->field_0x98 = 0x0;
  iVar8->field_0x9c = 0x0;
  param_1 = 0x6386;
  iVar8->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  mem_op_1000_179c(0x18,param_4,0x1000);
  puVar3 = (param_4 | uVar2);
  if (puVar3 == 0x0) {
    iVar8->field_0x90 = 0x0;
  }
  else {
    struct_1040_a598((u16 *)CONCAT22(param_4,uVar2));
    &iVar8->field_0x90 = uVar2;
    *(uchar **)(&iVar8->field_0x90 + 0x2) = puVar3;
  }
  *iVar8->field_0x90 = 0x6;
  iVar4 = *iVar8->field_0x90;
  uVar2 = iVar4 * 0xa + 0x2;
  mem_op_1000_179c(uVar2,puVar3,0x1000);
  piStack12 = CONCAT22(puVar3,uVar2);
  if ((puVar3 | uVar2) == 0x0) {
    piVar1 = iVar8->field_0x90;
    (piVar1 + 0x2) = 0x0;
  }
  else {
    *piStack12 = iVar4;
    pass1_1000_5586((uchar *)0xa564,&ctx.PTR_LOOP_1050_1040,iVar4,0xa,uVar2 + 0x2,
                    puVar3);
    piVar1 = iVar8->field_0x90;
    uVar5 = (piVar1 >> 0x10);
    iVar4 = piVar1;
    (iVar4 + 0x2) = uVar2 + 0x2;
    *(uchar **)(iVar4 + 0x4) = puVar3;
  }
  piVar1 = iVar8->field_0x90;
  (piVar1 + 0x6) = param_2;
  piVar1 = iVar8->field_0x90;
  (piVar1 + 0xa) = 0x4;
  piVar1 = iVar8->field_0x90;
  (piVar1 + 0x12) = iVar8->field_0xa;
  uVar6 = pass1_1040_5d12(param_1);
  uVar2 = (uVar6 >> 0x10);
  if ((uVar2 | uVar6) == 0x0) {
    iVar8->field_0x9a = 0x0;
  }
  else {
    iVar8->field_0x9a = (uVar6 + 0x20);
  }
  return;
}



void 
pass1_1040_57d4(astruct_1 *param_1,uchar *param_2,param_3: i16,param_4: u16,
               param_5: u16)

{
  pass1_1040_5d42(param_1);
  pass1_1040_5eaa(param_1);
  pass1_1040_5dc4(param_1,param_2,param_3,param_5);
  unk_win_ui_op_1040_b230(param_1,param_4,param_5);
  return;
}


fn pass1_1040_5cd6(param_1: u32) -> u16

{
  let iVar1: i16;
  let uVar2: u16;
  let uVar3: u32;
  
  uVar3 = pass1_1040_5d12(param_1);
  if (uVar3 != 0x0) {
    iVar1 = (uVar3 + 0x20);
    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x9a) != iVar1) {
      (param_1 + 0x9a) = iVar1;
      return 0x1;
    }
  }
  return 0x0;
}



fn pass1_1040_5d12(param_1: u32) -> u32

{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u32;
  astruct_440 *iVar4;
  let uVar4: u16;
  let uVar5: u32;
  
  uVar3 = (param_1 + 0x90);
  uVar4 = (uVar3 >> 0x10);
  iVar4 = (astruct_440 *)uVar3;
  uVar1 = iVar4->field_0x6;
  uVar2 = iVar4->field_0x8;
  if ((uVar2 | uVar1) != 0x0) {
    uVar5 = struct_op_1030_73a8(CONCAT22(uVar2,uVar1));
    return uVar5;
  }
  return 0x0;
}



fn pass1_1040_5d42(param_1: u32)
{
  let uVar1: u16;
  let cVar2: u8;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u32;
  
  uVar5 = pass1_1040_5d12(param_1);
  if (uVar5 != 0x0) {
    uVar1 = (uVar5 + 0xc);
    iVar3 = param_1;
    uVar4 = (param_1 >> 0x10);
    if (uVar1 == 0x5f) {
      (iVar3 + 0x96) = 0x53;
      return;
    }
    if (uVar1 < 0x60) {
      cVar2 = uVar1;
      if (cVar2 == '(') {
        (iVar3 + 0x96) = 0x54;
        return;
      }
      if (cVar2 == ')') {
        (iVar3 + 0x96) = 0x55;
        return;
      }
      if (cVar2 == ']') {
        (iVar3 + 0x96) = 0x51;
        return;
      }
      if (cVar2 == '^') {
        (iVar3 + 0x96) = 0x52;
        return;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_5dc4(param_1: u32,uchar *param_2,param_3: i16,param_4: u16)
{
  code **ppcVar1;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let puVar6: *mut u8
  let extraout_DX: u16;
  astruct_724 *iVar7;
  let uVar7: u16;
  let puVar8: *mut u16;
  let puVar9: u32;
  let uVar10: u16;
  let iStack18: i16;
  
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_4,param_2,param_3);
  puVar6 = (puVar8 >> 0x10);
  uVar3 = puVar8;
  uVar7 = (param_1 >> 0x10);
  iVar7 = (astruct_724 *)param_1;
  pass1_1010_a5ca(uVar3,puVar6,iVar7->field_0x9a,uVar3,puVar6);
  if (uVar3 == 0x0) {
    iVar7->field_0x94 = 0x0;
    iVar7->field_0x9c = 0x1;
  }
  if (-0x1 < uVar3) {
    if (iVar7->field_0x9a < 0x72) {
      uVar10 = 0x31;
    }
    else {
      uVar10 = 0x41;
    }
    puVar9 = 
             mixed_1010_20ba(_PTR_LOOP_1050_0ed0,uVar10,param_4,puVar6,param_3);
    uVar4 = iVar7->field_0x9a;
    ppcVar1 = (code **)(*puVar9 + 0x14);
    (**ppcVar1)(0x1010,puVar9,(puVar9 >> 0x10),uVar4,uVar4 >> 0xf);
    if ((extraout_DX | uVar4) == 0x0) {
      iStack18 = 0x0;
    }
    else {
      uVar2 = (uVar4 + 0x16);
      iStack18 = (uVar2 + 0x4);
    }
    if ((iStack18 != 0x0) && (uVar3 != 0x0)) {
      uVar5 = ((iStack18 - uVar3) * 0x64) / iStack18;
      uVar4 = uVar5 / 0xa;
      iVar7->field_0x94 = uVar4;
      if (0x4 < uVar5 % 0xa) {
        iVar7->field_0x94 = uVar4 + 0x1;
      }
    }
  }
  return;
}



i16  pass1_1040_5eaa(param_1: u32)

{
  let iVar1: i16;
  let iVar2: i16;
  let iVar3: i16;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  iVar1 = (iVar3 + 0x9a);
  iVar2 = iVar1;
  if (true) {
    iVar2 = iVar3;
    switch(iVar1) {
    case 0x0:
    case 0x70:
    case 0x71:
      (iVar3 + 0x98) = 0x0;
      return iVar3;
    case 0x1:
    case 0x2:
      (iVar3 + 0x98) = 0xd;
      return iVar3;
    case 0x3:
      (iVar3 + 0x98) = 0xe;
      return iVar3;
    case 0x4:
    case 0x4b:
      (iVar3 + 0x98) = 0xf;
      break;
    case 0x5:
      (iVar3 + 0x98) = 0x10;
      return iVar3;
    case 0x6:
      (iVar3 + 0x98) = 0x11;
      return iVar3;
    case 0x7:
      (iVar3 + 0x98) = 0x12;
      break;
    case 0x8:
      (iVar3 + 0x98) = 0x13;
      break;
    case 0x9:
    case 0xa:
    case 0xb:
      (iVar3 + 0x98) = 0x14;
      break;
    case 0xc:
      (iVar3 + 0x98) = 0x18;
      break;
    case 0xd:
      (iVar3 + 0x98) = 0x19;
      break;
    case 0xe:
    case 0x76:
      (iVar3 + 0x98) = 0x17;
      break;
    case 0xf:
    case 0x10:
    case 0x11:
      (iVar3 + 0x98) = 0x1a;
      break;
    case 0x12:
      (iVar3 + 0x98) = 0x1b;
      break;
    case 0x13:
      (iVar3 + 0x98) = 0x1c;
      break;
    case 0x14:
      (iVar3 + 0x98) = 0x1d;
      break;
    case 0x15:
    case 0x16:
    case 0x17:
    case 0x18:
    case 0x19:
      (iVar3 + 0x98) = 0x1e;
      break;
    case 0x1a:
      (iVar3 + 0x98) = 0x1f;
      break;
    case 0x1b:
      (iVar3 + 0x98) = 0x20;
      break;
    case 0x1c:
    case 0x1d:
    case 0x1e:
      (iVar3 + 0x98) = 0x21;
      break;
    case 0x1f:
      (iVar3 + 0x98) = 0x22;
      break;
    case 0x20:
      (iVar3 + 0x98) = 0x23;
      break;
    case 0x21:
      (iVar3 + 0x98) = 0x24;
      break;
    case 0x22:
      (iVar3 + 0x98) = 0x25;
      break;
    case 0x23:
    case 0x24:
    case 0x25:
    case 0x26:
    case 0x27:
    case 0x28:
    case 0x29:
    case 0x2a:
    case 0x2b:
      (iVar3 + 0x98) = 0x26;
      break;
    case 0x2c:
      (iVar3 + 0x98) = 0x27;
      break;
    case 0x2d:
      (iVar3 + 0x98) = 0x28;
      break;
    case 0x2e:
    case 0x2f:
    case 0x30:
    case 0x31:
      (iVar3 + 0x98) = 0x29;
      break;
    case 0x32:
    case 0x33:
    case 0x34:
    case 0x35:
    case 0x4d:
      (iVar3 + 0x98) = 0x2a;
      break;
    case 0x36:
      (iVar3 + 0x98) = 0x2b;
      break;
    case 0x37:
    case 0x38:
    case 0x39:
      (iVar3 + 0x98) = 0x2c;
      break;
    case 0x3a:
      (iVar3 + 0x98) = 0x2d;
      break;
    case 0x3b:
    case 0x3c:
      (iVar3 + 0x98) = 0x2e;
      break;
    case 0x3d:
      (iVar3 + 0x98) = 0x2f;
      break;
    case 0x3e:
      (iVar3 + 0x98) = 0x30;
      break;
    case 0x3f:
      (iVar3 + 0x98) = 0x31;
      break;
    case 0x40:
      (iVar3 + 0x98) = 0x32;
      break;
    case 0x41:
      (iVar3 + 0x98) = 0x33;
      break;
    case 0x42:
      (iVar3 + 0x98) = 0x34;
      break;
    case 0x43:
      (iVar3 + 0x98) = 0x35;
      break;
    case 0x44:
      (iVar3 + 0x98) = 0x36;
      break;
    case 0x45:
      (iVar3 + 0x98) = 0x37;
      break;
    case 0x46:
      (iVar3 + 0x98) = 0x38;
      break;
    case 0x47:
      (iVar3 + 0x98) = 0x39;
      break;
    case 0x48:
    case 0x49:
    case 0x4a:
      (iVar3 + 0x98) = 0x3a;
      break;
    case 0x4c:
      (iVar3 + 0x98) = 0x3b;
      break;
    case 0x4e:
      (iVar3 + 0x98) = 0x3c;
      break;
    case 0x4f:
    case 0x50:
      (iVar3 + 0x98) = 0x3d;
      break;
    case 0x51:
    case 0x52:
    case 0x53:
    case 0x54:
    case 0x55:
      (iVar3 + 0x98) = 0x3e;
      break;
    case 0x56:
    case 0x57:
    case 0x58:
    case 0x59:
    case 0x5a:
      (iVar3 + 0x98) = 0x3f;
      break;
    case 0x5b:
      (iVar3 + 0x98) = 0x40;
      break;
    case 0x5c:
    case 0x5d:
    case 0x5e:
      (iVar3 + 0x98) = 0x41;
      break;
    case 0x5f:
    case 0x60:
    case 0x61:
      (iVar3 + 0x98) = 0x42;
      break;
    case 0x62:
    case 0x63:
    case 0x64:
    case 0x65:
    case 0x66:
      (iVar3 + 0x98) = 0x43;
      break;
    case 0x67:
    case 0x68:
      (iVar3 + 0x98) = 0x44;
      break;
    case 0x69:
      (iVar3 + 0x98) = 0x45;
      break;
    case 0x6a:
      (iVar3 + 0x98) = 0x46;
      break;
    case 0x6b:
      (iVar3 + 0x98) = 0x47;
      break;
    case 0x6c:
      (iVar3 + 0x98) = 0x48;
      break;
    case 0x6d:
      (iVar3 + 0x98) = 0x49;
      break;
    case 0x6e:
      (iVar3 + 0x98) = 0x4a;
      break;
    case 0x6f:
      (iVar3 + 0x98) = 0x4b;
      break;
    case 0x74:
      (iVar3 + 0x98) = 0x15;
      break;
    case 0x75:
      (iVar3 + 0x98) = 0x16;
      break;
    default:
      (iVar3 + 0x98) = 0x4c;
    }
  }
  return iVar2;
}



astruct_18 *  pass1_1040_6360(astruct_18 *param_1,param_2: u8)

{
  unk_draw_op_1040_b0f8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1040_6402(astruct_57 *param_1,param_2: u16,uchar *param_3,param_4: i16,
               param_5: u16)

{
  code **ppcVar1;
  astruct_725 *iVar2;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x1850,param_2);
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_725 *)param_1;
  iVar2->field_0x8e = 0x0;
  iVar2->field_0x92 = 0x0;
  param_1 = 0x67ba;
  iVar2->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,param_3,param_4);
  &iVar2->field_0x92 = puVar3;
  (&iVar2->field_0x92 + 0x2) = (puVar3 >> 0x10);
  ppcVar1 = (code **)(*iVar2->field_0x92 + 0x4);
  (**ppcVar1)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_6470(astruct_18 *param_1,param_2: u16)
{
  astruct_18 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_18 *)param_1;
  param_1->field_0x0 = 0x67ba;
  iVar1->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  if (*(long *)&iVar1->field_0x92 != 0x0) {
    pass1_1010_1ea6(&iVar1->field_0x92,(long)param_1,param_2);
  }
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,iVar1->field_0x6);
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar1->field_0x8e,0x1000);
  ui_cleanup_op_1040_782c(param_1,0x1000);
  return;
}


astruct_18 * 
pass1_1040_6794(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  pass1_1040_6470(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1040_6826(astruct_57 *param_1,param_2: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_2,0xfda));
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x94) = 0x0;
  (iVar1 + 0x98) = 0x0;
  param_1 = 0x6f32;
  (iVar1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  return;
}



fn pass1_1040_6862(astruct_18 *param_1)
{
  param_1->field_0x0 = 0x6f32;
  (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  unk_draw_op_1040_b0f8(param_1);
  return;
}



u16 
pass1_1040_68d2(param_1: *mut u32,i16 *param_2,param_3: u16,param_4: u16,param_5: i16,
               param_6: u16)

{
  code **ppcVar1;
  let uVar2: u16;
  
  if (param_5 == 0x2b) {
    if (*param_2 == 0x4) {
      win_ui_get_prop_op_1040_9566(CONCAT22(param_3,param_2),param_6);
    }
  }
  else {
    if (param_5 != 0x111) {
      uVar2 = pass1_1040_b316(param_1,param_2,param_3,param_4,param_5);
      return uVar2;
    }
    ppcVar1 = (code **)(*param_1 + 0x80);
    (**ppcVar1)(param_6,param_1,param_2,CONCAT22(param_4,param_3));
  }
  return 0x1;
}



fn pass1_1040_692e(param_1: *mut u32)
{
  code **ppcVar1;
  
  ppcVar1 = (code **)(*param_1 + 0x7c);
  (**ppcVar1)();
  return;
}


fn pass1_1040_6cac(param_1: u32,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  pass1_1010_1ea6((iVar4 + 0x94),param_1 & 0xffff | uVar5 << 0x10,param_2
                 );
  puVar1 = (iVar4 + 0x98);
  uVar2 = (iVar4 + 0x9a);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1010,puVar1,uVar2,0x1);
  }
  (iVar4 + 0x98) = 0x0;
  (iVar4 + 0x94) = 0x0;
  return;
}



fn pass1_1040_6cfa(param_1: u32) -> u16

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((param_1 + 0x98) + 0x8);
  (**ppcVar1)();
  return 0x1;
}


astruct_18 *  pass1_1040_6f0c(astruct_18 *param_1,param_2: u8)

{
  pass1_1040_6862(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1040_6fb6(astruct_57 *param_1,param_2: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_2,0xfd9));
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x94) = 0x0;
  (iVar1 + 0x98) = 0x0;
  param_1 = 0x76a4;
  (iVar1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  return;
}


u16 
pass1_1040_7044(param_1: *mut u32,i16 *param_2,param_3: u16,param_4: u16,param_5: i16,
               param_6: u16)

{
  code **ppcVar1;
  let uVar2: u16;
  
  if (param_5 == 0x2b) {
    if (*param_2 == 0x4) {
      win_ui_get_prop_op_1040_9566(CONCAT22(param_3,param_2),param_6);
    }
  }
  else {
    if (param_5 != 0x111) {
      uVar2 = pass1_1040_b316(param_1,param_2,param_3,param_4,param_5);
      return uVar2;
    }
    ppcVar1 = (code **)(*param_1 + 0x80);
    (**ppcVar1)(param_6,param_1,param_2,CONCAT22(param_4,param_3));
  }
  return 0x1;
}



fn pass1_1040_70a0(param_1: *mut u32)
{
  code **ppcVar1;
  
  ppcVar1 = (code **)(*param_1 + 0x7c);
  (**ppcVar1)();
  return;
}


fn pass1_1040_741e(param_1: u32,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  pass1_1010_1ea6((iVar4 + 0x94),param_1 & 0xffff | uVar5 << 0x10,param_2
                 );
  puVar1 = (iVar4 + 0x98);
  uVar2 = (iVar4 + 0x9a);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1010,puVar1,uVar2,0x1);
  }
  (iVar4 + 0x98) = 0x0;
  (iVar4 + 0x94) = 0x0;
  return;
}



fn pass1_1040_746c(param_1: u32) -> u16

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((param_1 + 0x98) + 0x8);
  (**ppcVar1)();
  return 0x1;
}


astruct_18 *  pass1_1040_767e(astruct_18 *param_1,param_2: u8)

{
  unk_draw_op_1040_b0f8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1040_78de(void)
{
  return;
}


fn pass1_1040_79c0(param_1: *mut u32,i16 *param_2,param_3: u16,param_4: u16,param_5: u16) -> u16

{
  code **ppcVar1;
  let cVar2: u8;
  let uVar3: u16;
  let unaff_CS: u16;
  
  if (param_5 == 0xa1) {
    ppcVar1 = (code **)(*param_1 + 0x38);
    uVar3 = (**ppcVar1)();
    return uVar3;
  }
  if (param_5 < 0xa2) {
    if (param_5 == 0x85) {
      ppcVar1 = (code **)(*param_1 + 0x1c);
      (**ppcVar1)();
      return 0x1;
    }
    if (param_5 < 0x86) {
      cVar2 = param_5;
      if (cVar2 == '\x02') {
        ppcVar1 = (code **)(*param_1 + 0x24);
        (**ppcVar1)();
        return 0x1;
      }
      if (cVar2 == '\f') {
        ppcVar1 = (code **)(*param_1 + 0x18);
        (**ppcVar1)();
        return 0x1;
      }
      if (cVar2 == '\x0f') {
        ppcVar1 = (code **)(*param_1 + 0x60);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (cVar2 == '+') {
        if (*param_2 != 0x4) {
          return 0x1;
        }
        win_ui_get_prop_op_1040_9566(CONCAT22(param_3,param_2),unaff_CS);
        return 0x1;
      }
    }
  }
  else {
    if (param_5 == 0x114) {
      ppcVar1 = (code **)(*param_1 + 0x58);
      uVar3 = (**ppcVar1)();
      return uVar3;
    }
    if (param_5 < 0x115) {
      if (param_5 == 0x104) {
        ppcVar1 = (code **)(*param_1 + 0x30);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (param_5 == 0x111) {
        ppcVar1 = (code **)(*param_1 + 0x10);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
    }
    else {
      if (param_5 == 0x115) {
        ppcVar1 = (code **)(*param_1 + 0x54);
        uVar3 = (**ppcVar1)();
        return uVar3;
      }
      if (param_5 == 0x201) {
        ppcVar1 = (code **)(*param_1 + 0x44);
        (**ppcVar1)();
        return 0x1;
      }
      if (param_5 == 0x204) {
        ppcVar1 = (code **)(*param_1 + 0x28);
        (**ppcVar1)();
        return 0x1;
      }
    }
  }
  return 0x0;
}



fn pass1_1040_8054(void) -> u16

{
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_805a(uchar *param_1) -> u32

{
  let unaff_DI: i16;
  let uVar1: u16;
  let unaff_SS: u16;
  
  if (_PTR_LOOP_1050_4230 == 0x0) {
    mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x28,unaff_SS,param_1,unaff_DI);
  }
  uVar1 = (_PTR_LOOP_1050_4230 >> 0x10);
  return CONCAT22((_PTR_LOOP_1050_4230 + 0x10),
                  (_PTR_LOOP_1050_4230 + 0xe));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_807e(param_1: u32,param_2: u16,param_3: u16)
{
  let uVar1: u16;
  code **ppcVar2;
  let puVar3: u32;
  let puVar4: u32;
  let in_DX: *mut u8
  let uVar5: u16;
  let extraout_DX: *mut u8
  let puVar6: *mut u8
  let extraout_DX_00: *mut u8
  let puVar7: *mut u8
  astruct_395 *iVar9;
  let uVar8: u16;
  astruct_43 *paVar9;
  let uStack10: u32;
  astruct_393 *iVar8;
  
  if (param_2 == 0x1) {
    pass1_1040_805a(in_DX);
    return;
  }
  paVar9 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,param_2,param_3);
  uVar5 = (paVar9 >> 0x10);
  puVar3 = paVar9;
  if ((uVar5 | puVar3) != 0x0) {
    ppcVar2 = (code **)(paVar9 + 0x14);
    puVar4 = puVar3;
    (**ppcVar2)(0x1010,puVar3,uVar5);
    uStack10 = CONCAT22(extraout_DX,puVar4);
    uVar8 = (param_1 >> 0x10);
    iVar9 = (astruct_395 *)param_1;
    puVar6 = extraout_DX;
    if (iVar9->field_0x70 != (astruct_90 *)0x0) {
      puVar4 = *(u32 **)&iVar9->field_0x70;
      uVar1 = (&iVar9->field_0x70 + 0x2);
      puVar6 = (uVar1 | puVar4);
      if (puVar6 != 0x0) {
        ppcVar2 = (code **)*puVar4;
        (**ppcVar2)();
        puVar6 = extraout_DX_00;
      }
    }
    mem_op_1000_179c(0x14,puVar6,0x1000);
    puVar7 = (puVar6 | puVar4);
    if (puVar7 == 0x0) {
      puVar4 = 0x0;
      puVar7 = 0x0;
    }
    else {
      struct_1008_4c58((u16 *)CONCAT22(puVar6,puVar4));
    }
    *(u32 **)&iVar9->field_0x70 = puVar4;
    *(uchar **)(&iVar9->field_0x70 + 0x2) = puVar7;
    pass1_1008_4d84(iVar9->field_0x70,uStack10,puVar7);
    if (paVar9 != (astruct_43 *)0x0) {
      ppcVar2 = (code **)paVar9;
      (**ppcVar2)(0x1008,puVar3,uVar5,0x1);
    }
    return;
  }
  return;
}


fn pass1_1040_824a(param_1: u32,param_2: i16) -> u16

{
  if ((param_1 + 0x6) != param_2) {
    return 0x1;
  }
  return 0x0;
}



astruct_18 * 
pass1_1040_83e6(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  ui_cleanup_op_1040_782c(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_57 * 
pass1_1040_8478(astruct_57 *param_1,param_2: u16,char *param_3,char *param_4,
               param_5: u16,param_6: u16)

{
  let uVar1: u16;
  astruct_712 *iVar2;
  let uVar2: u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfc3,param_5);
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_712 *)param_1;
  iVar2->field_0x8e = 0x0;
  iVar2->field_0x98 = param_2;
  iVar2->field_0x9a = 0x0;
  iVar2->field_0xb2 = 0x0;
  param_1 = 0x8ddc;
  iVar2->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  iVar2->field_0x9e = 0x0;
  iVar2->field_0xa2 = 0x12c;
  uVar1 = str_op_1008_60e8(param_4,param_6);
  iVar2->field_0x90 = uVar1;
  iVar2->field_0x92 = param_6;
  uVar1 = str_op_1008_60e8(param_3,param_6);
  iVar2->field_0x94 = uVar1;
  iVar2->field_0x96 = param_6;
  load_icon_1040_8b92(param_1,0x1008);
  ctx.PTR_LOOP_1050_5df8 = 0x0;
  return param_1;
}


fn pass1_1040_869a(astruct_18 *param_1)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1->field_0x0 = 0x8ddc;
  (iVar1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x90),0x1000);
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x94),0x1000);
  ui_cleanup_op_1040_782c(param_1,0x1000);
  return;
}



void 
pass1_1040_8978(param_1: *mut u32,param_2: u16,param_3: u16,param_4: u16,
               WNDCLASS16 *param_5)

{
  code **ppcVar1;
  
  unk_win_msg_op_1008_9510(&ctx.PTR_LOOP_1050_5df4,0x1008,param_5);
  win_1008_5c5c(param_5,param_3,param_4,_PTR_LOOP_1050_02a0,param_2);
  ppcVar1 = (code **)(*param_1 + 0x74);
  (**ppcVar1)(0x1008,param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1040_89a4(param_1: *mut u32,param_2: *mut u16,uchar *param_3,param_4: i16,
               WNDCLASS16 *param_5)

{
  let uVar1: u16;
  let uVar2: u16;
  code **ppcVar3;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let puVar7: *mut u16;
  
  unk_win_msg_op_1008_9510(&ctx.PTR_LOOP_1050_5df4,0x1008,param_5);
  uVar1 = (param_2 + 0x2);
  uVar2 = *param_2;
  uVar6 = 0x1010;
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_5,param_3,param_4);
  uVar5 = (puVar7 >> 0x10);
  uVar4 = puVar7;
  if ((uVar4 + 0x72) != 0x0) {
    uVar6 = 0x1008;
    win_1008_5c7c(_PTR_LOOP_1050_02a0,CONCAT22(uVar1,uVar2),param_5,uVar4,uVar5);
    (param_1 + 0x8c) = uVar4;
  }
  ppcVar3 = (code **)(*param_1 + 0x74);
  (**ppcVar3)(uVar6,param_1);
  return;
}


fn pass1_1040_8b3c(param_1: u16,param_2: u32,param_3: u32,param_4: u16)
{
  if ((param_3._2_2_ != 0x0) &&
     ((param_3._2_2_ == (&ctx.PTR_LOOP_1050_0000 + 0x1) ||
       param_3._2_2_ == &ctx.PTR_LOOP_1050_0002 ||
      (((&ctx.PTR_LOOP_1050_0002 + 0x1) < param_3._2_2_ + -0x2 &&
       (param_3._2_2_ + -0x6 < &ctx.PTR_LOOP_1050_0002)))))) {
    ctx.PTR_LOOP_1050_5df4 = 0x0;
    ctx.PTR_LOOP_1050_5df8 = param_3._2_2_;
    return;
  }
  post_win_msg_1040_7b3c
            (CONCAT22(param_2,param_1),(param_2 >> 0x10),
             param_3,param_3._2_2_,param_4);
  return;
}



astruct_18 *  pass1_1040_8db6(astruct_18 *param_1,param_2: u8)

{
  pass1_1040_869a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1040_8e58(param_1: i16,param_2: u16,param_3: u16,param_4: u32) -> u16

{
  pass1_1040_b040((astruct_57 *)CONCAT22(param_2,param_1),CONCAT22(param_4,param_3),
                  (param_4 >> 0x10));
  CONCAT22(param_2,param_1) = 0x8f3c;
  (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  return CONCAT22(param_2,param_1);
}



fn pass1_1040_8e82(astruct_18 *param_1)
{
  param_1->field_0x0 = 0x8f3c;
  (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  unk_draw_op_1040_b0f8(param_1);
  return;
}



astruct_18 *  pass1_1040_8f16(astruct_18 *param_1,param_2: u8)

{
  pass1_1040_8e82(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


fn pass1_1040_9252(param_1: u32,param_2: u16)
{
  let piVar1: *mut i16;
  let iVar2: i16;
  astruct_161 *iVar3;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_161 *)param_1;
  if (iVar3->field_0x4 != 0x0) {
    draw_text_1040_9650(param_1 & 0xffff | uVar3 << 0x10,param_2);
  }
  if (iVar3->field_0x8 != 0x0) {
    pass1_1040_9618(param_1 & 0xffff | uVar3 << 0x10);
  }
  iVar2 = iVar3->field_0x32;
  if (iVar3->field_0x22 < iVar2) {
    iVar3->field_0x22 = iVar2;
  }
  iVar2 = iVar3->field_0x34;
  if (iVar3->field_0x24 < iVar2) {
    iVar3->field_0x24 = iVar2;
  }
  iVar2 = iVar3->field_0x26 + iVar3->field_0x2a;
  if (iVar3->field_0x22 < iVar2) {
    iVar3->field_0x22 = iVar2;
  }
  iVar2 = iVar3->field_0x28 + iVar3->field_0x2c;
  if (iVar3->field_0x24 < iVar2) {
    iVar3->field_0x24 = iVar2;
  }
  piVar1 = &iVar3->field_0x22;
  *piVar1 = *piVar1 + iVar3->field_0x36;
  piVar1 = &iVar3->field_0x24;
  *piVar1 = *piVar1 + iVar3->field_0x36;
  return;
}


LRESULT  pass1_1040_93e6(param_1: u32,HWND16 param_2)

{
  LRESULT LVar1;
  
  LVar1 = SendMessage16(param_2,0x0,0x0,
                        CONCAT22(0x111,(param_1 + 0x1c)));
  return LVar1;
}


fn pass1_1040_9422(param_1: *mut u32)
{
  code **ppcVar1;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x8) != 0x0) {
    ppcVar1 = (code **)(*param_1 + 0x10);
    (**ppcVar1)();
  }
  if (*(long *)(param_1 + 0x4) != 0x0) {
    ppcVar1 = (code **)(*param_1 + 0x14);
    (**ppcVar1)();
  }
  return;
}


fn pass1_1040_9618(param_1: u32)
{
  let uVar1: u16;
  astruct_162 *iVar2;
  let uVar2: u16;
  let uVar3: u32;
  
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_162 *)param_1;
  uVar3 = pass1_1008_4772(iVar2->field_0x8);
  uVar1 = (uVar3 >> 0x10);
  iVar2->field_0x2a = (uVar3 + 0x4);
  iVar2->field_0x2c = (uVar3 + 0x8);
  return;
}


fn pass1_1040_97da(param_1: *mut u16,param_2: u8) -> u16

{
  mix_win_ui_op_1040_911e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1040_9824(param_1: *mut u32)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  *param_1 = 0x0;
  (iVar1 + 0x4) = 0x0;
  (iVar1 + 0x56) = 0x0;
  (iVar1 + 0x5a) = 0x0;
  (iVar1 + 0x5c) = 0x0;
  *(iVar1 + 0x6) = 0x0;
  return;
}


fn pass1_1040_a204(param_1: *mut u16,param_2: u8) -> u16

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


u32 
pass1_1040_a2cc(param_1: i16,param_2: u32,param_3: u32,param_4: u16,uchar *param_5,
               param_6: u16,param_7: u16)

{
  let uVar1: u16;
  
  if (param_3._2_2_ == 0x1826) {
    if ((param_3 == 0x1) ||
       ((0x1 < param_3 - 0x1 && (param_3 - 0x3 < 0x2)))) {
      uVar1 = 0x1;
    }
    else {
      uVar1 = 0x0;
    }
    return uVar1;
  }
  pass1_1040_b54a(param_1,param_2,(param_2 >> 0x10),param_3,param_5,
                  param_6,param_7);
  return CONCAT22(param_5,param_4);
}


astruct_18 * 
pass1_1040_a4c2(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  free_proc_inst_1040_a294(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1040_a564(param_1: *mut u32)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x0;
  (param_1 + 0x4) = 0x0;
  (param_1 + 0x6) = 0x0;
  return;
}



fn pass1_1040_a582(param_1: *mut u32)
{
  fn_ptr_1000_17ce((astruct_18 *)*param_1,0x1000);
  return;
}



fn pass1_1040_a5d0(param_1: u32)
{
  let uVar1: u16;
  let uVar2: u16;
  astruct_258 *iVar4;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar4 = (astruct_258 *)param_1;
  uVar1 = iVar4->field_0x2;
  uVar2 = iVar4->field_0x4;
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1000_54e8((uchar *)0xa582,&ctx.PTR_LOOP_1050_1040,(uVar1 - 0x2),0xa,
                    uVar1,uVar2);
    fn_ptr_1000_17ce((astruct_18 *)CONCAT22(uVar2,uVar1 - 0x2),0x1000);
  }
  fn_ptr_1000_17ce((astruct_18 *)iVar4->field_0xc,0x1000);
  return;
}



fn pass1_1040_a640(astruct_57 *param_1,param_2: u32,param_3: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  struct_1040_b082(param_1,CONCAT22(param_3,0x1f1));
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x94) = param_2;
  (iVar1 + 0x98) = 0x0;
  (iVar1 + 0xea) = 0x0;
  param_1 = 0xac08;
  (iVar1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  return;
}



fn pass1_1040_a84a(param_1: u32,param_2: u16)
{
  win_ui_dlg_op_1040_a94a(param_1,param_2);
  return;
}


astruct_18 *  pass1_1040_abe2(astruct_18 *param_1,param_2: u8)

{
  unk_draw_op_1040_b0f8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1040_ac84(astruct_57 *param_1,param_2: u16,uchar *param_3,param_4: i16,
               param_5: u16)

{
  astruct_726 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  struct_1040_b082(param_1,CONCAT22(param_2,0x1f3));
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_726 *)param_1;
  iVar1->field_0x94 = 0x0;
  &iVar1->field_0x98 = 0x0;
  param_1 = 0xafc4;
  iVar1->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  iVar1->field_0x94 = _PTR_LOOP_1050_5ef0;
  _PTR_LOOP_1050_5ef0 = 0x0;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3d,param_5,param_3,param_4);
  iVar1->field_0x98 = puVar2;
  iVar1->field_0x9a = (puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_ace8(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xafc4;
  (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  unk_draw_op_1040_b0f8(param_1);
  return;
}



fn pass1_1040_ad14(param_1: u32,param_2: u16)
{
  win_ui_op_1040_ae04(param_1,param_2);
  return;
}



void 
pass1_1040_ad24(param_1: i16,param_2: u16,param_3: u16,param_4: u32,uchar *param_5,
               param_6: u16,param_7: u16)

{
  if (param_4._2_2_ == 0xeb) {
    win_ui_op_1040_ae04(CONCAT22(param_2,param_1),param_7);
  }
  else {
    if (param_4._2_2_ != 0x1f0) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4,param_5,param_6,param_7);
      return;
    }
    msg_box_ui_op_1040_ad66(CONCAT22(param_2,param_1),0x0,param_5,param_7);
  }
  return;
}


astruct_18 *  pass1_1040_af9e(astruct_18 *param_1,param_2: u8)

{
  pass1_1040_ace8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1040_b040(astruct_57 *param_1,param_2: u32,param_3: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,(param_2 + 0x12),param_3);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x8e) = 0x0;
  (iVar1 + 0x90) = param_2;
  param_1 = 0xb772;
  (iVar1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  return;
}


fn pass1_1040_b0bc(astruct_57 *param_1,param_2: u32,param_3: u32)
{
  let iVar1: i16;
  let uVar2: u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,param_3,(param_3 >> 0x10));
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x8e) = 0x0;
  (iVar1 + 0x90) = param_2;
  param_1 = 0xb772;
  (iVar1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  return;
}


void 
pass1_1040_b17c(param_1: u32,param_2: u32,uchar *param_3,param_4: i16,param_5: i16,
               param_6: u16)

{
  let piVar1: *mut i16;
  let uVar2: u32;
  char *pcVar3;
  let uVar4: u16;
  let iVar5: i16;
  let uVar6: u16;
  let puVar7: *mut u8
  let puVar8: *mut u16;
  let puStack12: *mut u16;
  let iStack4: i16;
  
  iStack4 = 0x0;
  while( true ) {
    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    piVar1 = (iVar5 + 0x90);
    puVar7 = (piVar1 >> 0x10);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    param_5 = (iStack4 * 0x2 + param_2);
    uVar2 = (piVar1 + 0x2);
    (iStack4 * 0xa + uVar2 + 0x4) = param_5;
    iStack4 += 0x1;
    param_3 = puVar7;
  }
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_3,param_5);
  uVar4 = (puVar8 >> 0x10);
  uVar2 = (iVar5 + 0x90);
  puStack12 = (uVar2 + 0x2);
  for (iStack4 = 0x0; piVar1 = (iVar5 + 0x90),
      *piVar1 != iStack4 && iStack4 <= *piVar1; iStack4 += 0x1) {
    uVar2 = (iVar5 + 0x90);
    uVar2 = (uVar2 + 0x6);
    pcVar3 = pass1_1010_b038((uchar *)puVar8,uVar2,(uVar2 >> 0x10),
                             *(uchar **)(puStack12 + 0x4),param_4);
    string_1040_a626(puStack12,CONCAT22(uVar4,pcVar3),uVar4);
    puStack12 = (puStack12 & 0xffff0000 | (puStack12 + 0xa));
  }
  return;
}


fn pass1_1040_b316(param_1: *mut u32,param_2: u16,param_3: u16,param_4: u16,param_5: i16) -> u16

{
  code **ppcVar1;
  let uStack4: u16;
  
  if (param_5 == 0xf) {
    ppcVar1 = (code **)(*param_1 + 0x60);
    uStack4 = (**ppcVar1)();
  }
  else {
    if (param_5 == 0x111) {
      ppcVar1 = (code **)(*param_1 + 0x10);
      (**ppcVar1)();
      uStack4 = 0x1;
    }
    else {
      uStack4 = pass1_1040_79c0(param_1,param_2,param_3,param_4,param_5);
    }
  }
  return uStack4;
}


fn pass1_1040_b45e(param_1: u32,HWND16 param_2)
{
  let uVar1: u32;
  let piVar2: *mut i16;
  let iVar3: i16;
  let uVar4: u16;
  let iStack8: i16;
  INT16 *pIStack6;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (*(long *)(iVar3 + 0x90) != 0x0) {
    uVar1 = (iVar3 + 0x90);
    (uVar1 + 0x14) = (iVar3 + 0x6);
    uVar1 = (iVar3 + 0x90);
    pIStack6 = *(INT16 **)(uVar1 + 0x2);
    for (iStack8 = 0x0; piVar2 = (iVar3 + 0x90),
        *piVar2 != iStack8 && iStack8 <= *piVar2; iStack8 += 0x1) {
      SetDlgItemText16(param_2,*pIStack6,(SEGPTR)(pIStack6 + 0x2));
      pIStack6 = (INT16 *)(pIStack6 & 0xffff0000 | (pIStack6 + 0xa));
      param_2 = s_tile2_bmp_1050_1538;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_b4c8(param_1: u32,uchar *param_2,param_3: i16,param_4: u16)
{
  let iVar1: i16;
  let uVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  let puVar5: *mut u16;
  
  uVar4 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x90) != 0x0) {
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_4,param_2,param_3);
    uVar2 = (param_1 + 0x90);
    iVar1 = (uVar2 + 0xa);
    iVar3 = iVar1 + -0x4;
    if (iVar3 == 0x0) {
      ui_op_1010_79aa(puVar5,0xfd9,0x0,param_4);
      if (iVar3 == 0x0) {
        uVar4 = 0xe;
//LAB_1040_b50f:
        unk_win_op_1010_7300
                  (puVar5,CONCAT22(iVar3,iVar3),uVar4,CONCAT22(iVar3,iVar3));
        return;
      }
    }
    else {
      if (((0x0 < iVar1 + -0x5) && (!SBORROW2(iVar1 + -0x5,0x1))) &&
         (iVar3 = iVar1 + -0x7, iVar3 == 0x0 || iVar1 + -0x6 < 0x1)) {
        ui_op_1010_79aa(puVar5,0xfda,0x0,param_4);
        if (iVar3 == 0x0) {
          uVar4 = 0xd;
          goto LAB_1040_b50f;
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1040_b54a(param_1: i16,param_2: u16,param_3: u16,param_4: u32,uchar *param_5,
               param_6: u16,param_7: u16)

{
  astruct_18 *paVar1;
  code **ppcVar2;
  let uVar3: u32;
  let iVar4: i16;
  let iVar5: i16;
  astruct_18 *paVar6;
  let uVar7: u16;
  astruct_515 *iVar6;
  let unaff_DI: i16;
  let uVar8: u16;
  let puVar9: *mut u16;
  let uVar10: u8;
  let uVar11: u8;
  let uVar12: u16;
  let uVar13: u16;
  let uVar14: u16;
  
  if (param_4._2_2_ == 0xea) {
    ppcVar2 = (code **)(CONCAT22(param_2,param_1) + 0x5c);
    (**ppcVar2)(param_6,param_1,param_2);
  }
  else {
    if (param_4._2_2_ == 0xeb) {
      puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_7,param_5,unaff_DI);
      uVar7 = (puVar9 >> 0x10);
      paVar1 = *(astruct_18 **)(param_1 + 0x90);
      if (paVar1 != (astruct_18 *)0x0) {
        uVar8 = (paVar1 >> 0x10);
        uVar12 = 0x1010;
        paVar6 = paVar1;
        pass1_1010_ad64(puVar9,CONCAT22((paVar1 + 0xa),uVar7),
                        (paVar1 + 0x6),paVar1,uVar7);
        (param_1 + 0x90) = paVar6;
        (param_1 + 0x92) = uVar7;
        if ((uVar7 | (param_1 + 0x90)) == 0x0) {
          *(astruct_18 **)(param_1 + 0x90) = paVar1;
        }
        else {
          if (paVar1 != (astruct_18 *)0x0) {
            pass1_1040_a5d0(paVar1);
            uVar12 = 0x1000;
            fn_ptr_1000_17ce(paVar1,0x1000);
          }
          ppcVar2 = (code **)(CONCAT22(param_2,param_1) + 0x70);
          (**ppcVar2)(uVar12,param_1,param_2);
        }
      }
    }
    else {
      if (param_4._2_2_ == 0x1790) {
        puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_7,param_5,unaff_DI);
        uVar3 = (param_1 + 0x90);
        uVar3 = (uVar3 + 0x6);
        iVar4 = pass1_1010_7d38(puVar9,(puVar9 >> 0x10),uVar3,
                                (uVar3 >> 0x10),param_7);
        iVar5 = iVar4;
        ui_op_1010_79aa(puVar9,0xfab,0x0,param_7);
        if (iVar5 != 0x0) {
          return;
        }
        if (iVar4 == 0x0) {
          uVar3 = (param_1 + 0x90);
          uVar8 = (uVar3 >> 0x10);
          iVar6 = (astruct_515 *)uVar3;
          uVar3 = iVar6->field_0x6;
          uVar13 = uVar3;
          uVar14 = (uVar3 >> 0x10);
          uVar12 = 0x14;
        }
        else {
          uVar3 = (param_1 + 0x90);
          uVar8 = (uVar3 >> 0x10);
          iVar6 = (astruct_515 *)uVar3;
          uVar3 = iVar6->field_0x6;
          uVar13 = uVar3;
          uVar14 = (uVar3 >> 0x10);
          uVar12 = 0x9;
        }
        uVar10 = (u8)uVar8;
        uVar11 = (u8)(uVar8 >> 0x8);
      }
      else {
        if (param_4._2_2_ == 0x1824) {
          puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_7,param_5,unaff_DI);
          iVar6 = (astruct_515 *)puVar9;
          uVar3 = (param_1 + 0x90);
          ui_op_1010_79aa(puVar9,0xfc5,*(long *)(uVar3 + 0x6),param_7);
          if (iVar6 != (astruct_515 *)0x0) {
            return;
          }
          uVar3 = (param_1 + 0x90);
          uVar3 = (uVar3 + 0x6);
          uVar13 = uVar3;
          uVar14 = (uVar3 >> 0x10);
          uVar12 = 0x12;
          uVar10 = 0x0;
          uVar11 = 0x0;
        }
        else {
          if (param_4._2_2_ != 0x1830) {
            post_win_msg_1040_7b3c
                      (CONCAT13((param_2 >> 0x8),
                                         CONCAT12(param_2,param_1)),param_3,
                       param_4,param_4._2_2_,param_6);
            return;
          }
          puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_7,param_5,unaff_DI);
          iVar6 = (astruct_515 *)puVar9;
          uVar3 = (param_1 + 0x90);
          ui_op_1010_79aa(puVar9,0xfb6,*(long *)(uVar3 + 0x6),param_7);
          if (iVar6 != (astruct_515 *)0x0) {
            return;
          }
          uVar3 = (param_1 + 0x90);
          uVar3 = (uVar3 + 0x6);
          uVar13 = uVar3;
          uVar14 = (uVar3 >> 0x10);
          uVar12 = 0xc;
          uVar10 = 0x0;
          uVar11 = 0x0;
        }
      }
      unk_win_op_1010_7300
                (puVar9,CONCAT13(uVar11,CONCAT12(uVar10,iVar6)),uVar12,
                 CONCAT22(uVar14,uVar13));
    }
  }
  return;
}



ULONG  pass1_1040_b74c(astruct_18 *param_1,param_2: u8)

{
  unk_draw_op_1040_b0f8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return (ULONG)param_1;
}



fn pass1_1040_b7ee(astruct_57 *param_1,param_2: i32,param_3: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  let uVar3: u16;
  
  pass1_1040_b0bc(param_1,0x0,CONCAT22(param_3,0xfab));
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x94) = 0x0;
  (iVar1 + 0x98) = 0x0;
  (iVar1 + 0xb0) = 0x0;
  (iVar1 + 0xb4) = 0x0;
  (iVar1 + 0xb6) = 0x0;
  param_1 = 0xbeba;
  (iVar1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  if (param_2 != 0x0) {
    uVar3 = (param_2 >> 0x10);
    (iVar1 + 0xb0) = (param_2 + 0x6);
    (iVar1 + 0xb4) = (param_2 + 0x14);
  }
  return;
}



u16 
pass1_1040_b864(param_1: *mut u32,i16 *param_2,param_3: u16,param_4: u16,param_5: i16,
               param_6: u16)

{
  code **ppcVar1;
  let uVar2: u16;
  
  if (param_5 == 0x2b) {
    if (*param_2 == 0x4) {
      win_ui_get_prop_op_1040_9566(CONCAT22(param_3,param_2),param_6);
    }
  }
  else {
    if (param_5 != 0x111) {
      uVar2 = pass1_1040_b316(param_1,param_2,param_3,param_4,param_5);
      return uVar2;
    }
    ppcVar1 = (code **)(*param_1 + 0x7c);
    (**ppcVar1)(param_6,param_1,param_2,CONCAT22(param_4,param_3));
  }
  return 0x1;
}



fn pass1_1040_b8be(param_1: *mut u32)
{
  code **ppcVar1;
  
  ppcVar1 = (code **)(*param_1 + 0x80);
  (**ppcVar1)();
  return;
}



fn pass1_1040_bb5a(param_1: u32) -> u16

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((param_1 + 0x94) + 0x8);
  (**ppcVar1)();
  return 0x0;
}


astruct_18 *  pass1_1040_be94(astruct_18 *param_1,param_2: u8)

{
  unk_draw_op_1040_b0f8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn struct_1040_bf3e(param_1: *mut u16,param_2: u16) -> u16

{
  astruct_442 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_442 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x3aa8;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = param_2;
  *param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x6 = 0x0;
  *param_1 = 0xc53e;
  iVar1->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  return param_1;
}



fn pass1_1040_bf92(param_1: *mut u16,param_2: u16)
{
  astruct_514 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_514 *)param_1;
  *param_1 = 0xc53e;
  iVar1->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  pass1_1010_1ea6(iVar1->field_0x6,param_1 & 0xffff | uVar1 << 0x10,param_2)
  ;
  unk_destroy_win_op_1010_2fa0(iVar1->field_0x6,0x1010);
  *param_1 = 0x3ab0;
  iVar1->field_0x2 = 0x1008;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  return;
}



fn pass1_1040_bfde(param_1: u32,param_2: *mut u32,param_3: u16)
{
  code **ppcVar1;
  let uVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  *(u32 **)(iVar3 + 0x6) = param_2;
  ppcVar1 = (code **)(*param_2 + 0x4);
  (**ppcVar1)();
  uVar2 = (iVar3 + 0x6);
  (uVar2 + 0x22) = (iVar3 + 0x4);
  pass1_1010_2ee2(*(u32 **)(iVar3 + 0x6),param_3,0x1010);
  return;
}


fn pass1_1040_c518(param_1: u32,param_2: u8,param_3: u16) -> u32

{
  pass1_1040_bf92((u16 *)param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



void 
pass1_1040_c54a(param_1: *mut u16,param_2: u16,param_3: *mut u32,param_4: u16,
               param_5: u16)

{
  code **ppcVar1;
  let iVar3: i16;
  let extraout_DX: u16;
  astruct_164 *iVar2;
  let uVar4: u16;
  let puVar5: u32;
  let uVar6: u16;
  let uVar7: u32;
  
  iVar3 = (param_3 + 0x12) + 0xc8;
  uVar7 = 0x0;
  uVar6 = 0x0;
  ppcVar1 = (code **)(*param_3 + 0x14);
  puVar5 = param_3;
  (**ppcVar1)();
  mixed_struct_op_1040_8fb8
            (param_1,0x0,CONCAT22(extraout_DX,iVar3),puVar5,
             (puVar5 >> 0x10),uVar6,uVar7,
             (uVar7 >> 0x10),extraout_DX,param_4,param_5);
  uVar4 = (param_1 >> 0x10);
  iVar2 = (astruct_164 *)param_1;
  iVar2->field_0x42 = param_3;
  iVar2->field_0x46 = 0x0;
  iVar2->field_0x48 = param_2;
  *param_1 = 0xc9f2;
  iVar2->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  pass1_1040_c630((param_1 & 0xffff | uVar4 << 0x10),param_4,
                  param_5);
  return;
}



fn pass1_1040_c5ac(param_1: *mut u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0xc9f2;
  (iVar4 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  ctx.PTR_LOOP_1050_5f02 = ctx.PTR_LOOP_1050_5f02 + -0x1;
  if (ctx.PTR_LOOP_1050_5f02 == 0x0) {
    puVar1 = (iVar4 + 0x8);
    uVar2 = (iVar4 + 0xa);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    puVar1 = (iVar4 + 0xc);
    uVar2 = (iVar4 + 0xe);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
  }
  mix_win_ui_op_1040_911e(param_1);
  return;
}



fn pass1_1040_c60e(param_1: u32) -> u16

{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x42) != 0x0) {
    uVar1 = (param_1 + 0x42);
    return (uVar1 + 0x12);
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_c630(param_1: *mut u32,param_2: u16,param_3: u16)
{
  let iVar1: i16;
  code **ppcVar2;
  let uVar3: u32;
  let uVar4: u32;
  astruct_165 *iVar4;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = (astruct_165 *)param_1;
  uVar3 = iVar4->field_0x42;
  if ((uVar3 + 0x12) != 0x71) {
    iVar4->field_0x36 = 0x5;
    iVar4->field_0x26 = 0x5;
    iVar4->field_0x28 = 0x5;
    iVar1 = iVar4->field_0x36;
    iVar4->field_0x30 = iVar1;
    iVar4->field_0x2e = iVar1;
    if (ctx.PTR_LOOP_1050_5f02 == 0x0) {
      _PTR_LOOP_1050_5f04 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0xff,param_3);
      param_2 = 0x1010;
      _PTR_LOOP_1050_5f08 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x100,param_3);
    }
    ctx.PTR_LOOP_1050_5f02 = ctx.PTR_LOOP_1050_5f02 + 0x1;
    iVar4->field_0x8 = _PTR_LOOP_1050_5f04;
    iVar4->field_0xc = _PTR_LOOP_1050_5f08;
    pass1_1040_9618(param_1);
    iVar4->field_0x20 = 0x0;
    iVar4->field_0x1e = 0xc8;
    iVar4->field_0x22 = 0xa0;
    iVar4->field_0x24 = iVar4->field_0x2c + iVar4->field_0x36;
    iVar4->field_0x2e = iVar4->field_0x36 * 0x3 + iVar4->field_0x2a;
    iVar4->field_0x30 = iVar4->field_0x36;
    iVar4->field_0x32 = iVar4->field_0x22 - iVar4->field_0x36;
    iVar4->field_0x3c = 0x25;
    uVar4 = *param_1;
    ppcVar2 = (code **)(uVar4 + 0x4);
    (**ppcVar2)(param_2,param_1);
    ppcVar2 = (code **)(uVar4 + 0x8);
    (**ppcVar2)(param_2,param_1,uVar5);
  }
  return;
}



fn pass1_1040_c71e(param_1: u32,param_2: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  pass1_1040_9252(param_1,param_2);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x28) = (iVar1 + 0x24) / 0x2 - (iVar1 + 0x2c) / 0x2;
  return;
}


void 
pass1_1040_c94a(param_1: i16,param_2: u16,uchar *param_3,param_4: i16,param_5: u16,
               param_6: u16)

{
  let uVar1: u16;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let puVar5: *mut u16;
  
  if ((param_1 + 0x48) != 0x0) {
    puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_3,param_4);
    uVar4 = (puVar5 >> 0x10);
    uVar2 = (param_1 + 0x42);
    uVar1 = (uVar2 + 0x12);
    param_5 = 0x1010;
    uVar3 = uVar1;
    pass1_1010_a5ca(puVar5,uVar4,uVar1,uVar1,uVar4);
    if (uVar3 == 0xffff) {
      (param_1 + 0x3c) = 0xf9;
    }
    else {
      if (uVar3 == 0x0) {
        (param_1 + 0x3c) = 0x25;
        if ((uVar1 == 0x5b) || (uVar1 == 0x9)) {
          (param_1 + 0x3c) = 0xfe;
        }
      }
      else {
        (param_1 + 0x3c) = 0xfb;
      }
    }
  }
  draw_text_1040_94fc((astruct_37 *)CONCAT22(param_2,param_1),param_5);
  return;
}



fn pass1_1040_c9cc(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1040_c5ac(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1040_ca16(astruct_57 *param_1,param_2: u16,uchar *param_3,param_4: i16,
               param_5: u16)

{
  astruct_727 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  struct_1040_b082(param_1,CONCAT22(param_2,0x1840));
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_727 *)param_1;
  iVar1->field_0x94 = _PTR_LOOP_1050_5f0c;
  &iVar1->field_0x98 = 0x0;
  iVar1->field_0x9c = 0x0;
  iVar1->field_0x9e = 0x0;
  param_1 = 0xd07c;
  iVar1->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3e,param_5,param_3,param_4);
  iVar1->field_0x98 = puVar2;
  iVar1->field_0x9a = (puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_ca74(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xd07c;
  (param_1 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ctx.PTR_LOOP_1050_5f10 = 0x0;
  unk_draw_op_1040_b0f8(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_caa6(param_1: u16,param_2: u32,uchar *param_3,param_4: i16,param_5: u16)
{
  let puVar1: *mut u16;
  let iVar2: i16;
  
  iVar2 = 0x0;
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,param_3,param_4);
  pass1_1010_038e(puVar1,iVar2,param_5);
  destroy_window_1040_b726
            ((ULONG *)CONCAT22(param_2,param_1),(param_2 >> 0x10),0x1010);
  return;
}


LRESULT  pass1_1040_cc66(param_1: u32,param_2: u16)

{
  code **ppcVar1;
  LRESULT LVar2;
  
  ppcVar1 = (code **)((param_1 + 0x98) + 0x10);
  (**ppcVar1)();
  LVar2 = send_dlg_msg_1040_cf1c(param_1,param_2);
  return LVar2;
}



void 
pass1_1040_cc8c(param_1: i16,param_2: u16,param_3: u16,param_4: u32,uchar *param_5,
               param_6: u16,param_7: u16)

{
  if (param_4._2_2_ == 0xeb) {
    send_dlg_msg_1040_cf1c(CONCAT22(param_2,param_1),param_7);
  }
  else {
    if (param_4._2_2_ == (s_vrpal_bmp_1050_183a + 0x7)) {
      msg_box_op_1040_cce4(CONCAT22(param_2,param_1),0x0,param_5,param_7);
    }
    else {
      if (param_4._2_2_ != (s_vrpal_bmp_1050_183a + 0x8)) {
        pass1_1040_b54a(param_1,param_2,param_3,param_4,param_5,param_6,param_7);
        return;
      }
      if (param_4 == 0x1) {
        send_dlg_item_1040_ce76(CONCAT22(param_2,param_1),param_6,param_7);
      }
    }
  }
  return;
}


fn pass1_1040_cdac(param_1: u32,param_2: u16,param_3: u16,param_4: i16,HWND16 param_5) -> u16

{
  let piVar1: *mut i16;
  let iVar2: i16;
  let bVar3: bool;
  let iVar4: i16;
  let uVar5: u16;
  
  bVar3 = false;
  iVar4 = param_1;
  uVar5 = (param_1 >> 0x10);
  if (param_4 == 0x0) {
    iVar2 = (iVar4 + 0x9e);
    piVar1 = (iVar4 + 0x9c);
    if (*piVar1 == iVar2 || *piVar1 < iVar2) goto LAB_1040_cdef;
    piVar1 = (iVar4 + 0x9e);
    *piVar1 = *piVar1 + 0x1;
  }
  else {
    if (param_4 != 0x1) goto LAB_1040_cdef;
    if ((iVar4 + 0x9e) < 0x1) goto LAB_1040_cdef;
    piVar1 = (iVar4 + 0x9e);
    *piVar1 = *piVar1 + -0x1;
  }
  bVar3 = true;
//LAB_1040_cdef:
  if (bVar3) {
    SetDlgItemInt16(param_5,0x0,(iVar4 + 0x9e),0x18e);
  }
  return 0x0;
}


astruct_18 *  pass1_1040_d056(astruct_18 *param_1,param_2: u8)

{
  pass1_1040_ca74(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_d0f8(astruct_57 *param_1,param_2: u16)
{
  let uVar1: u16;
  let in_DX: *mut u8
  let uVar2: u16;
  let puVar3: *mut u8
  let puVar4: *mut u8
  astruct_438 *iVar5;
  let unaff_DI: i16;
  let uVar5: u16;
  let unaff_SS: u16;
  let puVar6: *mut u16;
  let uVar7: u32;
  astruct_392 *iVar8;
  
  struct_1040_b082(param_1,CONCAT22(param_2,0x1845));
  uVar5 = (param_1 >> 0x10);
  iVar5 = (astruct_438 *)param_1;
  &iVar5->field_0x94 = 0x0;
  iVar5->field_0x98 = _PTR_LOOP_1050_5f16;
  &iVar5->field_0x9c = 0x0;
  iVar5->field_0xa0 = 0x0;
  param_1 = 0xd8c4;
  iVar5->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x47,unaff_SS,in_DX,unaff_DI);
  uVar2 = (puVar6 >> 0x10);
  iVar5->field_0x94 = puVar6;
  iVar5->field_0x96 = uVar2;
  uVar7 = pass1_1018_5732(iVar5->field_0x94,uVar2,iVar5->field_0x98,puVar6,uVar2,
                          unaff_SS);
  puVar3 = (uVar7 >> 0x10);
  iVar5->field_0x9c = uVar7;
  iVar5->field_0x9e = puVar3;
  uVar1 = puVar3 | iVar5->field_0x9c;
  if (uVar1 == 0x0) {
    mem_op_1000_179c(0xc,puVar3,0x1000);
    puVar4 = (puVar3 | uVar1);
    if (puVar4 == 0x0) {
      &iVar5->field_0x9c = 0x0;
    }
    else {
      pass1_1010_8ef2((u16 *)CONCAT22(puVar3,uVar1),puVar4,unaff_SS);
      iVar5->field_0x9c = uVar1;
      iVar5->field_0x9e = puVar4;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_d1bc(param_1: *mut astruct_18)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_513 *iVar4;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_513 *)param_1;
  param_1->field_0x0 = 0xd8c4;
  iVar4->field_0x2 = &ctx.PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,iVar4->field_0x6);
  puVar1 = iVar4->field_0x9c;
  uVar2 = iVar4->field_0x9e;
  if (uVar2 | puVar1) != 0x0 {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(&ctx.PTR_LOOP_1050_1038,puVar1,uVar2,0x1);
  }
  unk_draw_op_1040_b0f8(param_1);
  return;
}



fn pass1_1040_d29c(param_1: u32,param_2: u16)
{
  send_ldg_item_msg_1040_d79c(param_1,param_2);
  return;
}


fn pass1_1040_d76e(param_1: u32)
{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  uVar1 = (iVar2 + 0x94);
  pass1_1018_5742(uVar1,(uVar1 >> 0x10),*(u32 **)(iVar2 + 0x9c),
                  (iVar2 + 0x98));
  (iVar2 + 0x9c) = 0x0;
  return;
}



fn  pass1_1040_d89e(param_1: *mut astruct_18,param_2: u8) -> *mut astruct_18

{
  pass1_1040_d1bc(param_1);
  if (param_2 & 0x1) != 0x0 {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}





