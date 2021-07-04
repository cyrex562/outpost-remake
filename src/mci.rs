
fn mci_send_command_1008_53ae(param_1: u32,param_2: u16,param_3: u16,param_4: u16)

{
  let DVar1: u32;
  CHAR local_432 [0x400];
  let local_32: u16;
  let uStack48: u16;
  let local_2e: u16;
  let uStack44: u16;
  let uStack34: u16;
  let uStack32: u16;
  let local_1e: u32;
  let iStack26: i16;
  let uStack22: u16;
  let uStack20: u16;
  let uStack18: u32;
  let uStack14: u32;
  let uStack10: u16;
  let uStack8: u16;
  let uStack6: u16;
  
  local_1e = 0x0;
  uStack22 = 0x28c;
  uStack20 = SUB42(&USHORT_1050_1050,0x0);
  uStack18 = param_1;
  uStack14 = 0x0;
  uStack10 = 0x0;
  uStack8 = 0x4000;
  uStack6 = param_2;
  DVar1 = mciSendCommand16(param_3,&local_1e,CONCAT22(0x200,param_4),0x8030003);
  uStack32 = (DVar1 >> 0x10);
  uStack34 = DVar1;
  if (iStack26 != 0x0) {
    if ((uStack32 | uStack34) != 0x0) {
      mciGetErrorString16(0x4001538,local_432,param_4);
    }
    pass1_1000_4906((astruct_20 *)CONCAT22(param_4,&local_2e),(WNDCLASS16 *)0x0,0xc);
    local_2e = param_2;
    uStack44 = 0x0;
    DVar1 = mciSendCommand16(0x1000,&local_2e,CONCAT22(0x2,param_4),0x8060000);
    uStack32 = (DVar1 >> 0x10);
    uStack34 = DVar1;
    if ((uStack32 | uStack34) != 0x0) {
      mciGetErrorString16(0x4001538,local_432,param_4);
    }
    local_32 = param_2;
    uStack48 = 0x0;
    DVar1 = mciSendCommand16(s_tile2_bmp_1050_1538,&local_32,
                             CONCAT22(0x1,param_4),0x8040000);
    uStack32 = (DVar1 >> 0x10);
    uStack34 = DVar1;
    if ((uStack32 | uStack34) != 0x0) {
      mciGetErrorString16(0x4001538,local_432,param_4);
    }
  }
  return;
}


fn mci_send_command_1008_5cb6(param_1: u32,param_2: i16,param_3: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  let unaff_SS: u16;
  
  mciSendCommand16(param_3,0x0,0x0,0x8040000);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if (((iVar1 + 0xa) == 0x0) || ((iVar1 + 0xa) != param_2)) {
    (iVar1 + 0x12) = 0x0;
    iVar1 = 0x11;
  }
  else {
    (iVar1 + 0x10) = 0x0;
    iVar1 = 0x10;
  }
  pass1_1010_1f62(unaff_SS,param_1 & 0xffff | uVar2 << 0x10,iVar1);
  return;
}
