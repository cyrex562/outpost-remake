


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
mixed_struct_op_1040_8fb8
          (param_1: *mut u16,param_2: u16,char *param_3,param_4: u16,param_5: u16,
          param_6: u16,param_7: u16,param_8: u16,param_9: u16,LPVOID param_10,
          param_11: u16)

{
  let uVar1: u16;
  let uVar2: u16;
  LPVOID pvVar3;
  let iVar4: i16;
  let uVar5: u16;
  astruct_43 *paVar6;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0x389a;
  (iVar4 + 0x2) = 0x1008;
  (iVar4 + 0x4) = 0x0;
  (iVar4 + 0x8) = 0x0;
  (iVar4 + 0xc) = 0x0;
  (iVar4 + 0x10) = 0x0;
  (iVar4 + 0x14) = 0x0;
  (iVar4 + 0x18) = 0x0;
  (iVar4 + 0x1a) = param_8;
  (iVar4 + 0x1c) = param_7;
  (iVar4 + 0x36) = 0x5;
  (iVar4 + 0x38) = 0x0;
  (iVar4 + 0x3a) = 0x0;
  (iVar4 + 0x3c) = 0x2;
  (iVar4 + 0x3e) = 0x0;
  (iVar4 + 0x40) = param_2;
  *param_1 = 0x9800;
  (iVar4 + 0x2) = &ctx.PTR_LOOP_1050_1040;
  uVar1 = (iVar4 + 0x36);
  (iVar4 + 0x28) = uVar1;
  (iVar4 + 0x26) = uVar1;
  (iVar4 + 0x2c) = 0x0;
  (iVar4 + 0x2a) = 0x0;
  if ((param_6 != 0x0) && (param_5 != 0x0)) {
    (iVar4 + 0x38) = 0x1;
    paVar6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,param_6,param_11);
    (iVar4 + 0x8) = paVar6;
    (iVar4 + 0xa) = (paVar6 >> 0x10);
    param_10 = (LPVOID)0x1010;
    paVar6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,param_5,param_11);
    param_9 = (paVar6 >> 0x10);
    (iVar4 + 0xc) = paVar6;
    (iVar4 + 0xe) = param_9;
    if (param_4 == 0x0) {
      (iVar4 + 0x10) = 0x0;
    }
    else {
      param_10 = (LPVOID)0x1010;
      paVar6 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,param_4,param_11);
      param_9 = (paVar6 >> 0x10);
      (iVar4 + 0x10) = paVar6;
      (iVar4 + 0x12) = param_9;
    }
  }
  uVar1 = (iVar4 + 0x36);
  (iVar4 + 0x30) = uVar1;
  (iVar4 + 0x2e) = uVar1;
  (iVar4 + 0x32) = 0x0;
  if (param_3 != 0x0) {
    param_10 = (LPVOID)0x1008;
    uVar2 = str_op_1008_60e8(param_3,param_9);
    (iVar4 + 0x4) = uVar2;
    (iVar4 + 0x6) = param_9;
  }
  (iVar4 + 0x22) = 0x0;
  (iVar4 + 0x1e) = 0x0;
  (iVar4 + 0x20) = 0x0;
  if (_PTR_LOOP_1050_5e18 == 0x0) {
    pvVar3 = MakeProcInstance16(param_10,(HANDLE16)PTR_LOOP_1050_038c);
    _PTR_LOOP_1050_5e18 = CONCAT22(param_9,pvVar3);
  }
  ctx.PTR_LOOP_1050_5e16 = ctx.PTR_LOOP_1050_5e16 + 0x1;
  return;
}



fn struct_1040_a598(param_1: *mut u16)
{
  astruct_259 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_259 *)param_1;
  *param_1 = 0x0;
  iVar1.field_0x2 = 0x0;
  iVar1.field_0x6 = 0x0;
  iVar1.field_0xa = 0x0;
  iVar1.field_0xc = 0x0;
  iVar1.field_0x10 = 0x0;
  iVar1.field_0x12 = 0x0;
  iVar1.field_0x14 = 0x0;
  iVar1.field_0x16 = 0x0;
  return;
}



fn struct_1040_b082(astruct_57 *param_1,param_2: u32)
{
  astruct_437 *iVar1;
  let uVar1: u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,param_2,(param_2 >> 0x10));
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_437 *)param_1;
  iVar1.field_0x8e = 0x0;
  iVar1.field_0x90 = 0x0;
  param_1 = 0xb772;
  iVar1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
  return;
}

