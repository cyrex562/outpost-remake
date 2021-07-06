
fn struct_1020_1738(astruct_57 *param_1,param_2: u16,param_3: u32)
{
  astruct_278 *iVar1;
  let uVar1: u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfcd,(param_3 + 0x8));
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_278 *)param_1;
  iVar1.field_0x8e = 0x0;
  iVar1.field_0x92 = 0x0;
  iVar1.field_0x96 = 0x0;
  param_1 = 0x1e7a;
  iVar1.field_0x2 = 0x1020;
  return;
}


fn struct_1020_2524(astruct_20 *param_1,param_2: u16,param_3: u16,param_4: u16)
{
  let extraout_DX: *mut u8
  let uVar1: u16;
  astruct_20 *iVar2;
  let unaff_DI: i16;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  unk_draw_op_1020_7f7a(param_1,0x7,CONCAT22(param_3,param_2));
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_20 *)param_1;
  &iVar2[0x1].field_0xc = 0x0;
  iVar2[0x1].field_0x10 = 0x0;
  param_1.field_0x0 = 0x270c;
  iVar2.field_0x2 = 0x1020;
  ((astruct_20 *)(iVar2 + 0x1)).field_0x0 = 0x27a8;
  iVar2[0x1].field_0x2 = 0x1020;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2a,param_4,extraout_DX,unaff_DI);
  uVar1 = (puVar3 >> 0x10);
  &iVar2[0x1].field_0x10 = puVar3;
  (&iVar2[0x1].field_0x10 + 0x2) = uVar1;
  &iVar2[0x1].field_0x4 = &iVar2[0x1].field_0x10;
  (&iVar2[0x1].field_0x4 + 0x2) = uVar1;
  return;
}


fn struct_1020_3644(param_1: *mut u16,param_2: u16,param_3: u32,param_4: u16)
{
  astruct_272 *iVar2;
  short in_buf_len_5;
  astruct_270 *iVar1;
  
  struct_1020_790e(param_1,0x0,param_2,param_3,param_4);
  in_buf_len_5 = (short)(param_1 >> 0x10);
  iVar2 = (astruct_272 *)param_1;
  iVar2.field_0xf2 = 0x389a;
  iVar2.field_0xf4 = 0x1008;
  iVar2.field_0xf2 = 0x3aa8;
  iVar2.field_0xf4 = 0x1008;
  iVar2.field_0x100 = 0x0;
  iVar2.field_0x10a = 0x0;
  iVar2.field_0x10e = 0x0;
  *param_1 = 0x3d08;
  iVar2.field_0x2 = 0x1020;
  iVar2.field_0xf2 = 0x3d9c;
  iVar2.field_0xf4 = 0x1020;
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,&iVar2.field_0xa,
             in_buf_len_5);
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | &iVar2.field_0x5b),
             s_VrMode_1050_42ca);
  iVar2.field_0xac = 0x44c00000;
  window_op_1020_38aa(param_1);
  return;
}


fn struct_1020_7554(param_1: u16,astruct_20 *param_2,param_3: u16,param_4: u16)
{
  let extraout_DX: *mut u8
  let uVar1: u16;
  astruct_129 *iVar2;
  let unaff_DI: i16;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  unk_draw_op_1020_7f7a(param_2,0x5,CONCAT22(param_4,param_3));
  uVar2 = (param_2 >> 0x10);
  iVar2 = (astruct_129 *)param_2;
  iVar2.field_0xee = 0x0;
  &iVar2.field_0xf2 = 0x0;
  param_2.field_0x0 = 0x7780;
  iVar2.field_0x2 = 0x1020;
  iVar2.field_0xe2 = 0x781c;
  iVar2.field_0xe4 = 0x1020;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x25,param_1,extraout_DX,unaff_DI);
  uVar1 = (puVar3 >> 0x10);
  iVar2.field_0xf2 = puVar3;
  iVar2.field_0xf4 = uVar1;
  iVar2.field_0xe6 = iVar2.field_0xf2;
  iVar2.field_0xe8 = uVar1;
  return;
}



void 
struct_1020_790e(param_1: *mut u16,param_2: u32,param_3: u16,param_4: u32,param_5: u16
                )

{
  astruct_271 *iVar1;
  let uVar1: u16;
  
  unk_draw_op_1008_7f62(param_1,param_3,param_4,param_5);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_271 *)param_1;
  iVar1.field_0xe0 = 0x0;
  iVar1.field_0xe4 = 0x0;
  iVar1.field_0xe8 = 0x0;
  iVar1.field_0xec = 0x0;
  iVar1.field_0xee = param_2;
  *param_1 = 0x7b86;
  iVar1.field_0x2 = 0x1020;
  return;
}


fn struct_1020_847a(param_1: *mut u16,param_2: i16,param_3: u16)
{
  let uVar1: u16;
  let puVar2: *mut u8
  astruct_280 *iVar3;
  let iVar4: i16;
  let puVar5: *mut u16;
  
  iVar4 = (param_1 >> 0x10);
  iVar3 = (astruct_280 *)param_1;
  *param_1 = 0x389a;
  iVar3.field_0x2 = 0x1008;
  iVar3.field_0x4 = 0x0;
  iVar3.field_0x6 = param_2;
  iVar3.field_0x8 = (astruct_20 *)0x0;
  iVar3.field_0xc = (astruct_20 *)0x0;
  puVar5 = pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(iVar3 + 0x1)));
  *param_1 = 0x87aa;
  iVar3.field_0x2 = 0x1020;
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_3,
                           (puVar5 >> 0x10),iVar4);
  puVar2 = (puVar5 >> 0x10);
  pass1_1008_3f62((param_1 & 0xffff0000 | ZEXT24(iVar3 + 0x1)),
                  (puVar5 & 0xffff0000 | (puVar5 + 0xe)));
  uVar1 = iVar3.field_0x6 << 0x3;
  mem_op_1000_179c(uVar1,puVar2,0x1000);
  &iVar3.field_0x8 = uVar1;
  *(uchar **)(&iVar3.field_0x8 + 0x2) = puVar2;
  uVar1 = iVar3.field_0x6 << 0x2;
  mem_op_1000_179c(uVar1,puVar2,0x1000);
  &iVar3.field_0xc = uVar1;
  *(uchar **)(&iVar3.field_0xc + 0x2) = puVar2;
  pass1_1000_4906(iVar3.field_0x8,(WNDCLASS16 *)0x0,iVar3.field_0x6 << 0x3);
  pass1_1000_4906(iVar3.field_0xc,(WNDCLASS16 *)0x0,iVar3.field_0x6 << 0x2);
  return;
}


fn set_struct_op_1020_921c(astruct_42 *param_1,param_2: u16)
{
  HDC16 HVar1;
  let in_DX: *mut u8
  astruct_42 *iVar3;
  let unaff_DI: i16;
  astruct_42 *uVar3;
  HWND16 unaff_CS;
  let unaff_SS: u16;
  let pUVar3: *mut u16;
  
  uVar3 = (astruct_42 *)(param_1 >> 0x10);
  iVar3 = (astruct_42 *)param_1;
  param_1.field_0x0 = 0x389a;
  iVar3.field_0x2 = 0x1008;
  param_1.field_0x0 = 0x3aa8;
  iVar3.field_0x2 = 0x1008;
  iVar3.field_0x4 = param_2;
  param_1.field_0x0 = 0x3ab0;
  iVar3.field_0x2 = 0x1008;
  iVar3.field_0x6 = 0x0;
  iVar3.field_0xa = 0x0;
  iVar3.field_0xc = 0x0;
  iVar3.field_0xe = 0x0;
  iVar3.field_0x10 = 0x0;
  iVar3.field_0x12 = 0x0;
  param_1.field_0x0 = 0x96c8;
  iVar3.field_0x2 = 0x1020;
  HVar1 = GetDC16(unaff_CS);
  iVar3.field_0xa = HVar1;
  pUVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,unaff_SS,in_DX,unaff_DI);
  pUVar3._2_2_ = (pUVar3 >> 0x10);
  iVar3.field_0xc = (pUVar3 + 0xa);
  iVar3.field_0xe = (pUVar3 + 0xc);
  return;
}



fn struct_1020_c444(astruct_75 *param_1,param_2: u32,param_3: u32)
{
  astruct_75 *iVar1;
  astruct_75 *uVar1;
  
  struct_op_1030_1cd8(param_1,param_2,param_3);
  uVar1 = (astruct_75 *)(param_1 >> 0x10);
  iVar1 = (astruct_75 *)param_1;
  (iVar1 + 0x1) = 0x0;
  &iVar1[0x1].field_0x4 = 0x0;
  param_1.field_0x0 = 0xc834;
  iVar1.field_0x2 = 0x1020;
  return;
}



fn struct_1020_c9ea(param_1: *mut u16) -> u16

{
  struct_1028_0954(param_1);
  *param_1 = 0xcc7c;
  (param_1 + 0x2) = 0x1020;
  return param_1;
}


fn struct_1020_cce4(param_1: *mut u16) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0xcd7e;
  (param_1 + 0x2) = 0x1020;
  return param_1;
}


fn struct_1020_cde6(param_1: *mut u16) -> u16

{
  struct_1028_0954(param_1);
  *param_1 = 0xd004;
  (param_1 + 0x2) = 0x1020;
  return param_1;
}


fn struct_1020_d06c(param_1: *mut u16) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0xd314;
  (param_1 + 0x2) = 0x1020;
  return param_1;
}


fn struct_1020_d37c(param_1: *mut u16) -> u16

{
  let uVar1: u16;
  
  struct_1028_b354(param_1);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x20) = 0x0;
  *param_1 = 0xd53e;
  (param_1 + 0x2) = 0x1020;
  return param_1;
}



fn struct_1020_d5a6(param_1: *mut u16) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0xd7fe;
  (param_1 + 0x2) = 0x1020;
  return param_1;
}



fn struct_1020_d866(param_1: *mut u16) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0xd8ec;
  (param_1 + 0x2) = 0x1020;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn struct_1020_d954(param_1: *mut u16)
{
  let extraout_DX: *mut u8
  astruct_182 *iVar1;
  let unaff_DI: i16;
  let uVar1: u16;
  let unaff_SS: u16;
  let puVar2: *mut u16;
  
  struct_1030_dc96(param_1);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_182 *)param_1;
  iVar1.field_0x24 = 0x0;
  iVar1.field_0x26 = 0x0;
  &iVar1.field_0x28 = 0x0;
  *param_1 = 0xe792;
  iVar1.field_0x2 = 0x1020;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,unaff_SS,extraout_DX,unaff_DI);
  iVar1.field_0x28 = puVar2;
  iVar1.field_0x2a = (puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 * 
struct_1020_d99e(param_1: *mut u16,param_2: u16,param_3: i16,param_4: u32,param_5: u16,
                param_6: u16)

{
  let unaff_DI: i16;
  let puVar1: *mut u16;
  let uVar2: u16;
  astruct_178 *iVar2;
  
  iVar2 = (astruct_178 *)param_1;
  uVar2 = (param_1 >> 0x10);
  puVar1 = pass1_1030_dcc2(iVar2,uVar2,param_3,param_4,param_5);
  iVar2.field_0x24 = param_2;
  iVar2.field_0x26 = 0x0;
  &iVar2.field_0x28 = 0x0;
  *param_1 = 0xe792;
  iVar2.field_0x2 = 0x1020;
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,
                           (puVar1 >> 0x10),unaff_DI);
  iVar2.field_0x28 = puVar1;
  iVar2.field_0x2a = (puVar1 >> 0x10);
  iVar2.field_0x10 = 0x49;
  return param_1;
}



fn struct_1020_e7fa(param_1: *mut u16) -> u16

{
  struct_1028_b354(param_1);
  *param_1 = 0xe88e;
  (param_1 + 0x2) = 0x1020;
  return param_1;
}



fn struct_1020_e8f6(param_1: *mut u16) -> u16

{
  let uVar1: u16;
  
  struct_1030_dc96(param_1);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x24) = 0x0;
  *param_1 = 0xeef6;
  (param_1 + 0x2) = 0x1020;
  return param_1;
}

