
StructD * pass1_1028_816e(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_81aa(uchar param_1,param_2: *mut astruct_97)

{
  struct_op_1028_d1dc(param_2,0x1b57);
  param_2->offset_0x0 = 0x836e;
  ((int)param_2 + 0x2) = 0x1028;
  unk_str_op_1000_3d3e((char *)((u32)param_2 & 0xffff0000 | (u32)((int)param_2 + 0x8)),s_SCFactory_1050_5002);
  return param_2;
}



u16 pass1_1028_81e0(mut param_1: u16 )

{
  let mut iVar1: i16;
  code **ppcVar2;
  astruct_92 *paVar3;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  let mut unaff_CS: u16;
  u32 *puStack24;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x700);
switchD_1028_8225_caseD_0:
  do {
    while( true ) {
      uVar4 = param_1;
      paVar3 = &local_14;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar3));
      puStack24 = (u32 *)CONCAT22(uVar4,paVar3);
      param_1 = uVar4 | paVar3;
      if (param_1 == 0x0) {
        return 0x1;
      }
      iVar1 = &paVar3->field5_0xc;
      if (iVar1 < 0x35) goto code_r0x10288222;
      if (0x61 < iVar1) break;
      if ((iVar1 < 0x5d) && ((iVar1 != 0x37 && (iVar1 != 0x47)))) goto switchD_1028_8225_caseD_1;
    }
  } while ((iVar1 == 0x6a) ||
          ((0x8 < iVar1 + -0x6a &&
           ((iVar1 == 0x75 || iVar1 + -0x74 < 0x1 || ((0x0 < iVar1 + -0x76 && (iVar1 + -0x78 < 0x2))))))));
  goto switchD_1028_8225_caseD_1;
code_r0x10288222:
  unaff_CS = 0x1028;
  switch(iVar1) {
  case 0x1:
  case 0x2:
  case 0x3:
  case 0x4:
  case 0x6:
  case 0x7:
  case 0x8:
  case 0xa:
  case 0xb:
  case 0xc:
  case 0xd:
  case 0xe:
  case 0xf:
  case 0x11:
switchD_1028_8225_caseD_1:
    if ((paVar3 + 0x1) == 0x5) {
      ppcVar2 = (code **)((int)*puStack24 + 0x30);
      (**ppcVar2)(unaff_CS);
      param_1 = extraout_DX;
    }
  }
  goto switchD_1028_8225_caseD_0;
}
pub fn pass1_1028_82b4(mut param_1: u16 ,u8 *param_2,mut param_3: u32)

{
  u32 *puVar1;
  u32 *puVar2;
  u32 *puVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar6;
  u32 *puVar7;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x108,paVar6);
  uVar5 = paVar6;
  puStack10 = (u16 *)CONCAT22(uVar5,param_1);
  if ((uVar5 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (param_3 >> 0x10);
    (u32)(param_1 + 0x4) = (u32)((int)param_3 + 0x4);
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
    *puStack10 = 0x836e;
    (param_1 + 0x2) = 0x1028;
  }
  return;
}



StructD * pass1_1028_8342(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_837e(param_1: *mut astruct_97)

{
  struct_op_1028_d1dc(param_1,0xf9f);
  param_1->offset_0x0 = 0x84ba;
  ((int)param_1 + 0x2) = 0x1028;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x8)),s_SCFillResources_1050_500c);
  return param_1;
}



u16 pass1_1028_83b4(mut param_1: u16 )

{
  astruct_92 *paVar1;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x400);
  while( true ) {
    paVar1 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar1));
    if ((param_1 | paVar1) == 0x0) break;
    ((int)&paVar1[0x1c].field5_0xc + 0x2) = 0x1;
    param_1 = param_1 | paVar1;
  }
  return 0x1;
}
pub fn pass1_1028_8400(mut param_1: u16 ,u8 *param_2,mut param_3: u32)

{
  u32 *puVar1;
  u32 *puVar2;
  u32 *puVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar6;
  u32 *puVar7;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x108,paVar6);
  uVar5 = paVar6;
  puStack10 = (u16 *)CONCAT22(uVar5,param_1);
  if ((uVar5 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (param_3 >> 0x10);
    (u32)(param_1 + 0x4) = (u32)((int)param_3 + 0x4);
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
    *puStack10 = 0x84ba;
    (param_1 + 0x2) = 0x1028;
  }
  return;
}



StructD * pass1_1028_848e(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1028_84ca(param_1: *mut astruct_97,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  let mut offset: u16;
  astruct_97 *iVar2;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x3e7);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_97 *)param_1;
  iVar2->field259_0x108 = param_5;
  &iVar2->field_0x10a = param_4;
  iVar2->field262_0x10c = param_3;
  (u32)&iVar2->field263_0x10e = param_2;
  param_1->offset_0x0 = 0x8688;
    // just 0x1028
  iVar2->segment_0x2 = 0x1028;
  if (iVar2->field259_0x108 == 0x1) {
    // just 0x501c
    offset = s_max_1050_501c;
  }
  else {
    // just 0x5020
    offset = s_min_1050_5020;
  }
  sys_1000_3f9c((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2->string_0x8)),
                s_SCForceMorale__s_for_colony__08l_1050_5024,offset);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1028_853e(param_1: *mut astruct_685,mut param_2: u32)

{
  let mut uVar1: u16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_2 >> 0x10);
  iVar2 = (int)param_2;
  if ((iVar2 + 0x108) == 0x0) {
    return 0x0;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(iVar2 + 0x10e));
  if ((iVar2 + 0x108) == 0x1) {
    uVar1 = 0x3e8;
  }
  else {
    uVar1 = 0x0;
  }
  pass1_1038_4d0e((astruct_685 *)CONCAT22((int)param_1,(int)((u32)param_1 >> 0x10)),uVar1);
  return 0x1;
}
pub fn pass1_1028_858c(param_1: *mut astruct_318,u8 *param_2,param_3: *mut astruct_319)

{
  u32 *puVar1;
  u32 *puVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  astruct_319 *iVar5;
  u32 *puVar6;
  u32 *puVar7;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x112,paVar5);
  uVar4 = paVar5;
  puStack10 = (u16 *)CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    param_1->field2_0x2 = 0x1008;
    uVar8 = ((u32)param_3 >> 0x10);
    iVar5 = (astruct_319 *)param_3;
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
    param_1->field258_0x10a = iVar5->field259_0x10a;
    param_1->field259_0x10c = iVar5->field260_0x10c;
    param_1->field260_0x10e = iVar5->field261_0x10e;
    *puStack10 = 0x8688;
    param_1->field2_0x2 = 0x1028;
  }
  return;
}



StructD * pass1_1028_865c(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_8698(param_1: *mut astruct_97,mut param_2: u32,mut param_3: u32)

{
  pass1_1028_6af2(param_1,param_2,param_3);
  param_1->offset_0x0 = 0x87e0;
  ((int)param_1 + 0x2) = 0x1028;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_86c2(StructD *param_1,mut param_2: u32)

{
  astruct_67 *paVar1;
  let mut in_stack_0000fe94: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc2: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut iVar8: i16;

  uVar7 = 0x0;
  iVar8 = 0x1d;
  uVar5 = 0x1;
  uVar6 = 0x0;
  uVar3 = 0x0;
  iVar4 = 0x0;
  uVar2 = 0x0;
  paVar1 = (astruct_67 *)
           mixed_1010_20ba((astruct_57 *)param_1,_u16_1050_0ed0,(u8 **)0x37,in_stack_0000fe94,in_stack_0000ffb8,
                           in_stack_0000ffbe,in_stack_0000ffc2);
  post_win_msg_1008_a0e4(paVar1,CONCAT22(uVar3,uVar2),iVar4,uVar5,CONCAT22(uVar7,uVar6),iVar8);
  pass1_1028_6b2c(param_2);
  return;
}
pub fn pass1_1028_86f4(param_1: *mut astruct_320,u8 *param_2,param_3: *mut astruct_321)

{
  u32 *puVar1;
  u32 *puVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  astruct_321 *iVar5;
  u32 *puVar6;
  u32 *puVar7;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x110,paVar5);
  uVar4 = paVar5;
  puStack10 = (u16 *)CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    param_1->field2_0x2 = 0x1008;
    uVar8 = ((u32)param_3 >> 0x10);
    iVar5 = (astruct_321 *)param_3;
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
    param_1->field258_0x10c = iVar5->field259_0x10c;
    *puStack10 = 0x6e50;
    param_1->field2_0x2 = 0x1028;
    *puStack10 = 0x87e0;
    param_1->field2_0x2 = 0x1028;
  }
  return;
}



StructD * pass1_1028_87b4(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn struct_op_1028_87f0(param_1: *mut astruct_97,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,u32 *param_5,mut param_6: u16 ,
                        mut param_7: u32,mut param_8: u32)

{
  astruct_97 *iVar1;
  astruct_97 *puVar1;

  struct_op_1028_d1dc(param_1,0x3e8);
  puVar1 = (astruct_97 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  (u32)&iVar1->field259_0x108 = param_8;
  (u32)&iVar1->field262_0x10c = param_7;
  iVar1->field264_0x110 = 0x0;
  iVar1->field265_0x114 = *param_5;
  iVar1->field266_0x118 = (param_5 + 0x1);
  iVar1->field267_0x11a = param_4;
  iVar1->field268_0x11c = param_3;
  iVar1->field269_0x11e = param_2;
  iVar1->field271_0x122 = 0x0;
  iVar1->field270_0x120 = 0x0;
  param_1->offset_0x0 = 0x8d8e;
  iVar1->segment_0x2 = 0x1028;
  sys_1000_3f9c((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),
                s_SCInternalPutBldg_site_0x_08lx__b_1050_5046,param_8);
  return;
}
pub fn struct_op_1028_8888(param_1: *mut astruct_97,mut param_2: u16 ,mut param_3: u16 ,u32 *param_4,mut param_5: u16 ,mut param_6: u32,
                        mut param_7: u32,mut param_8: u32)

{
  astruct_97 *iVar1;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x3e8);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  (u32)&iVar1->field259_0x108 = param_8;
  (u32)&iVar1->field262_0x10c = param_7;
  iVar1->field264_0x110 = param_6;
  iVar1->field265_0x114 = *param_4;
  iVar1->field266_0x118 = (param_4 + 0x1);
  iVar1->field267_0x11a = param_3;
  iVar1->field268_0x11c = 0x0;
  iVar1->field269_0x11e = param_2;
  iVar1->field271_0x122 = 0x0;
  iVar1->field270_0x120 = 0x0;
  param_1->offset_0x0 = 0x8d8e;
    // just 0x1028
  iVar1->segment_0x2 = 0x1028;
  sys_1000_3f9c((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),
                s_SCInternalPutBldg2_site_0x_08lx__1050_506f,param_8);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_8920(mut param_1: u16 ,mut param_2: u32)

{
  u32 **ppuVar1;
  code **ppcVar2;
  u32 **ppuVar3;
  let mut iVar4: i16;
  let mut BVar5: bool;
  let mut uVar6: u32;
  u8 *puVar7;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar10;
  let mut iVar11: i16;
  astruct_684 *iVar12;
  let mut unaff_SI: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut in_stack_0000fd4e: u16;
  let mut in_stack_0000fe72: u16;
  let mut in_stack_0000fe78: u16;
  let mut in_stack_0000fe7c: u16;
  u8 uVar14;
  let mut uVar15: u16;
  u8 **local_156 [0x43];
  let mut local_4a: u32;
  let mut iStack70: i16;
  let mut uStack68: u32;
  let mut uStack56: u32;
  u32 *puStack52;
  let mut uStack48: u16;
  u32 *puStack46;
  let mut uStack42: u32;
  u8 local_26 [0x4];
  let mut uStack34: u32;
  let mut uStack30: u32;
  let mut uStack26: u32;
  let mut uStack22: u32;
  u32 *puStack18;
  let mut uStack14: u16;
  u8 local_c [0x2];
  u8 local_a [0x2];
  u8 local_8 [0x2];
  let mut uStack6: u32;

  uVar12 = (param_2 >> 0x10);
  iVar11 = (int)param_2;
  ppuVar1 = (u32 **)(iVar11 + 0x114);
  ppuVar3 = ppuVar1;
  pass1_1030_64ce(ppuVar1,param_1,_PTR_LOOP_1050_5740,(u16 *)(param_2 & 0xffff0000 | ZEXT24(ppuVar1)),
                  *(i32 *)(iVar11 + 0x108),(u32 *)CONCAT22(0x1050,local_26));
  uStack6 = *ppuVar3;
  pass1_1008_3eb4((astruct_615 *)(param_2 & 0xffff0000 | ZEXT24(ppuVar1)),(u16 *)CONCAT22(0x1050,local_c),
                  (u16 *)CONCAT13(0x10,CONCAT12(0x50,local_a)),(u16 *)CONCAT22(0x1050,local_8));
  paVar10 = (astruct_57 *)CONCAT22(in_register_0000000a,uStack6);
  puStack46 = uStack6;
  uStack56 = uStack6;
  uStack56._3_1_ = (char)((u32)uStack6 >> 0x18);
  uStack14 = (uStack56._3_1_ != '\0');
  if (uStack14 == 0x0) {
    uVar6 = (u32)(iVar11 + 0x114U);
    pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x500);
    puStack18 = (u32 *)(uVar6 & 0xffff | (long)paVar10 << 0x10);
    uVar13 = 0x1030;
    pass1_1030_61fe(uVar6,paVar10,_PTR_LOOP_1050_5740,uVar6 & 0xffff | (long)paVar10 << 0x10,
                    param_2 & 0xff000000 | (u32)CONCAT12((char)(param_2 >> 0x10),iVar11 + 0x114U),
                    *(i32 *)(iVar11 + 0x108));
    uStack56 = NULL;
    if (((iVar11 + 0x11a) == 0xa) || ((iVar11 + 0x11a) == 0x37)) {
      if ((iVar11 + 0x11a) == 0x37) {
        uStack56 = (u32*)(iVar11 + 0x10c);
      }
      iVar4 = iVar11 + 0x114;
      pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x400);
      (iVar11 + 0x10c) = iVar4;
      (iVar11 + 0x10e) = (int)paVar10;
      puStack46 = mixed_1010_20ba(paVar10,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fd4e,
                                  in_stack_0000fe72,in_stack_0000fe78,in_stack_0000fe7c);
      paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)puStack46 >> 0x10);
      uVar6 = (u32)puStack46 & 0xffff;
      puVar7 = ((u32)puStack46 >> 0x10);
      uVar13 = 0x1018;
      pass1_1018_0196(uVar6,puVar7,uVar6 | ZEXT24(puVar7) << 0x10,(u32)(iVar11 + 0x10c),
                      (u32)(iVar11 + 0x108));
      iVar4 = (int)uVar6;
      if (*(i32 *)(iVar11 + 0x110) != 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(iVar11 + 0x10c));
        uStack42 = CONCAT22((int)paVar10,iVar4);
        uVar6 = (u32)(iVar11 + 0x110);
        (u32)(iVar4 + 0x200) = uVar6;
        uStack68 = uVar6;
      }
    }
    uStack6._0_2_ = uVar6;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(iVar11 + 0x10c));
    uVar8 = paVar10;
    puStack52 = (u32 *)CONCAT22(uVar8,uStack6);
    paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)(uVar8 | uStack6));
    if ((uVar8 | uStack6) != 0x0) {
      ppcVar2 = (code **)((int)*puStack52 + 0x8);
      (**ppcVar2)(uVar13,uStack6,uVar8,0x0,(char)puStack18,(int)((u32)puStack18 >> 0x10),0x0);
    }
  }
  else {
    puStack18 = uStack6;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)puStack18);
  uStack22 = (astruct_358 *)CONCAT22(paVar10,uStack6);
  pass1_1030_73ee(paVar10,
                  (astruct_294 *)CONCAT13((char)((u32)paVar10 >> 0x8),CONCAT12((char)paVar10,uStack6)),
                  (u32)(iVar11 + 0x10c));
  BVar5 = pass1_1008_c6ae(_u16_1050_06e0,(iVar11 + 0x11a),0x31);
  if ((BVar5 == 0x0) && ((iVar11 + 0x11c) == 0x0)) {
    paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000);
    local_4a = (u32)((int)uStack22 + 0xc);
    iStack70 = ((int)uStack22 + 0x10);
    uStack68 = uStack68 & 0xffff0000 | ZEXT24(&local_4a);
    if (iStack70 < 0x1) {
      uStack48 = 0x5;
    }
    else {
      uStack48 = 0x6;
    }
    ((int)uStack22 + 0x14) = uStack48;
  }
  uStack26 = (u32)((int)uStack22 + 0x16);
  uVar8 = ((int)uStack22 + 0x18);
  paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)uVar8);
  if ((uVar8 | uStack26) != 0x0) {
    struct_1030_e4fa((astruct_97 *)CONCAT13(0x10,CONCAT12(0x50,local_156)),uStack26 & 0xffff | (u32)uVar8 << 0x10);
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_156));
    local_156[0] = &PTR_pass1_1008_377e_1008_389a;
  }
  uStack30 = pass1_1028_e2e0(paVar10,_PTR_LOOP_1050_65e2,0x7);
  uVar8 = uStack30;
  uVar9 = (uStack30 >> 0x10) | uVar8;
  if (uVar9 == 0x0) {
    return;
  }
  pass1_1030_7e5a(uVar9,uStack22,uStack30);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack30);
  uStack34 = (u32 *)CONCAT22(uVar9,uVar8);
  uVar13 = SUB42(puStack18,0x0);
  uVar15 = ((u32)puStack18 >> 0x10);
  uVar14 = (u8)uVar9;
  iVar12 = (astruct_684 *)*uStack34;
  ppcVar2 = (code **)&iVar12->field4_0x4;
  (**ppcVar2)();
  ppcVar2 = (code **)&iVar12->field28_0x20;
  (**ppcVar2)(0x1030,uStack34,uVar8,uVar14,uVar13,uVar15);
  ppcVar2 = (code **)&iVar12->field22_0x18;
  (**ppcVar2)(0x1030,(int)uStack34,(char)((u32)uStack34 >> 0x10),0x1);
  if ((iVar11 + 0x11a) == 0x37) {
    (u32)((int)uStack34 + 0x20) = (u32)(iVar11 + 0x10c);
  }
  (u32)(iVar11 + 0x120) = uStack34;
  return;
}
pub fn pass1_1028_8c46(param_1: *mut astruct_322,u8 *param_2,param_3: *mut astruct_323)

{
  u32 *puVar1;
  u32 *puVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  astruct_323 *iVar5;
  u32 *puVar6;
  u32 *puVar7;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x124,paVar5);
  uVar4 = paVar5;
  puStack10 = (u16 *)CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    param_1->field2_0x2 = 0x1008;
    uVar8 = ((u32)param_3 >> 0x10);
    iVar5 = (astruct_323 *)param_3;
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
    param_1->field258_0x10c = iVar5->field259_0x10c;
    param_1->field259_0x110 = iVar5->field260_0x110;
    param_1->field260_0x114 = iVar5->field261_0x114;
    param_1->field261_0x118 = iVar5->field262_0x118;
    param_1->field262_0x11a = iVar5->field263_0x11a;
    param_1->field263_0x11c = iVar5->field264_0x11c;
    param_1->field264_0x11e = iVar5->field265_0x11e;
    param_1->field265_0x120 = iVar5->field266_0x120;
    *puStack10 = 0x8d8e;
    param_1->field2_0x2 = 0x1028;
  }
  return;
}



StructD * pass1_1028_8d62(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1028_8d9e(param_1: *mut astruct_97,mut param_2: u32,mut param_3: u32,mut param_4: u32)

{
  astruct_97 *iVar1;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x3e8);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  (u32)&iVar1->field259_0x108 = param_4;
  (u32)&iVar1->field262_0x10c = param_3;
  iVar1->field264_0x110 = param_2;
  iVar1->field265_0x114 = 0x0;
  param_1->offset_0x0 = 0x8fb0;
    // just 0x1028
  iVar1->segment_0x2 = 0x1028;
  return;
}
pub fn pass1_1028_8dec(param_1: *mut u16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x8fb0;
  (iVar1 + 0x2) = 0x1028;
  fn_ptr_1000_17ce(*(char **)(iVar1 + 0x114));
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_8e1e(mut param_1: i16,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_3 >> 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)((int)param_3 + 0x10c));
  pass1_1030_355c((u32)(param_1 + 0x1f6),(u32)((int)param_3 + 0x114));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_8e5c(mut param_1: u32,mut param_2: i16,u8 *param_3)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(iVar2 + 0x108));
  uVar1 = (u32)(param_2 + 0x1f6);
  pass1_1030_35a4(param_3,uVar1,*(i32 *)(iVar2 + 0x110));
  (iVar2 + 0x114) = (int)uVar1;
  *(u8 **)(iVar2 + 0x116) = param_3;
  return;
}
pub fn pass1_1028_8ea6(param_1: *mut astruct_324,u8 *param_2,param_3: *mut astruct_325)

{
  u32 *puVar1;
  u32 *puVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  astruct_325 *iVar5;
  u32 *puVar6;
  u32 *puVar7;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x118,paVar5);
  uVar4 = paVar5;
  puStack10 = (u16 *)CONCAT22(uVar4,param_1);
  iVar5 = (astruct_325 *)param_3;
  uVar8 = ((u32)param_3 >> 0x10);
  if ((uVar4 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    param_1->field2_0x2 = 0x1008;
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
    param_1->field258_0x10c = iVar5->field259_0x10c;
    param_1->field259_0x110 = iVar5->field260_0x110;
    param_1->field260_0x114 = iVar5->field261_0x114;
    *puStack10 = 0x8fb0;
    param_1->field2_0x2 = 0x1028;
  }
  iVar5->field261_0x114 = 0x0;
  return;
}



StructD * pass1_1028_8f8a(StructD *param_1,param_2: u8)

{
  pass1_1028_8dec(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_8fc0(param_1: *mut astruct_97,mut param_2: u32,mut param_3: u32)

{
  pass1_1028_6af2(param_1,param_2,param_3);
  param_1->offset_0x0 = 0x90d6;
  ((int)param_1 + 0x2) = 0x1028;
  return param_1;
}
pub fn pass1_1028_8fea(param_1: *mut astruct_326,u8 *param_2,param_3: *mut astruct_327)

{
  u32 *puVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  astruct_327 *iVar5;
  u32 *puVar6;
  u32 *puVar7;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;
  u32 *puVar1;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x110,paVar5);
  uVar4 = paVar5;
  puStack10 = (u16 *)CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    param_1->field2_0x2 = 0x1008;
    uVar8 = ((u32)param_3 >> 0x10);
    iVar5 = (astruct_327 *)param_3;
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
    param_1->field258_0x10c = iVar5->field259_0x10c;
    *puStack10 = 0x6e50;
    param_1->field2_0x2 = 0x1028;
    *puStack10 = 0x90d6;
    param_1->field2_0x2 = 0x1028;
  }
  return;
}
