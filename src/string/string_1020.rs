

fn string_1020_79b4(param_1: u16,param_2: u32,param_3: i16,char *param_4)
{
  unk_str_op_1000_3d3e
            ((param_2 & 0xffff0000 | (param_2 + 0xa)),param_4);
  if (param_3 != 0x0) {
    draw_op_1020_7cc8(*(ULONG *)(param_2 + 0xe8),0x1000,param_1);
  }
  return;
}


fn string_op_1020_c2f8(param_1: u16) -> *mut u8

{
  char *pcVar1;
  
  switch(param_1) {
  case 0x1:
    break;
  case 0x2:
    break;
  case 0x3:
    break;
  case 0x4:
    break;
  case 0x5:
    break;
  case 0x6:
    break;
  case 0x7:
    break;
  case 0x8:
    break;
  case 0x9:
    break;
  case 0xa:
    break;
  case 0xb:
    break;
  case 0xc:
    break;
  case 0xd:
    break;
  case 0xe:
    break;
  case 0xf:
    break;
  case 0x10:
  }
  pcVar1 = load_string_1010_847e
                     (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  return pcVar1;
}

