
fn file_1038_774e(param_1: u32,param_2: u32,uchar *param_3,param_4: u16)
{
  let uVar1: u16;
  astruct_307 *iVar2;
  let BVar2: bool;
  let iVar3: i16;
  let uVar4: u16;
  let uVar6: u16;
  let local_8: u16;
  let local_6: u16;
  let local_4: u16;
  let puVar5: u32;
  
  if (ctx.PTR_LOOP_1050_0312 < 0x2) {
    return;
  }
  iVar2 = (astruct_307 *)param_1;
  iVar2 = (astruct_307 *)&iVar2->field_0x4;
  puVar5 = (param_1 & 0xffff0000 | ZEXT24(iVar2));
  pass1_1008_766e(param_2,puVar5,param_4,0x1008,param_3);
  if (puVar5 != 0x0) {
    uVar1 = (param_1 >> 0x10);
    uVar4 = param_2;
    uVar6 = (param_2 >> 0x10);
    BVar2 = read_file_1008_7dee(uVar4,uVar6,&iVar2->field_0x8,0x0,uVar1,0x4,0x1008);
    if ((((((BVar2 != 0x0) &&
           (iVar3 = file_1008_77cc(param_2,(long *)(param_1 & 0xffff0000 |
                                                   &iVar2->field_0xe),param_3
                                   ,0x1008,param_4), iVar3 != 0x0)) &&
          (BVar2 = read_file_1008_7dee(uVar4,uVar6,&local_4,0x0,param_4,0x2,0x1008
                                      ), BVar2 != 0x0)) &&
         ((BVar2 = read_file_1008_7dee(uVar4,uVar6,&local_6,0x0,param_4,0x2,0x1008
                                      ), BVar2 != 0x0 &&
          (BVar2 = read_file_1008_7dee(uVar4,uVar6,&local_8,0x0,param_4,0x2,0x1008
                                      ), BVar2 != 0x0)))) &&
        ((BVar2 = read_file_1008_7dee(uVar4,uVar6,&iVar2->field_0x16,0x0,uVar1,0x4,0x1008)
         , BVar2 != 0x0 &&
         ((BVar2 = read_file_1008_7bc8(param_2,
                                               (param_1 & 0xffff0000 |
                                               &iVar2->field_0x1a),0x1008,
                                       param_4), BVar2 != 0x0 &&
          (BVar2 = read_file_1008_7dee(uVar4,uVar6,&iVar2->field_0x20,0x0,uVar1,0x4,0x1008
                                      ), BVar2 != 0x0)))))) &&
       ((BVar2 = read_file_1008_7dee(uVar4,uVar6,&iVar2->field_0x24,0x0,uVar1,0x2,0x1008),
        BVar2 != 0x0 &&
        ((BVar2 = read_file_1008_7dee(uVar4,uVar6,&iVar2->field_0x26,0x0,uVar1,0x2,0x1008)
         , BVar2 != 0x0 &&
         (BVar2 = read_file_1008_7dee(uVar4,uVar6,&iVar2->field_0x28,0x0,uVar1,0x2,0x1008)
         , BVar2 != 0x0)))))) {
      iVar2->field_0xc = local_4;
      uVar4 = switch_1008_72bc(uVar4,uVar6,local_6);
      iVar2->field_0x12 = uVar4;
      iVar2->field_0x14 = local_8;
      return;
    }
  }
  ctx.PTR_LOOP_1050_0310 = 0x6d2;
  return;
}


fn read_file_1038_7c02(param_1: *mut u32,param_2: u32,param_3: u16,
                   param_4: u16) -> u16

{
  code **ppcVar1;
  let BVar2: bool;
  let uVar3: u16;
  let uVar4: u16;
  let extraout_DX: *mut u8
  let puVar5: *mut u8
  let extraout_DX_00: *mut u8
  u16_t unaff_SS;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u32;
  let uVar10: u16;
  let local_12: [u16;0x2];
  let uStack14: u32;
  let local_4: u16;
  
  if (ctx.PTR_LOOP_1050_0312 < 0x2) {
    return 0x1;
  }
  uVar6 = param_2;
  uVar8 = (param_2 >> 0x10);
  read_file_1008_7cfe(uVar6,uVar8,0x17,0x1008,unaff_SS);
  if ((param_3 != 0x0) &&
     (BVar2 = read_file_1008_7dee(uVar6,uVar8,&local_4,0x0,unaff_SS,0x2,0x1008),
     BVar2 != 0x0)) {
    while (local_4 != 0x0) {
      uVar7 = 0x2a;
      uVar3 = local_4;
      local_4 = local_4 - 0x1;
      uVar9 = param_2;
      mem_op_1000_179c(0x2a,param_4,0x1000);
      puVar5 = (param_4 | uVar3);
      if (puVar5 == 0x0) {
        uVar3 = 0x0;
        puVar5 = 0x0;
      }
      else {
        struct_1038_6520(CONCAT22(param_4,uVar3));
      }
      uVar10 = (uVar9 >> 0x10);
      uStack14 = CONCAT22(puVar5,uVar3);
      file_1038_774e(CONCAT22(puVar5,uVar3),CONCAT22(uVar9,uVar7),puVar5,unaff_SS);
      if (uVar3 == 0x0) {
        return 0x0;
      }
      ppcVar1 = (*param_1 + 0x4);
      (**ppcVar1)(0x1000,*param_1,(*param_1 >> 0x10),uStack14,
                  (uStack14 >> 0x10),uVar10);
      param_4 = (u16_t)extraout_DX;
    }
    local_4 = local_4 - 0x1;
    BVar2 = read_file_1008_7dee(uVar6,uVar8,local_12,0x0,unaff_SS,0x2,0x1008);
    if (BVar2 != 0x0) {
      while( true ) {
        if (local_12[0] == 0x0) {
          return 0x1;
        }
        uVar7 = 0x14;
        uVar3 = local_12[0];
        local_12[0] = local_12[0] - 0x1;
        uVar9 = param_2;
        mem_op_1000_179c(0x14,param_4,0x1000);
        puVar5 = (param_4 | uVar3);
        if (puVar5 == 0x0) {
          uVar3 = 0x0;
          puVar5 = 0x0;
        }
        else {
          pass1_1030_ae6c(CONCAT22(param_4,uVar3));
        }
        uVar10 = (uVar9 >> 0x10);
        uVar4 = uVar3;
        file_1030_b836(CONCAT22(puVar5,uVar3),CONCAT22(uVar9,uVar7),puVar5,unaff_SS);
        if (uVar4 == 0x0) break;
        uVar7 = (param_1 >> 0x10);
        uVar9 = (param_1 + 0x4);
        ppcVar1 = ((param_1 + 0x4) + 0x4)
        ;
        (**ppcVar1)(0x1030,uVar9,(uVar9 >> 0x10),uVar3,puVar5,uVar10);
        param_4 = (u16_t)extraout_DX_00;
      }
      return 0x0;
    }
  }
  return 0x0;
}

