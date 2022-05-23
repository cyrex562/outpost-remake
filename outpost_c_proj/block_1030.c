//
// Created by cyrex on 2022-05-22.
//

u16 * switch_1030_0000(u16 param_1,astruct_57 *param_2,u16 param_3,u16 param_4,i16 param_5)

{
  u16 uVar1;
  uchar *puVar2;
  u16 uVar3;
  astruct_57 *paVar4;
  astruct_180 *paVar5;
  u16 *puVar6;

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
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_489e((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x9:
    mem_op_1000_179c(0x22,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_2bdc((u16 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0xa:
    mem_op_1000_179c(0x26,param_2);
    uVar1 = (u16)param_2;
    uVar3 = uVar1 | param_1;
    goto joined_r0x103002a1;
  case 0xb:
    mem_op_1000_179c(0x2c,param_2);
    puVar2 = (uchar *)((u16)param_2 | param_1);
    if (puVar2 != NULL) {
      puVar6 = struct_1028_3670(puVar2,(u16 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0xc:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_355e((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0xd:
    mem_op_1000_179c(0x26,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_3484((u16 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0xe:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_406c((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0xf:
  case 0x32:
  case 0x33:
  case 0x5f:
  case 0x60:
    mem_op_1000_179c(0x24,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_0c24((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x10:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_0b42((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x11:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_4354((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x12:
  case 0x13:
  case 0x14:
  case 0x61:
  case 0x62:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_4b84((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x15:
  case 0x16:
  case 0x17:
    mem_op_1000_179c(0x24,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_1bbc((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  default:
    mem_op_1000_179c(0x20,param_2);
    uVar3 = (u16)param_2 | param_1;
    if (uVar3 != 0x0) {
      struct_1028_b354((astruct_180 *)CONCAT22((u16)param_2,param_1));
      goto LAB_1030_0058;
    }
    break;
  case 0x1a:
  case 0x1b:
  case 0x1c:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      paVar5 = set_fn_ptr_1030_be34((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)paVar5 >> 0x10);
      param_1 = (u16)paVar5;
      goto LAB_1030_0058;
    }
    break;
  case 0x1d:
  case 0x1e:
  case 0x1f:
    mem_op_1000_179c(0x26,param_2);
    uVar3 = (u16)param_2 | param_1;
    paVar4 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)uVar3);
    if (uVar3 != 0x0) {
      struct_1028_0068(paVar4,(astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)paVar4;
      goto LAB_1030_0058;
    }
    break;
  case 0x20:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_50d8((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x21:
  case 0x22:
  case 0x23:
    mem_op_1000_179c(0x24,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_3e94((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x24:
  case 0x25:
  case 0x26:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_d06c((u16 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x27:
  case 0x28:
  case 0x5c:
  case 0x5d:
  case 0x5e:
    mem_op_1000_179c(0x22,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      paVar5 = struct_1030_c6f6((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)paVar5 >> 0x10);
      param_1 = (u16)paVar5;
      goto LAB_1030_0058;
    }
    break;
  case 0x29:
  case 0x2a:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_cce4((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x2b:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_26b4((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x2c:
  case 0x2d:
    mem_op_1000_179c(0x2a,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_49aa((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x2e:
  case 0x2f:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_e7fa((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x30:
  case 0x31:
  case 0x6b:
  case 0x6c:
    mem_op_1000_179c(0x22,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_d37c((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x34:
  case 0x35:
    mem_op_1000_179c(0x2c,param_2);
    uVar3 = (u16)param_2 | param_1;
    paVar4 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)uVar3);
    if (uVar3 != 0x0) {
      struct_1028_37a6(paVar4,(astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)paVar4;
      goto LAB_1030_0058;
    }
    break;
  case 0x36:
    mem_op_1000_179c(0x26,param_2);
    uVar1 = (u16)param_2;
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
    if (((u16)param_2 | param_1) != 0x0) {
      paVar5 = struct_1030_c9a8((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)paVar5 >> 0x10);
      param_1 = (u16)paVar5;
      goto LAB_1030_0058;
    }
    break;
  case 0x39:
  case 0x3a:
    mem_op_1000_179c(0x24,param_2);
    uVar3 = (u16)param_2 | param_1;
    if (uVar3 != 0x0) {
      puVar6 = struct_1028_60bc((astruct_180 *)CONCAT22((u16)param_2,param_1),
                                (u32)param_2 & 0xffff0000 | (u32)uVar3,param_1);
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x3b:
  case 0x3c:
    mem_op_1000_179c(0x24,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_44d2((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x3d:
    mem_op_1000_179c(0x22,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_cde6((u16 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x3e:
    mem_op_1000_179c(0x26,param_2);
    uVar3 = (u16)param_2 | param_1;
    paVar4 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)uVar3);
    if (uVar3 != 0x0) {
      struct_1028_1f56(paVar4,(astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)paVar4;
      goto LAB_1030_0058;
    }
    break;
  case 0x3f:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_25da((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x40:
    mem_op_1000_179c(0x22,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_c9ea((u16 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x46:
  case 0x69:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_d5a6((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x47:
  case 0x48:
  case 0x49:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_d866((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x4b:
  case 0x4c:
  case 0x4d:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      paVar5 = struct_1030_d8f6((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)paVar5 >> 0x10);
      param_1 = (u16)paVar5;
      goto LAB_1030_0058;
    }
    break;
  case 0x4e:
  case 0x4f:
  case 0x50:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_5c54((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x51:
  case 0x52:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_5966((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x53:
  case 0x54:
  case 0x55:
    mem_op_1000_179c(0x22,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      paVar5 = set_fn_ptr_1028_5ed8((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)paVar5 >> 0x10);
      param_1 = (u16)paVar5;
      goto LAB_1030_0058;
    }
    break;
  case 0x56:
  case 0x57:
  case 0x58:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_53c6((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x59:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = pass1_1028_5884((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x5a:
  case 0x5b:
    mem_op_1000_179c(0x26,param_2);
    puVar2 = (uchar *)((u16)param_2 | param_1);
    if (puVar2 != NULL) {
      puVar6 = pass1_1028_5524(puVar2,(u16 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x63:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      paVar5 = set_fn_ptr_1028_5df6((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)paVar5 >> 0x10);
      param_1 = (u16)paVar5;
      goto LAB_1030_0058;
    }
    break;
  case 0x64:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_5a48((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x65:
  case 0x66:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_52e8((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x67:
  case 0x68:
    mem_op_1000_179c(0x20,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_57a6((astruct_180 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x6d:
    mem_op_1000_179c(0x20,param_2);
    uVar3 = (u16)param_2 | param_1;
    if (uVar3 != 0x0) {
      param_1 = struct_1028_5630((astruct_180 *)CONCAT22((u16)param_2,param_1));
      goto LAB_1030_0058;
    }
    break;
  case 0x6f:
  case 0x70:
  case 0x71:
    mem_op_1000_179c(0x20,param_2);
    uVar3 = (u16)param_2;
    param_2 = (astruct_57 *)((u32)param_2 & 0xffff0000);
    if ((uVar3 | param_1) == 0x0) {
      param_1 = 0x0;
    }
    else {
      puVar6 = struct_1020_d866((astruct_180 *)CONCAT22(uVar3,param_1));
      param_2 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
    }
  case 0x72:
  case 0x76:
    mem_op_1000_179c(0x26,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1020_e8f6((u16 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x73:
  case 0x77:
  case 0x78:
    mem_op_1000_179c(0x2c,param_2);
    uVar3 = (u16)param_2 | param_1;
    if (uVar3 != 0x0) {
      struct_1020_d954((astruct_180 *)CONCAT22((u16)param_2,param_1));
      goto LAB_1030_0058;
    }
    break;
  case 0x74:
    mem_op_1000_179c(0x24,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_178c((u16 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
      goto LAB_1030_0058;
    }
    break;
  case 0x75:
    mem_op_1000_179c(0x24,param_2);
    if (((u16)param_2 | param_1) != 0x0) {
      puVar6 = struct_1028_2afa((u16 *)CONCAT22((u16)param_2,param_1));
      uVar3 = (u16)((u32)puVar6 >> 0x10);
      param_1 = (u16)puVar6;
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
    uVar3 = (u16)param_2 | param_1;
    if (uVar3 != 0x0) {
      param_1 = struct_1028_27f0((astruct_180 *)CONCAT22((u16)param_2,param_1));
      goto LAB_1030_0058;
    }
  }
  param_1 = 0x0;
  uVar3 = 0x0;
LAB_1030_0058:
  return (u16 *)CONCAT22(uVar3,param_1);
}



u16 * switch_1030_07ac(astruct_12 *param_1,astruct_12 *param_2,u16 param_3,u16 param_4,u16 param_5,
                         u16 param_6,u16 param_7,u32 param_8)

{
  uchar *puVar1;
  u16 uVar2;
  u16 uVar3;
  StructD *pSVar4;
  u16 in_register_0000000a;
  astruct_57 *paVar5;
  astruct_57 *paVar6;
  u32 uVar7;
  u16 *puVar8;

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
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_48c0(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x9:
    mem_op_1000_179c(0x22,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_2bfe(uVar3,(astruct_179 *)param_1,(u16)paVar5,param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0xa:
    mem_op_1000_179c(0x26,paVar5);
    uVar3 = (u16)paVar5;
    pSVar4 = (StructD *)(uVar3 | (u16)param_1);
    goto joined_r0x10300adb;
  case 0xb:
    mem_op_1000_179c(0x2c,paVar5);
    puVar1 = (uchar *)((u16)paVar5 | (u16)param_1);
    if (puVar1 != NULL) {
      puVar8 = pass1_1028_3692(puVar1,(int)param_1,(u16)paVar5,param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0xc:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_3580(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0xd:
    mem_op_1000_179c(0x26,paVar5);
    puVar1 = (uchar *)((u16)paVar5 | (u16)param_1);
    if (puVar1 != NULL) {
      puVar8 = pass1_1028_34a6(puVar1,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0xe:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_408e(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
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
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_0c50(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x10:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_0b64(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x11:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_4376(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
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
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_4ba6(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x15:
  case 0x16:
  case 0x17:
    mem_op_1000_179c(0x24,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_1be8(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  default:
    mem_op_1000_179c(0x20,paVar5);
    pSVar4 = (StructD *)((u16)paVar5 | (u16)param_1);
    if (pSVar4 != NULL) {
      pass1_1028_b39e(pSVar4,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      goto LAB_1030_0818;
    }
    break;
  case 0x1a:
  case 0x1b:
  case 0x1c:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1030_be56(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x1d:
  case 0x1e:
  case 0x1f:
    mem_op_1000_179c(0x26,paVar5);
    pSVar4 = (StructD *)((u16)paVar5 | (u16)param_1);
    if (pSVar4 != NULL) {
      pass1_1028_00cc(pSVar4,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      goto LAB_1030_0818;
    }
    break;
  case 0x20:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_50fa(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x21:
  case 0x22:
  case 0x23:
    mem_op_1000_179c(0x24,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      uVar7 = pass1_1028_3ec8(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(uVar7 >> 0x10);
      param_1 = (astruct_12 *)uVar7;
      goto LAB_1030_0818;
    }
    break;
  case 0x24:
  case 0x25:
  case 0x26:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_d08e(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
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
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1030_c71e(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x29:
  case 0x2a:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_cd06(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x2b:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_26d6(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x2c:
  case 0x2d:
    mem_op_1000_179c(0x2a,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      uVar7 = pass1_1028_49de(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(uVar7 >> 0x10);
      param_1 = (astruct_12 *)uVar7;
      goto LAB_1030_0818;
    }
    break;
  case 0x2e:
  case 0x2f:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_e81c(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x30:
  case 0x31:
  case 0x6b:
  case 0x6c:
    mem_op_1000_179c(0x22,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_d3a4(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_5,param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x34:
  case 0x35:
    mem_op_1000_179c(0x2c,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    paVar6 = (astruct_57 *)((u32)paVar5 & 0xffff0000 | (u32)uVar3);
    if (uVar3 != 0x0) {
      pass1_1028_3816(paVar6,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)paVar6;
      goto LAB_1030_0818;
    }
    break;
  case 0x36:
    mem_op_1000_179c(0x26,paVar5);
    uVar3 = (u16)paVar5;
    pSVar4 = (StructD *)(uVar3 | (u16)param_1);
joined_r0x10300adb:
    if (pSVar4 != NULL) {
      pass1_1030_c09c((u16)pSVar4,(astruct_12 *)CONCAT22(uVar3,param_1),param_6,param_8);
      goto LAB_1030_0818;
    }
    break;
  case 0x37:
  case 0x38:
    mem_op_1000_179c(0x9a,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      uVar7 = pass1_1030_c9e4(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(uVar7 >> 0x10);
      param_1 = (astruct_12 *)uVar7;
      goto LAB_1030_0818;
    }
    break;
  case 0x39:
  case 0x3a:
    mem_op_1000_179c(0x24,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      uVar7 = pass1_1028_611e((StructD *)CONCAT22(param_1,uVar3),(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,
                              param_8);
      pSVar4 = (StructD *)(uVar7 >> 0x10);
      param_1 = (astruct_12 *)uVar7;
      goto LAB_1030_0818;
    }
    break;
  case 0x3b:
  case 0x3c:
    mem_op_1000_179c(0x24,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_44fe(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x3d:
    mem_op_1000_179c(0x22,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_ce08(uVar3,(astruct_179 *)param_1,(u16)paVar5,param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x3e:
    mem_op_1000_179c(0x26,paVar5);
    pSVar4 = (StructD *)((u16)paVar5 | (u16)param_1);
    if (pSVar4 != NULL) {
      pass1_1028_1fc8((uchar *)pSVar4,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      goto LAB_1030_0818;
    }
    break;
  case 0x3f:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_25fc(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x40:
    mem_op_1000_179c(0x22,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_ca0c(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x46:
  case 0x69:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_d5c8(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x47:
  case 0x48:
  case 0x49:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_d888(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x4b:
  case 0x4c:
  case 0x4d:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      uVar7 = pass1_1030_d942(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)(uVar7 >> 0x10);
      param_1 = (astruct_12 *)uVar7;
      goto LAB_1030_0818;
    }
    break;
  case 0x4e:
  case 0x4f:
  case 0x50:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_5c76(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x51:
  case 0x52:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_5988(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x53:
  case 0x54:
  case 0x55:
    mem_op_1000_179c(0x22,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_5f00(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x56:
  case 0x57:
  case 0x58:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_53e8(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x59:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_58a6(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x5a:
  case 0x5b:
    mem_op_1000_179c(0x26,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_5546(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x63:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_5e18(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x64:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_5a6a(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x65:
  case 0x66:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_530a(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x67:
  case 0x68:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_57c8(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x6d:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_5652(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x6f:
  case 0x70:
  case 0x71:
    mem_op_1000_179c(0x20,paVar5);
    uVar3 = (u16)paVar5;
    uVar2 = uVar3 | (u16)param_1;
    paVar5 = (astruct_57 *)((u32)paVar5 & 0xffff0000);
    if (uVar2 == 0x0) {
      param_1 = NULL;
    }
    else {
      puVar8 = pass1_1020_d888(uVar2,(astruct_12 *)CONCAT22(uVar3,param_1),param_6,param_8);
      paVar5 = (astruct_57 *)((u32)paVar5 & 0xffff0000 | (u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
    }
  case 0x72:
  case 0x76:
    mem_op_1000_179c(0x26,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1020_e91e(uVar3,(int)param_1,(u16)paVar5,param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x73:
  case 0x77:
  case 0x78:
    mem_op_1000_179c(0x2c,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = struct_1020_d99e((astruct_57 *)((u32)paVar5 & 0xffff0000 | (u32)uVar3),
                                (astruct_12 *)CONCAT22((u16)paVar5,param_1),param_5,param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x74:
    mem_op_1000_179c(0x24,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_17ae(uVar3,(int)param_1,(u16)paVar5,param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
    break;
  case 0x75:
    mem_op_1000_179c(0x24,paVar5);
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_2b1c(uVar3,(int)param_1,(u16)paVar5,param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
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
    uVar3 = (u16)paVar5 | (u16)param_1;
    if (uVar3 != 0x0) {
      puVar8 = pass1_1028_2812(uVar3,(astruct_12 *)CONCAT22((u16)paVar5,param_1),param_6,param_8);
      pSVar4 = (StructD *)((u32)puVar8 >> 0x10);
      param_1 = (astruct_12 *)puVar8;
      goto LAB_1030_0818;
    }
  }
  param_1 = NULL;
  pSVar4 = NULL;
LAB_1030_0818:
  return (u16 *)CONCAT22(pSVar4,param_1);
}



void pass1_1030_10b0(astruct_12 *param_1,astruct_12 *param_2,u16 param_3,u16 param_4,u16 param_5,
                    u32 param_6,u32 param_7)

{
  u32 uVar1;
  u32 uVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 *puVar6;
  u16 uStack8;

  puVar6 = switch_1030_07ac(param_1,param_2,param_3,param_4,param_5,(u16)param_6,(u16)((u32)param_6 >> 0x10),
                            param_7);
  uVar3 = (u16)((u32)puVar6 >> 0x10);
  uVar1 = *(u32 *)((u16)puVar6 + 0x4);
  uVar2 = uVar1;
  uVar4 = uVar3;
  pass1_1028_e1ec(CONCAT22(param_4,param_3),param_7);
  uVar5 = uVar4 | (u16)uVar2;
  if (uVar5 != 0x0) {
    pass1_1030_7e5a(uVar5,(astruct_358 *)(uVar2 & 0xffff | (u32)uVar4 << 0x10),uVar1);
  }
  uStack8 = (u16)(uVar1 >> 0x10);
  pass1_1030_1358(*(astruct_291 **)(param_3 + 0x26),(u16)puVar6,uVar3,
                  uVar1 & 0xffff | (u32)(uStack8 & 0xff) << 0x10);
  return;
}



void pass1_1030_1120(u16 param_1,astruct_57 *param_2,u32 param_3)

{
  uchar *puVar1;

  mem_op_1000_179c(0x3b2,param_2);
  puVar1 = (uchar *)((u16)param_2 | param_1);
  if (puVar1 == NULL) {
    param_1 = 0x0;
    puVar1 = NULL;
  }
  else {
    struct_1030_2112(param_1,puVar1,(astruct_366 *)CONCAT22((u16)param_2,param_1),0x0);
  }
  pass1_1030_1358(*(astruct_291 **)((int)param_3 + 0x2a),param_1,(u16)puVar1,
                  *(u32 *)(param_1 + 0x4) & 0xffff | (u32)((param_1 + 0x6) & 0xff) << 0x10);
  return;
}



StructD * pass1_1030_117a(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void struct_1030_11aa(astruct_156 *param_1,i32 param_2,i32 param_3)

{
  astruct_156 *iVar1;
  u16 uVar1;

  uVar1 = (u16)((u32)param_1 >> 0x10);
  iVar1 = (astruct_156 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar1->field1_0x2 = 0x1008;
  iVar1->field2_0x4 = 0x0;
  iVar1->field3_0x6 = NULL;
  iVar1->field4_0xa = 0x0;
  iVar1->field5_0xe = param_3;
  iVar1->field6_0x12 = 0x0;
  iVar1->field7_0x16 = param_2;
  iVar1->field8_0x1a = 0x1;
    // just 0x1624
  param_1->field0_0x0 = (int)s_462_bmp_1050_1620 + 0x4;
  iVar1->field1_0x2 = 0x1030;
  if (iVar1->field5_0xe == 0x0) {
    iVar1->field5_0xe = 0x5;
  }
  if (iVar1->field7_0x16 == 0x0) {
    iVar1->field7_0x16 = 0x5;
  }
  struct_1030_1550(param_1);
  *iVar1->field3_0x6 = 0x0;
  return;
}



void pass1_1030_1244(StructD *param_1)

{
  u32 *puVar1;
  u32 *puVar2;
  u16 uVar3;
  code **ppcVar4;
  u32 uVar5;
  StructD *iVar6;
  i16 iVar7;
  i16 iVar8;
  u16 uVar9;
  u16 uVar10;
  u32 uStack6;

  uVar9 = (u16)((u32)param_1 >> 0x10);
  iVar6 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = (int)s_462_bmp_1050_1620 + 0x4;
  iVar6->address_offset_field_0x2 = 0x1030;
  if (iVar6->field14_0x1a != 0x0) {
    uStack6 = 0x1;
    while( true ) {
      puVar1 = (u32 *)&iVar6->field6_0xa;
      if (*puVar1 < uStack6 || *puVar1 == uStack6) break;
      iVar8 = (int)uStack6 * 0x4;
      uVar5 = *(u32 *)&iVar6->field_0x6;
      uVar10 = (u16)((u32)uVar5 >> 0x10);
      iVar7 = (int)uVar5;
      puVar2 = (u32 *)(iVar7 + iVar8);
      uVar3 = (iVar7 + iVar8 + 0x2);
      if ((uVar3 | (u16)puVar2) != 0x0) {
        ppcVar4 = (code **)*puVar2;
        (**ppcVar4)();
      }
      uStack6 += 0x1;
    }
  }
  fn_ptr_1000_17ce(*(char **)&iVar6->field_0x6);
  param_1->address_offset_field_0x0 = 0x389a;
  iVar6->address_offset_field_0x2 = 0x1008;
  return;
}



void pass1_1030_12ca(astruct_176 *param_1)

{
  u32 *puVar1;
  u32 uVar2;
  astruct_176 *iVar3;
  u16 uVar3;
  u32 uStack6;

  uStack6 = 0x1;
  while( true ) {
    uVar3 = (u16)((u32)param_1 >> 0x10);
    iVar3 = (astruct_176 *)param_1;
    puVar1 = &iVar3->field6_0xa;
    if (*puVar1 < uStack6 || *puVar1 == uStack6) {
      iVar3->field4_0x4 = 0x0;
      return;
    }
    uVar2 = iVar3->field5_0x6;
    if (*(i32 *)((int)uVar2 + (int)uStack6 * 0x4) == 0x0) break;
    uStack6 += 0x1;
  }
  return;
}



void bad_1030_1312(void)

{
  return;
}



void pass1_1030_1358(astruct_291 *param_1,u16 param_2,u16 param_3,u32 param_4)

{
  u32 *puVar1;
  u16 *puVar2;
  i32 lVar3;
  astruct_291 *pstruct_291_2;
  i16 iVar4;
  astruct_291 *pstruct_291_1;
  u16 uVar5;
  bool bVar6;

  if (param_4 == 0x0) {
    return;
  }
  pstruct_291_1 = (astruct_291 *)((u32)param_1 >> 0x10);
  pstruct_291_2 = (astruct_291 *)param_1;
  puVar1 = (u32 *)&pstruct_291_2->field7_0xa;
  if ((*puVar1 < param_4 || *puVar1 == param_4) || (pstruct_291_2->field6_0x6 == 0x0)) {
    puVar2 = (u16 *)((int)&pstruct_291_2->field13_0x12 + 0x2);
    bVar6 = *puVar2 < param_4;
    if ((bVar6 || *puVar2 == param_4) &&
       ((bVar6 || (puVar1 = &pstruct_291_2->field13_0x12,
                  puVar1 < (u16)param_4 || puVar1 == (u16)param_4)))) {
      struct_1030_1550((astruct_156 *)((u32)param_1 & 0xffff | ZEXT24(pstruct_291_1) << 0x10));
    }
    puVar1 = &pstruct_291_2->field13_0x12;
    if (*puVar1 < param_4 || *puVar1 == param_4) {
      return;
    }
    if (pstruct_291_2->field6_0x6 == 0x0) {
      return;
    }
    puVar2 = &pstruct_291_2->field8_0xc;
    bVar6 = *puVar2 < param_4;
    if ((bVar6 || *puVar2 == param_4) &&
       ((bVar6 || (puVar2 = &pstruct_291_2->field7_0xa, *puVar2 < (u16)param_4 || *puVar2 == (u16)param_4)))) {
      pstruct_291_2->field7_0xa = (u16)(param_4 + 0x1);
      pstruct_291_2->field8_0xc = (u16)(param_4 + 0x1 >> 0x10);
    }
  }
  lVar3 = pstruct_291_2->field6_0x6;
  uVar5 = (u16)((u32)lVar3 >> 0x10);
  iVar4 = (int)lVar3;
  (iVar4 + (u16)param_4 * 0x4) = param_2;
  (iVar4 + (u16)param_4 * 0x4 + 0x2) = param_3;
  return;
}



u16 pass1_1030_13f6(u16 param_1,u16 param_2,astruct_291 *param_3,u32 param_4)

{
  code **ppcVar1;
  u16 uVar2;
  u32 *puStack8;
  u16 uStack4;

  uStack4 = 0x0;
  bad_1030_1312();
  puStack8 = (u32 *)CONCAT22(param_2,param_1);
  if ((param_2 | param_1) != 0x0) {
    uStack4 = 0x1;
    uVar2 = (u16)((u32)param_3 >> 0x10);
    if ((((int)param_3 + 0x1a) != 0x0) && ((param_2 | param_1) != 0x0)) {
      ppcVar1 = (code **)*puStack8;
      (**ppcVar1)();
    }
    pass1_1030_1358(param_3,0x0,0x0,param_4);
    ((int)param_3 + 0x4) = 0x1;
  }
  return uStack4;
}



void pass1_1030_145a(astruct_346 *param_1,i32 param_2)

{
  u32 uVar1;
  u16 uVar2;
  astruct_346 *pstruct_1;
  astruct_346 *pstruct_1_hi;

  pstruct_1_hi = (astruct_346 *)((u32)param_1 >> 0x10);
  pstruct_1 = (astruct_346 *)param_1;
  fn_ptr_1000_17ce((char *)pstruct_1->field6_0x6);
  pstruct_1->field6_0x6 = 0x0;
  pstruct_1->field7_0xa = 0x0;
  uVar1 = pstruct_1->field10_0x16 + param_2;
  uVar2 = (u16)(uVar1 >> 0x10);
  if (uVar1 < pstruct_1->field8_0xe) {
    uVar1 = (u32)&pstruct_1->field8_0xe;
    uVar2 = ((int)&pstruct_1->field8_0xe + 0x2);
  }
  &pstruct_1->field8_0xe = (int)uVar1;
  ((int)&pstruct_1->field8_0xe + 0x2) = uVar2;
  pstruct_1->field9_0x12 = 0x0;
  return;
}



void pass1_1030_14b4(astruct_156 *param_1,u16 param_2,u16 param_3,u32 param_4)

{
  u32 *puVar1;
  u16 *puVar2;
  u32 *puVar3;
  astruct_156 *pstruct_1;
  astruct_344 *iVar4;
  astruct_156 *pstruct_1_hi;
  u16 uVar4;
  bool bVar5;

  pstruct_1_hi = (astruct_156 *)((u32)param_1 >> 0x10);
  pstruct_1 = (astruct_156 *)param_1;
  puVar1 = &pstruct_1->field4_0xa;
  if ((*puVar1 < param_4 || *puVar1 == param_4) || (pstruct_1->field3_0x6 == NULL)) {
    puVar2 = (u16 *)((int)&pstruct_1->field6_0x12 + 0x2);
    bVar5 = *puVar2 < param_4;
    if ((bVar5 || *puVar2 == param_4) &&
       ((bVar5 || (puVar3 = &pstruct_1->field6_0x12, puVar3 < (u16)param_4 || puVar3 == (u16)param_4
                  )))) {
      struct_1030_1550((astruct_156 *)((u32)param_1 & 0xffff | ZEXT24(pstruct_1_hi) << 0x10));
    }
    puVar1 = &pstruct_1->field6_0x12;
    if ((*puVar1 < param_4 || *puVar1 == param_4) || (pstruct_1->field3_0x6 == NULL)) {
      return;
    }
    puVar2 = (u16 *)((int)&pstruct_1->field4_0xa + 0x2);
    bVar5 = *puVar2 < param_4;
    if ((bVar5 || *puVar2 == param_4) &&
       ((bVar5 || (puVar3 = &pstruct_1->field4_0xa, puVar3 < (u16)param_4 || puVar3 == (u16)param_4)
        ))) {
      &pstruct_1->field4_0xa = (int)(param_4 + 0x1);
      ((int)&pstruct_1->field4_0xa + 0x2) = (int)(param_4 + 0x1 >> 0x10);
    }
  }
  puVar3 = pstruct_1->field3_0x6;
  uVar4 = (u16)((u32)puVar3 >> 0x10);
  iVar4 = (astruct_344 *)puVar3;
  (iVar4 + (u16)param_4 * 0x4) = param_2;
  (iVar4 + (u16)param_4 * 0x4 + 0x2) = param_3;
  return;
}



void pass1_1030_154c(void)

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void struct_1030_1550(astruct_156 *param_1)

{
  i32 *plVar1;
  u16 uVar2;
  u32 *puVar3;
  u16 uVar4;
  u32 in_EDX;
  StructD *pSVar5;
  astruct_156 *iVar5;
  u16 uVar6;
  i32 lVar7;
  u32 *puStack10;
  u32 uStack6;

  uVar6 = (u16)((u32)param_1 >> 0x10);
  iVar5 = (astruct_156 *)param_1;
  if (iVar5->field6_0x12 == 0x0) {
    uVar4 = &iVar5->field5_0xe;
    pSVar5 = (StructD *)(in_EDX & 0xffff0000 | (u32)((int)&iVar5->field5_0xe + 0x2));
  }
  else {
    uVar2 = &iVar5->field6_0x12;
    plVar1 = &iVar5->field7_0x16;
    uVar4 = uVar2 + plVar1;
    pSVar5 = (StructD *)
             (in_EDX & 0xffff0000 |
             (u32)(((int)&iVar5->field6_0x12 + 0x2) + ((int)&iVar5->field7_0x16 + 0x2) +
                    (u16)CARRY2(uVar2,plVar1)));
  }
  uStack6 = CONCAT22((int)pSVar5,uVar4);
  if (iVar5->field3_0x6 == NULL) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar5);
      PTR_LOOP_1050_5f2e = (u8 *)pSVar5;
    }
    else {
    }
    uVar4 = fn_ptr_op_1000_1708(uVar4 << 0x2,0x0,0x1,(u16)PTR_LOOP_1050_5f2c,(u16)PTR_LOOP_1050_5f2e);
  }
  else {
    puVar3 = iVar5->field3_0x6;
    lVar7 = pass1_1000_0ed4(0x1,uVar4 * 0x4,
                            ((int)pSVar5 * 0x2 + (u16)CARRY2(uVar4,uVar4)) * 0x2 +
                            (u16)CARRY2(uVar4 * 0x2,uVar4 * 0x2),(astruct_172 *)puVar3,
                            (astruct_172 *)((u32)puVar3 >> 0x10));
    PTR_LOOP_1050_5f2e = (u8 *)((u32)lVar7 >> 0x10);
    uVar4 = (u16)lVar7;
  }
  puStack10 = (u32 *)CONCAT22(PTR_LOOP_1050_5f2e,uVar4);
  if (((u16)PTR_LOOP_1050_5f2e | uVar4) != 0x0) {
    iVar5->field6_0x12 = uStack6;
    iVar5->field3_0x6 = puStack10;
  }
  return;
}



StructD * pass1_1030_15fe(StructD *param_1,u8 param_2)

{
  pass1_1030_1244(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void struct_1030_1628(astruct_180 *param_1)

{
  astruct_180 *iVar1;
  u16 uVar1;

  uVar1 = (u16)((u32)param_1 >> 0x10);
  iVar1 = (astruct_180 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar1->field1_0x2 = 0x1008;
  *(u32 *)&iVar1->field_0x4 = 0x0;
  *(u32 *)&iVar1->field_0x8 = 0x0;
  param_1->field0_0x0 = 0x17ba;
  iVar1->field1_0x2 = 0x1030;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_165e(astruct_57 *param_1,astruct_175 *param_2,u32 param_3,u32 param_4)

{
  u16 uVar1;
  astruct_175 *iVar1;
  u16 uVar2;

  uVar1 = SUB42(param_1,0x0);
  uVar2 = (u16)((u32)param_2 >> 0x10);
  iVar1 = (astruct_175 *)param_2;
  param_2->field0_0x0 = 0x389a;
  iVar1->field1_0x2 = 0x1008;
  *(u32 *)&iVar1->field2_0x4 = 0x0;
  iVar1->field4_0x8 = param_4;
  param_2->field0_0x0 = 0x17ba;
  iVar1->field1_0x2 = 0x1030;
  pass1_1030_5c8a(_PTR_LOOP_1050_5736,param_3);
  iVar1->field2_0x4 = (int)param_4;
  iVar1->field3_0x6 = uVar1;
  return;
}



void pass1_1030_16b2(u16 *param_1)

{
  u16 uVar1;

  uVar1 = (u16)((u32)param_1 >> 0x10);
  *param_1 = 0x17ba;
  ((int)param_1 + 0x2) = 0x1030;
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  return;
}



void pass1_1030_16d6(astruct_731 *param_1,u32 param_2)

{
  BOOL16 BVar1;
  u16 uVar2;
  HFILE16 in_stack_0000ffd8;
  u32 local_10 [0x2];
  u32 local_8;

  uVar2 = (u16)((u32)param_1 >> 0x10);
  local_10[0] = *(u32 *)((int)param_1 + 0x4);
  BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_10),(char *)0x4,in_stack_0000ffd8);
  if (BVar1 != 0x0) {
    local_8 = *(u32 *)((int)param_1 + 0x8);
    BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_8),(char *)0x4,in_stack_0000ffd8);
    if (BVar1 != 0x0) {
      return;
    }
  }
  u16_1050_0310 = 0x6d0;
  return;
}



void file_1030_1730(astruct_373 *param_1,HFILE16 *param_2)

{
  BOOL16 BVar1;

  BVar1 = read_file_1008_7dee(param_2,(u8 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x4)),0x4);
  if (BVar1 != 0x0) {
    BVar1 = read_file_1008_7dee(param_2,(u8 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x8)),0x4);
    if (BVar1 != 0x0) {
      return;
    }
  }
  u16_1050_0310 = 0x6d2;
  return;
}



void pass1_1030_177a(u32 param_1,u32 param_2)

{
  *(u32 *)((int)param_1 + 0x8) = param_2;
  return;
}



void FUN_1030_178e(void)

{
  return;
}



StructD * pass1_1030_1794(StructD *param_1,u8 param_2)

{
  pass1_1030_16b2(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1030_17ce(astruct_180 *param_1,u32 param_2,u32 param_3,astruct_57 *param_4)

{
  u16 uVar1;
  u16 uVar2;
  u16 uVar3;
  astruct_180 *iVar4;
  astruct_180 *uVar4;

  uVar1 = struct_1030_1628(param_1);
  uVar4 = (astruct_180 *)((u32)param_1 >> 0x10);
  iVar4 = (astruct_180 *)param_1;
  *(u32 *)&iVar4->field10_0xc = 0x0;
  param_1->field0_0x0 = 0x1a16;
  iVar4->field1_0x2 = 0x1030;
  if ((param_3 != 0x0) || (param_2 != 0x0)) {
    mem_op_1000_179c(0x18,param_4);
    uVar3 = (u16)param_4 | uVar1;
    if (uVar3 == 0x0) {
      uVar2 = 0x0;
      uVar3 = 0x0;
    }
    else {
      uVar2 = struct_op_1030_1cd8((astruct_75 *)CONCAT22((u16)param_4,uVar1),param_2,param_3);
    }
    iVar4->field10_0xc = uVar2;
    iVar4->field11_0xe = uVar3;
  }
  return &param_1->field0_0x0;
}



u16 * pass1_1030_183c(u16 param_1,astruct_57 *param_2,u16 *param_3,u32 param_4,u32 param_5,u32 param_6,
                        u32 param_7)

{
  u16 uVar1;
  u16 uVar2;
  i16 iVar3;
  u16 uVar4;

  pass1_1030_165e(param_2,(astruct_175 *)param_3,param_6,param_7);
  uVar4 = (u16)((u32)param_3 >> 0x10);
  iVar3 = (int)param_3;
  *(u32 *)(iVar3 + 0xc) = 0x0;
  *param_3 = 0x1a16;
  (iVar3 + 0x2) = 0x1030;
  if ((param_5 != 0x0) || (param_4 != 0x0)) {
    mem_op_1000_179c(0x18,param_2);
    uVar2 = (u16)param_2 | param_1;
    if (uVar2 == 0x0) {
      uVar1 = 0x0;
      uVar2 = 0x0;
    }
    else {
      uVar1 = struct_op_1030_1cd8((astruct_75 *)CONCAT22((u16)param_2,param_1),param_4,param_5);
    }
    (iVar3 + 0xc) = uVar1;
    (iVar3 + 0xe) = uVar2;
  }
  return param_3;
}



void pass1_1030_18b2(u16 *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;

  uVar5 = (u16)((u32)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0x1a16;
  (iVar4 + 0x2) = 0x1030;
  puVar1 = (u32 *)(iVar4 + 0xc);
  uVar2 = (iVar4 + 0xe);
  if ((uVar2 | (u16)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1030_16b2(param_1);
  return;
}



void pass1_1030_18f0(u16 param_1,u32 param_2,i16 param_3,i16 param_4)

{
  code **ppcVar1;
  u32 uVar2;
  u16 extraout_DX;
  i16 extraout_DX_00;
  i16 iVar3;
  u16 uVar4;
  u32 uStack10;
  u32 uStack6;

  uVar4 = (u16)(param_2 >> 0x10);
  iVar3 = (int)param_2;
  if (*(i32 *)(iVar3 + 0xc) != 0x0) {
    ppcVar1 = (code **)((int)*(u32 *)*(u32 *)(iVar3 + 0xc) + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(extraout_DX,param_1);
    for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
      ppcVar1 = (code **)((int)*(u32 *)*(u32 *)(iVar3 + 0xc) + 0x4);
      uVar2 = uStack6;
      (**ppcVar1)();
      if (((int)uVar2 == param_3) && (extraout_DX_00 == param_4)) {
        ppcVar1 = (code **)((int)*(u32 *)*(u32 *)(iVar3 + 0xc) + 0x8);
        (**ppcVar1)();
      }
    }
  }
  return;
}



u16 pass1_1030_1972(void)

{
  return 0x1;
}



u16 pass1_1030_1978(u16_t param_1,astruct_731 *param_2,u32 param_3)

{
  pass1_1030_16d6(param_2,param_3);
  if (param_1 != 0x0) {
    write_to_file_1008_7954(param_1,param_3,(u32 *)*(u32 *)((int)param_2 + 0xc));
    if (param_1 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return param_1;
    }
    param_1 = 0x1;
  }
  return param_1;
}



void file_1030_19b4(i16 param_1,u16 param_2,astruct_373 *param_3,HFILE16 *param_4)

{
  i32 *plVar1;
  u16 in_register_0000000a;
  astruct_57 *paVar2;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  file_1030_1730(param_3,param_4);
  if (param_1 != 0x0) {
    plVar1 = (i32 *)((u32)param_3 & 0xffff0000 | (u32)((int)param_3 + 0xc));
    file_1008_76e4(paVar2,param_4,plVar1);
    if ((int)plVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
  }
  return;
}



StructD * pass1_1030_19f0(StructD *param_1,u8 param_2)

{
  pass1_1030_18b2(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1030_1a32(u16 *param_1,u16 param_2,uchar *param_3)

{
  u16 in_register_0000000a;

  pass1_1030_183c(param_2,(astruct_57 *)CONCAT22(in_register_0000000a,param_3),param_1,0x1,0x16,0xff000000,0x0);
  PTR_LOOP_1050_5168 = (u8 *)((u32)param_1 >> 0x10);
  PTR_LOOP_1050_5166 = (u8 *)param_1;
  *(u32 *)(PTR_LOOP_1050_5166 + 0x10) = 0x0;
  *param_1 = 0x1cbc;
  (PTR_LOOP_1050_5166 + 0x2) = 0x1030;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_1a74(u16 *param_1)

{
  *param_1 = 0x1cbc;
  ((int)param_1 + 0x2) = 0x1030;
  _PTR_LOOP_1050_5166 = 0x0;
  pass1_1030_18b2(param_1);
  return;
}



u16 pass1_1030_1a9c(u32 param_1,u32 param_2)

{
  i16 *piVar1;
  u16_t in_AX;
  u16 uVar2;
  BOOL16 BVar3;
  i16 iVar4;
  u16 uVar5;
  HFILE16 in_stack_0000ffde;
  u16 local_c [0x5];

  uVar2 = pass1_1030_1978(in_AX,(astruct_731 *)param_1,param_2);
  if (uVar2 != 0x0) {
    uVar5 = (u16)(param_1 >> 0x10);
    iVar4 = (int)param_1;
    local_c[0] = *(u32 *)(iVar4 + 0x10);
    BVar3 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffde);
    if (BVar3 != 0x0) {
      if (**(i16 **)(iVar4 + 0x10) == 0x0) {
        return 0x1;
      }
      piVar1 = *(i16 **)(iVar4 + 0x10);
      BVar3 = write_to_file_1008_7e1c
                        ((u8 *)param_2,*(u32 *)((int)piVar1 + 0x2),(char *)(u32)(u16)(*piVar1 * 0x2),
                         in_stack_0000ffde);
      if (BVar3 != 0x0) {
        return 0x1;
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 file_1030_1b18(i16 param_1,u8 *param_2,u32 param_3,u32 param_4)

{
  u16 uVar1;
  i16 *piVar2;
  u32 uVar3;
  u16 uVar4;
  BOOL16 BVar5;
  u16 uVar6;
  u16 in_register_0000000a;
  StructD *pSVar7;
  astruct_57 *paVar8;
  i16 iVar9;
  astruct_368 *iVar7;
  u16 uVar10;
  u16 uVar11;

  pSVar7 = (StructD *)CONCAT22(in_register_0000000a,param_2);
  file_1030_19b4(param_1,param_2,(astruct_373 *)param_3,(HFILE16 *)param_4);
  if (param_1 != 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar7);
    }
    else {
      pSVar7 = (StructD *)((u32)pSVar7 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    uVar4 = fn_ptr_op_1000_1708(0x6,0x0,0x1,(u16)PTR_LOOP_1050_5f2c,(u16)pSVar7);
    uVar10 = (u16)(param_3 >> 0x10);
    iVar9 = (int)param_3;
    (iVar9 + 0x10) = uVar4;
    (iVar9 + 0x12) = (int)pSVar7;
    uVar1 = (iVar9 + 0x12);
    paVar8 = (astruct_57 *)((u32)pSVar7 & 0xffff0000 | (u32)uVar1);
    BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22(uVar1,(iVar9 + 0x10)),0x2);
    if (BVar5 != 0x0) {
      piVar2 = *(i16 **)(iVar9 + 0x10);
      if (*piVar2 == 0x0) {
        return 0x1;
      }
      uVar1 = *piVar2 * 0x2;
      uVar6 = uVar1;
      mem_op_1000_179c(uVar1,paVar8);
      uVar3 = *(u32 *)(iVar9 + 0x10);
      uVar11 = (u16)((u32)uVar3 >> 0x10);
      iVar7 = (astruct_368 *)uVar3;
      iVar7->field2_0x2 = uVar6;
      iVar7->field3_0x4 = (uchar *)paVar8;
      uVar3 = *(u32 *)(iVar9 + 0x10);
      BVar5 = read_file_1008_7dee((HFILE16 *)param_4,*(u8 **)((int)uVar3 + 0x2),(u32)uVar1);
      if (BVar5 != 0x0) {
        return 0x1;
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_1be2(u16 param_1,astruct_57 *param_2,u32 param_3)

{
  code **ppcVar1;
  u16 *puVar2;
  astruct_57 *uVar3;
  i16 iVar3;
  u16 uVar4;
  u32 uVar5;
  u16 uStack4;

  uVar4 = (u16)(param_3 >> 0x10);
  iVar3 = (int)param_3;
  if (*(i32 *)(iVar3 + 0xc) == 0x0) {
    mem_op_1000_179c(0x18,param_2);
    uVar3 = (astruct_57 *)param_2;
    param_2 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)((u16)uVar3 | param_1));
    if (((u16)uVar3 | param_1) == 0x0) {
      *(u32 *)(iVar3 + 0xc) = 0x0;
    }
    else {
      struct_op_1030_1cd8((astruct_75 *)CONCAT22(uVar3,param_1),0x5,0x5);
      (iVar3 + 0xc) = param_1;
      (iVar3 + 0xe) = (int)param_2;
    }
  }
  for (uStack4 = 0x0; puVar2 = (u16*)(iVar3 + 0x10), uStack4 <= *puVar2 && *puVar2 != uStack4; uStack4 += 0x1) {
    uVar5 = pass1_1028_e2e0(param_2,_PTR_LOOP_1050_65e2,0x1);
    param_2 = (astruct_57 *)((u32)param_2 & 0xffff0000 | uVar5 >> 0x10);
    ppcVar1 = (code **)((int)*(u32 *)*(u32 *)(iVar3 + 0xc) + 0x8);
    (**ppcVar1)(0x1028,*(u32 *)(iVar3 + 0xc),(int)uVar5,(int)(uVar5 >> 0x10),uStack4,0x0);
  }
  return;
}



StructD * pass1_1030_1c96(StructD *param_1,u8 param_2)

{
  pass1_1030_1a74(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void struct_op_1030_1cd8(astruct_75 *param_1,u32 param_2,u32 param_3)

{
  astruct_75 *struct_var1;
  astruct_75 *struct_var2;

  struct_var2 = (astruct_75 *)((u32)param_1 >> 0x10);
  struct_var1 = (astruct_75 *)param_1;
  param_1->field0_0x0 = 0x389a;
  struct_var1->field1_0x2 = 0x1008;
  struct_var1->field2_0x4 = 0x0;
  struct_var1->field3_0x8 = 0x0;
  struct_var1->field4_0xc = param_3;
  struct_var1->field5_0x10 = 0x0;
  struct_var1->field6_0x14 = param_2;
  param_1->field0_0x0 = 0x2044;
  struct_var1->field1_0x2 = 0x1030;
  return;
}



void pass1_1030_1d28(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = (u16)((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x2044;
  iVar1->address_offset_field_0x2 = 0x1030;
  fn_ptr_1000_17ce(*(char **)&iVar1->hfile_0x4);
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_1d58(u32 param_1)

{
  code **ppcVar1;
  u32 uVar2;

  ppcVar1 = (code **)((int)*(u32 *)param_1 + 0x4);
  uVar2 = (**ppcVar1)();
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2);
  return;
}



u32 pass1_1030_1d7c(u16 param_1,u16 param_2,u32 param_3)

{
  u32 uVar1;

  pass1_1030_1d58(param_3);
  if ((param_2 | param_1) != 0x0) {
    uVar1 = struct_op_1030_73a8((astruct_419 *)CONCAT22(param_2,param_1),param_1,param_2 | param_1);
    return uVar1;
  }
  return 0x0;
}



u32 pass1_1030_1daa(u32 param_1)

{
  u16 uVar1;

  uVar1 = (u16)(param_1 >> 0x10);
  return CONCAT22(((int)param_1 + 0xa),((int)param_1 + 0x8));
}



void pass1_1030_1dbc(void)

{
  return;
}



void pass1_1030_1dfc(u32 *param_1,u16 param_2,u16 param_3,u32 param_4)

{
  u32 *puVar1;
  u16 *puVar2;
  code **ppcVar3;
  u32 uVar4;
  i16 iVar5;
  u16 uVar6;
  bool bVar7;

  uVar6 = (u16)((u32)param_1 >> 0x10);
  iVar5 = (int)param_1;
  puVar1 = (u32 *)(iVar5 + 0x8);
  if ((*puVar1 < param_4 || *puVar1 == param_4) || (*(i32 *)(iVar5 + 0x4) == 0x0)) {
    puVar2 = (u16 *)(iVar5 + 0x12);
    bVar7 = *puVar2 < param_4;
    if ((bVar7 || *puVar2 == param_4) &&
       ((bVar7 || (puVar2 = (u16 *)(iVar5 + 0x10), *puVar2 < (u16)param_4 || *puVar2 == (u16)param_4)))) {
      ppcVar3 = (code **)((int)*param_1 + 0x20);
      (**ppcVar3)();
    }
    puVar1 = (u32 *)(iVar5 + 0x10);
    if ((*puVar1 < param_4 || *puVar1 == param_4) || (*(i32 *)(iVar5 + 0x4) == 0x0)) {
      return;
    }
    puVar2 = (u16 *)(iVar5 + 0xa);
    bVar7 = *puVar2 < param_4;
    if ((bVar7 || *puVar2 == param_4) &&
       ((bVar7 || (puVar2 = (u16 *)(iVar5 + 0x8), *puVar2 < (u16)param_4 || *puVar2 == (u16)param_4)))) {
      (iVar5 + 0x8) = (int)(param_4 + 0x1);
      (iVar5 + 0xa) = (int)(param_4 + 0x1 >> 0x10);
    }
  }
  uVar4 = *(u32 *)(iVar5 + 0x4);
  uVar6 = (u16)((u32)uVar4 >> 0x10);
  iVar5 = (int)uVar4;
  (iVar5 + (u16)param_4 * 0x4) = param_2;
  (iVar5 + (u16)param_4 * 0x4 + 0x2) = param_3;
  return;
}



void pass1_1030_1e96(u32 *param_1)

{
  u32 *puVar1;
  code **ppcVar2;
  u32 uVar3;
  u16 uVar4;
  u32 uStack6;

  uStack6 = 0x0;
  while( true ) {
    uVar4 = (u16)((u32)param_1 >> 0x10);
    puVar1 = (u32 *)((int)param_1 + 0x8);
    if ((*puVar1 < uStack6 || *puVar1 == uStack6) ||
       (uVar3 = *(u32 *)((int)param_1 + 0x4), *(i32 *)((int)uVar3 + (int)uStack6 * 0x4) == 0x0)) break;
    uStack6 += 0x1;
  }
  ppcVar2 = (code **)((int)*param_1 + 0x8);
  (**ppcVar2)();
  return;
}



void pass1_1030_1eee(u32 param_1,u32 param_2)

{
  u32 uVar1;
  i16 iVar2;
  u16 uVar3;

  uVar3 = (u16)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  uVar1 = *(u32 *)(iVar2 + 0xc);
  param_2 = (iVar2 + 0xe);
  if (uVar1 < param_2) {
    uVar1 = param_2 & 0xffff;
  }
  (iVar2 + 0xc) = (int)uVar1;
  (iVar2 + 0xe) = param_2;
  return;
}



void pass1_1030_1f16(u32 *param_1,u32 param_2)

{
  i32 *plVar1;
  code **ppcVar2;
  u32 uVar3;
  i16 iVar4;
  u16 uVar5;

  uVar5 = (u16)((u32)param_1 >> 0x10);
  iVar4 = (int)param_1;
  if ((*(i32 *)(iVar4 + 0x4) == 0x0) || (*(u32 *)(iVar4 + 0x10) <= *(u32 *)(iVar4 + 0x8))) {
    ppcVar2 = (code **)((int)*param_1 + 0x20);
    (**ppcVar2)();
  }
  uVar3 = *(u32 *)(iVar4 + 0x4);
  *(u32 *)((iVar4 + 0x8) * 0x4 + (int)uVar3) = param_2;
  plVar1 = (i32 *)(iVar4 + 0x8);
  *plVar1 = *plVar1 + 0x1;
  return;
}



void FUN_1030_1f6c(void)

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void FUN_1030_1f70(u16 param_1,u32 param_2)

{
  u16 *puVar1;
  u16 uVar2;
  u32 uVar3;
  u16 uVar4;
  u32 in_EDX;
  StructD *pSVar5;
  i16 iVar6;
  u16 uVar7;
  i32 lVar8;
  u32 uStack10;
  u32 uStack6;

  uVar7 = (u16)((u32)param_2 >> 0x10);
  iVar6 = (int)param_2;
  if (*(i32 *)(iVar6 + 0x10) == 0x0) {
    uVar4 = (iVar6 + 0xc);
    pSVar5 = (StructD *)(in_EDX & 0xffff0000 | (u32)(iVar6 + 0xe));
  }
  else {
    uVar2 = (iVar6 + 0x10);
    puVar1 = (u16 *)(iVar6 + 0x14);
    uVar4 = uVar2 + *puVar1;
    pSVar5 = (StructD *)
             (in_EDX & 0xffff0000 |
             (u32)((iVar6 + 0x12) + (iVar6 + 0x16) + (u16)CARRY2(uVar2,*puVar1)));
  }
  uStack6 = CONCAT22((int)pSVar5,uVar4);
  if (*(i32 *)(iVar6 + 0x4) == 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar5);
      PTR_LOOP_1050_5f2e = (u8 *)pSVar5;
    }
    else {
    }
    uVar4 = fn_ptr_op_1000_1708(uVar4 << 0x2,0x0,0x1,(u16)PTR_LOOP_1050_5f2c,(u16)PTR_LOOP_1050_5f2e);
  }
  else {
    uVar3 = *(u32 *)(iVar6 + 0x4);
    lVar8 = pass1_1000_0ed4(0x1,uVar4 * 0x4,
                            ((int)pSVar5 * 0x2 + (u16)CARRY2(uVar4,uVar4)) * 0x2 +
                            (u16)CARRY2(uVar4 * 0x2,uVar4 * 0x2),(astruct_172 *)uVar3,
                            (astruct_172 *)((u32)uVar3 >> 0x10));
    PTR_LOOP_1050_5f2e = (u8 *)((u32)lVar8 >> 0x10);
    uVar4 = (u16)lVar8;
  }
  uStack10 = CONCAT22(PTR_LOOP_1050_5f2e,uVar4);
  if (((u16)PTR_LOOP_1050_5f2e | uVar4) != 0x0) {
    *(u32 *)(iVar6 + 0x10) = uStack6;
    *(u32 *)(iVar6 + 0x4) = uStack10;
  }
  return;
}



StructD * pass1_1030_201e(StructD *param_1,u8 param_2)

{
  pass1_1030_1d28(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void pass1_1030_2068(astruct_180 *param_1)

{
  u32 in_EDX;
  astruct_180 *iVar1;
  astruct_180 *uVar3;
  i16 iStack4;

  struct_1030_17ce(param_1,0x0,0x0,in_EDX);
  uVar3 = (astruct_180 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_180 *)param_1;
  param_1->field0_0x0 = 0x293c;
  iVar1->field1_0x2 = 0x1030;
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1->field12_0x10)),NULL,0x106);
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1[0x8].field15_0x16)),NULL,0x86);
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1[0xc].field18_0x1c)),NULL,0xa);
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1[0x15].field10_0xc)),NULL,0x106);
  iStack4 = 0x0;
  do {
    (&iVar1->field12_0x10)[iStack4] = 0xffff;
    (&iVar1[0xd].field_0x6 + iStack4 * 0x2) = 0x19;
    iStack4 += 0x1;
  } while (iStack4 < 0x83);
  return;
}



void struct_1030_2112(u16 param_1,uchar *param_2,astruct_366 *param_3,u32 param_4)

{
  u16 in_register_0000000a;
  astruct_366 *pstruct_1;
  u16 pstruct_1_hi;
  i16 iStack4;

  pass1_1030_183c(param_1,(astruct_57 *)CONCAT22(in_register_0000000a,param_2),&param_3->field0_0x0,0x1,0x1,0x8000000,
                  param_4);
  pstruct_1_hi = (u16)((u32)param_3 >> 0x10);
  pstruct_1 = (astruct_366 *)param_3;
  param_3->field0_0x0 = 0x293c;
  pstruct_1->field1_0x2 = 0x1030;
  iStack4 = 0x0;
  do {
    (&pstruct_1->field14_0x10)[iStack4] = 0xffff;
    (&pstruct_1->field405_0x1a6)[iStack4] = 0x19;
    iStack4 += 0x1;
  } while (iStack4 < 0x83);
  pass1_1000_4906((StructD *)((u32)param_3 & 0xffff0000 | ZEXT24(&pstruct_1->field_0x116)),NULL,0x86);
  pass1_1000_4906((StructD *)((u32)param_3 & 0xffff0000 | ZEXT24(&pstruct_1->field_0x19c)),NULL,0xa);
  pass1_1000_4906((StructD *)((u32)param_3 & 0xffff0000 | ZEXT24(pstruct_1 + 0x1)),NULL,0x106);
  pstruct_1->field14_0x10 = 0x0;
  pstruct_1->field17_0x14 = 0x0;
  pstruct_1->field18_0x16 = 0x0;
  pstruct_1->field27_0x20 = 0x0;
  pstruct_1->field62_0x44 = 0x0;
  pstruct_1->field73_0x50 = 0x0;
  pstruct_1->field98_0x6a = 0x0;
  pstruct_1->field123_0x84 = 0x0;
  pstruct_1->field190_0xc8 = 0x0;
  pstruct_1->field217_0xe4 = 0x0;
  pstruct_1->field228_0xf0 = 0x0;
  pstruct_1->field231_0xf4 = 0x0;
  pstruct_1->field232_0xf6 = 0x0;
  pstruct_1->field242_0x102 = 0x0;
  pstruct_1->field239_0xfe = 0x0;
  pstruct_1->field405_0x1a6 = 0x0;
  pstruct_1->field408_0x1aa = 0x0;
  pstruct_1->field409_0x1ac = 0x0;
  pstruct_1->field418_0x1b6 = 0x0;
  pstruct_1->field453_0x1da = 0x0;
  pstruct_1->field464_0x1e6 = 0x0;
  pstruct_1->field489_0x200 = 0x0;
  pstruct_1->field514_0x21a = 0x0;
  pstruct_1->field581_0x25e = 0x0;
  pstruct_1->field608_0x27a = 0x0;
  pstruct_1->field619_0x286 = 0x0;
  pstruct_1->field622_0x28a = 0x0;
  pstruct_1->field623_0x28c = 0x0;
  pstruct_1->field633_0x298 = 0x0;
  pstruct_1->field630_0x294 = 0x0;
  return;
}



i16 pass1_1030_2242(astruct_168 *param_1,i16 param_2)

{
  i16 iVar1;
  astruct_168 *iVar2;
  astruct_168 *paVar2;
  u16 uVar3;

  uVar3 = (u16)((u32)param_1 >> 0x10);
  iVar2 = (astruct_168 *)param_1;
  paVar2 = (astruct_168 *)&iVar2->field_0x10;
  if (-0x1 < ((int)paVar2 + param_2 * 0x2)) {
    iVar1 = (&iVar2->field_0x10 + param_2 * 0x2);
    paVar2 = iVar2 + 0x1;
    if ((&paVar2->field_0x0 + param_2 * 0x2) <= iVar1) {
      return iVar1;
    }
  }
  return (&paVar2->field_0x0 + param_2 * 0x2);
}



void pass1_1030_227a(u16_t param_1,u32 param_2,u32 param_3)

{
  u16 uVar1;
  i16 iVar2;
  BOOL16 BVar3;
  HFILE16 in_stack_0000ffe8;

  uVar1 = pass1_1030_1978(param_1,(astruct_731 *)param_2,param_3);
  if (uVar1 != 0x0) {
    iVar2 = (int)param_2;
    BVar3 = write_to_file_1008_7e1c
                      ((u8 *)param_3,param_2 & 0xffff0000 | (u32)(iVar2 + 0x10),(char *)0x106,in_stack_0000ffe8);
    if (BVar3 != 0x0) {
      BVar3 = write_to_file_1008_7e1c
                        ((u8 *)param_3,param_2 & 0xffff0000 | (u32)(iVar2 + 0x116),(char *)0x86,in_stack_0000ffe8);
      if (BVar3 != 0x0) {
        BVar3 = write_to_file_1008_7e1c
                          ((u8 *)param_3,param_2 & 0xffff0000 | (u32)(iVar2 + 0x19c),(char *)0xa,in_stack_0000ffe8);
        if (BVar3 != 0x0) {
          BVar3 = write_to_file_1008_7e1c
                            ((u8 *)param_3,param_2 & 0xffff0000 | (u32)(iVar2 + 0x1a6),(char *)0x106,in_stack_0000ffe8
                            );
          if (BVar3 != 0x0) {
            BVar3 = write_to_file_1008_7e1c
                              ((u8 *)param_3,param_2 & 0xffff0000 | (u32)(iVar2 + 0x2ac),(char *)0x106,
                               in_stack_0000ffe8);
            if (BVar3 != 0x0) {
              return;
            }
          }
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}



void pass1_1030_232e(i16 param_1,u16 param_2,u32 param_3,u32 param_4)

{
  i16 iVar1;
  BOOL16 BVar2;

  file_1030_19b4(param_1,param_2,(astruct_373 *)param_3,(HFILE16 *)param_4);
  if (param_1 != 0x0) {
    iVar1 = (int)param_3;
    BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(param_3 & 0xffff0000 | (u32)(iVar1 + 0x10)),0x106);
    if (BVar2 != 0x0) {
      BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(param_3 & 0xffff0000 | (u32)(iVar1 + 0x116)),0x86);
      if (BVar2 != 0x0) {
        BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(param_3 & 0xffff0000 | (u32)(iVar1 + 0x19c)),0xa);
        if (BVar2 != 0x0) {
          BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(param_3 & 0xffff0000 | (u32)(iVar1 + 0x1a6)),0x106);
          if (BVar2 != 0x0) {
            BVar2 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(param_3 & 0xffff0000 | (u32)(iVar1 + 0x2ac)),0x106);
            if (BVar2 != 0x0) {
              return;
            }
          }
        }
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_23e2(i16 param_1,uchar *param_2,u32 param_3,i16 param_4,u16 param_5)

{
  code **ppcVar1;
  u32 uVar2;
  bool bVar3;
  bool bVar4;
  i16 iVar5;
  undefined3 extraout_var;
  u16 uVar6;
  u16 uVar7;
  u16 uVar8;
  u16 in_register_0000000a;
  astruct_57 *paVar9;
  u16 uVar10;
  u16 uVar11;
  u16 unaff_SI;
  i16 iVar12;
  u16 uVar13;
  u32 *puVar14;
  u16 in_stack_0000fe90;
  u16 in_stack_0000fe92;
  u16 in_stack_0000ffb4;
  u16 in_stack_0000ffb6;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc0;
  i16 iStack8;

  paVar9 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar13 = (u16)(param_3 >> 0x10);
  uVar10 = (u16)param_3;
  if ((uVar10 + 0x10 + param_5 * 0x2) < 0x0) {
    uVar7 = param_5;
    if (param_4 == 0x0) {
      puVar14 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x31),in_stack_0000fe92,
                                in_stack_0000ffb6,in_stack_0000ffbc,in_stack_0000ffc0);
      paVar9 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | (u32)puVar14 >> 0x10);
      ppcVar1 = (code **)((int)*puVar14 + 0x14);
      (**ppcVar1)(0x1010,(int)puVar14,(int)((u32)puVar14 >> 0x10),param_5,(int)param_5 >> 0xf);
      uVar6 = SUB42(paVar9,0x0);
    }
    else {
      puVar14 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x41),in_stack_0000fe92,
                                in_stack_0000ffb6,in_stack_0000ffbc,in_stack_0000ffc0);
      paVar9 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | (u32)puVar14 >> 0x10);
      ppcVar1 = (code **)((int)*puVar14 + 0x14);
      (**ppcVar1)(0x1010,(int)puVar14,(int)((u32)puVar14 >> 0x10),param_5,(int)param_5 >> 0xf);
      uVar6 = SUB42(paVar9,0x0);
    }
    uVar2 = *(u32 *)(uVar7 + 0x16);
    param_1 = ((int)uVar2 + 0x4);
    (uVar10 + param_5 * 0x2 + 0x10) = param_1;
  }
  if ((uVar10 + 0x10 + param_5 * 0x2) != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
    pass1_1030_2852();
    bVar3 = false;
    iStack8 = param_1;
    if ((uVar10 + 0x152) != 0x0) {
      bVar4 = pass1_1030_266c(uVar10,CONCAT22(param_5,uVar13));
      if ((int)CONCAT31(extraout_var,bVar4) != 0x0) {
        iStack8 = param_1 + 0x1;
        bVar3 = true;
      }
    }
    iVar5 = param_5 * 0x2;
    iStack8 = (uVar10 + iVar5 + 0x10) - iStack8;
    (uVar10 + iVar5 + 0x10) = iStack8;
    if (iStack8 < 0x0) {
      (uVar10 + iVar5 + 0x10) = 0x0;
    }
    iVar5 = param_5 * 0x2;
    if ((uVar10 + 0x2ac + iVar5) == 0x0) {
      iVar12 = iVar5 + uVar10;
      (iVar12 + 0x2ac) = 0x1;
      uVar7 = (uVar10 + iVar5 + 0x1a6) - 0x1;
      paVar9 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | (u32)uVar7);
      (iVar12 + 0x1a6) = uVar7;
      if ((uVar10 + iVar5 + 0x1a6) < 0x0) {
        (iVar12 + 0x1a6) = 0x0;
      }
    }
    if (((uVar10 + 0x10 + param_5 * 0x2) != 0x0) ||
       (uVar11 = uVar10 + 0x1a6, (uVar11 + param_5 * 0x2) != 0x0)) {
      if ((uVar10 + 0x10 + param_5 * 0x2) == 0x0) {
        (uVar10 + param_5 * 0x2 + 0x10) = 0x1;
      }
      return;
    }
    uVar7 = param_5;
    puVar14 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(param_5,0x32),in_stack_0000fe90,
                              in_stack_0000ffb4,in_stack_0000ffba,in_stack_0000ffbe);
    uVar8 = (u16)((u32)puVar14 >> 0x10);
    pass1_1010_6cf8(uVar11,uVar8,0x1010,(u32)puVar14,uVar7);
    pass1_1030_26ac(uVar8,param_3,param_5);
    if (bVar3) {
      iVar5 = pass1_1030_28dc(param_3,param_5);
      (uVar10 + iVar5 * 0x2 + 0x19c) = 0x0;
    }
  }
  return;
}



BOOL16 pass1_1030_25b2(u32 param_1,i16 param_2)

{
  if (((int)param_1 + 0x10 + param_2 * 0x2) == 0x0) {
    return 0x1;
  }
  return 0x0;
}



void pass1_1030_25d8(u32 param_1,u16 param_2,i16 param_3)

{
  ((int)param_1 + param_3 * 0x2 + 0x10) = param_2;
  return;
}



void pass1_1030_25f0(u32 param_1,i16 param_2,i16 param_3)

{
  u16 uVar1;

  uVar1 = (u16)(param_1 >> 0x10);
  if (param_2 == 0x0) {
    param_2 = ((int)param_1 + 0x116 + param_3 * 0x2) + 0x1;
  }
  ((int)param_1 + param_3 * 0x2 + 0x116) = param_2;
  return;
}



bool pass1_1030_2622(u32 param_1,i16 param_2)

{
  i16 iVar1;

  if ((param_2 != 0x70) && (param_2 != 0x1)) {
    iVar1 = pass1_1030_28dc(param_1,0x0);
    if (-0x1 < iVar1) {
      ((int)param_1 + iVar1 * 0x2 + 0x19c) = param_2;
    }
    return -0x1 < iVar1;
  }
  return false;
}



bool pass1_1030_266c(u16 param_1,u32 param_2)

{
  i16 iVar1;

  iVar1 = pass1_1030_28dc(CONCAT22((int)param_2,param_1),(int)(param_2 >> 0x10));
  return iVar1 != -0x1;
}



void pass1_1030_2690(u32 param_1)

{
  pass1_1000_4906((StructD *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0x2ac)),NULL,0x106);
  return;
}



void pass1_1030_26ac(u16 param_1,u32 param_2,u16 param_3)

{
  i16 *piVar1;
  u32 uVar2;
  i16 iVar3;
  u16 uVar4;
  char cVar5;
  astruct_92 *paVar6;
  u16 uVar7;
  i16 iVar8;
  i16 iVar9;
  u16 uVar10;
  i16 iVar11;
  u16 uVar12;
  i16 iStack38;
  astruct_92 local_14;

  iVar11 = (int)param_2;
  uVar12 = (u16)(param_2 >> 0x10);
  if (param_3 != 0x13) {
    if (0x13 < (int)param_3) {
      if (param_3 != 0x5f) {
        if ((int)(param_3 - 0x5f) < 0x6) {
          return;
        }
        if (param_3 != 0x66 && 0x0 < (int)(param_3 - 0x65)) {
          if ((int)(param_3 - 0x66) < 0x5) {
            return;
          }
          if (0x4 < (int)(param_3 - 0x6b)) {
            return;
          }
        }
      }
      pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x400);
      while( true ) {
        uVar10 = param_1;
        paVar6 = &local_14;
        pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar6));
        param_1 = uVar10 | (u16)paVar6;
        if (param_1 == 0x0) break;
        if (*(i32 *)(iVar11 + 0x4) == paVar6[0x1c].field4_0x8) {
          uVar7 = ((int)&paVar6[0x1].field3_0x4 + 0x2) + 0x19;
          if (0x3e8 < (int)uVar7) {
            uVar7 = 0x3e8;
          }
          pass1_1038_4d0e((astruct_685 *)CONCAT22(uVar10,paVar6),uVar7);
        }
      }
      return;
    }
    if (param_3 == 0x12) {
      pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x400);
      while( true ) {
        uVar10 = param_1;
        paVar6 = &local_14;
        pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar6));
        param_1 = uVar10 | (u16)paVar6;
        if (param_1 == 0x0) break;
        if (*(i32 *)(iVar11 + 0x4) == paVar6[0x1c].field4_0x8) {
          uVar2 = *(u32 *)&paVar6[0x1b].field6_0x10;
          iVar9 = (int)uVar2;
          uVar4 = (u16)((u32)uVar2 >> 0x10);
          piVar1 = (i16 *)(iVar9 + 0x182);
          *piVar1 = *piVar1 + -0x19;
          iVar8 = (iVar9 + 0x182);
          if (iVar8 < 0x1) {
            iVar8 = 0x1;
          }
          (iVar9 + 0x182) = iVar8;
        }
      }
      return;
    }
    if (0x12 < param_3) {
      return;
    }
    cVar5 = (char)param_3;
    if (cVar5 != '\n') {
      if ((char)(cVar5 + -0xa) < '\x06') {
        return;
      }
      if ('\x01' < (char)(cVar5 + -0x10)) {
        return;
      }
    }
  }
  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x400);
  while( true ) {
    uVar10 = param_1;
    paVar6 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar6));
    param_1 = uVar10 | (u16)paVar6;
    if (param_1 == 0x0) break;
    if (*(i32 *)(iVar11 + 0x4) == paVar6[0x1c].field4_0x8) {
      uVar2 = *(u32 *)&paVar6[0x1b].field6_0x10;
      iVar8 = (int)uVar2 + 0x180;
      uVar4 = (u16)((u32)uVar2 >> 0x10);
      iStack38 = 0x1;
      do {
        iVar3 = iStack38 * 0x2;
        piVar1 = (i16 *)(iVar3 + iVar8);
        *piVar1 = *piVar1 + -0x1;
        iVar9 = (iVar3 + iVar8);
        if (iVar9 < 0x1) {
          iVar9 = 0x1;
        }
        (iVar3 + iVar8) = iVar9;
        iStack38 += 0x1;
      } while (iStack38 < 0x6);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_2852(void)

{
  return;
}



i16 pass1_1030_28dc(u32 param_1,i16 param_2)

{
  i16 iStack4;

  iStack4 = 0x0;
  while( true ) {
    if (0x4 < iStack4) {
      return -0x1;
    }
    if (((int)param_1 + 0x19c + iStack4 * 0x2) == param_2) break;
    iStack4 += 0x1;
  }
  return iStack4;
}



StructD * pass1_1030_2916(StructD *param_1,u8 param_2)

{
  pass1_1030_18b2(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void pass1_1030_2958(astruct_180 *param_1,u32 param_2)

{
  astruct_180 *pstruct_1;
  astruct_180 *pstruct_1_hi;

  struct_1030_17ce(param_1,0x5,0xf,param_2);
  pstruct_1_hi = (astruct_180 *)((u32)param_1 >> 0x10);
  pstruct_1 = (astruct_180 *)param_1;
  *(u32 *)&pstruct_1->field12_0x10 = 0x0;
  pstruct_1->field14_0x14 = 0x0;
  pstruct_1->field15_0x16 = 0x0;
  pstruct_1->field16_0x18 = 0x2710;
  pstruct_1->field17_0x1a = 0x0;
  param_1->field0_0x0 = 0x3130;
  pstruct_1->field1_0x2 = 0x1030;
  return;
}



void struct_1030_299a(u16 param_1,astruct_57 *param_2,astruct_352 *param_3,u32 param_4)

{
  astruct_352 *pstruct_1;
  astruct_352 *pstruct_1_hi;

  pass1_1030_183c(param_1,param_2,&param_3->u16_field_0x0,0x5,0xf,0x2000000,param_4);
  pstruct_1_hi = (astruct_352 *)((u32)param_3 >> 0x10);
  pstruct_1 = (astruct_352 *)param_3;
  pstruct_1->field14_0x10 = 0x0;
  pstruct_1->field15_0x14 = 0x0;
  pstruct_1->field16_0x16 = 0x0;
  pstruct_1->field17_0x18 = 0x2710;
  pstruct_1->field18_0x1a = 0x0;
  param_3->u16_field_0x0 = 0x3130;
  pstruct_1->field1_0x2 = 0x1030;
  return;
}



void pass1_1030_29e6(StructD *param_1)

{
  u16 uVar1;
  char *pcVar2;
  StructD *iVar4;
  u16 uVar3;

  uVar3 = (u16)((u32)param_1 >> 0x10);
  iVar4 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x3130;
  iVar4->address_offset_field_0x2 = 0x1030;
  pcVar2 = *(char **)&iVar4->field_0x10;
  uVar1 = iVar4->field11_0x12;
  if ((uVar1 | (u16)pcVar2) != 0x0) {
    pass1_1030_8496((u32)pcVar2 & 0xffff | (u32)uVar1 << 0x10);
    fn_ptr_1000_17ce(pcVar2);
  }
  pass1_1030_18b2(&param_1->address_offset_field_0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_2a2c(StructD *param_1,astruct_678 *param_2)

{
  i16 *piVar1;
  astruct_678 *iVar2;
  u16 uVar2;
  astruct_67 *paVar3;
  u16 in_stack_0000fe94;
  u16 in_stack_0000ffb8;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc2;
  u16 uVar4;
  i16 iVar5;
  u16 uVar6;
  u16 uVar7;
  u16 uVar8;
  i16 iVar9;

  uVar2 = (u16)((u32)param_2 >> 0x10);
  iVar2 = (astruct_678 *)param_2;
  if (0x0 < iVar2->field23_0x18) {
    piVar1 = &iVar2->field23_0x18;
    *piVar1 = *piVar1 + -0x1;
  }
  if (iVar2->field22_0x16 == 0x0) {
    iVar2->field22_0x16 = 0x1;
  }
  if (iVar2->field24_0x1a == 0x0) {
    iVar2->field24_0x1a = 0x2;
  }
  if (iVar2->field23_0x18 < 0x1) {
    iVar2->field22_0x16 = 0x2;
    iVar2->field24_0x1a = 0x1;
    uVar8 = 0x0;
    iVar9 = 0x21;
    uVar6 = 0x1;
    uVar7 = 0x0;
    uVar4 = 0x0;
    iVar5 = 0x0;
    uVar2 = 0x0;
    paVar3 = (astruct_67 *)
             mixed_1010_20ba((astruct_57 *)param_1,_u16_1050_0ed0,(u8 **)0x37,in_stack_0000fe94,in_stack_0000ffb8
                             ,in_stack_0000ffbe,in_stack_0000ffc2);
    post_win_msg_1008_a0e4(paVar3,CONCAT22(uVar4,uVar2),iVar5,uVar6,CONCAT22(uVar8,uVar7),iVar9);
  }
  return;
}



u16 pass1_1030_2a98(u32 param_1)

{
  i16 *piVar1;
  u16 uVar2;

  uVar2 = (u16)(param_1 >> 0x10);
  piVar1 = (i16 *)((int)param_1 + 0x14);
  *piVar1 = *piVar1 + 0x1;
  return ((int)param_1 + 0x14);
}



u16 pass1_1030_2aaa(u32 param_1)

{
  u32 uVar1;
  u16 uVar2;

  uVar2 = (u16)(param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x10) == 0x0) {
    return 0x0;
  }
  uVar1 = *(u32 *)((int)param_1 + 0x10);
  return ((int)uVar1 + 0xc);
}



void pass1_1030_2aca(u16_t param_1,astruct_730 *param_2,u32 param_3)

{
  u16 *puVar1;
  u16 uVar2;
  BOOL16 BVar3;
  i16 iVar4;
  astruct_730 *iVar6;
  u16 uVar5;
  u16 uVar6;
  HFILE16 in_stack_0000ffc8;
  u32 local_18 [0x3];
  u16 local_c [0x3];
  u16 local_6 [0x2];

  uVar2 = pass1_1030_1978(param_1,(astruct_731 *)param_2,param_3);
  if (uVar2 == 0x0) {
    return;
  }
  uVar5 = (u16)((u32)param_2 >> 0x10);
  iVar6 = (astruct_730 *)param_2;
  local_c[0] = *iVar6->field16_0x10;
  BVar3 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffc8);
  if (((BVar3 != 0x0) &&
      (puVar1 = iVar6->field16_0x10, BVar3 = pass1_1008_7c2a(param_3,*(char **)((int)puVar1 + 0x2)), BVar3 != 0x0)) &&
     (puVar1 = iVar6->field16_0x10,
     iVar4 = write_to_file_1008_7b4c(param_3,(astruct_615 *)((u32)puVar1 & 0xffff0000 | (u32)((int)puVar1 + 0x6))),
     iVar4 != 0x0)) {
    puVar1 = iVar6->field16_0x10;
    local_6[0] = ((int)puVar1 + 0xc);
    BVar3 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffc8);
    if (BVar3 != 0x0) {
      puVar1 = iVar6->field16_0x10;
      local_18[0] = *(u32 *)((int)puVar1 + 0xe);
      BVar3 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_18),(char *)0x4,in_stack_0000ffc8);
      if ((BVar3 != 0x0) &&
         (puVar1 = iVar6->field16_0x10,
         BVar3 = write_to_file_1008_7e1c
                           ((u8 *)param_3,(u32)puVar1 & 0xffff0000 | (u32)((int)puVar1 + 0x12),(char *)0x10,
                            in_stack_0000ffc8), BVar3 != 0x0)) {
        puVar1 = iVar6->field16_0x10;
        local_c[0] = ((int)puVar1 + 0x22);
        BVar3 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffc8);
        if ((BVar3 != 0x0) &&
           ((puVar1 = iVar6->field16_0x10, ((int)puVar1 + 0x22) == 0x0 ||
            (puVar1 = iVar6->field16_0x10, uVar6 = (u16)((u32)puVar1 >> 0x10), iVar4 = (int)puVar1,
            BVar3 = write_to_file_1008_7e1c
                              ((u8 *)param_3,*(u32 *)(iVar4 + 0x24),(char *)(u32)(u16)((iVar4 + 0x22) * 0x2),
                               in_stack_0000ffc8), BVar3 != 0x0)))) {
          local_c[0] = iVar6->field17_0x14;
          BVar3 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffc8);
          if (BVar3 != 0x0) {
            local_c[0] = iVar6->field18_0x16;
            BVar3 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffc8);
            if (BVar3 != 0x0) {
              local_c[0] = iVar6->field19_0x18;
              BVar3 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffc8);
              if (BVar3 != 0x0) {
                local_c[0] = iVar6->field20_0x1a;
                BVar3 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffc8);
                if (BVar3 != 0x0) {
                  return;
                }
              }
            }
          }
        }
      }
    }
  }
  u16_1050_0310 = 0x6d0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_2c8a(i16 param_1,StructD *param_2,astruct_373 *param_3,HFILE16 *param_4)

{
  u16 *puVar1;
  u16 uVar2;
  BOOL16 BVar3;
  u8 *puVar4;
  u16 uVar5;
  astruct_57 *paVar6;
  astruct_374 *iVar7;
  astruct_371 *iVar8;
  astruct_372 *iVar9;
  u16 unaff_SI;
  u16 uVar7;
  u16 *puVar8;
  u32 *puVar9;
  u16 in_stack_0000fa72;
  u16 in_stack_0000fb96;
  u16 in_stack_0000fb9c;
  u16 in_stack_0000fba0;
  u16 *puStack1038;
  astruct_430 *local_406;
  u16 local_404;
  u8 local_402 [0x400];
  astruct_373 *uVar9;
  astruct_373 *iVar14;

  iVar14 = (astruct_373 *)param_3;
  uVar9 = (astruct_373 *)((u32)param_3 >> 0x10);
  file_1030_19b4(param_1,(int)param_2,param_3,param_4);
  if (param_1 == 0x0) {
    return;
  }
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_2);
  }
  else {
    param_2 = (StructD *)((u32)param_2 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
  }
  uVar2 = fn_ptr_op_1000_1708(0x28,0x0,0x1,(u16)PTR_LOOP_1050_5f2c,(u16)param_2);
  uVar5 = (u16)param_2;
  puStack1038 = (u16 *)CONCAT22(uVar5,uVar2);
  paVar6 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)(uVar5 | uVar2));
  if ((uVar5 | uVar2) == 0x0) {
    iVar14->field13_0x10 = NULL;
  }
  else {
    puVar8 = pass1_1008_3e38((astruct_19 *)CONCAT22(uVar5,uVar2 + 0x6));
    paVar6 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)puVar8 >> 0x10);
    iVar14->field13_0x10 = puStack1038;
  }
  BVar3 = read_file_1008_7dee(param_4,(u8 *)iVar14->field13_0x10,0x2);
  if (BVar3 != 0x0) {
    puVar4 = local_402;
    read_file_1008_7c6e((HFILE16)param_4,(u16)((u32)param_4 >> 0x10),(char *)CONCAT22(0x1050,puVar4));
    if (puVar4 != NULL) {
      uVar2 = str_op_1008_60e8((u16)paVar6,(char *)CONCAT22(0x1050,local_402));
      puVar1 = iVar14->field13_0x10;
      uVar7 = (u16)((u32)puVar1 >> 0x10);
      iVar7 = (astruct_374 *)puVar1;
      iVar7->field2_0x2 = uVar2;
      iVar7->field3_0x4 = (uchar *)paVar6;
      puVar1 = iVar14->field13_0x10;
      BVar3 = read_file_1008_7bc8((u32)param_4,(u16 *)((u32)puVar1 & 0xffff0000 | (u32)((int)puVar1 + 0x6)));
      if ((((BVar3 != 0x0) &&
           (puVar1 = iVar14->field13_0x10,
           BVar3 = read_file_1008_7dee(param_4,(u8 *)((u32)puVar1 & 0xffff0000 | (u32)((int)puVar1 + 0xc)),0x2),
           BVar3 != 0x0)) &&
          (puVar1 = iVar14->field13_0x10,
          BVar3 = read_file_1008_7dee(param_4,(u8 *)((u32)puVar1 & 0xffff0000 | (u32)((int)puVar1 + 0xe)),0x4),
          BVar3 != 0x0)) &&
         ((puVar1 = iVar14->field13_0x10,
          BVar3 = read_file_1008_7dee(param_4,(u8 *)((u32)puVar1 & 0xffff0000 | (u32)((int)puVar1 + 0x12)),0x10),
          BVar3 != 0x0 &&
          (puVar1 = iVar14->field13_0x10,
          BVar3 = read_file_1008_7dee(param_4,(u8 *)((u32)puVar1 & 0xffff0000 | (u32)((int)puVar1 + 0x22)),0x2),
          BVar3 != 0x0)))) {
        puVar1 = iVar14->field13_0x10;
        if (((int)puVar1 + 0x22) != 0x0) {
          puVar1 = iVar14->field13_0x10;
          uVar7 = (u16)((u32)puVar1 >> 0x10);
          iVar8 = (astruct_371 *)puVar1;
          uVar2 = iVar8->field34_0x22 * 0x2;
          mem_op_1000_179c(uVar2,paVar6);
          iVar8->field35_0x24 = uVar2;
          iVar8->field36_0x26 = (uchar *)paVar6;
          puVar1 = iVar14->field13_0x10;
          uVar7 = (u16)((u32)puVar1 >> 0x10);
          iVar9 = (astruct_372 *)puVar1;
          BVar3 = read_file_1008_7dee(param_4,(u8 *)iVar9->field35_0x24,(u32)(u16)(iVar9->field34_0x22 * 0x2));
          if (BVar3 == 0x0) {
            u16_1050_0310 = 0x6d2;
            return;
          }
        }
        BVar3 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar14->field_0x14)),0x2);
        if (((BVar3 != 0x0) &&
            (BVar3 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_404),0x2), BVar3 != 0x0)) &&
           ((BVar3 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar14->field_0x18)),0x2),
            BVar3 != 0x0 && (BVar3 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_406),0x2), BVar3 != 0x0)))
           ) {
          iVar14->field16_0x16 = local_404;
          iVar14->field19_0x1a = local_406;
          puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fa72,
                                   in_stack_0000fb96,in_stack_0000fb9c,in_stack_0000fba0);
          pass1_1018_04a4((u32)puVar9,iVar14->field4_0x4);
          pass1_1010_82f8(_u16_1050_14cc,*iVar14->field13_0x10);
          return;
        }
      }
    }
  }
  u16_1050_0310 = 0x6d2;
  return;
}



i16 pass1_1030_2f1a(u32 param_1,u16 *param_2,u16 *param_3)

{
  i16 iVar1;
  u32 uVar2;
  i16 iVar3;

  uVar2 = *(u32 *)((int)param_1 + 0x10);
  iVar3 = (int)uVar2;
  iVar1 = (iVar3 + 0xc);
  if (iVar1 - 0x1U < 0x9) {
    switch(iVar1) {
    default:
      *param_3 = 0x19;
      *param_2 = 0x2d;
      return iVar3;
    case 0x3:
    case 0x4:
    case 0x5:
      *param_3 = 0xa;
      *param_2 = 0xf;
      return iVar3;
    case 0x6:
      *param_3 = 0xa;
      *param_2 = 0x19;
      return iVar3;
    case 0x7:
      *param_3 = 0x19;
      *param_2 = 0x37;
      return iVar3;
    }
  }
  *param_3 = 0x0;
  *param_2 = 0x0;
  return 0x0;
}



u16 pass1_1030_2fac(astruct_598 *param_1)

{
  i32 lVar1;
  astruct_598 *iVar2;
  u16 uVar2;

  uVar2 = (u16)((u32)param_1 >> 0x10);
  iVar2 = (astruct_598 *)param_1;
  if (iVar2->field16_0x10 == 0x0) {
    return 0x0;
  }
  lVar1 = iVar2->field16_0x10;
  if (((int)lVar1 + 0xc) < 0x2) {
    return 0x4;
  }
  lVar1 = iVar2->field16_0x10;
  if (((int)lVar1 + 0xc) < 0x5) {
    return 0x3;
  }
  lVar1 = iVar2->field16_0x10;
  if (((int)lVar1 + 0xc) < 0x8) {
    return 0x2;
  }
  return 0x1;
}



void pass1_1030_3006(u32 param_1,u32 param_2)

{
  *(u32 *)((int)param_1 + 0x10) = param_2;
  return;
}



void pass1_1030_301a(u16 param_1,u32 param_2,char *param_3)

{
  u32 uVar1;
  u16 uVar2;
  i16 iVar4;
  astruct_608 *iVar3;
  u16 uVar5;

  uVar5 = (u16)(param_2 >> 0x10);
  iVar4 = (int)param_2;
  if (*(i32 *)(iVar4 + 0x10) != 0x0) {
    uVar1 = *(u32 *)(iVar4 + 0x10);
    fn_ptr_1000_17ce(*(char **)((int)uVar1 + 0x2));
    uVar2 = str_op_1008_60e8(param_1,param_3);
    uVar1 = *(u32 *)(iVar4 + 0x10);
    uVar5 = (u16)((u32)uVar1 >> 0x10);
    iVar3 = (astruct_608 *)uVar1;
    iVar3->field2_0x2 = uVar2;
    iVar3->field3_0x4 = param_1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1030_3058(u16 param_1,astruct_57 *param_2,astruct_375 *param_3)

{
  u16 *puVar1;
  code **ppcVar2;
  u32 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  astruct_375 *iVar4;
  astruct_375 *uVar7;
  u32 uVar8;
  u16 uStack4;

  uVar7 = (astruct_375 *)((u32)param_3 >> 0x10);
  iVar4 = (astruct_375 *)param_3;
  if (iVar4->field12_0xc == NULL) {
    mem_op_1000_179c(0x18,param_2);
    uVar5 = (u16)param_2;
    uVar6 = uVar5 | param_1;
    param_2 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)uVar6);
    if (uVar6 == 0x0) {
      iVar4->field12_0xc = NULL;
    }
    else {
      uVar4 = struct_op_1030_1cd8((astruct_75 *)CONCAT22(uVar5,param_1),0x5,0x5);
      &iVar4->field12_0xc = uVar4;
      ((int)&iVar4->field12_0xc + 0x2) = (int)param_2;
    }
  }
  for (uStack4 = 0x0; uVar3 = iVar4->field13_0x10, puVar1 = (u16 *)((int)uVar3 + 0x22),
      uStack4 <= *puVar1 && *puVar1 != uStack4; uStack4 += 0x1) {
    uVar8 = pass1_1028_e2e0(param_2,_PTR_LOOP_1050_65e2,0x3);
    param_2 = (astruct_57 *)((u32)param_2 & 0xffff0000 | uVar8 >> 0x10);
    ppcVar2 = (code **)((int)*iVar4->field12_0xc + 0x8);
    (**ppcVar2)(0x1028,iVar4->field12_0xc,(int)uVar8,(int)(uVar8 >> 0x10),uStack4,0x0);
  }
  return 0x1;
}



StructD * pass1_1030_310a(StructD *param_1,u8 param_2)

{
  pass1_1030_29e6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_314c(u16 param_1,astruct_364 *param_2,u32 param_3)

{
  u16 in_register_0000000a;
  astruct_57 *paVar1;
  astruct_364 *pstruct_1;
  u16 unaff_SI;
  u16 uVar2;
  u16 in_stack_0000fe9a;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffc8;
  i16 iStack12;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar2 = (u16)((u32)param_2 >> 0x10);
  pstruct_1 = (astruct_364 *)param_2;
  param_2->field0_0x0 = 0x389a;
  pstruct_1->field1_0x2 = 0x1008;
  pstruct_1->field366_0x170 = 0x0;
  pstruct_1->field409_0x1a4 = param_3;
  pstruct_1->field410_0x1a8 = 0x5;
  pstruct_1->field411_0x1aa = 0x0;
  pstruct_1->field412_0x1ae = 0x10;
  param_2->field0_0x0 = 0x3af2;
  pstruct_1->field1_0x2 = 0x1030;
  pass1_1000_4906((StructD *)((u32)param_2 & 0xffff0000 | ZEXT24(&pstruct_1->field_0x4)),NULL,0x16c);
  pass1_1000_4906((StructD *)((u32)param_2 & 0xffff0000 | ZEXT24(&pstruct_1->field_0x18c)),NULL,0x18);
  pass1_1000_4906((StructD *)((u32)param_2 & 0xffff0000 | ZEXT24(&pstruct_1->field_0x174)),NULL,0xc);
  pass1_1000_4906((StructD *)((u32)param_2 & 0xffff0000 | ZEXT24(&pstruct_1->field_0x180)),NULL,0xc);
  mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000fe9a,in_stack_0000ffbe,
                  in_stack_0000ffc4,in_stack_0000ffc8);
  if ((int)PTR_LOOP_1050_13ae < 0x2) {
    pass1_1030_34da(param_2);
  }
  else {
    pstruct_1->field369_0x176 = 0x1;
    pstruct_1->field370_0x178 = 0x2;
    pstruct_1->field371_0x17a = 0x2;
    pstruct_1->field372_0x17c = 0x60001;
    for (iStack12 = 0x1; iStack12 < 0x6; iStack12 += 0x1) {
      (&pstruct_1->field_0x180 + iStack12 * 0x2) = 0x64;
    }
  }
  return;
}



void pass1_1030_3258(u32 param_1,u16 param_2)

{
  ((int)param_1 + 0x1ae) = param_2;
  return;
}



void pass1_1030_326a(u32 param_1,u16 param_2,astruct_692 *param_3)

{
  u16 uVar1;
  u32 uVar2;
  u16 uVar3;
  astruct_692 *iVar4;
  u16 uVar4;
  i32 lStack6;

  uVar4 = (u16)((u32)param_3 >> 0x10);
  iVar4 = (astruct_692 *)param_3;
  if (iVar4->field426_0x1aa == 0x0) {
    iVar4->field426_0x1aa = 0x1;
  }
  else {
    param_1 = iVar4->field426_0x1aa * 0x2;
    iVar4->field426_0x1aa = param_1;
  }
  uVar1 = (u16)param_1;
  pass1_1030_38b8();
  lStack6 = CONCAT22(param_2,uVar1);
  uVar2 = iVar4->field426_0x1aa;
  uVar3 = ((int)&iVar4->field426_0x1aa + 0x2);
  if (lStack6 < (long)uVar2) {
    uVar2 = (u32)uVar1;
    uVar3 = param_2;
  }
  &iVar4->field426_0x1aa = (int)uVar2;
  ((int)&iVar4->field426_0x1aa + 0x2) = uVar3;
  pass1_1030_375a((u32)param_3 & 0xffff | (u32)uVar4 << 0x10,0x0,uVar2 & 0xffff | (u32)uVar3 << 0x10);
  return;
}



void write_to_file_1030_32e4(u32 param_1,u32 param_2)

{
  i16 iVar1;
  BOOL16 BVar2;
  u16 uVar3;
  HFILE16 in_stack_0000ffd0;
  u32 local_16 [0x2];
  u16 local_c;
  u32 local_a [0x2];

  iVar1 = (int)param_1;
  BVar2 = write_to_file_1008_7e1c
                    ((u8 *)param_2,param_1 & 0xffff0000 | (u32)(iVar1 + 0x4),(char *)0x16c,in_stack_0000ffd0);
  if (BVar2 != 0x0) {
    BVar2 = write_to_file_1008_7e1c
                      ((u8 *)param_2,param_1 & 0xffff0000 | (u32)(iVar1 + 0x174),&DAT_0000_000c,in_stack_0000ffd0);
    if (BVar2 != 0x0) {
      BVar2 = write_to_file_1008_7e1c
                        ((u8 *)param_2,param_1 & 0xffff0000 | (u32)(iVar1 + 0x180),&DAT_0000_000c,in_stack_0000ffd0);
      if (BVar2 != 0x0) {
        BVar2 = write_to_file_1008_7e1c
                          ((u8 *)param_2,param_1 & 0xffff0000 | (u32)(iVar1 + 0x18c),(char *)0x18,in_stack_0000ffd0);
        if (BVar2 != 0x0) {
          uVar3 = (u16)(param_1 >> 0x10);
          local_c = (iVar1 + 0x1a8);
          BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffd0);
          if (BVar2 != 0x0) {
            local_16[0] = *(u32 *)(iVar1 + 0x1aa);
            BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_16),(char *)0x4,in_stack_0000ffd0);
            if (BVar2 != 0x0) {
              local_a[0] = *(u32 *)(iVar1 + 0x170);
              BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_a),(char *)0x4,in_stack_0000ffd0);
              if (BVar2 != 0x0) {
                local_c = (iVar1 + 0x1ae);
                BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffd0);
                if (BVar2 != 0x0) {
                  return;
                }
              }
            }
          }
        }
      }
    }
  }
  u16_1050_0310 = 0x6d0;
  return;
}



void read_file_1030_33f0(astruct_430 *param_1,HFILE16 *param_2)

{
  astruct_430 *iVar2;
  BOOL16 BVar1;

  iVar2 = (astruct_430 *)param_1;
  iVar2 = (astruct_430 *)&iVar2->field_0x4;
  BVar1 = read_file_1008_7dee(param_2,(u8 *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar2)),0x16c);
  if (((((BVar1 != 0x0) &&
        (BVar1 = read_file_1008_7dee(param_2,(u8 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x174)),0xc),
        BVar1 != 0x0)) &&
       (BVar1 = read_file_1008_7dee(param_2,(u8 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x180)),0xc),
       BVar1 != 0x0)) &&
      ((BVar1 = read_file_1008_7dee(param_2,(u8 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x18c)),0x18),
       BVar1 != 0x0 &&
       (BVar1 = read_file_1008_7dee(param_2,(u8 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x1a8)),0x2),
       BVar1 != 0x0)))) &&
     (BVar1 = read_file_1008_7dee(param_2,(u8 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x1aa)),0x4),
     BVar1 != 0x0)) {
    if ((int)u16_1050_0312 < 0x2) {
      return;
    }
    BVar1 = read_file_1008_7dee(param_2,(u8 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x170)),0x4);
    if ((BVar1 != 0x0) &&
       (BVar1 = read_file_1008_7dee(param_2,(u8 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x1ae)),0x2),
       BVar1 != 0x0)) {
      return;
    }
  }
  u16_1050_0310 = 0x6d2;
  return;
}



void pass1_1030_34da(astruct_364 *param_1)

{
  astruct_364 *iVar1;
  u16 uVar1;

  uVar1 = (u16)((u32)param_1 >> 0x10);
  iVar1 = (astruct_364 *)param_1;
  iVar1->field369_0x176 = 0x1;
  iVar1->field370_0x178 = 0x1;
  iVar1->field371_0x17a = 0x1;
  &iVar1->field372_0x17c = 0x1;
  ((int)&iVar1->field372_0x17c + 0x2) = 0x4;
  &iVar1->field_0x182 = 0x32;
  &iVar1->field_0x184 = 0xa;
  &iVar1->field_0x186 = 0xa;
  &iVar1->field_0x188 = 0xa;
  &iVar1->field_0x18a = 0x4b;
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1->field_0x18c)),NULL,0x18);
  return;
}



void pass1_1030_3534(u32 param_1,u32 param_2)

{
  *(u32 *)((int)param_1 + 0x4) = param_2;
  return;
}



void pass1_1030_3548(u32 param_1,i32 param_2)

{
  i32 *plVar1;

  plVar1 = (i32 *)((int)param_1 + 0x4);
  *plVar1 = *plVar1 + param_2;
  return;
}



void pass1_1030_355c(u32 param_1,u32 param_2)

{
  i16 iVar1;
  u16 uVar2;
  i16 iStack4;

  iStack4 = 0x0;
  do {
    iVar1 = iStack4 * 0x4;
    uVar2 = (u16)(param_1 >> 0x10);
    *(i32 *)((int)param_1 + iVar1 + 0x4) = *(i32 *)(iVar1 + (int)param_2) + *(i32 *)((int)param_1 + 0x4 + iVar1);
    iStack4 += 0x1;
  } while (iStack4 < 0x5b);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_35a4(uchar *param_1,u32 param_2,i32 param_3)

{
  u16 *puVar1;
  i16 *piVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u32 uVar6;
  i16 iVar7;
  u16 in_register_0000000a;
  StructD *pSVar8;
  u16 uVar9;
  u8 local_c [0x2];
  u32 local_a;
  u32 uStack6;

  pSVar8 = (StructD *)CONCAT22(in_register_0000000a,param_1);
  vsprintf_op_1030_840a((u16)param_1,(u32)s_Pop_Leaving__ld_1050_516a);
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar8);
  }
  else {
    pSVar8 = (StructD *)(_PTR_LOOP_1050_5f2c >> 0x10);
  }
  uVar4 = fn_ptr_op_1000_1708(0x16c,0x0,0x1,(u16)PTR_LOOP_1050_5f2c,(u16)pSVar8);
  uStack6 = CONCAT22((int)pSVar8,uVar4);
  pass1_1030_3948(param_2,(u16 *)CONCAT22(0x1050,local_c),(i16 *)CONCAT13(0x10,CONCAT12(0x50,&local_a)),0x3);
  uVar6 = (u32)((int)&local_a + 0x2U);
  pass1_1030_3948(param_2,(u16 *)CONCAT22(0x1050,(int)&local_a + 0x2U),(i16 *)CONCAT13(0x10,CONCAT12(0x50,local_c)),
                  0x4);
  do {
    uVar5 = (u16)uVar6;
    if (param_3 < 0x1) break;
    pass1_1008_612e(uVar5,(int)local_a,(int)((u32)local_a >> 0x10));
    uVar6 = ZEXT24(&param_3);
    pass1_1030_3a3a(param_2,(i32 *)CONCAT13(0x10,CONCAT12(0x50,&param_3)),uVar5);
    uVar9 = (u16)((u32)uStack6 >> 0x10);
    puVar1 = (u16 *)(uVar5 * 0x4 + (int)uStack6);
    uVar3 = *puVar1;
    *puVar1 = *puVar1 + (u16)uVar6;
    piVar2 = (i16 *)(uVar5 * 0x4 + (int)uStack6 + 0x2);
    *piVar2 = *piVar2 + (int)pSVar8 + (u16)CARRY2(uVar3,(u16)uVar6);
    pass1_1030_38f2(param_2,0x3);
    uVar5 = (u16)uVar6;
    iVar7 = (int)pSVar8;
    pass1_1030_38f2(param_2,0x4);
  } while ((iVar7 + (int)pSVar8 + (u16)CARRY2(uVar5,(u16)uVar6) | uVar5 + (u16)uVar6) != 0x0);
  pass1_1000_4906((StructD *)(param_2 & 0xffff0000 | (u32)((int)param_2 + 0x18c)),NULL,0x18);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_3694(uchar *param_1,u32 param_2,i16 param_3,i32 param_4)

{
  u16 *puVar1;
  i16 *piVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u32 uVar6;
  u16 uVar7;
  u16 in_register_0000000a;
  StructD *pSVar8;

  pSVar8 = (StructD *)CONCAT22(in_register_0000000a,param_1);
  vsprintf_op_1030_840a((u16)param_1,(u32)s_Pop_Leaving__ld_1050_517a);
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar8);
  }
  else {
    pSVar8 = (StructD *)(_PTR_LOOP_1050_5f2c >> 0x10);
  }
  uVar4 = fn_ptr_op_1000_1708(0x16c,0x0,0x1,(u16)PTR_LOOP_1050_5f2c,(u16)pSVar8);
  uVar7 = SUB42(pSVar8,0x0);
  uVar6 = (u32)(param_3 - 0x1U);
  if (((param_3 < 0x1) || (SBORROW2(param_3,0x1))) ||
     (uVar6 = (u32)(param_3 - 0x5U), param_3 - 0x5U != 0x0 && 0x3 < (int)(param_3 - 0x1U))) {
    while (uVar5 = (u16)uVar6, 0x0 < param_4) {
      pass1_1008_612e(uVar5,0x0,0x5a);
      uVar6 = ZEXT24(&param_4);
      pass1_1030_3a3a(param_2,(i32 *)CONCAT13(0x10,CONCAT12(0x50,&param_4)),uVar5);
      puVar1 = (u16 *)(uVar5 * 0x4 + uVar4);
      uVar3 = *puVar1;
      *puVar1 = *puVar1 + (u16)uVar6;
      piVar2 = (i16 *)(uVar5 * 0x4 + uVar4 + 0x2);
      *piVar2 = *piVar2 + (int)pSVar8 + (u16)CARRY2(uVar3,(u16)uVar6);
    }
  }
  else {
    pass1_1030_39dc(param_2,(i32 *)CONCAT22(0x1050,&param_4),
                    CONCAT13((char)((u32)pSVar8 >> 0x8),CONCAT12((char)pSVar8,uVar4)),param_3);
  }
  pass1_1000_4906((StructD *)(param_2 & 0xffff0000 | (u32)((int)param_2 + 0x18c)),NULL,0x18);
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1030_375a(u32 param_1,i16 param_2,i32 param_3)

{
  i16 iVar1;
  i16 iVar2;
  u16 uVar3;
  i32 lVar4;
  i32 lVar5;
  i16 iVar6;
  i16 iVar7;
  u16 uVar8;
  i16 iStack20;
  u32 uStack18;
  i16 local_6;
  i16 local_4;

  iVar6 = (int)param_1;
  if (param_2 == 0x0) {
    local_4 = 0x5a;
    while ((-0x1 < local_4 && (pass1_1030_3a3a(param_1,(i32 *)CONCAT22(0x1050,&param_3),local_4), param_3 != 0x0))) {
      local_4 += -0x1;
    }
  }
  else {
    pass1_1030_3948(param_1,(u16 *)CONCAT22(0x1050,&local_4),(i16 *)CONCAT22(0x1050,&local_6),param_2);
    iVar2 = (local_4 - local_6) + 0x1;
    lVar4 = param_3 / (long)iVar2;
    lVar5 = lVar4 * iVar2;
    uVar3 = (u16)lVar5;
    uStack18 = CONCAT22(((int)((u32)param_3 >> 0x10) - (int)((u32)lVar5 >> 0x10)) - (u16)((u16)param_3 < uVar3),
                        (u16)param_3 - uVar3);
    for (iStack20 = local_6; iStack20 <= local_4; iStack20 += 0x1) {
      iVar7 = iStack20 * 0x4;
      uVar8 = (u16)(param_1 >> 0x10);
      *(i32 *)(iVar6 + iVar7 + 0x4) = *(i32 *)(iVar6 + iVar7 + 0x4) - lVar4;
      iVar2 = (iVar6 + iVar7 + 0x6);
      if ((uStack18 | (u16)uStack18) != 0x0) {
        iVar1 = (iVar6 + iVar7 + 0x4);
        (iVar6 + iVar7 + 0x4) = iVar1 + -0x1;
        (iVar6 + iVar7 + 0x6) = iVar2 - (u16)(iVar1 == 0x0);
        uStack18 += -0x1;
      }
      if ((iVar6 + iStack20 * 0x4 + 0x6) < 0x0) {
        *(u32 *)(iVar6 + iStack20 * 0x4 + 0x4) = 0x0;
      }
    }
  }
  pass1_1000_4906((StructD *)(param_1 & 0xffff0000 | (u32)(iVar6 + 0x18c)),NULL,0x18);
  return;
}



void pass1_1030_387c(u32 param_1)

{
  i16 iStack4;

  iStack4 = 0x5a;
  do {
    *(u32 *)(iStack4 * 0x4 + (int)param_1 + 0x4) = *(u32 *)(iStack4 * 0x4 + (int)param_1);
    iStack4 += -0x1;
  } while (0x0 < iStack4);
  *(u32 *)((int)param_1 + 0x4) = 0x0;
  return;
}



void pass1_1030_38b8(void)

{
  i16 iStack8;

  iStack8 = 0x0;
  do {
    iStack8 += 0x1;
  } while (iStack8 < 0x5b);
  return;
}



void pass1_1030_38f2(u32 param_1,i16 param_2)

{
  i16 iStack12;
  i16 local_a;
  i16 local_8;
  u32 uStack6;

  uStack6 = 0x0;
  pass1_1030_3948(param_1,(u16 *)CONCAT22(0x1050,&local_a),(i16 *)CONCAT22(0x1050,&local_8),param_2);
  for (iStack12 = local_8; iStack12 <= local_a; iStack12 += 0x1) {
  }
  return;
}



void pass1_1030_3948(u32 param_1,u16 *param_2,i16 *param_3,i16 param_4)

{
  u16 uVar1;

  if (param_4 == 0x1) {
    *param_3 = 0x0;
    *param_2 = 0x3;
    return;
  }
  uVar1 = (u16)(param_1 >> 0x10);
  if (param_4 == 0x2) {
    *param_3 = 0x4;
    *param_2 = ((int)param_1 + 0x1ae);
    return;
  }
  if (param_4 == 0x3) {
    *param_3 = ((int)param_1 + 0x1ae) + 0x1;
    *param_2 = 0x27;
    return;
  }
  if (param_4 != 0x4) {
    if (param_4 == 0x5) {
      *param_3 = 0x4c;
    }
    else {
      *param_3 = 0x0;
    }
    *param_2 = 0x5a;
    return;
  }
  *param_3 = 0x28;
  *param_2 = 0x4b;
  return;
}



void pass1_1030_39dc(u32 param_1,i32 *param_2,u32 param_3,i16 param_4)

{
  i16 iVar1;
  u16 in_DX;
  u16 uVar2;
  i16 iStack8;
  i16 local_6;
  i16 local_4;

  pass1_1030_3948(param_1,(u16 *)CONCAT22(0x1050,&local_6),(i16 *)CONCAT22(0x1050,&local_4),param_4);
  iStack8 = local_6;
  while( true ) {
    if (iStack8 < local_4) {
      return;
    }
    iVar1 = local_4;
    pass1_1030_3a3a(param_1,param_2,iStack8);
    uVar2 = (u16)(param_3 >> 0x10);
    (iStack8 * 0x4 + (int)param_3) = iVar1;
    (iStack8 * 0x4 + (int)param_3 + 0x2) = in_DX;
    if (*param_2 == 0x0) break;
    iStack8 += -0x1;
  }
  return;
}



void pass1_1030_3a3a(u32 param_1,i32 *param_2,i16 param_3)

{
  i16 *piVar1;
  i16 iVar2;
  i16 iVar3;
  u16 uVar4;
  u16 uVar5;
  i16 iVar6;
  i16 iVar7;
  i16 iVar8;
  u16 uVar9;

  iVar2 = ((int)param_2 + 0x2);
  uVar9 = (u16)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  iVar7 = iVar6 + 0x4;
  iVar8 = param_3 * 0x4;
  piVar1 = (i16 *)(iVar7 + iVar8 + 0x2);
  iVar3 = *piVar1;
  if ((iVar3 < iVar2) ||
     ((uVar5 = (u16)*param_2, *piVar1 == iVar2 || iVar3 < iVar2 && ((iVar7 + iVar8) < uVar5)))) {
    *param_2 = *param_2 - *(i32 *)(iVar6 + 0x4 + param_3 * 0x4);
    *(u32 *)(iVar6 + param_3 * 0x4 + 0x4) = 0x0;
  }
  else {
    uVar4 = (iVar7 + iVar8);
    iVar3 = (iVar7 + iVar8 + 0x2);
    (iVar6 + iVar8 + 0x4) = uVar4 - uVar5;
    (iVar6 + iVar8 + 0x6) = (iVar3 - iVar2) - (u16)(uVar4 < uVar5);
    *param_2 = 0x0;
  }
  return;
}



StructD * pass1_1030_3ac6(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u32 * pass1_1030_3af6(u32 *param_1,u16 param_2,u16 param_3,u32 *param_4,u16 param_5)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = (u16)((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = *param_4;
  (iVar1 + 0x4) = (param_4 + 0x1);
  (iVar1 + 0x6) = param_3;
  (iVar1 + 0x8) = param_2;
  return param_1;
}



u16 pass1_1030_3b28(void)

{
  u16 *puVar1;
  u32 *puVar2;
  u8 local_8 [0x6];

  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xffc4,0x0);
  pass1_1030_3af6((u32 *)&u16_1050_65e6,0x115,0x15b,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x0,0x19);
  pass1_1030_3af6((u32 *)&u16_1050_65f0,0x116,0x15c,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xffdd,0x32);
  pass1_1030_3af6((u32 *)&u16_1050_65fa,0x117,0x15d,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x0,0x4b);
  pass1_1030_3af6((u32 *)&u16_1050_6604,0x118,0x15e,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xf,0x64);
  pass1_1030_3af6((u32 *)&u16_1050_660e,0x119,0x15f,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x28,0x7d);
  pass1_1030_3af6((u32 *)&u16_1050_6618,0x11a,0x160,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xffec,0x96);
  pass1_1030_3af6((u32 *)&u16_1050_6622,0x11b,0x161,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x14,0xaf);
  pass1_1030_3af6((u32 *)&u16_1050_662c,0x11c,0x162,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x1e,0xc8);
  pass1_1030_3af6((u32 *)&u16_1050_6636,0x11d,0x163,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xfffb,0xe1);
  pass1_1030_3af6((u32 *)&u16_1050_6640,0x11e,0x164,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x32,0xfa);
  pass1_1030_3af6((u32 *)&u16_1050_664a,0x11f,0x165,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x1e,0xe1);
  pass1_1030_3af6((u32 *)&u16_1050_6654,0x120,0x166,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xffe7,0xfa);
  pass1_1030_3af6((u32 *)&u16_1050_665e,0x121,0x167,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x0,0x113);
  pass1_1030_3af6((u32 *)&u16_1050_6668,0x122,0x168,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x28,0x12c);
  pass1_1030_3af6((u32 *)&u16_1050_6672,0x123,0x169,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xf,0x145);
  pass1_1030_3af6((u32 *)&u16_1050_667c,0x124,0x16a,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xffec,0x15e);
  pass1_1030_3af6((u32 *)&u16_1050_6686,0x125,0x16b,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x0,0x0);
  pass1_1030_3af6((u32 *)&u16_1050_6690,0x126,0x16c,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x2d,0x19);
  pass1_1030_3af6((u32 *)&u16_1050_669a,0x127,0x16d,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xa,0x32);
  pass1_1030_3af6((u32 *)&u16_1050_66a4,0x128,0x16e,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xffe2,0x4b);
  pass1_1030_3af6((u32 *)&u16_1050_66ae,0x129,0x16f,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x5,0x64);
  pass1_1030_3af6((u32 *)&u16_1050_66b8,0x12a,0x170,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x32,0x7d);
  pass1_1030_3af6((u32 *)&u16_1050_66c2,0x12b,0x171,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xffc9,0x96);
  pass1_1030_3af6((u32 *)&u16_1050_66cc,0x12c,0x172,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xfffb,0xaf);
  pass1_1030_3af6((u32 *)&u16_1050_66d6,0x12d,0x173,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xffe7,0xc8);
  pass1_1030_3af6((u32 *)&u16_1050_66e0,0x12e,0x174,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x32,0x32);
  pass1_1030_3af6((u32 *)&u16_1050_66ea,0x12f,0x175,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x3c,0x64);
  pass1_1030_3af6((u32 *)&u16_1050_66f4,0x130,0x176,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xffc4,0xe1);
  pass1_1030_3af6((u32 *)&u16_1050_66fe,0x131,0x177,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x0,0x19);
  pass1_1030_3af6((u32 *)&u16_1050_6708,0x132,0x178,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x5,0xaf);
  pass1_1030_3af6((u32 *)&u16_1050_6712,0x133,0x179,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x0,0x19);
  pass1_1030_3af6((u32 *)&u16_1050_671c,0x134,0x17a,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x23,0x19);
  pass1_1030_3af6((u32 *)&u16_1050_6726,0x135,0x17b,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xfffb,0x32);
  pass1_1030_3af6((u32 *)&u16_1050_6730,0x136,0x17c,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xf,0x32);
  pass1_1030_3af6((u32 *)&u16_1050_673a,0x137,0x17d,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x2d,0x4b);
  pass1_1030_3af6((u32 *)&u16_1050_6744,0x138,0x17e,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x1e,0x4b);
  pass1_1030_3af6((u32 *)&u16_1050_674e,0x139,0x17f,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x2d,0x64);
  pass1_1030_3af6((u32 *)&u16_1050_6758,0x13a,0x180,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xffe7,0x7d);
  pass1_1030_3af6((u32 *)&u16_1050_6762,0x13b,0x181,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x5,0xaf);
  pass1_1030_3af6((u32 *)&u16_1050_676c,0x13c,0x182,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x0,0xc8);
  pass1_1030_3af6((u32 *)&u16_1050_6776,0x13d,0x183,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xffce,0xc8);
  pass1_1030_3af6((u32 *)&u16_1050_6780,0x13e,0x184,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xf,0xfa);
  pass1_1030_3af6((u32 *)&u16_1050_678a,0x13f,0x185,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x1e,0x113);
  pass1_1030_3af6((u32 *)&u16_1050_6794,0x140,0x186,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xffe2,0x12c);
  pass1_1030_3af6((u32 *)&u16_1050_679e,0x141,0x187,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x64,0x12c);
  pass1_1030_3af6((u32 *)&u16_1050_67a8,0x142,0x188,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x32,0x145);
  pass1_1030_3af6((u32 *)&u16_1050_67b2,0x143,0x189,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x64,0x145);
  pass1_1030_3af6((u32 *)&u16_1050_67bc,0x144,0x18a,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x1e,0x15e);
  pass1_1030_3af6((u32 *)&u16_1050_67c6,0x145,0x18b,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xffd3,0x15e);
  pass1_1030_3af6((u32 *)&u16_1050_67d0,0x146,0x18c,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x32,0xfa);
  pass1_1030_3af6((u32 *)&u16_1050_67da,0x147,0x18d,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xf,0x19);
  pass1_1030_3af6((u32 *)&u16_1050_67e4,0x148,0x18e,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x0,0x32);
  pass1_1030_3af6((u32 *)&u16_1050_67ee,0x149,0x18f,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x0,0xaf);
  pass1_1030_3af6((u32 *)&u16_1050_67f8,0x14a,0x190,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xfffb,0xe1);
  pass1_1030_3af6((u32 *)&u16_1050_6802,0x14b,0x191,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xa,0x15e);
  pass1_1030_3af6((u32 *)&u16_1050_680c,0x14c,0x192,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x0,0x19);
  pass1_1030_3af6((u32 *)&u16_1050_6816,0x14d,0x193,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x1e,0x32);
  pass1_1030_3af6((u32 *)&u16_1050_6820,0x14e,0x194,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xfffb,0x64);
  pass1_1030_3af6((u32 *)&u16_1050_682a,0x14f,0x195,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xf,0x64);
  pass1_1030_3af6((u32 *)&u16_1050_6834,0x150,0x196,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x1e,0x7d);
  pass1_1030_3af6((u32 *)&u16_1050_683e,0x151,0x197,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xffdd,0xe1);
  pass1_1030_3af6((u32 *)&u16_1050_6848,0x152,0x198,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x0,0x113);
  pass1_1030_3af6((u32 *)&u16_1050_6852,0x153,0x199,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x2d,0x12c);
  pass1_1030_3af6((u32 *)&u16_1050_685c,0x154,0x19a,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xffe7,0x145);
  pass1_1030_3af6((u32 *)&u16_1050_6866,0x155,0x19b,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xa,0x15e);
  pass1_1030_3af6((u32 *)&u16_1050_6870,0x156,0x19c,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x0,0x4b);
  pass1_1030_3af6((u32 *)&u16_1050_687a,0x157,0x19d,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x5,0x64);
  pass1_1030_3af6((u32 *)&u16_1050_6884,0x158,0x19e,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0xffec,0x96);
  pass1_1030_3af6((u32 *)&u16_1050_688e,0x159,0x19f,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_8),0x0,0x0,0x113);
  puVar2 = pass1_1030_3af6((u32 *)&u16_1050_6898,0x15a,0x1a0,(u32 *)puVar1,(u16)((u32)puVar1 >> 0x10));
  return (u16)puVar2;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void struct_1030_44be(astruct_57 *param_1,astruct_138 *param_2,u16 param_3,u16 param_4,u16 param_5,
                     u16 param_6,u16 param_7)

{
  astruct_138 *iVar1;
  u16 uVar1;
  u32 *puVar2;
  u16 uStack18;

  uVar1 = (u16)((u32)param_2 >> 0x10);
  iVar1 = (astruct_138 *)param_2;
  *(u32 *)param_2 = 0x0;
  iVar1->field8_0x8 = 0x0;
  iVar1->field15_0x12 = 0x0;
  iVar1->field332_0x152 = 0x0;
  iVar1->field333_0x154 = 0x0;
  iVar1->field334_0x156 = 0x0;
  iVar1->field335_0x158 = 0x0;
  iVar1->field336_0x15a = 0x0;
  iVar1->field337_0x15c = 0x0;
  iVar1->field338_0x160 = 0x0;
  iVar1->field339_0x164 = 0x0;
  iVar1->field1364_0x568 = 0x0;
  _param_7 = (u8 **)CONCAT22(uStack18,0x2);
  puVar2 = mixed_1010_20ba(param_1,_u16_1050_0ed0,_param_7,param_3,param_4,param_5,param_6);
  iVar1->field1364_0x568 = *(u32 *)((int)puVar2 + 0x64);
  return;
}



void pass1_1030_4538(u32 *param_1)

{
  u16 uVar1;

  fn_ptr_1000_17ce((char *)*param_1);
  uVar1 = (u16)((u32)param_1 >> 0x10);
  fn_ptr_1000_17ce(*(char **)((int)param_1 + 0x12));
  fn_ptr_1000_17ce(*(char **)((int)param_1 + 0x15c));
  return;
}



u32 struct_1030_4574(astruct_159 *param_1)

{
  astruct_159 *iVar1;
  u16 uVar1;

  uVar1 = (u16)((u32)param_1 >> 0x10);
  iVar1 = (astruct_159 *)param_1;
  iVar1->field12_0xc = DAT_1050_518c;
  iVar1->field13_0xe = 0x518e;
  iVar1->field14_0x10 = (int)&DAT_1050_1050;
  return (u32)param_1 & 0xffff0000 | ZEXT24(&iVar1->field12_0xc);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_4594(uchar *param_1,u16 param_2,u16 param_3,i16 param_4)

{
  u16 uVar1;
  u16 *puVar2;
  u16 uVar3;
  u16 in_register_0000000a;
  astruct_57 *paVar4;
  i16 iVar5;
  i16 iVar6;
  u16 uVar7;
  u16 *puVar8;
  u16 *puStack8;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  puVar2 = (u16 *)(u32)(param_4 - 0x1U);
  mem_op_1000_179c(0x10,paVar4);
  puStack8 = (u16 *)((u32)puVar2 & 0xffff | (long)paVar4 << 0x10);
  uVar3 = (u16)paVar4 | (u16)puVar2;
  if (uVar3 == 0x0) {
    puStack8 = NULL;
  }
  else {
    puVar8 = pass1_1008_3e38((astruct_19 *)CONCAT22((u16)paVar4,(u16)puVar2 + 0x4));
    uVar3 = (u16)((u32)puVar8 >> 0x10);
    puVar2 = puStack8;
  }
  uVar1 = SUB42(puVar2,0x0);
  iVar5 = (param_4 - 0x1U) * 0x12;
  load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),(iVar5 + 0x51b8));
  uVar7 = (u16)((u32)puStack8 >> 0x10);
  iVar6 = (int)puStack8;
  *puStack8 = uVar1;
  (iVar6 + 0x2) = uVar3;
  (iVar6 + 0xa) = (iVar5 + 0x51ba);
  pass1_1008_3e76((u16 *)((u32)puStack8 & 0xffff0000 | (u32)(iVar6 + 0x4)),(iVar5 + 0x51c0),
                  (iVar5 + 0x51be),(iVar5 + 0x51bc));
  (iVar6 + 0xc) = iVar5 + 0x51c2;
  (iVar6 + 0xe) = (int)&DAT_1050_1050;
  return;
}



void pass1_1030_4628(uchar *param_1,u16 param_2,u16 param_3,i16 param_4)

{
  u32 uVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;
  u16 in_register_0000000a;
  astruct_57 *paVar5;
  i16 iVar6;
  i16 iVar7;
  i16 *piVar8;
  u16 uVar9;
  i16 iStack24;
  i16 *piStack20;
  i16 iStack10;
  i16 *piStack8;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar2 = param_4 - 0x1;
  uVar3 = uVar2;
  mem_op_1000_179c(0x28,paVar5);
  uVar4 = (u16)paVar5;
  piStack20 = (i16 *)CONCAT22(uVar4,uVar3);
  if ((uVar4 | uVar3) == 0x0) {
    piStack8 = NULL;
  }
  else {
    pass1_1008_3e38((astruct_19 *)CONCAT22(uVar4,uVar3 + 0x6));
    piStack8 = piStack20;
  }
  uVar9 = (u16)((u32)piStack8 >> 0x10);
  iVar6 = (int)piStack8;
  *(u32 *)(iVar6 + 0x2) = 0x0;
  iVar7 = uVar2 * 0x5e;
  pass1_1008_3e76((u16 *)((u32)piStack8 & 0xffff0000 | (u32)(iVar6 + 0x6)),(iVar7 + 0x5336),
                  (iVar7 + 0x5334),(iVar7 + 0x5332));
  (iVar6 + 0xc) = (iVar7 + 0x5348);
  *piStack8 = param_4;
  *(u32 *)(iVar6 + 0xe) = *(u32 *)(iVar7 + 0x534a);
  iStack10 = 0x0;
  do {
    uVar3 = ((uVar2 * 0x2f + iStack10) * 0x2 + 0x5338);
    (iVar6 + iStack10 * 0x2 + 0x12) = uVar3;
    iStack10 += 0x1;
  } while (iStack10 < 0x8);
  uVar1 = *(u32 *)((int)&DAT_1050_5350 + uVar2 * 0x5e);
  pass1_1008_612e(uVar3,(int)uVar1,(int)((u32)uVar1 >> 0x10));
  (iVar6 + 0x22) = uVar3;
  piVar8 = (i16 *)(uVar2 * 0x5e + 0x5354);
  *(i16 **)(iVar6 + 0x24) = piVar8;
  (iVar6 + 0x26) = (int)&DAT_1050_1050;
  iVar7 = *piVar8;
  pass1_1000_4906((StructD *)CONCAT22(0x1050,piVar8),NULL,0x1e);
  iStack10 = 0x0;
LAB_1030_474c:
  if ((int)uVar3 <= iStack10) {
    return;
  }
  do {
    uVar4 = ((uVar2 * 0x5e + 0x534e) + iVar7) - 0x1;
    pass1_1008_612e(uVar4,iVar7,uVar4);
    iStack24 = 0x0;
    while( true ) {
      if (iStack10 < iStack24) {
        uVar1 = *(u32 *)(iVar6 + 0x24);
        ((int)uVar1 + iStack10 * 0x2) = uVar4;
        iStack10 += 0x1;
        goto LAB_1030_474c;
      }
      uVar1 = *(u32 *)(iVar6 + 0x24);
      if (((int)uVar1 + iStack24 * 0x2) == uVar4) break;
      iStack24 += 0x1;
    }
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_4782(u32 param_1,u16 param_2,u16 param_3,i16 param_4,i16 param_5,i16 param_6,i16 param_7)

{
  i16 iVar1;
  u16 uVar2;
  uchar **ppuVar3;
  u16 uVar4;
  u16 uVar5;
  astruct_57 *paVar6;
  u32 uVar7;
  astruct_57 *paVar8;
  StructD *pSVar9;
  i16 iVar10;
  u16 unaff_SI;
  u16 uVar11;
  u16 uVar12;
  u32 *puVar13;
  u32 uVar14;
  u16 in_stack_0000fdc8;
  u16 in_stack_0000fdca;
  u16 in_stack_0000fdcc;
  u16 in_stack_0000feec;
  u16 in_stack_0000feee;
  u16 in_stack_0000fef0;
  u16 in_stack_0000fef2;
  u16 in_stack_0000fef4;
  u16 in_stack_0000fef6;
  u16 in_stack_0000fef8;
  u16 in_stack_0000fefa;
  u8 uVar15;
  u8 uVar16;
  uchar *local_c4;
  u16 uStack194;
  u8 *local_c0;
  u16 uStack190;
  i16 iStack188;
  char *pcStack184;
  i16 iStack180;
  char *pcStack178;
  char *pcStack174;
  u16 uStack170;
  u16 uStack168;
  u16 uStack166;
  u16 uStack164;
  u16 uStack162;
  uchar **ppuStack160;
  i16 iStack158;
  i16 iStack156;
  i16 iStack154;
  u16 uStack152;
  char *pcStack150;
  u8 local_92 [0x80];
  u32 uStack18;
  u32 uStack14;
  u16 uStack10;
  u16 uStack8;
  i16 *piStack6;

  uVar15 = (u8)unaff_SI;
  uVar16 = (u8)((u16)unaff_SI >> 0x8);
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a((StructD *)param_1);
  }
  else {
    param_1 = param_1 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10;
  }
  uStack194 = (u16)param_1;
  local_c4 = PTR_LOOP_1050_5f2c;
  uVar2 = fn_ptr_op_1000_1708(0x20,0x0,0x1,(u16)PTR_LOOP_1050_5f2c,uStack194);
  uVar4 = (u16)param_1;
  pcStack184 = (char *)CONCAT22(uVar4,uVar2);
  paVar8 = (astruct_57 *)(param_1 & 0xffff0000);
  paVar6 = (astruct_57 *)((u32)paVar8 | (u32)(uVar4 | uVar2));
  if ((uVar4 | uVar2) == 0x0) {
    uVar2 = 0x0;
  }
  else {
    pass1_1030_84ae(CONCAT22(uVar4,uVar2));
    paVar8 = paVar6;
  }
  piStack6 = (i16 *)CONCAT22((int)paVar8,uVar2);
  *piStack6 = param_6;
  pass1_1008_3f62((u16 *)CONCAT22((int)paVar8,uVar2 + 0x8),
                  (u16 *)CONCAT22(0x1050,(int)&u16_1050_65e6 + param_6 * 0xa));
  if (param_5 != 0x0) {
    puVar13 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT13(uVar16,CONCAT12(uVar15,0x2f)),
                              in_stack_0000fdc8,in_stack_0000feec,in_stack_0000fef2,in_stack_0000fef6);
    uVar7 = (u32)paVar8 & 0xffff0000;
    uStack10 = SUB42(puVar13,0x0);
    uStack8 = (u16)((u32)puVar13 >> 0x10);
    uStack14 = pass1_1018_04b8((u32)puVar13);
    uVar7 = uVar7 & 0xffff0000 | uStack14 >> 0x10;
    uVar11 = (u16)uStack14;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack14);
    uStack18 = CONCAT22((int)uVar7,uVar11);
    pcStack150 = load_string_1010_847e((u32)_u16_1050_14cc,0x62d);
    uVar7 = uVar7 & 0xffff0000 | (u32)pcStack150 >> 0x10;
    uVar2 = pass1_1030_2a98(uStack18);
    ((int)piStack6 + 0x2) = uVar2;
    sys_1000_3f9c((char *)CONCAT22(0x1050,local_92),pcStack150,uVar2);
    uVar2 = str_op_1008_60e8((u16)uVar7,(char *)CONCAT22(0x1050,local_92));
    uVar11 = (u16)((u32)piStack6 >> 0x10);
    ((int)piStack6 + 0x4) = uVar2;
    ((int)piStack6 + 0x6) = (int)uVar7;
    uVar11 = FUN_1010_830a(param_6,uVar7,0x1008,_u16_1050_14cc,(param_6 * 0xa + 0x65ec));
    uVar12 = (u16)((u32)piStack6 >> 0x10);
    ((int)piStack6 + 0xe) = uVar11;
    ((int)piStack6 + 0x10) = (int)uVar7;
    uVar11 = FUN_1010_830a(uVar11,uVar7,0x1010,_u16_1050_14cc,(param_6 * 0xa + 0x65ee));
    uVar12 = (u16)((u32)piStack6 >> 0x10);
    iVar10 = (int)piStack6;
    (iVar10 + 0x12) = uVar11;
    (iVar10 + 0x14) = (int)uVar7;
    uVar14 = pass1_1008_4772(*(astruct_76 **)(iVar10 + 0xe));
    uVar4 = (u16)(uVar7 >> 0x10);
    iStack154 = (int)uVar14;
    uStack152 = (u16)(uVar14 >> 0x10);
    iStack156 = (iStack154 + 0x4) + -0x1;
    iStack158 = (iStack154 + 0x8) + -0x1;
    if (param_4 != 0x0) {
      ppuStack160 = (uchar **)((int)&PTR_LOOP_1050_000e + 0x1);
      if (uStack14 == 0x0) {
        debug_print_1008_6048(uStack152,s_get_site_data_without_planet__1050_56de);
      }
      else {
        ppuVar3 = &local_c4;
        pass1_1030_2f1a(uStack18,(u16 *)CONCAT22(0x1050,&local_c0),(u16 *)CONCAT22(0x1050,ppuVar3));
        pass1_1008_612e((u16)ppuVar3,(int)local_c4,(int)local_c0);
        ppuStack160 = ppuVar3;
      }
      iVar10 = (int)ppuStack160 * 0xa;
      uVar11 = (u16)((u32)piStack6 >> 0x10);
      ((int)piStack6 + 0x1c) = iVar10;
      paVar8 = (astruct_57 *)((long)iVar10 % 0x64 & 0xffffU | (u32)uVar4 << 0x10);
      ((int)piStack6 + 0x1c) = iVar10 / 0x64;
      puVar13 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(param_7,0x2),in_stack_0000fdcc,
                                in_stack_0000fef0,in_stack_0000fef6,in_stack_0000fefa);
      paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)puVar13 >> 0x10);
      local_c4 = (uchar *)puVar13;
      uStack194 = (u16)((u32)puVar13 >> 0x10);
      local_c0 = PTR_LOOP_1050_13ae;
      uVar2 = 0x84;
      puVar13 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)0x840009,in_stack_0000fdca,in_stack_0000feee,
                                in_stack_0000fef4,in_stack_0000fef8);
      uVar7 = (u32)paVar8 & 0xffff0000;
      uStack190 = pass1_1010_65d0((u32)puVar13,uVar2);
      iStack188 = 0x3c;
      if ((int)local_c0 < 0x3) {
        if (0x0 < (int)uStack190) {
          iStack188 = 0x5a;
        }
      }
      else if (uStack190 == 0x1) {
        iStack188 = 0x44;
      }
      else if (uStack190 == 0x2) {
        iStack188 = 0x4b;
      }
      else if (uStack190 == 0x3) {
        iStack188 = 0x53;
      }
      else if (uStack190 == 0x4) {
        iStack188 = 0x5a;
      }
      iVar10 = iStack188 * (int)ppuStack160;
      ppuStack160 = (uchar **)(iVar10 / 0x64);
      paVar8 = (astruct_57 *)((long)iVar10 % 0x64 & 0xffffU | uVar7 & 0xffff0000);
      uVar11 = (u16)((u32)piStack6 >> 0x10);
      ((int)piStack6 + 0x1a) = (int)ppuStack160;
      uStack164 = (int)ppuStack160 + ((int)piStack6 + 0x1c);
      uVar4 = uStack164 * 0x6;
      uStack162 = uStack164;
      mem_op_1000_179c(uVar4,paVar8);
      uVar5 = (u16)paVar8;
      pcStack184 = (char *)CONCAT22(uVar5,uVar4);
      pSVar9 = (StructD *)((u32)paVar8 & 0xffff0000 | (u32)(uVar5 | uVar4));
      if ((uVar5 | uVar4) == 0x0) {
        *(u32 *)((int)piStack6 + 0x16) = 0x0;
      }
      else {
        pass1_1000_5586(0x3e38,0x1008,uStack164,0x6,uVar4,uVar5);
        *(u32 *)((int)piStack6 + 0x16) = pcStack184;
      }
      uStack170 = uStack162 * 0x2;
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar9);
      }
      else {
        pSVar9 = (StructD *)((u32)pSVar9 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
      }
      uVar2 = fn_ptr_op_1000_1708(uStack170,0x0,0x1,(u16)PTR_LOOP_1050_5f2c,(u16)pSVar9);
      pcStack174 = (char *)CONCAT22((int)pSVar9,uVar2);
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar9);
        PTR_LOOP_1050_5f2e = (u8 *)pSVar9;
      }
      else {
      }
      uVar2 = fn_ptr_op_1000_1708(uStack170,0x0,0x1,(u16)PTR_LOOP_1050_5f2c,(u16)PTR_LOOP_1050_5f2e);
      pcStack178 = (char *)CONCAT22(PTR_LOOP_1050_5f2e,uVar2);
      iStack180 = 0x0;
LAB_1030_4b57:
      uVar4 = uStack162;
      if (iStack180 < (int)uStack162) {
        do {
          pass1_1008_612e(uVar4,0x0,iStack156);
          uStack166 = uVar4;
          pass1_1008_612e(uVar4,0x0,iStack158);
          param_7 = 0x0;
          while( true ) {
            iVar10 = (int)pcStack174;
            uVar11 = (u16)((u32)pcStack174 >> 0x10);
            uVar12 = (u16)((u32)pcStack178 >> 0x10);
            uStack168 = uVar4;
            if (iStack180 <= param_7) {
              iVar1 = iStack180 * 0x2;
              (iVar1 + iVar10) = uStack166;
              (iVar1 + (int)pcStack178) = uVar4;
              uVar7 = *(u32 *)((int)piStack6 + 0x16);
              pass1_1008_3e76((u16 *)(uVar7 & 0xffff0000 | (u32)(u16)((int)uVar7 + iStack180 * 0x6)),0x0,uVar4,
                              (iVar1 + iVar10));
              iStack180 += 0x1;
              goto LAB_1030_4b57;
            }
            if (((param_7 * 0x2 + iVar10) == uStack166) && ((param_7 * 0x2 + (int)pcStack178) == uVar4)
               ) break;
            param_7 += 0x1;
          }
        } while( true );
      }
      fn_ptr_1000_17ce(pcStack174);
      pcStack184 = pcStack178;
      fn_ptr_1000_17ce(pcStack178);
    }
  }
  return;
}



void pass1_1030_4bbe(u16 param_1,astruct_117 *param_2,i16 param_3)

{
  u32 *puVar1;
  u32 *puVar2;
  u16 uVar3;
  i16 iVar4;
  astruct_117 *pstruct117_5;
  u32 *puVar5;
  u32 *puVar6;
  astruct_117 *uVar7;

  uVar7 = (astruct_117 *)((u32)param_2 >> 0x10);
  pstruct117_5 = (astruct_117 *)param_2;
  if (pstruct117_5->field15_0x12 == 0x0) {
    pass1_1030_4f5a(param_1,(astruct_117 *)((u32)param_2 & 0xffff | ZEXT24(uVar7) << 0x10));
  }
  puVar6 = &pstruct117_5->field16_0x16;
  uVar3 = ((int)&pstruct117_5->field15_0x12 + 0x2);
  puVar5 = (u32 *)(&pstruct117_5->field15_0x12 + param_3 * 0x98);
  for (iVar4 = 0x26; iVar4 != 0x0; iVar4 += -0x1) {
    puVar2 = puVar6;
    puVar6 = puVar6 + 0x1;
    puVar1 = puVar5;
    puVar5 = puVar5 + 0x1;
    *puVar2 = *puVar1;
  }
  return;
}



void pass1_1030_4c06(astruct_117 *param_1,i16 param_2,u16_t param_3)

{
  i32 *plVar1;
  i32 *plVar2;
  u16 uVar3;
  i16 iVar4;
  astruct_117 *iVar5;
  i32 *plVar5;
  i32 *plVar6;
  astruct_117 *uVar7;

  uVar7 = (astruct_117 *)((u32)param_1 >> 0x10);
  iVar5 = (astruct_117 *)param_1;
  if (iVar5[0xd].field10_0xa == 0x0) {
    pass1_1030_5044(param_3,(astruct_117 *)((u32)param_1 & 0xffff | ZEXT24(uVar7) << 0x10));
  }
  plVar6 = &iVar5[0x6].field15_0x12;
  uVar3 = ((int)&iVar5[0xd].field10_0xa + 0x2);
  plVar5 = (i32 *)(&iVar5[0xd].field10_0xa + param_2 * 0xae);
  for (iVar4 = 0x2b; iVar4 != 0x0; iVar4 += -0x1) {
    plVar2 = plVar6;
    plVar6 = plVar6 + 0x1;
    plVar1 = plVar5;
    plVar5 = plVar5 + 0x1;
    *plVar2 = *plVar1;
  }
  plVar6 = plVar5;
  return;
}



void pass1_1030_4c52(u16 param_1,u16 param_2,u32 param_3,u32 param_4,u16 param_5)

{
  u16 uVar1;
  i16 iVar2;
  i16 iVar3;
  u16 uVar4;
  char *pcStack8;
  i16 iStack4;

  iStack4 = 0x0;
  while( true ) {
    uVar1 = pass1_1000_47a4(param_4,(u32)s___1050_518a);
    pcStack8 = (char *)CONCAT22(param_5,uVar1);
    if ((param_5 | uVar1) == 0x0) break;
    if (*pcStack8 != '\"') {
      iVar2 = pass1_1000_3e2c(CONCAT22(param_5,uVar1));
      iVar3 = (int)param_3;
      uVar4 = (u16)(param_3 >> 0x10);
      if (iStack4 < 0x25) {
        (iStack4 * 0x4 + iVar3) = iVar2;
        (iStack4 * 0x4 + iVar3 + 0x2) = param_5;
      }
      else if (iStack4 == 0x25) {
        (iVar3 + 0x94) = iVar2;
      }
      else if (iStack4 == 0x26) {
        (iVar3 + 0x96) = iVar2;
      }
      else if (iStack4 == 0x27) {
        (iVar3 + 0x98) = iVar2;
      }
      else if (iStack4 == 0x28) {
        (iVar3 + 0x9a) = iVar2;
      }
      else if (iStack4 == 0x29) {
        (iVar3 + 0x9c) = iVar2;
      }
      else if (iStack4 == 0x2a) {
        (iVar3 + 0x9e) = iVar2;
      }
      else if (iStack4 == 0x2b) {
        (iVar3 + 0xa0) = iVar2;
      }
      else if (iStack4 == 0x2c) {
        (iVar3 + 0xa2) = iVar2;
      }
      iStack4 += 0x1;
    }
    param_4 = 0x0;
  }
  return;
}



void pass1_1030_4d3a(u16 param_1,astruct_117 *param_2,astruct_118 *param_3,u32 param_4)

{
  u16 uVar1;
  i16 iVar2;
  astruct_118 *pstruct118_3;
  astruct_118 *pstruct118_4;
  char *pcStack8;
  i16 iStack4;

  iStack4 = 0x0;
  while( true ) {
    uVar1 = pass1_1000_47a4(param_4,(u32)s___1050_518a);
    pcStack8 = (char *)CONCAT22(param_1,uVar1);
    if ((param_1 | uVar1) == 0x0) break;
    if (*pcStack8 != '\"') {
      iVar2 = pass1_1000_3e2c(CONCAT22(param_1,uVar1));
      pstruct118_3 = (astruct_118 *)param_3;
      pstruct118_4 = (astruct_118 *)((u32)param_3 >> 0x10);
      if (iStack4 < 0x25) {
        (&pstruct118_3->field_0x0 + iStack4 * 0x4) = iVar2;
        (&pstruct118_3->field_0x2 + iStack4 * 0x4) = param_1;
      }
      else if (iStack4 == 0x25) {
        pstruct118_3->field148_0x94 = iVar2;
      }
      else if (iStack4 == 0x26) {
        pstruct118_3->field149_0x96 = iVar2;
      }
      iStack4 += 0x1;
    }
    param_4 = 0x0;
  }
  return;
}



void pass1_1030_4dbc(astruct_117 *param_1,u32 param_2,i32 param_3)

{
  i32 *plVar1;
  i16 *piVar2;
  i32 lVar3;
  u16 uVar4;
  astruct_117 *iVar5;
  astruct_117 *uVar5;

  iVar5 = (astruct_117 *)param_1;
  uVar5 = (astruct_117 *)((u32)param_1 >> 0x10);
  if (0x0 < param_3) {
    *(u32 *)&iVar5[0xd].field_0xe = param_2;
    iVar5[0xd].field15_0x12 = param_3;
  }
  if ((*(i32 *)&iVar5[0xd].field_0xe == 0x0) ||
     (lVar3 = iVar5[0xd].field15_0x12, plVar1 = &iVar5[0xd].field15_0x12, *plVar1 = *plVar1 + -0x1, lVar3 == 0x0)) {
    *(u32 *)&iVar5[0xd].field_0xe = 0x0;
  }
  else {
    uVar4 = str_op_1000_3da4(*(char **)&iVar5[0xd].field_0xe);
    piVar2 = (i16 *)&iVar5[0xd].field_0xe;
    *piVar2 = *piVar2 + uVar4 + 0x2;
  }
  return;
}



void pass1_1030_4e34(astruct_117 *param_1,astruct_117 *param_2,i32 param_3,char *param_4)

{
  while (param_3 != 0x0) {
    if ((*param_4 == '\r') || (*param_4 == '\n')) {
      *param_4 = '\0';
    }
    param_4 = (char *)((u32)param_4 & 0xffff0000 | (u32)((int)param_4 + 0x1));
    param_3 = param_3 + -0x1;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 read_file_1030_4e70(astruct_117 *param_1,u32 *param_2,u8 **param_3,i32 param_4,u16 param_5)

{
  u16 uVar1;
  HFILE16 h_file;
  u16 unaff_SS;
  char *path;
  i32 lVar1;
  u16 uVar2;
  u8 *pbStack60;
  i32 iStack56;
  u32 uStack20;

  *param_3 = NULL;
  *param_2 = 0x0;
  if (param_4 != 0x0) {
    uVar2 = 0x0;
    path = pass1_1030_5164(param_1,param_4);
    param_5 = (u16)((u32)path >> 0x10);
    uVar1 = dos3_call_1000_51aa((u16)path,param_5,uVar2);
    if (uVar1 == 0x0) {
      *param_2 = uStack20;
      h_file = _lopen16(0x0,path);
      if (h_file != 0xffff) {
        lVar1 = mem_op_1000_0a48(0x1,(u16)*param_2,(int)(*param_2 >> 0x10),_PTR_LOOP_1050_5f2c);
        lVar1 = (u16)((u32)lVar1 >> 0x10);
        param_3 = (int)lVar1;
        ((int)param_3 + 0x2) = lVar1;
        param_5 = lVar1;
        if ((lVar1 | param_3) != 0x0) {
          iStack56 = WIN16_hread(*param_2,*param_3,h_file);
          uVar2 = (u16)((u32)iStack56 >> 0x10);
          _lclose16(h_file);
          pbStack60 = *param_3;
          while (iStack56 != 0x0) {
            if ((*(u8 *)(*pbStack60 + 0x608b) & 0x20) == 0x0) {
              *pbStack60 = *pbStack60 + 0x80;
            }
            pbStack60 = (u8 *)((u32)pbStack60 & 0xffff0000 | (u32)((int)pbStack60 + 0x1));
            iStack56 = iStack56 + -0x1;
          }
          return uVar2;
        }
      }
    }
  }
  return param_5;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_4f5a(u16 param_1,astruct_117 *param_2)

{
  char *pcVar1;
  i32 *plVar2;
  u16 uVar3;
  astruct_118 *iVar4;
  char *pcVar4;
  u16 uVar5;
  u16 uVar6;
  u16 extraout_var;
  astruct_117 *paVar8;
  astruct_117 *paVar9;
  u16 uStack22;
  u32 uStack20;
  u16 uStack14;
  u16 uStack12;
  i32 local_a;
  char *local_6;
  astruct_118 *uVar1;
  astruct_117 *pstruct117_7;
  astruct_117 *pstruct117_8;
  StructD *pSVar7;

  plVar2 = &local_a;
  uVar5 = read_file_1030_4e70(param_2,(u32 *)CONCAT22(0x1050,plVar2),(u8 **)CONCAT22(0x1050,&local_6),
                              (i32)s_bldgbld_dat_1050_56fc,param_1);
  pcVar1 = local_6;
  pSVar7 = (StructD *)CONCAT22(extraout_var,uVar5);
  if (plVar2 != NULL) {
    paVar8 = (astruct_117 *)param_2;
    paVar9 = (astruct_117 *)((u32)param_2 >> 0x10);
    pcVar4 = local_6;
    pass1_1030_4e34(paVar8,paVar9,local_a,local_6);
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar7);
      PTR_LOOP_1050_5f2e = (u8 *)pSVar7;
    }
    else {
    }
    uVar3 = fn_ptr_op_1000_1708((u16)pcVar4 * 0x98,0x0,0x1,(u16)PTR_LOOP_1050_5f2c,(u16)PTR_LOOP_1050_5f2e);
    &paVar8->field15_0x12 = uVar3;
    ((int)&paVar8->field15_0x12 + 0x2) = PTR_LOOP_1050_5f2e;
    pass1_1030_4dbc(param_2,(u32)local_6,(u32)pcVar4 & 0xffff);
    uStack20 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
    for (uStack22 = 0x0; uStack22 < (u16)pcVar4; uStack22 += 0x1) {
      uVar6 = ((int)&paVar8->field15_0x12 + 0x2);
      iVar4 = (astruct_118 *)(&paVar8->field15_0x12 + uStack22 * 0x98);
      pass1_1030_4d3a(uVar6,(astruct_117 *)((u32)param_2 & 0xffff | ZEXT24(paVar9) << 0x10),
                      (astruct_118 *)CONCAT22(uVar6,iVar4),uStack20);
      pass1_1030_4dbc(param_2,0x0,0x0);
      uStack20 = CONCAT22(uVar6,iVar4);
    }
    uStack12 = (u16)((u32)pcVar1 >> 0x10);
    uStack14 = (u16)pcVar1;
    if ((uStack12 | uStack14) != 0x0) {
      call_fn_ptr_1000_0dc6(pcVar1);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_5044(u16_t param_1,astruct_117 *param_2)

{
  char *pcVar1;
  i32 *plVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  i16 iVar6;
  char *pcVar7;
  u16 uVar9;
  u16 uVar10;
  u16 extraout_var;
  u32 uStack28;
  u16 uStack24;
  u32 uStack22;
  u16 uStack14;
  u16 uStack12;
  i32 local_a;
  char *local_6;
  u32 uVar8;
  StructD *pSVar11;
  astruct_117 *pstruct117_10;
  astruct_117 *pstruct117_10_hi;

  plVar2 = &local_a;
  uVar9 = read_file_1030_4e70(param_2,(u32 *)CONCAT22(0x1050,plVar2),(u8 **)CONCAT22(0x1050,&local_6),
                              (i32)s_bldgops_dat_1050_5708,param_1);
  pcVar1 = local_6;
  pSVar11 = (StructD *)CONCAT22(extraout_var,uVar9);
  if (plVar2 != NULL) {
    pstruct117_10 = (astruct_117 *)param_2;
    pstruct117_10_hi = (astruct_117 *)((u32)param_2 >> 0x10);
    pcVar7 = local_6;
    pass1_1030_4e34(pstruct117_10,pstruct117_10_hi,local_a,local_6);
    uVar3 = (u16)pcVar7;
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar11);
      PTR_LOOP_1050_5f2e = (u8 *)pSVar11;
    }
    else {
    }
    uVar4 = fn_ptr_op_1000_1708(uVar3 * 0xae,0x0,0x1,(u16)PTR_LOOP_1050_5f2c,(u16)PTR_LOOP_1050_5f2e);
    uVar8 = (u32)uVar4;
    uStack28 = CONCAT22(PTR_LOOP_1050_5f2e,uVar4);
    uVar10 = (u16)PTR_LOOP_1050_5f2e | uVar4;
    if (uVar10 == 0x0) {
      pstruct117_10[0xd].field10_0xa = 0x0;
    }
    else {
      pass1_1000_5586(0x51f0,0x1030,uVar3,0xae,uVar4,(u16)PTR_LOOP_1050_5f2e);
      pstruct117_10[0xd].field10_0xa = uStack28;
      uVar8 = uStack28;
    }
    uVar5 = (u16)uVar8;
    pass1_1030_4dbc(param_2,(u32)local_6,(u32)pcVar7 & 0xffff);
    uStack22 = CONCAT22(uVar10,uVar5);
    for (uStack24 = 0x0; uStack24 < uVar3; uStack24 += 0x1) {
      uVar10 = ((int)&pstruct117_10[0xd].field10_0xa + 0x2);
      iVar6 = &pstruct117_10[0xd].field10_0xa + uStack24 * 0xae;
      pass1_1030_4c52((u16)pstruct117_10,(u16)pstruct117_10_hi,CONCAT22(uVar10,iVar6),uStack22,uVar10);
      pass1_1030_4dbc(param_2,0x0,0x0);
      uStack22 = CONCAT22(uVar10,iVar6);
    }
    uStack12 = (u16)((u32)pcVar1 >> 0x10);
    uStack14 = (u16)pcVar1;
    if ((uStack12 | uStack14) != 0x0) {
      call_fn_ptr_1000_0dc6(pcVar1);
    }
  }
  return;
}



char * pass1_1030_5164(astruct_117 *param_1,u32 param_2)

{
  u16 uVar1;
  u16 uVar2;
  u16 uVar3;
  i32 lVar4;
  u8 local_a [0x8];

  uVar3 = (u16)((u32)param_1 >> 0x10);
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),*(u32 *)((int)param_1 + 0x568));
  do {
    lVar4 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    if (lVar4 == 0x0) {
      return (char *)param_2;
    }
    uVar1 = (int)param_1 + 0x168;
    unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | (u32)uVar1),*(char **)((int)lVar4 + 0x4));
    pass1_1000_3cea((u32)param_1 & 0xffff0000 | (u32)uVar1,(char *)param_2);
    uVar2 = dos3_call_1000_51aa(uVar1,uVar3,0x1);
  } while (uVar2 != 0x0);
  return (char *)((u32)param_1 & 0xffff0000 | (u32)uVar1);
}



void pass1_1030_51eb(void)

{
  pass1_1030_3b28();
  return;
}



u32 pass1_1030_51f0(u32 param_1)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = (u16)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  (iVar1 + 0xa4) = 0x0;
  (iVar1 + 0xa6) = 0x0;
  (iVar1 + 0xa8) = 0x0;
  (iVar1 + 0xaa) = 0x0;
  (iVar1 + 0xac) = 0x0;
  return param_1;
}



void pass1_1030_521c(astruct_97 *param_1,u32 param_2)

{
  astruct_97 *iVar1;
  u16 uVar1;

  struct_op_1028_d1dc(param_1,0x32c7);
  uVar1 = (u16)((u32)param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  *(u32 *)&iVar1->field259_0x108 = param_2;
  param_1->offset_0x0 = 0x55fe;
  iVar1->segment_0x2 = 0x1030;
  sys_1000_3f9c((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),s_SCGenKids_0x_08lx_1050_5714,
                (u16)param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1030_5260(u16 param_1,u16 param_2,u32 param_3)

{
  code **ppcVar1;
  u32 *puStack6;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,*(u32 *)((int)param_3 + 0x108));
  puStack6 = (u32 *)CONCAT22(param_2,param_1);
  ppcVar1 = (code **)((int)*puStack6 + 0x14);
  (**ppcVar1)();
  return 0x1;
}



void pass1_1030_5290(astruct_376 *param_1,uchar *param_2,astruct_377 *param_3)

{
  u32 *puVar1;
  u32 *puVar2;
  i16 iVar3;
  u16 uVar4;
  u16 in_register_0000000a;
  astruct_57 *paVar5;
  astruct_377 *iVar5;
  u32 *puVar6;
  u32 *puVar7;
  astruct_377 *uVar8;
  u16 *puStack10;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x10c,paVar5);
  uVar4 = (u16)paVar5;
  puStack10 = (u16 *)CONCAT22(uVar4,param_1);
  if ((uVar4 | (u16)param_1) != 0x0) {
    *puStack10 = 0x389a;
    param_1->field2_0x2 = 0x1008;
    uVar8 = (astruct_377 *)((u32)param_3 >> 0x10);
    iVar5 = (astruct_377 *)param_3;
    param_1->field3_0x4 = iVar5->field4_0x4;
    puVar6 = &iVar5->field5_0x8;
    puVar7 = &param_1->field4_0x8;
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar6;
      puVar6 = puVar6 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1->field2_0x2 = 0x1028;
    param_1->field257_0x108 = iVar5->field258_0x108;
    *puStack10 = 0x55fe;
    param_1->field2_0x2 = 0x1030;
  }
  return;
}



void pass1_1030_532e(astruct_97 *param_1,u32 param_2)

{
  astruct_97 *iVar1;
  astruct_97 *uVar1;

  struct_op_1028_d1dc(param_1,0x32c7);
  uVar1 = (astruct_97 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  *(u32 *)&iVar1->field259_0x108 = param_2;
  param_1->offset_0x0 = 0x55ee;
  iVar1->segment_0x2 = 0x1030;
  sys_1000_3f9c((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),s_SCSelect__u___d_1050_5726,
                (u16)*(u32 *)&iVar1->field_0x4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1030_538a(astruct_694 *param_1)

{
  u16 uVar1;
  u16 uVar2;
  u32 in_EDX;
  astruct_694 *iVar4;
  u16 uVar3;
  u32 *puVar4;
  u16 in_stack_0000fe98;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000fff0;

  uVar3 = (u16)((u32)param_1 >> 0x10);
  iVar4 = (astruct_694 *)param_1;
  uVar1 = ((int)&iVar4->field264_0x108 + 0x2);
  uVar2 = uVar1 >> 0x8;
  puVar4 = mixed_1010_20ba((astruct_57 *)(in_EDX & 0xffff0000 | (u32)uVar1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000fff0,0x2f),in_stack_0000fe98,in_stack_0000ffbc,
                           in_stack_0000ffc2,in_stack_0000ffc6);
  if (uVar2 == 0x1) {
    pass1_1018_04ca((u32)puVar4,iVar4->field264_0x108);
  }
  else if (uVar2 == 0x2) {
    pass1_1018_04a4((u32)puVar4,iVar4->field264_0x108);
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_53f4(u16 param_1,u32 param_2)

{
  u32 uVar1;
  u16 uVar2;
  u16 uVar3;
  u16 in_register_0000000a;
  astruct_57 *paVar4;
  i16 iVar5;
  u16 uVar6;
  u32 uVar7;
  u8 bStack291;
  u8 local_11e [0x10e];
  u32 uStack16;
  u32 uStack12;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar6 = (u16)(param_2 >> 0x10);
  iVar5 = (int)param_2;
  uStack12 = *(u32 *)(iVar5 + 0x108);
  uStack12._3_1_ = (char)((u32)uStack12 >> 0x18);
  if (uStack12._3_1_ == -0x1) {
    uVar7 = pass1_1028_e2e0(paVar4,_PTR_LOOP_1050_65e2,(u8)((u32)*(u32 *)(iVar5 + 0x108) >> 0x18));
    uVar3 = (u16)(uVar7 >> 0x10);
  }
  else {
    uStack16 = (u32 *)*(u32 *)(iVar5 + 0x108);
    uStack16._3_1_ = (char)((u32)uStack16 >> 0x18);
    if (uStack16._3_1_ == '\x03') {
      pass1_1028_e44a(_PTR_LOOP_1050_65e2,*(i32 *)(iVar5 + 0x108));
      uVar3 = SUB42(paVar4,0x0);
    }
    else {
      uVar1 = *(u32 *)(iVar5 + 0x108);
      pass1_1028_e372(_PTR_LOOP_1050_65e2,(u16)uVar1,(u16)((u32)uVar1 >> 0x10));
      uVar3 = SUB42(paVar4,0x0);
    }
  }
  uStack12 = *(u32 *)(iVar5 + 0x108);
  uStack12._3_1_ = (char)((u32)uStack12 >> 0x18);
  if (uStack12._3_1_ != '\x03') {
    pass1_1030_521c((astruct_97 *)CONCAT13(0x10,CONCAT12(0x50,local_11e)),*(u32 *)(iVar5 + 0x108));
    uStack16 = *_u16_1050_5748;
    fn_ptr_1028_d566(uStack16,(astruct_97 *)CONCAT22(0x1050,local_11e));
    bStack291 = (u8)((u32)*(u32 *)(iVar5 + 0x108) >> 0x18);
    uVar2 = (u16)bStack291;
    if (bStack291 == 0x2) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,*(u32 *)(iVar5 + 0x108));
      pass1_1010_82f8(_u16_1050_14cc,*(u16*)(uVar2 + 0x10));
    }
  }
  return;
}



void pass1_1030_54f8(astruct_378 *param_1,uchar *param_2,astruct_379 *param_3)

{
  u32 *puVar1;
  u32 *puVar2;
  i16 iVar3;
  u16 uVar4;
  u16 in_register_0000000a;
  astruct_57 *paVar5;
  astruct_379 *iVar5;
  u32 *puVar6;
  u32 *puVar7;
  astruct_379 *uVar8;
  u16 *puStack10;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x10c,paVar5);
  uVar4 = (u16)paVar5;
  puStack10 = (u16 *)CONCAT22(uVar4,param_1);
  if ((uVar4 | (u16)param_1) != 0x0) {
    *puStack10 = 0x389a;
    param_1->field2_0x2 = 0x1008;
    uVar8 = (astruct_379 *)((u32)param_3 >> 0x10);
    iVar5 = (astruct_379 *)param_3;
    param_1->field3_0x4 = iVar5->field4_0x4;
    puVar6 = &iVar5->field5_0x8;
    puVar7 = &param_1->field4_0x8;
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar6;
      puVar6 = puVar6 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1->field2_0x2 = 0x1028;
    param_1->field257_0x108 = iVar5->field258_0x108;
    *puStack10 = 0x55ee;
    param_1->field2_0x2 = 0x1030;
  }
  return;
}



StructD * pass1_1030_5596(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * pass1_1030_55c2(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1030_560e(astruct_180 *param_1)

{
  u32 in_EDX;
  astruct_180 *iVar1;
  astruct_180 *uVar2;

  struct_1030_17ce(param_1,0x64,0x1f4,in_EDX);
  uVar2 = (astruct_180 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_180 *)param_1;
  *(u32 *)&iVar1->field12_0x10 = 0x0;
  pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1->field14_0x14)));
  iVar1->field17_0x1a = 0x0;
  &iVar1->field18_0x1c = 0x0;
  param_1->field0_0x0 = (u16)s_procLo_1050_5bd0;
  iVar1->field1_0x2 = 0x1030;
  return &param_1->field0_0x0;
}



u16 * struct_1030_565a(u16 param_1,astruct_57 *param_2,astruct_352 *param_3,u32 param_4)

{
  astruct_353 *pstruct_1;
  astruct_352 *pstruct_1_hi;

  pass1_1030_183c(param_1,param_2,&param_3->u16_field_0x0,0x64,0x1f4,0x3000000,param_4);
  pstruct_1_hi = (astruct_352 *)((u32)param_3 >> 0x10);
  pstruct_1 = (astruct_353 *)param_3;
  pstruct_1->field15_0x10 = 0x0;
  pass1_1008_3e38((astruct_19 *)((u32)param_3 & 0xffff0000 | ZEXT24(&pstruct_1->field16_0x14)));
  pstruct_1->field21_0x1a = 0x0;
  pstruct_1->field22_0x1c = 0x0;
    // 0x5bd0
  param_3->u16_field_0x0 = (u16)s_procLo_1050_5bd0;
  pstruct_1->field2_0x2 = 0x1030;
  return &param_3->u16_field_0x0;
}



void pass1_1030_56b0(u16 *param_1)

{
  u16 uVar1;
  char *pcVar2;
  i16 iVar3;
  u16 uVar4;

  uVar4 = (u16)((u32)param_1 >> 0x10);
  iVar3 = (int)param_1;
  *param_1 = (u16)s_procLo_1050_5bd0;
  (iVar3 + 0x2) = 0x1030;
  pcVar2 = *(char **)(iVar3 + 0x10);
  uVar1 = (iVar3 + 0x12);
  if ((uVar1 | (u16)pcVar2) != 0x0) {
    fn_ptr_1030_84d0((u32)pcVar2 & 0xffff | (u32)uVar1 << 0x10);
    fn_ptr_1000_17ce(pcVar2);
  }
  pass1_1030_18b2(param_1);
  return;
}



void pass1_1030_56f6(u16_t param_1,astruct_731 *param_2,u32 param_3)

{
  i16 *piVar1;
  u32 uVar2;
  u32 uVar3;
  u16 uVar4;
  BOOL16 BVar5;
  i16 iVar6;
  astruct_731 *iVar7;
  astruct_731 *uVar8;
  HFILE16 in_stack_0000ffd6;
  u16 local_e [0x3];
  u16 local_8 [0x2];
  i16 iStack4;

  uVar4 = pass1_1030_1978(param_1,param_2,param_3);
  if (uVar4 != 0x0) {
    uVar8 = (astruct_731 *)((u32)param_2 >> 0x10);
    iVar7 = (astruct_731 *)param_2;
    local_e[0] = *(u16*)&iVar7->field_0x10;
    BVar5 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_e),(char *)0x2,in_stack_0000ffd6);
    if (BVar5 != 0x0) {
      uVar3 = *(u32 *)&iVar7->field_0x10;
      local_8[0] = ((int)uVar3 + 0x2);
      BVar5 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_8),(char *)0x2,in_stack_0000ffd6);
      if ((BVar5 != 0x0) &&
         (uVar3 = *(u32 *)&iVar7->field_0x10, BVar5 = pass1_1008_7c2a(param_3,*(char **)((int)uVar3 + 0x4)),
         BVar5 != 0x0)) {
        uVar3 = *(u32 *)&iVar7->field_0x10;
        local_8[0] = ((int)uVar3 + 0x1a);
        BVar5 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_8),(char *)0x2,in_stack_0000ffd6);
        if (BVar5 != 0x0) {
          for (iStack4 = 0x0; uVar3 = *(u32 *)&iVar7->field_0x10, piVar1 = (i16 *)((int)uVar3 + 0x1a),
              *piVar1 != iStack4 && iStack4 <= *piVar1; iStack4 += 0x1) {
            uVar3 = *(u32 *)&iVar7->field_0x10;
            uVar2 = *(u32 *)((int)uVar3 + 0x16);
            iVar6 = write_to_file_1008_7b4c
                              (param_3,(astruct_615 *)(uVar2 & 0xffff0000 | (u32)(u16)((int)uVar2 + iStack4 * 0x6)));
            if (iVar6 == 0x0) goto LAB_1030_5734;
          }
          iVar6 = write_to_file_1008_7b4c
                            (param_3,(astruct_615 *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar7->field19_0x14)));
          if (iVar6 != 0x0) {
            local_8[0] = &iVar7->field_0x1c;
            BVar5 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_8),(char *)0x2,in_stack_0000ffd6);
            if (BVar5 != 0x0) {
              return;
            }
          }
        }
      }
    }
LAB_1030_5734:
    u16_1050_0310 = 0x6d0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void file_1030_581e(i16 param_1,uchar *param_2,astruct_381 *param_3,u32 param_4)

{
  i16 *piVar1;
  i16 iVar2;
  u32 uVar3;
  astruct_380 *paVar4;
  BOOL16 BVar5;
  u8 *puVar6;
  u16 uVar7;
  u32 uVar8;
  u16 uVar9;
  u16 uVar10;
  u16 in_register_0000000a;
  StructD *pSVar11;
  astruct_57 *paVar13;
  astruct_380 *iVar9;
  u16 uVar14;
  u16 in_stack_0000fae2;
  u32 uStack1040;
  i16 iStack1036;
  u8 local_408 [0x400];
  u32 uStack8;
  i16 local_4;
  astruct_381 *uVar15;
  astruct_381 *iVar12;
  astruct_57 *paVar12;

  pSVar11 = (StructD *)CONCAT22(in_register_0000000a,param_2);
  iVar12 = (astruct_381 *)param_3;
  uVar15 = (astruct_381 *)((u32)param_3 >> 0x10);
  file_1030_19b4(param_1,param_2,(astruct_373 *)param_3,(HFILE16 *)param_4);
  if (param_1 != 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar11);
    }
    else {
      pSVar11 = (StructD *)((u32)pSVar11 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    paVar4 = (astruct_380 *)fn_ptr_op_1000_1708(0x20,0x0,0x1,(u16)PTR_LOOP_1050_5f2c,(u16)pSVar11);
    uVar9 = (u16)pSVar11 | (u16)paVar4;
    paVar13 = (astruct_57 *)((u32)pSVar11 & 0xffff0000);
    paVar12 = (astruct_57 *)((u32)paVar13 | (u32)uVar9);
    if (uVar9 == 0x0) {
      paVar4 = NULL;
    }
    else {
      pass1_1030_84ae(CONCAT22((u16)pSVar11,paVar4));
      paVar13 = paVar12;
    }
    iVar12->field16_0x10 = paVar4;
    iVar12->field17_0x12 = (uchar *)paVar13;
    BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22(0x1050,&local_4),0x2);
    if (BVar5 != 0x0) {
      uVar8 = *(u32 *)((int)_PTR_LOOP_1050_65e2 + 0x52);
      uStack8 = uVar8;
      pass1_1030_4782((u32)paVar13,(u16)uVar8,(u16)((u32)uVar8 >> 0x10),0x0,0x1,local_4,in_stack_0000fae2);
      iVar12->field16_0x10 = (astruct_380 *)uVar8;
      iVar12->field17_0x12 = (uchar *)paVar13;
      BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)CONCAT22((uchar *)paVar13,&iVar12->field16_0x10->field_0x2),
                                  0x2);
      if (BVar5 != 0x0) {
        puVar6 = local_408;
        read_file_1008_7c6e((HFILE16)param_4,(u16)(param_4 >> 0x10),(char *)CONCAT22(0x1050,puVar6));
        if (puVar6 != NULL) {
          uVar8 = *(u32 *)&iVar12->field16_0x10;
          fn_ptr_1000_17ce(*(char **)((int)uVar8 + 0x4));
          uVar7 = str_op_1008_60e8((u16)paVar13,(char *)CONCAT22(0x1050,local_408));
          uVar8 = *(u32 *)&iVar12->field16_0x10;
          uVar14 = (u16)((u32)uVar8 >> 0x10);
          iVar9 = (astruct_380 *)uVar8;
          iVar9->field4_0x4 = uVar7;
          iVar9->field5_0x6 = (uchar *)paVar13;
          uVar3 = *(u32 *)&iVar12->field16_0x10;
          BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(uVar3 & 0xffff0000 | (u32)((int)uVar3 + 0x1a)),0x2);
          if (BVar5 != 0x0) {
            uVar8 = *(u32 *)&iVar12->field16_0x10;
            iVar2 = ((int)uVar8 + 0x1a);
            uVar9 = iVar2 * 0x6;
            mem_op_1000_179c(uVar9,paVar13);
            uVar10 = (u16)paVar13;
            uStack1040 = CONCAT22(uVar10,uVar9);
            if ((uVar10 | uVar9) == 0x0) {
              uVar8 = *(u32 *)&iVar12->field16_0x10;
              *(u32 *)((int)uVar8 + 0x16) = 0x0;
            }
            else {
              pass1_1000_5586(0x3e38,0x1008,iVar2,0x6,uVar9,uVar10);
              uVar8 = *(u32 *)&iVar12->field16_0x10;
              *(u32 *)((int)uVar8 + 0x16) = uStack1040;
            }
            for (iStack1036 = 0x0; uVar8 = *(u32 *)&iVar12->field16_0x10, piVar1 = (i16 *)((int)uVar8 + 0x1a),
                *piVar1 != iStack1036 && iStack1036 <= *piVar1; iStack1036 += 0x1) {
              uVar8 = *(u32 *)&iVar12->field16_0x10;
              uVar3 = *(u32 *)((int)uVar8 + 0x16);
              BVar5 = read_file_1008_7bc8(param_4,(u16 *)
                                                  (uVar3 & 0xffff0000 | (u32)(u16)((int)uVar3 + iStack1036 * 0x6)));
              if (BVar5 == 0x0) goto LAB_1030_58a7;
            }
            BVar5 = read_file_1008_7bc8(param_4,(u16 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar12->field_0x14)));
            if ((BVar5 != 0x0) &&
               (BVar5 = read_file_1008_7dee((HFILE16 *)param_4,
                                            (u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar12->field_0x1c)),0x2),
               BVar5 != 0x0)) {
              return;
            }
          }
        }
      }
    }
LAB_1030_58a7:
    u16_1050_0310 = 0x6d2;
  }
  return;
}



void pass1_1030_5a52(u32 param_1,u32 *param_2,u32 *param_3)

{
  u32 uVar1;
  u16 uVar2;

  uVar2 = (u16)(param_1 >> 0x10);
  uVar1 = *(u32 *)((int)param_1 + 0x10);
  *param_3 = *(u32 *)((int)uVar1 + 0xe);
  uVar1 = *(u32 *)((int)param_1 + 0x10);
  *param_2 = *(u32 *)((int)uVar1 + 0x12);
  return;
}



void pass1_1030_5a80(u32 param_1,u32 param_2)

{
  u32 *puVar1;
  u16 uVar2;
  u32 uVar3;
  u8 local_20 [0xc];
  u32 local_14;
  u32 uStack14;
  u32 uStack10;
  i16 iStack6;
  u16 uStack4;

  uVar2 = (u16)(param_1 >> 0x10);
  *(u32 *)((int)param_1 + 0x10) = param_2;
  uVar3 = pass1_1008_4772(*(astruct_76 **)((int)param_2 + 0xe));
  uStack4 = (u16)(uVar3 >> 0x10);
  iStack6 = (int)uVar3;
  uStack10 = *(u32 *)(iStack6 + 0x4);
  uStack14 = *(u32 *)(iStack6 + 0x8);
  pass1_1008_3e54((u16 *)CONCAT22(0x1050,&local_14),0x0,(int)uStack14 - 0x1,(int)uStack10 - 0x1);
  puVar1 = (u32 *)((int)param_1 + 0x14);
  pass1_1008_6cb4((astruct_362 *)CONCAT22(0x1050,local_20),&local_14,(u16)&DAT_1050_1050,puVar1,uVar2);
  pass1_1008_6d64((u16 *)CONCAT22(0x1050,local_20),(u16 *)(param_1 & 0xffff0000 | ZEXT24(puVar1)));
  return;
}



i16 pass1_1030_5b00(u32 param_1)

{
  return ((int)param_1 + 0x4) + 0xb;
}



void pass1_1030_5b1c(u32 param_1,u16 *param_2,u16 *param_3)

{
  u16 uVar1;

  uVar1 = (u16)(param_1 >> 0x10);
  *param_3 = ((int)param_1 + 0x1a);
  *param_2 = ((int)param_1 + 0x1c);
  return;
}



void pass1_1030_5b3e(u32 param_1,i16 param_2,u16 param_3)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = (u16)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  (iVar1 + 0x1a) = param_3;
  if ((iVar1 + 0x1c) < param_2) {
    (iVar1 + 0x1c) = param_2;
  }
  return;
}



u32 pass1_1030_5b5c(i16 param_1,u16 param_2)

{
  return CONCAT22(param_2,param_1 + 0x14);
}



void pass1_1030_5b6c(u16 param_1,astruct_610 *param_2,char *param_3)

{
  i32 lVar1;
  u16 uVar2;
  astruct_610 *iVar4;
  astruct_609 *iVar3;
  u16 uVar3;

  uVar3 = (u16)((u32)param_2 >> 0x10);
  iVar4 = (astruct_610 *)param_2;
  if (iVar4->field16_0x10 != 0x0) {
    lVar1 = iVar4->field16_0x10;
    fn_ptr_1000_17ce(*(char **)((int)lVar1 + 0x4));
    uVar2 = str_op_1008_60e8(param_1,param_3);
    lVar1 = iVar4->field16_0x10;
    uVar3 = (u16)((u32)lVar1 >> 0x10);
    iVar3 = (astruct_609 *)lVar1;
    iVar3->field4_0x4 = uVar2;
    iVar3->field5_0x6 = param_1;
  }
  return;
}



StructD * pass1_1030_5baa(StructD *param_1,u8 param_2)

{
  pass1_1030_56b0(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_5bec(u32 param_1)

{
  _PTR_LOOP_1050_5736 = param_1;
  pass1_1000_54a0(param_1,0x0,0x24);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_5c0e(void)

{
  _PTR_LOOP_1050_5736 = 0x0;
  return;
}



BOOL16 pass1_1030_5c1a(u32 param_1,u32 param_2)

{
  BOOL16 BVar1;
  HFILE16 in_stack_0000ffe8;

  BVar1 = write_to_file_1008_7cac(param_2);
  if (BVar1 != 0x0) {
    BVar1 = write_to_file_1008_7e1c((u8 *)param_2,param_1,(char *)0x24,in_stack_0000ffe8);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return BVar1;
    }
    BVar1 = 0x1;
  }
  return BVar1;
}



BOOL16 read_file_1030_5c52(u16_t param_1,u8 *param_2,HFILE16 *param_3)

{
  BOOL16 BVar1;

  read_file_1008_7cfe((int)param_3,(int)((u32)param_3 >> 0x10),0x9);
  if (param_1 != 0x0) {
    BVar1 = read_file_1008_7dee(param_3,param_2,0x24);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return BVar1;
    }
    param_1 = 0x1;
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_5c8a(u32 param_1,u32 param_2)

{
  i32 *plVar1;
  u16 uVar2;
  u32 uVar3;
  u16 uVar4;
  astruct_177 *iVar5;
  u16 uVar5;
  u32 uStack6;

  uStack6 = 0x0;
  uVar2 = (u16)param_2._3_1_;
  if (uVar2 == 0xff) {
    return;
  }
  uVar5 = (u16)((u32)_PTR_LOOP_1050_65e2 >> 0x10);
  iVar5 = (astruct_177 *)((int)_PTR_LOOP_1050_65e2 + 0xa);
  uVar3 = *(u32 *)(iVar5 + uVar2 * 0x4);
  uVar4 = (iVar5 + uVar2 * 0x4 + 0x2);
  if (((int)uVar3 + 0x4) != 0x0) {
    pass1_1030_12ca((astruct_176 *)(uVar3 & 0xffff | (u32)uVar4 << 0x10));
    uStack6 = uVar3 & 0xffff | (u32)uVar4 << 0x10;
  }
  if (uStack6 == 0x0) {
    plVar1 = (i32 *)(uVar2 * 0x4 + (int)param_1);
    *plVar1 = *plVar1 + 0x1;
  }
  return;
}



u16 * pass1_1030_5d0a(u16 *param_1)

{
  u32 in_EDX;
  u16 uVar1;

  struct_1030_17ce((astruct_180 *)param_1,0x1,0x4,in_EDX);
  uVar1 = (u16)((u32)param_1 >> 0x10);
  *(u32 *)((int)param_1 + 0x10) = 0x0;
  *param_1 = 0x613e;
  ((int)param_1 + 0x2) = 0x1030;
  return param_1;
}



u16 * pass1_1030_5d3c(u16 param_1,uchar *param_2,u16 *param_3,u32 param_4)

{
  u16 in_register_0000000a;
  u16 uVar1;

  pass1_1030_183c(param_1,(astruct_57 *)CONCAT22(in_register_0000000a,param_2),param_3,0x1,0x4,0x1000000,param_4);
  uVar1 = (u16)((u32)param_3 >> 0x10);
  *(u32 *)((int)param_3 + 0x10) = 0x0;
  *param_3 = 0x613e;
  ((int)param_3 + 0x2) = 0x1030;
  return param_3;
}



void pass1_1030_5d78(u16 *param_1)

{
  u16 uVar1;
  char *pcVar2;
  i16 iVar3;
  u16 uVar4;

  uVar4 = (u16)((u32)param_1 >> 0x10);
  iVar3 = (int)param_1;
  *param_1 = 0x613e;
  (iVar3 + 0x2) = 0x1030;
  pcVar2 = *(char **)(iVar3 + 0x10);
  uVar1 = (iVar3 + 0x12);
  if ((uVar1 | (u16)pcVar2) != 0x0) {
    pass1_1030_8480((StructD *)((u32)pcVar2 & 0xffff | (u32)uVar1 << 0x10));
    fn_ptr_1000_17ce(pcVar2);
  }
  pass1_1030_18b2(param_1);
  return;
}



void pass1_1030_5dbe(u16_t param_1,u32 param_2,u32 param_3)

{
  u32 uVar1;
  u32 uVar2;
  u16 uVar3;
  BOOL16 BVar4;
  i16 iVar5;
  i16 iVar6;
  u16 uVar7;
  HFILE16 in_stack_0000ffde;
  u16 local_c [0x5];

  uVar3 = pass1_1030_1978(param_1,(astruct_731 *)param_2,param_3);
  if (uVar3 != 0x0) {
    uVar7 = (u16)(param_2 >> 0x10);
    iVar6 = (int)param_2;
    BVar4 = pass1_1008_7c2a(param_3,*(char **)*(u32 *)(iVar6 + 0x10));
    if ((BVar4 != 0x0) &&
       (uVar1 = *(u32 *)(iVar6 + 0x10),
       iVar5 = write_to_file_1008_7b4c(param_3,(astruct_615 *)(uVar1 & 0xffff0000 | (u32)((int)uVar1 + 0x4))),
       iVar5 != 0x0)) {
      uVar2 = *(u32 *)(iVar6 + 0x10);
      local_c[0] = ((int)uVar2 + 0xa);
      BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffde);
      if (BVar4 != 0x0) {
        uVar2 = *(u32 *)(iVar6 + 0x10);
        if (((int)uVar2 + 0xa) == 0x0) {
          return;
        }
        uVar2 = *(u32 *)(iVar6 + 0x10);
        uVar7 = (u16)((u32)uVar2 >> 0x10);
        iVar6 = (int)uVar2;
        BVar4 = write_to_file_1008_7e1c
                          ((u8 *)param_3,*(u32 *)(iVar6 + 0xc),(char *)(u32)(u16)((iVar6 + 0xa) * 0x2),
                           in_stack_0000ffde);
        if (BVar4 != 0x0) {
          return;
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void file_1030_5e70(i16 param_1,u8 *param_2,u32 param_3,u32 param_4)

{
  u32 uVar1;
  u16 *puVar2;
  u32 uVar3;
  u16 uVar4;
  u8 *puVar5;
  BOOL16 BVar6;
  u16 uVar7;
  u16 uVar8;
  u16 in_register_0000000a;
  StructD *pSVar9;
  astruct_57 *paVar10;
  i16 iVar11;
  u16 uVar12;
  u16 *puVar13;
  u32 *puVar14;
  u16 in_stack_0000fa88;
  u16 in_stack_0000fbac;
  u16 in_stack_0000fbb2;
  u16 in_stack_0000fbb6;
  i16 iVar15;
  u16 uVar16;
  u32 uStack1034;
  u8 local_402 [0x400];

  pSVar9 = (StructD *)CONCAT22(in_register_0000000a,param_2);
  iVar15 = (int)param_3;
  uVar16 = (u16)(param_3 >> 0x10);
  file_1030_19b4(param_1,param_2,(astruct_373 *)param_3,(HFILE16 *)param_4);
  if (param_1 != 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar9);
    }
    else {
      pSVar9 = (StructD *)((u32)pSVar9 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    uVar4 = fn_ptr_op_1000_1708(0x10,0x0,0x1,(u16)PTR_LOOP_1050_5f2c,(u16)pSVar9);
    uVar7 = (u16)pSVar9;
    uStack1034 = CONCAT22(uVar7,uVar4);
    paVar10 = (astruct_57 *)((u32)pSVar9 & 0xffff0000 | (u32)(uVar7 | uVar4));
    if ((uVar7 | uVar4) == 0x0) {
      *(u32 *)(iVar15 + 0x10) = 0x0;
    }
    else {
      puVar13 = pass1_1008_3e38((astruct_19 *)CONCAT22(uVar7,uVar4 + 0x4));
      paVar10 = (astruct_57 *)((u32)pSVar9 & 0xffff0000 | (u32)puVar13 >> 0x10);
      *(u32 *)(iVar15 + 0x10) = uStack1034;
    }
    puVar5 = local_402;
    read_file_1008_7c6e((HFILE16)param_4,(u16)(param_4 >> 0x10),(char *)CONCAT22(0x1050,puVar5));
    if (puVar5 != NULL) {
      uVar4 = str_op_1008_60e8((u16)paVar10,(char *)CONCAT22(0x1050,local_402));
      puVar2 = (u16*)(iVar15 + 0x10);
      *puVar2 = uVar4;
      ((int)puVar2 + 0x2) = (int)paVar10;
      uVar1 = *(u32 *)(iVar15 + 0x10);
      BVar6 = read_file_1008_7bc8(param_4,(u16 *)(uVar1 & 0xffff0000 | (u32)((int)uVar1 + 0x4)));
      if (BVar6 != 0x0) {
        uVar1 = *(u32 *)(iVar15 + 0x10);
        uVar7 = (int)uVar1 + 0xa;
        BVar6 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(uVar1 & 0xffff0000 | (u32)uVar7),0x2);
        if (BVar6 != 0x0) {
          uVar3 = *(u32 *)(iVar15 + 0x10);
          uVar12 = (u16)((u32)uVar3 >> 0x10);
          iVar11 = (int)uVar3;
          if ((iVar11 + 0xa) == 0x0) {
LAB_1030_5fb7:
            puVar14 = mixed_1010_20ba(paVar10,_u16_1050_0ed0,(u8 **)CONCAT22(uVar7,0x2f),in_stack_0000fa88,
                                      in_stack_0000fbac,in_stack_0000fbb2,in_stack_0000fbb6);
            pass1_1018_04ca((u32)puVar14,*(u32 *)(iVar15 + 0x4));
            return;
          }
          uVar8 = (iVar11 + 0xa) * 0x2;
          uVar7 = uVar8;
          mem_op_1000_179c(uVar8,paVar10);
          uVar3 = *(u32 *)(iVar15 + 0x10);
          uVar12 = (u16)((u32)uVar3 >> 0x10);
          iVar11 = (int)uVar3;
          (iVar11 + 0xc) = uVar8;
          (iVar11 + 0xe) = (int)paVar10;
          uVar3 = *(u32 *)(iVar15 + 0x10);
          BVar6 = read_file_1008_7dee((HFILE16 *)param_4,*(u8 **)((int)uVar3 + 0xc),(u32)uVar7);
          if (BVar6 != 0x0) goto LAB_1030_5fb7;
        }
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



void pass1_1030_5fe2(u32 param_1,u32 param_2)

{
  *(u32 *)((int)param_1 + 0x10) = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_5ff6(u16 param_1,astruct_57 *param_2,u32 param_3)

{
  u16 *puVar1;
  code **ppcVar2;
  u32 uVar3;
  i16 iVar4;
  u16 uVar5;
  i16 iVar6;
  u16 uVar7;
  u16 uVar8;
  u8 local_6c [0x58];
  u32 uStack20;
  u32 uStack16;
  u32 uStack12;
  u16 uStack8;
  u16 uStack6;
  u16 uStack4;

  uVar7 = (u16)(param_3 >> 0x10);
  iVar6 = (int)param_3;
  if (*(i32 *)(iVar6 + 0xc) == 0x0) {
    mem_op_1000_179c(0x18,param_2);
    uStack6 = (u16)param_2;
    param_2 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)(uStack6 | param_1));
    uStack8 = param_1;
    if ((uStack6 | param_1) == 0x0) {
      *(u32 *)(iVar6 + 0xc) = 0x0;
    }
    else {
      struct_op_1030_1cd8((astruct_75 *)CONCAT22(uStack6,param_1),0x5,0x5);
      (iVar6 + 0xc) = param_1;
      (iVar6 + 0xe) = (int)param_2;
    }
  }
  for (uStack4 = 0x0; uVar3 = *(u32 *)(iVar6 + 0x10), puVar1 = (u16 *)((int)uVar3 + 0xa),
      uStack4 <= *puVar1 && *puVar1 != uStack4; uStack4 += 0x1) {
    uStack12 = pass1_1028_e2e0(param_2,_PTR_LOOP_1050_65e2,0x2);
    param_2 = (astruct_57 *)((u32)param_2 & 0xffff0000 | uStack12 >> 0x10);
    iVar4 = (int)uStack12;
    uVar3 = *(u32 *)(iVar6 + 0xc);
    ppcVar2 = (code **)((int)*(u32 *)*(u32 *)(iVar6 + 0xc) + 0x8);
    (**ppcVar2)(0x1028,(int)uVar3,(int)((u32)uVar3 >> 0x10),iVar4,(char)(uStack12 >> 0x10),uStack4,0x0);
    pass1_1030_8344(_u16_1050_5748,uStack12);
    uStack16 = CONCAT22((int)param_2,iVar4);
    uStack20 = *(u32 *)(iVar4 + 0x10);
    if (*(i32 *)((int)uStack20 + 0x2) == 0x0) {
      sys_1000_3f9c((char *)CONCAT22(0x1050,local_6c),s__s__d_1050_573a,
                    (u16)*(u32 *)*(u32 *)(iVar6 + 0x10));
      uVar5 = str_op_1008_60e8((u16)param_2,(char *)CONCAT22(0x1050,local_6c));
      uVar8 = (u16)((u32)uStack20 >> 0x10);
      ((int)uStack20 + 0x2) = uVar5;
      ((int)uStack20 + 0x4) = (int)param_2;
    }
  }
  return;
}



StructD * pass1_1030_6118(StructD *param_1,u8 param_2)

{
  pass1_1030_5d78(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_615a(StructD *param_1,astruct_137 *param_2)

{
  u16 extraout_DX;
  u16 uVar1;
  astruct_137 *iVar2;
  u16 uVar2;

  uVar2 = (u16)((u32)param_2 >> 0x10);
  iVar2 = (astruct_137 *)param_2;
  uVar1 = 0x0;
  *(u32 *)param_2 = 0x0;
  *(u32 *)&iVar2->field4_0x4 = 0x0;
  mem_op_1000_179c(0xc,(astruct_57 *)param_1);
  extraout_DX = (u16)param_1 | uVar1;
  if (extraout_DX == 0x0) {
    *(u32 *)&iVar2->field4_0x4 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22((u16)param_1,uVar1));
    iVar2->field4_0x4 = uVar1;
    iVar2->field5_0x6 = extraout_DX;
  }
  _PTR_LOOP_1050_5740 = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_61b0(u16 *param_1)

{
  u16 uVar1;
  u32 *puVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;

  uVar5 = (u16)((u32)param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar1 = (iVar4 + 0x2);
  if ((uVar1 | (u16)(u32 *)*param_1) != 0x0) {
    ppcVar3 = (code **)*(u32 *)*param_1;
    (**ppcVar3)();
  }
  puVar2 = (u32 *)(iVar4 + 0x4);
  uVar1 = (iVar4 + 0x6);
  if ((uVar1 | (u16)puVar2) != 0x0) {
    ppcVar3 = (code **)*puVar2;
    (**ppcVar3)();
  }
  _PTR_LOOP_1050_5740 = 0x0;
  return;
}



void pass1_1030_61fe(u16 param_1,u16 param_2,u32 param_3,u32 param_4,u32 param_5,i32 param_6)

{
  pass1_1030_677a(param_3,param_6);
  pass1_1030_8aa0(CONCAT22(param_2,param_1),param_4,(u16 *)param_5,param_2);
  return;
}



u16 pass1_1030_6222(u16 param_1,uchar *param_2,u32 param_3,i16 param_4,u32 param_5,u32 param_6)

{
  code **ppcVar1;
  u16 uVar2;
  u16 in_register_0000000a;
  astruct_57 *paVar3;
  u32 uStack6;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x4c,paVar3);
  uVar2 = (u16)paVar3 | param_1;
  if (uVar2 == 0x0) {
    param_1 = 0x0;
    uVar2 = 0x0;
  }
  else {
    pass1_1030_88ce((u16 *)CONCAT22((u16)paVar3,param_1),param_5,param_6);
  }
  uStack6 = CONCAT22(uVar2,param_1);
  ppcVar1 = (code **)((int)*(u32 *)*(u32 *)((int)param_3 + 0x4) + 0x4);
  (**ppcVar1)();
  if (param_4 != 0x0) {
    pass1_1030_8d08(uStack6,uVar2);
  }
  return 0x0;
}



void pass1_1030_627e(u16 param_1,u16 param_2,u32 param_3,u16 *param_4,i32 param_5)

{
  u32 local_12 [0x2];
  u32 uStack10;
  u32 uStack6;

  uStack6 = 0x0;
  pass1_1030_677a(param_3,param_5);
  uStack10 = CONCAT22(param_2,param_1);
  if ((param_2 | param_1) != 0x0) {
    pass1_1030_8b00(uStack10,param_4,(u16 *)CONCAT22(0x1050,local_12));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1030_62e4(u32 *param_1,u16 *param_2,i32 param_3)

{
  code **ppcVar1;
  u32 *puVar2;
  u16 uVar3;
  u16 uVar4;
  u32 in_EDX;
  astruct_57 *paVar5;
  u32 uVar6;
  u16 uVar7;
  u16 *puVar8;
  i16 local_64 [0x3];
  u32 uStack94;
  u16 uStack88;
  u16 uStack78;
  u16 uStack76;
  u32 local_40;
  u32 uStack60;
  u16 uStack56;
  u32 *puStack54;
  u16 uStack52;
  u32 *puStack50;
  u16 uStack48;
  u16 uStack46;
  i16 iStack44;
  u8 local_2a [0x2];
  i16 local_28;
  i16 local_26;
  u16 local_24;
  u8 local_22 [0x2];
  u8 local_20 [0x2];
  u16 local_1e;
  u16 local_1c;
  u16 local_1a;
  u8 local_18 [0x6];
  u8 local_12 [0x6];
  u8 local_c [0x6];
  u32 uStack6;

  uVar7 = (u16)((u32)param_1 >> 0x10);
  puVar2 = *(u32 **)param_1;
  uStack52 = ((int)param_1 + 0x2);
  paVar5 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uStack52);
  puStack54 = puVar2;
  puStack50 = puVar2;
  uStack48 = uStack52;
  if ((uStack52 | (u16)puVar2) != 0x0) {
    ppcVar1 = (code **)*puVar2;
    (**ppcVar1)();
  }
  mem_op_1000_179c(0x18,paVar5);
  uStack52 = (u16)paVar5;
  uVar3 = uStack52 | (u16)puVar2;
  puStack54 = puVar2;
  if (uVar3 == 0x0) {
    puVar2 = NULL;
    uVar3 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT13((char)((u32)paVar5 >> 0x8),CONCAT12((char)paVar5,puVar2)),0x5,0x5);
  }
  *(u32 **)param_1 = puVar2;
  ((int)param_1 + 0x2) = uVar3;
  pass1_1030_677a((u32)param_1,param_3);
  uStack6 = CONCAT22(uVar3,puVar2);
  if ((uVar3 | (u16)puVar2) != 0x0) {
    pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_c));
    pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_12));
    puVar8 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_18));
    uVar6 = (u32)puVar8 >> 0x10;
    pass1_1008_6d3e(param_2,(u16 *)CONCAT22(0x1050,local_12),(u16 *)CONCAT22(0x1050,local_c));
    pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,local_c),(u16 *)CONCAT13(0x10,CONCAT12(0x50,&local_1e)),
                    (u16 *)CONCAT22(0x1050,&local_1c),(u16 *)CONCAT22(0x1050,&local_1a));
    pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,local_12),(u16 *)CONCAT13(0x10,CONCAT12(0x50,&local_24)),
                    (u16 *)CONCAT22(0x1050,local_22),(u16 *)CONCAT22(0x1050,local_20));
    pass1_1008_6d64(param_2,(u16 *)CONCAT22(0x1050,local_18));
    pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,local_18),(u16 *)CONCAT13(0x10,CONCAT12(0x50,local_2a)),
                    (u16 *)CONCAT22(0x1050,&local_28),(u16 *)CONCAT22(0x1050,&local_26));
    if (local_24 == local_1e) {
      iStack44 = 0x0;
      uStack46 = local_1c;
      while( true ) {
        uVar4 = (u16)uVar6;
        uVar3 = local_28 + local_1c;
        if ((int)uVar3 <= (int)uStack46) break;
        for (uStack56 = local_1a; (int)uStack56 < (int)(local_26 + local_1a); uStack56 += 0x1) {
          uStack88 = local_1e;
          pass1_1008_3e54((u16 *)CONCAT13(0x10,CONCAT12(0x50,local_64)),local_1e,uStack46,uStack56);
          pass1_1030_8b00(uStack6,(u16 *)CONCAT22(0x1050,local_64),(u16 *)CONCAT22(0x1050,&local_40));
          uStack60 = local_40;
          local_64[0] = iStack44;
          uStack60._0_2_ = (u16)local_40;
          uStack78 = (u16)uStack60;
          uStack76 = local_40;
          uStack76._1_1_ = (char)((u32)local_40 >> 0x18);
          if (uStack76._1_1_ == '\0') {
            uStack60._0_2_ = 0x0;
            local_40 = 0x0;
          }
          uVar6 = (u32)local_40;
          uStack94 = CONCAT22(local_40,(u16)uStack60);
          ppcVar1 = (code **)((int)*(u32 *)*param_1 + 0x8);
          iStack44 = iStack44 + 0x1;
          (**ppcVar1)();
        }
        uStack46 += 0x1;
      }
      ppcVar1 = (code **)((int)*(u32 *)*param_1 + 0x10);
      (**ppcVar1)(0x1008,*param_1);
      if ((uVar4 | uVar3) != 0x0) {
        return;
      }
    }
  }
  return;
}



void pass1_1030_64ce(u16 param_1,u16 param_2,u32 param_3,u16 *param_4,i32 param_5,u32 *param_6)

{
  u32 *puVar1;
  u16 uVar2;
  u32 local_e;
  u32 uStack10;
  u32 uStack6;

  uStack6 = 0x0;
  pass1_1030_677a(param_3,param_5);
  uStack10 = CONCAT22(param_2,param_1);
  uVar2 = param_2 | param_1;
  if (uVar2 != 0x0) {
    puVar1 = &local_e;
    pass1_1030_8b00(uStack10,param_4,(u16 *)CONCAT22(0x1050,puVar1));
    uStack6 = *puVar1;
  }
  *param_6 = uStack6;
  return;
}



void pass1_1030_6522(u32 *param_1,u32 param_2,u32 param_3)

{
  code **ppcVar1;
  u32 *puVar2;
  u16 uVar3;
  u16 uVar4;
  u32 in_EDX;
  astruct_57 *paVar5;
  u32 uVar6;
  u16 uVar7;
  u16 *puVar8;
  u8 local_64 [0xc];
  u16 uStack88;
  u32 local_40;
  u32 uStack60;
  u16 uStack56;
  u32 *puStack54;
  u16 uStack52;
  u32 *puStack50;
  u16 uStack48;
  u16 uStack46;
  i16 iStack44;
  u8 local_2a [0x2];
  i16 local_28;
  i16 local_26;
  u16 local_24;
  u8 local_22 [0x2];
  u8 local_20 [0x2];
  u16 local_1e;
  u16 local_1c;
  u16 local_1a;
  u8 local_18 [0x6];
  u8 local_12 [0x6];
  u8 local_c [0x6];
  u32 uStack6;

  uVar7 = (u16)((u32)param_1 >> 0x10);
  puVar2 = *(u32 **)param_1;
  uStack52 = ((int)param_1 + 0x2);
  paVar5 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uStack52);
  puStack54 = puVar2;
  puStack50 = puVar2;
  uStack48 = uStack52;
  if ((uStack52 | (u16)puVar2) != 0x0) {
    ppcVar1 = (code **)*puVar2;
    (**ppcVar1)();
  }
  mem_op_1000_179c(0x18,paVar5);
  uStack52 = (u16)paVar5;
  uVar3 = uStack52 | (u16)puVar2;
  puStack54 = puVar2;
  if (uVar3 == 0x0) {
    puVar2 = NULL;
    uVar3 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT13((char)((u32)paVar5 >> 0x8),CONCAT12((char)paVar5,puVar2)),0x5,0x5);
  }
  *(u32 **)param_1 = puVar2;
  ((int)param_1 + 0x2) = uVar3;
  pass1_1030_677a((u32)param_1,param_3);
  uStack6 = CONCAT22(uVar3,puVar2);
  if ((uVar3 | (u16)puVar2) != 0x0) {
    pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_c));
    pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_12));
    puVar8 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_18));
    uVar6 = (u32)puVar8 >> 0x10;
    pass1_1008_6d3e((u16 *)param_2,(u16 *)CONCAT22(0x1050,local_12),(u16 *)CONCAT22(0x1050,local_c));
    pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,local_c),(u16 *)CONCAT13(0x10,CONCAT12(0x50,&local_1e)),
                    (u16 *)CONCAT22(0x1050,&local_1c),(u16 *)CONCAT22(0x1050,&local_1a));
    pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,local_12),(u16 *)CONCAT13(0x10,CONCAT12(0x50,&local_24)),
                    (u16 *)CONCAT22(0x1050,local_22),(u16 *)CONCAT22(0x1050,local_20));
    pass1_1008_6d64((u16 *)param_2,(u16 *)CONCAT22(0x1050,local_18));
    pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,local_18),(u16 *)CONCAT13(0x10,CONCAT12(0x50,local_2a)),
                    (u16 *)CONCAT22(0x1050,&local_28),(u16 *)CONCAT22(0x1050,&local_26));
    if (local_24 == local_1e) {
      iStack44 = 0x0;
      uStack46 = local_1c;
      while( true ) {
        uVar4 = (u16)uVar6;
        uVar3 = local_28 + local_1c;
        if ((int)uVar3 <= (int)uStack46) break;
        for (uStack56 = local_1a; (int)uStack56 < (int)(local_26 + local_1a); uStack56 += 0x1) {
          uStack88 = local_1e;
          pass1_1008_3e54((u16 *)CONCAT13(0x10,CONCAT12(0x50,local_64)),local_1e,uStack46,uStack56);
          pass1_1030_8b00(uStack6,(u16 *)CONCAT22(0x1050,local_64),(u16 *)CONCAT22(0x1050,&local_40));
          uStack60 = local_40;
          uVar6 = (u32)(u16)(iStack44 >> 0xf);
          ppcVar1 = (code **)((int)*(u32 *)*param_1 + 0x8);
          iStack44 = iStack44 + 0x1;
          (**ppcVar1)();
        }
        uStack46 += 0x1;
      }
      ppcVar1 = (code **)((int)*(u32 *)*param_1 + 0x10);
      (**ppcVar1)(0x1008,*param_1);
      if ((uVar4 | uVar3) != 0x0) {
        return;
      }
    }
  }
  return;
}



void pass1_1030_66de(u32 param_1,u32 param_2)

{
  u32 uVar1;
  u8 local_a [0x8];

  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),*(u32 *)((int)param_1 + 0x4));
  while( true ) {
    uVar1 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    if (uVar1 == 0x0) break;
    pass1_1030_8bac(uVar1,(u16)param_2);
  }
  return;
}



void pass1_1030_671c(u16 param_1,u16 param_2,u32 param_3,u32 param_4,u16 *param_5,i32 param_6)

{
  pass1_1030_677a(param_3,param_6);
  pass1_1030_8bdc(CONCAT22(param_2,param_1),param_4,param_5);
  return;
}



void pass1_1030_6740(u32 param_1)

{
  u32 uVar1;
  u8 local_a [0x8];

  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),*(u32 *)((int)param_1 + 0x4));
  while( true ) {
    uVar1 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    if (uVar1 == 0x0) break;
    pass1_1030_8c38(uVar1);
  }
  return;
}



void pass1_1030_677a(u32 param_1,i32 param_2)

{
  u8 *puVar1;
  u16 extraout_DX;
  u16 uVar2;
  u8 local_a [0x8];

  uVar2 = (u16)(param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x4) == 0x0) {
    return;
  }
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),*(u32 *)((int)param_1 + 0x4));
  do {
    puVar1 = local_a;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar1));
    if ((extraout_DX | (u16)puVar1) == 0x0) {
      return;
    }
  } while (*(i32 *)(puVar1 + 0x24) != param_2);
  return;
}



void pass1_1030_67cc(astruct_180 *param_1)

{
  astruct_180 *iVar1;
  u16 uVar1;

  struct_1030_1628(param_1);
  iVar1 = (astruct_180 *)param_1;
  iVar1 = (astruct_180 *)&iVar1->field10_0xc;
  pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = (u16)((u32)param_1 >> 0x10);
  iVar1->field13_0x12 = 0x0;
  iVar1->field14_0x14 = 0x0;
  *(u32 *)&iVar1->field15_0x16 = 0x0;
  *(u32 *)&iVar1->field17_0x1a = 0x0;
  *(u32 *)((int)&iVar1->field18_0x1c + 0x2) = 0x0;
  *(u32 *)&iVar1[0x1].field1_0x2 = 0x0;
  *(u32 *)&iVar1[0x1].field_0x6 = 0x0;
  *(u32 *)&iVar1[0x1].field_0xa = 0x0;
  *(u32 *)&iVar1[0x1].field11_0xe = 0x0;
  iVar1[0x1].field13_0x12 = 0x0;
  iVar1[0x1].field14_0x14 = 0x0;
  iVar1[0x1].field16_0x18 = 0x0;
  iVar1[0x1].field15_0x16 = 0x0;
  &iVar1[0x1].field18_0x1c = 0x0;
  iVar1[0x1].field17_0x1a = 0x0;
  (iVar1 + 0x2)->field0_0x0 = 0x0;
  ((int)&iVar1[0x1].field18_0x1c + 0x2) = 0x0;
  param_1->field0_0x0 = 0x8114;
  iVar1->field1_0x2 = 0x1030;
  return;
}



void pass1_1030_684c(u16 *param_1,u32 *param_2,u16 param_3,u16 param_4,u16 param_5,u32 param_6)

{
  astruct_57 *in_EDX;
  i16 iVar1;
  u16 uVar2;

  pass1_1030_165e(in_EDX,(astruct_175 *)param_1,0x5000000,param_6);
  uVar2 = (u16)((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(u32 *)(iVar1 + 0xc) = *param_2;
  (iVar1 + 0x10) = (param_2 + 0x1);
  (iVar1 + 0x12) = param_4;
  (iVar1 + 0x14) = param_4;
  *(u32 *)(iVar1 + 0x16) = 0x0;
  *(u32 *)(iVar1 + 0x1a) = 0x0;
  *(u32 *)(iVar1 + 0x1e) = 0x0;
  *(u32 *)(iVar1 + 0x22) = 0x0;
  *(u32 *)(iVar1 + 0x26) = 0x0;
  *(u32 *)(iVar1 + 0x2a) = 0x0;
  *(u32 *)(iVar1 + 0x2e) = 0x0;
  (iVar1 + 0x32) = 0x0;
  (iVar1 + 0x34) = 0x0;
  *(u32 *)(iVar1 + 0x36) = 0x0;
  *(u32 *)(iVar1 + 0x3a) = 0x0;
  *(u32 *)(iVar1 + 0x3e) = 0x0;
  *param_1 = 0x8114;
  (iVar1 + 0x2) = 0x1030;
  return;
}



void pass1_1030_68dc(astruct_611 *param_1)

{
  u16 uVar1;
  u16 uVar2;
  u32 *puVar3;
  char *pcVar4;
  code **ppcVar5;
  astruct_611 *iVar6;
  u16 uVar6;
  u16 unaff_CS;
  char *pcStack10;

  uVar6 = (u16)((u32)param_1 >> 0x10);
  iVar6 = (astruct_611 *)param_1;
  param_1 = 0x8114;
  iVar6->field2_0x2 = 0x1030;
  pcVar4 = *(char **)&iVar6->field_0x22;
  uVar1 = iVar6->field33_0x24;
  if ((uVar1 | (u16)pcVar4) != 0x0) {
    fn_ptr_1020_ba7e((u32 *)((u32)pcVar4 & 0xffff | (u32)uVar1 << 0x10));
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(pcVar4);
  }
  uVar1 = iVar6->field34_0x26;
  uVar2 = iVar6->field35_0x28;
  pcStack10 = (char *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    fn_ptr_1020_ba7e((u32 *)CONCAT22(uVar2,uVar1));
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(pcStack10);
  }
  puVar3 = iVar6->field29_0x1e;
  uVar1 = iVar6->field30_0x20;
  if ((uVar1 | (u16)puVar3) != 0x0) {
    ppcVar5 = (code **)*puVar3;
    (**ppcVar5)(unaff_CS,puVar3,uVar1,0x1);
  }
  puVar3 = iVar6->field48_0x36;
  uVar1 = iVar6->field49_0x38;
  if ((uVar1 | (u16)puVar3) != 0x0) {
    ppcVar5 = (code **)*puVar3;
    (**ppcVar5)(unaff_CS,puVar3,uVar1,0x1);
  }
  puVar3 = iVar6->field50_0x3a;
  uVar1 = iVar6->field51_0x3c;
  if ((uVar1 | (u16)puVar3) != 0x0) {
    ppcVar5 = (code **)*puVar3;
    (**ppcVar5)(unaff_CS,puVar3,uVar1,0x1);
  }
  puVar3 = iVar6->field52_0x3e;
  uVar1 = iVar6->field53_0x40;
  if ((uVar1 | (u16)puVar3) != 0x0) {
    ppcVar5 = (code **)*puVar3;
    (**ppcVar5)(unaff_CS,puVar3,uVar1,0x1);
  }
  pass1_1030_16b2((u16 *)param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_69cc(u16 param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;
  BOOL16 BVar2;
  i16 iVar3;
  u16 uVar4;
  u32 uVar5;

  uVar4 = (u16)(param_3 >> 0x10);
  iVar3 = (int)param_3;
  if (*(i32 *)(iVar3 + 0x3e) != 0x0) {
    return;
  }
  if ((*(i32 *)(iVar3 + 0x22) != 0x0) && (pass1_1020_ba94(*(i32 **)(iVar3 + 0x22)), (param_2 | param_1) != 0x0)) {
    return;
  }
  uVar1 = pass1_1030_6fa0(param_3);
  BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x4);
  if ((BVar2 != 0x0) &&
     (uVar5 = pass1_1028_67d4(*(u32 *)(iVar3 + 0x1a)), ((u16)(uVar5 >> 0x10) | (u16)uVar5) != 0x0)) {
    return;
  }
  return;
}



void pass1_1030_6a2c(u16 param_1,StructD *param_2,astruct_382 *param_3,astruct_383 *param_4)

{
  code **ppcVar1;
  u16 uVar3;
  astruct_384 *iVar2;
  u16 uVar4;
  u16 uVar5;
  u16 in_register_0000000a;
  astruct_57 *paVar6;
  astruct_382 *iVar4;
  astruct_383 *iVar5;
  astruct_382 *uVar6;
  astruct_383 *uVar2;
  u8 local_a [0x8];

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar6 = (astruct_382 *)((u32)param_3 >> 0x10);
  iVar4 = (astruct_382 *)param_3;
  if (iVar4->field62_0x3e == NULL) {
    mem_op_1000_179c(0xc,paVar6);
    uVar4 = (u16)paVar6;
    uVar5 = uVar4 | param_1;
    paVar6 = (astruct_57 *)(u32)uVar5;
    if (uVar5 == 0x0) {
      iVar4->field62_0x3e = NULL;
    }
    else {
      uVar3 = set_struct_1008_574a((astruct_57 *)CONCAT22(uVar4,param_1));
      &iVar4->field62_0x3e = uVar3;
      ((int)&iVar4->field62_0x3e + 0x2) = (int)paVar6;
    }
  }
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)iVar4->field62_0x3e);
  do {
    do {
      uVar4 = (u16)paVar6;
      iVar2 = (astruct_384 *)pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
      paVar6 = (astruct_57 *)(u32)(uVar4 | (u16)iVar2);
      if ((uVar4 | (u16)iVar2) == 0x0) goto LAB_1030_6af4;
      uVar2 = (astruct_383 *)((u32)param_4 >> 0x10);
      iVar5 = (astruct_383 *)param_4;
    } while ((iVar2->field5_0x6 != iVar5->field5_0x6) || (iVar2->field4_0x4 != iVar5->field4_0x4));
  } while (iVar2->field6_0x8 != iVar5->field6_0x8);
  iVar2->field7_0xa = iVar2->field7_0xa + iVar5->field7_0xa;
  iVar2->field8_0xc = iVar2->field8_0xc + iVar5->field8_0xc;
  param_4 = NULL;
LAB_1030_6af4:
  if (param_4 != NULL) {
    ppcVar1 = (code **)((int)*iVar4->field62_0x3e + 0x8);
    (**ppcVar1)(0x1008,iVar4->field62_0x3e,param_4);
  }
  return;
}



u32 pass1_1030_6b16(astruct_412 *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  u32 uVar4;
  astruct_412 *iVar5;
  u16 uVar5;
  u32 uVar6;

  uVar5 = (u16)((u32)param_1 >> 0x10);
  iVar5 = (astruct_412 *)param_1;
  if (*(i32 *)&iVar5->field_0x3a == 0x0) {
    return 0x0;
  }
  ppcVar3 = (code **)((int)**(u32 **)&iVar5->field_0x3a + 0x10);
  uVar6 = (**ppcVar3)();
  uVar4 = *(u32 *)&iVar5->field_0x3a;
  if (((int)uVar4 + 0x8) == 0x0) {
    puVar1 = *(u32 **)&iVar5->field_0x3a;
    uVar2 = iVar5->field60_0x3c;
    if ((uVar2 | (u16)puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    *(u32 *)&iVar5->field_0x3a = 0x0;
  }
  return uVar6;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_6b86(u16 param_1,u32 param_2)

{
  code **ppcVar1;
  u32 uVar2;
  u16 extraout_DX;
  u16 uVar3;
  u16 extraout_DX_00;
  i16 iVar4;
  u16 uVar5;
  u16 unaff_CS;
  u32 uStack12;
  u32 uStack8;

  uVar5 = (u16)(param_2 >> 0x10);
  iVar4 = (int)param_2;
  if (*(i32 *)(iVar4 + 0x1e) == 0x0) {
    param_1 = 0x0;
    uVar3 = 0x0;
  }
  else {
    ppcVar1 = (code **)((int)*(u32 *)*(u32 *)(iVar4 + 0x1e) + 0x10);
    (**ppcVar1)();
    uVar3 = extraout_DX;
  }
  uStack8 = CONCAT22(uVar3,param_1);
  for (uStack12 = 0x0; uStack12 < uStack8; uStack12 += 0x1) {
    ppcVar1 = (code **)((int)*(u32 *)*(u32 *)(iVar4 + 0x1e) + 0x4);
    uVar2 = uStack8;
    (**ppcVar1)(unaff_CS,*(u32 *)(iVar4 + 0x1e));
    if ((extraout_DX_00 | (u16)uVar2) != 0x0) {
      unaff_CS = 0x1028;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2 & 0xffff | (u32)extraout_DX_00 << 0x10);
    }
  }
  return;
}



void pass1_1030_6c1a(u32 param_1,i16 param_2)

{
  i16 *piVar1;
  i16 iVar2;
  i16 iVar3;
  u16 uVar4;

  uVar4 = (u16)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  iVar2 = (iVar3 + 0x32);
  (iVar3 + 0x32) = param_2;
  piVar1 = (i16 *)(iVar3 + 0x34);
  *piVar1 = *piVar1 + (param_2 - iVar2);
  iVar2 = (iVar3 + 0x32);
  if (iVar2 < 0x0) {
    iVar2 = 0x0;
  }
  (iVar3 + 0x32) = iVar2;
  return;
}



void pass1_1030_6c4c(u32 param_1,i16 param_2)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = (u16)(param_1 >> 0x10);
  iVar1 = ((int)param_1 + 0x32);
  if (param_2 < iVar1) {
    iVar1 = param_2;
  }
  ((int)param_1 + 0x34) = iVar1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_6c66(u16 param_1,uchar *param_2,astruct_386 *param_3,i16 param_4,astruct_385 *param_5)

{
  code **ppcVar1;
  u16 uVar2;
  BOOL16 BVar3;
  u16 uVar4;
  u16 uVar5;
  u16 in_register_0000000a;
  astruct_57 *paVar6;
  astruct_386 *iVar7;
  astruct_385 *iVar6;
  astruct_386 *uVar7;
  astruct_385 *uVar8;
  u16 uVar9;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar9 = 0x1030;
  uVar7 = (astruct_386 *)((u32)param_3 >> 0x10);
  iVar7 = (astruct_386 *)param_3;
  if (iVar7->field55_0x3a == NULL) {
    uVar9 = 0x1000;
    mem_op_1000_179c(0xc,paVar6);
    uVar5 = (u16)paVar6;
    uVar4 = uVar5 | param_1;
    paVar6 = (astruct_57 *)((u32)paVar6 & 0xffff0000 | (u32)uVar4);
    if (uVar4 == 0x0) {
      iVar7->field55_0x3a = NULL;
    }
    else {
      uVar9 = 0x1008;
      set_struct_1008_574a((astruct_57 *)CONCAT22(uVar5,param_1));
      &iVar7->field55_0x3a = param_1;
      ((int)&iVar7->field55_0x3a + 0x2) = (int)paVar6;
    }
  }
  ppcVar1 = (code **)((int)*iVar7->field55_0x3a + 0x8);
  (**ppcVar1)(uVar9,iVar7->field55_0x3a,param_5);
  if (param_4 != 0x0) {
    uVar8 = (astruct_385 *)((u32)param_5 >> 0x10);
    iVar6 = (astruct_385 *)param_5;
    if (iVar6->field5_0x6 != 0x0) {
      pass1_1030_6e9c((astruct_301 *)param_3,(u32)iVar6->field7_0xa,iVar6->field5_0x6);
      return;
    }
    if (iVar6->field4_0x4 != 0x0) {
      uVar5 = iVar6->field7_0xa;
      uVar4 = -uVar5;
      uVar5 = -(u16)(uVar5 != 0x0);
      pass1_1030_7ddc(uVar4,(astruct_57 *)((u32)paVar6 & 0xffff0000 | (u32)uVar5),(u32)param_3,
                      CONCAT13((char)(uVar5 >> 0x8),CONCAT12((char)uVar5,uVar4)),iVar6->field4_0x4);
      return;
    }
    if (iVar6->field6_0x8 != 0x0) {
      uVar2 = pass1_1030_6fa0((u32)param_3);
      BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0x4);
      if (BVar3 != 0x0) {
        pass1_1028_6356(iVar7->field26_0x1a,0x0,iVar6->field7_0xa,0x0);
      }
    }
  }
  return;
}



u32 pass1_1030_6d4e(u16 param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;
  u16 uStack6;
  u16 uStack4;

  uStack6 = 0x0;
  uStack4 = 0x0;
  uVar1 = (u16)(param_3 >> 0x10);
  if (*(i32 *)((int)param_3 + 0x36) != 0x0) {
    pass1_1010_9092(param_1,*(u32 *)((int)param_3 + 0x36));
    uStack6 = param_1;
    uStack4 = param_2;
  }
  return CONCAT22(uStack4,uStack6);
}



void pass1_1030_6d80(astruct_299 *param_1,u32 param_2)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  astruct_299 *iVar4;
  u16 uVar4;

  uVar4 = (u16)((u32)param_1 >> 0x10);
  iVar4 = (astruct_299 *)param_1;
  puVar1 = *(u32 **)&iVar4->field54_0x36;
  uVar2 = ((int)&iVar4->field54_0x36 + 0x2);
  if ((uVar2 | (u16)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  iVar4->field54_0x36 = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1030_6db4(uchar *param_1)

{
  i16 iVar1;
  u16 uVar2;
  u16 in_register_0000000a;
  u32 *puVar3;
  u16 in_stack_0000fe98;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000fff0;

  puVar3 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000fff0,0x2f),in_stack_0000fe98,in_stack_0000ffbc,
                           in_stack_0000ffc2,in_stack_0000ffc6);
  uVar2 = (u16)((u32)puVar3 >> 0x10);
  iVar1 = (int)puVar3;
  pass1_1010_ed3e((u32)puVar3);
  return (iVar1 + 0x18);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_6ddc(u32 param_1)

{
  u16 uVar1;
  BOOL16 BVar2;

  uVar1 = pass1_1030_6fa0(param_1);
  BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x1e);
  if (BVar2 != 0x0) {
    pass1_1030_d0c6(*(u32 *)((int)param_1 + 0x1a));
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_6e14(u32 param_1)

{
  u32 uVar1;
  u16 uVar2;
  BOOL16 BVar3;

  uVar2 = pass1_1030_6fa0(param_1);
  BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0x1e);
  if (BVar3 != 0x0) {
    uVar1 = *(u32 *)((int)param_1 + 0x1a);
    pass1_1030_d102((int)uVar1,(u16)((u32)uVar1 >> 0x10));
    return;
  }
  return;
}



void pass1_1030_6e4c(u32 param_1)

{
  u32 uVar1;
  u16 in_AX;
  u16 in_DX;
  i16 iVar2;
  u16 uVar3;

  uVar3 = (u16)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(i32 *)(iVar2 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar3 << 0x10),in_AX,in_DX);
  }
  if ((*(i32 *)(iVar2 + 0x1a) != 0x0) && (uVar1 = *(u32 *)(iVar2 + 0x1a), ((int)uVar1 + 0x12) == 0x4)) {
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_6e9c(astruct_301 *param_1,i32 param_2,i16 param_3)

{
  code **ppcVar1;
  u16 uVar2;
  u16 uVar3;
  u32 uVar4;
  u16 extraout_DX;
  u16 extraout_DX_00;
  u16 uVar5;
  astruct_301 *iVar6;
  u16 uVar6;
  u32 uStack10;
  u32 uStack6;

  uVar6 = (u16)((u32)param_1 >> 0x10);
  iVar6 = (astruct_301 *)param_1;
  uVar2 = ((int)&iVar6->field30_0x1e + 0x2) | &iVar6->field30_0x1e;
  if (uVar2 != 0x0) {
    ppcVar1 = (code **)((int)*iVar6->field30_0x1e + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(extraout_DX,uVar2);
    for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
      ppcVar1 = (code **)((int)*iVar6->field30_0x1e + 0x4);
      uVar4 = uStack6;
      (**ppcVar1)();
      uVar2 = (u16)uVar4;
      uVar5 = extraout_DX_00 | uVar2;
      if (uVar5 != 0x0) {
        uVar3 = uVar2;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | (u32)extraout_DX_00 << 0x10);
        if ((uVar3 + 0xc) == param_3) {
          param_2 += -0x1;
          pass1_1028_e332(_PTR_LOOP_1050_65e2,uVar2,extraout_DX_00);
          ppcVar1 = (code **)((int)*iVar6->field30_0x1e + 0x8);
          (**ppcVar1)(0x1028,iVar6->field30_0x1e,0x0,uStack10);
        }
        if ((param_2 | (u16)param_2) == 0x0) {
          return;
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_6f5a(u32 param_1)

{
  u16 uVar1;
  BOOL16 BVar2;

  uVar1 = pass1_1030_6fa0(param_1);
  BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x4);
  if (BVar2 != 0x0) {
    pass1_1028_6302(*(u32 *)((int)param_1 + 0x1a));
  }
  return;
}



u16 pass1_1030_6fa0(u32 param_1)

{
  u32 uVar1;
  u16 in_AX;
  u16 in_DX;
  i16 iVar2;
  u16 uVar3;

  uVar3 = (u16)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(i32 *)(iVar2 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar3 << 0x10),in_AX,in_DX);
  }
  if (*(i32 *)(iVar2 + 0x1a) != 0x0) {
    uVar1 = *(u32 *)(iVar2 + 0x1a);
    return ((int)uVar1 + 0xc);
  }
  return 0x0;
}



void pass1_1030_6fd4(u32 param_1)

{
  u32 uVar1;
  u16 in_AX;
  u16 in_DX;
  u16 uVar2;

  uVar2 = (u16)(param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar2 << 0x10),in_AX,in_DX);
  }
  uVar1 = *(u32 *)((int)param_1 + 0x1a);
  if (((int)uVar1 + 0x12) == 0x5) {
    return;
  }
  return;
}



void pass1_1030_701c(u32 param_1)

{
  u32 uVar1;
  u16 in_AX;
  u16 in_DX;
  u16 uVar2;

  uVar2 = (u16)(param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar2 << 0x10),in_AX,in_DX);
  }
  uVar1 = *(u32 *)((int)param_1 + 0x1a);
  if (((int)uVar1 + 0x12) == 0x5) {
    return;
  }
  return;
}



void pass1_1030_7064(u32 param_1)

{
  u32 uVar1;
  u16 in_AX;
  u16 in_DX;
  u16 uVar2;

  uVar2 = (u16)(param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar2 << 0x10),in_AX,in_DX);
  }
  uVar1 = *(u32 *)((int)param_1 + 0x1a);
  if (((int)uVar1 + 0x12) == 0x5) {
    return;
  }
  return;
}



void pass1_1030_70ac(u32 param_1)

{
  u32 uVar1;
  u16 in_AX;
  u16 in_DX;
  u16 uVar2;

  uVar2 = (u16)(param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar2 << 0x10),in_AX,in_DX);
  }
  uVar1 = *(u32 *)((int)param_1 + 0x1a);
  if (((int)uVar1 + 0x12) == 0x5) {
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_70f4(u32 param_1)

{
  u16 uVar1;
  u32 uVar2;
  u16 in_AX;
  BOOL16 BVar3;
  u16 in_DX;
  i16 iVar4;
  u16 uVar5;
  i32 *plVar6;

  uVar5 = (u16)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(i32 *)(iVar4 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar5 << 0x10),in_AX,in_DX);
  }
  uVar2 = *(u32 *)(iVar4 + 0x1a);
  uVar1 = ((int)uVar2 + 0xc);
  BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x1);
  if (BVar3 == 0x0) {
    BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x2);
    if ((BVar3 == 0x0) || (*(i32 *)(iVar4 + 0x22) == 0x0)) {
      return;
    }
    plVar6 = *(i32 **)(iVar4 + 0x22);
  }
  else {
    uVar2 = *(u32 *)(iVar4 + 0x1a);
    plVar6 = *(i32 **)((int)uVar2 + 0x28);
  }
  pass1_1020_ba94(plVar6);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_7176(u32 param_1)

{
  u32 uVar1;
  u16 in_AX;
  u16 in_DX;
  i16 iVar2;
  u16 uVar3;
  i32 local_1a;
  i16 local_16 [0x2];
  u16 uStack18;
  u16 uStack14;
  BOOL16 BStack10;
  u16 uStack8;
  i32 lStack6;

  lStack6 = 0x0;
  uVar3 = (u16)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(i32 *)(iVar2 + 0x22) == 0x0) {
    return;
  }
  if (*(i32 *)(iVar2 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)param_1,in_AX,in_DX);
  }
  uVar1 = *(u32 *)(iVar2 + 0x1a);
  uStack8 = ((int)uVar1 + 0xc);
  BStack10 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x3);
  if ((BStack10 != 0x0) && (uVar1 = *(u32 *)(iVar2 + 0x1a), ((int)uVar1 + 0x12) == 0x5)) {
    uVar1 = *(u32 *)(iVar2 + 0x22);
    uStack14 = ((int)uVar1 + 0x4);
    for (uStack18 = 0x0; uStack18 < uStack14; uStack18 += 0x1) {
      pass1_1020_bb16(*(u32 **)(iVar2 + 0x22),(u32 *)CONCAT22(0x1050,&local_1a),(u16 *)CONCAT22(0x1050,local_16),
                      uStack18);
      if (0x0 < local_16[0]) {
        lStack6 += local_1a;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_7226(u32 param_1)

{
  u32 uVar1;
  u32 uVar2;
  u16 in_AX;
  BOOL16 BVar3;
  u16 in_DX;
  i16 iVar4;
  u16 uVar5;

  uVar5 = (u16)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(i32 *)(iVar4 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar5 << 0x10),in_AX,in_DX);
  }
  uVar2 = *(u32 *)(iVar4 + 0x1a);
  BVar3 = pass1_1008_c6ae(_u16_1050_06e0,((int)uVar2 + 0xc),0x10);
  if (((BVar3 != 0x0) && (uVar2 = *(u32 *)(iVar4 + 0x1a), ((int)uVar2 + 0x12) == 0x5)) &&
     (uVar1 = *(u32 *)(iVar4 + 0x1a), uVar2 = *(u32 *)(uVar1 & 0xffff0000 | (u32)((int)uVar1 + 0x14)),
     ((int)uVar2 + 0xa4) == 0x1e)) {
    return;
  }
  return;
}



void fn_ptr_1030_7296(astruct_292 *param_1)

{
  u16 uVar1;
  u16 uVar2;
  astruct_292 *iVar3;
  astruct_292 *uVar3;
  char *pcStack6;

  uVar3 = (astruct_292 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_292 *)param_1;
  uVar1 = iVar3->field34_0x22;
  uVar2 = iVar3->field35_0x24;
  pcStack6 = (char *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    fn_ptr_1020_ba7e((u32 *)CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack6);
  }
  *(u32 *)&iVar3->field34_0x22 = 0x0;
  return;
}



void pass1_1030_72d0(astruct_292 *param_1)

{
  u16 uVar1;
  u16 uVar2;
  astruct_292 *iVar3;
  u16 uVar3;
  char *pcStack6;

  uVar3 = (u16)((u32)param_1 >> 0x10);
  iVar3 = (astruct_292 *)param_1;
  uVar1 = (iVar3 + 0x1);
  uVar2 = &iVar3[0x1].field_0x2;
  pcStack6 = (char *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    fn_ptr_1020_ba7e((u32 *)CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack6);
  }
  *(u32 *)(iVar3 + 0x1) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_730a(u16 param_1,astruct_290 *param_2)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  u32 uVar4;
  u16 extraout_DX;
  u16 extraout_DX_00;
  astruct_290 *pstruct295_1;
  astruct_290 *pstruct295_2;
  u16 unaff_CS;
  u32 *puVar5;
  u32 uStack10;
  u32 uStack6;

  pstruct295_2 = (astruct_290 *)((u32)param_2 >> 0x10);
  pstruct295_1 = (astruct_290 *)param_2;
  if (pstruct295_1->field30_0x1e != NULL) {
    puVar5 = pstruct295_1->field30_0x1e;
    ppcVar3 = (code **)((int)*pstruct295_1->field30_0x1e + 0x10);
    (**ppcVar3)();
    uStack6 = CONCAT22(extraout_DX,param_1);
    for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
      ppcVar3 = (code **)((int)*pstruct295_1->field30_0x1e + 0x4);
      uVar4 = uStack6;
      (**ppcVar3)(unaff_CS);
      if ((extraout_DX_00 | (u16)uVar4) != 0x0) {
        unaff_CS = 0x1028;
        pass1_1028_e332(_PTR_LOOP_1050_65e2,(u16)uVar4,extraout_DX_00);
      }
    }
    // WARNING: Load size is inaccurate
    puVar1 = pstruct295_1->field30_0x1e;
    uVar2 = ((int)&pstruct295_1->field30_0x1e + 0x2);
    if ((uVar2 | (u16)puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)(unaff_CS,puVar1,uVar2,0x1,puVar5);
    }
    pstruct295_1->field30_0x1e = NULL;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 struct_op_1030_73a8(astruct_419 *param_1,u16 param_2,u16 param_3)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = (u16)((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(i32 *)(iVar1 + 0x16) == 0x0) {
    return 0x0;
  }
  if (*(i32 *)(iVar1 + 0x1a) == 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,*(u32 *)(iVar1 + 0x16));
    (iVar1 + 0x1a) = param_2;
    (iVar1 + 0x1c) = param_3;
  }
  return CONCAT22((iVar1 + 0x1c),(iVar1 + 0x1a));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_73ee(u16 param_1,astruct_294 *param_2,u32 param_3)

{
  astruct_294 *iVar1;
  u16 uVar1;

  uVar1 = (u16)((u32)param_2 >> 0x10);
  iVar1 = (astruct_294 *)param_2;
  iVar1->field42_0x2a = param_3;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_3);
  iVar1->field43_0x2e = (int)param_3;
  iVar1->field44_0x30 = param_1;
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1030_7418(i16 param_1,astruct_731 *param_2,u32 param_3)

{
  u32 uVar1;
  astruct_731 *iVar2;
  i16 iVar3;
  BOOL16 BVar4;
  u8 *puVar5;
  u16 extraout_DX;
  u16 extraout_DX_00;
  u16 uVar6;
  HFILE16 in_stack_0000ffac;
  u16 uStack62;
  u16 local_2a [0x2];
  u8 local_26 [0xe];
  u32 local_18;
  u32 local_14 [0x2];
  u16 local_c;
  u32 local_a;
  u16 local_6 [0x2];

  pass1_1030_16d6(param_2,param_3);
  if (param_1 == 0x0) {
    return;
  }
  iVar2 = (astruct_731 *)param_2;
  iVar2 = (astruct_731 *)&iVar2->field_0xc;
  iVar3 = write_to_file_1008_7b4c(param_3,(astruct_615 *)((u32)param_2 & 0xffff0000 | ZEXT24(iVar2)));
  if (iVar3 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  uVar6 = (u16)((u32)param_2 >> 0x10);
  local_c = iVar2->field18_0x12;
  BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffac);
  if (BVar4 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  local_6[0] = iVar2->field19_0x14;
  BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffac);
  if (BVar4 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  local_18 = iVar2->field20_0x16;
  BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_18),(char *)0x4,in_stack_0000ffac);
  if (BVar4 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  write_to_file_1008_7954(BVar4,param_3,iVar2->field25_0x1e);
  if (BVar4 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  write_to_file_1008_7a22(param_3,iVar2->field26_0x22);
  if (BVar4 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  write_to_file_1008_7a22(param_3,iVar2->field27_0x26);
  if (BVar4 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  local_a = iVar2->field28_0x2a;
  BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_a),(char *)0x4,in_stack_0000ffac);
  if (BVar4 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  local_c = iVar2->field33_0x32;
  BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffac);
  if (BVar4 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  local_c = iVar2->field34_0x34;
  BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffac);
  if (BVar4 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  pass1_1008_79f0(param_3,iVar2->field35_0x36);
  if (BVar4 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  if (iVar2->field36_0x3a == 0x0) {
    local_18 &= 0xffff0000;
  }
  else {
    uVar1 = iVar2->field36_0x3a;
    local_18 = local_18 & 0xffff0000 | (u32)((int)uVar1 + 0x8);
  }
  local_6[0] = (u16)local_18;
  BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffac);
  if (BVar4 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  pass1_1008_5784((char *)CONCAT22(0x1050,local_26),iVar2->field36_0x3a);
  while( true ) {
    puVar5 = local_26;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar5));
    local_14[0] = CONCAT22(extraout_DX,puVar5);
    if ((extraout_DX | (u16)puVar5) == 0x0) {
      if (iVar2->field37_0x3e == 0x0) {
        uStack62 = 0x0;
      }
      else {
        uVar1 = iVar2->field37_0x3e;
        uStack62 = ((int)uVar1 + 0x8);
      }
      local_2a[0] = uStack62;
      BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_2a),(char *)0x2,in_stack_0000ffac);
      if (BVar4 == 0x0) {
        u16_1050_0310 = 0x6d0;
        return;
      }
      pass1_1008_5784((char *)CONCAT22(0x1050,local_26),iVar2->field37_0x3e);
      while( true ) {
        puVar5 = local_26;
        pass1_1008_5b12((char *)CONCAT22(0x1050,puVar5));
        if ((extraout_DX_00 | (u16)puVar5) == 0x0) {
          return;
        }
        local_18 = local_18 & 0xffff0000 | (u32)(puVar5 + 0x4);
        BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_18),(char *)0x2,in_stack_0000ffac);
        if (BVar4 == 0x0) {
          u16_1050_0310 = 0x6d0;
          return;
        }
        local_14[0] = local_14[0] & 0xffff0000 | (u32)(puVar5 + 0x6);
        BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_14),(char *)0x2,in_stack_0000ffac);
        if (BVar4 == 0x0) {
          u16_1050_0310 = 0x6d0;
          return;
        }
        local_c = (puVar5 + 0x8);
        BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffac);
        if (BVar4 == 0x0) break;
        local_c = (puVar5 + 0xa);
        BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffac);
        if (BVar4 == 0x0) {
          u16_1050_0310 = 0x6d0;
          return;
        }
        local_6[0] = (puVar5 + 0xc);
        BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffac);
        if (BVar4 == 0x0) {
          u16_1050_0310 = 0x6d0;
          return;
        }
      }
      u16_1050_0310 = 0x6d0;
      return;
    }
    local_6[0] = (puVar5 + 0x4);
    BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffac);
    if (BVar4 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return;
    }
    local_6[0] = ((int)local_14[0] + 0x6);
    BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffac);
    if (BVar4 == 0x0) break;
    local_6[0] = ((int)local_14[0] + 0x8);
    BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffac);
    if (BVar4 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return;
    }
    local_6[0] = ((int)local_14[0] + 0xa);
    BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffac);
    if (BVar4 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return;
    }
    local_6[0] = ((int)local_14[0] + 0xc);
    BVar4 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffac);
    if (BVar4 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return;
    }
  }
  u16_1050_0310 = 0x6d0;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void file_1030_778c(i16 param_1,uchar *param_2,astruct_373 *param_3,HFILE16 *param_4)

{
  code **ppcVar1;
  astruct_387 *iVar3;
  BOOL16 BVar2;
  i16 iVar6;
  i32 *plVar7;
  astruct_169 *paVar8;
  u16 uVar9;
  u16 uVar11;
  astruct_99 *uVar10;
  u16 in_register_0000000a;
  astruct_57 *paVar12;
  astruct_99 *iVar4;
  astruct_99 *iVar5;
  u16 uVar13;
  u16 uVar15;
  astruct_99 *uVar14;
  u16 local_56 [0x2];
  u16 uStack82;
  astruct_99 *paStack74;
  u16 local_46 [0x2];
  u16 local_42 [0x2];
  u32 local_3e [0x3];
  astruct_99 *paStack50;
  u16 local_2e;
  astruct_99 *paStack44;
  u16 local_28 [0x2];
  u16 local_24 [0x2];
  u16 local_20 [0x9];
  u16 uStack14;
  u16 local_4;
  astruct_388 *uVar5;
  astruct_99 *uVar8;

  paVar12 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  file_1030_1730(param_3,param_4);
  if (param_1 != 0x0) {
    iVar3 = (astruct_387 *)param_3;
    iVar3 = (astruct_387 *)&iVar3->field_0xc;
    BVar2 = read_file_1008_7bc8((u32)param_4,(u16 *)((u32)param_3 & 0xffff0000 | ZEXT24(iVar3)));
    if ((BVar2 != 0x0) && (BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_4),0x2), BVar2 != 0x0)) {
      uVar13 = (u16)((u32)param_3 >> 0x10);
      iVar3->field18_0x12 = local_4;
      BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_4),0x2);
      if (BVar2 != 0x0) {
        iVar3->field19_0x14 = local_4;
        BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar3->field_0x16)),0x4);
        if (BVar2 != 0x0) {
          plVar7 = (i32 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar3->field_0x1e));
          file_1008_76e4(paVar12,param_4,plVar7);
          if (((((int)plVar7 != 0x0) &&
               (iVar6 = file_1008_77cc((int)paVar12,(u32)param_4,
                                       (i32 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar3->field_0x22))), iVar6 != 0x0
               )) && (iVar6 = file_1008_77cc((int)paVar12,(u32)param_4,
                                             (i32 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar3->field_0x26))),
                     iVar6 != 0x0)) &&
             (BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar3->field40_0x2a)),0x4
                                         ), BVar2 != 0x0)) {
            if (iVar3->field40_0x2a != 0x0) {
              pass1_1028_e1ec(_PTR_LOOP_1050_65e2,iVar3->field40_0x2a);
              iVar3->field41_0x2e = BVar2;
              iVar3->field42_0x30 = (uchar *)paVar12;
            }
            if ((int)u16_1050_0312 < 0x2) {
              return;
            }
            BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar3->field_0x32)),0x2);
            if ((BVar2 != 0x0) &&
               (BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar3->field_0x34)),0x2
                                           ), BVar2 != 0x0)) {
              paVar8 = (astruct_169 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar3->field_0x36));
              pass1_1008_766e((uchar *)paVar12,(u32)param_4,paVar8);
              if (((int)paVar8 != 0x0) &&
                 (BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,local_20),0x2), BVar2 != 0x0)) {
                for (uStack14 = 0x0; uVar15 = (u16)((u32)paVar12 >> 0x10), uStack14 < local_20[0];
                    uStack14 += 0x1) {
                  local_3e[0] = _PTR_LOOP_1050_68a2;
                  paStack50 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
                  uVar11 = (u16)((u32)paStack50 >> 0x10);
                  uVar5 = (astruct_388 *)paStack50;
                  paVar12 = (astruct_57 *)CONCAT22(uVar15,uVar11 | (u16)uVar5);
                  if ((uVar11 | (u16)uVar5) == 0x0) {
                    paStack44 = NULL;
                  }
                  else {
                    paStack50->field0_0x0 = 0x389a;
                    uVar5->field2_0x2 = 0x1008;
                    uVar5->field3_0x4 = 0x0;
                    uVar5->field4_0x6 = 0x0;
                    uVar5->field5_0x8 = 0x0;
                    uVar5->field6_0xa = 0x0;
                    uVar5->field7_0xc = 0x0;
                    paStack50->field0_0x0 = 0x56ce;
                    uVar5->field2_0x2 = 0x1018;
                    paStack44 = paStack50;
                  }
                  BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,local_28),0x2);
                  if (((BVar2 == 0x0) ||
                      (BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,local_24),0x2), BVar2 == 0x0)) ||
                     ((BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_2e),0x2), BVar2 == 0x0 ||
                      ((BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)paStack44 & 0xffff0000 |
                                                                  (u32)((int)paStack44 + 0xa)),0x2), BVar2 == 0x0 ||
                       (BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)paStack44 & 0xffff0000 |
                                                                  (u32)((int)paStack44 + 0xc)),0x2), BVar2 == 0x0)))))
                     ) goto LAB_1030_77be;
                  uVar15 = (u16)((u32)paStack44 >> 0x10);
                  iVar4 = (astruct_99 *)paStack44;
                  &iVar4->field2_0x4 = local_28[0];
                  ((int)&iVar4->field2_0x4 + 0x2) = local_24[0];
                  &iVar4->field3_0x8 = local_2e;
                  if (iVar3->field51_0x3a == NULL) {
                    uVar11 = local_2e;
                    mem_op_1000_179c(0xc,paVar12);
                    uVar9 = (u16)paVar12;
                    paStack50 = (astruct_99 *)CONCAT22(uVar9,uVar11);
                    paVar12 = (astruct_57 *)((u32)paVar12 & 0xffff0000 | (u32)(uVar9 | uVar11));
                    if ((uVar9 | uVar11) == 0x0) {
                      iVar3->field51_0x3a = NULL;
                    }
                    else {
                      set_struct_1008_574a((astruct_57 *)CONCAT22(uVar9,uVar11));
                      &iVar3->field51_0x3a = uVar11;
                      ((int)&iVar3->field51_0x3a + 0x2) = (int)paVar12;
                    }
                  }
                  ppcVar1 = (code **)((int)*iVar3->field51_0x3a + 0x8);
                  (**ppcVar1)();
                }
                BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,local_56),0x2);
                if (BVar2 != 0x0) {
                  uStack82 = 0x0;
                  while( true ) {
                    uVar15 = (u16)((u32)paVar12 >> 0x10);
                    if (local_56[0] <= uStack82) {
                      return;
                    }
                    paStack44 = (astruct_99 *)_PTR_LOOP_1050_68a2;
                    paStack50 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
                    uVar10 = (astruct_99 *)((u32)paStack50 >> 0x10);
                    uVar8 = (astruct_99 *)paStack50;
                    paVar12 = (astruct_57 *)CONCAT22(uVar15,(u16)uVar10 | (u16)uVar8);
                    if (((u16)uVar10 | (u16)uVar8) == 0x0) {
                      paStack74 = NULL;
                    }
                    else {
                      paStack50->field0_0x0 = 0x389a;
                      uVar8->field1_0x2 = 0x1008;
                      &uVar8->field2_0x4 = 0x0;
                      ((int)&uVar8->field2_0x4 + 0x2) = 0x0;
                      &uVar8->field3_0x8 = 0x0;
                      ((int)&uVar8->field3_0x8 + 0x2) = 0x0;
                      uVar8->field4_0xc = 0x0;
                      paStack50->field0_0x0 = 0x56ce;
                      uVar8->field1_0x2 = 0x1018;
                      paStack74 = paStack50;
                    }
                    BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,local_46),0x2);
                    if ((((BVar2 == 0x0) ||
                         (BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,local_42),0x2), BVar2 == 0x0)) ||
                        (BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,local_3e),0x2), BVar2 == 0x0)) ||
                       ((BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)paStack74 & 0xffff0000 |
                                                                   (u32)((int)paStack74 + 0xa)),0x2), BVar2 == 0x0 ||
                        (BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)paStack74 & 0xffff0000 |
                                                                   (u32)((int)paStack74 + 0xc)),0x2), BVar2 == 0x0))))
                    break;
                    uVar14 = (astruct_99 *)((u32)paStack74 >> 0x10);
                    iVar5 = (astruct_99 *)paStack74;
                    &iVar5->field2_0x4 = local_46[0];
                    ((int)&iVar5->field2_0x4 + 0x2) = local_42[0];
                    &iVar5->field3_0x8 = (u16)local_3e[0];
                    if (iVar3->field52_0x3e == NULL) {
                      mem_op_1000_179c(0xc,paVar12);
                      uVar11 = (u16)paVar12;
                      paStack50 = (astruct_99 *)CONCAT22(uVar11,(u16)local_3e[0]);
                      paVar12 = (astruct_57 *)((u32)paVar12 & 0xffff0000 | (u32)(uVar11 | (u16)local_3e[0]));
                      if ((uVar11 | (u16)local_3e[0]) == 0x0) {
                        iVar3->field52_0x3e = NULL;
                      }
                      else {
                        set_struct_1008_574a((astruct_57 *)CONCAT22(uVar11,(u16)local_3e[0]));
                        &iVar3->field52_0x3e = (u16)local_3e[0];
                        ((int)&iVar3->field52_0x3e + 0x2) = (int)paVar12;
                      }
                    }
                    ppcVar1 = (code **)((int)*iVar3->field52_0x3e + 0x8);
                    (**ppcVar1)();
                    uStack82 += 0x1;
                  }
                }
              }
            }
          }
        }
      }
    }
LAB_1030_77be:
    u16_1050_0310 = 0x6d2;
  }
  return;
}



u16 pass1_1030_7bee(u32 param_1)

{
  code **ppcVar1;
  u16 in_AX;
  u16 uVar2;
  u16 in_DX;
  i16 iVar3;
  u16 uVar4;

  uVar4 = (u16)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(i32 *)(iVar3 + 0x16) == 0x0) {
    return 0x0;
  }
  if (*(i32 *)(iVar3 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar4 << 0x10),in_AX,in_DX);
  }
  ppcVar1 = (code **)((int)*(u32 *)*(u32 *)(iVar3 + 0x1a) + 0x44);
  uVar2 = (**ppcVar1)();
  return uVar2;
}



u32 pass1_1030_7c28(u16 param_1,u16 param_2,u32 param_3,u16 param_4)

{
  u16 uVar1;
  u32 uVar2;

  uVar1 = (u16)(param_3 >> 0x10);
  if (*(i32 *)((int)param_3 + 0x22) == 0x0) {
    return 0x0;
  }
  uVar2 = *(u32 *)((int)param_3 + 0x22);
  uVar2 = pass1_1020_bae6(param_1,param_2,(u16)uVar2,CONCAT22(param_4,(int)((u32)uVar2 >> 0x10)));
  return uVar2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_7c50(u16 param_1,astruct_57 *param_2,astruct_305 *param_3,i32 param_4,i16 param_5)

{
  i16 *piVar1;
  code **ppcVar2;
  astruct_57 *uVar4;
  u16 uVar3;
  astruct_305 *iVar8;
  u16 uVar5;
  u32 uVar6;
  u32 *puVar7;
  u32 uVar8;
  u32 *puStack18;

  uVar5 = (u16)((u32)param_3 >> 0x10);
  iVar8 = (astruct_305 *)param_3;
  if (iVar8->field30_0x1e == NULL) {
    mem_op_1000_179c(0x18,param_2);
    uVar4 = (astruct_57 *)param_2;
    param_2 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)((u16)uVar4 | param_1));
    if (((u16)uVar4 | param_1) == 0x0) {
      iVar8->field30_0x1e = NULL;
    }
    else {
      struct_op_1030_1cd8((astruct_75 *)CONCAT22(uVar4,param_1),0x5,0x5);
      &iVar8->field30_0x1e = param_1;
      ((int)&iVar8->field30_0x1e + 0x2) = (int)param_2;
    }
  }
  if (param_5 == 0x4) {
    piVar1 = &iVar8->field49_0x34;
    *piVar1 = *piVar1 + (int)param_4;
  }
  while (param_4 != 0x0) {
    uVar6 = pass1_1028_e2e0(param_2,_PTR_LOOP_1050_65e2,0x6);
    param_2 = (astruct_57 *)((u32)param_2 & 0xffff0000 | uVar6 >> 0x10);
    uVar3 = (u16)uVar6;
    puVar7 = iVar8->field30_0x1e;
    ppcVar2 = (code **)((int)*iVar8->field30_0x1e + 0xc);
    uVar8 = uVar6;
    (**ppcVar2)();
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar6);
    puStack18 = (u32 *)CONCAT22((int)param_2,uVar3);
    ppcVar2 = (code **)((int)*puStack18 + 0x14);
    (**ppcVar2)(0x1028,uVar3,(int)param_2,param_3,puVar7,uVar8);
    param_4 = param_4 + -0x1;
  }
  return;
}



void pass1_1030_7d1c(u16 param_1,u16 param_2,astruct_397 *param_3,u16 param_4,u32 param_5)

{
  u16 uVar1;
  u16 in_register_0000000a;
  astruct_57 *paVar2;
  astruct_397 *iVar2;
  astruct_397 *uVar3;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar3 = (astruct_397 *)((u32)param_3 >> 0x10);
  iVar2 = (astruct_397 *)param_3;
  if (iVar2->field34_0x22 == NULL) {
    mem_op_1000_179c(0xa,paVar2);
    uVar1 = (u16)paVar2 | param_1;
    if (uVar1 == 0x0) {
      iVar2->field34_0x22 = NULL;
    }
    else {
      pass1_1020_ba3e((astruct_172 *)CONCAT22((u16)paVar2,param_1),0xa,0x2);
      &iVar2->field34_0x22 = param_1;
      ((int)&iVar2->field34_0x22 + 0x2) = uVar1;
    }
  }
  pass1_1020_bb8a(iVar2->field34_0x22,param_4,param_5);
  return;
}



void pass1_1030_7d7c(u16 param_1,uchar *param_2,astruct_398 *param_3,u16 param_4,u32 param_5)

{
  u16 uVar1;
  u16 in_register_0000000a;
  astruct_57 *paVar2;
  astruct_398 *iVar2;
  astruct_398 *uVar3;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar3 = (astruct_398 *)((u32)param_3 >> 0x10);
  iVar2 = (astruct_398 *)param_3;
  if (iVar2->field38_0x26 == NULL) {
    mem_op_1000_179c(0xa,paVar2);
    uVar1 = (u16)paVar2 | param_1;
    if (uVar1 == 0x0) {
      iVar2->field38_0x26 = NULL;
    }
    else {
      pass1_1020_ba3e((astruct_172 *)CONCAT22((u16)paVar2,param_1),0xa,0x2);
      &iVar2->field38_0x26 = param_1;
      ((int)&iVar2->field38_0x26 + 0x2) = uVar1;
    }
  }
  pass1_1020_bb8a(iVar2->field38_0x26,param_4,param_5);
  return;
}



void pass1_1030_7ddc(u16 param_1,astruct_57 *param_2,u32 param_3,i32 param_4,u16 param_5)

{
  u32 uVar1;
  astruct_57 *uVar2;
  i16 iVar2;
  u16 uVar3;
  i32 lVar4;

  uVar3 = (u16)(param_3 >> 0x10);
  iVar2 = (int)param_3;
  if (*(i32 *)(iVar2 + 0x22) == 0x0) {
    mem_op_1000_179c(0xa,param_2);
    uVar2 = (astruct_57 *)param_2;
    param_2 = (astruct_57 *)(u32)((u16)uVar2 | param_1);
    if (((u16)uVar2 | param_1) == 0x0) {
      *(u32 *)(iVar2 + 0x22) = 0x0;
    }
    else {
      pass1_1020_ba3e((astruct_172 *)CONCAT22(uVar2,param_1),0xa,0x2);
      (iVar2 + 0x22) = param_1;
      (iVar2 + 0x24) = (int)param_2;
    }
  }
  uVar1 = *(u32 *)(iVar2 + 0x22);
  lVar4 = pass1_1020_bae6(param_1,(u16)param_2,(u16)uVar1,CONCAT22(param_5,(int)((u32)uVar1 >> 0x10)));
  pass1_1020_bb8a(*(i32 **)(iVar2 + 0x22),(u16)(lVar4 + param_4),
                  CONCAT22(param_5,(int)((u32)(lVar4 + param_4) >> 0x10)));
  return;
}



void pass1_1030_7e5a(u16 param_1,astruct_358 *param_2,u32 param_3)

{
  astruct_358 *pstruct_1;
  u16 uVar1;

  uVar1 = (u16)((u32)param_2 >> 0x10);
  pstruct_1 = (astruct_358 *)param_2;
  pstruct_1->field19_0x16 = param_3;
  pstruct_1->field20_0x1a = 0x0;
  pass1_1030_6fa0((u32)param_2 & 0xffff | (u32)uVar1 << 0x10);
  if (pstruct_1->field37_0x2e != 0x0) {
    pass1_1038_4b20(param_1,pstruct_1->field37_0x2e,pstruct_1->field19_0x16,pstruct_1->field4_0x4);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 pass1_1030_7ea0(u32 param_1)

{
  u32 uVar1;
  u16 uVar2;
  BOOL16 BVar3;

  uVar2 = pass1_1030_6fa0(param_1);
  BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0xb);
  if (BVar3 != 0x0) {
    uVar1 = *(u32 *)((int)param_1 + 0x1a);
    if (((int)uVar1 + 0x12) == 0x5) {
      return 0x1;
    }
    BVar3 = 0x0;
  }
  return BVar3;
}



void pass1_1030_7eda(u32 param_1,u16 param_2)

{
  u16 in_DX;
  u16 uVar1;
  u16 local_c;
  u16 uStack10;
  u16 uStack8;
  u16 uStack6;
  u16 uStack4;

  local_c = 0x0;
  uStack10 = 0x0;
  uStack6 = 0x0;
  uStack4 = 0x0;
  uStack8 = param_2;
  uVar1 = (u16)(param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar1 << 0x10),param_2,in_DX);
  }
  pass1_1028_bb96(*(astruct_295 **)((int)param_1 + 0x1a),(u32 *)&local_c,(u16)&DAT_1050_1050);
  return;
}



void pass1_1030_7f1a(u32 param_1,u16 param_2)

{
  u16 in_DX;
  u16 uVar1;
  u16 local_c;
  u16 uStack10;
  u16 uStack8;
  u16 uStack6;
  u16 uStack4;

  local_c = 0x0;
  uStack8 = 0x0;
  uStack6 = 0x0;
  uStack4 = 0x0;
  uStack10 = param_2;
  uVar1 = (u16)(param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar1 << 0x10),param_2,in_DX);
  }
  pass1_1028_bb96(*(astruct_295 **)((int)param_1 + 0x1a),(u32 *)&local_c,(u16)&DAT_1050_1050);
  return;
}



u16 pass1_1030_7f5a(u32 param_1)

{
  u16 in_AX;
  u16 in_DX;
  u16 uVar1;
  u32 uVar2;

  uVar1 = (u16)(param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar1 << 0x10),in_AX,in_DX);
  }
  uVar2 = pass1_1028_bb6a(*(u32 *)((int)param_1 + 0x1a));
  if (uVar2 != 0x0) {
    return ((int)uVar2 + 0x4);
  }
  return 0x0;
}



u16 pass1_1030_7f98(u32 param_1)

{
  u16 in_AX;
  u16 in_DX;
  u16 uVar1;
  u32 uVar2;

  uVar1 = (u16)(param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar1 << 0x10),in_AX,in_DX);
  }
  uVar2 = pass1_1028_bb6a(*(u32 *)((int)param_1 + 0x1a));
  if (uVar2 != 0x0) {
    return ((int)uVar2 + 0x2);
  }
  return 0x0;
}



void pass1_1030_7fd6(u16 param_1,u32 param_2)

{
  i16 iVar1;
  u32 uVar2;
  u16 in_AX;
  i16 iVar3;
  u16 uVar4;
  u32 uVar5;

  uVar4 = (u16)(param_2 >> 0x10);
  iVar3 = (int)param_2;
  if (*(i32 *)(iVar3 + 0x1a) == 0x0) {
    uVar5 = struct_op_1030_73a8((astruct_419 *)(param_2 & 0xffff | (u32)uVar4 << 0x10),in_AX,param_1);
    param_1 = (u16)(uVar5 >> 0x10);
  }
  uVar2 = *(u32 *)(iVar3 + 0x1a);
  iVar1 = ((int)uVar2 + 0xc);
  if (((0x32 < iVar1) && (!SBORROW2(iVar1,0x33))) &&
     ((iVar1 == 0x34 || iVar1 + -0x33 < 0x1 || ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 0x2)))))) {
    pass1_1028_1416(param_1,*(u32 *)(iVar3 + 0x1a));
  }
  return;
}



void pass1_1030_8030(u16 param_1,u32 param_2)

{
  i16 iVar1;
  astruct_15 *paVar2;
  u32 uVar3;
  u16 in_AX;
  i16 iVar4;
  u16 uVar5;
  u32 uVar6;

  uVar5 = (u16)(param_2 >> 0x10);
  iVar4 = (int)param_2;
  if (*(i32 *)(iVar4 + 0x1a) == 0x0) {
    uVar6 = struct_op_1030_73a8((astruct_419 *)(param_2 & 0xffff | (u32)uVar5 << 0x10),in_AX,param_1);
    param_1 = (u16)(uVar6 >> 0x10);
  }
  uVar3 = *(u32 *)(iVar4 + 0x1a);
  iVar1 = ((int)uVar3 + 0xc);
  if (((0x32 < iVar1) && (!SBORROW2(iVar1,0x33))) &&
     ((iVar1 == 0x34 || iVar1 + -0x33 < 0x1 || ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 0x2)))))) {
    paVar2 = *(astruct_15 **)(iVar4 + 0x1a);
    pass1_1028_1106((int)paVar2,param_1,paVar2);
  }
  return;
}



u32 pass1_1030_8086(u32 param_1)

{
  u16 uVar1;

  uVar1 = (u16)(param_1 >> 0x10);
  return CONCAT22(((int)param_1 + 0x18),((int)param_1 + 0x16)) & 0xffffff;
}



void pass1_1030_809c(u32 param_1)

{
  u16 in_AX;
  u16 in_DX;
  u16 uVar1;

  uVar1 = (u16)(param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar1 << 0x10),in_AX,in_DX);
  }
  return;
}



astruct_611 * pass1_1030_80ee(astruct_611 *param_1,u8 param_2)

{
  pass1_1030_68dc(param_1);
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void struct_1030_8128(astruct_57 *param_1,astruct_135 *param_2)

{
  u16 uVar1;
  u16 uVar2;
  StructD *pSVar3;
  astruct_57 *paVar5;
  astruct_135 *iVar4;
  u16 uVar5;
  astruct_57 *paVar4;

  uVar5 = (u16)((u32)param_2 >> 0x10);
  iVar4 = (astruct_135 *)param_2;
  uVar1 = 0x0;
  *(u32 *)param_2 = 0x0;
  *(u32 *)&iVar4->field2_0x4 = 0x0;
  iVar4->field4_0x8 = 0x0;
  _u16_1050_5748 = param_2;
  mem_op_1000_179c(0x56,param_1);
  uVar2 = (u16)param_1 | uVar1;
  paVar4 = (astruct_57 *)((u32)param_1 & 0xffff0000 | (u32)uVar2);
  if (uVar2 != 0x0) {
    pass1_1028_d81c(paVar4,(astruct_136 *)CONCAT22((u16)param_1,uVar1),(u32)param_2);
  }
  mem_op_1000_179c(0x8,paVar4);
  uVar2 = (u16)paVar4 | uVar1;
  paVar5 = (astruct_57 *)((u32)paVar4 & 0xffff0000 | (u32)uVar2);
  if (uVar2 == 0x0) {
    *(u32 *)param_2 = 0x0;
  }
  else {
    struct_1028_d22e(paVar5,(u32 *)CONCAT22((u16)paVar4,uVar1),(u32)param_2);
    param_2->field0_0x0 = uVar1;
    iVar4->field1_0x2 = (uchar *)paVar5;
  }
  mem_op_1000_179c(0x8,paVar5);
  uVar2 = (u16)paVar5 | uVar1;
  paVar4 = (astruct_57 *)((u32)paVar5 & 0xffff0000 | (u32)uVar2);
  if (uVar2 == 0x0) {
    *(u32 *)&iVar4->field2_0x4 = 0x0;
  }
  else {
    pass1_1028_cfd2((u32 *)CONCAT22((u16)paVar5,uVar1),param_2);
    iVar4->field2_0x4 = uVar1;
    iVar4->field3_0x6 = (uchar *)paVar4;
  }
  mem_op_1000_179c(0x24,paVar4);
  uVar2 = (u16)paVar4 | uVar1;
  paVar5 = (astruct_57 *)((u32)paVar4 & 0xffff0000 | (u32)uVar2);
  if (uVar2 != 0x0) {
    pass1_1030_5bec(CONCAT22((u16)paVar4,uVar1));
  }
  mem_op_1000_179c(0x8,paVar5);
  pSVar3 = (StructD *)((u16)paVar5 | uVar1);
  if (pSVar3 != NULL) {
    pass1_1038_78e2(pSVar3,(astruct_431 *)CONCAT22((u16)paVar5,uVar1));
  }
  u16_1050_574a = (u16)((u32)_u16_1050_5748 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_8210(u16 *param_1)

{
  u16 uVar1;
  u16 uVar2;
  char *pcVar3;
  i16 iVar4;
  u16 uVar5;
  char *pcStack10;
  char *pcStack6;

  pcVar3 = _PTR_LOOP_1050_65e2;
  if (_PTR_LOOP_1050_65e2 != NULL) {
    pass1_1028_daba((u32)_PTR_LOOP_1050_65e2);
    fn_ptr_1000_17ce(pcVar3);
  }
  uVar5 = (u16)((u32)param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar1 = *param_1;
  uVar2 = (iVar4 + 0x2);
  pcStack10 = (char *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1028_d282((astruct_446 *)CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack10);
  }
  uVar1 = (iVar4 + 0x4);
  uVar2 = (iVar4 + 0x6);
  pcStack6 = (char *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1028_cff2(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack6);
  }
  pcVar3 = _PTR_LOOP_1050_5736;
  if (_PTR_LOOP_1050_5736 != NULL) {
    pass1_1030_5c0e();
    fn_ptr_1000_17ce(pcVar3);
  }
  pcVar3 = _PTR_LOOP_1050_5a64;
  if (((u16)PTR_LOOP_1050_5a66 | (u16)_PTR_LOOP_1050_5a64) != 0x0) {
    pass1_1038_7964((u16 *)((u32)_PTR_LOOP_1050_5a64 & 0xffff | ZEXT24(PTR_LOOP_1050_5a66) << 0x10));
    fn_ptr_1000_17ce(pcVar3);
  }
  _u16_1050_5748 = 0x0;
  return;
}



void pass1_1030_82f0(u32 param_1,u32 param_2)

{
  pass1_1028_d078(*(u32 *)((int)param_1 + 0x4),param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_8308(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 *param_5,u16 *param_6,
                    u32 param_7)

{
  pass1_1028_e198(param_1,param_2,_PTR_LOOP_1050_65e2,param_5,param_6,param_7);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 pass1_1030_8326(void)

{
  return CONCAT22(((int)_PTR_LOOP_1050_65e2 + 0x2),*_PTR_LOOP_1050_65e2);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_8334(void)

{
  *_PTR_LOOP_1050_65e2 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_8344(u32 param_1,u32 param_2)

{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_2);
  return;
}



void fn_ptr_1030_835a(u32 **param_1,char *param_2)

{
  fn_ptr_1028_d566(*param_1,(astruct_97 *)param_2);
  return;
}



void pass1_1030_8372(u32 **param_1,u32 param_2,u32 *param_3)

{
  pass1_1028_d52c(*param_1,param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_838e(u32 *param_1)

{
  struct_1028_d2b0((u32 *)*param_1);
  pass1_1028_d01a(*(u32 **)((int)param_1 + 0x4));
  send_msg_1028_e242(_PTR_LOOP_1050_65e2,0x1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_83ba(undefined1 param_1,u32 **param_2,i32 param_3)

{
  i32 lVar1;

  while (lVar1 = param_3 + -0x1, param_3 != 0x0) {
    struct_1028_d2b0(*param_2);
    pass1_1028_d01a(*(u32 **)((int)param_2 + 0x4));
    param_3 = lVar1;
    if (lVar1 != 0x0) {
      send_msg_1028_e242(_PTR_LOOP_1050_65e2,0x0);
    }
  }
  send_msg_1028_e242(_PTR_LOOP_1050_65e2,0x1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void vsprintf_op_1030_840a(u16 param_1,u32 param_2)

{
  WORD *pWVar1;
  u8 local_106 [0x100];
  WORD *pWStack6;
  u16 uStack4;

  if (u16_1050_574c != 0x0) {
    pWVar1 = (WORD *)&stack0x0008;
    uStack4 = SUB42(&DAT_1050_1050,0x0);
    pWStack6 = pWVar1;
    if (u16_1050_5750 == 0x0) {
      pass1_1000_2b3c(param_1,(u16)s_simres_out_1050_5758,(u16)&DAT_1050_1050,(u16)s_w_1050_5756,
                      (u16)&DAT_1050_1050,(int)&stack0xfffe);
      _u16_1050_5752 = CONCAT22(param_1,pWVar1);
      u16_1050_5750 = 0x1;
    }
    wvsprintf16(pWStack6,(char *)CONCAT22((int)param_2,uStack4),(char *)CONCAT22(local_106,(int)(param_2 >> 0x10)));
    pass1_1000_2b5c((u16)_u16_1050_5752,(u16)((u32)_u16_1050_5752 >> 0x10),(u16)s__s_1050_5763,
                    (u16)&DAT_1050_1050);
    pass1_1000_2f48(_u16_1050_5752);
  }
  return;
}



void pass1_1030_8480(StructD *param_1)

{
  fn_ptr_1000_17ce(*(char **)param_1);
  return;
}



void pass1_1030_8496(u32 param_1)

{
  fn_ptr_1000_17ce(*(char **)((int)param_1 + 0x2));
  return;
}



void pass1_1030_84ae(u32 param_1)

{
  pass1_1008_3e38((astruct_19 *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0x8)));
  ((int)param_1 + 0x1e) = 0x1;
  return;
}



void fn_ptr_1030_84d0(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;

  uVar5 = (u16)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if ((iVar4 + 0x1e) != 0x0) {
    puVar1 = (u32 *)(iVar4 + 0xe);
    uVar2 = (iVar4 + 0x10);
    if ((uVar2 | (u16)puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    puVar1 = (u32 *)(iVar4 + 0x12);
    uVar2 = (iVar4 + 0x14);
    if ((uVar2 | (u16)puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    fn_ptr_1000_17ce(*(char **)(iVar4 + 0x4));
    fn_ptr_1000_17ce(*(char **)(iVar4 + 0x16));
  }
  return;
}



void struct_1030_8544(astruct_355 *param_1,astruct_356 *param_2)

{
  astruct_356 *iVar1;
  astruct_355 *iVar2;
  astruct_356 *uVar1;
  astruct_355 *uVar2;

  param_1->field0_0x0 = param_2->field0_0x0;
  uVar1 = (astruct_356 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_356 *)param_2;
  uVar2 = (astruct_355 *)((u32)param_1 >> 0x10);
  iVar2 = (astruct_355 *)param_1;
  iVar2->field2_0x4 = iVar1->field3_0x4;
  pass1_1008_3f62((u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->field_0x8)),
                  (u16 *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar1->field_0x8)));
  iVar2->field9_0xe = iVar1->field10_0xe;
  iVar2->field10_0x12 = iVar1->field11_0x12;
  iVar2->field11_0x16 = iVar1->field12_0x16;
  iVar2->field12_0x1a = iVar1->field13_0x1a;
  iVar2->field15_0x1e = 0x0;
  return;
}



void pass1_1030_85be(astruct_172 *param_1,u16 param_2,i16 param_3)

{
  astruct_172 *iVar1;
  astruct_172 *uVar1;

  uVar1 = (astruct_172 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_172 *)param_1;
  *(u32 *)param_1 = 0x0;
  iVar1->field2_0x4 = 0x0;
  iVar1->field3_0x6 = param_3;
  iVar1->field4_0x8 = param_2;
  iVar1[0x1].field2_0x4 = 0x0;
  if (iVar1->field3_0x6 == 0x0) {
    iVar1->field3_0x6 = 0x5;
  }
  pass1_1030_878c(param_1);
  return;
}



void pass1_1030_8604(StructD *param_1)

{
  fn_ptr_1000_17ce(*(char **)param_1);
  return;
}



void pass1_1030_861a(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 param_5)

{
  u32 *puStack6;

  pass1_1030_8854(param_3,param_4,param_5);
  puStack6 = (u32 *)CONCAT22(param_2,param_1);
  if ((param_2 | param_1) == 0x0) {
    *(u32 *)(param_3 + 0xa) = 0x0;
  }
  else {
    *(u32 *)(param_3 + 0xa) = *puStack6;
  }
  return;
}



void pass1_1030_8660(u16 param_1,u16 param_2,u32 param_3,u32 *param_4,u16 param_5)

{
  u16 uVar1;
  u16 uVar2;
  u16 uVar3;
  u32 *puStack6;

  uVar2 = (u16)param_3;
  uVar3 = (u16)(param_3 >> 0x10);
  pass1_1030_8854(uVar2,uVar3,param_5);
  puStack6 = (u32 *)CONCAT22(param_2,param_1);
  uVar1 = param_2 | param_1;
  if (uVar1 == 0x0) {
    pass1_1030_8854(uVar2,uVar3,0x0);
    puStack6 = (u32 *)CONCAT22(uVar1,param_1);
    uVar1 |= param_1;
    if (uVar1 == 0x0) {
      pass1_1030_878c((astruct_172 *)param_3);
      pass1_1030_8854(uVar2,uVar3,0x0);
      puStack6 = (u32 *)CONCAT22(uVar1,param_1);
      if ((uVar1 | param_1) == 0x0) {
        return;
      }
    }
    ((int)puStack6 + 0x4) = param_5;
    *puStack6 = *param_4;
    pass1_1030_8834((u16 *)param_3);
  }
  else {
    *puStack6 = *param_4;
  }
  return;
}



void pass1_1030_86ec(astruct_612 *param_1,u16 param_2)

{
  astruct_612 *iVar1;
  u16 uVar1;

  fn_ptr_1000_17ce(*(char **)param_1);
  uVar1 = (u16)((u32)param_1 >> 0x10);
  iVar1 = (astruct_612 *)param_1;
  *(u32 *)param_1 = 0x0;
  iVar1->field4_0x4 = 0x0;
  iVar1->field5_0x6 = param_2;
  iVar1->field12_0xe = 0x0;
  return;
}



void pass1_1030_871e(i32 *param_1,u32 *param_2,u16 param_3)

{
  i16 *piVar1;
  astruct_681 *iVar2;
  u16 uVar2;

  uVar2 = (u16)((u32)param_1 >> 0x10);
  iVar2 = (astruct_681 *)param_1;
  if (*param_1 == 0x0) {
    pass1_1030_878c((astruct_172 *)((u32)param_1 & 0xffff | (u32)uVar2 << 0x10));
  }
  piVar1 = &iVar2->field14_0xe;
  *piVar1 = *piVar1 + 0x1;
  ((int)*param_1 + iVar2->field14_0xe * 0x6 + 0x4) = param_3;
  *(u32 *)(iVar2->field14_0xe * 0x6 + (int)*param_1) = *param_2;
  return;
}



void pass1_1030_877c(u16 *param_1)

{
  pass1_1030_8834(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_878c(astruct_172 *param_1)

{
  u16 *puVar1;
  u16 uVar2;
  u16 uVar3;
  u32 in_EDX;
  StructD *pSVar4;
  astruct_172 *iVar4;
  astruct_172 *uVar4;
  i32 lVar5;
  u32 uStack12;

  uVar4 = (astruct_172 *)((u32)param_1 >> 0x10);
  iVar4 = (astruct_172 *)param_1;
  if (iVar4->field2_0x4 == 0x0) {
    pSVar4 = (StructD *)(in_EDX & 0xffff0000);
    uVar2 = iVar4->field3_0x6;
  }
  else {
    uVar3 = iVar4->field2_0x4;
    puVar1 = &iVar4->field4_0x8;
    uVar2 = uVar3 + *puVar1;
    pSVar4 = (StructD *)(in_EDX & 0xffff0000 | (u32)CARRY2(uVar3,*puVar1));
  }
  if ((int)pSVar4 == 0x0) {
    if (*(i32 *)param_1 == 0x0) {
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar4);
        PTR_LOOP_1050_5f2e = (u8 *)pSVar4;
      }
      else {
      }
      uVar3 = fn_ptr_op_1000_1708(uVar2 * 0x6,0x0,0x1,(u16)PTR_LOOP_1050_5f2c,(u16)PTR_LOOP_1050_5f2e);
    }
    else {
      lVar5 = pass1_1000_0ed4(0x1,uVar2 * 0x6,0x0,(astruct_172 *)*(u32 *)param_1,
                              (astruct_172 *)((u32)*(u32 *)param_1 >> 0x10));
      PTR_LOOP_1050_5f2e = (u8 *)((u32)lVar5 >> 0x10);
      uVar3 = (u16)lVar5;
    }
    uStack12 = CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
    if (((u16)PTR_LOOP_1050_5f2e | uVar3) != 0x0) {
      iVar4->field2_0x4 = uVar2;
      *(u32 *)param_1 = uStack12;
      pass1_1030_8834((u16 *)((u32)param_1 & 0xffff | ZEXT24(uVar4) << 0x10));
    }
  }
  return;
}



void pass1_1030_8834(u16 *param_1)

{
  u32 uVar1;

  uVar1 = *(u32 *)((int)param_1 + 0x2);
  pass1_1000_4aea(*param_1,(u16)uVar1,(int)((u32)uVar1 >> 0x10),0x6,(uchar *)0x888e);
  return;
}



void pass1_1030_8854(u16 param_1,u16 param_2,u16 param_3)

{
  u32 uVar1;
  u32 local_c;
  u16 uStack8;

  uStack8 = param_3;
  local_c = 0x0;
  uVar1 = *(u32 *)(param_1 + 0x2);
  pass1_1000_49c6((u16)&local_c,(u16)&DAT_1050_1050,*_param_1,(u16)uVar1,(u16)((u32)uVar1 >> 0x10),0x6,
                  (uchar *)0x888e);
  return;
}



u16 pass1_1030_888e(u32 param_1,u32 param_2)

{
  i16 *piVar1;
  i16 iVar2;
  u16 uVar3;
  u16 uVar4;

  uVar3 = (u16)(param_1 >> 0x10);
  iVar2 = ((int)param_1 + 0x4);
  uVar4 = (u16)(param_2 >> 0x10);
  piVar1 = (i16 *)((int)param_2 + 0x4);
  if (*piVar1 != iVar2 && iVar2 <= *piVar1) {
    return 0xffff;
  }
  if (((int)param_2 + 0x4) < ((int)param_1 + 0x4)) {
    return 0x1;
  }
  return 0x0;
}



void pass1_1030_88ce(u16 *param_1,u32 param_2,u32 param_3)

{
  u32 uVar1;
  uchar *puVar2;
  u16 uVar3;
  u32 in_EDX;
  u16 uVar6;
  astruct_57 *paVar4;
  astruct_354 *iVar4;
  u16 uVar7;
  u32 uVar8;
  u16 *puStack38;
  i16 iStack34;
  u8 local_20 [0x2];
  i16 local_1e;
  i16 local_1c;
  u8 local_1a [0x6];
  u8 local_14 [0x6];
  u32 uStack14;
  u32 uStack10;
  i16 iStack6;
  u16 uStack4;
  astruct_57 *paVar5;

  uVar7 = (u16)((u32)param_1 >> 0x10);
  iVar4 = (astruct_354 *)param_1;
  *param_1 = 0x389a;
  iVar4->field2_0x2 = 0x1008;
  uVar6 = (u16)((u32)in_EDX >> 0x10);
  pass1_1030_84ae((u32)param_1 & 0xffff0000 | ZEXT24(&iVar4->field_0x4));
  iVar4->field32_0x24 = param_3;
  puStack38 = (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar4->field_0x28));
  pass1_1008_6c90((u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar4->field_0x28)));
  *(u32 *)&iVar4->field45_0x34 = 0x0;
  *param_1 = 0x8e38;
  iVar4->field2_0x2 = 0x1030;
  struct_1030_8544((astruct_355 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar4->field_0x4)),(astruct_356 *)param_2);
  uVar8 = pass1_1008_4772(iVar4->field17_0x12);
  uStack4 = (u16)(uVar8 >> 0x10);
  iStack6 = (int)uVar8;
  uStack10 = *(u32 *)(iStack6 + 0x4);
  uStack14 = *(u32 *)(iStack6 + 0x8);
  pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_14),0x0,(int)uStack14 - 0x1,(int)uStack10 - 0x1);
  pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_1a),0x0,0x0,0x0);
  pass1_1008_6d18(puStack38,(u16 *)CONCAT22(0x1050,local_14),(u16 *)CONCAT22(0x1050,local_1a));
  pass1_1008_6d64(puStack38,(u16 *)CONCAT22(0x1050,local_1a));
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,local_1a),(u16 *)CONCAT22(0x1050,local_20),
                  (u16 *)CONCAT22(0x1050,&local_1e),(u16 *)CONCAT22(0x1050,&local_1c));
  puVar2 = (uchar *)((u32)((long)local_1e * (long)local_1c) >> 0x10);
  uVar1 = (long)local_1e * (long)local_1c & 0xffff;
  iVar4->field45_0x34 = (int)uVar1;
  iVar4->field46_0x36 = puVar2;
  paVar4 = (astruct_57 *)CONCAT22(uVar6,puVar2);
  for (iStack34 = 0x0; iStack34 < 0x5; iStack34 += 0x1) {
    mem_op_1000_179c(0x10,paVar4);
    uVar3 = (u16)paVar4 | (u16)uVar1;
    paVar5 = (astruct_57 *)((u32)paVar4 & 0xffff0000 | (u32)uVar3);
    if (uVar3 == 0x0) {
      *(u32 *)(&iVar4[0x1].field_0x0 + iStack34 * 0x4) = 0x0;
    }
    else {
      pass1_1030_85be((astruct_172 *)(uVar1 & 0xffff | (long)paVar4 << 0x10),0x19,0x64);
      (&iVar4[0x1].field_0x0 + iStack34 * 0x4) = (int)uVar1;
      (&iVar4[0x1].field2_0x2)[iStack34 * 0x2] = (int)paVar5;
    }
    paVar4 = paVar5;
  }
  return;
}



void pass1_1030_8a2c(StructD *param_1)

{
  u16 uVar1;
  char *pcVar2;
  StructD *iVar3;
  u16 uVar3;
  i16 iStack4;

  uVar3 = (u16)((u32)param_1 >> 0x10);
  iVar3 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x8e38;
  iVar3->address_offset_field_0x2 = 0x1030;
  iStack4 = 0x0;
  do {
    pcVar2 = *(char **)(&iVar3->field_0x38 + iStack4 * 0x4);
    uVar1 = (&iVar3->field_0x3a + iStack4 * 0x4);
    if ((uVar1 | (u16)pcVar2) != 0x0) {
      pass1_1030_8604((StructD *)((u32)pcVar2 & 0xffff | (u32)uVar1 << 0x10));
      fn_ptr_1000_17ce(pcVar2);
    }
    iStack4 += 0x1;
  } while (iStack4 < 0x5);
  fn_ptr_1030_84d0((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->hfile_0x4));
  param_1->address_offset_field_0x0 = 0x389a;
  iVar3->address_offset_field_0x2 = 0x1008;
  return;
}



void pass1_1030_8aa0(u32 param_1,u32 param_2,u16 *param_3,u16 param_4)

{
  u16 uVar1;
  u32 local_12;
  u8 *puStack14;
  u32 uStack12;
  u8 local_8 [0x2];
  u8 local_6 [0x2];
  u8 local_4 [0x2];

  puStack14 = local_8;
  pass1_1008_3eb4((astruct_615 *)param_3,(u16 *)CONCAT13(0x10,CONCAT12(0x50,puStack14)),
                  (u16 *)CONCAT22(0x1050,local_6),(u16 *)CONCAT22(0x1050,local_4));
  bad_1030_8cd2();
  uStack12 = CONCAT22(param_4,puStack14);
  uVar1 = param_4 | (u16)puStack14;
  if (uVar1 != 0x0) {
    pass1_1030_8d9e(param_1);
    local_12 = param_2;
    pass1_1030_8660((u16)&local_12,uVar1,uStack12,(u32 *)CONCAT22(0x1050,&local_12),(u16)puStack14);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1030_8b00(u32 param_1,u16 *param_2,u16 *param_3)

{
  u32 *puVar1;
  i16 *piVar2;
  u16 uVar3;
  u32 local_2a;
  u32 uStack38;
  u32 uStack28;
  u32 *puStack18;
  u32 *puStack16;
  i16 *piStack14;
  i16 local_c;
  u8 local_a [0x4];
  u32 uStack6;

  uStack6 = 0x0;
  puVar1 = (u32 *)(local_a + 0x2);
  piVar2 = &local_c;
  pass1_1008_3eb4((astruct_615 *)param_2,(u16 *)CONCAT13(0x10,CONCAT12(0x50,piVar2)),
                  (u16 *)CONCAT22(0x1050,local_a),(u16 *)CONCAT22(0x1050,puVar1));
  bad_1030_8cd2();
  puStack16 = puVar1;
  piStack14 = piVar2;
  pass1_1030_8d9e(param_1);
  puStack18 = puVar1;
  pass1_1030_861a((u16)puVar1,(u16)piVar2,(u16)puStack16,(u16)piStack14,(u16)puVar1);
  uStack38 = *puVar1;
  uVar3 = ((int)puVar1 + 0x2);
  uStack38._3_1_ = (char)((u32)uStack38 >> 0x18);
  uStack6 = uStack38;
  if (uStack38._3_1_ == '\0') {
    puVar1 = &local_2a;
    uStack28 = uStack38;
    pass1_1030_8c66(param_1,local_c,(u8 *)local_a,(u16)((u32)local_a >> 0x10),(u32 *)CONCAT22(0x1050,puVar1),
                    uVar3);
    uStack6 = *puVar1;
    uVar3 = ((int)puVar1 + 0x2);
  }
  *param_3 = (u16)uStack6;
  ((int)param_3 + 0x2) = uVar3;
  return;
}



void pass1_1030_8bac(u32 param_1,u16 param_2)

{
  i16 iStack4;

  iStack4 = 0x0;
  do {
    pass1_1030_86ec(*(astruct_612 **)((int)param_1 + 0x38 + iStack4 * 0x4),param_2);
    iStack4 += 0x1;
  } while (iStack4 < 0x5);
  return;
}



void pass1_1030_8bdc(u32 param_1,u32 param_2,u16 *param_3)

{
  u8 *puVar1;
  u32 local_12;
  u8 *puStack14;
  i32 *plStack12;
  u8 local_8 [0x2];
  u8 local_6 [0x2];
  u8 local_4 [0x2];

  puStack14 = local_4;
  puVar1 = local_8;
  pass1_1008_3eb4((astruct_615 *)param_3,(u16 *)CONCAT13(0x10,CONCAT12(0x50,puVar1)),
                  (u16 *)CONCAT22(0x1050,local_6),(u16 *)CONCAT22(0x1050,puStack14));
  bad_1030_8cd2();
  plStack12 = (i32 *)CONCAT22(puVar1,puStack14);
  pass1_1030_8d9e(param_1);
  local_12 = param_2;
  pass1_1030_871e(plStack12,(u32 *)CONCAT22(0x1050,&local_12),(u16)puStack14);
  return;
}



void pass1_1030_8c38(u32 param_1)

{
  i16 iStack4;

  iStack4 = 0x0;
  do {
    pass1_1030_877c((u16*)((int)param_1 + 0x38 + iStack4 * 0x4));
    iStack4 += 0x1;
  } while (iStack4 < 0x5);
  return;
}



void pass1_1030_8c66(u32 param_1,i16 param_2,u8 *param_3,u16 param_4,u32 *param_5,u16 param_6)

{
  u8 bVar1;
  u16 uVar2;
  u32 uStack6;

  pass1_1008_4544(*(astruct_76 **)((int)param_1 + 0x12));
  bVar1 = *param_3;
  uVar2 = (u16)bVar1;
  uStack6 = (u32)(uVar2 + 0x1);
  if (0x0 < param_2) {
    if (uVar2 == 0x0) {
      uStack6 = 0x7;
    }
    else if (((bVar1 == 0x0) || (SBORROW2(uVar2,0x1))) || (0x1 < (int)(uVar2 - 0x1))) {
      uStack6 = 0x9;
    }
    else {
      uStack6 = 0x8;
    }
  }
  *param_5 = uStack6;
  return;
}



void bad_1030_8cd2(void)

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_8d08(u32 param_1,u16 param_2)

{
  i16 *piVar1;
  u32 uVar2;
  i16 iVar3;
  u16 uVar4;
  u16 in_register_0000000a;
  astruct_57 *paVar5;
  u16 uVar6;
  u32 uVar7;
  astruct_358 *paStack16;
  i16 iStack4;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  iStack4 = 0x0;
  while( true ) {
    uVar6 = (u16)(param_1 >> 0x10);
    piVar1 = (i16 *)((int)param_1 + 0x1e);
    if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
    iVar3 = iStack4 * 0x6;
    uVar2 = *(u32 *)((int)param_1 + 0x1a);
    ((int)uVar2 + iVar3 + 0x4) = 0x0;
    pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x500);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22((int)paVar5,iVar3));
    paStack16 = (astruct_358 *)CONCAT22((int)paVar5,iVar3);
    uVar7 = pass1_1028_e2e0(paVar5,_PTR_LOOP_1050_65e2,0x7);
    paVar5 = (astruct_57 *)((u32)paVar5 & 0xffff0000 | uVar7 >> 0x10);
    uVar4 = (u16)(uVar7 >> 0x10);
    pass1_1030_7e5a(uVar4,paStack16,uVar7 & 0xffff | (u32)uVar4 << 0x10);
    iStack4 += 0x1;
  }
  return;
}



void pass1_1030_8d9e(u32 param_1)

{
  u8 local_c [0x2];
  u8 local_a [0x2];
  u8 local_8 [0x6];

  pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_8));
  pass1_1008_6d64((u16 *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0x28)),(u16 *)CONCAT22(0x1050,local_8));
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,local_8),(u16 *)CONCAT22(0x1050,local_c),(char *)CONCAT22(0x1050,local_a));
  return;
}



StructD * pass1_1030_8e12(StructD *param_1,u8 param_2)

{
  pass1_1030_8a2c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 pass1_1030_8e3c(StructD *param_1,u32 param_2,u32 param_3,u16 param_4)

{
  u16 uVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  astruct_57 *paVar7;
  astruct_57 *paVar9;
  u32 *puVar10;
  u16 in_stack_0000fe8a;
  u16 in_stack_0000ffae;
  u16 in_stack_0000ffb4;
  u16 in_stack_0000ffb8;
  u16 uVar11;
  astruct_57 *paVar8;

  uVar1 = (u16)((u32)param_1 >> 0x10);
  paVar7 = (astruct_57 *)(param_1 & 0xffff0000 | (u32)param_1 & 0xffff);
  mem_op_1000_179c(0xc,paVar7);
  uVar4 = (u16)paVar7 | uVar1;
  paVar9 = (astruct_57 *)((u32)paVar7 & 0xffff0000);
  paVar8 = (astruct_57 *)((u32)paVar9 | (u32)uVar4);
  if (uVar4 == 0x0) {
    uVar2 = 0x0;
  }
  else {
    uVar2 = set_struct_1008_574a((astruct_57 *)CONCAT22((u16)paVar7,uVar1));
    paVar9 = paVar8;
  }
  uVar5 = SUB42(paVar9,0x0);
  if (param_3._3_1_ == '\x04') {
    puVar10 = mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(param_4,0x2f),in_stack_0000fe8a,
                              in_stack_0000ffae,in_stack_0000ffb4,in_stack_0000ffb8);
    uVar4 = (u16)((u32)puVar10 >> 0x10);
    uVar1 = ((int)puVar10 + 0x1e);
    uVar3 = uVar1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_3);
    uVar11 = (u16)(param_2 >> 0x10);
    uVar6 = uVar4;
    if ((int)uVar1 < 0x1) {
      pass1_1030_9296(uVar4,param_2,(u32 *)CONCAT22(uVar5,uVar2),CONCAT22(uVar4,uVar3));
      pass1_1030_951a(uVar6,param_2,(u32 *)CONCAT22(uVar5,uVar2),CONCAT22(uVar4,uVar3));
    }
    else {
      pass1_1030_9adc(uVar3,uVar4,(u16)param_2,uVar11,(u32 *)CONCAT22(uVar5,uVar2),CONCAT22(uVar4,uVar3));
      pass1_1030_9c1c(param_2,(u32 *)CONCAT22(uVar5,uVar2),CONCAT22(uVar4,uVar3));
    }
    pass1_1030_9d42(uVar6,(u16)param_2,uVar11,(u32 *)CONCAT22(uVar5,uVar2),CONCAT22(uVar4,uVar3));
  }
  return CONCAT22(uVar5,uVar2);
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_8f04(u32 param_1,u16 param_2,u16 param_3,u16 param_4,u32 param_5)

{
  u16 uVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;
  u32 uVar5;
  u16 uVar6;
  i16 iStack8;
  u32 uStack6;

  pass1_1038_53ba(param_5,0x1);
  if ((((param_2 != 0x0) || (0x1 < (u16)param_1)) &&
      ((pass1_1038_53ba(param_5,0x2), param_2 != 0x0 || (0x1 < (u16)param_1)))) &&
     ((pass1_1038_53ba(param_5,0x3), param_2 != 0x0 || (0x1 < (u16)param_1)))) {
    pass1_1038_53ba(param_5,0x4);
    uVar5 = (u32)param_2;
    if ((param_2 != 0x0) || (0x1 < (u16)param_1)) {
      empty_1038_540a();
      uStack6 = param_1 & 0xffff | uVar5 << 0x10;
      iStack8 = 0x0;
      do {
        uVar3 = (u16)uVar5;
        uVar2 = (u16)param_1;
        if (0x0 < (iStack8 * 0x2 + (int)_PTR_LOOP_1050_580e)) {
          empty_1038_540a();
          uVar6 = (u16)((u32)_PTR_LOOP_1050_580e >> 0x10);
          uVar1 = (iStack8 * 0x2 + (int)_PTR_LOOP_1050_580e);
          param_1 = (u32)uVar1;
          uVar4 = (int)uVar1 >> 0xf;
          uVar5 = (u32)uVar4;
          if ((uVar3 <= uVar4) && ((uVar3 < uVar4 || (uVar2 < uVar1)))) {
            if (0x1c < iStack8) {
              return;
            }
            uVar2 = (iStack8 * 0x2 + (int)_PTR_LOOP_1050_580e);
            param_1 = (u32)(int)uVar2;
            uVar5 = param_1 >> 0x10;
            if ((long)uStack6 < (long)param_1) {
              return;
            }
            uStack6 = CONCAT22(((int)(uStack6 >> 0x10) - ((int)uVar2 >> 0xf)) - (u16)((u16)uStack6 < uVar2),
                               (u16)uStack6 - uVar2);
          }
        }
        iStack8 += 0x1;
        if (0x24 < iStack8) {
          return;
        }
      } while( true );
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 pass1_1030_8fe4(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 *param_5,i32 param_6)

{
  i16 iVar1;
  u16 uVar2;
  u32 uVar3;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_5,param_6);
  uVar2 = param_2 | param_1;
  if (uVar2 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    if ((uVar2 | param_1) != 0x0) {
      uVar3 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar2,param_1),param_1,uVar2 | param_1);
      if ((uVar3 != 0x0) && ((iVar1 = ((int)uVar3 + 0xc), iVar1 == 0x5 || (iVar1 == 0x9)))) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_9048(u32 param_1,i16 param_2,u32 param_3)

{
  u32 uVar1;
  code **ppcVar2;
  BOOL16 BVar3;
  i16 iVar4;
  i16 *piVar5;
  u32 uVar6;
  u32 uVar7;
  u16 uVar8;
  u16 uVar9;
  u32 in_EDX;
  astruct_57 *paVar10;
  u16 uVar11;
  u32 *puVar12;
  u16 uVar13;
  u16 uVar14;
  u32 *puVar15;
  u16 *puVar16;
  u32 uVar17;
  u16 uVar18;
  u16 uVar19;
  u8 uVar20;
  u32 uStack36;
  u8 local_18 [0x2];
  i16 local_16;
  i16 local_14;
  i16 local_12;
  i16 iStack16;
  u32 uStack12;
  u16 uStack8;
  u16 uStack6;
  i16 iStack4;

  uVar11 = (u16)((u32)in_EDX >> 0x10);
  iStack4 = 0x8 - (u16)(param_2 == 0x0);
  puVar15 = pass1_1008_c6fa(_u16_1050_06e0,iStack4);
  uStack6 = (u16)((u32)puVar15 >> 0x10);
  paVar10 = (astruct_57 *)CONCAT22(uVar11,uStack6);
  uVar9 = (u16)puVar15;
  uStack8 = uVar9;
  pass1_1038_4e78(uVar9,paVar10,param_3,puVar15);
  uStack12 = (u32 *)CONCAT22((int)paVar10,uVar9);
  uVar14 = 0x1008;
  puVar16 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,&local_12));
  uVar7 = (u32)puVar16 >> 0x10;
  uVar1 = *(u32 *)((int)param_3 + 0x8);
  uVar13 = (u16)((u32)uStack12 >> 0x10);
  uVar11 = SUB42(uStack12,0x0);
  ppcVar2 = (code **)((int)*uStack12 + 0x10);
  uVar6 = uVar1;
  (**ppcVar2)(0x1008,uVar11,uVar13);
  uVar6 = uVar6 & 0xffff | uVar7 << 0x10;
  uStack36 = 0x0;
  while( true ) {
    uVar9 = (u16)uVar7;
    if (uVar6 <= uStack36) {
      if (uStack12 != NULL) {
        ppcVar2 = (code **)*uStack12;
        (**ppcVar2)(uVar14,(int)uStack12,(char)((u32)uStack12 >> 0x10),0x1,uVar11,uVar13,uStack12,uStack12);
      }
      return;
    }
    ppcVar2 = (code **)((int)*uStack12 + 0x4);
    uVar7 = uVar6;
    (**ppcVar2)();
    iVar4 = (int)uVar7;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7 & 0xffff | (u32)uVar9 << 0x10);
    uVar8 = uVar9;
    pass1_1008_3f62((u16 *)CONCAT22(0x1050,&local_12),(u16 *)CONCAT22(uVar9,iVar4 + 0xc));
    piVar5 = &local_12;
    uVar14 = 0x1008;
    pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,piVar5),(u16 *)CONCAT22(0x1050,local_18),
                    (u16 *)CONCAT22(0x1050,&local_16),(u16 *)CONCAT22(0x1050,&local_14));
    uVar17 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar9,iVar4),piVar5,uVar8);
    uVar7 = uVar17 >> 0x10;
    uVar9 = (u16)(uVar17 >> 0x10);
    iVar4 = ((int)uVar17 + 0xc);
    if (iVar4 - 0x7aU < 0x6) break;
LAB_1030_91fa:
    uStack36 += 0x1;
  }
  uVar14 = 0x1030;
  uVar18 = (u16)param_1;
  uVar19 = (u16)(param_1 >> 0x10);
  switch(iVar4) {
  default:
    iStack16 = local_16 + -0x1;
    BVar3 = pass1_1030_8fe4((u16)&local_12,uVar9,uVar18,uVar19,(u16 *)CONCAT22(0x1050,&local_12),uVar1);
    if (BVar3 != 0x0) goto LAB_1030_91cb;
    iStack16 = local_16 + 0x1;
    BVar3 = pass1_1030_8fe4((u16)&local_12,(u16)uVar7,uVar18,uVar19,(u16 *)CONCAT22(0x1050,&local_12),uVar1);
    if (BVar3 == 0x0) {
      iStack16 = local_16;
      local_12 = local_14 + -0x1;
      BVar3 = pass1_1030_8fe4((u16)&local_12,(u16)uVar7,uVar18,uVar19,(u16 *)CONCAT22(0x1050,&local_12),uVar1);
      goto joined_r0x1030911e;
    }
LAB_1030_9144:
    break;
  case 0x7b:
  case 0x7e:
    iStack16 = local_16 + -0x1;
    BVar3 = pass1_1030_8fe4((u16)&local_12,uVar9,uVar18,uVar19,(u16 *)CONCAT22(0x1050,&local_12),uVar1);
    if (BVar3 == 0x0) {
      iStack16 = local_16 + 0x1;
      goto LAB_1030_912c;
    }
    if (uStack12 == NULL) {
      return;
    }
    uVar14 = (u16)((u32)uStack12 >> 0x10);
    puVar12 = (u32 *)uStack12;
    uVar20 = (u8)((u32)uStack12 >> 0x10);
    goto LAB_1030_90e6;
  case 0x7c:
  case 0x7d:
    local_12 = local_14 + -0x1;
    BVar3 = pass1_1030_8fe4((u16)&local_12,uVar9,uVar18,uVar19,(u16 *)CONCAT22(0x1050,&local_12),uVar1);
joined_r0x1030911e:
    if (BVar3 == 0x0) {
      local_12 = local_14 + 0x1;
LAB_1030_912c:
      BVar3 = pass1_1030_8fe4((u16)&local_12,(u16)uVar7,uVar18,uVar19,(u16 *)CONCAT22(0x1050,&local_12),uVar1);
      if (BVar3 != 0x0) goto LAB_1030_9144;
      goto LAB_1030_91fa;
    }
LAB_1030_91cb:
  }
  puVar12 = (u32 *)uStack12;
  if ((uStack12 | (u16)puVar12) != 0x0) {
    uVar14 = (u16)((u32)uStack12 >> 0x10);
    uVar20 = (u8)((u32)uStack12 >> 0x10);
LAB_1030_90e6:
    ppcVar2 = (code **)*puVar12;
    (**ppcVar2)(0x1030,puVar12,uVar20,0x1,uVar11,uVar13,uStack12,uStack12);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_9296(u16 param_1,u32 param_2,u32 *param_3,u32 param_4)

{
  u16 uVar1;
  u8 *puVar2;
  astruct_99 *in_EAX;
  u32 uVar3;
  u16 uVar4;
  u16 uVar6;
  u16 in_register_0000000a;
  astruct_57 *paVar7;
  astruct_99 *iVar11;
  astruct_99 *pstruct99_10;
  astruct_99 *iVar9;
  u16 in_stack_0000fe6a;
  u16 in_stack_0000ff8e;
  u16 in_stack_0000ff94;
  u16 in_stack_0000ff98;
  u16 in_stack_0000ffc2;
  u8 local_3a [0xc];
  u32 uStack46;
  u32 uStack36;
  u32 uStack30;
  u16 uStack26;
  astruct_99 *pstruct99_18;
  u32 uStack14;
  u32 *puStack10;
  astruct_99 *paStack6;
  astruct_99 *uVar5;
  code **fn_ptr_1;

  pass1_1038_53ba(param_4,0x1);
  uVar4 = param_1 | (u16)in_EAX;
  if (uVar4 != 0x0) {
    uStack30 = _PTR_LOOP_1050_5768;
    uVar3 = _PTR_LOOP_1050_5768;
    pstruct99_18 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
    uVar6 = (u16)((u32)pstruct99_18 >> 0x10);
    in_EAX = (astruct_99 *)(uVar3 & 0xffff0000 | (u32)pstruct99_18 & 0xffff);
    uVar4 = uVar6 | (u16)((u32)pstruct99_18 & 0xffff);
    if (uVar4 == 0x0) {
      paStack6 = NULL;
    }
    else {
      iVar11 = (astruct_99 *)pstruct99_18;
      pstruct99_18->field0_0x0 = 0x389a;
      iVar11->field1_0x2 = 0x1008;
      &iVar11->field2_0x4 = 0x73;
      pstruct99_18->field0_0x0 = 0x9ec8;
      iVar11->field1_0x2 = 0x1030;
      in_EAX = pstruct99_18;
      paStack6 = pstruct99_18;
    }
    fn_ptr_1 = (code **)((int)*param_3 + 0x4);
    (**fn_ptr_1)(0x1000,param_3,(int)paStack6,(int)((u32)paStack6 >> 0x10));
  }
  pass1_1038_53ba(param_4,0x2);
  uVar4 |= (u16)in_EAX;
  if (uVar4 != 0x0) {
    uStack30 = _PTR_LOOP_1050_5768;
    uVar3 = _PTR_LOOP_1050_5768;
    pstruct99_18 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
    uVar6 = (u16)((u32)pstruct99_18 >> 0x10);
    in_EAX = (astruct_99 *)(uVar3 & 0xffff0000 | (u32)pstruct99_18 & 0xffff);
    uVar4 = uVar6 | (u16)((u32)pstruct99_18 & 0xffff);
    if (uVar4 == 0x0) {
      paStack6 = NULL;
    }
    else {
      pstruct99_10 = (astruct_99 *)pstruct99_18;
      pstruct99_18->field0_0x0 = 0x389a;
      pstruct99_10->field1_0x2 = 0x1008;
      &pstruct99_10->field2_0x4 = 0x74;
      pstruct99_18->field0_0x0 = 0x9ec8;
      pstruct99_10->field1_0x2 = 0x1030;
      in_EAX = pstruct99_18;
      paStack6 = pstruct99_18;
    }
    fn_ptr_1 = (code **)((int)*param_3 + 0x8);
    (**fn_ptr_1)(0x1000,param_3,(int)paStack6,(int)((u32)paStack6 >> 0x10));
  }
  pass1_1038_53ba(param_4,0x3);
  uVar4 |= (u16)in_EAX;
  paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,uVar4);
  if (uVar4 != 0x0) {
    uStack36 = _PTR_LOOP_1050_5768;
    uVar3 = _PTR_LOOP_1050_5768;
    pstruct99_18 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
    in_EAX = (astruct_99 *)(uVar3 & 0xffff0000 | (u32)pstruct99_18 & 0xffff);
    uVar4 = (u16)((u32)pstruct99_18 >> 0x10);
    uVar6 = uVar4 | (u16)((u32)pstruct99_18 & 0xffff);
    paVar7 = (astruct_57 *)((u32)paVar7 & 0xffff0000 | (u32)uVar6);
    if (uVar6 == 0x0) {
      paStack6 = NULL;
    }
    else {
      iVar9 = (astruct_99 *)pstruct99_18;
      pstruct99_18->field0_0x0 = 0x389a;
      iVar9->field1_0x2 = 0x1008;
      &iVar9->field2_0x4 = 0x75;
      pstruct99_18->field0_0x0 = 0x9ec8;
      iVar9->field1_0x2 = 0x1030;
      in_EAX = pstruct99_18;
      paStack6 = pstruct99_18;
    }
    fn_ptr_1 = (code **)((int)*param_3 + 0x8);
    (**fn_ptr_1)(0x1000,param_3,(int)paStack6,(int)((u32)paStack6 >> 0x10));
  }
  pass1_1030_8f04((u32)in_EAX,(u16)paVar7,(u16)param_2,(u16)(param_2 >> 0x10),param_4);
  if ((int)in_EAX != 0x0) {
    uStack36 = _PTR_LOOP_1050_5768;
    pstruct99_18 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
    uVar5 = (astruct_99 *)pstruct99_18;
    uVar4 = (u16)((u32)pstruct99_18 >> 0x10);
    paVar7 = (astruct_57 *)((u32)paVar7 & 0xffff0000 | (u32)(uVar4 | (u16)uVar5));
    if ((uVar4 | (u16)uVar5) == 0x0) {
      paStack6 = NULL;
    }
    else {
      pstruct99_18->field0_0x0 = 0x389a;
      uVar5->field1_0x2 = 0x1008;
      &uVar5->field2_0x4 = 0x37;
      pstruct99_18->field0_0x0 = 0x9ec8;
      uVar5->field1_0x2 = 0x1030;
      paStack6 = pstruct99_18;
    }
    fn_ptr_1 = (code **)((int)*param_3 + 0x8);
    (**fn_ptr_1)(0x1000,param_3,(int)paStack6,(int)((u32)paStack6 >> 0x10));
  }
  puStack10 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffc2,0x8),in_stack_0000fe6a,
                              in_stack_0000ff8e,in_stack_0000ff94,in_stack_0000ff98);
  uVar1 = (u16)((u32)puStack10 >> 0x10);
  uStack14 = *(u32 *)((int)puStack10 + 0xe);
  uVar4 = ((int)puStack10 + 0x10);
  uVar3 = (u32)uVar4;
  if ((uVar4 | (u16)uStack14) != 0x0) {
    pass1_1008_5784((char *)CONCAT22(0x1050,local_3a),uStack14 & 0xffff | (u32)uVar4 << 0x10);
    while( true ) {
      uVar4 = (u16)uVar3;
      puVar2 = local_3a;
      pass1_1008_5b12((char *)CONCAT22(0x1050,puVar2));
      uStack46 = CONCAT22(uVar4,puVar2);
      uVar3 = (u32)(uVar4 | (u16)puVar2);
      if ((uVar4 | (u16)puVar2) == 0x0) break;
      if (((puVar2 + 0x4) == 0x3e) || ((puVar2 + 0x4) == 0x41)) {
        uStack30 = _PTR_LOOP_1050_5768;
        pstruct99_18 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
        uVar6 = (u16)((u32)pstruct99_18 >> 0x10);
        uVar4 = (u16)pstruct99_18;
        uVar3 = (u32)(uVar6 | uVar4);
        if ((uVar6 | uVar4) == 0x0) {
          paStack6 = NULL;
        }
        else {
          uStack26 = ((int)uStack46 + 0x4);
          pstruct99_18->field0_0x0 = 0x389a;
          (uVar4 + 0x2) = 0x1008;
          (uVar4 + 0x4) = uStack26;
          pstruct99_18->field0_0x0 = 0x9ec8;
          (uVar4 + 0x2) = 0x1030;
          paStack6 = pstruct99_18;
        }
        fn_ptr_1 = (code **)((int)*param_3 + 0x8);
        (**fn_ptr_1)(0x1000,param_3,(int)paStack6,(int)((u32)paStack6 >> 0x10));
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_951a(u16 param_1,u32 param_2,u32 *param_3,u32 param_4)

{
  code **ppcVar1;
  u16 uVar6;
  u16 uVar7;
  u16 uVar8;
  u16 uVar9;
  u16 in_register_0000000a;
  astruct_57 *paVar10;
  i16 iVar11;
  u16 *puVar12;
  u16 unaff_SI;
  u16 uVar13;
  u16 uVar14;
  u8 uVar15;
  u32 *puVar16;
  u32 uVar17;
  u16 in_stack_0000fe48;
  u16 in_stack_0000ff6c;
  u16 in_stack_0000ff72;
  u16 in_stack_0000ff76;
  u8 uVar18;
  u8 uVar19;
  u8 uVar20;
  u32 *puStack76;
  u32 uStack70;
  u32 uStack62;
  astruct_99 *paStack58;
  u16 local_36;
  u16 uStack52;
  u32 uStack46;
  u16 uStack42;
  u16 uStack40;
  i16 iStack38;
  u32 *puStack36;
  u32 *puStack32;
  i16 iStack28;
  i16 iStack20;
  u32 uStack18;
  u32 uStack14;
  u32 *puStack10;
  astruct_99 *paStack6;
  astruct_99 *uVar2;
  astruct_99 *uVar3;
  astruct_99 *uVar4;
  astruct_99 *uVar5;

  paVar10 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  puStack10 = mixed_1010_20ba(paVar10,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x35),in_stack_0000fe48,
                              in_stack_0000ff6c,in_stack_0000ff72,in_stack_0000ff76);
  paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)puStack10 >> 0x10);
  uVar6 = (int)puStack10 + 0xa;
  uStack14 = (u32)puStack10 & 0xffff0000 | (u32)uVar6;
  pass1_1030_9048(param_2,0x0,param_4);
  uVar13 = (u16)((u32)param_3 >> 0x10);
  uVar20 = SUB41(param_3,0x0);
  if (uVar6 != 0x0) {
    iStack28 = 0x0;
    puStack32 = mixed_1010_20ba(paVar10,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x8),in_stack_0000fe48,
                                in_stack_0000ff6c,in_stack_0000ff72,in_stack_0000ff76);
    uVar14 = (u16)((u32)puStack32 >> 0x10);
    puStack36 = *(u32 **)((int)puStack32 + 0xe);
    uVar6 = ((int)puStack32 + 0x10);
    uVar17 = (u32)uVar6;
    if ((uVar6 | (u16)puStack36) != 0x0) {
      pass1_1008_5784((char *)CONCAT22(0x1050,&local_36),(u32)puStack36 & 0xffff | (u32)uVar6 << 0x10);
      while( true ) {
        uVar6 = (u16)uVar17;
        puVar12 = &local_36;
        pass1_1008_5b12((char *)CONCAT22(0x1050,puVar12));
        uStack46 = CONCAT22(uVar6,puVar12);
        uVar17 = (u32)(uVar6 | (u16)puVar12);
        if ((uVar6 | (u16)puVar12) == 0x0) break;
        if ((puVar12[0x2] != 0x3e) && (puVar12[0x2] != 0x41)) {
          paStack6 = pass1_1000_07fc((u32)_PTR_LOOP_1050_5768);
          uVar7 = (u16)((u32)paStack6 >> 0x10);
          uVar6 = (u16)paStack6;
          uVar17 = (u32)(uVar7 | uVar6);
          if ((uVar7 | uVar6) == 0x0) {
            paStack6 = NULL;
          }
          else {
            uVar14 = ((int)uStack46 + 0x4);
            paStack6->field0_0x0 = 0x389a;
            (uVar6 + 0x2) = 0x1008;
            (uVar6 + 0x4) = uVar14;
            paStack6->field0_0x0 = 0x9ec8;
            (uVar6 + 0x2) = 0x1030;
          }
          ppcVar1 = (code **)((int)*param_3 + 0x8);
          (**ppcVar1)(0x0,uVar20,uVar13,(int)paStack6,(int)((u32)paStack6 >> 0x10));
          if (((int)uStack46 + 0x4) == 0x13) {
            iStack28 = 0x1;
          }
        }
      }
    }
    for (iStack38 = 0xa; iStack38 < 0x41; iStack38 += 0x1) {
      if ((((((iStack38 != 0x37) && (iStack38 != 0x35)) && (iStack38 != 0x36)) &&
           ((iStack38 != 0x25 && (iStack38 != 0x26)))) &&
          ((iStack38 != 0x27 && (((iStack38 * 0x2 + (int)uStack14) != 0x0 && (iStack38 != 0x13)))))) &&
         ((iStack38 != 0x14 || (iStack28 == 0x0)))) {
        paStack6 = pass1_1000_07fc((u32)_PTR_LOOP_1050_5768);
        uVar7 = (u16)((u32)paStack6 >> 0x10);
        uVar6 = (u16)paStack6;
        if ((uVar7 | uVar6) == 0x0) {
          paStack6 = NULL;
        }
        else {
          paStack6->field0_0x0 = 0x389a;
          (uVar6 + 0x2) = 0x1008;
          (uVar6 + 0x4) = iStack38;
          paStack6->field0_0x0 = 0x9ec8;
          (uVar6 + 0x2) = 0x1030;
        }
        ppcVar1 = (code **)((int)*param_3 + 0x8);
        (**ppcVar1)(0x0,uVar20,uVar13,(int)paStack6,(int)((u32)paStack6 >> 0x10));
      }
    }
  }
  uVar14 = (u16)(uStack14 >> 0x10);
  if (((int)uStack14 + 0x6a) == 0x0) {
    if (((int)uStack14 + 0x6c) != 0x0) {
      paStack58 = pass1_1000_07fc((u32)_PTR_LOOP_1050_5768);
      uVar6 = (u16)((u32)paStack58 >> 0x10);
      puVar12 = (u16 *)paStack58;
      if ((uVar6 | (u16)puVar12) == 0x0) goto LAB_1030_973e;
      paStack58->field0_0x0 = 0x389a;
      puVar12[0x1] = 0x1008;
      puVar12[0x2] = 0x36;
      goto LAB_1030_9728;
    }
  }
  else {
    paStack58 = pass1_1000_07fc((u32)_PTR_LOOP_1050_5768);
    uVar6 = (u16)((u32)paStack58 >> 0x10);
    puVar12 = (u16 *)paStack58;
    if ((uVar6 | (u16)puVar12) == 0x0) {
LAB_1030_973e:
      paStack6 = NULL;
    }
    else {
      paStack58->field0_0x0 = 0x389a;
      puVar12[0x1] = 0x1008;
      puVar12[0x2] = 0x35;
LAB_1030_9728:
      *puVar12 = 0x9ec8;
      puVar12[0x1] = 0x1030;
      paStack6 = paStack58;
    }
    ppcVar1 = (code **)((int)*param_3 + 0x8);
    (**ppcVar1)(0x0,uVar20,uVar13,(int)paStack6,(int)((u32)paStack6 >> 0x10));
  }
  uVar14 = (u16)(uStack14 >> 0x10);
  iVar11 = (int)uStack14;
  if ((iVar11 + 0x4a) == 0x0) {
    if ((iVar11 + 0x4c) == 0x0) {
      if ((iVar11 + 0x4e) == 0x0) goto LAB_1030_97e8;
      paStack58 = pass1_1000_07fc((u32)_PTR_LOOP_1050_5768);
      uVar6 = (u16)((u32)paStack58 >> 0x10);
      puVar12 = (u16 *)paStack58;
      if ((uVar6 | (u16)puVar12) != 0x0) {
        paStack58->field0_0x0 = 0x389a;
        puVar12[0x1] = 0x1008;
        puVar12[0x2] = 0x27;
        goto LAB_1030_9879;
      }
    }
    else {
      paStack58 = pass1_1000_07fc((u32)_PTR_LOOP_1050_5768);
      uVar6 = (u16)((u32)paStack58 >> 0x10);
      puVar12 = (u16 *)paStack58;
      if ((uVar6 | (u16)puVar12) != 0x0) {
        paStack58->field0_0x0 = 0x389a;
        puVar12[0x1] = 0x1008;
        puVar12[0x2] = 0x26;
        goto LAB_1030_9879;
      }
    }
LAB_1030_97d0:
    paStack6 = NULL;
  }
  else {
    paStack58 = pass1_1000_07fc((u32)_PTR_LOOP_1050_5768);
    uVar6 = (u16)((u32)paStack58 >> 0x10);
    puVar12 = (u16 *)paStack58;
    if ((uVar6 | (u16)puVar12) == 0x0) goto LAB_1030_97d0;
    paStack58->field0_0x0 = 0x389a;
    puVar12[0x1] = 0x1008;
    puVar12[0x2] = 0x25;
LAB_1030_9879:
    *puVar12 = 0x9ec8;
    puVar12[0x1] = 0x1030;
    paStack6 = paStack58;
  }
  ppcVar1 = (code **)((int)*param_3 + 0x8);
  (**ppcVar1)(0x0,uVar20,uVar13,(int)paStack6,(int)((u32)paStack6 >> 0x10));
LAB_1030_97e8:
  uStack18 = (u32)puStack10 & 0xffff0000 | (u32)((int)puStack10 + 0x11e);
  if (((int)puStack10 + 0x138) != 0x0) {
    puVar16 = pass1_1008_c6fa(_u16_1050_06e0,0x4);
    uVar17 = (u32)puVar16 >> 0x10;
    uVar6 = (u16)puVar16;
    uVar15 = 0x38;
    pass1_1038_4d6e(uVar6,(uchar *)((u32)puVar16 >> 0x10),(astruct_691 *)param_4,puVar16);
    uVar14 = (u16)uVar17;
    puStack76 = (u32 *)CONCAT22(uVar14,uVar6);
    ppcVar1 = (code **)((int)*puStack76 + 0x10);
    uVar7 = uVar6;
    (**ppcVar1)((int)&u16_1050_1038,uVar6,uVar14);
    uStack70 = CONCAT22((int)uVar17,uVar7);
    for (uStack62 = 0x0; uVar7 = (u16)uVar17, uStack62 < uStack70; uStack62 += 0x1) {
      ppcVar1 = (code **)((int)*puStack76 + 0x4);
      uVar17 = uStack70;
      (**ppcVar1)(uVar15,(char)uVar6,uVar14,(int)uStack62,(int)(uStack62 >> 0x10));
      uVar8 = (u16)uVar17;
      iVar11 = 0xd;
      local_36 = uVar8;
      uStack52 = uVar7;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar17 & 0xffff | (u32)uVar7 << 0x10);
      uStack46 = CONCAT22(uVar7,uVar8);
      uVar17 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar7,uVar8),uVar8,uVar7);
      uVar7 = (u16)(uVar17 >> 0x10);
      uStack42 = (u16)uVar17;
      uVar15 = 0x28;
      uStack40 = uVar7;
      uVar9 = pass1_1028_6744(uVar17,iVar11);
      uVar17 = (u32)(uVar7 | uVar9);
      if ((uVar7 | uVar9) != 0x0) {
        puStack32 = _PTR_LOOP_1050_5768;
        paStack6 = pass1_1000_07fc((u32)_PTR_LOOP_1050_5768);
        uVar7 = (u16)((u32)paStack6 >> 0x10);
        uVar5 = (astruct_99 *)paStack6;
        if ((uVar7 | (u16)uVar5) == 0x0) {
          paStack6 = NULL;
        }
        else {
          paStack6->field0_0x0 = 0x389a;
          uVar5->field1_0x2 = 0x1008;
          &uVar5->field2_0x4 = 0x4c;
          paStack6->field0_0x0 = 0x9ec8;
          uVar5->field1_0x2 = 0x1030;
        }
        ppcVar1 = (code **)((int)*param_3 + 0x8);
        (**ppcVar1)(0x0,uVar20,uVar13,(int)paStack6,(int)((u32)paStack6 >> 0x10));
        puStack36 = _PTR_LOOP_1050_5768;
        paStack6 = pass1_1000_07fc((u32)_PTR_LOOP_1050_5768);
        uVar7 = (u16)((u32)paStack6 >> 0x10);
        uVar4 = (astruct_99 *)paStack6;
        if ((uVar7 | (u16)uVar4) == 0x0) {
          paStack6 = NULL;
        }
        else {
          paStack6->field0_0x0 = 0x389a;
          uVar4->field1_0x2 = 0x1008;
          &uVar4->field2_0x4 = 0x4d;
          paStack6->field0_0x0 = 0x9ec8;
          uVar4->field1_0x2 = 0x1030;
        }
        uVar18 = SUB41(paStack6,0x0);
        uVar19 = (u8)((u32)paStack6 >> 0x10);
        ppcVar1 = (code **)((int)*param_3 + 0x8);
        puVar16 = param_3;
        (**ppcVar1)();
        puStack36 = _PTR_LOOP_1050_5768;
        uVar15 = 0x0;
        paStack6 = pass1_1000_07fc((u32)_PTR_LOOP_1050_5768);
        uVar7 = (u16)((u32)paStack6 >> 0x10);
        uVar3 = (astruct_99 *)paStack6;
        if ((uVar7 | (u16)uVar3) == 0x0) {
          paStack6 = NULL;
        }
        else {
          paStack6->field0_0x0 = 0x389a;
          uVar3->field1_0x2 = 0x1008;
          &uVar3->field2_0x4 = 0x4e;
          paStack6->field0_0x0 = 0x9ec8;
          uVar3->field1_0x2 = 0x1030;
        }
        ppcVar1 = (code **)((int)*param_3 + 0x8);
        (**ppcVar1)(0x1000,param_3,paStack6,puVar16,uVar18,uVar19);
        break;
      }
    }
    if (puStack76 != NULL) {
      ppcVar1 = (code **)*puStack76;
      (**ppcVar1)(uVar15,uVar6,uVar14,0x1);
    }
  }
  for (iStack20 = 0x7a; iStack20 < 0x7d; iStack20 += 0x1) {
    if ((iStack20 * 0x2 + (int)uStack14) != 0x0) {
      paStack6 = pass1_1000_07fc((u32)_PTR_LOOP_1050_5768);
      uVar6 = (u16)((u32)paStack6 >> 0x10);
      uVar2 = (astruct_99 *)paStack6;
      if ((uVar6 | (u16)uVar2) == 0x0) {
        paStack6 = NULL;
      }
      else {
        paStack6->field0_0x0 = 0x389a;
        uVar2->field1_0x2 = 0x1008;
        &uVar2->field2_0x4 = iStack20;
        paStack6->field0_0x0 = 0x9ec8;
        uVar2->field1_0x2 = 0x1030;
      }
      ppcVar1 = (code **)((int)*param_3 + 0x8);
      (**ppcVar1)(0x0,uVar20,uVar13,(int)paStack6,(int)((u32)paStack6 >> 0x10));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_9adc(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u32 *param_5,u32 param_6)

{
  code **ppcVar1;
  astruct_99 *pstruct99_2;
  u16 uVar2;
  u16 extraout_DX;
  u16 extraout_DX_00;
  astruct_99 *pstruct99_7;
  astruct_99 *iVar6;
  astruct_99 *pstruct99_6;
  astruct_99 *pstruct99_3;

  pass1_1038_53ba(param_6,0x1);
  uVar2 = param_2 | param_1;
  if (uVar2 != 0x0) {
    pstruct99_6 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
    uVar2 = (u16)((u32)pstruct99_6 >> 0x10);
    pstruct99_2 = (astruct_99 *)((u32)pstruct99_6 & 0xffff);
    if ((uVar2 | (u16)pstruct99_2) == 0x0) {
      pstruct99_6 = NULL;
    }
    else {
      pstruct99_7 = (astruct_99 *)pstruct99_6;
      pstruct99_6->field0_0x0 = 0x389a;
      pstruct99_7->field1_0x2 = 0x1008;
      &pstruct99_7->field2_0x4 = 0x77;
      pstruct99_6->field0_0x0 = 0x9ec8;
      pstruct99_7->field1_0x2 = 0x1030;
      pstruct99_2 = pstruct99_6;
    }
    param_1 = (u16)pstruct99_2;
    ppcVar1 = (code **)((int)*param_5 + 0x4);
    (**ppcVar1)(0x1000,param_5,(int)pstruct99_6,(int)((u32)pstruct99_6 >> 0x10));
    uVar2 = extraout_DX;
  }
  pass1_1038_53ba(param_6,0x2);
  uVar2 |= param_1;
  if (uVar2 != 0x0) {
    pstruct99_6 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
    uVar2 = (u16)((u32)pstruct99_6 >> 0x10);
    pstruct99_2 = (astruct_99 *)((u32)pstruct99_6 & 0xffff);
    if ((uVar2 | (u16)pstruct99_2) == 0x0) {
      pstruct99_6 = NULL;
    }
    else {
      iVar6 = (astruct_99 *)pstruct99_6;
      pstruct99_6->field0_0x0 = 0x389a;
      iVar6->field1_0x2 = 0x1008;
      &iVar6->field2_0x4 = 0x78;
      pstruct99_6->field0_0x0 = 0x9ec8;
      iVar6->field1_0x2 = 0x1030;
      pstruct99_2 = pstruct99_6;
    }
    param_1 = (u16)pstruct99_2;
    ppcVar1 = (code **)((int)*param_5 + 0x8);
    (**ppcVar1)(0x1000,param_5,(int)pstruct99_6,(int)((u32)pstruct99_6 >> 0x10));
    uVar2 = extraout_DX_00;
  }
  pass1_1038_53ba(param_6,0x3);
  if ((uVar2 | param_1) != 0x0) {
    pstruct99_6 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
    uVar2 = (u16)((u32)pstruct99_6 >> 0x10);
    pstruct99_3 = (astruct_99 *)pstruct99_6;
    if ((uVar2 | (u16)pstruct99_3) == 0x0) {
      pstruct99_6 = NULL;
    }
    else {
      pstruct99_6->field0_0x0 = 0x389a;
      pstruct99_3->field1_0x2 = 0x1008;
      &pstruct99_3->field2_0x4 = 0x75;
      pstruct99_6->field0_0x0 = 0x9ec8;
      pstruct99_3->field1_0x2 = 0x1030;
    }
    ppcVar1 = (code **)((int)*param_5 + 0x8);
    (**ppcVar1)(0x1000,param_5,(int)pstruct99_6,(int)((u32)pstruct99_6 >> 0x10));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_9c1c(u32 param_1,u32 *param_2,u32 param_3)

{
  code **ppcVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;
  i16 iVar5;
  i16 iVar6;
  astruct_57 *in_EDX;
  u16 unaff_SI;
  u32 *puVar7;
  u16 in_stack_0000fe72;
  u16 in_stack_0000ff96;
  u16 in_stack_0000ff9c;
  u16 in_stack_0000ffa0;
  i16 iStack24;
  i16 iStack16;
  astruct_99 *paStack6;

  puVar7 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x35),in_stack_0000fe72,
                           in_stack_0000ff96,in_stack_0000ff9c,in_stack_0000ffa0);
  iVar5 = (int)puVar7 + 0xa;
  uVar4 = (u16)((u32)puVar7 >> 0x10);
  iVar6 = iVar5;
  pass1_1030_9048(param_1,0x1,param_3);
  if (iVar6 != 0x0) {
    for (iStack24 = 0x4f; iStack24 < 0x70; iStack24 += 0x1) {
      if ((iStack24 * 0x2 + iVar5) != 0x0) {
        paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
        uVar3 = (u16)((u32)paStack6 >> 0x10);
        uVar2 = (u16)paStack6;
        if ((uVar3 | uVar2) == 0x0) {
          paStack6 = NULL;
        }
        else {
          paStack6->field0_0x0 = 0x389a;
          (uVar2 + 0x2) = 0x1008;
          (uVar2 + 0x4) = iStack24;
          paStack6->field0_0x0 = 0x9ec8;
          (uVar2 + 0x2) = 0x1030;
        }
        ppcVar1 = (code **)((int)*param_2 + 0x8);
        (**ppcVar1)(0x1000,param_2,(int)paStack6,(int)((u32)paStack6 >> 0x10));
      }
    }
  }
  for (iStack16 = 0x7d; iStack16 < 0x80; iStack16 += 0x1) {
    if ((iStack16 * 0x2 + iVar5) != 0x0) {
      paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_5768);
      uVar3 = (u16)((u32)paStack6 >> 0x10);
      uVar2 = (u16)paStack6;
      if ((uVar3 | uVar2) == 0x0) {
        paStack6 = NULL;
      }
      else {
        paStack6->field0_0x0 = 0x389a;
        (uVar2 + 0x2) = 0x1008;
        (uVar2 + 0x4) = iStack16;
        paStack6->field0_0x0 = 0x9ec8;
        (uVar2 + 0x2) = 0x1030;
      }
      ppcVar1 = (code **)((int)*param_2 + 0x8);
      (**ppcVar1)(0x1000,param_2,(int)paStack6,(int)((u32)paStack6 >> 0x10));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_9d42(u16 param_1,u16 param_2,u16 param_3,u32 *param_4,u32 param_5)

{
  u32 *puVar1;
  StructD **ppSVar2;
  StructD *pSVar3;
  code **ppcVar4;
  u16 *puVar5;
  char *pcVar6;
  char *pcVar7;
  u16 extraout_DX;
  u16 uVar9;
  i16 iVar10;
  u16 uVar11;
  char string_a6 [0x4];
  u32 uStack158;
  i16 iStack154;
  StructD *local_98;
  u32 uStack12;
  u32 uStack8;
  i16 iStack4;
  u32 uVar8;
  astruct_117 *pstruct117_2;

  uVar11 = (u16)(param_5 >> 0x10);
  if (((int)param_5 + 0x206) == 0x0) {
    iStack4 = ((int)param_5 + 0x204);
    puVar5 = pass1_1000_4906((StructD *)CONCAT22(0x1050,&local_98),NULL,0x94);
    uVar8 = ZEXT24(puVar5);
    iStack154 = 0x11;
    do {
      empty_1038_540a();
      uVar11 = (u16)uVar8;
      (&local_98 + iStack154) = uVar11;
      ((int)&local_98 + iStack154 * 0x4 + 0x2) = param_1;
      iStack154 += 0x1;
    } while (iStack154 < 0x25);
    empty_1038_540a();
    uStack158 = CONCAT22(param_1,uVar11);
    pass1_1008_5784((char *)CONCAT22(0x1050,string_a6),(u32)param_4);
    pstruct117_2 = *(astruct_117 **)((int)_PTR_LOOP_1050_65e2 + 0x52);
    while( true ) {
      pcVar6 = string_a6;
      pass1_1008_5b12((char *)CONCAT22(0x1050,pcVar6));
      uVar9 = extraout_DX | (u16)pcVar6;
      if (uVar9 == 0x0) break;
      pcVar7 = pcVar6;
      pass1_1030_4bbe(uVar9,pstruct117_2,(pcVar6 + 0x4));
      if (iStack4 == 0x0) {
        for (iStack154 = 0x11; iStack154 < 0x25; iStack154 += 0x1) {
          iVar10 = iStack154 * 0x4;
          if ((*(i32 *)(pcVar7 + iVar10) != 0x0) &&
             (pSVar3 = (&local_98)[iStack154], ppSVar2 = (StructD **)(pcVar7 + iVar10),
             pSVar3 <= *ppSVar2 && *ppSVar2 != pSVar3)) {
            puVar1 = (u32 *)(pcVar7 + iVar10);
            if (uStack158 <= *puVar1 && *puVar1 != uStack158) goto LAB_1030_9e17;
            uStack158 -= *(i32 *)(pcVar7 + iVar10);
          }
        }
      }
      else {
        puVar1 = (u32 *)(pcVar7 + 0x8c);
        if ((uStack12 <= *puVar1 && *puVar1 != uStack12) ||
           (puVar1 = (u32 *)(pcVar7 + 0x90), uStack8 <= *puVar1 && *puVar1 != uStack8)) {
LAB_1030_9e17:
          ppcVar4 = (code **)((int)*param_4 + 0xc);
          (**ppcVar4)(0x1008,param_4,pcVar6,extraout_DX);
        }
      }
    }
  }
  return;
}



u16 * pass1_1030_9e9c(u16 *param_1,u8 param_2)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a((astruct_611 *)param_1);
  }
  return param_1;
}



void pass1_1030_9ecc(u32 *param_1,astruct_424 *param_2)

{
  u16 uVar1;

  uVar1 = (u16)((u32)param_1 >> 0x10);
  *param_1 = 0x0;
  *(astruct_424 **)((int)param_1 + 0x4) = param_2;
  ((int)param_1 + 0x8) = 0x0;
  return;
}



u16 pass1_1030_9ef2(u32 *param_1)

{
  i16 iVar1;
  u16 uVar2;
  u16 in_AX;
  u16 in_DX;
  u32 uVar3;

  if (*param_1 != 0x0) {
    uVar3 = struct_op_1030_73a8((astruct_419 *)*param_1,in_AX,in_DX);
    uVar2 = (u16)(uVar3 >> 0x10);
    iVar1 = ((int)uVar3 + 0xc);
    if (((iVar1 != 0x5) && (iVar1 != 0x9)) && (((int)uVar3 + 0x12) < 0x5)) {
      return 0x0;
    }
    pass1_1030_9f64(param_1);
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_9f40(uchar param_1,u32 param_2,u16 param_3)

{
  u16 uVar1;

  uVar1 = pass1_1008_c646((u16)_u16_1050_06e0,CONCAT22(param_3,(int)((u32)_u16_1050_06e0 >> 0x10)));
  ((int)param_2 + 0x8) = uVar1;
  pass1_1030_9f7a((u16 *)param_2,uVar1);
  return;
}



void pass1_1030_9f64(u32 *param_1)

{
  ((int)param_1 + 0x8) = 0x0;
  *param_1 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_9f7a(u16 *param_1,u16 param_2)

{
  u32 uVar1;
  BOOL16 BVar2;
  u32 *puVar3;
  u16 uVar4;
  u16 extraout_DX;
  u16 uVar5;
  u16 uVar6;
  u16 *puVar7;
  u8 local_130 [0x120];
  astruct_15 *paStack16;
  u32 uStack12;
  u32 local_8;
  i16 iStack4;

  puVar7 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,&local_8));
  uVar4 = (u16)((u32)puVar7 >> 0x10);
  BVar2 = pass1_1008_c6ae(_u16_1050_06e0,param_2,0x28);
  if (BVar2 != 0x0) {
    iStack4 = 0x1;
  }
  puVar3 = &local_8;
  pass1_1030_a278((u16)puVar3,uVar4,param_1,(u16 *)CONCAT22(0x1050,puVar3));
  if (puVar3 != NULL) {
    uVar6 = (u16)((u32)param_1 >> 0x10);
    uVar5 = (u16)param_1;
    uVar1 = *(u32 *)(uVar5 + 0x4);
    uStack12 = *(u32 *)((int)uVar1 + 0x8);
    uVar1 = *(u32 *)(uVar5 + 0x4);
    struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_130),0x0,0x0,param_2,&local_8,(u16)&DAT_1050_1050,
                        *(u32 *)((int)uVar1 + 0x4),uStack12);
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_130));
    pass1_1028_b58e(paStack16);
    *param_1 = (u16)paStack16;
    (uVar5 + 0x2) = extraout_DX;
    if (0x0 < iStack4) {
      pass1_1030_a044(extraout_DX,uVar5,uVar6,(u16 *)CONCAT22(0x1050,&local_8),uStack12);
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_a044(u16 param_1,u16 param_2,u16 param_3,u16 *param_4,u32 param_5)

{
  code **ppcVar1;
  u16 *puVar2;
  u8 *puVar3;
  i16 iVar4;
  u32 uVar5;
  u16 uVar6;
  u16 extraout_DX;
  u16 uVar7;
  u32 *puVar8;
  u16 uVar9;
  u16 uVar10;
  u16 uVar11;
  u16 local_17e;
  u16 uStack380;
  i16 iStack90;
  u32 *puStack78;
  u16 uStack70;
  i16 iStack68;
  u32 uStack66;
  u32 *puStack62;
  u8 local_3a [0xc];
  u32 local_2e;
  u16 uStack42;
  i16 iStack40;
  u16 uStack38;
  i16 local_24;
  i16 local_22;
  u32 uStack32;
  u32 uStack28;
  u32 uStack24;
  u16 *puStack20;
  u16 uStack18;
  i16 iStack16;
  i16 iStack14;
  u32 uStack12;
  u16 local_8;
  i16 local_6;
  i16 local_4;

  puVar2 = &local_8;
  pass1_1008_3eb4((astruct_615 *)param_4,(u16 *)CONCAT22(0x1050,puVar2),(u16 *)CONCAT22(0x1050,&local_6),
                  (u16 *)CONCAT22(0x1050,&local_4));
  pass1_1030_627e((u16)puVar2,param_1,(u32)_PTR_LOOP_1050_5740,param_4,param_5);
  puStack20 = puVar2;
  uStack18 = param_1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_1,puVar2));
  uStack24 = CONCAT22(param_1,puVar2);
  uStack28 = *(u32 *)(puVar2 + 0x17);
  uVar5 = *(u32 *)((int)uStack28 + 0x4);
  uStack32 = uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
  iStack40 = (int)uVar5;
  uStack38 = param_1;
  puVar8 = (u32 *)pass1_1030_5b5c(iStack40,param_1);
  uVar6 = (u16)((u32)puVar8 >> 0x10);
  local_2e = *puVar8;
  uStack42 = ((int)puVar8 + 0x4);
  puStack78 = &local_2e;
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_2e),(u16 *)CONCAT22(0x1050,&local_24),(char *)CONCAT22(0x1050,&local_22)
                 );
  iStack14 = local_4 + 0x1;
  uStack12 = CONCAT22(local_4 + -0x1,local_6 - 0x1U);
  iStack16 = local_6 + 0x1;
  if (local_4 + -0x1 < 0x0) {
    uStack12 = (u32)(local_6 - 0x1U);
  }
  if (local_22 <= iStack14) {
    iStack14 = local_22 + -0x1;
  }
  if ((int)(u16)uStack12 < 0x0) {
    uStack12 &= 0xffff0000;
  }
  if (local_24 <= iStack16) {
    iStack16 = local_24 + -0x1;
  }
  pass1_1008_6c90((u16 *)CONCAT22(0x1050,local_3a));
  uVar7 = 0x1008;
  pass1_1008_6cec((u16 *)CONCAT22(0x1050,local_3a),local_8,CONCAT22(iStack14,iStack16),local_8,uStack12);
  puVar3 = local_3a;
  pass1_1030_6522(_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar3),param_5);
  puStack62 = (u32 *)CONCAT22(uVar6,puVar3);
  if ((uVar6 | (u16)puVar3) != 0x0) {
    uStack66 = 0x0;
    iStack68 = 0x0;
    for (uStack70 = (u16)uStack12; (int)uStack70 <= iStack16; uStack70 += 0x1) {
      for (puStack78 = (u32 *)uStack12; (int)puStack78 <= iStack14;
          puStack78 = (u32 *)((int)puStack78 + 0x1)) {
        ppcVar1 = (code **)((int)*puStack62 + 0x4);
        iVar4 = iStack68;
        iStack68 = iStack68 + 0x1;
        (**ppcVar1)(uVar7,(int)puStack62,(int)((u32)puStack62 >> 0x10));
        uStack66 = CONCAT22(extraout_DX,iVar4);
        uStack66._3_1_ = (char)((u16)extraout_DX >> 0x8);
        if (uStack66._3_1_ == '\0') {
          iStack90 = iVar4;
          if (iVar4 == 0x7) {
            pass1_1008_3e76(param_4,local_8,uStack70,(u16)puStack78);
            uVar10 = (u16)uStack32;
            uVar11 = (u16)((u32)uStack32 >> 0x10);
            uVar9 = 0x6;
          }
          else if (iVar4 == 0x8) {
            pass1_1008_3e76(param_4,local_8,uStack70,(u16)puStack78);
            uVar10 = (u16)uStack32;
            uVar11 = (u16)((u32)uStack32 >> 0x10);
            uVar9 = 0x7;
          }
          else {
            if (iVar4 != 0x9) goto LAB_1030_a1d0;
            pass1_1008_3e76(param_4,local_8,uStack70,(u16)puStack78);
            uVar10 = (u16)uStack32;
            uVar11 = (u16)((u32)uStack32 >> 0x10);
            uVar9 = 0x8;
          }
          uVar7 = 0x1028;
          struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,&local_17e),0x0,0x0,uVar9,(u32 *)param_4,
                              (u16)((u32)param_4 >> 0x10),CONCAT22(uVar11,uVar10),param_5);
          fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_17e));
          local_17e = 0x389a;
          uStack380 = 0x1008;
        }
LAB_1030_a1d0:
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_a278(u16 param_1,i16 param_2,u16 *param_3,u16 *param_4)

{
  i16 iVar1;
  u32 uVar2;
  u16 extraout_DX;
  u32 *puVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  u8 local_134 [0x120];
  astruct_15 *paStack20;
  u32 uStack16;
  astruct_15 *paStack12;
  u16 uStack6;
  u16 uStack4;

  uStack4 = 0x1;
  pass1_1030_a39a((u32)param_3,param_4);
  if (param_1 != 0x0) {
    return;
  }
  uStack6 = param_1;
  pass1_1030_a3ae((u32)param_3,param_4);
  puVar3 = (u32 *)param_4;
  uVar5 = (u16)((u32)param_4 >> 0x10);
  if (param_1 == 0x0) {
    pass1_1030_a57e(0x0,param_2,(u32)param_3,param_4);
    if (param_1 == 0x0) {
      pass1_1030_a844(0x0,param_2,(astruct_426 *)param_3,param_4);
      if (param_1 == 0x0) {
        uStack4 = 0x0;
        goto LAB_1030_a305;
      }
      iVar1 = (puVar3 + 0x1);
    }
    else {
      iVar1 = (puVar3 + 0x1);
    }
    if (iVar1 < 0x1) {
      uStack6 = 0x73;
    }
    else {
      uStack6 = 0x77;
    }
  }
  else if ((puVar3 + 0x1) < 0x1) {
    uStack6 = 0x7a;
  }
  else {
    uStack6 = 0x7f;
  }
LAB_1030_a305:
  if (uStack6 != 0x0) {
    uVar6 = (u16)((u32)param_3 >> 0x10);
    uVar4 = (u16)param_3;
    uVar2 = *(u32 *)(uVar4 + 0x4);
    uStack16 = *(u32 *)((int)uVar2 + 0x8);
    uVar2 = *(u32 *)(uVar4 + 0x4);
    struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_134),0x0,0x0,uStack6,puVar3,uVar5,
                        *(u32 *)((int)uVar2 + 0x4),uStack16);
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_134));
    paStack12 = paStack20;
    pass1_1028_b58e(paStack20);
    *param_3 = (u16)paStack20;
    (uVar4 + 0x2) = extraout_DX;
    if (0x0 < (puVar3 + 0x1)) {
      pass1_1030_a044(extraout_DX,uVar4,uVar6,(u16 *)((u32)param_4 & 0xffff | (u32)uVar5 << 0x10),uStack16);
    }
  }
  return;
}



void pass1_1030_a39a(u32 param_1,u16 *param_2)

{
  pass1_1030_aa18(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_a3ae(u32 param_1,u16 *param_2)

{
  code **ppcVar1;
  u32 uVar2;
  u16 uVar3;
  u16 uVar4;
  BOOL16 BVar5;
  u32 uVar6;
  u16 uVar7;
  u32 in_EDX;
  u16 uVar10;
  astruct_57 *paVar8;
  u32 uVar9;
  u16 uVar11;
  i16 iVar12;
  u16 uVar13;
  u16 uVar14;
  u32 *puVar15;
  u16 *puVar16;
  u16 uVar17;
  u32 uStack44;
  i16 local_28;
  i16 local_26;
  i16 local_24;
  u8 local_22 [0x6];
  i16 local_1c;
  i16 iStack26;
  i32 lStack22;
  u32 uStack18;
  u32 *puStack14;
  u16 uStack10;
  u16 uStack8;
  i16 iStack6;
  u16 uStack4;

  uVar10 = (u16)((u32)in_EDX >> 0x10);
  uStack4 = 0x0;
  iStack6 = ((int)param_2 + 0x4);
  puVar15 = pass1_1008_c6fa(_u16_1050_06e0,0x45);
  uStack8 = (u16)((u32)puVar15 >> 0x10);
  paVar8 = (astruct_57 *)CONCAT22(uVar10,uStack8);
  uVar3 = (u16)puVar15;
  uVar13 = (u16)(param_1 >> 0x10);
  uVar11 = (u16)param_1;
  uStack10 = uVar3;
  pass1_1038_4e78(uVar3,paVar8,*(u32 *)(uVar11 + 0x4),puVar15);
  uVar14 = SUB42(paVar8,0x0);
  puStack14 = (u32 *)CONCAT22(uVar14,uVar3);
  ppcVar1 = (code **)((int)*puStack14 + 0x10);
  uVar17 = uVar3;
  uVar10 = uVar14;
  (**ppcVar1)((int)&u16_1050_1038,uVar3,uVar14);
  uStack18 = CONCAT22(uVar14,uVar3);
  uVar2 = *(u32 *)(uVar11 + 0x4);
  lStack22 = *(i32 *)((int)uVar2 + 0x8);
  pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,&local_1c));
  puVar16 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_22));
  uVar9 = (u32)puVar16 >> 0x10;
  uStack44 = 0x0;
  do {
    uVar3 = (u16)uVar9;
    if (uStack18 <= uStack44) {
LAB_1030_a4e7:
      if (puStack14 != NULL) {
        ppcVar1 = (code **)*puStack14;
        (**ppcVar1)(0x1008,(int)puStack14,(char)((u32)puStack14 >> 0x10),0x1,uVar17,uVar10,puStack14,puStack14);
      }
      return;
    }
    uVar6 = uStack18;
    pass1_1030_1d58((u32)puStack14);
    uVar7 = uVar3 | (u16)uVar6;
    uVar9 = (u32)uVar7;
    if (uVar7 != 0x0) {
      uVar9 = (u32)uVar3;
      pass1_1008_3f62((u16 *)CONCAT22(0x1050,&local_1c),(u16 *)CONCAT22(uVar3,(u16)uVar6 + 0xc));
      pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_1c),(u16 *)CONCAT22(0x1050,&local_28),
                      (u16 *)CONCAT22(0x1050,&local_26),(u16 *)CONCAT22(0x1050,&local_24));
      if ((local_28 == iStack6) &&
         (uVar2 = *(u32 *)(uVar11 + 0x4), uVar14 = (u16)((u32)uVar2 >> 0x10), iVar12 = (int)uVar2,
         uVar2 = *(u32 *)(iVar12 + 0x4),
         uVar4 = pass1_1030_addc((int)&local_1c,(u16)uVar9,uVar11,uVar13,(u16 *)CONCAT22(0x1050,&local_1c),
                                 (u16)uVar2,(u16)((u32)uVar2 >> 0x10),*(u32 *)(iVar12 + 0x8)), uVar4 != 0x0))
      {
        pass1_1008_3f62((u16 *)CONCAT22(0x1050,local_22),(u16 *)CONCAT22(0x1050,&local_1c));
        iStack26 = local_26 + -0x1;
        BVar5 = pass1_1030_ad22(&local_1c,(int)uVar9,uVar11,uVar13,(u16 *)CONCAT22(0x1050,&local_1c),lStack22);
        if (BVar5 == 0x0) {
          iStack26 = local_26 + 0x1;
          BVar5 = pass1_1030_ad22(&local_1c,(int)uVar9,uVar11,uVar13,(u16 *)CONCAT22(0x1050,&local_1c),lStack22);
          if (BVar5 == 0x0) {
            iStack26 = local_26;
            local_1c = local_24 + -0x1;
            BVar5 = pass1_1030_ad22(&local_1c,(int)uVar9,uVar11,uVar13,(u16 *)CONCAT22(0x1050,&local_1c),lStack22);
            if (BVar5 == 0x0) {
              local_1c = local_24 + 0x1;
              BVar5 = pass1_1030_ad22(&local_1c,(int)uVar9,uVar11,uVar13,(u16 *)CONCAT22(0x1050,&local_1c),lStack22);
              if (BVar5 == 0x0) goto LAB_1030_a45b;
            }
          }
        }
        pass1_1008_3f62(param_2,(u16 *)CONCAT22(0x1050,local_22));
        uStack4 = 0x1;
        goto LAB_1030_a4e7;
      }
    }
LAB_1030_a45b:
    uStack44 += 0x1;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_a57e(i16 param_1,i16 param_2,u32 param_3,u16 *param_4)

{
  u32 uVar1;
  code **ppcVar2;
  u32 uVar3;
  u16 uVar4;
  u16 uVar5;
  i16 *piVar6;
  u32 uVar7;
  u16 uVar8;
  u16 in_register_0000000a;
  astruct_57 *paVar9;
  u32 uVar10;
  u16 uVar11;
  u16 uVar12;
  i16 iVar13;
  u32 *puVar14;
  u16 uVar15;
  u16 uVar16;
  u16 uVar17;
  u16 uVar18;
  u32 *puVar19;
  u16 *puVar20;
  u32 uVar21;
  u8 uVar22;
  u32 uStack40;
  u8 local_1c [0x2];
  i16 local_1a;
  i16 local_18;
  i16 local_16;
  i16 iStack20;
  u32 uStack16;
  u16 uStack12;
  u16 uStack10;
  i16 iStack8;
  i16 iStack6;
  u16 uStack4;

  uStack4 = 0x0;
  uVar15 = (u16)(param_3 >> 0x10);
  uVar11 = (u16)param_3;
  pass1_1038_53ba(*(u32 *)(uVar11 + 0x4),0x1);
  if ((param_2 != 0x0) || (param_1 != 0x0)) {
    iStack6 = ((int)param_4 + 0x4);
    iStack8 = 0x8 - (u16)(iStack6 == 0x0);
    puVar19 = pass1_1008_c6fa(_u16_1050_06e0,iStack8);
    uStack10 = (u16)((u32)puVar19 >> 0x10);
    paVar9 = (astruct_57 *)CONCAT22(in_register_0000000a,uStack10);
    uVar4 = (u16)puVar19;
    uStack12 = uVar4;
    pass1_1038_4e78(uVar4,paVar9,*(u32 *)(uVar11 + 0x4),puVar19);
    uStack16 = (u32 *)CONCAT22((int)paVar9,uVar4);
    uVar18 = 0x1008;
    puVar20 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,&local_16));
    uVar10 = (u32)puVar20 >> 0x10;
    uVar3 = *(u32 *)(uVar11 + 0x4);
    uVar1 = *(u32 *)((int)uVar3 + 0x8);
    uVar16 = (u16)((u32)uStack16 >> 0x10);
    uVar12 = SUB42(uStack16,0x0);
    ppcVar2 = (code **)((int)*uStack16 + 0x10);
    uVar7 = uVar1;
    (**ppcVar2)(0x1008,uVar12,uVar16);
    uVar7 = uVar7 & 0xffff | uVar10 << 0x10;
    for (uStack40 = 0x0; uVar4 = (u16)uVar10, uStack40 < uVar7; uStack40 += 0x1) {
      uVar21 = uVar7;
      pass1_1030_1d58((u32)uStack16);
      uVar8 = uVar4 | (u16)uVar21;
      uVar10 = (u32)uVar8;
      if (uVar8 != 0x0) {
        uVar10 = (u32)uVar4;
        pass1_1008_3f62((u16 *)CONCAT22(0x1050,&local_16),(u16 *)CONCAT22(uVar4,(u16)uVar21 + 0xc));
        uVar18 = 0x1008;
        pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_16),(u16 *)CONCAT22(0x1050,local_1c),
                        (u16 *)CONCAT22(0x1050,&local_1a),(u16 *)CONCAT22(0x1050,&local_18));
        uVar3 = *(u32 *)(uVar11 + 0x4);
        uVar17 = (u16)((u32)uVar3 >> 0x10);
        iVar13 = (int)uVar3;
        uVar3 = *(u32 *)(iVar13 + 0x4);
        uVar5 = pass1_1030_addc((int)&local_16,(u16)uVar10,uVar11,uVar15,(u16 *)CONCAT22(0x1050,&local_16),
                                (u16)uVar3,(u16)((u32)uVar3 >> 0x10),*(u32 *)(iVar13 + 0x8));
        if (uVar5 == 0x0) goto LAB_1030_a660;
        uVar21 = struct_op_1030_73a8((astruct_419 *)(uVar21 & 0xffff | (u32)uVar4 << 0x10),uVar5,(int)uVar10);
        uVar10 = uVar21 >> 0x10;
        uVar5 = (u16)(uVar21 >> 0x10);
        iVar13 = ((int)uVar21 + 0xc);
        if (0x5 < iVar13 - 0x7aU) goto LAB_1030_a660;
        uVar18 = 0x1030;
        switch(iVar13) {
        default:
          iStack20 = local_1a + -0x1;
          piVar6 = &local_16;
          pass1_1030_ad86(uVar5,uVar11,uVar15,(u16 *)CONCAT22(0x1050,piVar6),uVar1);
          if (piVar6 != NULL) goto LAB_1030_a7df;
          iStack20 = local_1a + 0x1;
          piVar6 = &local_16;
          pass1_1030_ad86((u16)uVar10,uVar11,uVar15,(u16 *)CONCAT22(0x1050,piVar6),uVar1);
          if (piVar6 == NULL) {
            iStack20 = local_1a;
            local_16 = local_18 + -0x1;
            piVar6 = &local_16;
            pass1_1030_ad86((u16)uVar10,uVar11,uVar15,(u16 *)CONCAT22(0x1050,piVar6),uVar1);
            goto joined_r0x1030a722;
          }
LAB_1030_a748:
          pass1_1008_3f62(param_4,(u16 *)CONCAT22(0x1050,&local_16));
          break;
        case 0x7b:
        case 0x7e:
          iStack20 = local_1a + -0x1;
          piVar6 = &local_16;
          pass1_1030_ad86(uVar5,uVar11,uVar15,(u16 *)CONCAT22(0x1050,piVar6),uVar1);
          if (piVar6 == NULL) {
            iStack20 = local_1a + 0x1;
            goto LAB_1030_a730;
          }
          pass1_1008_3f62(param_4,(u16 *)CONCAT22(0x1050,&local_16));
          if (uStack16 == NULL) {
            return;
          }
          uVar18 = (u16)((u32)uStack16 >> 0x10);
          puVar14 = (u32 *)uStack16;
          uVar22 = (u8)((u32)uStack16 >> 0x10);
          goto LAB_1030_a6ea;
        case 0x7c:
        case 0x7d:
          local_16 = local_18 + -0x1;
          piVar6 = &local_16;
          pass1_1030_ad86(uVar5,uVar11,uVar15,(u16 *)CONCAT22(0x1050,piVar6),uVar1);
joined_r0x1030a722:
          if (piVar6 == NULL) {
            local_16 = local_18 + 0x1;
LAB_1030_a730:
            piVar6 = &local_16;
            pass1_1030_ad86((u16)uVar10,uVar11,uVar15,(u16 *)CONCAT22(0x1050,piVar6),uVar1);
            if (piVar6 != NULL) goto LAB_1030_a748;
            goto LAB_1030_a660;
          }
LAB_1030_a7df:
          pass1_1008_3f62(param_4,(u16 *)CONCAT22(0x1050,&local_16));
        }
        puVar14 = (u32 *)uStack16;
        if ((uStack16 | (u16)puVar14) != 0x0) {
          uVar18 = (u16)((u32)uStack16 >> 0x10);
          uVar22 = (u8)((u32)uStack16 >> 0x10);
LAB_1030_a6ea:
          ppcVar2 = (code **)*puVar14;
          (**ppcVar2)(0x1008,puVar14,uVar22,0x1,uVar12,uVar16,uStack16,uStack16);
        }
        return;
      }
LAB_1030_a660:
    }
    if (uStack16 != NULL) {
      ppcVar2 = (code **)*uStack16;
      (**ppcVar2)(uVar18,(int)uStack16,(char)((u32)uStack16 >> 0x10),0x1,uVar12,uVar16,uStack16,uStack16);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_a844(i16 param_1,i16 param_2,astruct_426 *param_3,u16 *param_4)

{
  i16 iVar1;
  u32 uVar2;
  code **ppcVar3;
  u16 uVar4;
  u16 uVar5;
  i16 *piVar6;
  u32 *puVar7;
  u16 extraout_DX;
  u16 uVar9;
  u16 uVar10;
  astruct_426 *uVar8;
  astruct_427 *iVar8;
  i16 iVar11;
  u16 uVar12;
  u16 uVar13;
  u16 *puVar14;
  u32 uVar15;
  u32 uStack34;
  i16 local_1c;
  i16 local_1a;
  i16 local_18;
  i16 local_16;
  i16 iStack20;
  u16 uStack16;
  i32 lStack14;
  u32 uStack10;
  u32 *puStack6;

  uVar12 = (u16)((u32)param_3 >> 0x10);
  uVar8 = (astruct_426 *)param_3;
  pass1_1038_53ba(uVar8->field4_0x4,0x1);
  if ((param_2 != 0x0) || (param_1 != 0x0)) {
    uVar15 = uVar8->field4_0x4;
    uVar13 = (u16)(uVar15 >> 0x10);
    iVar8 = (astruct_427 *)uVar15;
    puVar7 = iVar8->field12_0xc;
    ppcVar3 = (code **)((int)*puVar7 + 0x10);
    puStack6 = puVar7;
    (**ppcVar3)((int)&u16_1050_1038,(int)puVar7,((int)&iVar8->field12_0xc + 0x2));
    uStack10 = (u32)puVar7 & 0xffff | (u32)extraout_DX << 0x10;
    uVar15 = uVar8->field4_0x4;
    lStack14 = *(i32 *)((int)uVar15 + 0x8);
    uStack16 = 0x0;
    puVar14 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,&local_16));
    uVar9 = (u16)((u32)puVar14 >> 0x10);
    iVar1 = ((int)param_4 + 0x4);
    for (uStack34 = 0x0; uStack34 < uStack10; uStack34 += 0x1) {
      uVar15 = pass1_1030_1d7c((int)uStack10,uVar9,(u32)puStack6);
      uVar4 = (u16)(uVar15 >> 0x10);
      uVar10 = uVar4 | (u16)uVar15;
      uVar9 = uVar10;
      if ((uVar10 != 0x0) &&
         (uVar4 = pass1_1008_c6ae(_u16_1050_06e0,((u16)uVar15 + 0xc),0x46), uVar9 = uVar10, uVar4 != 0x0
         )) {
        pass1_1030_1d58((u32)puStack6);
        uVar9 = uVar10 | uVar4;
        if ((uVar10 | uVar4) != 0x0) {
          pass1_1008_3f62((u16 *)CONCAT22(0x1050,&local_16),(u16 *)CONCAT22(uVar10,uVar4 + 0xc));
          pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_16),(u16 *)CONCAT22(0x1050,&local_1c),
                          (u16 *)CONCAT22(0x1050,&local_1a),(u16 *)CONCAT22(0x1050,&local_18));
          uVar9 = uVar10;
          if ((iVar1 == local_1c) &&
             (uVar15 = uVar8->field4_0x4, uVar13 = (u16)(uVar15 >> 0x10), iVar11 = (int)uVar15,
             uVar2 = *(u32 *)(iVar11 + 0x4),
             uVar5 = pass1_1030_addc((int)&local_16,uVar10,(u16)uVar8,uVar12,(u16 *)CONCAT22(0x1050,&local_16),
                                     (u16)uVar2,(u16)((u32)uVar2 >> 0x10),*(u32 *)(iVar11 + 0x8)),
             uVar9 = uVar10, uVar5 != 0x0)) {
            iStack20 = local_1a + -0x1;
            piVar6 = &local_16;
            pass1_1030_ad86(uVar10,(u16)uVar8,uVar12,(u16 *)CONCAT22(0x1050,piVar6),lStack14);
            if (piVar6 != NULL) {
LAB_1030_a98e:
              pass1_1008_3f62(param_4,(u16 *)CONCAT22(0x1050,&local_16));
              return;
            }
            iStack20 = local_1a + 0x1;
            piVar6 = &local_16;
            pass1_1030_ad86(uVar10,(u16)uVar8,uVar12,(u16 *)CONCAT22(0x1050,piVar6),lStack14);
            if (piVar6 != NULL) goto LAB_1030_a98e;
            iStack20 = local_1a;
            local_16 = local_18 + -0x1;
            piVar6 = &local_16;
            pass1_1030_ad86(uVar10,(u16)uVar8,uVar12,(u16 *)CONCAT22(0x1050,piVar6),lStack14);
            if (piVar6 != NULL) goto LAB_1030_a98e;
            local_16 = local_18 + 0x1;
            piVar6 = &local_16;
            pass1_1030_ad86(uVar10,(u16)uVar8,uVar12,(u16 *)CONCAT22(0x1050,piVar6),lStack14);
            uVar9 = uVar10;
            if (piVar6 != NULL) goto LAB_1030_a98e;
          }
        }
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_aa18(u32 param_1,u16 *param_2)

{
  u32 uVar1;
  code **ppcVar2;
  u32 uVar3;
  u16 uVar4;
  BOOL16 BVar5;
  u32 uVar6;
  u16 uVar7;
  u16 uVar8;
  u32 in_EDX;
  astruct_57 *paVar9;
  u32 uVar10;
  u16 uVar11;
  u16 uVar12;
  i16 iVar13;
  u32 *puVar14;
  u16 uVar15;
  u16 uVar16;
  u16 uVar17;
  u16 uVar18;
  u32 *puVar19;
  u16 *puVar20;
  u32 uVar21;
  u8 uVar22;
  u32 uStack38;
  u8 local_1a [0x2];
  i16 local_18;
  i16 local_16;
  i16 local_14;
  i16 iStack18;
  u32 uStack14;
  u16 uStack10;
  u16 uStack8;
  i16 iStack6;
  i16 iStack4;

  uVar12 = (u16)((u32)in_EDX >> 0x10);
  iStack4 = ((int)param_2 + 0x4);
  iStack6 = 0x8 - (u16)(iStack4 == 0x0);
  puVar19 = pass1_1008_c6fa(_u16_1050_06e0,iStack6);
  uStack8 = (u16)((u32)puVar19 >> 0x10);
  paVar9 = (astruct_57 *)CONCAT22(uVar12,uStack8);
  uVar8 = (u16)puVar19;
  uVar15 = (u16)(param_1 >> 0x10);
  uVar11 = (u16)param_1;
  uStack10 = uVar8;
  pass1_1038_4e78(uVar8,paVar9,*(u32 *)(uVar11 + 0x4),puVar19);
  uStack14 = (u32 *)CONCAT22((int)paVar9,uVar8);
  uVar18 = 0x1008;
  puVar20 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,&local_14));
  uVar10 = (u32)puVar20 >> 0x10;
  uVar3 = *(u32 *)(uVar11 + 0x4);
  uVar1 = *(u32 *)((int)uVar3 + 0x8);
  uVar16 = (u16)((u32)uStack14 >> 0x10);
  uVar12 = SUB42(uStack14,0x0);
  ppcVar2 = (code **)((int)*uStack14 + 0x10);
  uVar6 = uVar1;
  (**ppcVar2)(0x1008,uVar12,uVar16);
  uVar6 = uVar6 & 0xffff | uVar10 << 0x10;
  uStack38 = 0x0;
  while( true ) {
    uVar8 = (u16)uVar10;
    if (uVar6 <= uStack38) {
      if (uStack14 != NULL) {
        ppcVar2 = (code **)*uStack14;
        (**ppcVar2)(uVar18,(int)uStack14,(char)((u32)uStack14 >> 0x10),0x1,uVar12,uVar16,uStack14,uStack14);
      }
      return;
    }
    uVar21 = uVar6;
    pass1_1030_1d58((u32)uStack14);
    uVar7 = uVar8 | (u16)uVar21;
    uVar10 = (u32)uVar7;
    if (uVar7 != 0x0) break;
LAB_1030_aadc:
    uStack38 += 0x1;
  }
  uVar10 = (u32)uVar8;
  pass1_1008_3f62((u16 *)CONCAT22(0x1050,&local_14),(u16 *)CONCAT22(uVar8,(u16)uVar21 + 0xc));
  uVar18 = 0x1008;
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_14),(u16 *)CONCAT22(0x1050,local_1a),
                  (u16 *)CONCAT22(0x1050,&local_18),(u16 *)CONCAT22(0x1050,&local_16));
  uVar3 = *(u32 *)(uVar11 + 0x4);
  uVar17 = (u16)((u32)uVar3 >> 0x10);
  iVar13 = (int)uVar3;
  uVar3 = *(u32 *)(iVar13 + 0x4);
  uVar4 = pass1_1030_addc((int)&local_14,(u16)uVar10,uVar11,uVar15,(u16 *)CONCAT22(0x1050,&local_14),(u16)uVar3
                          ,(u16)((u32)uVar3 >> 0x10),*(u32 *)(iVar13 + 0x8));
  if (uVar4 == 0x0) goto LAB_1030_aadc;
  uVar21 = struct_op_1030_73a8((astruct_419 *)(uVar21 & 0xffff | (u32)uVar8 << 0x10),uVar4,(int)uVar10);
  uVar10 = uVar21 >> 0x10;
  uVar8 = (u16)(uVar21 >> 0x10);
  iVar13 = ((int)uVar21 + 0xc);
  if (0x5 < iVar13 - 0x7aU) goto LAB_1030_aadc;
  uVar18 = 0x1030;
  switch(iVar13) {
  default:
    iStack18 = local_18 + -0x1;
    BVar5 = pass1_1030_acbe((u16)&local_14,uVar8,uVar11,uVar15,(u16 *)CONCAT22(0x1050,&local_14),uVar1);
    if (BVar5 != 0x0) goto LAB_1030_ac5b;
    iStack18 = local_18 + 0x1;
    BVar5 = pass1_1030_acbe((u16)&local_14,(u16)uVar10,uVar11,uVar15,(u16 *)CONCAT22(0x1050,&local_14),uVar1);
    if (BVar5 == 0x0) {
      iStack18 = local_18;
      local_14 = local_16 + -0x1;
      BVar5 = pass1_1030_acbe((u16)&local_14,(u16)uVar10,uVar11,uVar15,(u16 *)CONCAT22(0x1050,&local_14),uVar1);
      goto joined_r0x1030ab9e;
    }
LAB_1030_abc4:
    pass1_1008_3f62(param_2,(u16 *)CONCAT22(0x1050,&local_14));
    break;
  case 0x7b:
  case 0x7e:
    iStack18 = local_18 + -0x1;
    BVar5 = pass1_1030_acbe((u16)&local_14,uVar8,uVar11,uVar15,(u16 *)CONCAT22(0x1050,&local_14),uVar1);
    if (BVar5 == 0x0) {
      iStack18 = local_18 + 0x1;
      goto LAB_1030_abac;
    }
    pass1_1008_3f62(param_2,(u16 *)CONCAT22(0x1050,&local_14));
    if (uStack14 == NULL) {
      return;
    }
    uVar18 = (u16)((u32)uStack14 >> 0x10);
    puVar14 = (u32 *)uStack14;
    uVar22 = (u8)((u32)uStack14 >> 0x10);
    goto LAB_1030_ab66;
  case 0x7c:
  case 0x7d:
    local_14 = local_16 + -0x1;
    BVar5 = pass1_1030_acbe((u16)&local_14,uVar8,uVar11,uVar15,(u16 *)CONCAT22(0x1050,&local_14),uVar1);
joined_r0x1030ab9e:
    if (BVar5 == 0x0) {
      local_14 = local_16 + 0x1;
LAB_1030_abac:
      BVar5 = pass1_1030_acbe((u16)&local_14,(u16)uVar10,uVar11,uVar15,(u16 *)CONCAT22(0x1050,&local_14),uVar1);
      if (BVar5 != 0x0) goto LAB_1030_abc4;
      goto LAB_1030_aadc;
    }
LAB_1030_ac5b:
    pass1_1008_3f62(param_2,(u16 *)CONCAT22(0x1050,&local_14));
  }
  puVar14 = (u32 *)uStack14;
  if ((uStack14 | (u16)puVar14) != 0x0) {
    uVar18 = (u16)((u32)uStack14 >> 0x10);
    uVar22 = (u8)((u32)uStack14 >> 0x10);
LAB_1030_ab66:
    ppcVar2 = (code **)*puVar14;
    (**ppcVar2)(0x1008,puVar14,uVar22,0x1,uVar12,uVar16,uStack14,uStack14);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 pass1_1030_acbe(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 *param_5,i32 param_6)

{
  i16 iVar1;
  u16 uVar2;
  u32 uVar3;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_5,param_6);
  uVar2 = param_2 | param_1;
  if (uVar2 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    if ((uVar2 | param_1) != 0x0) {
      uVar3 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar2,param_1),param_1,uVar2 | param_1);
      if ((uVar3 != 0x0) && ((iVar1 = ((int)uVar3 + 0xc), iVar1 == 0x5 || (iVar1 == 0x9)))) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 pass1_1030_ad22(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 *param_5,i32 param_6)

{
  BOOL16 BVar1;
  u16 uVar2;
  u32 uVar3;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_5,param_6);
  uVar2 = param_2 | param_1;
  if (uVar2 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    if ((uVar2 | param_1) != 0x0) {
      uVar3 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar2,param_1),param_1,uVar2 | param_1);
      if (uVar3 != 0x0) {
        BVar1 = pass1_1008_c6ae(_u16_1050_06e0,((int)uVar3 + 0xc),0x46);
        return BVar1;
      }
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_ad86(u16 param_1,u16 param_2,u16 param_3,u16 *param_4,i32 param_5)

{
  u32 uVar1;
  u32 *puVar2;
  char cStack17;
  u32 local_a;
  i16 iStack6;

  puVar2 = &local_a;
  pass1_1030_64ce(puVar2,param_1,_PTR_LOOP_1050_5740,param_4,param_5,(u32 *)CONCAT22(0x1050,puVar2));
  uVar1 = *puVar2;
  cStack17 = (char)((u32)uVar1 >> 0x18);
  if (cStack17 == '\0') {
    iStack6 = (int)uVar1;
    if (((0x0 < iStack6) && (!SBORROW2(iStack6,0x1))) &&
       ((iStack6 == 0x3 || iStack6 + -0x2 < 0x1 || ((0x3 < iStack6 + -0x3 && (iStack6 + -0x7 < 0x2)))))) {
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1030_addc(i16 param_1,u16 param_2,u16 param_3,u16 param_4,u16 *param_5,u16 param_6,
                      u16 param_7,u32 param_8)

{
  u32 *puVar1;
  i16 local_14;
  i16 local_12;
  i16 local_10;
  i16 local_e;
  u32 local_c;
  u16 uStack8;
  i16 iStack6;
  u16 uStack4;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_8);
  iStack6 = param_1;
  uStack4 = param_2;
  puVar1 = (u32 *)pass1_1030_5b5c(param_1,param_2);
  local_c = *puVar1;
  uStack8 = ((int)puVar1 + 0x4);
  pass1_1008_3e94(param_5,(u16 *)CONCAT22(0x1050,&local_10),(char *)CONCAT22(0x1050,&local_e));
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_14),(char *)CONCAT22(0x1050,&local_12))
  ;
  if ((((0x1 < local_e) && (0x1 < local_10)) && (local_e < local_12 + -0x1)) && (local_10 < local_14 + -0x1)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Unable to use type for symbol uVar1

void pass1_1030_ae6c(astruct_399 *param_1)

{
  u16 uVar2;
  u16 uVar3;
  u32 in_EDX;
  u16 uVar5;
  astruct_57 *paVar4;
  astruct_399 *iVar4;
  astruct_399 *uVar4;
  u16 *puVar6;
  u32 uVar1;

  uVar5 = (u16)((u32)in_EDX >> 0x10);
  uVar4 = (astruct_399 *)((u32)param_1 >> 0x10);
  iVar4 = (astruct_399 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar4->field1_0x2 = 0x1008;
  iVar4->field2_0x4 = 0x0;
  puVar6 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar4->field3_0x8)));
  paVar4 = (astruct_57 *)CONCAT22(uVar5,(int)((u32)puVar6 >> 0x10));
  uVar2 = 0x0;
  iVar4->field8_0xe = 0x0;
  *(u32 *)&iVar4->field9_0x10 = 0x0;
  param_1->field0_0x0 = 0xb932;
  iVar4->field1_0x2 = 0x1030;
  mem_op_1000_179c(0xc,paVar4);
  uVar3 = (u16)paVar4 | uVar2;
  if (uVar3 == 0x0) {
    *(u32 *)&iVar4->field9_0x10 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22((u16)paVar4,uVar2));
    iVar4->field9_0x10 = uVar2;
    iVar4->field10_0x12 = uVar3;
  }
  uVar1 = *(u32 *)&iVar4->field9_0x10;
  ((int)uVar1 + 0xa) = 0x0;
  return;
}



// WARNING: Unable to use type for symbol uVar1

void pass1_1030_aefa(astruct_400 *param_1,u32 param_2)

{
  u16 uVar2;
  u16 *puVar3;
  u16 uVar4;
  u32 in_EDX;
  u16 uVar6;
  astruct_57 *paVar5;
  astruct_400 *iVar5;
  u16 uVar7;
  u16 *puVar8;
  u32 uVar1;

  uVar6 = (u16)((u32)in_EDX >> 0x10);
  uVar7 = (u16)((u32)param_1 >> 0x10);
  iVar5 = (astruct_400 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar5->field1_0x2 = 0x1008;
  iVar5->field2_0x4 = 0x0;
  puVar8 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field3_0x8)));
  paVar5 = (astruct_57 *)CONCAT22(uVar6,(int)((u32)puVar8 >> 0x10));
  iVar5->field8_0xe = 0x0;
  *(u32 *)&iVar5->field9_0x10 = 0x0;
  param_1->field0_0x0 = 0xb932;
  iVar5->field1_0x2 = 0x1030;
  iVar5->field2_0x4 = *(u32 *)((int)param_2 + 0x4);
  puVar3 = (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5->field3_0x8));
  pass1_1008_3f62(puVar3,(u16 *)(param_2 & 0xffff0000 | (u32)((int)param_2 + 0xc)));
  uVar2 = (u16)puVar3;
  mem_op_1000_179c(0xc,paVar5);
  uVar4 = (u16)paVar5 | uVar2;
  if (uVar4 == 0x0) {
    uVar2 = 0x0;
    uVar4 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22((u16)paVar5,uVar2));
  }
  iVar5->field9_0x10 = uVar2;
  iVar5->field10_0x12 = uVar4;
  uVar1 = *(u32 *)&iVar5->field9_0x10;
  ((int)uVar1 + 0xa) = 0x0;
  return;
}



void pass1_1030_afa6(StructD *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  u32 uVar4;
  StructD *iVar5;
  u16 uVar5;

  uVar5 = (u16)((u32)param_1 >> 0x10);
  iVar5 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0xb932;
  iVar5->address_offset_field_0x2 = 0x1030;
  if (*(i32 *)&iVar5->field_0x10 != 0x0) {
    uVar4 = *(u32 *)&iVar5->field_0x10;
    ((int)uVar4 + 0xa) = 0x1;
  }
  puVar1 = *(u32 **)&iVar5->field_0x10;
  uVar2 = iVar5->field11_0x12;
  if ((uVar2 | (u16)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  param_1->address_offset_field_0x0 = 0x389a;
  iVar5->address_offset_field_0x2 = 0x1008;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_affc(u32 param_1)

{
  i16 iVar1;
  u16 uVar2;
  u16 uVar3;
  BOOL16 BVar4;
  u16 uVar5;
  u16 uVar6;
  u32 uVar7;
  u32 uVar8;
  i16 iStack12;
  u32 uStack10;
  u32 local_6;

  uVar8 = ZEXT24(&local_6);
  pass1_1030_b718(param_1,(u16)param_1,(u16)param_1,
                  (u16 *)(param_1 & 0xffff0000 | (u32)((u16)param_1 + 0x8)),(u32 *)CONCAT22(0x1050,&local_6));
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,local_6);
  uStack10 = (astruct_419 *)(uVar8 & 0xffff | ZEXT24(param_1) << 0x10);
  uVar5 = (u16)param_1 | (u16)uVar8;
  if (uVar5 != 0x0) {
    uVar7 = struct_op_1030_73a8((astruct_419 *)(uVar8 & 0xffff | ZEXT24(param_1) << 0x10),(u16)uVar8,uVar5);
    uVar5 = (u16)(uVar7 >> 0x10);
    iVar1 = ((int)uVar7 + 0xc);
    uVar8 = (u32)(iVar1 - 0x16U);
    if ((0x15 < iVar1) && (!SBORROW2(iVar1,0x16))) {
      uVar8 = (u32)(iVar1 - 0x17U);
      if (iVar1 - 0x17U != 0x0 && 0x0 < (int)(iVar1 - 0x16U)) {
        uVar8 = (u32)(iVar1 - 0x19U);
        if ((iVar1 + -0x18 < 0x1) ||
           (uVar8 = (u32)(iVar1 - 0x1aU), iVar1 - 0x1aU != 0x0 && 0x0 < (int)(iVar1 - 0x19U))) goto LAB_1030_b064;
      }
      ((int)uVar7 + 0x20) = 0x0;
    }
  }
LAB_1030_b064:
  iStack12 = 0x6;
  do {
    uVar3 = (u16)uVar8;
    uVar6 = uVar5;
    if (iStack12 == 0x0) {
LAB_1030_b0fc:
      if ((uStack10 | (u16)uStack10) != 0x0) {
        uVar8 = struct_op_1030_73a8(uStack10,uStack10 | (u16)uStack10,uVar6);
        uVar2 = (u16)(uVar8 >> 0x10);
        iVar1 = ((int)uVar8 + 0xc);
        if (((0x15 < iVar1) && (!SBORROW2(iVar1,0x16))) &&
           ((iVar1 == 0x17 || iVar1 + -0x16 < 0x1 || ((0x0 < iVar1 + -0x18 && (iVar1 + -0x19 < 0x2)))))) {
          ((int)uVar8 + 0x20) = 0x1;
        }
      }
      return;
    }
    pass1_1030_b578(param_1);
    uVar6 = uVar5 | uVar3;
    if (uVar6 == 0x0) goto LAB_1030_b0fc;
    uStack10 = (astruct_419 *)CONCAT22(uVar5,uVar3);
    uVar8 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar5,uVar3),uVar3,uVar5);
    uVar6 = (u16)(uVar8 >> 0x10);
    iVar1 = ((int)uVar8 + 0xc);
    pass1_1008_3f62((u16 *)(param_1 & 0xffff0000 | (u32)((u16)param_1 + 0x8)),
                    (u16 *)CONCAT22(uVar5,uVar3 + 0xc));
    if ((iVar1 == 0x18) || (iVar1 == 0x3f)) {
      pass1_1030_b142(param_1,(u32)uStack10);
    }
    BVar4 = pass1_1008_c6ae(_u16_1050_06e0,iVar1,0x40);
    uVar8 = (u32)BVar4;
    if (BVar4 != 0x0) {
      pass1_1030_b454(param_1,(u32)uStack10);
      goto LAB_1030_b0fc;
    }
    iStack12 += -0x1;
    uVar5 = uVar6;
  } while( true );
}



void pass1_1030_b13c(void)

{
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1030_b142(u32 param_1,u32 param_2)

{
  i16 iVar1;
  u16 in_AX;
  u16 in_DX;
  i16 iVar2;
  u16 uVar3;
  bool bVar4;
  u32 uVar5;
  u32 uStack12;

  uVar5 = struct_op_1030_73a8((astruct_419 *)param_2,in_AX,in_DX);
  uVar3 = (u16)(uVar5 >> 0x10);
  iVar1 = (int)uVar5;
  iVar2 = (iVar1 + 0xc);
  uStack12 = 0x0;
  if (iVar2 == 0x18) {
    uStack12 = pass1_1028_1c1c();
    uVar3 = (iVar1 + 0x22);
  }
  else {
    if (iVar2 != 0x3f) goto LAB_1030_b1a6;
    uStack12 = pass1_1028_20b0();
    uVar3 = (iVar1 + 0x24);
  }
  uStack12 = CONCAT22(uStack12,uVar3);
LAB_1030_b1a6:
  uVar3 = (u16)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((iVar2 + 0xe) == 0x1) {
    bVar4 = (uStack12 & 0x10000) == 0x0;
  }
  else if ((iVar2 + 0xe) == 0x2) {
    bVar4 = (uStack12 & 0x20000) == 0x0;
  }
  else if ((iVar2 + 0xe) == 0x3) {
    bVar4 = (uStack12 & 0x40000) == 0x0;
  }
  else {
    bVar4 = (uStack12 & 0x80000) == 0x0;
  }
  if ((bVar4) || ((int)uStack12 != 0x0)) {
    bVar4 = false;
    while( true ) {
      if (((uStack12 & 0x10000) != 0x0) && ((int)uStack12 == 0x0)) goto LAB_1030_b239;
      if (((uStack12 & 0x20000) != 0x0) && ((int)uStack12 == 0x0)) goto LAB_1030_b247;
      if (((uStack12 & 0x40000) != 0x0) && ((int)uStack12 == 0x0)) goto LAB_1030_b255;
      if (((uStack12 & 0x80000) != 0x0) && ((int)uStack12 == 0x0)) goto LAB_1030_b263;
      if (bVar4) break;
      uStack12._1_3_ = (u163)(uStack12 >> 0x8) & 0xffff00;
      iVar1 = (iVar2 + 0xe);
      if (iVar1 == 0x1) {
        uStack12 = CONCAT31(uStack12._1_3_,0x4);
      }
      else if (iVar1 == 0x2) {
        uStack12 = CONCAT31(uStack12._1_3_,0x8);
      }
      else if (iVar1 == 0x3) {
        uStack12 = CONCAT31(uStack12._1_3_,0x1);
      }
      else {
        uStack12 = CONCAT31(uStack12._1_3_,0x2);
      }
      bVar4 = true;
    }
    if ((iVar2 + 0xe) == 0x1) {
LAB_1030_b255:
      (iVar2 + 0xe) = 0x3;
      return;
    }
    if ((iVar2 + 0xe) == 0x2) {
LAB_1030_b263:
      (iVar2 + 0xe) = 0x4;
      return;
    }
    if ((iVar2 + 0xe) == 0x3) {
LAB_1030_b239:
      (iVar2 + 0xe) = 0x1;
      return;
    }
    if ((iVar2 + 0xe) == 0x4) {
LAB_1030_b247:
      (iVar2 + 0xe) = 0x2;
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_b2aa(uchar *param_1,u32 param_2,u16 *param_3)

{
  u16 uVar1;
  BOOL16 BVar2;
  u32 uVar3;
  u8 bStack23;
  u32 local_6;

  pass1_1030_b718(param_1,(u16)param_2,(u16)(param_2 >> 0x10),param_3,(u32 *)CONCAT22(0x1050,&local_6));
  bStack23 = (u8)(local_6 >> 0x18);
  uVar1 = (u16)bStack23;
  if (bStack23 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,local_6 & 0xffff | (u32)local_6 << 0x10);
    if ((local_6 | uVar1) != 0x0) {
      uVar3 = struct_op_1030_73a8((astruct_419 *)CONCAT22(local_6,uVar1),uVar1,local_6 | uVar1);
      BVar2 = pass1_1008_c6ae(_u16_1050_06e0,((int)uVar3 + 0xc),0x42);
      if (BVar2 != 0x0) {
        pass1_1008_3f62((u16 *)(param_2 & 0xffff0000 | (u32)((u16)param_2 + 0x8)),
                        (u16 *)CONCAT22(local_6,uVar1 + 0xc));
        return;
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

u32 pass1_1030_b344(u32 param_1)

{
  uchar *puVar1;
  u32 *puStack18;
  uchar *puStack16;
  u8 local_e [0x2];
  i16 local_c;
  i16 local_a;
  u32 local_8;
  u16 uStack4;

  local_8 = *(u32 *)((int)param_1 + 0x8);
  uStack4 = ((int)param_1 + 0xc);
  puVar1 = param_1;
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,local_e),
                  (u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_a));
  local_8 = local_8 & 0xffff | (u32)(local_c - 0x1) << 0x10;
  puStack18 = &local_8;
  pass1_1030_b2aa(puVar1,param_1,(u16 *)CONCAT22(0x1050,puStack18));
  puStack16 = (uchar *)((u16)puVar1 | (u16)puStack18);
  if (puStack16 == NULL) {
    local_8 = local_8 & 0xffff | (u32)(local_c + 0x1) << 0x10;
    puStack18 = &local_8;
    pass1_1030_b2aa(NULL,param_1,(u16 *)CONCAT22(0x1050,puStack18));
    puVar1 = (uchar *)((u16)puStack16 | (u16)puStack18);
    if (puVar1 == NULL) {
      local_8._0_2_ = local_a + -0x1;
      local_8 = local_c;
      puStack18 = &local_8;
      pass1_1030_b2aa(NULL,param_1,(u16 *)CONCAT22(0x1050,puStack18));
      puStack16 = (uchar *)((u16)puVar1 | (u16)puStack18);
      if (puStack16 == NULL) {
        local_8 = CONCAT22(local_8,local_a + 0x1);
        puStack18 = &local_8;
        pass1_1030_b2aa(NULL,param_1,(u16 *)CONCAT22(0x1050,puStack18));
        if (((u16)puStack16 | (u16)puStack18) == 0x0) {
          return 0x0;
        }
        ((int)param_1 + 0xe) = 0x2;
      }
      else {
        ((int)param_1 + 0xe) = 0x4;
        puStack16 = puVar1;
      }
    }
    else {
      ((int)param_1 + 0xe) = 0x3;
    }
  }
  else {
    ((int)param_1 + 0xe) = 0x1;
    puStack16 = puVar1;
  }
  return CONCAT22(puStack16,puStack18);
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1030_b454(u32 param_1,u32 param_2)

{
  u32 *puVar1;
  code **ppcVar2;
  u8 *puVar3;
  u16 extraout_DX;
  i16 iVar4;
  u16 extraout_DX_00;
  u16 uVar5;
  i16 iVar6;
  u16 uVar7;
  u32 uVar8;
  u32 uVar9;
  i32 lStack38;
  u32 uStack30;
  u8 local_12 [0x4];
  u32 uStack14;
  u32 uStack10;
  i32 lStack6;

  lStack6 = *(i32 *)((int)param_2 + 0x4);
  uVar7 = (u16)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  pass1_1008_5784((char *)CONCAT22(0x1050,local_12),*(u32 *)(iVar6 + 0x10));
  while( true ) {
    puVar3 = local_12;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar3));
    uStack10 = CONCAT22(extraout_DX,puVar3);
    if ((extraout_DX | (u16)puVar3) == 0x0) break;
    if (*(i32 *)(puVar3 + 0x20) == lStack6) {
      ppcVar2 = (code **)((int)*(u32 *)*(u32 *)(iVar6 + 0x10) + 0xc);
      (**ppcVar2)();
      uStack14 = 0x0;
      pass1_1038_69fe(uStack10);
    }
  }
  uVar8 = struct_op_1030_73a8((astruct_419 *)param_2,puVar3,0x0);
  iVar4 = (int)(uVar8 >> 0x10);
  puVar1 = (u32 *)*(u32 *)((int)uVar8 + 0x20);
  puVar3 = local_12;
  pass1_1008_5784((char *)CONCAT22(0x1050,puVar3),(u32)puVar1);
  pass1_1030_b13c();
  uStack30 = CONCAT22(-(u16)((u8 *)((int)s_Unsupported_FileStructType_in_Op_1050_01ca + 0x2aU) < puVar3) - iVar4
                      ,0x1f4 - (int)puVar3);
  do {
    puVar3 = local_12;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar3));
    uStack10 = CONCAT22(extraout_DX_00,puVar3);
    uVar5 = extraout_DX_00 | (u16)puVar3;
    if (uVar5 == 0x0) {
      return;
    }
    pass1_1038_6984(CONCAT22(extraout_DX_00,puVar3));
    lStack38 = CONCAT22(uVar5,puVar3);
    if (((int)uVar5 <= uStack30) && (((int)uVar5 < uStack30 || (puVar3 <= (u8 *)uStack30)))) {
      uVar9 = *(u32 *)(iVar6 + 0x10);
      ppcVar2 = (code **)((int)*(u32 *)*(u32 *)(iVar6 + 0x10) + 0x8);
      (**ppcVar2)();
      uStack30 -= lStack38;
      ppcVar2 = (code **)((int)*puVar1 + 0xc);
      (**ppcVar2)((int)&u16_1050_1038,(int)puVar1,(int)((u32)puVar1 >> 0x10),uStack10,uVar9);
      uStack14 = 0x0;
    }
  } while (0x0 < uStack30);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_b578(u32 param_1)

{
  i16 iVar1;
  u32 *puVar2;
  u16 uVar3;
  uchar *puVar4;
  bool bVar5;
  u32 uVar6;
  u32 uStack48;
  u8 local_1c [0x2];
  i16 local_1a;
  i16 local_18;
  u32 local_16;
  u16 uStack18;
  u16 uStack16;
  u32 uStack14;
  u16 uStack10;
  u16 uStack8;
  u32 local_6;

  pass1_1030_b718(param_1,(u16)param_1,(u16)param_1,
                  (u16 *)(param_1 & 0xffff0000 | (u32)((u16)param_1 + 0x8)),(u32 *)CONCAT22(0x1050,&local_6));
  uStack48._3_1_ = (u8)(local_6 >> 0x18);
  uStack10 = (u16)uStack48._3_1_;
  if (uStack48._3_1_ == 0x0) {
    return;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,local_6 & 0xffff | (u32)local_6 << 0x10);
  uStack8 = local_6;
  uStack14 = struct_op_1030_73a8((astruct_419 *)CONCAT22(local_6,uStack10),uStack10,local_6);
  uStack16 = ((int)uStack14 + 0xc);
  local_16 = *(u32 *)((u16)param_1 + 0x8);
  uStack18 = ((u16)param_1 + 0xc);
  puVar4 = param_1;
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_16),(u16 *)CONCAT22(0x1050,local_1c),
                  (u16 *)CONCAT22(0x1050,&local_1a),(u16 *)CONCAT22(0x1050,&local_18));
  iVar1 = ((u16)param_1 + 0xe);
  if (iVar1 == 0x0) {
    pass1_1030_b344(param_1 & 0xffff | ZEXT24(param_1) << 0x10);
    return;
  }
  if (iVar1 == 0x1) {
    uVar3 = local_1a - 0x1;
LAB_1030_b63e:
    local_16 = local_16 & 0xffff | (u32)uVar3 << 0x10;
    puVar2 = &local_16;
    pass1_1030_b2aa(puVar4,param_1 & 0xffff | ZEXT24(param_1) << 0x10,(u16 *)CONCAT22(0x1050,puVar2));
    uStack48 = CONCAT22(puVar4,puVar2);
    if (((u16)puVar4 | (u16)puVar2) == 0x0) {
      return;
    }
    uVar6 = struct_op_1030_73a8((astruct_419 *)CONCAT22(puVar4,puVar2),puVar2,(u16)puVar4 | (u16)puVar2);
    uVar3 = ((int)uVar6 + 0xc);
    if (uVar3 == 0x3f) goto LAB_1030_b6e0;
    if (0x3f < uVar3) {
      return;
    }
    if ((char)uVar3 == '\x16') goto LAB_1030_b6e0;
    bVar5 = (char)uVar3 == '\x18';
  }
  else {
    if (iVar1 == 0x2) {
      uVar3 = local_18 + 0x1;
    }
    else {
      if (iVar1 == 0x3) {
        uVar3 = local_1a + 0x1;
        goto LAB_1030_b63e;
      }
      if (iVar1 != 0x4) {
        return;
      }
      uVar3 = local_18 - 0x1;
    }
    local_16 = local_16 & 0xffff0000 | (u32)uVar3;
    puVar2 = &local_16;
    pass1_1030_b2aa(puVar4,param_1 & 0xffff | ZEXT24(param_1) << 0x10,(u16 *)CONCAT22(0x1050,puVar2));
    uStack48 = CONCAT22(puVar4,puVar2);
    if (((u16)puVar4 | (u16)puVar2) == 0x0) {
      return;
    }
    uVar6 = struct_op_1030_73a8((astruct_419 *)CONCAT22(puVar4,puVar2),puVar2,(u16)puVar4 | (u16)puVar2);
    iVar1 = ((int)uVar6 + 0xc);
    if (iVar1 < 0x17) {
      return;
    }
    if (SBORROW2(iVar1,0x17)) {
      return;
    }
    if (iVar1 == 0x18 || iVar1 + -0x17 < 0x1) goto LAB_1030_b6e0;
    bVar5 = iVar1 == 0x3f;
  }
  if (!bVar5) {
    return;
  }
LAB_1030_b6e0:
  pass1_1008_3f62((u16 *)(param_1 & 0xffff0000 | (u32)((u16)param_1 + 0x8)),
                  (u16 *)(uStack48 & 0xffff0000 | (u32)((int)uStack48 + 0xc)));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_b718(uchar *param_1,u16 param_2,u16 param_3,u16 *param_4,u32 *param_5)

{
  u32 *puVar1;
  u16 uVar2;
  u16 in_register_0000000a;
  u32 *puVar3;
  u16 in_stack_0000fe96;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffee;

  puVar3 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000ffee,0x2f),in_stack_0000fe96,in_stack_0000ffba,
                           in_stack_0000ffc0,in_stack_0000ffc4);
  uVar2 = (u16)((u32)puVar3 >> 0x10);
  puVar1 = (u32 *)&stack0xffee;
  pass1_1030_64ce(puVar1,uVar2,_PTR_LOOP_1050_5740,param_4,*(i32 *)((int)puVar3 + 0x20),
                  (u32 *)CONCAT22(0x1050,puVar1));
  *param_5 = *puVar1;
  return;
}



void pass1_1030_b768(u32 param_1,u32 param_2)

{
  u32 uVar1;
  BOOL16 BVar2;
  i16 iVar3;
  u8 *puVar4;
  u16 extraout_DX;
  i16 iVar5;
  u16 uVar6;
  HFILE16 in_stack_0000ffc8;
  u16 local_22 [0x4];
  u8 local_1a [0xa];
  u32 local_10;
  u8 *puStack12;
  u16 uStack10;
  u16 local_8 [0x3];

  uVar6 = (u16)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  local_10 = *(u32 *)(iVar5 + 0x4);
  BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_10),(char *)0x4,in_stack_0000ffc8);
  if ((BVar2 != 0x0) &&
     (iVar3 = write_to_file_1008_7b4c(param_2,(astruct_615 *)(param_1 & 0xffff0000 | (u32)(iVar5 + 0x8))),
     iVar3 != 0x0)) {
    local_8[0] = (iVar5 + 0xe);
    BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_8),(char *)0x2,in_stack_0000ffc8);
    if (BVar2 != 0x0) {
      uVar1 = *(u32 *)(iVar5 + 0x10);
      local_22[0] = ((int)uVar1 + 0x8);
      local_10 = local_10 & 0xffff0000 | (u32)local_22[0];
      BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_22),(char *)0x2,in_stack_0000ffc8);
      if (BVar2 == 0x0) {
        return;
      }
      pass1_1008_5784((char *)CONCAT22(0x1050,local_1a),*(u32 *)(iVar5 + 0x10));
      do {
        puVar4 = local_1a;
        pass1_1008_5b12((char *)CONCAT22(0x1050,puVar4));
        if ((extraout_DX | (u16)puVar4) == 0x0) {
          return;
        }
        puStack12 = puVar4;
        uStack10 = extraout_DX;
        pass1_1038_75ca((int)puVar4,CONCAT22(extraout_DX,puVar4),param_2);
      } while (puVar4 != NULL);
      return;
    }
  }
  u16_1050_0310 = 0x6d0;
  return;
}



void file_1030_b836(uchar *param_1,astruct_401 *param_2,u32 param_3)

{
  code **ppcVar1;
  astruct_401 *iVar4;
  BOOL16 BVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 in_register_0000000a;
  astruct_57 *paVar6;
  u16 uVar8;
  u32 uVar9;
  u16 local_12 [0x7];
  u16 local_4;
  astruct_57 *paVar7;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  iVar4 = (astruct_401 *)param_2;
  iVar4 = (astruct_401 *)&iVar4->field4_0x4;
  BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)((u32)param_2 & 0xffff0000 | ZEXT24(iVar4)),0x4);
  if (((BVar2 == 0x0) ||
      (BVar2 = read_file_1008_7bc8(param_3,(u16 *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar4->field_0x8))),
      BVar2 == 0x0)) ||
     (BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)CONCAT22(0x1050,&local_4),0x2), BVar2 == 0x0)) {
    u16_1050_0310 = 0x6d2;
  }
  else {
    uVar8 = (u16)((u32)param_2 >> 0x10);
    iVar4->field13_0xe = local_4;
    BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)CONCAT22(0x1050,local_12),0x2);
    if (BVar2 != 0x0) {
      while( true ) {
        if (local_12[0] == 0x0) {
          return;
        }
        uVar3 = local_12[0];
        uVar9 = param_3;
        local_12[0] = local_12[0] - 0x1;
        mem_op_1000_179c(0x2a,paVar6);
        uVar4 = (u16)paVar6;
        uVar5 = uVar4 | uVar3;
        paVar6 = (astruct_57 *)((u32)paVar6 & 0xffff0000);
        paVar7 = (astruct_57 *)((u32)paVar6 | (u32)uVar5);
        if (uVar5 == 0x0) {
          uVar3 = 0x0;
        }
        else {
          struct_1038_6520((astruct_308 *)CONCAT22(uVar4,uVar3));
          paVar6 = paVar7;
        }
        file_1038_774e((uchar *)paVar6,(astruct_169 *)CONCAT22((uchar *)paVar6,uVar3),uVar9);
        if (uVar3 == 0x0) break;
        ppcVar1 = (code **)((int)*iVar4->field14_0x10 + 0x4);
        (**ppcVar1)();
      }
    }
  }
  return;
}



StructD * pass1_1030_b90c(StructD *param_1,u8 param_2)

{
  pass1_1030_afa6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void pass1_1030_b936(u16 param_1,astruct_365 *param_2,u16 param_3,u16 param_4,u32 param_5)

{
  pass1_1028_b22c(param_1,(u16 *)CONCAT22(param_3,param_2),param_4,param_5);
  param_2->field12_0xe = 0x0;
  param_2->field13_0x12 = 0x0;
  CONCAT22(param_3,param_2) = 0xbc0c;
  param_2->field1_0x2 = 0x1030;
  return;
}



void pass1_1030_b96c(u16 *param_1)

{
  u16 uVar1;
  char *pcVar2;
  i16 iVar3;
  u16 uVar4;

  uVar4 = (u16)((u32)param_1 >> 0x10);
  iVar3 = (int)param_1;
  *param_1 = 0xbc0c;
  (iVar3 + 0x2) = 0x1030;
  pcVar2 = *(char **)(iVar3 + 0xe);
  uVar1 = (iVar3 + 0x10);
  if ((uVar1 | (u16)pcVar2) != 0x0) {
    fn_ptr_1020_ba7e((u32 *)((u32)pcVar2 & 0xffff | (u32)uVar1 << 0x10));
    fn_ptr_1000_17ce(pcVar2);
  }
  pass1_1028_b260(param_1);
  return;
}



void pass1_1030_b9b2(u32 param_1)

{
  u16 uVar1;

  uVar1 = (u16)(param_1 >> 0x10);
  *(u32 *)((int)param_1 + 0xe) = 0x0;
  *(u32 *)((int)param_1 + 0x12) = 0x0;
  return;
}



void pass1_1030_b9da(astruct_57 *param_1,astruct_172 *param_2,astruct_402 *param_3,u32 param_4)

{
  u32 *puVar1;
  u16 uVar2;
  astruct_57 *uVar3;
  u32 uVar4;
  u16 uVar5;
  astruct_57 *paVar7;
  astruct_402 *iVar7;
  astruct_402 *uVar6;
  u32 uVar8;
  u16 uStack12;
  u16 uStack4;
  astruct_57 *paVar6;

  uVar6 = (astruct_402 *)((u32)param_3 >> 0x10);
  iVar7 = (astruct_402 *)param_3;
  paVar6 = param_1;
  if (*(i32 *)&iVar7->field14_0xe == 0x0) {
    mem_op_1000_179c(0xa,param_1);
    uVar3 = (astruct_57 *)((u16)param_1 | (u16)param_2);
    paVar6 = (astruct_57 *)ZEXT24(uVar3);
    if (uVar3 == NULL) {
      *(u32 *)&iVar7->field14_0xe = 0x0;
    }
    else {
      pass1_1020_ba3e((astruct_172 *)((u32)param_2 & 0xffff | (long)param_1 << 0x10),0x5,0x5);
      iVar7->field14_0xe = (astruct_172 *)param_2;
      iVar7->field15_0x10 = (i16)paVar6;
    }
    iVar7->field16_0x12 = 0x0;
  }
  for (uStack4 = 0x4; (int)uStack4 < 0xe; uStack4 += 0x1) {
    uVar8 = pass1_1030_7c28((u16)param_2,(u16)paVar6,param_4,uStack4);
    uVar2 = (u16)(uVar8 >> 0x10);
    param_2 = (astruct_172 *)(uVar8 & 0xffff);
    uVar5 = uVar2 | (u16)param_2;
    paVar6 = (astruct_57 *)(u32)uVar5;
    if (uVar5 != 0x0) {
      uVar4 = 0x64 - iVar7->field16_0x12;
      paVar7 = (astruct_57 *)(uVar4 >> 0x10);
      uStack12 = (u16)uVar8;
      if (uVar8 < uVar4) {
        uVar4 = uVar8 & 0xffff;
        paVar7 = (astruct_57 *)(u32)uVar2;
      }
      uVar5 = (u16)uVar4;
      param_2 = (astruct_172 *)(uVar4 & 0xffff | (long)paVar7 << 0x10);
      paVar6 = paVar7;
      pass1_1030_7d1c(uVar5,(int)paVar7,(astruct_397 *)param_4,uStack12 - uVar5,
                      CONCAT22(uStack4,(uVar2 - (int)paVar7) - (u16)(uStack12 < uVar5)));
      pass1_1020_bb8a(*(i32 **)&iVar7->field14_0xe,uVar5,(u32)paVar7 | (u32)uStack4 << 0x10);
      puVar1 = &iVar7->field16_0x12;
      *puVar1 = (long)&param_2->field0_0x0 + *puVar1;
      string_1020_c0ca(uStack4);
      vsprintf_op_1030_840a((u16)paVar6,(u32)s_truck_0x_08lx_loaded__ld_of__s_f_1050_576c);
      if (0x63 < iVar7->field16_0x12) break;
    }
  }
  if (iVar7->field16_0x12 != 0x0) {
    return;
  }
  return;
}



void pass1_1030_bb0e(u16 param_1,u32 param_2,u32 param_3,u16 param_4)

{
  u32 *puVar1;
  u32 uVar2;
  u16 uVar3;
  u32 uVar4;
  u16 uVar5;
  u16 in_register_0000000a;
  u32 uVar7;
  u16 uStack8;
  astruct_57 *paVar6;

  uVar3 = pass1_1030_7bee(param_3);
  uVar4 = (u32)uVar3;
  if (uVar3 != 0x0) {
    return;
  }
  pass1_1030_b9b2(param_2);
  uVar2 = uVar4 & 0xffff;
  puVar1 = (u32 *)(uVar2 | (u32)param_4 << 0x10);
  uVar5 = param_4 | (u16)uVar4;
  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,uVar5);
  if (uVar5 != 0x0) {
    for (uStack8 = 0x4; (int)uStack8 < 0x25; uStack8 += 0x1) {
      uVar7 = pass1_1020_bae6((u16)uVar4,(u16)paVar6,(u16)uVar2,CONCAT22(uStack8,param_4));
      uVar4 = uVar7 & 0xffff;
      uVar5 = (u16)(uVar7 >> 0x10) | (u16)uVar4;
      paVar6 = (astruct_57 *)((u32)paVar6 & 0xffff0000 | (u32)uVar5);
      if (uVar5 != 0x0) {
        pass1_1030_7ddc((u16)uVar4,paVar6,param_3,uVar7,uStack8);
        uVar3 = pass1_1030_7bee(param_3);
        uVar4 = (u32)uVar3;
        if (uVar3 != 0x0) {
          return;
        }
        string_1020_c0ca(uStack8);
        vsprintf_op_1030_840a((u16)paVar6,(u32)s_truck_0x_08lx_unloaded__ld_of__s_1050_5798);
        pass1_1020_bb8a((i32 *)puVar1,0x0,(u32)uStack8 << 0x10);
      }
    }
    if (puVar1 != NULL) {
      fn_ptr_1020_ba7e(puVar1);
      fn_ptr_1000_17ce((char *)puVar1);
    }
  }
  return;
}



StructD * pass1_1030_bbe6(StructD *param_1,u8 param_2)

{
  pass1_1030_b96c(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1030_bc24(u16 param_1,i16 param_2,u16 param_3,u16 param_4,u32 param_5)

{
  pass1_1028_b22c(param_1,(u16 *)CONCAT22(param_3,param_2),param_4,param_5);
  CONCAT22(param_3,param_2) = 0xbc96;
  (param_2 + 0x2) = 0x1030;
  return (u16 *)CONCAT22(param_3,param_2);
}



void pass1_1030_bc4e(u16 *param_1)

{
  *param_1 = 0xbc96;
  ((int)param_1 + 0x2) = 0x1030;
  pass1_1028_b260(param_1);
  return;
}



void FUN_1030_bc6c(void)

{
  return;
}



StructD * pass1_1030_bc70(StructD *param_1,u8 param_2)

{
  pass1_1030_bc4e(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u32 pass1_1030_bcae(u16 param_1,u16 param_2)

{
  return CONCAT22(param_2,param_1);
}



void pass1_1030_bcbc(u16 param_1,u32 param_2,u32 param_3,u16 param_4,u32 param_5)

{
  pass1_1030_bcde(param_1,(u16)param_2,CONCAT22((u16)param_3,param_2),
                  (u16 *)CONCAT22(param_4,param_3),*(i32 *)((int)param_5 + 0x4));
  return;
}



void pass1_1030_bcde(u16 param_1,u16 param_2,u32 param_3,u16 *param_4,i32 param_5)

{
  i16 iVar1;
  u16 uVar2;
  i16 local_14;
  i16 local_12;
  i16 local_10;
  i16 local_e;
  u32 local_c;
  u16 uStack8;
  i32 lStack6;

  uVar2 = (u16)(param_3 >> 0x10);
  iVar1 = (int)param_3;
  lStack6 = *(i32 *)(iVar1 + 0x8);
  if (lStack6 != param_5) {
    return;
  }
  local_c = *(u32 *)(iVar1 + 0xc);
  uStack8 = (iVar1 + 0x10);
  pass1_1008_3e94(param_4,(u16 *)CONCAT22(0x1050,&local_10),(char *)CONCAT22(0x1050,&local_e));
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_14),(char *)CONCAT22(0x1050,&local_12))
  ;
  pass1_1000_49b2(local_e - local_12);
  pass1_1000_49b2(local_10 - local_14);
  return;
}



void pass1_1030_bd74(u16 param_1,u16 param_2,u32 param_3,astruct_670 *param_4)

{
  astruct_670 *iVar1;
  i16 iVar2;
  u16 uVar3;
  u16 uVar4;
  i16 local_1e;
  i16 local_1c;
  i16 local_1a;
  i16 local_18;
  u32 local_16;
  u16 uStack18;
  u32 local_10;
  u16 uStack12;
  i32 lStack10;
  i32 lStack6;

  uVar3 = (u16)((u32)param_4 >> 0x10);
  iVar1 = (astruct_670 *)param_4;
  lStack6 = iVar1->field8_0x8;
  uVar4 = (u16)(param_3 >> 0x10);
  iVar2 = (int)param_3;
  lStack10 = *(i32 *)(iVar2 + 0x8);
  if (lStack10 != lStack6) {
    return;
  }
  local_10 = iVar1->field9_0xc;
  uStack12 = iVar1->field10_0x10;
  local_16 = *(u32 *)(iVar2 + 0xc);
  uStack18 = (iVar2 + 0x10);
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_10),(u16 *)CONCAT22(0x1050,&local_1a),(char *)CONCAT22(0x1050,&local_18)
                 );
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_16),(u16 *)CONCAT22(0x1050,&local_1e),(char *)CONCAT22(0x1050,&local_1c)
                 );
  pass1_1000_49b2(local_18 - local_1c);
  pass1_1000_49b2(local_1a - local_1e);
  return;
}



astruct_180 * set_fn_ptr_1030_be34(astruct_180 *param_1)

{
  struct_1028_b354(param_1);
  param_1->field0_0x0 = 0xc006;
  ((int)param_1 + 0x2) = 0x1030;
  return param_1;
}



u16 * pass1_1030_be56(u16 param_1,astruct_12 *param_2,i16 param_3,u32 param_4)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2->field0_0x0 = 0xc006;
  ((int)param_2 + 0x2) = 0x1030;
  return &param_2->field0_0x0;
}



// WARNING: Unable to use type for symbol uVar2

void pass1_1030_be80(uchar *param_1,astruct_15 *param_2)

{
  i16 *piVar1;
  StructD *pSVar2;
  i16 iVar3;
  BOOL16 BVar4;
  u32 uVar5;
  u16 extraout_DX;
  u16 uVar6;
  astruct_15 *pstruct15_7;
  u16 uVar7;
  i16 iVar8;
  StructD *uVar2;

  pass1_1028_bf22(param_1,(u32)param_2);
  uVar7 = (u16)((u32)param_2 >> 0x10);
  pstruct15_7 = (astruct_15 *)param_2;
  if (pstruct15_7->field15_0x12 == 0x5) {
    pSVar2 = pstruct15_7->field16_0x14;
    ((int)pSVar2 + 0xa4) = 0x1e;
    uVar2 = pstruct15_7->field16_0x14;
    ((int)uVar2 + 0xac) = 0x1;
    iVar8 = pstruct15_7->field10_0xc;
    iVar3 = iVar8 + -0x1b;
    if (iVar3 == 0x0) {
      pSVar2 = pstruct15_7->field16_0x14;
      ((int)pSVar2 + 0xaa) = 0xa;
    }
    else {
      iVar3 = iVar8 + -0x1c;
      if (iVar3 == 0x0) {
        pSVar2 = pstruct15_7->field16_0x14;
        ((int)pSVar2 + 0xaa) = 0xb;
      }
      else {
        iVar3 = iVar8 + -0x1d;
        if (iVar3 == 0x0) {
          pSVar2 = pstruct15_7->field16_0x14;
          ((int)pSVar2 + 0xaa) = 0xc;
        }
      }
    }
    pass1_1028_b58e(param_2);
    uVar5 = *(u32 *)(iVar3 + 0x2e);
    iVar8 = 0xc;
    uVar6 = extraout_DX;
    pass1_1038_3fb0(uVar5);
    BVar4 = pass1_1030_25b2(uVar5 & 0xffff | (u32)uVar6 << 0x10,iVar8);
    if (BVar4 != 0x0) {
      pSVar2 = pstruct15_7->field16_0x14;
      piVar1 = (i16 *)((int)pSVar2 + 0xaa);
      *piVar1 = *piVar1 + 0x1;
    }
    BVar4 = pass1_1030_25b2(uVar5 & 0xffff | (u32)uVar6 << 0x10,0xe);
    if (BVar4 != 0x0) {
      pSVar2 = pstruct15_7->field16_0x14;
      piVar1 = (i16 *)((int)pSVar2 + 0xaa);
      *piVar1 = *piVar1 + 0x1;
    }
    BVar4 = pass1_1030_25b2(uVar5 & 0xffff | (u32)uVar6 << 0x10,0x76);
    if (BVar4 != 0x0) {
      pSVar2 = pstruct15_7->field16_0x14;
      piVar1 = (i16 *)((int)pSVar2 + 0xaa);
      *piVar1 = *piVar1 + 0x1;
    }
  }
  return;
}



void pass1_1030_bf6e(u32 param_1)

{
  u16 *puVar1;
  u32 uVar2;
  u16 uVar3;
  u16 uVar4;
  u32 in_EDX;
  i16 iVar5;
  u16 uVar6;
  u32 uVar7;
  u16 uVar8;
  u32 uStack6;

  uVar8 = 0x1e;
  uVar3 = pass1_1028_b58e((astruct_15 *)param_1);
  uVar4 = (u16)in_EDX;
  uStack6 = CONCAT22(uVar4,uVar3);
  uVar7 = pass1_1030_7c28(uVar3,uVar4,CONCAT22(uVar4,uVar3),uVar8);
  uVar4 = 0x3e8 - (int)uVar7;
  uVar2 = *(u32 *)((int)param_1 + 0x14);
  uVar6 = (u16)((u32)uVar2 >> 0x10);
  iVar5 = (int)uVar2;
  puVar1 = (u16 *)(iVar5 + 0xaa);
  uVar3 = -(u16)(uVar4 < *puVar1);
  pass1_1030_7ddc(uVar3,(astruct_57 *)(in_EDX & 0xffff0000 | uVar7 >> 0x10),uStack6,
                  (u32)((uVar4 - *puVar1 & uVar3) + (iVar5 + 0xaa)),0x1e);
  return;
}



u16 pass1_1030_bfb8(u32 param_1)

{
  u32 uVar1;
  u32 uVar2;
  u16 uVar3;

  uVar3 = 0x1e;
  uVar1 = pass1_1028_b58e((astruct_15 *)param_1);
  uVar2 = pass1_1030_7c28((u16)uVar1,(u16)(uVar1 >> 0x10),uVar1,uVar3);
  return 0x3e8 - (int)uVar2;
}



StructD * pass1_1030_bfe0(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void struct_1030_c06e(astruct_180 *param_1)

{
  astruct_180 *iVar1;
  u16 uVar1;

  struct_1028_b354(param_1);
  uVar1 = (u16)((u32)param_1 >> 0x10);
  iVar1 = (astruct_180 *)param_1;
  *(u32 *)(iVar1 + 0x1) = 0x0;
  &iVar1[0x1].field_0x4 = 0x0;
  param_1->field0_0x0 = 0xc68e;
  iVar1->field1_0x2 = 0x1030;
  return;
}



void pass1_1030_c09c(u16 param_1,astruct_12 *param_2,i16 param_3,u32 param_4)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  *(u32 *)((int)param_2 + 0x20) = 0x0;
  ((int)param_2 + 0x24) = 0x0;
  param_2->field0_0x0 = 0xc68e;
  ((int)param_2 + 0x2) = 0x1030;
  return;
}



u16 pass1_1030_c0d2(u32 param_1)

{
  if (0x0 < ((int)param_1 + 0x24)) {
    return 0x1;
  }
  return 0x0;
}



u16 pass1_1030_c0ec(u32 param_1)

{
  u16 uVar1;

  uVar1 = (u16)(param_1 >> 0x10);
  if ((((int)param_1 + 0xc) != 0xb) && (((int)param_1 + 0x24) < 0x1)) {
    return 0x0;
  }
  return 0x1;
}



void pass1_1030_c10e(u32 param_1)

{
  i16 *piVar1;
  i16 iVar2;
  u16 uVar3;

  uVar3 = (u16)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (0x0 < (iVar2 + 0x24)) {
    piVar1 = (i16 *)(iVar2 + 0x24);
    *piVar1 = *piVar1 + -0x1;
    return;
  }
  (iVar2 + 0xc) = 0x37;
  return;
}



void pass1_1030_c12e(i16 param_1,u32 param_2,i16 param_3)

{
  i16 *piVar1;
  u32 uVar2;
  i16 iVar3;
  u16 extraout_DX;
  i16 iVar4;
  u16 uVar5;
  u16 uVar6;
  astruct_397 *paStack6;

  pass1_1028_b58e((astruct_15 *)param_2);
  paStack6 = (astruct_397 *)CONCAT22(extraout_DX,param_1);
  uVar2 = *(u32 *)(param_1 + 0x2e);
  uVar5 = (u16)(param_2 >> 0x10);
  iVar4 = (int)param_2;
  iVar3 = (int)uVar2;
  if ((iVar4 + 0x24) < 0x1) {
    pass1_1030_7d1c(iVar3,extraout_DX,paStack6,0x0,0x230000);
  }
  else {
    if (param_3 == 0x0) {
      uVar6 = 0x0;
    }
    else {
      uVar6 = 0x32;
    }
    pass1_1030_7d1c(iVar3,extraout_DX,paStack6,uVar6,0x230000);
    piVar1 = (i16 *)(iVar4 + 0x24);
    *piVar1 = *piVar1 + -0x1;
  }
  if ((0x0 < (iVar4 + 0x24)) && ((iVar4 + 0x24) < 0x19)) {
    (iVar3 + 0x1fe) = 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_c1b2(uchar *param_1,astruct_695 *param_2)

{
  i16 iVar1;
  u16 in_register_0000000a;
  astruct_57 *paVar2;
  astruct_695 *iVar2;
  u16 uVar3;
  u32 *puVar4;
  u16 in_stack_0000fe94;
  u16 in_stack_0000fe96;
  u16 in_stack_0000ffb8;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc4;
  i16 iVar5;
  u16 in_stack_0000ffee;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  pass1_1028_be9e((astruct_15 *)param_2);
  uVar3 = (u16)((u32)param_2 >> 0x10);
  iVar2 = (astruct_695 *)param_2;
  if (iVar2->field17_0x12 == 0x5) {
    if (iVar2->field12_0xc == 0xb) {
      pass1_1030_c652((uchar *)paVar2,0xc1d7);
      iVar5 = 0x82;
      puVar4 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)0x820008,in_stack_0000fe94,in_stack_0000ffb8,
                               in_stack_0000ffbe,in_stack_0000ffc2);
      paVar2 = (astruct_57 *)((u32)paVar2 & 0xffff0000 | (u32)puVar4 >> 0x10);
      iVar1 = (int)puVar4;
      pass1_1010_9f8c((u32)puVar4,iVar5);
      iVar2->field34_0x24 = iVar1 * 0x3;
      mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffee,0x2),in_stack_0000fe96,
                      in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
      if ((int)PTR_LOOP_1050_13ae < 0x3) {
        iVar1 = iVar2->field34_0x24;
        if (iVar1 < 0x32) {
          iVar1 = 0x32;
        }
        iVar2->field34_0x24 = iVar1;
        return;
      }
    }
    else {
      iVar2->field34_0x24 = 0x64;
    }
  }
  return;
}



void pass1_1030_c230(u32 param_1,u32 param_2)

{
  BOOL16 BVar1;
  u16 uVar2;
  HFILE16 in_stack_0000ffd8;
  u32 local_10 [0x2];
  u16 local_8 [0x3];

  BVar1 = write_to_file_1028_b5ec((astruct_731 *)param_1,param_2);
  if (BVar1 != 0x0) {
    uVar2 = (u16)(param_1 >> 0x10);
    local_10[0] = *(u32 *)((int)param_1 + 0x20);
    BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_10),(char *)0x4,in_stack_0000ffd8);
    if (BVar1 != 0x0) {
      local_8[0] = ((int)param_1 + 0x24);
      BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_8),(char *)0x2,in_stack_0000ffd8);
      if (BVar1 != 0x0) {
        return;
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}



void pass1_1030_c29c(i16 param_1,uchar *param_2,astruct_373 *param_3,HFILE16 *param_4)

{
  BOOL16 BVar1;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 != 0x0) {
    BVar1 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | (u32)((int)param_3 + 0x20)),0x4);
    if (BVar1 != 0x0) {
      BVar1 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | (u32)((int)param_3 + 0x24)),0x2);
      if (BVar1 != 0x0) {
        return;
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_c2fa(i16 param_1,uchar *param_2,u32 param_3)

{
  u32 uVar1;
  u32 uVar2;
  u32 uVar3;
  i16 iVar4;
  u16 uVar5;
  u32 uVar6;
  u16 uVar7;
  u16 in_register_0000000a;
  astruct_57 *paVar8;
  u16 unaff_SI;
  u16 uVar9;
  u16 uVar10;
  u32 *puVar11;
  astruct_27 *paVar12;
  u16 in_stack_0000fe4e;
  u16 in_stack_0000ff72;
  u16 in_stack_0000ff78;
  u16 in_stack_0000ff7c;
  u16 uVar13;
  u16 uStack84;
  i32 lStack80;
  i16 iStack56;
  astruct_305 *paStack10;
  u32 uStack6;
  astruct_698 *iVar5;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar9 = (u16)(param_3 >> 0x10);
  if (((int)param_3 + 0xc) != 0xb) {
    pass1_1028_bd38((u16)param_2,(astruct_15 *)param_3);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,*(u32 *)((int)param_3 + 0x20));
    uVar7 = (u16)paVar8;
    uStack6 = CONCAT22(uVar7,param_1);
    iVar4 = param_1;
    pass1_1028_b58e((astruct_15 *)param_3);
    paStack10 = (astruct_305 *)CONCAT22((int)paVar8,iVar4);
    uVar1 = *(u32 *)(iVar4 + 0x2e);
    puVar11 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fe4e,
                              in_stack_0000ff72,in_stack_0000ff78,in_stack_0000ff7c);
    paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)puVar11 >> 0x10);
    uVar10 = (u16)(uVar1 >> 0x10);
    pass1_1010_ed22((u32)puVar11,*(u32 *)((int)uVar1 + 0x4));
    uVar2 = *(u32 *)(param_1 + 0x1f6);
    uVar6 = uVar2;
    pass1_1030_3694((uchar *)paVar8,uVar2,0x3,0x2);
    uVar9 = (u16)(uVar2 >> 0x10);
    uVar3 = *(u32 *)((int)uVar1 + 0x1f6);
    pass1_1030_355c(uVar3,uVar6 & 0xffff | (long)paVar8 << 0x10);
    uVar10 = (u16)(uVar3 >> 0x10);
    iStack56 = 0x0;
    do {
      iVar5 = (astruct_698 *)(iStack56 * 0x2);
      (iVar5 + (int)uVar3 + 0x174) = (iVar5 + (int)uVar2 + 0x174);
      uVar5 = (iVar5 + (int)uVar2 + 0x180);
      uVar6 = (u32)uVar5;
      (iVar5 + (int)uVar3 + 0x180) = uVar5;
      iStack56 += 0x1;
    } while (iStack56 < 0x6);
    for (uStack84 = 0x11; uVar5 = (u16)uVar6, (int)uStack84 < 0x25; uStack84 += 0x1) {
      if (0x0 < (uStack84 * 0x2 + (int)_PTR_LOOP_1050_580e)) {
        empty_1038_540a();
        lStack80 = CONCAT22((int)paVar8,uVar5);
        uVar9 = (u16)((u32)_PTR_LOOP_1050_580e >> 0x10);
        iVar4 = (uStack84 * 0x2 + (int)_PTR_LOOP_1050_580e);
        paVar8 = (astruct_57 *)((u32)(long)iVar4 >> 0x10);
        uVar13 = uStack84;
        if (lStack80 < iVar4) {
          iVar4 = (uStack84 * 0x2 + (int)_PTR_LOOP_1050_580e);
          paVar8 = (astruct_57 *)(u32)(u16)(iVar4 >> 0xf);
          uVar13 = 0x21;
        }
        pass1_1038_52b8(uStack6,CONCAT22((int)paVar8,iVar4),uVar13);
        uVar5 = (uStack84 * 0x2 + (int)_PTR_LOOP_1050_580e);
        pass1_1030_7ddc(uVar5,paVar8,(u32)paStack10,(long)(int)uVar5,uStack84);
        iVar4 = ((int)_PTR_LOOP_1050_580e + uStack84 * 0x2);
        uVar6 = (u32)iVar4;
        pass1_1038_5694(uVar1,(long)iVar4,uStack84);
      }
    }
    pass1_1030_7c50(uVar5,paVar8,paStack10,0x2,0x1);
    pass1_1030_7c50(uVar5,paVar8,paStack10,0x2,0x2);
    pass1_1030_7c50(uVar5,paVar8,paStack10,0x2,0x3);
    pass1_1030_7c50(uVar5,paVar8,paStack10,0x2,0x4);
    pass1_1038_44d8(uVar5,(int)paVar8,param_1,uVar7,0x2,0x1);
    pass1_1038_44d8(uVar5,(int)paVar8,param_1,uVar7,0x2,0x2);
    pass1_1038_44d8(uVar5,(int)paVar8,param_1,uVar7,0x2,0x3);
    pass1_1038_44d8(uVar5,(int)paVar8,param_1,uVar7,0x2,0x4);
    paVar12 = (astruct_27 *)
              mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2b),in_stack_0000fe4e,
                              in_stack_0000ff72,in_stack_0000ff78,in_stack_0000ff7c);
    pass1_1010_043a(paVar12,*(i32 *)(param_1 + 0x4),0x7);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_c52e(i16 param_1,u16 param_2,u32 param_3,u16 *param_4,u32 param_5,u32 param_6)

{
  BOOL16 BVar1;
  u32 *puVar2;
  astruct_92 *paVar3;
  u32 *puVar4;
  u32 uVar5;
  u16 uVar6;
  u16 uVar7;
  u32 uVar8;
  u16 uVar9;
  u16 *puVar10;
  astruct_92 local_32;
  u32 local_20;
  u32 uStack28;
  u32 *puStack24;
  u32 uStack22;
  u16 uStack18;
  u16 uStack16;
  u32 local_c;
  u16 uStack8;
  u32 uStack6;

  uVar9 = (u16)(param_3 >> 0x10);
  BVar1 = pass1_1028_c314(param_1,param_2,(u16)param_3,uVar9,param_4,(u16)param_5,(u16)(param_5 >> 0x10),
                          param_6);
  if (BVar1 != 0x0) {
    puVar2 = &local_c;
    pass1_1030_64ce(puVar2,param_2,_PTR_LOOP_1050_5740,param_4,param_6,(u32 *)CONCAT22(0x1050,puVar2));
    local_20 = *puVar2;
    local_20._3_1_ = (u8)(local_20 >> 0x18);
    uStack8 = (u16)local_20._3_1_;
    if (local_20._3_1_ == 0x0) {
      uStack22 = local_20;
      uStack6 = local_20;
      pass1_1028_c7b6(param_2,(u16)param_3,uVar9,param_4,param_6);
      if ((uStack8 != 0x4) && (uStack8 != 0x0)) {
        uVar8 = pass1_1030_bcae((u16)&local_20,(u16)&DAT_1050_1050);
        uVar6 = (u16)(uVar8 >> 0x10);
        pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_32),0x1,0x0,0x400);
        while( true ) {
          paVar3 = &local_32;
          pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar3));
          uStack28 = CONCAT22(uVar6,paVar3);
          uVar7 = uVar6 | (u16)paVar3;
          if (uVar7 == 0x0) {
            return;
          }
          uVar5 = *(u32 *)&paVar3->field6_0x10;
          uVar8 = param_6;
          uStack22 = uVar5;
          puVar10 = param_4;
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5);
          uStack18 = (u16)uVar5;
          puVar4 = &local_20;
          uStack16 = uVar7;
          pass1_1030_bcde((u16)puVar4,(u16)&DAT_1050_1050,uVar5 & 0xffff | (u32)uVar7 << 0x10,puVar10,uVar8);
          if ((int)puVar4 < 0x0) break;
          uVar6 = uVar7;
          puStack24 = puVar4;
          if ((int)puVar4 < 0x1f) {
            PTR_LOOP_1050_50ca = (u8 *)0x6ae;
            return;
          }
        }
        PTR_LOOP_1050_50ca = (u8 *)0x6af;
        return;
      }
      PTR_LOOP_1050_50ca = (u8 *)0x6a8;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: stack

void pass1_1030_c652(uchar *param_1,u16 param_2)

{
  u16 in_register_0000000a;
  astruct_250 *paVar1;
  u16 in_stack_0000fea8;
  u16 in_stack_0000ffcc;
  u16 in_stack_0000ffd2;
  u16 in_stack_0000ffd6;

  paVar1 = (astruct_250 *)
           mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(param_2,0x8),in_stack_0000fea8,in_stack_0000ffcc,in_stack_0000ffd2,
                           in_stack_0000ffd6);
  pass1_1010_9794(paVar1);
  return;
}



StructD * pass1_1030_c668(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_180 * struct_1030_c6f6(astruct_180 *param_1)

{
  u16 uVar1;

  struct_1028_b354(param_1);
  uVar1 = (u16)((u32)param_1 >> 0x10);
  ((int)param_1 + 0x20) = 0x0;
  param_1->field0_0x0 = 0xc940;
  ((int)param_1 + 0x2) = 0x1030;
  return param_1;
}



u16 * pass1_1030_c71e(u16 param_1,astruct_12 *param_2,i16 param_3,u32 param_4)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  ((int)param_2 + 0x20) = 0x0;
  param_2->field0_0x0 = 0xc940;
  ((int)param_2 + 0x2) = 0x1030;
  return &param_2->field0_0x0;
}



void pass1_1030_c74e(u16 param_1,astruct_15 *param_2,u32 param_3)

{
  pass1_1028_b46e(param_1,param_2,param_3);
  ((int)param_2 + 0x20) = 0x70;
  return;
}



void pass1_1030_c76c(u32 *param_1)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = (u16)((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (((iVar1 + 0x12) != 0x6) && ((iVar1 + 0x12) != 0x5)) {
    return;
  }
  iVar1 = (iVar1 + 0x20);
  if (iVar1 != 0x0) {
    if (((iVar1 < 0x70) || (SBORROW2(iVar1,0x70))) || (0x1 < iVar1 + -0x70)) {
      pass1_1028_be2a((astruct_15 *)param_1);
      return;
    }
  }
  pass1_1028_bdac((astruct_15 *)param_1,0x6);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_c7b0(u32 param_1)

{
  i16 iVar1;
  u32 uVar2;
  u32 uVar3;
  i16 iVar4;
  i16 iVar5;
  BOOL16 BVar6;
  u32 uVar7;
  uchar *extraout_DX;
  uchar *puVar8;
  i16 iVar9;
  u16 uVar10;

  uVar10 = (u16)(param_1 >> 0x10);
  iVar9 = (int)param_1;
  iVar1 = (iVar9 + 0x20);
  if (iVar1 != 0x0) {
    iVar4 = iVar1 + -0x70;
    iVar5 = iVar4;
    if (((iVar1 < 0x70) || (SBORROW2(iVar1,0x70))) || (iVar5 = iVar1 + -0x71, iVar5 != 0x0 && 0x0 < iVar4)) {
      pass1_1028_b58e((astruct_15 *)(param_1 & 0xffff | (u32)uVar10 << 0x10));
      uVar2 = *(u32 *)(iVar5 + 0x2e);
      uVar7 = *(u32 *)((int)uVar2 + 0x200);
      puVar8 = extraout_DX;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7);
      uVar3 = uVar7 & 0xffff | ZEXT24(puVar8) << 0x10;
      BVar6 = pass1_1008_c6ae(_u16_1050_06e0,(iVar9 + 0xc),0x11);
      pass1_1030_23e2(BVar6,puVar8,uVar3,BVar6,(iVar9 + 0x20));
      if (BVar6 != 0x0) {
        if ((iVar9 + 0x20) == 0x1) {
          pass1_1030_25d8(uVar3,0x64,(iVar9 + 0x20));
          return;
        }
        (iVar9 + 0x20) = 0x70;
      }
    }
  }
  return;
}



BOOL16 pass1_1030_c84e(u32 param_1,u32 param_2)

{
  BOOL16 BVar1;
  HFILE16 in_stack_0000ffde;
  u16 local_c [0x5];

  BVar1 = write_to_file_1028_b5ec((astruct_731 *)param_1,param_2);
  if (BVar1 != 0x0) {
    local_c[0] = ((int)param_1 + 0x20);
    BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffde);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return BVar1;
    }
    BVar1 = 0x1;
  }
  return BVar1;
}



BOOL16 pass1_1030_c894(BOOL16 param_1,uchar *param_2,astruct_373 *param_3,HFILE16 *param_4)

{
  BOOL16 BVar1;
  u16 local_4;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 != 0x0) {
    BVar1 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_4),0x2);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return BVar1;
    }
    ((int)param_3 + 0x20) = local_4;
    param_1 = 0x1;
  }
  return param_1;
}



u32 pass1_1030_c8da(u32 param_1,u32 param_2,u32 param_3)

{
  u32 uVar1;

  uVar1 = 0x0;
  if (param_3 == 0x1) {
    ((int)param_1 + 0x20) = param_2;
  }
  else {
    uVar1 = FUN_1030_178e();
  }
  return uVar1;
}



StructD * pass1_1030_c91a(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_180 * struct_1030_c9a8(astruct_180 *param_1)

{
  astruct_180 *iVar1;
  u16 uVar1;

  struct_1028_b354(param_1);
  uVar1 = (u16)((u32)param_1 >> 0x10);
  iVar1 = (astruct_180 *)param_1;
  iVar1[0x4].field16_0x18 = 0x1;
  param_1->field0_0x0 = 0xd88e;
  iVar1->field1_0x2 = 0x1030;
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1 + 0x1)),NULL,0x78);
  return param_1;
}



u32 pass1_1030_c9e4(u16 param_1,astruct_12 *param_2,i16 param_3,u32 param_4)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  ((int)param_2 + 0x98) = 0x1;
  param_2->field0_0x0 = 0xd88e;
  ((int)param_2 + 0x2) = 0x1030;
  pass1_1000_4906((StructD *)((u32)param_2 & 0xffff0000 | (u32)((int)param_2 + 0x20)),NULL,0x78);
  return (u32)param_2;
}



void pass1_1030_ca26(u16 param_1,astruct_15 *param_2,u32 param_3)

{
  u16 uVar1;
  u16 extraout_DX;
  i16 iVar2;
  u16 uVar3;
  u16 uStack4;

  for (uStack4 = 0x0; iVar2 = (int)param_2, uVar3 = (u16)((u32)param_2 >> 0x10), (int)uStack4 < 0xa;
      uStack4 += 0x1) {
    if (((iVar2 + uStack4 * 0xc + 0x26) == 0x2) || ((iVar2 + uStack4 * 0xc + 0x26) == 0x1)) {
      (iVar2 + uStack4 * 0xc + 0x26) = 0x4;
      param_1 = uStack4;
    }
    else {
      uVar1 = uStack4;
      pass1_1028_b58e(param_2);
      iVar2 = uStack4 * 0xc + iVar2;
      pass1_1030_6e9c((astruct_301 *)CONCAT22(extraout_DX,uVar1),0x1,(iVar2 + 0x24));
      param_1 = 0x0;
      *(u32 *)(iVar2 + 0x20) = 0x0;
      (iVar2 + 0x24) = 0x0;
      (iVar2 + 0x26) = 0x0;
    }
  }
  pass1_1028_b46e(param_1,param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_cac2(i16 param_1,u32 *param_2)

{
  u32 uVar1;
  u32 *puVar2;
  code **ppcVar3;
  u32 uVar4;
  u16 uVar5;
  u32 uVar6;
  u32 *puVar7;
  u32 uVar8;
  u16 extraout_DX;
  u16 extraout_DX_00;
  u16 extraout_DX_01;
  u16 uVar9;
  u16 uVar10;
  u32 uStack34;
  i16 iStack30;
  i16 iStack28;

  pass1_1028_be9e((astruct_15 *)param_2);
  uVar10 = (u16)((u32)param_2 >> 0x10);
  if ((((int)param_2 + 0x12) == 0x5) && (PTR_LOOP_1050_5812 == NULL)) {
    PTR_LOOP_1050_5812 = (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1);
    pass1_1028_b58e((astruct_15 *)((u32)param_2 & 0xffff | (u32)uVar10 << 0x10));
    uVar1 = *(u32 *)(param_1 + 0x2e);
    uVar6 = *(u32 *)((int)uVar1 + 0x10);
    uVar10 = extraout_DX;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar6);
    puVar2 = (u32 *)*(u32 *)((int)uVar6 + 0x1e);
    ppcVar3 = (code **)((int)*puVar2 + 0x10);
    puVar7 = puVar2;
    (**ppcVar3)(0x1028,(int)puVar2,((int)uVar6 + 0x20));
    uVar4 = (u32)puVar7 & 0xffff | (u32)extraout_DX_00 << 0x10;
    iStack28 = 0x0;
    iStack30 = pass1_1030_d144((u32)param_2);
    uStack34 = 0x0;
    while ((uStack34 < uVar4 && (iStack30 != 0x0))) {
      ppcVar3 = (code **)((int)*puVar2 + 0x4);
      uVar8 = uVar4;
      (**ppcVar3)(0x1028,(int)puVar2,(int)((u32)puVar2 >> 0x10),(char)uStack34,(int)(uStack34 >> 0x10));
      uVar5 = (u16)uVar8;
      uVar9 = extraout_DX_01 | uVar5;
      if (uVar9 != 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar8 & 0xffff | (u32)extraout_DX_01 << 0x10);
        uVar5 = (uVar5 + 0xc);
        if ((0x0 < (int)uVar5) && (!SBORROW2(uVar5,0x1))) {
          if (uVar5 != 0x3 && 0x0 < (int)(uVar5 - 0x2)) {
            if (uVar5 != 0x4) goto LAB_1030_cbbc;
            iStack28 += 0x1;
          }
          pass1_1030_6e9c((astruct_301 *)(uVar6 & 0xffff | (u32)uVar10 << 0x10),0x1,uVar5);
          pass1_1030_d180((u32)param_2,uVar5);
          iStack30 += -0x1;
        }
      }
LAB_1030_cbbc:
      uStack34 += 0x1;
    }
    while (iStack28 < 0x4) {
      pass1_1030_d180((u32)param_2,0x4);
      iStack28 = iStack28 + 0x1;
    }
  }
  return;
}



u16 pass1_1030_cbf0(i16 param_1,u16 param_2,i16 param_3)

{
  astruct_595 *iVar1;
  i16 iStack4;

  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return 0x0;
    }
    iVar1 = (astruct_595 *)(iStack4 * 0xc + param_1);
    if ((iVar1->field36_0x24 == param_3) && (iVar1->field37_0x26 == 0x3)) break;
    iStack4 += 0x1;
  }
  iVar1->field37_0x26 = 0x0;
  iVar1->field38_0x28 = 0x0;
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_cc44(i16 param_1,u16 param_2,i16 param_3,u32 param_4,i16 param_5)

{
  code **ppcVar1;
  i16 iVar2;
  u8 *puVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  u16 extraout_DX;
  u16 extraout_DX_00;
  uchar *puVar7;
  u16 extraout_DX_01;
  astruct_304 *iVar7;
  astruct_303 *iVar8;
  u8 uVar8;
  u32 *puVar9;
  u32 *puVar10;
  uchar *puVar11;
  u8 local_32 [0x8];
  u32 *puStack42;
  u32 uStack38;
  u32 uStack34;
  u32 *puStack30;
  u16 uStack26;
  uchar *puStack24;
  u16 uStack22;
  uchar *puStack20;
  u32 *puStack18;
  i16 iStack14;
  u16 uStack12;
  i16 iStack10;
  u32 uStack8;
  i16 iStack4;

  iStack4 = 0x0;
  uStack8 = *(u32 *)((int)param_4 + 0x4);
  iStack10 = 0x0;
  do {
    if (0x9 < iStack10) {
      return;
    }
    iVar8 = (astruct_303 *)(iStack10 * 0xc + param_1);
    if (((iVar8->field35_0x28 == (int)uStack8) && (iVar8->field36_0x2a == uStack8)) &&
       (iVar8->field33_0x24 == param_5)) {
      if (iVar8->field34_0x26 == 0x4) {
        iVar2 = param_5;
        pass1_1028_b58e((astruct_15 *)CONCAT22(param_2,param_1));
        iStack14 = iVar2;
        uStack12 = extraout_DX_00;
        pass1_1030_6e9c((astruct_301 *)
                        CONCAT13((char)((u16)extraout_DX_00 >> 0x8),CONCAT12((char)extraout_DX_00,iStack14)),0x1,
                        iVar8->field33_0x24);
        iVar8->field32_0x20 = 0x0;
        iVar8->field33_0x24 = 0x0;
        iVar8->field34_0x26 = 0x0;
        puStack42 = NULL;
        puStack18 = NULL;
        _DAT_0000_0006 = param_5;
        uRam0000000a = 0x1;
        uVar4 = switch_1020_c3b4(param_5);
        ((int)puStack18 + 0xc) = uVar4;
        puVar10 = pass1_1008_c6fa(_u16_1050_06e0,0x4);
        puVar7 = (uchar *)((u32)puVar10 >> 0x10);
        uVar6 = (u16)puVar10;
        uVar5 = uVar6;
        puVar11 = puVar7;
        uStack22 = uVar6;
        puStack20 = puVar7;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
        uVar8 = 0x38;
        uStack26 = uVar6;
        puStack24 = puVar7;
        pass1_1038_4d6e(uVar6,puVar7,(astruct_691 *)CONCAT22(puVar7,uVar6),(u32 *)CONCAT22(puVar11,uVar5));
        puStack30 = (u32 *)CONCAT22(puVar7,uVar6);
        ppcVar1 = (code **)((int)*puStack30 + 0x10);
        (**ppcVar1)((int)&u16_1050_1038,uVar6,puVar7);
        uStack34 = CONCAT22(extraout_DX_01,uVar6);
        uVar6 = extraout_DX_01;
        for (uStack38 = 0x0; uStack38 < uStack34; uStack38 += 0x1) {
          puVar9 = (u32 *)pass1_1030_1d7c((int)uStack34,uVar6,(u32)puStack30);
          uVar5 = (u16)((u32)puVar9 >> 0x10);
          uVar6 = uVar5 | (u16)puVar9;
          if (uVar6 != 0x0) {
            puVar3 = local_32;
            ppcVar1 = (code **)((int)*puVar9 + 0x40);
            (**ppcVar1)(0x38,(char)puVar9,uVar5,puVar3,(int)&DAT_1050_1050);
            uVar6 = extraout_DX;
            if (puVar3 == NULL) {
              uVar8 = 0x28;
              pass1_1028_6408((u32)puVar9,puStack18);
              break;
            }
          }
        }
        puStack42 = puStack30;
        if (puStack30 != NULL) {
          ppcVar1 = (code **)*puStack30;
          (**ppcVar1)(uVar8,(int)puStack30,(int)((u32)puStack30 >> 0x10),0x1);
        }
      }
      else {
        iVar7 = (astruct_304 *)(iStack10 * 0xc + param_1);
        iVar7->field38_0x26 = 0x0;
        iVar7->field39_0x28 = 0x0;
      }
      iStack4 += 0x1;
      param_3 += -0x1;
      if (param_3 == 0x0) {
        return;
      }
    }
    iStack10 += 0x1;
  } while( true );
}



i16 pass1_1030_cde8(i16 param_1,u16 param_2,i16 param_3)

{
  i16 iVar1;
  i16 iStack4;

  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return -0x1;
    }
    iVar1 = iStack4 * 0xc + param_1;
    if (((iVar1 + 0x24) == param_3) && ((iVar1 + 0x26) == 0x0)) break;
    iStack4 += 0x1;
  }
  return iStack4;
}



// WARNING: Could not reconcile some variable overlaps

i16 pass1_1030_ce2e(i16 param_1,u16 param_2,i16 param_3)

{
  i16 iVar1;
  u32 uStack6;

  for (uStack6 = 0x0; (int)uStack6 < 0xa; uStack6 = uStack6 & 0xffff0000 | (u32)((int)uStack6 + 0x1)) {
    iVar1 = (int)uStack6 * 0xc + param_1;
    if (((iVar1 + 0x24) == param_3) && ((iVar1 + 0x26) == 0x0)) {
      uStack6 = uStack6 & 0xffff | (u32)(uStack6 + 0x1) << 0x10;
    }
  }
  return uStack6;
}



void pass1_1030_ce72(u32 param_1,i16 param_2,u32 param_3,i16 param_4)

{
  i32 lVar1;
  astruct_300 *iVar2;
  i16 iStack10;

  lVar1 = *(i32 *)((int)param_3 + 0x4);
  iStack10 = 0x0;
  do {
    if (0x9 < iStack10) {
      return;
    }
    iVar2 = (astruct_300 *)(iStack10 * 0xc + (int)param_1);
    if ((iVar2->field36_0x24 == param_4) && (iVar2->field38_0x28 == 0x0)) {
      iVar2->field38_0x28 = lVar1;
      if (param_4 == 0x4) {
        iVar2->field37_0x26 = 0x2;
      }
      else {
        ((int)param_1 + iStack10 * 0xc + 0x26) = 0x1;
      }
      param_2 += -0x1;
      if (param_2 == 0x0) {
        return;
      }
    }
    iStack10 += 0x1;
  } while( true );
}



void pass1_1030_cef8(u32 param_1,u32 param_2,u16 param_3,i16 param_4)

{
  u16 uVar1;
  i16 iVar2;
  u16 uVar3;
  u16 uVar4;

  uVar3 = (u16)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  (iVar2 + param_4 * 0xc + 0x26) = param_3;
  uVar4 = (u16)(param_2 >> 0x10);
  uVar1 = ((int)param_2 + 0x6);
  (iVar2 + param_4 * 0xc + 0x28) = ((int)param_2 + 0x4);
  (iVar2 + param_4 * 0xc + 0x2a) = uVar1;
  return;
}



u16 pass1_1030_cf3a(u32 param_1,i16 param_2)

{
  i16 iStack4;

  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return 0x0;
    }
    if (((int)param_1 + iStack4 * 0xc + 0x24) == param_2) break;
    iStack4 += 0x1;
  }
  return 0x1;
}



void pass1_1030_cf78(astruct_15 *param_1,u16 param_2)

{
  u32 uVar1;
  u16 extraout_DX;
  astruct_680 *iVar3;
  u16 uVar2;
  i16 iStack4;

  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return;
    }
    uVar1 = (u32)param_2;
    uVar2 = (u16)((u32)param_1 >> 0x10);
    if (((int)param_1 + iStack4 * 0xc + 0x24) == param_2) break;
    iStack4 += 0x1;
  }
  pass1_1028_b58e(param_1);
  if (param_2 == 0x5) {
    pass1_1038_4900(*(u32 *)((int)uVar1 + 0x2e));
  }
  else {
    pass1_1030_6e9c((astruct_301 *)(uVar1 & 0xffff | (u32)extraout_DX << 0x10),0x1,param_2);
  }
  iVar3 = (astruct_680 *)(iStack4 * 0xc + (int)param_1);
  iVar3->field32_0x20 = 0x0;
  iVar3->field33_0x24 = 0x0;
  iVar3->field34_0x26 = 0x0;
  return;
}



void pass1_1030_d00c(astruct_15 *param_1,u16 param_2)

{
  u32 uVar1;
  u16 extraout_DX;
  astruct_696 *local_BX_40;
  i16 iVar2;
  i16 iStack4;

  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return;
    }
    iVar2 = iStack4 * 0xc + (int)param_1;
    if (((iVar2 + 0x26) == 0x0) && (uVar1 = (u32)param_2, (iVar2 + 0x24) == param_2)) break;
    iStack4 += 0x1;
  }
  pass1_1028_b58e(param_1);
  if (param_2 == 0x5) {
    pass1_1038_4900(*(u32 *)((int)uVar1 + 0x2e));
  }
  else {
    pass1_1030_6e9c((astruct_301 *)(uVar1 & 0xffff | (u32)extraout_DX << 0x10),0x1,param_2);
  }
  local_BX_40 = (astruct_696 *)(iStack4 * 0xc + (int)param_1);
  local_BX_40->field32_0x20 = 0x0;
  local_BX_40->field33_0x24 = 0x0;
  local_BX_40->field34_0x26 = 0x0;
  return;
}



u16 pass1_1030_d0a8(u32 param_1)

{
  u16 uVar1;
  u16 uVar2;

  uVar2 = (u16)(param_1 >> 0x10);
  uVar1 = ((int)param_1 + 0x98);
  pass1_1030_d56a(param_1 & 0xffff | (u32)uVar2 << 0x10);
  return uVar1;
}



// WARNING: Could not reconcile some variable overlaps

i16 pass1_1030_d0c6(u32 param_1)

{
  u32 uStack6;

  for (uStack6 = 0x0; (int)uStack6 < 0xa; uStack6 = uStack6 & 0xffff0000 | (u32)((int)uStack6 + 0x1)) {
    if (*(i32 *)((int)param_1 + (int)uStack6 * 0xc + 0x20) != 0x0) {
      uStack6 = uStack6 & 0xffff | (u32)(uStack6 + 0x1) << 0x10;
    }
  }
  return uStack6;
}



// WARNING: Could not reconcile some variable overlaps

i16 pass1_1030_d102(i16 param_1,u16 param_2)

{
  i16 iVar1;
  u32 uStack6;

  for (uStack6 = 0x0; (int)uStack6 < 0xa; uStack6 = uStack6 & 0xffff0000 | (u32)((int)uStack6 + 0x1)) {
    iVar1 = (int)uStack6 * 0xc + param_1;
    if ((*(i32 *)(iVar1 + 0x20) != 0x0) && ((iVar1 + 0x26) != 0x0)) {
      uStack6 = uStack6 & 0xffff | (u32)(uStack6 + 0x1) << 0x10;
    }
  }
  return uStack6;
}



// WARNING: Could not reconcile some variable overlaps

i16 pass1_1030_d144(u32 param_1)

{
  u32 uStack6;

  for (uStack6 = 0x0; (int)uStack6 < 0xa; uStack6 = uStack6 & 0xffff0000 | (u32)((int)uStack6 + 0x1)) {
    if (*(i32 *)((int)param_1 + (int)uStack6 * 0xc + 0x20) == 0x0) {
      uStack6 = uStack6 & 0xffff | (u32)(uStack6 + 0x1) << 0x10;
    }
  }
  return uStack6;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_d180(u32 param_1,u16 param_2)

{
  u16 uVar1;
  u32 in_EDX;
  u16 uVar3;
  i16 iVar4;
  u16 uVar5;
  i16 iStack4;
  astruct_57 *paVar2;

  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return;
    }
    uVar5 = (u16)(param_1 >> 0x10);
    uVar3 = (u16)param_1;
    if (((uVar3 + iStack4 * 0xc + 0x22) | (uVar3 + iStack4 * 0xc + 0x20)) == 0x0) break;
    iStack4 += 0x1;
  }
  uVar1 = ((int)_PTR_LOOP_1050_65e2 + 0x2) + (u16)(0xff37 < *_PTR_LOOP_1050_65e2);
  paVar2 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uVar1);
  iVar4 = iStack4 * 0xc + uVar3;
  (iVar4 + 0x20) = *_PTR_LOOP_1050_65e2 + 0xc8;
  (iVar4 + 0x22) = uVar1;
  (iVar4 + 0x24) = param_2;
  uVar1 = param_2;
  pass1_1030_d340(uVar3,uVar5,param_1 & 0xffff0000 | (u32)(iVar4 + 0x20));
  pass1_1028_b58e((astruct_15 *)param_1);
  if (param_2 == 0x5) {
    pass1_1038_48e0(*(u32 *)(uVar1 + 0x2e),0x1);
    return;
  }
  pass1_1030_7c50(uVar1,paVar2,(astruct_305 *)CONCAT22((int)paVar2,uVar1),0x1,param_2);
  return;
}



u16 pass1_1030_d230(u32 param_1)

{
  i16 iStack4;

  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return 0x1;
    }
    if (*(i32 *)((int)param_1 + iStack4 * 0xc + 0x20) == 0x0) break;
    iStack4 += 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_d26c(u32 param_1)

{
  u32 *puVar1;
  u32 uVar2;
  i16 iVar3;
  u32 uVar4;
  u16 extraout_DX;
  i16 iVar5;
  i16 iStack8;

  uVar2 = *_PTR_LOOP_1050_65e2;
  for (iStack8 = 0x0; iStack8 < 0xa; iStack8 += 0x1) {
    iVar5 = iStack8 * 0xc + (int)param_1;
    if ((((iVar5 + 0x22) | (iVar5 + 0x20)) != 0x0) &&
       (puVar1 = (u32 *)(iVar5 + 0x20), *puVar1 < uVar2 || *puVar1 == uVar2)) {
      uVar4 = uVar2;
      pass1_1030_d3b2((int)uVar2,(int)param_1,param_1,iStack8);
      iVar3 = (int)uVar4;
      if (iVar3 == 0x0) {
        pass1_1028_b58e((astruct_15 *)param_1);
        if ((iVar5 + 0x24) == 0x5) {
          pass1_1038_4900(*(u32 *)(iVar3 + 0x2e));
        }
        else {
          pass1_1030_6e9c((astruct_301 *)CONCAT22(extraout_DX,iVar3),0x1,((int)param_1 + iStack8 * 0xc + 0x24));
        }
        iVar5 = iStack8 * 0xc + (int)param_1;
        *(u32 *)(iVar5 + 0x20) = 0x0;
        (iVar5 + 0x24) = 0x0;
        (iVar5 + 0x26) = 0x0;
      }
    }
  }
  return;
}



void pass1_1030_d340(u16 param_1,u16 param_2,u32 param_3)

{
  i16 iVar1;
  i16 iVar2;
  u16 uVar3;

  uVar3 = (u16)(param_3 >> 0x10);
  iVar2 = (int)param_3;
  iVar1 = (iVar2 + 0x4);
  if (((0x0 < iVar1) && (!SBORROW2(iVar1,0x1))) && ((iVar1 == 0x4 || iVar1 + -0x1 < 0x3 || (iVar1 == 0xc)))) {
    (iVar2 + 0x6) = 0x0;
    return;
  }
  (iVar2 + 0x6) = 0x1;
  return;
}



u16 pass1_1030_d36e(u32 param_1,i16 param_2)

{
  i16 iStack4;

  iStack4 = 0x0;
  while( true ) {
    if (0x9 < iStack4) {
      return 0x0;
    }
    if ((iStack4 != param_2) && (((int)param_1 + iStack4 * 0xc + 0x24) == 0x8)) break;
    iStack4 += 0x1;
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_d3b2(i16 param_1,i16 param_2,u16 param_3,i16 param_4)

{
  i16 iVar1;
  astruct_691 *paVar2;
  code **ppcVar3;
  bool bVar4;
  u16 uVar5;
  u16 uVar6;
  u16 extraout_DX;
  uchar *puVar7;
  u16 extraout_DX_00;
  u16 extraout_DX_01;
  u16 uVar8;
  u16 uVar9;
  u32 *puVar10;
  u32 uVar11;
  u32 *puStack26;
  u32 uStack18;
  u32 uStack14;

  pass1_1028_b58e((astruct_15 *)CONCAT22(param_3,param_2));
  paVar2 = *(astruct_691 **)(param_1 + 0x2e);
  uVar5 = pass1_1030_d36e(CONCAT22(param_3,param_2),param_4);
  if (uVar5 == 0x0) {
    puVar10 = pass1_1008_c6fa(_u16_1050_06e0,0x1e);
    puVar7 = (uchar *)((u32)puVar10 >> 0x10);
    uVar6 = (u16)puVar10;
    pass1_1038_4d6e(uVar6,puVar7,paVar2,puVar10);
    puStack26 = (u32 *)CONCAT22(puVar7,uVar6);
    ppcVar3 = (code **)((int)*puStack26 + 0x10);
    uVar8 = uVar6;
    (**ppcVar3)((int)&u16_1050_1038,uVar6,puVar7);
    uStack18 = CONCAT22(extraout_DX_00,uVar8);
    bVar4 = false;
    for (uStack14 = 0x0; uStack14 < uStack18; uStack14 += 0x1) {
      uVar11 = pass1_1030_1d7c((int)uStack14,uStack14,(u32)puStack26);
      uVar8 = (u16)(uVar11 >> 0x10);
      if ((((uVar8 | (u16)uVar11) != 0x0) && (*(i32 *)((u16)uVar11 + 0x4) != *(i32 *)(param_2 + 0x4))) &&
         (uVar5 = pass1_1030_cf3a(uVar11,0x8), uVar5 != 0x0)) {
        bVar4 = true;
        break;
      }
    }
    if (puStack26 != NULL) {
      ppcVar3 = (code **)*puStack26;
      (**ppcVar3)(0x38,uVar6,puVar7,0x1);
    }
    if (!bVar4) {
      return;
    }
  }
  puVar10 = pass1_1008_c6fa(_u16_1050_06e0,0x4);
  puVar7 = (uchar *)((u32)puVar10 >> 0x10);
  uVar6 = (u16)puVar10;
  uVar9 = SUB42(&u16_1050_1038,0x0);
  pass1_1038_4d6e(uVar6,puVar7,paVar2,puVar10);
  puStack26 = (u32 *)CONCAT22(puVar7,uVar6);
  ppcVar3 = (code **)((int)*puStack26 + 0x10);
  uVar8 = uVar6;
  (**ppcVar3)((int)&u16_1050_1038,uVar6,puVar7);
  uStack18 = CONCAT22(extraout_DX_01,uVar8);
  bVar4 = false;
  uStack14 = 0x0;
  do {
    if (uStack18 <= uStack14) {
LAB_1030_d51b:
      if (puStack26 != NULL) {
        ppcVar3 = (code **)*puStack26;
        (**ppcVar3)(uVar9,(char)uVar6,(char)puVar7,0x1);
      }
      if (!bVar4) {
        return;
      }
      uVar6 = *_PTR_LOOP_1050_65e2;
      iVar1 = ((int)_PTR_LOOP_1050_65e2 + 0x2);
      (param_2 + param_4 * 0xc + 0x20) = uVar6 + 0xc8;
      (param_2 + param_4 * 0xc + 0x22) = iVar1 + (u16)(0xff37 < uVar6);
      return;
    }
    uVar11 = pass1_1030_1d7c((int)uStack14,uStack14,(u32)puStack26);
    uVar8 = (u16)(uVar11 >> 0x10) | (u16)uVar11;
    if (uVar8 != 0x0) {
      uVar9 = 0x1028;
      uVar5 = pass1_1028_6744(uVar11,0x7);
      if ((uVar8 | uVar5) != 0x0) {
        uVar9 = 0x1028;
        pass1_1028_6228(uVar11,0x1,0x0,0x7);
        bVar4 = true;
        goto LAB_1030_d51b;
      }
    }
    uStack14 += 0x1;
  } while( true );
}



i16 pass1_1030_d56a(u32 param_1)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = (u16)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  switch((iVar1 + 0x98) + -0x1) {
  case 0x0:
    (iVar1 + 0x98) = 0x2;
    break;
  case 0x1:
    (iVar1 + 0x98) = 0x3;
    break;
  case 0x2:
    (iVar1 + 0x98) = 0x4;
    break;
  case 0x3:
    (iVar1 + 0x98) = 0xc;
    break;
  default:
    (iVar1 + 0x98) = 0x1;
    return iVar1;
  case 0x7:
    (iVar1 + 0x98) = 0x9;
    return iVar1;
  case 0x8:
    (iVar1 + 0x98) = 0xb;
    return iVar1;
  case 0xa:
    (iVar1 + 0x98) = 0x5;
    return iVar1;
  case 0xb:
    (iVar1 + 0x98) = 0x8;
    return iVar1;
  }
  return iVar1;
}



void pass1_1030_d61c(u32 param_1,u32 param_2)

{
  BOOL16 BVar1;
  i16 iVar2;
  u16 uVar3;
  HFILE16 in_stack_0000ffcc;
  u32 local_1a;
  u8 *local_16;
  u16 local_14;
  u32 local_12 [0x3];
  i16 iStack4;

  BVar1 = write_to_file_1028_b5ec((astruct_731 *)param_1,param_2);
  if (BVar1 != 0x0) {
    for (iStack4 = 0x0; iStack4 < 0xa; iStack4 += 0x1) {
      uVar3 = (u16)(param_1 >> 0x10);
      iVar2 = (int)param_1;
      local_12[0] = *(u32 *)(iVar2 + iStack4 * 0xc + 0x20);
      BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_12),(char *)0x4,in_stack_0000ffcc);
      if (BVar1 == 0x0) goto LAB_1030_d701;
      local_14 = (iVar2 + iStack4 * 0xc + 0x24);
      BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_14),(char *)0x2,in_stack_0000ffcc);
      if (BVar1 == 0x0) goto LAB_1030_d701;
      local_16 = (u8 *)(iVar2 + iStack4 * 0xc + 0x26);
      BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_16),(char *)0x2,in_stack_0000ffcc);
      if (BVar1 == 0x0) goto LAB_1030_d701;
      local_1a = *(u32 *)(iVar2 + iStack4 * 0xc + 0x28);
      BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_1a),(char *)0x4,in_stack_0000ffcc);
      if (BVar1 == 0x0) goto LAB_1030_d701;
    }
    local_16 = PTR_LOOP_1050_5812;
    BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_16),(char *)0x2,in_stack_0000ffcc);
    if (BVar1 != 0x0) {
      return;
    }
LAB_1030_d701:
    u16_1050_0310 = 0x6d0;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1030_d72e(i16 param_1,uchar *param_2,astruct_373 *param_3,HFILE16 *param_4)

{
  u16 uVar1;
  BOOL16 BVar2;
  i16 iVar3;
  i16 iStack10;
  u32 local_8;
  u16 local_4;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 == 0x0) {
    return;
  }
  iStack10 = 0x0;
  while( true ) {
    if (0x9 < iStack10) {
    // just 0x5812
      if ((0x3 < (int)u16_1050_0312) &&
         (BVar2 = read_file_1008_7dee(param_4,(u8 *)&PTR_LOOP_1050_5812,0x2), BVar2 == 0x0)) {
        u16_1050_0310 = 0x6d2;
        return;
      }
      return;
    }
    BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_8),0x4);
    if (BVar2 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
    BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_4),0x2);
    if (BVar2 == 0x0) break;
    iVar3 = iStack10 * 0xc + (int)param_3;
    (iVar3 + 0x20) = (u16)local_8;
    (iVar3 + 0x22) = local_8;
    uVar1 = switch_1008_72bc(param_4,local_4);
    (iVar3 + 0x24) = uVar1;
    if ((int)u16_1050_0312 < 0x2) {
      iVar3 = iStack10 * 0xc + (int)param_3;
      (iVar3 + 0x26) = 0x3;
      *(u32 *)(iVar3 + 0x28) = 0x0;
    }
    else {
      BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_4),0x2);
      if (BVar2 == 0x0) {
        u16_1050_0310 = 0x6d2;
        return;
      }
      BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_8),0x4);
      if (BVar2 == 0x0) {
        u16_1050_0310 = 0x6d2;
        return;
      }
      iVar3 = iStack10 * 0xc + (int)param_3;
      (iVar3 + 0x26) = local_4;
      *(u32 *)(iVar3 + 0x28) = local_8;
    }
    iStack10 += 0x1;
  }
  u16_1050_0310 = 0x6d2;
  return;
}



StructD * pass1_1030_d868(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_180 * struct_1030_d8f6(astruct_180 *param_1)

{
  astruct_180 *iVar1;
  u16 uVar1;

  struct_1028_b354(param_1);
  uVar1 = (u16)((u32)param_1 >> 0x10);
  iVar1 = (astruct_180 *)param_1;
  param_1->field0_0x0 = 0xdc2e;
  iVar1->field1_0x2 = 0x1030;
  if (iVar1->field10_0xc == 0x4c) {
    iVar1->field11_0xe = 0x43;
  }
  else if (iVar1->field10_0xc == 0x4d) {
    iVar1->field11_0xe = 0x44;
  }
  else {
    iVar1->field11_0xe = 0x45;
  }
  return param_1;
}



u32 pass1_1030_d942(u16 param_1,astruct_12 *param_2,i16 param_3,u32 param_4)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2->field0_0x0 = 0xdc2e;
  ((int)param_2 + 0x2) = 0x1030;
  if (((int)param_2 + 0xc) == 0x4c) {
    ((int)param_2 + 0xe) = 0x43;
  }
  else if (((int)param_2 + 0xc) == 0x4d) {
    ((int)param_2 + 0xe) = 0x44;
  }
  else {
    ((int)param_2 + 0xe) = 0x45;
  }
  return (u32)param_2;
}



void pass1_1030_d994(u32 *param_1)

{
  i16 *piVar1;
  u32 uVar2;
  i16 iVar3;
  i16 iVar4;
  u16 uVar5;
  u32 uVar6;

  uVar5 = (u16)((u32)param_1 >> 0x10);
  iVar4 = (int)param_1;
  if ((iVar4 + 0x12) != 0x4) {
    return;
  }
  uVar6 = pass1_1028_b4f2((astruct_15 *)param_1);
  iVar3 = (int)uVar6;
  if (*(i32 *)(iVar3 + 0x200) == 0x8000002) {
    uVar2 = *(u32 *)(iVar4 + 0x14);
    piVar1 = (i16 *)((int)uVar2 + 0x94);
    *piVar1 = *piVar1 + -0x1;
  }
  else {
    pass1_1028_cb04((astruct_15 *)param_1);
    if (iVar3 == 0x0) {
      return;
    }
    pass1_1030_dace((u32)param_1);
    if (iVar3 == 0x0) {
      return;
    }
    uVar2 = *(u32 *)(iVar4 + 0x14);
    piVar1 = (i16 *)((int)uVar2 + 0x94);
    *piVar1 = *piVar1 + -0x1;
    pass1_1028_c952((astruct_15 *)param_1);
    pass1_1030_da22((u32)param_1);
  }
  uVar2 = *(u32 *)(iVar4 + 0x14);
  if (((int)uVar2 + 0x94) < 0x1) {
    pass1_1028_bdac((astruct_15 *)param_1,0x5);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_da22(u32 param_1)

{
  u32 *puVar1;
  code **ppcVar2;
  u16 uVar3;
  BOOL16 BVar4;
  u16 uVar5;
  u32 *puVar6;
  u16 extraout_DX;
  u16 uVar7;
  u16 uVar8;
  u32 uVar9;
  u32 uStack18;

  uVar9 = pass1_1028_b4f2((astruct_15 *)param_1);
  uVar3 = (u16)(uVar9 >> 0x10);
  puVar1 = (u32 *)*(u32 *)((int)uVar9 + 0xc);
  ppcVar2 = (code **)((int)*puVar1 + 0x10);
  puVar6 = puVar1;
  (**ppcVar2)(0x1028,(int)puVar1,((int)uVar9 + 0xe));
  uStack18 = 0x0;
  while( true ) {
    if (((u32)puVar6 & 0xffff | (u32)extraout_DX << 0x10) <= uStack18) {
      return;
    }
    uVar9 = pass1_1030_1d7c((int)((u32)puVar6 & 0xffff),extraout_DX,(u32)puVar1);
    uVar7 = (u16)(uVar9 >> 0x10);
    uVar8 = uVar7 | (u16)uVar9;
    if (((uVar8 != 0x0) &&
        (BVar4 = pass1_1008_c6ae(_u16_1050_06e0,((u16)uVar9 + 0xc),0x4), BVar4 != 0x0)) &&
       (uVar5 = pass1_1028_6744(uVar9,0xd), (uVar8 | uVar5) != 0x0)) break;
    uStack18 += 0x1;
  }
  pass1_1028_6228(uVar9,0x1,0x0,0xd);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_dace(u32 param_1)

{
  u32 *puVar1;
  code **ppcVar2;
  u16 uVar3;
  BOOL16 BVar4;
  u16 uVar5;
  u32 *puVar6;
  u16 extraout_DX;
  u16 uVar7;
  u16 uVar8;
  u32 uVar9;
  u32 uStack20;

  uVar9 = pass1_1028_b4f2((astruct_15 *)param_1);
  uVar3 = (u16)(uVar9 >> 0x10);
  puVar1 = (u32 *)*(u32 *)((int)uVar9 + 0xc);
  ppcVar2 = (code **)((int)*puVar1 + 0x10);
  puVar6 = puVar1;
  (**ppcVar2)(0x1028,(int)puVar1,((int)uVar9 + 0xe));
  uStack20 = 0x0;
  uVar8 = extraout_DX;
  do {
    if (((u32)puVar6 & 0xffff | (u32)extraout_DX << 0x10) <= uStack20) {
      return;
    }
    uVar9 = pass1_1030_1d7c((int)((u32)puVar6 & 0xffff),uVar8,(u32)puVar1);
    uVar7 = (u16)(uVar9 >> 0x10);
    uVar8 = uVar7 | (u16)uVar9;
    if ((uVar8 != 0x0) && (BVar4 = pass1_1008_c6ae(_u16_1050_06e0,((u16)uVar9 + 0xc),0x4), BVar4 != 0x0)
       ) {
      uVar5 = pass1_1028_6744(uVar9,0xd);
      uVar8 |= uVar5;
      if (uVar8 != 0x0) {
        return;
      }
    }
    uStack20 += 0x1;
  } while( true );
}



u16 pass1_1030_db72(void)

{
  return 0x1;
}



void pass1_1030_db78(u32 param_1)

{
  u16 uVar1;

  uVar1 = (u16)(param_1 >> 0x10);
  if (((int)param_1 + 0x12) == 0x6) {
    pass1_1028_bdac((astruct_15 *)(param_1 & 0xffff | (u32)uVar1 << 0x10),0x5);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_db92(u16 param_1,u16 param_2,u16 *param_3,u32 param_4,i32 param_5)

{
  i16 iVar1;
  u8 *puVar2;
  u32 uVar3;
  u16 uVar4;
  u32 uVar5;
  u8 local_4 [0x2];

  uVar5 = pass1_1030_bcae((u16)local_4,(u16)&DAT_1050_1050);
  uVar4 = (u16)(uVar5 >> 0x10);
  iVar1 = (int)uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4);
  uVar3 = *(u32 *)(iVar1 + 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3);
  puVar2 = local_4;
  pass1_1030_bcde((u16)puVar2,(u16)&DAT_1050_1050,uVar3 & 0xffff | (u32)uVar4 << 0x10,param_3,param_5);
  if ((int)puVar2 < 0x0) {
    PTR_LOOP_1050_50ca = (u8 *)0x6af;
    return;
  }
  return;
}



u16 pass1_1030_dc02(void)

{
  return 0x1;
}



StructD * pass1_1030_dc08(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_180 * struct_1030_dc96(astruct_180 *param_1)

{
  u16 uVar1;

  struct_1028_b354(param_1);
  uVar1 = (u16)((u32)param_1 >> 0x10);
  *(u32 *)((int)param_1 + 0x20) = 0x0;
  param_1->field0_0x0 = 0xe036;
  ((int)param_1 + 0x2) = 0x1030;
  return param_1;
}



u16 * pass1_1030_dcc2(u16 param_1,astruct_12 *param_2,i16 param_3,u32 param_4)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  *(u32 *)((int)param_2 + 0x20) = 0x0;
  param_2->field0_0x0 = 0xe036;
  ((int)param_2 + 0x2) = 0x1030;
  return &param_2->field0_0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_dcf4(u16 param_1,astruct_15 *param_2)

{
  code **ppcVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;
  u16 extraout_DX;
  u16 uVar5;
  uchar *puVar6;
  u16 extraout_DX_00;
  u16 uVar7;
  astruct_15 *iVar9;
  u16 uVar8;
  u32 *puVar9;
  u32 uVar10;
  u32 uStack28;
  u32 uStack24;
  u32 *puStack20;
  i16 iStack12;

  uVar8 = (u16)((u32)param_2 >> 0x10);
  iVar9 = (astruct_15 *)param_2;
  param_2->field0_0x0 = 0xe036;
  iVar9->field1_0x2 = 0x1030;
  if (_PTR_LOOP_1050_65e2 != 0x0) {
    pass1_1028_b58e(param_2);
    if (*(i32 *)&iVar9->field24_0x20 == 0x0) {
      uVar2 = extraout_DX | param_1;
      if (uVar2 == 0x0) {
        uVar5 = extraout_DX;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
      }
      else {
        uVar2 = (param_1 + 0x2e);
        uVar5 = (param_1 + 0x30);
      }
      puVar9 = pass1_1008_c6fa(_u16_1050_06e0,0x1e);
      puVar6 = (uchar *)((u32)puVar9 >> 0x10);
      uVar3 = (u16)puVar9;
      pass1_1038_4d6e(uVar3,puVar6,(astruct_691 *)CONCAT22(uVar5,uVar2),puVar9);
      puStack20 = (u32 *)CONCAT22(puVar6,uVar3);
      ppcVar1 = (code **)((int)*puStack20 + 0x10);
      uVar5 = uVar3;
      (**ppcVar1)((int)&u16_1050_1038,uVar3,puVar6);
      uStack24 = CONCAT22(extraout_DX_00,uVar5);
      uVar2 = extraout_DX_00;
      for (uStack28 = 0x0; uStack28 < uStack24; uStack28 += 0x1) {
        uVar10 = pass1_1030_1d7c(uVar5,uVar2,(u32)puStack20);
        uVar7 = (u16)(uVar10 >> 0x10);
        uVar2 = uVar7 | (u16)uVar10;
        if (uVar2 != 0x0) {
          uVar4 = pass1_1030_dfcc((u32)param_2);
          uVar4 = pass1_1030_cbf0((u16)uVar10,uVar7,uVar4);
          if (uVar4 != 0x0) break;
        }
      }
      if (puStack20 != NULL) {
        ppcVar1 = (code **)*puStack20;
        (**ppcVar1)(0x38,uVar3,puVar6,0x1);
      }
    }
    else {
      uVar2 = extraout_DX;
      uVar5 = param_1;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,*(u32 *)&iVar9->field24_0x20);
      if ((uVar2 | uVar5) != 0x0) {
        iStack12 = 0x0;
        switch(iVar9->field10_0xc) {
        case 0x73:
        case 0x77:
          iStack12 = 0x1;
          break;
        case 0x74:
        case 0x78:
          iStack12 = 0x2;
          break;
        case 0x75:
          iStack12 = 0x3;
          break;
        case 0x76:
          iStack12 = 0x5;
        }
        if (iStack12 != 0x0) {
          pass1_1030_cc44(uVar5,uVar2,0x1,CONCAT22(extraout_DX,param_1),iStack12);
        }
      }
    }
  }
  pass1_1028_b418(&param_2->field0_0x0);
  return;
}



void pass1_1030_de7c(u32 param_1,u32 param_2)

{
  BOOL16 BVar1;
  HFILE16 in_stack_0000ffda;
  u32 local_10 [0x3];

  BVar1 = write_to_file_1028_b5ec((astruct_731 *)param_1,param_2);
  if (BVar1 != 0x0) {
    local_10[0] = *(u32 *)((int)param_1 + 0x20);
    BVar1 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_10),(char *)0x4,in_stack_0000ffda);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return;
    }
  }
  return;
}



void pass1_1030_dec4(i16 param_1,uchar *param_2,astruct_373 *param_3,HFILE16 *param_4)

{
  BOOL16 BVar1;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (((param_1 != 0x0) && (0x1 < (int)u16_1050_0312)) &&
     (BVar1 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | (u32)((int)param_3 + 0x20)),0x4),
     BVar1 == 0x0)) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  return;
}



void pass1_1030_df0c(u16 param_1,astruct_15 *param_2)

{
  u32 uVar1;
  u32 uVar2;
  i32 lVar3;
  u16 uVar4;
  i16 iVar5;
  u32 uVar6;
  u16 extraout_DX;
  u16 uVar7;
  u16 uVar8;
  u16 uVar9;
  u16 uVar10;
  u16 uStack24;
  u16 uStack22;
  u16 uStack14;
  u16 uStack10;

  pass1_1028_b58e(param_2);
  uVar1 = *(u32 *)(param_1 + 0x2e);
  uStack10 = (u16)uVar1;
  if (((param_1 + 0x30) | uStack10) != 0x0) {
    uVar9 = (u16)((u32)uVar1 >> 0x10);
    uVar1 = *(u32 *)(uStack10 + 0x210);
    uVar7 = (uStack10 + 0x212);
    uStack14 = (u16)uVar1;
    if ((uVar7 | uStack14) != 0x0) {
      uVar2 = *(u32 *)(uStack14 + 0xa);
      uVar4 = pass1_1030_dfcc((u32)param_2);
      if (uVar4 != 0x0) {
        uStack24 = 0x1;
        uStack22 = 0x0;
        while (CONCAT22(uStack22,uStack24) < uVar2) {
          uVar6 = uVar2;
          uVar10 = uVar4;
          bad_1030_1312();
          uVar8 = uVar7;
          iVar5 = pass1_1030_cde8((int)uVar6,uVar7,uVar10);
          if (-0x1 < iVar5) {
            pass1_1030_cef8(uVar6 & 0xffff | (u32)uVar7 << 0x10,CONCAT22(extraout_DX,param_1),0x1,iVar5);
            *(u32 *)((int)param_2 + 0x20) = *(u32 *)((int)uVar6 + 0x4);
            return;
          }
          lVar3 = CONCAT22(uStack22,uStack24) + 0x1;
          uStack24 = (u16)lVar3;
          uVar7 = uVar8;
          uStack22 = (u16)((u32)lVar3 >> 0x10);
        }
      }
    }
  }
  return;
}



u16 pass1_1030_dfcc(u32 param_1)

{
  i16 iVar1;
  u16 uStack4;

  iVar1 = ((int)param_1 + 0xc);
  if (iVar1 == 0x73) {
LAB_1030_dfde:
    uStack4 = 0x1;
  }
  else {
    if (iVar1 != 0x74) {
      if (iVar1 == 0x75) {
        return 0x3;
      }
      if (iVar1 == 0x77) goto LAB_1030_dfde;
      if (iVar1 != 0x78) {
        return 0x0;
      }
    }
    uStack4 = 0x2;
  }
  return uStack4;
}



StructD * pass1_1030_e010(StructD *param_1,u8 param_2)

{
  u16 in_AX;

  pass1_1030_dcf4(in_AX,(astruct_15 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1030_e09e(astruct_97 *param_1)

{
  struct_op_1028_d1dc(param_1,0x2af7);
  param_1->offset_0x0 = 0xe2ae;
  ((int)param_1 + 0x2) = 0x1030;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x8)),s_SCAiInput_1050_5972);
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_e0d4(uchar *param_1)

{
  i16 *piVar1;
  u16 uVar2;
  astruct_425 *paVar3;
  astruct_425 *paVar4;
  u16 uVar5;
  u16 in_register_0000000a;
  i16 iVar7;
  u16 unaff_SI;
  u16 uVar8;
  u32 *puVar9;
  u16 in_stack_0000fe7c;
  u16 in_stack_0000ffa0;
  u16 in_stack_0000ffa6;
  u16 in_stack_0000ffaa;
  u32 uStack42;
  u8 local_1c [0x8];
  u32 uStack20;
  u16 uStack16;
  u32 uStack14;
  u32 uStack10;
  i16 iStack6;
  u16 uStack4;
  u32 uVar6;

  puVar9 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(unaff_SI,0x40),in_stack_0000fe7c,in_stack_0000ffa0,in_stack_0000ffa6,
                           in_stack_0000ffaa);
  uStack4 = (u16)((u32)puVar9 >> 0x10);
  iStack6 = (int)puVar9;
  uStack10 = pass1_1008_b820(iStack6,uStack4,(u32)puVar9);
  uVar2 = (u16)uStack10;
  uVar5 = (u16)(uStack10 >> 0x10) | uVar2;
  uVar6 = (u32)uVar5;
  if (uVar5 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x8000001);
    uStack14 = CONCAT22((int)uVar6,uVar2);
    uStack16 = (u16)((uVar2 + 0x154) != 0x0);
    pass1_1008_5784((char *)CONCAT22(0x1050,local_1c),uStack10);
    while( true ) {
      uVar2 = (u16)uVar6;
      paVar3 = (astruct_425 *)local_1c;
      pass1_1008_5b12((char *)CONCAT22(0x1050,paVar3));
      uStack20 = CONCAT22(uVar2,paVar3);
      uVar6 = (u32)(uVar2 | (u16)paVar3);
      if ((uVar2 | (u16)paVar3) == 0x0) break;
      if (&paVar3->field_0x8 != 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,*(u32 *)&paVar3->field_0xa);
        paVar4 = paVar3;
        pass1_1038_354a((astruct_424 *)CONCAT22((uchar *)uVar6,paVar3),(u16)paVar3,(uchar *)uVar6);
        if (paVar4 != NULL) {
          uVar8 = (u16)((u32)uStack20 >> 0x10);
          if (uStack16 == 0x0) {
            iVar7 = ((int)uStack20 + 0xe) * 0xc;
            uStack42 = *(u32 *)(iVar7 + 0x58c4);
            uVar2 = (iVar7 + 0x58c8);
          }
          else {
            iVar7 = ((int)uStack20 + 0xe) * 0xc;
            uStack42 = *(u32 *)(iVar7 + 0x58be);
            uVar2 = (iVar7 + 0x58c2);
          }
          uVar5 = uVar2;
          pass1_1038_35a8(uVar2,(uchar *)uVar6,(((int)uStack20 + 0x10) * 0x2 + (int)uStack42),paVar3)
          ;
          if (uVar5 != 0x0) {
            uVar8 = (u16)((u32)uStack20 >> 0x10);
            iVar7 = (int)uStack20;
            piVar1 = (i16 *)(iVar7 + 0x10);
            *piVar1 = *piVar1 + 0x1;
            if ((int)uVar2 <= (iVar7 + 0x10)) {
              (iVar7 + 0x10) = 0x0;
            }
          }
        }
      }
    }
  }
  return;
}



void pass1_1030_e1f4(u16 param_1,uchar *param_2,u32 param_3)

{
  u32 *puVar1;
  u32 *puVar2;
  u32 *puVar3;
  i16 iVar4;
  u16 uVar5;
  u16 in_register_0000000a;
  astruct_57 *paVar6;
  u32 *puVar7;
  u16 uVar8;
  u16 *puStack10;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x108,paVar6);
  uVar5 = (u16)paVar6;
  puStack10 = (u16 *)CONCAT22(uVar5,param_1);
  if ((uVar5 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (u16)(param_3 >> 0x10);
    *(u32 *)(param_1 + 0x4) = *(u32 *)((int)param_3 + 0x4);
    puVar3 = (u32 *)((int)param_3 + 0x8);
    puVar7 = (u32 *)(param_1 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    *puStack10 = 0xe2ae;
    (param_1 + 0x2) = 0x1030;
  }
  return;
}



StructD * pass1_1030_e282(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void struct_1030_e2be(astruct_97 *param_1,u16 param_2,u32 param_3,u32 param_4)

{
  astruct_97 *iVar1;
  u16 uVar1;

  struct_op_1028_d1dc(param_1,0x2af7);
  uVar1 = (u16)((u32)param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  *(u32 *)&iVar1->field259_0x108 = param_4;
  *(u32 *)&iVar1->field262_0x10c = param_3;
  &iVar1->field264_0x110 = param_2;
  param_1->offset_0x0 = 0xe4ea;
  iVar1->segment_0x2 = 0x1030;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1030_e300(uchar *param_1,u32 param_2)

{
  u16 in_register_0000000a;
  astruct_27 *paVar1;
  u16 in_stack_0000fea2;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000ffcc;
  u16 in_stack_0000ffd0;
  u32 in_stack_0000fff8;

  paVar1 = (astruct_27 *)
           mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22((int)((u32)in_stack_0000fff8 >> 0x10),0x2b),in_stack_0000fea2,
                           in_stack_0000ffc6,in_stack_0000ffcc,in_stack_0000ffd0);
  pass1_1010_089e(paVar1,((int)param_2 + 0x110),0x2);
  return 0x1;
}



u16 pass1_1030_e328(u16 param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;

  uVar1 = (u16)(param_3 >> 0x10);
  if (((int)param_3 + 0x110) == 0x0) {
    pass1_1030_e4ba();
  }
  else {
    pass1_1030_e410(param_1,param_2,param_3 & 0xffff | (u32)uVar1 << 0x10);
  }
  return 0x1;
}



void pass1_1030_e34e(u16 param_1,uchar *param_2,astruct_403 *param_3)

{
  u32 *puVar1;
  u32 *puVar2;
  i16 iVar3;
  u16 uVar4;
  u16 in_register_0000000a;
  astruct_57 *paVar5;
  astruct_403 *iVar5;
  u32 *puVar6;
  u32 *puVar7;
  u16 uVar8;
  u16 *puStack10;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x112,paVar5);
  uVar4 = (u16)paVar5;
  puStack10 = (u16 *)CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (u16)((u32)param_3 >> 0x10);
    iVar5 = (astruct_403 *)param_3;
    *(u32 *)(param_1 + 0x4) = iVar5->field4_0x4;
    puVar6 = &iVar5->field5_0x8;
    puVar7 = (u32 *)(param_1 + 0x8);
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar6;
      puVar6 = puVar6 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    *(u32 *)(param_1 + 0x108) = iVar5->field258_0x108;
    *(u32 *)(param_1 + 0x10c) = iVar5->field259_0x10c;
    (param_1 + 0x110) = iVar5->field260_0x110;
    *puStack10 = 0xe4ea;
    (param_1 + 0x2) = 0x1030;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_e410(u16 param_1,u16 param_2,u32 param_3)

{
  uchar *puVar1;
  u16 uVar2;
  u16 *puVar3;
  u16 in_stack_0000fe9c;
  u8 local_10 [0x6];
  u8 local_a [0x4];
  u16 uStack6;
  u16 uStack4;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,*(u32 *)((int)param_3 + 0x10c));
  puVar1 = (uchar *)(param_2 | param_1);
  if (puVar1 != NULL) {
    uStack6 = param_1;
    uStack4 = param_2;
    pass1_1038_4fd8(param_1,CONCAT22(param_2,param_1),0x21);
    if (param_1 == 0x0) {
      pass1_1020_a43e(puVar1,(u16 *)CONCAT22(0x1050,local_a));
      puVar3 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_10),0x0,0x2,0xfffd);
      uVar2 = (u16)((u32)puVar3 >> 0x10);
      pass1_1020_a49a(uVar2,in_stack_0000fe9c,CONCAT22(0x1050,local_a),(i16 *)CONCAT22(0x1050,local_10),0x7a);
      pass1_1008_3e76((u16 *)CONCAT22(0x1050,local_10),0x0,0x3,0xfffe);
      pass1_1020_a49a(uVar2,in_stack_0000fe9c,CONCAT22(0x1050,local_a),(i16 *)CONCAT22(0x1050,local_10),0x7a);
      pass1_1008_3e76((u16 *)CONCAT22(0x1050,local_10),0x0,0x3,0xfffd);
      pass1_1020_a49a(uVar2,in_stack_0000fe9c,CONCAT22(0x1050,local_a),(i16 *)CONCAT22(0x1050,local_10),0x21);
    }
  }
  return;
}



void pass1_1030_e4ba(void)

{
  return;
}



StructD * pass1_1030_e4be(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void struct_1030_e4fa(astruct_97 *param_1,u32 param_2)

{
  astruct_97 *iVar1;
  u16 uVar1;

  struct_op_1028_d1dc(param_1,0x3e80);
  uVar1 = (u16)((u32)param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  *(u32 *)&iVar1->field259_0x108 = param_2;
  param_1->offset_0x0 = 0xe62e;
  iVar1->segment_0x2 = 0x1030;
  sys_1000_3f9c((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),s_SCKillBldg__0x_08lx_1050_597c,
                (u16)*(u32 *)&iVar1->field259_0x108);
  return;
}



u16 pass1_1030_e540(void)

{
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1030_e546(u32 param_1)

{
  u32 uVar1;

  uVar1 = *(u32 *)((int)param_1 + 0x108);
  pass1_1028_e332(_PTR_LOOP_1050_65e2,(u16)uVar1,(u16)((u32)uVar1 >> 0x10));
  return 0x1;
}



void pass1_1030_e564(u16 param_1,uchar *param_2,astruct_405 *param_3)

{
  u32 *puVar1;
  u32 *puVar2;
  i16 iVar3;
  u16 uVar4;
  u16 in_register_0000000a;
  astruct_57 *paVar5;
  astruct_405 *iVar5;
  u32 *puVar6;
  u32 *puVar7;
  u16 uVar8;
  u16 *puStack10;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x10c,paVar5);
  uVar4 = (u16)paVar5;
  puStack10 = (u16 *)CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (u16)((u32)param_3 >> 0x10);
    iVar5 = (astruct_405 *)param_3;
    *(u32 *)(param_1 + 0x4) = iVar5->field4_0x4;
    puVar6 = &iVar5->field5_0x8;
    puVar7 = (u32 *)(param_1 + 0x8);
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar6;
      puVar6 = puVar6 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    *(u32 *)(param_1 + 0x108) = iVar5->field258_0x108;
    *puStack10 = 0xe62e;
    (param_1 + 0x2) = 0x1030;
  }
  return;
}



StructD * pass1_1030_e602(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1030_e63e(astruct_97 *param_1,u16 param_2)

{
  astruct_97 *iVar1;
  astruct_97 *uVar1;

  iVar1 = (astruct_97 *)param_1;
  uVar1 = (astruct_97 *)((u32)param_1 >> 0x10);
  struct_op_1028_d1dc(param_1,0xf9f);
  iVar1->field259_0x108 = param_2;
  param_1->offset_0x0 = 0xe78a;
  iVar1->segment_0x2 = 0x1030;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),s_SCKillColony_1050_5990);
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1030_e67c(StructD *param_1,u32 param_2,u16 param_3)

{
  u16 uVar1;
  astruct_67 *paVar2;
  u16 in_stack_0000fea0;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffce;
  u16 uStack8;

  _param_3 = (u8 **)CONCAT22(uStack8,0x37);
  paVar2 = (astruct_67 *)
           mixed_1010_20ba((astruct_57 *)param_1,_u16_1050_0ed0,_param_3,in_stack_0000fea0,in_stack_0000ffc4,
                           in_stack_0000ffca,in_stack_0000ffce);
  uVar1 = pass1_1008_aaa8((u16)paVar2,(u16)((u32)paVar2 >> 0x10),((int)param_2 + 0x108));
  if (uVar1 != 0x0) {
    post_win_msg_1008_a0e4(paVar2,0x0,0x0,0x1,0x0,uVar1);
  }
  return 0x1;
}



void pass1_1030_e6c2(u16 param_1,uchar *param_2,astruct_406 *param_3)

{
  u32 *puVar1;
  u32 *puVar2;
  i16 iVar3;
  u16 uVar4;
  u16 in_register_0000000a;
  astruct_57 *paVar5;
  astruct_406 *iVar5;
  u32 *puVar6;
  u32 *puVar7;
  u16 uVar8;
  u16 *puStack10;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x10a,paVar5);
  uVar4 = (u16)paVar5;
  puStack10 = (u16 *)CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (u16)((u32)param_3 >> 0x10);
    iVar5 = (astruct_406 *)param_3;
    *(u32 *)(param_1 + 0x4) = iVar5->field4_0x4;
    puVar6 = &iVar5->field5_0x8;
    puVar7 = (u32 *)(param_1 + 0x8);
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar6;
      puVar6 = puVar6 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    (param_1 + 0x108) = iVar5->field258_0x108;
    *puStack10 = 0xe78a;
    (param_1 + 0x2) = 0x1030;
  }
  return;
}



StructD * pass1_1030_e75e(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1030_e79a(astruct_97 *param_1)

{
  struct_op_1028_d1dc(param_1,0xf9f);
  param_1->offset_0x0 = 0xe890;
  ((int)param_1 + 0x2) = 0x1030;
  unk_str_op_1000_3d3e
            ((char *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x8)),s_SCKillRebelColony_1050_599e);
  return param_1;
}



u16 pass1_1030_e7d0(void)

{
  return 0x1;
}



void pass1_1030_e7d6(u16 param_1,uchar *param_2,u32 param_3)

{
  u32 *puVar1;
  u32 *puVar2;
  u32 *puVar3;
  i16 iVar4;
  u16 uVar5;
  u16 in_register_0000000a;
  astruct_57 *paVar6;
  u32 *puVar7;
  u16 uVar8;
  u16 *puStack10;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x108,paVar6);
  uVar5 = (u16)paVar6;
  puStack10 = (u16 *)CONCAT22(uVar5,param_1);
  if ((uVar5 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (u16)(param_3 >> 0x10);
    *(u32 *)(param_1 + 0x4) = *(u32 *)((int)param_3 + 0x4);
    puVar3 = (u32 *)((int)param_3 + 0x8);
    puVar7 = (u32 *)(param_1 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    *puStack10 = 0xe890;
    (param_1 + 0x2) = 0x1030;
  }
  return;
}



StructD * pass1_1030_e864(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void pass1_1030_e8a0(astruct_97 *param_1,u32 param_2,u16 param_3,u32 param_4)

{
  astruct_97 *iVar1;
  u16 uVar1;

  struct_op_1028_d1dc(param_1,0x2710);
  uVar1 = (u16)((u32)param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  *(u32 *)&iVar1->field259_0x108 = param_2;
  *(u32 *)&iVar1->field262_0x10c = param_4;
  &iVar1->field264_0x110 = param_3;
  param_1->offset_0x0 = 0xeb40;
  iVar1->segment_0x2 = 0x1030;
  sys_1000_3f9c((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),s_SCMoveBas_to_0x_08lx_1050_59b0,
                (u16)*(u32 *)&iVar1->field262_0x10c);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1030_e8f8(u16 param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;
  u16 uVar2;
  i16 iVar3;
  u16 uVar4;
  u32 uVar5;
  char *pcStack20;
  u32 uStack6;

  uVar4 = (u16)(param_3 >> 0x10);
  iVar3 = (int)param_3;
  if (*(i32 *)(iVar3 + 0x108) != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,*(u32 *)(iVar3 + 0x10c));
    uStack6 = CONCAT22(param_2,param_1);
    uVar5 = struct_op_1030_73a8((astruct_419 *)CONCAT22(param_2,param_1),param_1,param_2);
    if (((int)uVar5 + 0xc) == (iVar3 + 0x110)) {
      pass1_1030_ea50(param_3,uStack6);
    }
    uVar1 = (iVar3 + 0x108);
    uVar2 = (iVar3 + 0x10a);
    pcStack20 = (char *)CONCAT22(uVar2,uVar1);
    if ((uVar2 | uVar1) != 0x0) {
      fn_ptr_1020_ba7e((u32 *)CONCAT22(uVar2,uVar1));
      fn_ptr_1000_17ce(pcStack20);
    }
    *(u32 *)(iVar3 + 0x108) = 0x0;
  }
  return 0x1;
}



void pass1_1030_e98e(u16 param_1,uchar *param_2,astruct_407 *param_3)

{
  u32 *puVar1;
  u32 *puVar2;
  i16 iVar3;
  u16 uVar4;
  u16 in_register_0000000a;
  astruct_57 *paVar5;
  astruct_407 *iVar5;
  u32 *puVar6;
  u32 *puVar7;
  u16 uVar8;
  u16 *puStack10;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x112,paVar5);
  uVar4 = (u16)paVar5;
  puStack10 = (u16 *)CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (u16)((u32)param_3 >> 0x10);
    iVar5 = (astruct_407 *)param_3;
    *(u32 *)(param_1 + 0x4) = iVar5->field4_0x4;
    puVar6 = &iVar5->field5_0x8;
    puVar7 = (u32 *)(param_1 + 0x8);
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar6;
      puVar6 = puVar6 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    *(u32 *)(param_1 + 0x108) = iVar5->field258_0x108;
    *(u32 *)(param_1 + 0x10c) = iVar5->field259_0x10c;
    (param_1 + 0x110) = iVar5->field260_0x110;
    *puStack10 = 0xeb40;
    (param_1 + 0x2) = 0x1030;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_ea50(u32 param_1,u32 param_2)

{
  u32 uVar1;
  BOOL16 BVar2;
  astruct_57 *in_EDX;
  u32 uVar3;
  u16 uVar4;
  i16 iVar5;
  u16 uVar6;
  u32 uVar7;
  u32 local_12;
  u16 local_e;
  i16 iStack12;
  u16 uStack10;
  u16 uStack8;
  u32 uStack6;

  uStack6 = 0x1869f;
  uVar6 = (u16)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  BVar2 = pass1_1008_c6ae(_u16_1050_06e0,(iVar5 + 0x110),0x3);
  if (BVar2 != 0x0) {
    uVar7 = struct_op_1030_73a8((astruct_419 *)param_2,BVar2,(int)in_EDX);
    uVar3 = (u32)in_EDX & 0xffff0000;
    local_e = (u16)uVar7;
    iStack12 = (int)(uVar7 >> 0x10);
    uStack6 = pass1_1028_45e2(local_e,iStack12,uVar7);
    in_EDX = (astruct_57 *)(uVar3 & 0xffff0000);
  }
  uVar1 = *(u32 *)(iVar5 + 0x108);
  uStack8 = ((int)uVar1 + 0x4);
  uStack10 = 0x0;
  while( true ) {
    uVar4 = (u16)((u32)in_EDX >> 0x10);
    if (uStack8 <= uStack10) {
      return;
    }
    pass1_1020_bb16(*(u32 **)(iVar5 + 0x108),(u32 *)CONCAT22(0x1050,&local_12),(u16 *)CONCAT22(0x1050,&local_e),
                    uStack10);
    in_EDX = (astruct_57 *)CONCAT22(uVar4,uStack6);
    if (uStack6 < local_12) {
      pass1_1030_7ddc((u16)uStack6,in_EDX,param_2,uStack6,local_e);
      uStack6 = 0x0;
    }
    else {
      uStack6 -= local_12;
      pass1_1030_7ddc((u16)local_12,in_EDX,param_2,local_12,local_e);
    }
    if ((uStack6 | (u16)uStack6) == 0x0) break;
    uStack10 += 0x1;
  }
  return;
}



StructD * pass1_1030_eb14(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1030_eb50(astruct_97 *param_1)

{
  struct_op_1028_d1dc(param_1,0x1f3f);
  param_1->offset_0x0 = 0xecb2;
  ((int)param_1 + 0x2) = 0x1030;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x8)),s_SCMines_1050_59c6);
  return param_1;
}



u16 pass1_1030_eb86(u16 param_1)

{
  i16 iVar1;
  code **ppcVar2;
  astruct_92 *paVar3;
  u16 uVar4;
  u16 extraout_DX;
  u32 *puStack24;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x700);
  while( true ) {
    uVar4 = param_1;
    paVar3 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar3));
    puStack24 = (u32 *)CONCAT22(uVar4,paVar3);
    param_1 = uVar4 | (u16)paVar3;
    if (param_1 == 0x0) break;
    if ((paVar3 + 0x1) == 0x5) {
      iVar1 = &paVar3->field5_0xc;
      if (((0x32 < iVar1) && (!SBORROW2(iVar1,0x33))) &&
         ((iVar1 == 0x34 || iVar1 + -0x33 < 0x1 || ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 0x2)))))) {
        ppcVar2 = (code **)((int)*puStack24 + 0x2c);
        (**ppcVar2)(0x1028);
        param_1 = extraout_DX;
      }
    }
  }
  return 0x1;
}



void pass1_1030_ebf8(u16 param_1,u16 param_2,u32 param_3)

{
  u32 *puVar1;
  u32 *puVar2;
  u32 *puVar3;
  i16 iVar4;
  u16 uVar5;
  u16 in_register_0000000a;
  astruct_57 *paVar6;
  u32 *puVar7;
  u16 uVar8;
  u16 *puStack10;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x108,paVar6);
  uVar5 = (u16)paVar6;
  puStack10 = (u16 *)CONCAT22(uVar5,param_1);
  if ((uVar5 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (u16)(param_3 >> 0x10);
    *(u32 *)(param_1 + 0x4) = *(u32 *)((int)param_3 + 0x4);
    puVar3 = (u32 *)((int)param_3 + 0x8);
    puVar7 = (u32 *)(param_1 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    *puStack10 = 0xecb2;
    (param_1 + 0x2) = 0x1030;
  }
  return;
}



StructD * pass1_1030_ec86(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1030_ecc2(uchar param_1,astruct_97 *param_2)

{
  struct_op_1028_d1dc(param_2,0xf9f);
  param_2->offset_0x0 = 0xb96;
  ((int)param_2 + 0x2) = (int)&u16_1050_1038;
  unk_str_op_1000_3d3e((char *)((u32)param_2 & 0xffff0000 | (u32)((int)param_2 + 0x8)),s_SCMorale_1050_59ce);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1030_ecf8(uchar param_1,astruct_57 *param_2,u32 param_3)

{
  i16 iVar1;
  u32 *puVar2;
  code **ppcVar3;
  u16 uVar4;
  u32 uVar5;
  astruct_92 *paVar6;
  i16 iVar7;
  u32 uVar8;
  u16 uVar9;
  u16 uVar10;
  u16 uVar11;
  StructD *pSVar13;
  u16 uVar15;
  bool bVar16;
  u32 *puVar17;
  u32 *puVar18;
  u16 in_stack_0000fe40;
  u16 in_stack_0000ff42;
  u16 in_stack_0000ff64;
  u16 in_stack_0000ff6a;
  u16 in_stack_0000ff6e;
  u16 uVar19;
  u16 in_stack_0000ff98;
  u32 uStack64;
  i16 iStack56;
  u16 uStack54;
  astruct_685 *paStack38;
  astruct_92 local_22;
  u16 uStack12;
  astruct_57 *paVar12;
  StructD *pSVar14;

  uStack12 = 0x0;
  puVar17 = mixed_1010_20ba(param_2,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ff98,0x2f),in_stack_0000fe40,
                            in_stack_0000ff64,in_stack_0000ff6a,in_stack_0000ff6e);
  uVar8 = (u32)param_2 & 0xffff0000 | (u32)puVar17 >> 0x10;
  uVar10 = (u16)puVar17;
  pass1_1010_ed3e((u32)puVar17);
  uVar9 = (u16)uVar8 | uVar10;
  paVar12 = (astruct_57 *)(uVar8 & 0xffff0000 | (u32)uVar9);
  if (uVar9 != 0x0) {
    uStack12 = pass1_1030_2aaa(CONCAT22((u16)uVar8,uVar10));
  }
  if ((int)uStack12 < 0x2) {
    uStack12 = 0x0;
  }
  puVar17 = mixed_1010_20ba(paVar12,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ff98,0x2),in_stack_0000fe40,
                            in_stack_0000ff64,in_stack_0000ff6a,in_stack_0000ff6e);
  pSVar13 = (StructD *)((u32)paVar12 & 0xffff0000 | (u32)puVar17 >> 0x10);
  if ((0x0 < (int)PTR_LOOP_1050_13ae) && (!SBORROW2((int)PTR_LOOP_1050_13ae,0x1))) {
    if (PTR_LOOP_1050_13ae == (u8 *)&u16_1050_0002 || (int)(PTR_LOOP_1050_13ae + -0x1) < 0x1) {
      if (0x6 < (int)uStack12) {
        uStack12 -= 0x2;
        goto LAB_1030_ed5b;
      }
      bVar16 = SBORROW2(uStack12,0x4);
      iVar1 = uStack12 - 0x4;
    }
    else {
      if (PTR_LOOP_1050_13ae != (u8 *)((int)&u16_1050_0002 + 0x1)) goto LAB_1030_ed5b;
      bVar16 = SBORROW2(uStack12,0x7);
      iVar1 = uStack12 - 0x7;
    }
    if (bVar16 == iVar1 < 0x0) {
      uStack12 -= 0x1;
    }
  }
LAB_1030_ed5b:
  pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_22)),0x1,0x0,0x400);
  while( true ) {
    paVar6 = &local_22;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar6));
    uVar10 = (u16)pSVar13;
    paStack38 = (astruct_685 *)CONCAT22(uVar10,paVar6);
    if ((uVar10 | (u16)paVar6) == 0x0) break;
    uVar9 = (u16)*(u32 *)&paVar6[0x1b].field6_0x10;
    pSVar13 = (StructD *)((u32)pSVar13 & 0xffff0000 | (u32)(paVar6 + 0x1c));
    if ((((int)&paVar6[0x1c].field3_0x4 + 0x2) != 0x0) && (paVar6[0x1c].field4_0x8 != 0x8000002)) {
      pass1_1030_38b8();
      uVar9 = (u16)pSVar13 | uVar9;
      uVar8 = (u32)pSVar13 & 0xffff0000;
      pSVar13 = (StructD *)(uVar8 | uVar9);
      if (uVar9 != 0x0) {
        puVar2 = (u32 *)paVar6->field5_0xc;
        uVar9 = ((int)&paVar6->field5_0xc + 0x2);
        uVar8 |= uVar9;
        ppcVar3 = (code **)((int)*puVar2 + 0x10);
        puVar18 = puVar2;
        (**ppcVar3)(0x1028,(int)puVar2,uVar9);
        uVar5 = (u32)puVar18 & 0xffff | uVar8 << 0x10;
        uStack54 = ((int)&paVar6[0x1].field3_0x4 + 0x2);
        uVar15 = SUB42(&u16_1050_1038,0x0);
        pass1_1038_4760(CONCAT22(uVar10,paVar6));
        iVar1 = paVar6[0x1].field6_0x10;
        iStack56 = iVar1 / 0xa;
        iVar7 = (paVar6 + 0x2);
        if (iVar7 < 0x33) {
          if (iVar7 < 0x32) {
            iStack56 += -0x1;
          }
        }
        else {
          uStack54 += 0x1;
        }
        pSVar13 = (StructD *)(uVar8 & 0xffff0000 | (long)iVar1 % 0xa & 0xffffU);
        for (uStack64 = 0x0; uStack64 < uVar5; uStack64 += 0x1) {
          ppcVar3 = (code **)((int)*puVar2 + 0x4);
          uVar8 = uVar5;
          (**ppcVar3)(uVar15,(char)puVar2,(int)((u32)puVar2 >> 0x10),(int)uStack64,(int)(uStack64 >> 0x10));
          uVar9 = (u16)uVar8;
          uVar11 = (u16)pSVar13 | uVar9;
          pSVar14 = (StructD *)((u32)pSVar13 & 0xffff0000 | (u32)uVar11);
          if (uVar11 != 0x0) {
            uVar15 = 0x1028;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar8 & 0xffff | (long)pSVar13 << 0x10);
            puVar18 = (u32 *)struct_op_1030_73a8((astruct_419 *)CONCAT22((int)pSVar14,uVar9),uVar9,(int)pSVar14);
            uVar9 = (u16)puVar18;
            uVar11 = (u16)((u32)puVar18 >> 0x10);
            pSVar14 = (StructD *)((u32)pSVar14 & 0xffff0000 | (u32)(uVar11 | uVar9));
            if (((uVar11 | uVar9) != 0x0) && ((uVar9 + 0x12) == 0x5)) {
              ppcVar3 = (code **)((int)*puVar18 + 0x48);
              (**ppcVar3)(0x1028,uVar9,uVar11);
              if ((int)uVar9 < 0x0) {
                iStack56 += uVar9;
              }
              else {
                uStack54 += uVar9;
              }
            }
          }
          pSVar13 = pSVar14;
        }
        iVar1 = (paVar6 + 0x1d);
        uVar19 = (u16)(param_3 >> 0x10);
        uVar4 = (u16)param_3;
        iVar7 = iVar1;
        pass1_1038_01c0(uVar4,uVar19,(u32)paStack38);
        iVar7 -= iVar1;
        iStack56 = (iStack56 - uStack12) - iVar7;
        pass1_1038_008e((uchar *)pSVar13,uVar4,uVar19,(u32)paStack38);
        if (iVar7 < 0x0) {
          iStack56 += iVar7;
        }
        else {
          uStack54 += iVar7;
        }
        if (0x3e8 < (int)uStack54) {
          uStack54 = 0x3e8;
        }
        if ((int)uStack54 < 0x0) {
          uStack54 = 0x0;
        }
        uStack54 += iStack56;
        if (0x3e8 < (int)uStack54) {
          uStack54 = 0x3e8;
        }
        if ((int)uStack54 < 0x0) {
          uStack54 = 0x0;
        }
        pass1_1038_4d0e(paStack38,uStack54);
        if (paVar6->field3_0x4 == 0x4000001) {
          pass1_1038_08d4(param_1,(u32)pSVar13,uVar4,CONCAT22(uStack54,uVar19),(u32)paStack38);
        }
        pass1_1038_095e(pSVar13,uVar4,uVar19,uStack54,(u32)paStack38,in_stack_0000ff42);
      }
    }
  }
  return;
}
