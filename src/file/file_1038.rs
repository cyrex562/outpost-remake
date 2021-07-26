
pub fn file_1038_774e(param_1: u32,param_2: u32,param_3: U32Ptr,param_4: u16)
{
  let u_var1: u16;
  let i_var2: &mut Struct307;
  let BVar2: bool;
  let iVar3: i16;
  let u_var4: u16;
  let u_var6: u16;
  let local_8: u16;
  let local_6: u16;
  let local_4: u16;
  let pu_var5: u32;
  
  if (ctx.PTR_LOOP_1050_0312 < 0x2) {
    return;
  }
  i_var2 = param_1;
  i_var2 = &i_var2.field_0x4;
  pu_var5 = (param_1 & 0xffff0000 | ZEXT24(i_var2));
  pass1_1008_766e(param_2,pu_var5,param_4,0x1008,param_3);
  if (pu_var5 != 0x0) {
   // u_var1 = (param_1 >> 0x10);
    u_var4 = param_2;
   // u_var6 = (param_2 >> 0x10);
    BVar2 = read_file_1008_7dee(u_var4,u_var6,&i_var2.field_0x8,0x0,u_var1,0x4,0x1008);
    if ((((((BVar2 != 0x0) &&
           (iVar3 = file_1008_77cc(param_2,(param_1 & 0xffff0000 |
                                                   &i_var2.field_0xe),param_3
                                   ,0x1008,param_4), iVar3 != 0x0)) &&
          (BVar2 = read_file_1008_7dee(u_var4,u_var6,&local_4,0x0,param_4,0x2,0x1008
                                      ), BVar2 != 0x0)) &&
         ((BVar2 = read_file_1008_7dee(u_var4,u_var6,&local_6,0x0,param_4,0x2,0x1008
                                      ), BVar2 != 0x0 &&
          (BVar2 = read_file_1008_7dee(u_var4,u_var6,&local_8,0x0,param_4,0x2,0x1008
                                      ), BVar2 != 0x0)))) &&
        ((BVar2 = read_file_1008_7dee(u_var4,u_var6,&i_var2.field_0x16,0x0,u_var1,0x4,0x1008)
         , BVar2 != 0x0 &&
         ((BVar2 = read_file_1008_7bc8(param_2,
                                               (param_1 & 0xffff0000 |
                                               &i_var2.field_0x1a),0x1008,
                                       param_4), BVar2 != 0x0 &&
          (BVar2 = read_file_1008_7dee(u_var4,u_var6,&i_var2.field_0x20,0x0,u_var1,0x4,0x1008
                                      ), BVar2 != 0x0)))))) &&
       ((BVar2 = read_file_1008_7dee(u_var4,u_var6,&i_var2.field_0x24,0x0,u_var1,0x2,0x1008),
        BVar2 != 0x0 &&
        ((BVar2 = read_file_1008_7dee(u_var4,u_var6,&i_var2.field_0x26,0x0,u_var1,0x2,0x1008)
         , BVar2 != 0x0 &&
         (BVar2 = read_file_1008_7dee(u_var4,u_var6,&i_var2.field_0x28,0x0,u_var1,0x2,0x1008)
         , BVar2 != 0x0)))))) {
      i_var2.field_0xc = local_4;
      u_var4 = switch_1008_72bc(u_var4,u_var6,local_6);
      i_var2.field_0x12 = u_var4;
      i_var2.field_0x14 = local_8;
      return;
    }
  }
  ctx.PTR_LOOP_1050_0310 = 0x6d2;
  return;
}


pub fn read_file_1038_7c02(param_1: U32Ptr,param_2: u32,param_3: u16,
                   param_4: u16) -> u16

{
  let ppcVar1: u32;
  let BVar2: bool;
  let u_var3: u16;
  let u_var4: u16;
  let extraout_dx: U32Ptr;
  let pu_var5: U32Ptr;
  let extraout_DX_00: U32Ptr;
  let unaff_SS: u16;
  let u_var6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u32;
  let u_var10: u16;
  let local_12: [u16;0x2];
  let uStack14: u32;
  let local_4: u16;
  
  if (ctx.PTR_LOOP_1050_0312 < 0x2) {
    return 0x1;
  }
  u_var6 = param_2;
 // uVar8 = (param_2 >> 0x10);
  read_file_1008_7cfe(u_var6,uVar8,0x17,0x1008,unaff_SS);
  if ((param_3 != 0x0) &&
     (BVar2 = read_file_1008_7dee(u_var6,uVar8,&local_4,0x0,unaff_SS,0x2,0x1008),
     BVar2 != 0x0)) {
    while (local_4 != 0x0) {
      uVar7 = 0x2a;
      u_var3 = local_4;
      local_4 = local_4 - 0x1;
      uVar9 = param_2;
      mem_op_1000_179c(0x2a,param_4,0x1000);
      pu_var5 = (param_4 | u_var3);
      if (pu_var5 == 0x0) {
        u_var3 = 0x0;
        pu_var5 = 0x0;
      }
      else {
        struct_1038_6520(CONCAT22(param_4,u_var3));
      }
     // u_var10 = (uVar9 >> 0x10);
      uStack14 = CONCAT22(pu_var5,u_var3);
      file_1038_774e(CONCAT22(pu_var5,u_var3),CONCAT22(uVar9,uVar7),pu_var5,unaff_SS);
      if (u_var3 == 0x0) {
        return 0x0;
      }
      ppcVar1 = (*param_1 + 0x4);
      (**ppcVar1)(0x1000,*param_1,(*param_1 >> 0x10),uStack14,
                  (uStack14 >> 0x10),u_var10);
      param_4 = extraout_dx;
    }
    local_4 = local_4 - 0x1;
    BVar2 = read_file_1008_7dee(u_var6,uVar8,local_12,0x0,unaff_SS,0x2,0x1008);
    if (BVar2 != 0x0) {
      loop {
        if (local_12[0] == 0x0) {
          return 0x1;
        }
        uVar7 = 0x14;
        u_var3 = local_12[0];
        local_12[0] = local_12[0] - 0x1;
        uVar9 = param_2;
        mem_op_1000_179c(0x14,param_4,0x1000);
        pu_var5 = (param_4 | u_var3);
        if (pu_var5 == 0x0) {
          u_var3 = 0x0;
          pu_var5 = 0x0;
        }
        else {
          pass1_1030_ae6c(CONCAT22(param_4,u_var3));
        }
       // u_var10 = (uVar9 >> 0x10);
        u_var4 = u_var3;
        file_1030_b836(CONCAT22(pu_var5,u_var3),CONCAT22(uVar9,uVar7),pu_var5,unaff_SS);
        if (u_var4 == 0x0) { break; }
       // uVar7 = (param_1 >> 0x10);
        uVar9 = (param_1 + 0x4);
        ppcVar1 = ((param_1 + 0x4) + 0x4)
        ;
        (**ppcVar1)(0x1030,uVar9,(uVar9 >> 0x10),u_var3,pu_var5,u_var10);
        param_4 = extraout_DX_00;
      }
      return 0x0;
    }
  }
  return 0x0;
}

