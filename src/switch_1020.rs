
fn switch_1020_c3b4(param_1: u16) -> u16

{
  let uStack6: u16;
  
  uStack6 = 0x1;
  switch(param_1) {
  case 0x1:
  case 0x2:
  case 0x3:
  case 0x5:
  case 0x8:
  case 0x9:
  case 0xa:
  case 0xb:
  case 0xc:
    uStack6 = 0x3;
    break;
  case 0x4:
    uStack6 = 0x6;
    break;
  case 0x6:
  case 0xf:
  case 0x10:
  case 0x11:
  case 0x12:
  case 0x13:
    uStack6 = 0xa;
    break;
  case 0x7:
    uStack6 = 0x2;
    break;
  case 0xd:
  case 0xe:
    uStack6 = 0x1;
  }
  return uStack6;
}

