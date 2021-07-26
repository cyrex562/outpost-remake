
u16 * 
switch_1030_0000(param_1: u16,param_2: u16,param_3: i16,param_4: U32Ptr,param_5: u16,
                param_6: u16,param_7: u16)

{
  let pu_var1: U32Ptr;
  let u_var2: u16;
  let u_var3: u16;
  let puVar4: U32Ptr;
  
                    // Segment:    7
                    // Offset:     000516c0
                    // Length:     ef76
                    // Min Alloc:  ef76
                    // Flags:      0d50
                    //     Code
                    //     Moveable
                    //     Preload
                    //     Impure (Non-shareable)
                    // 
  puVar4 = CONCAT22(param_4,param_5);
  u_var2 = param_3 - 0x1;
  if (false) {
switchD_1030_069f_caseD_19:
    mem_op_1000_179c(0x20,param_4,0x1000);
    u_var3 = param_4 | u_var2;
    if (u_var3 == 0x0) {
      return 0x0;
    }
    struct_1028_b354(CONCAT22(param_4,u_var2));
    return CONCAT22(u_var3,u_var2);
  }
  u_var2 = param_5;
  switch(param_3 - 0x1) {
  0x0 =>
  0x1 =>
  0x2 =>
  0x3 =>
  0x4 =>
  0x5 =>
  0x6 =>
  0x7 =>
  0x8 =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_489e(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x9 =>
    mem_op_1000_179c(0x22,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_2bdc(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0xa =>
    mem_op_1000_179c(0x26,param_4,0x1000);
    u_var2 = param_4 | param_5;
//     TODO: goto joined_r0x103002a1;
  0xb =>
    mem_op_1000_179c(0x2c,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_3670(CONCAT22(param_4,param_5),
                                (param_4 | param_5),param_6,param_7);
      return puVar4;
    }
    break;
  0xc =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_355e(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0xd =>
    mem_op_1000_179c(0x26,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_3484(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0xe =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_406c(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0xf =>
  0x32 =>
  0x33 =>
  0x5f =>
  0x60 =>
    mem_op_1000_179c(0x24,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_0c24(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x10 =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_0b42(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x11 =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_4354(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x12 =>
  0x13 =>
  0x14 =>
  0x61 =>
  0x62 =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_4b84(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x15 =>
  0x16 =>
  0x17 =>
    mem_op_1000_179c(0x24,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_1bbc(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  default:
//     TODO: goto switchD_1030_069f_caseD_19;
  0x1a =>
  0x1b =>
  0x1c =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1030_be34(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x1d =>
  0x1e =>
  0x1f =>
    mem_op_1000_179c(0x26,param_4,0x1000);
    pu_var1 = (param_4 | param_5);
    if (pu_var1 != 0x0) {
      struct_1028_0068(CONCAT22(param_4,param_5),pu_var1);
      return CONCAT22(pu_var1,param_5);
    }
    break;
  0x20 =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_50d8(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x21 =>
  0x22 =>
  0x23 =>
    mem_op_1000_179c(0x24,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_3e94(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x24 =>
  0x25 =>
  0x26 =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1020_d06c(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x27 =>
  0x28 =>
  0x5c =>
  0x5d =>
  0x5e =>
    mem_op_1000_179c(0x22,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1030_c6f6(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x29 =>
  0x2a =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1020_cce4(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x2b =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_26b4(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x2c =>
  0x2d =>
    mem_op_1000_179c(0x2a,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_49aa(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x2e =>
  0x2f =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1020_e7fa(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x30 =>
  0x31 =>
  0x6b =>
  0x6c =>
    mem_op_1000_179c(0x22,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1020_d37c(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x34 =>
  0x35 =>
    mem_op_1000_179c(0x2c,param_4,0x1000);
    pu_var1 = (param_4 | param_5);
    if (pu_var1 != 0x0) {
      struct_1028_37a6(CONCAT22(param_4,param_5),pu_var1,param_6,param_7);
      return CONCAT22(pu_var1,param_5);
    }
    break;
  0x36 =>
    mem_op_1000_179c(0x26,param_4,0x1000);
    u_var2 = param_4 | param_5;
joined_r0x103002a1:
    if (u_var2 != 0x0) {
      struct_1030_c06e(CONCAT22(param_4,param_5));
      return CONCAT22(u_var2,param_5);
    }
    break;
  0x37 =>
  0x38 =>
    mem_op_1000_179c(0x9a,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1030_c9a8(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x39 =>
  0x3a =>
    mem_op_1000_179c(0x24,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_60bc(CONCAT22(param_4,param_5),param_5,
                                (param_4 | param_5));
      return puVar4;
    }
    break;
  0x3b =>
  0x3c =>
    mem_op_1000_179c(0x24,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_44d2(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x3d =>
    mem_op_1000_179c(0x22,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1020_cde6(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x3e =>
    mem_op_1000_179c(0x26,param_4,0x1000);
    pu_var1 = (param_4 | param_5);
    if (pu_var1 != 0x0) {
      struct_1028_1f56(CONCAT22(param_4,param_5),pu_var1);
      return CONCAT22(pu_var1,param_5);
    }
    break;
  0x3f =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_25da(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x40 =>
    mem_op_1000_179c(0x22,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1020_c9ea(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x46 =>
  0x69 =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1020_d5a6(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x47 =>
  0x48 =>
  0x49 =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1020_d866(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x4b =>
  0x4c =>
  0x4d =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1030_d8f6(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x4e =>
  0x4f =>
  0x50 =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_5c54(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x51 =>
  0x52 =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_5966(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x53 =>
  0x54 =>
  0x55 =>
    mem_op_1000_179c(0x22,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_5ed8(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x56 =>
  0x57 =>
  0x58 =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_53c6(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x59 =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = pass1_1028_5884(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x5a =>
  0x5b =>
    mem_op_1000_179c(0x26,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = pass1_1028_5524(CONCAT22(param_4,param_5),
                               (param_4 | param_5));
      return puVar4;
    }
    break;
  0x63 =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = pass1_1028_5df6(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x64 =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_5a48(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x65 =>
  0x66 =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_52e8(param_5,param_4);
      return puVar4;
    }
    break;
  0x67 =>
  0x68 =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_57a6(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x6d =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_5630(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x6f =>
  0x70 =>
  0x71 =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) == 0x0) {
      puVar4 = 0x0;
    }
    else {
      puVar4 = struct_1020_d866(CONCAT22(param_4,param_5));
    }
  0x72 =>
  0x76 =>
    mem_op_1000_179c(0x26,(puVar4 >> 0x10),0x1000);
    if (puVar4 != 0x0) {
      puVar4 = struct_1020_e8f6(puVar4);
      return puVar4;
    }
    break;
  0x73 =>
  0x77 =>
  0x78 =>
    mem_op_1000_179c(0x2c,param_4,0x1000);
    u_var2 = param_4 | param_5;
    if (u_var2 != 0x0) {
      struct_1020_d954(CONCAT22(param_4,param_5));
      return CONCAT22(u_var2,param_5);
    }
    break;
  0x74 =>
    mem_op_1000_179c(0x24,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_178c(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x75 =>
    mem_op_1000_179c(0x24,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_2afa(CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  0x79 =>
  0x7a =>
  0x7b =>
  0x7c =>
  0x7d =>
  0x7e =>
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_27f0(CONCAT22(param_4,param_5));
      return puVar4;
    }
  }
  return 0x0;
}



u16 * 
switch_1030_07ac(param_1: u16,param_2: u16,param_3: u16,param_4: u16,
                param_5: u16,param_6: u32,param_7: U32Ptr,param_8: &mut Struct179,
                param_9: u16,param_10: u16,param_11: u16)

{
  let paVar1: &mut Struct179;
  let u_var2: u16;
  let pu_var3: U32Ptr;
  let u_var4: u16;
  let pu_var5: U32Ptr;
  
  pu_var5 = CONCAT22(param_7,param_8);
  paVar1 = (param_4 - 0x1);
  if (false) {
switchD_1030_0fa3_caseD_19:
    mem_op_1000_179c(0x20,param_7,0x1000);
    u_var4 = param_7 | paVar1;
    if (u_var4 == 0x0) {
      return 0x0;
    }
    pass1_1028_b39e(CONCAT22(param_7,paVar1),param_4,param_6,u_var4);
    return CONCAT22(u_var4,paVar1);
  }
  paVar1 = param_8;
  switch((param_4 - 0x1)) {
  0x0 =>
  0x1 =>
  0x2 =>
  0x3 =>
  0x4 =>
  0x5 =>
  0x6 =>
  0x7 =>
  0x8 =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_48c0(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x9 =>
    mem_op_1000_179c(0x22,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_2bfe(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0xa =>
    mem_op_1000_179c(0x26,param_7,0x1000);
    u_var2 = param_7 | param_8;
//     TODO: goto joined_r0x10300adb;
  0xb =>
    mem_op_1000_179c(0x2c,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_3692(param_8,param_7,param_4,param_6,
                               (param_7 | param_8),param_9,param_10);
      return pu_var5;
    }
    break;
  0xc =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_3580(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0xd =>
    mem_op_1000_179c(0x26,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_34a6(param_8,param_7,param_4,param_6,
                               (param_7 | param_8));
      return pu_var5;
    }
    break;
  0xe =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_408e(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0xf =>
  0x32 =>
  0x33 =>
  0x5f =>
  0x60 =>
    mem_op_1000_179c(0x24,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_0c50(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x10 =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_0b64(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x11 =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_4376(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x12 =>
  0x13 =>
  0x14 =>
  0x61 =>
  0x62 =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_4ba6(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x15 =>
  0x16 =>
  0x17 =>
    mem_op_1000_179c(0x24,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_1be8(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  default:
//     TODO: goto switchD_1030_0fa3_caseD_19;
  0x1a =>
  0x1b =>
  0x1c =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1030_be56(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x1d =>
  0x1e =>
  0x1f =>
    mem_op_1000_179c(0x26,param_7,0x1000);
    pu_var3 = (param_7 | param_8);
    if (pu_var3 != 0x0) {
      pass1_1028_00cc(param_8,param_7,param_4,param_6,pu_var3);
      return CONCAT22(pu_var3,param_8);
    }
    break;
  0x20 =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_50fa(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x21 =>
  0x22 =>
  0x23 =>
    mem_op_1000_179c(0x24,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 =
               pass1_1028_3ec8(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x24 =>
  0x25 =>
  0x26 =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1020_d08e(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x27 =>
  0x28 =>
  0x5c =>
  0x5d =>
  0x5e =>
    mem_op_1000_179c(0x22,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1030_c71e(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x29 =>
  0x2a =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1020_cd06(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x2b =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_26d6(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x2c =>
  0x2d =>
    mem_op_1000_179c(0x2a,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 =
               pass1_1028_49de(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x2e =>
  0x2f =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1020_e81c(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x30 =>
  0x31 =>
  0x6b =>
  0x6c =>
    mem_op_1000_179c(0x22,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1020_d3a4(CONCAT22(param_7,param_8),param_3,param_4,param_6
                               ,param_7 | param_8);
      return pu_var5;
    }
    break;
  0x34 =>
  0x35 =>
    mem_op_1000_179c(0x2c,param_7,0x1000);
    pu_var3 = (param_7 | param_8);
    if (pu_var3 != 0x0) {
      pass1_1028_3816(param_8,param_7,param_4,param_6,pu_var3,param_9,param_10
                     );
      return CONCAT22(pu_var3,param_8);
    }
    break;
  0x36 =>
    mem_op_1000_179c(0x26,param_7,0x1000);
    u_var2 = param_7 | param_8;
joined_r0x10300adb:
    if (u_var2 != 0x0) {
      pass1_1030_c09c(param_8,param_7,param_4,param_6,u_var2);
      return CONCAT22(u_var2,param_8);
    }
    break;
  0x37 =>
  0x38 =>
    mem_op_1000_179c(0x9a,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 =
               pass1_1030_c9e4(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x39 =>
  0x3a =>
    mem_op_1000_179c(0x24,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 =
               pass1_1028_611e(param_8,param_7,param_4,param_6,param_8,
                               (param_7 | param_8));
      return pu_var5;
    }
    break;
  0x3b =>
  0x3c =>
    mem_op_1000_179c(0x24,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_44fe(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x3d =>
    mem_op_1000_179c(0x22,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1020_ce08(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x3e =>
    mem_op_1000_179c(0x26,param_7,0x1000);
    pu_var3 = (param_7 | param_8);
    if (pu_var3 != 0x0) {
      pass1_1028_1fc8(param_8,param_7,param_4,param_6,pu_var3);
      return CONCAT22(pu_var3,param_8);
    }
    break;
  0x3f =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_25fc(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x40 =>
    mem_op_1000_179c(0x22,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1020_ca0c(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x46 =>
  0x69 =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1020_d5c8(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x47 =>
  0x48 =>
  0x49 =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1020_d888(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x4b =>
  0x4c =>
  0x4d =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 =
               pass1_1030_d942(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x4e =>
  0x4f =>
  0x50 =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_5c76(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x51 =>
  0x52 =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_5988(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x53 =>
  0x54 =>
  0x55 =>
    mem_op_1000_179c(0x22,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_5f00(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x56 =>
  0x57 =>
  0x58 =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_53e8(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x59 =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_58a6(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x5a =>
  0x5b =>
    mem_op_1000_179c(0x26,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_5546(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x63 =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_5e18(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x64 =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_5a6a(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x65 =>
  0x66 =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_530a(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x67 =>
  0x68 =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_57c8(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x6d =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_5652(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x6f =>
  0x70 =>
  0x71 =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) == 0x0) {
      pu_var5 = 0x0;
    }
    else {
      pu_var5 = pass1_1020_d888(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
    }
  0x72 =>
  0x76 =>
    mem_op_1000_179c(0x26,(pu_var5 >> 0x10),0x1000);
   // u_var4 = (pu_var5 >> 0x10);
    if (pu_var5 != 0x0) {
      pu_var5 = pass1_1020_e91e(pu_var5,u_var4,param_4,param_6,u_var4 | pu_var5);
      return pu_var5;
    }
    break;
  0x73 =>
  0x77 =>
  0x78 =>
    mem_op_1000_179c(0x2c,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = struct_1020_d99e(CONCAT22(param_7,param_8),param_3,param_4,
                                param_6,param_7 | param_8,param_11);
      return pu_var5;
    }
    break;
  0x74 =>
    mem_op_1000_179c(0x24,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_17ae(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x75 =>
    mem_op_1000_179c(0x24,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_2b1c(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
    break;
  0x79 =>
  0x7a =>
  0x7b =>
  0x7c =>
  0x7d =>
  0x7e =>
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      pu_var5 = pass1_1028_2812(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return pu_var5;
    }
  }
  return 0x0;
}

