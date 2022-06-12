
u16 * switch_1030_0000(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16)

{
  let mut uVar1: u16;
  u8 *puVar2;
  let mut uVar3: u16;
  let mut paVar4: *mut Struct57;
  astruct_180 *paVar5;
  let mut puVar6: *mut u16;

  switch(param_5 + -0x1) {
  case 0x0:
  case 0x1:
  case 0x2:
  case 0x3:
  case 0x4:
  case 0x5:
  case 0x6:
  case 0x7:
  case 0x8:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_489e((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x9:
    mem_op_1000_179c(0x22,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_2bdc((u16 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0xa:
    mem_op_1000_179c(0x26,param_2);
    uVar1 = param_2;
    uVar3 = uVar1 | param_1;
    goto joined_r0x103002a1;
  case 0xb:
    mem_op_1000_179c(0x2c,param_2);
    puVar2 = (param_2 | param_1);
    if (puVar2 != NULL) {
      puVar6 = struct_1028_3670(puVar2,(u16 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0xc:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_355e((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0xd:
    mem_op_1000_179c(0x26,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_3484((u16 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0xe:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_406c((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0xf:
  case 0x32:
  case 0x33:
  case 0x5f:
  case 0x60:
    mem_op_1000_179c(0x24,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_0c24((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x10:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_0b42((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x11:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_4354((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x12:
  case 0x13:
  case 0x14:
  case 0x61:
  case 0x62:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_4b84((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x15:
  case 0x16:
  case 0x17:
    mem_op_1000_179c(0x24,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_1bbc((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  default:
    mem_op_1000_179c(0x20,param_2);
    uVar3 = param_2 | param_1;
    if (uVar3 != 0x0) {
      struct_1028_b354((astruct_180 *)CONCAT22(param_2,param_1));
      goto LAB_1030_0058;
    }
    break;
  case 0x1a:
  case 0x1b:
  case 0x1c:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      paVar5 = set_fn_ptr_1030_be34((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (paVar5 >> 0x10);
      param_1 = paVar5;
      goto LAB_1030_0058;
    }
    break;
  case 0x1d:
  case 0x1e:
  case 0x1f:
    mem_op_1000_179c(0x26,param_2);
    uVar3 = param_2 | param_1;
    paVar4 = (astruct_57 *)(param_2 & 0xffff0000 | uVar3);
    if (uVar3 != 0x0) {
      struct_1028_0068(paVar4,(astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = paVar4;
      goto LAB_1030_0058;
    }
    break;
  case 0x20:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_50d8((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x21:
  case 0x22:
  case 0x23:
    mem_op_1000_179c(0x24,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_3e94((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x24:
  case 0x25:
  case 0x26:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_d06c((u16 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x27:
  case 0x28:
  case 0x5c:
  case 0x5d:
  case 0x5e:
    mem_op_1000_179c(0x22,param_2);
    if ((param_2 | param_1) != 0x0) {
      paVar5 = struct_1030_c6f6((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (paVar5 >> 0x10);
      param_1 = paVar5;
      goto LAB_1030_0058;
    }
    break;
  case 0x29:
  case 0x2a:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_cce4((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x2b:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_26b4((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x2c:
  case 0x2d:
    mem_op_1000_179c(0x2a,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_49aa((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x2e:
  case 0x2f:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_e7fa((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x30:
  case 0x31:
  case 0x6b:
  case 0x6c:
    mem_op_1000_179c(0x22,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_d37c((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x34:
  case 0x35:
    mem_op_1000_179c(0x2c,param_2);
    uVar3 = param_2 | param_1;
    paVar4 = (astruct_57 *)(param_2 & 0xffff0000 | uVar3);
    if (uVar3 != 0x0) {
      struct_1028_37a6(paVar4,(astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = paVar4;
      goto LAB_1030_0058;
    }
    break;
  case 0x36:
    mem_op_1000_179c(0x26,param_2);
    uVar1 = param_2;
    uVar3 = uVar1 | param_1;
joined_r0x103002a1:
    if (uVar3 != 0x0) {
      struct_1030_c06e((astruct_180 *)CONCAT22(uVar1,param_1));
      goto LAB_1030_0058;
    }
    break;
  case 0x37:
  case 0x38:
    mem_op_1000_179c(0x9a,param_2);
    if ((param_2 | param_1) != 0x0) {
      paVar5 = struct_1030_c9a8((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (paVar5 >> 0x10);
      param_1 = paVar5;
      goto LAB_1030_0058;
    }
    break;
  case 0x39:
  case 0x3a:
    mem_op_1000_179c(0x24,param_2);
    uVar3 = param_2 | param_1;
    if (uVar3 != 0x0) {
      puVar6 = struct_1028_60bc((astruct_180 *)CONCAT22(param_2,param_1),
                                param_2 & 0xffff0000 | uVar3,param_1);
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x3b:
  case 0x3c:
    mem_op_1000_179c(0x24,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_44d2((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x3d:
    mem_op_1000_179c(0x22,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_cde6((u16 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x3e:
    mem_op_1000_179c(0x26,param_2);
    uVar3 = param_2 | param_1;
    paVar4 = (astruct_57 *)(param_2 & 0xffff0000 | uVar3);
    if (uVar3 != 0x0) {
      struct_1028_1f56(paVar4,(astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = paVar4;
      goto LAB_1030_0058;
    }
    break;
  case 0x3f:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_25da((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x40:
    mem_op_1000_179c(0x22,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_c9ea((u16 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x46:
  case 0x69:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_d5a6((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x47:
  case 0x48:
  case 0x49:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_d866((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x4b:
  case 0x4c:
  case 0x4d:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      paVar5 = struct_1030_d8f6((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (paVar5 >> 0x10);
      param_1 = paVar5;
      goto LAB_1030_0058;
    }
    break;
  case 0x4e:
  case 0x4f:
  case 0x50:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_5c54((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x51:
  case 0x52:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_5966((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x53:
  case 0x54:
  case 0x55:
    mem_op_1000_179c(0x22,param_2);
    if ((param_2 | param_1) != 0x0) {
      paVar5 = set_fn_ptr_1028_5ed8((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (paVar5 >> 0x10);
      param_1 = paVar5;
      goto LAB_1030_0058;
    }
    break;
  case 0x56:
  case 0x57:
  case 0x58:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_53c6((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x59:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = pass1_1028_5884((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x5a:
  case 0x5b:
    mem_op_1000_179c(0x26,param_2);
    puVar2 = (param_2 | param_1);
    if (puVar2 != NULL) {
      puVar6 = pass1_1028_5524(puVar2,(u16 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x63:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      paVar5 = set_fn_ptr_1028_5df6((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (paVar5 >> 0x10);
      param_1 = paVar5;
      goto LAB_1030_0058;
    }
    break;
  case 0x64:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_5a48((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x65:
  case 0x66:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_52e8((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x67:
  case 0x68:
    mem_op_1000_179c(0x20,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_57a6((astruct_180 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x6d:
    mem_op_1000_179c(0x20,param_2);
    uVar3 = param_2 | param_1;
    if (uVar3 != 0x0) {
      param_1 = struct_1028_5630((astruct_180 *)CONCAT22(param_2,param_1));
      goto LAB_1030_0058;
    }
    break;
  case 0x6f:
  case 0x70:
  case 0x71:
    mem_op_1000_179c(0x20,param_2);
    uVar3 = param_2;
    param_2 = (astruct_57 *)(param_2 & 0xffff0000);
    if ((uVar3 | param_1) == 0x0) {
      param_1 = 0x0;
    }
    else {
      puVar6 = struct_1020_d866((astruct_180 *)CONCAT22(uVar3,param_1));
      param_2 = (astruct_57 *)(param_2 & 0xffff0000 | puVar6 >> 0x10);
      param_1 = puVar6;
    }
  case 0x72:
  case 0x76:
    mem_op_1000_179c(0x26,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_e8f6((u16 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x73:
  case 0x77:
  case 0x78:
    mem_op_1000_179c(0x2c,param_2);
    uVar3 = param_2 | param_1;
    if (uVar3 != 0x0) {
      struct_1020_d954((astruct_180 *)CONCAT22(param_2,param_1));
      goto LAB_1030_0058;
    }
    break;
  case 0x74:
    mem_op_1000_179c(0x24,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_178c((u16 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x75:
    mem_op_1000_179c(0x24,param_2);
    if ((param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_2afa((u16 *)CONCAT22(param_2,param_1));
      uVar3 = (puVar6 >> 0x10);
      param_1 = puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x79:
  case 0x7a:
  case 0x7b:
  case 0x7c:
  case 0x7d:
  case 0x7e:
    mem_op_1000_179c(0x20,param_2);
    uVar3 = param_2 | param_1;
    if (uVar3 != 0x0) {
      param_1 = struct_1028_27f0((astruct_180 *)CONCAT22(param_2,param_1));
      goto LAB_1030_0058;
    }
  }
  param_1 = 0x0;
  uVar3 = 0x0;//
LAB_1030_0058:
  return (u16 *)CONCAT22(uVar3,param_1);
}



u16 * switch_1030_07ac(param_1: *mut astruct_12,param_2: *mut astruct_12,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,
                         mut param_6: u16 ,mut param_7: u16 ,mut param_8: u32)

{
  u8 *puVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  StructD *pSVar4;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  let mut paVar6: *mut Struct57;
  let mut uVar7: u32;
  let mut puVar8: *mut u16;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  switch(param_6 - 0x1) {
  case 0x0:
  case 0x1:
  case 0x2:
  case 0x3:
  case 0x4:
  case 0x5:
  case 0x6:
  case 0x7:
  case 0x8:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_48c0(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x9:
    mem_op_1000_179c(0x22,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_2bfe(uVar3,(astruct_179 *)param_1,paVar5,param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0xa:
    mem_op_1000_179c(0x26,paVar5);
    uVar3 = paVar5;
    pSVar4 = (StructD *)(uVar3 | param_1);
    goto joined_r0x10300adb;
  case 0xb:
    mem_op_1000_179c(0x2c,paVar5);
    puVar1 = (paVar5 | param_1);
    if (puVar1 != NULL) {
      puVar8 = pass1_1028_3692(puVar1,param_1,paVar5,param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0xc:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_3580(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0xd:
    mem_op_1000_179c(0x26,paVar5);
    puVar1 = (paVar5 | param_1);
    if (puVar1 != NULL) {
      puVar8 = pass1_1028_34a6(puVar1,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0xe:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_408e(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0xf:
  case 0x32:
  case 0x33:
  case 0x5f:
  case 0x60:
    mem_op_1000_179c(0x24,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_0c50(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x10:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_0b64(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x11:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_4376(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x12:
  case 0x13:
  case 0x14:
  case 0x61:
  case 0x62:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_4ba6(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x15:
  case 0x16:
  case 0x17:
    mem_op_1000_179c(0x24,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_1be8(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  default:
    mem_op_1000_179c(0x20,paVar5);
    pSVar4 = (StructD *)(paVar5 | param_1);
    if (pSVar4 != NULL) {
      pass1_1028_b39e(pSVar4,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      goto LAB_1030_0818;
    }
    break;
  case 0x1a:
  case 0x1b:
  case 0x1c:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1030_be56(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x1d:
  case 0x1e:
  case 0x1f:
    mem_op_1000_179c(0x26,paVar5);
    pSVar4 = (StructD *)(paVar5 | param_1);
    if (pSVar4 != NULL) {
      pass1_1028_00cc(pSVar4,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      goto LAB_1030_0818;
    }
    break;
  case 0x20:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_50fa(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x21:
  case 0x22:
  case 0x23:
    mem_op_1000_179c(0x24,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      uVar7 = pass1_1028_3ec8(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(uVar7 >> 0x10);
      param_1 = (astruct_12 *)uVar7;
      goto LAB_1030_0818;
    }
    break;
  case 0x24:
  case 0x25:
  case 0x26:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_d08e(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x27:
  case 0x28:
  case 0x5c:
  case 0x5d:
  case 0x5e:
    mem_op_1000_179c(0x22,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1030_c71e(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x29:
  case 0x2a:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_cd06(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x2b:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_26d6(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x2c:
  case 0x2d:
    mem_op_1000_179c(0x2a,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      uVar7 = pass1_1028_49de(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(uVar7 >> 0x10);
      param_1 = (astruct_12 *)uVar7;
      goto LAB_1030_0818;
    }
    break;
  case 0x2e:
  case 0x2f:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_e81c(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x30:
  case 0x31:
  case 0x6b:
  case 0x6c:
    mem_op_1000_179c(0x22,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_d3a4(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_5,param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x34:
  case 0x35:
    mem_op_1000_179c(0x2c,paVar5);
    uVar3 = paVar5 | param_1;
    paVar6 = (astruct_57 *)(paVar5 & 0xffff0000 | uVar3);
    if (uVar3 != 0x0) {
      pass1_1028_3816(paVar6,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)paVar6;
      goto LAB_1030_0818;
    }
    break;
  case 0x36:
    mem_op_1000_179c(0x26,paVar5);
    uVar3 = paVar5;
    pSVar4 = (StructD *)(uVar3 | param_1);
joined_r0x10300adb:
    if (pSVar4 != NULL) {
      pass1_1030_c09c(pSVar4,(astruct_12 *)CONCAT22(uVar3,param_1),param_6,param_8);
      goto LAB_1030_0818;
    }
    break;
  case 0x37:
  case 0x38:
    mem_op_1000_179c(0x9a,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      uVar7 = pass1_1030_c9e4(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(uVar7 >> 0x10);
      param_1 = (astruct_12 *)uVar7;
      goto LAB_1030_0818;
    }
    break;
  case 0x39:
  case 0x3a:
    mem_op_1000_179c(0x24,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      uVar7 = pass1_1028_611e((StructD *)CONCAT22(param_1,uVar3),(astruct_12 *)CONCAT22(paVar5,param_1),param_6,
                              param_8);
      pSVar4 = (StructD *)(uVar7 >> 0x10);
      param_1 = (astruct_12 *)uVar7;
      goto LAB_1030_0818;
    }
    break;
  case 0x3b:
  case 0x3c:
    mem_op_1000_179c(0x24,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_44fe(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x3d:
    mem_op_1000_179c(0x22,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_ce08(uVar3,(astruct_179 *)param_1,paVar5,param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x3e:
    mem_op_1000_179c(0x26,paVar5);
    pSVar4 = (StructD *)(paVar5 | param_1);
    if (pSVar4 != NULL) {
      pass1_1028_1fc8(pSVar4,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      goto LAB_1030_0818;
    }
    break;
  case 0x3f:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_25fc(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x40:
    mem_op_1000_179c(0x22,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_ca0c(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x46:
  case 0x69:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_d5c8(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x47:
  case 0x48:
  case 0x49:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_d888(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x4b:
  case 0x4c:
  case 0x4d:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      uVar7 = pass1_1030_d942(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(uVar7 >> 0x10);
      param_1 = (astruct_12 *)uVar7;
      goto LAB_1030_0818;
    }
    break;
  case 0x4e:
  case 0x4f:
  case 0x50:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_5c76(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x51:
  case 0x52:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_5988(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x53:
  case 0x54:
  case 0x55:
    mem_op_1000_179c(0x22,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_5f00(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x56:
  case 0x57:
  case 0x58:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_53e8(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x59:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_58a6(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x5a:
  case 0x5b:
    mem_op_1000_179c(0x26,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_5546(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x63:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_5e18(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x64:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_5a6a(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x65:
  case 0x66:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_530a(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x67:
  case 0x68:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_57c8(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x6d:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_5652(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x6f:
  case 0x70:
  case 0x71:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5;
    uVar2 = uVar3 | param_1;
    paVar5 = (astruct_57 *)(paVar5 & 0xffff0000);
    if (uVar2 == 0x0) {
      param_1 = NULL;
    }
    else {
      puVar8 = pass1_1020_d888(uVar2,(astruct_12 *)CONCAT22(uVar3,param_1),param_6,param_8);
      paVar5 = (astruct_57 *)(paVar5 & 0xffff0000 | puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
    }
  case 0x72:
  case 0x76:
    mem_op_1000_179c(0x26,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_e91e(uVar3,param_1,paVar5,param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x73:
  case 0x77:
  case 0x78:
    mem_op_1000_179c(0x2c,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = struct_1020_d99e((astruct_57 *)(paVar5 & 0xffff0000 | uVar3),
                                (astruct_12 *)CONCAT22(paVar5,param_1),param_5,param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x74:
    mem_op_1000_179c(0x24,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_17ae(uVar3,param_1,paVar5,param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x75:
    mem_op_1000_179c(0x24,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_2b1c(uVar3,param_1,paVar5,param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x79:
  case 0x7a:
  case 0x7b:
  case 0x7c:
  case 0x7d:
  case 0x7e:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = paVar5 | param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_2812(uVar3,(astruct_12 *)CONCAT22(paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
  }
  param_1 = NULL;
  pSVar4 = NULL;//
LAB_1030_0818:
  return (u16 *)CONCAT22(pSVar4,param_1);
}
