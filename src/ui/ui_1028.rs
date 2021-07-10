
pub fn post_msg_1028_76da()
{
  let lVar1: i32;
  let uVar2: u16;
  let in_DX: *mut u8;
  let unaff_DI: i16;
  let unaff_SS: u16;
  let puVar3: *mut u16;
  let uStack10: u16;
  let uStack8: u16;
  
  puVar3 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0,0x2c,unaff_SS,in_DX,unaff_DI);
 // uVar2 = (puVar3 >> 0x10);
  lVar1 = (puVar3 + 0xc);
 // uStack8 = (lVar1 >> 0x10);
  uStack10 = lVar1;
  if (((uStack8 | uStack10) != 0x0) && (*_PTR_LOOP_1050_65e2 == lVar1)) {
    PostMessage16(0x1010,0x0,0x0,0x1110106);
    (puVar3 + 0xc) = 0x0;
  }
  return;
}


pub fn send_msg_1028_e242(param_1: *mut u32,param_2: i16,param_3: HWND16)
{
  let puVar1: *mut u8;
  let unaff_DI: i16;
  let unaff_SS: u16;
  LRESULT LVar2;
  
  puVar1 = (*param_1 % 0x64);
  if (*param_1 % 0x64 == 0x0) {
    LVar2 = SendMessage16(param_3,0x0,0x0,0x410000);
   // puVar1 = (LVar2 >> 0x10);
  }
  *param_1 = *param_1 + 0x1;
  if (param_2 != 0x0) {
    pass1_1028_e28a(puVar1,unaff_DI,unaff_SS);
  }
  return;
}
