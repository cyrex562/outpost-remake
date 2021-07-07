pub fn call_fn_ptr_1038_9ffa(param_1: HWND16,param_2: u16,param_3: i16,param_4: u16) -> u16

{
  let ppcVar1: u32;
  let iVar2: &mut Struct43;
  let puVar2: &mut Struct43;
  HDC16 local_4;
  let uVar2: u16;
  
  uVar2 = (param_3 + 0x6);
  local_4 = GetDC16(param_1);
  puVar2 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc,0x3,param_2);
  iVar2 = puVar2;
  ppcVar1 = &iVar2.field_0x8;
  (**ppcVar1)(0x1010,puVar2,(puVar2 >> 0x10),&local_4,param_2,uVar2);
  ppcVar1 = &iVar2.field_0x4;
  (**ppcVar1)(0x1010,puVar2,0x50005,&local_4,param_2);
  ppcVar1 = &iVar2.field_0xc;
  (**ppcVar1)(0x1010,puVar2,&local_4,param_2);
  ReleaseDC16(0x1010,local_4);
  return 0x0;
}