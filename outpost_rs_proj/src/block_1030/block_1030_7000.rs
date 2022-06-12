
pub fn pass1_1030_701c(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar2 << 0x10),in_AX,in_DX);
  }
  uVar1 = (u32)((int)param_1 + 0x1a);
  if (((int)uVar1 + 0x12) == 0x5) {
    return;
  }
  return;
}
pub fn pass1_1030_7064(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar2 << 0x10),in_AX,in_DX);
  }
  uVar1 = (u32)((int)param_1 + 0x1a);
  if (((int)uVar1 + 0x12) == 0x5) {
    return;
  }
  return;
}
pub fn pass1_1030_70ac(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar2 << 0x10),in_AX,in_DX);
  }
  uVar1 = (u32)((int)param_1 + 0x1a);
  if (((int)uVar1 + 0x12) == 0x5) {
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_70f4(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut in_AX: u16;
  let mut BVar3: bool;
  let mut in_DX: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  i32 *plVar6;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(i32 *)(iVar4 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar5 << 0x10),in_AX,in_DX);
  }
  uVar2 = (u32)(iVar4 + 0x1a);
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
    uVar2 = (u32)(iVar4 + 0x1a);
    plVar6 = *(i32 **)((int)uVar2 + 0x28);
  }
  pass1_1020_ba94(plVar6);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_7176(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  i32 local_1a;
  i16 local_16 [0x2];
  let mut uStack18: u16;
  let mut uStack14: u16;
  let mut BStack10: bool;
  let mut uStack8: u16;
  i32 lStack6;

  lStack6 = 0x0;
  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(i32 *)(iVar2 + 0x22) == 0x0) {
    return;
  }
  if (*(i32 *)(iVar2 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)param_1,in_AX,in_DX);
  }
  uVar1 = (u32)(iVar2 + 0x1a);
  uStack8 = ((int)uVar1 + 0xc);
  BStack10 = pass1_1008_c6ae(_u16_1050_06e0,uStack8,0x3);
  if ((BStack10 != 0x0) && (uVar1 = (u32)(iVar2 + 0x1a), ((int)uVar1 + 0x12) == 0x5)) {
    uVar1 = (u32)(iVar2 + 0x22);
    uStack14 = ((int)uVar1 + 0x4);
    for (uStack18 = 0x0; uStack18 < uStack14; uStack18 += 0x1) {
      pass1_1020_bb16((u32*)(iVar2 + 0x22),(u32 *)CONCAT22(0x1050,&local_1a),(u16 *)CONCAT22(0x1050,local_16),
                      uStack18);
      if (0x0 < local_16[0]) {
        lStack6 += local_1a;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_7226(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut in_AX: u16;
  let mut BVar3: bool;
  let mut in_DX: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(i32 *)(iVar4 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar5 << 0x10),in_AX,in_DX);
  }
  uVar2 = (u32)(iVar4 + 0x1a);
  BVar3 = pass1_1008_c6ae(_u16_1050_06e0,((int)uVar2 + 0xc),0x10);
  if (((BVar3 != 0x0) && (uVar2 = (u32)(iVar4 + 0x1a), ((int)uVar2 + 0x12) == 0x5)) &&
     (uVar1 = (u32)(iVar4 + 0x1a), uVar2 = (u32)(uVar1 & 0xffff0000 | (u32)((int)uVar1 + 0x14)),
     ((int)uVar2 + 0xa4) == 0x1e)) {
    return;
  }
  return;
}
pub fn fn_ptr_1030_7296(param_1: *mut astruct_292)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
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
  (u32)&iVar3->field34_0x22 = 0x0;
  return;
}
pub fn pass1_1030_72d0(param_1: *mut astruct_292)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  astruct_292 *iVar3;
  let mut uVar3: u16;
  char *pcStack6;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar3 = (astruct_292 *)param_1;
  uVar1 = (iVar3 + 0x1);
  uVar2 = &iVar3[0x1].field_0x2;
  pcStack6 = (char *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    fn_ptr_1020_ba7e((u32 *)CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack6);
  }
  (u32)(iVar3 + 0x1) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_730a(mut param_1: u16 ,param_2: *mut astruct_290)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut uVar4: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  astruct_290 *pstruct295_1;
  astruct_290 *pstruct295_2;
  let mut unaff_CS: u16;
  u32 *puVar5;
  let mut uStack10: u32;
  let mut uStack6: u32;

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
      if ((extraout_DX_00 | uVar4) != 0x0) {
        unaff_CS = 0x1028;
        pass1_1028_e332(_PTR_LOOP_1050_65e2,uVar4,extraout_DX_00);
      }
    }
    // WARNING: Load size is inaccurate
    puVar1 = pstruct295_1->field30_0x1e;
    uVar2 = ((int)&pstruct295_1->field30_0x1e + 0x2);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)(unaff_CS,puVar1,uVar2,0x1,puVar5);
    }
    pstruct295_1->field30_0x1e = NULL;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn struct_op_1030_73a8(param_1: *mut astruct_419,mut param_2: u16 ,mut param_3: u16 ) -> u32

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(i32 *)(iVar1 + 0x16) == 0x0) {
    return 0x0;
  }
  if (*(i32 *)(iVar1 + 0x1a) == 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(iVar1 + 0x16));
    (iVar1 + 0x1a) = param_2;
    (iVar1 + 0x1c) = param_3;
  }
  return CONCAT22((iVar1 + 0x1c),(iVar1 + 0x1a));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_73ee(mut param_1: u16 ,param_2: *mut astruct_294,mut param_3: u32)

{
  astruct_294 *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_2 >> 0x10);
  iVar1 = (astruct_294 *)param_2;
  iVar1->field42_0x2a = param_3;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_3);
  iVar1->field43_0x2e = (int)param_3;
  iVar1->field44_0x30 = param_1;
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1030_7418(mut param_1: i16,param_2: *mut astruct_731,mut param_3: u32)

{
  let mut uVar1: u32;
  astruct_731 *iVar2;
  let mut iVar3: i16;
  let mut BVar4: bool;
  u8 *puVar5;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar6: u16;
  HFILE16 in_stack_0000ffac;
  let mut uStack62: u16;
  u16 local_2a [0x2];
  u8 local_26 [0xe];
  let mut local_18: u32;
  u32 local_14 [0x2];
  let mut local_c: u16;
  let mut local_a: u32;
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
  uVar6 = ((u32)param_2 >> 0x10);
  local_c = iVar2->field18_0x12;
  BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffac);
  if (BVar4 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  local_6[0] = iVar2->field19_0x14;
  BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffac);
  if (BVar4 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  local_18 = iVar2->field20_0x16;
  BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_18),(char *)0x4,in_stack_0000ffac);
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
  BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_a),(char *)0x4,in_stack_0000ffac);
  if (BVar4 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  local_c = iVar2->field33_0x32;
  BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffac);
  if (BVar4 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  local_c = iVar2->field34_0x34;
  BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffac);
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
  local_6[0] = local_18;
  BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffac);
  if (BVar4 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  pass1_1008_5784((char *)CONCAT22(0x1050,local_26),iVar2->field36_0x3a);
  while( true ) {
    puVar5 = local_26;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar5));
    local_14[0] = CONCAT22(extraout_DX,puVar5);
    if ((extraout_DX | puVar5) == 0x0) {
      if (iVar2->field37_0x3e == 0x0) {
        uStack62 = 0x0;
      }
      else {
        uVar1 = iVar2->field37_0x3e;
        uStack62 = ((int)uVar1 + 0x8);
      }
      local_2a[0] = uStack62;
      BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_2a),(char *)0x2,in_stack_0000ffac);
      if (BVar4 == 0x0) {
        u16_1050_0310 = 0x6d0;
        return;
      }
      pass1_1008_5784((char *)CONCAT22(0x1050,local_26),iVar2->field37_0x3e);
      while( true ) {
        puVar5 = local_26;
        pass1_1008_5b12((char *)CONCAT22(0x1050,puVar5));
        if ((extraout_DX_00 | puVar5) == 0x0) {
          return;
        }
        local_18 = local_18 & 0xffff0000 | (u32)(puVar5 + 0x4);
        BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_18),(char *)0x2,in_stack_0000ffac);
        if (BVar4 == 0x0) {
          u16_1050_0310 = 0x6d0;
          return;
        }
        local_14[0] = local_14[0] & 0xffff0000 | (u32)(puVar5 + 0x6);
        BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_14),(char *)0x2,in_stack_0000ffac);
        if (BVar4 == 0x0) {
          u16_1050_0310 = 0x6d0;
          return;
        }
        local_c = (puVar5 + 0x8);
        BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffac);
        if (BVar4 == 0x0) break;
        local_c = (puVar5 + 0xa);
        BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffac);
        if (BVar4 == 0x0) {
          u16_1050_0310 = 0x6d0;
          return;
        }
        local_6[0] = (puVar5 + 0xc);
        BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffac);
        if (BVar4 == 0x0) {
          u16_1050_0310 = 0x6d0;
          return;
        }
      }
      u16_1050_0310 = 0x6d0;
      return;
    }
    local_6[0] = (puVar5 + 0x4);
    BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffac);
    if (BVar4 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return;
    }
    local_6[0] = ((int)local_14[0] + 0x6);
    BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffac);
    if (BVar4 == 0x0) break;
    local_6[0] = ((int)local_14[0] + 0x8);
    BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffac);
    if (BVar4 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return;
    }
    local_6[0] = ((int)local_14[0] + 0xa);
    BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffac);
    if (BVar4 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return;
    }
    local_6[0] = ((int)local_14[0] + 0xc);
    BVar4 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffac);
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
pub fn file_1030_778c(mut param_1: i16,u8 *param_2,param_3: *mut astruct_373,HFILE16 *param_4)

{
  code **ppcVar1;
  astruct_387 *iVar3;
  let mut BVar2: bool;
  let mut iVar6: i16;
  i32 *plVar7;
  astruct_169 *paVar8;
  let mut uVar9: u16;
  let mut uVar11: u16;
  astruct_99 *uVar10;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar12;
  astruct_99 *iVar4;
  astruct_99 *iVar5;
  let mut uVar13: u16;
  let mut uVar15: u16;
  astruct_99 *uVar14;
  u16 local_56 [0x2];
  let mut uStack82: u16;
  astruct_99 *paStack74;
  u16 local_46 [0x2];
  u16 local_42 [0x2];
  u32 local_3e [0x3];
  astruct_99 *paStack50;
  let mut local_2e: u16;
  astruct_99 *paStack44;
  u16 local_28 [0x2];
  u16 local_24 [0x2];
  u16 local_20 [0x9];
  let mut uStack14: u16;
  let mut local_4: u16;
  astruct_388 *uVar5;
  astruct_99 *uVar8;

  paVar12 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  file_1030_1730(param_3,param_4);
  if (param_1 != 0x0) {
    iVar3 = (astruct_387 *)param_3;
    iVar3 = (astruct_387 *)&iVar3->field_0xc;
    BVar2 = read_file_1008_7bc8((u32)param_4,(u16 *)((u32)param_3 & 0xffff0000 | ZEXT24(iVar3)));
    if ((BVar2 != 0x0) && (BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_4),0x2), BVar2 != 0x0)) {
      uVar13 = ((u32)param_3 >> 0x10);
      iVar3->field18_0x12 = local_4;
      BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_4),0x2);
      if (BVar2 != 0x0) {
        iVar3->field19_0x14 = local_4;
        BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 | ZEXT24(&iVar3->field_0x16)),0x4);
        if (BVar2 != 0x0) {
          plVar7 = (i32 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar3->field_0x1e));
          file_1008_76e4(paVar12,param_4,plVar7);
          if (((((int)plVar7 != 0x0) &&
               (iVar6 = file_1008_77cc((int)paVar12,(u32)param_4,
                                       (i32 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar3->field_0x22))), iVar6 != 0x0
               )) && (iVar6 = file_1008_77cc((int)paVar12,(u32)param_4,
                                             (i32 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar3->field_0x26))),
                     iVar6 != 0x0)) &&
             (BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 | ZEXT24(&iVar3->field40_0x2a)),0x4
                                         ), BVar2 != 0x0)) {
            if (iVar3->field40_0x2a != 0x0) {
              pass1_1028_e1ec(_PTR_LOOP_1050_65e2,iVar3->field40_0x2a);
              iVar3->field41_0x2e = BVar2;
              iVar3->field42_0x30 = paVar12;
            }
            if ((int)u16_1050_0312 < 0x2) {
              return;
            }
            BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 | ZEXT24(&iVar3->field_0x32)),0x2);
            if ((BVar2 != 0x0) &&
               (BVar2 = read_file_1008_7dee(param_4,((u32)param_3 & 0xffff0000 | ZEXT24(&iVar3->field_0x34)),0x2
                                           ), BVar2 != 0x0)) {
              paVar8 = (astruct_169 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar3->field_0x36));
              pass1_1008_766e(paVar12,(u32)param_4,paVar8);
              if (((int)paVar8 != 0x0) &&
                 (BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,local_20),0x2), BVar2 != 0x0)) {
                for (uStack14 = 0x0; uVar15 = ((u32)paVar12 >> 0x10), uStack14 < local_20[0];
                    uStack14 += 0x1) {
                  local_3e[0] = _PTR_LOOP_1050_68a2;
                  paStack50 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
                  uVar11 = ((u32)paStack50 >> 0x10);
                  uVar5 = (astruct_388 *)paStack50;
                  paVar12 = (astruct_57 *)CONCAT22(uVar15,uVar11 | uVar5);
                  if ((uVar11 | uVar5) == 0x0) {
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
                  BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,local_28),0x2);
                  if (((BVar2 == 0x0) ||
                      (BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,local_24),0x2), BVar2 == 0x0)) ||
                     ((BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_2e),0x2), BVar2 == 0x0 ||
                      ((BVar2 = read_file_1008_7dee(param_4,((u32)paStack44 & 0xffff0000 |
                                                                  (u32)((int)paStack44 + 0xa)),0x2), BVar2 == 0x0 ||
                       (BVar2 = read_file_1008_7dee(param_4,((u32)paStack44 & 0xffff0000 |
                                                                  (u32)((int)paStack44 + 0xc)),0x2), BVar2 == 0x0)))))
                     ) goto LAB_1030_77be;
                  uVar15 = ((u32)paStack44 >> 0x10);
                  iVar4 = (astruct_99 *)paStack44;
                  &iVar4->field2_0x4 = local_28[0];
                  ((int)&iVar4->field2_0x4 + 0x2) = local_24[0];
                  &iVar4->field3_0x8 = local_2e;
                  if (iVar3->field51_0x3a == NULL) {
                    uVar11 = local_2e;
                    mem_op_1000_179c(0xc,paVar12);
                    uVar9 = paVar12;
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
                BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,local_56),0x2);
                if (BVar2 != 0x0) {
                  uStack82 = 0x0;
                  while( true ) {
                    uVar15 = ((u32)paVar12 >> 0x10);
                    if (local_56[0] <= uStack82) {
                      return;
                    }
                    paStack44 = (astruct_99 *)_PTR_LOOP_1050_68a2;
                    paStack50 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
                    uVar10 = (astruct_99 *)((u32)paStack50 >> 0x10);
                    uVar8 = (astruct_99 *)paStack50;
                    paVar12 = (astruct_57 *)CONCAT22(uVar15,uVar10 | uVar8);
                    if ((uVar10 | uVar8) == 0x0) {
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
                    BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,local_46),0x2);
                    if ((((BVar2 == 0x0) ||
                         (BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,local_42),0x2), BVar2 == 0x0)) ||
                        (BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,local_3e),0x2), BVar2 == 0x0)) ||
                       ((BVar2 = read_file_1008_7dee(param_4,((u32)paStack74 & 0xffff0000 |
                                                                   (u32)((int)paStack74 + 0xa)),0x2), BVar2 == 0x0 ||
                        (BVar2 = read_file_1008_7dee(param_4,((u32)paStack74 & 0xffff0000 |
                                                                   (u32)((int)paStack74 + 0xc)),0x2), BVar2 == 0x0))))
                    break;
                    uVar14 = (astruct_99 *)((u32)paStack74 >> 0x10);
                    iVar5 = (astruct_99 *)paStack74;
                    &iVar5->field2_0x4 = local_46[0];
                    ((int)&iVar5->field2_0x4 + 0x2) = local_42[0];
                    &iVar5->field3_0x8 = local_3e[0];
                    if (iVar3->field52_0x3e == NULL) {
                      mem_op_1000_179c(0xc,paVar12);
                      uVar11 = paVar12;
                      paStack50 = (astruct_99 *)CONCAT22(uVar11,local_3e[0]);
                      paVar12 = (astruct_57 *)((u32)paVar12 & 0xffff0000 | (u32)(uVar11 | local_3e[0]));
                      if ((uVar11 | local_3e[0]) == 0x0) {
                        iVar3->field52_0x3e = NULL;
                      }
                      else {
                        set_struct_1008_574a((astruct_57 *)CONCAT22(uVar11,local_3e[0]));
                        &iVar3->field52_0x3e = local_3e[0];
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
    }//
LAB_1030_77be:
    u16_1050_0310 = 0x6d2;
  }
  return;
}



u16 pass1_1030_7bee(mut param_1: u32)

{
  code **ppcVar1;
  let mut in_AX: u16;
  let mut uVar2: u16;
  let mut in_DX: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  iVar3 = (int)param_1;
  if (*(i32 *)(iVar3 + 0x16) == 0x0) {
    return 0x0;
  }
  if (*(i32 *)(iVar3 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar4 << 0x10),in_AX,in_DX);
  }
  ppcVar1 = (code **)((int)(u32)(u32)(iVar3 + 0x1a) + 0x44);
  uVar2 = (**ppcVar1)();
  return uVar2;
}



pub fn pass1_1030_7c28(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 ) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u32;

  uVar1 = (param_3 >> 0x10);
  if (*(i32 *)((int)param_3 + 0x22) == 0x0) {
    return 0x0;
  }
  uVar2 = (u32)((int)param_3 + 0x22);
  uVar2 = pass1_1020_bae6(param_1,param_2,uVar2,CONCAT22(param_4,(int)((u32)uVar2 >> 0x10)));
  return uVar2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1030_7c50(mut param_1: u16 ,param_2: *mut astruct_57,param_3: *mut astruct_305,i32 param_4,mut param_5: i16)

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  astruct_57 *uVar4;
  let mut uVar3: u16;
  astruct_305 *iVar8;
  let mut uVar5: u16;
  let mut uVar6: u32;
  u32 *puVar7;
  let mut uVar8: u32;
  u32 *puStack18;

  uVar5 = ((u32)param_3 >> 0x10);
  iVar8 = (astruct_305 *)param_3;
  if (iVar8->field30_0x1e == NULL) {
    mem_op_1000_179c(0x18,param_2);
    uVar4 = (astruct_57 *)param_2;
    param_2 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)(uVar4 | param_1));
    if ((uVar4 | param_1) == 0x0) {
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
    uVar3 = uVar6;
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
pub fn pass1_1030_7d1c(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_397,mut param_4: u16 ,mut param_5: u32)

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar2;
  astruct_397 *iVar2;
  astruct_397 *uVar3;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar3 = (astruct_397 *)((u32)param_3 >> 0x10);
  iVar2 = (astruct_397 *)param_3;
  if (iVar2->field34_0x22 == NULL) {
    mem_op_1000_179c(0xa,paVar2);
    uVar1 = paVar2 | param_1;
    if (uVar1 == 0x0) {
      iVar2->field34_0x22 = NULL;
    }
    else {
      pass1_1020_ba3e((astruct_172 *)CONCAT22(paVar2,param_1),0xa,0x2);
      &iVar2->field34_0x22 = param_1;
      ((int)&iVar2->field34_0x22 + 0x2) = uVar1;
    }
  }
  pass1_1020_bb8a(iVar2->field34_0x22,param_4,param_5);
  return;
}
pub fn pass1_1030_7d7c(mut param_1: u16 ,u8 *param_2,param_3: *mut astruct_398,mut param_4: u16 ,mut param_5: u32)

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar2;
  astruct_398 *iVar2;
  astruct_398 *uVar3;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar3 = (astruct_398 *)((u32)param_3 >> 0x10);
  iVar2 = (astruct_398 *)param_3;
  if (iVar2->field38_0x26 == NULL) {
    mem_op_1000_179c(0xa,paVar2);
    uVar1 = paVar2 | param_1;
    if (uVar1 == 0x0) {
      iVar2->field38_0x26 = NULL;
    }
    else {
      pass1_1020_ba3e((astruct_172 *)CONCAT22(paVar2,param_1),0xa,0x2);
      &iVar2->field38_0x26 = param_1;
      ((int)&iVar2->field38_0x26 + 0x2) = uVar1;
    }
  }
  pass1_1020_bb8a(iVar2->field38_0x26,param_4,param_5);
  return;
}
pub fn pass1_1030_7ddc(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u32,i32 param_4,mut param_5: u16 )

{
  let mut uVar1: u32;
  astruct_57 *uVar2;
  let mut iVar2: i16;
  let mut uVar3: u16;
  i32 lVar4;

  uVar3 = (param_3 >> 0x10);
  iVar2 = (int)param_3;
  if (*(i32 *)(iVar2 + 0x22) == 0x0) {
    mem_op_1000_179c(0xa,param_2);
    uVar2 = (astruct_57 *)param_2;
    param_2 = (astruct_57 *)(u32)(uVar2 | param_1);
    if ((uVar2 | param_1) == 0x0) {
      (u32)(iVar2 + 0x22) = 0x0;
    }
    else {
      pass1_1020_ba3e((astruct_172 *)CONCAT22(uVar2,param_1),0xa,0x2);
      (iVar2 + 0x22) = param_1;
      (iVar2 + 0x24) = (int)param_2;
    }
  }
  uVar1 = (u32)(iVar2 + 0x22);
  lVar4 = pass1_1020_bae6(param_1,param_2,uVar1,CONCAT22(param_5,(int)((u32)uVar1 >> 0x10)));
  pass1_1020_bb8a(*(i32 **)(iVar2 + 0x22),(lVar4 + param_4),
                  CONCAT22(param_5,(int)((u32)(lVar4 + param_4) >> 0x10)));
  return;
}
pub fn pass1_1030_7e5a(mut param_1: u16 ,param_2: *mut astruct_358,mut param_3: u32)

{
  astruct_358 *pstruct_1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_2 >> 0x10);
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

BOOL16 pass1_1030_7ea0(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut BVar3: bool;

  uVar2 = pass1_1030_6fa0(param_1);
  BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,0xb);
  if (BVar3 != 0x0) {
    uVar1 = (u32)((int)param_1 + 0x1a);
    if (((int)uVar1 + 0x12) == 0x5) {
      return 0x1;
    }
    BVar3 = 0x0;
  }
  return BVar3;
}
pub fn pass1_1030_7eda(mut param_1: u32,mut param_2: u16 )

{
  let mut in_DX: u16;
  let mut uVar1: u16;
  let mut local_c: u16;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut uStack6: u16;
  let mut uStack4: u16;

  local_c = 0x0;
  uStack10 = 0x0;
  uStack6 = 0x0;
  uStack4 = 0x0;
  uStack8 = param_2;
  uVar1 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar1 << 0x10),param_2,in_DX);
  }
  pass1_1028_bb96(*(astruct_295 **)((int)param_1 + 0x1a),(u32 *)&local_c,&DAT_1050_1050);
  return;
}
pub fn pass1_1030_7f1a(mut param_1: u32,mut param_2: u16 )

{
  let mut in_DX: u16;
  let mut uVar1: u16;
  let mut local_c: u16;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut uStack6: u16;
  let mut uStack4: u16;

  local_c = 0x0;
  uStack8 = 0x0;
  uStack6 = 0x0;
  uStack4 = 0x0;
  uStack10 = param_2;
  uVar1 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar1 << 0x10),param_2,in_DX);
  }
  pass1_1028_bb96(*(astruct_295 **)((int)param_1 + 0x1a),(u32 *)&local_c,&DAT_1050_1050);
  return;
}



u16 pass1_1030_7f5a(mut param_1: u32)

{
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut uVar1: u16;
  let mut uVar2: u32;

  uVar1 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar1 << 0x10),in_AX,in_DX);
  }
  uVar2 = pass1_1028_bb6a((u32)((int)param_1 + 0x1a));
  if (uVar2 != 0x0) {
    return ((int)uVar2 + 0x4);
  }
  return 0x0;
}



u16 pass1_1030_7f98(mut param_1: u32)

{
  let mut in_AX: u16;
  let mut in_DX: u16;
  let mut uVar1: u16;
  let mut uVar2: u32;

  uVar1 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x1a) == 0x0) {
    struct_op_1030_73a8((astruct_419 *)(param_1 & 0xffff | (u32)uVar1 << 0x10),in_AX,in_DX);
  }
  uVar2 = pass1_1028_bb6a((u32)((int)param_1 + 0x1a));
  if (uVar2 != 0x0) {
    return ((int)uVar2 + 0x2);
  }
  return 0x0;
}
pub fn pass1_1030_7fd6(mut param_1: u16 ,mut param_2: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u32;
  let mut in_AX: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u32;

  uVar4 = (param_2 >> 0x10);
  iVar3 = (int)param_2;
  if (*(i32 *)(iVar3 + 0x1a) == 0x0) {
    uVar5 = struct_op_1030_73a8((astruct_419 *)(param_2 & 0xffff | (u32)uVar4 << 0x10),in_AX,param_1);
    param_1 = (uVar5 >> 0x10);
  }
  uVar2 = (u32)(iVar3 + 0x1a);
  iVar1 = ((int)uVar2 + 0xc);
  if (((0x32 < iVar1) && (!SBORROW2(iVar1,0x33))) &&
     ((iVar1 == 0x34 || iVar1 + -0x33 < 0x1 || ((0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 0x2)))))) {
    pass1_1028_1416(param_1,(u32)(iVar3 + 0x1a));
  }
  return;
}
