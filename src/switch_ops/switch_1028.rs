
u16 * 
switch_1030_0000(param_1: u16,param_2: u16,param_3: i16,uchar *param_4,param_5: u16,
                param_6: u16,param_7: u16)

{
  let puVar1: *mut u8
  let uVar2: u16;
  let uVar3: u16;
  let puVar4: *mut u16;
  
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
  uVar2 = param_3 - 0x1U;
  if (false) {
switchD_1030_069f_caseD_19:
    mem_op_1000_179c(0x20,param_4,0x1000);
    uVar3 = param_4 | uVar2;
    if (uVar3 == 0x0) {
      return 0x0;
    }
    struct_1028_b354((u16 *)CONCAT22(param_4,uVar2));
    return CONCAT22(uVar3,uVar2);
  }
  uVar2 = param_5;
  switch(param_3 - 0x1U) {
  case 0x0:
  case 0x1:
  case 0x2:
  case 0x3:
  case 0x4:
  case 0x5:
  case 0x6:
  case 0x7:
  case 0x8:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_489e((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x9:
    mem_op_1000_179c(0x22,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_2bdc((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0xa:
    mem_op_1000_179c(0x26,param_4,0x1000);
    uVar2 = param_4 | param_5;
    goto joined_r0x103002a1;
  case 0xb:
    mem_op_1000_179c(0x2c,param_4,0x1000);
    if ((uchar *)(param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_3670((u16 *)CONCAT22(param_4,param_5),
                                (param_4 | param_5),param_6,param_7);
      return puVar4;
    }
    break;
  case 0xc:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_355e((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0xd:
    mem_op_1000_179c(0x26,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_3484((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0xe:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_406c((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0xf:
  case 0x32:
  case 0x33:
  case 0x5f:
  case 0x60:
    mem_op_1000_179c(0x24,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_0c24((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x10:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_0b42((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x11:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_4354((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x12:
  case 0x13:
  case 0x14:
  case 0x61:
  case 0x62:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_4b84((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x15:
  case 0x16:
  case 0x17:
    mem_op_1000_179c(0x24,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_1bbc((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  default:
    goto switchD_1030_069f_caseD_19;
  case 0x1a:
  case 0x1b:
  case 0x1c:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1030_be34((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x1d:
  case 0x1e:
  case 0x1f:
    mem_op_1000_179c(0x26,param_4,0x1000);
    puVar1 = (param_4 | param_5);
    if (puVar1 != 0x0) {
      struct_1028_0068((u16 *)CONCAT22(param_4,param_5),puVar1);
      return CONCAT22(puVar1,param_5);
    }
    break;
  case 0x20:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_50d8((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x21:
  case 0x22:
  case 0x23:
    mem_op_1000_179c(0x24,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_3e94((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x24:
  case 0x25:
  case 0x26:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1020_d06c((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x27:
  case 0x28:
  case 0x5c:
  case 0x5d:
  case 0x5e:
    mem_op_1000_179c(0x22,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1030_c6f6((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x29:
  case 0x2a:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1020_cce4((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x2b:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_26b4((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x2c:
  case 0x2d:
    mem_op_1000_179c(0x2a,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_49aa((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x2e:
  case 0x2f:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1020_e7fa((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x30:
  case 0x31:
  case 0x6b:
  case 0x6c:
    mem_op_1000_179c(0x22,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1020_d37c((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x34:
  case 0x35:
    mem_op_1000_179c(0x2c,param_4,0x1000);
    puVar1 = (param_4 | param_5);
    if (puVar1 != 0x0) {
      struct_1028_37a6((u16 *)CONCAT22(param_4,param_5),puVar1,param_6,param_7);
      return CONCAT22(puVar1,param_5);
    }
    break;
  case 0x36:
    mem_op_1000_179c(0x26,param_4,0x1000);
    uVar2 = param_4 | param_5;
joined_r0x103002a1:
    if (uVar2 != 0x0) {
      struct_1030_c06e((u16 *)CONCAT22(param_4,param_5));
      return CONCAT22(uVar2,param_5);
    }
    break;
  case 0x37:
  case 0x38:
    mem_op_1000_179c(0x9a,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1030_c9a8((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x39:
  case 0x3a:
    mem_op_1000_179c(0x24,param_4,0x1000);
    if ((uchar *)(param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_60bc((u16 *)CONCAT22(param_4,param_5),param_5,
                                (param_4 | param_5));
      return puVar4;
    }
    break;
  case 0x3b:
  case 0x3c:
    mem_op_1000_179c(0x24,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_44d2((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x3d:
    mem_op_1000_179c(0x22,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1020_cde6((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x3e:
    mem_op_1000_179c(0x26,param_4,0x1000);
    puVar1 = (param_4 | param_5);
    if (puVar1 != 0x0) {
      struct_1028_1f56((u16 *)CONCAT22(param_4,param_5),puVar1);
      return CONCAT22(puVar1,param_5);
    }
    break;
  case 0x3f:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_25da((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x40:
    mem_op_1000_179c(0x22,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1020_c9ea((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x46:
  case 0x69:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1020_d5a6((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x47:
  case 0x48:
  case 0x49:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1020_d866((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x4b:
  case 0x4c:
  case 0x4d:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1030_d8f6((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x4e:
  case 0x4f:
  case 0x50:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_5c54((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x51:
  case 0x52:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_5966((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x53:
  case 0x54:
  case 0x55:
    mem_op_1000_179c(0x22,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_5ed8((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x56:
  case 0x57:
  case 0x58:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_53c6((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x59:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = pass1_1028_5884((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x5a:
  case 0x5b:
    mem_op_1000_179c(0x26,param_4,0x1000);
    if ((uchar *)(param_4 | param_5) != 0x0) {
      puVar4 = pass1_1028_5524((u16 *)CONCAT22(param_4,param_5),
                               (param_4 | param_5));
      return puVar4;
    }
    break;
  case 0x63:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = pass1_1028_5df6((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x64:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_5a48((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x65:
  case 0x66:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_52e8(param_5,param_4);
      return puVar4;
    }
    break;
  case 0x67:
  case 0x68:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_57a6((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x6d:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_5630((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x6f:
  case 0x70:
  case 0x71:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) == 0x0) {
      puVar4 = 0x0;
    }
    else {
      puVar4 = struct_1020_d866((u16 *)CONCAT22(param_4,param_5));
    }
  case 0x72:
  case 0x76:
    mem_op_1000_179c(0x26,(puVar4 >> 0x10),0x1000);
    if (puVar4 != 0x0) {
      puVar4 = struct_1020_e8f6(puVar4);
      return puVar4;
    }
    break;
  case 0x73:
  case 0x77:
  case 0x78:
    mem_op_1000_179c(0x2c,param_4,0x1000);
    uVar2 = param_4 | param_5;
    if (uVar2 != 0x0) {
      struct_1020_d954((u16 *)CONCAT22(param_4,param_5));
      return CONCAT22(uVar2,param_5);
    }
    break;
  case 0x74:
    mem_op_1000_179c(0x24,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_178c((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x75:
    mem_op_1000_179c(0x24,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_2afa((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
    break;
  case 0x79:
  case 0x7a:
  case 0x7b:
  case 0x7c:
  case 0x7d:
  case 0x7e:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if ((param_4 | param_5) != 0x0) {
      puVar4 = struct_1028_27f0((u16 *)CONCAT22(param_4,param_5));
      return puVar4;
    }
  }
  return 0x0;
}



u16 * 
switch_1030_07ac(param_1: u16,param_2: u16,param_3: u16,param_4: u16,
                param_5: u16,param_6: u32,uchar *param_7,astruct_179 *param_8,
                param_9: u16,param_10: u16,param_11: u16)

{
  astruct_179 *paVar1;
  let uVar2: u16;
  let puVar3: *mut u8
  let uVar4: u16;
  let puVar5: *mut u16;
  
  puVar5 = CONCAT22(param_7,param_8);
  paVar1 = (astruct_179 *)(param_4 - 0x1);
  if (false) {
switchD_1030_0fa3_caseD_19:
    mem_op_1000_179c(0x20,param_7,0x1000);
    uVar4 = param_7 | paVar1;
    if (uVar4 == 0x0) {
      return 0x0;
    }
    pass1_1028_b39e((u16 *)CONCAT22(param_7,paVar1),param_4,param_6,uVar4);
    return CONCAT22(uVar4,paVar1);
  }
  paVar1 = param_8;
  switch((astruct_179 *)(param_4 - 0x1)) {
  case (astruct_179 *)0x0:
  case (astruct_179 *)0x1:
  case (astruct_179 *)0x2:
  case (astruct_179 *)0x3:
  case (astruct_179 *)0x4:
  case (astruct_179 *)0x5:
  case (astruct_179 *)0x6:
  case (astruct_179 *)0x7:
  case (astruct_179 *)0x8:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_48c0(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x9:
    mem_op_1000_179c(0x22,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_2bfe(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0xa:
    mem_op_1000_179c(0x26,param_7,0x1000);
    uVar2 = param_7 | param_8;
    goto joined_r0x10300adb;
  case (astruct_179 *)0xb:
    mem_op_1000_179c(0x2c,param_7,0x1000);
    if ((uchar *)(param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_3692(param_8,param_7,param_4,param_6,
                               (param_7 | param_8),param_9,param_10);
      return puVar5;
    }
    break;
  case (astruct_179 *)0xc:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_3580(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0xd:
    mem_op_1000_179c(0x26,param_7,0x1000);
    if ((uchar *)(param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_34a6(param_8,param_7,param_4,param_6,
                               (param_7 | param_8));
      return puVar5;
    }
    break;
  case (astruct_179 *)0xe:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_408e(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0xf:
  case (astruct_179 *)0x32:
  case (astruct_179 *)0x33:
  case (astruct_179 *)0x5f:
  case (astruct_179 *)0x60:
    mem_op_1000_179c(0x24,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_0c50(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x10:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_0b64(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x11:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_4376(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x12:
  case (astruct_179 *)0x13:
  case (astruct_179 *)0x14:
  case (astruct_179 *)0x61:
  case (astruct_179 *)0x62:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_4ba6(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x15:
  case (astruct_179 *)0x16:
  case (astruct_179 *)0x17:
    mem_op_1000_179c(0x24,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_1be8(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  default:
    goto switchD_1030_0fa3_caseD_19;
  case (astruct_179 *)0x1a:
  case (astruct_179 *)0x1b:
  case (astruct_179 *)0x1c:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1030_be56(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x1d:
  case (astruct_179 *)0x1e:
  case (astruct_179 *)0x1f:
    mem_op_1000_179c(0x26,param_7,0x1000);
    puVar3 = (param_7 | param_8);
    if (puVar3 != 0x0) {
      pass1_1028_00cc(param_8,param_7,param_4,param_6,puVar3);
      return CONCAT22(puVar3,param_8);
    }
    break;
  case (astruct_179 *)0x20:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_50fa(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x21:
  case (astruct_179 *)0x22:
  case (astruct_179 *)0x23:
    mem_op_1000_179c(0x24,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 =
               pass1_1028_3ec8(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x24:
  case (astruct_179 *)0x25:
  case (astruct_179 *)0x26:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1020_d08e(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x27:
  case (astruct_179 *)0x28:
  case (astruct_179 *)0x5c:
  case (astruct_179 *)0x5d:
  case (astruct_179 *)0x5e:
    mem_op_1000_179c(0x22,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1030_c71e(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x29:
  case (astruct_179 *)0x2a:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1020_cd06(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x2b:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_26d6(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x2c:
  case (astruct_179 *)0x2d:
    mem_op_1000_179c(0x2a,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 =
               pass1_1028_49de(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x2e:
  case (astruct_179 *)0x2f:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1020_e81c(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x30:
  case (astruct_179 *)0x31:
  case (astruct_179 *)0x6b:
  case (astruct_179 *)0x6c:
    mem_op_1000_179c(0x22,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1020_d3a4((u16 *)CONCAT22(param_7,param_8),param_3,param_4,param_6
                               ,param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x34:
  case (astruct_179 *)0x35:
    mem_op_1000_179c(0x2c,param_7,0x1000);
    puVar3 = (param_7 | param_8);
    if (puVar3 != 0x0) {
      pass1_1028_3816(param_8,param_7,param_4,param_6,puVar3,param_9,param_10
                     );
      return CONCAT22(puVar3,param_8);
    }
    break;
  case (astruct_179 *)0x36:
    mem_op_1000_179c(0x26,param_7,0x1000);
    uVar2 = param_7 | param_8;
joined_r0x10300adb:
    if (uVar2 != 0x0) {
      pass1_1030_c09c(param_8,param_7,param_4,param_6,uVar2);
      return CONCAT22(uVar2,param_8);
    }
    break;
  case (astruct_179 *)0x37:
  case (astruct_179 *)0x38:
    mem_op_1000_179c(0x9a,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 =
               pass1_1030_c9e4(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x39:
  case (astruct_179 *)0x3a:
    mem_op_1000_179c(0x24,param_7,0x1000);
    if ((uchar *)(param_7 | param_8) != 0x0) {
      puVar5 =
               pass1_1028_611e(param_8,param_7,param_4,param_6,param_8,
                               (param_7 | param_8));
      return puVar5;
    }
    break;
  case (astruct_179 *)0x3b:
  case (astruct_179 *)0x3c:
    mem_op_1000_179c(0x24,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_44fe(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x3d:
    mem_op_1000_179c(0x22,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1020_ce08(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x3e:
    mem_op_1000_179c(0x26,param_7,0x1000);
    puVar3 = (param_7 | param_8);
    if (puVar3 != 0x0) {
      pass1_1028_1fc8(param_8,param_7,param_4,param_6,puVar3);
      return CONCAT22(puVar3,param_8);
    }
    break;
  case (astruct_179 *)0x3f:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_25fc(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x40:
    mem_op_1000_179c(0x22,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1020_ca0c(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x46:
  case (astruct_179 *)0x69:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1020_d5c8(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x47:
  case (astruct_179 *)0x48:
  case (astruct_179 *)0x49:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1020_d888(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x4b:
  case (astruct_179 *)0x4c:
  case (astruct_179 *)0x4d:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 =
               pass1_1030_d942(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x4e:
  case (astruct_179 *)0x4f:
  case (astruct_179 *)0x50:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_5c76(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x51:
  case (astruct_179 *)0x52:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_5988(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x53:
  case (astruct_179 *)0x54:
  case (astruct_179 *)0x55:
    mem_op_1000_179c(0x22,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_5f00(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x56:
  case (astruct_179 *)0x57:
  case (astruct_179 *)0x58:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_53e8(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x59:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_58a6(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x5a:
  case (astruct_179 *)0x5b:
    mem_op_1000_179c(0x26,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_5546(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x63:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_5e18(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x64:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_5a6a(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x65:
  case (astruct_179 *)0x66:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_530a(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x67:
  case (astruct_179 *)0x68:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_57c8(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x6d:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_5652(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x6f:
  case (astruct_179 *)0x70:
  case (astruct_179 *)0x71:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) == 0x0) {
      puVar5 = 0x0;
    }
    else {
      puVar5 = pass1_1020_d888(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
    }
  case (astruct_179 *)0x72:
  case (astruct_179 *)0x76:
    mem_op_1000_179c(0x26,(puVar5 >> 0x10),0x1000);
    uVar4 = (puVar5 >> 0x10);
    if (puVar5 != 0x0) {
      puVar5 = pass1_1020_e91e(puVar5,uVar4,param_4,param_6,uVar4 | puVar5);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x73:
  case (astruct_179 *)0x77:
  case (astruct_179 *)0x78:
    mem_op_1000_179c(0x2c,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = struct_1020_d99e((u16 *)CONCAT22(param_7,param_8),param_3,param_4,
                                param_6,param_7 | param_8,param_11);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x74:
    mem_op_1000_179c(0x24,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_17ae(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x75:
    mem_op_1000_179c(0x24,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_2b1c(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
    break;
  case (astruct_179 *)0x79:
  case (astruct_179 *)0x7a:
  case (astruct_179 *)0x7b:
  case (astruct_179 *)0x7c:
  case (astruct_179 *)0x7d:
  case (astruct_179 *)0x7e:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if ((param_7 | param_8) != 0x0) {
      puVar5 = pass1_1028_2812(param_8,param_7,param_4,param_6,
                               param_7 | param_8);
      return puVar5;
    }
  }
  return 0x0;
}

