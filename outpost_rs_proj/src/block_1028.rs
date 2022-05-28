//
// Created by cyrex on 2022-05-22.
//

#include "block_1028.h"
pub fn struct_1028_0068(param_1: *mut astruct_57,param_2: *mut astruct_180)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  astruct_180 *iVar2;
  let mut uVar3: u16;

  struct_1028_b354(param_2);
  uVar3 = ((u32)param_2 >> 0x10);
  iVar2 = (astruct_180 *)param_2;
  uVar1 = 0x0;
  (iVar2 + 0x1)->field0_0x0 = 0x0;
  (u32)&iVar2[0x1].field1_0x2 = 0x0;
  param_2.field0_0x0 = 0x8ec;
  iVar2.field1_0x2 = 0x1028;
  mem_op_1000_179c(0xc,param_1);
  uVar2 = param_1 | uVar1;
  if (uVar2 == 0x0) {
    (u32)&iVar2[0x1].field1_0x2 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(param_1,uVar1));
    iVar2[0x1].field1_0x2 = uVar1;
    &iVar2[0x1].field_0x4 = uVar2;
  }
  return;
}
pub fn pass1_1028_00cc(StructD *param_1,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  uVar1 = 0x0;
  ((int)param_2 + 0x20) = 0x0;
  (u32)((int)param_2 + 0x22) = 0x0;
  param_2.field0_0x0 = 0x8ec;
  ((int)param_2 + 0x2) = 0x1028;
  mem_op_1000_179c(0xc,paVar3);
  uVar2 = paVar3 | uVar1;
  if (uVar2 == 0x0) {
    (u32)((int)param_2 + 0x22) = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar3,uVar1));
    ((int)param_2 + 0x22) = uVar1;
    ((int)param_2 + 0x24) = uVar2;
  }
  return;
}
pub fn pass1_1028_0138(param_1: *mut u16)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = ((u32)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0x8ec;
  (iVar4 + 0x2) = 0x1028;
  puVar1 = (u32 *)(iVar4 + 0x22);
  uVar2 = (iVar4 + 0x24);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1028_b418(param_1);
  return;
}
pub fn pass1_1028_0176(mut param_1: u16 ,param_2: *mut astruct_15,mut param_3: u32)

{
  code **ppcVar1;
  u32 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_EDX: u32;
  let mut uVar7: u16;
  astruct_15 *iVar9;
  astruct_15 *uVar8;
  astruct_298 *iVar8;
  astruct_57 *paVar6;

  uVar3 = ((u32)in_EDX >> 0x10);
  iVar9 = (astruct_15 *)param_2;
  uVar8 = (astruct_15 *)((u32)param_2 >> 0x10);
  pass1_1028_b46e(param_1,param_2,param_3);
  puVar2 = (u32 *)iVar9.field25_0x22;
  uVar5 = (iVar9 + 0x1)->field0_0x0;
  uVar4 = uVar5 | puVar2;
  paVar6 = (astruct_57 *)CONCAT22(uVar3,uVar4);
  if (uVar4 != 0x0) {
    ppcVar1 = (code **)*puVar2;
    puVar2 = (u32 *)(**ppcVar1)();
  }
  mem_op_1000_179c(0xc,paVar6);
  uVar5 = paVar6 | puVar2;
  if (uVar5 == 0x0) {
    uVar4 = 0x0;
    uVar5 = 0x0;
  }
  else {
    uVar4 = set_struct_1008_574a((astruct_57 *)CONCAT22(paVar6,puVar2));
  }
  iVar9.field25_0x22 = uVar4;
  (iVar9 + 0x1)->field0_0x0 = uVar5;
  uVar7 = 0x14;
  uVar3 = pass1_1028_b58e(param_2);
  pass1_1030_7f1a(CONCAT22(uVar5,uVar3),uVar7);
  return;
}
pub fn pass1_1028_01ec(u32 *param_1)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (((iVar2 + 0x12) == 0x6) || ((iVar2 + 0x12) == 0x5)) {
    uVar1 = (u32)(iVar2 + 0x14);
    uVar3 = ((u32)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    if (((iVar2 + 0xa6) == 0x14) || ((iVar2 + 0xa8) == 0x10)) {
      pass1_1028_bdac((astruct_15 *)param_1,0x6);
      return;
    }
    pass1_1028_be2a((astruct_15 *)param_1);
  }
  return;
}



u16 write_to_file_1028_0234(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u32;
  BOOL16 BVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  HFILE16 in_stack_0000ffba;
  u16 local_1a [0x3];
  u16 local_14 [0x2];
  let mut uStack16: u16;
  i32 lStack14;
  u8 local_a [0x8];

  BVar2 = write_to_file_1028_b5ec((astruct_731 *)param_1,param_2);
  if (BVar2 != 0x0) {
    uVar4 = (param_1 >> 0x10);
    iVar3 = (int)param_1;
    local_1a[0] = (iVar3 + 0x20);
    BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_1a),(char *)0x2,in_stack_0000ffba);
    if (BVar2 != 0x0) {
      pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)(iVar3 + 0x22));
      uVar1 = (u32)(iVar3 + 0x22);
      local_14[0] = ((int)uVar1 + 0x8);
      uStack16 = local_14[0];
      BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_14),(char *)0x2,in_stack_0000ffba);
      while (BVar2 != 0x0) {
        lStack14 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
        if (lStack14 == 0x0) {
          return 0x1;
        }
        local_14[0] = ((int)lStack14 + 0x4);
        BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_14),(char *)0x2,in_stack_0000ffba);
        if (BVar2 == 0x0) break;
        local_14[0] = ((int)lStack14 + 0x6);
        BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_14),(char *)0x2,in_stack_0000ffba);
        if (BVar2 == 0x0) break;
        local_14[0] = ((int)lStack14 + 0x8);
        BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_14),(char *)0x2,in_stack_0000ffba);
        if (BVar2 == 0x0) break;
        local_14[0] = ((int)lStack14 + 0xa);
        BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_14),(char *)0x2,in_stack_0000ffba);
        if (BVar2 == 0x0) break;
        local_14[0] = ((int)lStack14 + 0xc);
        BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_14),(char *)0x2,in_stack_0000ffba);
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_0374(mut param_1: i16,u8 *param_2,param_3: *mut astruct_373,HFILE16 *param_4)

{
  code **ppcVar1;
  BOOL16 BVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  u16 local_18 [0x2];
  astruct_99 *paStack20;
  u16 local_10 [0x2];
  u16 local_c [0x3];
  let mut uStack6: u16;
  let mut local_4: u16;
  astruct_728 *uVar2;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 != 0x0) {
    BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | (u32)((int)param_3 + 0x20)),0x2);
    if (BVar2 != 0x0) {
      BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_4),0x2);
      if (BVar2 != 0x0) {
        uStack6 = 0x0;
        while( true ) {
          if (local_4 <= uStack6) {
            return;
          }
          paStack20 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
          uVar4 = ((u32)paStack20 >> 0x10);
          uVar2 = (astruct_728 *)paStack20;
          if ((uVar4 | uVar2) == 0x0) {
            paStack20 = NULL;
          }
          else {
            paStack20.field0_0x0 = 0x389a;
            uVar2.field2_0x2 = 0x1008;
            uVar2.field3_0x4 = 0x0;
            uVar2.field4_0x6 = 0x0;
            uVar2.field5_0x8 = 0x0;
            uVar2.field6_0xa = 0x0;
            uVar2.field7_0xc = 0x0;
            paStack20.field0_0x0 = 0x56ce;
            uVar2.field2_0x2 = 0x1018;
          }
          BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,local_10),0x2);
          if (BVar2 == 0x0) break;
          BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,local_c),0x2);
          if (BVar2 == 0x0) break;
          BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,local_18),0x2);
          if (BVar2 == 0x0) break;
          BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)paStack20 & 0xffff0000 | (u32)((int)paStack20 + 0xa)),0x2)
          ;
          if (BVar2 == 0x0) break;
          BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)paStack20 & 0xffff0000 | (u32)((int)paStack20 + 0xc)),0x2)
          ;
          if (BVar2 == 0x0) break;
          ((int)paStack20 + 0x4) = local_10[0];
          uVar3 = switch_1008_72bc(param_4,local_c[0]);
          uVar5 = ((u32)paStack20 >> 0x10);
          ((int)paStack20 + 0x6) = uVar3;
          ((int)paStack20 + 0x8) = local_18[0];
          ppcVar1 = (code **)((int)(u32)(u32)((int)param_3 + 0x22) + 0x8);
          (**ppcVar1)();
          uStack6 += 0x1;
        }
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



u16 pass1_1028_04ee(mut param_1: u32,u32 *param_2)

{
  i16 *piVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  i32 lVar5;
  u8 local_a [0x8];

  *param_2 = 0x0;
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_1 + 0x22));
  do {
    lVar5 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    if (lVar5 == 0x0) {
      return 0x0;
    }
    uVar2 = ((int)lVar5 + 0xc);
    uVar4 = ((u32)param_2 >> 0x10);
    uVar3 = param_2;
    param_2 = param_2 + uVar2;
    piVar1 = (i16 *)((int)param_2 + 0x2);
    *piVar1 = *piVar1 + CARRY2(uVar3,uVar2);
  } while ((((int)param_2 + 0x2) == 0x0) && (param_2 < 0x1e));
  return 0x1;
}
pub fn pass1_1028_0550(param_1: *mut astruct_15)

{
  astruct_57 *in_EDX;
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;

  pass1_1028_be9e(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  if (((int)param_1 + 0x12) == 0x5) {
    uVar3 = 0x0;
    iVar4 = 0x4;
    uVar2 = 0x1;
    uVar1 = pass1_1028_b58e((astruct_15 *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10));
    pass1_1030_7c50(uVar1,in_EDX,(astruct_305 *)CONCAT22((int)in_EDX,uVar1),CONCAT22(uVar3,uVar2),iVar4);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_0582(u32 *param_1,mut param_2: u16 ,param_3: *mut astruct_15)

{
  u32 **ppuVar1;
  i32 *plVar2;
  let mut uVar3: u32;
  code **ppcVar4;
  u8 *puVar5;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u32;
  let mut extraout_DX: u16;
  let mut uVar9: u16;
  let mut extraout_DX_00: u16;
  let mut extraout_DX_01: u16;
  let mut iVar10: i16;
  let mut iVar11: i16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut unaff_CS: u16;
  u8 local_138 [0x10e];
  let mut local_2a: u32;
  astruct_99 *paStack38;
  astruct_99 *paStack34;
  let mut uStack30: u32;
  let mut uStack18: u32;
  let mut uStack14: u32;
  u8 local_a [0x4];
  let mut uStack6: u32;

  uVar12 = ((u32)param_3 >> 0x10);
  iVar10 = (int)param_3;
  uVar8 = (u32)(iVar10 + 0x14);
  uVar13 = (uVar8 >> 0x10);
  iVar11 = (int)uVar8;
  uStack6 = uVar8 & 0xffff0000 | (u32)(iVar11 + 0xa4);
  if (((iVar11 + 0xa6) != 0x0) && ((iVar11 + 0xac) != 0x0)) {
    pass1_1028_081e((int)param_1,param_2,param_3);
    param_1 = (u32*)(iVar10 + 0x20);
    ppuVar1 = (u32 **)((int)uStack6 + 0x8);
    if (*ppuVar1 < param_1 || *ppuVar1 == param_1) {
      puVar5 = local_a;
      ppcVar4 = (code **)((int)(u32)param_3 + 0x40);
      (**ppcVar4)();
      uVar8 = ZEXT24(puVar5);
      param_2 = extraout_DX;
      if (puVar5 == NULL) {
        if (((int)uStack6 + 0x2) == 0xc) {
          uStack14 = pass1_1028_b4f2(param_3);
          param_2 = (uStack14 >> 0x10);
          uVar8 = (u32)((int)uStack14 + 0x1f6);
          plVar2 = (i32 *)((int)uVar8 + 0x170);
          *plVar2 = *plVar2 + 0x1;
          uStack18 = uVar8;
        }
        else {
          uStack18 = _PTR_LOOP_1050_68a2;
          paStack38 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
          uVar9 = ((u32)paStack38 >> 0x10);
          uVar6 = paStack38;
          if ((uVar9 | uVar6) == 0x0) {
            paStack34 = NULL;
          }
          else {
            paStack38.field0_0x0 = 0x389a;
            (uVar6 + 0x2) = 0x1008;
            (uVar6 + 0x4) = 0x0;
            (uVar6 + 0x6) = 0x0;
            (uVar6 + 0x8) = 0x0;
            (uVar6 + 0xa) = 0x0;
            (uVar6 + 0xc) = 0x0;
            paStack38.field0_0x0 = 0x56ce;
            (uVar6 + 0x2) = 0x1018;
            paStack34 = paStack38;
          }
          uVar13 = (uStack6 >> 0x10);
          iVar11 = (int)uStack6;
          uVar14 = ((u32)paStack34 >> 0x10);
          ((int)paStack34 + 0x6) = (iVar11 + 0x2);
          ((int)paStack34 + 0xa) = (iVar11 + 0x6);
          unaff_CS = 0x1020;
          uVar7 = switch_1020_c3b4((iVar11 + 0x2));
          uVar6 = uVar7 * ((int)uStack6 + 0x6);
          uVar8 = (u32)uVar6;
          ((int)paStack34 + 0xc) = uVar6;
          uVar3 = (u32)(iVar10 + 0x22);
          ppcVar4 = (code **)((int)(u32)(u32)(iVar10 + 0x22) + 0x4);
          (**ppcVar4)(0x1020,(int)uVar3,(int)((u32)uVar3 >> 0x10));
          param_2 = extraout_DX_00;
        }
      }
      param_1 = (u32 *)uVar8;
      (iVar10 + 0x20) = 0x0;
    }
  }
  uVar13 = (uStack6 >> 0x10);
  if ((((int)uStack6 + 0x4) != 0x0) && (((int)uStack6 + 0x8) != 0x0)) {
    pass1_1028_081e((int)param_1,param_2,param_3);
    param_1 = (u32*)(iVar10 + 0x20);
    ppuVar1 = (u32 **)((int)uStack6 + 0x8);
    if (*ppuVar1 < param_1 || *ppuVar1 == param_1) {
      param_1 = &local_2a;
      ppcVar4 = (code **)((int)(u32)param_3 + 0x40);
      (**ppcVar4)(unaff_CS,param_3);
      if (param_1 == NULL) {
        uStack18 = _PTR_LOOP_1050_68a2;
        paStack38 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar9 = ((u32)paStack38 >> 0x10);
        uVar6 = paStack38;
        if ((uVar9 | uVar6) == 0x0) {
          paStack34 = NULL;
        }
        else {
          paStack38.field0_0x0 = 0x389a;
          (uVar6 + 0x2) = 0x1008;
          (uVar6 + 0x4) = 0x0;
          (uVar6 + 0x6) = 0x0;
          (uVar6 + 0x8) = 0x0;
          (uVar6 + 0xa) = 0x0;
          (uVar6 + 0xc) = 0x0;
          paStack38.field0_0x0 = 0x56ce;
          (uVar6 + 0x2) = 0x1018;
          paStack34 = paStack38;
        }
        uVar13 = (uStack6 >> 0x10);
        iVar11 = (int)uStack6;
        uVar14 = ((u32)paStack34 >> 0x10);
        ((int)paStack34 + 0x8) = (iVar11 + 0x4);
        ((int)paStack34 + 0xa) = (iVar11 + 0x6);
        uVar7 = pass1_1020_c42e((iVar11 + 0x4));
        param_1 = (u32 *)(uVar7 * ((int)uStack6 + 0x6));
        (u32*)((int)paStack34 + 0xc) = param_1;
        uVar3 = (u32)(iVar10 + 0x22);
        ppcVar4 = (code **)((int)(u32)(u32)(iVar10 + 0x22) + 0x4);
        (**ppcVar4)(0x1020,(int)uVar3,(int)((u32)uVar3 >> 0x10));
      }
      (iVar10 + 0x20) = 0x0;
    }
  }
  if ((iVar10 + 0xc) != 0xe) {
    pass1_1028_b58e((astruct_15 *)((u32)param_3 & 0xffff | (u32)uVar12 << 0x10));
    local_2a = CONCAT22(extraout_DX_01,param_1);
    paStack34 = (astruct_99 *)(u32)((int)param_1 + 0x2e);
    uStack30 = (u32)((int)paStack34 + 0x4);
    pass1_1028_68de((astruct_97 *)CONCAT22(0x1050,local_138),0x1,uStack30);
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_138));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_081e(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15)

{
  i16 *piVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;

  pass1_1028_b58e(param_3);
  uVar4 = (u32)(param_1 + 0x2e);
  iVar2 = ((int)uVar4 + 0x18);
  uVar7 = ((u32)param_3 >> 0x10);
  iVar6 = (int)param_3;
  piVar1 = (i16 *)(iVar6 + 0x20);
  *piVar1 = *piVar1 + 0x1;
  uVar5 = *_PTR_LOOP_1050_65e2;
  uVar3 = ((int)_PTR_LOOP_1050_65e2 + 0x2);
  if (iVar2 < 0xfa) {
    uVar5 &= 0x1;
  }
  else {
    if (0x1c1 < iVar2) {
      if (iVar2 < 0x226) {
        return;
      }
      if ((iVar2 < 0x2ee) && (CONCAT22(uVar3,uVar5) % 0x3 != 0x0)) {
        return;
      }
      piVar1 = (i16 *)(iVar6 + 0x20);
      *piVar1 = *piVar1 + 0x1;
      return;
    }
    uVar5 = ((qword)CONCAT22(uVar3,uVar5) % 0x3);
  }
  if (uVar5 != 0x0) {
    return;
  }
  piVar1 = (i16 *)(iVar6 + 0x20);
  *piVar1 = *piVar1 + -0x1;
  return;
}



StructD * pass1_1028_08c6(StructD *param_1,u8 param_2)

{
  pass1_1028_0138(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_180 * struct_1028_0954(param_1: *mut astruct_180)

{
  astruct_180 *iVar1;
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_180 *)param_1;
  (iVar1 + 0x1)->field0_0x0 = 0x0;
  param_1.field0_0x0 = 0xada;
  iVar1.field1_0x2 = 0x1028;
  iVar1.field11_0xe = 0x4b;
  return param_1;
}



u16 * pass1_1028_0982(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  ((int)param_2 + 0x20) = 0x0;
  param_2.field0_0x0 = 0xada;
  ((int)param_2 + 0x2) = 0x1028;
  ((int)param_2 + 0xe) = 0x4b;
  return &param_2.field0_0x0;
}
pub fn pass1_1028_09b8(param_1: *mut astruct_15)

{
  let mut uVar1: u32;

  uVar1 = pass1_1028_b58e(param_1);
  ((int)uVar1 + 0x14) = 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_09d4(mut param_1: i16,mut param_2: u16 ,mut param_3: u32,param_4: *mut u16,mut param_5: u32,i32 param_6)

{
  let mut iVar1: i16;
  u8 *puVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut uVar7: u16;
  u8 local_6 [0x2];
  BOOL16 BStack4;

  uVar6 = param_3;
  uVar7 = (param_3 >> 0x10);
  BStack4 = pass1_1028_c314(param_1,param_2,uVar6,uVar7,param_4,param_5,(param_5 >> 0x10),param_6);
  if (BStack4 == 0x0) {
    return;
  }
  pass1_1028_c7b6(param_2,uVar6,uVar7,param_4,param_6);
  if ((BStack4 != 0x0) && (BStack4 != 0x4)) {
    if (((int)(BStack4 - 0x5) < 0x1) || ((SBORROW2(BStack4 - 0x5,0x1) || (0x3 < (int)(BStack4 - 0x6))))) {
      if (((uVar6 + 0xc) != 0x3e) && ((uVar6 + 0xc) != 0x41)) {
        return;
      }
      uVar5 = pass1_1030_bcae(local_6,&DAT_1050_1050);
      uVar4 = (uVar5 >> 0x10);
      iVar1 = (int)uVar5;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
      uVar3 = (u32)(iVar1 + 0x10);
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3);
      puVar2 = local_6;
      pass1_1030_bcde(puVar2,&DAT_1050_1050,uVar3 & 0xffff | (u32)uVar4 << 0x10,param_4,param_6);
      if ((int)puVar2 < 0x0) {
        PTR_LOOP_1050_50ca = (u8 *)0x6af;
        return;
      }
      if (0x5 < (int)puVar2) {
        PTR_LOOP_1050_50ca = (u8 *)0x6b5;
        return;
      }
      return;
    }
  }
  PTR_LOOP_1050_50ca = (u8 *)0x6a8;
  return;
}



StructD * pass1_1028_0ab4(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_0b42(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0xbbc;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_0b64(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0xbbc;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn FUN_1028_0b8e(void)

{
  return;
}
pub fn FUN_1028_0b92(void)

{
  return;
}



StructD * pass1_1028_0b96(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_0c24(param_1: *mut astruct_180)

{
  astruct_180 *iVar1;
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_180 *)param_1;
  (iVar1 + 0x1)->field0_0x0 = 0x0;
  iVar1[0x1].field1_0x2 = 0x0;
  param_1.field0_0x0 = (int)s_480_bmp_1050_1721 + 0x3;
  iVar1.field1_0x2 = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_0c50(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  ((int)param_2 + 0x20) = 0x0;
  ((int)param_2 + 0x22) = 0x0;
  param_2.field0_0x0 = (int)s_480_bmp_1050_1721 + 0x3;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_0c84(mut param_1: i16,param_2: *mut astruct_15,mut param_3: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  u32 *puVar3;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  u32 *puVar6;
  u32 *puVar7;
  u8 bStack55;
  u8 local_32 [0xa];
  let mut uStack40: u32;
  let mut uStack36: u32;
  u32 *puStack28;
  let mut local_1a: u32;
  let mut iStack22: i16;
  let mut uStack20: u16;
  let mut iStack18: i16;
  let mut uStack16: u16;
  let mut iStack14: i16;
  let mut local_c: u32;
  let mut iStack8: i16;
  astruct_292 *paStack6;

  pass1_1028_b58e(param_2);
  paStack6 = (astruct_292 *)CONCAT22(extraout_DX,param_1);
  local_c = (u32)(param_1 + 0xc);
  iStack18 = (param_1 + 0x10);
  puStack28 = &local_c;
  uStack16 = extraout_DX;
  iStack14 = iStack18;
  iStack8 = iStack18;
  pass1_1028_bab6(iStack18,extraout_DX,param_2);
  uVar2 = pass1_1030_2fac((astruct_598 *)CONCAT22(uStack16,iStack18));
  local_1a = local_c;
  iStack22 = iStack8;
  uStack36 = CONCAT22(uStack36,&local_1a);
  iStack14 += 0x1;
  uStack20 = uVar2;
  if (iStack14 <= (int)uVar2) {
    puVar7 = (u32 *)CONCAT22(0x1050,local_32);
    iStack22 = iStack14;
    uVar5 = pass1_1028_bb24(param_2);
    uVar4 = (uVar5 >> 0x10);
    puVar3 = &local_1a;
    pass1_1030_64ce(puVar3,uVar4,_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar3),
                    uVar5 & 0xffff | (u32)uVar4 << 0x10,puVar7);
    uStack40 = *puVar3;
    uVar4 = ((int)puVar3 + 0x2);
    bStack55 = (u8)(uStack40 >> 0x18);
    uVar2 = bStack55;
    uStack36 = uStack40;
    if (bStack55 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack40 & 0xffff | (u32)uVar4 << 0x10);
      puVar6 = (u32 *)struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar4,uVar2),uVar2,uVar4);
      uVar2 = puVar6;
      ppcVar1 = (code **)((int)*puVar6 + 0x58);
      (**ppcVar1)();
    }
  }
  pass1_1028_b46e(uVar2,param_2,param_3);
  fn_ptr_1030_7296(paStack6);
  return;
}



u16 pass1_1028_0d80(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  uVar1 = ((int)param_1 + 0x20);
  pass1_1028_1646((astruct_409 *)(param_1 & 0xffff | (u32)uVar2 << 0x10));
  return uVar1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_0d9c(mut param_1: i16,param_2: *mut astruct_15)

{
  code **ppcVar1;
  u32 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  BOOL16 BVar5;
  let mut extraout_DX: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  u32 *puVar8;
  let mut uStack58: u32;
  u8 local_32 [0x6];
  u32 *puStack44;
  let mut uStack40: u32;
  let mut uStack36: u32;
  u32 *puStack28;
  let mut local_1a: u32;
  let mut iStack22: i16;
  let mut uStack20: u16;
  let mut iStack18: i16;
  let mut uStack16: u16;
  let mut iStack14: i16;
  let mut local_c: u32;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_b514(param_2);
  pass1_1028_b58e(param_2);
  local_c = (u32)(param_1 + 0xc);
  iStack18 = (param_1 + 0x10);
  puStack28 = &local_c;
  uStack16 = extraout_DX;
  iStack14 = iStack18;
  iStack8 = iStack18;
  iStack6 = param_1;
  uStack4 = extraout_DX;
  pass1_1028_bab6(iStack18,extraout_DX,param_2);
  uStack20 = pass1_1030_2fac((astruct_598 *)CONCAT22(uStack16,iStack18));
  local_1a = local_c;
  uStack36 = CONCAT22(uStack36,&local_1a);
  iStack22 = iStack14 + 0x1;
  if (iStack22 <= (int)uStack20) {
    puVar8 = (u32 *)CONCAT22(0x1050,local_32);
    iStack14 = iStack22;
    uVar7 = pass1_1028_bb24(param_2);
    uVar6 = (uVar7 >> 0x10);
    puVar2 = &local_1a;
    pass1_1030_64ce(puVar2,uVar6,_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar2),
                    uVar7 & 0xffff | (u32)uVar6 << 0x10,puVar8);
    uStack40 = *puVar2;
    uVar6 = ((int)puVar2 + 0x2);
    uStack58._3_1_ = (u8)(uStack40 >> 0x18);
    uVar3 = uStack58._3_1_;
    if (uStack58._3_1_ != 0x0) {
      uStack36 = uStack40;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack40 & 0xffff | (u32)uVar6 << 0x10);
      uStack58 = (astruct_419 *)CONCAT22(uVar6,uVar3);
      uVar4 = pass1_1030_6fa0(CONCAT22(uVar6,uVar3));
      BVar5 = pass1_1008_c6ae(_u16_1050_06e0,uVar4,0x13);
      if (BVar5 != 0x0) {
        puStack44 = (u32 *)struct_op_1030_73a8(uStack58,BVar5,uVar6);
        ppcVar1 = (code **)((int)*puStack44 + 0x24);
        (**ppcVar1)();
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_0ea6(mut param_1: u16 ,param_2: *mut astruct_15)

{
  let mut puVar1: *mut u16;
  BOOL16 BVar2;
  let mut uVar3: u16;
  astruct_15 *iVar3;
  astruct_15 *uVar4;

  uVar4 = (astruct_15 *)((u32)param_2 >> 0x10);
  iVar3 = (astruct_15 *)param_2;
  if (iVar3.field10_0xc != 0x10) {
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0,iVar3.field10_0xc,0x13);
    if (BVar2 == 0x0) {
      BVar2 = pass1_1008_c6ae(_u16_1050_06e0,iVar3.field10_0xc,0x2);
      if (((BVar2 != 0x0) && (iVar3.field15_0x12 != 0x7)) && (iVar3.field15_0x12 != 0x4)) {
        uVar3 = pass1_1028_1556(BVar2,param_1,(astruct_15 *)((u32)param_2 & 0xffff | ZEXT24(uVar4) << 0x10));
        if (uVar3 == 0x0) goto LAB_1028_0f0a;
        if (iVar3.field15_0x12 == 0x9) {
          iVar3.field15_0x12 = 0x5;
        }
      }
    }
    else if ((int)iVar3.field25_0x22 < 0x1) {
      if ((iVar3.field15_0x12 != 0x5) && (iVar3.field15_0x12 != 0x6)) {
        return;
      }
      fn_ptr_1000_17ce((char *)iVar3.field16_0x14);
      iVar3.field16_0x14 = NULL;//
LAB_1028_0f0a:
      iVar3.field15_0x12 = 0x9;
      return;
    }
    pass1_1028_be2a(param_2);
    if (iVar3.field15_0x12 == 0x5) {
      puVar1 = &iVar3.field25_0x22;
      *puVar1 = *puVar1 - 0x1;
      pass1_1028_b58e((astruct_15 *)((u32)param_2 & 0xffff | ZEXT24(uVar4) << 0x10));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_0fa4(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_15)

{
  BOOL16 BVar1;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar4;
  astruct_15 *iVar2;
  astruct_15 *uVar2;
  u32 *puVar5;
  let mut in_stack_0000fe9c: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffca: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut iVar8: i16;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  pass1_1028_be9e(param_3);
  puVar5 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x2),in_stack_0000fe9c,in_stack_0000ffc0
                           ,in_stack_0000ffc6,in_stack_0000ffca);
  paVar4 = (astruct_57 *)((u32)paVar4 & 0xffff0000 | (u32)puVar5 >> 0x10);
  iVar8 = ((int)puVar5 + 0x82);
  uVar2 = (astruct_15 *)((u32)param_3 >> 0x10);
  iVar2 = (astruct_15 *)param_3;
  if (iVar2.field15_0x12 == 0x5) {
    BVar1 = pass1_1008_c6ae(_u16_1050_06e0,iVar2.field10_0xc,0x2);
    if ((BVar1 != 0x0) && ((PTR_LOOP_1050_4fbc == NULL || (iVar8 != 0x0)))) {
      PTR_LOOP_1050_4fbc = (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1);
      uVar7 = 0x0;
      iVar8 = 0x4;
      uVar6 = 0x1;
      uVar3 = pass1_1028_b58e(param_3);
      pass1_1030_7c50(uVar3,paVar4,(astruct_305 *)CONCAT22((int)paVar4,uVar3),CONCAT22(uVar7,uVar6),iVar8);
    }
    iVar2.field25_0x22 = 0x64;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 pass1_1028_1024(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15)

{
  BOOL16 BVar1;
  u32 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut iStack26: i16;
  let mut iStack24: i16;
  let mut local_16: u32;
  let mut iStack18: i16;
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut uStack12: u32;
  let mut uStack8: u16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_bab6(param_1,param_2,param_3);
  iStack6 = param_1;
  uStack4 = param_2;
  uStack8 = pass1_1030_2fac((astruct_598 *)CONCAT22(param_2,param_1));
  uStack12 = pass1_1028_bb24(param_3);
  uVar6 = pass1_1028_b58e(param_3);
  uStack14 = (uVar6 >> 0x10);
  local_16 = (u32)((int)uVar6 + 0xc);
  iStack26 = 0x0;
  iStack24 = 0x0;
  while( true ) {
    if ((int)uStack8 < iStack26) {
      return iStack24;
    }
    iStack18 = iStack26;
    puVar2 = &local_16;
    pass1_1030_627e(puVar2,(uVar6 >> 0x10),_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar2),uStack12);
    uStack16 = uVar6;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar6 & 0xffff0000 | ZEXT24(puVar2));
    uStack16 = uVar6;
    uVar3 = (uVar6 >> 0x10) | puVar2;
    if (uVar3 == 0x0) break;
    uVar6 = struct_op_1030_73a8((astruct_419 *)(uVar6 & 0xffff0000 | ZEXT24(puVar2)),puVar2,uVar3);
    uVar4 = (uVar6 >> 0x10);
    uVar3 = uVar6;
    uVar5 = uVar4 | uVar3;
    if (uVar6 == 0x0) {
      return iStack24;
    }
    BVar1 = pass1_1008_c6ae(_u16_1050_06e0,(uVar3 + 0xc),0x13);
    uVar6 = CONCAT22(uVar5,uStack16);
    if ((BVar1 != 0x0) && ((uVar3 + 0x12) == 0x5)) {
      iStack24 += 0x1;
    }
    iStack26 += 0x1;
  }
  return iStack24;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 pass1_1028_1106(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15)

{
  BOOL16 BVar1;
  u32 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut iStack26: i16;
  let mut iStack24: i16;
  let mut local_16: u32;
  let mut iStack18: i16;
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut uStack12: u32;
  let mut uStack8: u16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_bab6(param_1,param_2,param_3);
  iStack6 = param_1;
  uStack4 = param_2;
  uStack8 = pass1_1030_2fac((astruct_598 *)CONCAT22(param_2,param_1));
  uStack12 = pass1_1028_bb24(param_3);
  uVar5 = pass1_1028_b58e(param_3);
  uStack14 = (uVar5 >> 0x10);
  local_16 = (u32)((int)uVar5 + 0xc);
  iStack26 = 0x0;
  iStack24 = 0x0;
  while( true ) {
    if ((int)uStack8 < iStack26) {
      return iStack24;
    }
    iStack18 = iStack26;
    puVar2 = &local_16;
    pass1_1030_627e(puVar2,(uVar5 >> 0x10),_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar2),uStack12);
    uStack16 = uVar5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5 & 0xffff0000 | ZEXT24(puVar2));
    uStack16 = uVar5;
    uVar3 = (uVar5 >> 0x10) | puVar2;
    if (uVar3 == 0x0) break;
    uVar5 = struct_op_1030_73a8((astruct_419 *)(uVar5 & 0xffff0000 | ZEXT24(puVar2)),puVar2,uVar3);
    uVar4 = (uVar5 >> 0x10);
    uVar3 = uVar4 | uVar5;
    if (uVar5 == 0x0) {
      return iStack24;
    }
    BVar1 = pass1_1008_c6ae(_u16_1050_06e0,(uVar5 + 0xc),0x13);
    uVar5 = CONCAT22(uVar3,uStack16);
    if (BVar1 != 0x0) {
      iStack24 += 0x1;
    }
    iStack26 += 0x1;
  }
  return iStack24;
}



bool pass1_1028_11de(param_1: *mut astruct_15)

{
  let mut uVar1: u32;

  uVar1 = pass1_1028_b58e(param_1);
  return ((int)uVar1 + 0x10) == 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_15 * pass1_1028_121e(mut param_1: u16 ,param_2: *mut astruct_15)

{
  let mut bVar1: bool;
  u8 extraout_AH;
  u32 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u32;
  astruct_15 *paVar5;
  let mut local_c: u32;
  let mut uStack8: u16;
  let mut uStack6: u32;

  bVar1 = pass1_1028_11de(param_2);
  if (CONCAT11(extraout_AH,bVar1) != 0x0) {
    return param_2;
  }
  uStack6 = pass1_1028_b58e(param_2);
  local_c = (u32)((int)uStack6 + 0xc);
  uStack8 = 0x0;
  uVar4 = pass1_1028_bb24(param_2);
  uVar3 = (uVar4 >> 0x10);
  puVar2 = &local_c;
  pass1_1030_627e(puVar2,uVar3,_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar2),
                  uVar4 & 0xffff | (u32)uVar3 << 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(uVar3,puVar2));
  if ((uVar3 | puVar2) == 0x0) {
    return NULL;
  }
  paVar5 = (astruct_15 *)struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar3,puVar2),puVar2,uVar3 | puVar2);
  return paVar5;
}



u16 pass1_1028_12be(param_1: *mut astruct_15,u32 *param_2)

{
  i16 *piVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut bVar4: bool;
  u8 extraout_AH;
  let mut uVar5: u16;
  astruct_15 *paVar6;
  let mut uVar7: u32;
  let mut uVar8: u32;
  let mut uStack8: u16;

  bVar4 = pass1_1028_11de(param_1);
  if (CONCAT11(extraout_AH,bVar4) == 0x0) {
    paVar6 = pass1_1028_121e(&DAT_1050_1050,param_1);
    ppcVar3 = (code **)((int)(u32)paVar6 + 0x40);
    uVar5 = (**ppcVar3)();
    return uVar5;
  }
  *param_2 = 0x0;
  uVar7 = pass1_1028_b58e(param_1);
  uStack8 = 0x4;
  uVar8 = uVar7;
  do {
    uVar8 = pass1_1030_7c28(uVar8,(uVar8 >> 0x10),uVar7,uStack8);
    uVar2 = param_2;
    param_2 = param_2 + uVar8;
    piVar1 = (i16 *)((int)param_2 + 0x2);
    *piVar1 = *piVar1 + (int)(uVar8 >> 0x10) + CARRY2(uVar2,uVar8);
    uStack8 += 0x1;
  } while ((int)uStack8 < 0xe);
  if (0x1f4 < *param_2) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_134a(param_1: *mut astruct_15)

{
  i16 *piVar1;
  code **ppcVar2;
  BOOL16 BVar3;
  i32 *plVar4;
  let mut uVar5: u16;
  astruct_57 *paVar6;
  let mut uVar7: u16;
  astruct_15 *paVar8;
  i32 lStack26;
  let mut iStack22: i16;
  let mut uStack18: u32;
  let mut uStack10: u32;
  i32 local_6;

  uVar7 = ((u32)param_1 >> 0x10);
  BVar3 = pass1_1008_c6ae(_u16_1050_06e0,((int)param_1 + 0xc),0x13);
  if (BVar3 != 0x0) {
    plVar4 = &local_6;
    ppcVar2 = (code **)((int)(u32)param_1 + 0x40);
    (**ppcVar2)(0x1008,param_1,plVar4,(int)&DAT_1050_1050);
    if (plVar4 != NULL) {
      piVar1 = (i16 *)((int)param_1 + 0x22);
      *piVar1 = *piVar1 + 0x1;
      return;
    }
    uStack10 = 0x1f4 - local_6;
    paVar8 = pass1_1028_121e(&DAT_1050_1050,param_1);
    uVar5 = ((u32)paVar8 >> 0x10);
    uVar7 = SUB42(paVar8,0x0);
    pass1_1028_b58e(paVar8);
    uStack18 = CONCAT22(uVar5,uVar7);
    for (iStack22 = 0x0; iStack22 < 0xa; iStack22 += 0x1) {
      uStack10._0_2_ = (iStack22 * 0x2 + 0x4fbe);
      paVar6 = (astruct_57 *)((u32)(long)(int)uStack10 >> 0x10);
      if ((long)uStack10 < (long)(int)uStack10) {
        paVar6 = (astruct_57 *)(uStack10 >> 0x10);
      }
      lStack26 = CONCAT22((int)paVar6,uStack10);
      pass1_1030_7ddc(uStack10,paVar6,uStack18,
                      CONCAT13((char)((u32)paVar6 >> 0x8),CONCAT12((char)paVar6,uStack10)),iStack22 + 0x4);
      uStack10 -= lStack26;
      if ((long)uStack10 < 0x1) {
        return;
      }
    }
  }
  return;
}



i16 pass1_1028_1416(mut param_1: u16 ,mut param_2: u32)

{
  let mut bVar1: bool;
  u8 extraout_AH;
  let mut iVar2: i16;
  let mut uVar3: u16;
  astruct_15 *paVar4;

  bVar1 = pass1_1028_11de((astruct_15 *)param_2);
  if (CONCAT11(extraout_AH,bVar1) == 0x0) {
    paVar4 = pass1_1028_121e(&DAT_1050_1050,(astruct_15 *)param_2);
    uVar3 = ((u32)paVar4 >> 0x10);
    iVar2 = pass1_1028_1416(uVar3,(u32)paVar4 & 0xffff | (u32)uVar3 << 0x10);
    return iVar2;
  }
  iVar2 = pass1_1028_1024(CONCAT11(extraout_AH,bVar1),param_1,(astruct_15 *)param_2);
  return iVar2 * 0xf;
}



u16 write_to_file_1028_1452(param_1: *mut astruct_731,u8 *param_2)

{
  BOOL16 BVar1;
  let mut uVar2: u16;
  HFILE16 in_stack_0000ffda;
  u16 local_c [0x3];
  u8 *local_6 [0x2];

  BVar1 = write_to_file_1028_b5ec(param_1,(u32)param_2);
  if (BVar1 != 0x0) {
    uVar2 = ((u32)param_1 >> 0x10);
    local_c[0] = ((int)param_1 + 0x22);
    BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffda);
    if (BVar1 != 0x0) {
      local_6[0] = (u8 *)((int)param_1 + 0x20);
      BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffda);
      if (BVar1 != 0x0) {
        local_6[0] = PTR_LOOP_1050_4fbc;
        BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_6),(char *)0x2,in_stack_0000ffda);
        if (BVar1 != 0x0) {
          return 0x1;
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return 0x0;
}
pub fn pass1_1028_14d8(mut param_1: i16,u8 *param_2,param_3: *mut astruct_373,HFILE16 *param_4)

{
  BOOL16 BVar1;
  let mut local_4: u16;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 != 0x0) {
    BVar1 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | (u32)((int)param_3 + 0x22)),0x2);
    if ((BVar1 != 0x0) && (BVar1 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_4),0x2), BVar1 != 0x0)) {
      ((int)param_3 + 0x20) = local_4;
      if ((int)u16_1050_0312 < 0x2) {
        return;
      }
      BVar1 = read_file_1008_7dee(param_4,(u8 *)&PTR_LOOP_1050_4fbc,0x2);
      if (BVar1 != 0x0) {
        return;
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1028_1556(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15)

{
  let mut iVar1: i16;
  u32 *puVar2;
  BOOL16 BVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut iStack26: i16;
  let mut local_16: u32;
  let mut iStack18: i16;
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut uStack12: u32;
  let mut uStack8: u16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_bab6(param_1,param_2,param_3);
  iStack6 = param_1;
  uStack4 = param_2;
  uStack8 = pass1_1030_2fac((astruct_598 *)CONCAT22(param_2,param_1));
  uStack12 = pass1_1028_bb24(param_3);
  uVar7 = pass1_1028_b58e(param_3);
  uStack14 = (uVar7 >> 0x10);
  local_16 = (u32)((int)uVar7 + 0xc);
  iStack26 = 0x1;
  while( true ) {
    if ((int)uStack8 < iStack26) {
      return 0x0;
    }
    iStack18 = iStack26;
    puVar2 = &local_16;
    pass1_1030_627e(puVar2,(uVar7 >> 0x10),_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar2),uStack12);
    uStack16 = uVar7;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7 & 0xffff0000 | ZEXT24(puVar2));
    uStack16 = uVar7;
    uVar4 = (uVar7 >> 0x10) | puVar2;
    if (uVar4 == 0x0) {
      return 0x0;
    }
    uVar7 = struct_op_1030_73a8((astruct_419 *)(uVar7 & 0xffff0000 | ZEXT24(puVar2)),puVar2,uVar4);
    uVar5 = (uVar7 >> 0x10);
    uVar4 = uVar7;
    uVar6 = uVar5 | uVar4;
    if (uVar7 == 0x0) {
      return 0x0;
    }
    iVar1 = (uVar4 + 0xc);
    BVar3 = pass1_1008_c6ae(_u16_1050_06e0,iVar1,0x13);
    uVar7 = CONCAT22(uVar6,uStack16);
    if ((BVar3 == 0x0) && (iVar1 != 0x75)) break;
    if ((uVar4 + 0x12) != 0x9) {
      return 0x1;
    }
    iStack26 += 0x1;
  }
  return 0x0;
}



astruct_409 * pass1_1028_1646(param_1: *mut astruct_409)

{
  astruct_409 *paVar1;
  astruct_409 *uVar2;
  let mut uVar3: u16;

  uVar3 = ((u32)param_1 >> 0x10);
  uVar2 = (astruct_409 *)param_1;
  paVar1 = (astruct_409 *)(uVar2.field32_0x20 + -0x4);
  if (paVar1 < (astruct_409 *)((int)&u16_1050_0008 + 0x1U)) {
    switch(paVar1) {
    case NULL:
      uVar2.field32_0x20 = 0x5;
      break;
    case (astruct_409 *)0x1:
      uVar2.field32_0x20 = 0x6;
      break;
    case (astruct_409 *)0x2:
      uVar2.field32_0x20 = 0x7;
      break;
    case (astruct_409 *)0x3:
      uVar2.field32_0x20 = 0x8;
      break;
    case (astruct_409 *)0x4:
      uVar2.field32_0x20 = 0x9;
      break;
    case (astruct_409 *)0x5:
      uVar2.field32_0x20 = 0xa;
      return uVar2;
    case (astruct_409 *)0x6:
      uVar2.field32_0x20 = 0xb;
      return uVar2;
    case (astruct_409 *)0x7:
      uVar2.field32_0x20 = 0xc;
      return uVar2;
    case (astruct_409 *)0x8:
      uVar2.field32_0x20 = 0xd;
      return uVar2;
    }
    return uVar2;
  }
  uVar2.field32_0x20 = 0x4;
  return paVar1;
}



StructD * pass1_1028_16fe(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_178c(param_1: *mut u16)

{
  struct_1030_dc96((astruct_180 *)param_1);
  *param_1 = 0x1b54;
  ((int)param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_17ae(mut param_1: u16 ,mut param_2: i16,mut param_3: u16 ,mut param_4: i16,mut param_5: u32)

{
  pass1_1030_dcc2(param_1,(astruct_12 *)CONCAT22(param_3,param_2),param_4,param_5);
  CONCAT22(param_3,param_2) = 0x1b54;
  (param_2 + 0x2) = 0x1028;
  return (u16 *)CONCAT22(param_3,param_2);
}
pub fn pass1_1028_17d8(mut param_1: u16 ,param_2: *mut astruct_15)

{
  let mut extraout_DX: u16;

  pass1_1030_df0c(param_1,param_2);
  pass1_1028_b58e(param_2);
  pass1_1038_57dc((u32)(param_1 + 0x2e),0x1,0x3);
  return;
}
pub fn pass1_1028_1812(u32 *param_1)

{
  pass1_1028_bdac((astruct_15 *)param_1,0x2);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_1824(mut param_1: i16,mut param_2: u16 ,mut param_3: u32,u32 *param_4,mut param_5: u32,mut param_6: u32)

{
  BOOL16 BVar1;
  u32 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  let mut unaff_SI: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut in_stack_0000fe6c: u16;
  let mut in_stack_0000ff90: u16;
  let mut in_stack_0000ff96: u16;
  let mut in_stack_0000ff9a: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut local_2a: u32;
  let mut iStack38: i16;
  let mut iStack36: i16;
  let mut uStack34: u16;
  let mut uStack32: u16;
  let mut uStack30: u16;
  let mut uStack28: u16;
  let mut iStack24: i16;
  u32 *puStack22;
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut local_c: u32;
  let mut iStack8: i16;
  let mut uStack6: u32;

  uVar8 = param_3;
  uVar9 = (param_3 >> 0x10);
  pass1_1028_c3aa(uVar8,uVar9,(u16 *)param_4,param_5,param_6);
  if (param_1 == 0x0) {
    return;
  }
  BVar1 = pass1_1028_c314(param_1,param_2,uVar8,uVar9,(u16 *)param_4,param_5,(param_5 >> 0x10),
                          param_6);
  if (BVar1 == 0x0) {
    return;
  }
  puVar2 = &local_c;
  pass1_1030_64ce(puVar2,param_2,_PTR_LOOP_1050_5740,(u16 *)param_4,param_6,(u32 *)CONCAT22(0x1050,puVar2));
  uStack6 = *puVar2;
  uStack30 = ((int)puVar2 + 0x2);
  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,uStack30);
  uVar6 = ((u32)param_4 >> 0x10);
  iStack8 = ((int)param_4 + 0x4);
  puStack22 = (u32 *)(uStack6 & 0xffff | (u32)uStack30 << 0x10);
  uStack32 = uStack6;
  uVar3 = uStack30 >> 0x8;
  if ((char)(uStack30 >> 0x8) != '\0') {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6 & 0xffff | (u32)uStack30 << 0x10);
    uStack30 = paVar5;
    uStack32 = uVar3;
    uStack28 = pass1_1030_6fa0(CONCAT22(uStack30,uVar3));
    if (uStack28 == 0x10) {
      if (iStack8 != 0x0) {
        PTR_LOOP_1050_50ca = (u8 *)0x6ab;
        return;
      }
      return;
    }
    if ((uStack28 == 0x60) || (uStack28 == 0x61)) {
      puStack22 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fe6c,
                                  in_stack_0000ff90,in_stack_0000ff96,in_stack_0000ff9a);
      uVar7 = pass1_1018_04b8((u32)puStack22);
      uStack34 = (uVar7 >> 0x10);
      uStack16 = uVar7;
      iStack36 = ((int)puStack22 + 0x1e);
      iStack24 = iStack36;
      uStack14 = uStack34;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7);
      uVar4 = pass1_1030_2fac((astruct_598 *)CONCAT22(uStack34,iStack36));
      if ((int)uVar4 <= iStack24) {
        PTR_LOOP_1050_50ca = (u8 *)0x6ac;
        return;
      }
      local_2a = *param_4;
      iStack38 = iStack8 + 0x1;
      puVar2 = &local_2a;
      pass1_1028_c7b6(uVar6,uVar8,uVar9,(u16 *)CONCAT22(0x1050,puVar2),param_6);
      if (puVar2 == NULL) {
        return;
      }
      if (puVar2 == (u32 *)((int)&u32_1050_0004 + 0x2)) {
        return;
      }
      return;
    }
  }
  PTR_LOOP_1050_50ca = (u8 *)0x6aa;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_199a(mut param_1: i16,param_2: *mut astruct_15)

{
  i16 *piVar1;
  let mut uVar2: u32;
  let mut in_EDX: u32;
  astruct_57 *paVar3;
  let mut unaff_SI: u16;
  let mut in_stack_0000fd42: u16;
  let mut in_stack_0000fe66: u16;
  let mut in_stack_0000fe6c: u16;
  let mut in_stack_0000fe70: u16;
  i16 *piVar4;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  let mut uVar7: u16;
  let mut local_15e: u16;
  let mut uStack348: u16;
  u32 *puStack50;
  let mut uStack42: u32;
  let mut uStack38: u16;
  i16 *piStack36;
  let mut local_22: i16;
  let mut local_20: u16;
  let mut uStack30: u32;
  u32 *puStack26;
  let mut local_16: i16;
  let mut local_14: u32;
  let mut local_10: u32;
  let mut uStack12: u16;
  let mut uStack10: u32;
  let mut iStack6: i16;
  let mut uStack4: u16;

  piVar1 = (i16 *)((int)param_2 + 0x14);
  *piVar1 = *piVar1 + -0x1;
  if (*piVar1 == 0x0) {
    pass1_1028_b58e(param_2);
    uStack4 = in_EDX;
    uStack10 = (u32)(param_1 + 0x2e);
    iStack6 = param_1;
    pass1_1038_5804(uStack10,0x1,0x3);
    paVar3 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uStack4);
    local_10 = (u32)(iStack6 + 0xc);
    uStack12 = (iStack6 + 0x10);
    puStack50 = &local_10;
    pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_10),(u16 *)CONCAT22(0x1050,&local_16),
                    (u16 *)CONCAT22(0x1050,&local_14),(u16 *)CONCAT22(0x1050,(int)&local_14 + 0x2));
    puStack26 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fd42,
                                in_stack_0000fe66,in_stack_0000fe6c,in_stack_0000fe70);
    uVar2 = (u32)((int)puStack26 + 0x20);
    puVar6 = &local_20;
    uVar7 = SUB42(&DAT_1050_1050,0x0);
    piStack36 = &local_22;
    uVar5 = SUB42(&DAT_1050_1050,0x0);
    piVar4 = piStack36;
    uStack30 = uVar2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2);
    uStack38 = uVar2;
    pass1_1030_5b1c(uVar2 & 0xffff | ZEXT24(piStack36) << 0x10,(u16 *)CONCAT22(uVar5,piVar4),
                    (u16 *)CONCAT22(uVar7,puVar6));
    if (local_22 < local_16 + 0x1) {
      pass1_1030_5b3e(CONCAT22(piStack36,uStack38),local_16 + 0x1,local_20);
      pass1_1030_5b1c(CONCAT22(piStack36,uStack38),(u16 *)CONCAT22(0x1050,&local_22),
                      (u16 *)CONCAT22(0x1050,&local_20));
    }
    uVar5 = (uStack10 >> 0x10);
    uStack42 = (u32)((int)uStack10 + 0x4);
    struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,&local_15e),0x0,0x0,(-(local_16 == 0x0) & 0xffd3) + 0x60,
                        &local_10,&DAT_1050_1050,
                        uStack42 & 0xffff | (u32)((int)uStack10 + 0x6) << 0x10,uStack30);
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_15e));
    local_15e = 0x389a;
    uStack348 = 0x1008;
    pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_10),local_16 + 0x1,local_14,((u32)local_14 >> 0x10));
    struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,&local_15e),0x0,0x0,0x60,&local_10,&DAT_1050_1050,uStack42
                        ,uStack30);
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_15e));
  }
  return;
}
pub fn pass1_1028_1b1e(mut param_1: u32)

{
  ((int)param_1 + 0x14) = 0x7;
  return;
}



StructD * pass1_1028_1b2e(mut param_1: u16 ,StructD *param_2,u8 param_3)

{
  pass1_1030_dcf4(param_1,(astruct_15 *)param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



u16 * struct_1028_1bbc(param_1: *mut astruct_180)

{
  astruct_180 *iVar1;
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_180 *)param_1;
  (iVar1 + 0x1)->field0_0x0 = 0x0;
  iVar1[0x1].field1_0x2 = 0x0;
  param_1.field0_0x0 = 0x1eee;
  iVar1.field1_0x2 = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_1be8(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  ((int)param_2 + 0x20) = 0x0;
  ((int)param_2 + 0x22) = 0x0;
  param_2.field0_0x0 = 0x1eee;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



u16 pass1_1028_1c1c(void)

{
  return 0x0;
}



u16 pass1_1028_1c22(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (((iVar2 + 0x20) != 0x0) && (((iVar2 + 0x12) == 0x5 || ((iVar2 + 0x12) == 0x6)))) {
    if ((iVar2 + 0xc) == 0x16) {
      return 0x19;
    }
    if ((iVar2 + 0xc) == 0x17) {
      return 0x1a;
    }
  }
  uVar1 = pass1_1028_bc1c(param_1 & 0xffff | (u32)uVar3 << 0x10);
  return uVar1;
}
pub fn pass1_1028_1c66(param_1: *mut astruct_15)

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut uVar3: u32;

  if (((int)param_1 + 0x12) != 0x6) {
    return;
  }
  uVar3 = pass1_1028_b4f2(param_1);
  if (*(i32 *)((int)uVar3 + 0x200) != 0x8000002) {
    ppcVar1 = (code **)((int)(u32)param_1 + 0x64);
    iVar2 = (**ppcVar1)();
    if (iVar2 == 0x0) {
      return;
    }
    pass1_1028_cb04(param_1);
    if (iVar2 == 0x0) {
      iVar2 = 0x6;
      goto LAB_1028_1cbd;
    }
    pass1_1028_c952(param_1);
  }
  iVar2 = 0x5;//
LAB_1028_1cbd:
  pass1_1028_bdac(param_1,iVar2);
  return;
}



// WARNING: Could not reconcile some variable overlaps

u16 pass1_1028_1cca(mut param_1: u32,u32 *param_2,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,i32 param_6)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  u8 local_e [0x2];
  let mut local_c: i16;
  let mut local_a: i16;
  let mut local_8: u32;
  let mut uStack4: u16;

  local_8 = *param_2;
  uStack4 = (param_2 + 0x1);
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,local_e),
                  (u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_a));
  local_8 = local_8 & 0xffff | (u32)(local_c - 0x1) << 0x10;
  uVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  uVar1 = pass1_1028_1e14(&local_8,param_3,uVar2,uVar3,0x16,(u16 *)CONCAT22(0x1050,&local_8),param_6);
  if (uVar1 == 0x0) {
    local_8 = local_8 & 0xffff | (u32)(local_c + 0x1) << 0x10;
    uVar1 = pass1_1028_1e14(&local_8,param_3,uVar2,uVar3,0x16,(u16 *)CONCAT22(0x1050,&local_8),param_6);
    if (uVar1 == 0x0) {
      local_8._0_2_ = local_a + -0x1;
      local_8 = local_c;
      uVar1 = pass1_1028_1e14(&local_8,param_3,uVar2,uVar3,0x17,(u16 *)CONCAT22(0x1050,&local_8),param_6);
      if (uVar1 == 0x0) {
        local_8 = CONCAT22(local_8,local_a + 0x1);
        uVar1 = pass1_1028_1e14(&local_8,param_3,uVar2,uVar3,0x17,(u16 *)CONCAT22(0x1050,&local_8),param_6);
        if (uVar1 == 0x0) {
          return uVar1;
        }
      }
    }
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_1da4(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16,mut param_4: u32,i32 param_5)

{
  let mut iVar1: i16;
  u8 *puVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u32;
  u8 local_4 [0x2];

  uVar5 = pass1_1030_bcae(local_4,&DAT_1050_1050);
  uVar4 = (uVar5 >> 0x10);
  iVar1 = (int)uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4);
  uVar3 = (u32)(iVar1 + 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3);
  puVar2 = local_4;
  pass1_1030_bcde(puVar2,&DAT_1050_1050,uVar3 & 0xffff | (u32)uVar4 << 0x10,param_3,param_5);
  if ((int)puVar2 < 0x0) {
    PTR_LOOP_1050_50ca = (u8 *)0x6af;
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1028_1e14(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16,param_6: *mut u16,i32 param_7)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u32;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_6,param_7);
  uVar2 = param_2 | param_1;
  if (uVar2 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    if ((uVar2 | param_1) != 0x0) {
      uVar3 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar2,param_1),uVar2 | param_1,uVar2);
      if (uVar3 != 0x0) {
        iVar1 = ((int)uVar3 + 0xc);
        if (((iVar1 == 0x18) || (iVar1 == 0x3f)) || (iVar1 == param_5)) {
          return 0x1;
        }
      }
    }
  }
  return 0x0;
}



u16 pass1_1028_1e8a(param_1: *mut astruct_15)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar3: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  if ((*(u8 *)((int)param_1 + 0x1a) & 0x2) == 0x0) {
    uVar4 = 0x0;
    uVar5 = 0x23;
    uVar3 = 0x1;
    uVar2 = pass1_1028_b58e((astruct_15 *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10));
    pass1_1030_7d7c(uVar2,(u8 *)(uVar2 >> 0x10),(astruct_398 *)uVar2,uVar3,CONCAT22(uVar5,uVar4));
    pass1_1028_bdac(param_1,0x6);
    return 0x0;
  }
  return 0x1;
}



StructD * pass1_1028_1ec8(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Unable to use type for symbol uVar1
pub fn struct_1028_1f56(param_1: *mut astruct_57,param_2: *mut astruct_180)

{
  let mut extraout_DX: u16;
  let mut uVar2: u16;
  astruct_180 *iVar3;
  let mut uVar3: u16;
  let mut uVar1: u32;

  struct_1028_b354(param_2);
  uVar3 = ((u32)param_2 >> 0x10);
  iVar3 = (astruct_180 *)param_2;
  uVar2 = 0x0;
  (u32)(iVar3 + 0x1) = 0x0;
  &iVar3[0x1].field_0x4 = 0x0;
  param_2.field0_0x0 = 0x2572;
  iVar3.field1_0x2 = 0x1028;
  mem_op_1000_179c(0xc,param_1);
  extraout_DX = param_1 | uVar2;
  if (extraout_DX == 0x0) {
    (u32)(iVar3 + 0x1) = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(param_1,uVar2));
    (iVar3 + 0x1)->field0_0x0 = uVar2;
    iVar3[0x1].field1_0x2 = extraout_DX;
  }
  uVar1 = (u32)(iVar3 + 0x1);
  ((int)uVar1 + 0xa) = 0x0;
  return;
}
pub fn pass1_1028_1fc8(u8 *param_1,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar4;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  uVar2 = 0x0;
  (u32)((int)param_2 + 0x20) = 0x0;
  ((int)param_2 + 0x24) = 0x0;
  param_2.field0_0x0 = 0x2572;
  ((int)param_2 + 0x2) = 0x1028;
  mem_op_1000_179c(0xc,paVar4);
  uVar3 = paVar4 | uVar2;
  if (uVar3 == 0x0) {
    (u32)((int)param_2 + 0x20) = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar4,uVar2));
    ((int)param_2 + 0x20) = uVar2;
    ((int)param_2 + 0x22) = uVar3;
  }
  uVar1 = (u32)((int)param_2 + 0x20);
  ((int)uVar1 + 0xa) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_2042(param_1: *mut astruct_15)

{
  astruct_21 *paVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut uVar4: u32;
  astruct_15 *iVar5;
  astruct_15 *uVar5;
  let mut uVar6: u32;

  uVar5 = (astruct_15 *)((u32)param_1 >> 0x10);
  iVar5 = (astruct_15 *)param_1;
  param_1.field0_0x0 = 0x2572;
  iVar5.field1_0x2 = 0x1028;
  uVar4 = (u32)&iVar5.field24_0x20;
  ((int)uVar4 + 0xa) = 0x1;
  paVar1 = iVar5.field24_0x20;
  uVar2 = iVar5.field25_0x22;
  if ((uVar2 | paVar1) != 0x0) {
    ppcVar3 = (code **)(u32)paVar1;
    (**ppcVar3)();
  }
  if ((_PTR_LOOP_1050_65e2 != 0x0) && (_PTR_LOOP_1050_5a64 != 0x0)) {
    uVar6 = pass1_1028_b58e(param_1);
    pass1_1038_79f2(_PTR_LOOP_1050_5a64,uVar6,&DAT_1050_1050);
  }
  pass1_1028_b418(&param_1.field0_0x0);
  return;
}



u16 pass1_1028_20b0(void)

{
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_20b6(param_1: *mut astruct_15)

{
  let mut uVar1: u16;
  u8 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 auStack22 [0x2];
  let mut iStack20: i16;
  let mut iStack18: i16;
  let mut uStack16: u32;
  let mut uStack12: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  pass1_1028_be9e(param_1);
  uVar4 = ((u32)param_1 >> 0x10);
  uVar3 = param_1;
  if ((uVar3 + 0x12) == 0x5) {
    uStack6 = pass1_1028_bb24((astruct_15 *)((u32)param_1 & 0xffff | (u32)uVar4 << 0x10));
    uStack10 = pass1_1028_b58e(param_1);
    puVar2 = (u8 *)(uStack10 >> 0x10);
    uStack16 = (u32)((int)uStack10 + 0xc);
    uStack12 = ((int)uStack10 + 0x10);
    pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&uStack16),(u16 *)CONCAT22(0x1050,auStack22),
                    (u16 *)CONCAT22(0x1050,&iStack20),(u16 *)CONCAT22(0x1050,&iStack18));
    uStack16 = uStack16 & 0xffff | (u32)(iStack20 - 0x1) << 0x10;
    uVar1 = pass1_1028_21ba(&uStack16,puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,&uStack16),uStack6);
    if (uVar1 == 0x0) {
      uStack16 = uStack16 & 0xffff | (u32)(iStack20 + 0x1) << 0x10;
      uVar1 = pass1_1028_21ba(&uStack16,puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,&uStack16),uStack6);
      if (uVar1 == 0x0) {
        uStack16 = CONCAT22(iStack20,iStack18 + -0x1);
        uVar1 = pass1_1028_21ba(&uStack16,puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,&uStack16),uStack6);
        if (uVar1 == 0x0) {
          uStack16 = uStack16 & 0xffff0000 | (u32)(iStack18 + 0x1);
          uVar1 = pass1_1028_21ba(&uStack16,puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,&uStack16),uStack6)
          ;
          if (uVar1 == 0x0) {
            return;
          }
        }
      }
    }
    pass1_1038_79b2(uVar1,puVar2,_PTR_LOOP_1050_5a64,uStack10);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1028_21ba(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut u16,i32 param_6)

{
  let mut uVar1: u16;
  let mut uVar2: u32;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_5,param_6);
  uVar1 = param_2 | param_1;
  if (uVar1 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    if ((uVar1 | param_1) != 0x0) {
      uVar2 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar1,param_1),uVar1 | param_1,uVar1);
      if ((uVar2 != 0x0) && (((int)uVar2 + 0xc) == 0x40)) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16
pass1_1028_2220(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16,param_6: *mut u16,i32 param_7
               )

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u32;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_6,param_7);
  uVar2 = param_2 | param_1;
  if (uVar2 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    if ((uVar2 | param_1) != 0x0) {
      uVar3 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar2,param_1),uVar2 | param_1,uVar2);
      if ((uVar3 != 0x0) && ((iVar1 = ((int)uVar3 + 0xc), iVar1 == 0x40 || (iVar1 == param_5)))) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps

i16 pass1_1028_2290(mut param_1: u32,u32 *param_2,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,i32 param_6)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  u8 local_e [0x2];
  let mut local_c: i16;
  let mut local_a: i16;
  let mut local_8: u32;
  let mut uStack4: u16;

  local_8 = *param_2;
  uStack4 = (param_2 + 0x1);
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,local_e),
                  (u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_a));
  local_8 = local_8 & 0xffff | (u32)(local_c - 0x1) << 0x10;
  uVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  iVar1 = pass1_1028_2220(&local_8,param_3,uVar2,uVar3,0x16,(u16 *)CONCAT22(0x1050,&local_8),param_6);
  if (iVar1 == 0x0) {
    local_8 = local_8 & 0xffff | (u32)(local_c + 0x1) << 0x10;
    iVar1 = pass1_1028_2220(&local_8,param_3,uVar2,uVar3,0x16,(u16 *)CONCAT22(0x1050,&local_8),param_6);
    if (iVar1 == 0x0) {
      local_8._0_2_ = local_a + -0x1;
      local_8 = local_c;
      iVar1 = pass1_1028_2220(&local_8,param_3,uVar2,uVar3,0x17,(u16 *)CONCAT22(0x1050,&local_8),param_6);
      if (iVar1 == 0x0) {
        local_8 = CONCAT22(local_8,local_a + 0x1);
        iVar1 = pass1_1028_2220(&local_8,param_3,uVar2,uVar3,0x17,(u16 *)CONCAT22(0x1050,&local_8),param_6);
        if (iVar1 == 0x0) {
          return iVar1;
        }
      }
    }
  }
  return 0x1;
}



u16 pass1_1028_236a(param_1: *mut astruct_15)

{
  let mut uVar1: u16;
  astruct_398 *paVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  if ((*(u8 *)((int)param_1 + 0x1a) & 0x2) == 0x0) {
    uVar4 = 0x0;
    uVar5 = 0x23;
    uVar3 = 0x1;
    paVar2 = (astruct_398 *)pass1_1028_b58e((astruct_15 *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10));
    pass1_1030_7d7c(paVar2,(u8 *)((u32)paVar2 >> 0x10),paVar2,uVar3,CONCAT22(uVar5,uVar4));
    pass1_1028_bdac(param_1,0x6);
    return 0x0;
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_23a8(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16,mut param_4: u32,i32 param_5)

{
  let mut iVar1: i16;
  u8 *puVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u32;
  u8 local_4 [0x2];

  uVar5 = pass1_1030_bcae(local_4,&DAT_1050_1050);
  uVar4 = (uVar5 >> 0x10);
  iVar1 = (int)uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4);
  uVar3 = (u32)(iVar1 + 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3);
  puVar2 = local_4;
  pass1_1030_bcde(puVar2,&DAT_1050_1050,uVar3 & 0xffff | (u32)uVar4 << 0x10,param_3,param_5);
  if ((int)puVar2 < 0x0) {
    PTR_LOOP_1050_50ca = (u8 *)0x6af;
    return;
  }
  return;
}



BOOL16 pass1_1028_2418(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u32;
  BOOL16 BVar2;
  let mut uVar3: u16;
  let mut uVar4: u32;
  HFILE16 in_stack_0000ffce;
  u16 local_1c [0x6];
  let mut uStack16: u16;
  let mut iStack14: i16;
  let mut uStack12: u16;
  u8 local_a [0x8];

  BVar2 = write_to_file_1028_b5ec((astruct_731 *)param_1,param_2);
  if (BVar2 != 0x0) {
    uVar3 = (param_1 >> 0x10);
    pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_1 + 0x20));
    uVar1 = (u32)((int)param_1 + 0x20);
    local_1c[0] = ((int)uVar1 + 0x8);
    uStack16 = local_1c[0];
    BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_1c),(char *)0x2,in_stack_0000ffce);
    if (BVar2 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return BVar2;
    }
    while( true ) {
      uVar4 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
      iStack14 = (int)uVar4;
      if (uVar4 == 0x0) break;
      pass1_1038_75ca(iStack14,uVar4,param_2);
      uStack12 = (uVar4 >> 0x10);
      if ((BOOL16)uVar4 == 0x0) {
        return (BOOL16)uVar4;
      }
    }
    BVar2 = 0x1;
  }
  return BVar2;
}



BOOL16 file_1028_24a2(mut param_1: i16,u8 *param_2,param_3: *mut astruct_373,HFILE16 *param_4)

{
  code **ppcVar1;
  BOOL16 BVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  HFILE16 *pHVar7;
  let mut uStack6: u16;
  let mut local_4: u16;
  astruct_57 *paVar6;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 == 0x0) {
    return 0x0;
  }
  if (0x1 < (int)u16_1050_0312) {
    BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_4),0x2);
    if (BVar2 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return 0x0;
    }
    for (uStack6 = 0x0; uStack6 < local_4; uStack6 += 0x1) {
      uVar3 = local_4;
      pHVar7 = param_4;
      mem_op_1000_179c(0x2a,paVar5);
      uVar4 = paVar5 | uVar3;
      paVar6 = (astruct_57 *)((u32)paVar5 & 0xffff0000 | (u32)uVar4);
      if (uVar4 == 0x0) {
        uVar3 = 0x0;
        paVar5 = (astruct_57 *)((u32)paVar5 & 0xffff0000);
      }
      else {
        struct_1038_6520((astruct_308 *)CONCAT22(paVar5,uVar3));
        paVar5 = paVar6;
      }
      file_1038_774e((u8 *)paVar5,(astruct_169 *)CONCAT22((u8 *)paVar5,uVar3),(u32)pHVar7);
      if (uVar3 == 0x0) {
        return 0x0;
      }
      ppcVar1 = (code **)((int)(u32)(u32)((int)param_3 + 0x20) + 0x8);
      (**ppcVar1)();
    }
  }
  return 0x1;
}



StructD * pass1_1028_254c(StructD *param_1,u8 param_2)

{
  pass1_1028_2042((astruct_15 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_25da(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
    // just 0x264c
  param_1.field0_0x0 = (int)s_fem16_wav_1050_2644 + 0x8;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_25fc(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = (int)s_fem16_wav_1050_2644 + 0x8;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



StructD * pass1_1028_2626(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_26b4(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x2788;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_26d6(StructD *param_1,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x2788;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn pass1_1028_2700(param_1: *mut astruct_15)

{
  let mut uVar1: u16;
  let mut uVar2: u32;

  pass1_1028_be2a(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  if (((int)param_1 + 0x12) == 0x5) {
    uVar2 = pass1_1028_b4f2((astruct_15 *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10));
    ((int)uVar2 + 0x204) = 0x1;
  }
  return;
}
pub fn pass1_1028_272e(param_1: *mut astruct_15)

{
  let mut uVar1: u32;

  pass1_1028_be9e(param_1);
  uVar1 = pass1_1028_b4f2(param_1);
  if (((int)param_1 + 0x12) == 0x5) {
    ((int)uVar1 + 0x204) = 0x1;
  }
  return;
}



StructD * pass1_1028_2762(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_180 * struct_1028_27f0(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x2a92;
  ((int)param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_2812(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x2a92;
  ((int)param_2 + 0x2) = 0x1028;
  ((int)param_2 + 0xe) = ((int)param_2 + 0xc);
  return &param_2.field0_0x0;
}



// WARNING: Could not reconcile some variable overlaps

u16 pass1_1028_2844(mut param_1: u32,u32 *param_2,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,i32 param_6)

{
  BOOL16 BVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 local_e [0x2];
  let mut local_c: i16;
  let mut local_a: i16;
  let mut local_8: u32;
  let mut uStack4: u16;

  local_8 = *param_2;
  uStack4 = (param_2 + 0x1);
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,local_e),
                  (u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_a));
  local_8 = local_8 & 0xffff | (u32)(local_c - 0x1) << 0x10;
  uVar3 = param_1;
  uVar4 = (param_1 >> 0x10);
  BVar1 = pass1_1028_c5a6(&local_8,param_3,uVar3,uVar4,0x7b,(u16 *)CONCAT22(0x1050,&local_8),param_6);
  if (BVar1 == 0x0) {
    uVar2 = pass1_1028_297c(&local_8,param_3,param_1,(u16 *)CONCAT22(0x1050,&local_8),param_6);
    if (uVar2 == 0x0) {
      local_8 = local_8 & 0xffff | (u32)(local_c + 0x1) << 0x10;
      BVar1 = pass1_1028_c5a6(&local_8,param_3,uVar3,uVar4,0x7b,(u16 *)CONCAT22(0x1050,&local_8),param_6);
      if (BVar1 == 0x0) {
        uVar2 = pass1_1028_297c(&local_8,param_3,param_1,(u16 *)CONCAT22(0x1050,&local_8),param_6);
        if (uVar2 == 0x0) {
          local_8._0_2_ = local_a + -0x1;
          local_8 = local_c;
          BVar1 = pass1_1028_c5a6(&local_8,param_3,uVar3,uVar4,0x7c,(u16 *)CONCAT22(0x1050,&local_8),param_6);
          if (BVar1 == 0x0) {
            uVar2 = pass1_1028_297c(&local_8,param_3,param_1,(u16 *)CONCAT22(0x1050,&local_8),param_6);
            if (uVar2 == 0x0) {
              local_8 = CONCAT22(local_8,local_a + 0x1);
              BVar1 = pass1_1028_c5a6(&local_8,param_3,uVar3,uVar4,0x7c,(u16 *)CONCAT22(0x1050,&local_8),
                                      param_6);
              if (BVar1 == 0x0) {
                uVar3 = pass1_1028_297c(&local_8,param_3,param_1,(u16 *)CONCAT22(0x1050,&local_8),param_6);
                if (uVar3 == 0x0) {
                  return uVar3;
                }
              }
            }
          }
        }
      }
    }
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1028_297c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,param_4: *mut u16,i32 param_5)

{
  char cVar1;
  let mut uVar2: u16;
  let mut uVar3: u32;

  pass1_1028_c7b6(param_2,param_3,(param_3 >> 0x10),param_4,param_5);
  if (param_1 == 0x0) {
    pass1_1030_627e(0x0,param_2,_PTR_LOOP_1050_5740,param_4,param_5);
    uVar2 = param_2 | param_1;
    if (uVar2 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
      uVar3 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar2,param_1),param_1,uVar2);
      uVar2 = ((int)uVar3 + 0xc);
      if (0x4a < (int)uVar2) {
        switch(uVar2) {
        case 0x4c:
        case 0x4d:
        case 0x4e:
        case 0x60:
        case 0x61:
        case 0x62:
        case 0x63:
        case 0x6e:
        case 0x73:
        case 0x74:
        case 0x75:
        case 0x76:
        case 0x77:
        case 0x78:
        case 0x79:
          goto switchD_1028_2a0b_caseD_4c;
        default:
          goto switchD_1028_2a0b_caseD_4f;
        }
      }
      if (((int)uVar2 < 0x48) && (uVar2 != 0x41)) {
        if (uVar2 < 0x42) {
          cVar1 = (char)uVar2;
          if (cVar1 < '5') {
            if ('2' < cVar1) {
              return 0x0;
            }
            cVar1 += -0x10;
          }
          else {
            cVar1 += -0x3e;
          }
          if (cVar1 == '\0') {
            return 0x0;
          }
        }
switchD_1028_2a0b_caseD_4f:
        return 0x1;
      }
    }
  }
switchD_1028_2a0b_caseD_4c:
  return 0x0;
}



StructD * pass1_1028_2a6c(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_2afa(param_1: *mut u16)

{
  struct_1030_dc96((astruct_180 *)param_1);
  *param_1 = 0x2b74;
  ((int)param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_2b1c(mut param_1: u16 ,mut param_2: i16,mut param_3: u16 ,mut param_4: i16,mut param_5: u32)

{
  pass1_1030_dcc2(param_1,(astruct_12 *)CONCAT22(param_3,param_2),param_4,param_5);
  CONCAT22(param_3,param_2) = 0x2b74;
  (param_2 + 0x2) = 0x1028;
  return (u16 *)CONCAT22(param_3,param_2);
}
pub fn FUN_1028_2b46(void)

{
  return;
}
pub fn FUN_1028_2b4a(void)

{
  return;
}



StructD * pass1_1028_2b4e(mut param_1: u16 ,StructD *param_2,u8 param_3)

{
  pass1_1030_dcf4(param_1,(astruct_15 *)param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



u16 * struct_1028_2bdc(param_1: *mut u16)

{
  struct_1028_0954((astruct_180 *)param_1);
  *param_1 = 0x341c;
  ((int)param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_2bfe(mut param_1: u16 ,param_2: *mut astruct_179,mut param_3: u16 ,mut param_4: i16,mut param_5: u32)

{
  pass1_1028_0982(param_1,(astruct_12 *)CONCAT22(param_3,param_2),param_4,param_5);
  CONCAT22(param_3,param_2) = 0x341c;
  param_2.field2_0x2 = 0x1028;
  return (u16 *)CONCAT22(param_3,param_2);
}



u16 pass1_1028_2c28(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,u32 *param_4,mut param_5: u32,mut param_6: u32)

{
  u32 *puVar1;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut local_e: u16;
  let mut local_c: u16;
  let mut local_a: u16;
  let mut local_8: u32;
  let mut uStack4: u16;

  pass1_1028_09d4(param_1,param_2,param_3,(u16 *)param_4,param_5,param_6);
  if (param_1 != 0x0) {
    local_8 = *param_4;
    uStack4 = ((int)param_4 + 0x4);
    puVar2 = &local_e;
    pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,puVar2),
                    (u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_a));
    pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c - 0x1,local_a - 0x1);
    puVar1 = &local_8;
    uVar3 = param_3;
    uVar4 = (param_3 >> 0x10);
    pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
    if (puVar1 != NULL) {
      pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c - 0x1,local_a);
      puVar1 = &local_8;
      pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
      if (puVar1 != NULL) {
        pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c - 0x1,local_a + 0x1);
        puVar1 = &local_8;
        pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
        if (puVar1 != NULL) {
          pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c,local_a - 0x1);
          puVar1 = &local_8;
          pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
          if (puVar1 != NULL) {
            pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c,local_a);
            puVar1 = &local_8;
            pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
            if (puVar1 != NULL) {
              pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c,local_a + 0x1);
              puVar1 = &local_8;
              pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
              if (puVar1 != NULL) {
                pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c + 0x1,local_a - 0x1);
                puVar1 = &local_8;
                pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
                if (puVar1 != NULL) {
                  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c + 0x1,local_a);
                  puVar1 = &local_8;
                  pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
                  if (puVar1 != NULL) {
                    pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c + 0x1,local_a + 0x1);
                    puVar1 = &local_8;
                    pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
                    if (puVar1 != NULL) {
                      return 0x1;
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
    PTR_LOOP_1050_50ca = (u8 *)0x6a8;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_2e40(uchar param_1,mut param_2: i16,u8 *param_3,u32 *param_4)

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_27 *paVar2;
  let mut in_stack_0000fe9e: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000ffcc: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_3);
  pass1_1028_be9e((astruct_15 *)param_4);
  if (((int)param_4 + 0x12) == 0x5) {
    pass1_1028_2f18(param_1,param_2,(astruct_15 *)param_4);
    pass1_1028_3246(param_2,paVar1,(astruct_15 *)param_4);
    uVar3 = 0x1;
    iVar4 = 0x5;
    paVar2 = (astruct_27 *)
             mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fe9e,in_stack_0000ffc2,
                             in_stack_0000ffc8,in_stack_0000ffcc);
    pass1_1010_089e(paVar2,uVar3,iVar4);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_2e84(StructD *param_1,param_2: *mut astruct_15,mut param_3: u16 ,mut param_4: i16)

{
  astruct_57 *paVar1;
  astruct_67 *paVar2;
  astruct_27 *paVar3;
  u32 *puVar4;
  let mut in_stack_0000fe88: u16;
  let mut in_stack_0000fe90: u16;
  let mut in_stack_0000fe96: u16;
  let mut in_stack_0000ffac: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut iVar11: i16;

  pass1_1028_09b8(param_2);
  if (param_4 == 0x0) {
    uVar10 = 0x0;
    iVar11 = 0x8;
    uVar8 = 0x1;
    uVar9 = 0x0;
    uVar6 = 0x0;
    iVar7 = 0x0;
    uVar5 = 0x0;
    paVar2 = (astruct_67 *)
             mixed_1010_20ba((astruct_57 *)param_1,_u16_1050_0ed0,(u8 **)0x37,in_stack_0000fe88,in_stack_0000ffac
                             ,in_stack_0000ffb2,in_stack_0000ffb6);
    paVar1 = (astruct_57 *)((u32)param_1 & 0xffff0000 | (u32)paVar2 >> 0x10);
    post_win_msg_1008_a0e4(paVar2,CONCAT22(uVar6,uVar5),iVar7,uVar8,CONCAT22(uVar10,uVar9),iVar11);
    uVar6 = 0x400;
    iVar7 = 0x3;
    uVar5 = 0x1;
    paVar3 = (astruct_27 *)
             mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fe90,in_stack_0000ffb4,
                             in_stack_0000ffba,in_stack_0000ffbe);
    paVar1 = (astruct_57 *)((u32)paVar1 & 0xffff0000 | (u32)paVar3 >> 0x10);
    pass1_1010_043a(paVar3,CONCAT22(uVar6,uVar5),iVar7);
    pass1_1010_043a(paVar3,0x4000001,0x4);
    puVar4 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x2f),in_stack_0000fe96,
                             in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
    paVar1 = (astruct_57 *)((u32)paVar1 & 0xffff0000 | (u32)puVar4 >> 0x10);
    pass1_1010_ec84((u32)puVar4);
    puVar4 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x8),in_stack_0000fe96,
                             in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
    pass1_1010_9766(((u32)puVar4 >> 0x10),(u32)puVar4);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_2f18(uchar param_1,mut param_2: i16,param_3: *mut astruct_15)

{
  let mut iVar1: i16;
  u32 *puVar2;
  let mut extraout_DX: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut puVar5: *mut u16;
  u8 local_944 [0x124];
  u8 local_820 [0x124];
  u8 local_6fc [0x124];
  u8 local_5d8 [0x124];
  u8 local_4b4 [0x124];
  let mut local_390: u32;
  u8 local_38a [0x124];
  u8 local_266 [0x124];
  u8 local_142 [0x124];
  let mut local_1e: u32;
  let mut local_1a: u16;
  let mut local_18: u32;
  let mut uStack20: u16;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uStack6 = pass1_1028_bb24(param_3);
  iVar1 = (int)uStack6;
  pass1_1028_b58e(param_3);
  uStack10 = CONCAT22(extraout_DX,iVar1);
  uStack14 = (u32)(iVar1 + 0x2e);
  uStack18 = (u32)((int)uStack14 + 0x4);
  local_18 = (u32)(iVar1 + 0xc);
  uStack20 = (iVar1 + 0x10);
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_18),(u16 *)CONCAT22(0x1050,&local_1e),
                  (u16 *)CONCAT22(0x1050,(int)&local_1e + 0x2),(u16 *)CONCAT22(0x1050,&local_1a));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_18),local_1e,local_1e - 0x1,local_1a - 0x1);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_142),0x0,0x0,0xd,&local_18,&DAT_1050_1050,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_142));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_18),local_1e,local_1e + 0x1,local_1a + 0x1);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_266),0x0,0x0,0xc,&local_18,&DAT_1050_1050,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_266));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_18),local_1e,local_1e + 0x1,local_1a - 0x1);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_38a),0x0,0x0,0xe,&local_18,&DAT_1050_1050,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_38a));
  puVar5 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,&local_390),local_1e,local_1e - 0x1,local_1a + 0x1);
  uVar3 = ((u32)puVar5 >> 0x10);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_4b4),0x0,0x0,0xb,&local_390,&DAT_1050_1050,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_4b4));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_18),local_1e,local_1e - 0x1,local_1a);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_5d8),0x0,0x0,0x7a,&local_18,&DAT_1050_1050,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_5d8));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_18),local_1e,((u32)local_1e >> 0x10),local_1a + 0x1);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_6fc),0x0,0x0,0x7a,&local_18,&DAT_1050_1050,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_6fc));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_18),local_1e,local_1e + 0x1,local_1a);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_820),0x0,0x0,0x7a,&local_18,&DAT_1050_1050,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_820));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_18),local_1e,((u32)local_1e >> 0x10),local_1a - 0x1);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_944),0x0,0x0,0x7a,&local_18,&DAT_1050_1050,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_944));
  puVar2 = &local_390;
  pass1_1030_627e(puVar2,uVar3,_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar2),uStack6);
  uVar4 = ((u32)uStack14 >> 0x10);
  (u32*)((int)uStack14 + 0x10) = puVar2;
  ((int)uStack14 + 0x12) = uVar3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_3246(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  let mut in_stack_0000fe80: u16;
  let mut in_stack_0000ffa4: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffae: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut in_stack_0000ffd8: u16;
  u8 local_20 [0x6];
  u32 *puStack26;
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  pass1_1028_b58e(param_3);
  uStack6 = CONCAT22((int)paVar3,param_1);
  uStack10 = (u32)(param_1 + 0x2e);
  uVar2 = (u32)((int)uStack10 + 0x10);
  uVar5 = 0x0;
  iVar6 = 0x1;
  uVar4 = 0x1;
  uStack14 = uVar2;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2);
  uVar1 = uVar2;
  uStack16 = SUB42(paVar3,0x0);
  uStack18 = uVar1;
  pass1_1030_7c50(uVar1,paVar3,(astruct_305 *)(uVar2 & 0xffff | (long)paVar3 << 0x10),CONCAT22(uVar5,uVar4),iVar6);
  pass1_1030_7c50(uVar1,paVar3,(astruct_305 *)CONCAT22(uStack16,uStack18),0x1,0x2);
  pass1_1030_7c50(uVar1,paVar3,(astruct_305 *)CONCAT22(uStack16,uStack18),0x1,0x3);
  pass1_1030_7c50(uVar1,paVar3,(astruct_305 *)CONCAT22(uStack16,uStack18),0x1,0x5);
  puStack26 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffd8,0x2),in_stack_0000fe80,
                              in_stack_0000ffa4,in_stack_0000ffaa,in_stack_0000ffae);
  paVar3 = (astruct_57 *)((u32)paVar3 & 0xffff0000 | (u32)puStack26 >> 0x10);
  uVar1 = puStack26;
  if ((uVar1 + 0x82) == 0x0) {
    pass1_1030_7c50(uVar1,paVar3,(astruct_305 *)CONCAT22(uStack16,uStack18),0x4,0x4);
  }
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0xc8,0x11);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0xc8,0x12);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0xc8,0x13);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0xc8,0x14);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0x14,0x15);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0x14,0x16);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0xc8,0x17);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0xc8,0x18);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0xc8,0x19);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0x14,0x1a);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0x14,0x1b);
  pass1_1030_7ddc(uVar1,paVar3,CONCAT22(uStack16,uStack18),0x14,0x1c);
  if (*(i32 *)((int)uStack10 + 0x200) == 0x8000002) {
    pass1_1020_a43e((u8 *)paVar3,(u16 *)CONCAT22(0x1050,local_20));
    pass1_1020_a89e(CONCAT22(0x1050,local_20),(u32 *)((int)uStack6 + 0xc),((u32)uStack6 >> 0x10));
  }
  return;
}



StructD * pass1_1028_33f6(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_3484(param_1: *mut u16)

{
  astruct_57 *in_EDX;

  struct_1028_0068(in_EDX,(astruct_180 *)param_1);
  *param_1 = 0x34f6;
  ((int)param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_34a6(u8 *param_1,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_00cc((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x34f6;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



StructD * pass1_1028_34d0(StructD *param_1,u8 param_2)

{
  pass1_1028_0138(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_355e(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x3608;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_3580(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x3608;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



u16 pass1_1028_35aa(void)

{
  return 0x1;
}
pub fn pass1_1028_35b0(param_1: *mut astruct_15,mut param_2: i16)

{
  astruct_397 *paVar1;
  let mut uVar2: u16;

  paVar1 = (astruct_397 *)pass1_1028_b58e(param_1);
  if (param_2 == 0x0) {
    uVar2 = 0x0;
  }
  else {
    uVar2 = 0x32;
  }
  pass1_1030_7d1c((int)paVar1,(int)((u32)paVar1 >> 0x10),paVar1,uVar2,0x230000);
  return;
}



StructD * pass1_1028_35e2(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_3670(u8 *param_1,param_2: *mut u16)

{
  let mut in_register_0000000a: u16;

  struct_1028_37a6((astruct_57 *)CONCAT22(in_register_0000000a,param_1),(astruct_180 *)param_2);
  *param_2 = 0x373e;
  ((int)param_2 + 0x2) = 0x1028;
  return param_2;
}



u16 * pass1_1028_3692(u8 *param_1,mut param_2: i16,mut param_3: u16 ,mut param_4: i16,mut param_5: u32)

{
  let mut in_register_0000000a: u16;

  pass1_1028_3816((astruct_57 *)CONCAT22(in_register_0000000a,param_1),(astruct_12 *)CONCAT22(param_3,param_2),param_4,
                  param_5);
  CONCAT22(param_3,param_2) = 0x373e;
  (param_2 + 0x2) = 0x1028;
  return (u16 *)CONCAT22(param_3,param_2);
}



u16 pass1_1028_36bc(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,u32 *param_4)

{
  i16 *piVar1;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut iStack4: i16;

  uVar5 = CONCAT22(param_2,param_1);
  *param_4 = 0x0;
  uVar4 = (param_3 >> 0x10);
  if (*(i32 *)((int)param_3 + 0x28) != 0x0) {
    iStack4 = 0x4;
    while( true ) {
      if (0x1c < iStack4) break;
      uVar3 = (u32)((int)param_3 + 0x28);
      uVar5 = pass1_1020_bae6(uVar5,((u32)uVar5 >> 0x10),uVar3,
                              CONCAT22(iStack4,(int)((u32)uVar3 >> 0x10)));
      uVar2 = param_4;
      param_4 = param_4 + uVar5;
      piVar1 = (i16 *)((int)param_4 + 0x2);
      *piVar1 = *piVar1 + (int)((u32)uVar5 >> 0x10) + CARRY2(uVar2,uVar5);
      if (0xf9 < *param_4) {
        return 0x1;
      }
      iStack4 += 0x1;
    }
  }
  return 0x0;
}



StructD * pass1_1028_3718(StructD *param_1,u8 param_2)

{
  pass1_1028_388e(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn struct_1028_37a6(param_1: *mut astruct_57,param_2: *mut astruct_180)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  astruct_180 *iVar3;
  let mut uVar3: u16;

  struct_1028_b354(param_2);
  uVar3 = ((u32)param_2 >> 0x10);
  iVar3 = (astruct_180 *)param_2;
  uVar1 = 0x0;
  (u32)(iVar3 + 0x1) = 0x0;
  (u32)&iVar3[0x1].field_0x4 = 0x0;
  (u32)&iVar3[0x1].field_0x8 = 0x0;
  param_2.field0_0x0 = 0x3e2c;
  iVar3.field1_0x2 = 0x1028;
  mem_op_1000_179c(0xa,param_1);
  uVar2 = param_1 | uVar1;
  if (uVar2 == 0x0) {
    (u32)&iVar3[0x1].field_0x8 = 0x0;
  }
  else {
    pass1_1020_ba3e((astruct_172 *)CONCAT22(param_1,uVar1),0x5,0x5);
    &iVar3[0x1].field_0x8 = uVar1;
    &iVar3[0x1].field_0xa = uVar2;
  }
  return;
}
pub fn pass1_1028_3816(param_1: *mut astruct_57,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;

  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  uVar1 = 0x0;
  (u32)((int)param_2 + 0x20) = 0x0;
  (u32)((int)param_2 + 0x24) = 0x0;
  (u32)((int)param_2 + 0x28) = 0x0;
  param_2.field0_0x0 = 0x3e2c;
  ((int)param_2 + 0x2) = 0x1028;
  mem_op_1000_179c(0xa,param_1);
  uVar2 = param_1 | uVar1;
  if (uVar2 == 0x0) {
    (u32)((int)param_2 + 0x28) = 0x0;
  }
  else {
    pass1_1020_ba3e((astruct_172 *)CONCAT22(param_1,uVar1),0x5,0x5);
    ((int)param_2 + 0x28) = uVar1;
    ((int)param_2 + 0x2a) = uVar2;
  }
  return;
}
pub fn pass1_1028_388e(param_1: *mut u16)

{
  let mut uVar1: u16;
  char *pcVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar3 = (int)param_1;
  *param_1 = 0x3e2c;
  (iVar3 + 0x2) = 0x1028;
  pcVar2 = *(char **)(iVar3 + 0x28);
  uVar1 = (iVar3 + 0x2a);
  if ((uVar1 | pcVar2) != 0x0) {
    fn_ptr_1020_ba7e((u32 *)((u32)pcVar2 & 0xffff | (u32)uVar1 << 0x10));
    fn_ptr_1000_17ce(pcVar2);
  }
  pass1_1028_b418(param_1);
  return;
}



u16 pass1_1028_38d4(mut param_1: i16,mut param_2: u16 ,u32 *param_3,param_4: *mut u16,mut param_5: u32,mut param_6: u32)

{
  code **ppcVar1;
  BOOL16 BVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;

  uVar4 = param_3;
  uVar5 = ((u32)param_3 >> 0x10);
  pass1_1028_c7b6(param_2,uVar4,uVar5,param_4,param_6);
  if ((param_1 == 0x5) || (param_1 == 0x6)) {
    ppcVar1 = (code **)((int)*param_3 + 0x60);
    uVar3 = (**ppcVar1)();
    if (uVar3 != 0x0) {
      pass1_1028_c23e(uVar3,((u32)uVar3 >> 0x10),uVar4,uVar5,param_4,param_5,param_6);
      if ((int)uVar3 != 0x0) {
        BVar2 = pass1_1028_c314((int)uVar3,((u32)uVar3 >> 0x10),uVar4,uVar5,param_4,param_5,
                                (param_5 >> 0x10),param_6);
        if (BVar2 != 0x0) {
          return 0x1;
        }
      }
    }
  }
  else {
    PTR_LOOP_1050_50ca = (u8 *)0x6a8;
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_3958(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15)

{
  i32 *plVar1;
  let mut iVar2: i16;
  qword qVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uStack52: u32;
  u16 local_2c [0x2];
  let mut local_28: u32;
  let mut iStack36: i16;
  let mut uStack34: u32;
  let mut uStack30: u32;
  let mut uStack22: u16;
  let mut uStack20: u16;
  let mut uStack18: u32;
  let mut uStack14: u32;
  u32 *puStack10;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_b58e(param_3);
  puStack10 = (u32*)(param_1 + 0x22);
  uVar6 = (param_1 + 0x24);
  uVar7 = (u32)uVar6;
  if ((uVar6 | puStack10) != 0x0) {
    iStack6 = param_1;
    uStack4 = param_2;
    if (u16_1050_574c != 0x0) {
      uStack30 = (u32)(puStack10 + 0x4);
      for (uStack34 = 0x0; uStack34 < uStack30; uStack34 += 0x1) {
        pass1_1020_bb16(puStack10,(u32 *)CONCAT22(0x1050,local_2c),(u16 *)CONCAT22(0x1050,&local_28),uStack34
                       );
      }
    }
    uStack14 = (u32)(iStack6 + 0x2e);
    uStack18 = *_PTR_LOOP_1050_65e2;
    uStack20 = uStack18 & 0x1;
    for (uStack22 = 0x4; (int)uStack22 < 0xe; uStack22 += 0x1) {
      local_2c[0] = uStack22;
      local_28 = pass1_1020_bae6(uStack22,uVar7,puStack10,
                                 CONCAT22(uStack22,(int)((u32)puStack10 >> 0x10)));
      uVar6 = (local_28 >> 0x10) | local_28;
      uVar7 = (u32)uVar6;
      if (uVar6 != 0x0) {
        pass1_1020_bb8a((i32 *)puStack10,0x0,(u32)local_2c[0] << 0x10);
        uVar6 = -(local_28 + ((int)local_28 != 0x0));
        uVar7 = (u32)uVar6;
        uStack34 = CONCAT22(uVar6,-(int)local_28);
        pass1_1038_5694(uStack14,CONCAT22(uVar6,-(int)local_28),local_2c[0]);
        uStack30 = 0x0;
        iStack36 = 0x0;
        iVar8 = (int)param_3;
        uVar9 = ((u32)param_3 >> 0x10);
        switch(uStack22) {
        case 0x4:
          uStack30 = local_28 >> 0x1;
          if ((uStack30 == 0x0) && (uStack20 != 0x0)) {
            uStack30 = 0x1;
          }
          iStack36 = 0x11;
          break;
        case 0x5:
          uStack30 = local_28 >> 0x1;
          if ((uStack30 == 0x0) && (uStack20 != 0x0)) {
            uStack30 = 0x1;
          }
          iStack36 = 0x12;
          break;
        case 0x6:
          uStack30 = local_28 >> 0x1;
          if ((uStack30 == 0x0) && (uStack20 != 0x0)) {
            uStack30 = 0x1;
          }
          iStack36 = 0x13;
          break;
        case 0x7:
          uStack30 = local_28 >> 0x1;
          if ((uStack30 == 0x0) && (uStack20 != 0x0)) {
            uStack30 = 0x1;
          }
          iStack36 = 0x14;
          break;
        case 0x8:
          uStack30 = local_28;
          iStack36 = 0x1a;
          break;
        case 0x9:
          uStack30 = local_28;
          iStack36 = 0x1b;
          break;
        case 0xa:
          uStack30 = local_28;
          iStack36 = 0x1c;
          break;
        case 0xb:
          uStack30 = local_28;
          iStack36 = 0x17;
          break;
        case 0xc:
          iStack36 = 0x18;
          uStack30 = local_28;
          plVar1 = (i32 *)(iVar8 + 0x20);
          *plVar1 = *plVar1 + local_28;
          uVar6 = (iVar8 + 0x20);
          uVar4 = (iVar8 + 0x22);
          uVar5 = uVar6 >> 0x1 | ((uVar4 & 0x1) != 0x0) << 0xf;
          uStack52 = CONCAT22(uVar4 >> 0x1,uVar5);
          uVar5 = (uVar4 & 0xfffe) + CARRY2(uVar5,uVar5);
          uVar7 = (u32)uVar5;
          (iVar8 + 0x20) = uVar6 - (uVar6 & 0xfffe);
          (iVar8 + 0x22) = (uVar4 - uVar5) - (uVar6 < (uVar6 & 0xfffe));
          if (uStack52 != 0x0) {
            uVar10 = 0x15;//
LAB_1028_3b14:
            uStack30 = local_28;
            pass1_1020_bb8a(*(i32 **)(iVar8 + 0x28),uStack52,CONCAT22(uVar10,(int)((u32)uStack52 >> 0x10)));
          }
          break;
        case 0xd:
          iStack36 = 0x19;
          uStack30 = local_28;
          plVar1 = (i32 *)(iVar8 + 0x24);
          *plVar1 = *plVar1 + local_28;
          uVar6 = (iVar8 + 0x24);
          iVar2 = (iVar8 + 0x26);
          qVar3 = (qword)(local_28 & 0xffff0000 | (u32)uVar6) / 0x3;
          uStack52 = (long)qVar3;
          uStack52 = (int)(qVar3 >> 0x10);
          uVar4 = qVar3;
          uVar5 = uStack52 * 0x3 + CARRY2(uVar4,uVar4) + CARRY2(uVar4 * 0x2,uVar4);
          uVar7 = (u32)uVar5;
          (iVar8 + 0x24) = uVar6 + uVar4 * -0x3;
          (iVar8 + 0x26) = (iVar2 - uVar5) - (uVar6 < uVar4 * 0x3);
          if (uStack52 != 0x0) {
            uVar10 = 0x16;
            goto LAB_1028_3b14;
          }
        }
        if (((uStack30 | uStack30) != 0x0) && (iStack36 != 0x0)) {
          pass1_1020_bb70(*(i32 **)(iVar8 + 0x28),uStack30,CONCAT22(iStack36,uStack30));
        }
      }
    }
  }
  return;
}



pub fn pass1_1028_3c32(u32 *param_1) -> u32

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut local_6: u16;
  let mut iStack4: i16;

  ppcVar1 = (code **)((int)*param_1 + 0x40);
  iVar2 = (**ppcVar1)();
  if (iVar2 != 0x0) {
    return 0x0;
  }
  return CONCAT22(-(0x3e8 < local_6) - iStack4,0x3e8 - local_6);
}
pub fn pass1_1028_3c60(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,u32 *param_4)

{
  i16 *piVar1;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  i32 local_10;
  u8 local_c [0x4];
  let mut iStack8: i16;
  let mut uStack6: u16;
  let mut uStack4: u16;

  uVar6 = CONCAT22(param_2,param_1);
  *param_4 = 0x0;
  uVar5 = ((u32)param_3 >> 0x10);
  iVar4 = (int)param_3;
  if (*(i32 *)(iVar4 + 0x28) != 0x0) {
    iStack8 = 0x4;
    while( true ) {
      if (0x1c < iStack8) break;
      uVar3 = (u32)(iVar4 + 0x28);
      uVar6 = pass1_1020_bae6(uVar6,((u32)uVar6 >> 0x10),uVar3,
                              CONCAT22(iStack8,(int)((u32)uVar3 >> 0x10)));
      uVar2 = param_4;
      param_4 = param_4 + uVar6;
      piVar1 = (i16 *)((int)param_4 + 0x2);
      *piVar1 = *piVar1 + (int)((u32)uVar6 >> 0x10) + CARRY2(uVar2,uVar6);
      if (0x3e7 < *param_4) {
        return;
      }
      iStack8 += 0x1;
    }
  }
  uVar6 = (u32)(iVar4 + 0x28);
  uStack4 = ((int)uVar6 + 0x4);
  uStack6 = 0x0;
  while( true ) {
    if (uStack4 <= uStack6) {
      return;
    }
    pass1_1020_bb16((u32*)(iVar4 + 0x28),(u32 *)CONCAT22(0x1050,&local_10),(u16 *)CONCAT22(0x1050,local_c),
                    uStack6);
    *param_4 = *param_4 + local_10;
    if (0x3e7 < *param_4) break;
    uStack6 += 0x1;
  }
  return;
}
pub fn write_to_file_1028_3d0e(mut param_1: u16 ,param_2: *mut astruct_731,u8 *param_3)

{
  BOOL16 BVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;
  HFILE16 in_stack_0000ffd8;
  u32 local_10 [0x2];
  let mut local_8: u32;

  BVar1 = write_to_file_1028_b5ec(param_2,(u32)param_3);
  if (BVar1 != 0x0) {
    uVar3 = ((u32)param_2 >> 0x10);
    iVar2 = (int)param_2;
    local_10[0] = (u32)(iVar2 + 0x20);
    BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_10),(char *)0x4,in_stack_0000ffd8);
    if (BVar1 != 0x0) {
      local_8 = (u32)(iVar2 + 0x24);
      BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,&local_8),(char *)0x4,in_stack_0000ffd8);
      if (BVar1 != 0x0) {
        write_to_file_1008_7a22(param_3,*(i32 *)(iVar2 + 0x28));
        if (BVar1 != 0x0) {
          return;
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}
pub fn pass1_1028_3d92(mut param_1: i16,u8 *param_2,param_3: *mut astruct_373,HFILE16 *param_4)

{
  let mut iVar1: i16;
  BOOL16 BVar2;
  let mut uVar3: u16;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 != 0x0) {
    iVar1 = (int)param_3;
    BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | (u32)(iVar1 + 0x20)),0x4);
    if (BVar2 != 0x0) {
      BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | (u32)(iVar1 + 0x24)),0x4);
      if (BVar2 != 0x0) {
        uVar3 = pass1_1008_7ad4((u32)param_4,*(i32 **)(iVar1 + 0x28));
        if (uVar3 != 0x0) {
          return;
        }
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



StructD * pass1_1028_3e06(StructD *param_1,u8 param_2)

{
  pass1_1028_388e(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_3e94(param_1: *mut astruct_180)

{
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  (u32)((int)param_1 + 0x20) = 0x0;
  param_1.field0_0x0 = 0x4004;
  ((int)param_1 + 0x2) = 0x1028;
  pass1_1028_3fa2((astruct_180 *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10));
  return &param_1.field0_0x0;
}



pub fn pass1_1028_3ec8(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32) -> u32

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  (u32)((int)param_2 + 0x20) = 0x0;
  param_2.field0_0x0 = 0x4004;
  ((int)param_2 + 0x2) = 0x1028;
  pass1_1028_3fa2((astruct_180 *)((u32)param_2 & 0xffff | (u32)param_2 << 0x10));
  return (u32)param_2;
}
pub fn pass1_1028_3f04(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_15)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  astruct_397 *paStack6;

  uVar6 = 0x1f;
  pass1_1028_b58e(param_3);
  paStack6 = (astruct_397 *)CONCAT22(param_2,param_1);
  uStack10 = pass1_1030_7c28(param_1,param_2,CONCAT22(param_2,param_1),uVar6);
  uVar5 = (uStack10 >> 0x10);
  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,uVar5);
  uVar2 = uStack10 & 0xffff;
  pass1_1030_7d1c((int)uVar2,uVar5,paStack6,0x0,0x1f0000);
  uVar5 = ((u32)param_3 >> 0x10);
  iVar4 = (int)param_3;
  if ((iVar4 + 0xc) != 0x22) {
    if ((iVar4 + 0xc) == 0x23) {
      uVar1 = 0xa;
    }
    else {
      uVar1 = 0x5;
    }
    uStack14 = (u32)uVar1;
    uStack10 += *(i32 *)(iVar4 + 0x20);
    (u32)(iVar4 + 0x20) = uStack10 % (u32)uVar1;
    uVar2 = uStack10 / uStack14;
    paVar3 = (astruct_57 *)(uStack10 % uStack14);
    uStack10 += uVar2;
  }
  pass1_1030_7ddc(uVar2,paVar3,(u32)paStack6,uStack10,0x21);
  return;
}
pub fn pass1_1028_3fa2(param_1: *mut astruct_180)

{
  let mut uVar1: u16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((iVar2 + 0xc) != 0x22) {
    if ((iVar2 + 0xc) == 0x23) {
      uVar1 = 0xa;
    }
    else {
      uVar1 = 0x5;
    }
    uVar1 >>= 0x1;
    pass1_1008_612e(uVar1,0x0,uVar1);
    (iVar2 + 0x20) = uVar1;
    (iVar2 + 0x22) = (int)uVar1 >> 0xf;
  }
  return;
}



StructD * pass1_1028_3fde(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_406c(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x42ec;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_408e(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x42ec;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_40b8(mut param_1: u16 ,param_2: *mut astruct_15)

{
  code **ppcVar1;
  u32 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  u32 *puVar7;
  let mut uStack54: u32;
  u8 local_2c [0x6];
  u32 *puStack38;
  let mut uStack34: u32;
  u32 *puStack26;
  let mut uStack24: u32;
  let mut local_14: u32;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut local_c: u32;
  let mut iStack8: i16;
  let mut uStack6: u16;
  let mut uStack4: u16;

  pass1_1028_b58e(param_2);
  local_14 = (u32)(param_1 + 0xc);
  iStack8 = (param_1 + 0x10);
  puStack26 = &local_c;
  uStack34 = CONCAT22(uStack34,&local_14);
  iStack16 = iStack8 + 0x1;
  puVar7 = (u32 *)CONCAT22(0x1050,local_2c);
  iStack14 = iStack16;
  local_c = local_14;
  uStack6 = param_1;
  uStack4 = extraout_DX;
  uVar6 = pass1_1028_bb24(param_2);
  uVar5 = (uVar6 >> 0x10);
  puVar2 = &local_14;
  pass1_1030_64ce(puVar2,uVar5,_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar2),
                  uVar6 & 0xffff | (u32)uVar5 << 0x10,puVar7);
  uStack34 = *puVar2;
  uVar5 = ((int)puVar2 + 0x2);
  uStack54._3_1_ = (u8)(uStack34 >> 0x18);
  uVar3 = uStack54._3_1_;
  uStack24 = uStack34;
  if (uStack54._3_1_ != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack34 & 0xffff | (u32)uVar5 << 0x10);
    uStack54 = (astruct_419 *)CONCAT22(uVar5,uVar3);
    uVar4 = pass1_1030_6fa0(CONCAT22(uVar5,uVar3));
    if (uVar4 == 0x64) {
      puStack38 = (u32 *)struct_op_1030_73a8(uStack54,0x64,uVar5);
      ppcVar1 = (code **)((int)*puStack38 + 0x58);
      (**ppcVar1)();
    }
  }
  pass1_1028_b514(param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_4194(param_1: *mut astruct_15)

{
  let mut uVar1: u16;
  let mut in_EDX: u32;
  let mut uVar2: u16;
  let mut uVar3: u32;
  astruct_27 *paVar4;
  let mut in_stack_0000fe92: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffea: u16;

  uVar2 = ((u32)in_EDX >> 0x10);
  pass1_1028_be9e(param_1);
  uVar3 = pass1_1028_b4f2(param_1);
  uVar1 = (uVar3 >> 0x10);
  if ((*(i32 *)((int)uVar3 + 0x200) != 0x8000002) && (((int)param_1 + 0x12) == 0x5)) {
    paVar4 = (astruct_27 *)
             mixed_1010_20ba((astruct_57 *)CONCAT22(uVar2,uVar1),_u16_1050_0ed0,
                             (u8 **)CONCAT22(in_stack_0000ffea,0x2b),in_stack_0000fe92,in_stack_0000ffb6,
                             in_stack_0000ffbc,in_stack_0000ffc0);
    pass1_1010_043a(paVar4,*(i32 *)((int)uVar3 + 0x4),0xe);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_41ea(mut param_1: i16,param_2: *mut astruct_15)

{
  code **ppcVar1;
  u32 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  u32 *puVar7;
  let mut uStack54: u32;
  u8 local_2c [0x6];
  u32 *puStack38;
  let mut uStack34: u32;
  u32 *puStack26;
  let mut uStack24: u32;
  let mut local_14: u32;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut local_c: u32;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_b514(param_2);
  pass1_1028_b58e(param_2);
  local_14 = (u32)(param_1 + 0xc);
  iStack8 = (param_1 + 0x10);
  puStack26 = &local_c;
  uStack34 = CONCAT22(uStack34,&local_14);
  iStack16 = iStack8 + 0x1;
  puVar7 = (u32 *)CONCAT22(0x1050,local_2c);
  iStack14 = iStack16;
  local_c = local_14;
  iStack6 = param_1;
  uStack4 = extraout_DX;
  uVar6 = pass1_1028_bb24(param_2);
  uVar5 = (uVar6 >> 0x10);
  puVar2 = &local_14;
  pass1_1030_64ce(puVar2,uVar5,_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar2),
                  uVar6 & 0xffff | (u32)uVar5 << 0x10,puVar7);
  uStack34 = *puVar2;
  uVar5 = ((int)puVar2 + 0x2);
  uStack54._3_1_ = (u8)(uStack34 >> 0x18);
  uVar3 = uStack54._3_1_;
  if (uStack54._3_1_ != 0x0) {
    uStack24 = uStack34;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack34 & 0xffff | (u32)uVar5 << 0x10);
    uStack54 = (astruct_419 *)CONCAT22(uVar5,uVar3);
    uVar4 = pass1_1030_6fa0(CONCAT22(uVar5,uVar3));
    if (uVar4 == 0x64) {
      puStack38 = (u32 *)struct_op_1030_73a8(uStack54,0x64,uVar5);
      ppcVar1 = (code **)((int)*puStack38 + 0x24);
      (**ppcVar1)();
    }
  }
  return;
}
pub fn FUN_1028_42c2(void)

{
  return;
}



StructD * FUN_1028_42c6(mut param_1: u16 ,StructD *param_2,u8 param_3)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



u16 * struct_1028_4354(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x446a;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_4376(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x446a;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



u16 pass1_1028_43a0(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if ((((int)param_1 + 0x12) != 0x6) && (((int)param_1 + 0x12) != 0x5)) {
    return 0x0;
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_43c2(u8 *param_1,mut param_2: i16,mut param_3: u16 ,mut param_4: i16)

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  u32 *puVar2;
  let mut in_stack_0000fea2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000fffa: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  pass1_1028_bd38(param_1,(astruct_15 *)CONCAT22(param_3,param_2));
  if (param_4 == 0x0) {
    puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fffa,0x8),in_stack_0000fea2,
                             in_stack_0000ffc6,in_stack_0000ffcc,in_stack_0000ffd0);
    pass1_1010_988c((u32)puVar2,(param_2 + 0xc));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_43f6(u8 *param_1,param_2: *mut astruct_15,mut param_3: i16)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  u32 *puVar4;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;
  let mut uVar5: u16;

  uVar1 = 0x83;
  puVar4 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,(u8 **)0x830009,
                           in_stack_0000fe9a,in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  uVar3 = ((u32)puVar4 >> 0x10);
  uVar1 = pass1_1010_65d0((u32)puVar4,uVar1);
  if (0x0 < (int)uVar1) {
    uVar2 = pass1_1028_b58e(param_2);
    if (param_3 == 0x0) {
      uVar5 = 0x0;
    }
    else {
      uVar5 = 0x3e8;
    }
    pass1_1030_7d1c(uVar2,uVar3,(astruct_397 *)CONCAT22(uVar3,uVar2),uVar5,0x230000);
  }
  return;
}



StructD * pass1_1028_4444(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_44d2(param_1: *mut astruct_180)

{
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  (u32)((int)param_1 + 0x20) = 0x0;
  param_1.field0_0x0 = 0x4836;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_44fe(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  (u32)((int)param_2 + 0x20) = 0x0;
  param_2.field0_0x0 = 0x4836;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn pass1_1028_4530(param_1: *mut u16)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = ((u32)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0x4836;
  (iVar4 + 0x2) = 0x1028;
  puVar1 = (u32 *)(iVar4 + 0x20);
  uVar2 = (iVar4 + 0x22);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1028_b418(param_1);
  return;
}
pub fn pass1_1028_456e(mut param_1: u16 ,param_2: *mut astruct_15,mut param_3: u32)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  pass1_1028_b46e(param_1,param_2,param_3);
  uVar5 = ((u32)param_2 >> 0x10);
  iVar4 = (int)param_2;
  puVar1 = (u32 *)(iVar4 + 0x20);
  uVar2 = (iVar4 + 0x22);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (u32)(iVar4 + 0x20) = 0x0;
  return;
}
pub fn pass1_1028_45b0(param_1: *mut astruct_15)

{
  astruct_57 *in_EDX;
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;

  pass1_1028_be9e(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  if (((int)param_1 + 0x12) == 0x5) {
    uVar3 = 0x0;
    iVar4 = 0x4;
    uVar2 = 0x2;
    uVar1 = pass1_1028_b58e((astruct_15 *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10));
    pass1_1030_7c50(uVar1,in_EDX,(astruct_305 *)CONCAT22((int)in_EDX,uVar1),CONCAT22(uVar3,uVar2),iVar4);
  }
  return;
}



pub fn pass1_1028_45e2(mut param_1: u16 ,mut param_2: i16,mut param_3: u32) -> u32

{
  pass1_1028_478a(param_1,(astruct_15 *)param_3);
  return CONCAT22(-(0x3e8 < param_1) - param_2,0x3e8 - param_1);
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_45fe(mut param_1: i16,param_2: *mut astruct_15,mut param_3: u32)

{
  let mut uVar1: u32;
  code **ppcVar2;
  astruct_21 *paVar3;
  let mut uVar4: u16;
  astruct_57 *paVar5;
  astruct_15 *pstruct15_1;
  astruct_314 *iVar5;
  astruct_15 *pstruct15_2;
  let mut uVar6: u16;
  astruct_99 *paStack44;
  i32 local_28;
  astruct_21 *paStack34;
  let mut uStack32: u16;
  astruct_99 *paStack30;
  i16 local_1a [0x4];
  let mut uStack18: u32;
  let mut uStack14: u32;
  u32 *puStack10;
  let mut uStack6: u32;
  astruct_313 *uVar2;

  pass1_1028_b58e(param_2);
  uStack6 = CONCAT22((int)param_3,param_1);
  puStack10 = (u32*)(param_1 + 0x22);
  pstruct15_2 = (astruct_15 *)((u32)param_2 >> 0x10);
  pstruct15_1 = (astruct_15 *)param_2;
  paVar3 = pstruct15_1.field24_0x20;
  uStack32 = pstruct15_1.field25_0x22;
  paVar5 = (astruct_57 *)(param_3 & 0xffff0000 | (u32)uStack32);
  paStack30 = (astruct_99 *)CONCAT22(uStack32,paVar3);
  paStack34 = paVar3;
  if ((uStack32 | paVar3) != 0x0) {
    ppcVar2 = (code **)(u32)paVar3;
    (**ppcVar2)();
  }
  mem_op_1000_179c(0xc,paVar5);
  uStack32 = paVar5;
  uVar4 = uStack32 | paVar3;
  paStack34 = paVar3;
  if (uVar4 == 0x0) {
    paVar3 = NULL;
    uVar4 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(uStack32,paVar3));
  }
  pstruct15_1.field24_0x20 = paVar3;
  pstruct15_1.field25_0x22 = uVar4;
  if (puStack10 != NULL) {
    uStack14 = (u32)((int)puStack10 + 0x4);
    for (uStack18 = 0x0; uStack18 < uStack14; uStack18 += 0x1) {
      pass1_1020_bb16(puStack10,(u32 *)CONCAT22(0x1050,&local_28),(u16 *)CONCAT22(0x1050,local_1a),uStack18);
      if ((local_28 != 0x0) && (local_1a[0] != 0x0)) {
        paStack30 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar4 = ((u32)paStack30 >> 0x10);
        uVar2 = (astruct_313 *)paStack30;
        if ((uVar4 | uVar2) == 0x0) {
          paStack44 = NULL;
        }
        else {
          paStack30.field0_0x0 = 0x389a;
          uVar2.field2_0x2 = 0x1008;
          uVar2.field3_0x4 = 0x0;
          uVar2.field4_0x6 = 0x0;
          uVar2.field5_0x8 = 0x0;
          uVar2.field6_0xa = 0x0;
          uVar2.field7_0xc = 0x0;
          paStack30.field0_0x0 = 0x56ce;
          uVar2.field2_0x2 = 0x1018;
          paStack44 = paStack30;
        }
        uVar6 = ((u32)paStack44 >> 0x10);
        iVar5 = (astruct_314 *)paStack44;
        iVar5.field4_0x4 = local_1a[0];
        iVar5.field9_0xa = local_28;
        iVar5.field10_0xc = local_28;
        uVar1 = (u32)&pstruct15_1.field24_0x20;
        ppcVar2 = (code **)((int)*(u32*)&pstruct15_1.field24_0x20 + 0x8);
        (**ppcVar2)(0x0,(int)uVar1,(int)((u32)uVar1 >> 0x10),iVar5,uVar6);
      }
    }
  }
  return;
}



u16 pass1_1028_4768(mut param_1: u16 ,mut param_2: i16,mut param_3: u32)

{
  pass1_1028_478a(param_1,(astruct_15 *)param_3);
  if ((param_2 == 0x0) && (param_1 < 0x3e8)) {
    return 0x0;
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1028_478a(mut param_1: i16,param_2: *mut astruct_15)

{
  let mut extraout_DX: u16;
  i32 local_1e;
  i16 local_1a [0x4];
  let mut uStack18: u16;
  let mut uStack16: u16;
  i32 lStack14;
  u32 *puStack10;
  let mut uStack6: u32;

  pass1_1028_b58e(param_2);
  uStack6 = CONCAT22(extraout_DX,param_1);
  puStack10 = (u32*)(param_1 + 0x22);
  lStack14 = 0x0;
  if (((param_1 + 0x24) | puStack10) == 0x0) {
    return;
  }
  uStack16 = (puStack10 + 0x4);
  for (uStack18 = 0x0; uStack18 < uStack16; uStack18 += 0x1) {
    pass1_1020_bb16(puStack10,(u32 *)CONCAT22(0x1050,&local_1e),(u16 *)CONCAT22(0x1050,local_1a),uStack18);
    if (0x0 < local_1a[0]) {
      lStack14 += local_1e;
    }
  }
  return;
}



StructD * pass1_1028_4810(StructD *param_1,u8 param_2)

{
  pass1_1028_4530(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_489e(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = &PTR_LOOP_1050_4942;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_48c0(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = &PTR_LOOP_1050_4942;
  ((int)param_2 + 0x2) = 0x1028;
  ((int)param_2 + 0xe) = ((int)param_2 + 0xc);
  ((int)param_2 + 0x10) = ((int)param_2 + 0xc);
  return &param_2.field0_0x0;
}
pub fn pass1_1028_48fa(u32 *param_1)

{
  pass1_1028_bdac((astruct_15 *)param_1,0x0);
  return;
}
pub fn FUN_1028_490c(void)

{
  return;
}
pub fn FUN_1028_4910(void)

{
  return;
}
pub fn FUN_1028_4914(void)

{
  return;
}
pub fn FUN_1028_4918(void)

{
  return;
}



StructD * FUN_1028_491c(mut param_1: u16 ,StructD *param_2,u8 param_3)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



pub fn pass1_1028_4920(void) -> u32

{
  let mut unaff_BP: i16;

  pass1_1028_b418((u16*)(unaff_BP + 0x6));
  if ((*(u8 *)(unaff_BP + 0xa) & 0x1) != 0x0) {
    fn_ptr_1000_17ce(*(char **)(unaff_BP + 0x6));
  }
  return CONCAT22((unaff_BP + 0x8),(unaff_BP + 0x6));
}



u16 * struct_1028_49aa(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x4b1c;
  ((int)param_1 + 0x2) = 0x1028;
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x20)),NULL,0xa);
  return &param_1.field0_0x0;
}



pub fn pass1_1028_49de(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32) -> u32

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x4b1c;
  ((int)param_2 + 0x2) = 0x1028;
  pass1_1000_4906((StructD *)((u32)param_2 & 0xffff0000 | (u32)((int)param_2 + 0x20)),NULL,0xa);
  return (u32)param_2;
}
pub fn pass1_1028_4a1a(mut param_1: u32,mut param_2: u32)

{
  BOOL16 BVar1;
  HFILE16 in_stack_0000ffe8;

  BVar1 = write_to_file_1028_b5ec((astruct_731 *)param_1,param_2);
  if ((BVar1 != 0x0) &&
     (BVar1 = write_to_file_1008_7e1c
                        ((u8 *)param_2,param_1 & 0xffff0000 | (u32)((int)param_1 + 0x20),(char *)0xa,in_stack_0000ffe8
                        ), BVar1 == 0x0)) {
    u16_1050_0310 = 0x6d0;
    return;
  }
  return;
}
pub fn pass1_1028_4a5a(mut param_1: i16,u8 *param_2,param_3: *mut astruct_373,HFILE16 *param_4)

{
  BOOL16 BVar1;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if ((param_1 != 0x0) &&
     (BVar1 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | (u32)((int)param_3 + 0x20)),0xa),
     BVar1 == 0x0)) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  return;
}



u16 pass1_1028_4a9a(mut param_1: u32,mut param_2: i16)

{
  return ((int)param_1 + 0x20 + param_2 * 0x2);
}
pub fn pass1_1028_4ab2(mut param_1: u32,mut param_2: u16 ,mut param_3: i16)

{
  ((int)param_1 + param_3 * 0x2 + 0x20) = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_4aca(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar2;
  StructD *pSVar3;
  u32 *puVar4;
  let mut in_stack_0000fe9e: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000fff6: u16;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  if (((int)param_2 + 0x20) != 0x0) {
    puVar4 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff6,0x2f),in_stack_0000fe9e,
                             in_stack_0000ffc2,in_stack_0000ffc8,in_stack_0000ffcc);
    pSVar3 = (StructD *)((u32)paVar2 & 0xffff0000 | (u32)puVar4 >> 0x10);
    uVar1 = SUB42(puVar4,0x0);
    pass1_1010_ed3e((u32)puVar4);
    pass1_1030_2a2c(pSVar3,(astruct_678 *)CONCAT22((int)pSVar3,uVar1));
  }
  return;
}



StructD * pass1_1028_4af6(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_4b84(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
    // just 0x5070
  param_1.field0_0x0 = (int)s_SCInternalPutBldg2_site_0x_08lx__1050_506f + 0x1;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_4ba6(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = (int)s_SCInternalPutBldg2_site_0x_08lx__1050_506f + 0x1;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



u16 pass1_1028_4bd0(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  if ((((int)param_1 + 0x12) != 0x6) && (((int)param_1 + 0x12) != 0x5)) {
    return 0x0;
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_4bf2(mut param_1: i16,param_2: *mut astruct_15,mut param_3: u32)

{
  code **ppcVar1;
  u32 *puVar2;
  let mut uVar3: u16;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  u32 *puVar6;
  let mut uStack54: u32;
  u8 local_2c [0x6];
  u32 *puStack38;
  let mut uStack34: u32;
  u32 *puStack26;
  let mut uStack24: u32;
  let mut local_14: u32;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut local_c: u32;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_b58e(param_2);
  local_14 = (u32)(param_1 + 0xc);
  iStack8 = (param_1 + 0x10);
  puStack26 = &local_c;
  uStack34 = CONCAT22(uStack34,&local_14);
  iStack16 = iStack8 + 0x1;
  puVar6 = (u32 *)CONCAT22(0x1050,local_2c);
  iStack14 = iStack16;
  local_c = local_14;
  iStack6 = param_1;
  uStack4 = extraout_DX;
  uVar5 = pass1_1028_bb24(param_2);
  uVar4 = (uVar5 >> 0x10);
  puVar2 = &local_14;
  pass1_1030_64ce(puVar2,uVar4,_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar2),
                  uVar5 & 0xffff | (u32)uVar4 << 0x10,puVar6);
  uStack34 = *puVar2;
  uVar4 = ((int)puVar2 + 0x2);
  uStack54._3_1_ = (u8)(uStack34 >> 0x18);
  uVar3 = uStack54._3_1_;
  uStack24 = uStack34;
  if (uStack54._3_1_ != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack34 & 0xffff | (u32)uVar4 << 0x10);
    uStack54 = (astruct_419 *)CONCAT22(uVar4,uVar3);
    uVar3 = pass1_1030_6fa0(CONCAT22(uVar4,uVar3));
    if ((uVar3 == 0x62) || (uVar3 == 0x63)) {
      puStack38 = (u32 *)struct_op_1030_73a8(uStack54,uVar3,uVar4);
      uVar3 = puStack38;
      ppcVar1 = (code **)((int)*puStack38 + 0x58);
      (**ppcVar1)();
    }
  }
  pass1_1028_b46e(uVar3,param_2,param_3);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_4cd6(mut param_1: i16,param_2: *mut astruct_15)

{
  code **ppcVar1;
  u32 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  u32 *puVar7;
  let mut uStack54: u32;
  u8 local_2c [0x6];
  u32 *puStack38;
  let mut uStack34: u32;
  u32 *puStack26;
  let mut uStack24: u32;
  let mut local_14: u32;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut local_c: u32;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_b514(param_2);
  pass1_1028_b58e(param_2);
  local_14 = (u32)(param_1 + 0xc);
  iStack8 = (param_1 + 0x10);
  puStack26 = &local_c;
  uStack34 = CONCAT22(uStack34,&local_14);
  iStack16 = iStack8 + 0x1;
  puVar7 = (u32 *)CONCAT22(0x1050,local_2c);
  iStack14 = iStack16;
  local_c = local_14;
  iStack6 = param_1;
  uStack4 = extraout_DX;
  uVar6 = pass1_1028_bb24(param_2);
  uVar5 = (uVar6 >> 0x10);
  puVar2 = &local_14;
  pass1_1030_64ce(puVar2,uVar5,_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar2),
                  uVar6 & 0xffff | (u32)uVar5 << 0x10,puVar7);
  uStack34 = *puVar2;
  uVar5 = ((int)puVar2 + 0x2);
  uStack54._3_1_ = (u8)(uStack34 >> 0x18);
  uVar3 = uStack54._3_1_;
  if (uStack54._3_1_ != 0x0) {
    uStack24 = uStack34;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack34 & 0xffff | (u32)uVar5 << 0x10);
    uStack54 = (astruct_419 *)CONCAT22(uVar5,uVar3);
    uVar4 = pass1_1030_6fa0(CONCAT22(uVar5,uVar3));
    if ((uVar4 == 0x62) || (uVar4 == 0x63)) {
      puStack38 = (u32 *)struct_op_1030_73a8(uStack54,uVar4,uVar5);
      ppcVar1 = (code **)((int)*puStack38 + 0x24);
      (**ppcVar1)();
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_4db2(uchar param_1,u8 *param_2,param_3: *mut astruct_15,mut param_4: i16)

{
  BOOL16 BVar1;
  i16 *piVar2;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar4;
  let mut unaff_SI: u16;
  u32 *puVar5;
  let mut in_stack_0000fd42: u16;
  let mut in_stack_0000fe66: u16;
  let mut in_stack_0000fe6c: u16;
  let mut in_stack_0000fe70: u16;
  i16 *piVar6;
  let mut uVar7: u16;
  let mut puVar8: *mut u16;
  let mut uVar9: u16;
  u8 local_14e [0x124];
  let mut uStack42: u32;
  let mut uStack38: u32;
  let mut local_22: i16;
  u8 local_20 [0x2];
  u8 local_1e [0x2];
  let mut local_1c: u32;
  let mut iStack24: i16;
  let mut uStack22: u32;
  i16 *piStack18;
  let mut uStack16: u16;
  let mut local_e: i16;
  let mut local_c: u16;
  let mut uStack10: u32;
  u32 *puStack6;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  BVar1 = pass1_1008_c6ae(_u16_1050_06e0,((int)param_3 + 0xc),0x29);
  if (BVar1 != 0x0) {
    pass1_1028_bd38(paVar4,param_3);
    if ((param_4 == 0x0) && (((int)param_3 + 0xc) == 0x13)) {
      puVar5 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x8),in_stack_0000fd42,
                               in_stack_0000fe66,in_stack_0000fe6c,in_stack_0000fe70);
      paVar4 = (astruct_57 *)((u32)paVar4 & 0xffff0000 | (u32)puVar5 >> 0x10);
      pass1_1010_988c((u32)puVar5,((int)param_3 + 0xc));
    }
    puStack6 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fd42,
                               in_stack_0000fe66,in_stack_0000fe6c,in_stack_0000fe70);
    uVar3 = ((u32)puStack6 >> 0x10);
    uStack10 = (u32)((int)puStack6 + 0x20);
    puVar8 = &local_c;
    uVar9 = SUB42(&DAT_1050_1050,0x0);
    piVar2 = &local_e;
    uVar7 = SUB42(&DAT_1050_1050,0x0);
    piVar6 = piVar2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack10);
    piStack18 = piVar2;
    uStack16 = uVar3;
    pass1_1030_5b1c(CONCAT22(uVar3,piVar2),(u16 *)CONCAT22(uVar7,piVar6),(u16 *)CONCAT22(uVar9,puVar8));
    pass1_1028_b58e(param_3);
    uStack22 = CONCAT22(uVar3,piVar2);
    local_1c = (u32)(piVar2 + 0x6);
    iStack24 = piVar2[0x8];
    pass1_1028_c8ee(param_3,0x1,(u16 *)CONCAT22(0x1050,&local_1c));
    pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_1c),(u16 *)CONCAT22(0x1050,&local_22),
                    (u16 *)CONCAT22(0x1050,local_20),(u16 *)CONCAT22(0x1050,local_1e));
    if (local_e < local_22) {
      pass1_1030_5b3e(CONCAT22(uStack16,piStack18),local_22,local_c);
      pass1_1030_5b1c(CONCAT22(uStack16,piStack18),(u16 *)CONCAT22(0x1050,&local_e),
                      (u16 *)CONCAT22(0x1050,&local_c));
    }
    uStack38 = (u32)((int)uStack22 + 0x2e);
    uStack42 = (u32)((int)uStack38 + 0x4);
    struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_14e),0x0,0x0,0x62,&local_1c,&DAT_1050_1050,uStack42,
                        uStack10);
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_14e));
    pass1_1028_ccd0(param_3,(u16 *)CONCAT22(0x1050,&local_1c));
  }
  return;
}
pub fn pass1_1028_4f30(param_1: *mut astruct_15,mut param_2: i16)

{
  astruct_397 *paVar1;
  let mut uVar2: u16;

  paVar1 = (astruct_397 *)pass1_1028_b58e(param_1);
  if (param_2 == 0x0) {
    uVar2 = 0x0;
  }
  else {
    uVar2 = 0x3e8;
  }
  pass1_1030_7d1c((int)paVar1,(int)((u32)paVar1 >> 0x10),paVar1,uVar2,0x230000);
  return;
}



u16 pass1_1028_4f62(param_1: *mut astruct_15)

{
  let mut uVar1: u32;

  uVar1 = pass1_1028_b58e(param_1);
  if (((int)uVar1 + 0x10) == 0x0) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_4faa(param_1: *mut astruct_15,mut param_2: u16 ) -> u32

{
  let mut uVar1: u16;
  u32 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut local_c: u32;
  let mut uStack8: u16;
  let mut uStack6: u32;

  uVar1 = pass1_1028_4f62(param_1);
  if (uVar1 != 0x0) {
    return (u32)param_1;
  }
  uStack6 = pass1_1028_b58e(param_1);
  local_c = (u32)((int)uStack6 + 0xc);
  uStack8 = 0x0;
  uVar4 = pass1_1028_bb24(param_1);
  uVar3 = (uVar4 >> 0x10);
  puVar2 = &local_c;
  pass1_1030_627e(puVar2,uVar3,_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar2),
                  uVar4 & 0xffff | (u32)uVar3 << 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(uVar3,puVar2));
  if ((uVar3 | puVar2) == 0x0) {
    return 0x0;
  }
  uVar4 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar3,puVar2),puVar2,uVar3 | puVar2);
  return uVar4;
}



StructD * pass1_1028_504a(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_50d8(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x5280;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_50fa(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x5280;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn FUN_1028_5124(void)

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_5128(undefined1 param_1,mut param_2: u16 ,param_3: *mut astruct_15)

{
  i16 *piVar1;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  let mut unaff_SI: u16;
  let mut in_stack_0000fd42: u16;
  let mut in_stack_0000fe66: u16;
  let mut in_stack_0000fe6c: u16;
  let mut in_stack_0000fe70: u16;
  i16 *piVar4;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  let mut uVar7: u16;
  u8 local_14e [0x124];
  let mut uStack42: u32;
  let mut uStack38: u32;
  let mut local_22: i16;
  u8 local_20 [0x2];
  u8 local_1e [0x2];
  let mut local_1c: u32;
  let mut iStack24: i16;
  let mut uStack22: u32;
  i16 *piStack18;
  let mut uStack16: u16;
  let mut local_e: i16;
  let mut local_c: u16;
  let mut uStack10: u32;
  u32 *puStack6;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  pass1_1028_bd38(param_2,param_3);
  puStack6 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fd42,
                             in_stack_0000fe66,in_stack_0000fe6c,in_stack_0000fe70);
  uVar2 = ((u32)puStack6 >> 0x10);
  uStack10 = (u32)((int)puStack6 + 0x20);
  puVar6 = &local_c;
  uVar7 = SUB42(&DAT_1050_1050,0x0);
  piVar1 = &local_e;
  uVar5 = SUB42(&DAT_1050_1050,0x0);
  piVar4 = piVar1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack10);
  piStack18 = piVar1;
  uStack16 = uVar2;
  pass1_1030_5b1c(CONCAT22(uVar2,piVar1),(u16 *)CONCAT22(uVar5,piVar4),(u16 *)CONCAT22(uVar7,puVar6));
  pass1_1028_b58e(param_3);
  uStack22 = CONCAT22(uVar2,piVar1);
  local_1c = (u32)(piVar1 + 0x6);
  iStack24 = piVar1[0x8];
  pass1_1028_c8ee(param_3,0x1,(u16 *)CONCAT22(0x1050,&local_1c));
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_1c),(u16 *)CONCAT22(0x1050,&local_22),
                  (u16 *)CONCAT22(0x1050,local_20),(u16 *)CONCAT22(0x1050,local_1e));
  if (local_e < local_22) {
    pass1_1030_5b3e(CONCAT22(uStack16,piStack18),local_22,local_c);
    pass1_1030_5b1c(CONCAT22(uStack16,piStack18),(u16 *)CONCAT22(0x1050,&local_e),(u16 *)CONCAT22(0x1050,&local_c)
                   );
  }
  uStack38 = (u32)((int)uStack22 + 0x2e);
  uStack42 = (u32)((int)uStack38 + 0x4);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_14e),0x0,0x0,0x6f,&local_1c,&DAT_1050_1050,uStack42,
                      uStack10);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_14e));
  pass1_1028_ccd0(param_3,(u16 *)CONCAT22(0x1050,&local_1c));
  return;
}



StructD * pass1_1028_525a(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_52e8(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x535e;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_530a(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x535e;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn FUN_1028_5334(void)

{
  return;
}



StructD * FUN_1028_5338(mut param_1: u16 ,StructD *param_2,u8 param_3)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



pub fn pass1_1028_533c(void) -> u32

{
  let mut unaff_BP: i16;

  pass1_1028_b418((u16*)(unaff_BP + 0x6));
  if ((*(u8 *)(unaff_BP + 0xa) & 0x1) != 0x0) {
    fn_ptr_1000_17ce(*(char **)(unaff_BP + 0x6));
  }
  return CONCAT22((unaff_BP + 0x8),(unaff_BP + 0x6));
}



u16 * struct_1028_53c6(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x54bc;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_53e8(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x54bc;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn pass1_1028_5412(param_1: *mut astruct_15)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut iVar4: i16;

  uVar2 = ((u32)param_1 >> 0x10);
  if (((int)param_1 + 0x12) != 0x6) {
    return;
  }
  uVar3 = pass1_1028_b4f2(param_1);
  if (*(i32 *)((int)uVar3 + 0x200) != 0x8000002) {
    if (*(i32 *)((int)param_1 + 0x1c) == 0x8000002) {
      iVar4 = 0x6;
      goto code_r0x1028548e;
    }
    ppcVar1 = (code **)((int)(u32)param_1 + 0x64);
    iVar4 = (**ppcVar1)();
    if (iVar4 == 0x0) {
      return;
    }
    pass1_1028_c0f0(iVar4,param_1,0x1);
    if (iVar4 == 0x0) {
      iVar4 = 0x6;
      goto code_r0x1028548e;
    }
    pass1_1028_c952(param_1);
    pass1_1028_c00a(iVar4,param_1,0x1);
  }
  iVar4 = 0x5;
code_r0x1028548e:
  pass1_1028_bdac(param_1,iVar4);
  return;
}



StructD * pass1_1028_5496(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1028_5524(u8 *param_1,param_2: *mut u16)

{
  let mut in_register_0000000a: u16;

  struct_1028_0068((astruct_57 *)CONCAT22(in_register_0000000a,param_1),(astruct_180 *)param_2);
  *param_2 = 0x55c8;
  ((int)param_2 + 0x2) = 0x1028;
  return param_2;
}



u16 * pass1_1028_5546(StructD *param_1,param_2: *mut astruct_12,mut param_3: u16 ,mut param_4: u32)

{
  pass1_1028_00cc(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x55c8;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn pass1_1028_5570(param_1: *mut astruct_15)

{
  astruct_57 *in_EDX;
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;

  pass1_1028_0550(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  if (((int)param_1 + 0x12) == 0x5) {
    uVar3 = 0x0;
    iVar4 = 0x4;
    uVar2 = 0x1;
    uVar1 = pass1_1028_b58e((astruct_15 *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10));
    pass1_1030_7c50(uVar1,in_EDX,(astruct_305 *)CONCAT22((int)in_EDX,uVar1),CONCAT22(uVar3,uVar2),iVar4);
  }
  return;
}



StructD * pass1_1028_55a2(StructD *param_1,u8 param_2)

{
  pass1_1028_0138(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_180 * struct_1028_5630(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x56ac;
  ((int)param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_5652(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x56ac;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



u16 FUN_1028_567c(void)

{
  return 0x0;
}
pub fn FUN_1028_5682(void)

{
  return;
}



StructD * FUN_1028_5686(mut param_1: u16 ,StructD *param_2,u8 param_3)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}
pub fn FUN_1028_5714(void)

{
  return;
}



StructD * FUN_1028_5718(mut param_1: u16 ,StructD *param_2,u8 param_3)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



u16 * struct_1028_57a6(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x581c;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_57c8(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x581c;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn FUN_1028_57f2(void)

{
  return;
}



StructD * FUN_1028_57f6(mut param_1: u16 ,StructD *param_2,u8 param_3)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



pub fn pass1_1028_57fa(void) -> u32

{
  let mut unaff_BP: i16;

  pass1_1028_b418((u16*)(unaff_BP + 0x6));
  if ((*(u8 *)(unaff_BP + 0xa) & 0x1) != 0x0) {
    fn_ptr_1000_17ce(*(char **)(unaff_BP + 0x6));
  }
  return CONCAT22((unaff_BP + 0x8),(unaff_BP + 0x6));
}



u16 * pass1_1028_5884(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x58fe;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_58a6(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x58fe;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn FUN_1028_58d0(void)

{
  return;
}
pub fn FUN_1028_58d4(void)

{
  return;
}



StructD * FUN_1028_58d8(mut param_1: u16 ,StructD *param_2,u8 param_3)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



pub fn pass1_1028_58dc(void) -> u32

{
  let mut unaff_BP: i16;

  pass1_1028_b418((u16*)(unaff_BP + 0x6));
  if ((*(u8 *)(unaff_BP + 0xa) & 0x1) != 0x0) {
    fn_ptr_1000_17ce(*(char **)(unaff_BP + 0x6));
  }
  return CONCAT22((unaff_BP + 0x8),(unaff_BP + 0x6));
}



u16 * struct_1028_5966(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
    // just 0x59e0
  param_1.field0_0x0 = (int)s_mineToSmelter__no_mines_1050_59df + 0x1;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_5988(StructD *param_1,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = (int)s_mineToSmelter__no_mines_1050_59df + 0x1;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn FUN_1028_59b2(void)

{
  return;
}
pub fn FUN_1028_59b6(void)

{
  return;
}



StructD * FUN_1028_59ba(mut param_1: u16 ,StructD *param_2,u8 param_3)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



pub fn pass1_1028_59be(void) -> u32

{
  let mut unaff_BP: i16;

  pass1_1028_b418((u16*)(unaff_BP + 0x6));
  if ((*(u8 *)(unaff_BP + 0xa) & 0x1) != 0x0) {
    fn_ptr_1000_17ce(*(char **)(unaff_BP + 0x6));
  }
  return CONCAT22((unaff_BP + 0x8),(unaff_BP + 0x6));
}



u16 * struct_1028_5a48(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
    // just 0x5bec
  param_1.field0_0x0 = s_thisLo_1050_5bec;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_5a6a(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = s_thisLo_1050_5bec;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn FUN_1028_5a94(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_15,u32 *param_4)

{
  code **ppcVar1;
  let mut uVar2: u16;
  u8 *puVar3;
  let mut uVar4: u32;
  let mut extraout_DX: u16;
  let mut uVar5: u16;
  let mut extraout_DX_00: u16;
  let mut uVar6: u32;
  astruct_670 *paVar7;
  let mut uStack14: u32;
  u8 local_a [0x2];
  let mut uStack8: u16;
  let mut uStack6: u32;

  ppcVar1 = (code **)((int)*param_4 + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,param_1);
  if ((extraout_DX | param_1) == 0x0) {
    return;
  }
  uStack8 = 0x1;
  uVar6 = pass1_1030_bcae(local_a,&DAT_1050_1050);
  uVar5 = (uVar6 >> 0x10);
  uStack14 = 0x0;
  while( true ) {
    if (uStack6 <= uStack14) {
      return;
    }
    uVar4 = uStack6;
    pass1_1030_1d58((u32)param_4);
    uVar2 = uVar4;
    uVar6 = (u32)uVar5;
    paVar7 = (astruct_670 *)(uVar4 & 0xffff | (u32)uVar5 << 0x10);
    pass1_1028_b58e(param_3);
    puVar3 = local_a;
    uVar5 = extraout_DX_00;
    pass1_1030_bd74(puVar3,&DAT_1050_1050,CONCAT22(extraout_DX_00,uVar2),paVar7);
    if ((int)puVar3 < 0x5) break;
    uStack14 += 0x1;
  }
  struct_op_1030_73a8((astruct_419 *)(uVar4 & 0xffff | uVar6 << 0x10),puVar3,uVar5);
  return;
}
pub fn pass1_1028_5b42(param_1: *mut astruct_15)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut iVar4: i16;

  uVar2 = ((u32)param_1 >> 0x10);
  if (((int)param_1 + 0x12) != 0x6) {
    return;
  }
  uVar3 = pass1_1028_b4f2(param_1);
  if (*(i32 *)((int)uVar3 + 0x200) != 0x8000002) {
    if (*(i32 *)((int)param_1 + 0x1c) == 0x8000002) {
      iVar4 = 0x6;
      goto code_r0x10285bbe;
    }
    ppcVar1 = (code **)((int)(u32)param_1 + 0x64);
    iVar4 = (**ppcVar1)();
    if (iVar4 == 0x0) {
      return;
    }
    pass1_1028_c0f0(iVar4,param_1,0x2);
    if (iVar4 == 0x0) {
      iVar4 = 0x6;
      goto code_r0x10285bbe;
    }
    pass1_1028_c952(param_1);
    pass1_1028_c00a(iVar4,param_1,0x2);
  }
  iVar4 = 0x5;
code_r0x10285bbe:
  pass1_1028_bdac(param_1,iVar4);
  return;
}



StructD * pass1_1028_5bc6(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_5c54(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
    // just 0x5d8e
  param_1.field0_0x0 = (int)s_static_1050_5d8b + 0x3;
  ((int)param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_5c76(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = (int)s_static_1050_5d8b + 0x3;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn FUN_1028_5ca0(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15)

{
  let mut extraout_DX: u16;
  let mut uVar1: u32;
  u8 local_12e [0x124];
  let mut uStack10: u32;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_b58e(param_3);
  uStack10 = (u32)(param_1 + 0x2e);
  iStack6 = param_1;
  uStack4 = extraout_DX;
  uVar1 = pass1_1028_bb24(param_3);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_12e),0x0,0x0,0x65,(u32 *)(iStack6 + 0xc),uStack4,
                      (u32)((int)uStack10 + 0x4),uVar1);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_12e));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn FUN_1028_5d0e(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15)

{
  let mut uVar1: u32;
  let mut extraout_DX: u16;
  char local_11c [0x10e];

  pass1_1028_b58e(param_3);
  uVar1 = (u32)(param_1 + 0x2e);
  pass1_1028_68de((astruct_97 *)CONCAT22(0x1050,local_11c),0x1,(u32)((int)uVar1 + 0x4));
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_11c));
  return;
}



StructD * pass1_1028_5d68(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_180 * set_fn_ptr_1028_5df6(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
    // just 0x5e70
  param_1.field0_0x0 = (int)s_thisHi_1050_5e6f + 0x1;
  ((int)param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_5e18(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = (int)s_thisHi_1050_5e6f + 0x1;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn FUN_1028_5e42(void)

{
  return;
}
pub fn FUN_1028_5e46(void)

{
  return;
}



StructD * FUN_1028_5e4a(mut param_1: u16 ,StructD *param_2,u8 param_3)

{
  pass1_1028_b418(&param_2.address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



astruct_180 * set_fn_ptr_1028_5ed8(param_1: *mut astruct_180)

{
  let mut uVar1: u16;

  struct_1028_b354(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  ((int)param_1 + 0x20) = 0x0;
  param_1.field0_0x0 = 0x6054;
  ((int)param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_5f00(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  ((int)param_2 + 0x20) = 0x0;
  param_2.field0_0x0 = 0x6054;
  ((int)param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn FUN_1028_5f30(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15)

{
  BOOL16 BVar1;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut uVar2: u16;
  astruct_15 *pstruct15_5;
  let mut uVar5: u16;
  u32 **ppuVar1;
  let mut iVar3: i16;

  pass1_1028_be9e(param_3);
  uVar5 = ((u32)param_3 >> 0x10);
  pstruct15_5 = (astruct_15 *)param_3;
  if (pstruct15_5.field15_0x12 == 0x5) {
    pstruct15_5.field24_0x20 = (astruct_21 *)((int)s_New_failed_in_Op__Op__DialogCtr_1050_0053 + 0x11);
    pass1_1028_b58e(param_3);
    uVar3 = (u32)(param_1 + 0x2e);
    iVar3 = 0x61;
    uVar2 = extraout_DX;
    pass1_1038_3fb0(uVar3);
    BVar1 = pass1_1030_25b2(uVar3 & 0xffff | (u32)uVar2 << 0x10,iVar3);
    if (BVar1 != 0x0) {
      ppuVar1 = (u32 **)&pstruct15_5.field24_0x20;
      *ppuVar1 = *ppuVar1 + 0x19;
    }
  }
  return;
}



BOOL16 write_to_file_1028_5f82(param_1: *mut astruct_731,u8 *param_2)

{
  BOOL16 BVar1;
  HFILE16 in_stack_0000ffde;
  u16 local_c [0x5];

  BVar1 = write_to_file_1028_b5ec(param_1,(u32)param_2);
  if (BVar1 != 0x0) {
    local_c[0] = ((int)param_1 + 0x20);
    BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_c),(char *)0x2,in_stack_0000ffde);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return BVar1;
    }
    BVar1 = 0x1;
  }
  return BVar1;
}
pub fn FUN_1028_5fc8(mut param_1: u16 ,param_2: *mut astruct_373,HFILE16 *param_3)

{
  let mut in_AX: i16;
  BOOL16 BVar1;
  u8 *in_DX;

  file_1028_b81a(in_AX,in_DX,param_2,param_3);
  if ((in_AX != 0x0) &&
     (BVar1 = read_file_1008_7dee(param_3,(u8 *)((u32)param_2 & 0xffff0000 | (u32)((int)param_2 + 0x20)),0x2),
     BVar1 == 0x0)) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  return;
}
pub fn pass1_1028_6008(u32 *param_1)

{
  i16 *piVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;

  pass1_1028_be2a((astruct_15 *)param_1);
  uVar3 = ((u32)param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (((iVar2 + 0x12) == 0x5) && (0x0 < (iVar2 + 0x20))) {
    piVar1 = (i16 *)(iVar2 + 0x20);
    *piVar1 = *piVar1 + -0x1;
  }
  return;
}



StructD * pass1_1028_602e(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * struct_1028_60bc(param_1: *mut astruct_180,param_2: *mut astruct_57,mut param_3: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  astruct_180 *iVar1;
  let mut uVar3: u16;

  struct_1028_b354(param_1);
  uVar3 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_180 *)param_1;
  (u32)(iVar1 + 0x1) = 0x0;
  param_1.field0_0x0 = 0x6876;
  iVar1.field1_0x2 = 0x1028;
  mem_op_1000_179c(0xc,param_2);
  uVar2 = param_2 | param_3;
  if (uVar2 == 0x0) {
    (u32)(iVar1 + 0x1) = 0x0;
  }
  else {
    uVar1 = set_struct_1008_574a((astruct_57 *)CONCAT22(param_2,param_3));
    (iVar1 + 0x1)->field0_0x0 = uVar1;
    iVar1[0x1].field1_0x2 = uVar2;
  }
  return &param_1.field0_0x0;
}



pub fn pass1_1028_611e(StructD *param_1,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  astruct_57 *paVar4;

  uVar1 = ((u32)param_1 >> 0x10);
  paVar4 = (astruct_57 *)(param_1 & 0xffff0000 | (u32)param_1 & 0xffff);
  pass1_1028_b39e((StructD *)((u32)param_1 & 0xffff),param_2,param_3,param_4);
  (u32)((int)param_2 + 0x20) = 0x0;
  param_2.field0_0x0 = 0x6876;
  ((int)param_2 + 0x2) = 0x1028;
  mem_op_1000_179c(0xc,paVar4);
  uVar3 = paVar4 | uVar1;
  if (uVar3 == 0x0) {
    (u32)((int)param_2 + 0x20) = 0x0;
  }
  else {
    uVar2 = set_struct_1008_574a((astruct_57 *)CONCAT22(paVar4,uVar1));
    ((int)param_2 + 0x20) = uVar2;
    ((int)param_2 + 0x22) = uVar3;
  }
  return (u32)param_2;
}
pub fn pass1_1028_6186(StructD *param_1)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  StructD *iVar4;
  let mut uVar4: u16;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar4 = (StructD *)param_1;
  param_1.address_offset_field_0x0 = 0x6876;
  iVar4.address_offset_field_0x2 = 0x1028;
  puVar1 = (u32 *)iVar4.field19_0x20;
  uVar2 = iVar4.field20_0x22;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  return;
}
pub fn pass1_1028_61c4(mut param_1: u16 ,param_2: *mut astruct_15,mut param_3: u32)

{
  astruct_21 *paVar1;
  astruct_21 *uVar4;
  let mut uVar2: u16;
  let mut in_EDX: u32;
  let mut uVar5: u16;
  astruct_15 *iVar5;
  let mut uVar6: u16;
  astruct_21 *puVar1;
  astruct_57 *paVar3;
  code **fn_ptr_1;

  uVar5 = ((u32)in_EDX >> 0x10);
  pass1_1028_b46e(param_1,param_2,param_3);
  uVar6 = ((u32)param_2 >> 0x10);
  iVar5 = (astruct_15 *)param_2;
  paVar1 = iVar5.field24_0x20;
  uVar2 = iVar5.field25_0x22;
  uVar4 = (astruct_21 *)(uVar2 | paVar1);
  paVar3 = (astruct_57 *)CONCAT22(uVar5,uVar4);
  if (uVar4 != NULL) {
    fn_ptr_1 = (code **)(u32)paVar1;
    paVar1 = (astruct_21 *)(**fn_ptr_1)();
  }
  mem_op_1000_179c(0xc,paVar3);
  uVar2 = paVar3 | paVar1;
  if (uVar2 == 0x0) {
    paVar1 = NULL;
    uVar2 = 0x0;
  }
  else {
    paVar1 = (astruct_21 *)set_struct_1008_574a((astruct_57 *)CONCAT22(paVar3,paVar1));
  }
  iVar5.field24_0x20 = paVar1;
  iVar5.field25_0x22 = uVar2;
  return;
}
pub fn pass1_1028_6228(mut param_1: u32,mut param_2: u16 ,mut param_3: i16,mut param_4: i16)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut bVar8: bool;
  i32 lVar9;
  u8 local_a [0x4];
  let mut uStack6: u32;

  uVar7 = (param_1 >> 0x10);
  iVar6 = (int)param_1;
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)(iVar6 + 0x20));
  while( true ) {
    do {
      lVar9 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
      uVar5 = ((u32)lVar9 >> 0x10);
      iVar4 = (int)lVar9;
      if (lVar9 == 0x0) {
        return;
      }
    } while ((iVar4 + 0x6) != param_4);
    uVar1 = (iVar4 + 0xa);
    if ((param_3 == 0x0) && (param_2 < uVar1)) break;
    bVar8 = param_2 < uVar1;
    param_2 -= uVar1;
    param_3 -= bVar8;
    ppcVar3 = (code **)((int)(u32)(u32)(iVar6 + 0x20) + 0xc);
    (**ppcVar3)(0x1008,(u32)(iVar6 + 0x20));
    uStack6 = 0x0;
  }
  uVar2 = (iVar4 + 0xc);
  (iVar4 + 0xa) = uVar1 - param_2;
  (iVar4 + 0xc) = -(param_2 * (uVar2 / uVar1) - (iVar4 + 0xc));
  return;
}



pub fn pass1_1028_62c8(mut param_1: u32) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u32;

  uVar1 = (param_1 >> 0x10);
  if (((int)param_1 + 0x12) == 0x5) {
    uVar2 = pass1_1028_67d4(param_1 & 0xffff | (u32)uVar1 << 0x10);
    uVar1 = uVar2;
    if (((int)(uVar2 >> 0x10) == 0x0) && (uVar1 < 0x64)) {
      return CONCAT22(-(0x64 < uVar1),0x64 - uVar1);
    }
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1028_6302(mut param_1: u32) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  i32 lVar3;
  let mut uStack18: u32;
  u8 local_a [0x8];

  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_1 + 0x20));
  uStack18 = 0x0;
  while( true ) {
    lVar3 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    uVar2 = ((u32)lVar3 >> 0x10);
    if (lVar3 == 0x0) break;
    if (((int)lVar3 + 0x8) != 0x0) {
      uVar1 = ((int)lVar3 + 0xa);
      uStack18 = CONCAT22((int)(uStack18 >> 0x10) + CARRY2(uStack18,uVar1),uStack18 + uVar1);
    }
  }
  return uStack18;
}
pub fn pass1_1028_6356(mut param_1: u32,mut param_2: i16,mut param_3: u16 ,mut param_4: i16)

{
  i16 *piVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  code **ppcVar4;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut bVar9: bool;
  i32 lVar10;
  u8 local_a [0x4];
  let mut uStack6: u32;

  uVar8 = (param_1 >> 0x10);
  iVar7 = (int)param_1;
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)(iVar7 + 0x20));
  while( true ) {
    do {
      lVar10 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
      uVar6 = ((u32)lVar10 >> 0x10);
      iVar5 = (int)lVar10;
      if (lVar10 == 0x0) {
        return;
      }
    } while ((((iVar5 + 0x8) == 0x0) || ((param_2 != 0x0 && ((iVar5 + 0x8) != param_2)))) ||
            (((iVar5 + 0x8) == 0xf && (param_2 != 0xf))));
    uVar2 = (iVar5 + 0xa);
    if ((param_4 == 0x0) && (param_3 < uVar2)) break;
    bVar9 = param_3 < uVar2;
    param_3 -= uVar2;
    param_4 -= bVar9;
    ppcVar4 = (code **)((int)(u32)(u32)(iVar7 + 0x20) + 0xc);
    (**ppcVar4)(0x1008,(u32)(iVar7 + 0x20));
    uStack6 = 0x0;
  }
  uVar3 = (iVar5 + 0xc);
  piVar1 = (i16 *)(iVar5 + 0xa);
  *piVar1 = *piVar1 - param_3;
  piVar1 = (i16 *)(iVar5 + 0xc);
  *piVar1 = *piVar1 - param_3 * (uVar3 / uVar2);
  return;
}
pub fn pass1_1028_6408(mut param_1: u32,u32 *param_2)

{
  code **ppcVar1;
  let mut bVar2: bool;
  u8 *puVar3;
  let mut extraout_DX: u16;
  let mut iVar4: i16;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  u8 local_a [0x8];

  uVar6 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)(iVar4 + 0x20));
  bVar2 = false;
  while( true ) {
    puVar3 = local_a;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar3));
    iVar5 = (int)param_2;
    uVar7 = ((u32)param_2 >> 0x10);
    if ((extraout_DX | puVar3) == 0x0) break;
    if (((puVar3 + 0x4) == (iVar5 + 0x4)) && ((puVar3 + 0x6) == (iVar5 + 0x6))) {
      if ((puVar3 + 0x8) == (iVar5 + 0x8)) {
        bVar2 = true;
        (puVar3 + 0xa) = (puVar3 + 0xa) + (iVar5 + 0xa);
        (puVar3 + 0xc) = (puVar3 + 0xc) + (iVar5 + 0xc);
      }
    }
  }
  if (bVar2) {
    if (param_2 != NULL) {
      ppcVar1 = (code **)*param_2;
      (**ppcVar1)(0x1008,param_2,0x1,param_2,param_2);
      return;
    }
  }
  else {
    ppcVar1 = (code **)((int)(u32)(u32)(iVar4 + 0x20) + 0x4);
    (**ppcVar1)(0x1008,(u32)(iVar4 + 0x20),param_2);
  }
  return;
}



u16 pass1_1028_64d6(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u32;
  BOOL16 BVar2;
  let mut puVar3: *mut u16;
  let mut uVar4: u16;
  HFILE16 in_stack_0000ffc4;
  let mut local_26: u16;
  let mut local_24: u16;
  let mut local_22: u16;
  let mut local_20: u16;
  let mut local_1e: u16;
  u16 local_1c [0x6];
  let mut uStack16: u16;
  i32 lStack14;
  u8 local_a [0x8];

  BVar2 = write_to_file_1028_b5ec((astruct_731 *)param_1,param_2);
  if (BVar2 != 0x0) {
    uVar4 = (param_1 >> 0x10);
    pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_1 + 0x20));
    uVar1 = (u32)((int)param_1 + 0x20);
    local_1c[0] = ((int)uVar1 + 0x8);
    puVar3 = local_1c;
    uStack16 = local_1c[0];
    while( true ) {
      BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,puVar3),(char *)0x2,in_stack_0000ffc4);
      if (BVar2 == 0x0) break;
      lStack14 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
      if (lStack14 == 0x0) {
        return 0x1;
      }
      local_1e = ((int)lStack14 + 0x4);
      BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_1e),(char *)0x2,in_stack_0000ffc4);
      if (BVar2 == 0x0) break;
      local_20 = ((int)lStack14 + 0x6);
      BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_20),(char *)0x2,in_stack_0000ffc4);
      if (BVar2 == 0x0) break;
      local_22 = ((int)lStack14 + 0x8);
      BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_22),(char *)0x2,in_stack_0000ffc4);
      if (BVar2 == 0x0) break;
      local_24 = ((int)lStack14 + 0xa);
      BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_24),(char *)0x2,in_stack_0000ffc4);
      if (BVar2 == 0x0) break;
      local_26 = ((int)lStack14 + 0xc);
      puVar3 = &local_26;
    }
    u16_1050_0310 = 0x6d0;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_65e2(mut param_1: i16,u8 *param_2,param_3: *mut astruct_373,HFILE16 *param_4)

{
  code **ppcVar1;
  let mut uVar2: u16;
  BOOL16 BVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut local_16: u16;
  astruct_99 *paStack20;
  u16 local_10 [0x2];
  u16 local_c [0x3];
  let mut uStack6: u16;
  let mut local_4: u16;

  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 != 0x0) {
    BVar3 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_4),0x2);
    if (BVar3 != 0x0) {
      uStack6 = 0x0;
      while( true ) {
        if (local_4 <= uStack6) {
          return;
        }
        paStack20 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar5 = ((u32)paStack20 >> 0x10);
        uVar2 = paStack20;
        if ((uVar5 | uVar2) == 0x0) {
          paStack20 = NULL;
        }
        else {
          paStack20.field0_0x0 = 0x389a;
          (uVar2 + 0x2) = 0x1008;
          (uVar2 + 0x4) = 0x0;
          (uVar2 + 0x6) = 0x0;
          (uVar2 + 0x8) = 0x0;
          (uVar2 + 0xa) = 0x0;
          (uVar2 + 0xc) = 0x0;
          paStack20->field0_0x0 = 0x56ce;
          (uVar2 + 0x2) = 0x1018;
        }
        BVar3 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,local_10),0x2);
        if (BVar3 == 0x0) break;
        BVar3 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,local_c),0x2);
        if (BVar3 == 0x0) break;
        BVar3 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_16),0x2);
        if (BVar3 == 0x0) break;
        BVar3 = read_file_1008_7dee(param_4,(u8 *)((u32)paStack20 & 0xffff0000 | (u32)((int)paStack20 + 0xa)),0x2);
        if (BVar3 == 0x0) break;
        BVar3 = read_file_1008_7dee(param_4,(u8 *)((u32)paStack20 & 0xffff0000 | (u32)((int)paStack20 + 0xc)),0x2);
        if (BVar3 == 0x0) break;
        ((int)paStack20 + 0x4) = local_10[0];
        uVar4 = switch_1008_72bc(param_4,local_c[0]);
        uVar6 = ((u32)paStack20 >> 0x10);
        ((int)paStack20 + 0x6) = uVar4;
        ((int)paStack20 + 0x8) = local_16;
        ppcVar1 = (code **)((int)(u32)(u32)((int)param_3 + 0x20) + 0x8);
        (**ppcVar1)();
        uStack6 += 0x1;
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



u16 pass1_1028_6744(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u16;
  i32 lVar2;
  u8 local_a [0x8];

  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_1 + 0x20));
  do {
    lVar2 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    uVar1 = ((u32)lVar2 >> 0x10);
    if (lVar2 == 0x0) {
      return 0x0;
    }
  } while (((int)lVar2 + 0x6) != param_2);
  return ((int)lVar2 + 0xa);
}



u16 pass1_1028_678c(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u16;
  i32 lVar2;
  u8 local_a [0x8];

  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_1 + 0x20));
  do {
    lVar2 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    uVar1 = ((u32)lVar2 >> 0x10);
    if (lVar2 == 0x0) {
      return 0x0;
    }
  } while (((int)lVar2 + 0x8) != param_2);
  return ((int)lVar2 + 0xa);
}



// WARNING: Could not reconcile some variable overlaps

pub fn pass1_1028_67d4(mut param_1: u32) -> u32

{
  let mut uVar1: u16;
  i32 lVar2;
  let mut uStack18: u32;
  u8 local_a [0x8];

  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_1 + 0x20));
  uStack18 = 0x0;
  while( true ) {
    lVar2 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    if (lVar2 == 0x0) break;
    uVar1 = ((int)lVar2 + 0xc);
    uStack18 = CONCAT22((int)(uStack18 >> 0x10) + CARRY2(uStack18,uVar1),uStack18 + uVar1);
  }
  return uStack18;
}



u16 pass1_1028_6822(mut param_1: u32,param_2: *mut u16)

{
  let mut iVar1: i16;
  let mut uVar2: u32;

  uVar2 = pass1_1028_67d4(param_1);
  iVar1 = (int)(uVar2 >> 0x10);
  *param_2 = uVar2;
  ((int)param_2 + 0x2) = iVar1;
  if ((iVar1 == 0x0) && (*param_2 < 0x64)) {
    return 0x0;
  }
  return 0x1;
}



StructD * pass1_1028_6850(StructD *param_1,u8 param_2)

{
  pass1_1028_6186(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1028_68de(param_1: *mut astruct_97,mut param_2: u16 ,mut param_3: u32)

{
  astruct_97 *iVar1;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x3e8);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  (u32)&iVar1->field259_0x108 = param_3;
  iVar1->field262_0x10c = param_2;
  param_1->offset_0x0 = 0x6ae2;
  iVar1->segment_0x2 = 0x1028;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),s_SCAddSpew_1050_4fd2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_6926(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  u8 *puVar6;
  u8 *puVar7;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar8;
  let mut uVar9: u16;
  let mut uVar10: u16;
  u32 *puVar11;
  u32 *puStack14;

  uVar9 = (param_3 >> 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)((int)param_3 + 0x108));
  puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0xa);
  puVar6 = (u8 *)((u32)puVar11 >> 0x10);
  uVar4 = puVar11;
  uVar10 = SUB42(&u16_1050_1038,0x0);
  pass1_1038_4d6e(uVar4,puVar6,(astruct_691 *)CONCAT22(param_2,param_1),puVar11);
  puStack14 = (u32 *)CONCAT22(puVar6,uVar4);
  uVar3 = *puStack14;
  ppcVar2 = (code **)((int)uVar3 + 0x10);
  puVar7 = puVar6;
  uVar5 = uVar4;
  (**ppcVar2)((int)&u16_1050_1038,uVar4,puVar6);
  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,puVar7 | uVar5);
  if ((puVar7 | uVar5) != 0x0) {
    ppcVar2 = (code **)((int)uVar3 + 0x4);
    (**ppcVar2)(0x38,uVar4,puVar6,0x0,0x0);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22((int)paVar8,uVar5));
    uVar1 = ((int)param_3 + 0x10c);
    uVar10 = 0x1030;
    pass1_1030_7ddc(uVar1,paVar8,CONCAT22((int)paVar8,uVar5),CONCAT13((u8)((int)uVar1 >> 0xf),(int3)(int)uVar1),
                    0x1f);
  }
  if (puStack14 != NULL) {
    ppcVar2 = (code **)*puStack14;
    (**ppcVar2)(uVar10,uVar4,(char)puVar6,0x1);
  }
  return;
}
pub fn pass1_1028_69cc(param_1: *mut astruct_317,param_2: *mut astruct_57,param_3: *mut astruct_316)

{
  u32 *puVar1;
  u32 *puVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  astruct_316 *iVar5;
  u32 *puVar5;
  u32 *puVar6;
  let mut uVar7: u16;
  let mut puStack10: *mut u16;

  mem_op_1000_179c(0x10e,param_2);
  uVar4 = param_2;
  puStack10 = (u16 *)CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    param_1->field2_0x2 = 0x1008;
    uVar7 = ((u32)param_3 >> 0x10);
    iVar5 = (astruct_316 *)param_3;
    param_1->field3_0x4 = iVar5->field4_0x4;
    puVar5 = &iVar5->field5_0x8;
    puVar6 = &param_1->field4_0x8;
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 += -0x1) {
      puVar2 = puVar6;
      puVar6 = puVar6 + 0x1;
      puVar1 = puVar5;
      puVar5 = puVar5 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1->field2_0x2 = 0x1028;
    param_1->field257_0x108 = iVar5->field258_0x108;
    param_1->field258_0x10c = iVar5->field259_0x10c;
    *puStack10 = 0x6ae2;
    param_1->field2_0x2 = 0x1028;
  }
  return;
}



StructD * pass1_1028_6a7a(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * pass1_1028_6aa6(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1028_6af2(param_1: *mut astruct_97,mut param_2: u32,mut param_3: u32)

{
  astruct_97 *iVar1;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x1387);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  (u32)&iVar1->field259_0x108 = param_3;
  (u32)&iVar1->field262_0x10c = param_2;
  param_1->offset_0x0 = 0x6e50;
  iVar1->segment_0x2 = 0x1028;
  return;
}



u16 pass1_1028_6b2c(mut param_1: u32)

{
  let mut in_DX: u16;

  pass1_1028_6b40(in_DX,param_1);
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_6b40(mut param_1: u16 ,mut param_2: u32)

{
  code **ppcVar1;
  u8 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  u8 local_36 [0xe];
  u32 *puStack40;
  let mut uStack38: u16;
  let mut uStack36: u16;
  let mut uStack34: u16;
  let mut uStack32: u16;
  let mut uStack30: u16;
  let mut uStack28: u16;
  let mut uStack26: u16;
  let mut local_18: u32;
  let mut uStack20: u16;
  let mut uStack18: u32;
  let mut uStack14: u32;
  astruct_15 *pstruct15_10;
  u8 local_6 [0x2];
  let mut local_4: i16;

  puVar2 = local_6;
  pass1_1028_6daa((astruct_15 *)CONCAT22(puVar2,param_1),param_2,(u16 *)CONCAT22(0x1050,puVar2),
                  (u16 *)CONCAT22(0x1050,&local_4),&DAT_1050_1050);
  uVar5 = (param_2 >> 0x10);
  uVar4 = param_2;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(uVar4 + 0x10c));
  pstruct15_10 = (astruct_15 *)CONCAT22(param_1,puVar2);
  ppcVar1 = (code **)((int)(u32)pstruct15_10 + 0x24);
  (**ppcVar1)();
  uStack14 = pass1_1028_b58e(pstruct15_10);
  uStack18 = pass1_1028_bb24(pstruct15_10);
  local_18 = (u32)((int)uStack14 + 0xc);
  uStack20 = ((int)uStack14 + 0x10);
  puStack40 = &local_18;
  uStack26 = local_18;
  uStack28 = ((u32)local_18 >> 0x10);
  uStack32 = local_18 - 0x1;
  if ((int)uStack32 < 0x0) {
    uStack32 = 0x0;
  }
  uVar3 = local_4 - 0x1;
  uStack34 = local_18 + 0x1;
  if ((int)uVar3 < (int)(local_18 + 0x1)) {
    uStack34 = uVar3;
  }
  uStack36 = uStack28 - 0x1;
  if ((int)uStack36 < 0x0) {
    uStack36 = 0x0;
  }
  uStack38 = uStack28 + 0x1;
  if ((int)uVar3 < (int)(uStack28 + 0x1)) {
    uStack38 = uVar3;
  }
  uStack30 = uStack20;
  puVar6 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_36),uStack20,uStack36,uStack32);
  pass1_1028_6d24(((u32)puVar6 >> 0x10),uVar4,uVar5,(u16 *)CONCAT22(0x1050,local_36),uStack18);
  puVar6 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_36),uStack30,uStack36,uStack26);
  pass1_1028_6d24(((u32)puVar6 >> 0x10),uVar4,uVar5,(u16 *)CONCAT22(0x1050,local_36),uStack18);
  puVar6 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_36),uStack30,uStack36,uStack34);
  pass1_1028_6d24(((u32)puVar6 >> 0x10),uVar4,uVar5,(u16 *)CONCAT22(0x1050,local_36),uStack18);
  puVar6 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_36),uStack30,uStack28,uStack32);
  pass1_1028_6d24(((u32)puVar6 >> 0x10),uVar4,uVar5,(u16 *)CONCAT22(0x1050,local_36),uStack18);
  puVar6 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_36),uStack30,uStack28,uStack34);
  pass1_1028_6d24(((u32)puVar6 >> 0x10),uVar4,uVar5,(u16 *)CONCAT22(0x1050,local_36),uStack18);
  puVar6 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_36),uStack30,uStack38,uStack32);
  pass1_1028_6d24(((u32)puVar6 >> 0x10),uVar4,uVar5,(u16 *)CONCAT22(0x1050,local_36),uStack18);
  puVar6 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_36),uStack30,uStack38,uStack26);
  pass1_1028_6d24(((u32)puVar6 >> 0x10),uVar4,uVar5,(u16 *)CONCAT22(0x1050,local_36),uStack18);
  puVar6 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_36),uStack30,uStack38,uStack34);
  pass1_1028_6d24(((u32)puVar6 >> 0x10),uVar4,uVar5,(u16 *)CONCAT22(0x1050,local_36),uStack18);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_6d24(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut u16,i32 param_5)

{
  let mut iVar1: i16;
  code **ppcVar2;
  u32 *puVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  u32 *puVar6;
  u8 bStack27;
  let mut local_a: u32;
  let mut uStack6: u32;

  puVar3 = &local_a;
  pass1_1030_64ce(puVar3,param_1,_PTR_LOOP_1050_5740,param_4,param_5,(u32 *)CONCAT22(0x1050,puVar3));
  uStack6 = *puVar3;
  uVar5 = ((int)puVar3 + 0x2);
  bStack27 = (u8)(uStack6 >> 0x18);
  uVar4 = bStack27;
  if (bStack27 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6 & 0xffff | (u32)uVar5 << 0x10);
    puVar6 = (u32 *)struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar5,uVar4),uVar4,uVar5);
    iVar1 = ((int)puVar6 + 0xc);
    if (((iVar1 < 0x1) || (SBORROW2(iVar1,0x1))) ||
       ((iVar1 != 0x9 && 0x7 < iVar1 + -0x1 && ((iVar1 + -0x9 < 0x6a || (0x6 < iVar1 + -0x73)))))) {
      ppcVar2 = (code **)((int)*puVar6 + 0x24);
      (**ppcVar2)();
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_6daa(param_1: *mut astruct_15,mut param_2: u32,param_3: *mut u16,param_4: *mut u16,mut param_5: u16 )

{
  let mut uVar1: u32;
  u32 *puVar2;
  let mut local_18: u32;
  let mut uStack20: u16;
  let mut iStack18: i16;
  let mut uStack16: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut uStack6: u16;
  let mut uStack4: u16;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)((int)param_2 + 0x10c));
  uStack10 = pass1_1028_b4f2((astruct_15 *)CONCAT22((int)param_1,(int)((u32)param_1 >> 0x10)));
  uStack6 = ((u32)param_1 >> 0x10);
  uStack4 = SUB42(param_1,0x0);
  uStack16 = (uStack10 >> 0x10);
  uVar1 = (u32)((int)uStack10 + 0x8);
  uStack14 = uVar1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1);
  uStack6 = ((u32)param_1 >> 0x10);
  uStack4 = SUB42(param_1,0x0);
  iStack18 = (int)uVar1;
  puVar2 = (u32 *)pass1_1030_5b5c(iStack18,uStack16);
  uStack6 = ((u32)param_1 >> 0x10);
  uStack4 = SUB42(param_1,0x0);
  local_18 = *puVar2;
  uStack20 = ((int)puVar2 + 0x4);
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_18),param_3,(char *)param_4);
  uStack6 = ((u32)param_1 >> 0x10);
  uStack4 = SUB42(param_1,0x0);
  return;
}



StructD * pass1_1028_6e24(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_6e60(uchar param_1,param_2: *mut astruct_97)

{
  struct_op_1028_d1dc(param_2,0x32c7);
  param_2->offset_0x0 = 0x6fb0;
  ((int)param_2 + 0x2) = 0x1028;
  unk_str_op_1000_3d3e((char *)((u32)param_2 & 0xffff0000 | (u32)((int)param_2 + 0x8)),s_SCConstruct_1050_4fdc);
  return param_2;
}



u16 pass1_1028_6e96(mut param_1: u16 )

{
  astruct_92 *paVar1;
  code **ppcVar2;
  astruct_92 **ppaVar3;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  u32 *puStack24;
  astruct_92 *local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x700);
  while( true ) {
    ppaVar3 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,ppaVar3));
    puStack24 = (u32 *)CONCAT22(param_1,ppaVar3);
    uVar4 = param_1 | ppaVar3;
    if (uVar4 == 0x0) break;
    paVar1 = ppaVar3[0x9];
    param_1 = uVar4;
    if (((0x0 < (int)paVar1) && (!SBORROW2((int)paVar1,0x1))) && ((int)((int)&paVar1[-0x1].field6_0x10 + 0x1) < 0x4)) {
      ppcVar2 = (code **)((int)*puStack24 + 0x38);
      (**ppcVar2)();
      param_1 = extraout_DX;
    }
  }
  return 0x1;
}
pub fn pass1_1028_6ef6(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

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
    *puStack10 = 0x6fb0;
    (param_1 + 0x2) = 0x1028;
  }
  return;
}



StructD * pass1_1028_6f84(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_6fc0(uchar param_1,param_2: *mut astruct_97)

{
  struct_op_1028_d1dc(param_2,0x3e7);
  param_2->offset_0x0 = 0x749e;
  ((int)param_2 + 0x2) = 0x1028;
  unk_str_op_1000_3d3e((char *)((u32)param_2 & 0xffff0000 | (u32)((int)param_2 + 0x8)),s_SCEndSim_1050_4fea);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_6ff6(StructD *param_1,mut param_2: u32,mut param_3: u16 )

{
  let mut bVar1: bool;
  let mut bVar2: bool;
  let mut bVar3: bool;
  astruct_92 *paVar4;
  astruct_92 *paVar5;
  let mut uVar6: u16;
  let mut uVar7: u16;
  astruct_57 *paVar8;
  let mut uVar9: u32;
  let mut unaff_DI: i16;
  astruct_27 *paVar10;
  astruct_67 *paVar11;
  astruct_102 *paVar12;
  astruct_679 *paVar13;
  let mut in_stack_0000fe52: u16;
  let mut in_stack_0000fe5c: u16;
  let mut in_stack_0000fe60: u16;
  let mut in_stack_0000ff76: u16;
  let mut in_stack_0000ff7c: u16;
  let mut in_stack_0000ff80: u16;
  let mut in_stack_0000ff84: u16;
  let mut in_stack_0000ff86: u16;
  let mut in_stack_0000ff8a: u16;
  let mut in_stack_0000ff8e: u16;
  let mut uVar14: u16;
  let mut uVar15: u16;
  let mut iVar16: i16;
  u8 uVar17;
  u8 uVar18;
  let mut uVar19: u16;
  let mut uVar20: u16;
  let mut uVar21: u16;
  let mut iVar22: i16;
  astruct_92 local_46;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_14)),0x1,0x0,0x400);
  bVar3 = true;
  bVar2 = false;
  do {
    do {
      paVar4 = &local_14;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar4));
      uVar7 = param_1;
      param_1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)(uVar7 | paVar4));
      if ((uVar7 | paVar4) == 0x0) goto LAB_1028_7066;
    } while ((((int)&paVar4[0x1c].field3_0x4 + 0x2) == 0x0) || (paVar4[0x1c].field4_0x8 == 0x8000002));
    bVar2 = true;
    iVar16 = (int)(u32)&paVar4[0x1b].field6_0x10;
    pass1_1030_38b8();
  } while (((int)param_1 < 0x0) || (((int)param_1 < 0x1 && (iVar16 == 0x0))));
  bVar3 = false;//
LAB_1028_7066:
  if (local_14.field6_0x10 == 0x0) {
    paVar8 = (astruct_57 *)((u32)param_1 & 0xffff0000 | (u32)local_14.field5_0xc);
    local_14.field4_0x8._0_2_ = local_14.field5_0xc;
  }
  else {
    paVar8 = (astruct_57 *)((u32)param_1 & 0xffff0000);
    local_14.field4_0x8._0_2_ = 0x1;
  }
  local_14.field4_0x8 = SUB42(paVar8,0x0);
  bVar1 = false;
  while( true ) {
    paVar4 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar4));
    uVar7 = paVar8;
    uVar6 = uVar7 | paVar4;
    paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)uVar6);
    if (uVar6 == 0x0) break;
    if (paVar4[0x1c].field4_0x8 == 0x8000001) {
      bVar1 = true;
    }
  }
  if (!bVar1) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
    uVar7 = paVar8 | paVar4;
    paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)uVar7);
    if (uVar7 != 0x0) {
      PTR_LOOP_1050_4fe8 = (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1);
      uVar20 = 0x0;
      iVar16 = 0x1;
      paVar10 = (astruct_27 *)
                mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)0x2b,in_stack_0000fe5c,in_stack_0000ff80,
                                in_stack_0000ff86,in_stack_0000ff8a);
      paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)paVar10 >> 0x10);
      paVar4 = (astruct_92 *)paVar10;
      pass1_1010_089e(paVar10,uVar20,iVar16);
      pass1_1010_089e(paVar10,0x0,0x2);
      pass1_1010_089e(paVar10,0x0,0x3);
      pass1_1010_089e(paVar10,0x0,0x4);
      pass1_1010_089e(paVar10,0x0,0x5);
      pass1_1010_089e(paVar10,0x0,0x7);
      pass1_1010_089e(paVar10,0x0,0x8);
      pass1_1010_089e(paVar10,0x0,0xa);
    }
  }
  if ((bVar2) && (bVar3)) {
    uVar21 = 0x0;
    iVar22 = 0x6;
    uVar17 = 0x1;
    uVar18 = 0x0;
    uVar19 = 0x0;
    uVar15 = 0x0;
    iVar16 = 0x0;
    uVar14 = 0x0;
    paVar11 = (astruct_67 *)
              mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)0x37,in_stack_0000fe52,in_stack_0000ff76,
                              in_stack_0000ff7c,in_stack_0000ff80);
    paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)paVar11 >> 0x10);
    paVar4 = (astruct_92 *)paVar11;
    post_win_msg_1008_a0e4
              (paVar11,CONCAT22(uVar15,uVar14),iVar16,CONCAT11(uVar18,uVar17),CONCAT22(uVar21,uVar19),iVar22);
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x8000001);
  uVar7 = paVar8;
  paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)(uVar7 | paVar4));
  paVar5 = paVar4;
  if ((((((uVar7 | paVar4) != 0x0) &&
        (paVar5 = (astruct_92 *)pass1_1030_2242((astruct_168 *)CONCAT22(uVar7,paVar4),0x4), paVar5 == NULL)) &&
       (paVar5 = (astruct_92 *)pass1_1030_2242((astruct_168 *)CONCAT22(uVar7,paVar4),0x2a), paVar5 == NULL)) &&
      ((paVar5 = (astruct_92 *)pass1_1030_2242((astruct_168 *)CONCAT22(uVar7,paVar4),0x4b), paVar5 == NULL &&
       (paVar5 = (astruct_92 *)pass1_1030_2242((astruct_168 *)CONCAT22(uVar7,paVar4),0x54), paVar5 == NULL)))) &&
     ((paVar5 = (astruct_92 *)pass1_1030_2242((astruct_168 *)CONCAT22(uVar7,paVar4),0x2c), paVar5 == NULL &&
      ((paVar5 = (astruct_92 *)pass1_1030_2242((astruct_168 *)CONCAT22(uVar7,paVar4),0x3c), paVar5 == NULL &&
       (paVar5 = (astruct_92 *)pass1_1030_2242((astruct_168 *)CONCAT22(uVar7,paVar4),0x3d), paVar5 == NULL)))))) {
    if (local_14.field6_0x10 == 0x0) {
      paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)local_14.field5_0xc);
    }
    else {
      local_14.field5_0xc._0_2_ = 0x1;
      paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000);
    }
    local_14.field4_0x8 = SUB42(paVar8,0x0);
    bVar2 = false;
    bVar3 = false;
    local_14.field4_0x8._0_2_ = local_14.field5_0xc;
    do {
      do {
        paVar5 = &local_14;
        pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar5));
        uVar7 = paVar8;
        paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)(uVar7 | paVar5));
        if ((uVar7 | paVar5) == 0x0) goto LAB_1028_72d3;
      } while (paVar5[0x1c].field4_0x8 == 0x8000002);
      uVar20 = (param_2 >> 0x10);
      paVar4 = paVar5;
      if ((!bVar2) && (pass1_1028_740c(param_2,uVar20,0x22,CONCAT22(uVar7,paVar5)), paVar4 != NULL)) {
        bVar2 = true;
      }
      if ((!bVar3) && (pass1_1028_740c(param_2,uVar20,0x24,CONCAT22(uVar7,paVar5)), paVar4 != NULL)) {
        bVar3 = true;
      }
    } while ((!bVar2) || (!bVar3));
    uVar21 = 0x0;
    iVar22 = 0x14;
    uVar17 = 0x1;
    uVar18 = 0x0;
    uVar19 = 0x0;
    uVar15 = 0x0;
    iVar16 = 0x0;
    uVar14 = 0x0;
    paVar11 = (astruct_67 *)
              mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)0x37,in_stack_0000fe52,in_stack_0000ff76,
                              in_stack_0000ff7c,in_stack_0000ff80);
    paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)paVar11 >> 0x10);
    paVar5 = (astruct_92 *)paVar11;
    post_win_msg_1008_a0e4
              (paVar11,CONCAT22(uVar15,uVar14),iVar16,CONCAT11(uVar18,uVar17),CONCAT22(uVar21,uVar19),iVar22);
  }//
LAB_1028_72d3:
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
  uVar7 = paVar8 | paVar5;
  paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)uVar7);
  if (uVar7 != 0x0) {
    paVar12 = (astruct_102 *)
              mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x3b),in_stack_0000fe60,
                              in_stack_0000ff84,in_stack_0000ff8a,in_stack_0000ff8e);
    paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)paVar12 >> 0x10);
    pass1_1008_df4a(paVar12,unaff_DI,&DAT_1050_1050);
    paVar13 = (astruct_679 *)
              mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x3c),in_stack_0000fe60,
                              in_stack_0000ff84,in_stack_0000ff8a,in_stack_0000ff8e);
    uVar9 = (u32)paVar13 >> 0x10;
    pass1_1018_34a6(paVar13);
    pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_46)),0x1,0x0,0x400);
    while( true ) {
      uVar7 = uVar9;
      paVar4 = &local_46;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar4));
      uVar9 = (u32)(uVar7 | paVar4);
      if ((uVar7 | paVar4) == 0x0) break;
      if (paVar4[0x1c].field4_0x8 != 0x8000002) {
        pass1_1038_3ba0((astruct_428 *)CONCAT22(uVar7,paVar4));
      }
    }
  }
  return;
}
pub fn pass1_1028_737e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

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
    *puStack10 = 0x749e;
    (param_1 + 0x2) = 0x1028;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_740c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,mut param_4: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 *puVar5;
  let mut extraout_DX: u16;
  u32 *puVar6;
  i32 lStack14;
  u32 *puStack10;

  puVar6 = pass1_1008_c6fa(_u16_1050_06e0,param_3);
  puVar5 = (u8 *)((u32)puVar6 >> 0x10);
  uVar3 = puVar6;
  pass1_1038_4d6e(uVar3,puVar5,(astruct_691 *)param_4,puVar6);
  puStack10 = (u32 *)CONCAT22(puVar5,uVar3);
  uVar2 = *puStack10;
  ppcVar1 = (code **)uVar2 + 0x8;
  uVar4 = uVar3;
  (**ppcVar1)((int)&u16_1050_1038,uVar3,puVar5);
  lStack14 = CONCAT22(extraout_DX,uVar4);
  if (puStack10 != NULL) {
    ppcVar1 = (code **)uVar2;
    (**ppcVar1)((int)&u16_1050_1038,uVar3,(char)puVar5,0x1);
  }
  if (lStack14 != 0x0) {
    return;
  }
  return;
}



StructD * pass1_1028_7472(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_74ae(param_1: *mut astruct_97)

{
  struct_op_1028_d1dc(param_1,0x1387);
  param_1->offset_0x0 = 0x819a;
    // just 0x1028
  ((int)param_1 + 0x2) = 0x1028;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x8)),s_SCEvent_1050_4ff4);
  return param_1;
}



u16 pass1_1028_74e4(uchar param_1,param_2: *mut astruct_57,mut param_3: u32)

{
  let mut iVar1: i16;

  pass1_1028_7fb6(param_1,param_3);
  pass1_1028_7c4e(param_2,param_3);
  pass1_1028_7dfc(param_1,(u8 *)param_2,param_3);
  iVar1 = post_msg_1028_76da();
  pass1_1028_767e(iVar1,param_2);
  pass1_1028_75bc();
  pass1_1028_78b8(param_1,(long)param_2,param_3);
  return 0x1;
}
pub fn pass1_1028_752e(mut param_1: u16 ,u8 *param_2,mut param_3: u32)

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
    *puStack10 = 0x819a;
    (param_1 + 0x2) = 0x1028;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_75bc(void)

{
  astruct_92 *paVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut in_stack_0000ffcc: u16;
  let mut uVar5: u32;
  let mut uStack28: u32;
  astruct_92 local_18;

  uVar2 = ((qword)*_PTR_LOOP_1050_65e2 % 0x7b);
  uVar4 = (u32)uVar2;
  if ((uVar2 == 0x0) && (0x95 < *_PTR_LOOP_1050_65e2)) {
    uVar5 = CONCAT22(0x7603,in_stack_0000ffcc);
    pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_18),0x1,0x0,0x400);
    while( true ) {
      paVar1 = &local_18;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar1));
      uStack28 = CONCAT22(uVar4,paVar1);
      uVar2 = uVar4 | paVar1;
      uVar4 = uVar4 & 0xffff0000 | (u32)uVar2;
      if (uVar2 == 0x0) break;
      pass1_1008_612e(paVar1,0x1,0x64);
      if ((int)paVar1 < 0x6) {
        pass1_1038_362e(uStack28,paVar1,uVar5,uVar4);
      }
    }
    if (local_18.field6_0x10 != 0x0) {
      local_18.field5_0xc._0_2_ = 0x1;
      local_18.field5_0xc = 0x0;
    }
    uVar4 = (u32)local_18.field5_0xc;
    local_18.field4_0x8._0_2_ = local_18.field5_0xc;
    local_18.field4_0x8 = local_18.field5_0xc;
    while( true ) {
      uVar2 = uVar4;
      paVar1 = &local_18;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar1));
      uVar3 = uVar2 | paVar1;
      uVar4 = (u32)uVar3;
      if (uVar3 == 0x0) break;
      pass1_1038_3698(paVar1,uVar3,CONCAT22(uVar2,paVar1));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_767e(mut param_1: i16,mut param_2: u16 )

{
  let mut uVar1: u16;
  u32 *puVar2;
  let mut in_stack_0000fe94: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffec: u16;

  pass1_1028_e1ec((u32)_PTR_LOOP_1050_65e2,0x8000001);
  if (((param_1 + 0x152) != 0x0) && (uVar1 = ((qword)*_PTR_LOOP_1050_65e2 % 0x64), uVar1 == 0x0)) {
    puVar2 = mixed_1010_20ba((astruct_57 *)(u32)uVar1,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffec,0x40),
                             in_stack_0000fe94,in_stack_0000ffb8,in_stack_0000ffbe,in_stack_0000ffc2);
    load_str_and_sprintf_1008_b78a((StructD *)CONCAT22((int)puVar2,(int)((u32)puVar2 >> 0x10)),(u32)puVar2);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn post_msg_1028_76da(void)

{
  i32 lVar1;
  let mut uVar2: u16;
  astruct_57 *in_EDX;
  u32 *puVar3;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffe4: u32;
  let mut uStack10: u16;
  let mut uStack8: u16;

  puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22((int)((u32)in_stack_0000ffe4 >> 0x10),0x2c),
                           in_stack_0000fe8e,in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  uVar2 = ((u32)puVar3 >> 0x10);
  lVar1 = *(i32 *)((int)puVar3 + 0xc);
  uStack8 = ((u32)lVar1 >> 0x10);
  uStack10 = lVar1;
  if (((uStack8 | uStack10) != 0x0) && (*_PTR_LOOP_1050_65e2 == lVar1)) {
    PostMessage16(0x0,0x106,0x111,HWND16_1050_0396);
    (u32)((int)puVar3 + 0xc) = 0x0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_7742(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,param_4: *mut astruct_15)

{
  code **ppcVar1;
  astruct_670 *paVar2;
  let mut uVar3: u16;
  u8 *puVar4;
  let mut uVar5: u16;
  u8 *puVar6;
  let mut extraout_DX: u16;
  let mut uVar7: u16;
  let mut extraout_DX_00: u16;
  u32 *puVar8;
  astruct_691 *paVar9;
  let mut uVar10: u32;
  u8 uVar11;
  u8 uVar12;
  let mut uVar13: u16;
  let mut uStack26: u32;
  u8 local_16 [0x2];
  let mut uStack20: u32;
  let mut uStack16: u16;
  u32 *puStack14;
  let mut uStack10: u16;
  u8 *puStack8;
  let mut uStack6: u16;
  let mut uStack4: u16;

  puVar8 = pass1_1008_c6fa(_u16_1050_06e0,0x18);
  uVar5 = ((u32)puVar8 >> 0x10);
  uVar7 = SUB42(puVar8,0x0);
  uStack6 = uVar7;
  uStack4 = uVar5;
  paVar9 = (astruct_691 *)pass1_1028_b4f2(param_4);
  puVar6 = (u8 *)((u32)paVar9 >> 0x10);
  uVar3 = paVar9;
  uStack10 = uVar3;
  puStack8 = puVar6;
  pass1_1038_4d6e(uVar3,puVar6,paVar9,(u32 *)CONCAT22(uVar5,uVar7));
  puStack14 = (u32 *)CONCAT22(puVar6,uVar3);
  uStack16 = 0x0;
  ppcVar1 = (code **)((int)*puStack14 + 0x10);
  uVar13 = uVar3;
  (**ppcVar1)((int)&u16_1050_1038,uVar3,puVar6);
  uStack20 = CONCAT22(extraout_DX,uVar3);
  uVar10 = pass1_1030_bcae(local_16,&DAT_1050_1050);
  uVar7 = (uVar10 >> 0x10);
  uStack26 = 0x0;
  do {
    if (uStack20 <= uStack26) {//
LAB_1028_77e7:
      if (puStack14 != NULL) {
        ppcVar1 = (code **)*puStack14;
        (**ppcVar1)(0x1030,(int)puStack14,(char)((u32)puStack14 >> 0x10),0x1,uVar13,puVar6,puStack14,puStack14);
      }
      return;
    }
    uVar10 = uStack20;
    pass1_1030_1d58((u32)puStack14);
    uVar5 = uVar10;
    uVar11 = (u8)uVar10;
    uVar12 = (u8)(uVar10 >> 0x8);
    pass1_1028_b58e(param_4);
    puVar4 = local_16;
    paVar2 = (astruct_670 *)CONCAT22(uVar7,CONCAT11(uVar12,uVar11));
    uVar7 = extraout_DX_00;
    pass1_1030_bd74(puVar4,&DAT_1050_1050,CONCAT22(extraout_DX_00,uVar5),paVar2);
    if ((int)puVar4 <= param_3) {
      uStack16 = 0x1;
      goto LAB_1028_77e7;
    }
    uStack26 += 0x1;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_780c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar7;
  let mut uVar8: u16;
  u32 *puVar9;
  u32 *puVar10;
  let mut uStack18: u32;
  let mut uStack14: u32;
  u32 *puStack10;

  uVar8 = ((u32)in_EDX >> 0x10);
  puVar9 = pass1_1008_c6fa(_u16_1050_06e0,0x25);
  paVar7 = (astruct_57 *)CONCAT22(uVar8,(int)((u32)puVar9 >> 0x10));
  uVar2 = puVar9;
  uVar8 = SUB42(&u16_1050_1038,0x0);
  pass1_1038_4e78(uVar2,paVar7,param_3,puVar9);
  uVar6 = paVar7;
  puStack10 = (u32 *)CONCAT22(uVar6,uVar2);
  ppcVar1 = (code **)((int)*puStack10 + 0x10);
  uVar3 = uVar2;
  (**ppcVar1)((int)&u16_1050_1038,uVar2,uVar6);
  uStack14 = CONCAT22(uVar6,uVar3);
  uVar5 = (u32)(uVar6 | uVar3);
  if ((uVar6 | uVar3) == 0x0) {
    return;
  }
  uStack18 = 0x0;
  while( true ) {
    uVar3 = uVar5;
    if (uStack14 <= uStack18) break;
    ppcVar1 = (code **)((int)*puStack10 + 0x4);
    uVar5 = uStack14;
    (**ppcVar1)();
    uVar4 = uVar5;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5 & 0xffff | (u32)uVar3 << 0x10);
    uVar8 = 0x1030;
    puVar10 = (u32 *)struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar3,uVar4),uVar4,uVar3);
    uVar5 = (u32)puVar10 >> 0x10;
    ppcVar1 = (code **)((int)*puVar10 + 0x24);
    (**ppcVar1)();
    uStack18 += 0x1;
  }
  if (puStack10 != NULL) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(uVar8,uVar2,(char)paVar7,0x1);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_78b8(uchar param_1,i32 param_2,mut param_3: u32)

{
  u8 *puVar1;
  u32 *puVar2;
  let mut uVar3: u16;
  astruct_92 *paVar4;
  u8 *puVar5;
  let mut puVar6: *mut u16;
  let mut puVar7: *mut u16;
  let mut uVar8: u32;
  u8 *puVar9;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut iVar12: i16;
  astruct_57 *paVar13;
  let mut unaff_SI: u16;
  let mut bVar15: bool;
  let mut bVar16: bool;
  let mut puVar17: *mut u16;
  let mut puVar18: *mut u16;
  astruct_27 *paVar19;
  u32 *puVar20;
  let mut in_stack_0000fd48: u16;
  let mut in_stack_0000fd4e: u16;
  let mut in_stack_0000fd52: u16;
  let mut in_stack_0000fe6c: u16;
  let mut in_stack_0000fe72: u16;
  let mut in_stack_0000fe76: u16;
  let mut in_stack_0000fe78: u16;
  let mut in_stack_0000fe7c: u16;
  let mut uVar21: u16;
  let mut uVar22: u16;
  let mut uVar23: u16;
  let mut iVar24: i16;
  let mut uStack340: u16;
  let mut uStack338: u16;
  u32 *puStack74;
  u8 *puStack70;
  let mut uStack68: u16;
  astruct_92 local_42;
  astruct_92 local_30;
  u8 *local_1e [0x3];
  let mut local_18: u32;
  u8 *puStack20;
  let mut uStack18: u16;
  astruct_364 *paStack16;
  u8 *puStack12;
  let mut uStack10: u16;
  let mut uStack8: u16;
  let mut uStack6: u32;
  astruct_57 *paVar14;

  puVar9 = (u8 *)param_2;
  uVar8 = *_PTR_LOOP_1050_65e2;
  uStack6 = uVar8;
  if (uVar8 == 0x98) {
    pass1_1028_e1ec((u32)_PTR_LOOP_1050_65e2,0x4000002);
    puVar9 = (u8 *)param_2;
    paStack16 = (astruct_364 *)(uVar8 & 0xffff | param_2 << 0x10);
    if (*(i32 *)((int)uVar8 + 0x200) == 0x8000002) {
      pass1_1020_a43e(puVar9,(u16 *)CONCAT22(0x1050,&local_18));
      puVar17 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_1e));
      puVar9 = (u8 *)((u32)puVar17 >> 0x10);
      puVar2 = &local_18;
      pass1_1020_a49a(puVar9,in_stack_0000fd52,CONCAT22(0x1050,puVar2),(i16 *)CONCAT22(0x1050,local_1e),0x7a);
      pass1_1038_4f54(puVar2,(u32)paStack16,0x1);
      if (puVar2 == NULL) {
        pass1_1020_a49a(puVar9,in_stack_0000fd52,CONCAT22(0x1050,&local_18),NULL,0x35);
      }
    }
  }
  if ((0xe < uStack6) && (uStack6 < 0x16)) {
    puVar18 = pass1_1020_a43e(puVar9,(u16 *)CONCAT22(0x1050,local_1e));
    local_18 = uStack6 - 0xf;
    pass1_1020_a54c((u8 *)((u32)puVar18 >> 0x10),local_1e,&DAT_1050_1050,(i16)local_18);
  }
  paVar13 = (astruct_57 *)((qword)uStack6 % 0x7d);
  paVar14 = paVar13;
  if (paVar13 == NULL) {
    pass1_1008_612e(((qword)uStack6 % 0x7d),0x1,0x64);
    local_1e[0] = (u8 *)paVar13;
    if ((int)local_1e[0] < 0x1a) {
      pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_30),0x1,0x0,0x400);
      do {
        paVar13 = (astruct_57 *)ZEXT24(&local_30);
        pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,&local_30));
        uVar3 = paVar13;
        uVar10 = paVar14;
        local_18 = (u32)paVar13 & 0xffff | (long)paVar14 << 0x10;
        uVar11 = uVar10 | uVar3;
        paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)uVar11);
        if (uVar11 == 0x0) goto LAB_1028_79d6;
      } while (*(i32 *)(uVar3 + 0x200) == 0x8000002);
      pass1_1038_43cc(uVar3,uVar11,uVar3,uVar10,0x1,0x4);//
LAB_1028_79d6:
      local_30._0_2_ = (u8 *)0x389a;
      local_30.field2_0x2 = 0x1008;
    }
  }
  puVar5 = (u8 *)paVar13;
  if (uStack6 == 0x5) {
    uVar23 = SUB42(&DAT_1050_1050,0x0);
    uVar22 = SUB42(s_Rebel_1050_4ffc,0x0);
    pass1_1028_e1ec((u32)_PTR_LOOP_1050_65e2,0x4000002);
    local_30.field2_0x2 = paVar14;
    local_30._0_2_ = puVar5;
    pass1_1038_4d3c(CONCAT22(local_30.field2_0x2,puVar5),(char *)CONCAT22(uVar23,uVar22),local_30.field2_0x2);
  }
  if (uStack6 == 0x12c) {
    uVar23 = 0x400;
    iVar12 = 0xf;
    uVar22 = 0x1;
    paVar19 = (astruct_27 *)
              mixed_1010_20ba(paVar14,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fd48,in_stack_0000fe6c,
                              in_stack_0000fe72,in_stack_0000fe76);
    paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)paVar19 >> 0x10);
    puVar5 = (u8 *)paVar19;
    local_30.field2_0x2 = ((u32)paVar19 >> 0x10);
    local_30._0_2_ = puVar5;
    pass1_1010_043a(paVar19,CONCAT22(uVar23,uVar22),iVar12);
  }
  if (uStack6 == 0x3d) {
    puVar20 = mixed_1010_20ba(paVar14,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000fd4e,
                              in_stack_0000fe72,in_stack_0000fe78,in_stack_0000fe7c);
    paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)puVar20 >> 0x10);
    local_30._0_2_ = (u8 *)puVar20;
    local_30.field2_0x2 = ((u32)puVar20 >> 0x10);
    local_1e[0] = PTR_LOOP_1050_13ae;
    puVar5 = PTR_LOOP_1050_13ae;
    if (PTR_LOOP_1050_13ae != (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1)) {
      pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_42),0x1,0x0,0x400);
      while( true ) {
        paVar4 = &local_42;
        pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar4));
        uVar3 = paVar14;
        local_18 = CONCAT22(uVar3,paVar4);
        paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)(uVar3 | paVar4));
        if ((uVar3 | paVar4) == 0x0) break;
        paStack16 = *(astruct_364 **)&paVar4[0x1b].field6_0x10;
        pass1_1030_34da(paStack16);
      }
      uVar23 = 0x400;
      iVar12 = 0x10;
      uVar22 = 0x1;
      paVar19 = (astruct_27 *)
                mixed_1010_20ba(paVar14,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fd48,in_stack_0000fe6c,
                                in_stack_0000fe72,in_stack_0000fe76);
      paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)paVar19 >> 0x10);
      puVar5 = (u8 *)paVar19;
      uStack18 = ((u32)paVar19 >> 0x10);
      puStack20 = puVar5;
      pass1_1010_043a(paVar19,CONCAT22(uVar23,uVar22),iVar12);
      local_42._0_4_ = &PTR_pass1_1008_377e_1008_389a;
    }
  }
  if (uStack6 == 0x96) {
    pass1_1028_e1ec((u32)_PTR_LOOP_1050_65e2,0x4000001);
    puStack74 = (u32 *)CONCAT22((int)paVar14,puVar5);
    uVar21 = (param_3 >> 0x10);
    pass1_1028_780c(param_3,uVar21,CONCAT22((int)paVar14,puVar5));
    if (puVar5 != NULL) {
      uVar23 = 0x400;
      iVar12 = 0x1d;
      uVar22 = 0x1;
      paVar19 = (astruct_27 *)
                mixed_1010_20ba(paVar14,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fd48,in_stack_0000fe6c,
                                in_stack_0000fe72,in_stack_0000fe76);
      paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)paVar19 >> 0x10);
      puVar5 = (u8 *)paVar19;
      uStack68 = ((u32)paVar19 >> 0x10);
      puStack70 = puVar5;
      pass1_1010_043a(paVar19,CONCAT22(uVar23,uVar22),iVar12);
    }
    pass1_1028_e1ec((u32)_PTR_LOOP_1050_65e2,0x4000002);
    puStack74 = (u32 *)CONCAT22((int)paVar14,puVar5);
    pass1_1028_780c(param_3,uVar21,CONCAT22((int)paVar14,puVar5));
  }
  puVar20 = mixed_1010_20ba(paVar14,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000fd4e,
                            in_stack_0000fe72,in_stack_0000fe78,in_stack_0000fe7c);
  uStack10 = SUB42(puVar20,0x0);
  uStack8 = ((u32)puVar20 >> 0x10);
  puStack12 = PTR_LOOP_1050_13ae;
  if (0x2 < (int)PTR_LOOP_1050_13ae) {
    puStack74 = mixed_1010_20ba((astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)puVar20 >> 0x10),_u16_1050_0ed0,
                                (u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fd4e,in_stack_0000fe72,
                                in_stack_0000fe78,in_stack_0000fe7c);
    for (puStack70 = (u8 *)0x1; (int)puStack70 < 0x9; puStack70 = (u8 *)((int)puStack70 + 0x1)) {
      local_42._0_4_ = (u8 **)(u32)((int)puStack74 + 0x34 + (int)puStack70 * 0x4);
      if (local_42._0_4_ == (u8 **)uStack6) {
        puVar5 = (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1);
        local_30._0_2_ = (u8 *)0x1;
        pass1_1008_612e(0x1,0x1,0x64);
        puVar7 = (u16 *)((int)puStack70 - 0x7);
        if (puVar7 == NULL) {
          bVar16 = SBORROW2((int)puVar5,0x32);
          puVar1 = puVar5 + -0x32;
          bVar15 = puVar5 == (u8 *)((int)s_New_failed_in_Op__Op_1050_0020 + 0x12);//
LAB_1028_7b74:
          if (!bVar15 && bVar16 == (int)puVar1 < 0x0) {
            local_30._0_2_ = NULL;
          }
        }
        else {
          puVar7 = (u16 *)((int)puStack70 - 0x8);
          if (puVar7 == NULL) {
            bVar16 = SBORROW2((int)puVar5,0x19);
            puVar1 = puVar5 + -0x19;
            bVar15 = puVar1 == NULL;
            goto LAB_1028_7b74;
          }
        }
        local_1e[0] = puVar5;
        if (local_30._0_2_ != NULL) {
          pass1_1028_90e6((astruct_97 *)CONCAT22(0x1050,&uStack340),puStack70);
          puVar7 = &uStack340;
          fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,puVar7));
          uStack340 = 0x389a;
          uStack338 = 0x1008;
        }
        pass1_1008_612e(puVar7,0x0,0xa);
        local_18 = local_18 & 0xffff0000 | ZEXT24(puVar7);
        if (puStack70 == (u8 *)0x7) {
          iVar24 = 0x7;
          puVar6 = puVar7 + 0x37;
          iVar12 = (int)puVar6 >> 0xf;
        }
        else {
          if (puStack70 != (u8 *)0x8) goto LAB_1028_7ba0;
          iVar24 = 0x8;
          puVar6 = puVar7 + 0x32;
          iVar12 = ((int)puVar7 >> 0xf) + ((u16 *)0xff9b < puVar7);
        }
        uVar21 = (int)((u32)local_42._0_4_ >> 0x10) + iVar12 + CARRY2(local_42._0_2_,puVar6);
        local_42._0_4_ = (u8 **)CONCAT22(uVar21,local_42._0_2_ + (int)puVar6);
        pass1_1010_ebf8((u32)puStack74,local_42._0_2_ + (int)puVar6,uVar21,iVar24);
      }//
LAB_1028_7ba0:
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_7c4e(param_1: *mut astruct_57,mut param_2: u32)

{
  code **ppcVar1;
  u8 *puVar2;
  astruct_92 *paVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  u8 uVar7;
  u32 *puVar8;
  astruct_97 *paVar9;
  let mut in_stack_0000fd32: u16;
  let mut in_stack_0000fe56: u16;
  let mut in_stack_0000fe5c: u16;
  let mut in_stack_0000fe60: u16;
  let mut in_stack_0000fe8a: u16;
  let mut uVar10: u16;
  let mut local_156: u16;
  let mut uStack340: u16;
  let mut uStack70: u16;
  let mut uStack68: u16;
  let mut iStack66: i16;
  astruct_15 *paStack64;
  let mut uStack56: u32;
  let mut uStack52: u16;
  let mut uStack50: u32;
  u32 *puStack46;
  let mut uStack42: u16;
  u8 *puStack40;
  astruct_691 *paStack38;
  astruct_92 local_22;
  let mut iStack10: i16;

  mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fe8a,0x2),in_stack_0000fd32,
                  in_stack_0000fe56,in_stack_0000fe5c,in_stack_0000fe60);
  puVar2 = PTR_LOOP_1050_13ae;
  if (0x2 < (int)PTR_LOOP_1050_13ae) {
    uVar6 = *_PTR_LOOP_1050_65e2;
    iStack10 = (int)(uVar6 >> 0x10);
    if ((0x2 < uVar6) && (uVar6 = CONCAT22(iStack10 - (uVar6 < 0x2),uVar6 - 0x2) % 0x14, uVar6 == 0x0)
       ) {
      pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_22)),0x1,0x0,0x400);
      while( true ) {
        uVar5 = uVar6;
        paVar3 = &local_22;
        pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar3));
        paStack38 = (astruct_691 *)CONCAT22(uVar5,paVar3);
        uVar6 = (u32)(uVar5 | paVar3);
        if ((uVar5 | paVar3) == 0x0) break;
        if (paVar3[0x1c].field4_0x8 != 0x8000002) {
          puVar8 = pass1_1008_c6fa(_u16_1050_06e0,0x2a);
          uVar6 = (u32)puVar8 >> 0x10;
          uVar5 = puVar8;
          puStack40 = (u8 *)((u32)puVar8 >> 0x10);
          uVar7 = 0x38;
          uStack42 = uVar5;
          pass1_1038_4d6e(uVar5,puStack40,paStack38,puVar8);
          puStack46 = (u32 *)CONCAT22((int)uVar6,uVar5);
          ppcVar1 = (code **)((int)*puStack46 + 0x10);
          (**ppcVar1)((int)&u16_1050_1038,uVar5,(int)uVar6);
          uStack50 = CONCAT22((int)uVar6,uVar5);
          if (puVar2 == (u8 *)((int)&u16_1050_0002 + 0x1)) {
            uStack52 = 0x6;
          }
          else {
            uStack52 = 0xc;
          }
          for (uStack56 = 0x0; uStack56 < uStack50; uStack56 += 0x1) {
            paStack64 = (astruct_15 *)pass1_1030_1d7c((int)uStack50,(int)uVar6,(u32)puStack46);
            uVar6 = (u32)paStack64 >> 0x10;
            iVar4 = (int)paStack64;
            pass1_1028_7742(param_2,(param_2 >> 0x10),0x4,paStack64);
            uVar5 = uStack52;
            if (iVar4 == 0x0) {
              uVar5 = 0x19;
            }
            uVar7 = 0x8;
            uStack68 = uVar5;
            iStack66 = iVar4;
            pass1_1008_612e(uVar5,0x1,0x64);
            uStack70 = uVar5;
            if ((int)uVar5 <= (int)uStack68) {
              paVar9 = pass1_1028_8fc0((astruct_97 *)CONCAT13(0x10,CONCAT12(0x50,&local_156)),
                                       (u32)((int)paStack64 + 0x4),(u32)((int)paStack38 + 0x4));
              uVar6 = (u32)paVar9 >> 0x10;
              uVar7 = 0x30;
              fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_156));
              local_156 = 0x389a;
              uStack340 = 0x1008;
            }
          }
          uVar10 = ((u32)puStack46 >> 0x10);
          if (puStack46 != NULL) {
            ppcVar1 = (code **)*puStack46;
            (**ppcVar1)(uVar7,(int)puStack46,uVar10,0x1,(int)puStack46,uVar10,puStack46);
          }
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_7dfc(undefined1 param_1,u8 *param_2,mut param_3: u32)

{
  code **ppcVar1;
  u8 *puVar2;
  astruct_92 *paVar3;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut uVar5: u32;
  astruct_57 *paVar6;
  u8 uVar7;
  u32 *puVar8;
  u32 *puVar9;
  astruct_97 *paVar10;
  let mut in_stack_0000fd30: u16;
  let mut in_stack_0000fe54: u16;
  let mut in_stack_0000fe5a: u16;
  let mut in_stack_0000fe5e: u16;
  let mut in_stack_0000fe88: u16;
  let mut uVar11: u16;
  let mut local_158: u16;
  let mut uStack342: u16;
  let mut uStack72: u16;
  let mut uStack70: u16;
  astruct_15 *paStack68;
  let mut uStack60: u32;
  let mut uStack56: u16;
  let mut uStack54: u16;
  let mut iStack52: i16;
  let mut uStack50: u32;
  u32 *puStack46;
  let mut uStack42: u16;
  u8 *puStack40;
  astruct_691 *paStack38;
  astruct_92 local_22;

  mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_2),_u16_1050_0ed0,
                  (u8 **)CONCAT22(in_stack_0000fe88,0x2),in_stack_0000fd30,in_stack_0000fe54,in_stack_0000fe5a,
                  in_stack_0000fe5e);
  puVar2 = PTR_LOOP_1050_13ae;
  if (((0x2 < (int)PTR_LOOP_1050_13ae) && (0x3 < *_PTR_LOOP_1050_65e2)) &&
     (uVar5 = *_PTR_LOOP_1050_65e2 % 0x14, uVar5 == 0x0)) {
    pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_22)),0x1,0x0,0x400);
    while( true ) {
      paVar3 = &local_22;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar3));
      uVar4 = uVar5;
      paStack38 = (astruct_691 *)CONCAT22(uVar4,paVar3);
      uVar5 = uVar5 & 0xffff0000 | (u32)(uVar4 | paVar3);
      if ((uVar4 | paVar3) == 0x0) break;
      if (paVar3[0x1c].field4_0x8 != 0x8000002) {
        puVar8 = pass1_1008_c6fa(_u16_1050_06e0,0x29);
        paVar6 = (astruct_57 *)(uVar5 & 0xffff0000 | (u32)puVar8 >> 0x10);
        uVar4 = puVar8;
        puStack40 = (u8 *)((u32)puVar8 >> 0x10);
        uStack42 = uVar4;
        pass1_1038_4d6e(uVar4,puStack40,paStack38,puVar8);
        puStack46 = (u32 *)CONCAT22((int)paVar6,uVar4);
        ppcVar1 = (code **)((int)*puStack46 + 0x10);
        (**ppcVar1)((int)&u16_1050_1038,uVar4,(int)paVar6);
        uStack50 = CONCAT22((int)paVar6,uVar4);
        uVar7 = 0x10;
        puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fe88,0x2),in_stack_0000fd30,
                                 in_stack_0000fe54,in_stack_0000fe5a,in_stack_0000fe5e);
        uVar5 = (u32)paVar6 & 0xffff0000 | (u32)puVar9 >> 0x10;
        uStack56 = SUB42(puVar9,0x0);
        uStack54 = ((u32)puVar9 >> 0x10);
        if (puVar2 == (u8 *)((int)&u16_1050_0002 + 0x1)) {
          iStack52 = 0x5;
        }
        else {
          iStack52 = 0x1e;
        }
        for (uStack60 = 0x0; uStack60 < uStack50; uStack60 += 0x1) {
          paStack68 = (astruct_15 *)pass1_1030_1d7c((int)uStack50,(int)uVar5,(u32)puStack46);
          uVar5 = uVar5 & 0xffff0000 | (u32)paStack68 >> 0x10;
          uVar4 = paStack68;
          uVar7 = 0x8;
          pass1_1008_612e(uVar4,0x1,0x64);
          uStack70 = uVar4;
          if (((int)uVar4 <= iStack52) &&
             (pass1_1028_7742(param_3,(param_3 >> 0x10),0x4,paStack68), uStack72 = uVar4, uVar4 == 0x0)) {
            paVar10 = pass1_1028_b0de((astruct_97 *)CONCAT13(0x10,CONCAT12(0x50,&local_158)),
                                      (u32)((int)paStack68 + 0x4),(u32)((int)paStack38 + 0x4));
            uVar5 = uVar5 & 0xffff0000 | (u32)paVar10 >> 0x10;
            uVar7 = 0x30;
            fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_158));
            local_158 = 0x389a;
            uStack342 = 0x1008;
          }
        }
        in_stack_0000fe88 = SUB42(puStack46,0x0);
        uVar11 = ((u32)puStack46 >> 0x10);
        if (puStack46 != NULL) {
          ppcVar1 = (code **)*puStack46;
          (**ppcVar1)(uVar7,in_stack_0000fe88,uVar11,0x1,in_stack_0000fe88,uVar11,puStack46);
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_7fb6(uchar param_1,mut param_2: u32)

{
  code **ppcVar1;
  astruct_92 *paVar2;
  let mut uVar3: u16;
  let mut uVar4: u32;
  astruct_57 *paVar5;
  u8 uVar6;
  u32 *puVar7;
  u32 *puVar8;
  astruct_97 *paVar9;
  let mut in_stack_0000fd30: u16;
  let mut in_stack_0000fe54: u16;
  let mut in_stack_0000fe5a: u16;
  let mut in_stack_0000fe5e: u16;
  let mut in_stack_0000fe88: u16;
  let mut uVar10: u16;
  let mut local_158: u16;
  let mut uStack342: u16;
  let mut uStack72: u16;
  let mut uStack68: u16;
  let mut uStack66: u16;
  astruct_15 *paStack64;
  let mut uStack56: u32;
  let mut iStack52: i16;
  u8 *puStack50;
  let mut uStack48: u16;
  let mut uStack46: u16;
  let mut uStack44: u32;
  u32 *puStack40;
  let mut uStack36: u16;
  u8 *puStack34;
  astruct_691 *paStack32;
  astruct_92 local_1c;

  if ((0xb < *_PTR_LOOP_1050_65e2) && (uVar4 = *_PTR_LOOP_1050_65e2 % 0x32, uVar4 == 0x0)) {
    pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_1c)),0x1,0x0,0x400);
    while( true ) {
      paVar2 = &local_1c;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar2));
      uVar3 = uVar4;
      paStack32 = (astruct_691 *)CONCAT22(uVar3,paVar2);
      uVar4 = uVar4 & 0xffff0000 | (u32)(uVar3 | paVar2);
      if ((uVar3 | paVar2) == 0x0) break;
      if (paVar2[0x1c].field4_0x8 != 0x8000002) {
        puVar7 = pass1_1008_c6fa(_u16_1050_06e0,0x11);
        paVar5 = (astruct_57 *)(uVar4 & 0xffff0000 | (u32)puVar7 >> 0x10);
        uVar3 = puVar7;
        puStack34 = (u8 *)((u32)puVar7 >> 0x10);
        uStack36 = uVar3;
        pass1_1038_4d6e(uVar3,puStack34,paStack32,puVar7);
        puStack40 = (u32 *)CONCAT22((int)paVar5,uVar3);
        ppcVar1 = (code **)((int)*puStack40 + 0x10);
        (**ppcVar1)((int)&u16_1050_1038,uVar3,(int)paVar5);
        uStack44 = CONCAT22((int)paVar5,uVar3);
        uVar6 = 0x10;
        puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fe88,0x2),in_stack_0000fd30,
                                 in_stack_0000fe54,in_stack_0000fe5a,in_stack_0000fe5e);
        uVar4 = (u32)paVar5 & 0xffff0000 | (u32)puVar8 >> 0x10;
        uStack48 = SUB42(puVar8,0x0);
        uStack46 = ((u32)puVar8 >> 0x10);
        puStack50 = PTR_LOOP_1050_13ae;
        if ((int)PTR_LOOP_1050_13ae < 0x3) {
          iStack52 = 0x5;
        }
        else {
          iStack52 = 0x14;
        }
        for (uStack56 = 0x0; uStack56 < uStack44; uStack56 += 0x1) {
          uVar6 = 0x30;
          paStack64 = (astruct_15 *)pass1_1030_1d7c((int)uStack44,(int)uVar4,(u32)puStack40);
          uVar4 = uVar4 & 0xffff0000 | (u32)paStack64 >> 0x10;
          uVar3 = ((int)paStack64 + 0x20);
          uStack66 = uVar3;
          if (((uVar3 != 0x0) && (uVar3 != 0x70)) && (uVar3 != 0x71)) {
            uVar6 = 0x8;
            pass1_1008_612e(uVar3,0x1,0x64);
            uStack68 = uVar3;
            if (((int)uVar3 <= iStack52) &&
               (pass1_1028_7742(param_2,(param_2 >> 0x10),0x4,paStack64), uStack72 = uVar3, uVar3 == 0x0)) {
              paVar9 = pass1_1028_8698((astruct_97 *)CONCAT13(0x10,CONCAT12(0x50,&local_158)),
                                       (u32)((int)paStack64 + 0x4),(u32)((int)paStack32 + 0x4));
              uVar4 = uVar4 & 0xffff0000 | (u32)paVar9 >> 0x10;
              uVar6 = 0x30;
              fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_158));
              local_158 = 0x389a;
              uStack342 = 0x1008;
            }
          }
        }
        in_stack_0000fe88 = SUB42(puStack40,0x0);
        uVar10 = ((u32)puStack40 >> 0x10);
        if (puStack40 != NULL) {
          ppcVar1 = (code **)*puStack40;
          (**ppcVar1)(uVar6,in_stack_0000fe88,uVar10,0x1,in_stack_0000fe88,uVar10,puStack40);
        }
      }
    }
  }
  return;
}



StructD * pass1_1028_816e(StructD *param_1,u8 param_2)

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



StructD * pass1_1028_8342(StructD *param_1,u8 param_2)

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



StructD * pass1_1028_848e(StructD *param_1,u8 param_2)

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



StructD * pass1_1028_865c(StructD *param_1,u8 param_2)

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



StructD * pass1_1028_87b4(StructD *param_1,u8 param_2)

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
  BOOL16 BVar5;
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
      puVar7 = (u8 *)((u32)puStack46 >> 0x10);
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



StructD * pass1_1028_8d62(StructD *param_1,u8 param_2)

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



StructD * pass1_1028_8f8a(StructD *param_1,u8 param_2)

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



StructD * pass1_1028_90aa(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_90e6(param_1: *mut astruct_97,mut param_2: u16 )

{
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x1387);
  uVar1 = ((u32)param_1 >> 0x10);
  ((int)param_1 + 0x108) = param_2;
  param_1->offset_0x0 = 0x932c;
  ((int)param_1 + 0x2) = 0x1028;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_9114(StructD *param_1,mut param_2: u32,mut param_3: u16 )

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  astruct_67 *paVar4;
  u32 *puVar5;
  let mut in_stack_0000fe8c: u16;
  let mut in_stack_0000fe92: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc0: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut iVar8: i16;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut uStack10: u16;

  paVar4 = (astruct_67 *)
           mixed_1010_20ba((astruct_57 *)param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x37),in_stack_0000fe92,
                           in_stack_0000ffb6,in_stack_0000ffbc,in_stack_0000ffc0);
  uVar2 = ((u32)param_1 >> 0x10);
  uVar3 = param_2;
  iVar1 = (uVar3 + 0x108);
  if (iVar1 - 0x1U < 0x8) {
    uStack10 = *_PTR_LOOP_1050_65e2;
    iVar8 = (int)((u32)*_PTR_LOOP_1050_65e2 >> 0x10);
    switch(iVar1) {
    case 0x1:
      iVar1 = 0x16;
      break;
    case 0x2:
      iVar1 = 0x17;
      break;
    case 0x3:
      iVar1 = 0x18;
      break;
    case 0x4:
      iVar1 = 0x1b;
      break;
    case 0x5:
      iVar1 = 0x1f;
      break;
    case 0x6:
      iVar1 = 0x24;
      break;
    case 0x7:
      pass1_1008_612e(uVar3,0x0,0x14);
      iVar1 = ((int)uVar3 >> 0xf) + (0xff91 < uVar3);
      uVar6 = uStack10 + uVar3 + 0x6e;
      uVar7 = iVar8 + iVar1 + CARRY2(uStack10,uVar3 + 0x6e);
      iVar8 = 0x7;
      puVar5 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar2,iVar1),_u16_1050_0ed0,(u8 **)CONCAT22(uVar6,0x2f),
                               in_stack_0000fe8c,in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
      uVar2 = ((u32)puVar5 >> 0x10);
      uVar3 = puVar5;
      pass1_1010_ebf8((u32)puVar5,uVar6,uVar7,iVar8);
      pass1_1008_612e(uVar3,0x1,0x64);
      if (0x32 < (int)uVar3) {
        return;
      }
      pass1_1028_e1ec((u32)_PTR_LOOP_1050_65e2,0x4000001);
      pass1_1038_4900(CONCAT22(uVar2,uVar3));
      iVar1 = 0x2c;
      break;
    case 0x8:
      pass1_1008_612e(uVar3,0x0,0x14);
      iVar1 = ((int)uVar3 >> 0xf) + (0xff9b < uVar3);
      uVar6 = uStack10 + uVar3 + 0x64;
      uVar7 = iVar8 + iVar1 + CARRY2(uStack10,uVar3 + 0x64);
      iVar9 = 0x8;
      puVar5 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar2,iVar1),_u16_1050_0ed0,(u8 **)CONCAT22(uVar6,0x2f),
                               in_stack_0000fe8c,in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
      iVar1 = (int)((u32)puVar5 >> 0x10);
      iVar8 = (int)puVar5;
      pass1_1010_ebf8((u32)puVar5,uVar6,uVar7,iVar9);
      if (0x19 < (int)uVar3) {
        return;
      }
      uVar3 = 0x1;
      uVar10 = 0x2;
      pass1_1028_e1ec((u32)_PTR_LOOP_1050_65e2,0x4000001);
      pass1_1038_43cc(iVar8,iVar1,iVar8,iVar1,uVar3,uVar10);
      iVar1 = 0x2d;
    }
    post_win_msg_1008_a0e4(paVar4,0x0,0x0,0x1,0x0,iVar1);
  }
  return;
}
pub fn pass1_1028_9264(mut param_1: u16 ,u8 *param_2,mut param_3: u32)

{
  u32 *puVar1;
  u32 *puVar2;
  u32 *puVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar6;
  let mut iVar7: i16;
  u32 *puVar8;
  let mut uVar9: u16;
  let mut puStack10: *mut u16;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x10a,paVar6);
  uVar5 = paVar6;
  puStack10 = (u16 *)CONCAT22(uVar5,param_1);
  if ((uVar5 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    uVar9 = (param_3 >> 0x10);
    iVar7 = (int)param_3;
    (u32)(param_1 + 0x4) = (u32)(iVar7 + 0x4);
    puVar3 = (u32 *)(iVar7 + 0x8);
    puVar8 = (u32 *)(param_1 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar8;
      puVar8 = puVar8 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_1 + 0x2) = 0x1028;
    (param_1 + 0x108) = (iVar7 + 0x108);
    *puStack10 = 0x932c;
    (param_1 + 0x2) = 0x1028;
  }
  return;
}



StructD * pass1_1028_9300(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn struct_op_1028_933c(param_1: *mut astruct_97,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,u32 *param_5,mut param_6: u16 ,
                        mut param_7: u32,mut param_8: u32)

{
  astruct_97 *iVar1;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x3e8);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  (u32)&iVar1->field259_0x108 = param_8;
  (u32)&iVar1->field262_0x10c = param_7;
  iVar1->field264_0x110 = 0x0;
  iVar1->field265_0x114 = *param_5;
  iVar1->field266_0x118 = (param_5 + 0x1);
  iVar1->field267_0x11a = param_4;
  iVar1->field268_0x11c = param_2;
  iVar1->field270_0x120 = 0x0;
  iVar1->field269_0x11e = 0x0;
  iVar1->field271_0x122 = param_3;
  param_1->offset_0x0 = 0x9934;
  iVar1->segment_0x2 = 0x1028;
  sys_1000_3f9c((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),
                s_SCPutBldg_site_0x_08lx__bldg__u__1050_50ce,param_8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_93d4(mut param_1: u16 ,mut param_2: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  u8 *puVar4;
  let mut in_register_0000000a: u16;
  let mut iVar5: i16;
  let mut uVar6: u16;
  u8 local_112 [0x10c];
  let mut uStack6: u32;

  PTR_LOOP_1050_50ca = NULL;
  PTR_LOOP_1050_50cc = NULL;
  uVar6 = (param_2 >> 0x10);
  iVar5 = (int)param_2;
  uStack6 = pass1_1028_e2e0((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_PTR_LOOP_1050_65e2,0x7);
  puVar4 = (u8 *)(uStack6 >> 0x10);
  uVar2 = uStack6;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack6 & 0xffff | ZEXT24(puVar4) << 0x10);
  (iVar5 + 0x11e) = uVar2;
  *(u8 **)(iVar5 + 0x120) = puVar4;
  uVar3 = iVar5 + 0x114;
  ppcVar1 = (code **)((int)(u32)(u32)(iVar5 + 0x11e) + 0x1c);
  (**ppcVar1)();
  if (uVar3 != 0x0) {
    pass1_1028_9624(uVar3,puVar4,(astruct_688 *)param_2);
    ppcVar1 = (code **)((int)(u32)(u32)(iVar5 + 0x11e) + 0x20);
    (**ppcVar1)();
    ppcVar1 = (code **)((int)(u32)(u32)(iVar5 + 0x11e) + 0x18);
    (**ppcVar1)();
    pass1_1028_9600(puVar4,param_2);
    return;
  }
  (u32)(iVar5 + 0x11e) = 0x0;
  struct_1030_e4fa((astruct_97 *)CONCAT22(0x1050,local_112),uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_112));
  if (PTR_LOOP_1050_50ca == NULL) {
    PTR_LOOP_1050_50ca = (u8 *)0x6ad;
  }
  return;
}
pub fn pass1_1028_94e4(param_1: *mut astruct_328,u8 *param_2,param_3: *mut astruct_329)

{
  u32 *puVar1;
  u32 *puVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  astruct_329 *iVar5;
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
    iVar5 = (astruct_329 *)param_3;
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
    param_1->field265_0x122 = iVar5->field266_0x122;
    *puStack10 = 0x9934;
    param_1->field2_0x2 = 0x1028;
  }
  return;
}
pub fn pass1_1028_9600(u8 *param_1,mut param_2: u32)

{
  let mut puVar1: *mut u16;
  u8 local_6 [0x4];

  puVar1 = pass1_1020_a43e(param_1,(u16 *)CONCAT22(0x1050,local_6));
  pass1_1020_a80e(local_6,((u32)puVar1 >> 0x10),local_6,&DAT_1050_1050,
                  ((int)param_2 + 0x11a));
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_9624(mut param_1: u16 ,u8 *param_2,param_3: *mut astruct_688)

{
  code **ppcVar1;
  u32 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  BOOL16 BVar5;
  let mut uVar7: u32;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar8;
  astruct_688 *iVar9;
  let mut unaff_SI: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut in_stack_0000fd54: u16;
  let mut in_stack_0000fe78: u16;
  let mut in_stack_0000fe7e: u16;
  let mut in_stack_0000fe82: u16;
  u8 *puVar11;
  u8 *puVar12;
  let mut uStack332: u16;
  let mut uStack330: u16;
  let mut uStack64: u16;
  let mut uStack62: u32;
  let mut iStack58: i16;
  let mut uStack56: u32;
  u32 *puStack46;
  let mut uStack42: u32;
  u8 local_26 [0x4];
  let mut uStack34: u16;
  u8 *puStack32;
  let mut uStack30: u32;
  let mut uStack26: u32;
  u32 *puStack22;
  u8 local_12 [0x2];
  u8 local_10 [0x2];
  u8 local_e [0x2];
  let mut uStack12: u16;
  let mut uStack10: u32;
  u32 *puStack6;
  u32 *puVar6;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar9 = ((u32)param_3 >> 0x10);
  iVar9 = (astruct_688 *)param_3;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,iVar9->field265_0x10c);
  &iVar9->field266_0x110 = param_1;
  ((int)&iVar9->field266_0x110 + 0x2) = (int)paVar8;
  puStack6 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fd54,
                             in_stack_0000fe78,in_stack_0000fe7e,in_stack_0000fe82);
  uStack10 = (u8 *)((u32)puStack6 >> 0x10);
  puVar2 = &iVar9->field267_0x114;
  pass1_1030_64ce(puVar2,uStack10,_PTR_LOOP_1050_5740,(u16 *)((u32)param_3 & 0xffff0000 | ZEXT24(puVar2)),
                  iVar9->field264_0x108,(u32 *)CONCAT22(0x1050,local_26));
  uStack56 = (u32 *)*puVar2;
  uStack56._3_1_ = (char)((u32)uStack56 >> 0x18);
  uStack12 = (uStack56._3_1_ != '\0');
  uVar10 = 0x1008;
  puStack46 = uStack56;
  uStack10 = uStack56;
  pass1_1008_3eb4((astruct_615 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9->field267_0x114)),
                  (u16 *)CONCAT22(0x1050,local_12),(u16 *)CONCAT22(0x1050,local_10),
                  (u16 *)CONCAT22(0x1050,local_e));
  if (uStack12 == 0x0) {
    puVar2 = &iVar9->field267_0x114;
    pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x500);
    puStack22 = (u32 *)CONCAT22(uStack10,puVar2);
    uVar10 = 0x1030;
    pass1_1030_61fe(puVar2,uStack10,_PTR_LOOP_1050_5740,CONCAT22(uStack10,puVar2),
                    (u32)param_3 & 0xffff0000 | ZEXT24(&iVar9->field267_0x114),iVar9->field264_0x108);
    if ((iVar9->field270_0x11a == 0xa) || (iVar9->field270_0x11a == 0x37)) {
      if (iVar9->field270_0x11a == 0x37) {
        uStack56 = iVar9->field273_0x11e;
        uStack10 = *(u8 **)((int)&iVar9->field273_0x11e + 0x2);
        uStack42 = iVar9->field265_0x10c;
        (u32)((int)uStack56 + 0x20) = uStack42;
      }
      puVar2 = &iVar9->field267_0x114;
      pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x400);
      (u32*)&iVar9->field265_0x10c = puVar2;
      *(u8 **)((int)&iVar9->field265_0x10c + 0x2) = uStack10;
      uVar10 = 0x1018;
      pass1_1018_0196(puVar2,uStack10,(u32)puStack6,
                      CONCAT22(uStack10,&iVar9->field265_0x10c),iVar9->field264_0x108);
      if (iVar9->field270_0x11a == 0xa) {
        uVar10 = 0x1010;
        pass1_1010_ed22((u32)puStack6,iVar9->field265_0x10c);
      }
    }
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,iVar9->field265_0x10c);
    (u32*)&iVar9->field266_0x110 = puVar2;
    *(u8 **)((int)&iVar9->field266_0x110 + 0x2) = uStack10;
    uVar4 = uStack10 | &iVar9->field266_0x110;
    puVar6 = (u32 *)(u32)uVar4;
    if (uVar4 == 0x0) goto LAB_1028_9807;
    uVar3 = SUB42(puStack22,0x0);
    puVar12 = (u8 *)((u32)puStack22 >> 0x10);
    puVar11 = uStack10;
  }
  else {
    puStack22 = uStack10;
    puVar6 = uStack10;
    if (iVar9->field270_0x11a != 0x75) goto LAB_1028_9807;
    uVar3 = SUB42(uStack10,0x0);
    puVar12 = uStack10;
    puVar11 = *(u8 **)((int)&iVar9->field266_0x110 + 0x2);
  }
  ppcVar1 = (code **)((int)*iVar9->field266_0x110 + 0x8);
  (**ppcVar1)(uVar10,&iVar9->field266_0x110,puVar11,0x0,uVar3,puVar12,0x0);//
LAB_1028_9807:
  uVar10 = SUB42(puVar6,0x0);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)puStack22);
  uStack26 = (astruct_358 *)CONCAT22(uStack10,uVar10);
  pass1_1030_73ee(uStack10,(astruct_294 *)CONCAT22(uStack10,uVar10),iVar9->field265_0x10c);
  BVar5 = pass1_1008_c6ae(_u16_1050_06e0,iVar9->field270_0x11a,0x31);
  if ((BVar5 == 0x0) && (iVar9->field274_0x122 == 0x0)) {
    uStack62 = (u32)((int)uStack26 + 0xc);
    iStack58 = ((int)uStack26 + 0x10);
    uStack56 = (u32 *)((u32)uStack56 & 0xffff0000 | ZEXT24(&uStack62));
    if (iStack58 < 0x1) {
      uStack64 = 0x5;
    }
    else {
      uStack64 = 0x6;
    }
    ((int)uStack26 + 0x14) = uStack64;
    uStack10 = uStack26;
  }
  uVar7 = (u32)((int)uStack26 + 0x16);
  uStack30 = uVar7;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7);
  uStack34 = uVar7;
  puStack32 = uStack10;
  if (uStack30 != 0x0) {
    struct_1030_e4fa((astruct_97 *)CONCAT22(0x1050,&uStack332),uStack30);
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&uStack332));
    uStack332 = 0x389a;
    uStack330 = 0x1008;
  }
  ppcVar1 = (code **)((int)*iVar9->field273_0x11e + 0x4);
  (**ppcVar1)();
  puVar6 = iVar9->field273_0x11e;
  pass1_1030_7e5a(uStack10,uStack26,(u32)((int)puVar6 + 0x4));
  return;
}



StructD * pass1_1028_9908(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1028_9944(param_1: *mut astruct_97,mut param_2: u32,mut param_3: u32,mut param_4: u32)

{
  astruct_97 *iVar1;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x1387);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  (u32)&iVar1->field259_0x108 = param_4;
  (u32)&iVar1->field262_0x10c = param_3;
  iVar1->field264_0x110 = param_2;
  iVar1->field265_0x114 = 0x0;
  param_1->offset_0x0 = 0x9c52;
  iVar1->segment_0x2 = 0x1028;
  return;
}
pub fn pass1_1028_9992(param_1: *mut u16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = 0x9c52;
  (iVar1 + 0x2) = 0x1028;
  fn_ptr_1000_17ce(*(char **)(iVar1 + 0x114));
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_99c4(mut param_1: i16,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_3 >> 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)((int)param_3 + 0x10c));
  pass1_1030_355c((u32)(param_1 + 0x1f6),(u32)((int)param_3 + 0x114));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_9a02(mut param_1: u32,mut param_2: i16,StructD *param_3,mut param_4: u16 )

{
  i32 lVar1;
  let mut bVar2: bool;
  let mut uVar3: u16;
  astruct_92 *paVar4;
  let mut uVar5: u32;
  u8 *puVar6;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut iVar10: i16;
  let mut uVar11: u16;
  astruct_27 *paVar12;
  astruct_67 *paVar13;
  let mut in_stack_0000fe62: u16;
  let mut in_stack_0000fe70: u16;
  let mut in_stack_0000ff86: u16;
  let mut in_stack_0000ff8c: u16;
  let mut in_stack_0000ff90: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ff9e: u16;
  u8 uVar14;
  u8 uVar15;
  let mut uVar16: u16;
  let mut uVar17: u16;
  let mut iVar18: i16;
  astruct_92 local_30;
  astruct_57 *paVar9;

  uVar11 = (param_1 >> 0x10);
  iVar10 = (int)param_1;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(iVar10 + 0x108));
  puVar6 = (u8 *)param_3;
  uVar5 = (u32)(param_2 + 0x1f6);
  pass1_1030_3694(puVar6,uVar5,0x0,*(i32 *)(iVar10 + 0x110));
  uVar7 = uVar5;
  (iVar10 + 0x114) = uVar7;
  (iVar10 + 0x116) = (int)param_3;
  pass1_1030_38b8();
  uVar7 = param_3 | uVar7;
  paVar9 = (astruct_57 *)((u32)param_3 & 0xffff0000 | (u32)uVar7);
  if (uVar7 == 0x0) {
    lVar1 = *(i32 *)(param_2 + 0x200);
    paVar12 = (astruct_27 *)
              mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(param_4,0x2b),in_stack_0000fe70,
                              in_stack_0000ff94,in_stack_0000ff9a,in_stack_0000ff9e);
    uVar11 = ((u32)paVar9 >> 0x10);
    if (lVar1 == 0x8000002) {
      iVar10 = 0x1f;
    }
    else {
      iVar10 = 0xb;
    }
    pass1_1010_043a(paVar12,*(i32 *)(param_2 + 0x4),iVar10);
    if (lVar1 == 0x8000001) {
      uVar3 = 0x2;
    }
    else {
      uVar3 = 0x1;
    }
    paVar9 = (astruct_57 *)CONCAT22(uVar11,0x800);
    pass1_1038_349e((astruct_685 *)CONCAT22(puVar6,param_2),CONCAT22(0x800,uVar3));
    bVar2 = false;
    pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_30)),0x1,0x0,0x400);
    while( true ) {
      paVar4 = &local_30;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar4));
      uVar7 = paVar9;
      uVar8 = uVar7 | paVar4;
      paVar9 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | (u32)uVar8);
      if (uVar8 == 0x0) break;
      if (paVar4[0x1c].field4_0x8 != 0x8000002) {
        bVar2 = true;
      }
    }
    if (!bVar2) {
      uVar17 = 0x0;
      iVar18 = 0x3c;
      uVar14 = 0x1;
      uVar15 = 0x0;
      uVar16 = 0x0;
      uVar3 = 0x0;
      iVar10 = 0x0;
      uVar11 = 0x0;
      paVar13 = (astruct_67 *)
                mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)0x37,in_stack_0000fe62,in_stack_0000ff86,
                                in_stack_0000ff8c,in_stack_0000ff90);
      post_win_msg_1008_a0e4
                (paVar13,CONCAT22(uVar3,uVar11),iVar10,CONCAT11(uVar15,uVar14),CONCAT22(uVar17,uVar16),iVar18);
    }
  }
  return;
}
pub fn pass1_1028_9b48(param_1: *mut astruct_330,u8 *param_2,param_3: *mut astruct_331)

{
  u32 *puVar1;
  u32 *puVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  astruct_331 *iVar5;
  u32 *puVar6;
  u32 *puVar7;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x118,paVar5);
  uVar4 = paVar5;
  puStack10 = (u16 *)CONCAT22(uVar4,param_1);
  iVar5 = (astruct_331 *)param_3;
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
    *puStack10 = 0x9c52;
    param_1->field2_0x2 = 0x1028;
  }
  iVar5->field261_0x114 = 0x0;
  return;
}



StructD * pass1_1028_9c2c(StructD *param_1,u8 param_2)

{
  pass1_1028_9992(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * struct_1028_9c62(param_1: *mut astruct_97,mut param_2: u16 )

{
  struct_op_1028_d1dc(param_1,param_2);
  ((int)param_1 + 0x108) = param_2;
  param_1->offset_0x0 = 0x9eb6;
  ((int)param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 pass1_1028_9c90(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;

  uVar1 = ((int)param_1 + 0x108) - 0x3e8;
  if ((uVar1 < 0x3a99) && (uVar1 % 0x3e8 == 0x0)) {
    // WARNING: Could not recover jumptable at 0x10289dc0. Too many branches
    // WARNING: Treating indirect jump as call
    uVar2 = (code)((uVar1 / 0x3e8) * 0x2 + -0x623a))();
    return uVar2;
  }
  return 0x1;
}
pub fn pass1_1028_9dee(param_1: *mut astruct_332,mut param_2: u16 ,param_3: *mut astruct_333)

{
  u32 *puVar1;
  u32 *puVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  astruct_333 *iVar5;
  u32 *puVar6;
  u32 *puVar7;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x10a,paVar5);
  uVar4 = paVar5;
  puStack10 = (u16 *)CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    param_1->field2_0x2 = 0x1008;
    uVar8 = ((u32)param_3 >> 0x10);
    iVar5 = (astruct_333 *)param_3;
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
    *puStack10 = 0x9eb6;
    param_1->field2_0x2 = 0x1028;
  }
  return;
}



StructD * pass1_1028_9e8a(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_9ec6(param_1: *mut astruct_97)

{
  struct_op_1028_d1dc(param_1,(int)s_noth_bmp_1050_2321 + 0x6);
  param_1->offset_0x0 = 0xa6f6;
  ((int)param_1 + 0x2) = 0x1028;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x8)),(char *)0x105050f0);
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_9efc(param_1: *mut u16,StructD *param_2,mut param_3: u32,mut param_4: u16 )

{
  i32 lVar1;
  let mut iVar2: i16;
  astruct_92 *paVar3;
  let mut uVar4: u16;
  let mut iVar5: i16;
  let mut iVar6: i16;
  i32 lVar7;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  astruct_57 *paVar12;
  astruct_67 *paVar13;
  astruct_690 *paVar14;
  astruct_27 *paVar15;
  let mut in_stack_0000fe6e: u16;
  let mut in_stack_0000ff92: u16;
  let mut in_stack_0000ff98: u16;
  let mut in_stack_0000ff9c: u16;
  let mut uVar16: u16;
  astruct_92 local_18;
  astruct_57 *paVar11;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
  uVar8 = param_2 | param_1;
  paVar11 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)uVar8);
  if (uVar8 != 0x0) {
    pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_18)),0x1,0x0,0x400);
    while( true ) {
      paVar3 = &local_18;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar3));
      uVar8 = paVar11;
      paVar12 = (astruct_57 *)((u32)paVar11 & 0xffff0000 | (u32)(uVar8 | paVar3));
      if ((uVar8 | paVar3) == 0x0) break;
      lVar1 = paVar3[0x1c].field4_0x8;
      uVar4 = ((int)&paVar3[0x1c].field4_0x8 + 0x2);
      paVar11 = (astruct_57 *)((u32)paVar11 & 0xffff0000 | (u32)uVar4);
      if (((int)&paVar3[0x1c].field3_0x4 + 0x2) != 0x0) {
        uVar16 = (param_3 >> 0x10);
        lVar7 = lVar1;
        if (((int)lVar1 != 0x2) || (uVar4 != 0x800)) {
          pass1_1028_a3ae((int)lVar1,(long)paVar11,param_3,uVar16,CONCAT22(uVar8,paVar3));
        }
        uVar4 = lVar7;
        pass1_1028_a28a(param_3,uVar16,(astruct_691 *)CONCAT22(uVar8,paVar3));
        if (((int)paVar11 < 0x1) && (((int)paVar11 < 0x0 || (uVar4 < 0x64)))) {
          pass1_1028_a4ee(param_3,CONCAT22(uVar8,paVar3));
        }
        if (lVar1 != 0x8000002) {
          pass1_1038_42cc(CONCAT22(uVar8,paVar3));
          uVar9 = paVar11 | uVar4;
          paVar11 = (astruct_57 *)((u32)paVar11 & 0xffff0000 | (u32)uVar9);
          if (uVar9 != 0x0) {
            paVar13 = (astruct_67 *)
                      mixed_1010_20ba(paVar11,_u16_1050_0ed0,(u8 **)CONCAT22(param_4,0x37),in_stack_0000fe6e,
                                      in_stack_0000ff92,in_stack_0000ff98,in_stack_0000ff9c);
            paVar11 = (astruct_57 *)((u32)paVar11 & 0xffff0000 | (u32)paVar13 >> 0x10);
            post_win_msg_1008_a0e4(paVar13,0x0,uVar4,paVar3[0x1c].field6_0x10,paVar3->field3_0x4,0x2);
          }
        }
      }
    }
    local_18._0_2_ = 0x389a;
    local_18.field2_0x2 = 0x1008;
    paVar14 = (astruct_690 *)
              mixed_1010_20ba(paVar12,_u16_1050_0ed0,(u8 **)CONCAT22(param_4,0x8),in_stack_0000fe6e,
                              in_stack_0000ff92,in_stack_0000ff98,in_stack_0000ff9c);
    paVar11 = (astruct_57 *)((u32)paVar12 & 0xffff0000 | (u32)paVar14 >> 0x10);
    iVar2 = (int)paVar14;
    iVar5 = iVar2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
    uVar10 = SUB42(paVar11,0x0);
    iVar6 = iVar5;
    pass1_1010_9f72((u32)paVar14,0x3e);
    if (iVar6 != 0x0) {
      iVar6 = pass1_1010_96d0(paVar14);
      if (iVar6 < 0x1) {
        if (iVar6 < 0x0) {
          iVar6 = (int)(u32)(iVar5 + 0x1f6);
          pass1_1030_38b8();
          if (((int)paVar11 < 0x1) && (((int)paVar11 < 0x0 || (iVar6 == 0x0)))) {
            paVar13 = (astruct_67 *)
                      mixed_1010_20ba(paVar11,_u16_1050_0ed0,(u8 **)CONCAT22(iVar2,0x37),in_stack_0000fe6e,
                                      in_stack_0000ff92,in_stack_0000ff98,in_stack_0000ff9c);
            post_win_msg_1008_a0e4(paVar13,0x0,0x0,0x1,(u32)(iVar5 + 0x4),0x6);
          }
        }
      }
      else {
        paVar13 = (astruct_67 *)
                  mixed_1010_20ba(paVar11,_u16_1050_0ed0,(u8 **)CONCAT22(iVar2,0x37),in_stack_0000fe6e,
                                  in_stack_0000ff92,in_stack_0000ff98,in_stack_0000ff9c);
        paVar11 = (astruct_57 *)((u32)paVar11 & 0xffff0000 | (u32)paVar13 >> 0x10);
        post_win_msg_1008_a0e4(paVar13,0x0,iVar6,(iVar5 + 0x208),0x4000001,0x2);
        paVar15 = (astruct_27 *)
                  mixed_1010_20ba(paVar11,_u16_1050_0ed0,(u8 **)CONCAT22(iVar2,0x2b),in_stack_0000fe6e,
                                  in_stack_0000ff92,in_stack_0000ff98,in_stack_0000ff9c);
        pass1_1010_043a(paVar15,*(i32 *)(iVar5 + 0x4),0x14);
      }
    }
  }
  return;
}
pub fn pass1_1028_a0fa(param_1: *mut astruct_334,u8 *param_2,mut param_3: u32)

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
    param_1->field2_0x2 = 0x1008;
    uVar8 = (param_3 >> 0x10);
    param_1->field3_0x4 = (u32)((int)param_3 + 0x4);
    puVar3 = (u32 *)((int)param_3 + 0x8);
    puVar7 = &param_1->field4_0x8;
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1->field2_0x2 = 0x1028;
    *puStack10 = 0xa6f6;
    param_1->field2_0x2 = 0x1028;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_a188(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,mut param_4: i16,mut param_5: u32)

{
  let mut uVar1: u32;
  i32 lVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  i32 lVar8;
  i32 lVar9;
  let mut uVar10: u16;
  astruct_57 *paVar11;
  let mut iVar12: i16;
  let mut iVar13: i16;
  let mut unaff_SI: u16;
  let mut uVar14: u16;
  astruct_67 *paVar15;
  astruct_27 *paVar16;
  let mut in_stack_0000fe70: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ff9e: u16;
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut iStack12: i16;

  uVar14 = (param_5 >> 0x10);
  iVar12 = (int)param_5;
  uVar1 = (u32)(iVar12 + 0x1f6);
  uVar6 = (iVar12 + 0x1f8);
  uVar5 = (int)uVar1 + 0x18c;
  uVar4 = (uVar1 >> 0x10);
  uVar7 = (u32)uVar5;
  pass1_1030_38f2(uVar1 & 0xffff | (u32)uVar6 << 0x10,param_4);
  uVar3 = 0x64 / (long)param_3;
  uVar10 = (int)uVar3 >> 0xf;
  iVar13 = param_4 * 0x4;
  lVar2 = (uVar7 & 0xffff | (u32)uVar6 << 0x10) + *(i32 *)(iVar13 + uVar5);
  lVar8 = lVar2 / (long)(uVar3 & 0xffff | (u32)uVar10 << 0x10);
  lVar9 = lVar8 * (uVar3 & 0xffff | (u32)uVar10 << 0x10);
  uStack14 = lVar2;
  iStack12 = (int)((u32)lVar2 >> 0x10);
  uVar6 = lVar9;
  uVar10 = (iStack12 - (int)((u32)lVar9 >> 0x10)) - (uStack14 < uVar6);
  paVar11 = (astruct_57 *)(u32)uVar10;
  (uVar5 + iVar13) = uStack14 - uVar6;
  (uVar5 + iVar13 + 0x2) = uVar10;
  uStack16 = ((u32)lVar8 >> 0x10);
  uStack18 = lVar8;
  if ((uStack16 | uStack18) != 0x0) {
    pass1_1030_375a(uVar1,param_4,lVar8);
    if (*(i32 *)(iVar12 + 0x200) != 0x8000002) {
      paVar15 = (astruct_67 *)
                mixed_1010_20ba(paVar11,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x37),in_stack_0000fe70,
                                in_stack_0000ff94,in_stack_0000ff9a,in_stack_0000ff9e);
      paVar11 = (astruct_57 *)((u32)paVar11 & 0xffff0000 | (u32)paVar15 >> 0x10);
      post_win_msg_1008_a0e4(paVar15,0x0,uStack18,(iVar12 + 0x208),(u32)(iVar12 + 0x4),0x2);
      paVar16 = (astruct_27 *)
                mixed_1010_20ba(paVar11,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2b),in_stack_0000fe70,
                                in_stack_0000ff94,in_stack_0000ff9a,in_stack_0000ff9e);
      pass1_1010_043a(paVar16,*(i32 *)(iVar12 + 0x4),0xd);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_a28a(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_691)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  u8 *puVar5;
  u8 *puVar6;
  u8 *puVar7;
  let mut uVar8: u16;
  astruct_691 *iVar9;
  let mut uVar9: u16;
  u32 *puVar10;
  u32 *puStack10;

  puVar10 = pass1_1008_c6fa(_u16_1050_06e0,0xe);
  puVar5 = (u8 *)((u32)puVar10 >> 0x10);
  uVar2 = puVar10;
  pass1_1038_4d6e(uVar2,puVar5,param_3,puVar10);
  puStack10 = (u32 *)CONCAT22(puVar5,uVar2);
  uVar9 = ((u32)param_3 >> 0x10);
  iVar9 = (astruct_691 *)param_3;
  uVar4 = iVar9->field502_0x1f6;
  ppcVar1 = (code **)((int)*puStack10 + 0x10);
  puVar6 = puVar5;
  (**ppcVar1)((int)&u16_1050_1038,uVar2,puVar5);
  uVar3 = uVar4;
  puVar7 = puVar6;
  pass1_1030_38b8();
  if ((uVar4 & 0xffff | ZEXT24(puVar6) << 0x10) == 0x0) {
    uVar4 = 0x64;
    uVar8 = 0x0;
  }
  else {
    uVar4 = CONCAT22(puVar7,uVar3) / (long)(uVar4 & 0xffff | ZEXT24(puVar6) << 0x10);
    uVar8 = (uVar4 >> 0x10);
  }
  uVar4 = uVar4 & 0xffff | (u32)uVar8 << 0x10;
  if (puStack10 != NULL) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(0x1030,uVar2,(char)puVar5,0x1);
  }
  if ((long)uVar4 < 0x64) {
    if ((long)uVar4 < 0x55) {
      if ((long)uVar4 < 0x4b) {
        if ((long)uVar4 < 0x32) {
          if ((long)uVar4 < 0x19) {
            iVar9->field519_0x20a = 0x1;
            iVar9->field520_0x20c = 0xffff;
            return;
          }
          iVar9->field519_0x20a = 0x0;
          iVar9->field520_0x20c = 0x0;
          return;
        }
        iVar9->field519_0x20a = 0xfffb;
      }
      else {
        iVar9->field519_0x20a = 0xfff6;
      }
    }
    else {
      iVar9->field519_0x20a = 0xfff1;
    }
  }
  else {
    iVar9->field519_0x20a = 0xffec;
  }
  iVar9->field520_0x20c = 0x1;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_a3ae(mut param_1: i16,i32 param_2,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  BOOL16 BVar1;
  let mut uVar2: u16;
  let mut uVar3: u32;
  astruct_57 *paVar4;
  let mut uVar5: u16;
  let mut unaff_SI: u16;
  astruct_27 *paVar6;
  let mut in_stack_0000fd5a: u16;
  let mut in_stack_0000fe7e: u16;
  let mut in_stack_0000fe84: u16;
  let mut in_stack_0000fe88: u16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut local_146: u16;
  let mut uStack324: u16;
  let mut uStack32: u16;
  let mut uStack30: u16;
  let mut uStack26: u32;
  let mut uStack22: u32;
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut iStack6: i16;
  let mut uStack4: u16;

  iVar7 = (int)param_5;
  uVar8 = (param_5 >> 0x10);
  pass1_1038_3fb0(param_5);
  uStack4 = param_2;
  iStack6 = param_1;
  if (((iVar7 + 0x204) != 0x0) && (BVar1 = pass1_1030_25b2(CONCAT22(uStack4,param_1),0x82), BVar1 != 0x0)) {
    return;
  }
  uVar3 = (u32)(iVar7 + 0x1f6);
  uStack10 = uVar3;
  pass1_1030_38b8();
  uVar2 = uVar3;
  uStack14 = uVar3 & 0xffff | param_2 << 0x10;
  empty_1038_540a();
  uStack16 = param_2;
  paVar4 = (astruct_57 *)(param_2 & 0xffff0000U | (u32)(uStack16 | uVar2));
  uStack18 = uVar2;
  if (((uStack16 | uVar2) == 0x0) && (*(i32 *)(iVar7 + 0x200) != 0x8000002)) {
    pass1_1030_38b8();
    if ((-0x1 < (int)paVar4) && ((0x0 < (int)paVar4 || (uVar2 != 0x0)))) {
      paVar6 = (astruct_27 *)
               mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2b),in_stack_0000fd5a,
                               in_stack_0000fe7e,in_stack_0000fe84,in_stack_0000fe88);
      uStack30 = ((u32)paVar6 >> 0x10);
      uStack32 = SUB42(paVar6,0x0);
      pass1_1010_043a(paVar6,*(i32 *)(iVar7 + 0x4),0x11);
    }
  }
  uStack26 = uStack14;
  uVar2 = uStack18 * 0xa;
  uVar5 = (uStack16 * 0x5 + CARRY2(uStack18,uStack18) * 0x2 + CARRY2(uStack18 * 0x2,uStack18 * 0x2) +
          CARRY2(uStack18 * 0x4,uStack18)) * 0x2 + CARRY2(uStack18 * 0x5,uStack18 * 0x5);
  uStack22 = CONCAT22(uVar5,uVar2);
  if ((uVar5 <= uStack14) && ((uVar5 < uStack14 || (uVar2 < uStack14)))) {
    pass1_1028_ae66((astruct_97 *)CONCAT22(0x1050,&local_146),uStack14,CONCAT22(uVar5,uVar2),(u32)(iVar7 + 0x4));
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_146));
    uStack26 = uStack22;
    local_146 = 0x389a;
    uStack324 = 0x1008;
  }
  uStack26 += 0x9;
  pass1_1038_52b8(param_5,uStack26 / 0xa,0x1e);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_a4ee(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u32;
  code **ppcVar2;
  let mut uVar3: u16;
  BOOL16 BVar4;
  let mut uVar5: u16;
  let mut uVar6: u32;
  u8 *puVar7;
  u8 *puVar8;
  let mut uVar9: u16;
  let mut uVar10: u16;
  u32 *puVar11;
  let mut uVar12: u16;
  let mut iStack50: i16;
  u32 *puStack18;

  uVar9 = (param_2 >> 0x10);
  uVar1 = (u32)((int)param_2 + 0x1f6);
  uVar6 = *_PTR_LOOP_1050_65e2;
  puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0x26);
  puVar7 = (u8 *)((u32)puVar11 >> 0x10);
  uVar5 = puVar11;
  uVar10 = SUB42(&u16_1050_1038,0x0);
  pass1_1038_4d6e(uVar5,puVar7,(astruct_691 *)param_2,puVar11);
  puStack18 = (u32 *)CONCAT22(puVar7,uVar5);
  ppcVar2 = (code **)((int)*puStack18 + 0x10);
  uVar3 = uVar5;
  puVar8 = puVar7;
  (**ppcVar2)((int)&u16_1050_1038,uVar5,puVar7);
  if ((puVar8 | uVar3) != 0x0) {
    uVar10 = 0x1030;
    pass1_1030_3548(uVar1,CONCAT22(puVar8,uVar3));
  }
  if (puStack18 != NULL) {
    ppcVar2 = (code **)*puStack18;
    (**ppcVar2)(uVar10,uVar5,(char)puVar7,0x1);
  }
  uVar3 = (uVar6 % 0xc);
  uVar12 = (param_1 >> 0x10);
  uVar5 = uVar3;
  if (uVar6 % 0xc == 0x0) {
    pass1_1030_387c(uVar1);
    pass1_1028_a61e(uVar5,uVar3,param_1,uVar12,uVar1,param_2);
  }
  pass1_1038_3fb0(param_2);
  if ((((int)param_2 + 0x204) != 0x0) &&
     (BVar4 = pass1_1030_25b2(CONCAT13((char)(uVar3 >> 0x8),CONCAT12((char)uVar3,uVar5)),0x80), BVar4 != 0x0)) {
    return;
  }
  uVar9 = (uVar1 >> 0x10);
  uVar5 = (int)uVar1 + 0x180;
  uVar6 = (u32)uVar5;
  iStack50 = 0x1;
  do {
    if ((iStack50 * 0x2 + uVar5) != 0x0) {
      pass1_1008_612e(uVar6,0x1,0x64);
      if ((int)uVar6 <= (iStack50 * 0x2 + uVar5)) {
        pass1_1028_a188(param_1,uVar12,(iStack50 * 0x2 + (int)uVar1 + 0x174),iStack50,param_2);
      }
    }
    iStack50 += 0x1;
  } while (iStack50 < 0x6);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_a61e(mut param_1: u16 ,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32,mut param_6: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  astruct_27 *paVar5;
  let mut in_stack_0000fe84: u16;
  let mut in_stack_0000ffa8: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffdc: u16;
  let mut uStack16: u16;
  let mut uStack14: u32;

  pass1_1030_38b8();
  if ((param_2 < 0x3fff) || ((param_2 < 0x4000 && (param_1 != 0xffff)))) {
    pass1_1030_38f2(param_5,0x3);
    uVar1 = param_1;
    iVar3 = param_2;
    pass1_1030_38f2(param_5,0x4);
    uStack14 = CONCAT22(param_2 + iVar3 + CARRY2(param_1,uVar1),param_1 + uVar1);
    uStack16 = ((int)param_5 + 0x1a8);
    if (uStack16 == 0x0) {
      uStack16 = 0x5;
    }
    uVar2 = uStack14 / (long)(u32)uStack16;
    uStack14 = (uVar2 >> 0x10);
    uStack14 |= uVar2;
    if ((uStack14 != 0x0) && (uVar4 = (param_6 >> 0x10), *(i32 *)((int)param_6 + 0x200) != 0x8000002)
       ) {
      paVar5 = (astruct_27 *)
               mixed_1010_20ba((astruct_57 *)(uStack14 % (long)(u32)uStack16 & 0xffff0000U | (u32)uStack14),
                               _u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffdc,0x2b),in_stack_0000fe84,
                               in_stack_0000ffa8,in_stack_0000ffae,in_stack_0000ffb2);
      pass1_1010_043a(paVar5,*(i32 *)((int)param_6 + 0x4),0xc);
      pass1_1030_3534(param_5,uVar2);
    }
  }
  return;
}



StructD * pass1_1028_a6ca(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_a706(param_1: *mut astruct_97)

{
  struct_op_1028_d1dc(param_1,0xbb7);
  param_1->offset_0x0 = 0xa856;
  ((int)param_1 + 0x2) = 0x1028;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x8)),s_SCPrelimAlloc_1050_50f6);
  return param_1;
}



u16 pass1_1028_a73c(mut param_1: u16 )

{
  astruct_92 *paVar1;
  astruct_92 *paVar2;
  let mut uVar3: u16;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x400);
  while( true ) {
    paVar1 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar1));
    uVar3 = param_1 | paVar1;
    if (uVar3 == 0x0) break;
    paVar2 = paVar1;
    pass1_1038_5464(paVar1,CONCAT22(param_1,paVar1));
    pass1_1038_56d6(CONCAT22(param_1,paVar1),0x0);
    pass1_1038_518c(paVar2,CONCAT22(param_1,paVar1));
    param_1 = uVar3;
  }
  return 0x1;
}
pub fn pass1_1028_a79c(mut param_1: u16 ,u8 *param_2,mut param_3: u32)

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
    *puStack10 = 0xa856;
    (param_1 + 0x2) = 0x1028;
  }
  return;
}



StructD * pass1_1028_a82a(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_a866(param_1: *mut astruct_97)

{
  struct_op_1028_d1dc(param_1,0x36af);
  param_1->offset_0x0 = 0xa9ae;
  ((int)param_1 + 0x2) = 0x1028;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x8)),s_SCProdSched_1050_5104);
  return param_1;
}



u16 pass1_1028_a89c(mut param_1: u16 )

{
  astruct_92 *paVar1;
  let mut uVar2: u16;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x400);
  while( true ) {
    uVar2 = param_1;
    paVar1 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar1));
    param_1 = uVar2 | paVar1;
    if (param_1 == 0x0) break;
    if (paVar1[0x1c].field4_0x8 != 0x8000002) {
      pass1_1038_3fca(paVar1,CONCAT22(uVar2,paVar1));
    }
  }
  return 0x1;
}
pub fn pass1_1028_a8f4(param_1: *mut astruct_335,mut param_2: u16 ,mut param_3: u32)

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
    param_1->field2_0x2 = 0x1008;
    uVar8 = (param_3 >> 0x10);
    param_1->field3_0x4 = (u32)((int)param_3 + 0x4);
    puVar3 = (u32 *)((int)param_3 + 0x8);
    puVar7 = &param_1->field4_0x8;
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1->field2_0x2 = 0x1028;
    *puStack10 = 0xa9ae;
    param_1->field2_0x2 = 0x1028;
  }
  return;
}



StructD * pass1_1028_a982(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_a9be(param_1: *mut astruct_97)

{
  struct_op_1028_d1dc(param_1,0x176f);
  param_1->offset_0x0 = 0xab22;
  ((int)param_1 + 0x2) = 0x1028;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x8)),s_SCPower_1050_5110);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1028_a9f4(mut param_1: u16 )

{
  code **ppcVar1;
  astruct_92 *paVar2;
  BOOL16 BVar3;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  u32 *puStack24;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x700);
  while( true ) {
    paVar2 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar2));
    puStack24 = (u32 *)CONCAT22(param_1,paVar2);
    uVar4 = param_1 | paVar2;
    if (uVar4 == 0x0) break;
    BVar3 = pass1_1008_c6ae(_u16_1050_06e0,&paVar2->field5_0xc,0xc);
    param_1 = uVar4;
    if (BVar3 != 0x0) {
      ppcVar1 = (code **)((int)*puStack24 + 0x34);
      (**ppcVar1)(0x1008,paVar2);
      param_1 = extraout_DX;
    }
  }
  return 0x1;
}
pub fn pass1_1028_aa68(param_1: *mut astruct_336,u8 *param_2,mut param_3: u32)

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
    param_1->field2_0x2 = 0x1008;
    uVar8 = (param_3 >> 0x10);
    param_1->field3_0x4 = (u32)((int)param_3 + 0x4);
    puVar3 = (u32 *)((int)param_3 + 0x8);
    puVar7 = &param_1->field4_0x8;
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1->field2_0x2 = 0x1028;
    *puStack10 = 0xab22;
    param_1->field2_0x2 = 0x1028;
  }
  return;
}



StructD * pass1_1028_aaf6(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_ab32(param_1: *mut astruct_97)

{
  struct_op_1028_d1dc(param_1,0x2edf);
  param_1->offset_0x0 = 0xaca6;
  ((int)param_1 + 0x2) = 0x1028;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x8)),s_SCRchSched_1050_5118);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1028_ab68(mut param_1: u16 )

{
  let mut uVar1: u16;
  code **ppcVar2;
  astruct_92 *paVar3;
  BOOL16 BVar4;
  let mut uVar5: u16;
  let mut extraout_DX: u16;
  u32 *puStack24;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_14)),0x1,0x0,0x700);//
LAB_1028_ab7e:
  uVar5 = param_1;
  paVar3 = &local_14;
  pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar3));
  puStack24 = (u32 *)CONCAT22(uVar5,paVar3);
  param_1 = uVar5 | paVar3;
  if (param_1 == 0x0) {
    return 0x1;
  }
  uVar1 = &paVar3->field5_0xc;
  BVar4 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x11);
  if (BVar4 == 0x0) goto code_r0x1028abad;
  goto LAB_1028_abc0;
code_r0x1028abad:
  BVar4 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x12);
  if (BVar4 != 0x0) {//
LAB_1028_abc0:
    if ((paVar3 + 0x1) == 0x5) {
      ppcVar2 = (code **)((int)*puStack24 + 0x30);
      (**ppcVar2)(0x1008);
      param_1 = extraout_DX;
    }
  }
  goto LAB_1028_ab7e;
}
pub fn pass1_1028_abec(param_1: *mut astruct_337,u8 *param_2,mut param_3: u32)

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
    param_1->field2_0x2 = 0x1008;
    uVar8 = (param_3 >> 0x10);
    param_1->field3_0x4 = (u32)((int)param_3 + 0x4);
    puVar3 = (u32 *)((int)param_3 + 0x8);
    puVar7 = &param_1->field4_0x8;
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1->field2_0x2 = 0x1028;
    *puStack10 = 0xaca6;
    param_1->field2_0x2 = 0x1028;
  }
  return;
}



StructD * pass1_1028_ac7a(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_acb6(param_1: *mut astruct_97)

{
  struct_op_1028_d1dc(param_1,0x3e7f);
  param_1->offset_0x0 = 0xae56;
  ((int)param_1 + 0x2) = 0x1028;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x8)),s_SCSetup_1050_5124);
  return param_1;
}



// WARNING: Could not reconcile some variable overlaps

u16 pass1_1028_acec(mut param_1: u16 )

{
  astruct_92 *paVar1;
  astruct_92 *paVar2;
  let mut uVar3: u16;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x400);
  while( true ) {
    uVar3 = param_1;
    paVar1 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar1));
    param_1 = uVar3 | paVar1;
    if (param_1 == 0x0) break;
    paVar2 = paVar1;
    vsprintf_op_1030_840a(param_1,(u32)s_SCSetup__calcMe_clearing_colony_0_1050_512c);
    if (paVar1[0x1c].field4_0x8 != 0x8000002) {
      pass1_1038_5464(paVar2,CONCAT22(uVar3,paVar1));
      pass1_1038_56d6(CONCAT22(uVar3,paVar1),0x1);
    }
  }
  local_14._0_2_ = 0x389a;
  local_14.field2_0x2 = 0x1008;
  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x800);
  while( true ) {
    paVar1 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar1));
    uVar3 = param_1 | paVar1;
    if (uVar3 == 0x0) break;
    pass1_1030_2690(CONCAT22(param_1,paVar1));
    param_1 = uVar3;
  }
  return 0x1;
}
pub fn pass1_1028_ad9c(param_1: *mut astruct_338,mut param_2: u16 ,mut param_3: u32)

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
    param_1->field2_0x2 = 0x1008;
    uVar8 = (param_3 >> 0x10);
    param_1->field3_0x4 = (u32)((int)param_3 + 0x4);
    puVar3 = (u32 *)((int)param_3 + 0x8);
    puVar7 = &param_1->field4_0x8;
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1->field2_0x2 = 0x1028;
    *puStack10 = 0xae56;
    param_1->field2_0x2 = 0x1028;
  }
  return;
}



StructD * pass1_1028_ae2a(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1028_ae66(param_1: *mut astruct_97,mut param_2: u32,mut param_3: u32,mut param_4: u32)

{
  astruct_97 *iVar1;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x1387);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  (u32)&iVar1->field259_0x108 = param_4;
  (u32)&iVar1->field262_0x10c = param_3;
  iVar1->field264_0x110 = param_2;
  &iVar1->field265_0x114 = 0x0;
  param_1->offset_0x0 = 0xb0ce;
  iVar1->segment_0x2 = 0x1028;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),s_SCStarve_1050_5156);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_aec0(mut param_1: i16,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_3 >> 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)((int)param_3 + 0x108));
  pass1_1030_375a((u32)(param_1 + 0x1f6),0x0,(long)((int)param_3 + 0x114));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1028_af08(param_1: *mut astruct_57,param_2: *mut astruct_693,mut param_3: u16 )

{
  u8 *puVar1;
  u8 *puVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  astruct_57 *paVar5;
  astruct_693 *iVar6;
  let mut uVar6: u16;
  u32 *puVar7;
  astruct_67 *paVar8;
  astruct_27 *paVar9;
  let mut in_stack_0000fe86: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb4: u16;
  let mut iStack12: i16;
  let mut iStack10: i16;

  puVar7 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x2),in_stack_0000fe86,
                           in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
  paVar5 = (astruct_57 *)((u32)param_1 & 0xffff0000 | (u32)puVar7 >> 0x10);
  puVar1 = PTR_LOOP_1050_13ae + -0x1;
  if (((int)PTR_LOOP_1050_13ae < 0x1) || (SBORROW2((int)PTR_LOOP_1050_13ae,0x1))) {//
LAB_1028_af27:
    iStack10 = 0x1;
  }
  else {
    puVar2 = PTR_LOOP_1050_13ae + -0x2;
    if (puVar2 == NULL || (int)puVar1 < 0x1) {
      iStack12 = 0x1;
      iStack10 = 0x1;
      goto LAB_1028_af42;
    }
    puVar1 = PTR_LOOP_1050_13ae + -0x4;
    if (puVar1 != NULL) goto LAB_1028_af27;
    iStack10 = 0x2;
  }
  iStack12 = 0x3;
  puVar2 = puVar1;//
LAB_1028_af42:
  pass1_1008_612e(puVar2,iStack10,iStack12);
  uVar6 = ((u32)param_2 >> 0x10);
  iVar6 = (astruct_693 *)param_2;
  iVar6->field273_0x114 = puVar2;
  paVar8 = (astruct_67 *)
           mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x37),in_stack_0000fe86,
                           in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
  paVar5 = (astruct_57 *)((u32)paVar5 & 0xffff0000 | (u32)paVar8 >> 0x10);
  iVar3 = (int)paVar8;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,iVar6->field264_0x108);
  uVar4 = SUB42(paVar5,0x0);
  post_win_msg_1008_a0e4(paVar8,0x0,(int)iVar6->field273_0x114,(iVar3 + 0x208),iVar6->field264_0x108,0x2);
  paVar9 = (astruct_27 *)
           mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x2b),in_stack_0000fe86,
                           in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
  pass1_1010_043a(paVar9,*(i32 *)(iVar3 + 0x4),0xd);
  return 0x1;
}
pub fn pass1_1028_afce(param_1: *mut astruct_339,mut param_2: u16 ,param_3: *mut astruct_340)

{
  u32 *puVar1;
  u32 *puVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  astruct_340 *iVar5;
  u32 *puVar6;
  u32 *puVar7;
  let mut uVar8: u16;
  let mut puStack10: *mut u16;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x116,paVar5);
  uVar4 = paVar5;
  puStack10 = (u16 *)CONCAT22(uVar4,param_1);
  if ((uVar4 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    param_1->field2_0x2 = 0x1008;
    uVar8 = ((u32)param_3 >> 0x10);
    iVar5 = (astruct_340 *)param_3;
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
    *puStack10 = 0xb0ce;
    param_1->field2_0x2 = 0x1028;
  }
  return;
}



StructD * pass1_1028_b0a2(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_97 * pass1_1028_b0de(param_1: *mut astruct_97,mut param_2: u32,mut param_3: u32)

{
  pass1_1028_6af2(param_1,param_2,param_3);
  param_1->offset_0x0 = 0xb1f4;
  ((int)param_1 + 0x2) = 0x1028;
  return param_1;
}
pub fn pass1_1028_b108(param_1: *mut astruct_341,mut param_2: u16 ,param_3: *mut astruct_342)

{
  u32 *puVar1;
  u32 *puVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  astruct_342 *iVar5;
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
    iVar5 = (astruct_342 *)param_3;
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
    *puStack10 = 0xb1f4;
    param_1->field2_0x2 = 0x1028;
  }
  return;
}



StructD * pass1_1028_b1c8(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1028_b204(param_1: *mut u16)

{
  let mut uVar1: u16;

  struct_1030_1628((astruct_180 *)param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  ((int)param_1 + 0xc) = 0x0;
  *param_1 = 0xb33c;
  ((int)param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_b22c(mut param_1: u16 ,param_2: *mut u16,mut param_3: u16 ,mut param_4: u32)

{
  let mut in_register_0000000a: u16;
  let mut uVar1: u16;

  pass1_1030_165e((astruct_57 *)CONCAT22(in_register_0000000a,param_1),(astruct_175 *)param_2,0x6000000,param_4);
  uVar1 = ((u32)param_2 >> 0x10);
  ((int)param_2 + 0xc) = param_3;
  *param_2 = 0xb33c;
  ((int)param_2 + 0x2) = 0x1028;
  return param_2;
}
pub fn pass1_1028_b260(param_1: *mut u16)

{
  *param_1 = 0xb33c;
  ((int)param_1 + 0x2) = 0x1028;
  pass1_1030_16b2(param_1);
  return;
}
pub fn FUN_1028_b27e(void)

{
  return;
}



BOOL16 FUN_1028_b282(mut param_1: u16 ,mut param_2: u32,mut param_3: u32)

{
  BOOL16 in_AX;
  BOOL16 BVar1;
  HFILE16 in_stack_0000ffde;
  u16 auStack12 [0x5];

  pass1_1030_16d6((astruct_731 *)param_2,param_3);
  if (in_AX != 0x0) {
    auStack12[0] = ((int)param_2 + 0xc);
    BVar1 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,auStack12),(char *)0x2,in_stack_0000ffde);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return BVar1;
    }
    in_AX = 0x1;
  }
  return in_AX;
}



BOOL16 pass1_1028_b2c8(BOOL16 param_1,param_2: *mut astruct_373,HFILE16 *param_3)

{
  BOOL16 BVar1;
  let mut uVar2: u16;
  let mut local_4: u16;

  file_1030_1730(param_2,param_3);
  if (param_1 != 0x0) {
    BVar1 = read_file_1008_7dee(param_3,(u8 *)CONCAT22(0x1050,&local_4),0x2);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return BVar1;
    }
    uVar2 = switch_1008_72bc(param_3,local_4);
    ((int)param_2 + 0xc) = uVar2;
    param_1 = 0x1;
  }
  return param_1;
}



StructD * pass1_1028_b316(StructD *param_1,u8 param_2)

{
  pass1_1028_b260(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn struct_1028_b354(param_1: *mut astruct_180)

{
  astruct_180 *iVar1;
  let mut uVar1: u16;

  struct_1030_1628(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_180 *)param_1;
  iVar1->field10_0xc = 0x0;
  iVar1->field11_0xe = 0x0;
  iVar1->field12_0x10 = 0x0;
  iVar1->field13_0x12 = 0x0;
  iVar1->field16_0x18 = 0x0;
  iVar1->field17_0x1a = 0x0;
  iVar1->field18_0x1c = 0x0;
  param_1->field0_0x0 = 0xcf6a;
  iVar1->field1_0x2 = 0x1028;
  iVar1->field15_0x16 = 0x0;
  iVar1->field14_0x14 = 0x0;
  return;
}
pub fn pass1_1028_b39e(StructD *param_1,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  let mut in_register_0000000a: u16;
  astruct_12 *iVar1;
  let mut uVar1: u16;

  pass1_1030_165e((astruct_57 *)CONCAT22(in_register_0000000a,param_1),(astruct_175 *)param_2,0x7000000,param_4);
  uVar1 = ((u32)param_2 >> 0x10);
  iVar1 = (astruct_12 *)param_2;
  iVar1->field10_0xc = param_3;
  iVar1->field11_0xe = 0x42;
  iVar1->field12_0x10 = 0x0;
  iVar1->field13_0x12 = 0x0;
  iVar1->field18_0x18 = 0x0;
  iVar1->field19_0x1a = 0x0;
  iVar1->field20_0x1c = 0x0;
  param_2->field0_0x0 = 0xcf6a;
  iVar1->field1_0x2 = 0x1028;
  pass1_1028_bf76(0x0,(astruct_12 *)((u32)param_2 & 0xffff | (u32)uVar1 << 0x10));
  (u32)&iVar1->field_0x14 = 0x0;
  if ((0x4e < (int)iVar1->field10_0xc) && ((int)iVar1->field10_0xc < 0x70)) {
    iVar1->field11_0xe = 0x6b;
  }
  return;
}
pub fn pass1_1028_b418(param_1: *mut u16)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar2 = (int)param_1;
  *param_1 = 0xcf6a;
  (iVar2 + 0x2) = 0x1028;
  iVar1 = (iVar2 + 0x12);
  if (((iVar1 == 0x4) || (iVar1 == 0x5)) ||
     ((iVar1 == 0x6 && ((iVar1 = (iVar2 + 0x18), iVar1 == 0x4 || (iVar1 == 0x5)))))) {
    fn_ptr_1000_17ce(*(char **)(iVar2 + 0x14));
  }
  pass1_1030_16b2(param_1);
  return;
}
pub fn pass1_1028_b46e(mut param_1: u16 ,param_2: *mut astruct_15,mut param_3: u32)

{
  let mut iVar1: i16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar4;
  astruct_302 *paVar5;
  let mut uVar6: u16;
  let mut uVar7: u16;

  uVar3 = ((u32)in_EDX >> 0x10);
  paVar5 = (astruct_302 *)pass1_1028_b4f2(param_2);
  paVar4 = (astruct_57 *)CONCAT22(uVar3,(int)((u32)paVar5 >> 0x10));
  iVar2 = (int)paVar5;
  uVar6 = 0x0;
  uVar7 = 0x0;
  pass1_1028_b58e(param_2);
  uVar3 = SUB42(paVar4,0x0);
  pass1_1030_6d80((astruct_299 *)CONCAT22(uVar3,iVar2),CONCAT22(uVar7,uVar6));
  iVar1 = (iVar2 + 0x32);
  if (iVar1 != 0x0) {
    pass1_1030_6c4c(CONCAT22(uVar3,iVar2),0x0);
    pass1_1038_387e(paVar4,paVar5,0x0,iVar1,CONCAT22(uVar3,iVar2));
  }
  fn_ptr_1030_7296((astruct_292 *)CONCAT22(uVar3,iVar2));
  (u32)((int)param_2 + 0x1c) = (u32)((int)param_3 + 0x200);
  return;
}



u16 FUN_1028_b4e6(void)

{
  return 0x0;
}



u16 FUN_1028_b4ec(void)

{
  return 0x0;
}



pub fn pass1_1028_b4f2(param_1: *mut astruct_15) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u32;

  uVar2 = pass1_1028_b58e(param_1);
  uVar1 = ((u32)uVar2 >> 0x10);
  return CONCAT22(((int)uVar2 + 0x30),((int)uVar2 + 0x2e));
}
pub fn pass1_1028_b514(param_1: *mut astruct_15)

{
  let mut iVar1: i16;
  astruct_15 *pstruct15_1;
  let mut pstruct15_1_hi: u16;
  astruct_290 *paVar3;

  pstruct15_1_hi = ((u32)param_1 >> 0x10);
  pstruct15_1 = (astruct_15 *)param_1;
  iVar1 = pstruct15_1->field15_0x12;
  if (((iVar1 == 0x4) || (iVar1 == 0x5)) ||
     ((iVar1 == 0x6 && ((iVar1 = pstruct15_1->field17_0x18, iVar1 == 0x4 || (iVar1 == 0x5)))))) {
    fn_ptr_1000_17ce((char *)pstruct15_1->field16_0x14);
  }
  pstruct15_1->field16_0x14 = NULL;
  pstruct15_1->field15_0x12 = 0x7;
  paVar3 = (astruct_290 *)pass1_1028_b58e((astruct_15 *)((u32)param_1 & 0xffff | (u32)pstruct15_1_hi << 0x10));
  paVar3._0_2_ = paVar3;
  fn_ptr_1030_7296((astruct_292 *)paVar3);
  pass1_1030_72d0((astruct_292 *)paVar3);
  pass1_1030_730a(paVar3,paVar3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_b58e(param_1: *mut astruct_15)

{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)((int)param_1 + 0x8));
  return;
}



u16 pass1_1028_b5a8(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (((int)param_1 + 0x12) != 0x5) {
    return 0x0;
  }
  uVar1 = (u32)((int)param_1 + 0x14);
  return ((int)uVar1 + 0x94);
}



u16 pass1_1028_b5ca(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (((int)param_1 + 0x12) != 0x5) {
    return 0x0;
  }
  uVar1 = (u32)((int)param_1 + 0x14);
  return ((int)uVar1 + 0x9c);
}



BOOL16 write_to_file_1028_b5ec(param_1: *mut astruct_731,mut param_2: u32)

{
  let mut uVar1: u32;
  BOOL16 BVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  HFILE16 in_stack_0000ffbc;
  u16 local_e [0x3];
  u16 local_8 [0x2];
  let mut iStack4: i16;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar3 = (int)param_1;
  local_e[0] = (iVar3 + 0xc);
  BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_e),(char *)0x2,in_stack_0000ffbc);
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  pass1_1030_16d6(param_1,param_2);
  if (BVar2 == 0x0) {
    return 0x0;
  }
  local_8[0] = (iVar3 + 0xc);
  BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_8),(char *)0x2,in_stack_0000ffbc);
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  local_8[0] = (iVar3 + 0xe);
  BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_8),(char *)0x2,in_stack_0000ffbc);
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  local_8[0] = (iVar3 + 0x10);
  BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_8),(char *)0x2,in_stack_0000ffbc);
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  local_8[0] = (iVar3 + 0x12);
  BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_8),(char *)0x2,in_stack_0000ffbc);
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  local_8[0] = (iVar3 + 0x18);
  BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_8),(char *)0x2,in_stack_0000ffbc);
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  local_8[0] = (iVar3 + 0x1a);
  BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_8),(char *)0x2,in_stack_0000ffbc);
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  iStack4 = (iVar3 + 0x12);
  if (iStack4 == 0x6) {
    iStack4 = (iVar3 + 0x18);
  }
  if (iStack4 < 0x1) {
    return 0x1;
  }
  if (SBORROW2(iStack4,0x1)) {
    return 0x1;
  }
  if (iStack4 == 0x3 || iStack4 + -0x2 < 0x1) {
    local_8[0] = (iVar3 + 0x14);
  }
  else if (iStack4 == 0x4) {
    if (*(i32 *)(iVar3 + 0x14) == 0x0) {
      local_8[0] = 0x0;
      BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_8),(char *)0x2,in_stack_0000ffbc);
      goto joined_r0x1028b766;
    }
    uVar1 = (u32)(iVar3 + 0x14);
    local_8[0] = ((int)uVar1 + 0x94);
  }
  else {
    if (iStack4 != 0x5) {
      return 0x1;
    }
    uVar1 = (u32)(iVar3 + 0x14);
    local_8[0] = ((int)uVar1 + 0xa4);
    BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_8),(char *)0x2,in_stack_0000ffbc);
    if (BVar2 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return 0x0;
    }
    uVar1 = (u32)(iVar3 + 0x14);
    local_8[0] = ((int)uVar1 + 0xa6);
    BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_8),(char *)0x2,in_stack_0000ffbc);
    if (BVar2 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return 0x0;
    }
    uVar1 = (u32)(iVar3 + 0x14);
    local_8[0] = ((int)uVar1 + 0xa8);
    BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_8),(char *)0x2,in_stack_0000ffbc);
    if (BVar2 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return 0x0;
    }
    uVar1 = (u32)(iVar3 + 0x14);
    local_8[0] = ((int)uVar1 + 0xaa);
    BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_8),(char *)0x2,in_stack_0000ffbc);
    if (BVar2 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return 0x0;
    }
    uVar1 = (u32)(iVar3 + 0x14);
    local_8[0] = ((int)uVar1 + 0xac);
  }
  BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,local_8),(char *)0x2,in_stack_0000ffbc);
joined_r0x1028b766:
  if (BVar2 == 0x0) {
    u16_1050_0310 = 0x6d0;
    return 0x0;
  }
  return 0x1;
}



// WARNING: Unable to use type for symbol temp_7fffad5008a
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn file_1028_b81a(mut param_1: i16,u8 *param_2,param_3: *mut astruct_373,HFILE16 *param_4)

{
  BOOL16 BVar1;
  let mut iVar2: i16;
  u32 *puVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar6;
  let mut uVar7: u32;
  let mut uVar8: u16;
  u16 local_2a [0x2];
  u8 local_26 [0x16];
  u32 *puStack16;
  let mut uStack14: u16;
  let mut iStack10: i16;
  let mut local_8: i16;
  let mut local_6: i16;
  let mut local_4: i16;
  u32 *temp_7fffad5008a;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  temp_7fffad5008a = (u32 *)param_3;
  uVar5 = ((u32)param_3 >> 0x10);
  file_1030_1730(param_3,param_4);
  if (param_1 == 0x0) {
    return;
  }
  BVar1 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_4),0x2);
  if (BVar1 == 0x0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  BVar1 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_6),0x2);
  if (BVar1 == 0x0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  BVar1 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_8),0x2);
  if (BVar1 == 0x0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  uVar4 = param_4;
  uVar8 = ((u32)param_4 >> 0x10);
  iVar2 = switch_1008_73ea(uVar4,uVar8,local_4);
  (temp_7fffad5008a + 0x3) = iVar2;
  iVar2 = switch_1008_73ea(uVar4,uVar8,local_6);
  ((int)temp_7fffad5008a + 0xe) = iVar2;
  iVar2 = switch_1008_73ea(uVar4,uVar8,local_8);
  (temp_7fffad5008a + 0x4) = iVar2;
  BVar1 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_4),0x2);
  if (BVar1 == 0x0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  BVar1 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_6),0x2);
  if (BVar1 == 0x0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  BVar1 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | (u32)((int)temp_7fffad5008a + 0x1a)),0x2);
  if (BVar1 == 0x0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  ((int)temp_7fffad5008a + 0x12) = local_4;
  (temp_7fffad5008a + 0x6) = local_6;
  iStack10 = ((int)temp_7fffad5008a + 0x12);
  if (iStack10 == 0x6) {
    iStack10 = (temp_7fffad5008a + 0x6);
  }
  switch(iStack10) {
  case 0x1:
  case 0x2:
  case 0x3:
    puVar3 = temp_7fffad5008a + 0x5;//
LAB_1028_b968:
    BVar1 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(uVar5,puVar3),0x2);
    break;
  case 0x4:
    uVar7 = pass1_1028_e0bc(temp_7fffad5008a,paVar6,_PTR_LOOP_1050_65e2,(temp_7fffad5008a + 0x3));
    uStack14 = (uVar7 >> 0x10);
    (temp_7fffad5008a + 0x5) = (int)uVar7;
    ((int)temp_7fffad5008a + 0x16) = uStack14;
    if ((uStack14 | (temp_7fffad5008a + 0x5)) != 0x0) {
      puVar3 = (u32 *)((temp_7fffad5008a + 0x5) + 0x94);
      uVar5 = uStack14;
      puStack16 = puVar3;
      goto LAB_1028_b968;
    }
    BVar1 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,local_26),0x2);
    break;
  case 0x5:
    puVar3 = temp_7fffad5008a;
    pass1_1028_e100((u8 *)paVar6,_PTR_LOOP_1050_65e2,(temp_7fffad5008a + 0x3));
    (u32*)(temp_7fffad5008a + 0x5) = puVar3;
    uStack14 = paVar6;
    ((int)temp_7fffad5008a + 0x16) = uStack14;
    puStack16 = (u32 *)((temp_7fffad5008a + 0x5) + 0xa4);
    BVar1 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(uStack14,puStack16),0x2);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
    BVar1 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,local_2a),0x2);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
    uVar7 = temp_7fffad5008a[0x5];
    BVar1 = read_file_1008_7dee(param_4,(u8 *)(uVar7 & 0xffff0000 | (u32)((int)uVar7 + 0xa8)),0x2);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
    uVar7 = temp_7fffad5008a[0x5];
    BVar1 = read_file_1008_7dee(param_4,(u8 *)(uVar7 & 0xffff0000 | (u32)((int)uVar7 + 0xaa)),0x2);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
    uVar7 = temp_7fffad5008a[0x5];
    BVar1 = read_file_1008_7dee(param_4,(u8 *)(uVar7 & 0xffff0000 | (u32)((int)uVar7 + 0xac)),0x2);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
    uVar4 = switch_1008_72bc(param_4,local_2a[0]);
    uVar7 = temp_7fffad5008a[0x5];
    ((int)uVar7 + 0xa6) = uVar4;
    return;
  default:
    goto switchD_1028_ba97_caseD_6;
  case 0x9:
    puVar3 = temp_7fffad5008a;
    pass1_1028_e100((u8 *)paVar6,_PTR_LOOP_1050_65e2,(temp_7fffad5008a + 0x3));
    (u32*)(temp_7fffad5008a + 0x5) = puVar3;
    ((int)temp_7fffad5008a + 0x16) = (int)paVar6;
    goto switchD_1028_ba97_caseD_6;
  }
  if (BVar1 == 0x0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
switchD_1028_ba97_caseD_6:
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_bab6(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15)

{
  let mut uVar1: u32;

  uVar1 = pass1_1028_bad4(param_1,param_2,param_3);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1);
  return;
}



pub fn pass1_1028_bad4(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15) -> u32

{
  pass1_1028_baf6(param_3);
  return CONCAT22((param_1 + 0xa),(param_1 + 0x8));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_baf6(param_1: *mut astruct_15)

{
  let mut uVar1: u32;

  uVar1 = pass1_1028_bb24(param_1);
  if (uVar1 == 0x0) {
    return;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1);
  return;
}



pub fn pass1_1028_bb24(param_1: *mut astruct_15) -> u32

{
  let mut uVar1: u16;
  astruct_15 *uVar2;
  let mut uVar3: u32;

  uVar2 = (astruct_15 *)((u32)param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x8) == 0x0) {
    return 0x0;
  }
  uVar3 = pass1_1028_b58e((astruct_15 *)((u32)param_1 & 0xffff | ZEXT24(uVar2) << 0x10));
  uVar1 = ((u32)uVar3 >> 0x10);
  return CONCAT22(((int)uVar3 + 0xa),((int)uVar3 + 0x8));
}
pub fn pass1_1028_bb56(mut param_1: u32,mut param_2: u32)

{
  pass1_1030_177a(param_1,param_2);
  return;
}



pub fn pass1_1028_bb6a(mut param_1: u32) -> u32

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (((iVar1 + 0x12) != 0x5) && ((iVar1 + 0x12) != 0x6)) {
    return 0x0;
  }
  return CONCAT22((iVar1 + 0x16),(iVar1 + 0x14) + 0xa4);
}
pub fn pass1_1028_bb96(param_1: *mut astruct_295,u32 *param_2,mut param_3: u16 )

{
  u32 *puVar1;
  u32 *puVar2;
  let mut uVar3: u32;
  let mut iVar6: i16;
  astruct_295 *iVar5;
  astruct_297 *iVar4;
  u32 *puVar7;
  astruct_295 *uVar8;
  let mut uVar9: u16;

  uVar8 = (astruct_295 *)((u32)param_1 >> 0x10);
  iVar5 = (astruct_295 *)param_1;
  if ((&iVar5->field_0x12 == 0x5) || (&iVar5->field_0x12 == 0x6)) {
    uVar3 = iVar5->field20_0x14;
    uVar9 = ((u32)uVar3 >> 0x10);
    puVar7 = (u32 *)((int)uVar3 + 0xa4);
    for (iVar6 = 0x2; iVar6 != 0x0; iVar6 += -0x1) {
      puVar2 = puVar7;
      puVar7 = puVar7 + 0x1;
      puVar1 = param_2;
      param_2 = param_2 + 0x1;
      *puVar2 = *puVar1;
    }
    puVar7 = param_2;
    pass1_1028_c724(param_1);
    uVar3 = iVar5->field20_0x14;
    uVar9 = ((u32)uVar3 >> 0x10);
    iVar4 = (astruct_297 *)uVar3;
    if (iVar4->field170_0xaa == 0x0) {
      iVar4->field170_0xaa = 0x1;
    }
  }
  return;
}
pub fn pass1_1028_bbf0(mut param_1: u16 ,mut param_2: u16 ,u32 *param_3)

{
  *param_3 = 0x0;
  return;
}
pub fn pass1_1028_bc02(u32 *param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)*param_1 + 0x40);
  (**ppcVar1)();
  return;
}



u16 pass1_1028_bc1c(mut param_1: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  if ((iVar1 + 0x12) == 0x4) {
    return (iVar1 + 0xe);
  }
  if ((iVar1 + 0x12) == 0x7) {
    return (iVar1 + 0x10);
  }
  return (iVar1 + 0xc);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1028_bc4a(u32 *param_1,u8 *param_2,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  char *pcVar2;

  pcVar2 = (char *)pass1_1028_e0bc(param_1,(astruct_57 *)CONCAT22(in_register_0000000a,param_2),_PTR_LOOP_1050_65e2,
                                   ((int)param_3 + 0xc));
  uVar1 = ((int)pcVar2 + 0x96);
  fn_ptr_1000_17ce(pcVar2);
  return uVar1;
}
pub fn pass1_1028_bc7e(u32 *param_1)

{
  pass1_1028_bdac((astruct_15 *)param_1,0x4);
  return;
}



u16 pass1_1028_bc90(mut param_1: i16,mut param_2: u16 ,u32 *param_3,param_4: *mut u16,mut param_5: u32,mut param_6: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut iVar3: i16;
  BOOL16 BVar4;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut uVar7: u16;

  uVar6 = param_3;
  uVar7 = ((u32)param_3 >> 0x10);
  pass1_1028_c7b6(param_2,uVar6,uVar7,param_4,param_6);
  if ((param_1 == 0x5) || (param_1 == 0x6)) {
    uVar2 = *param_3;
    ppcVar1 = (code **)((int)uVar2 + 0x60);
    iVar3 = (**ppcVar1)();
    if (iVar3 != 0x0) {
      ppcVar1 = (code **)((int)uVar2 + 0x5c);
      uVar5 = (**ppcVar1)();
      if (uVar5 != 0x0) {
        pass1_1028_c23e(uVar5,((u32)uVar5 >> 0x10),uVar6,uVar7,param_4,param_5,param_6);
        if ((int)uVar5 != 0x0) {
          BVar4 = pass1_1028_c314((int)uVar5,((u32)uVar5 >> 0x10),uVar6,uVar7,param_4,param_5,
                                  (param_5 >> 0x10),param_6);
          if (BVar4 != 0x0) {
            return 0x1;
          }
        }
      }
    }
  }
  else {
    PTR_LOOP_1050_50ca = (u8 *)0x6a8;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_bd38(mut param_1: u16 ,param_2: *mut astruct_15)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u32;
  astruct_117 *pstruct117_4;
  astruct_117 *pstruct117_6;
  astruct_117 *pstruct117_5;
  let mut extraout_DX: u16;
  let mut iStack20: i16;

  pstruct117_5 = *(astruct_117 **)((int)_PTR_LOOP_1050_65e2 + 0x52);
  pass1_1030_4bbe(param_1,pstruct117_5,((int)param_2 + 0xc));
  pstruct117_4 = (astruct_117 *)pstruct117_5;
  pstruct117_6 = pstruct117_4;
  pass1_1028_b58e(param_2);
  uVar3 = (u32)((int)&pstruct117_6[0x1].field15_0x12 + 0x2);
  iStack20 = 0x11;
  do {
    uVar1 = (&pstruct117_4->field_0x0 + iStack20 * 0x4);
    uVar2 = (&pstruct117_4->field_0x2 + iStack20 * 0x4);
    if ((uVar2 | uVar1) != 0x0) {
      pass1_1038_5770(uVar3,CONCAT22(uVar2,uVar1),iStack20);
    }
    iStack20 += 0x1;
  } while (iStack20 < 0x25);
  return;
}
pub fn pass1_1028_bdac(param_1: *mut astruct_15,mut param_2: i16)

{
  let mut iVar1: i16;
  code **ppcVar2;
  astruct_15 *pstruct15_3;
  param_1: *mut astruct_15_hi;
  let mut unaff_CS: u16;

  param_1_hi = (astruct_15 *)((u32)param_1 >> 0x10);
  pstruct15_3 = (astruct_15 *)param_1;
  if (pstruct15_3->field15_0x12 != param_2) {
    if (pstruct15_3->field15_0x12 == 0x6) {
      if (pstruct15_3->field17_0x18 == param_2) {
        pstruct15_3->field15_0x12 = pstruct15_3->field17_0x18;
        pstruct15_3->field17_0x18 = 0x0;
        return;
      }
    }
    else {
      if (param_2 != 0x6) {
        iVar1 = pstruct15_3->field15_0x12;
        if ((iVar1 == 0x4) || (iVar1 == 0x5)) {
          unaff_CS = 0x1000;
          fn_ptr_1000_17ce((char *)pstruct15_3->field16_0x14);
        }
        pstruct15_3->field15_0x12 = param_2;
        ppcVar2 = (code **)((int)(u32)param_1 + 0x3c);
        (**ppcVar2)(unaff_CS,param_1);
        return;
      }
      pstruct15_3->field17_0x18 = pstruct15_3->field15_0x12;
      pstruct15_3->field15_0x12 = 0x6;
    }
  }
  return;
}
pub fn pass1_1028_be2a(param_1: *mut astruct_15)

{
  code **ppcVar1;
  astruct_15 *uVar2;
  let mut uVar3: u32;
  let mut iVar4: i16;

  uVar2 = (astruct_15 *)((u32)param_1 >> 0x10);
  if (((int)param_1 + 0x12) != 0x6) {
    return;
  }
  uVar3 = pass1_1028_b4f2(param_1);
  if (*(i32 *)((int)uVar3 + 0x200) != 0x8000002) {
    if (*(i32 *)((int)param_1 + 0x1c) == 0x8000002) {
      iVar4 = 0x6;
      goto code_r0x1028be96;
    }
    ppcVar1 = (code **)((int)(u32)param_1 + 0x64);
    iVar4 = (**ppcVar1)();
    if (iVar4 == 0x0) {
      return;
    }
    pass1_1028_cb04(param_1);
    if (iVar4 == 0x0) {
      iVar4 = 0x6;
      goto code_r0x1028be96;
    }
    pass1_1028_c952(param_1);
  }
  iVar4 = 0x5;
code_r0x1028be96:
  pass1_1028_bdac(param_1,iVar4);
  return;
}



// WARNING: Unable to use type for symbol uVar2
pub fn pass1_1028_be9e(param_1: *mut astruct_15)

{
  i16 *piVar1;
  StructD *pSVar2;
  let mut iVar3: i16;
  astruct_15 *pstruct15_4;
  astruct_15 *uVar5;
  let mut uVar4: u32;
  StructD *uVar2;

  uVar5 = (astruct_15 *)((u32)param_1 >> 0x10);
  pstruct15_4 = (astruct_15 *)param_1;
  if (pstruct15_4->field15_0x12 == 0x4) {
    uVar4 = pass1_1028_b4f2(param_1);
    iVar3 = (int)uVar4;
    if (*(i32 *)(iVar3 + 0x200) == 0x8000002) {
      pSVar2 = pstruct15_4->field16_0x14;
      piVar1 = (i16 *)((int)pSVar2 + 0x94);
      *piVar1 = *piVar1 + -0x1;
    }
    else {
      pass1_1028_cb04(param_1);
      if (iVar3 == 0x0) {
        return;
      }
      pSVar2 = pstruct15_4->field16_0x14;
      piVar1 = (i16 *)((int)pSVar2 + 0x94);
      *piVar1 = *piVar1 + -0x1;
      pass1_1028_c952(param_1);
    }
    uVar2 = pstruct15_4->field16_0x14;
    if (((int)uVar2 + 0x94) < 0x1) {
      pass1_1028_bdac(param_1,0x5);
    }
  }
  return;
}
pub fn FUN_1028_bf16(void)

{
  return;
}
pub fn FUN_1028_bf1a(void)

{
  return;
}
pub fn FUN_1028_bf1e(void)

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_bf22(u8 *param_1,mut param_2: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar5 = (param_2 >> 0x10);
  iVar4 = (int)param_2;
  iVar1 = (iVar4 + 0x12);
  if (iVar1 == 0x4) {
    uVar6 = pass1_1028_e0bc(NULL,paVar3,_PTR_LOOP_1050_65e2,(iVar4 + 0xc));
    uVar2 = (uVar6 >> 0x10);
    iVar1 = (int)uVar6;
  }
  else {
    iVar1 += -0x5;
    if (iVar1 != 0x0) {
      if (iVar1 != 0x1) {
        (u32)(iVar4 + 0x14) = 0x0;
      }
      return;
    }
    pass1_1028_e100(param_1,_PTR_LOOP_1050_65e2,(iVar4 + 0xc));
    uVar2 = SUB42(paVar3,0x0);
  }
  (iVar4 + 0x14) = iVar1;
  (iVar4 + 0x16) = uVar2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_bf76(mut param_1: u16 ,param_2: *mut astruct_12)

{
  BOOL16 BVar1;
  astruct_12 *iVar2;
  let mut uVar2: u16;

  pass1_1008_612e(param_1,0x1,0x3);
  uVar2 = ((u32)param_2 >> 0x10);
  iVar2 = (astruct_12 *)param_2;
  BVar1 = pass1_1008_c6ae(_u16_1050_06e0,iVar2->field10_0xc,0x28);
  if (BVar1 == 0x0) {
    if (param_1 == 0x1) {
      iVar2->field12_0x10 = 0x48;
      return;
    }
    if (param_1 != 0x2) {
      iVar2->field12_0x10 = 0x4a;
      return;
    }
    iVar2->field12_0x10 = 0x49;
    return;
  }
  if (param_1 == 0x1) {
    iVar2->field12_0x10 = 0x70;
    return;
  }
  if (param_1 != 0x2) {
    iVar2->field12_0x10 = 0x72;
    return;
  }
  iVar2->field12_0x10 = 0x71;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_c00a(mut param_1: i16,param_2: *mut astruct_15,i32 param_3)

{
  astruct_691 *paVar1;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  u8 *puVar5;
  let mut extraout_DX_00: u16;
  let mut extraout_DX_01: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  u32 *puVar8;
  let mut uVar9: u32;
  let mut uVar10: u32;
  let mut uStack26: u32;
  let mut uStack22: u32;
  u32 *puStack18;

  pass1_1028_b58e(param_2);
  paVar1 = *(astruct_691 **)(param_1 + 0x2e);
  puVar8 = pass1_1008_c6fa(_u16_1050_06e0,0x4);
  puVar5 = (u8 *)((u32)puVar8 >> 0x10);
  uVar3 = puVar8;
  uVar7 = SUB42(&u16_1050_1038,0x0);
  pass1_1038_4d6e(uVar3,puVar5,paVar1,puVar8);
  puStack18 = (u32 *)CONCAT22(puVar5,uVar3);
  ppcVar2 = (code **)((int)*puStack18 + 0x10);
  uVar6 = uVar3;
  (**ppcVar2)((int)&u16_1050_1038,uVar3,puVar5);
  uStack22 = CONCAT22(extraout_DX_00,uVar6);
  uStack26 = 0x0;
  do {
    if (uStack22 <= uStack26) {//
LAB_1028_c0d6:
      if (puStack18 != NULL) {
        ppcVar2 = (code **)*puStack18;
        (**ppcVar2)(uVar7,uVar3,(char)puVar5,0x1);
      }
      return;
    }
    ppcVar2 = (code **)((int)*puStack18 + 0x4);
    uVar9 = uStack22;
    (**ppcVar2)((char)uVar7,uVar3,puVar5,(char)uStack26,(int)(uStack26 >> 0x10));
    uVar4 = uVar9;
    uVar6 = extraout_DX_01;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar9 & 0xffff | (u32)extraout_DX_01 << 0x10);
    uVar7 = 0x1030;
    uVar9 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar6,uVar4),uVar4,uVar6);
    uVar10 = pass1_1028_6302(uVar9);
    uVar6 = (uVar10 >> 0x10);
    if ((param_3 <= uVar6) && ((param_3 < uVar6 || (param_3 <= uVar10)))) {
      pass1_1028_6356(uVar9,0x0,param_3,param_3);
      goto LAB_1028_c0d6;
    }
    pass1_1028_6356(uVar9,0x0,uVar10,uVar6);
    param_3 -= uVar10;
    uStack26 += 0x1;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_c0f0(mut param_1: i16,param_2: *mut astruct_15,i32 param_3)

{
  astruct_691 *paVar1;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  u8 *puVar5;
  u8 *extraout_DX_00;
  let mut extraout_DX_01: u16;
  let mut uVar6: u16;
  u8 *puVar7;
  u8 *extraout_DX_02;
  let mut uVar8: u16;
  u32 *puVar9;
  let mut uVar10: u32;
  let mut uStack28: u32;
  let mut uStack24: u32;
  u32 *puStack20;
  astruct_398 *paStack6;

  pass1_1028_b58e(param_2);
  paStack6 = (astruct_398 *)CONCAT22(extraout_DX,param_1);
  paVar1 = *(astruct_691 **)(param_1 + 0x2e);
  pass1_1028_cb04(param_2);
  uVar8 = ((u32)paVar1 >> 0x10);
  if ((((int)paVar1 + 0x204) == 0x0) && (((int)paVar1 + 0x206) == 0x0)) {
    puVar9 = pass1_1008_c6fa(_u16_1050_06e0,0x4);
    puVar5 = (u8 *)((u32)puVar9 >> 0x10);
    uVar3 = puVar9;
    uVar8 = SUB42(&u16_1050_1038,0x0);
    pass1_1038_4d6e(uVar3,puVar5,paVar1,puVar9);
    puStack20 = (u32 *)CONCAT22(puVar5,uVar3);
    ppcVar2 = (code **)((int)*puStack20 + 0x10);
    uVar6 = uVar3;
    (**ppcVar2)((int)&u16_1050_1038,uVar3,puVar5);
    uStack24 = CONCAT22(extraout_DX_00,uVar6);
    puVar7 = extraout_DX_00;
    for (uStack28 = 0x0; uStack28 < uStack24; uStack28 += 0x1) {
      ppcVar2 = (code **)((int)*puStack20 + 0x4);
      uVar10 = uStack24;
      (**ppcVar2)((char)uVar8,uVar3,puVar5,(char)uStack28,(int)(uStack28 >> 0x10));
      uVar4 = uVar10;
      uVar6 = extraout_DX_01;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar10 & 0xffff | (u32)extraout_DX_01 << 0x10);
      uVar8 = 0x1030;
      uVar10 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar6,uVar4),uVar4,uVar6);
      uVar10 = pass1_1028_6302(uVar10);
      puVar7 = (u8 *)(uVar10 >> 0x10);
      uVar6 = uVar10;
      if ((param_3 <= puVar7) && ((param_3 < puVar7 || (param_3 <= uVar6)))) {
        param_3 = 0x0;
        break;
      }
      param_3 = CONCAT22(param_3 + (-(param_3 < uVar6) - (int)puVar7),param_3 - uVar6);
    }
    if (puStack20 != NULL) {
      ppcVar2 = (code **)*puStack20;
      (**ppcVar2)(uVar8,uVar3,(char)puVar5,0x1);
      puVar7 = extraout_DX_02;
    }
    if (param_3 != 0x0) {
      pass1_1030_7d7c(puStack20,puVar7,paStack6,param_3,CONCAT22(0x1d,(int)((u32)param_3 >> 0x10)));
    }
  }
  return;
}
pub fn pass1_1028_c1f8(mut param_1: i16,mut param_2: u16 ,param_3: *mut astruct_15,param_4: *mut u16,param_5: *mut u16)

{
  u32 *puVar1;
  let mut local_c: u32;
  let mut uStack8: u16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_baf6(param_3);
  iStack6 = param_1;
  uStack4 = param_2;
  puVar1 = (u32 *)pass1_1030_5b5c(param_1,param_2);
  local_c = *puVar1;
  uStack8 = ((int)puVar1 + 0x4);
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_c),param_4,(char *)param_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_c23e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut u16,mut param_6: u32,i32 param_7)

{
  let mut uVar1: u32;
  code **ppcVar2;
  u32 *puVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut extraout_DX: u16;
  u32 *puStack22;
  astruct_294 *paStack10;
  let mut uStack6: u32;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_5,param_7);
  uStack6 = CONCAT22(param_2,param_1);
  uVar7 = param_2 | param_1;
  if (uVar7 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    paStack10 = (astruct_294 *)CONCAT22(uVar7,param_1);
    uVar1 = (u32)(param_1 + 0x2a);
    if (uVar1 != param_6) {
      uVar6 = param_6;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1);
      uVar4 = uVar6;
      puVar3 = (u32 *)(uVar6 & 0xffff | (u32)uVar7 << 0x10);
      uVar8 = uVar7;
      uVar5 = uVar4;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_6);
      puStack22 = (u32 *)CONCAT22(uVar8,uVar5);
      if (((puVar3 == NULL) || ((uVar8 | uVar5) == 0x0)) || (*(i32 *)(uVar5 + 0x200) != *(i32 *)(uVar4 + 0x200))) {
        return;
      }
      ppcVar2 = (code **)((int)*puVar3 + 0x18);
      (**ppcVar2)(0x1030,uVar4,uVar7,uStack6);
      ppcVar2 = (code **)((int)*puStack22 + 0x8);
      (**ppcVar2)();
      pass1_1030_73ee(extraout_DX,paStack10,param_6);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 pass1_1028_c314(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut u16,mut param_6: u16 ,
                      mut param_7: u16 ,mut param_8: u32)

{
  u32 *puVar1;
  let mut local_14: i16;
  let mut local_12: i16;
  let mut local_10: i16;
  let mut local_e: i16;
  let mut local_c: u32;
  let mut uStack8: u16;
  let mut iStack6: i16;
  let mut uStack4: u16;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_8);
  iStack6 = param_1;
  uStack4 = param_2;
  puVar1 = (u32 *)pass1_1030_5b5c(param_1,param_2);
  local_c = *puVar1;
  uStack8 = ((int)puVar1 + 0x4);
  pass1_1008_3e94(param_5,(u16 *)CONCAT22(0x1050,&local_10),(char *)CONCAT22(0x1050,&local_e));
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_14),(char *)CONCAT22(0x1050,&local_12))
  ;
  if ((((0x1 < local_e) && (0x1 < local_10)) && (local_e < local_12 + -0x1)) && (local_10 < local_14 + -0x1)) {
    return 0x1;
  }
  PTR_LOOP_1050_50ca = (u8 *)0x6b8;
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_c3aa(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16,mut param_4: u32,mut param_5: u32)

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  u8 *puVar7;
  let mut uVar8: u32;
  let mut uVar9: u32;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar12;
  let mut uVar13: u32;
  let mut uVar14: u16;
  let mut uVar15: u32;
  u32 *puVar16;
  u32 *puVar17;
  let mut in_stack_0000fe72: u16;
  let mut in_stack_0000ff96: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ffa0: u16;
  let mut puVar18: *mut u16;
  u8 uVar19;
  u8 uVar20;
  let mut uVar21: u16;
  let mut uVar22: u16;
  let mut uStack40: u32;
  let mut uStack36: u32;
  u32 *puStack32;
  u8 *puStack24;
  u8 local_4 [0x2];

  uVar10 = ((u32)in_EDX >> 0x10);
  uVar15 = pass1_1030_bcae(local_4,&DAT_1050_1050);
  paVar12 = (astruct_57 *)CONCAT22(uVar10,(int)(uVar15 >> 0x10));
  iVar2 = (int)uVar15;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4);
  uVar10 = SUB42(paVar12,0x0);
  uVar8 = (u32)(iVar2 + 0x10);
  uVar19 = SUB41(param_3,0x0);
  uVar20 = (u8)((u32)param_3 >> 0x8);
  uVar14 = ((u32)param_3 >> 0x10);
  uVar15 = param_5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar8);
  puStack24 = local_4;
  pass1_1030_bcde(puStack24,&DAT_1050_1050,uVar8 & 0xffff | (long)paVar12 << 0x10,
                  (u16 *)CONCAT22(uVar14,CONCAT11(uVar20,uVar19)),uVar15);
  if ((int)puStack24 < 0x0) {
    PTR_LOOP_1050_50ca = (u8 *)0x6af;
    return;
  }
  if (0x1e < (int)puStack24) {
    uVar3 = 0x87;
    puVar16 = mixed_1010_20ba(paVar12,_u16_1050_0ed0,(u8 **)0x870009,in_stack_0000fe72,in_stack_0000ff96,
                              in_stack_0000ff9c,in_stack_0000ffa0);
    uVar3 = pass1_1010_65d0((u32)puVar16,uVar3);
    if (uVar3 == 0x0) {
      puVar17 = pass1_1008_c6fa(_u16_1050_06e0,0x15);
      uVar15 = (u32)puVar17 >> 0x10;
      uVar4 = puVar17;
      uVar14 = SUB42(&u16_1050_1038,0x0);
      pass1_1038_4d6e(uVar4,(u8 *)((u32)puVar17 >> 0x10),(astruct_691 *)CONCAT22(uVar10,iVar2),puVar17);
      uVar10 = uVar15;
      puStack32 = (u32 *)CONCAT22(uVar10,uVar4);
      ppcVar1 = (code **)((int)*puStack32 + 0x10);
      uVar13 = uVar15;
      uVar5 = uVar4;
      uVar22 = uVar4;
      (**ppcVar1)((int)&u16_1050_1038,uVar4,uVar10);
      uStack36 = CONCAT22((int)uVar13,uVar5);
      uStack40 = 0x0;
      while( true ) {
        if (uStack36 <= uStack40) {
          if (puStack32 != NULL) {
            ppcVar1 = (code **)*puStack32;
            (**ppcVar1)(uVar14,uVar4,(char)uVar15,0x1,uVar22,uVar10,puStack32,puStack32);
          }
          PTR_LOOP_1050_50ca = (u8 *)0x6b6;
          PTR_LOOP_1050_50cc = puStack24 + -0x1e;
          return;
        }
        uVar19 = (u8)param_5;
        uVar20 = (u8)(param_5 >> 0x8);
        uVar9 = uStack36;
        puVar18 = param_3;
        uVar21 = (int)(param_5 >> 0x10);
        pass1_1030_1d58((u32)puStack32);
        uVar6 = uVar9;
        uVar11 = uVar13;
        puVar7 = local_4;
        uVar14 = 0x1030;
        pass1_1030_bcde(puVar7,&DAT_1050_1050,uVar9 & 0xffff | uVar13 << 0x10,puVar18,
                        CONCAT22(uVar21,CONCAT11(uVar20,uVar19)));
        if ((0x0 < (int)puVar7) && ((int)puVar7 < 0x1f)) break;
        if ((int)puVar7 < (int)puStack24) {
          puStack24 = puVar7;
        }
        uStack40 += 0x1;
      }
      if (puStack32 == NULL) {
        return;
      }
      ppcVar1 = (code **)*puStack32;
      (**ppcVar1)(0x1030,uVar4,(char)uVar15,0x1,uVar22,uVar10,puStack32,puStack32,uVar6,uVar11);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_c522(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16,mut param_4: u32,i32 param_5)

{
  let mut iVar1: i16;
  u8 *puVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u32;
  u8 local_4 [0x2];

  uVar5 = pass1_1030_bcae(local_4,&DAT_1050_1050);
  uVar4 = (uVar5 >> 0x10);
  iVar1 = (int)uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4);
  uVar3 = (u32)(iVar1 + 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3);
  puVar2 = local_4;
  pass1_1030_bcde(puVar2,&DAT_1050_1050,uVar3 & 0xffff | (u32)uVar4 << 0x10,param_3,param_5);
  if ((int)puVar2 < 0x0) {
    PTR_LOOP_1050_50ca = (u8 *)0x6af;
  }
  else {
    if ((int)puVar2 < 0x1f) {
      return;
    }
    PTR_LOOP_1050_50ca = (u8 *)0x6b6;
    PTR_LOOP_1050_50cc = puVar2 + -0x1e;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 pass1_1028_c5a6(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16,param_6: *mut u16,i32 param_7)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut iStack14: i16;
  astruct_419 *paStack10;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_6,param_7);
  uVar2 = param_2 | param_1;
  if (uVar2 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    paStack10 = (astruct_419 *)CONCAT22(uVar2,param_1);
    iVar1 = ((int)param_6 + 0x4);
    iStack14 = 0x7a;
    if (0x0 < iVar1) {
      iVar1 = param_5 + -0x7b;
      if (iVar1 == 0x0) {
        param_5 = 0x7e;
      }
      else {
        iVar1 = param_5 + -0x7c;
        if (iVar1 == 0x0) {
          param_5 = 0x7d;
        }
      }
      iStack14 = 0x7f;
    }
    if (paStack10 != NULL) {
      uVar3 = struct_op_1030_73a8(paStack10,iVar1,uVar2);
      if ((uVar3 != 0x0) && ((iVar1 = ((int)uVar3 + 0xc), iVar1 == iStack14 || (iVar1 == param_5)))) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps

BOOL16 pass1_1028_c64a(mut param_1: u32,u32 *param_2,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,i32 param_6)

{
  BOOL16 BVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  u8 local_e [0x2];
  let mut local_c: i16;
  let mut local_a: i16;
  let mut local_8: u32;
  let mut uStack4: u16;

  local_8 = *param_2;
  uStack4 = (param_2 + 0x1);
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,local_e),
                  (u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_a));
  local_8 = local_8 & 0xffff | (u32)(local_c - 0x1) << 0x10;
  uVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  BVar1 = pass1_1028_c5a6(&local_8,param_3,uVar2,uVar3,0x7b,(u16 *)CONCAT22(0x1050,&local_8),param_6);
  if (BVar1 == 0x0) {
    local_8 = local_8 & 0xffff | (u32)(local_c + 0x1) << 0x10;
    BVar1 = pass1_1028_c5a6(&local_8,param_3,uVar2,uVar3,0x7b,(u16 *)CONCAT22(0x1050,&local_8),param_6);
    if (BVar1 == 0x0) {
      local_8._0_2_ = local_a + -0x1;
      local_8 = local_c;
      BVar1 = pass1_1028_c5a6(&local_8,param_3,uVar2,uVar3,0x7c,(u16 *)CONCAT22(0x1050,&local_8),param_6);
      if (BVar1 == 0x0) {
        local_8 = CONCAT22(local_8,local_a + 0x1);
        BVar1 = pass1_1028_c5a6(&local_8,param_3,uVar2,uVar3,0x7c,(u16 *)CONCAT22(0x1050,&local_8),param_6);
        if (BVar1 == 0x0) {
          return BVar1;
        }
      }
    }
  }
  return 0x1;
}
pub fn pass1_1028_c724(param_1: *mut astruct_295)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  astruct_295 *iVar3;
  let mut uVar3: u16;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar3 = (astruct_295 *)param_1;
  uVar2 = iVar3->field20_0x14;
  if (((int)uVar2 + 0xac) != 0x0) {
    return;
  }
  uVar2 = iVar3->field20_0x14;
  uVar1 = ((int)uVar2 + 0xa6);
  if (uVar1 == 0xd) {
    uVar2 = iVar3->field20_0x14;
    ((int)uVar2 + 0xac) = 0x1;
    goto LAB_1028_c770;
  }
  if (uVar1 < 0xe) {
    if ((char)uVar1 == '\0') goto LAB_1028_c770;
    if ((char)uVar1 == '\a') {
      uVar2 = iVar3->field20_0x14;
      ((int)uVar2 + 0xac) = 0xa;
      goto LAB_1028_c770;
    }
  }
  uVar2 = iVar3->field20_0x14;
  ((int)uVar2 + 0xac) = 0x5;//
LAB_1028_c770:
  uVar2 = iVar3->field20_0x14;
  if (((int)uVar2 + 0xac) == 0x0) {
    uVar2 = iVar3->field20_0x14;
    if (((int)uVar2 + 0xa8) != 0x0) {
      uVar2 = iVar3->field20_0x14;
      ((int)uVar2 + 0xac) = 0x1;
    }
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_c7b6(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut u16,i32 param_5)

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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_c89c(mut param_1: i16,param_2: *mut astruct_15,param_3: *mut u16,u32 *param_4)

{
  u32 *puVar1;
  let mut extraout_DX: u16;
  let mut uVar2: u16;
  u32 local_16 [0x3];
  i32 lStack10;
  let mut uStack6: u32;

  pass1_1028_b58e(param_2);
  uStack6 = CONCAT22(extraout_DX,param_1);
  lStack10 = *(i32 *)(param_1 + 0x8);
  puVar1 = local_16;
  uVar2 = extraout_DX;
  pass1_1030_64ce(puVar1,extraout_DX,_PTR_LOOP_1050_5740,param_3,lStack10,(u32 *)CONCAT22(0x1050,puVar1));
  *param_4 = *puVar1;
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1028_c8ee(param_1: *mut astruct_15,mut param_2: i16,param_3: *mut u16)

{
  let mut local_8: u16;
  let mut local_6: u32;

  pass1_1008_3eb4((astruct_615 *)param_3,(u16 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,&local_6),
                  (u16 *)CONCAT22(0x1050,(int)&local_6 + 0x2));
  if (param_2 == 0x1) {
    local_8 += 0x1;
  }
  else if (param_2 == 0x2) {
    local_6 = local_6 & 0xffff0000 | (u32)((int)local_6 - 0x1);
  }
  else if (param_2 == 0x3) {
    local_6 = local_6 & 0xffff0000 | (u32)((int)local_6 + 0x1);
  }
  else if (param_2 == 0x4) {
    local_6 = local_6 & 0xffff | (u32)(local_6 + 0x1) << 0x10;
  }
  else if (param_2 == 0x5) {
    local_6 = local_6 & 0xffff | (u32)(local_6 - 0x1) << 0x10;
  }
  pass1_1008_3e76(param_3,local_8,local_6,(local_6 >> 0x10));
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_c952(param_1: *mut astruct_15)

{
  let mut uVar2: u32;
  astruct_600 *uVar3;
  BOOL16 BVar3;
  let mut uVar4: u16;
  StructD *pSVar5;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  astruct_15 *pstruct15_9;
  astruct_15 *uVar10;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uStack30: u32;
  let mut uStack16: u16;
  let mut uStack14: u16;
  StructD *uVar1;

  uVar10 = (astruct_15 *)((u32)param_1 >> 0x10);
  pstruct15_9 = (astruct_15 *)param_1;
  uVar1 = pstruct15_9->field16_0x14;
  uVar3 = (astruct_600 *)uVar1;
  uVar7 = ((int)&pstruct15_9->field16_0x14 + 0x2) | uVar3;
  if (uVar7 != 0x0) {
    pSVar5 = uVar1;
    pass1_1028_b58e(param_1);
    uVar2 = (u32)((int)pSVar5 + 0x2e);
    uStack14 = uVar2;
    if (((((int)pSVar5 + 0x30) | uStack14) != 0x0) &&
       (uVar11 = (uVar2 >> 0x10), (uStack14 + 0x206) == 0x0)) {
      BVar3 = pass1_1008_c6ae(_u16_1050_06e0,pstruct15_9->field10_0xc,0x32);
      if (BVar3 == 0x0) {
        BVar3 = pass1_1008_c6ae(_u16_1050_06e0,pstruct15_9->field10_0xc,0x33);
        if ((BVar3 != 0x0) && ((int)((qword)*_PTR_LOOP_1050_65e2 % 0x5) == 0x0)) {
          return;
        }
      }
      else if ((int)((qword)*_PTR_LOOP_1050_65e2 % 0xa) == 0x0) {
        return;
      }
      uVar12 = ((u32)uVar1 >> 0x10);
      if ((uStack14 + 0x204) == 0x0) {
        for (uStack16 = 0x0; (int)uStack16 < 0x25; uStack16 += 0x1) {
          uStack30 = (u32)(&uVar3->field_0x0 + uStack16 * 0x4);
          uVar7 = uStack30;
          uVar9 = (&uVar3->field_0x2 + uStack16 * 0x4) | uVar7;
          if (uVar9 != 0x0) {
            uVar6 = uStack30;
            empty_1038_540a();
            uStack30 = (uStack30 >> 0x10);
            if ((uVar6 & 0xffff | (u32)uVar9 << 0x10) < uStack30) {
              uVar4 = uVar7 - uVar6;
              iVar8 = (uStack30 - uVar9) - (uVar7 < uVar6);
              pass1_1038_52b8(uVar2,CONCAT22(iVar8,uVar4),0x21);
              uStack30 = CONCAT22((uStack30 - iVar8) - (uVar7 < uVar4),uVar7 - uVar4);
            }
            if ((uStack30 | uStack30) != 0x0) {
              pass1_1038_52b8(uVar2,uStack30,uStack16);
            }
          }
        }
      }
      else {
        uVar7 = uVar3->field140_0x8c;
        uVar9 = uVar3->field141_0x8e;
        if ((uVar9 | uVar7) != 0x0) {
          pass1_1038_52b8(uVar2,CONCAT22(uVar9,uVar7),0x23);
        }
        uVar7 = uVar3->field142_0x90;
        uVar9 = uVar3->field143_0x92;
        if ((uVar9 | uVar7) != 0x0) {
          pass1_1038_52b8(uVar2,CONCAT22(uVar9,uVar7),0x24);
          return;
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_cb04(param_1: *mut astruct_15)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  astruct_398 *paVar3;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut uVar6: u32;
  i32 lVar7;
  u8 *puVar8;
  let mut in_EDX: u32;
  astruct_57 *paVar9;
  let mut iVar10: i16;
  let mut unaff_SI: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut bVar13: bool;
  astruct_27 *paVar14;
  let mut in_stack_0000fe70: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ff9e: u16;
  u8 *puStack52;
  let mut uStack38: u16;
  u8 *puStack36;
  let mut iStack22: i16;
  let mut uStack18: u16;
  u8 *puStack16;
  let mut uStack14: u16;

  uVar1 = (u32)((int)param_1 + 0x14);
  if (uVar1 != 0x0) {
    uVar5 = uVar1;
    pass1_1028_b58e(param_1);
    paVar3 = (astruct_398 *)(uVar5 & 0xffff | in_EDX << 0x10);
    uVar2 = (u32)((int)uVar5 + 0x2e);
    uStack18 = ((int)uVar5 + 0x30);
    paVar9 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uStack18);
    uStack14 = uVar2;
    uStack18 |= uStack14;
    if (uStack18 != 0x0) {
      uVar11 = ((u32)uVar2 >> 0x10);
      if ((uStack14 + 0x206) != 0x0) {
        return;
      }
      iVar10 = (int)uVar1;
      uVar12 = (uVar1 >> 0x10);
      if ((uStack14 + 0x204) != 0x0) {
        uVar2 = (u32)(iVar10 + 0x8c);
        uVar6 = uVar2;
        empty_1038_540a();
        puStack36 = (u8 *)((u32)uVar2 >> 0x10);
        puVar8 = (u8 *)paVar9;
        if ((puVar8 <= puStack36) &&
           ((uVar4 = uVar6, uStack38 = uVar2, puVar8 < puStack36 || (uVar4 < uStack38)))) {
          pass1_1030_7d7c(uVar4,puVar8,paVar3,uStack38 - uVar4,
                          CONCAT22(0x23,puStack36 + (-(uStack38 < uVar4) - (int)puVar8)));
          paVar14 = (astruct_27 *)
                    mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2b),in_stack_0000fe70,
                                    in_stack_0000ff94,in_stack_0000ff9a,in_stack_0000ff9e);
          paVar9 = (astruct_57 *)((u32)paVar14 >> 0x10);
          pass1_1010_043a(paVar14,*(i32 *)(uStack14 + 0x4),0x12);
        }
        puVar8 = (u8 *)paVar9;
        uVar2 = (u32)(iVar10 + 0x90);
        uVar6 = uVar2;
        empty_1038_540a();
        puStack36 = (u8 *)((u32)uVar2 >> 0x10);
        if ((puVar8 <= puStack36) &&
           ((uVar4 = uVar6, uStack38 = uVar2, puVar8 < puStack36 || (uVar4 < uStack38)))) {
          pass1_1030_7d7c(uVar4,puVar8,paVar3,uStack38 - uVar4,
                          CONCAT22(0x24,puStack36 + (-(uStack38 < uVar4) - (int)puVar8)));
        }
        return;
      }
      empty_1038_540a();
      puStack16 = (u8 *)paVar9;
      for (iStack22 = 0x11; iStack22 < 0x25; iStack22 += 0x1) {
        uVar1 = (u32)(iStack22 * 0x4 + iVar10);
        uVar5 = uVar1;
        empty_1038_540a();
        uVar5 = uVar5 & 0xffff | (long)paVar9 << 0x10;
        puStack52 = (u8 *)(uVar1 >> 0x10);
        paVar9 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | ZEXT24(puStack52));
        if (uVar5 < uVar1) {
          if ((((iStack22 == 0x23) || (iStack22 == 0x24)) || (puStack16 < puStack52)) ||
             ((uVar4 = uVar1, puStack16 <= puStack52 && (uStack18 < uVar4)))) {
            lVar7 = uVar1 - uVar5;
            uVar4 = lVar7;
            pass1_1030_7d7c(uVar4,puStack52,paVar3,uVar4,CONCAT22(iStack22,(int)((u32)lVar7 >> 0x10)));
            if (iStack22 == 0x23) {
              paVar14 = (astruct_27 *)
                        mixed_1010_20ba(paVar9,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2b),in_stack_0000fe70,
                                        in_stack_0000ff94,in_stack_0000ff9a,in_stack_0000ff9e);
              paVar9 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | (u32)paVar14 >> 0x10);
              pass1_1010_043a(paVar14,*(i32 *)(uStack14 + 0x4),0x12);
            }
          }
          else {
            bVar13 = uStack18 < uVar4;
            uStack18 -= uVar4;
            puStack16 = puStack16 + (-bVar13 - (int)puStack52);
          }
        }
      }
      return;
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_ccd0(param_1: *mut astruct_15,param_2: *mut u16)

{
  code **ppcVar1;
  let mut puVar2: *mut u16;
  u8 *puVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  astruct_57 *in_EDX;
  let mut in_stack_0000fd28: u16;
  let mut in_stack_0000fe4c: u16;
  let mut in_stack_0000fe52: u16;
  let mut in_stack_0000fe56: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut in_stack_0000fe80: u16;
  let mut local_178: u16;
  let mut uStack374: u16;
  let mut iStack84: i16;
  let mut uStack72: u16;
  let mut uStack64: u16;
  let mut iStack62: i16;
  let mut uStack60: u32;
  u32 *puStack56;
  let mut uStack52: u32;
  u32 *puStack48;
  u8 local_2c [0xc];
  let mut local_20: i16;
  let mut local_1e: i16;
  let mut uStack28: u32;
  let mut uStack24: u32;
  let mut uStack20: u32;
  let mut iStack16: i16;
  let mut iStack14: i16;
  let mut uStack12: u16;
  let mut uStack10: u16;
  let mut local_8: u16;
  let mut local_6: i16;
  let mut local_4: i16;

  puVar2 = &local_8;
  pass1_1008_3eb4((astruct_615 *)param_2,(u16 *)CONCAT22(0x1050,puVar2),(u16 *)CONCAT22(0x1050,&local_6),
                  (u16 *)CONCAT22(0x1050,&local_4));
  pass1_1028_b58e(param_1);
  uVar7 = in_EDX;
  uStack20 = CONCAT22(uVar7,puVar2);
  uStack24 = (u32)(puVar2 + 0x17);
  uStack28 = (u32)((int)uStack24 + 0x4);
  pass1_1028_c1f8((int)&local_20,uVar7,param_1,(u16 *)CONCAT22(0x1050,&local_20),(u16 *)CONCAT22(0x1050,&local_1e)
                 );
  uStack10 = local_4 - 0x1;
  iStack14 = local_4 + 0x1;
  uStack12 = local_6 - 0x1;
  iStack16 = local_6 + 0x1;
  if ((int)uStack10 < 0x0) {
    uStack10 = 0x0;
  }
  if (local_1e <= iStack14) {
    iStack14 = local_1e + -0x1;
  }
  if ((int)uStack12 < 0x0) {
    uStack12 = 0x0;
  }
  if (local_20 <= iStack16) {
    iStack16 = local_20 + -0x1;
  }
  pass1_1008_6c90((u16 *)CONCAT22(0x1050,local_2c));
  pass1_1008_6cec((u16 *)CONCAT22(0x1050,local_2c),local_8,CONCAT22(iStack14,iStack16),local_8,
                  CONCAT22(uStack10,uStack12));
  puStack48 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fe80,0x2f),in_stack_0000fd28,
                              in_stack_0000fe4c,in_stack_0000fe52,in_stack_0000fe56);
  uVar5 = ((u32)puStack48 >> 0x10);
  uStack52 = (u32)((int)puStack48 + 0x20);
  puVar3 = local_2c;
  pass1_1030_6522(_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar3),uStack52);
  puStack56 = (u32 *)CONCAT22(uVar5,puVar3);
  if ((uVar5 | puVar3) != 0x0) {
    uStack60 = 0x0;
    iStack62 = 0x0;
    for (uStack64 = uStack12; (int)uStack64 <= iStack16; uStack64 += 0x1) {
      for (uStack72 = uStack10; iVar4 = iStack62, (int)uStack72 <= iStack14; uStack72 += 0x1) {
        iVar6 = iStack62 >> 0xf;
        ppcVar1 = (code **)((int)*puStack56 + 0x4);
        iStack62 = iStack62 + 0x1;
        (**ppcVar1)(0x1030,(int)puStack56,(int)((u32)puStack56 >> 0x10),iVar4,iVar6);
        uStack60 = CONCAT22(iVar6,iVar4);
        uStack60._3_1_ = (char)(iVar6 >> 0x8);
        if (uStack60._3_1_ == '\0') {
          iStack84 = iVar4;
          if (iVar4 == 0x7) {
            pass1_1008_3e76(param_2,local_8,uStack64,uStack72);
            uVar10 = uStack52;
            uVar11 = (uStack52 >> 0x10);
            uVar8 = uStack28;
            uVar9 = ((u32)uStack28 >> 0x10);
            uVar7 = 0x6;
          }
          else if (iVar4 == 0x8) {
            pass1_1008_3e76(param_2,local_8,uStack64,uStack72);
            uVar10 = uStack52;
            uVar11 = (uStack52 >> 0x10);
            uVar8 = uStack28;
            uVar9 = ((u32)uStack28 >> 0x10);
            uVar7 = 0x7;
          }
          else {
            if (iVar4 != 0x9) goto LAB_1028_ce2c;
            pass1_1008_3e76(param_2,local_8,uStack64,uStack72);
            uVar10 = uStack52;
            uVar11 = (uStack52 >> 0x10);
            uVar8 = uStack28;
            uVar9 = ((u32)uStack28 >> 0x10);
            uVar7 = 0x8;
          }
          struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,&local_178),0x0,0x0,uVar7,(u32 *)param_2,
                              ((u32)param_2 >> 0x10),CONCAT22(uVar9,uVar8),CONCAT22(uVar11,uVar10));
          fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_178));
          local_178 = 0x389a;
          uStack374 = 0x1008;
        }//
LAB_1028_ce2c:
      }
    }
  }
  return;
}



u16 pass1_1028_ced2(param_1: *mut astruct_15)

{
  astruct_15 *uVar1;
  let mut bVar1: bool;
  let mut bVar2: bool;
  astruct_398 *paVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;

  uVar1 = (astruct_15 *)((u32)param_1 >> 0x10);
  bVar1 = (*(u8 *)((int)param_1 + 0x1a) & 0x2) == 0x0;
  if (bVar1) {
    uVar5 = 0x0;
    uVar6 = 0x23;
    uVar4 = 0x1;
    paVar3 = (astruct_398 *)pass1_1028_b58e((astruct_15 *)((u32)param_1 & 0xffff | ZEXT24(uVar1) << 0x10));
    pass1_1030_7d7c(paVar3,(u8 *)((u32)paVar3 >> 0x10),paVar3,uVar4,CONCAT22(uVar6,uVar5));
  }
  bVar2 = (*(u8 *)((int)param_1 + 0x1a) & 0x1) == 0x0;
  if (bVar2) {
    uVar5 = 0x0;
    uVar6 = 0xe;
    uVar4 = 0x1;
    paVar3 = (astruct_398 *)pass1_1028_b58e((astruct_15 *)((u32)param_1 & 0xffff | ZEXT24(uVar1) << 0x10));
    pass1_1030_7d7c(paVar3,(u8 *)((u32)paVar3 >> 0x10),paVar3,uVar4,CONCAT22(uVar6,uVar5));
  }
  if (bVar2 || bVar1) {
    pass1_1028_bdac(param_1,0x6);
    return 0x0;
  }
  return 0x1;
}



StructD * pass1_1028_cf44(StructD *param_1,u8 param_2)

{
  pass1_1028_b418(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1028_cfd2(u32 *param_1,mut param_2: u32)

{
  *param_1 = param_2;
  (u32)((int)param_1 + 0x4) = 0x0;
  return;
}
pub fn pass1_1028_cff2(mut param_1: u32)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut uVar4: u16;

  uVar4 = (param_1 >> 0x10);
  puVar1 = (u32 *)((int)param_1 + 0x4);
  uVar2 = ((int)param_1 + 0x6);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  return;
}
pub fn pass1_1028_d01a(u32 *param_1)

{
  u32 *puVar1;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut extraout_DX: u16;
  u32 *puStack14;

  puVar1 = (u32 *)*(u32*)*param_1;
  puStack14 = puVar1;
  while( true ) {
    uVar4 = puStack14;
    fn_ptr_1028_d728((u32)puVar1);
    puStack14 = (u32 *)CONCAT22(extraout_DX,uVar4);
    if ((extraout_DX | uVar4) == 0x0) break;
    uVar3 = *puStack14;
    ppcVar2 = (code **)uVar3 + 0x2;
    (**ppcVar2)();
    if (puStack14 != NULL) {
      ppcVar2 = (code **)uVar3;
      (**ppcVar2)();
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_d078(mut param_1: u32,mut param_2: u32)

{
  code **ppcVar1;
  let mut in_EDX: u32;
  astruct_57 *paVar2;
  let mut uVar3: u32;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  let mut uVar7: u32;
  u8 local_16 [0x4];
  u32 *puStack18;
  let mut uStack16: u16;
  let mut uStack14: u32;
  let mut uStack10: u16;
  let mut uStack8: u16;
  u32 *puStack6;
  let mut uStack4: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  puStack6 = (u32 *)(iVar4 + 0x4);
  uStack16 = (iVar4 + 0x6);
  paVar2 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uStack16);
  uStack14 = CONCAT22(uStack16,puStack6);
  puStack18 = puStack6;
  if ((uStack16 | puStack6) != 0x0) {
    ppcVar1 = (code **)*puStack6;
    (**ppcVar1)();
  }
  mem_op_1000_179c(0x1c,paVar2);
  uStack16 = paVar2;
  uVar7 = (u32)paVar2 & 0xffff0000;
  uVar3 = uVar7 | (uStack16 | puStack6);
  puStack18 = puStack6;
  if ((uStack16 | puStack6) == 0x0) {
    puStack6 = NULL;
  }
  else {
    struct_op_1008_8e9e((astruct_78 *)CONCAT22(uStack16,puStack6),0x6,0x24);
    uVar7 = uVar3;
  }
  (iVar4 + 0x4) = puStack6;
  (iVar4 + 0x6) = (int)uVar7;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_2);
  uStack4 = uVar7;
  uVar7 &= 0xffff0000;
  if ((u8 *)(uStack4 | puStack6) == NULL) {
    puVar6 = pass1_1018_dcf6((u16 *)CONCAT22(0x1050,local_16));
    uVar7 = pass1_1018_dd1e(local_16,(astruct_57 *)(uVar7 & 0xffff0000 | (u32)puVar6 >> 0x10),local_16,
                            &DAT_1050_1050,0x0,0xa0000);
    pass1_1008_8faa(*(astruct_78 **)(iVar4 + 0x4),uVar7);
    return;
  }
  uVar7 = pass1_1038_565e((u8 *)(uStack4 | puStack6),CONCAT22(uStack4,puStack6));
  uStack8 = (uVar7 >> 0x10);
  uStack10 = uVar7;
  if ((uStack8 | uStack10) != 0x0) {
    pass1_1028_d172(param_1,uVar7 & 0xffff | (u32)uStack8 << 0x10);
  }
  return;
}
pub fn pass1_1028_d172(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_EDX: u32;
  let mut uVar5: u16;
  let mut uVar3: u32;
  let mut puVar6: *mut u16;
  let mut uVar7: u32;
  u8 local_e [0x8];
  u8 local_6 [0x4];
  astruct_57 *paVar4;

  uVar5 = ((u32)in_EDX >> 0x10);
  puVar6 = pass1_1018_dcf6((u16 *)CONCAT22(0x1050,local_6));
  uVar3 = CONCAT22(uVar5,(int)((u32)puVar6 >> 0x10));
  pass1_1008_5784((char *)CONCAT22(0x1050,local_e),param_2);
  while( true ) {
    uVar1 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_e));
    uVar2 = uVar3 | uVar1;
    paVar4 = (astruct_57 *)(uVar3 & 0xffff0000 | (u32)uVar2);
    if (uVar2 == 0x0) break;
    uVar7 = pass1_1018_dd1e(local_6,paVar4,local_6,&DAT_1050_1050,0x0,
                            (u32)(uVar1 + 0x4) << 0x10);
    uVar3 = (u32)paVar4 & 0xffff0000 | uVar7 >> 0x10;
    pass1_1008_8faa(*(astruct_78 **)((int)param_1 + 0x4),uVar7);
  }
  return;
}



astruct_97 * struct_op_1028_d1dc(param_1: *mut astruct_97,mut param_2: u16 )

{
  astruct_97 *iVar1;
  let mut uVar1: u16;
  let mut in_stack_0000fffa: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  param_1->offset_0x0 = 0x389a;
  iVar1->segment_0x2 = 0x1008;
  &iVar1->field_0x4 = param_2;
  &iVar1->field_0x6 = 0x0;
  param_1->offset_0x0 = 0x6ad2;
  iVar1->segment_0x2 = 0x1028;
  sys_1000_3f9c((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar1->string_0x8)),s_ctor_1050_5160,in_stack_0000fffa);
  return param_1;
}



u16 FUN_1028_d222(void)

{
  return 0x1;
}



u16 FUN_1028_d228(void)

{
  return 0x1;
}
pub fn struct_1028_d22e(param_1: *mut astruct_57,u32 *param_2,mut param_3: u32)

{
  let mut uVar1: u16;
  StructD *pSVar2;
  let mut uVar3: u16;

  uVar3 = ((u32)param_2 >> 0x10);
  *param_2 = 0x0;
  (u32)((int)param_2 + 0x4) = param_3;
  mem_op_1000_179c(0xc,param_1);
  uVar1 = param_3;
  pSVar2 = (StructD *)(param_1 | uVar1);
  if (pSVar2 == NULL) {
    *param_2 = 0x0;
  }
  else {
    struct_1028_d59c(pSVar2,(astruct_158 *)(param_3 & 0xffff | (long)param_1 << 0x10));
    param_2 = uVar1;
    *(StructD **)((int)param_2 + 0x2) = pSVar2;
  }
  return;
}
pub fn pass1_1028_d282(param_1: *mut astruct_446)

{
  char *pcStack6;
  astruct_446 *uVar1;
  astruct_446 *uVar2;

  uVar1 = *(astruct_446 **)param_1;
  uVar2 = *(astruct_446 **)((int)param_1 + 0x2);
  pcStack6 = (char *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1028_d658((astruct_446 *)CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack6);
  }
  return;
}
pub fn struct_1028_d2b0(u32 *param_1)

{
  let mut local_10c: u16;
  let mut uStack266: u16;

  struct_1028_9c62((astruct_97 *)CONCAT22(0x1050,&local_10c),0x3e80);
  fn_ptr_1028_d566(param_1,(astruct_97 *)CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((astruct_97 *)CONCAT22(0x1050,&local_10c),0x3a98);
  fn_ptr_1028_d566(param_1,(astruct_97 *)CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((astruct_97 *)CONCAT22(0x1050,&local_10c),0x36b0);
  fn_ptr_1028_d566(param_1,(astruct_97 *)CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((astruct_97 *)CONCAT22(0x1050,&local_10c),0x32c8);
  fn_ptr_1028_d566(param_1,(astruct_97 *)CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((astruct_97 *)CONCAT22(0x1050,&local_10c),0x2ee0);
  fn_ptr_1028_d566(param_1,(astruct_97 *)CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((astruct_97 *)CONCAT22(0x1050,&local_10c),0x2af8);
  fn_ptr_1028_d566(param_1,(astruct_97 *)CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((astruct_97 *)CONCAT22(0x1050,&local_10c),0x2710);
  fn_ptr_1028_d566(param_1,(astruct_97 *)CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
    // just 0x2328
  struct_1028_9c62((astruct_97 *)CONCAT22(0x1050,&local_10c),(int)s_noth_bmp_1050_2321 + 0x7);
  fn_ptr_1028_d566(param_1,(astruct_97 *)CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((astruct_97 *)CONCAT22(0x1050,&local_10c),0x1f40);
  fn_ptr_1028_d566(param_1,(astruct_97 *)CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((astruct_97 *)CONCAT22(0x1050,&local_10c),0x1b58);
  fn_ptr_1028_d566(param_1,(astruct_97 *)CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((astruct_97 *)CONCAT22(0x1050,&local_10c),0x1770);
  fn_ptr_1028_d566(param_1,(astruct_97 *)CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((astruct_97 *)CONCAT22(0x1050,&local_10c),0x1388);
  fn_ptr_1028_d566(param_1,(astruct_97 *)CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((astruct_97 *)CONCAT22(0x1050,&local_10c),0xfa0);
  fn_ptr_1028_d566(param_1,(astruct_97 *)CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((astruct_97 *)CONCAT22(0x1050,&local_10c),0xbb8);
  fn_ptr_1028_d566(param_1,(astruct_97 *)CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  struct_1028_9c62((astruct_97 *)CONCAT22(0x1050,&local_10c),0x3e8);
  fn_ptr_1028_d566(param_1,(astruct_97 *)CONCAT22(0x1050,&local_10c));
  local_10c = 0x389a;
  uStack266 = 0x1008;
  pass1_1028_d6b2(*param_1);
  return;
}



BOOL16 pass1_1028_d52c(u32 *param_1,mut param_2: u32,u32 *param_3)

{
  code **ppcVar1;
  let mut iVar2: i16;
  BOOL16 BVar3;

  ppcVar1 = (code **)((int)*param_3 + 0x8);
  iVar2 = (**ppcVar1)();
  if (iVar2 != 0x0) {
    BVar3 = pass1_1028_d776(*param_1,param_2,param_3);
    if (BVar3 != 0x0) {
      return 0x1;
    }
  }
  return 0x0;
}



BOOL16 fn_ptr_1028_d566(u32 *param_1,param_2: *mut astruct_97)

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;

  ppcVar1 = (code **)((int)(u32)param_2 + 0x8);
  iVar2 = (**ppcVar1)();
  if (iVar2 != 0x0) {
    uVar3 = fn_ptr_1028_d742(*param_1,(u32 *)param_2);
    if (uVar3 != 0x0) {
      return 0x1;
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn struct_1028_d59c(StructD *param_1,param_2: *mut astruct_158)

{
  let mut puVar1: *mut u16;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  astruct_158 *iVar5;
  astruct_158 *uVar5;
  let mut puStack14: *mut u16;
  astruct_57 *paVar6;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar5 = (astruct_158 *)((u32)param_2 >> 0x10);
  iVar5 = (astruct_158 *)param_2;
  (u32)param_2 = 0x0;
  iVar5->field3_0x4 = NULL;
  iVar5->field4_0x8 = NULL;
  puVar2 = (u16 *)*_u16_1050_5748;
  (u16*)param_2 = puVar2;
  mem_op_1000_179c(0xc,paVar5);
  puVar1 = (u16 *)((u32)puVar2 & 0xffff | (long)paVar5 << 0x10);
  uVar3 = paVar5 | puVar2;
  paVar6 = (astruct_57 *)((u32)paVar5 & 0xffff0000 | (u32)uVar3);
  if (uVar3 == 0x0) {
    iVar5->field3_0x4 = NULL;
  }
  else {
    set_struct_1008_574a((astruct_57 *)((u32)puVar2 & 0xffff | (long)paVar5 << 0x10));
    *puVar1 = 0xd804;
    (puVar2 + 0x2) = 0x1028;
    iVar5->field3_0x4 = puVar1;
    puVar2 = puVar1;
  }
  uVar3 = puVar2;
  mem_op_1000_179c(0xc,paVar6);
  uVar4 = paVar6;
  puStack14 = (u16 *)CONCAT22(uVar4,uVar3);
  if ((uVar4 | uVar3) == 0x0) {
    iVar5->field4_0x8 = NULL;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(uVar4,uVar3));
    *puStack14 = 0xd804;
    (uVar3 + 0x2) = 0x1028;
    iVar5->field4_0x8 = puStack14;
  }
  return;
}
pub fn pass1_1028_d658(param_1: *mut astruct_446)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  astruct_446 *iVar4;
  astruct_446 *uVar4;

  uVar4 = (astruct_446 *)((u32)param_1 >> 0x10);
  iVar4 = (astruct_446 *)param_1;
  puVar1 = iVar4->field4_0x4;
  uVar2 = iVar4->field5_0x6;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar4->field6_0x8;
  uVar2 = iVar4->field7_0xa;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  return;
}



u16 pass1_1028_d69e(mut param_1: u32)

{
  let mut uVar1: u32;

  uVar1 = (u32)((int)param_1 + 0x4);
  return ((int)uVar1 + 0x8);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_d6b2(mut param_1: u32)

{
  u32 *puVar1;
  let mut uVar2: u32;
  code **ppcVar3;
  u32 *puVar4;
  let mut uVar5: u16;
  let mut extraout_DX: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;

  uVar2 = *_PTR_LOOP_1050_65e2;
  while( true ) {
    uVar6 = (param_1 >> 0x10);
    uVar7 = pass1_1020_c860((u32)((int)param_1 + 0x8));
    uVar5 = (uVar7 >> 0x10);
    if (((uVar5 | uVar7) == 0x0) || (puVar1 = (u32 *)(uVar7 + 0xc), uVar2 <= *puVar1 && *puVar1 != uVar2))
    break;
    ppcVar3 = (code **)((int)(u32)(u32)((int)param_1 + 0x8) + 0x10);
    uVar7 = uVar2;
    (**ppcVar3)();
    puVar4 = (u32 *)(uVar7 & 0xffff | (u32)extraout_DX << 0x10);
    fn_ptr_1028_d742(param_1,(u32 *)(uVar7 & 0xffff | (u32)extraout_DX << 0x10));
    if (puVar4 != NULL) {
      ppcVar3 = (code **)*puVar4;
      (**ppcVar3)(0x1020,(int)uVar7,extraout_DX,0x1);
    }
  }
  return;
}
pub fn fn_ptr_1028_d728(mut param_1: u32)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x4) + 0x10);
  (**ppcVar1)();
  return;
}



u16 fn_ptr_1028_d742(mut param_1: u32,u32 *param_2)

{
  code **ppcVar1;
  let mut uVar2: u32;

  ppcVar1 = (code **)((int)*param_2 + 0xc);
  uVar2 = (**ppcVar1)();
  pass1_1020_c872((u32)((int)param_1 + 0x4),(u32)((int)uVar2 + 0x4),uVar2);
  return 0x1;
}



BOOL16 pass1_1028_d776(mut param_1: u32,mut param_2: u32,u32 *param_3)

{
  code **ppcVar1;
  let mut uVar2: u32;

  ppcVar1 = (code **)((int)*param_3 + 0xc);
  uVar2 = (**ppcVar1)();
  pass1_1020_c872((u32)((int)param_1 + 0x8),param_2,uVar2);
  return 0x1;
}



BOOL16 pass1_1028_d7a0(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  BOOL16 BVar1;

  BVar1 = write_to_file_1008_7cac(param_3);
  if (BVar1 != 0x0) {
    BVar1 = 0x1;
  }
  return BVar1;
}



i16 read_file_1028_d7ba(u16_t param_1,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  read_file_1008_7cfe((int)param_4,(int)((u32)param_4 >> 0x10),0x8);
  if (param_1 == 0x0) {
    u16_1050_0310 = 0x6d4;
    return param_1;
  }
  return 0x1;
}



StructD * pass1_1028_d7de(StructD *param_1,u8 param_2)

{
  pass1_1008_57c4(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_d81c(param_1: *mut astruct_57,param_2: *mut astruct_136,mut param_3: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  StructD *pSVar3;
  let mut uVar4: u16;
  astruct_57 *paVar6;
  astruct_57 *paVar8;
  astruct_136 *iVar6;
  let mut uVar9: u16;
  let mut in_stack_0000fe88: u16;
  let mut in_stack_0000ffac: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffde: u16;
  StructD *pSVar5;
  astruct_57 *paVar7;

  uVar9 = ((u32)param_2 >> 0x10);
  iVar6 = (astruct_136 *)param_2;
  param_2->field0_0x0 = 0x0;
  iVar6->field1_0x4 = param_3;
  (u32)&iVar6->field44_0x52 = 0x0;
  _PTR_LOOP_1050_65e2 = param_2;
  iVar6->field28_0x32 = 0xec36;
  iVar6->field29_0x34 = 0x1028;
  iVar6->field30_0x36 = 0xecac;
  iVar6->field31_0x38 = 0x1028;
  iVar6->field32_0x3a = 0xed2c;
  iVar6->field33_0x3c = 0x1028;
  iVar6->field34_0x3e = 0xedc4;
  iVar6->field35_0x40 = 0x1028;
  iVar6->field36_0x42 = 0xee54;
  iVar6->field37_0x44 = 0x1028;
  iVar6->field38_0x46 = 0xef00;
  iVar6->field39_0x48 = 0x1028;
  iVar6->field40_0x4a = 0x10b0;
  iVar6->field41_0x4c = 0x1030;
  iVar6->field42_0x4e = 0x1120;
  iVar6->field43_0x50 = 0x1030;
  mem_op_1000_179c(0x8,param_1);
  uVar4 = param_3;
  uVar2 = param_1 | uVar4;
  pSVar5 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)uVar2);
  if (uVar2 != 0x0) {
    pass1_1030_615a(pSVar5,(astruct_137 *)(param_3 & 0xffff | (long)param_1 << 0x10));
  }
  mem_op_1000_179c(0x56c,(astruct_57 *)pSVar5);
  uVar2 = pSVar5 | uVar4;
  paVar7 = (astruct_57 *)((u32)pSVar5 & 0xffff0000);
  paVar6 = (astruct_57 *)((u32)paVar7 | (u32)uVar2);
  if (uVar2 == 0x0) {
    uVar4 = 0x0;
  }
  else {
    struct_1030_44be(paVar6,(astruct_138 *)CONCAT22(pSVar5,uVar4),in_stack_0000fe88,in_stack_0000ffac,
                     in_stack_0000ffb2,in_stack_0000ffb6,in_stack_0000ffde);
    paVar7 = paVar6;
  }
  iVar6->field44_0x52 = uVar4;
  iVar6->field45_0x54 = (u8 *)paVar7;
  mem_op_1000_179c(0x4,paVar7);
  pSVar3 = (StructD *)(paVar7 | uVar4);
  paVar6 = (astruct_57 *)((u32)paVar7 & 0xffff0000 | ZEXT24(pSVar3));
  if (pSVar3 != NULL) {
    struct_1008_bde0(pSVar3,(astruct_139 *)CONCAT22(paVar7,uVar4));
  }
  puVar1 = pass1_1000_4906((StructD *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar6->field_0xa)),NULL,0x24);
  mem_op_1000_179c(0x1c,paVar6);
  uVar4 = paVar6 | puVar1;
  paVar7 = (astruct_57 *)((u32)paVar6 & 0xffff0000 | (u32)uVar4);
  if (uVar4 == 0x0) {
    (u32)&iVar6->field8_0xe = 0x0;
  }
  else {
    struct_1030_11aa((astruct_156 *)CONCAT22(paVar6,puVar1),0x5,0x15);
    iVar6->field8_0xe = puVar1;
    iVar6->field9_0x10 = (u8 *)paVar7;
  }
  mem_op_1000_179c(0x1c,paVar7);
  uVar4 = paVar7 | puVar1;
  paVar6 = (astruct_57 *)((u32)paVar7 & 0xffff0000);
  paVar8 = (astruct_57 *)((u32)paVar6 | (u32)uVar4);
  if (uVar4 == 0x0) {
    puVar1 = NULL;
  }
  else {
    struct_1030_11aa((astruct_156 *)CONCAT22(paVar7,puVar1),0x5,0xa);
    paVar6 = paVar8;
  }
  iVar6->field10_0x12 = puVar1;
  iVar6->field11_0x14 = (u8 *)paVar6;
  mem_op_1000_179c(0x1c,paVar6);
  uVar4 = paVar6 | puVar1;
  paVar7 = (astruct_57 *)((u32)paVar6 & 0xffff0000);
  paVar8 = (astruct_57 *)((u32)paVar7 | (u32)uVar4);
  if (uVar4 == 0x0) {
    puVar1 = NULL;
  }
  else {
    struct_1030_11aa((astruct_156 *)CONCAT22(paVar6,puVar1),0x5,0x19);
    paVar7 = paVar8;
  }
  iVar6->field12_0x16 = puVar1;
  iVar6->field13_0x18 = (u8 *)paVar7;
  mem_op_1000_179c(0x1c,paVar7);
  uVar4 = paVar7 | puVar1;
  paVar6 = (astruct_57 *)((u32)paVar7 & 0xffff0000);
  paVar8 = (astruct_57 *)((u32)paVar6 | (u32)uVar4);
  if (uVar4 == 0x0) {
    puVar1 = NULL;
  }
  else {
    struct_1030_11aa((astruct_156 *)CONCAT22(paVar7,puVar1),0x5,0xa);
    paVar6 = paVar8;
  }
  iVar6->field14_0x1a = puVar1;
  iVar6->field15_0x1c = (u8 *)paVar6;
  mem_op_1000_179c(0x1c,paVar6);
  uVar4 = paVar6 | puVar1;
  paVar7 = (astruct_57 *)((u32)paVar6 & 0xffff0000);
  paVar8 = (astruct_57 *)((u32)paVar7 | (u32)uVar4);
  if (uVar4 == 0x0) {
    puVar1 = NULL;
  }
  else {
    struct_1030_11aa((astruct_156 *)CONCAT22(paVar6,puVar1),0x64,0x1f4);
    paVar7 = paVar8;
  }
  iVar6->field16_0x1e = puVar1;
  iVar6->field17_0x20 = (u8 *)paVar7;
  mem_op_1000_179c(0x1c,paVar7);
  uVar4 = paVar7 | puVar1;
  paVar6 = (astruct_57 *)((u32)paVar7 & 0xffff0000);
  paVar8 = (astruct_57 *)((u32)paVar6 | (u32)uVar4);
  if (uVar4 == 0x0) {
    puVar1 = NULL;
  }
  else {
    struct_1030_11aa((astruct_156 *)CONCAT22(paVar7,puVar1),0x19,0x64);
    paVar6 = paVar8;
  }
  iVar6->field18_0x22 = puVar1;
  iVar6->field19_0x24 = (u8 *)paVar6;
  mem_op_1000_179c(0x1c,paVar6);
  uVar4 = paVar6 | puVar1;
  paVar7 = (astruct_57 *)((u32)paVar6 & 0xffff0000);
  paVar8 = (astruct_57 *)((u32)paVar7 | (u32)uVar4);
  if (uVar4 == 0x0) {
    puVar1 = NULL;
  }
  else {
    struct_1030_11aa((astruct_156 *)CONCAT22(paVar6,puVar1),0x64,0x1f4);
    paVar7 = paVar8;
  }
  iVar6->field20_0x26 = puVar1;
  iVar6->field21_0x28 = (u8 *)paVar7;
  mem_op_1000_179c(0x1c,paVar7);
  uVar4 = paVar7 | puVar1;
  if (uVar4 == 0x0) {
    puVar1 = NULL;
    uVar4 = 0x0;
  }
  else {
    struct_1030_11aa((astruct_156 *)CONCAT22(paVar7,puVar1),0x5,0x2);
  }
  iVar6->field22_0x2a = puVar1;
  iVar6->field23_0x2c = uVar4;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_daba(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  u32 *puVar3;
  code **ppcVar4;
  u32 *puVar5;
  let mut puVar6: *mut u16;
  astruct_447 *iVar5;
  let mut uVar7: u16;
  let mut unaff_CS: u16;
  char *pcStack14;

  puVar6 = _PTR_LOOP_1050_5740;
  if (_PTR_LOOP_1050_5740 != NULL) {
    pass1_1030_61b0(_PTR_LOOP_1050_5740);
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce((char *)puVar6);
  }
  uVar7 = (param_1 >> 0x10);
  iVar5 = (astruct_447 *)param_1;
  uVar1 = iVar5->field66_0x52;
  uVar2 = iVar5->field67_0x54;
  pcStack14 = (char *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1030_4538((u32 *)CONCAT22(uVar2,uVar1));
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(pcStack14);
  }
  if (_PTR_LOOP_1050_5166 != NULL) {
    ppcVar4 = (code **)*_PTR_LOOP_1050_5166;
    (**ppcVar4)(unaff_CS,(int)_PTR_LOOP_1050_5166);
  }
  puVar5 = _u16_1050_06e0;
  _PTR_LOOP_1050_65e2 = 0x0;
  if (_u16_1050_06e0 != NULL) {
    pass1_1008_c626(_u16_1050_06e0);
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce((char *)puVar5);
  }
  puVar3 = iVar5->field14_0xe;
  uVar1 = iVar5->field15_0x10;
  if ((uVar1 | puVar3) != 0x0) {
    ppcVar4 = (code **)*puVar3;
    (**ppcVar4)(unaff_CS,puVar3,uVar1,0x1);
  }
  puVar3 = iVar5->field16_0x12;
  uVar1 = iVar5->field17_0x14;
  if ((uVar1 | puVar3) != 0x0) {
    ppcVar4 = (code **)*puVar3;
    (**ppcVar4)(unaff_CS,puVar3,uVar1,0x1);
  }
  puVar3 = iVar5->field18_0x16;
  uVar1 = iVar5->field19_0x18;
  if ((uVar1 | puVar3) != 0x0) {
    ppcVar4 = (code **)*puVar3;
    (**ppcVar4)(unaff_CS,puVar3,uVar1,0x1);
  }
  puVar3 = iVar5->field20_0x1a;
  uVar1 = iVar5->field21_0x1c;
  if ((uVar1 | puVar3) != 0x0) {
    ppcVar4 = (code **)*puVar3;
    (**ppcVar4)(unaff_CS,puVar3,uVar1,0x1);
  }
  puVar3 = iVar5->field22_0x1e;
  uVar1 = iVar5->field23_0x20;
  if ((uVar1 | puVar3) != 0x0) {
    ppcVar4 = (code **)*puVar3;
    (**ppcVar4)(unaff_CS,puVar3,uVar1,0x1);
  }
  puVar3 = iVar5->field24_0x22;
  uVar1 = iVar5->field25_0x24;
  if ((uVar1 | puVar3) != 0x0) {
    ppcVar4 = (code **)*puVar3;
    (**ppcVar4)(unaff_CS,puVar3,uVar1,0x1);
  }
  puVar3 = iVar5->field26_0x26;
  uVar1 = iVar5->field27_0x28;
  if ((uVar1 | puVar3) != 0x0) {
    ppcVar4 = (code **)*puVar3;
    (**ppcVar4)(unaff_CS,puVar3,uVar1,0x1);
  }
  puVar3 = iVar5->field28_0x2a;
  uVar1 = iVar5->field29_0x2c;
  if ((uVar1 | puVar3) != 0x0) {
    ppcVar4 = (code **)*puVar3;
    (**ppcVar4)(unaff_CS,puVar3,uVar1,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_dc52(param_1: *mut astruct_92,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 )

{
  let mut uVar1: u32;
  astruct_92 *iVar2;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_92 *)param_1;
  param_1 = 0x389a;
  iVar2->field2_0x2 = 0x1008;
  iVar2->field3_0x4 = (u32)((int)_PTR_LOOP_1050_65e2 + (param_4 >> 0x8) * 0x4 + 0xa);
  iVar2->field4_0x8 = 0x1;
  iVar2->field6_0x10 = param_2;
  param_1 = 0x11a6;
  iVar2->field2_0x2 = 0x1030;
  uVar1 = iVar2->field3_0x4;
  iVar2->field5_0xc = (u32)((int)uVar1 + 0xa);
  if (param_2 == 0x0) {
    iVar2->field4_0x8 = iVar2->field5_0xc;
  }
  else {
    iVar2->field4_0x8 = 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn write_to_file_1028_dce2(u16_t param_1,u32 *param_2,u8 *param_3) -> u32

{
  code **ppcVar1;
  BOOL16 BVar2;
  astruct_92 *paVar3;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut iVar5: i16;
  let mut uVar6: u16;
  HFILE16 in_stack_0000ffc0;
  u32 local_26 [0x2];
  u16 local_1e [0x3];
  let mut uStack24: u32;
  astruct_92 local_14;

  BVar2 = write_to_file_1008_7cac(param_3);
  if (BVar2 != 0x0) {
    local_26[0] = *param_2;
    BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_26),(char *)0x4,in_stack_0000ffc0);
    if (BVar2 != 0x0) {
      uVar6 = ((u32)param_2 >> 0x10);
      iVar5 = (int)param_2;
      local_1e[0] = (iVar5 + 0x8);
      BVar2 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_1e),(char *)0x2,in_stack_0000ffc0);
      if (BVar2 != 0x0) {
        ppcVar1 = (code **)((int)*_PTR_LOOP_1050_5166 + 0xc);
        (**ppcVar1)(0x1008,(int)_PTR_LOOP_1050_5166,(int)((u32)_PTR_LOOP_1050_5166 >> 0x10),param_3);
        param_1 = extraout_DX;
        if (BVar2 != 0x0) {
          BVar2 = write_to_file_1008_7cac(param_3);
          if (BVar2 != 0x0) {
            param_1 = write_file_fn_1028_e56c(param_1,iVar5,uVar6,param_3,0x1000000);
            if (BVar2 != 0x0) {
              BVar2 = write_to_file_1008_7cac(param_3);
              if (BVar2 != 0x0) {
                param_1 = write_file_fn_1028_e56c(param_1,iVar5,uVar6,param_3,0x2000000);
                if (BVar2 != 0x0) {
                  BVar2 = write_to_file_1008_7cac(param_3);
                  if (BVar2 != 0x0) {
                    param_1 = write_file_fn_1028_e56c(param_1,iVar5,uVar6,param_3,0x3000000);
                    if (BVar2 != 0x0) {
                      BVar2 = write_to_file_1008_7cac(param_3);
                      if (BVar2 != 0x0) {
                        param_1 = write_file_fn_1028_e56c(param_1,iVar5,uVar6,param_3,0x4000000);
                        if (BVar2 != 0x0) {
                          BVar2 = write_to_file_1008_7cac(param_3);
                          if (BVar2 != 0x0) {
                            param_1 = write_file_fn_1028_e56c(param_1,iVar5,uVar6,param_3,0x5000000);
                            if (BVar2 != 0x0) {
                              BVar2 = write_to_file_1008_7cac(param_3);
                              if (BVar2 != 0x0) {
                                param_1 = write_file_fn_1028_e56c(param_1,iVar5,uVar6,param_3,0x6000000);
                                if (BVar2 != 0x0) {
                                  BVar2 = write_to_file_1008_7cac(param_3);
                                  if (BVar2 != 0x0) {
                                    param_1 = write_file_fn_1028_e56c(param_1,iVar5,uVar6,param_3,0x7000000);
                                    if (BVar2 != 0x0) {
                                      BVar2 = write_to_file_1008_7cac(param_3);
                                      if (BVar2 != 0x0) {
                                        param_1 = write_file_fn_1028_e56c(param_1,iVar5,uVar6,param_3,0x8000000);
                                        if (BVar2 != 0x0) {
                                          pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x400);
                                          while( true ) {
                                            uVar4 = param_1;
                                            paVar3 = &local_14;
                                            pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar3));
                                            uStack24 = CONCAT22(uVar4,paVar3);
                                            param_1 = uVar4 | paVar3;
                                            if (param_1 == 0x0) break;
                                            if (paVar3[0x1c].field4_0x8 != 0x8000002) {
                                              pass1_1038_3ba0((astruct_428 *)CONCAT22(uVar4,paVar3));
                                            }
                                          }
                                          return 0x10000;
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
  return (u32)param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn read_file_1028_def2(u16_t param_1,u8 *param_2,HFILE16 *param_3)

{
  code **ppcVar1;
  BOOL16 BVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;

  uVar3 = param_3;
  uVar4 = ((u32)param_3 >> 0x10);
  read_file_1008_7cfe(uVar3,uVar4,0xa);
  if (param_1 != 0x0) {
    BVar2 = read_file_1008_7dee(param_3,param_2,0x4);
    if (BVar2 != 0x0) {
      BVar2 = read_file_1008_7dee(param_3,(u8 *)((u32)param_2 & 0xffff0000 | (u32)((int)param_2 + 0x8)),0x2);
      if (BVar2 != 0x0) {
        ppcVar1 = (code **)((int)*_PTR_LOOP_1050_5166 + 0x10);
        (**ppcVar1)(0x1008,(int)_PTR_LOOP_1050_5166,(int)((u32)_PTR_LOOP_1050_5166 >> 0x10),param_3);
        if (BVar2 != 0x0) {
          read_file_1008_7cfe(uVar3,uVar4,0xc);
          if (BVar2 != 0x0) {
            pass1_1028_e628((u32)param_2,uVar3,uVar4,0x0,0x100);
            if (BVar2 != 0x0) {
              read_file_1008_7cfe(uVar3,uVar4,0xd);
              if (BVar2 != 0x0) {
                pass1_1028_e628((u32)param_2,uVar3,uVar4,0x0,0x200);
                if (BVar2 != 0x0) {
                  read_file_1008_7cfe(uVar3,uVar4,0xe);
                  if (BVar2 != 0x0) {
                    pass1_1028_e628((u32)param_2,uVar3,uVar4,0x0,0x300);
                    if (BVar2 != 0x0) {
                      read_file_1008_7cfe(uVar3,uVar4,0xf);
                      if (BVar2 != 0x0) {
                        pass1_1028_e628((u32)param_2,uVar3,uVar4,0x0,0x400);
                        if (BVar2 != 0x0) {
                          read_file_1008_7cfe(uVar3,uVar4,0x10);
                          if (BVar2 != 0x0) {
                            pass1_1028_e628((u32)param_2,uVar3,uVar4,0x0,0x500);
                            if (BVar2 != 0x0) {
                              read_file_1008_7cfe(uVar3,uVar4,0x11);
                              if (BVar2 != 0x0) {
                                pass1_1028_e628((u32)param_2,uVar3,uVar4,0x0,0x600);
                                if (BVar2 != 0x0) {
                                  read_file_1008_7cfe(uVar3,uVar4,0x12);
                                  if (BVar2 != 0x0) {
                                    pass1_1028_e628((u32)param_2,uVar3,uVar4,0x0,0x700);
                                    if (BVar2 != 0x0) {
                                      read_file_1008_7cfe(uVar3,uVar4,0x13);
                                      if (BVar2 != 0x0) {
                                        pass1_1028_e628((u32)param_2,uVar3,uVar4,0x0,0x800);
                                        if (BVar2 != 0x0) {
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
  return;
}
pub fn pass1_1028_e0a0(u8 *param_1,mut param_2: u32,mut param_3: u32)

{
  let mut uVar1: u32;
  let mut in_register_0000000a: u16;
  let mut in_stack_0000ff10: u16;

  uVar1 = (u32)((int)param_2 + 0x52);
  pass1_1030_4782(CONCAT22(in_register_0000000a,param_1),uVar1,((u32)uVar1 >> 0x10),0x1,(int)param_3,
                  (int)(param_3 >> 0x10),in_stack_0000ff10);
  return;
}



pub fn pass1_1028_e0bc(u32 *param_1,param_2: *mut astruct_57,mut param_3: u32,mut param_4: i16) -> u32

{
  u32 *puVar1;
  u32 *puVar2;
  astruct_57 *uVar5;
  u32 *puVar3;
  let mut iVar4: i16;
  astruct_57 *paVar5;
  u32 *puVar6;

  mem_op_1000_179c(0x98,param_2);
  uVar5 = (astruct_57 *)param_2;
  puVar3 = param_1;
  paVar5 = uVar5;
  pass1_1030_4bbe(uVar5,*(astruct_117 **)((int)param_3 + 0x52),param_4);
  puVar6 = param_1;
  for (iVar4 = 0x26; iVar4 != 0x0; iVar4 += -0x1) {
    puVar2 = puVar6;
    puVar6 = puVar6 + 0x1;
    puVar1 = puVar3;
    puVar3 = puVar3 + 0x1;
    *puVar2 = *puVar1;
  }
  return (long)param_2 << 0x10 | ZEXT24(param_1);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_e100(u8 *param_1,mut param_2: u32,mut param_3: u16 )

{
  u32 *puVar1;
  u32 *puVar2;
  astruct_311 *uVar4;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  StructD *pSVar6;
  u32 *puVar7;
  u32 *puVar8;
  let mut uVar9: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;
  let mut uVar3: u32;

  pSVar6 = (StructD *)CONCAT22(in_register_0000000a,param_1);
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar6);
    PTR_LOOP_1050_5f2e = (u8 *)pSVar6;
  }
  else {
  }
  uVar4 = (astruct_311 *)fn_ptr_op_1000_1708(0xae,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
  uVar3 = ZEXT24(uVar4);
  uStack10 = CONCAT22(PTR_LOOP_1050_5f2e,uVar4);
  uVar5 = PTR_LOOP_1050_5f2e | uVar4;
  if (uVar5 == 0x0) {
    uStack6 = 0x0;
  }
  else {
    uVar4->field164_0xa4 = 0x0;
    uVar4->field165_0xa8 = 0x0;
    uVar4->field166_0xac = 0x0;
    uStack6 = uStack10;
    uVar3 = uStack10;
  }
  puVar7 = (u32 *)uVar3;
  pass1_1030_4c06(*(astruct_117 **)((int)param_2 + 0x52),param_3,uVar5);
  uVar9 = (uStack6 >> 0x10);
  puVar8 = (u32 *)uStack6;
  for (iVar4 = 0x2b; iVar4 != 0x0; iVar4 += -0x1) {
    puVar2 = puVar8;
    puVar8 = puVar8 + 0x1;
    puVar1 = puVar7;
    puVar7 = puVar7 + 0x1;
    *puVar2 = *puVar1;
  }
  puVar8 = puVar7;
  return;
}
pub fn pass1_1028_e198(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,param_4: *mut u16,param_5: *mut u16,mut param_6: u32)

{
  pass1_1028_e1ec(param_3,param_6);
  pass1_1030_5b1c(CONCAT22(param_2,param_1),param_4,param_5);
  return;
}
pub fn bad_1028_e1bc(void)

{
  return;
}
pub fn pass1_1028_e1ec(mut param_1: u32,mut param_2: u32)

{
  if (param_2._3_1_ == '\0') {
    return;
  }
  if (param_2._3_1_ == -0x1) {
    return;
  }
  bad_1030_1312();
  return;
}
pub fn send_msg_1028_e242(u32 *param_1,mut param_2: i16)

{
  u8 *puVar1;
  let mut unaff_DI: i16;
  LRESULT LVar2;

  puVar1 = (u8 *)(*param_1 % 0x64);
  if (*param_1 % 0x64 == 0x0) {
    LVar2 = SendMessage16(0x0,0x0,0x41,HWND16_1050_0396);
    puVar1 = (u8 *)((u32)LVar2 >> 0x10);
  }
  *param_1 = *param_1 + 0x1;
  if (param_2 != 0x0) {
    pass1_1028_e28a(puVar1,unaff_DI,&DAT_1050_1050);
  }
  return;
}


/*
Unable to decompile 'pass1_1028_e28a'
Cause:
Low-level Error: Symbol $$undef00000009 extends beyond the end of the address space
*/


// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1028_e2ac(mut param_1: u32,mut param_2: u16 )

{
  let mut uStack6: u32;

  uStack6 = (u32)((int)param_1 + (param_2 >> 0x8) * 0x4 + 0x2e);
  ((code)uStack6)();
  return;
}



pub fn pass1_1028_e2e0(param_1: *mut astruct_57,mut param_2: u32,u8 param_3) -> u32

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u32;
  u16 auStack10 [0x3];
  let mut uStack4: u16;

  uStack4 = param_3;
  if (uStack4 == 0xff) {
    uVar3 = pass1_1028_ebee(0x0,param_1,param_2);
    return uVar3;
  }
  uVar2 = (param_2 >> 0x10);
  iVar1 = (int)param_2 + 0x2e;
  auStack10[0] = (iVar1 + uStack4 * 0x4 + 0x2);
  uVar3 = (u32)auStack10[0];
  uVar2 = (code)(iVar1 + uStack4 * 0x4))();
  return CONCAT22((int)uVar3,uVar2);
}
pub fn pass1_1028_e332(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 )

{
  if ((param_3._1_1_ != 0x0) && (param_3._1_1_ < 0xa)) {
    pass1_1030_13f6(param_2,param_3 & 0xff,*(astruct_291 **)((int)param_1 + 0xa + param_3._1_1_ * 0x4),
                    CONCAT22(param_3,param_2) & 0xffffff);
  }
  return;
}
pub fn pass1_1028_e372(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 )

{
  astruct_291 *paVar1;
  let mut uVar2: u32;
  let mut uVar3: u32;
  code **ppcVar4;
  let mut uVar5: u32;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uStack32: u32;
  let mut uStack16: u32;
  let mut uStack10: u16;

  if (param_3 >> 0x8 != 0xff) {
    paVar1 = *(astruct_291 **)((int)param_1 + 0xa + (param_3 >> 0x8) * 0x4);
    uVar2 = (u32)((int)paVar1 + 0xa);
    uVar7 = param_3 & 0xff;
    uStack16 = CONCAT22(param_3,param_2) & 0xffffff;
    pass1_1028_e1ec(param_1,CONCAT22(param_3,param_2));
    uVar5 = (u32)(param_2 + 0x8);
    pass1_1028_e1ec(param_1,uVar5);
    for (uStack32 = 0x1; uStack10 = (uVar2 >> 0x10), uStack32 < uVar2; uStack32 += 0x1) {
      if (uStack32 != uStack16) {
        uVar6 = uStack16;
        bad_1030_1312();
        uVar8 = uStack10 | uVar6;
        if (uVar8 != 0x0) {
          uVar3 = (u32)(uVar6 + 0x4);
          pass1_1030_13f6((int)uVar3,uVar8,paVar1,uStack32);
          ppcVar4 = (code **)((int)(u32)(uVar5 & 0xffff | (u32)uVar7 << 0x10) + 0x18);
          (**ppcVar4)(0x1030,(int)(uVar5 & 0xffff),uVar7,uVar3);
        }
      }
    }
  }
  return;
}
pub fn pass1_1028_e44a(mut param_1: u32,i32 param_2)

{
  astruct_291 *paVar1;
  astruct_291 *paVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uStack18: u32;
  let mut uStack12: u16;

  pass1_1028_e372(param_1,param_2,((u32)param_2 >> 0x10));
  uVar8 = (param_1 >> 0x10);
  paVar1 = *(astruct_291 **)((int)param_1 + 0x26);
  paVar2 = *(astruct_291 **)((int)param_1 + 0x1e);
  uVar3 = (u32)((int)paVar2 + 0xa);
  for (uStack18 = 0x1; uStack12 = (uVar3 >> 0x10), uStack18 < uVar3; uStack18 += 0x1) {
    uVar6 = uVar3;
    bad_1030_1312();
    uVar5 = uVar6;
    if (((uStack12 | uVar5) != 0x0) && (*(i32 *)(uVar5 + 0x8) != param_2)) {
      uVar8 = (uVar5 + 0x16);
      uVar5 = (uVar5 + 0x18);
      uVar7 = uVar5 & 0xff;
      uVar4 = pass1_1030_13f6(uVar8,uVar7,paVar1,CONCAT22(uVar5,uVar8) & 0xffffff);
      pass1_1030_13f6(uVar4,uVar7,paVar2,uStack18);
    }
  }
  return;
}
pub fn pass1_1028_e4ec(param_1: *mut astruct_92)

{
  u32 *puVar1;
  i32 *plVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut in_DX: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;

  uVar5 = 0x0;
  uVar7 = ((u32)param_1 >> 0x10);
  iVar6 = (int)param_1;
  if ((iVar6 + 0x10) == 0x0) {
    do {
      if (*(i32 *)(iVar6 + 0x8) == 0x0) {
        return;
      }
      plVar2 = (i32 *)(iVar6 + 0x8);
      *plVar2 = *plVar2 + -0x1;
      bad_1030_1312();
      in_DX |= uVar5;
    } while (in_DX == 0x0);
  }
  else {
    do {
      uVar3 = (u32)(iVar6 + 0xc);
      puVar1 = (u32 *)(iVar6 + 0x8);
      if (uVar3 <= *puVar1 && *puVar1 != uVar3) {
        return;
      }
      uVar4 = (u32)(iVar6 + 0x8);
      plVar2 = (i32 *)(iVar6 + 0x8);
      *plVar2 = *plVar2 + 0x1;
      bad_1030_1312();
      in_DX |= uVar4;
    } while (in_DX == 0x0);
  }
  return;
}



u16_t write_file_fn_1028_e56c(u16_t param_1,mut param_2: u16 ,mut param_3: u16 ,u8 *param_4,mut param_5: u32)

{
  code **ppcVar1;
  astruct_92 *paVar2;
  BOOL16 BVar3;
  u16_t extraout_DX;
  HFILE16 in_stack_0000ffbe;
  u32 local_2a [0x3];
  u32 *puStack28;
  let mut uStack24: u32;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,param_5,((u32)param_5 >> 0x10));
  uStack24 = 0x0;
  while( true ) {
    paVar2 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar2));
    puStack28 = (u32 *)CONCAT22(param_1,paVar2);
    param_1 |= paVar2;
    if (param_1 == 0x0) break;
    uStack24 += 0x1;
  }
  local_2a[0] = uStack24;
  BVar3 = write_to_file_1008_7e1c(param_4,CONCAT22(0x1050,local_2a),(char *)0x4,in_stack_0000ffbe);
  if (BVar3 == 0x0) {
    u16_1050_0310 = 0x6d0;
  }
  else {
    local_14.field4_0x8._0_2_ = local_14.field5_0xc;
    local_14.field4_0x8 = local_14.field5_0xc;
    if (local_14.field6_0x10 != 0x0) {
      local_14.field4_0x8._0_2_ = 0x1;
      local_14.field5_0xc = 0x0;
      local_14.field4_0x8 = local_14.field5_0xc;
    }
    do {
      paVar2 = &local_14;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar2));
      puStack28 = (u32 *)CONCAT22(local_14.field5_0xc,paVar2);
      if ((local_14.field5_0xc | paVar2) == 0x0) {
        return 0x0;
      }
      ppcVar1 = (code **)((int)*puStack28 + 0xc);
      (**ppcVar1)(0x1008,paVar2,local_14.field5_0xc);
      local_2a[0] = local_2a[0] & 0xffff0000 | ZEXT24(paVar2);
      local_14.field5_0xc = extraout_DX;
      param_1 = extraout_DX;
    } while (paVar2 != NULL);
  }
  return param_1;
}



// WARNING: Instruction at (ram,0x10287af1) overlaps instruction at (ram,0x10287af0)
//
// WARNING: Control flow encountered bad instruction data
// WARNING: Removing unreachable block (ram,0x1028e2f6)
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: ram
pub fn pass1_1028_e628(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16,mut param_5: i16)

{
  char *pcVar1;
  i16 *piVar2;
  let mut uVar3: u32;
  char cVar4;
  let mut uVar5: u32;
  let mut uVar6: u32;
  i32 lVar7;
  code **ppcVar8;
  let mut uVar9: u16;
  u8 *puVar10;
  BOOL16 BVar11;
  let mut uVar12: u16;
  let mut uVar13: u32;
  let mut iVar14: i16;
  u8 *puVar15;
  let mut puVar16: *mut u16;
  astruct_57 *paVar17;
  let mut in_EDX: u32;
  let mut uVar19: u32;
  astruct_348 *uVar18;
  astruct_349 *paVar20;
  let mut uVar21: u16;
  astruct_349 *uVar20;
  let mut unaff_SI: u16;
  let mut unaff_DI: u16;
  let mut unaff_ES: u16;
  let mut uVar22: u16;
  let mut uVar23: u16;
  let mut bVar24: bool;
  let mut bVar25: bool;
  astruct_27 *paVar26;
  u32 *puVar27;
  astruct_180 *paVar28;
  let mut puVar29: *mut u16;
  u32 *puVar30;
  let mut in_stack_0000fe64: u16;
  let mut in_stack_0000fe6a: u16;
  let mut local_154: u16;
  let mut uStack338: u16;
  let mut local_14c: u16;
  let mut uStack330: u16;
  let mut in_stack_0000ff88: u16;
  let mut in_stack_0000ff8e: u16;
  let mut in_stack_0000ff92: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff98: u16;
  u8 uVar31;
  u8 uVar32;
  u8 uVar33;
  u8 uVar34;
  let mut uVar35: u16;
  u8 uVar36;
  u8 uVar37;
  let mut iVar38: i16;
  let mut uVar39: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffcc: u16;
  let mut local_30: u32;
  let mut uStack44: u16;
  let mut uStack42: u16;
  let mut uStack40: u16;
  let mut uStack38: u16;
  u32 *puStack36;
  u8 *puStack32;
  let mut uStack30: u16;
  let mut uStack28: u16;
  let mut uStack26: u16;
  u8 **ppuStack24;
  u8 *local_16;
  u8 *local_14;
  let mut local_12: i16;
  u8 *local_10;
  u8 *puStack14;
  code *pcStack12;
  u32 *puStack10;
  u32 *local_6;

  uVar39 = ((u32)in_EDX >> 0x10);
  uVar23 = SUB42(&DAT_1050_1050,0x0);
  uVar21 = unaff_SI;
  BVar11 = read_file_1008_7dee((HFILE16 *)CONCAT22(param_3,param_2),(u8 *)CONCAT22(0x1050,&local_6),0x4);
  if (BVar11 == 0x0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  puStack10 = NULL;
  if ((param_4 == 0x0) && ((char)(param_5 - 0x100U) == '\0')) {
    puVar16 = (u16 *)(param_5 - 0x100U >> 0x7);
    paVar17 = (astruct_57 *)CONCAT22(uVar39,puVar16);
    if ((u16 *)&PTR_LOOP_1050_000e < puVar16) {
      return;
    }
    uVar35 = (param_1 >> 0x10);
    uVar20 = (astruct_349 *)param_1;
    switch(puVar16) {
    case NULL:
      pass1_1030_145a((astruct_346 *)uVar20->field14_0xe,(long)local_6);
      uStack28 = 0x0;
      uStack26 = 0x0;
      while ((u32 *)CONCAT22(uStack26,uStack28) < local_6) {
        puVar30 = local_6;
        mem_op_1000_179c(0x14,paVar17);
        puStack32 = (u8 *)puVar30;
        uStack30 = paVar17;
        uVar19 = (u32)paVar17 & 0xffff0000;
        if ((uStack30 | puStack32) == 0x0) {
          puVar16 = NULL;
          uVar19 = (u32)paVar17 & 0xffff0000;
        }
        else {
          puVar29 = pass1_1030_5d0a((u16 *)((u32)puVar30 & 0xffff | (long)paVar17 << 0x10));
          uVar19 = uVar19 & 0xffff0000 | (u32)puVar29 >> 0x10;
          puVar16 = (u16 *)puVar29;
        }
        local_16 = (u8 *)uVar19;
        ppcVar8 = (code **)((int)(u32)CONCAT22(local_16,puVar16) + 0x10);
        ppuStack24 = (u8 **)puVar16;
        (**ppcVar8)();
        if (puVar16 == NULL) {
          return;
        }
        uVar6 = (u32)(ppuStack24 + 0x2);
        uVar5 = ZEXT24(ppuStack24[0x3]);
        puStack14 = (u8 *)uVar6;
        pcStack12 = (code *)(uVar6 >> 0x10);
        paVar17 = (astruct_57 *)(uVar19 & 0xffff0000 | uVar5 & 0xffff00ff);
        pass1_1030_14b4((astruct_156 *)uVar20->field14_0xe,ppuStack24,local_16,
                        uVar6 & 0xffff | (uVar5 & 0xff) << 0x10);
        lVar7 = CONCAT22(uStack26,uStack28) + 0x1;
        uStack28 = lVar7;
        uStack26 = ((u32)lVar7 >> 0x10);
      }
      break;
    case (u16 *)0x1:
    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    case (u16 *)0x2:
      pass1_1030_145a((astruct_346 *)uVar20->field15_0x12,(long)local_6);
      uStack40 = 0x0;
      uStack38 = 0x0;
      while ((u32 *)CONCAT22(uStack38,uStack40) < local_6) {
        puVar30 = local_6;
        mem_op_1000_179c(0x1c,paVar17);
        puStack32 = (u8 *)puVar30;
        uStack30 = paVar17;
        uVar19 = (u32)paVar17 & 0xffff0000 | (u32)(uStack30 | puStack32);
        if ((uStack30 | puStack32) == 0x0) {
          uVar9 = 0x0;
          uVar19 = (u32)paVar17 & 0xffff0000;
        }
        else {
          uVar9 = puStack32;
          pass1_1030_2958((astruct_180 *)((u32)puVar30 & 0xffff | (long)paVar17 << 0x10),uVar19);
        }
        puStack36 = (u32 *)CONCAT22((int)uVar19,uVar9);
        ppcVar8 = (code **)((int)*puStack36 + 0x10);
        (**ppcVar8)();
        if (uVar9 == 0x0) {
          return;
        }
        uVar21 = ((u32)puStack36 >> 0x10);
        uVar18 = (astruct_348 *)puStack36;
        uVar6 = (u32)&uVar18->field_0x4;
        uVar5 = (u32)uVar18->field6_0x6;
        puStack14 = (u8 *)uVar6;
        pcStack12 = (code *)(uVar6 >> 0x10);
        paVar17 = (astruct_57 *)(uVar19 & 0xffff0000 | uVar5 & 0xffff00ff);
        pass1_1030_14b4((astruct_156 *)uVar20->field15_0x12,uVar18,uVar21,
                        uVar6 & 0xffff | (uVar5 & 0xff) << 0x10);
        lVar7 = CONCAT22(uStack38,uStack40) + 0x1;
        uStack40 = lVar7;
        uStack38 = ((u32)lVar7 >> 0x10);
      }
      break;
    case (u16 *)0x3:
      puVar10 = &uVar20->field_0x114;
      pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x500);
      puVar15 = (u8 *)paVar17;
      local_16 = puVar10;
      local_14 = puVar15;
      pass1_1030_61fe(puVar10,puVar15,_PTR_LOOP_1050_5740,CONCAT22(puVar15,puVar10),
                      param_1 & 0xffff0000 | ZEXT24(&uVar20->field_0x114),*(i32 *)&uVar20->field_0x108);
      if ((uVar20->field250_0x11a == 0xa) || (uVar20->field250_0x11a == 0x37)) {
        if (uVar20->field250_0x11a == 0x37) {
          puVar15 = *(u8 **)((int)&uVar20->field253_0x11e + 0x2);
          uVar19 = uVar20->field242_0x10c;
          uStack42 = uVar19;
          uStack40 = (uVar19 >> 0x10);
          (u32)((int)uVar20->field253_0x11e + 0x20) = uVar19;
        }
        puVar10 = &uVar20->field_0x114;
        pass1_1028_e2ac(_PTR_LOOP_1050_65e2,0x400);
        *(u8 **)&uVar20->field242_0x10c = puVar10;
        *(u8 **)((int)&uVar20->field242_0x10c + 0x2) = puVar15;
        pass1_1018_0196(puVar10,puVar15,(u32)local_6,CONCAT22(puVar15,&uVar20->field242_0x10c),
                        (u32)&uVar20->field_0x108);
        if (uVar20->field250_0x11a == 0xa) {
          pass1_1010_ed22((u32)local_6,uVar20->field242_0x10c);
        }
      }
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar20->field242_0x10c);
      *(u8 **)&uVar20->field243_0x110 = puVar10;
      *(u8 **)((int)&uVar20->field243_0x110 + 0x2) = puVar15;
      uStack26 = puVar15 | &uVar20->field243_0x110;
      if (uStack26 != 0x0) {
        ppcVar8 = (code **)((int)*uVar20->field243_0x110 + 0x8);
        (**ppcVar8)();
      }
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(local_14,local_16));
      ppuStack24 = (u8 **)puVar15;
      pass1_1030_73ee(puVar15,(astruct_294 *)CONCAT22(puVar15,uStack26),uVar20->field242_0x10c);
      BVar11 = pass1_1008_c6ae(_u16_1050_06e0,uVar20->field250_0x11a,0x31);
      if ((BVar11 == 0x0) && (uVar20->field254_0x122 == 0x0)) {
        uVar23 = ((u32)(u32)(uStack26 + 0xc) >> 0x10);
        if ((uStack26 + 0x10) < 0x1) {
          uVar39 = 0x5;
        }
        else {
          uVar39 = 0x6;
        }
        (uStack26 + 0x14) = uVar39;
        puVar15 = (u8 *)ppuStack24;
      }
      uVar13 = (u32)(uStack26 + 0x16);
      uStack30 = uVar13;
      uStack28 = (uVar13 >> 0x10);
      pass1_1028_e1ec((u32)&PTR_LOOP_1050_65e2,uVar13);
      puStack36 = (u32 *)CONCAT22((int)uVar13,puStack36._0_2_);
      puStack32 = puVar15;
      if (CONCAT22(uStack28,uStack30) != 0x0) {
        struct_1030_e4fa((astruct_97 *)CONCAT22(0x1050,&local_14c),CONCAT22(uStack28,uStack30));
        fn_ptr_1030_835a((u32 **)(u32)&u16_1050_5748,(char *)CONCAT22(0x1050,&local_14c));
        local_14c = 0x389a;
        uStack330 = 0x1008;
      }
      ppcVar8 = (code **)((int)*uVar20->field253_0x11e + 0x4);
      (**ppcVar8)();
      puVar30 = uVar20->field253_0x11e;
      pass1_1030_7e5a(puVar15,(astruct_358 *)CONCAT22(ppuStack24,uStack26),(u32)((int)puVar30 + 0x4));
      return;
    case (u16 *)0x4:
      pass1_1030_145a((astruct_346 *)uVar20->field16_0x16,(long)local_6);
      uStack40 = 0x0;
      uStack38 = 0x0;
      while ((u32 *)CONCAT22(uStack38,uStack40) < local_6) {
        puVar30 = local_6;
        mem_op_1000_179c(0x1e,paVar17);
        puStack32 = (u8 *)puVar30;
        uStack30 = paVar17;
        uVar19 = (u32)paVar17 & 0xffff0000;
        if ((uStack30 | puStack32) == 0x0) {
          iVar14 = 0x0;
          uVar19 = (u32)paVar17 & 0xffff0000;
        }
        else {
          puVar29 = pass1_1030_560e((astruct_180 *)((u32)puVar30 & 0xffff | (long)paVar17 << 0x10));
          uVar19 = uVar19 & 0xffff0000 | (u32)puVar29 >> 0x10;
          iVar14 = (int)puVar29;
        }
        puStack36 = (u32 *)CONCAT22((int)uVar19,iVar14);
        ppcVar8 = (code **)((int)*puStack36 + 0x10);
        (**ppcVar8)();
        if (iVar14 == 0x0) {
          return;
        }
        uVar39 = ((u32)puStack36 >> 0x10);
        uVar6 = (u32)((int)puStack36 + 0x4);
        puStack14 = (u8 *)uVar6;
        pcStack12 = (code *)(uVar6 >> 0x10);
        uVar5 = (u32)((int)puStack36 + 0x10);
        uStack28 = uVar5;
        uStack26 = (uVar5 >> 0x10);
        pass1_1030_6222(uStack28,(u8 *)uVar19,_PTR_LOOP_1050_5740,0x0,uVar5,uVar6);
        paVar17 = (astruct_57 *)(uVar19 & 0xffff0000 | ZEXT24(pcStack12) & 0xffff00ff);
        pass1_1030_14b4((astruct_156 *)uVar20->field16_0x16,puStack36,((u32)puStack36 >> 0x10),
                        CONCAT22((int)(ZEXT24(pcStack12) & 0xffff00ff),puStack14));
        lVar7 = CONCAT22(uStack38,uStack40) + 0x1;
        uStack40 = lVar7;
        uStack38 = ((u32)lVar7 >> 0x10);
      }
      break;
    case (u16 *)0x5:
      *puVar16 = 0x5280;
      puVar16[0x1] = 0x1028;
      return;
    case (u16 *)0x6:
      pass1_1030_145a((astruct_346 *)uVar20->field17_0x1a,(long)local_6);
      for (local_30 = NULL; local_30 < local_6; local_30 = (u32 *)((long)local_30 + 0x1)) {
        puVar30 = local_6;
        mem_op_1000_179c(0x21e,paVar17);
        puStack32 = (u8 *)puVar30;
        uStack30 = paVar17;
        uVar19 = (u32)paVar17 & 0xffff0000 | (u32)(uStack30 | puStack32);
        if ((uStack30 | puStack32) == 0x0) {
          uVar9 = 0x0;
          uVar19 = (u32)paVar17 & 0xffff0000;
        }
        else {
          uVar9 = puStack32;
          pass1_1038_30aa((astruct_180 *)((u32)puVar30 & 0xffff | (long)paVar17 << 0x10),uVar19);
        }
        uStack42 = uVar19;
        ppcVar8 = (code **)((int)(u32)CONCAT22(uStack42,uVar9) + 0x10);
        uStack44 = uVar9;
        (**ppcVar8)();
        if (uVar9 == 0x0) {
          return;
        }
        uVar6 = (u32)(uStack44 + 0x4);
        uVar5 = (u32)(uStack44 + 0x6);
        puStack14 = (u8 *)uVar6;
        pcStack12 = (code *)(uVar6 >> 0x10);
        paVar17 = (astruct_57 *)(uVar19 & 0xffff0000 | uVar5 & 0xffff00ff);
        pass1_1030_14b4((astruct_156 *)uVar20->field17_0x1a,uStack44,uStack42,uVar6 & 0xffff | (uVar5 & 0xff) << 0x10);
      }
      break;
    default:
      pass1_1030_145a((astruct_346 *)uVar20->field18_0x1e,(long)local_6);
      pass1_1030_66de(_PTR_LOOP_1050_5740,(u32)local_6);
      local_30 = NULL;
      while( true ) {
        if (local_6 <= local_30) {
          pass1_1030_154c();
          pass1_1030_6740(_PTR_LOOP_1050_5740);
          return;
        }
        local_14 = (u8 *)_PTR_LOOP_1050_5744;
        local_12 = (int)(_PTR_LOOP_1050_5744 >> 0x10);
        paVar28 = (astruct_180 *)pass1_1000_07fc(_PTR_LOOP_1050_5744);
        uStack30 = ((u32)paVar28 >> 0x10);
        puStack32 = (u8 *)paVar28;
        uVar9 = uStack30 | puStack32;
        if (uVar9 == 0x0) {
          uVar12 = 0x0;
          uVar9 = 0x0;
        }
        else {
          uVar12 = puStack32;
          pass1_1030_67cc(paVar28);
        }
        ppcVar8 = (code **)((int)(u32)CONCAT22(uVar9,uVar12) + 0x10);
        uStack44 = uVar12;
        uStack42 = uVar9;
        (**ppcVar8)();
        if (uVar12 == 0x0) break;
        uVar19 = (u32)(uStack44 + 0x4);
        puStack14 = (u8 *)uVar19;
        pcStack12 = (code *)(uVar19 >> 0x10);
        lVar7 = *(i32 *)(uStack44 + 0x8);
        uStack40 = lVar7;
        uStack38 = ((u32)lVar7 >> 0x10);
        puStack36 = (u32 *)((u32)puStack36 & 0xffff0000 | ZEXT24(&stack0xffca));
        pass1_1030_671c(&stack0xffca,uStack42,_PTR_LOOP_1050_5740,uVar19,(u16 *)CONCAT22(0x1050,&stack0xffca)
                        ,lVar7);
        pass1_1030_14b4((astruct_156 *)uVar20->field18_0x1e,uStack44,uStack42,CONCAT22(pcStack12,puStack14) & 0xffffff);
        local_30 = (u32 *)((long)local_30 + 0x1);
      }
      return;
    case (u16 *)0x9:
      local_6 = (u32 *)((u32)local_6 & 0xffff);
      pcStack12 = (code *)uVar20->field22_0x2e;
      puStack10 = (u32 *)(u32)uVar20->field23_0x30;
      (*pcStack12)();
      return;
    case (u16 *)0xa:
      pass1_1030_145a((astruct_346 *)uVar20->field19_0x22,(long)local_6);
      uVar39 = 0x0;
      uVar23 = 0x0;
      while ((u32 *)CONCAT22(uVar23,uVar39) < local_6) {
        puVar30 = local_6;
        mem_op_1000_179c(0xe,paVar17);
        puStack32 = (u8 *)puVar30;
        uStack30 = paVar17;
        uVar19 = (u32)paVar17 & 0xffff0000;
        if ((uStack30 | puStack32) == 0x0) {
          iVar14 = 0x0;
          uVar19 = (u32)paVar17 & 0xffff0000;
        }
        else {
          puVar29 = pass1_1028_b204((u16 *)((u32)puVar30 & 0xffff | (long)paVar17 << 0x10));
          uVar19 = uVar19 & 0xffff0000 | (u32)puVar29 >> 0x10;
          iVar14 = (int)puVar29;
        }
        local_30 = (u32 *)CONCAT22((int)uVar19,iVar14);
        ppcVar8 = (code **)((int)*local_30 + 0x10);
        (**ppcVar8)();
        if (iVar14 == 0x0) {
          return;
        }
        uVar22 = ((u32)local_30 >> 0x10);
        uVar21 = local_30;
        uVar6 = (u32)(uVar21 + 0x4);
        uVar5 = (u32)(uVar21 + 0x6);
        puStack14 = (u8 *)uVar6;
        pcStack12 = (code *)(uVar6 >> 0x10);
        paVar17 = (astruct_57 *)(uVar19 & 0xffff0000 | uVar5 & 0xffff00ff);
        pass1_1030_14b4((astruct_156 *)uVar20->field19_0x22,uVar21,uVar22,uVar6 & 0xffff | (uVar5 & 0xff) << 0x10);
        lVar7 = CONCAT22(uVar23,uVar39) + 0x1;
        uVar39 = lVar7;
        uVar23 = ((u32)lVar7 >> 0x10);
      }
      break;
    case (u16 *)0xb:
      if (puVar16 < (u16 *)((int)&PTR_LOOP_1050_000e + 0x1)) {
        pcVar1 = (char *)(unaff_SI + 0x23);
        cVar4 = *pcVar1;
        *pcVar1 = *pcVar1 << 0x6;
        uVar39 = 0x2b;
        piVar2 = (i16 *)((int)puVar16 + unaff_SI);
        *piVar2 = *piVar2 + (-0x6600 - ((char)(cVar4 << 0x5) < '\0'));
      }
      else {
        uVar39 = 0x7af0;
        pass1_1028_780c(uVar21,unaff_DI,CONCAT22(in_stack_0000ffcc,in_stack_0000ffca));
        if (param_4 == 0x0) goto code_r0x10287b17;
      }
      uVar31 = 0x0;
      uVar32 = 0x4;
      iVar14 = 0x1d;
      paVar26 = (astruct_27 *)
                mixed_1010_20ba(paVar17,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fe64,in_stack_0000ff88,
                                in_stack_0000ff8e,in_stack_0000ff92);
      paVar17 = (astruct_57 *)((u32)paVar17 & 0xffff0000 | (u32)paVar26 >> 0x10);
      param_4 = (int)paVar26;
      pass1_1010_043a(paVar26,CONCAT13(uVar32,CONCAT12(uVar31,(int)((u32)paVar26 >> 0x10))),iVar14);
code_r0x10287b17:
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000002);
      pass1_1028_780c(uVar20,uVar35,CONCAT22((int)paVar17,param_4));
      puStack10 = mixed_1010_20ba(paVar17,_u16_1050_0ed0,(u8 **)CONCAT22(uVar39,0x2),in_stack_0000fe6a,
                                  in_stack_0000ff8e,in_stack_0000ff94,in_stack_0000ff98);
      pcStack12 = (code *)PTR_LOOP_1050_13ae;
      if (0x2 < (int)PTR_LOOP_1050_13ae) {
        puVar27 = mixed_1010_20ba((astruct_57 *)((u32)paVar17 & 0xffff0000 | (u32)puStack10 >> 0x10),_u16_1050_0ed0,
                                  (u8 **)CONCAT22(uVar39,0x2f),in_stack_0000fe6a,in_stack_0000ff8e,
                                  in_stack_0000ff94,in_stack_0000ff98);
        uVar31 = SUB41(puVar27,0x0);
        uVar32 = (u8)((u32)puVar27 >> 0x8);
        uVar33 = 0x1;
        uVar34 = 0x0;
        uVar39 = (int)((u32)puVar27 >> 0x10);
        while (CONCAT11(uVar34,uVar33) < 0x9) {
          uVar23 = uVar39;
          if ((u32 *)*(i32 *)(CONCAT11(uVar32,uVar31) + 0x34 + CONCAT11(uVar34,uVar33) * 0x4) == local_6) {
            uVar9 = 0x1;
            local_30 = (u32 *)CONCAT22(local_30,0x1);
            uVar33 = 0xd7;
            uVar34 = 0x7b;
            pass1_1008_612e(0x1,0x1,0x64);
            puVar16 = (u16 *)(CONCAT11(uVar34,uVar33) - 0x7);
            if (puVar16 == NULL) {
              bVar25 = SBORROW2(uVar9,0x32);
              iVar14 = uVar9 - 0x32;
              bVar24 = uVar9 == 0x32;//
LAB_1028_7b74:
              if (!bVar24 && bVar25 == iVar14 < 0x0) {
                local_30 = (u32 *)((u32)local_30 & 0xffff0000);
              }
            }
            else {
              puVar16 = (u16 *)(CONCAT11(uVar34,uVar33) - 0x8);
              if (puVar16 == NULL) {
                bVar25 = SBORROW2(uVar9,0x19);
                iVar14 = uVar9 - 0x19;
                bVar24 = iVar14 == 0x0;
                goto LAB_1028_7b74;
              }
            }
            uStack30 = uVar9;
            if ((int)local_30 != 0x0) {
              pass1_1028_90e6((astruct_97 *)CONCAT13(0x10,CONCAT12(0x50,&local_154)),CONCAT11(uVar34,uVar33));
              puVar16 = &local_154;
              uVar39 = 0x1008;
              uVar31 = 0xc;
              uVar32 = 0x7c;
              fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,puVar16));
              local_154 = 0x389a;
              uStack338 = 0x1008;
            }
            uVar36 = 0x0;
            uVar37 = 0x0;
            uVar33 = 0x23;
            uVar34 = 0x7c;
            pass1_1008_612e(puVar16,0x0,0xa);
            ppuStack24 = (u8 **)puVar16;
            if (CONCAT11(uVar34,uVar33) == 0x7) {
              iVar38 = 0x7;
              puVar16 = puVar16 + 0x37;
              iVar14 = (int)puVar16 >> 0xf;
            }
            else {
              uVar23 = uVar39;
              if (CONCAT11(uVar34,uVar33) != 0x8) goto LAB_1028_7ba0;
              iVar38 = 0x8;
              puVar16 = puVar16 + 0x32;
              iVar14 = ((int)puVar16 >> 0xf) + ((u16 *)0xff9b < puVar16);
            }
            uVar21 = iVar38 + iVar14 + CARRY2(CONCAT11(uVar37,uVar36),puVar16);
            uVar23 = CONCAT11(uVar32,uVar31);
            uVar33 = (u8)uVar39;
            uVar34 = (u8)(uVar39 >> 0x8);
            uVar31 = 0x8;
            uVar32 = 0x10;
            pass1_1010_ebf8(CONCAT13(uVar34,CONCAT12(uVar33,uVar23)),CONCAT11(uVar37,uVar36) + (int)puVar16,uVar21,
                            uVar21);
          }//
LAB_1028_7ba0:
          iVar14 = CONCAT11(uVar34,uVar33) + 0x1;
          uVar33 = (u8)iVar14;
          uVar39 = uVar23;
          uVar34 = (u8)(iVar14 >> 0x8);
        }
      }
      return;
    case (u16 *)0xc:
      paVar20 = uVar20;
      pass1_1030_145a((astruct_346 *)uVar20->field20_0x26,(long)local_6);
      uVar39 = 0x0;
      uVar23 = 0x0;
      while ((u32 *)CONCAT22(uVar23,uVar39) < local_6) {
        BVar11 = read_file_1008_7dee((HFILE16 *)CONCAT22(param_3,param_2),(u8 *)CONCAT22(0x1050,&local_30),0x2);
        if (BVar11 == 0x0) {
          u16_1050_0310 = 0x6d2;
          return;
        }
        uStack44 = switch_1008_73ea(param_2,param_3,(int)local_30);
        puVar30 = (u32 *)switch_1030_0000(paVar20,paVar17,uVar20,uVar35,uStack44);
        uVar19 = (u32)paVar17 & 0xffff0000;
        uVar21 = puVar30;
        uStack38 = ((u32)puVar30 >> 0x10);
        ppcVar8 = (code **)((int)*puVar30 + 0x10);
        uStack40 = uVar21;
        (**ppcVar8)();
        if (uVar21 == 0x0) {
          return;
        }
        uVar6 = (u32)(uStack40 + 0x4);
        uVar5 = (u32)(uStack40 + 0x6);
        puStack14 = (u8 *)uVar6;
        pcStack12 = (code *)(uVar6 >> 0x10);
        paVar17 = (astruct_57 *)(uVar19 & 0xffff0000 | uVar5 & 0xffff00ff);
        paVar20 = uVar20;
        pass1_1030_14b4((astruct_156 *)uVar20->field20_0x26,uStack40,uStack38,uVar6 & 0xffff | (uVar5 & 0xff) << 0x10);
        lVar7 = CONCAT22(uVar23,uVar39) + 0x1;
        uVar39 = lVar7;
        uVar23 = ((u32)lVar7 >> 0x10);
      }
      break;
    case (u16 *)0xd:
      puStack10 = (u32 *)(ZEXT24(puVar16) << 0x10);
      uVar3 = (u32)&PTR_LOOP_1050_000c;
      local_10 = (u8 *)uVar3;
      puStack14 = (u8 *)((u32)uVar3 >> 0x10);
      pcStack12 = *(code **)&PTR_LOOP_1050_0010;
      ppuStack24 = &local_10;
      pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_10),(u16 *)CONCAT22(0x1050,&local_16),
                      (u16 *)CONCAT22(0x1050,&local_14),(u16 *)CONCAT22(0x1050,&local_12));
      ppuStack24 = (u8 **)(local_14 + -0x1);
      puVar15 = (u8 *)paVar17;
      puStack14 = (u8 *)ppuStack24;
      uVar9 = pass1_1028_21ba(&local_10,puVar15,uVar20,uVar35,(u16 *)CONCAT22(0x1050,&local_10),
                              (long)local_6);
      if (uVar9 == 0x0) {
        ppuStack24 = (u8 **)(local_14 + 0x1);
        puStack14 = (u8 *)ppuStack24;
        uVar9 = pass1_1028_21ba(&local_10,puVar15,uVar20,uVar35,(u16 *)CONCAT22(0x1050,&local_10)
                                ,(long)local_6);
        if (uVar9 == 0x0) {
          puStack14 = local_14;
          ppuStack24 = (u8 **)(local_12 + -0x1);
          local_10 = (u8 *)ppuStack24;
          uVar9 = pass1_1028_21ba(&local_10,puVar15,uVar20,uVar35,
                                  (u16 *)CONCAT22(0x1050,&local_10),(long)local_6);
          if (uVar9 == 0x0) {
            ppuStack24 = (u8 **)(local_12 + 0x1);
            local_10 = (u8 *)ppuStack24;
            uVar9 = pass1_1028_21ba(&local_10,puVar15,uVar20,uVar35,
                                    (u16 *)CONCAT22(0x1050,&local_10),(long)local_6);
            if (uVar9 == 0x0) {
              return;
            }
          }
        }
      }
      pass1_1038_79b2(uVar9,puVar15,_PTR_LOOP_1050_5a64,(u32)puStack10);
      return;
    case (u16 *)0xe:
      pass1_1030_145a((astruct_346 *)uVar20->field21_0x2a,(long)local_6);
      uVar39 = 0x0;
      uVar23 = 0x0;
      while ((u32 *)CONCAT22(uVar23,uVar39) < local_6) {
        puVar30 = local_6;
        mem_op_1000_179c(0x3b2,paVar17);
        puStack32 = (u8 *)puVar30;
        uStack30 = paVar17;
        uVar19 = (u32)paVar17 & 0xffff0000 | (u32)(uStack30 | puStack32);
        if ((uStack30 | puStack32) == 0x0) {
          uVar9 = 0x0;
          uVar19 = (u32)paVar17 & 0xffff0000;
        }
        else {
          uVar9 = puStack32;
          pass1_1030_2068((astruct_180 *)((u32)puVar30 & 0xffff | (long)paVar17 << 0x10));
        }
        local_30 = (u32 *)CONCAT22((int)uVar19,uVar9);
        ppcVar8 = (code **)((int)*local_30 + 0x10);
        (**ppcVar8)();
        if (uVar9 == 0x0) {
          return;
        }
        uVar22 = ((u32)local_30 >> 0x10);
        uVar21 = local_30;
        uVar6 = (u32)(uVar21 + 0x4);
        uVar5 = (u32)(uVar21 + 0x6);
        puStack14 = (u8 *)uVar6;
        pcStack12 = (code *)(uVar6 >> 0x10);
        paVar17 = (astruct_57 *)(uVar19 & 0xffff0000 | uVar5 & 0xffff00ff);
        pass1_1030_14b4(uVar20->field21_0x2a,uVar21,uVar22,uVar6 & 0xffff | (uVar5 & 0xff) << 0x10);
        lVar7 = CONCAT22(uVar23,uVar39) + 0x1;
        uVar39 = lVar7;
        uVar23 = ((u32)lVar7 >> 0x10);
      }
    }
    pass1_1030_154c();
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1028_ebee(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u32) -> u32

{
  u8 *puVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u32;

  mem_op_1000_179c(0x14,param_2);
  puVar1 = (u8 *)(param_2 | param_1);
  if (puVar1 != NULL) {
    pass1_1030_1a32((u16 *)CONCAT22(param_2,param_1),param_1,puVar1);
  }
  uVar4 = struct_1030_4574(*(astruct_159 **)((int)param_3 + 0x52));
  uVar3 = ((u32)_PTR_LOOP_1050_5166 >> 0x10);
  iVar2 = (int)_PTR_LOOP_1050_5166;
  (iVar2 + 0x10) = (int)uVar4;
  (iVar2 + 0x12) = (int)(uVar4 >> 0x10);
  uVar3 = ((u32)_PTR_LOOP_1050_5166 >> 0x10);
  return CONCAT22(((int)_PTR_LOOP_1050_5166 + 0x6),((int)_PTR_LOOP_1050_5166 + 0x4));
}
pub fn pass1_1028_ec36(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: i16,mut param_6: u16 ,
                    mut param_7: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  u8 *puVar4;
  u8 *puVar5;
  let mut uVar6: u16;
  let mut puVar7: *mut u16;

  mem_op_1000_179c(0x14,param_2);
  puVar4 = (u8 *)(param_2 | param_1);
  if (puVar4 == NULL) {
    uVar2 = 0x0;
    puVar4 = NULL;
  }
  else {
    puVar7 = pass1_1030_5d3c(param_1,puVar4,(u16 *)CONCAT22(param_2,param_1),param_7);
    puVar4 = (u8 *)((u32)puVar7 >> 0x10);
    uVar2 = puVar7;
  }
  uVar6 = (param_3 >> 0x10);
  uVar1 = (u32)((int)param_3 + 0x52);
  puVar5 = puVar4;
  uVar3 = uVar2;
  pass1_1030_4594(puVar4,uVar1,((u32)uVar1 >> 0x10),param_5);
  pass1_1030_5fe2(CONCAT22(puVar4,uVar2),CONCAT22(puVar5,uVar3));
  pass1_1030_1358(*(astruct_291 **)((int)param_3 + 0xe),uVar2,puVar4,
                  (u32)(uVar2 + 0x4) & 0xffff | ((u32)(uVar2 + 0x6) & 0xff) << 0x10);
  return;
}
pub fn pass1_1028_ecac(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,i16 *param_5,mut param_6: u16 ,
                    mut param_7: u32)

{
  let mut uVar1: u32;
  i16 **ppiVar2;
  let mut uVar3: u16;
  u8 *puVar4;
  u8 *puVar5;
  let mut uVar7: u16;
  astruct_57 *paVar6;

  mem_op_1000_179c(0x1c,param_2);
  uVar3 = param_2 | param_1;
  paVar6 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)uVar3);
  if (uVar3 == 0x0) {
    param_1 = 0x0;
    puVar4 = NULL;
  }
  else {
    struct_1030_299a(param_1,paVar6,(astruct_352 *)CONCAT22(param_2,param_1),param_7);
    puVar4 = (u8 *)paVar6;
  }
  uVar7 = (param_3 >> 0x10);
  uVar1 = (u32)((int)param_3 + 0x52);
  puVar5 = puVar4;
  ppiVar2 = (i16 **)param_5;
  pass1_1030_4628(puVar4,uVar1,((u32)uVar1 >> 0x10),(int)param_5);
  *ppiVar2 = param_5;
  pass1_1030_3006(CONCAT22(puVar4,param_1),CONCAT22(puVar5,ppiVar2));
  pass1_1030_1358(*(astruct_291 **)((int)param_3 + 0x12),param_1,puVar4,
                  (u32)(param_1 + 0x4) & 0xffff | ((u32)(param_1 + 0x6) & 0xff) << 0x10);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_ed2c(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: i16,mut param_6: u16 ,
                    mut param_7: u32)

{
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  u8 *puVar6;
  u8 *puVar7;
  let mut uVar9: u32;
  let mut uVar10: u16;
  let mut puVar11: *mut u16;
  let mut in_stack_0000fef8: u16;
  let mut uVar1: u32;
  astruct_57 *paVar8;

  mem_op_1000_179c(0x1e,param_2);
  uVar4 = param_2 | param_1;
  uVar9 = (u32)param_2 & 0xffff0000;
  paVar8 = (astruct_57 *)(uVar9 | uVar4);
  if (uVar4 == 0x0) {
    uVar2 = 0x0;
  }
  else {
    puVar11 = struct_1030_565a(param_1,paVar8,(astruct_352 *)CONCAT22(param_2,param_1),param_7);
    uVar9 = (u32)paVar8 & 0xffff0000 | (u32)puVar11 >> 0x10;
    uVar2 = puVar11;
  }
  uVar5 = uVar9;
  uVar10 = (param_3 >> 0x10);
  uVar1 = (u32)((int)param_3 + 0x52);
  uVar3 = uVar2;
  pass1_1030_4782(uVar9,uVar1,((u32)uVar1 >> 0x10),0x1,0x1,param_5,in_stack_0000fef8);
  puVar6 = (u8 *)uVar9;
  puVar7 = puVar6;
  pass1_1030_5a80(CONCAT22(uVar5,uVar2),CONCAT22(puVar6,uVar3));
  uVar9 = (u32)(uVar2 + 0x4);
  pass1_1030_6222(uVar9,puVar7,_PTR_LOOP_1050_5740,0x1,CONCAT22(puVar6,uVar3),uVar9);
  pass1_1030_1358(*(astruct_291 **)((int)param_3 + 0x16),uVar2,uVar5,uVar9 & 0xffffff);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_edc4(param_1: *mut astruct_57,mut param_2: u32,mut param_3: u16 ,param_4: *mut u16,i32 param_5,undefined1 param_6)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 local_1a [0x4];
  let mut uStack22: u32;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut puStack6: *mut u16;
  astruct_57 *paVar5;

  puStack6 = param_4;
  pass1_1030_64ce((int)param_4,(int)param_1,_PTR_LOOP_1050_5740,param_4,param_5,(u32 *)CONCAT22(0x1050,local_1a));
  uVar2 = (u32)param_4;
  uStack14 = uVar2;
  uStack10 = uVar2;
  mem_op_1000_179c(0x21e,param_1);
  uVar1 = uVar2;
  uVar3 = param_1 | uVar1;
  paVar5 = (astruct_57 *)((u32)param_1 & 0xffff0000 | (u32)uVar3);
  if (uVar3 == 0x0) {
    uVar1 = 0x0;
    uVar4 = 0x0;
  }
  else {
    pass1_1038_3222(uVar1,paVar5,(astruct_363 *)(uVar2 & 0xffff | (long)param_1 << 0x10),uStack14,param_5);
    uVar4 = paVar5;
  }
  uStack18 = CONCAT22(uVar4,uVar1);
  uStack22 = (u32)(uVar1 + 0x4);
  pass1_1030_1358(*(astruct_291 **)((int)param_2 + 0x1a),uVar1,uVar4,
                  uStack22 & 0xffff | (u32)((uVar1 + 0x6) & 0xff) << 0x10);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_ee54(mut param_1: u32,mut param_2: u16 ,param_3: *mut u16,mut param_4: u32)

{
  let mut in_DX: u16;
  let mut uVar1: u16;
  astruct_99 *paVar2;
  u8 local_16 [0x4];
  let mut uStack18: u32;
  let mut uStack14: u16;
  let mut uStack12: u16;
  let mut uStack10: u32;
  let mut puStack6: *mut u16;

  puStack6 = param_3;
  pass1_1030_64ce((int)param_3,in_DX,_PTR_LOOP_1050_5740,param_3,param_4,(u32 *)CONCAT22(0x1050,local_16));
  uStack10 = (u32)param_3;
  paVar2 = pass1_1000_07fc(_PTR_LOOP_1050_5744);
  uVar1 = ((u32)paVar2 >> 0x10);
  uStack14 = paVar2;
  uStack12 = uVar1 | uStack14;
  if (uStack12 == 0x0) {
    uStack14 = 0x0;
    uStack12 = 0x0;
  }
  else {
    pass1_1030_684c((u16 *)((u32)paVar2 & 0xffff | (u32)uVar1 << 0x10),(u32 *)puStack6,
                    ((u32)puStack6 >> 0x10),uStack10,((u32)uStack10 >> 0x10),param_4);
  }
  uStack18 = (u32)(uStack14 + 0x4);
  pass1_1030_61fe(uStack18,uStack12,_PTR_LOOP_1050_5740,uStack18,(u32)puStack6,param_4);
  pass1_1030_1358(*(astruct_291 **)((int)param_1 + 0x1e),uStack14,uStack12,
                  uStack18 & 0xffff | (u32)(uStack18 & 0xff) << 0x10);
  return;
}
pub fn pass1_1028_ef00(param_1: *mut astruct_57,mut param_2: u32,mut param_3: u16 ,param_4: *mut astruct_365,mut param_5: u16 ,mut param_6: u16
                    )

{
  astruct_365 *paVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut puVar4: *mut u16;

  if ((astruct_365 *)param_4 == (astruct_365 *)&u32_1050_0004) {
    mem_op_1000_179c(0x16,param_1);
    uVar2 = param_1 | (astruct_365 *)param_4;
    if (uVar2 != 0x0) {
      pass1_1030_b936(uVar2,(astruct_365 *)param_4,param_1,0x4,_param_5);
      goto LAB_1028_ef8b;
    }
  }
  else if ((astruct_365 *)param_4 == (astruct_365 *)&PTR_LOOP_1050_000c) {
    mem_op_1000_179c(0xe,param_1);
    uVar3 = param_1 | (astruct_365 *)param_4;
    if (uVar3 != 0x0) {
      puVar4 = pass1_1030_bc24(uVar3,(int)(astruct_365 *)param_4,param_1,0xc,_param_5);
      uVar2 = ((u32)puVar4 >> 0x10);
      param_4._0_2_ = (astruct_365 *)puVar4;
      goto LAB_1028_ef8b;
    }
  }
  else {
    paVar1 = (astruct_365 *)param_4;
    mem_op_1000_179c(0xe,param_1);
    uVar3 = param_1 | paVar1;
    if (uVar3 != 0x0) {
      puVar4 = pass1_1028_b22c(uVar3,(u16 *)CONCAT22(param_1,paVar1),(astruct_365 *)param_4,_param_5);
      uVar2 = ((u32)puVar4 >> 0x10);
      param_4._0_2_ = (astruct_365 *)puVar4;
      goto LAB_1028_ef8b;
    }
  }
  param_4._0_2_ = NULL;
  uVar2 = 0x0;//
LAB_1028_ef8b:
  pass1_1030_1358(*(astruct_291 **)((int)param_2 + 0x22),(astruct_365 *)param_4,uVar2,
                  (u32)&((astruct_365 *)param_4)->field_0x4 & 0xffff |
                  (u32)(&((astruct_365 *)param_4)->field_0x6 & 0xff) << 0x10);
  return;
}
