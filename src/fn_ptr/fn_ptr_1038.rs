fn call_fn_ptr_1038_9ffa(HWND16 param_1,param_2: u16,param_3: i16,param_4: u16) -> u16

{
  code **ppcVar1;
  astruct_43 *iVar2;
  astruct_43 *puVar2;
  HDC16 local_4;
  let uVar2: u16;
  
  uVar2 = (param_3 + 0x6);
  local_4 = GetDC16(param_1);
  puVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x3,param_2);
  iVar2 = (astruct_43 *)puVar2;
  ppcVar1 = (code **)&iVar2->field_0x8;
  (**ppcVar1)(0x1010,puVar2,(puVar2 >> 0x10),&local_4,param_2,uVar2);
  ppcVar1 = (code **)&iVar2->field_0x4;
  (**ppcVar1)(0x1010,puVar2,0x50005,&local_4,param_2);
  ppcVar1 = (code **)&iVar2->field_0xc;
  (**ppcVar1)(0x1010,puVar2,&local_4,param_2);
  ReleaseDC16(0x1010,local_4);
  return 0x0;
}