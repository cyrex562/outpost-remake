
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_5050(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;

  uVar7 = (param_3 >> 0x10);
  iVar6 = param_3;
  if ((iVar6 + 0xc) == 0) {
    param_1 = 0;
    uVar4 = 0;
  }
  else {
    ppcVar1 = ((iVar6 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar4 = extraout_DX;
  }
  uStack10 = CONCAT22(uVar4,param_1);
  for (uStack14 = 0; uStack14 < uStack10; uStack14 += 1) {
    uVar3 = uStack10;
    pass1_1030_1d58((iVar6 + 0xc));
    uVar5 = uVar4 | uVar3;
    if (uVar5 != 0) {
      uVar2 = pass1_1030_6fa0(uVar3 & 0xffff | uVar4 << 0x10);
      pass1_1008_c6ae(_u16_1050_06e0,uVar2,param_4);
    }
    uVar4 = uVar5;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_50e0(mut param_1: u16 ,mut param_2: u32,mut param_3: u16 )

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar8: u32;
  let mut uStack14: u32;
  let mut uStack10: u32;

  uVar7 = (param_2 >> 0x10);
  iVar6 = param_2;
  if ((iVar6 + 0xc) == 0) {
    param_1 = 0;
    uVar4 = 0;
  }
  else {
    ppcVar1 = ((iVar6 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar4 = extraout_DX;
  }
  uStack10 = CONCAT22(uVar4,param_1);
  for (uStack14 = 0; uStack14 < uStack10; uStack14 += 1) {
    uVar8 = uStack10;
    pass1_1030_1d58((iVar6 + 0xc));
    uVar5 = uVar4 | uVar8;
    if (uVar5 != 0) {
      uVar2 = pass1_1030_6fa0(uVar8 & 0xffff | uVar4 << 0x10);
      BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,param_3);
      if (BVar3 != 0) {
        uVar8 = struct_op_1030_73a8((uVar8 & 0xffff | uVar4 << 0x10),BVar3,uVar5);
        uVar5 = (uVar8 >> 0x10);
      }
    }
    uVar4 = uVar5;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_518c(mut param_1: u16 ,mut param_2: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u32;
  let mut ppcVar3: *mut *mut code;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar6: u16;
  let mut iVar7: i16;
  let mut iVar8: i16;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut unaff_CS: u8;
  let mut bVar12: bool;
  let mut uVar13: u32;
  let mut iStack34: i16;
  let mut uStack32: u32;
  let mut puStack28: *mut u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar10 = (param_2 >> 0x10);
  iVar7 = param_2;
  if ((iVar7 + 0x206) == 0) {
    if ((iVar7 + 0xc) == 0) {
      param_1 = 0;
      uVar11 = 0;
    }
    else {
      ppcVar3 = ((iVar7 + 0xc) + 0x10);
      (**ppcVar3)();
      uVar11 = extraout_DX;
    }
    uStack6 = CONCAT22(uVar11,param_1);
    for (uStack10 = 0; uStack10 < uStack6; uStack10 += 1) {
      uVar2 = (iVar7 + 0xc);
      ppcVar3 = ((iVar7 + 0xc) + 0x4);
      uVar5 = uStack6;
      (**ppcVar3)(unaff_CS,uVar2,(uVar2 >> 0x10),uStack10,(uStack10 >> 0x10));
      uVar4 = uVar5;
      uVar6 = extraout_DX_00 | uVar4;
      if (uVar6 != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5 & 0xffff | extraout_DX_00 << 0x10);
        unaff_CS = 0x30;
        uVar13 = struct_op_1030_73a8(CONCAT22(uVar6,uVar4),uVar4,uVar6);
        uVar6 = (uVar13 >> 0x10);
        iVar8 = (uVar13 + 0x12);
        uVar4 = uVar13 + 0x14;
        uVar5 = uVar4;
        puStack28 = (uVar13 & 0xffff0000 | uVar4);
        uStack32 = 0;
        if ((iVar8 == 0x4) || (iVar8 == 0x5)) {
          uVar5 = *puStack28;
          uStack32 = uVar5;
        }
        if (uStack32 != 0) {
          for (iStack34 = 0x11; iStack34 < 0x25; iStack34 += 1) {
            if ((((iVar7 + 0x204) == 0) || (iStack34 == 0x23)) || (iStack34 == 0x24)) {
              empty_1038_540a();
              iVar8 = iStack34 * 0x4;
              uVar11 = (uStack32 >> 0x10);
              iVar9 = uStack32;
              puVar1 = (iVar8 + iVar9 + 2);
              bVar12 = *puVar1 < uVar6;
              if ((bVar12 || *puVar1 == uVar6) &&
                 ((bVar12 || (puVar1 = (iVar8 + iVar9), *puVar1 < uVar5 || *puVar1 == uVar5)))) {
                pass1_1038_5770(param_2,(iVar8 + iVar9),iStack34);
              }
            }
          }
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_52b8(mut param_1: u32,mut param_2: u32,mut param_3: u16 )

{
  let mut puVar1: *mut u32;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut extraout_DX: u16;
  let mut uVar6: u16;
  let mut extraout_DX_00: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut unaff_CS: u16;
  let mut uVar9: u32;
  let mut uVar10: u16;
  let mut uStack26: u16;
  let mut iStack24: i16;
  paStack22: *mut astruct_397;
  let mut uStack14: u32;
  let mut uStack10: u32;
  iVar10: *mut astruct_601;

  iVar4 = -param_2;
  iVar10 = param_1;
  pass1_1038_5694(param_1,CONCAT22(-(param_2 + (param_2 != 0)),iVar4),param_3);
  if (param_3 != 0x24) {
    uVar8 = (param_1 >> 0x10);
    if (iVar10.field12_0xc.is_null()) {
      iVar4 = 0;
      uVar6 = 0;
    }
    else {
      ppcVar2 = (*iVar10.field12_0xc + 0x10);
      (**ppcVar2)();
      uVar6 = extraout_DX;
    }
    uStack10 = CONCAT22(uVar6,iVar4);
    for (uStack14 = 0; uVar3 = param_2, uStack14 < uStack10; uStack14 += 1) {
      puVar1 = iVar10.field12_0xc;
      ppcVar2 = (*iVar10.field12_0xc + 0x4);
      uVar9 = uStack10;
      (**ppcVar2)(unaff_CS,puVar1,(puVar1 >> 0x10),uStack14,(uStack14 >> 0x10));
      uVar5 = uVar9;
      uVar7 = extraout_DX_00 | uVar5;
      if (uVar7 != 0) {
        uVar10 = param_3;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar9 & 0xffff | extraout_DX_00 << 0x10);
        paStack22 = CONCAT22(uVar7,uVar5);
        unaff_CS = 0x1030;
        uVar9 = pass1_1030_7c28(uVar5,uVar7,CONCAT22(uVar7,uVar5),uVar10);
        uVar7 = (uVar9 >> 0x10);
        uVar5 = uVar9;
        if ((uVar7 | uVar5) != 0) {
          if (uVar9 < param_2) {
            param_2 -= uVar9;
            uStack26 = 0;
            iStack24 = 0;
          }
          else {
            uStack26 = uVar5 - param_2;
            iStack24 = (uVar7 - param_2) - (uVar5 < param_2);
            param_2 = 0;
            uVar9 = uVar3;
          }
          unaff_CS = 0x1030;
          pass1_1030_7d1c(uVar9,param_2,paStack22,uStack26,CONCAT22(param_3,iStack24));
          if (param_2 == 0) {
            return;
          }
        }
      }
    }
  }
  return;
}
pub fn pass1_1038_53ba(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x1a2 + param_2 * 0x4) < (param_1 + 0x14e + param_2 * 0x4)) {
    return;
  }
  return;
}
pub fn empty_1038_540a()

{
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_5464(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut extraout_DX_01: u16;
  let mut extraout_DX_02: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut unaff_CS: u16;
  let mut local_2e: u16;
  let mut uStack44: u16;
  let mut local_2a: u16;
  let mut uStack40: u16;
  let mut puStack34: *mut u32;
  let mut uStack30: u16;
  let mut uStack28: u16;
  let mut puStack26: *mut u32;
  let mut uStack22: u32;
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  pass1_1038_56ba(param_2);
  pass1_1038_57c0(param_2);
  uVar8 = (param_2 >> 0x10);
  iVar6 = param_2;
  if ((iVar6 + 0xc) == 0) {
    param_1 = 0;
    uVar5 = 0;
  }
  else {
    ppcVar2 = ((iVar6 + 0xc) + 0x10);
    (**ppcVar2)();
    uVar5 = extraout_DX;
  }
  uStack10 = CONCAT22(uVar5,param_1);
  for (uStack14 = 0; uStack14 < uStack10; uStack14 += 1) {
    uVar1 = (iVar6 + 0xc);
    ppcVar2 = ((iVar6 + 0xc) + 0x4);
    uVar4 = uStack10;
    (**ppcVar2)(unaff_CS,uVar1,(uVar1 >> 0x10),uStack14,(uStack14 >> 0x10));
    uVar3 = uVar4;
    uVar5 = extraout_DX_02 | uVar3;
    uStack18 = uVar3;
    uStack16 = extraout_DX_02;
    if (uVar5 != 0) {
      unaff_CS = 0x1028;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | extraout_DX_02 << 0x10);
      uStack22 = CONCAT22(uVar5,uVar3);
      puStack26 = (uVar3 + 0x22);
      if (((uVar3 + 0x24) | puStack26) == 0) {
        uStack28 = 0;
      }
      else {
        uStack28 = (puStack26 + 0x4);
      }
      for (uStack30 = 0; uStack30 < uStack28; uStack30 += 1) {
        unaff_CS = 0x1020;
        pass1_1020_bb16(puStack26,CONCAT13(0x10,CONCAT12(0x50,&local_2e)),CONCAT22(0x1050,&local_2a),
                        uStack30);
        if (CONCAT22(uStack44,local_2e) != 0) {
          pass1_1038_5694(param_2,CONCAT22(uStack44,local_2e),local_2a);
        }
      }
      uVar9 = (uStack22 >> 0x10);
      puStack34 = (uStack22 + 0x1e);
      uVar5 = (uStack22 + 0x20);
      uVar3 = uVar5 | puStack34;
      if (uVar3 == 0) {
        uVar3 = 0;
      }
      else {
        ppcVar2 = (*puStack34 + 0x10);
        (**ppcVar2)(unaff_CS,puStack34,uVar5);
        uVar5 = extraout_DX_00;
      }
      uStack28 = uVar3;
      for (uStack30 = 0; uStack30 < uStack28; uStack30 += 1) {
        ppcVar2 = (*puStack34 + 0x4);
        uVar3 = uStack28;
        (**ppcVar2)(unaff_CS,puStack34,(puStack34 >> 0x10),uStack30,0x0);
        uVar5 = extraout_DX_01 | uVar3;
        local_2a = uVar3;
        uStack40 = extraout_DX_01;
        if (uVar5 != 0) {
          unaff_CS = 0x1028;
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(extraout_DX_01,uVar3));
          iVar7 = (uVar3 + 0xc) * 0x4;
          (iVar6 + iVar7 + 0x14e) = (iVar6 + 0x14e + iVar7) + 1;
        }
      }
    }
  }
  uVar4 = uStack10;
  pass1_1030_38f2((iVar6 + 0x1f6),0x3);
  uVar3 = uVar4;
  uStack6 = uVar3;
  uStack6 = uVar5;
  pass1_1030_38f2((iVar6 + 0x1f6),0x4);
  uStack6 = CONCAT22(uStack6 + uVar5 + CARRY2(uStack6,uVar3),uStack6 + uVar3);
  if (uStack6 == 0) {
    pass1_1030_38f2((iVar6 + 0x1f6),0x2);
    uStack6 = CONCAT22(uVar5,uVar3);
  }
  uVar1 = (iVar6 + 0x1f6);
  uStack6 += (uVar1 + 0x170);
  pass1_1038_5694(param_2,uStack6,0x24);
  return;
}



pub fn pass1_1038_565e(param_1: *mut u8,mut param_2: u32) -> u32

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut in_stack_0000ffce: u16;
  let mut local_4: [u8;0x2] = [0;0x2];

  uVar2 = (param_2 >> 0x10);
  iVar1 = param_2;
  uVar3 = pass1_1030_8e3c(CONCAT22(local_4,param_1),CONCAT22(0x1050,local_4),(iVar1 + 0x4),
                          in_stack_0000ffce);
  pass1_1038_582c(param_2,uVar3);
  return CONCAT22((iVar1 + 0x16),(iVar1 + 0x14));
}
pub fn pass1_1038_5694(mut param_1: u32,param_2: i32,mut param_3: i16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  (param_1 + param_3 * 0x4 + 0x26) = (param_1 + 0x26 + param_3 * 0x4) + param_2;
  return;
}
pub fn pass1_1038_56ba(mut param_1: u32)

{
  pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x26)),NULL,0x94);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_56d6(mut param_1: u32,mut param_2: i16)

{
  let mut ppcVar1: *mut *mut code;
  let mut iVar2: i16;
  let mut puVar3: *mut u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut extraout_DX: u16;
  let mut uVar6: u16;
  let mut extraout_DX_00: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  iVar2 = param_1;
  uVar9 = 0x1000;
  puVar3 = pass1_1000_4906((param_1 & 0xffff0000 | (iVar2 + 0xba)),NULL,0x94);
  if (param_2 != 0) {
    uVar8 = (param_1 >> 0x10);
    if ((iVar2 + 0xc) == 0) {
      puVar3 = NULL;
      uVar6 = 0;
    }
    else {
      ppcVar1 = ((iVar2 + 0xc) + 0x10);
      (**ppcVar1)();
      uVar6 = extraout_DX;
    }
    uStack6 = CONCAT22(uVar6,puVar3);
    for (uStack10 = 0; uStack10 < uStack6; uStack10 += 1) {
      ppcVar1 = ((iVar2 + 0xc) + 0x4);
      uVar5 = uStack6;
      (**ppcVar1)(uVar9,(iVar2 + 0xc));
      uVar4 = uVar5;
      uVar7 = extraout_DX_00 | uVar4;
      if (uVar7 != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5 & 0xffff | extraout_DX_00 << 0x10);
        uVar9 = 0x1030;
        pass1_1030_72d0(CONCAT22(uVar7,uVar4));
      }
    }
  }
  return;
}
pub fn pass1_1038_5770(mut param_1: u32,param_2: i32,mut param_3: i16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  (param_1 + param_3 * 0x4 + 0xba) = (param_1 + 0xba + param_3 * 0x4) + param_2;
  return;
}
pub fn pass1_1038_5798(mut param_1: u32,param_2: i32,mut param_3: i16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  (param_1 + param_3 * 0x4 + 0x14e) = (param_1 + 0x14e + param_3 * 0x4) + param_2;
  return;
}
pub fn pass1_1038_57c0(mut param_1: u32)

{
  pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x14e)),NULL,0x54);
  return;
}
pub fn pass1_1038_57dc(mut param_1: u32,param_2: i32,mut param_3: i16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  (param_1 + param_3 * 0x4 + 0x1a2) = (param_1 + 0x1a2 + param_3 * 0x4) + param_2;
  return;
}
pub fn pass1_1038_5804(mut param_1: u32,param_2: i32,mut param_3: i16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  (param_1 + param_3 * 0x4 + 0x1a2) = (param_1 + 0x1a2 + param_3 * 0x4) - param_2;
  return;
}
pub fn pass1_1038_582c(mut param_1: u32,mut param_2: u32)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut ppcVar3: *mut *mut code;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0x14);
  uVar2 = (iVar4 + 0x16);
  if ((uVar2 | puVar1) != 0) {
    ppcVar3 = *puVar1;
    (**ppcVar3)();
  }
  (iVar4 + 0x14) = param_2;
  return;
}
pub fn pass1_1038_5860(mut param_1: u32,mut param_2: u16 ,mut param_3: u32,mut param_4: i16)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: i16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uStack14: u32;
  let mut iStack6: i16;
  let mut iStack4: i16;

  if (param_4 == 0) {
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    ppcVar1 = ((iVar4 + 0xc) + 0x10);
    uVar2 = param_3;
    (**ppcVar1)();
    uVar2 = uVar2 & 0xffff | extraout_DX << 0x10;
    for (uStack14 = 0; uStack14 < uVar2; uStack14 += 1) {
      ppcVar1 = ((iVar4 + 0xc) + 0x4);
      uVar3 = uVar2;
      (**ppcVar1)();
      iStack6 = param_3;
      if ((uVar3 == iStack6) && (iStack4 = (param_3 >> 0x10), extraout_DX_00 == iStack4)) {
        return;
      }
    }
    ppcVar1 = ((iVar4 + 0xc) + 0xc);
    (**ppcVar1)();
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_58e6(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32,mut param_5: u32,mut param_6: i16)

{
  let mut iVar1: i16;
  let mut ppcVar2: *mut *mut code;
  let mut uVar3: u32;
  let mut BVar4: bool;
  let mut puVar5: *mut u32;
  let mut uVar6: u16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u32;
  let mut local_12: u32;
  let mut iStack14: i16;
  let mut iStack12: i16;
  let mut uStack6: u32;

  for (uStack6 = 0; uStack6 < param_3; uStack6 += 1) {
    uVar9 = (param_4 >> 0x10);
    iVar7 = param_4;
    if (((uStack6 * 0x4 + iVar7) != 0) &&
       (uVar3 = (uStack6 * 0x4 + iVar7),
       BVar4 = pass1_1008_c6ae(_u16_1050_06e0,(uVar3 + 0xc),0x2e), BVar4 != 0)) {
      uVar8 = (param_5 >> 0x10);
      iVar1 = (uStack6 * 0x4 + param_5);
      uVar8 = (uStack6 * 0x4 + param_5 + 2);
      local_12 = (iVar1 + 0xc);
      iStack12 = (iVar1 + 0x10);
      iStack14 = iStack12;
      if (iStack12 == param_6) {
        iStack14 = iStack12 + -0x1;
        uVar10 = pass1_1028_bb24(*(astruct_15 **)(uStack6 * 0x4 + iVar7));
        uVar6 = (uVar10 >> 0x10);
        puVar5 = &local_12;
        pass1_1030_627e(puVar5,uVar6,_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar5),
                        uVar10 & 0xffff | uVar6 << 0x10);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(uVar6,puVar5));
        if ((uVar6 | puVar5) != 0) {
          uVar10 = struct_op_1030_73a8(CONCAT22(uVar6,puVar5),puVar5,uVar6 | puVar5);
          uVar6 = (uVar10 + 0x1a);
          if (((uVar6 & 0x2) != 0) && ((uVar6 & 1) != 0)) {
            uVar3 = (uStack6 * 0x4 + iVar7);
            (uVar3 + 0x1a) = 0x3;
            ppcVar2 = ((uStack6 * 0x4 + iVar7) + 0x28);
            (**ppcVar2)();
          }
        }
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_5a16(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut BVar3: bool;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uStack6: u32;

  for (uStack6 = 0; uStack6 < param_3; uStack6 += 1) {
    uVar5 = (param_4 >> 0x10);
    iVar4 = param_4;
    if (((uStack6 * 0x4 + iVar4) != 0) &&
       (uVar2 = (uStack6 * 0x4 + iVar4),
       BVar3 = pass1_1008_c6ae(_u16_1050_06e0,(uVar2 + 0xc),0x2f), BVar3 != 0)) {
      uVar2 = (uStack6 * 0x4 + iVar4);
      (uVar2 + 0x1a) = 0x3;
      ppcVar1 = ((uStack6 * 0x4 + iVar4) + 0x28);
      (**ppcVar1)();
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_5a96(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut BVar3: bool;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uStack6: u32;

  for (uStack6 = 0; uStack6 < param_3; uStack6 += 1) {
    uVar5 = (param_4 >> 0x10);
    iVar4 = param_4;
    if (((uStack6 * 0x4 + iVar4) != 0) &&
       (uVar2 = (uStack6 * 0x4 + iVar4),
       BVar3 = pass1_1008_c6ae(_u16_1050_06e0,(uVar2 + 0xc),0x2c), BVar3 != 0)) {
      ppcVar1 = ((uStack6 * 0x4 + iVar4) + 0x54);
      (**ppcVar1)();
      if (BVar3 != 0) {
        uVar2 = (iVar4 + uStack6 * 0x4);
        (uVar2 + 0x1a) = 0x3;
        ppcVar1 = ((uStack6 * 0x4 + iVar4) + 0x28);
        (**ppcVar1)();
        uVar2 = (iVar4 + uStack6 * 0x4);
        (uVar2 + 0x1a) = 0x2;
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_5b3c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut BVar4: bool;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut uStack6: u32;

  for (uStack6 = 0; uStack6 < param_3; uStack6 += 1) {
    uVar6 = (param_4 >> 0x10);
    iVar5 = param_4;
    if ((((uStack6 * 0x4 + iVar5) != 0) &&
        (uVar2 = (uStack6 * 0x4 + iVar5),
        BVar4 = pass1_1008_c6ae(_u16_1050_06e0,(uVar2 + 0xc),0x2d), BVar4 != 0)) &&
       (ppcVar1 = ((uStack6 * 0x4 + iVar5) + 0x50), (**ppcVar1)(),
       BVar4 != 0)) {
      uVar2 = (uStack6 * 0x4 + iVar5);
      uVar3 = (uStack6 * 0x4 + iVar5);
      (uVar3 + 0x1a) = (uVar2 + 0x1a) | 0x1;
      ppcVar1 = ((uStack6 * 0x4 + iVar5) + 0x28);
      (**ppcVar1)();
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1038_5be8(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 ,mut param_5: i16,param_6: *mut u16) -> u16

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut BVar4: bool;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut iStack14: i16;
  paStack10: *mut astruct_419;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_6,(param_3 + 0x8));
  uVar5 = param_2 | param_1;
  if (uVar5 != 0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    paStack10 = CONCAT22(uVar5,param_1);
    iVar3 = (param_6 + 0x4);
    iStack14 = 0x7a;
    if (0x0 < iVar3) {
      iVar3 = param_5 + -0x7b;
      if (iVar3 == 0) {
        param_5 = 0x7e;
      }
      else {
        iVar3 = param_5 + -0x7c;
        if (iVar3 == 0) {
          param_5 = 0x7d;
        }
      }
      iStack14 = 0x7f;
    }
    uVar6 = struct_op_1030_73a8(paStack10,iVar3,uVar5);
    uVar2 = (uVar6 >> 0x10);
    iVar3 = uVar6;
    if (((((iVar3 + 0x1a) & param_4) == 0) &&
        (((iVar1 = (iVar3 + 0xc), iVar1 == iStack14 || (iVar1 == param_5)) ||
         (BVar4 = pass1_1008_c6ae(_u16_1050_06e0,iVar1,0x2b), BVar4 != 0)))) && ((iVar3 + 0x12) != 0x7)) {
      (iVar3 + 0x1a) = (iVar3 + 0x1a) | param_4;
      return 0x1;
    }
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1038_5cc6(mut param_1: u32,mut param_2: u32,mut param_3: u32,mut param_4: u32,mut param_5: i16,mut param_6: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  let mut local_14: i16;
  let mut local_12: i16;
  let mut local_10: i16;
  let mut uStack14: u32;
  let mut local_a: i16;
  let mut iStack8: i16;
  let mut iStack4: i16;

  puVar6 = pass1_1008_3e38(CONCAT22(0x1050,&local_a));
  uVar4 = (puVar6 >> 0x10);
  loop {
    iStack4 = 0;
    for (uStack14 = 0; uStack14 < param_2; uStack14 += 1) {
      uVar5 = (param_4 >> 0x10);
      if ((uStack14 * 0x4 + param_4) != 0) {
        uVar1 = (uStack14 * 0x4 + param_4);
        pass1_1008_3f62(CONCAT22(0x1050,&local_a),(uVar1 & 0xffff0000 | (uVar1 + 0xc)));
        pass1_1008_3eb4(CONCAT22(0x1050,&local_a),CONCAT22(0x1050,&local_14),
                        CONCAT22(0x1050,&local_12),CONCAT22(0x1050,&local_10));
        if (local_14 == param_5) {
          uVar5 = (param_3 >> 0x10);
          if (((uStack14 * 0x4 + param_3) != 0) &&
             (uVar2 = (uStack14 * 0x4 + param_3),
             ((uVar2 + 0x1a) & param_6) != 0)) {
            iStack8 = local_12 + -0x1;
            uVar3 = pass1_1038_5be8(&local_a,uVar4,param_1,param_6,0x7b,CONCAT22(0x1050,&local_a));
            if (uVar3 != 0) {
              iStack4 = 0x1;
            }
            iStack8 = local_12 + 1;
            uVar3 = pass1_1038_5be8(&local_a,uVar4,param_1,param_6,0x7b,CONCAT22(0x1050,&local_a));
            if (uVar3 != 0) {
              iStack4 = 0x1;
            }
            iStack8 = local_12;
            local_a = local_10 + -0x1;
            uVar3 = pass1_1038_5be8(&local_a,uVar4,param_1,param_6,0x7c,CONCAT22(0x1050,&local_a));
            if (uVar3 != 0) {
              iStack4 = 0x1;
            }
            local_a = local_10 + 1;
            uVar3 = pass1_1038_5be8(&local_a,uVar4,param_1,param_6,0x7c,CONCAT22(0x1050,&local_a));
            if (uVar3 != 0) {
              iStack4 = 0x1;
            }
          }
        }
      }
    }
  } while (iStack4 != 0);
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1038_5e16(mut param_1: i16,mut param_2: u32,mut param_3: u32)

{
  let mut BVar1: bool;
  let mut puVar2: *mut u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  in_stack_0000ffb6: mut HFILE16;
  u32 local_14 [0x2];
  let mut local_c: u32;
  let mut puStack6: *mut u32;

  pass1_1030_16d6(param_2,param_3);
  if (param_1 != 0) {
    uVar4 = (param_2 >> 0x10);
    iVar3 = param_2;
    puVar2 = (iVar3 + 0xc);
    puStack6 = puVar2;
    pass1_1008_7898(puVar2,param_3,puVar2);
    if (puVar2 != 0) {
      local_14[0] = (iVar3 + 0x10);
      BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_14),0x4,in_stack_0000ffb6);
      if (BVar1 != 0) {
        local_c = (iVar3 + 0x18);
        BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_c),0x2,in_stack_0000ffb6);
        if (BVar1 != 0) {
          local_c = (iVar3 + 0x1a);
          BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_c),0x2,in_stack_0000ffb6);
          if (BVar1 != 0) {
            local_c = CONCAT22(local_c,(iVar3 + 0x1c));
            BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_c),0x2,in_stack_0000ffb6);
            if (BVar1 != 0) {
              local_c = (iVar3 + 0x1e);
              BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_c),0x4,in_stack_0000ffb6);
              if (BVar1 != 0) {
                local_c = local_c & 0xffff0000 | (iVar3 + 0x22);
                BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_c),0x2,in_stack_0000ffb6);
                if (BVar1 != 0) {
                  local_c = local_c & 0xffff0000 | (iVar3 + 0x24);
                  BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_c),0x2,in_stack_0000ffb6)
                  ;
                  if (BVar1 != 0) {
                    BVar1 = write_to_file_1008_7e1c
                                      (param_3,param_2 & 0xffff0000 | (iVar3 + 0x26),0x94,
                                       in_stack_0000ffb6);
                    if (BVar1 != 0) {
                      BVar1 = write_to_file_1008_7e1c
                                        (param_3,param_2 & 0xffff0000 | (iVar3 + 0x14e),0x54,
                                         in_stack_0000ffb6);
                      if (BVar1 != 0) {
                        BVar1 = write_to_file_1008_7e1c
                                          (param_3,param_2 & 0xffff0000 | (iVar3 + 0x1a2),0x54,
                                           in_stack_0000ffb6);
                        if (BVar1 != 0) {
                          write_to_file_1030_32e4((iVar3 + 0x1f6),param_3);
                          BVar1 = pass1_1008_7c2a(param_3,*(iVar3 + 0x1fa));
                          if (BVar1 != 0) {
                            local_c = local_c & 0xffff0000 | (iVar3 + 0x1fe);
                            BVar1 = write_to_file_1008_7e1c
                                              (param_3,CONCAT22(0x1050,&local_c),0x2,in_stack_0000ffb6);
                            if (BVar1 != 0) {
                              local_c = (iVar3 + 0x200);
                              BVar1 = write_to_file_1008_7e1c
                                                (param_3,CONCAT22(0x1050,&local_c),0x4,in_stack_0000ffb6);
                              if (BVar1 != 0) {
                                local_c = local_c & 0xffff0000 | (iVar3 + 0x204);
                                BVar1 = write_to_file_1008_7e1c
                                                  (param_3,CONCAT22(0x1050,&local_c),0x2,in_stack_0000ffb6
                                                  );
                                if (BVar1 != 0) {
                                  local_c = local_c & 0xffff0000 | (iVar3 + 0x206);
                                  BVar1 = write_to_file_1008_7e1c
                                                    (param_3,CONCAT22(0x1050,&local_c),0x2,
                                                     in_stack_0000ffb6);
                                  if (BVar1 != 0) {
                                    local_c = local_c & 0xffff0000 | (iVar3 + 0x208);
                                    BVar1 = write_to_file_1008_7e1c
                                                      (param_3,CONCAT22(0x1050,&local_c),0x2,
                                                       in_stack_0000ffb6);
                                    if (BVar1 != 0) {
                                      local_c = local_c & 0xffff0000 | (iVar3 + 0x20a);
                                      BVar1 = write_to_file_1008_7e1c
                                                        (param_3,CONCAT22(0x1050,&local_c),0x2,
                                                         in_stack_0000ffb6);
                                      if (BVar1 != 0) {
                                        local_c = local_c & 0xffff0000 | (iVar3 + 0x20c);
                                        BVar1 = write_to_file_1008_7e1c
                                                          (param_3,CONCAT22(0x1050,&local_c),0x2,
                                                           in_stack_0000ffb6);
                                        if (BVar1 != 0) {
                                          local_c = local_c & 0xffff0000 | (iVar3 + 0x20e);
                                          BVar1 = write_to_file_1008_7e1c
                                                            (param_3,CONCAT22(0x1050,&local_c),0x2,
                                                             in_stack_0000ffb6);
                                          if (BVar1 != 0) {
                                            local_c = local_c & 0xffff0000 | (iVar3 + 0x214);
                                            BVar1 = write_to_file_1008_7e1c
                                                              (param_3,CONCAT22(0x1050,&local_c),0x2,
                                                               in_stack_0000ffb6);
                                            if (BVar1 != 0) {
                                              local_c = (iVar3 + 0x216);
                                              BVar1 = write_to_file_1008_7e1c
                                                                (param_3,CONCAT22(0x1050,&local_c),0x4,
                                                                 in_stack_0000ffb6);
                                              if (BVar1 != 0) {
                                                return;
                                              }
                                            }
                                          }
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}
