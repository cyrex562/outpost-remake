

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

StructD * pass1_1010_2024(StructD *param_1)

{
  let mut struct_1_hi: u16;

  struct_1_hi = ((u32)param_1 >> 0x10);
  ((int)param_1 + 0x124) = 0x0;
  _u16_1050_0ed0 = param_1;
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff | (u32)struct_1_hi << 0x10),NULL,0x124);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_2050(mut param_1: u32)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut uVar4: u16;
  let mut iStack4: i16;

  pass1_1010_2816(param_1);
  iStack4 = 0x0;
  do {
    uVar4 = (param_1 >> 0x10);
    puVar1 = (u32 *)(iStack4 * 0x4 + (int)param_1);
    uVar2 = (iStack4 * 0x4 + (int)param_1 + 0x2);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    iStack4 += 0x1;
  } while (iStack4 < 0x49);
  _u16_1050_0ed0 = 0x0;
  return;
}
pub fn pass1_1010_209e(mut param_1: u32,mut param_2: u16 )

{
  pass1_1010_2816(param_1);
  ((int)param_1 + 0x124) = param_2;
  return;
}



u32 * mixed_1010_20ba(param_1: *mut astruct_57,mut param_2: u32,u8 **param_3,mut param_4: u16 ,mut param_5: u16 ,
                     mut param_6: u16 ,mut param_7: u16 )

{
  code **ppcVar1;
  u8 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  astruct_638 *paVar5;
  let mut unaff_SI: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut puVar9: *mut u16;
  let mut uVar10: u32;
  let mut puVar11: *mut u16;
  let mut uVar12: u32;
  astruct_19 *paVar13;
  let mut in_stack_0000fe8a: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffb8: u16;
  u32 *puStack6;

  pass1_1010_2816(param_2);
  paVar5 = (astruct_638 *)(param_3 * 0x4);
  uVar7 = (param_2 >> 0x10);
  iVar6 = (int)param_2;
  puStack6 = (u32*)((int)&paVar5.field_0x0 + iVar6);
  if (puStack6 != NULL) {
    return puStack6;
  }
  switch(param_3) {
  case 0x1:
    mem_op_1000_179c(0x18,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) {//
LAB_1010_2126:
      paVar5 = NULL;
      puVar2 = NULL;
    }
    else {
      struct_1010_3b7a((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    }
    break;
  case 0x2:
    mem_op_1000_179c(0x84,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    win_sys_op_1010_5404(param_4,(astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    break;
  case 0x3:
    mem_op_1000_179c(0x53c,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    struct_1010_a1d8((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    break;
  case 0x4:
    mem_op_1000_179c(0x18a,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    struct_1018_2b10((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    break;
  case 0x5:
    mem_op_1000_179c(0x14,param_1);
    if ((param_1 | paVar5) == 0x0) goto LAB_1010_2126;
    puVar11 = pass1_1008_eabc((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    puVar2 = ((u32)puVar11 >> 0x10);
    paVar5 = (astruct_638 *)puVar11;
    break;
  case 0x6:
    mem_op_1000_179c(0x58,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    pass1_1018_18b8((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    break;
  case 0x7:
    mem_op_1000_179c(0xe,param_1);
    uVar4 = (astruct_19 *)param_1 | paVar5;
    if (uVar4 == 0x0) goto LAB_1010_2126;
    uVar10 = pass1_1010_3d82((u32)param_1 & 0xffff0000 | (u32)uVar4,(astruct_19 *)paVar5,(astruct_19 *)param_1,
                             param_3);
    puVar2 = (uVar10 >> 0x10);
    paVar5 = (astruct_638 *)uVar10;
    break;
  case 0x8:
    mem_op_1000_179c(0x20,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    struct_1010_95aa((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    break;
  case 0x9:
    mem_op_1000_179c(0x26,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    struct_1010_6326((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    break;
  case 0xa:
    mem_op_1000_179c(0x1c,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    paVar13 = pass1_1010_0eac(puVar2,(astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    puVar2 = ((u32)paVar13 >> 0x10);
    paVar5 = (astruct_638 *)paVar13;
    break;
  case 0xb:
    mem_op_1000_179c(0x1c,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    uVar10 = pass1_1008_aefe(puVar2,(astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    puVar2 = (uVar10 >> 0x10);
    paVar5 = (astruct_638 *)uVar10;
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
    mem_op_1000_179c(0xaa,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    struct_1018_0570((astruct_19 *)CONCAT22(param_1,paVar5),param_3,param_5);
    break;
  case 0x25:
    mem_op_1000_179c(0x1c,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    pass1_1018_4aaa(puVar2,(astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    break;
  case 0x26:
    mem_op_1000_179c(0x1c,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    pass1_1008_d99e(puVar2,(astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    break;
  case 0x27:
    mem_op_1000_179c(0x58,param_1);
    puVar2 = ((astruct_19 *)param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    pass1_1008_9d36((astruct_19 *)paVar5,(astruct_19 *)param_1,param_3);
    break;
  case 0x28:
    mem_op_1000_179c(0x2c,param_1);
    uVar4 = (astruct_19 *)param_1 | paVar5;
    uVar10 = (u32)param_1 & 0xffff0000 | (u32)uVar4;
    if (uVar4 == 0x0) goto LAB_1010_2126;
    pass1_1010_28e6(uVar10,0x1000,(astruct_19 *)paVar5,(astruct_19 *)param_1,param_3);
    puVar2 = uVar10;
    break;
  case 0x29:
    mem_op_1000_179c(0x72,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    struct_1018_229c(puVar2,(astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    break;
  case 0x2a:
    mem_op_1000_179c(0x1c,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    pass1_1010_503e(puVar2,(astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    break;
  case 0x2b:
    mem_op_1000_179c(0x1a,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    struct_1010_02e0((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    break;
  case 0x2c:
    mem_op_1000_179c(0x10,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    pass1_1008_eb2a((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    break;
  case 0x2d:
    mem_op_1000_179c(0x80,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    pass1_1010_3e3c((astruct_19 *)CONCAT22(param_1,paVar5),param_3,param_6);
    break;
  case 0x2e:
    mem_op_1000_179c(0x806,param_1);
    if ((param_1 | paVar5) == 0x0) goto LAB_1010_2126;
    uVar10 = pass1_1018_1ff4((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    puVar2 = (uVar10 >> 0x10);
    paVar5 = (astruct_638 *)uVar10;
    break;
  case 0x2f:
    mem_op_1000_179c(0x58,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    struct_1010_e9e4((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    break;
  case 0x30:
    mem_op_1000_179c(0xe,param_1);
    if ((param_1 | paVar5) == 0x0) goto LAB_1010_2126;
    puVar9 = pass1_1010_3702((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    puVar2 = ((u32)puVar9 >> 0x10);
    paVar5 = (astruct_638 *)puVar9;
    break;
  case 0x31:
    uVar8 = 0x1000;
    mem_op_1000_179c(0x60,param_1);
    uVar4 = param_1 | paVar5;
    if (uVar4 == 0x0) {//
LAB_1010_2680:
      uVar8 = 0x1000;
      paVar5 = NULL;
      puVar2 = NULL;
    }
    else {
      uVar10 = pass1_1010_9298((StructD *)CONCAT22(paVar5,uVar4),(astruct_19 *)CONCAT22(param_1,paVar5),
                               param_3);
      puVar2 = (uVar10 >> 0x10);
      paVar5 = (astruct_638 *)uVar10;
    }
    goto LAB_1010_2683;
  case 0x32:
    mem_op_1000_179c(0x26,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    pass1_1010_6abc((astruct_19 *)CONCAT22(param_1,paVar5),param_3,param_6);
    break;
  case 0x33:
    mem_op_1000_179c(0x72,param_1);
    uVar4 = (astruct_19 *)param_1 | paVar5;
    if (uVar4 == 0x0) {//
LAB_1010_25b8:
      uVar8 = 0x0;
      uVar3 = 0x0;
    }
    else {
      paVar13 = pass1_1010_195e((u32)param_1 & 0xffff0000 | (u32)uVar4,(astruct_19 *)paVar5,(astruct_19 *)param_1,
                                param_3);
      uVar3 = ((u32)paVar13 >> 0x10);
      uVar8 = SUB42(paVar13,0x0);
    }
    goto LAB_1010_25bb;
  case 0x34:
    mem_op_1000_179c(0x72,param_1);
    uVar4 = (astruct_19 *)param_1 | paVar5;
    if (uVar4 == 0x0) goto LAB_1010_25b8;
    paVar13 = pass1_1010_1b6e((StructD *)((u32)param_1 & 0xffff0000 | (u32)uVar4),(astruct_19 *)paVar5,
                              (astruct_19 *)param_1,param_3);
    uVar3 = ((u32)paVar13 >> 0x10);
    uVar8 = SUB42(paVar13,0x0);//
LAB_1010_25bb:
    puStack6 = (u32 *)CONCAT22(uVar3,uVar8);
    pass1_1010_1146(CONCAT22(uVar3,uVar8),0x0);
    goto switchD_1010_2765_caseD_38;
  case 0x35:
    mem_op_1000_179c(0x14a,param_1);
    if ((param_1 | paVar5) == 0x0) goto LAB_1010_2126;
    paVar13 = pass1_1010_6700((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    puVar2 = ((u32)paVar13 >> 0x10);
    paVar5 = (astruct_638 *)paVar13;
    break;
  case 0x36:
    mem_op_1000_179c(0x10,param_1);
    puVar2 = ((astruct_19 *)param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    pass1_1008_d790((astruct_19 *)paVar5,(astruct_19 *)param_1,param_3);
    break;
  case 0x37:
    mem_op_1000_179c(0x420,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    struct_1008_9fd2((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    break;
  default:
    goto switchD_1010_2765_caseD_38;
  case 0x39:
    mem_op_1000_179c(0x36,param_1);
    uVar4 = (astruct_19 *)param_1 | paVar5;
    uVar10 = (u32)param_1 & 0xffff0000 | (u32)uVar4;
    if (uVar4 == 0x0) goto LAB_1010_2126;
    pass1_1010_4a8a(uVar10,(astruct_19 *)paVar5,(astruct_19 *)param_1,param_3,param_7,in_stack_0000fe8a,
                    in_stack_0000ffae,in_stack_0000ffb4,in_stack_0000ffb8);
    puVar2 = uVar10;
    break;
  case 0x3a:
    mem_op_1000_179c(0xc,param_1);
    if ((param_1 | paVar5) == 0x0) goto LAB_1010_2126;
    puVar9 = pass1_1008_d72e((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    puVar2 = ((u32)puVar9 >> 0x10);
    paVar5 = (astruct_638 *)puVar9;
    break;
  case 0x3b:
    mem_op_1000_179c(0x22,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    struct_1008_dd4e((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    break;
  case 0x3c:
    mem_op_1000_179c(0x146,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    pass1_1018_331c(puVar2,paVar5,param_1,param_3);
    break;
  case 0x3d:
    mem_op_1000_179c(0xe,param_1);
    if ((param_1 | paVar5) == 0x0) goto LAB_1010_2126;
    uVar10 = pass1_1010_8c32((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    puVar2 = (uVar10 >> 0x10);
    paVar5 = (astruct_638 *)uVar10;
    break;
  case 0x3e:
    mem_op_1000_179c(0x18,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    pass1_1018_5070((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    break;
  case 0x3f:
    mem_op_1000_179c(0x12,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    pass1_1008_c72a((astruct_19 *)CONCAT22(param_1,paVar5),param_3,unaff_SI);
    break;
  case 0x40:
    mem_op_1000_179c(0x24,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    pass1_1008_af94((astruct_19 *)CONCAT22(param_1,paVar5),param_3,unaff_SI);
    break;
  case 0x41:
    mem_op_1000_179c(0x60,param_1);
    uVar4 = param_1 | paVar5;
    if (uVar4 == 0x0) goto LAB_1010_2680;
    uVar8 = 0x1008;
    uVar12 = struct_1008_ecb2(paVar5,uVar4,(astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    puVar2 = ((u32)uVar12 >> 0x10);
    paVar5 = (astruct_638 *)uVar12;//
LAB_1010_2683:
    *(astruct_638 **)(param_3 * 0x4 + iVar6) = paVar5;
    *(u8 **)(param_3 * 0x4 + iVar6 + 0x2) = puVar2;
    ppcVar1 = (code **)((int)(u32)paVar5 + 0x10);
    (**ppcVar1)(uVar8,paVar5,puVar2);
    break;
  case 0x42:
    mem_op_1000_179c(0xc,param_1);
    if ((param_1 | paVar5) == 0x0) goto LAB_1010_2126;
    puVar9 = pass1_1008_ec10((int)paVar5,param_1,param_3);
    puVar2 = ((u32)puVar9 >> 0x10);
    paVar5 = (astruct_638 *)puVar9;
    break;
  case 0x43:
    mem_op_1000_179c(0x12,param_1);
    if ((param_1 | paVar5) == 0x0) goto LAB_1010_2126;
    puVar9 = pass1_1010_2bfc((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    puVar2 = ((u32)puVar9 >> 0x10);
    paVar5 = (astruct_638 *)puVar9;
    break;
  case 0x45:
    mem_op_1000_179c(0xe,param_1);
    if ((param_1 | paVar5) == 0x0) goto LAB_1010_2126;
    paVar13 = pass1_1010_0000((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    puVar2 = ((u32)paVar13 >> 0x10);
    paVar5 = (astruct_638 *)paVar13;
    break;
  case 0x46:
    mem_op_1000_179c(0x1a,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    struct_1010_50b2((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    break;
  case 0x47:
    mem_op_1000_179c(0xe,param_1);
    if ((param_1 | paVar5) == 0x0) goto LAB_1010_2126;
    puVar9 = pass1_1018_56e6((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
    puVar2 = ((u32)puVar9 >> 0x10);
    paVar5 = (astruct_638 *)puVar9;
    break;
  case 0x48:
    mem_op_1000_179c(0x1c,param_1);
    puVar2 = (param_1 | paVar5);
    if (puVar2 == NULL) goto LAB_1010_2126;
    unk_draw_op_1008_da12((astruct_19 *)CONCAT22(param_1,paVar5),param_3);
  }
  puStack6 = (u32 *)CONCAT22(puVar2,paVar5);
switchD_1010_2765_caseD_38:
  (u32*)(param_3 * 0x4 + iVar6) = puStack6;
  return puStack6;
}
pub fn pass1_1010_2816(mut param_1: u32)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut iVar5: i16;
  let mut uVar6: u16;

  uVar6 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  if ((iVar4 + 0x124) != 0x0) {
    iVar5 = (iVar4 + 0x124) * 0x4;
    puVar1 = (u32 *)(iVar5 + iVar4);
    uVar2 = (iVar5 + iVar4 + 0x2);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    (u32)((iVar4 + 0x124) * 0x4 + iVar4) = 0x0;
    (iVar4 + 0x124) = 0x0;
  }
  return;
}



u16 pass1_1010_286c(void)

{
  let mut puVar1: *mut u16;

  pass1_1008_3e54((u16 *)&PTR_LOOP_1048_0000,0x0,0x5,0x12c);
  pass1_1008_3e54((u16 *)(s__1050_65a0 + 0x6),0x0,0x9b,0x20);
  pass1_1008_3e54((u16 *)0x105065ac,0x0,0xf5,0x3f);
  pass1_1008_3e54((u16 *)0x105065b2,0x0,0x114,0x4a);
  pass1_1008_3e54((u16 *)0x105065b8,0x0,0x135,0x45);
  pass1_1008_3e54((u16 *)0x105065be,0x0,0xf5,0x7b);
  puVar1 = pass1_1008_3e54((u16 *)0x105065c4,0x0,0x117,0x91);
  return puVar1;
}
pub fn pass1_1010_28e6(mut param_1: u32,mut param_2: u16 ,param_3: *mut astruct_19,param_4: *mut astruct_19,mut param_5: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut puVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut unaff_DS: u16;
  let mut iStack6: i16;

  struct_op_1018_4cda((astruct_19 *)CONCAT22(param_4,param_3),param_5);
  uVar5 = 0x0;
  (u32)&param_3.field15_0x1c = 0x0;
  param_3.field17_0x20 = 0x0;
  param_3.field18_0x22 = 0x0;
  param_3.field19_0x24 = 0x0;
  &param_3.field20_0x26 = 0x0;
    // 0x2bdc
  CONCAT22(param_4,param_3) = 0x2be4;
  param_3.segment_0x2 = 0x1010;
  *(astruct_19 **)&PTR_LOOP_1050_4230 = param_3;
  *(astruct_19 **)0x4232 = param_4;
  pass1_1018_4dce(param_1,(astruct_19 *)CONCAT22(param_4,param_3),0x56);
  uVar2 = FUN_1010_830a(uVar5,param_1,0x1018,*(astruct_87 **)&u32_1048_14cc,0x4);
  param_3.field15_0x1c = uVar2;
  param_3.field16_0x1e = param_1;
  if (*(i32 *)0x5f2c == 0x0) {
    puVar2 = mem_op_1000_160a((StructD *)param_1);
  }
  else {
    puVar2 = 0x5f2c;
    param_1 = param_1 & 0xffff0000 | (u32)0x5f2e;
  }
  uVar4 = fn_ptr_op_1000_1708(0x40,0x0,0x1,puVar2,param_1);
  ((int)&param_3.field20_0x26 + 0x2) = uVar4;
  param_3.field21_0x2a = param_1;
  for (iStack6 = 0x0; iStack6 < 0x10; iStack6 += 0x1) {
    uVar5 = FUN_1010_830a(iStack6 + 0x56,param_1,0x1000,*(astruct_87 **)&u32_1048_14cc,iStack6 + 0x56);
    uVar1 = (u32)((int)&param_3.field20_0x26 + 0x2);
    uVar7 = ((u32)uVar1 >> 0x10);
    iVar6 = (int)uVar1;
    (iVar6 + iStack6 * 0x4) = uVar5;
    (iVar6 + iStack6 * 0x4 + 0x2) = (int)param_1;
  }
  return;
}
pub fn pass1_1010_29c6(StructD *param_1)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  StructD *iVar5;
  let mut uVar4: u16;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar5 = (StructD *)param_1;
  param_1.address_offset_field_0x0 = (int)s_add16_wav_1050_2bdc + 0x8;
  iVar5.address_offset_field_0x2 = 0x1010;
  if (*(i32 *)&iVar5.field_0x1c != 0x0) {
    puVar1 = (u32*)&iVar5.field_0x1c;
    uVar2 = &iVar5.field_0x1e;
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    (u32)&iVar5.field_0x1c = 0x0;
    fn_ptr_1000_17ce(*(char **)&iVar5.field_0x28);
    (u32)&iVar5.field_0x28 = 0x0;
  }
  clenaup_win_ui_1018_4d22(param_1);
  return;
}



// WARNING: Instruction at (ram,0x10104b2b) overlaps instruction at (ram,0x10104b2a)
//
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: stack

u16_t FUN_1010_2a32(u8 *buffer_param_2,mut param_2: u32,HFILE16 *hfile_param,mut param_4: u16 )

{
  let mut piVar1: *mut i16;
  char *pcVar2;
  u8 *pbVar3;
  u8 bVar4;
  u32 *puVar5;
  u32 *puVar6;
  code **ppcVar7;
  code *pcVar8;
  let mut uVar9: u32;
  let mut uVar10: u16;
  HDC16 HVar11;
  HPALETTE16 HVar12;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut BVar15: bool;
  let mut iVar16: i16;
  u8 bVar17;
  u8 *puVar18;
  u8 *puVar19;
  let mut uVar20: u16;
  let mut uVar21: u16;
  let mut in_EDX: u32;
  u32 *in_BX;
  let mut unaff_BP: u16;
  let mut unaff_SI: i16;
  let mut iVar23: i16;
  let mut iVar24: i16;
  let mut unaff_ES: u16;
  let mut uVar25: u16;
  u8 bVar26;
  let mut bVar27: bool;
  DEVMODEA *init_data;
  let mut in_stack_00000000: i16;
  let mut in_stack_00000002: u16;
  let mut uStack54: u16;
  let mut puStack46: *mut u16;
  let mut uStack42: u16;
  u8 *read_buffer_38;
  u8 *read_buffer_22;
  HGDIOBJ16 HStack30;
  HGDIOBJ16 HStack28;
  u8 in_stack_0000ffec;
  u8 uVar28;
  u8 in_stack_0000ffed;
  u8 uVar29;
  u8 uVar30;
  u8 uVar31;
  u8 uVar32;
  u8 uVar33;
  u8 uVar34;
  u8 uVar35;
  u8 uVar36;
  u8 uVar37;
  u8 uVar38;
  astruct_57 *paVar22;

  uVar28 = (u8)unaff_BP;
  uVar29 = (u8)(unaff_BP >> 0x8);
  bVar26 = 0x0;
  uVar38 = 0x0;
  if ((param_2 + 0xec76 & 0x3) != 0x0) goto LAB_1010_2ad8;
  uVar10 = param_2 + 0xec76 >> 0x1;
  if (0x1c < uVar10) goto LAB_1010_2ad8;
  uVar14 = in_EDX;
  switch(uVar10) {
  default:
    goto switchD_1010_2ab5_caseD_0;
  case 0x1:
  case 0x3:
  case 0xb:
    (uVar10 + 0xa) = in_BX;
  case 0x9:
  case 0xf:
  case 0x15:
  case 0x1b:
    (uVar10 + 0xa) = in_BX;
    (uVar10 + 0x10) = in_BX;
    (uVar10 + 0xc) = in_BX;
    return uVar10;
  case 0x5:
    BVar15 = write_to_file_1008_7e1c
                       (param_2,ZEXT24(in_BX),
                        (char *)CONCAT13((char)(in_stack_00000000 >> 0x8),
                                         CONCAT12((char)in_stack_00000000,unaff_BP)),
                        CONCAT11(in_stack_0000ffed,in_stack_0000ffec));
    if (BVar15 != 0x0) {
      return 0x1;
    }
    u16_1050_0310 = 0x6d0;
    return 0x0;
  case 0x6:
    bVar26 = 0x0;
    goto LAB_1010_2ad8;
  case 0x7:
    ppcVar7 = (code **)*in_BX;
    (**ppcVar7)();
    iVar16 = (int)(u32 *)param_2 + 0x105;
    puVar18 = in_EDX;
    pass1_1010_8170(puVar18,_u16_1050_14cc,iVar16);
    iVar23 = (int)(u32 *)param_2 * 0x4;
    ((int)buffer_param_2 + iVar23 + 0x26) = iVar16;
    *(u8 **)((int)buffer_param_2 + iVar23 + 0x28) = puVar18;
    uVar36 = 0x50;
    uVar37 = 0x10;
    uVar34 = 0x80;
    uVar35 = 0x13;
    uVar30 = 0x0;
    uVar31 = 0x0;
    uVar32 = 0x0;
    uVar33 = 0x0;
    uVar28 = 0x0;
    uVar29 = 0x0;
    init_data = (DEVMODEA *)pass1_1008_4772(*(astruct_76 **)((int)buffer_param_2 + 0x26 + iVar23));
    uVar25 = ((u32)init_data >> 0x10);
    HVar11 = CreateDC16(init_data,(char *)CONCAT13(uVar29,CONCAT12(uVar28,uVar25)),
                        (char *)CONCAT13(uVar33,CONCAT12(uVar32,CONCAT11(uVar31,uVar30))),
                        (char *)CONCAT13(uVar37,CONCAT12(uVar36,CONCAT11(uVar35,uVar34))));
    uVar28 = (u8)HVar11;
    uVar29 = (u8)(HVar11 >> 0x8);
    HVar12 = palette_op_1008_4e08
                       ((HPALETTE16)&stack0xffec,uVar25,*(astruct_13 **)((int)_PTR_LOOP_1050_4230 + 0xe),
                        (HDC16 *)CONCAT13(0x10,CONCAT12(0x50,&stack0xffec)));
    HStack28 = SelectObject16(CONCAT11(uVar38,bVar26),CONCAT11(uVar29,uVar28));
    HStack30 = SelectObject16(CONCAT11(uVar29,uVar28),CONCAT11(uVar29,uVar28));
    for (read_buffer_22 = 0x0; piVar1 = ((int)buffer_param_2 + 0x74),
        *piVar1 != read_buffer_22 && read_buffer_22 <= *piVar1; read_buffer_22 += 0x1) {
      iVar16 = (read_buffer_22 * 0x10 + (int)(u32 *)param_2) * 0x8;
      uVar25 = ((int)buffer_param_2 + 0x72);
      uVar13 = pass1_1000_484c(CONCAT13(0x10,CONCAT12(0x50,&stack0xfff2)),
                               CONCAT13((char)(uVar25 >> 0x8),
                                        CONCAT12((char)uVar25,iVar16 + ((int)buffer_param_2 + 0x70))),0x8);
      if (uVar13 != 0x0) {
        uVar9 = (u32)((int)buffer_param_2 + 0x70);
        uVar25 = ((u32)uVar9 >> 0x10);
        iVar23 = (int)uVar9;
        iVar24 = iVar16 + iVar23;
        Rectangle16(*(INT16 *)(iVar24 + 0x6),*(INT16 *)(iVar24 + 0x4),*(INT16 *)(iVar24 + 0x2),
                    *(INT16 *)(iVar23 + iVar16),CONCAT11(uVar29,uVar28));
      }
    }
    HVar12 = SelectPalette16(0x0,HVar12,CONCAT11(uVar29,uVar28));
    DeleteObject16(HVar12);
    SelectObject16(HStack28,CONCAT11(uVar29,uVar28));
    SelectObject16(HStack30,CONCAT11(uVar29,uVar28));
    uVar38 = 0x38;
    uVar30 = 0x15;
    DeleteDC16(CONCAT11(uVar29,uVar28));
    BVar15 = DeleteObject16(CONCAT11(uVar30,uVar38));
    return BVar15;
  case 0x8:
    bVar26 = 0x3;
    goto LAB_1010_2ad8;
  case 0xd:
    pbVar3 = (uVar10 + unaff_SI);
    bVar26 = *pbVar3;
    bVar4 = *pbVar3 + (u8)in_EDX;
    *pbVar3 = bVar4 + (uVar10 < 0x1c);
    puVar5 = (u32 *)(CARRY1(bVar26,(u8)in_EDX) || CARRY1(bVar4,uVar10 < 0x1c));
    puVar6 = in_BX + -0x404;
    bVar26 = in_BX < (u32 *)0x1010 || puVar6 < puVar5;
    uVar21 = (int)puVar6 - (int)puVar5;
    pcVar8 = (code *)swi(0x4);
    if (SBORROW2((int)in_BX,0x1010) != SBORROW2((int)puVar6,(int)puVar5)) {
      (*pcVar8)();
    }
    puVar19 = in_EDX;
    bVar27 = uVar21 < 0x1010 || uVar21 + 0xeff0 < bVar26;
    pbVar3 = (uVar10 + unaff_SI);
    bVar26 = *pbVar3;
    bVar17 = (u8)in_EDX;
    bVar4 = *pbVar3;
    *pbVar3 = bVar4 + bVar17 + bVar27;
    pcVar2 = (char *)(uVar10 + unaff_SI);
    *pcVar2 = *pcVar2 + bVar17 + (CARRY1(bVar26,bVar17) || CARRY1(bVar4 + bVar17,bVar27));
    struct_op_1018_4cda((astruct_19 *)CONCAT13((char)in_stack_00000000,CONCAT12(uVar29,CONCAT11(uVar28,uVar38))),
                        CONCAT11((char)in_stack_00000002,(char)(in_stack_00000000 >> 0x8)));
    iVar16 = CONCAT11(uVar28,uVar38);
    uVar30 = (u8)in_stack_00000000;
    piVar1 = CONCAT13(uVar30,CONCAT12(uVar29,iVar16));
    *piVar1 = (int)s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x1;
    (iVar16 + 0x2) = 0x1010;
    pass1_1018_4dce(puVar19,(astruct_19 *)CONCAT13(uVar30,CONCAT12(uVar29,iVar16)),0x1b3);
    _PTR_LOOP_1050_4230 = CONCAT13(uVar30,CONCAT12(uVar29,CONCAT11(uVar28,uVar38)));
    return CONCAT11(uVar28,uVar38);
  case 0xe:
    ((int)buffer_param_2 + 0x20) = (u32 *)param_2;
    break;
  case 0x11:
    do {
    // WARNING: Do nothing block with infinite loop
    } while( true );
  case 0x12:
    PTR_LOOP_1050_10c6 = (0x0 < (int)(u32 *)param_2);
    PTR_LOOP_1050_1142 = (0x2 < (int)(u32 *)param_2);
    break;
  case 0x13:
    iVar16 = (int)buffer_param_2 * 0x8 + in_stack_00000000;
    if ((((iVar16 + 0x22) != 0x0) || ((iVar16 + 0x24) != 0x0)) ||
       (((iVar16 + 0x26) != 0x0 || ((iVar16 + 0x28) != 0x0)))) {
      HStack28 = 0x1010;
      HStack30 = 0x627c;
      sys_1000_3f9c(*(char **)(in_stack_00000000 + 0xe),s__d__d__d__d_1050_14ae,
                    (u32)((int)buffer_param_2 * 0x8 + in_stack_00000000 + 0x22));
      HStack28 = 0x1000;
      HStack30 = 0x62a0;
      in_BX = (u32 *)
              WritePrivateProfileString16
                        (*(char **)(in_stack_00000000 + 0xa),*(char **)(in_stack_00000000 + 0xe),
                         *(char **)((int)buffer_param_2 * 0x4 + 0x1446),s_windows_1050_13b8);
    }
    return (u16_t)in_BX;
  case 0x14:
    ((int)buffer_param_2 + 0x24) = (u32 *)param_2;
    break;
  case 0x17:
    uVar21 = uVar14 - 0x1;
    paVar22 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uVar21);
    pbVar3 = (uVar10 + unaff_SI);
    *pbVar3 = *pbVar3 | (u8)uVar21;
    ((int)buffer_param_2 + 0x12) = in_BX;
    ((int)buffer_param_2 + 0x14) = uVar21;
    uStack42 = 0x0;
    while( true ) {
      if (uStack54 <= uStack42) {
        uVar38 = (u8)((u32)buffer_param_2 >> 0x10);
        BVar15 = read_file_1008_7dee((HFILE16 *)param_2,
                                     (((u32)buffer_param_2 & 0xff00) << 0x10 |
                                           (u32)CONCAT12(uVar38,(int)buffer_param_2 + 0x1a)),0x2);
        if (((BVar15 != 0x0) &&
            (BVar15 = read_file_1008_7dee((HFILE16 *)param_2,
                                          (((u32)buffer_param_2 & 0xff00) << 0x10 |
                                                (u32)CONCAT12(uVar38,(int)buffer_param_2 + 0x1c)),0x2), BVar15 != 0x0)
            ) && (BVar15 = read_file_1008_7dee((HFILE16 *)param_2,
                                               (((u32)buffer_param_2 & 0xff00) << 0x10 |
                                                     (u32)CONCAT12(uVar38,(int)buffer_param_2 + 0x1e)),0x2),
                 BVar15 != 0x0)) {
          return 0x1;
        }
        u16_1050_0310 = 0x6d2;
        return 0x0;
      }
      uVar10 = uStack54;
      mem_op_1000_179c(0x8,paVar22);
      uVar21 = paVar22;
      puStack46 = (u16 *)CONCAT22(uVar21,uVar10);
      paVar22 = (astruct_57 *)((u32)paVar22 & 0xffff0000 | (u32)(uVar21 | uVar10));
      if ((uVar21 | uVar10) == 0x0) {
        read_buffer_38 = NULL;
      }
      else {
        *puStack46 = 0x389a;
        (uVar10 + 0x2) = 0x1008;
        *puStack46 = 0xa1c4;
        (uVar10 + 0x2) = 0x1010;
        read_buffer_38 = puStack46;
      }
      BVar15 = read_file_1008_7dee((HFILE16 *)param_2,CONCAT13(0x10,CONCAT12(0x50,&read_buffer_22)),0x2);
      if ((BVar15 == 0x0) ||
         (BVar15 = read_file_1008_7dee((HFILE16 *)param_2,
                                       ((u32)read_buffer_38 & 0xff000000 |
                                             (u32)CONCAT12((char)((u32)read_buffer_38 >> 0x10),
                                                             (int)read_buffer_38 + 0x6)),0x2), BVar15 == 0x0)) break;
      iVar16 = switch_1008_73ea((u32 *)param_2,param_2,(int)read_buffer_22);
      ((int)read_buffer_38 + 0x4) = iVar16;
      ppcVar7 = (code **)((int)(u32)(u32)((int)buffer_param_2 + 0x12) + 0x4);
      (**ppcVar7)();
      uStack42 += 0x1;
    }
    if (read_buffer_38 == NULL) {
      u16_1050_0310 = 0x6d2;
      return 0x0;
    }
    ppcVar7 = (code **)(u32)read_buffer_38;
    (**ppcVar7)();
    u16_1050_0310 = 0x6d2;
    return 0x0;
  case 0x18:
    bVar26 = 0x2;
    goto LAB_1010_2ad8;
  case 0x19:
    uVar14 = pass1_1010_6ca2(uVar14,(u32)buffer_param_2,0x8);
    uVar20 = in_EDX;
    if (uVar14 != 0x0) {
      pass1_1010_715c(uVar14,uVar20,CONCAT22(0x1a,(int)buffer_param_2),0x1a);
      buffer_param_2 = 0x1a001a;
    }
    if ((u32 *)param_2 == (u32 *)0x2c) {
      pass1_1010_715c(uVar14,uVar20,CONCAT22(0x1d,(int)buffer_param_2),0x1d);
    }
    uVar14 = pass1_1010_6ca2(uVar20,0x5a,0x2);
    if (uVar14 != 0x0) {
      pass1_1010_715c(uVar14,uVar20,0x1c005a,0x1c);
    }
    return 0x1;
  case 0x1a:
    ((int)buffer_param_2 + 0x26) = (u32 *)param_2;
  }
  bVar26 = 0x1;//
LAB_1010_2ad8:
  if ((bVar26 == 0x1) || (bVar26 == 0x2)) {
    if (bVar26 == 0x1) {
      param_2 = (u32)(((int)buffer_param_2 + 0x20) + ((int)buffer_param_2 + 0x22) +
                            ((int)buffer_param_2 + 0x24) + ((int)buffer_param_2 + 0x26));
    }
    if ((u32 *)param_2 != NULL) {
      uVar10 = (int)(u32 *)param_2 / 0x2 + 0x1;
      if (0x5 < (int)uVar10) {
        uVar10 = 0x5;
      }
      param_2 = (u32)uVar10;
      if ((bVar26 == 0x1) && (PTR_LOOP_1050_10c6 != NULL)) {
        if (0x4 < (int)uVar10) {
          uVar10 = 0x4;
        }
        param_2 = (u32)uVar10;
      }
    }
  }
  (bVar26 * 0x7c + 0xed6) = (u32 *)param_2;
  in_BX = (u32 *)param_2;
  pass1_1010_1f62((astruct_27 *)buffer_param_2,0xc);
switchD_1010_2ab5_caseD_0:
  return (u16_t)in_BX;
}
pub fn pass1_1010_2b50(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16)

{
  pass1_1008_3f62(param_3,(u16 *)&PTR_LOOP_1048_0000);
  return;
}



pub fn pass1_1010_2b66(mut param_1: u32) -> u32

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22(((int)param_1 + 0x1e),((int)param_1 + 0x1c));
}
pub fn pass1_1010_2b78(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,mut param_4: u32)

{
  u32 *puVar1;
  u32 *puVar2;
  u32 *puVar3;
  let mut iVar4: i16;
  u32 *puVar5;

  puVar3 = (u32 *)(param_3 * 0x7c + 0xed4);
  puVar5 = (u32 *)param_4;
  for (iVar4 = 0x1f; iVar4 != 0x0; iVar4 += -0x1) {
    puVar2 = puVar5;
    puVar5 = puVar5 + 0x1;
    puVar1 = puVar3;
    puVar3 = puVar3 + 0x1;
    *puVar2 = *puVar1;
  }
  return;
}



astruct_76 * pass1_1010_2b98(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar1 = (u32)((int)param_1 + 0x28);
  uVar3 = ((u32)uVar1 >> 0x10);
  iVar2 = (int)uVar1;
  return (astruct_76 *)
         CONCAT22((param_2 * 0x4 + iVar2 + -0x156),(param_2 * 0x4 + iVar2 + -0x158));
}
pub fn pass1_1010_2bb9(void)

{
  pass1_1010_286c();
  return;
}



StructD * pass1_1010_2bbe(StructD *param_1,param_2: u8)

{
  pass1_1010_29c6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1010_2bfc(param_1: *mut astruct_19,mut param_2: u16 )

{
  struct_op_1010_1d48(param_1,param_2);
  ((int)param_1 + 0xa) = 0x0;
  ((int)param_1 + 0xc) = 0x0;
  ((int)param_1 + 0xe) = 0x0;
  ((int)param_1 + 0x10) = 0x0;
  param_1.offset_0x0 = 0x2cc2;
  ((int)param_1 + 0x2) = 0x1010;
  return &param_1.offset_0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

char * unk_load_str_op_1010_2c34(u32 *param_1)

{
  char *in_buffer_4;
  let mut pUVar1: *mut u16;
  short in_buf_len_5;
  short sVar2;
  astruct_57 *in_EDX;
  astruct_57 *paVar3;
  u32 *puVar4;
  let mut in_stack_0000fe9e: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000fff6: u16;

  puVar4 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff6,0x3),in_stack_0000fe9e,
                           in_stack_0000ffc2,in_stack_0000ffc8,in_stack_0000ffcc);
  paVar3 = (astruct_57 *)((u32)in_EDX & 0xffff0000 | (u32)puVar4 >> 0x10);
  in_buffer_4 = (char *)puVar4;
  mem_op_1000_179c(0x80,paVar3);
  in_buf_len_5 = (short)paVar3;
  sVar2 = in_buf_len_5;
  load_string_1010_84e0(_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x80,in_buffer_4,in_buf_len_5);
  pUVar1 = pass1_1000_3cea(CONCAT22(in_buf_len_5,in_buffer_4),s__1050_11c8);
  pass1_1010_e964(sVar2);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,in_buffer_4),(char *)CONCAT22(sVar2,pUVar1));
  return in_buffer_4;
}



u16 * pass1_1010_2c9c(param_1: *mut u16,param_2: u8)

{
  pass1_1010_1d80((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn struct_1010_2cd2(param_1: *mut astruct_19,mut param_2: u16 )

{
  let mut in_EDX: u32;
  let mut uVar1: u16;
  astruct_19 *paVar2;
  u32 *puVar3;
  let mut in_stack_0000fe82: u16;
  let mut in_stack_0000ffa6: u16;
  let mut in_stack_0000ffac: u16;
  let mut in_stack_0000ffb0: u16;
  let mut piVar4: *mut i16;
  let mut uVar5: u16;
  let mut piVar6: *mut i16;
  let mut uVar7: u16;
  let mut local_6: i16;
  let mut local_4: i16;

  uVar1 = ((u32)in_EDX >> 0x10);
  paVar2 = struct_op_1010_1d48(param_1,param_2);
  (u32)((int)param_1 + 0x12) = 0x0;
  ((int)param_1 + 0x16) = 0x0;
  ((int)param_1 + 0x18) = 0x0;
  ((int)param_1 + 0x22) = 0x0;
  ((int)param_1 + 0x24) = 0x0;
  ((int)param_1 + 0x26) = 0x0;
  ((int)param_1 + 0x28) = 0x0;
  (u32)((int)param_1 + 0x52) = 0x0;
  (u32)((int)param_1 + 0x56) = 0x0;
  ((int)param_1 + 0x5a) = 0x0;
  ((int)param_1 + 0x5e) = 0x0;
  ((int)param_1 + 0x5c) = 0x0;
  param_1.offset_0x0 = 0x36da;
  ((int)param_1 + 0x2) = 0x1010;
  piVar6 = &local_4;
  uVar7 = SUB42(&DAT_1050_1050,0x0);
  piVar4 = &local_6;
  uVar5 = SUB42(&DAT_1050_1050,0x0);
  puVar3 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar1,(int)((u32)paVar2 >> 0x10)),_u16_1050_0ed0,
                           (u8 **)CONCAT22(piVar4,0x48),in_stack_0000fe82,in_stack_0000ffa6,in_stack_0000ffac,
                           in_stack_0000ffb0);
  pass1_1008_3e94((u16 *)((u32)puVar3 & 0xffff0000 | (u32)((int)puVar3 + 0xe)),(u16 *)CONCAT22(uVar5,piVar4),
                  (char *)CONCAT22(uVar7,piVar6));
  (u32)((int)param_1 + 0xe) = 0x19001db;
  ((int)param_1 + 0xa) = 0x140 - (((int)param_1 + 0xe) / 0x2 - local_4);
  ((int)param_1 + 0xc) = 0xf0 - (((int)param_1 + 0x10) / 0x2 - local_6);
  (u32)((int)param_1 + 0x1a) = 0xa006e;
  (u32)((int)param_1 + 0x1e) = 0xa012c;
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x2a)),NULL,0x28);
  return;
}
pub fn pass1_1010_2db2(param_1: *mut astruct_455)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  astruct_455 *iVar5;
  let mut uVar4: u16;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar5 = (astruct_455 *)param_1;
  param_1.field0_0x0 = 0x36da;
  iVar5.field1_0x2 = 0x1010;
  puVar1 = (u32 *)iVar5[0xa].field3_0x6;
  uVar2 = (iVar5 + 0xb)->field0_0x0;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce(*(char **)&iVar5[0xb].field2_0x4);
  pass1_1010_1d80((StructD *)param_1);
  return;
}



// WARNING: Unable to use type for symbol uVar1

pub fn pass1_1010_2e02(mut param_1: u32,mut param_2: i16) -> u32

{
  astruct_163 *iVar2;
  let mut uVar2: u16;
  let mut uVar1: u32;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x5c) != 0x0) {
    uVar1 = (u32)((int)param_1 + 0x5c);
    uVar2 = ((u32)uVar1 >> 0x10);
    iVar2 = (astruct_163 *)uVar1;
    return CONCAT22((iVar2 + param_2 * 0x4 + 0x2),(iVar2 + param_2 * 0x4));
  }
  return 0x0;
}
pub fn pass1_1010_2e30(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x5c) != 0x0) {
    uVar1 = (u32)((int)param_1 + 0x5c);
    uVar3 = ((u32)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    (iVar2 + param_4 * 0x4) = param_2;
    (iVar2 + param_4 * 0x4 + 0x2) = param_3;
  }
  return;
}
pub fn pass1_1010_2e5c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut uStack12: u32;

  uStack12 = param_3;
  if (param_3 == 0x0) {
    return;
  }
  for (; (uStack12 & 0xf) != 0x0; uStack12 >>= 0x4) {
  }
  return;
}
pub fn pass1_1010_2ee2(u32 *param_1)

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut extraout_DX: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  astruct_65 *paStack6;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(i32 *)(iVar3 + 0x52) != 0x0) {
    return;
  }
  iVar2 = 0x0;
  (iVar3 + 0x28) = 0x0;
  uVar5 = *param_1;
  ppcVar1 = (code **)((int)uVar5 + 0x20);
  (**ppcVar1)();
  if (iVar2 == 0x0) {
    paStack6 = *(astruct_65 **)(iVar3 + 0x56);
  }
  else {
    ppcVar1 = (code **)((int)uVar5 + 0x14);
    (**ppcVar1)();
    paStack6 = (astruct_65 *)CONCAT22(extraout_DX,iVar2);
    uVar5 = pass1_1010_2e02((u32)param_1,(iVar2 + 0x12));
    pass1_1010_35a4((int)(uVar5 >> 0x10),param_1,uVar5);
  }
  pass1_1010_32f4((astruct_27 *)param_1,paStack6);
  pass1_1010_1f62((astruct_27 *)param_1,0x8);
  if (*(i32 *)(iVar3 + 0x52) != 0x0) {
    return;
  }
  return;
}



// WARNING: Unable to use type for symbol uVar3
pub fn unk_destroy_win_op_1010_2fa0(param_1: *mut astruct_873)

{
  astruct_872 **ppaVar1;
  let mut uVar2: u32;
  astruct_873 *iVar3;
  let mut uVar4: u16;
  astruct_872 *iStack4;
  let mut uVar3: u32;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar3 = (astruct_873 *)param_1;
  iVar3.field39_0x28 = 0x0;
  iStack4 = NULL;
  while( true ) {
    ppaVar1 = &iVar3.field22_0x16;
    if (*ppaVar1 == iStack4 || (int)*ppaVar1 < (int)iStack4) break;
    uVar3 = (u32)(&iVar3.field_0x2a + (int)iStack4 * 0x4);
    DestroyWindow16(*(HWND16 *)((int)uVar3 + 0x18));
    iStack4 = iStack4 + 0x1;
  }
  iVar3.field22_0x16 = NULL;
  if ((iVar3.field82_0x54 | &iVar3.field_0x52) != 0x0) {
    iStack4 = NULL;
    do {
      uVar2 = (u32)&iVar3.field_0x52;
      if (*(i32 *)((int)uVar2 + (int)iStack4 * 0x4) != 0x0) {
        uVar2 = (u32)&iVar3.field_0x52;
        uVar2 = (u32)((int)uVar2 + (int)iStack4 * 0x4);
        DestroyWindow16(*(HWND16 *)((int)uVar2 + 0x18));
        uVar2 = (u32)&iVar3.field_0x52;
        (u32)((int)uVar2 + (int)iStack4 * 0x4) = 0x0;
      }
      iStack4 = iStack4 + 0x1;
    } while ((int)iStack4 < 0xa);
    fn_ptr_1000_17ce(*(char **)&iVar3.field_0x52);
    (u32)&iVar3.field_0x52 = 0x0;
  }
  return;
}
