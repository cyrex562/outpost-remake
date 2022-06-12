
u16 pass1_1020_a426(void)

{
  let mut puVar1: *mut u16;

  pass1_1008_3e38((astruct_19 *)&PTR_1048_4230);
  puVar1 = pass1_1008_3e38((astruct_19 *)0x10484236);
  return puVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 * pass1_1020_a43e(u8 *param_1,param_2: *mut u16)

{
  let mut in_register_0000000a: u16;
  let mut in_stack_0000fe9c: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000fff2: u32;

  *param_2 = 0xba36;
  ((int)param_2 + 0x2) = 0x1020;
  if (_PTR_LOOP_1050_4e74 != 0x0) {
    return param_2;
  }
  mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                  (u8 **)CONCAT22((int)((u32)in_stack_0000fff2 >> 0x10),0x2),in_stack_0000fe9c,
                  in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
  if ((0x0 < (int)PTR_LOOP_1050_13ae) && (!SBORROW2((int)PTR_LOOP_1050_13ae,0x1))) {
    if (PTR_LOOP_1050_13ae == &u16_1050_0002 || (int)(PTR_LOOP_1050_13ae + -0x1) < 0x1) {
      PTR_LOOP_1050_4e74 = 0x44b4;
      goto LAB_1020_a482;
    }
    if (PTR_LOOP_1050_13ae == &u32_1050_0004) {
      PTR_LOOP_1050_4e74 = 0x4b2c;
      goto LAB_1020_a482;
    }
  }
  PTR_LOOP_1050_4e74 = 0x47f0;//
LAB_1020_a482:
  _PTR_LOOP_1050_4e74 = CONCAT22(0x1050,PTR_LOOP_1050_4e74);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_a49a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,i16 *param_4,mut param_5: u16 )

{
  let mut uVar1: u32;
  let mut in_register_0000000a: u16;
  let mut uVar2: u16;
  let mut in_stack_0000fd62: u16;
  let mut in_stack_0000fe86: u16;
  let mut in_stack_0000fe8c: u16;
  let mut in_stack_0000fe90: u16;
  let mut iVar3: i16;
  u8 local_136 [0x128];
  let mut uStack14: u16;
  let mut uStack12: u16;
  let mut uStack10: u32;
  u32 *puStack6;

  puStack6 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                             (u8 **)CONCAT22(param_2,0x2f),in_stack_0000fd62,in_stack_0000fe86,in_stack_0000fe8c,
                             in_stack_0000fe90);
  uStack12 = ((u32)puStack6 >> 0x10);
  uVar1 = (u32)((int)puStack6 + 0x20);
  uStack10 = uVar1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1);
  uStack14 = uVar1;
  if (param_4 != NULL) {
    uVar2 = ((u32)param_4 >> 0x10);
    if (((u32 *)param_4 + 0x1) == 0x0) {
      iVar3 = (int)&PTR_LOOP_1050_4230;
    }
    else {
      iVar3 = (int)s_dib_1050_4234 + 0x2;
    }
    pass1_1008_3f32(param_4,CONCAT22(0x1048,iVar3));
    struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_136),0x0,0x0,param_5,(u32 *)param_4,uVar2,
                        (u32)((int)_PTR_LOOP_1050_4e70 + 0x4),uStack10);
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_136));
    return;
  }
  pass1_1020_abc0(param_3,param_5,uVar1 & 0xffff | (u32)uStack12 << 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_a54c(u8 *param_1,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut in_register_0000000a: u16;
  let mut unaff_SI: u16;
  let mut in_stack_0000fd58: u16;
  let mut in_stack_0000fe7c: u16;
  let mut in_stack_0000fe82: u16;
  let mut in_stack_0000fe86: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  u8 local_140 [0x124];
  astruct_19 **ppaStack28;
  let mut local_18: i16;
  let mut local_16: u16;
  astruct_19 *local_14;
  u8 *puStack16;
  let mut uStack14: u16;
  let mut uStack12: u16;
  let mut uStack10: u32;
  u32 *puStack6;

  puStack6 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                             (u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fd58,in_stack_0000fe7c,in_stack_0000fe82
                             ,in_stack_0000fe86);
  uStack12 = ((u32)puStack6 >> 0x10);
  uVar2 = (u32)((int)puStack6 + 0x20);
  uStack10 = uVar2;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2);
  uStack14 = uVar2;
  local_14 = PTR_1048_4230;
  puStack16 = PTR_LOOP_1048_4234;
  ppaStack28 = &local_14;
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_14),(u16 *)CONCAT22(0x1050,&local_18),(char *)CONCAT22(0x1050,&local_16)
                 );
  if ((param_4 < 0x0) || (0x5 < param_4)) {
    pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_14),0x0,local_18 - 0x9,local_16);
    uVar6 = uStack10;
    uVar7 = (uStack10 >> 0x10);
    uVar1 = (u32)((int)_PTR_LOOP_1050_4e70 + 0x4);
    uVar4 = uVar1;
    uVar5 = ((u32)uVar1 >> 0x10);
    uVar3 = 0x14;
  }
  else {
    pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_14),0x0,(local_18 - param_4) - 0x3,local_16);
    uVar6 = uStack10;
    uVar7 = (uStack10 >> 0x10);
    uVar1 = (u32)((int)_PTR_LOOP_1050_4e70 + 0x4);
    uVar4 = uVar1;
    uVar5 = ((u32)uVar1 >> 0x10);
    uVar3 = 0x7b;
  }
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_140),0x0,0x0,uVar3,(u32 *)&local_14,&DAT_1050_1050,
                      CONCAT22(uVar5,uVar4),CONCAT22(uVar7,uVar6));
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_140));
  return;
}



BOOL16 pass1_1020_a644(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut BVar1: bool;

  BVar1 = write_to_file_1008_7cac(param_3);
  if (BVar1 != 0x0) {
    BVar1 = 0x1;
  }
  return BVar1;
}



BOOL16 read_file_1020_a65e(u16_t param_1,mut param_2: u32,HFILE16 *param_3)

{
  let mut BVar1: bool;
  let mut in_DX: u16;
  u8 local_a [0x2];
  u8 local_8 [0x2];
  u8 local_6 [0x2];
  u8 local_4 [0x2];
  u16_t uVar3;
  u16_t uVar2;

  read_file_1008_7cfe((int)param_3,(int)((u32)param_3 >> 0x10),0xb);
  if (param_1 != 0x0) {
    if (0x1 < (int)u16_1050_0312) {//
LAB_1020_a6dc:
      pass1_1020_b97e(param_1,in_DX,param_2,((u32)param_2 >> 0x10),0x0);
      return 0x1;
    }
    BVar1 = read_file_1008_7dee(param_3,CONCAT22(0x1050,local_4),0x2);
    if (BVar1 != 0x0) {
      BVar1 = read_file_1008_7dee(param_3,CONCAT22(0x1050,local_8),0x2);
      if (BVar1 != 0x0) {
        BVar1 = read_file_1008_7dee(param_3,CONCAT22(0x1050,local_6),0x2);
        if (BVar1 != 0x0) {
          param_1 = read_file_1008_7dee(param_3,CONCAT22(0x1050,local_a),0x2);
          if (param_1 != 0x0) goto LAB_1020_a6dc;
        }
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_a6ee(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32,mut param_5: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut puVar3: *mut u16;
  let mut in_stack_0000fd48: u16;
  let mut in_stack_0000fe6c: u16;
  let mut in_stack_0000fe72: u16;
  let mut in_stack_0000fe76: u16;
  let mut uVar4: u16;
  u8 local_13e [0x120];
  let mut uStack30: u32;
  let mut BStack26: bool;
  let mut local_18: u32;
  let mut uStack20: u16;
  let mut iStack18: i16;
  let mut uStack16: u16;
  let mut uStack14: u32;
  u32 *puStack10;
  let mut uStack6: u32;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000002);
  uStack6 = CONCAT22(param_2,param_1);
  if (((param_2 | param_1) == 0x0) || (*(i32 *)(param_1 + 0x200) == 0x8000002)) {
    puStack10 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_2 | param_1),_u16_1050_0ed0,
                                (u8 **)CONCAT22(param_3,0x2f),in_stack_0000fd48,in_stack_0000fe6c,
                                in_stack_0000fe72,in_stack_0000fe76);
    uStack16 = ((u32)puStack10 >> 0x10);
    uVar1 = (u32)((int)puStack10 + 0x20);
    uStack14 = uVar1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1);
    iStack18 = (int)uVar1;
    puVar3 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,&local_18));
    uVar2 = ((u32)puVar3 >> 0x10);
    BStack26 = pass1_1008_c6ae(_u16_1050_06e0,param_5,0x28);
    if (BStack26 != 0x0) {
      uStack20 = 0x1;
    }
    uVar4 = (param_4 >> 0x10);
    pass1_1020_b2da(param_4,uVar4,(BStack26 != 0x0),(u16 *)CONCAT22(0x1050,&local_18),
                    CONCAT22(uStack16,iStack18));
    struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_13e),0x0,0x0,param_5,&local_18,&DAT_1050_1050,
                        (u32)((int)_PTR_LOOP_1050_4e70 + 0x4),(u32)(iStack18 + 0x4));
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_13e));
    if (BStack26 != 0x0) {
      pass1_1020_ad90(uVar2,param_4,uVar4,(u16 *)CONCAT22(0x1050,&local_18),(u32)(iStack18 + 0x4));
    }
    (u32)((int)uStack30 + 0x1c) = 0x8000001;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_a80e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  u32 *puVar4;
  let mut in_stack_0000fe8c: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffe4: u16;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000002);
  if (((param_2 | param_1) == 0x0) || (*(i32 *)(param_1 + 0x200) == 0x8000002)) {
    puVar4 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_2 | param_1),_u16_1050_0ed0,
                             (u8 **)CONCAT22(in_stack_0000ffe4,0x2f),in_stack_0000fe8c,in_stack_0000ffb0,
                             in_stack_0000ffb6,in_stack_0000ffba);
    uVar3 = ((u32)puVar4 >> 0x10);
    uVar2 = (u32)((int)puVar4 + 0x20);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2);
    uVar1 = uVar2;
    if (param_5 == 0xa) {
      pass1_1020_b872(CONCAT22(param_4,param_3),uVar2 & 0xffff | (u32)uVar3 << 0x10);
      return;
    }
    pass1_1020_b0aa(uVar3,param_3,param_4,param_5);
    if (uVar1 != 0x0) {
      pass1_1020_abc0(CONCAT22(param_4,param_3),uVar1,uVar2 & 0xffff | (u32)uVar3 << 0x10);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_a89e(mut param_1: u32,u32 *param_2,mut param_3: u16 )

{
  let mut piVar1: *mut i16;
  u8 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut uVar6: u16;
  astruct_57 *in_EDX;
  let mut uVar7: u32;
  astruct_57 *paVar8;
  let mut unaff_SI: u16;
  let mut in_stack_0000f892: u16;
  let mut in_stack_0000f9b6: u16;
  let mut in_stack_0000f9bc: u16;
  let mut in_stack_0000f9c0: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut local_5ee: u16;
  let mut uStack1516: u16;
  u32 *puStack1218;
  let mut iStack1214: i16;
  let mut uStack1212: u32;
  u8 local_4b8 [0x8];
  let mut uStack1200: u32;
  u32 *puStack1196;
  u8 local_4a8 [0x124];
  u8 local_384 [0x124];
  u8 local_260 [0x124];
  u8 local_13c [0x124];
  let mut local_18: u16;
  let mut local_16: u16;
  let mut local_14: u32;
  let mut uStack16: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  u32 *puStack6;

  puStack6 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000f892,
                             in_stack_0000f9b6,in_stack_0000f9bc,in_stack_0000f9c0);
  uVar7 = (u32)in_EDX & 0xffff0000 | (u32)puStack6 >> 0x10;
  uVar5 = (u32)((int)puStack6 + 0x20);
  uStack10 = uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5);
  uStack14 = uVar5 & 0xffff | uVar7 << 0x10;
  local_14 = *param_2;
  uStack16 = (param_2 + 0x1);
  puStack1218 = &local_14;
  paVar8 = (astruct_57 *)(uVar7 & 0xffff0000 | ZEXT24(&local_14));
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_14),(u16 *)CONCAT22(0x1050,&local_18),(char *)CONCAT22(0x1050,&local_16)
                 );
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_14),0x0,local_18,local_16 + 0x2);
  struct_op_1028_8888((astruct_97 *)CONCAT22(0x1050,local_13c),0x0,0x7a,&local_14,&DAT_1050_1050,0x8000002,
                      0x4000002,uStack10);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_13c));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_14),0x0,local_18 - 0x2,local_16);
  struct_op_1028_8888((astruct_97 *)CONCAT22(0x1050,local_260),0x0,0x47,&local_14,&DAT_1050_1050,0x8000002,
                      0x4000002,uStack10);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_260));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_14),0x1,local_18 - 0x2,local_16);
  struct_op_1028_8888((astruct_97 *)CONCAT22(0x1050,local_384),0x0,0x6a,&local_14,&DAT_1050_1050,0x8000002,
                      0x4000002,uStack10);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_384));
  uVar9 = param_1;
  uVar10 = (param_1 >> 0x10);
  pass1_1020_ad90(paVar8,uVar9,uVar10,(u16 *)CONCAT22(0x1050,&local_14),uStack10);
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_14),0x1,local_18 - 0x2,local_16 + 0x1);
  struct_op_1028_8888((astruct_97 *)CONCAT22(0x1050,local_4a8),0x0,0x7f,&local_14,&DAT_1050_1050,0x8000002,
                      0x4000002,uStack10);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_4a8));
  pass1_1020_ad90(paVar8,uVar9,uVar10,(u16 *)CONCAT22(0x1050,&local_14),uStack10);
  puStack1196 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x8),in_stack_0000f892,
                                in_stack_0000f9b6,in_stack_0000f9bc,in_stack_0000f9c0);
  uVar7 = (u32)puStack1196 >> 0x10;
  uStack1200 = (u32)((int)puStack1196 + 0x12);
  pass1_1008_5784((char *)CONCAT22(0x1050,local_4b8),uStack1200);
  iStack1214 = 0x0;
  do {
    do {
      uVar6 = uVar7;
      puVar2 = local_4b8;
      pass1_1008_5b12((char *)CONCAT22(0x1050,puVar2));
      uStack1212 = CONCAT22(uVar6,puVar2);
      uVar7 = (u32)(uVar6 | puVar2);
      if ((uVar6 | puVar2) == 0x0) {
        pass1_1010_9674((u32)puStack1196);
        return;
      }
    } while (((puVar2 + 0x4) != 0x3e) && ((puVar2 + 0x4) != 0x41));
    while (0x0 < ((int)uStack1212 + 0x6)) {
      if (iStack1214 == 0x0) {
        uVar4 = local_16 - 0x2;//
LAB_1020_ab4a:
        uVar3 = local_18 - 0x2;//
LAB_1020_ab51:
        iStack1214 = iStack1214 + 0x1;
        pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_14),0x0,uVar3,uVar4);
      }
      else {
        if (iStack1214 == 0x1) {
          uVar4 = local_16 + 0x2;
          goto LAB_1020_ab4a;
        }
        if (iStack1214 == 0x2) {
          uVar4 = local_16 + 0x2;//
LAB_1020_ab6e:
          uVar3 = local_18 + 0x2;
          goto LAB_1020_ab51;
        }
        if (iStack1214 == 0x3) {
          uVar4 = local_16 - 0x2;
          goto LAB_1020_ab6e;
        }
        iStack1214 = iStack1214 + 0x1;
        pass1_1020_b2da(uVar9,uVar10,0x0,(u16 *)CONCAT22(0x1050,&local_14),uStack14);
      }
      struct_op_1028_8888((astruct_97 *)CONCAT22(0x1050,&local_5ee),0x0,((int)uStack1212 + 0x4),&local_14,
                          &DAT_1050_1050,0x8000002,0x4000002,uStack10);
      fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_5ee));
      piVar1 = ((int)uStack1212 + 0x6);
      *piVar1 = *piVar1 + -0x1;
      local_5ee = 0x389a;
      uStack1516 = 0x1008;
    }
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_abc0(mut param_1: u32,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut puVar3: *mut u16;
  let mut uVar4: u16;
  u8 local_12e [0x124];
  let mut BStack10: bool;
  let mut local_8: u32;
  let mut uStack4: u16;

  puVar3 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,&local_8));
  uVar1 = ((u32)puVar3 >> 0x10);
  BStack10 = pass1_1008_c6ae(_u16_1050_06e0,param_2,0x28);
  if (BStack10 != 0x0) {
    uStack4 = 0x1;
  }
  uVar4 = (param_1 >> 0x10);
  pass1_1020_b2da(param_1,uVar4,(BStack10 != 0x0),(u16 *)CONCAT22(0x1050,&local_8),param_3);
  uVar2 = (param_3 >> 0x10);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_12e),0x0,0x0,param_2,&local_8,&DAT_1050_1050,
                      (u32)((int)_PTR_LOOP_1050_4e70 + 0x4),(u32)((int)param_3 + 0x4));
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_12e));
  if (BStack10 != 0x0) {
    pass1_1020_ad90(uVar1,param_1,uVar4,(u16 *)CONCAT22(0x1050,&local_8),(u32)((int)param_3 + 0x4));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_ac6e(mut param_1: u16 ,mut param_2: u32,mut param_3: i16,mut param_4: i16,mut param_5: i16)

{
  let mut uVar1: u16;
  u32 *puVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut in_EDX: u32;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  let mut in_stack_0000fd56: u16;
  let mut in_stack_0000fe7a: u16;
  let mut in_stack_0000fe80: u16;
  let mut in_stack_0000fe84: u16;
  let mut iVar7: i16;
  u8 local_146 [0x12c];
  let mut iStack26: i16;
  let mut uStack24: u16;
  let mut uStack22: u32;
  u32 *puStack18;
  let mut local_e: u32;
  let mut local_8: u16;
  let mut local_6: u32;

  uVar5 = ((u32)in_EDX >> 0x10);
  if (param_3 == 0x0) {
    iVar7 = (int)&PTR_LOOP_1050_4230;
  }
  else {
    iVar7 = (int)s_dib_1050_4234 + 0x2;
  }
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1048,iVar7),(u16 *)CONCAT22(0x1050,&local_8),
                  (u16 *)CONCAT22(0x1050,&local_6),(u16 *)CONCAT22(0x1050,(int)&local_6 + 0x2));
  if (param_5 == 0x0) {
    local_6 = local_6 & 0xffff | (u32)(local_6 + param_4) << 0x10;
  }
  else if (param_5 == 0x1) {
    local_6 = local_6 & 0xffff0000 | (u32)((int)local_6 + param_4);
  }
  else if (param_5 == 0x2) {
    local_6 = local_6 & 0xffff | (u32)(local_6 - param_4) << 0x10;
  }
  puVar6 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,&local_e),local_8,local_6,(local_6 >> 0x10));
  puStack18 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar5,(int)((u32)puVar6 >> 0x10)),_u16_1050_0ed0,
                              (u8 **)CONCAT22(param_1,0x2f),in_stack_0000fd56,in_stack_0000fe7a,in_stack_0000fe80
                              ,in_stack_0000fe84);
  uVar4 = ((u32)puStack18 >> 0x10);
  uVar3 = (u32)((int)puStack18 + 0x20);
  uStack22 = uVar3;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3);
  iStack26 = (int)uVar3;
  uStack24 = uVar4;
  uVar1 = pass1_1020_b1ae((int)&local_e,uVar4,param_2,(param_2 >> 0x10),
                          (u16 *)CONCAT22(0x1050,&local_e),(u32)(iStack26 + 0x4));
  if (uVar1 != 0x0) {
    puVar2 = &local_e;
    pass1_1020_b240(uVar4,param_2,CONCAT22(0x1050,puVar2),CONCAT22(uStack24,iStack26));
    if (puVar2 != NULL) {
      struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_146),0x0,0x0,(-(param_3 == 0x0) & 0xfffb) + 0x7f,
                          &local_e,&DAT_1050_1050,(u32)((int)_PTR_LOOP_1050_4e70 + 0x4),uStack22);
      fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_146));
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_ad90(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut u16,mut param_5: u32)

{
  code **ppcVar1;
  let mut puVar2: *mut u16;
  u8 *puVar3;
  let mut iVar4: i16;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut extraout_DX: u16;
  u32 *puVar7;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut local_17e: u16;
  let mut uStack380: u16;
  let mut iStack90: i16;
  u32 *puStack78;
  let mut uStack70: u16;
  let mut iStack68: i16;
  let mut uStack66: u32;
  u32 *puStack62;
  u8 local_3a [0xc];
  let mut local_2e: u32;
  let mut uStack42: u16;
  let mut iStack40: i16;
  let mut uStack38: u16;
  let mut local_24: i16;
  let mut local_22: i16;
  let mut uStack32: u32;
  let mut uStack28: u32;
  let mut uStack24: u32;
  let mut puStack20: *mut u16;
  let mut uStack18: u16;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut uStack12: u32;
  let mut local_8: u16;
  let mut local_6: i16;
  let mut local_4: i16;

  puVar2 = &local_8;
  pass1_1008_3eb4((astruct_615 *)param_4,(u16 *)CONCAT22(0x1050,puVar2),(u16 *)CONCAT22(0x1050,&local_6),
                  (u16 *)CONCAT22(0x1050,&local_4));
  pass1_1030_627e(puVar2,param_1,(u32)_PTR_LOOP_1050_5740,param_4,param_5);
  puStack20 = puVar2;
  uStack18 = param_1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_1,puVar2));
  uStack24 = CONCAT22(param_1,puVar2);
  uStack28 = (u32)(puVar2 + 0x17);
  uVar5 = (u32)((int)uStack28 + 0x4);
  uStack32 = uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
  iStack40 = (int)uVar5;
  uStack38 = param_1;
  puVar7 = (u32 *)pass1_1030_5b5c(iStack40,param_1);
  uVar6 = ((u32)puVar7 >> 0x10);
  local_2e = *puVar7;
  uStack42 = ((int)puVar7 + 0x4);
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
  if ((int)uStack12 < 0x0) {
    uStack12 &= 0xffff0000;
  }
  if (local_24 <= iStack16) {
    iStack16 = local_24 + -0x1;
  }
  pass1_1008_6c90((u16 *)CONCAT22(0x1050,local_3a));
  pass1_1008_6cec((u16 *)CONCAT22(0x1050,local_3a),local_8,CONCAT22(iStack14,iStack16),local_8,uStack12);
  puVar3 = local_3a;
  pass1_1030_6522(_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar3),param_5);
  puStack62 = (u32 *)CONCAT22(uVar6,puVar3);
  if ((uVar6 | puVar3) != 0x0) {
    uStack66 = 0x0;
    iStack68 = 0x0;
    for (uStack70 = uStack12; (int)uStack70 <= iStack16; uStack70 += 0x1) {
      for (puStack78 = (u32 *)uStack12; (int)puStack78 <= iStack14;
          puStack78 = (u32 *)((int)puStack78 + 0x1)) {
        ppcVar1 = (code **)((int)*puStack62 + 0x4);
        iVar4 = iStack68;
        iStack68 = iStack68 + 0x1;
        (**ppcVar1)(0x1030,(int)puStack62,(int)((u32)puStack62 >> 0x10));
        uStack66 = CONCAT22(extraout_DX,iVar4);
        uStack66._3_1_ = (char)(extraout_DX >> 0x8);
        if (uStack66._3_1_ == '\0') {
          iStack90 = iVar4;
          if (iVar4 == 0x7) {
            pass1_1008_3e76(param_4,local_8,uStack70,puStack78);
            uVar9 = uStack32;
            uVar10 = ((u32)uStack32 >> 0x10);
            uVar8 = 0x6;
          }
          else if (iVar4 == 0x8) {
            pass1_1008_3e76(param_4,local_8,uStack70,puStack78);
            uVar9 = uStack32;
            uVar10 = ((u32)uStack32 >> 0x10);
            uVar8 = 0x7;
          }
          else {
            if (iVar4 != 0x9) goto LAB_1020_af1c;
            pass1_1008_3e76(param_4,local_8,uStack70,puStack78);
            uVar9 = uStack32;
            uVar10 = ((u32)uStack32 >> 0x10);
            uVar8 = 0x8;
          }
          struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,&local_17e),0x0,0x0,uVar8,(u32 *)param_4,
                              ((u32)param_4 >> 0x10),CONCAT22(uVar10,uVar9),param_5);
          fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_17e));
          local_17e = 0x389a;
          uStack380 = 0x1008;
        }//
LAB_1020_af1c:
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_afc4(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut u16,i32 param_5)

{
  u32 *puVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  u8 bStack27;
  let mut local_a: u32;
  let mut uStack6: u32;

  puVar1 = &local_a;
  pass1_1030_64ce(puVar1,param_1,_PTR_LOOP_1050_5740,param_4,param_5,(u32 *)CONCAT22(0x1050,puVar1));
  uStack6 = *puVar1;
  uVar3 = ((int)puVar1 + 0x2);
  bStack27 = (u8)(uStack6 >> 0x18);
  uVar2 = bStack27;
  if (bStack27 == 0x0) {
    return;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6 & 0xffff | (u32)uVar3 << 0x10);
  uVar4 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar3,uVar2),uVar2,uVar3);
  uVar3 = (uVar4 >> 0x10);
  if ((uVar3 | uVar4) != 0x0) {
    switch((uVar4 + 0xc)) {
    case 0x1:
      break;
    case 0x2:
      break;
    case 0x3:
      break;
    case 0x4:
      break;
    case 0x5:
      break;
    case 0x6:
      break;
    case 0x7:
      return;
    case 0x8:
      return;
    case 0x9:
      return;
    }
    return;
  }
  return;
}

