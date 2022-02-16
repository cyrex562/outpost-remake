// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_ee54(ulong param_1,ushort param_2,ushort *param_3,ulong param_4)

{
  undefined2 in_DX;
  uint uVar1;
  ushort unaff_SS;
  astruct_99 *paVar2;
  undefined local_16 [0x4];
  undefined4 uStack18;
  uint uStack14;
  ushort uStack12;
  undefined4 uStack10;
  ushort *puStack6;
  
  puStack6 = param_3;
  pass1_1030_64ce(unaff_SS,(int)param_3,in_DX,_PTR_LOOP_1050_5740,param_3,param_4,(ulong *)CONCAT22(unaff_SS,local_16));
  uStack10 = *(undefined4 *)param_3;
  paVar2 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5744);
  uVar1 = (uint)((ulong)paVar2 >> 0x10);
  uStack14 = (uint)paVar2;
  uStack12 = uVar1 | uStack14;
  if (uStack12 == 0x0) {
    uStack14 = 0x0;
    uStack12 = 0x0;
  }
  else {
    pass1_1030_684c((ushort *)((ulong)paVar2 & 0xffff | (ulong)uVar1 << 0x10),(ulong *)puStack6,
                    (ushort)((ulong)puStack6 >> 0x10),(ushort)uStack10,(ushort)((ulong)uStack10 >> 0x10),param_4,
                    uStack12);
  }
  uStack18 = *(ulong *)(uStack14 + 0x4);
  pass1_1030_61fe(_PTR_LOOP_1050_5740,uStack18,(ulong)puStack6,param_4,(ushort)uStack18,uStack12,unaff_SS);
  pass1_1030_1358(*(ulong *)((int)param_1 + 0x1e),uStack14,uStack12,
                  uStack18 & 0xffff | (ulong)(uStack18._2_2_ & 0xff) << 0x10,unaff_SS);
  return;
}



void __stdcall16far
pass1_1028_ef00(ushort param_1,uchar *param_2,ulong param_3,ushort param_4,ushort param_5,ushort param_6,ushort param_7)

{
  ushort uVar1;
  ushort *puVar2;
  
  if (param_5 == 0x4) {
    mem_op_1000_179c(0x16,param_2,0x1000);
    uVar1 = (uint)param_2 | param_5;
    if (uVar1 != 0x0) {
      pass1_1030_b936((astruct_365 *)param_5,(ushort)param_2,0x4,_param_6,uVar1);
      goto LAB_1028_ef8b;
    }
  }
  else {
    if (param_5 == 0xc) {
      mem_op_1000_179c(0xe,param_2,0x1000);
      if (((uint)param_2 | param_5) != 0x0) {
        puVar2 = pass1_1030_bc24((uint)param_2 | param_5,param_5,(ushort)param_2,0xc,_param_6);
        uVar1 = (ushort)((ulong)puVar2 >> 0x10);
        param_5 = (ushort)(astruct_365 *)puVar2;
        goto LAB_1028_ef8b;
      }
    }
    else {
      uVar1 = param_5;
      mem_op_1000_179c(0xe,param_2,0x1000);
      if (((uint)param_2 | uVar1) != 0x0) {
        puVar2 = pass1_1028_b22c((ushort *)CONCAT22(param_2,uVar1),param_5,_param_6,(uint)param_2 | uVar1);
        uVar1 = (ushort)((ulong)puVar2 >> 0x10);
        param_5 = (ushort)(astruct_365 *)puVar2;
        goto LAB_1028_ef8b;
      }
    }
  }
  param_5 = 0x0;
  uVar1 = 0x0;
LAB_1028_ef8b:
  pass1_1030_1358(*(ulong *)((int)param_3 + 0x22),param_5,uVar1,
                  *(ulong *)(param_5 + 0x4) & 0xffff | (ulong)(*(uint *)(param_5 + 0x6) & 0xff) << 0x10,param_1);
  return;
}



ushort * __stdcall16far
switch_1030_0000(ushort param_1,ushort param_2,int param_3,uchar *param_4,uint param_5,ushort param_6,ushort param_7)

{
  uchar *puVar1;
  uint uVar2;
  ushort *puVar3;
  
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
  puVar3 = (ushort *)CONCAT22(param_4,param_5);
  switch(param_3 + -0x1) {
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
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_489e((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x9:
    mem_op_1000_179c(0x22,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_2bdc((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0xa:
    mem_op_1000_179c(0x26,param_4,0x1000);
    uVar2 = (uint)param_4 | param_5;
    goto joined_r0x103002a1;
  case 0xb:
    mem_op_1000_179c(0x2c,param_4,0x1000);
    if ((uchar *)((uint)param_4 | param_5) != (uchar *)0x0) {
      puVar3 = struct_1028_3670((ushort *)CONCAT22(param_4,param_5),(uchar *)((uint)param_4 | param_5),param_6,param_7);
      return puVar3;
    }
    break;
  case 0xc:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_355e((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0xd:
    mem_op_1000_179c(0x26,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_3484((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0xe:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_406c((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0xf:
  case 0x32:
  case 0x33:
  case 0x5f:
  case 0x60:
    mem_op_1000_179c(0x24,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_0c24((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x10:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_0b42((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x11:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_4354((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x12:
  case 0x13:
  case 0x14:
  case 0x61:
  case 0x62:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_4b84((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x15:
  case 0x16:
  case 0x17:
    mem_op_1000_179c(0x24,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_1bbc((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  default:
    mem_op_1000_179c(0x20,param_4,0x1000);
    uVar2 = (uint)param_4 | param_5;
    if (uVar2 != 0x0) {
      struct_1028_b354((ushort *)CONCAT22(param_4,param_5));
      return (ushort *)CONCAT22(uVar2,param_5);
    }
    break;
  case 0x1a:
  case 0x1b:
  case 0x1c:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1030_be34((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x1d:
  case 0x1e:
  case 0x1f:
    mem_op_1000_179c(0x26,param_4,0x1000);
    puVar1 = (uchar *)((uint)param_4 | param_5);
    if (puVar1 != (uchar *)0x0) {
      struct_1028_0068((ushort *)CONCAT22(param_4,param_5),puVar1);
      return (ushort *)CONCAT22(puVar1,param_5);
    }
    break;
  case 0x20:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_50d8((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x21:
  case 0x22:
  case 0x23:
    mem_op_1000_179c(0x24,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_3e94((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x24:
  case 0x25:
  case 0x26:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1020_d06c((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x27:
  case 0x28:
  case 0x5c:
  case 0x5d:
  case 0x5e:
    mem_op_1000_179c(0x22,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1030_c6f6((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x29:
  case 0x2a:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1020_cce4((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x2b:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_26b4((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x2c:
  case 0x2d:
    mem_op_1000_179c(0x2a,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_49aa((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x2e:
  case 0x2f:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1020_e7fa((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x30:
  case 0x31:
  case 0x6b:
  case 0x6c:
    mem_op_1000_179c(0x22,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1020_d37c((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x34:
  case 0x35:
    mem_op_1000_179c(0x2c,param_4,0x1000);
    puVar1 = (uchar *)((uint)param_4 | param_5);
    if (puVar1 != (uchar *)0x0) {
      struct_1028_37a6((ushort *)CONCAT22(param_4,param_5),puVar1,param_6,param_7);
      return (ushort *)CONCAT22(puVar1,param_5);
    }
    break;
  case 0x36:
    mem_op_1000_179c(0x26,param_4,0x1000);
    uVar2 = (uint)param_4 | param_5;
joined_r0x103002a1:
    if (uVar2 != 0x0) {
      struct_1030_c06e((ushort *)CONCAT22(param_4,param_5));
      return (ushort *)CONCAT22(uVar2,param_5);
    }
    break;
  case 0x37:
  case 0x38:
    mem_op_1000_179c(0x9a,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1030_c9a8((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x39:
  case 0x3a:
    mem_op_1000_179c(0x24,param_4,0x1000);
    if ((uchar *)((uint)param_4 | param_5) != (uchar *)0x0) {
      puVar3 = struct_1028_60bc((ushort *)CONCAT22(param_4,param_5),param_5,(uchar *)((uint)param_4 | param_5));
      return puVar3;
    }
    break;
  case 0x3b:
  case 0x3c:
    mem_op_1000_179c(0x24,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_44d2((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x3d:
    mem_op_1000_179c(0x22,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1020_cde6((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x3e:
    mem_op_1000_179c(0x26,param_4,0x1000);
    puVar1 = (uchar *)((uint)param_4 | param_5);
    if (puVar1 != (uchar *)0x0) {
      struct_1028_1f56((ushort *)CONCAT22(param_4,param_5),puVar1);
      return (ushort *)CONCAT22(puVar1,param_5);
    }
    break;
  case 0x3f:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_25da((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x40:
    mem_op_1000_179c(0x22,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1020_c9ea((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x46:
  case 0x69:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1020_d5a6((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x47:
  case 0x48:
  case 0x49:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1020_d866((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x4b:
  case 0x4c:
  case 0x4d:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1030_d8f6((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x4e:
  case 0x4f:
  case 0x50:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_5c54((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x51:
  case 0x52:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_5966((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x53:
  case 0x54:
  case 0x55:
    mem_op_1000_179c(0x22,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_5ed8((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x56:
  case 0x57:
  case 0x58:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_53c6((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x59:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = pass1_1028_5884((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x5a:
  case 0x5b:
    mem_op_1000_179c(0x26,param_4,0x1000);
    if ((uchar *)((uint)param_4 | param_5) != (uchar *)0x0) {
      puVar3 = pass1_1028_5524((ushort *)CONCAT22(param_4,param_5),(uchar *)((uint)param_4 | param_5));
      return puVar3;
    }
    break;
  case 0x63:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = pass1_1028_5df6((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x64:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_5a48((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x65:
  case 0x66:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = (ushort *)struct_1028_52e8(param_5,param_4);
      return puVar3;
    }
    break;
  case 0x67:
  case 0x68:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_57a6((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x6d:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = (ushort *)struct_1028_5630((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x6f:
  case 0x70:
  case 0x71:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) == 0x0) {
      puVar3 = (ushort *)0x0;
    }
    else {
      puVar3 = struct_1020_d866((ushort *)CONCAT22(param_4,param_5));
    }
  case 0x72:
  case 0x76:
    mem_op_1000_179c(0x26,(uchar *)((ulong)puVar3 >> 0x10),0x1000);
    if (puVar3 != (ushort *)0x0) {
      puVar3 = struct_1020_e8f6(puVar3);
      return puVar3;
    }
    break;
  case 0x73:
  case 0x77:
  case 0x78:
    mem_op_1000_179c(0x2c,param_4,0x1000);
    uVar2 = (uint)param_4 | param_5;
    if (uVar2 != 0x0) {
      struct_1020_d954((ushort *)CONCAT22(param_4,param_5));
      return (ushort *)CONCAT22(uVar2,param_5);
    }
    break;
  case 0x74:
    mem_op_1000_179c(0x24,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_178c((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x75:
    mem_op_1000_179c(0x24,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = struct_1028_2afa((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
    break;
  case 0x79:
  case 0x7a:
  case 0x7b:
  case 0x7c:
  case 0x7d:
  case 0x7e:
    mem_op_1000_179c(0x20,param_4,0x1000);
    if (((uint)param_4 | param_5) != 0x0) {
      puVar3 = (ushort *)struct_1028_27f0((ushort *)CONCAT22(param_4,param_5));
      return puVar3;
    }
  }
  return (ushort *)0x0;
}



ushort * __stdcall16far
switch_1030_07ac(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5,ulong param_6,uchar *param_7
                ,astruct_179 *param_8,ushort param_9,ushort param_10,ushort param_11)

{
  ushort uVar1;
  uchar *puVar2;
  uint uVar3;
  ushort *puVar4;
  
  puVar4 = (ushort *)CONCAT22(param_7,param_8);
  switch(param_4 - 0x1) {
  case 0x0:
  case 0x1:
  case 0x2:
  case 0x3:
  case 0x4:
  case 0x5:
  case 0x6:
  case 0x7:
  case 0x8:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_48c0((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x9:
    mem_op_1000_179c(0x22,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_2bfe(param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0xa:
    mem_op_1000_179c(0x26,param_7,0x1000);
    uVar1 = (uint)param_7 | (uint)param_8;
    goto joined_r0x10300adb;
  case 0xb:
    mem_op_1000_179c(0x2c,param_7,0x1000);
    if ((uchar *)((uint)param_7 | (uint)param_8) != (uchar *)0x0) {
      puVar4 = pass1_1028_3692((int)param_8,(ushort)param_7,param_4,param_6,(uchar *)((uint)param_7 | (uint)param_8),
                               param_9,param_10);
      return puVar4;
    }
    break;
  case 0xc:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_3580((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0xd:
    mem_op_1000_179c(0x26,param_7,0x1000);
    if ((uchar *)((uint)param_7 | (uint)param_8) != (uchar *)0x0) {
      puVar4 = pass1_1028_34a6((int)param_8,(ushort)param_7,param_4,param_6,(uchar *)((uint)param_7 | (uint)param_8));
      return puVar4;
    }
    break;
  case 0xe:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_408e((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0xf:
  case 0x32:
  case 0x33:
  case 0x5f:
  case 0x60:
    mem_op_1000_179c(0x24,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_0c50((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x10:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_0b64((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x11:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_4376((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x12:
  case 0x13:
  case 0x14:
  case 0x61:
  case 0x62:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_4ba6((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x15:
  case 0x16:
  case 0x17:
    mem_op_1000_179c(0x24,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_1be8((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  default:
    mem_op_1000_179c(0x20,param_7,0x1000);
    uVar3 = (uint)param_7 | (uint)param_8;
    if (uVar3 != 0x0) {
      pass1_1028_b39e((ushort *)CONCAT22(param_7,param_8),param_4,param_6,uVar3);
      return (ushort *)CONCAT22(uVar3,param_8);
    }
    break;
  case 0x1a:
  case 0x1b:
  case 0x1c:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1030_be56((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x1d:
  case 0x1e:
  case 0x1f:
    mem_op_1000_179c(0x26,param_7,0x1000);
    puVar2 = (uchar *)((uint)param_7 | (uint)param_8);
    if (puVar2 != (uchar *)0x0) {
      pass1_1028_00cc((int)param_8,(ushort)param_7,param_4,param_6,puVar2);
      return (ushort *)CONCAT22(puVar2,param_8);
    }
    break;
  case 0x20:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_50fa((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x21:
  case 0x22:
  case 0x23:
    mem_op_1000_179c(0x24,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = (ushort *)pass1_1028_3ec8((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x24:
  case 0x25:
  case 0x26:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1020_d08e((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x27:
  case 0x28:
  case 0x5c:
  case 0x5d:
  case 0x5e:
    mem_op_1000_179c(0x22,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1030_c71e((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x29:
  case 0x2a:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1020_cd06((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x2b:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_26d6((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x2c:
  case 0x2d:
    mem_op_1000_179c(0x2a,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = (ushort *)pass1_1028_49de((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x2e:
  case 0x2f:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1020_e81c((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x30:
  case 0x31:
  case 0x6b:
  case 0x6c:
    mem_op_1000_179c(0x22,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1020_d3a4((ushort *)CONCAT22(param_7,param_8),param_3,param_4,param_6,(uint)param_7 | (uint)param_8
                              );
      return puVar4;
    }
    break;
  case 0x34:
  case 0x35:
    mem_op_1000_179c(0x2c,param_7,0x1000);
    puVar2 = (uchar *)((uint)param_7 | (uint)param_8);
    if (puVar2 != (uchar *)0x0) {
      pass1_1028_3816((int)param_8,(ushort)param_7,param_4,param_6,puVar2,param_9,param_10);
      return (ushort *)CONCAT22(puVar2,param_8);
    }
    break;
  case 0x36:
    mem_op_1000_179c(0x26,param_7,0x1000);
    uVar1 = (uint)param_7 | (uint)param_8;
joined_r0x10300adb:
    if (uVar1 != 0x0) {
      pass1_1030_c09c((int)param_8,(ushort)param_7,param_4,param_6,uVar1);
      return (ushort *)CONCAT22(uVar1,param_8);
    }
    break;
  case 0x37:
  case 0x38:
    mem_op_1000_179c(0x9a,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = (ushort *)pass1_1030_c9e4((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x39:
  case 0x3a:
    mem_op_1000_179c(0x24,param_7,0x1000);
    if ((uchar *)((uint)param_7 | (uint)param_8) != (uchar *)0x0) {
      puVar4 = (ushort *)
               pass1_1028_611e((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_8,
                               (uchar *)((uint)param_7 | (uint)param_8));
      return puVar4;
    }
    break;
  case 0x3b:
  case 0x3c:
    mem_op_1000_179c(0x24,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_44fe((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x3d:
    mem_op_1000_179c(0x22,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1020_ce08(param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x3e:
    mem_op_1000_179c(0x26,param_7,0x1000);
    puVar2 = (uchar *)((uint)param_7 | (uint)param_8);
    if (puVar2 != (uchar *)0x0) {
      pass1_1028_1fc8((int)param_8,(ushort)param_7,param_4,param_6,puVar2);
      return (ushort *)CONCAT22(puVar2,param_8);
    }
    break;
  case 0x3f:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_25fc((int)param_8,param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x40:
    mem_op_1000_179c(0x22,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1020_ca0c(param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x46:
  case 0x69:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1020_d5c8((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x47:
  case 0x48:
  case 0x49:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1020_d888((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x4b:
  case 0x4c:
  case 0x4d:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = (ushort *)pass1_1030_d942((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x4e:
  case 0x4f:
  case 0x50:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_5c76((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x51:
  case 0x52:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_5988((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x53:
  case 0x54:
  case 0x55:
    mem_op_1000_179c(0x22,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_5f00((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x56:
  case 0x57:
  case 0x58:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_53e8((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x59:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_58a6((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x5a:
  case 0x5b:
    mem_op_1000_179c(0x26,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_5546((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x63:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_5e18((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x64:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_5a6a((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x65:
  case 0x66:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_530a((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x67:
  case 0x68:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_57c8((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x6d:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_5652((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x6f:
  case 0x70:
  case 0x71:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) == 0x0) {
      puVar4 = (ushort *)0x0;
    }
    else {
      puVar4 = pass1_1020_d888((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
    }
  case 0x72:
  case 0x76:
    mem_op_1000_179c(0x26,(uchar *)((ulong)puVar4 >> 0x10),0x1000);
    uVar3 = (uint)((ulong)puVar4 >> 0x10);
    if (puVar4 != (ushort *)0x0) {
      puVar4 = pass1_1020_e91e((uint)puVar4,uVar3,param_4,param_6,uVar3 | (uint)puVar4);
      return puVar4;
    }
    break;
  case 0x73:
  case 0x77:
  case 0x78:
    mem_op_1000_179c(0x2c,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = struct_1020_d99e((ushort *)CONCAT22(param_7,param_8),param_3,param_4,param_6,
                                (uint)param_7 | (uint)param_8,param_11);
      return puVar4;
    }
    break;
  case 0x74:
    mem_op_1000_179c(0x24,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_17ae((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x75:
    mem_op_1000_179c(0x24,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_2b1c((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
    break;
  case 0x79:
  case 0x7a:
  case 0x7b:
  case 0x7c:
  case 0x7d:
  case 0x7e:
    mem_op_1000_179c(0x20,param_7,0x1000);
    if (((uint)param_7 | (uint)param_8) != 0x0) {
      puVar4 = pass1_1028_2812((int)param_8,(ushort)param_7,param_4,param_6,(uint)param_7 | (uint)param_8);
      return puVar4;
    }
  }
  return (ushort *)0x0;
}

