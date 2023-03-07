//
// Created by cyrex on 2/25/2022.
//

#include "unk_18.h"

#include "op_int.h"

u16*  mixed_1010_20ba(u32 param_1,u16 param_2,u16 param_3,u8 *param_4,int param_5)

{
  fn_ptr_1 **ppcVar1;
  u16 u_var2;
  u8 *puVar3;
  u8 *extraout_DX;
  Struct636 *paVar4;
  int iVar5;
  u16 uVar6;
  u16 uVar7;
  u16 *puVar8;
  u16 *puVar9;
  u32 uVar10;
  u32 uVar11;
  u16 *pu_stack6;
  
  pass1_1010_2816(param_1);
  paVar4 = (Struct636 *)(param_2 * 4);
  uVar6 = (u16)(param_1 >> 0x10);
  iVar5 = (int)param_1;
  pu_stack6 = *(u16 **)((int)&paVar4->field_0x0 + iVar5);
  if (pu_stack6 != (u16 *)0x0) {
    return pu_stack6;
  }
  if (false) goto switchD_1010:2765_caseD_38;
  switch(param_2) {
  case 1:
    mem_op_1000_179c(0x18,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) {
LAB_1010_2126:
      paVar4 = (Struct636 *)0x0;
      puVar3 = (u8 *)0x0;
    }
    else {
      struct_1010_3b7a((Struct648 *)paVar4,(u16)param_4,param_2);
    }
    break;
  case 2:
    mem_op_1000_179c(0x84,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    win_sys_op_1010_5404((Struct54 *)paVar4,(u16)param_4,param_2,param_3);
    break;
  case 3:
    mem_op_1000_179c(0x53c,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    struct_1010_a1d8((Struct627 *)paVar4,(u16)param_4,param_2,param_3);
    break;
  case 4:
    mem_op_1000_179c(0x18a,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    struct_1018_2b10((Struct55 *)CONCAT22(param_4,paVar4),param_2,param_3);
    break;
  case 5:
    mem_op_1000_179c(0x14,param_4,0x1000);
    if (((uint)param_4 | (uint)paVar4) == 0) goto LAB_1010_2126;
    puVar9 = pass1_1008_eabc((int)paVar4,(u16)param_4,param_2);
    puVar3 = (u8 *)((u32)puVar9 >> 0x10);
    paVar4 = (Struct636 *)puVar9;
    break;
  case 6:
    mem_op_1000_179c(0x58,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    pass1_1018_18b8(param_3,(Struct55 *)CONCAT22(param_4,paVar4),param_2);
    break;
  case 7:
    mem_op_1000_179c(0xe,param_4,0x1000);
    if (((uint)param_4 | (uint)paVar4) == 0) goto LAB_1010_2126;
    uVar11 = pass1_1010_3d82((Struct628 *)paVar4,(u16)param_4,param_2,param_3);
    puVar3 = (u8 *)(uVar11 >> 0x10);
    paVar4 = (Struct636 *)uVar11;
    break;
  case 8:
    mem_op_1000_179c(0x20,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    struct_1010_95aa((Struct629 *)paVar4,(u16)param_4,param_2);
    break;
  case 9:
    mem_op_1000_179c(0x26,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    struct_1010_6326((Struct630 *)paVar4,(u16)param_4,param_2);
    break;
  case 10:
    mem_op_1000_179c(0x1c,param_4,0x1000);
    if ((u8 *)((uint)param_4 | (uint)paVar4) == (u8 *)0x0) goto LAB_1010_2126;
    uVar11 = pass1_1010_0eac((u8 *)paVar4,param_4,param_2,(u8 *)((uint)param_4 | (uint)paVar4),param_3);
    puVar3 = (u8 *)(uVar11 >> 0x10);
    paVar4 = (Struct636 *)uVar11;
    break;
  case 0xb:
    mem_op_1000_179c(0x1c,param_4,0x1000);
    if ((u8 *)((uint)param_4 | (uint)paVar4) == (u8 *)0x0) goto LAB_1010_2126;
    uVar11 = pass1_1008_aefe((u8 *)paVar4,param_4,param_2,(u8 *)((uint)param_4 | (uint)paVar4),param_3);
    puVar3 = (u8 *)(uVar11 >> 0x10);
    paVar4 = (Struct636 *)uVar11;
    break;
  case 0xc:
  case 0xd:
  case 0xe:
  case 0xf:
  case 0x10:
  case 0x11:
  case 0x12:
  case 0x13:
  case 0x14:
  case 0x15:
  case 0x16:
  case 0x17:
  case 0x18:
  case 0x19:
  case 0x1a:
  case 0x1b:
  case 0x1c:
  case 0x1d:
  case 0x1e:
  case 0x1f:
  case 0x20:
  case 0x21:
  case 0x22:
  case 0x23:
  case 0x24:
    mem_op_1000_179c(0xaa,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    struct_1018_0570((Struct55 *)CONCAT22(param_4,paVar4),param_2,param_3);
    break;
  case 0x25:
    mem_op_1000_179c(0x1c,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    pass1_1018_4aaa((int)paVar4,(u16)param_4,param_2,puVar3,param_3);
    break;
  case 0x26:
    mem_op_1000_179c(0x1c,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    pass1_1008_d99e((int)paVar4,(u16)param_4,param_2,puVar3,param_3);
    break;
  case 0x27:
    mem_op_1000_179c(0x58,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    pass1_1008_9d36((u8 *)paVar4,param_4,param_2,param_3);
    break;
  case 0x28:
    mem_op_1000_179c(0x2c,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    pass1_1010_28e6((Struct631 *)paVar4,param_4,param_2,puVar3,param_3);
    break;
  case 0x29:
    mem_op_1000_179c(0x72,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    struct_1018_229c((Struct632 *)paVar4,param_4,param_2,puVar3,param_3);
    break;
  case 0x2a:
    mem_op_1000_179c(0x1c,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    pass1_1010_503e((int)paVar4,(u16)param_4,param_2,puVar3,param_3);
    break;
  case 0x2b:
    mem_op_1000_179c(0x1a,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    struct_1010_02e0((Struct79 *)paVar4,(Struct79 *)param_4,param_2);
    break;
  case 0x2c:
    mem_op_1000_179c(0x10,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    pass1_1008_eb2a((int)paVar4,(u16)param_4,param_2);
    break;
  case 0x2d:
    mem_op_1000_179c(0x80,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    pass1_1010_3e3c((Struct55 *)CONCAT22(param_4,paVar4),param_2,param_3);
    break;
  case 0x2e:
    mem_op_1000_179c(0x806,param_4,0x1000);
    if (((uint)param_4 | (uint)paVar4) == 0) goto LAB_1010_2126;
    uVar11 = pass1_1018_1ff4((Struct634 *)paVar4,(u16)param_4,param_2);
    puVar3 = (u8 *)(uVar11 >> 0x10);
    paVar4 = (Struct636 *)uVar11;
    break;
  case 0x2f:
    mem_op_1000_179c(0x58,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    struct_1010_e9e4((Struct261 *)paVar4,(u16)param_4,param_2);
    break;
  case 0x30:
    mem_op_1000_179c(0xe,param_4,0x1000);
    if (((uint)param_4 | (uint)paVar4) == 0) goto LAB_1010_2126;
    puVar8 = pass1_1010_3702((int)paVar4,(u16)param_4,param_2);
    puVar3 = (u8 *)((u32)puVar8 >> 0x10);
    paVar4 = (Struct636 *)puVar8;
    break;
  case 0x31:
    u_var2 = 0x60;
    uVar7 = 0x1000;
    mem_op_1000_179c(0x60,param_4,0x1000);
    if ((u8 *)((uint)param_4 | (uint)paVar4) == (u8 *)0x0) {
LAB_1010_2680:
      uVar7 = 0x1000;
      paVar4 = (Struct636 *)0x0;
      puVar3 = (u8 *)0x0;
    }
    else {
      uVar11 = pass1_1010_9298((Struct79 *)paVar4,(Struct79 *)param_4,param_2,(u16)paVar4,
                               (u8 *)((uint)param_4 | (uint)paVar4),param_3);
      puVar3 = (u8 *)(uVar11 >> 0x10);
      paVar4 = (Struct636 *)uVar11;
    }
    goto LAB_1010_2683;
  case 0x32:
    mem_op_1000_179c(0x26,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    pass1_1010_6abc((Struct635 *)paVar4,(u16)param_4,param_2);
    break;
  case 0x33:
    mem_op_1000_179c(0x72,param_4,0x1000);
    if (((uint)param_4 | (uint)paVar4) == 0) {
LAB_1010_25b8:
      u_var2 = 0;
      uVar7 = 0;
    }
    else {
      uVar10 = pass1_1010_195e((Struct79 *)paVar4,(Struct79 *)param_4,param_2);
      uVar7 = (u16)((u32)uVar10 >> 0x10);
      u_var2 = (u16)uVar10;
    }
    goto LAB_1010_25bb;
  case 0x34:
    mem_op_1000_179c(0x72,param_4,0x1000);
    if ((u8 *)((uint)param_4 | (uint)paVar4) == (u8 *)0x0) goto LAB_1010_25b8;
    uVar11 = pass1_1010_1b6e((Struct79 *)paVar4,(Struct79 *)param_4,param_2,param_3,
                             (u8 *)((uint)param_4 | (uint)paVar4));
    uVar7 = (u16)(uVar11 >> 0x10);
    u_var2 = (u16)uVar11;
LAB_1010_25bb:
    pu_stack6 = (u16 *)CONCAT22(uVar7,u_var2);
    pass1_1010_1146(CONCAT22(uVar7,u_var2),0,param_5,param_3);
    goto switchD_1010:2765_caseD_38;
  case 0x35:
    mem_op_1000_179c(0x14a,param_4,0x1000);
    if (((uint)param_4 | (uint)paVar4) == 0) goto LAB_1010_2126;
    uVar11 = pass1_1010_6700(paVar4,(u16)param_4,param_2);
    puVar3 = (u8 *)(uVar11 >> 0x10);
    paVar4 = (Struct636 *)uVar11;
    break;
  case 0x36:
    mem_op_1000_179c(0x10,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    pass1_1008_d790((Struct647 *)paVar4,(u16)param_4,param_2,param_3);
    break;
  case 0x37:
    mem_op_1000_179c(0x420,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    struct_1008_9fd2((Struct79 *)paVar4,(Struct79 *)param_4,param_2);
    break;
  default:
    goto switchD_1010:2765_caseD_38;
  case 0x39:
    mem_op_1000_179c(0x36,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    pass1_1010_4a8a((Struct637 *)paVar4,(u16)param_4,param_2,param_3);
    break;
  case 0x3a:
    mem_op_1000_179c(0xc,param_4,0x1000);
    if (((uint)param_4 | (uint)paVar4) == 0) goto LAB_1010_2126;
    puVar8 = pass1_1008_d72e((int)paVar4,(u16)param_4,param_2);
    puVar3 = (u8 *)((u32)puVar8 >> 0x10);
    paVar4 = (Struct636 *)puVar8;
    break;
  case 0x3b:
    mem_op_1000_179c(0x22,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    struct_1008_dd4e((Struct209 *)paVar4,(u16)param_4,param_2);
    break;
  case 0x3c:
    mem_op_1000_179c(0x146,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    pass1_1018_331c((Struct638 *)paVar4,(u16)param_4,param_2,param_3,puVar3);
    break;
  case 0x3d:
    mem_op_1000_179c(0xe,param_4,0x1000);
    if (((uint)param_4 | (uint)paVar4) == 0) goto LAB_1010_2126;
    uVar11 = pass1_1010_8c32((Struct640 *)paVar4,(u16)param_4,param_2,param_3);
    puVar3 = (u8 *)(uVar11 >> 0x10);
    paVar4 = (Struct636 *)uVar11;
    break;
  case 0x3e:
    mem_op_1000_179c(0x18,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    pass1_1018_5070((Struct641 *)paVar4,(u16)param_4,param_2);
    break;
  case 0x3f:
    mem_op_1000_179c(0x12,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    pass1_1008_c72a((Struct642 *)paVar4,(u16)param_4,param_2);
    break;
  case 0x40:
    mem_op_1000_179c(0x24,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    pass1_1008_af94((Struct643 *)paVar4,(u16)param_4,param_2);
    break;
  case 0x41:
    u_var2 = 0x60;
    mem_op_1000_179c(0x60,param_4,0x1000);
    if (((uint)param_4 | (uint)paVar4) == 0) goto LAB_1010_2680;
    uVar7 = SEG_1008;
    struct_1008_ecb2((Struct221 *)paVar4,(u16)param_4,param_2);
    puVar3 = extraout_DX;
LAB_1010_2683:
    *(Struct636 **)(param_2 * 4 + iVar5) = paVar4;
    *(u8 **)(param_2 * 4 + iVar5 + 2) = puVar3;
    ppcVar1 = (fn_ptr_1 **)((int)*(u32 *)paVar4 + 0x10);
    (**ppcVar1)(uVar7,paVar4,puVar3,u_var2);
    break;
  case 0x42:
    mem_op_1000_179c(0xc,param_4,0x1000);
    if (((uint)param_4 | (uint)paVar4) == 0) goto LAB_1010_2126;
    puVar8 = pass1_1008_ec10((int)paVar4,(u16)param_4,param_2);
    puVar3 = (u8 *)((u32)puVar8 >> 0x10);
    paVar4 = (Struct636 *)puVar8;
    break;
  case 0x43:
    mem_op_1000_179c(0x12,param_4,0x1000);
    if (((uint)param_4 | (uint)paVar4) == 0) goto LAB_1010_2126;
    puVar8 = pass1_1010_2bfc((Struct644 *)paVar4,(u16)param_4,param_2);
    puVar3 = (u8 *)((u32)puVar8 >> 0x10);
    paVar4 = (Struct636 *)puVar8;
    break;
  case 0x45:
    mem_op_1000_179c(0xe,param_4,0x1000);
    if (((uint)param_4 | (uint)paVar4) == 0) goto LAB_1010_2126;
    uVar11 = pass1_1010_0000((Struct645 *)paVar4,(u16)param_4,param_2,param_3);
    puVar3 = (u8 *)(uVar11 >> 0x10);
    paVar4 = (Struct636 *)uVar11;
    break;
  case 0x46:
    mem_op_1000_179c(0x1a,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    struct_1010_50b2((Struct646 *)paVar4,(u16)param_4,param_2);
    break;
  case 0x47:
    mem_op_1000_179c(0xe,param_4,0x1000);
    if (((uint)param_4 | (uint)paVar4) == 0) goto LAB_1010_2126;
    puVar8 = pass1_1018_56e6((int)paVar4,(u16)param_4,param_2);
    puVar3 = (u8 *)((u32)puVar8 >> 0x10);
    paVar4 = (Struct636 *)puVar8;
    break;
  case 0x48:
    mem_op_1000_179c(0x1c,param_4,0x1000);
    puVar3 = (u8 *)((uint)param_4 | (uint)paVar4);
    if (puVar3 == (u8 *)0x0) goto LAB_1010_2126;
    unk_draw_op_1008_da12((Struct19 *)paVar4,(u16)param_4,param_2);
  }
  pu_stack6 = (u16 *)CONCAT22(puVar3,paVar4);
switchD_1010:2765_caseD_38:
  *(u16 **)(param_2 * 4 + iVar5) = pu_stack6;
  return pu_stack6;
}

StructD * FUN_1008_9f8c(StructD *param_1,byte param_2)

{
  pass1_1008_9e5a(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}

StructD * FUN_1018_2ab4(StructD *param_1,byte param_2)

{
  pass1_1018_2440(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}

StructD * FUN_1018_e428(StructD *param_1,byte param_2)

{
  pass1_1018_e2a0(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}

StructD * FUN_1020_775a(StructD *param_1,byte param_2)

{
  pass1_1020_75c4(param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}