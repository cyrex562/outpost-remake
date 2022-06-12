//
// Created by cyrex on 2022-05-22.
//

#include "block_1020.h"











// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 pass1_1020_e044(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  astruct_598 *paVar4;

  uVar3 = (param_1 >> 0x10);
  paVar4 = (astruct_598 *)pass1_1018_04b8((param_1 + 0x28));
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,paVar4);
  uVar2 = pass1_1030_2fac(paVar4);
  uVar1 = (param_1 + 0x28);
  if (uVar2 <= (uVar1 + 0x1e)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_e08e(mut param_1: u16 ,param_2: *mut astruct_15)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut iVar4: i16;
  let mut uVar5: u32;
  let mut uVar6: u32;
  let mut extraout_DX: u16;
  let mut uVar7: u16;
  astruct_15 *pstruct15_7;
  astruct_15 *pstruct15_7_seg;
  let mut piVar8: *mut i16;
  let mut uVar9: u16;
  let mut puVar10: *mut u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut local_158: u16;
  let mut uStack342: u16;
  let mut uStack50: u32;
  let mut puStack42: *mut u32;
  let mut local_22: i16;
  u8 local_20 [0x2];
  u8 local_1e [0x2];
  let mut uStack28: u16;
  let mut piStack26: *mut i16;
  let mut local_18: i16;
  let mut local_16: u16;
  let mut uStack20: u32;
  let mut local_10: u32;
  let mut iStack12: i16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  pstruct15_7_seg = (astruct_15 *)(param_2 >> 0x10);
  pstruct15_7 = (astruct_15 *)param_2;
  iVar3 = pstruct15_7->field10_0xc;
  if (iVar3 == 0x74) {
    uVar1 = (pstruct15_7 + 0x1)->field0_0x0;
    iVar3 = uVar1 - 0x1;
    if (iVar3 == 0x0) goto LAB_1020_e0ae;
    iVar3 = uVar1 - 0x6;
    if (iVar3 != 0x0) goto LAB_1020_e0b9;
    uVar11 = 0x1;
  }
  else if (iVar3 == 0x78) {
    uVar1 = (pstruct15_7 + 0x1)->field0_0x0;
    iVar4 = uVar1 - 0x1;
    if (iVar4 != 0x0) {
      iVar3 = uVar1 - 0x2;
      if ((0x0 < iVar4) && (!SBORROW2(iVar4,0x1))) {
        if (uVar1 - 0x5 == 0x0 || iVar3 < 0x3) {
          iVar3 = uVar1 - 0x5;
          pass1_1020_e49a(param_2);
        }
        else {
          iVar3 = uVar1 - 0x6;
          if (iVar3 == 0x0) {
            pass1_1020_e39c(0x0,param_2,0x6);
            pass1_1020_dca8(param_1,param_2);
          }
        }
      }
      goto LAB_1020_e0b9;
    }
    uVar11 = 0x6a;
    iVar3 = iVar4;
  }
  else {
    iVar3 += -0x79;
    if (iVar3 == 0x0) {
      pass1_1020_e49a(param_2);
      return;
    }//
LAB_1020_e0ae:
    uVar11 = 0x47;
  }
  pass1_1020_e39c(iVar3,param_2,uVar11);//
LAB_1020_e0b9:
  pass1_1028_b58e(param_2);
  uStack6 = CONCAT22(extraout_DX,iVar3);
  uVar5 = (iVar3 + 0x2e);
  uVar7 = (iVar3 + 0x30);
  uStack10 = uVar5;
  if (pstruct15_7->field10_0xc != 0x79) {
    pass1_1038_5804(uVar5 & 0xffff | uVar7 << 0x10,0x1,0x2);
  }
  if ((pstruct15_7 + 0x1)->field0_0x0 == 0x6) {
    pass1_1038_43cc(uVar5,uVar7,uStack10,(uStack10 >> 0x10),0x1,0x2);
  }
  local_10 = (uStack6 + 0xc);
  iStack12 = (uStack6 + 0x10);
  puStack42 = &local_10;
  if (((pstruct15_7 + 0x1)->field0_0x0 == 0x6) && (iStack12 == 0x0)) {
    return;
  }
  uVar2 = &pstruct15_7[0x1].field_0x4;
  uVar6 = (uVar2 + 0x20);
  puVar10 = &local_16;
  uVar12 = SUB42(&DAT_1050_1050,0x0);
  piStack26 = &local_18;
  uVar9 = SUB42(&DAT_1050_1050,0x0);
  piVar8 = piStack26;
  uStack20 = uVar6;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar6);
  uStack28 = uVar6;
  pass1_1030_5b1c(uVar6 & 0xffff | ZEXT24(piStack26) << 0x10,(u16 *)CONCAT22(uVar9,piVar8),
                  (u16 *)CONCAT22(uVar12,puVar10));
  pass1_1028_c8ee(param_2,(pstruct15_7 + 0x1)->field0_0x0,(u16 *)CONCAT22(0x1050,&local_10));
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_10),(u16 *)CONCAT22(0x1050,&local_22),
                  (u16 *)CONCAT22(0x1050,local_20),(u16 *)CONCAT22(0x1050,local_1e));
  if ((pstruct15_7 + 0x1)->field0_0x0 == 0x1) {
    if (local_18 < local_22) {
      pass1_1030_5b3e(CONCAT22(piStack26,uStack28),local_22,local_16);
      pass1_1030_5b1c(CONCAT22(piStack26,uStack28),(u16 *)CONCAT22(0x1050,&local_18),
                      (u16 *)CONCAT22(0x1050,&local_16));
    }
    uStack50 = (uStack10 + 0x4);
    struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,&local_158),0x0,0x0,0x6a,&local_10,&DAT_1050_1050,uStack50
                        ,uStack20);
    fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,&local_158));
    local_158 = 0x389a;
    uStack342 = 0x1008;
  }
  pass1_1028_ccd0(param_2,(u16 *)CONCAT22(0x1050,&local_10));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_e294(param_1: *mut astruct_15)

{
  let mut uVar1: u32;
  let mut puVar2: *mut u32;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  astruct_15 *pstruct15_5;
  astruct_15 *uVar6;
  char cStack347;
  u8 local_150 [0xc];
  let mut puStack324: *mut u32;
  char string_140 [0x124];
  let mut iStack20: i16;
  let mut local_10: u32;
  let mut uStack12: u16;
  let mut iStack10: i16;
  let mut uStack8: u16;
  let mut uStack6: u32;

  uVar6 = (astruct_15 *)(param_1 >> 0x10);
  pstruct15_5 = (astruct_15 *)param_1;
  if ((0x1 < (pstruct15_5 + 0x1)->field0_0x0) && ((pstruct15_5 + 0x1)->field0_0x0 < 0x6)) {
    uVar1 = &pstruct15_5[0x1].field_0x4;
    uVar3 = (uVar1 + 0x20);
    uStack6 = uVar3;
    pass1_1028_b58e(param_1);
    iStack10 = uVar3;
    local_10 = (iStack10 + 0xc);
    uStack12 = (iStack10 + 0x10);
    puStack324 = &local_10;
    uVar4 = extraout_DX;
    uStack8 = extraout_DX;
    pass1_1028_c8ee(param_1,(pstruct15_5 + 0x1)->field0_0x0,(u16 *)CONCAT22(0x1050,puStack324));
    puVar2 = &local_10;
    pass1_1028_c89c(puVar2,param_1,(u16 *)CONCAT22(0x1050,puVar2),CONCAT22(0x1050,local_150));
    uVar3 = *puVar2;
    cStack347 = (char)(uVar3 >> 0x18);
    if ((cStack347 == '\0') && (iStack20 = uVar3, iStack20 == 0x9)) {
      &pstruct15_5->field16_0x14 = 0x1;
    }
    uVar1 = (iStack10 + 0x2e);
    struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,string_140),0x0,&pstruct15_5->field16_0x14 + 0x1,0x79,
                        &local_10,&DAT_1050_1050,(uVar1 + 0x4),uStack6);
    fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,string_140));
  }
  pstruct15_5[0x1].field1_0x2 = 0x1;
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_e39c(mut param_1: i16,param_2: *mut astruct_15,mut param_3: u16 )

{
  let mut uVar3: u16;
  let mut extraout_DX: u16;
  char local_13c [0x124];
  let mut local_c: u32;
  let mut iStack8: i16;
  let mut uStack6: u32;
  let mut uVar1: u32;
  let mut uVar2: u32;

  pass1_1028_b58e(param_2);
  uStack6 = CONCAT22(extraout_DX,param_1);
  local_c = (param_1 + 0xc);
  iStack8 = (param_1 + 0x10);
  if (iStack8 < 0x1) {
    uVar3 = 0x5;
  }
  else {
    uVar3 = 0x6;
  }
  (param_1 + 0x14) = uVar3;
  uVar2 = (param_2 + 0x28);
  uVar1 = (param_1 + 0x2e);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_13c),0x0,0x1,param_3,&local_c,&DAT_1050_1050,
                      (uVar1 + 0x4),(uVar2 + 0x20));
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_13c));
  return;
}
pub fn pass1_1020_e44c(undefined1 param_1,mut param_2: u16 ,mut param_3: u32)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_3 >> 0x10);
  iVar2 = param_3;
  if ((iVar2 + 0x12) == 0x2) {
    piVar1 = (iVar2 + 0x14);
    *piVar1 = *piVar1 + -0x1;
    if (((iVar2 + 0x26) == 0x0) && ((iVar2 + 0xc) == 0x78)) {
      pass1_1020_e294((astruct_15 *)(param_3 & 0xffff | uVar3 << 0x10));
    }
    if ((iVar2 + 0x14) == 0x0) {
      pass1_1020_e08e(param_2,(astruct_15 *)(param_3 & 0xffff | uVar3 << 0x10));
      return;
    }
    if ((iVar2 + 0x24) == 0x6) {
      (iVar2 + 0xe) = 0x49;
    }
  }
  return;
}
pub fn pass1_1020_e49a(param_1: *mut astruct_15)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut uVar3: u32;
  let mut uStack10: u16;

  uVar3 = pass1_1028_b58e(param_1);
  iVar1 = (uVar3 + 0x14);
  uStack10 = 0x0;
  iVar2 = iVar1 + -0x6;
  if (iVar2 == 0x0) {
    uStack10 = 0x9;
  }
  else {
    iVar2 = iVar1 + -0x7;
    if (iVar2 == 0x0) {
      uStack10 = 0x6;
    }
    else {
      iVar2 = iVar1 + -0x8;
      if (iVar2 == 0x0) {
        uStack10 = 0x7;
      }
      else {
        iVar2 = iVar1 + -0x9;
        if (iVar2 == 0x0) {
          uStack10 = 0x8;
        }
      }
    }
  }
  pass1_1020_e39c(iVar2,param_1,uStack10);
  return;
}



i16 pass1_1020_e4fa(param_1: *mut astruct_15,mut param_2: u16 )

{
  let mut uVar1: u32;
  let mut iStack4: i16;

  switch(param_2) {
  case 0x2:
  case 0x5:
  case 0x6:
  case 0x7:
    iStack4 = 0x4;
    break;
  case 0x3:
  case 0x8:
    iStack4 = 0x5;
    break;
  default:
    uVar1 = pass1_1028_b58e(param_1);
    iStack4 = (uVar1 + 0x14) + 0x2;
  }
  return iStack4;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_e558(mut param_1: i16,param_2: *mut astruct_15)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  u8 bStack45;
  u8 local_24 [0xc];
  let mut uStack24: u32;
  let mut uStack20: u32;
  let mut local_10: u32;
  let mut uStack12: u16;
  let mut iStack10: i16;
  let mut uStack8: u16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  uVar7 = (param_2 >> 0x10);
  iVar6 = param_2;
  if ((iVar6 + 0xc) == 0x79) {
    param_1 = (iVar6 + 0x24);
    (iVar6 + 0x14) = param_1;
    (iVar6 + 0x24) = 0x0;
  }
  if ((iVar6 + 0x24) != 0x6) {
    pass1_1028_b58e(param_2);
    uStack8 = (param_1 + 0x14);
    iStack6 = param_1;
    uStack4 = extraout_DX;
    iStack10 = pass1_1020_e4fa(param_2,uStack8);
    local_10 = (iStack6 + 0xc);
    uStack12 = (iStack6 + 0x10);
    uStack24 = CONCAT22(uStack24,&local_10);
    uVar4 = uStack4;
    pass1_1028_c8ee(param_2,(iVar6 + 0x24),(u16 *)CONCAT22(0x1050,&local_10));
    puVar1 = &local_10;
    pass1_1028_c89c(puVar1,param_2,(u16 *)CONCAT22(0x1050,puVar1),CONCAT22(0x1050,local_24));
    uStack24 = *puVar1;
    uVar5 = (puVar1 + 0x2);
    bStack45 = (u8)(uStack24 >> 0x18);
    uVar2 = bStack45;
    uStack20 = uStack24;
    uStack20 = uStack24;
    if (bStack45 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack24 & 0xffff | uVar5 << 0x10);
      uStack20 = (uVar2 + 0x14);
    }
    uStack8 = uStack20;
    iVar3 = pass1_1020_e4fa(param_2,uStack20);
    (iVar6 + 0x14) = iStack10 + iVar3;
    return;
  }
  (iVar6 + 0x14) = 0x1;
  return;
}



u32 * pass1_1020_e652(mut param_1: u32,u32 *param_2,mut param_3: u16 ,i32 param_4)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut local_8: u32;
  let mut uStack4: u16;

  local_8 = *param_2;
  uStack4 = (param_2 + 0x1);
  uVar2 = (param_1 >> 0x10);
  pass1_1028_c8ee((astruct_15 *)param_1,(param_1 + 0x24),(u16 *)CONCAT22(0x1050,&local_8));
  puVar1 = &local_8;
  pass1_1028_c7b6(param_3,param_1,uVar2,(u16 *)CONCAT22(0x1050,puVar1),param_4);
  if (puVar1 != NULL) {
    puVar1 = (&PTR_LOOP_1050_0000 + 0x1);
  }
  return puVar1;
}



BOOL16 write_to_file_1020_e6a4(mut param_1: u32,u8 *param_2)

{
  let mut in_AX: i16;
  let mut BVar1: bool;
  let mut uVar2: u16;
  HFILE16 in_stack_0000ffdc;
  u16 local_c [0x3];
  u16 local_6 [0x2];

  pass1_1030_de7c(param_1,param_2);
  if (in_AX != 0x0) {
    uVar2 = (param_1 >> 0x10);
    local_c[0] = (param_1 + 0x24);
    BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_c),0x2,in_stack_0000ffdc);
    if (BVar1 != 0x0) {
      local_6[0] = (param_1 + 0x26);
      BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_6),0x2,in_stack_0000ffdc);
      if (BVar1 != 0x0) {
        return 0x1;
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return 0x0;
}
pub fn pass1_1020_e70e(mut param_1: i16,u8 *param_2,mut param_3: u32,mut param_4: u32)

{
  let mut BVar1: bool;

  pass1_1030_dec4(param_1,param_2,(astruct_373 *)param_3,(HFILE16 *)param_4);
  if (param_1 != 0x0) {
    BVar1 = read_file_1008_7dee((HFILE16 *)param_4,(param_3 & 0xffff0000 | (param_3 + 0x24)),0x2);
    if (BVar1 != 0x0) {
      BVar1 = read_file_1008_7dee((HFILE16 *)param_4,(param_3 & 0xffff0000 | (param_3 + 0x26)),0x2);
      if (BVar1 != 0x0) {
        return;
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



StructD * pass1_1020_e76c(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  pass1_1030_dcf4(param_1,(astruct_15 *)param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_2);
  }
  return param_2;
}



u16 * struct_1020_e7fa(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1->field0_0x0 = 0xe88e;
  (param_1 + 0x2) = 0x1020;
  return &param_1->field0_0x0;
}



u16 * pass1_1020_e81c(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2->field0_0x0 = 0xe88e;
  (param_2 + 0x2) = 0x1020;
  return &param_2->field0_0x0;
}
pub fn pass1_1020_e846(param_1: *mut u16)

{
  *param_1 = 0xe88e;
  (param_1 + 0x2) = 0x1020;
  pass1_1028_b418(param_1);
  return;
}
pub fn FUN_1020_e864(void)

{
  return;
}



StructD * pass1_1020_e868(StructD *param_1,param_2: u8)

{
  pass1_1020_e846(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * struct_1020_e8f6(param_1: *mut u16)

{
  let mut uVar1: u16;

  struct_1030_dc96((astruct_180 *)param_1);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x24) = 0x0;
  *param_1 = 0xeef6;
  (param_1 + 0x2) = 0x1020;
  return param_1;
}



u16 * pass1_1020_e91e(mut param_1: u16 ,mut param_2: i16,mut param_3: u16 ,mut param_4: i16,mut param_5: u32)

{
  pass1_1030_dcc2(param_1,(astruct_12 *)CONCAT22(param_3,param_2),param_4,param_5);
  (param_2 + 0x24) = 0x0;
  CONCAT22(param_3,param_2) = 0xeef6;
  (param_2 + 0x2) = 0x1020;
  return (u16 *)CONCAT22(param_3,param_2);
}



BOOL16 pass1_1020_e94e(BOOL16 param_1,mut param_2: u32,mut param_3: u32)

{
  let mut BVar1: bool;
  HFILE16 in_stack_0000ffde;
  u16 local_c [0x5];

  pass1_1030_de7c(param_2,param_3);
  if (param_1 != 0x0) {
    local_c[0] = (param_2 + 0x24);
    BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_c),0x2,in_stack_0000ffde);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return BVar1;
    }
    param_1 = 0x1;
  }
  return param_1;
}
pub fn pass1_1020_e994(mut param_1: i16,u8 *param_2,mut param_3: u32,mut param_4: u32)

{
  let mut BVar1: bool;

  pass1_1030_dec4(param_1,param_2,(astruct_373 *)param_3,(HFILE16 *)param_4);
  if ((param_1 != 0x0) &&
     (BVar1 = read_file_1008_7dee((HFILE16 *)param_4,(param_3 & 0xffff0000 | (param_3 + 0x24)),0x2),
     BVar1 == 0x0)) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  return;
}
pub fn pass1_1020_e9d4(mut param_1: u16 ,param_2: *mut astruct_15)

{
  let mut extraout_DX: u16;

  pass1_1030_df0c(param_1,param_2);
  pass1_1028_b58e(param_2);
  pass1_1038_57dc((param_1 + 0x2e),0x1,0x1);
  return;
}
pub fn pass1_1020_ea0e(u32 *param_1)

{
  pass1_1028_bdac((astruct_15 *)param_1,0x1);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_ea20(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,param_4: *mut u16,mut param_5: u32,mut param_6: u32)

{
  let mut uVar1: u16;
  code **ppcVar2;
  char cVar3;
  let mut puVar4: *mut u32;
  let mut uVar5: u16;
  let mut uVar6: u16;
  u8 *puVar7;
  let mut in_register_0000000a: u16;
  let mut paVar8: *mut Struct57;
  let mut uVar9: u32;
  let mut uVar10: u32;
  let mut uVar11: u16;
  let mut in_stack_0000fd5e: u16;
  let mut in_stack_0000fe82: u16;
  let mut in_stack_0000fe88: u16;
  let mut in_stack_0000fe8c: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut iVar14: i16;
  u8 local_146 [0x10c];
  let mut uStack58: u16;
  let mut uStack56: u16;
  astruct_419 *paStack50;
  let mut puStack46: *mut u32;
  let mut puStack42: *mut u32;
  let mut uStack38: u32;
  let mut uStack34: u32;
  let mut uStack28: u32;
  let mut local_12: u32;
  let mut iStack14: i16;
  let mut puStack12: *mut u32;
  let mut uStack8: u32;
  let mut BStack4: bool;

  uVar12 = param_3;
  uVar13 = (param_3 >> 0x10);
  pass1_1028_c3aa(uVar12,uVar13,param_4,param_5,param_6);
  if (param_1 == 0x0) {
    return;
  }
  pass1_1028_c23e(param_1,param_2,uVar12,uVar13,param_4,param_5,param_6);
  if (param_1 == 0x0) {
    return;
  }
  BStack4 = pass1_1028_c314(param_1,param_2,uVar12,uVar13,param_4,param_5,(param_5 >> 0x10),param_6);
  if (BStack4 == 0x0) {
    return;
  }
  pass1_1028_c7b6(param_2,uVar12,uVar13,param_4,param_6);
  if ((((BStack4 == 0x5) || (BStack4 == 0x4)) || (BStack4 == 0x6)) || (BStack4 == 0x9)) {
    PTR_LOOP_1050_50ca = 0x6a8;
    return;
  }
  if (BStack4 != 0x0) {
    return;
  }
  puVar4 = &local_12;
  pass1_1030_64ce(puVar4,param_2,_PTR_LOOP_1050_5740,param_4,param_6,CONCAT22(0x1050,puVar4));
  uStack38 = *puVar4;
  uVar1 = (puVar4 + 0x2);
  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,uVar1);
  uStack38._3_1_ = (u8)(uStack38 >> 0x18);
  uStack58 = uStack38._3_1_;
  uStack28 = uStack38;
  uStack8 = uStack38;
  if (uStack38._3_1_ == 0x0) goto LAB_1020_eb4e;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack38 & 0xffff | uVar1 << 0x10);
  uVar11 = SUB42(paVar8,0x0);
  uStack38 = CONCAT22(uVar11,uStack58);
  uStack34 = (uStack58 + 0x2e);
  if ((uStack34 + 0x4) != param_5) {
    PTR_LOOP_1050_50ca = 0x6b7;
    return;
  }
  uStack28 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar11,uStack58),param_5,uVar11);
  paVar8 = (astruct_57 *)(paVar8 & 0xffff0000 | uStack28 >> 0x10);
  uVar11 = (uStack28 >> 0x10);
  uVar1 = (uStack28 + 0xc);
  uStack58 = uVar1;
  if (uVar1 != 0x41) {
    if (0x41 < uVar1) {
      if (uVar1 == 0x6b) {
        PTR_LOOP_1050_50ca = 0x6b1;
        return;
      }
      if (uVar1 < 0x6c) {
        if (uVar1 == 0x42) {
          PTR_LOOP_1050_50ca = 0x6b1;
          return;
        }
        uStack58 = uVar1 - 0x4b;
        if (uStack58 == 0x0) {
          PTR_LOOP_1050_50ca = 0x6b1;
          return;
        }
      }
      else {
        if (uVar1 == 0x6e) {
          return;
        }
        uStack58 = uVar1 - 0x73;
        if ((0x4 < (uVar1 - 0x6e)) && (uStack58 = uVar1 - 0x79, uStack58 == 0x0 || (uVar1 - 0x73) < 0x6)) {
          PTR_LOOP_1050_50ca = 0x6b0;
          return;
        }
      }
      goto LAB_1020_eb4e;
    }
    if (uVar1 != 0x3e) {
      if (uVar1 < 0x3f) {
        cVar3 = (char)uVar1;
        if (cVar3 != '\v') {
          if (cVar3 == '\x10') {
            return;
          }
          uStack58 = uVar1 & 0xff00 | (u8)(cVar3 - 0x37U);
          if ((u8)(cVar3 - 0x37U) != 0x0) goto LAB_1020_eb4e;
        }
        PTR_LOOP_1050_50ca = 0x6b4;
        return;
      }
      goto LAB_1020_eb4e;
    }
  }
  if ((uStack28 + 0x12) == 0x4) {
    PTR_LOOP_1050_50ca = 0x6b1;
    return;
  }//
LAB_1020_eb4e:
  uVar11 = 0x1000;
  mem_op_1000_179c(0xb4,paVar8);
  uStack56 = paVar8;
  uVar10 = paVar8 & 0xffff0000;
  uVar9 = uVar10 | (uStack56 | uStack58);
  if ((uStack56 | uStack58) == 0x0) {
    iStack14 = 0x0;
  }
  else {
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    iStack14 = string_1040_8520(uVar9,(astruct_57 *)
                                      CONCAT13((char)(paVar8 >> 0x8),CONCAT12((char)paVar8,uStack58)),
                                HWND16_1050_0396,0x20024,0x5e8057b);
    uVar10 = uVar9;
  }
  puStack12 = CONCAT22(uVar10,iStack14);
  ppcVar2 = (code **)(*puStack12 + 0x74);
  iVar14 = iStack14;
  (**ppcVar2)(uVar11,iStack14,uVar10);
  if (iStack14 != 0x7) {
    paVar8 = (astruct_57 *)(uVar10 & 0xffff0000 | uStack8 >> 0x10);
    puStack46 = uStack8;
    uStack34 = uStack8;
    uStack34._3_1_ = (u8)(uStack8 >> 0x18);
    uVar5 = uStack34._3_1_;
    if (uStack34._3_1_ != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack8);
      paStack50 = (astruct_419 *)CONCAT22(paVar8,uVar5);
      fn_ptr_1030_7296((astruct_292 *)CONCAT22(paVar8,uVar5));
      pass1_1030_730a(uVar5,(astruct_290 *)paStack50);
      puStack46 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(iVar14,0x2f),in_stack_0000fd5e,
                                  in_stack_0000fe82,in_stack_0000fe88,in_stack_0000fe8c);
      uVar6 = (puStack46 >> 0x10);
      uVar11 = SUB42(puStack46,0x0);
      pass1_1010_ec68(puStack46,paStack50);
      puStack42 = struct_op_1030_73a8(paStack50,uVar11,uVar6);
      puVar7 = (puStack42 >> 0x10);
      puVar4 = puStack42;
      ppcVar2 = (code **)(*puStack42 + 0x24);
      (**ppcVar2)(0x1030,(char)puStack42,puVar7);
      uVar5 = pass1_1028_bc4a(puVar4,puVar7,puStack42);
      (uVar12 + 0x24) = uVar5;
      struct_1030_e4fa((astruct_97 *)CONCAT22(0x1050,local_146),(paStack50 + 0x16));
      fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_146));
    }
    return;
  }
  PTR_LOOP_1050_50ca = (&PTR_LOOP_1050_0000 + 0x1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_ecb0(mut param_1: i16,mut param_2: u16 ,mut param_3: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uStack8: u16;

  uVar2 = (param_3 >> 0x10);
  iVar1 = param_3;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(iVar1 + 0x8));
  if ((iVar1 + 0x12) == 0x1) {
    switch((param_1 + 0x14)) {
    case 0x2:
    case 0x7:
      uStack8 = 0x2;
      break;
    case 0x3:
    case 0x8:
      uStack8 = 0x3;
      break;
    default:
      uStack8 = (param_1 + 0x14);
      break;
    case 0x5:
    case 0x6:
      uStack8 = 0x1;
    }
    (iVar1 + 0x14) = uStack8;
    return;
  }
  pass1_1028_bf22(param_2,param_3 & 0xffff | uVar2 << 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_ed3c(mut param_1: i16,param_2: *mut astruct_15)

{
  StructD **ppSVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  astruct_15 *pstruct15_5_1;
  astruct_15 *pstruct15_5;
  char local_138 [0x112];
  let mut local_12: i16;
  u8 local_10 [0x2];
  u8 local_e [0x2];
  let mut local_c: u32;
  let mut uStack8: u16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pstruct15_5 = (astruct_15 *)(param_2 >> 0x10);
  pstruct15_5_1 = (astruct_15 *)param_2;
  ppSVar1 = &pstruct15_5_1->field16_0x14;
  ppSVar1 = ppSVar1 + -0x1;
  if (ppSVar1 == 0x0) {
    pstruct15_5_1->field15_0x12 = 0x0;
    pass1_1028_b58e(param_2);
    local_c = (param_1 + 0xc);
    uStack8 = (param_1 + 0x10);
    iStack6 = param_1;
    uStack4 = extraout_DX;
    pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_12),
                    (u16 *)CONCAT22(0x1050,local_10),(u16 *)CONCAT22(0x1050,local_e));
    if (local_12 < 0x1) {
      uVar3 = 0x5;
    }
    else {
      uVar3 = 0x6;
    }
    (iStack6 + 0x14) = uVar3;
    if (local_12 < 0x1) {
      iVar4 = 0x5;
    }
    else {
      iVar4 = 0x9;
    }
    pass1_1020_ee3a(iVar4,param_2,iVar4);
    pass1_1028_b58e(param_2);
    uVar2 = (iVar4 + 0x2e);
    pass1_1038_5804(uVar2,0x1,0x1);
    if (0x0 < (pstruct15_5_1 + 0x1)->field0_0x0) {
      pass1_1028_68de((astruct_97 *)CONCAT22(0x1050,local_138),(pstruct15_5_1 + 0x1)->field0_0x0,
                      (uVar2 + 0x4));
      fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_138));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1020_ee3a(mut param_1: i16,param_2: *mut astruct_15,mut param_3: u16 )

{
  let mut uVar1: u32;
  let mut extraout_DX: u16;
  let mut uVar2: u32;
  char local_13c [0x124];
  let mut local_c: u32;
  let mut uStack8: u16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_b58e(param_2);
  local_c = (param_1 + 0xc);
  uStack8 = (param_1 + 0x10);
  iStack6 = param_1;
  uStack4 = extraout_DX;
  uVar2 = pass1_1028_bb24(param_2);
  uVar1 = (iStack6 + 0x2e);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_13c),0x0,0x1,param_3,&local_c,&DAT_1050_1050,
                      (uVar1 + 0x4),uVar2);
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_13c));
  return;
}



StructD * pass1_1020_eed0(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  pass1_1030_dcf4(param_1,(astruct_15 *)param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_2);
  }
  return param_2;
}
pub fn pass1_1020_ef5e(param_1: *mut u16)

{
  *param_1 = 0x0;
  (param_1 + 0x2) = 0x1028;
  pass1_1028_b418(param_1);
  return;
}
pub fn FUN_1020_ef7c(void)

{
  return;
}
pub fn FUN_1020_ef80(void)

{
  return;
}
pub fn FUN_1020_ef84(void)

{
  return;
}
pub fn FUN_1020_ef88(void)

{
  return;
}
pub fn FUN_1020_ef8c(void)

{
  return;
}
pub fn FUN_1020_ef90(void)

{
  return;
}



StructD * pass1_1020_ef94(StructD *param_1,param_2: u8)

{
  pass1_1020_ef5e(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}

