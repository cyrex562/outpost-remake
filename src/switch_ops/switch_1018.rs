
pub fn switch_1018_3b9e(param_1: u32,param_2: u16,param_3: u16,param_4: u16,param_5: u16) -> u32

{
  let uVar1: u32;
  let iVar2: &mut Struct263;
  let uVar2: u16;
  let uStack14: u32;
  let uStack6: u16;
  let uStack4: u16;
  
  uStack6 = 0x0;
  uStack4 = 0x0;
 // uVar2 = (param_1 >> 0x10);
  iVar2 = param_1;
  uVar1 = iVar2.field_0x122;
  pass1_1008_e852(uVar1,(uVar1 >> 0x10),iVar2.field_0x126,param_5,
                  param_4);
  pass1_1030_8344(ctx.PTR__LOOP_1050_5748,(ctx.PTR__LOOP_1050_5748 >> 0x10)
                  ,CONCAT22(param_4,param_3));
  uStack14 = CONCAT22(param_4,param_3);
  switch(param_2) {
  0x188 =>
    if (iVar2.field_0xa == 0x0) {
      pass1_1008_d3ae(param_1 & 0xffff | uVar2 << 0x10);
    }
    uStack6 = &iVar2.field_0xa;
    uStack4 = (&iVar2.field_0xa + 0x2);
    break;
  0x189 =>
    if (iVar2.field_0xe == 0x0) {
      unk_str_op_1008_d4f6(param_1 & 0xffff | uVar2 << 0x10,uStack14);
    }
    uStack6 = &iVar2.field_0xe;
    uStack4 = (&iVar2.field_0xe + 0x2);
    break;
  0x18a =>
    if (iVar2.field_0x12 == 0x0) {
      unk_str_op_1008_d1c6(param_1 & 0xffff | uVar2 << 0x10,uStack14);
    }
    uStack6 = &iVar2.field_0x12;
    uStack4 = (&iVar2.field_0x12 + 0x2);
    break;
  0x18b =>
    if (iVar2.field_0x16 == 0x0) {
      pass1_1008_cfa0(param_1 & 0xffff | uVar2 << 0x10,uStack14);
    }
    uStack6 = &iVar2.field_0x16;
    uStack4 = (&iVar2.field_0x16 + 0x2);
    break;
  0x18c =>
    if (iVar2.field_0x1a == 0x0) {
      pass1_1008_cda2(param_1 & 0xffff | uVar2 << 0x10,uStack14,param_5);
    }
    uStack6 = &iVar2.field_0x1a;
    uStack4 = (&iVar2.field_0x1a + 0x2);
    break;
  0x18d =>
    if (iVar2.field_0x1e == 0x0) {
      pass1_1008_cbc4(param_1 & 0xffff | uVar2 << 0x10,uStack14,param_5);
    }
    uStack6 = &iVar2.field_0x1e;
    uStack4 = (&iVar2.field_0x1e + 0x2);
  }
  return CONCAT22(uStack4,uStack6);
}


pub fn switch_1018_3ee6(param_1: u32,param_2: i32,param_3: i16,param_4: u16,param_5: U32Ptr)
{
  let i_var1: i16;
  let mut pcVar2: String; 
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u32;
  let puVar8: U32Ptr;
  let unaff_SS: u16;
  let puVar9: U32Ptr;
  let lVar10: i32;
  let iVar11: i16;
  let IVar12: i16;
  let uVar13: u16;
  let uStack26: u32;
  let puStack22: U32Ptr;
  let lStack18: i32;
  let lStack14: i32;
  let uStack10: i16;
  let uStack8: u16;
  let piStack6: U32Ptr;
  
  if (false) {
switchD_1018_3f8b_caseD_2:
    i_var1 = param_3 * 0x4 + 0x40ce;
  }
  else {
    switch(param_4) {
    0x1 =>
      i_var1 = param_3 * 0x4 + 0x40b6;
      break;
    default:
//       TODO: goto switchD_1018_3f8b_caseD_2;
    0x3 =>
      i_var1 = param_3 * 0x4 + 0x40e2;
      break;
    0x4 =>
      i_var1 = param_3 * 0x4 + 0x40ee;
      break;
    0x8 =>
      i_var1 = param_3 * 0x4 + 0x40f2;
      break;
    0x9 =>
      i_var1 = param_3 * 0x4 + 0x4106;
      break;
    0xa =>
      i_var1 = param_3 * 0x4 + 0x410a;
      break;
    0x14 =>
      i_var1 = param_3 * 0x4 + 0x410e;
      break;
    0x16 =>
      i_var1 = param_3 * 0x4 + 0x4112;
      break;
    0x17 =>
      i_var1 = param_3 * 0x4 + 0x4116;
      break;
    0x19 =>
      i_var1 = param_3 * 0x4 + 0x411a;
    }
  }
  piStack6 = CONCAT22(0x1050,i_var1);
  if (piStack6 == 0x0) {
    return;
  }
  iStack10 = 0x0;
  uStack8 = 0x0;
  iVar11 = *piStack6;
  uVar13 = param_1;
 // uVar3 = (param_1 >> 0x10);
  if (iVar11 == 0x1) {
    uVar13 = pass1_1018_456a(uVar13,uVar3,(i_var1 + 0x2));
    lStack14 = CONCAT22(param_5,uVar13);
    pcVar2 = string_1020_c0d8((i_var1 + 0x2));
    uVar3 = str_op_1008_60e8(CONCAT22(param_5,pcVar2),param_5);
    puVar8 = param_5;
    uVar13 = uVar3;
    mem_op_1000_179c(0x10,param_5,0x1000);
    puStack22 = CONCAT22(puVar8,uVar13);
    if ((puVar8 | uVar13) != 0x0) {
      lVar10 = param_2 / lStack14;
      uStack8 = (param_2 % lStack14);
      struct_1018_4790(puStack22,lVar10,CONCAT22(param_5,uVar3),(i_var1 + 0x2));
      iStack10 = lVar10;
//       TODO: goto LAB_1018_425e;
    }
  }
  else {
    if (iVar11 == 0x2) {
      uVar13 = pass1_1018_451e(uVar13,uVar3,(i_var1 + 0x2));
      lStack18 = CONCAT22(param_5,uVar13);
      pcVar2 = string_op_1020_c222((i_var1 + 0x2));
      uVar3 = str_op_1008_60e8(CONCAT22(param_5,pcVar2),param_5);
      puVar8 = param_5;
      uVar13 = uVar3;
      mem_op_1000_179c(0x10,param_5,0x1000);
      puStack22 = CONCAT22(puVar8,uVar13);
      if ((puVar8 | uVar13) != 0x0) {
        puVar9 = struct_1018_48b0(puStack22,param_2 / lStack18,CONCAT22(param_5,uVar3),
                                  (i_var1 + 0x2));
       // uStack8 = (puVar9 >> 0x10);
        iStack10 = puVar9;
//         TODO: goto LAB_1018_425e;
      }
    }
    else {
      if (iVar11 != 0x3) {
        if (iVar11 != 0x4) {
            // goto
            // LAB_1018_425e;
        }
        i_var1 = (i_var1 + 0x2);
        uVar5 = i_var1 - 0x1;
        iVar11 = ctx._PTR_LOOP_1050_14cc;
        IVar12 = (ctx.PTR__LOOP_1050_14cc >> 0x10);
        if (uVar5 == 0x0) {
          load_string_1010_84ac(iVar11,IVar12,0x1010);
          uVar6 = uVar5;
          puVar8 = param_5;
          mem_op_1000_179c(0x14,param_5,0x1000);
          puStack22 = CONCAT22(puVar8,uVar6);
          if ((puVar8 | uVar6) == 0x0) {
              // goto
              // LAB_1018_40bc;
          }
          uVar13 = 0x2;
          lVar10 = 0x14;
        }
        else {
          uVar5 = i_var1 - 0x2;
          if (uVar5 == 0x0) {
            load_string_1010_84ac(iVar11,IVar12,0x1010);
            uVar6 = uVar5;
            puVar8 = param_5;
            mem_op_1000_179c(0x14,param_5,0x1000);
            puStack22 = CONCAT22(puVar8,uVar6);
            if ((puVar8 | uVar6) == 0x0) {
                // goto
                // LAB_1018_40bc;
            }
            uVar13 = 0x3;
            lVar10 = 0x16;
          }
          else {
            uVar5 = i_var1 - 0x3;
            if (uVar5 == 0x0) {
              load_string_1010_84ac(iVar11,IVar12,0x1010);
              uVar6 = uVar5;
              puVar8 = param_5;
              mem_op_1000_179c(0x14,param_5,0x1000);
              puStack22 = CONCAT22(puVar8,uVar6);
              if ((puVar8 | uVar6) == 0x0) {
                  // goto
                  // LAB_1018_40bc;
              }
              uVar13 = 0x4;
              lVar10 = 0x17;
            }
            else {
              uVar5 = i_var1 - 0x4;
              if (uVar5 != 0x0) {
                  // goto
                  // LAB_1018_425e;
              }
              load_string_1010_84ac(iVar11,IVar12,0x1010);
              uVar6 = uVar5;
              puVar8 = param_5;
              mem_op_1000_179c(0x14,param_5,0x1000);
              puStack22 = CONCAT22(puVar8,uVar6);
              if ((puVar8 | uVar6) == 0x0) {
                  // goto
                  // LAB_1018_40bc;
              }
              uVar13 = 0x4;
              lVar10 = 0xa;
            }
          }
        }
        puVar9 = struct_1018_4842(puStack22,param_2 / lVar10,CONCAT22(param_5,uVar5),
                                  uVar13);
       // uStack8 = (puVar9 >> 0x10);
        iStack10 = puVar9;
//         TODO: goto LAB_1018_425e;
      }
      uVar4 = pass1_1008_c646(ctx.PTR__LOOP_1050_06e0,
                              CONCAT22((i_var1 + 0x2),
                                       (ctx.PTR__LOOP_1050_06e0 >> 0x10)),unaff_SS
                             );
      if (uVar4 == 0x0) {
        uVar4 = 0x4f;
      }
      uVar13 = switch_1018_43ec(uVar13,uVar3,uVar4);
      lStack14 = CONCAT22(param_5,uVar13);
      uVar13 = pass1_1020_bd80(uVar4);
      uVar5 = str_op_1008_60e8(CONCAT22(param_5,uVar13),param_5);
      uStack26 = CONCAT22(param_5,uVar5);
      mem_op_1000_179c(0x14,param_5,0x1000);
      puStack22 = CONCAT22(param_5,uVar5);
      if ((param_5 | uVar5) != 0x0) {
        uVar7 = param_2 / lStack14;
        uStack8 = (param_2 % lStack14);
        struct_1018_47c8(puStack22,uVar7,uStack26,uVar4,0x0);
        iStack10 = uVar7;
//         TODO: goto LAB_1018_425e;
      }
    }
  }
//LAB_1018_40bc:
  iStack10 = 0x0;
  uStack8 = 0x0;
//LAB_1018_425e:
  if ((iStack10 + 0x8) == 0x0) {
    (iStack10 + 0x8) = 0x1;
  }
  return;
}


pub fn switch_1018_43ec(param_1: u16,param_2: u16,param_3: u16) -> u16

{
  let uStack6: u16;
  
  if (false) {
switchD_1018_444f_caseD_10:
    uStack6 = 0x1;
  }
  else {
    switch(param_3) {
    0xf =>
    0x35 =>
    0x36 =>
      uStack6 = 0x7;
      break;
    default:
//       TODO: goto switchD_1018_444f_caseD_10;
    0x11 =>
    0x13 =>
    0x14 =>
    0x15 =>
    0x2d =>
    0x2e =>
    0x6e =>
      uStack6 = 0x9;
      break;
    0x12 =>
    0x31 =>
    0x32 =>
    0x52 =>
    0x53 =>
    0x54 =>
    0x55 =>
    0x56 =>
    0x5a =>
    0x5b =>
    0x5c =>
    0x5d =>
    0x5e =>
    0x5f =>
      uStack6 = 0x4;
      break;
    0x1b =>
    0x1c =>
    0x1d =>
    0x28 =>
    0x29 =>
    0x2c =>
    0x2f =>
    0x30 =>
    0x68 =>
    0x69 =>
      uStack6 = 0x5;
      break;
    0x1e =>
    0x1f =>
    0x20 =>
    0x33 =>
    0x34 =>
      uStack6 = 0x6;
      break;
    0x22 =>
    0x23 =>
    0x24 =>
      uStack6 = 0x8;
      break;
    0x25 =>
    0x26 =>
    0x27 =>
      uStack6 = 0x2;
      break;
    0x38 =>
    0x39 =>
    0x4f =>
    0x50 =>
    0x51 =>
    0x57 =>
    0x58 =>
    0x59 =>
    0x66 =>
    0x67 =>
    0x6c =>
    0x6d =>
      uStack6 = 0x3;
    }
  }
  return uStack6;
}

