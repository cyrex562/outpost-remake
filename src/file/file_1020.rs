
bool 
read_file_1020_a65e(param_1: u32,param_2: u32,param_3: u16,
                   param_4: u16)

{
  let BVar1: bool;
  let in_DX: u16;
  let local_a: [u8;2];
  let local_8: [u8;2];
  let local_6: [u8;2];
  let local_4: [u8;2];
  u16_t uVar3;
  u16_t uVar2;
  
  uVar2 = (u16_t)param_2;
  uVar3 = (u16_t)(param_2 >> 0x10);
  read_file_1008_7cfe(uVar2,uVar3,0xb,0x1008,param_3);
  if (param_4 != 0x0) {
    if (0x1 < PTR_LOOP_1050_0312) {
LAB_1020_a6dc:
      pass1_1020_b97e(param_3,param_4,in_DX,param_1,
                      (param_1 >> 0x10),0x0);
      return 0x1;
    }
    BVar1 = read_file_1008_7dee(uVar2,uVar3,local_4,0x0,param_3,0x2,0x1008);
    if (BVar1 != 0x0) {
      BVar1 = read_file_1008_7dee(uVar2,uVar3,local_8,0x0,param_3,0x2,0x1008);
      if (BVar1 != 0x0) {
        BVar1 = read_file_1008_7dee(uVar2,uVar3,local_6,0x0,param_3,0x2,0x1008);
        if (BVar1 != 0x0) {
          param_4 = read_file_1008_7dee(uVar2,uVar3,local_a,0x0,param_3,0x2,0x1008
                                       );
          if (param_4 != 0x0) goto LAB_1020_a6dc;
        }
      }
    }
    PTR_LOOP_1050_0310 = 0x6d2;
  }
  return 0x0;
}



fn write_to_file_1020_d3d4(param_1: u32,param_2: u32,param_3: u16) -> bool

{
  let BVar1: bool;
  let local_c: [u16;0x5];
  
  BVar1 = write_to_file_1028_b5ec(param_1,param_2,param_3);
  if (BVar1 != 0x0) {
    local_c[0] = (param_1 + 0x20);
    BVar1 = write_to_file_1008_7e1c
                      (param_2,(param_2 >> 0x10),local_c,param_3,
                       0x2,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = 0x6d0;
      return BVar1;
    }
    BVar1 = 0x1;
  }
  return BVar1;
}


fn write_to_file_1020_e6a4(param_1: u32,param_2: u32,param_3: u16) -> bool

{
  let in_AX: i16;
  let BVar1: bool;
  let uVar2: u16;
  let uVar3: u16;
  let local_c: [u16;0x3];
  let local_6: [u16;0x2];
  
  pass1_1030_de7c(param_1,param_2,param_3);
  if (in_AX != 0x0) {
    uVar2 = (param_1 >> 0x10);
    local_c[0] = (param_1 + 0x24);
    uVar3 = (param_2 >> 0x10);
    BVar1 = write_to_file_1008_7e1c
                      (param_2,uVar3,local_c,param_3,0x2,0x1008);
    if (BVar1 != 0x0) {
      local_6[0] = (param_1 + 0x26);
      BVar1 = write_to_file_1008_7e1c
                        (param_2,uVar3,local_6,param_3,0x2,0x1008)
      ;
      if (BVar1 != 0x0) {
        return 0x1;
      }
    }
    PTR_LOOP_1050_0310 = 0x6d0;
  }
  return 0x0;
}
