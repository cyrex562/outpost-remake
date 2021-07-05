
fn struct_1018_0570(astruct_55 *param_1,param_2: u16,param_3: u16)
{
  let puVar1: u32;
  code **ppcVar2;
  let puVar3: *mut u16;
  let uVar4: u16;
  let puVar5: *mut u8
  let uVar6: u16;
  let extraout_DX: *mut u8
  let unaff_DI: i16;
  let puVar7: *mut u16;
  let uVar8: u16;
  astruct_262 *uVar9;
  
  uVar9 = (astruct_262 *)param_1;
  uVar8 = (param_1 >> 0x10);
  get_sys_metrics_1018_4b1e(param_1,0x0,param_2);
  uVar9->field_0x20 = 0x389a;
  uVar9->field_0x22 = 0x1008;
  uVar9->field_0x20 = 0x3aa8;
  uVar9->field_0x22 = 0x1008;
  uVar9->field_0x24 = 0x0;
  uVar9->field_0x2c = 0x0;
  pass1_1008_3e38((u16 *)
                  (param_1 & 0xffff0000 | &uVar9->field_0x30));
  puVar7 = pass1_1008_3e38((u16 *)
                           (param_1 & 0xffff0000 | &uVar9->field_0x36)
                          );
  puVar5 = (uchar *)(puVar7 >> 0x10);
  uVar9->field_0x3c = 0x0;
  pass1_1008_6c90((u16 *)
                  (param_1 & 0xffff0000 | &uVar9->field_0x40));
  uVar6 = 0x0;
  uVar9->field_0x4c = 0x0;
  uVar9->field_0x5a = 0x0;
  uVar9->field_0x5e = 0x0;
  uVar9->field_0x60 = 0x0;
  uVar9->field_0x64 = 0xff00;
  uVar9->field_0x66 = 0x0;
  uVar9->field_0x68 = 0x10000fb;
  uVar9->field_0x6c = 0x10000f9;
  uVar9->field_0x70 = 0x10000ff;
  uVar9->field_0x74 = 0x10000fe;
  uVar9->field_0x78 = 0x10000fc;
  uVar9->field_0x7c = 0x0;
  uVar9->field_0x80 = 0x0;
  uVar9->field_0x84 = 0x1;
  uVar9->field_0x86 = 0x0;
  uVar9->field_0x88 = 0x0;
  uVar9->field_0x8c = 0x0;
  uVar9->field_0x8e = 0x0;
  uVar9->field_0x92 = 0x0;
  uVar9->field_0x94 = 0x0;
  uVar9->field_0x98 = 0x0;
  uVar9->field_0x9a = 0x0;
  &uVar9->field_0xa2 = 0x0;
  uVar9->field_0xa6 = 0xffff;
  uVar9->field_0xa8 = 0x0;
  param_1->field_0x0 = 0x1874;
  uVar9->field_0x2 = 0x1018;
  uVar9->field_0x20 = 0x18b0;
  uVar9->field_0x22 = 0x1018;
  if ((PTR_LOOP_1050_3960 == 0x0) && (_PTR_LOOP_1050_3962 == 0x0)) {
    mem_op_1000_179c(0x8,puVar5,0x1000);
    _PTR_LOOP_1050_3962 = CONCAT22(puVar5,uVar6);
    pass1_1000_4906((astruct_20 *)CONCAT22(puVar5,uVar6),(WNDCLASS16 *)0x0,0x8);
  }
  PTR_LOOP_1050_3960 = PTR_LOOP_1050_3960 + 0x1;
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_3,puVar5,unaff_DI);
  &uVar9->field_0x2c = puVar7;
  (&uVar9->field_0x2c + 0x2) = (puVar7 >> 0x10);
  if (param_1 == (astruct_55 *)0x0) {
    puVar3 = (u16 *)0x0;
    uVar6 = 0x0;
  }
  else {
    puVar3 = &uVar9->field_0x20;
    uVar6 = uVar8;
  }
  puVar1 = uVar9->field_0x2c;
  ppcVar2 = (code **)(*uVar9->field_0x2c + 0x4);
  (**ppcVar2)(0x1010,puVar1,(puVar1 >> 0x10),0x0,puVar3,uVar6);
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_3,extraout_DX,unaff_DI);
  puVar5 = (uchar *)(puVar7 >> 0x10);
  if ((puVar7 + 0x80) != 0x0) {
    uVar9->field_0x84 = 0x2;
  }
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x9,param_3,puVar5,unaff_DI);
  puVar5 = (uchar *)(puVar7 >> 0x10);
  uVar9->field_0x9e = puVar7;
  uVar9->field_0xa0 = puVar5;
  uVar4 = pass1_1010_65d0(param_3,puVar7 & 0xffff0000 | uVar9->field_0x9e,
                          0x88);
  if (uVar4 != 0x0) {
    uVar9->field_0xa8 = 0x1;
  }
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3b,param_3,puVar5,unaff_DI);
  uVar9->field_0xa2 = puVar7;
  uVar9->field_0xa4 = (puVar7 >> 0x10);
  return;
}


void 
struct_1018_229c(astruct_632 *param_1,uchar *param_2,param_3: u16,uchar *param_4,
                param_5: u16)

{
  let piVar1: *mut i16;
  astruct_43 *paVar2;
  let iStack4: i16;
  
  struct_op_1018_4cda(param_1,param_2,param_3);
  param_1->field_0x1c = 0x389a;
  param_1->field_0x1e = 0x1008;
  param_1->field_0x1c = 0x3aa8;
  param_1->field_0x1e = 0x1008;
  param_1->field_0x20 = 0x0;
  param_1->field_0x24 = 0x0;
  param_1->field_0x26 = 0x0;
  &param_1->field_0x2a = 0x0;
  param_1->field_0x3e = 0x0;
  param_1->field_0x40 = 0x0;
  param_1->field_0x42 = 0x0;
  param_1->field_0x44 = 0x0;
  &param_1->field_0x6e = 0x0;
  CONCAT22(param_2,param_1) = 0x2ada;
  param_1->field_0x2 = 0x1018;
  param_1->field_0x1c = (s_fem132_wav_1050_2aec + 0x6);
  param_1->field_0x1e = 0x1018;
  PTR_LOOP_1050_4230 = param_1;
  PTR_LOOP_1050_4232 = param_2;
  pass1_1018_4dce(CONCAT22(param_2,param_1),0x105,param_4,param_5);
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1a8,param_5);
  param_1->field_0x2a = paVar2;
  param_1->field_0x2c = (paVar2 >> 0x10);
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,&param_1->field_0x2e),(WNDCLASS16 *)0x0,
                  0x10);
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,&param_1->field_0x46),(WNDCLASS16 *)0x0,
                  0x28);
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x6c,param_5);
  param_1->field_0x2e = paVar2;
  param_1->field_0x30 = (paVar2 >> 0x10);
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x68,param_5);
  param_1->field_0x32 = paVar2;
  param_1->field_0x34 = (paVar2 >> 0x10);
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x66,param_5);
  param_1->field_0x36 = paVar2;
  param_1->field_0x38 = (paVar2 >> 0x10);
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x6a,param_5);
  param_1->field_0x3a = paVar2;
  param_1->field_0x3c = (paVar2 >> 0x10);
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1cd,param_5);
  param_1->field_0x6e = paVar2;
  param_1->field_0x70 = (paVar2 >> 0x10);
  iStack4 = 0x0;
  do {
    paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,iStack4 + 0x8f,param_5);
    (&param_1->field_0x46 + iStack4 * 0x4) = paVar2;
    (&param_1->field_0x48 + iStack4 * 0x4) = (paVar2 >> 0x10);
    iStack4 += 0x1;
  } while (iStack4 < 0xa);
  if (CONCAT22(param_2,param_1) == 0x0) {
    piVar1 = (i16 *)0x0;
    param_2 = (uchar *)0x0;
  }
  else {
    piVar1 = &param_1->field_0x1c;
  }
  pass1_1008_9262(_PTR_LOOP_1050_0388,(_PTR_LOOP_1050_0388 >> 0x10),
                  0x73,CONCAT22(param_2,piVar1),piVar1,param_2);
  return;
}


fn struct_1018_2b10(astruct_55 *param_1,param_2: u16,param_3: u16)
{
  let puVar1: u32;
  code **ppcVar2;
  let puVar3: *mut u16;
  let uVar4: u16;
  let unaff_DI: i16;
  let puVar5: *mut u16;
  astruct_43 *paVar6;
  let uVar7: u32;
  let uVar8: u16;
  astruct_626 *uVar9;
  
  uVar9 = (astruct_626 *)param_1;
  uVar8 = (param_1 >> 0x10);
  puVar5 = get_sys_metrics_1018_4b1e(param_1,0x1,param_2);
  uVar9->field_0x20 = 0x389a;
  uVar9->field_0x22 = 0x1008;
  uVar9->field_0x20 = 0x3aa8;
  uVar9->field_0x22 = 0x1008;
  uVar9->field_0x24 = (astruct_76 *)0x0;
  uVar9->field_0x174 = 0x0;
  uVar9->field_0x176 = 0x0;
  uVar9->field_0x178 = 0x0;
  uVar9->field_0x17a = 0x0;
  uVar9->field_0x17e = 0x0;
  uVar9->field_0x182 = 0x0;
  uVar9->field_0x186 = 0x0;
  param_1->field_0x0 = 0x32d8;
  uVar9->field_0x2 = 0x1018;
  uVar9->field_0x20 = 0x3314;
  uVar9->field_0x22 = 0x1018;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_3,
                           (uchar *)(puVar5 >> 0x10),unaff_DI);
  &uVar9->field_0x182 = puVar5;
  (&uVar9->field_0x182 + 0x2) = (puVar5 >> 0x10);
  if (param_1 == (astruct_55 *)0x0) {
    puVar3 = (u16 *)0x0;
    uVar4 = 0x0;
  }
  else {
    puVar3 = &uVar9->field_0x20;
    uVar4 = uVar8;
  }
  puVar1 = uVar9->field_0x182;
  ppcVar2 = (code **)(*uVar9->field_0x182 + 0x4);
  (**ppcVar2)(0x1010,puVar1,(puVar1 >> 0x10),0x0,puVar3,uVar4);
  puVar1 = uVar9->field_0x182;
  uVar9->field_0x17a = (puVar1 + 0x24);
  paVar6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x6e,param_3);
  &uVar9->field_0x24 = paVar6;
  (&uVar9->field_0x24 + 0x2) = (paVar6 >> 0x10);
  uVar9->field_0x28 = 0x0;
  uVar7 = pass1_1008_4772(uVar9->field_0x24);
  uVar4 = (uVar7 >> 0x10);
  pass1_1018_4b78(param_1,param_3);
  uVar9->field_0x16c = 0x1;
  uVar9->field_0x16e = 0x1;
  uVar9->field_0x170 = (uVar7 + 0x4) + uVar9->field_0x16c;
  uVar9->field_0x172 = (uVar7 + 0x8) + -0x19;
  return;
}


fn struct_1018_4720(param_1: *mut u16,param_2: u32,param_3: u32)
{
  astruct_204 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_204 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = param_3;
  iVar1->field_0x8 = param_2;
  iVar1->field_0xc = 0x0;
  *param_1 = &PTR_LOOP_1050_4aa6;
  iVar1->field_0x2 = 0x1018;
  return;
}


fn struct_1018_4790(param_1: *mut u16,param_2: u32,param_3: u32,param_4: u16) -> u16

{
  astruct_266 *iVar1;
  let uVar1: u16;
  
  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_266 *)param_1;
  iVar1->field_0xe = param_4;
  *param_1 = 0x4a92;
  iVar1->field_0x2 = 0x1018;
  iVar1->field_0xc = 0x1;
  return param_1;
}



fn struct_1018_47c8(param_1: *mut u16,param_2: u32,param_3: u32,param_4: u16,param_5: u32)
{
  astruct_264 *iVar1;
  let uVar1: u16;
  
  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_264 *)param_1;
  iVar1->field_0xe = param_5;
  iVar1->field_0x12 = param_4;
  *param_1 = &PTR_LOOP_1050_4a9a;
  iVar1->field_0x2 = 0x1018;
  iVar1->field_0xc = 0x2;
  return;
}


fn struct_1018_4842(param_1: *mut u16,param_2: u32,param_3: u32,param_4: u16) -> u16

{
  astruct_265 *iVar1;
  let uVar1: u16;
  
  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_265 *)param_1;
  iVar1->field_0xe = param_4;
  iVar1->field_0x10 = 0x0;
  *param_1 = &PTR_LOOP_1050_4a8e;
  iVar1->field_0x2 = 0x1018;
  iVar1->field_0xc = 0x4;
  return param_1;
}


fn struct_1018_48b0(param_1: *mut u16,param_2: u32,param_3: u32,param_4: u16) -> u16

{
  astruct_207 *iVar1;
  let uVar1: u16;
  
  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_207 *)param_1;
  iVar1->field_0xe = param_4;
  *param_1 = &PTR_LOOP_1050_4a96;
  iVar1->field_0x2 = 0x1018;
  iVar1->field_0xc = 0x5;
  return param_1;
}



fn struct_1018_48e8(param_1: *mut u16,param_2: u32,param_3: u32,param_4: u16) -> u16

{
  let iVar1: i16;
  let uVar2: u16;
  
  struct_1018_4720(param_1,param_2,param_3);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0xe) = param_4;
  *param_1 = 0x4a9e;
  (iVar1 + 0x2) = 0x1018;
  (iVar1 + 0xc) = 0x6;
  return param_1;
}



fn struct_1018_4920(param_1: *mut u16,param_2: u32,param_3: u32,param_4: u32)
{
  astruct_203 *iVar1;
  let uVar1: u16;
  
  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_203 *)param_1;
  iVar1->field_0xe = param_4;
  *param_1 = &PTR_LOOP_1050_4a8a;
  iVar1->field_0x2 = 0x1018;
  iVar1->field_0xc = 0x7;
  return;
}


fn struct_op_1018_4cda(param_1: i16,param_2: u16,param_3: u16)
{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  (param_1 + 0xa) = 0x0;
  (param_1 + 0xe) = 0x0;
  (param_1 + 0x12) = 0x0;
  (param_1 + 0x14) = 0x0;
  (param_1 + 0x16) = 0x0;
  (param_1 + 0x18) = 0x1;
  (param_1 + 0x1a) = 0x0;
  CONCAT22(param_2,param_1) =
       (s_SCInternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12);
  (param_1 + 0x2) = 0x1018;
  return;
}


fn struct_1018_66cc(astruct_20 *param_1,param_2: u16,param_3: u16,param_4: u16)
{
  let extraout_DX: *mut u8
  let uVar1: u16;
  astruct_20 *iVar2;
  let unaff_DI: i16;
  astruct_20 *uVar2;
  let puVar2: *mut u16;
  
  unk_draw_op_1020_7f7a(param_1,0xa,CONCAT22(param_3,param_2));
  uVar2 = (astruct_20 *)(param_1 >> 0x10);
  iVar2 = (astruct_20 *)param_1;
  &iVar2[0x1].field_0xc = 0x0;
  iVar2[0x1].field_0x10 = 0x0;
  param_1->field_0x0 = 0x6880;
  iVar2->field_0x2 = 0x1018;
  ((astruct_20 *)(iVar2 + 0x1))->field_0x0 = 0x691c;
  iVar2[0x1].field_0x2 = 0x1018;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0xb,param_4,extraout_DX,unaff_DI);
  uVar1 = (puVar2 >> 0x10);
  &iVar2[0x1].field_0x10 = puVar2;
  (&iVar2[0x1].field_0x10 + 0x2) = uVar1;
  &iVar2[0x1].field_0x4 = &iVar2[0x1].field_0x10;
  (&iVar2[0x1].field_0x4 + 0x2) = uVar1;
  return;
}


astruct_20 * 
struct_op_1018_6a0e(astruct_20 *param_1,param_2: u16,param_3: u16,param_4: u16,
                   param_5: u16,param_6: u16,param_7: u32,param_8: u16)

{
  let iVar1: i16;
  let uVar2: u16;
  
  unk_draw_op_1008_61b2(param_1,param_3,param_6,param_7,param_8);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0xea) = param_5;
  (iVar1 + 0xec) = param_4;
  (iVar1 + 0xee) = param_2;
  (iVar1 + 0xf0) = 0x0;
  param_1->field_0x0 = 0x6c66;
  (iVar1 + 0x2) = 0x1018;
  (iVar1 + 0xe0) = 0x1;
  (iVar1 + 0xe2) = 0x0;
  (iVar1 + 0xe4) = 0x0;
  (iVar1 + 0xe6) = 0x1df027f;
  return param_1;
}


astruct_20 * 
struct_1018_6d02(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0xb,0x9c,0x8b,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa27e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_6d38(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0xc,0x9d,0xd0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb562;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_6d6e(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0xd,0x9e,0xd1,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9822;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_6da4(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0xe,0x9f,0xd2,param_2,param_3,param_4);
  param_1->field_0x0 = 0xab06;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_6dda(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0xf,0xa0,0xd4,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbdea;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_6e10(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x10,0xa1,0xda,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa0aa;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_6e46(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x11,0xa2,0xdc,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb38e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_6e7c(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x12,0xa3,0xd3,param_2,param_3,param_4);
  param_1->field_0x0 = 0x964e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_6eb2(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x13,0xa4,0xdb,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa932;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_6ee8(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x14,0xa5,0xa5,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbc16;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_6f1e(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x15,0xa7,0xb2,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9e3a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_6f54(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x16,0xa8,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb11e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_6f8a(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x17,0xaf,0xc0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x93de;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_6fc0(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x18,0xb0,0xc1,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa6c2;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_6ff6(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x19,0xb1,0x80,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb9a6;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_702c(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x1ec,0x1a,0xb2,0xc3,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9c66;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7062(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x1b,0xb3,0xc4,param_2,param_3,param_4);
  param_1->field_0x0 = 0xaf4a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7098(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x1c,0xb4,0xd8,param_2,param_3,param_4);
  param_1->field_0x0 = 0xc22e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_70ce(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x1d,0xb5,0x7b,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa4ee;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7104(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x1e,0xb6,0xd9,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb7d2;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_713a(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x1f,0xb7,0x7d,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9a92;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7170(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x21,0xb9,0xdd,param_2,param_3,param_4);
  param_1->field_0x0 = 0xad76;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_71a6(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x23,0xd3,0xd6,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb69a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_71dc(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x1ed,0x24,0xd4,0xd7,param_2,param_3,param_4);
  param_1->field_0x0 = 0x995a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7212(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x25,0xe9,0xee,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa452;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7248(astruct_20 *param_1,param_2: u16,Uparam_3: i32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x63,0xa6,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xc05a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_727e(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x64,0xa9,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa31a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_72b4(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x65,0xaa,0xbb,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb5fe;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_72ea(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x66,0xab,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x98be;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7320(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x67,0xac,0xbd,param_2,param_3,param_4);
  param_1->field_0x0 = 0xaba2;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7356(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x68,0xad,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbe86;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_738c(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x69,0xae,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xac3e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_73c2(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x35,0xba,0x81,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbf22;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_73f8(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x39,0xbb,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa146;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_745e(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x22,0xbc,0xd5,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb42a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7494(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x36,0xbd,0xcd,param_2,param_3,param_4);
  param_1->field_0x0 = 0x96ea;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_74ca(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x37,0xbe,0x83,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa9ce;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7500(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x38,0xbf,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbcb2;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7536(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x3a,0xc0,0x85,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9f72;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_756c(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x1e2,0x3b,0xc1,0x86,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb256;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
pass1_1018_75a2(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x3c,0xc2,0x87,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9516;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
pass1_1018_75d8(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x3d,0xc3,0x88,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa7fa;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_760e(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x3e,0xc4,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbade;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7644(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x3f,0xc5,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9d02;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_767a(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x40,0xc6,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xafe6;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_76b0(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x41,0xc7,0x8d,param_2,param_3,param_4);
  param_1->field_0x0 = 0xc2ca;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_76e6(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x42,0xc8,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa58a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_771c(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x43,0xc9,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb86e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7752(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x44,0xcc,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9b2e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7788(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x45,0xcd,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xae12;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_77be(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x46,0xd1,0x92,param_2,param_3,param_4);
  param_1->field_0x0 = 0xc0f6;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_77f4(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x47,0xd2,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa3b6;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_782a(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x48,0xd5,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xacda;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7860(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x49,0xd6,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbfbe;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7896(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x1f4,0x4a,0xd7,0x98,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa1e2;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_78cc(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x4b,0xd8,0x99,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb4c6;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7902(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x4c,0xd9,0xee,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9786;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7938(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x4d,0xda,0x9c,param_2,param_3,param_4);
  param_1->field_0x0 = 0xaa6a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_796e(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x4e,0xdb,0x9d,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbd4e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_79a4(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x4f,0xdc,0x9e,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa00e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_79da(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x50,0xdd,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb2f2;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7a10(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x1d9,0x51,0xde,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x95b2;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7a46(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x52,0xdf,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa896;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7a7c(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x53,0xe0,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xbb7a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7ab2(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x1e4,0x55,0xe2,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb082;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7ae8(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x1e4,0x56,0xe3,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xc366;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7b1e(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x1da,0x57,0xe4,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa626;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7b54(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x1d8,0x58,0xe5,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb90a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7b8a(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x59,0xe6,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9bca;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7bc0(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x1ef,0x5a,0xe7,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xaeae;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7bf6(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x5b,0xe8,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xc192;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7c2c(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x5c,0xea,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb736;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7c62(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x5d,0xeb,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x99f6;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7c98(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x1e6,0x5e,0xec,0xee,param_2,param_3,param_4);
  param_1->field_0x0 = 0xba42;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7cce(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x1da,0x5f,0xed,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x9ed6;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7d04(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x0,0x60,0xee,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0xb1ba;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7d3a(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x1f0,0x61,0xef,0x0,param_2,param_3,param_4);
  param_1->field_0x0 = 0x947a;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * 
struct_1018_7d70(astruct_20 *param_1,param_2: u16,param_3: u32,param_4: u16)

{
  struct_op_1018_6a0e(param_1,0x1f7,0x62,0xf0,0xcc,param_2,param_3,param_4);
  param_1->field_0x0 = 0xa75e;
  (param_1 + 0x2) = 0x1018;
  return param_1;
}


void 
struct_1020_0762(astruct_20 *param_1,param_2: u32,param_3: *mut u32,param_4: u16,
                param_5: u32,param_6: u32,param_7: u16)

{
  astruct_20 *iVar1;
  astruct_20 *uVar1;
  astruct_20 *paVar1;
  let uVar2: u16;
  
  paVar1 = (astruct_20 *)param_1;
  uVar2 = (param_1 >> 0x10);
  pass1_1020_01d8(paVar1,uVar2,param_2,(param_2 >> 0x10),param_4,
                  param_5,(param_5 >> 0x10),param_6,param_7);
  paVar1[0x1].field_0xe = 0x0;
  paVar1[0x1].field_0x10 = *param_3;
  param_1->field_0x0 = 0x81a;
  paVar1->field_0x2 = 0x1020;
  return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn struct_1020_0baa(param_1: *mut u16,param_2: u16,uchar *param_3,param_4: u16)
{
  let puVar1: *mut u8
  astruct_276 *iVar2;
  let unaff_DI: i16;
  let uVar2: u16;
  let puVar3: *mut u16;
  let puVar4: *mut u16;
  let puVar5: *mut u16;
  let uVar6: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_276 *)param_1;
  *param_1 = 0x389a;
  iVar2->field_0x2 = 0x1008;
  *param_1 = 0x3aa8;
  iVar2->field_0x2 = 0x1008;
  iVar2->field_0x4 = param_2;
  *param_1 = 0x3ab0;
  iVar2->field_0x2 = 0x1008;
  &iVar2->field_0x6 = 0x0;
  iVar2->field_0xa = 0x0;
  iVar2->field_0xc = 0x0;
  *param_1 = 0xdbc;
  iVar2->field_0x2 = 0x1020;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x7,param_4,param_3,unaff_DI);
  puVar1 = (uchar *)(puVar3 >> 0x10);
  iVar2->field_0x6 = puVar3;
  iVar2->field_0x8 = puVar1;
  puVar5 = &iVar2->field_0xa;
  puVar4 = &iVar2->field_0xc;
  uVar6 = uVar2;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,puVar1,unaff_DI);
  pass1_1008_3e94((u16 *)(puVar3 & 0xffff0000 | (puVar3 + 0xe)),
                  (u16 *)CONCAT22(uVar2,puVar4),(u16 *)CONCAT22(uVar6,puVar5));
  return;
}

