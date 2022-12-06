
StructD * pass1_1030_e010(StructD *param_1,param_2: u8)

{
  let mut in_AX: u16;

  pass1_1030_dcf4(in_AX,(astruct_15 *)param_1);
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_97 * pass1_1030_e09e(param_1: *mut astruct_97)

{
  struct_op_1028_d1dc(param_1,0x2af7);
  param_1->offset_0x0 = 0xe2ae;
  (param_1 + 0x2) = 0x1030;
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x8)),s_SCAiInput_1050_5972);
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_e0d4(param_1: *mut u8)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  astruct_425 *paVar3;
  astruct_425 *paVar4;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut iVar7: i16;
  let mut unaff_SI: u16;
  let mut uVar8: u16;
  let mut puVar9: *mut u32;
  let mut in_stack_0000fe7c: u16;
  let mut in_stack_0000ffa0: u16;
  let mut in_stack_0000ffa6: u16;
  let mut in_stack_0000ffaa: u16;
  let mut uStack42: u32;
  u8 local_1c [0x8];
  let mut uStack20: u32;
  let mut uStack16: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut iStack6: i16;
  let mut uStack4: u16;
  let mut uVar6: u32;

  puVar9 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(unaff_SI,0x40),in_stack_0000fe7c,in_stack_0000ffa0,in_stack_0000ffa6,
                           in_stack_0000ffaa);
  uStack4 = (puVar9 >> 0x10);
  iStack6 = puVar9;
  uStack10 = pass1_1008_b820(iStack6,uStack4,puVar9);
  uVar2 = uStack10;
  uVar5 = (uStack10 >> 0x10) | uVar2;
  uVar6 = uVar5;
  if (uVar5 != 0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x8000001);
    uStack14 = CONCAT22(uVar6,uVar2);
    uStack16 = ((uVar2 + 0x154) != 0);
    pass1_1008_5784(CONCAT22(0x1050,local_1c),uStack10);
    while( true ) {
      uVar2 = uVar6;
      paVar3 = (astruct_425 *)local_1c;
      pass1_1008_5b12(CONCAT22(0x1050,paVar3));
      uStack20 = CONCAT22(uVar2,paVar3);
      uVar6 = (uVar2 | paVar3);
      if ((uVar2 | paVar3) == 0) break;
      if (&paVar3->field_0x8 != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,&paVar3->field_0xa);
        paVar4 = paVar3;
        pass1_1038_354a((astruct_424 *)CONCAT22(uVar6,paVar3),paVar3,uVar6);
        if (paVar4 != NULL) {
          uVar8 = (uStack20 >> 0x10);
          if (uStack16 == 0) {
            iVar7 = (uStack20 + 0xe) * 0xc;
            uStack42 = (iVar7 + 0x58c4);
            uVar2 = (iVar7 + 0x58c8);
          }
          else {
            iVar7 = (uStack20 + 0xe) * 0xc;
            uStack42 = (iVar7 + 0x58be);
            uVar2 = (iVar7 + 0x58c2);
          }
          uVar5 = uVar2;
          pass1_1038_35a8(uVar2,uVar6,((uStack20 + 0x10) * 0x2 + uStack42),paVar3)
          ;
          if (uVar5 != 0) {
            uVar8 = (uStack20 >> 0x10);
            iVar7 = uStack20;
            piVar1 = (iVar7 + 0x10);
            *piVar1 = *piVar1 + 1;
            if (uVar2 <= (iVar7 + 0x10)) {
              (iVar7 + 0x10) = 0;
            }
          }
        }
      }
    }
  }
  return;
}
pub fn pass1_1030_e1f4(mut param_1: u16 ,param_2: *mut u8,mut param_3: u32)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut puVar3: *mut u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut puVar7: *mut u32;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x108,paVar6);
  uVar5 = paVar6;
  puStack10 = CONCAT22(uVar5,param_1);
  if ((uVar5 | param_1) != 0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (param_3 >> 0x10);
    (param_1 + 0x4) = (param_3 + 0x4);
    puVar3 = (param_3 + 0x8);
    puVar7 = (param_1 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0; iVar4 += -1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    *puStack10 = 0xe2ae;
    (param_1 + 0x2) = 0x1030;
  }
  return;
}



StructD * pass1_1030_e282(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_1030_e2be(param_1: *mut astruct_97,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  astruct_97 *iVar1;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x2af7);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  &iVar1->field259_0x108 = param_4;
  &iVar1->field262_0x10c = param_3;
  &iVar1->field264_0x110 = param_2;
  param_1->offset_0x0 = 0xe4ea;
  iVar1->segment_0x2 = 0x1030;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1030_e300(param_1: *mut u8,mut param_2: u32)

{
  let mut in_register_0000000a: u16;
  astruct_27 *paVar1;
  let mut in_stack_0000fea2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000fff8: u32;

  paVar1 = (astruct_27 *)
           mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22((in_stack_0000fff8 >> 0x10),0x2b),in_stack_0000fea2,
                           in_stack_0000ffc6,in_stack_0000ffcc,in_stack_0000ffd0);
  pass1_1010_089e(paVar1,(param_2 + 0x110),0x2);
  return 0x1;
}



u16 pass1_1030_e328(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_3 >> 0x10);
  if ((param_3 + 0x110) == 0) {
    pass1_1030_e4ba();
  }
  else {
    pass1_1030_e410(param_1,param_2,param_3 & 0xffff | uVar1 << 0x10);
  }
  return 0x1;
}
pub fn pass1_1030_e34e(mut param_1: u16 ,param_2: *mut u8,param_3: *mut astruct_403)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  astruct_403 *iVar5;
  let mut puVar6: *mut u32;
  let mut puVar7: *mut u32;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x112,paVar5);
  uVar4 = paVar5;
  puStack10 = CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (param_3 >> 0x10);
    iVar5 = (astruct_403 *)param_3;
    (param_1 + 0x4) = iVar5->field4_0x4;
    puVar6 = &iVar5->field5_0x8;
    puVar7 = (param_1 + 0x8);
    for (iVar3 = 0x40; iVar3 != 0; iVar3 += -1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 1;
      puVar1 = puVar6;
      puVar6 = puVar6 + 1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    (param_1 + 0x108) = iVar5->field258_0x108;
    (param_1 + 0x10c) = iVar5->field259_0x10c;
    (param_1 + 0x110) = iVar5->field260_0x110;
    *puStack10 = 0xe4ea;
    (param_1 + 0x2) = 0x1030;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_e410(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut puVar1: *mut u8;
  let mut uVar2: u16;
  let mut puVar3: *mut u16;
  let mut in_stack_0000fe9c: u16;
  u8 local_10 [0x6];
  u8 local_a [0x4];
  let mut uStack6: u16;
  let mut uStack4: u16;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(param_3 + 0x10c));
  puVar1 = (param_2 | param_1);
  if (puVar1 != NULL) {
    uStack6 = param_1;
    uStack4 = param_2;
    pass1_1038_4fd8(param_1,CONCAT22(param_2,param_1),0x21);
    if (param_1 == 0) {
      pass1_1020_a43e(puVar1,CONCAT22(0x1050,local_a));
      puVar3 = pass1_1008_3e54(CONCAT22(0x1050,local_10),0x0,0x2,0xfffd);
      uVar2 = (puVar3 >> 0x10);
      pass1_1020_a49a(uVar2,in_stack_0000fe9c,CONCAT22(0x1050,local_a),CONCAT22(0x1050,local_10),0x7a);
      pass1_1008_3e76(CONCAT22(0x1050,local_10),0x0,0x3,0xfffe);
      pass1_1020_a49a(uVar2,in_stack_0000fe9c,CONCAT22(0x1050,local_a),CONCAT22(0x1050,local_10),0x7a);
      pass1_1008_3e76(CONCAT22(0x1050,local_10),0x0,0x3,0xfffd);
      pass1_1020_a49a(uVar2,in_stack_0000fe9c,CONCAT22(0x1050,local_a),CONCAT22(0x1050,local_10),0x21);
    }
  }
  return;
}
pub fn pass1_1030_e4ba()

{
  return;
}



StructD * pass1_1030_e4be(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_1030_e4fa(param_1: *mut astruct_97,mut param_2: u32)

{
  astruct_97 *iVar1;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x3e80);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  &iVar1->field259_0x108 = param_2;
  param_1->offset_0x0 = 0xe62e;
  iVar1->segment_0x2 = 0x1030;
  sys_1000_3f9c((param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),s_SCKillBldg__0x_08lx_1050_597c,
                &iVar1->field259_0x108);
  return;
}



u16 pass1_1030_e540()

{
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1030_e546(mut param_1: u32)

{
  let mut uVar1: u32;

  uVar1 = (param_1 + 0x108);
  pass1_1028_e332(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
  return 0x1;
}
pub fn pass1_1030_e564(mut param_1: u16 ,param_2: *mut u8,param_3: *mut astruct_405)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  astruct_405 *iVar5;
  let mut puVar6: *mut u32;
  let mut puVar7: *mut u32;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x10c,paVar5);
  uVar4 = paVar5;
  puStack10 = CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (param_3 >> 0x10);
    iVar5 = (astruct_405 *)param_3;
    (param_1 + 0x4) = iVar5->field4_0x4;
    puVar6 = &iVar5->field5_0x8;
    puVar7 = (param_1 + 0x8);
    for (iVar3 = 0x40; iVar3 != 0; iVar3 += -1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 1;
      puVar1 = puVar6;
      puVar6 = puVar6 + 1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    (param_1 + 0x108) = iVar5->field258_0x108;
    *puStack10 = 0xe62e;
    (param_1 + 0x2) = 0x1030;
  }
  return;
}



StructD * pass1_1030_e602(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_97 * pass1_1030_e63e(param_1: *mut astruct_97,mut param_2: u16 )

{
  astruct_97 *iVar1;
  astruct_97 *uVar1;

  iVar1 = (astruct_97 *)param_1;
  uVar1 = (astruct_97 *)(param_1 >> 0x10);
  struct_op_1028_d1dc(param_1,0xf9f);
  iVar1->field259_0x108 = param_2;
  param_1->offset_0x0 = 0xe78a;
  iVar1->segment_0x2 = 0x1030;
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),s_SCKillColony_1050_5990);
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1030_e67c(StructD *param_1,mut param_2: u32,mut param_3: u16 )

{
  let mut uVar1: u16;
  astruct_67 *paVar2;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut uStack8: u16;

  _param_3 = (u8 **)CONCAT22(uStack8,0x37);
  paVar2 = (astruct_67 *)
           mixed_1010_20ba((astruct_57 *)param_1,_u16_1050_0ed0,_param_3,in_stack_0000fea0,in_stack_0000ffc4,
                           in_stack_0000ffca,in_stack_0000ffce);
  uVar1 = pass1_1008_aaa8(paVar2,(paVar2 >> 0x10),(param_2 + 0x108));
  if (uVar1 != 0) {
    post_win_msg_1008_a0e4(paVar2,0x0,0x0,0x1,0x0,uVar1);
  }
  return 0x1;
}
pub fn pass1_1030_e6c2(mut param_1: u16 ,param_2: *mut u8,param_3: *mut astruct_406)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  astruct_406 *iVar5;
  let mut puVar6: *mut u32;
  let mut puVar7: *mut u32;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x10a,paVar5);
  uVar4 = paVar5;
  puStack10 = CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (param_3 >> 0x10);
    iVar5 = (astruct_406 *)param_3;
    (param_1 + 0x4) = iVar5->field4_0x4;
    puVar6 = &iVar5->field5_0x8;
    puVar7 = (param_1 + 0x8);
    for (iVar3 = 0x40; iVar3 != 0; iVar3 += -1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 1;
      puVar1 = puVar6;
      puVar6 = puVar6 + 1;
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



StructD * pass1_1030_e75e(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_97 * pass1_1030_e79a(param_1: *mut astruct_97)

{
  struct_op_1028_d1dc(param_1,0xf9f);
  param_1->offset_0x0 = 0xe890;
  (param_1 + 0x2) = 0x1030;
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | (param_1 + 0x8)),s_SCKillRebelColony_1050_599e);
  return param_1;
}



u16 pass1_1030_e7d0()

{
  return 0x1;
}
pub fn pass1_1030_e7d6(mut param_1: u16 ,param_2: *mut u8,mut param_3: u32)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut puVar3: *mut u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut puVar7: *mut u32;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x108,paVar6);
  uVar5 = paVar6;
  puStack10 = CONCAT22(uVar5,param_1);
  if ((uVar5 | param_1) != 0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (param_3 >> 0x10);
    (param_1 + 0x4) = (param_3 + 0x4);
    puVar3 = (param_3 + 0x8);
    puVar7 = (param_1 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0; iVar4 += -1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    *puStack10 = 0xe890;
    (param_1 + 0x2) = 0x1030;
  }
  return;
}



StructD * pass1_1030_e864(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn pass1_1030_e8a0(param_1: *mut astruct_97,mut param_2: u32,mut param_3: u16 ,mut param_4: u32)

{
  astruct_97 *iVar1;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x2710);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  &iVar1->field259_0x108 = param_2;
  &iVar1->field262_0x10c = param_4;
  &iVar1->field264_0x110 = param_3;
  param_1->offset_0x0 = 0xeb40;
  iVar1->segment_0x2 = 0x1030;
  sys_1000_3f9c((param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),s_SCMoveBas_to_0x_08lx_1050_59b0,
                &iVar1->field262_0x10c);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1030_e8f8(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut pcStack20: *mut c_char;
  let mut uStack6: u32;

  uVar4 = (param_3 >> 0x10);
  iVar3 = param_3;
  if (*(i32 *)(iVar3 + 0x108) != 0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(iVar3 + 0x10c));
    uStack6 = CONCAT22(param_2,param_1);
    uVar5 = struct_op_1030_73a8((astruct_419 *)CONCAT22(param_2,param_1),param_1,param_2);
    if ((uVar5 + 0xc) == (iVar3 + 0x110)) {
      pass1_1030_ea50(param_3,uStack6);
    }
    uVar1 = (iVar3 + 0x108);
    uVar2 = (iVar3 + 0x10a);
    pcStack20 = CONCAT22(uVar2,uVar1);
    if ((uVar2 | uVar1) != 0) {
      fn_ptr_1020_ba7e(CONCAT22(uVar2,uVar1));
      fn_ptr_1000_17ce(pcStack20);
    }
    (iVar3 + 0x108) = 0;
  }
  return 0x1;
}
pub fn pass1_1030_e98e(mut param_1: u16 ,param_2: *mut u8,param_3: *mut astruct_407)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  astruct_407 *iVar5;
  let mut puVar6: *mut u32;
  let mut puVar7: *mut u32;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x112,paVar5);
  uVar4 = paVar5;
  puStack10 = CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (param_3 >> 0x10);
    iVar5 = (astruct_407 *)param_3;
    (param_1 + 0x4) = iVar5->field4_0x4;
    puVar6 = &iVar5->field5_0x8;
    puVar7 = (param_1 + 0x8);
    for (iVar3 = 0x40; iVar3 != 0; iVar3 += -1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 1;
      puVar1 = puVar6;
      puVar6 = puVar6 + 1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    (param_1 + 0x108) = iVar5->field258_0x108;
    (param_1 + 0x10c) = iVar5->field259_0x10c;
    (param_1 + 0x110) = iVar5->field260_0x110;
    *puStack10 = 0xeb40;
    (param_1 + 0x2) = 0x1030;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_ea50(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut BVar2: bool;
  let mut in_EDX: *mut Struct57;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut local_12: u32;
  let mut local_e: u16;
  let mut iStack12: i16;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut uStack6: u32;

  uStack6 = 0x1869f;
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  BVar2 = pass1_1008_c6ae(_u16_1050_06e0,(iVar5 + 0x110),0x3);
  if (BVar2 != 0) {
    uVar7 = struct_op_1030_73a8((astruct_419 *)param_2,BVar2,in_EDX);
    uVar3 = in_EDX & 0xffff0000;
    local_e = uVar7;
    iStack12 = (uVar7 >> 0x10);
    uStack6 = pass1_1028_45e2(local_e,iStack12,uVar7);
    in_EDX = (astruct_57 *)(uVar3 & 0xffff0000);
  }
  uVar1 = (iVar5 + 0x108);
  uStack8 = (uVar1 + 0x4);
  uStack10 = 0;
  while( true ) {
    uVar4 = (in_EDX >> 0x10);
    if (uStack8 <= uStack10) {
      return;
    }
    pass1_1020_bb16((iVar5 + 0x108),CONCAT22(0x1050,&local_12),CONCAT22(0x1050,&local_e),
                    uStack10);
    in_EDX = (astruct_57 *)CONCAT22(uVar4,uStack6);
    if (uStack6 < local_12) {
      pass1_1030_7ddc(uStack6,in_EDX,param_2,uStack6,local_e);
      uStack6 = 0;
    }
    else {
      uStack6 -= local_12;
      pass1_1030_7ddc(local_12,in_EDX,param_2,local_12,local_e);
    }
    if ((uStack6 | uStack6) == 0) break;
    uStack10 += 0x1;
  }
  return;
}



StructD * pass1_1030_eb14(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_97 * pass1_1030_eb50(param_1: *mut astruct_97)

{
  struct_op_1028_d1dc(param_1,0x1f3f);
  param_1->offset_0x0 = 0xecb2;
  (param_1 + 0x2) = 0x1030;
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x8)),s_SCMines_1050_59c6);
  return param_1;
}



u16 pass1_1030_eb86(mut param_1: u16 )

{
  let mut iVar1: i16;
  code **ppcVar2;
  astruct_92 *paVar3;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  let mut puStack24: *mut u32;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x700);
  while( true ) {
    uVar4 = param_1;
    paVar3 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar3));
    puStack24 = CONCAT22(uVar4,paVar3);
    param_1 = uVar4 | paVar3;
    if (param_1 == 0) break;
    if ((paVar3 + 1) == 0x5) {
      iVar1 = &paVar3->field5_0xc;
      if (((0x32 < iVar1) && (!SBORROW2(iVar1,0x33))) &&
         ((iVar1 == 0x34 || iVar1 + -0x33 < 0x1 || ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 0x2)))))) {
        ppcVar2 = (code **)(*puStack24 + 0x2c);
        (**ppcVar2)(0x1028);
        param_1 = extraout_DX;
      }
    }
  }
  return 0x1;
}
pub fn pass1_1030_ebf8(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  let mut puVar3: *mut u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut puVar7: *mut u32;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x108,paVar6);
  uVar5 = paVar6;
  puStack10 = CONCAT22(uVar5,param_1);
  if ((uVar5 | param_1) != 0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar8 = (param_3 >> 0x10);
    (param_1 + 0x4) = (param_3 + 0x4);
    puVar3 = (param_3 + 0x8);
    puVar7 = (param_1 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0; iVar4 += -1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    *puStack10 = 0xecb2;
    (param_1 + 0x2) = 0x1030;
  }
  return;
}



StructD * pass1_1030_ec86(StructD *param_1,param_2: u8)

{
  param_1->address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_97 * pass1_1030_ecc2(param_1: u8,param_2: *mut astruct_97)

{
  struct_op_1028_d1dc(param_2,0xf9f);
  param_2->offset_0x0 = 0xb96;
  (param_2 + 0x2) = &u16_1050_1038;
  unk_str_op_1000_3d3e((param_2 & 0xffff0000 | (param_2 + 0x8)),s_SCMorale_1050_59ce);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_ecf8(param_1: u8,param_2: *mut astruct_57,mut param_3: u32)

{
  let mut iVar1: i16;
  let mut puVar2: *mut u32;
  code **ppcVar3;
  let mut uVar4: u16;
  let mut uVar5: u32;
  astruct_92 *paVar6;
  let mut iVar7: i16;
  let mut uVar8: u32;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  StructD *pSVar13;
  let mut uVar15: u16;
  let mut bVar16: bool;
  let mut puVar17: *mut u32;
  let mut puVar18: *mut u32;
  let mut in_stack_0000fe40: u16;
  let mut in_stack_0000ff42: u16;
  let mut in_stack_0000ff64: u16;
  let mut in_stack_0000ff6a: u16;
  let mut in_stack_0000ff6e: u16;
  let mut uVar19: u16;
  let mut in_stack_0000ff98: u16;
  let mut uStack64: u32;
  let mut iStack56: i16;
  let mut uStack54: u16;
  astruct_685 *paStack38;
  astruct_92 local_22;
  let mut uStack12: u16;
  let mut paVar12: *mut Struct57;
  StructD *pSVar14;

  uStack12 = 0;
  puVar17 = mixed_1010_20ba(param_2,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ff98,0x2f),in_stack_0000fe40,
                            in_stack_0000ff64,in_stack_0000ff6a,in_stack_0000ff6e);
  uVar8 = param_2 & 0xffff0000 | puVar17 >> 0x10;
  uVar10 = puVar17;
  pass1_1010_ed3e(puVar17);
  uVar9 = uVar8 | uVar10;
  paVar12 = (astruct_57 *)(uVar8 & 0xffff0000 | uVar9);
  if (uVar9 != 0) {
    uStack12 = pass1_1030_2aaa(CONCAT22(uVar8,uVar10));
  }
  if (uStack12 < 0x2) {
    uStack12 = 0;
  }
  puVar17 = mixed_1010_20ba(paVar12,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ff98,0x2),in_stack_0000fe40,
                            in_stack_0000ff64,in_stack_0000ff6a,in_stack_0000ff6e);
  pSVar13 = (paVar12 & 0xffff0000 | puVar17 >> 0x10);
  if ((0x0 < PTR_LOOP_1050_13ae) && (!SBORROW2(PTR_LOOP_1050_13ae,1))) {
    if (PTR_LOOP_1050_13ae == &u16_1050_0002 || (PTR_LOOP_1050_13ae + -1) < 1) {
      if (0x6 < uStack12) {
        uStack12 -= 0x2;
    // TODO: goto LAB_1030_ed5b;
      }
      bVar16 = SBORROW2(uStack12,0x4);
      iVar1 = uStack12 - 0x4;
    }
    else {
      if (PTR_LOOP_1050_13ae != (&u16_1050_0002 + 1)) goto LAB_1030_ed5b;
      bVar16 = SBORROW2(uStack12,0x7);
      iVar1 = uStack12 - 0x7;
    }
    if (bVar16 == iVar1 < 0x0) {
      uStack12 -= 1;
    }
  }//
LAB_1030_ed5b:
  pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_22)),0x1,0x0,0x400);
  while( true ) {
    paVar6 = &local_22;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar6));
    uVar10 = pSVar13;
    paStack38 = (astruct_685 *)CONCAT22(uVar10,paVar6);
    if ((uVar10 | paVar6) == 0) break;
    uVar9 = &paVar6[0x1b].field6_0x10;
    pSVar13 = (pSVar13 & 0xffff0000 | (paVar6 + 0x1c));
    if (((&paVar6[0x1c].field3_0x4 + 0x2) != 0) && (paVar6[0x1c].field4_0x8 != 0x8000002)) {
      pass1_1030_38b8();
      uVar9 = pSVar13 | uVar9;
      uVar8 = pSVar13 & 0xffff0000;
      pSVar13 = (uVar8 | uVar9);
      if (uVar9 != 0) {
        puVar2 = paVar6->field5_0xc;
        uVar9 = (&paVar6->field5_0xc + 2);
        uVar8 |= uVar9;
        ppcVar3 = (code **)(*puVar2 + 0x10);
        puVar18 = puVar2;
        (**ppcVar3)(0x1028,puVar2,uVar9);
        uVar5 = puVar18 & 0xffff | uVar8 << 0x10;
        uStack54 = (&paVar6[0x1].field3_0x4 + 2);
        uVar15 = SUB42(&u16_1050_1038,0x0);
        pass1_1038_4760(CONCAT22(uVar10,paVar6));
        iVar1 = paVar6[0x1].field6_0x10;
        iStack56 = iVar1 / 0xa;
        iVar7 = (paVar6 + 2);
        if (iVar7 < 0x33) {
          if (iVar7 < 0x32) {
            iStack56 += -0x1;
          }
        }
        else {
          uStack54 += 0x1;
        }
        pSVar13 = (uVar8 & 0xffff0000 | iVar1 % 0xa & 0xffffU);
        for (uStack64 = 0; uStack64 < uVar5; uStack64 += 1) {
          ppcVar3 = (code **)(*puVar2 + 0x4);
          uVar8 = uVar5;
          (**ppcVar3)(uVar15,puVar2,(puVar2 >> 0x10),uStack64,(uStack64 >> 0x10));
          uVar9 = uVar8;
          uVar11 = pSVar13 | uVar9;
          pSVar14 = (pSVar13 & 0xffff0000 | uVar11);
          if (uVar11 != 0) {
            uVar15 = 0x1028;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar8 & 0xffff | pSVar13 << 0x10);
            puVar18 = struct_op_1030_73a8((astruct_419 *)CONCAT22(pSVar14,uVar9),uVar9,pSVar14);
            uVar9 = puVar18;
            uVar11 = (puVar18 >> 0x10);
            pSVar14 = (pSVar14 & 0xffff0000 | (uVar11 | uVar9));
            if (((uVar11 | uVar9) != 0) && ((uVar9 + 0x12) == 0x5)) {
              ppcVar3 = (code **)(*puVar18 + 0x48);
              (**ppcVar3)(0x1028,uVar9,uVar11);
              if (uVar9 < 0x0) {
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
        uVar19 = (param_3 >> 0x10);
        uVar4 = param_3;
        iVar7 = iVar1;
        pass1_1038_01c0(uVar4,uVar19,paStack38);
        iVar7 -= iVar1;
        iStack56 = (iStack56 - uStack12) - iVar7;
        pass1_1038_008e(pSVar13,uVar4,uVar19,paStack38);
        if (iVar7 < 0x0) {
          iStack56 += iVar7;
        }
        else {
          uStack54 += iVar7;
        }
        if (0x3e8 < uStack54) {
          uStack54 = 0x3e8;
        }
        if (uStack54 < 0x0) {
          uStack54 = 0;
        }
        uStack54 += iStack56;
        if (0x3e8 < uStack54) {
          uStack54 = 0x3e8;
        }
        if (uStack54 < 0x0) {
          uStack54 = 0;
        }
        pass1_1038_4d0e(paStack38,uStack54);
        if (paVar6->field3_0x4 == 0x4000001) {
          pass1_1038_08d4(param_1,pSVar13,uVar4,CONCAT22(uStack54,uVar19),paStack38);
        }
        pass1_1038_095e(pSVar13,uVar4,uVar19,uStack54,paStack38,in_stack_0000ff42);
      }
    }
  }
  return;
}
