
fn msg_box_op_1000_1f24(param_1: i16,param_2: u16,param_3: u16,param_4: u16) -> bool

{
  let piVar1: *mut i16;
  let unaff_CS: u16;
  
  if (param_3 < (param_1 + 0xc)) {
    msg_box_op_1000_214c(0x0,0x0,0xd940,&PTR_LOOP_1050_1040,param_4);
    return 0x1;
  }
  piVar1 = (i16 *)(param_1 + 0xc);
  *piVar1 = *piVar1 + 0x1;
  return 0x0;
}


fn msg_box_op_1000_214c
          (param_1: u16,param_2: i16,param_3: u16,param_4: u16,param_5: u16) -> bool

{
  let IVar1: i16;
  let iVar2: i16;
  LPCSTR text;
  
  text = (LPCSTR)(0x2 - (param_2 == 0x0) | 0x2110);
  MessageBeep16(param_5);
  do {
    IVar1 = MessageBox16((HWND16)s_tile2_bmp_1050_1538,text,(LPCSTR)0x1de8,0x1000);
    iVar2 = IVar1 + -0x1;
    if (iVar2 == 0x0) {
      return 0x0;
    }
    if ((0x0 < iVar2) && (!SBORROW2(iVar2,0x1))) {
      if (IVar1 == 0x3 || IVar1 + -0x2 < 0x1) {
        fatal_app_exit_1000_3e9e(s_tile2_bmp_1050_1538);
        return 0x0;
      }
      if (IVar1 == 0x4) {
        return 0x1;
      }
      if (IVar1 == 0x5) {
        return 0x0;
      }
    }
    if ((text & 0x2000) == 0x0) {
      return 0x0;
    }
    text = (LPCSTR)(text & 0xdfef | 0x1010);
  } while( true );
}