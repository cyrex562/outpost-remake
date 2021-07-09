

pub fn string_1020_79b4(param_1: u16,param_2: u32,param_3: i16,param_4: &mut String)
{
  unk_str_op_1000_3d3e
            ((param_2 & 0xffff0000 | (param_2 + 0xa)),param_4);
  if (param_3 != 0x0) {
    draw_op_1020_7cc8(*(ULONG *)(param_2 + 0xe8),0x1000,param_1);
  }
  return;
}


pub fn string_op_1020_c2f8(param_1: u16) -> *mut u8

{
  let mut pcVar1: String; 
  
  switch(param_1) {
  0x1 =>
    break;
  0x2 =>
    break;
  0x3 =>
    break;
  0x4 =>
    break;
  0x5 =>
    break;
  0x6 =>
    break;
  0x7 =>
    break;
  0x8 =>
    break;
  0x9 =>
    break;
  0xa =>
    break;
  0xb =>
    break;
  0xc =>
    break;
  0xd =>
    break;
  0xe =>
    break;
  0xf =>
    break;
  0x10 =>
  }
  pcVar1 = load_string_1010_847e
                     (ctx.PTR__LOOP_1050_14cc,(ctx.PTR__LOOP_1050_14cc >> 0x10)
                      ,0x1010);
  return pcVar1;
}

