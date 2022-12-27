pub fn pass1_1010_4994(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    param_2 = (param_2 & 0xffff0000 | (param_2 - 0x20));
    pass1_1010_3f00(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub fn pass1_1010_49a0(mut param_1: i16, mut param_2: u16) -> u32 {
    return CONCAT22(param_2, param_1 + 0xa);
}

pub fn pass1_1010_49b0(mut param_1: i16, mut param_2: u16) -> u32 {
    return CONCAT22(param_2, param_1 + 0x18);
}

pub fn pass1_1010_49c0(mut param_1: u32) -> u16 {
    return (param_1 + 0x14);
}

pub fn pass1_1010_49ce(mut param_1: u32, mut param_2: u16) {
    (param_1 + 0x14) = param_2;
    return;
}

pub fn pass1_1010_49e0(mut param_1: u32) -> u16 {
    return (param_1 + 0x16);
}

pub fn pass1_1010_49ee(mut param_1: u32, mut param_2: u16) {
    (param_1 + 0x16) = param_2;
    return;
}

pub fn pass1_1010_4a00(mut param_1: u32, mut param_2: u16) {
    (param_1 + 0x12) = param_2;
    return;
}



pub fn pass1_1010_4a12(mut param_1: u32) -> u16 {
    return (param_1 + 0x12);
}

pub fn pass1_1010_4a20(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_3f00(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1010_4c3e(
    mut param_1: u32,
    mut param_2: i16,
    mut param_3: i16,
    param_4: *mut u8,
    mut param_5: u16,
) {
    let mut piVar1: *mut i16;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut uVar4: u32;
    let mut iVar5: i16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut unaff_CS: u16;
    let mut uVar9: u32;
    let mut iStack14: i16;
    let mut local_c: [u8; 0x6] = [0; 0x6];
    let mut uStack6: u16;
    let mut iStack4: i16;

    uVar4 = CONCAT22(in_register_0000000a, param_4);
    uVar7 = (param_1 >> 0x10);
    iVar5 = param_1;
    pass1_1010_bffa(param_3, param_4, (iVar5 + 0x26));
    (iVar5 + 0x12) = param_3;
    (iVar5 + 0x14) = uVar4;
    if ((uVar4 | (iVar5 + 0x12)) != 0) {
        if (param_2 == 0) {
            uVar4 = (iVar5 + 0x12);
            (iVar5 + 0x30) = (uVar4 + 0x8);
        } else {
            (iVar5 + 0x2e) = 0x1;
            uVar2 = (iVar5 + 0x12);
            uVar2 = (uVar2 + 0x4);
            iVar6 = (uVar2 + 2);
            if ((iVar6 == 0x5) || (iVar6 == 0x6)) {
                (iVar5 + 0x30) = 0x1;
                (iVar5 + 0x20) = 0;
            } else {
                (iVar5 + 0x30) = 0x2;
                uVar2 = (iVar5 + 0x12);
                uVar2 = (uVar2 + 0x4);
                (iVar5 + 0x32) = uVar2;
                uVar3 = FUN_1010_830a(uVar2, uVar4, unaff_CS, _u16_1050_14cc, 0x1bf);
                uVar2 = (iVar5 + 0x12);
                uVar8 = (uVar2 >> 0x10);
                iVar6 = uVar2;
                (iVar6 + 0x4) = uVar3;
                (iVar6 + 0x6) = uVar4;
            }
        }
        iStack4 = 0x14;
        pass1_1008_3e38(CONCAT22(0x1050, local_c));
        uStack6 = 0;
        iStack14 = 0;
        loop {
            piVar1 = (iVar5 + 0x30);
            if (*piVar1 == iStack14 || *piVar1 < iStack14) {
                break;
            }
            uVar4 = (iVar5 + 0x12);
            uVar9 = pass1_1008_4772((uVar4 + iStack14 * 0x4));
            iStack4 += (-(iStack14 == 0) & 0x5) + 0x14 + (uVar9 + 0x4);
            iStack14 += 0x1;
        }
        if ((iVar5 + 0xe) < iStack4) {
            (iVar5 + 0xe) = iStack4;
        }
    }
    return;
}


// WARNING: This is an inlined function


pub fn struct_1010_4d5c(
    param_1: *mut u8,
    param_2: *mut astruct_245,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: i16,
) {
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar3: *mut astruct_245;
    let mut iVar5: *mut astruct_747;
    let mut uVar6: u16;
    let mut uVar2: u32;
    let mut uVar1: u32;

    paVar5 = CONCAT22(in_register_0000000a, param_1);
    uVar6 = (param_2 >> 0x10);
    iVar3 = param_2;
    if (&iVar3.field_0x1a == 0) {
        iVar4 = iVar3.field47_0x30 << 0x3;
        mem_op_1000_179c(iVar4, paVar5);
        iVar3.field_0x1a = iVar4;
        iVar3.field28_0x1c = paVar5;
    }
    uVar2 = &iVar3.field_0x1a;
    iVar5 = (param_7 * 0x8);
    (iVar5 + uVar2) = param_6;
    uVar3 = &iVar3.field_0x1a;
    (iVar5 + uVar3 + 0x2) = param_5;
    uVar1 = &iVar3.field_0x1a;
    (iVar5 + uVar1 + 0x4) = param_4;
    uVar3 = &iVar3.field_0x1a;
    (iVar5 + uVar3 + 0x6) = param_3;
    return;
}

pub fn pass1_1010_5004(param_1: *mut StructD, param_2: u8, mut param_3: u16) -> *mut u16 {
    free_rsrc_1010_4b3e(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return &param_1.address_offset_field_0x0;
}

pub fn pass1_1010_5074(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    clenaup_win_ui_1018_4d22(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1010_53ce(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_50f2(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1010_5dc6(mut param_1: u32, mut param_2: u32) -> u16 {
    let mut BVar1: bool;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut in_stack_0000ffdc: HFILE16;
    let mut local_c: [*mut u8; 3];
    let mut local_6: [u16; 0x2] = [0; 0x2];

    BVar1 = write_to_file_1008_7cac(param_2);
    if (BVar1 != 0) {
        uVar3 = (param_1 >> 0x10);
        iVar2 = param_1;
        BVar1 = pass1_1008_7c2a(param_2, *(iVar2 + 0x68));
        if (BVar1 != 0) {
            BVar1 = pass1_1008_7c2a(param_2, *(iVar2 + 0x6c));
            if (BVar1 != 0) {
                local_c[0] = PTR_LOOP_1050_13ae;
                BVar1 = write_to_file_1008_7e1c(
                    param_2,
                    CONCAT22(0x1050, local_c),
                    0x2,
                    in_stack_0000ffdc,
                );
                if (BVar1 != 0) {
                    local_6[0] = (iVar2 + 0x82);
                    BVar1 = write_to_file_1008_7e1c(
                        param_2,
                        CONCAT22(0x1050, local_6),
                        0x2,
                        in_stack_0000ffdc,
                    );
                    if (BVar1 != 0) {
                        return 0x1;
                    }
                }
            }
        }
        u16_1050_0310 = 0x6d0;
    }
    return 0x0;
}


pub fn pass1_1010_5e56(mut param_1: i16, mut param_2: u16, mut param_3: u32, mut param_4: u32) {
    let mut puVar1: *mut u8;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut HVar6: HFILE16;
    let mut uVar7: u16;
    let mut local_404: *mut u8;
    let mut local_402: [u8; 0x400] = [0; 0x400];

    HVar6 = param_4;
    uVar7 = (param_4 >> 0x10);
    read_file_1008_7cfe(HVar6, uVar7, 0x4);
    if (param_1 == 0) {
        u16_1050_0310 = 0x6d4;
    } else {
        puVar1 = local_402;
        read_file_1008_7c6e(HVar6, uVar7, CONCAT22(0x1050, puVar1));
        if (puVar1.is_null() == false) {
            uVar2 = str_op_1008_60e8(param_2, CONCAT22(0x1050, local_402));
            uVar5 = (param_3 >> 0x10);
            iVar4 = param_3;
            (iVar4 + 0x68) = uVar2;
            (iVar4 + 0x6a) = param_2;
            puVar1 = local_402;
            read_file_1008_7c6e(HVar6, uVar7, CONCAT22(0x1050, puVar1));
            if (puVar1.is_null() == false) {
                uVar2 = str_op_1008_60e8(param_2, CONCAT22(0x1050, local_402));
                (iVar4 + 0x6c) = uVar2;
                (iVar4 + 0x6e) = param_2;
                BVar3 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_404), 0x2);
                if (BVar3 != 0) {
                    PTR_LOOP_1050_13ae = local_404;
                    if (u16_1050_0312 < 0x2) {
                        return;
                    }
                    BVar3 =
                        read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (iVar4 + 0x82)), 0x2);
                    if (BVar3 != 0) {
                        return;
                    }
                }
            }
        }
        u16_1050_0310 = 0x6d2;
    }
    return;
}


pub fn write_to_file_1010_6372(param_1: *mut Struct729,mut param_2: u32)

{
  let mut BVar1: bool;
  let mut iVar2: *mut Struct729;
  let mut uVar2: u16;
  let mut in_stack_0000ffce: HFILE16;
  //let mut local_10: [u32;0x2] = [0;0x2];
  let mut local_10: [u32;2] = [0;2];
  let mut local_8: u32;

  BVar1 = write_to_file_1008_7cac(param_2);
  if (BVar1 != 0) {
    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    local_10[0] = iVar2.field10_0xa;
    BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_10),0x4,in_stack_0000ffce);
    if (BVar1 != 0) {
      local_8 = iVar2.field11_0xe;
      BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffce);
      if (BVar1 != 0) {
        local_8 = iVar2.field12_0x12;
        BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffce);
        if (BVar1 != 0) {
          local_8 = iVar2.field13_0x16;
          BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffce);
          if (BVar1 != 0) {
            local_8 = iVar2.field14_0x1a;
            BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffce);
            if (BVar1 != 0) {
              local_8 = iVar2.field15_0x1e;
              BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffce);
              if (BVar1 != 0) {
                local_8 = iVar2.field16_0x22;
                BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffce);
                if (BVar1 != 0) {
                  return;
                }
              }
            }
          }
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}


pub fn pass1_1010_648a(mut param_1: i16,mut param_2: u32,mut param_3: u32)

{
  let mut iVar1: i16;
  let mut BVar2: bool;

  read_file_1008_7cfe(param_3,(param_3 >> 0x10),0x7);
  if (param_1 != 0) {
    iVar1 = param_2;
    BVar2 = read_file_1008_7dee(param_3,(param_2 & 0xffff0000 | (iVar1 + 0xa)),0x4);
    if (BVar2 != 0) {
      BVar2 = read_file_1008_7dee(param_3,(param_2 & 0xffff0000 | (iVar1 + 0xe)),0x4);
      if (BVar2 != 0) {
        BVar2 = read_file_1008_7dee(param_3,(param_2 & 0xffff0000 | (iVar1 + 0x12)),0x4);
        if (BVar2 != 0) {
          BVar2 = read_file_1008_7dee(param_3,(param_2 & 0xffff0000 | (iVar1 + 0x16)),0x4);
          if (BVar2 != 0) {
            BVar2 = read_file_1008_7dee(param_3,(param_2 & 0xffff0000 | (iVar1 + 0x1a)),0x4);
            if (BVar2 != 0) {
              BVar2 = read_file_1008_7dee(param_3,(param_2 & 0xffff0000 | (iVar1 + 0x1e)),0x4);
              if (BVar2 != 0) {
                BVar2 = read_file_1008_7dee(param_3,(param_2 & 0xffff0000 | (iVar1 + 0x22)),0x4)
                ;
                if (BVar2 != 0) {
                  return;
                }
              }
            }
          }
        }
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}

pub fn  pass1_1010_66ca(param_1: *mut u16,param_2: u8) -> *mut u16

{
  pass1_1010_1d80(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

pub fn write_to_file_1010_6846(mut param_1: u32,param_2: *mut u8)

{
  let mut BVar1: bool;
  let mut iVar2: i16;
  let mut in_stack_0000ffde: HFILE16;
//   let mut local_c: [u16;0x5] = [0;0x5];
    let mut local_c: [u16;5] = [0;5];
  BVar1 = write_to_file_1008_7cac(param_2);
  if (BVar1 != 0) {
    iVar2 = param_1;
    BVar1 = write_to_file_1008_7e1c(param_2,param_1 & 0xffff0000 | (iVar2 + 0xa),0x114,in_stack_0000ffde)
    ;
    if (BVar1 != 0) {
      BVar1 = write_to_file_1008_7e1c
                        (param_2,param_1 & 0xffff0000 | (iVar2 + 0x11e),0x2a,in_stack_0000ffde);
      if (BVar1 != 0) {
        local_c[0] = (iVar2 + 0x148);
        BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_c),0x2,in_stack_0000ffde);
        if (BVar1 != 0) {
          return;
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}


pub fn pass1_1010_68c6(mut param_1: u16 ,param_2: *mut u8,mut param_3: u32,mut param_4: u32)

{
  let mut iVar2: *mut astruct_248;
  let mut BVar1: bool;
  let mut iVar3: i16;
  let mut BVar4: bool;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  let mut paVar7: *mut Struct57;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut pcStack18: *mut c_char;
  let mut pcStack10: *mut c_char;
  let mut local_6: i16;
  let mut uStack4: u16;

  paVar7 = CONCAT22(in_register_0000000a,param_2);
  uVar9 = param_4;
  uVar10 = (param_4 >> 0x10);
  read_file_1008_7cfe(uVar9,uVar10,0x3);
  if (param_1 == 0) {
    u16_1050_0310 = 0x6d4;
    return;
  }
  iVar2 = param_3;
  uVar8 = (param_3 >> 0x10);
  if (u16_1050_0312 < 0x2) {
    uVar11 = 0x102;
    uVar12 = 0;
    mem_op_1000_179c(0x102,paVar7);
    uVar6 = paVar7;
    pcStack10 = CONCAT22(uVar6,param_1);
    BVar1 = read_file_1008_7dee(param_4,CONCAT22(uVar6,param_1),CONCAT22(uVar12,uVar11));
    pcStack18 = pcStack10;
    // if (BVar1 == 0) goto LAB_1010_692c;
    uStack4 = 0x1;
    loop {
      iVar3 = switch_1008_73ea(uVar9,uVar10,uStack4);
      (&iVar2.field_0xa + iVar3 * 0x2) = (uStack4 * 0x2 + param_1);
      uStack4 += 0x1;
      if uStack4 >= 0x81 {break;}
    }
    fn_ptr_1000_17ce(pcStack10);
    BVar1 = pcStack10;
  }
  else {
    BVar1 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar2.field_0xa)),0x114);
    if (BVar1 == 0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
  }
  if (u16_1050_0312 < 0x2) {
    uVar11 = 0x2a;
    uVar12 = 0;
    mem_op_1000_179c(0x2a,paVar7);
    uVar6 = paVar7;
    pcStack18 = CONCAT22(uVar6,BVar1);
    BVar4 = read_file_1008_7dee(param_4,CONCAT22(uVar6,BVar1),CONCAT22(uVar12,uVar11));
    if (BVar4 == 0) {//
// LAB_1010_692c:
      u16_1050_0310 = 0x6d2;
      fn_ptr_1000_17ce((pcStack18 & 0xffff | uVar6 << 0x10));
      return;
    }
    uStack4 = 0;
    loop {
      uVar5 = switch_1008_72bc(param_4,uStack4);
      (&iVar2.field_0x11e + uVar5 * 0x2) = (uStack4 * 0x2 + BVar1);
      uStack4 += 0x1;
      if uStack4 >= 0x15 {break;}
    }
    fn_ptr_1000_17ce(pcStack18);
  }
  else {
    BVar1 = read_file_1008_7dee(param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar2.field_0x11e)),0x2a);
    if (BVar1 == 0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
  }
  BVar1 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_6),0x2);
  if (BVar1 == 0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  BVar1 = switch_1008_73ea(uVar9,uVar10,local_6);
  iVar2.field328_0x148 = BVar1;
  return;
}

pub fn pass1_1010_6a86(param_1: *mut u16,param_2: u8) -> *mut u16

{
  pass1_1010_1d80(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

pub fn FUN_1010_702e() -> u16 {
    let mut in_AX: u16;
    let mut in_DX: u16;
    let mut unaff_BP: i16;

    pass1_1010_715c(in_AX, in_DX, (unaff_BP + 0x6), 0x3c);
    pass1_1010_715c(in_AX, in_DX, (unaff_BP + 0x6), 0x3e);
    return 0x1;
}

pub fn FUN_1010_703e() -> u16 {
    return 0x0;
}

pub fn FUN_1010_7041() {
    return;
}

pub fn FUN_1010_7174(mut param_1: u16, mut param_2: u32, mut param_3: u16) {
    let mut uVar1: u32;
    let mut in_DX: u16;
    let mut iVar2: i16;
    let mut uVar3: u16;

    iVar2 = param_2;
    uVar3 = (param_2 >> 0x10);
    if (param_3 == 0x13) {
        uVar1 = (iVar2 + 0x18);
        destroy_window_1010_7b26(in_DX, param_2 & 0xffff0000 | (iVar2 - 0xa), (uVar1 + 0x28));
        return;
    }
    if (param_3 < 0x14) {
        if (param_3 == '\x01') {
            (iVar2 + 0xa) = 0;
            (iVar2 + 0x18) = 0;
            return;
        }
        if (param_3 == '\x05') {
            send_msg_1010_7c42(param_2 & 0xffff0000 | (iVar2 - 0xa));
            return;
        }
    }
    return;
}

pub fn pass1_1010_71b0() {
    let mut uVar1: u32;
    let mut unaff_BP: i16;

    uVar1 = (unaff_BP + 0x6);
    send_msg_1010_7c42(uVar1 & 0xffff0000 | (uVar1 - 0xa));
    return;
}


pub fn pass1_1010_71c2(mut param_1: u16, mut param_2: u16) {
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut unaff_BP: i16;
    let mut uVar4: u16;

    if (param_1 == 0x13) {
        uVar2 = (unaff_BP + 0x6);
        uVar2 = (uVar2 + 0x18);
        uVar1 = (unaff_BP + 0x6);
        destroy_window_1010_7b26(param_2, uVar1 & 0xffff0000 | (uVar1 - 0xa), (uVar2 + 0x28));
        return;
    }
    if (param_1 < 0x14) {
        if (param_1 == '\x01') {
            uVar2 = (unaff_BP + 0x6);
            uVar4 = (uVar2 >> 0x10);
            iVar3 = uVar2;
            (iVar3 + 0xa) = 0;
            (iVar3 + 0x18) = 0;
            return;
        }
        if (param_1 == '\x05') {
            uVar1 = (unaff_BP + 0x6);
            send_msg_1010_7c42(uVar1 & 0xffff0000 | (uVar1 - 0xa));
            return;
        }
    }
    return;
}


pub fn pass1_1010_7dc6(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xa));
    pass1_1010_6bb2(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub fn pass1_1010_7dd2(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn FUN_1010_7dfe(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1010_6bb2(&param_2.address_offset_field_0x0);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub fn pass1_1010_8ebc(mut param_1: u32, param_2: u8) -> u32 {
    pass1_1010_8c78(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1010_922e(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1010_8f78(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1010_9304(
    mut param_1: u16,
    param_2: *mut u8,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: i16,
) {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;

    paVar1 = CONCAT22(in_register_0000000a, param_2);
    if (param_5 != 0) {
        mem_op_1000_179c(param_5 << 0x2, paVar1);
        return;
    }
    mem_op_1000_179c(0x1a, paVar1);
    if ((paVar1 | param_1) != 0) {
        pass1_1010_9258(CONCAT22(paVar1, param_1));
        return;
    }
    return;
}


pub fn pass1_1010_9348(mut param_1: u32, mut param_2: i16) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    (param_2 * 0x8 + 0x319e) = param_2;
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x16) = param_2 * 0x8 + 0x3198;
    (iVar1 + 0x18) = 0x1050;
    (iVar1 + 0x12) = param_2;
    return;
}

pub fn pass1_1010_9372(
    param_1: u32,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: i16,
    mut param_5: i16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut cVar4: u8;
    let mut in_EDX: *mut Struct57;
    let mut uVar5: u16;
    let mut unaff_SI: u16;
    let mut uVar6: u16;
    let mut bVar7: bool;
    let mut uVar8: u32;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc8: u16;

    if (0x0 < param_4) {
        if (_PTR_LOOP_1050_3528.is_null()) {
            ppcVar1 = (*param_1 + 0x18);
            uVar3 = (**ppcVar1)();
            _PTR_LOOP_1050_3528 = mixed_1010_20ba(
                in_EDX,
                _u16_1050_0ed0,
                CONCAT22(unaff_SI, uVar3),
                in_stack_0000fe9a,
                in_stack_0000ffbe,
                in_stack_0000ffc4,
                in_stack_0000ffc8,
            );
        }
        uVar2 = (param_1 + 0xc);
        uVar8 = pass1_1010_2e02(_PTR_LOOP_1050_3528, (uVar2 + 0x12));
        uVar5 = param_2 + 1;
        uVar6 = param_3 + (0xfffe < param_2);
        // for (cVar4 = (param_4 -1) * '\x04'; cVar4 != '\0'; cVar4 += -1)

        cVar4 = param_4 - 1 * '\x04';
        while cVar4 != '\0' {
            bVar7 = CARRY2(uVar5, uVar5);
            uVar5 *= 0x2;
            uVar6 = uVar6 * 0x2 + bVar7;
            cVar4 -= 1;
        }
        pass1_1010_2e30(
            _PTR_LOOP_1050_3528,
            uVar5 | uVar8,
            uVar6 | (uVar8 >> 0x10),
            param_5,
        );
    }
    return;
}


pub fn pass1_1010_93f0(mut param_1: u32) {
    let mut puVar1: *mut u8;
    let mut uVar2: u16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut paVar5: *mut astruct_223;
    let mut local_1c: [u8; 0x1a] = [0; 0x1a];

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x56) == 0) {
        paVar5 = pass1_1010_9258(CONCAT22(0x1050, local_1c));
        uVar2 = (paVar5 >> 0x10);
        puVar1 = local_1c;
        pass1_1010_398e(puVar1, CONCAT22(0x1050, puVar1), 0x0, 0x0, 0x0);
        (iVar3 + 0x56) = puVar1;
        (iVar3 + 0x58) = uVar2;
        pass1_1010_927a(CONCAT22(0x1050, local_1c));
    }
    return;
}


pub fn load_string_1010_9432() -> *mut c_char {
    let mut pcVar1: *mut c_char;
    let mut in_stack_00000004: u32;

    pcVar1 = load_string_1010_847e(_u16_1050_14cc, *(in_stack_00000004 + 0x16));
    return pcVar1;
}

pub fn pass1_1010_944e(mut param_1: u16, mut param_2: u16, mut param_3: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;

    if ((param_1 + 0x56) == 0) {
        ppcVar1 = (CONCAT22(param_2, param_1) + 0x10);
        (**ppcVar1)();
    }
    uVar2 = pass1_1010_2e02(CONCAT22(param_2, param_1), param_3);
    pass1_1010_2e5c(param_1, param_2, uVar2);
    return;
}

pub fn FUN_1010_9482() -> u16 {
    return 0x0;
}


pub fn pass1_1010_9488(
    param_1: *mut u8,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u32,
) -> bool {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fe98: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut uVar6: u16;
    let mut in_stack_0000ffee: u32;

    uVar6 = (param_4 + 0x12);
    puVar5 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        CONCAT22((in_stack_0000ffee >> 0x10), 0x3),
        in_stack_0000fe98,
        in_stack_0000ffbc,
        in_stack_0000ffc2,
        in_stack_0000ffc6,
    );
    uVar1 = (puVar5 >> 0x10);
    uVar2 = puVar5;
    uVar3 = uVar6 - 0x32;
    uVar4 = uVar1;
    if (uVar3 == 0) {
        pass1_1010_a5ca(0x0, uVar1, uVar2, uVar1, 0x32);
        if (uVar3 != 0) {
            return false;
        }
        uVar6 = 0x4d;
    } else {
        uVar3 = uVar6 - 0x3f;
        if (uVar3 == 0) {
            pass1_1010_a5ca(0x0, uVar1, uVar2, uVar1, 0x3f);
            if (uVar3 != 0) {
                return false;
            }
            uVar6 = 0x4e;
        }
    }
    pass1_1010_a5ca(uVar3, uVar4, uVar2, uVar1, uVar6);
    return uVar3 == 0;
}

pub fn pass1_1010_9502(mut param_1: u32) -> u16 {
    let mut uVar1: u32;

    uVar1 = (param_1 + 0x16);
    return (uVar1 + 2);
}

pub fn pass1_1010_9514() -> u16 {
    return 0x31;
}

pub fn pass1_1010_951a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1010_927a(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1010_9540(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_92e6(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
