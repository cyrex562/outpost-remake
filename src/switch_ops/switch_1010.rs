
pub fn  switchD_1010:2ab5::caseD_13(param_1: u32,param_2: i16)

{
  let uVar1: u32;
  let iVar2: i16;
  let unaff_SS: u16;
  let in_AF: u8;
  
  iVar2 = param_2 * 0x8 + param_1;
  if (((((iVar2 + 0x22) != 0x0) || ((iVar2 + 0x24) != 0x0)) ||
      ((iVar2 + 0x26) != 0x0)) || ((iVar2 + 0x28) != 0x0)) {
    uVar1 = (param_1 + 0xe);
    sys_1000_3f9c(uVar1,(uVar1 >> 0x10),
                  s__d__d__d__d_1050_14ae,ctx.data_seg,
                  (param_2 * 0x8 + param_1 + 0x22),
                  &stack0xfffe,param_1._2_2_,0x1000,unaff_SS,in_AF);
    uVar1 = (param_1 + 0xa);
    WritePrivateProfileString16
              (&ctx.PTR_LOOP_1050_1000,uVar1,(uVar1 >> 0x10),
               (param_1 + 0xe));
  }
  return;
}


pub fn switch_1010_6646(param_1: u16,param_2: u16,param_3: *mut u16,param_4: u16)
{
  switch(param_4) {
  0x83 =>
    *param_3 = 0xa;
    break;
  0x84 =>
    *param_3 = 0xe;
    break;
  0x85 =>
    *param_3 = 0x12;
    break;
  0x86 =>
    *param_3 = 0x16;
    return;
  0x87 =>
    *param_3 = 0x1a;
    return;
  0x88 =>
    *param_3 = 0x1e;
    return;
  0x89 =>
    *param_3 = 0x22;
    return;
  default:
    *param_3 = 0x0;
    return;
  }
  return;
}
