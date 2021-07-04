
fn debug_print_1008_6048(param_1: u32,LPSTR param_2,param_3: *mut u16)
{
  let in_DX: u16;
  let unaff_ES: u16;
  ulet in_AF: u8;
  let uStack266: u16;
  LPCSTR pCStack6;
  let args: *mut u16;
  
  if (PTR_LOOP_1050_02ec != 0x0) {
    pCStack6 = &stack0x0008;
    args = param_3;
    if (DAT_1050_02ee == 0xffff) {
      param_2 = (LPSTR)&PTR_LOOP_1050_1000;
      uStack266 = pass1_1000_3ec0(0x2f4,&USHORT_1050_1050);
      DAT_1050_02ee = ((in_DX | uStack266) != 0x0);
    }
    if (DAT_1050_02ee != 0x0) {
      wvsprintf16(param_2,pCStack6,args);
      OutputDebugString16((LPCSTR)s_tile2_bmp_1050_1538);
      OutputDebugString16((LPCSTR)s_tile2_bmp_1050_1538);
      if (_PTR_LOOP_1050_02f0 != 0x0) {
        pass1_1000_2b5c(_PTR_LOOP_1050_02f0,
                        (_PTR_LOOP_1050_02f0 >> 0x10),0x2fd,
                        &USHORT_1050_1050,unaff_ES,&stack0xfffe,0x1000,
                        param_3);
        pass1_1000_2f48(_PTR_LOOP_1050_02f0,&stack0xfffe,unaff_ES,0x1000,
                        param_3,in_AF);
      }
    }
  }
  return;
}

