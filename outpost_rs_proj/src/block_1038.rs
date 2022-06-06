//
// Created by cyrex on 2022-05-22.
//

#include "block_1038.h"
pub fn pass1_1038_0000(mut param_1: u16 ,u8 *param_2,mut param_3: u32)

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

    // Segment:    8
    // Offset:     000606c0
    // Length:     ef91
    // Min Alloc:  ef91
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
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
    *puStack10 = 0xb96;
    (param_1 + 0x2) = (int)&u16_1050_1038;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_008e(u8 *param_1,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar8;
  let mut iVar9: i16;
  let mut uVar10: u16;
  u32 *puVar11;
  astruct_27 *paVar12;
  let mut in_stack_0000fe7c: u16;
  let mut in_stack_0000ffa0: u16;
  let mut in_stack_0000ffa6: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffd4: u16;
  let mut iStack32: i16;
  let mut iStack12: i16;
  let mut uVar7: u32;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar10 = (param_4 >> 0x10);
  iVar9 = (int)param_4;
  if (*(i32 *)(iVar9 + 0x4) != 0x4000001) {
    return;
  }
  puVar11 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffd4,0x2c),in_stack_0000fe7c,
                            in_stack_0000ffa0,in_stack_0000ffa6,in_stack_0000ffaa);
  paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)puVar11 >> 0x10);
  uVar3 = puVar11;
  uVar4 = ((u32)puVar11 >> 0x10);
  uVar5 = uVar3;
  pass1_1008_612e(uVar3,0x1,0x64);
  iStack12 = 0x0;
  iVar1 = (uVar3 + 0xa);
  if (iVar1 == 0x1) {
    iStack12 = 0x15;
  }
  else if (iVar1 != 0x2) {
    if (iVar1 == 0x3) {
      iStack12 = 0x16;
    }
    else if (iVar1 == 0x4) {
      if ((int)uVar5 < 0x32) {
        iStack12 = 0x17;
      }
      else {
        iStack12 = 0x18;
      }
    }
    else if (iVar1 == 0x5) {
      iStack12 = 0x19;
    }
  }
  if (iStack12 != 0x0) {
    paVar12 = (astruct_27 *)
              mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffd4,0x2b),in_stack_0000fe7c,
                              in_stack_0000ffa0,in_stack_0000ffa6,in_stack_0000ffaa);
    paVar8 = (astruct_57 *)((u32)paVar12 >> 0x10);
    pass1_1010_043a(paVar12,*(i32 *)(iVar9 + 0x4),iStack12);
  }
  pass1_1008_eb74((u8 *)paVar8,puVar11,0x0);
  if ((((uVar3 + 0xe) | (uVar3 + 0xc)) == 0x0) && ((iVar9 + 0x18) < 0xc9)) {
    uVar2 = *_PTR_LOOP_1050_65e2;
    uVar5 = uVar2;
    uVar7 = uVar2;
    pass1_1008_612e(uVar5,0x0,0x8);
    uVar6 = uVar7;
    iStack32 = (int)((u32)uVar2 >> 0x10);
    (uVar3 + 0xc) = uVar6 + uVar5 + 0x1e;
    (uVar3 + 0xe) = ((int)uVar6 >> 0xf) + iStack32 + CARRY2(uVar6,uVar5) + (0xffe1 < uVar6 + uVar5);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_01c0(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut iVar1: i16;
  u32 *puVar2;
  code **ppcVar3;
  let mut uVar4: u32;
  astruct_419 *paVar5;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut BVar8: bool;
  u8 *puVar9;
  u32 *puVar10;
  let mut uVar11: u32;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar15;
  let mut uVar16: u16;
  let mut uVar17: u16;
  u32 *puVar18;
  let mut uVar19: u32;
  let mut uVar20: u32;
  u8 uVar21;
  let mut uStack50: u32;
  let mut uStack30: u32;
  let mut uStack18: u32;
  u8 local_e [0x2];
  u32 *puStack12;
  let mut uStack8: u16;
  let mut uStack6: u16;
  let mut iStack4: i16;

  uVar16 = ((u32)in_EDX >> 0x10);
  iStack4 = 0x0;
  puVar18 = pass1_1008_c6fa(_u16_1050_06e0,0x29);
  uStack6 = ((u32)puVar18 >> 0x10);
  paVar15 = (astruct_57 *)CONCAT22(uVar16,uStack6);
  uVar7 = puVar18;
  uStack8 = uVar7;
  pass1_1038_4e78(uVar7,paVar15,param_3,puVar18);
  puStack12 = (u32 *)CONCAT22((int)paVar15,uVar7);
  uVar17 = 0x1030;
  uVar19 = pass1_1030_bcae(local_e,&DAT_1050_1050);
  uVar12 = (uVar19 >> 0x10);
  uVar16 = uVar19;
  ppcVar3 = (code **)((int)*puStack12 + 0x10);
  (**ppcVar3)(0x1030,(int)puStack12,(int)((u32)puStack12 >> 0x10));
  uStack18 = CONCAT22(uVar12,uVar16);
  uVar16 = (param_3 >> 0x10);
  puVar2 = (u32 *)(u32)((int)param_3 + 0xc);
  uVar7 = ((int)param_3 + 0xe);
  uVar19 = (u32)uVar7;
  uVar21 = SUB41(puVar2,0x0);
  ppcVar3 = (code **)((int)*puVar2 + 0x10);
  puVar10 = puVar2;
  (**ppcVar3)();
  uVar4 = (u32)puVar10 & 0xffff | uVar19 << 0x10;
  uStack30 = 0x0;
  do {
    if (uStack18 <= uStack30) {
      if (puStack12 != NULL) {
        ppcVar3 = (code **)*puStack12;
        (**ppcVar3)(uVar17,(int)puStack12,(int)((u32)puStack12 >> 0x10),0x1,uVar21,uVar7);
      }
      return;
    }
    uVar17 = 0x1030;
    uVar11 = uStack18;
    pass1_1030_1d58((u32)puStack12);
    uVar6 = uVar19 << 0x10;
    iVar1 = ((int)uVar11 + 0x10);
    for (uStack50 = 0x0; uVar13 = uVar19, uStack50 < uVar4; uStack50 += 0x1) {
      uVar17 = 0x1030;
      uVar20 = uVar4;
      pass1_1030_1d58((u32)puVar2);
      paVar5 = (astruct_419 *)(uVar20 & 0xffff | (u32)uVar13 << 0x10);
      uVar14 = uVar13 | uVar20;
      uVar19 = (u32)uVar14;
      if ((uVar14 != 0x0) && (uVar19 = (u32)uVar13, (uVar20 + 0x10) == iVar1)) {
        uVar20 = struct_op_1030_73a8(paVar5,iVar1,uVar13);
        uVar19 = uVar20 >> 0x10;
        uVar17 = 0x1008;
        BVar8 = pass1_1008_c6ae((u32)_u16_1050_06e0,((int)uVar20 + 0xc),0x30);
        if (BVar8 == 0x0) {
          puVar9 = local_e;
          uVar17 = 0x1030;
          pass1_1030_bd74(puVar9,&DAT_1050_1050,(u32)paVar5,(astruct_670 *)(uVar11 & 0xffff | uVar6));
          if ((int)puVar9 < 0x6) {
            iStack4 += 0x1;
            break;
          }
        }
      }
    }
    uStack30 += 0x1;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_0340(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16,mut param_5: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut iVar3: i16;
  let mut uVar4: u16;
  u8 local_13a [0x11c];
  let mut uStack30: u32;
  let mut uStack26: u32;
  let mut uStack22: u32;
  let mut local_12: u16;
  let mut uStack16: u16;
  let mut local_e: i16;
  let mut uStack12: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uStack6 = *_PTR_LOOP_1050_65e2;
  uStack10 = 0x0;
  uStack12 = 0x0;
  iVar3 = (int)param_5;
  uVar4 = (param_5 >> 0x10);
  pass1_1038_4cea(param_5,(u32 *)CONCAT22(0x1050,&local_12),(u16 *)CONCAT22(0x1050,&local_e));
  uVar2 = (u32)(iVar3 + 0x1f6);
  uStack22 = uVar2;
  pass1_1030_38b8();
  uVar1 = uVar2;
  uStack26 = uVar2 & 0xffff | (u32)param_1 << 0x10;
  if (param_4 == 0x0) {
    if (local_e != 0x8) {
      uStack10 = (long)(uVar2 & 0xffff | (u32)param_1 << 0x10) / 0x4;
      uStack12 = 0x8;
      goto LAB_1038_054b;
    }
  }
  else if (param_4 < 0xb) {
    if (local_e != 0x7) {
      uStack10 = (long)(uVar2 & 0xffff | (u32)param_1 << 0x10) / 0xa;
      uStack12 = 0x7;
      goto LAB_1038_054b;
    }
  }
  else if (param_4 < 0x1a) {
    if (local_e != 0x6) {
      uStack10 = (long)(uVar2 & 0xffff | (u32)param_1 << 0x10) / 0x14;
      uStack12 = 0x6;
      goto LAB_1038_054b;
    }
  }
  else if (param_4 < 0x33) {
    if (local_e != 0x5) {
      uStack10 = (long)(uVar2 & 0xffff | (u32)param_1 << 0x10) / 0x64;
      uStack12 = 0x5;
      goto LAB_1038_054b;
    }
  }
  else if (param_4 < 0x4c) {
    if (uStack6 % 0x3 != 0x0) goto LAB_1038_054b;
    if (local_e != 0x4) {
      uStack10 = (long)uStack26 / 0x64;
      uStack12 = 0x4;
      goto LAB_1038_054b;
    }
  }
  else if (param_4 < 0x65) {
    if (uStack6 % 0x5 != 0x0) goto LAB_1038_054b;
    if (local_e != 0x3) {
      uStack10 = (long)uStack26 / 0x64;
      uStack12 = 0x3;
      goto LAB_1038_054b;
    }
  }
  else if (param_4 < 0x97) {
    if (uStack6 % 0xa != 0x0) goto LAB_1038_054b;
    if (local_e != 0x2) {
      uStack10 = (long)uStack26 / 0x64;
      uStack12 = 0x2;
      goto LAB_1038_054b;
    }
  }
  else {
    if ((0xc8 < param_4) || (uStack6 % 0x14 != 0x0)) goto LAB_1038_054b;
    if (local_e != 0x1) {
      uStack10 = (long)uStack26 / 0x64;
      uStack12 = 0x1;
      goto LAB_1038_054b;
    }
  }
  if (((int)uStack16 <= (int)param_1) && (((int)uStack16 < (int)param_1 || (local_12 < uVar1)))) {
    uVar1 = local_12;
    param_1 = uStack16;
  }
  uStack10 = CONCAT22(param_1,uVar1);//
LAB_1038_054b:
  if (uStack12 != 0x0) {
    if ((uStack26 != 0x0) && (uStack10 == 0x0)) {
      uStack10 = 0x1;
    }
    pass1_1038_4cd0(param_5,uStack10,uStack12);
  }
  if ((uStack10 | uStack10) != 0x0) {
    if (*(i32 *)(iVar3 + 0x200) == 0x8000001) {
      uStack30._0_2_ = 0x2;
    }
    else {
      uStack30._0_2_ = 0x1;
    }
    uStack30 = CONCAT22(0x400,uStack30);
    pass1_1028_9944((astruct_97 *)CONCAT22(0x1050,local_13a),uStack10,CONCAT22(0x400,uStack30),
                    (u32)(iVar3 + 0x4));
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_13a));
    pass1_1028_9992((u16 *)CONCAT22(0x1050,local_13a));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_05d8(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16,mut param_5: u32)

{
  astruct_92 *paVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  u8 local_158 [0x118];
  let mut uStack64: u32;
  astruct_92 local_34;
  let mut uStack30: u32;
  astruct_92 *paStack22;
  let mut uStack20: u16;
  let mut local_12: u32;
  let mut local_e: i16;
  let mut uStack12: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uStack6 = *_PTR_LOOP_1050_65e2;
  uStack10 = 0x0;
  uStack12 = 0x0;
  pass1_1038_4cea(param_5,(u32 *)CONCAT22(0x1050,&local_12),(u16 *)CONCAT22(0x1050,&local_e));
  paStack22 = NULL;
  uStack20 = 0x0;
  uStack30 = 0x0;
  pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_34)),0x1,0x0,0x400);
  do {
    do {
      uVar3 = param_1;
      paVar1 = &local_34;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar1));
      uVar4 = uVar3 | paVar1;
      param_1 = (u32)uVar4;
      if (uVar4 == 0x0) goto LAB_1038_0668;
    } while (paVar1[0x1c].field4_0x8 != 0x8000002);
    uVar2 = (u32)&paVar1[0x1b].field6_0x10;
    pass1_1030_38b8();
    uStack30 = uVar2 & 0xffff | (u32)uVar4 << 0x10;
    uVar4 |= uVar2;
    param_1 = (u32)uVar4;
    paStack22 = paVar1;
    uStack20 = uVar3;
  } while (uVar4 == 0x0);//
LAB_1038_0668:
  local_34._0_2_ = 0x389a;
  local_34.field2_0x2 = 0x1008;
  if ((uStack20 | paStack22) == 0x0) {
    return;
  }
  if (param_4 == 0x3e8) {
    if (local_e != 0x10) {
      uStack10 = (long)uStack30 / 0x4;
      uStack12 = 0x10;
      goto LAB_1038_0841;
    }
  }
  else if (param_4 < 0x3de) {
    if (param_4 < 0x3cf) {
      if (param_4 < 0x3b6) {
        if (param_4 < 0x39d) {
          if (param_4 < 0x384) {
            if (param_4 < 0x352) {
              if ((param_4 < 0x320) || (uStack6 % 0x14 != 0x0)) goto LAB_1038_0841;
              if (local_e != 0x9) {
                uStack10 = (long)uStack30 / 0x64;
                uStack12 = 0x9;
                goto LAB_1038_0841;
              }
            }
            else {
              if (uStack6 % 0xa != 0x0) goto LAB_1038_0841;
              if (local_e != 0xa) {
                uStack10 = (long)uStack30 / 0x64;
                uStack12 = 0xa;
                goto LAB_1038_0841;
              }
            }
          }
          else {
            if (uStack6 % 0x5 != 0x0) goto LAB_1038_0841;
            if (local_e != 0xb) {
              uStack10 = (long)uStack30 / 0x64;
              uStack12 = 0xb;
              goto LAB_1038_0841;
            }
          }
        }
        else {
          if (uStack6 % 0x3 != 0x0) goto LAB_1038_0841;
          if (local_e != 0xc) {
            uStack10 = (long)uStack30 / 0x64;
            uStack12 = 0xc;
            goto LAB_1038_0841;
          }
        }
      }
      else if (local_e != 0xd) {
        uStack10 = (long)uStack30 / 0x64;
        uStack12 = 0xd;
        goto LAB_1038_0841;
      }
    }
    else if (local_e != 0xe) {
      uStack10 = (long)uStack30 / 0x14;
      uStack12 = 0xe;
      goto LAB_1038_0841;
    }
  }
  else if (local_e != 0xf) {
    uStack10 = (long)uStack30 / 0xa;
    uStack12 = 0xf;
    goto LAB_1038_0841;
  }
  uVar2 = uStack30;
  if ((long)local_12 < (long)uStack30) {
    uVar2 = local_12 & 0xffff;
    uStack30 = local_12;
  }
  uStack10 = uVar2 & 0xffff | (u32)uStack30 << 0x10;//
LAB_1038_0841:
  if (uStack12 != 0x0) {
    if ((uStack30 != 0x0) && (uStack10 == 0x0)) {
      uStack10 = 0x1;
    }
    pass1_1038_4cd0(param_5,uStack10,uStack12);
  }
  if ((uStack10 | uStack10) != 0x0) {
    uVar5 = (param_5 >> 0x10);
    if (*(i32 *)((int)param_5 + 0x200) == 0x8000001) {
      uStack64 = paStack22.field3_0x4;
    }
    else {
      uStack64 = 0x4000001;
    }
    pass1_1028_9944((astruct_97 *)CONCAT22(0x1050,local_158),uStack10,(u32)((int)param_5 + 0x4),uStack64);
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_158));
    pass1_1028_9992((u16 *)CONCAT22(0x1050,local_158));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1038_08d4(uchar param_1,mut param_2: u32,mut param_3: u16 ,i32 param_4,mut param_5: u32)

{
  let mut bVar1: bool;
  astruct_92 *paVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  astruct_92 local_16;

  bVar1 = false;
  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_16),0x1,0x0,0x400);
  do {
    paVar2 = &local_16;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar2));
    uVar3 = param_2;
    uVar4 = uVar3 | paVar2;
    param_2 = param_2 & 0xffff0000 | (u32)uVar4;
    if (uVar4 == 0x0) goto LAB_1038_0917;
  } while (paVar2[0x1c].field4_0x8 != 0x8000002);
  bVar1 = true;//
LAB_1038_0917:
  local_16._0_2_ = 0x389a;
  local_16.field2_0x2 = 0x1008;
  if (bVar1) {
    if (param_4 < 0xc90000) {
      pass1_1038_0340(uVar4,param_3,param_4,param_4,param_5);
      return;
    }
    if (0x31fffff < param_4) {
      pass1_1038_05d8(param_2,param_3,param_4,param_4,param_5);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_095e(StructD *param_1,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16,mut param_5: u32,mut param_6: u16 )

{
  code **ppcVar1;
  astruct_419 *paVar2;
  let mut bVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut uVar6: u32;
  u8 *puVar7;
  let mut uVar8: u32;
  astruct_57 *paVar9;
  let mut uVar10: u16;
  u32 *puVar11;
  astruct_15 *paVar12;
  let mut in_stack_0000fe60: u16;
  let mut in_stack_0000ff84: u16;
  let mut in_stack_0000ff8a: u16;
  let mut in_stack_0000ff8e: u16;
  let mut iVar13: i16;
  let mut uStack58: u32;
  let mut uStack54: u32;
  u8 local_28 [0x2];
  let mut uStack38: u32;
  let mut uStack34: u32;
  u32 *puStack30;
  let mut uStack26: u16;
  u8 *puStack24;
  u32 *puStack22;
  let mut uStack18: u32;
  let mut iStack14: i16;
  let mut iStack12: i16;
  let mut uStack10: u32;
  astruct_67 *paStack6;

  paStack6 = (astruct_67 *)
             mixed_1010_20ba((astruct_57 *)param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_6,0x37),in_stack_0000fe60
                             ,in_stack_0000ff84,in_stack_0000ff8a,in_stack_0000ff8e);
  uStack10 = *_PTR_LOOP_1050_65e2;
  uVar8 = uStack10 % 0xa;
  uVar10 = (param_5 >> 0x10);
  if (uVar8 == 0x0) {
    if (param_4 < 0xc9) {
      iVar13 = 0x3f;
    }
    else {
      if (param_4 < 0x320) goto LAB_1038_09c3;
      iVar13 = 0x3e;
    }
    post_win_msg_1008_a0e4(paStack6,0x0,0x0,0x1,(u32)((int)param_5 + 0x4),iVar13);
  }//
LAB_1038_09c3:
  iStack12 = ((int)param_5 + 0x22);
  iStack14 = 0x0;
  uStack18 = *_PTR_LOOP_1050_65e2;
  uVar8 &= 0xffff0000;
  if (iStack12 < 0x4b) {
    if (iStack12 < 0x3c) {
      if (iStack12 < 0x32) goto LAB_1038_0a1c;
      uVar4 = 0x1e;
    }
    else {
      uVar4 = 0xf;
    }
  }
  else {
    uVar4 = 0x5;
  }
  uVar8 = (uStack18 & 0xffff | (u32)((int)_PTR_LOOP_1050_65e2 + 0x2) << 0x10) % (u32)uVar4;
  if (uVar8 == 0x0) {
    iStack14 = 0x1;
  }//
LAB_1038_0a1c:
  uVar10 = (uVar8 >> 0x10);
  if (iStack14 != 0x0) {
    puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0xf);
    paVar9 = (astruct_57 *)CONCAT22(uVar10,(int)((u32)puVar11 >> 0x10));
    uVar4 = puVar11;
    pass1_1038_4e78(uVar4,paVar9,param_5,puVar11);
    puStack22 = (u32 *)CONCAT22((int)paVar9,uVar4);
    puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0x1a);
    puVar7 = (u8 *)((u32)puVar11 >> 0x10);
    uVar4 = puVar11;
    uStack26 = uVar4;
    puStack24 = puVar7;
    pass1_1038_4d6e(uVar4,puVar7,(astruct_691 *)param_5,puVar11);
    puStack30 = (u32 *)CONCAT22(puVar7,uVar4);
    ppcVar1 = (code **)((int)*puStack22 + 0x10);
    (**ppcVar1)(0x1008,(int)puStack22,(int)((u32)puStack22 >> 0x10));
    uStack34 = CONCAT22(puVar7,uVar4);
    ppcVar1 = (code **)((int)*puStack30 + 0x10);
    (**ppcVar1)(0x1008,(char)puStack30,(int)((u32)puStack30 >> 0x10));
    uStack38 = CONCAT22(puVar7,uVar4);
    paVar12 = (astruct_15 *)pass1_1030_bcae(local_28,&DAT_1050_1050);
    uStack54 = 0x0;
    while( true ) {
      uVar8 = (u32)paVar12 >> 0x10;
      uVar10 = 0x1030;
      if (uStack34 <= uStack54) break;
      uVar6 = uStack34;
      pass1_1030_1d58((u32)puStack22);
      paVar2 = (astruct_419 *)(uVar6 & 0xffff | uVar8 << 0x10);
      bVar3 = false;
      uStack58 = 0x0;
      while( true ) {
        uVar10 = uVar8;
        uVar6 = uStack38;
        if (uStack38 <= uStack58) break;
        uVar5 = uStack38;
        pass1_1030_1d58((u32)puStack30);
        uVar6 = ZEXT24(local_28);
        pass1_1030_bd74(local_28,&DAT_1050_1050,(u32)paVar2,
                        (astruct_670 *)(uVar5 & 0xffff | uVar8 << 0x10));
        uVar10 = uVar8;
        if ((int)uVar6 < 0x6) {
          bVar3 = true;
          break;
        }
        uStack58 += 0x1;
      }
      paVar12 = (astruct_15 *)struct_op_1030_73a8(paVar2,(int)uVar6,uVar10);
      if (!bVar3) {
        uVar10 = 0x1028;
        FUN_1028_5ca0((int)paVar12,0x1030,paVar12);
        break;
      }
      uStack54 += 0x1;
    }
    if (puStack22 != NULL) {
      ppcVar1 = (code **)*puStack22;
      (**ppcVar1)(uVar10,(int)puStack22,(char)((u32)puStack22 >> 0x10),0x1);
    }
    if (puStack30 != NULL) {
      ppcVar1 = (code **)*puStack30;
      (**ppcVar1)(uVar10,(int)puStack30,(char)((u32)puStack30 >> 0x10),0x1);
    }
  }
  return;
}



StructD * pass1_1038_0b6a(StructD *param_1,u8 param_2)

{
  param_1.address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_97 *
pass1_1038_0ba6(param_1: *mut astruct_97,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,
               mut param_6: u16 )

{
  astruct_57 *paVar1;
  astruct_97 *iVar2;
  let mut unaff_BP: u16;
  let mut uVar2: u16;
  astruct_97 *paVar3;
  u32 *puVar4;

  uVar2 = ((u32)param_2 >> 0x10);
  paVar3 = struct_op_1028_d1dc(param_1,0x270f);
  paVar1 = (astruct_57 *)CONCAT22(uVar2,(int)((u32)paVar3 >> 0x10));
  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_97 *)param_1;
  (u32)&iVar2.field259_0x108 = 0x0;
  param_1.offset_0x0 = 0x1c2e;
    // just 0x1038
  iVar2.segment_0x2 = &u16_1050_1038;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar2.string_0x8)),s_SCMove_1050_59d8);
  puVar4 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x2),param_3,param_4,param_5,param_6);
  iVar2.field259_0x108 = puVar4;
  &iVar2.field_0x10a = (int)((u32)puVar4 >> 0x10);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_0c00(uchar param_1,mut param_2: u32,mut param_3: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  astruct_92 *paVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  u8 *puVar7;
  u8 *puVar8;
  let mut uVar9: u32;
  let mut uVar10: u16;
  u32 *puVar11;
  u32 *puStack32;
  let mut uStack24: u32;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_14)),0x1,0x0,0x400);
  while( true ) {
    paVar3 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar3));
    uVar6 = param_2;
    uStack24 = CONCAT22(uVar6,paVar3);
    uVar9 = param_2 & 0xffff0000 | (u32)(uVar6 | paVar3);
    if ((uVar6 | paVar3) == 0x0) break;
    pass1_1038_0e78(param_3,CONCAT22(uVar6,paVar3));
    pass1_1038_1220(uVar9,param_3,CONCAT22(uVar6,paVar3));
    uVar10 = (uVar9 >> 0x10);
    puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0x1);
    puVar7 = (u8 *)((u32)puVar11 >> 0x10);
    uVar4 = puVar11;
    pass1_1038_4d6e(uVar4,puVar7,(astruct_691 *)CONCAT22(uVar6,paVar3),puVar11);
    puStack32 = (u32 *)CONCAT22(puVar7,uVar4);
    ppcVar1 = (code **)((int)*puStack32 + 0x10);
    uVar5 = uVar4;
    puVar8 = puVar7;
    (**ppcVar1)(0x1008,uVar4,puVar7);
    param_2 = CONCAT22(uVar10,puVar8 | uVar5);
    if ((puVar8 | uVar5) != 0x0) {
      uVar2 = (u32)((int)param_3 + 0x108);
      if (((int)uVar2 + 0x82) != 0x0) {
        pass1_1038_19a0(param_3,(u32 *)CONCAT22(puVar7,uVar4),CONCAT22(uVar6,paVar3),(int)&DAT_1050_1050,param_1);
      }
      pass1_1038_1940(param_3,(u32 *)CONCAT22(puVar7,uVar4),uStack24);
    }
    if (puStack32 != NULL) {
      ppcVar1 = (code **)*puStack32;
      (**ppcVar1)(0x8,uVar4,puVar7,0x1);
    }
    pass1_1038_1c3e(param_3,uStack24);
  }
  return;
}
pub fn pass1_1038_0cf0(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

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
  mem_op_1000_179c(0x10c,paVar6);
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
    (u32)(param_1 + 0x108) = (u32)(iVar7 + 0x108);
    *puStack10 = 0x1c2e;
    (param_1 + 0x2) = (int)&u16_1050_1038;
  }
  return;
}
pub fn pass1_1038_0d8e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  i32 lStack10;
  let mut uStack4: u16;

  uVar1 = pass1_1030_d0a8(param_4);
  uVar2 = pass1_1030_d144(param_4);
  lStack10 = (long)(int)uVar2;
  uVar2 = (int)uVar2 >> 0xf | uVar2;
  uStack4 = uVar1;
  if (uVar2 != 0x0) {
    do {
      uVar3 = pass1_1028_6744(param_3,uStack4);
      uVar2 |= uVar3;
      if (uVar2 != 0x0) {
        pass1_1028_6228(param_3,0x1,0x0,uStack4);
        lStack10 += -0x1;
        pass1_1030_d180(param_4,uStack4);
      }
      if (lStack10 == 0x0) {
        return;
      }
      uStack4 = pass1_1030_d0a8(param_4);
    } while (uStack4 != uVar1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_0e00(mut param_1: u16 ,mut param_2: u32,u32 *param_3,mut param_4: u32)

{
  code **ppcVar1;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  ppcVar1 = (code **)((int)*param_3 + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,param_1);
  for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
    ppcVar1 = (code **)((int)*param_3 + 0x4);
    uVar4 = uStack6;
    (**ppcVar1)();
    uVar3 = uVar4;
    uVar2 = extraout_DX_00 | uVar3;
    if (uVar2 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | (u32)extraout_DX_00 << 0x10);
      uVar4 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar2,uVar3),uVar3,uVar2);
      uVar3 = (uVar4 >> 0x10);
      if ((uVar3 | uVar4) != 0x0) {
        pass1_1038_0d8e(param_2,(param_2 >> 0x10),uVar4 & 0xffff | (u32)uVar3 << 0x10,param_4);
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_0e78(mut param_1: u32,mut param_2: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 *puVar5;
  let mut extraout_DX: u16;
  u8 *puVar6;
  let mut extraout_DX_00: u16;
  let mut extraout_DX_01: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  u32 *puVar9;
  let mut uVar10: u32;
  let mut uStack22: u32;
  let mut uStack18: u32;
  u32 *puStack14;
  u32 *puStack10;

  puVar9 = pass1_1008_c6fa(_u16_1050_06e0,0x4);
  puVar5 = (u8 *)((u32)puVar9 >> 0x10);
  uVar2 = puVar9;
  pass1_1038_4d6e(uVar2,puVar5,(astruct_691 *)param_2,puVar9);
  puStack10 = (u32 *)CONCAT22(puVar5,uVar2);
  uVar10 = *puStack10;
  ppcVar1 = (code **)uVar10 + 0x8;
  uVar3 = uVar2;
  (**ppcVar1)(0x1008,uVar2,puVar5);
  if ((extraout_DX | uVar3) == 0x0) {
    if (puStack10 != NULL) {
      ppcVar1 = (code **)uVar10;
      (**ppcVar1)(0x8,uVar2,(char)puVar5,0x1);
      return;
    }
  }
  else {
    uVar8 = 0x1008;
    puVar9 = pass1_1008_c6fa(_u16_1050_06e0,0x1e);
    puVar6 = (u8 *)((u32)puVar9 >> 0x10);
    uVar3 = puVar9;
    pass1_1038_4d6e(uVar3,puVar6,(astruct_691 *)param_2,puVar9);
    puStack14 = (u32 *)CONCAT22(puVar6,uVar3);
    ppcVar1 = (code **)((int)*puStack14 + 0x10);
    uVar4 = uVar3;
    (**ppcVar1)(0x1008,(char)uVar3,puVar6);
    uStack18 = CONCAT22(extraout_DX_00,uVar4);
    for (uStack22 = 0x0; uStack22 < uStack18; uStack22 += 0x1) {
      ppcVar1 = (code **)((int)*puStack14 + 0x4);
      uVar10 = uStack18;
      (**ppcVar1)();
      uVar4 = uVar10;
      uVar7 = extraout_DX_01 | uVar4;
      if (uVar7 != 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar10 & 0xffff | (u32)extraout_DX_01 << 0x10);
        uVar8 = 0x1030;
        uVar10 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar7,uVar4),uVar4,uVar7);
        if (((uVar10 >> 0x10) | uVar10) != 0x0) {
          pass1_1038_0e00(uVar10,param_1,puStack10,uVar10);
        }
      }
    }
    if (puStack10 != NULL) {
      ppcVar1 = (code **)*puStack10;
      (**ppcVar1)(uVar8,uVar2,(char)puVar5,0x1);
    }
    if (puStack14 != NULL) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(uVar8,uVar3,(char)puVar6,0x1);
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_0f8c(mut param_1: u16 ,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,u32 *param_5,mut param_6: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  code **ppcVar5;
  let mut uVar6: u32;
  qword qVar7;
  u8 *puVar8;
  let mut uVar9: u32;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut iVar13: i16;
  let mut uVar14: u16;
  let mut unaff_CS: u16;
  u32 *puVar15;
  astruct_99 *paStack80;
  let mut uStack76: u16;
  u8 local_30 [0x4];
  let mut uStack44: u32;
  u32 *puStack40;
  let mut uStack36: u32;
  u8 local_20 [0x4];
  u32 *puStack28;
  let mut uStack24: u16;
  let mut uStack22: u16;
  let mut uStack20: u16;
  let mut uStack18: u16;
  let mut uStack16: u32;
  let mut uStack12: u32;
  let mut uStack8: u16;
  let mut uStack6: u32;

  uStack6 = 0x64;
  uStack8 = 0x0;
  ppcVar5 = (code **)((int)*param_5 + 0x10);
  puVar15 = param_5;
  (**ppcVar5)();
  uStack12 = CONCAT22((int)param_2,param_1);
  uStack16 = 0x0;
  do {
    if (uStack12 <= uStack16) {
      return;
    }
    ppcVar5 = (code **)((int)*param_5 + 0x4);
    uVar9 = uStack12;
    uVar11 = param_2;
    (**ppcVar5)(unaff_CS,param_5,(char)uStack16,(int)(uStack16 >> 0x10),puVar15);
    uStack18 = uVar11;
    uVar12 = uVar9;
    uVar11 = uStack18 | uVar12;
    param_2 = (u32)uVar11;
    uStack20 = uVar12;
    if (uVar11 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar9 & 0xffff | (u32)uStack18 << 0x10);
      uStack22 = uVar11;
      unaff_CS = 0x1030;
      uStack24 = uVar12;
      puStack28 = (u32 *)struct_op_1030_73a8((astruct_419 *)CONCAT22(uStack22,uVar12),uVar12,uStack22);
      param_2 = (u32)puStack28 >> 0x10;
      puVar8 = local_20;
      ppcVar5 = (code **)((int)*puStack28 + 0x40);
      (**ppcVar5)(0x1030,(int)puStack28,(int)((u32)puStack28 >> 0x10),(char)puVar8,(int)&DAT_1050_1050);
      if (puVar8 == NULL) {
        uStack36 = pass1_1028_62c8((u32)puStack28);
        uVar9 = uStack36 >> 0x10;
        uStack8 = 0x1;
        puStack40 = (u32 *)(u32)((int)param_6 + 0x22);
        pass1_1008_5784((char *)CONCAT22(0x1050,local_30),(u32)puStack40);
        while( true ) {
          uVar11 = uVar9;
          puVar8 = local_30;
          unaff_CS = 0x1008;
          pass1_1008_5b12((char *)CONCAT22(0x1050,puVar8));
          param_2 = (u32)(uVar11 | puVar8);
          if ((uVar11 | puVar8) == 0x0) break;
          uVar2 = (puVar8 + 0x4);
          uVar3 = (puVar8 + 0x6);
          uVar4 = (puVar8 + 0x8);
          uVar12 = (puVar8 + 0xa);
          uVar6 = (u32)(puVar8 + 0xc) / (u32)uVar12;
          uVar9 = uStack36;
          if (uStack6 < uStack36) {
            uVar9 = uStack6 & 0xffff;
            uStack36 = uStack6;
          }
          uVar10 = uStack36 | uVar9;
          param_2 = (u32)uVar10;
          if (uVar10 == 0x0) break;
          qVar7 = (qword)(uVar9 & 0xffff | (u32)uStack36 << 0x10) / (qword)uVar6;
          param_2 = (u32)qVar7 >> 0x10;
          uStack76 = qVar7;
          if (uStack76 == 0x0) break;
          if (uStack76 < uVar12) {
            piVar1 = (i16 *)(puVar8 + 0xc);
            *piVar1 = *piVar1 - uVar9;
            piVar1 = (i16 *)(puVar8 + 0xa);
            *piVar1 = *piVar1 - uStack76;
          }
          else {
            ppcVar5 = (code **)((int)*puStack40 + 0xc);
            (**ppcVar5)(0x1008,(int)puStack40,(int)((u32)puStack40 >> 0x10),(char)puVar8,uVar11);
            uStack44 = 0x0;
            uStack76 = uVar12;
          }
          paStack80 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
          uVar12 = ((u32)paStack80 >> 0x10);
          uVar11 = paStack80;
          if ((uVar12 | uVar11) == 0x0) {
            paStack80 = NULL;
          }
          else {
            paStack80.field0_0x0 = 0x389a;
            (uVar11 + 0x2) = 0x1008;
            (uVar11 + 0x4) = 0x0;
            (uVar11 + 0x6) = 0x0;
            (uVar11 + 0x8) = 0x0;
            (uVar11 + 0xa) = 0x0;
            (uVar11 + 0xc) = 0x0;
            paStack80.field0_0x0 = 0x56ce;
            (uVar11 + 0x2) = 0x1018;
          }
          uVar14 = ((u32)paStack80 >> 0x10);
          iVar13 = (int)paStack80;
          (iVar13 + 0xa) = uStack76;
          uVar6 = uStack76 * uVar6;
          uVar9 = uVar6 >> 0x10;
          (iVar13 + 0xc) = (int)uVar6;
          (iVar13 + 0x4) = uVar2;
          (iVar13 + 0x6) = uVar3;
          (iVar13 + 0x8) = uVar4;
          pass1_1028_6408((u32)puStack28,(u32 *)paStack80);
        }
      }
      else {
        ppcVar5 = (code **)((int)*param_5 + 0x8);
        (**ppcVar5)(0x1030,param_5,0x0,0x0,(char)uStack16,(int)(uStack16 >> 0x10));
      }
    }
    uStack16 += 0x1;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_11b0(mut param_1: u16 ,mut param_2: u32,mut param_3: u32,u32 *param_4,u32 *param_5)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  ppcVar1 = (code **)((int)*param_5 + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22((int)param_2,param_1);
  uStack10 = 0x0;
  while( true ) {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar1 = (code **)((int)*param_5 + 0x4);
    uVar4 = uStack6;
    (**ppcVar1)();
    uVar2 = uVar4;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | param_2 << 0x10);
    uVar4 = struct_op_1030_73a8((astruct_419 *)CONCAT22((int)param_2,uVar2),uVar2,(int)param_2);
    param_2 = param_2 & 0xffff0000 | uVar4 >> 0x10;
    uVar3 = uVar4;
    pass1_1038_0f8c(uVar3,param_2,param_3,(param_3 >> 0x10),param_4,uVar4);
    if (uVar3 == 0x0) break;
    uStack10 += 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_1220(mut param_1: u32,mut param_2: u32,mut param_3: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  u8 *puVar6;
  u8 *puVar7;
  u8 *puVar8;
  let mut uVar10: u16;
  let mut uVar9: u32;
  u32 *puVar11;
  u8 uVar12;
  u32 *puStack14;
  u32 *puStack10;

  uVar10 = (param_1 >> 0x10);
  puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0x4);
  puVar6 = (u8 *)((u32)puVar11 >> 0x10);
  uVar3 = puVar11;
  pass1_1038_4d6e(uVar3,puVar6,(astruct_691 *)param_3,puVar11);
  puStack10 = (u32 *)CONCAT22(puVar6,uVar3);
  ppcVar1 = (code **)((int)*puStack10 + 0x10);
  puVar7 = puVar6;
  uVar4 = uVar3;
  (**ppcVar1)(0x1008,uVar3,puVar6);
  if ((puVar7 != NULL) || (uVar4 != 0x0)) {
    puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0x5);
    puVar8 = (u8 *)((u32)puVar11 >> 0x10);
    uVar4 = puVar11;
    pass1_1038_4d6e(uVar4,puVar8,(astruct_691 *)param_3,puVar11);
    puStack14 = (u32 *)CONCAT22(puVar8,uVar4);
    uVar12 = (u8)uVar4;
    uVar2 = *puStack14;
    ppcVar1 = (code **)uVar2 + 0x8;
    puVar7 = puVar8;
    uVar5 = uVar4;
    (**ppcVar1)(0x1008,uVar12,puVar8);
    uVar9 = CONCAT22(uVar10,puVar7);
    if (((puVar7 != NULL) || (uVar5 != 0x0)) &&
       (pass1_1038_11b0(uVar5,uVar9,param_2,(u32 *)CONCAT13((char)(puVar6 >> 0x8),CONCAT12((char)puVar6,uVar3)),
                        (u32 *)CONCAT22(puVar8,uVar4)), uVar5 == 0x0)) {
      if (puStack14 == NULL) {
        return;
      }
      ppcVar1 = (code **)uVar2;
      (**ppcVar1)(0x8,uVar12,(char)puVar8,0x1);
      return;
    }
    uVar10 = (uVar9 >> 0x10);
    if (puStack14 != NULL) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(0x8,uVar12,(char)puVar8,0x1);
    }
    puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0x6);
    puVar8 = (u8 *)((u32)puVar11 >> 0x10);
    uVar4 = puVar11;
    pass1_1038_4d6e(uVar4,puVar8,(astruct_691 *)param_3,puVar11);
    puStack14 = (u32 *)CONCAT22(puVar8,uVar4);
    ppcVar1 = (code **)((int)*puStack14 + 0x10);
    puVar7 = puVar8;
    uVar5 = uVar4;
    (**ppcVar1)(0x8,(char)uVar4,puVar8);
    if ((puVar7 != NULL) || (uVar5 != 0x0)) {
      pass1_1038_11b0(uVar5,CONCAT22(uVar10,puVar7),param_2,(u32 *)CONCAT22(puVar6,uVar3),
                      (u32 *)CONCAT22(puVar8,uVar4));
    }
    if (puStack14 != NULL) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(0x8,uVar4,(char)puVar8,0x1);
    }
  }
  if (puStack10 != NULL) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(0x8,uVar3,(char)puVar6,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_134a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,u32 *param_4,u32 *param_5,u32 *param_6)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar3: u16;
  let mut unaff_CS: u16;
  let mut uVar4: u32;
  u32 *puVar5;
  let mut uStack6: u32;

  ppcVar1 = (code **)((int)*param_6 + 0x10);
  puVar5 = param_6;
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,param_1);
  *param_4 = 0x0;
  do {
    if (uStack6 <= *param_5) {
      return;
    }
    uVar4 = *param_5;
    *param_5 = *param_5 + 0x1;
    ppcVar1 = (code **)((int)*param_6 + 0x4);
    (**ppcVar1)(unaff_CS,param_6,uVar4,puVar5);
    uVar2 = uVar4;
    uVar3 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | (u32)extraout_DX_00 << 0x10);
    uVar4 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar3,uVar2),uVar2,uVar3);
    uVar3 = (uVar4 >> 0x10);
    unaff_CS = 0x1028;
    uVar4 = pass1_1028_45e2(uVar4,uVar3,uVar4 & 0xffff | (u32)uVar3 << 0x10);
    uVar3 = (uVar4 >> 0x10);
    param_4 = (int)uVar4;
    ((int)param_4 + 0x2) = uVar3;
  } while ((uVar3 | param_4) == 0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_13da(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,u32 *param_4,u32 *param_5,u32 *param_6)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar3: u16;
  let mut unaff_CS: u16;
  let mut uVar4: u32;
  u32 *puVar5;
  let mut uStack6: u32;

  ppcVar1 = (code **)((int)*param_6 + 0x10);
  puVar5 = param_6;
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,param_1);
  *param_4 = 0x0;
  do {
    if (uStack6 <= *param_5) {
      return;
    }
    uVar4 = *param_5;
    *param_5 = *param_5 + 0x1;
    ppcVar1 = (code **)((int)*param_6 + 0x4);
    (**ppcVar1)(unaff_CS,param_6,uVar4,puVar5);
    uVar2 = uVar4;
    uVar3 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | (u32)extraout_DX_00 << 0x10);
    if ((uVar3 | uVar2) == 0x0) {
      return;
    }
    uVar4 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar3,uVar2),uVar2,uVar3 | uVar2);
    uVar3 = (uVar4 >> 0x10);
    if ((uVar3 | uVar4) == 0x0) {
      return;
    }
    unaff_CS = 0x1028;
    uVar4 = pass1_1028_3c32((u32 *)(uVar4 & 0xffff | (u32)uVar3 << 0x10));
    uVar3 = (uVar4 >> 0x10);
    param_4 = (int)uVar4;
    ((int)param_4 + 0x2) = uVar3;
  } while ((uVar3 | param_4) == 0x0);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_1482(mut param_1: u16 ,mut param_2: u32,u32 *param_3,u32 *param_4)

{
  code **ppcVar1;
  sqword sVar2;
  let mut uVar3: u16;
  u32 *puVar4;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  astruct_57 *paVar11;
  astruct_57 *paVar13;
  astruct_57 *paVar14;
  let mut uVar15: u32;
  let mut unaff_DI: u16;
  let mut uVar16: u16;
  u8 uVar17;
  u8 uVar18;
  i32 lStack74;
  let mut local_46: u32;
  u16 local_42 [0x4];
  let mut uStack58: u16;
  let mut uStack56: u16;
  u32 *puStack54;
  u32 *puStack50;
  let mut uStack46: u32;
  let mut uStack42: u16;
  let mut uStack40: u16;
  let mut uStack38: u16;
  let mut uStack36: u16;
  let mut uStack34: u32;
  let mut uStack30: u16;
  let mut uStack28: u16;
  let mut uStack26: u16;
  let mut uStack24: u16;
  let mut uStack22: u32;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut local_a: u32;
  let mut local_6: u32;
  astruct_57 *paVar12;

  local_6 = 0x0;
  local_a = 0x0;
  puVar4 = &local_a;
  uVar16 = (param_2 >> 0x10);
  uVar3 = param_2;
  pass1_1038_134a(puVar4,uVar3,uVar16,(u32 *)CONCAT22(0x1050,puVar4),(u32 *)CONCAT22(0x1050,&local_6),param_4);
  uStack14 = (astruct_15 *)CONCAT22(param_1,puVar4);
  ppcVar1 = (code **)((int)*param_3 + 0x10);
  (**ppcVar1)();
  uStack18 = CONCAT22(param_1,puVar4);
  uStack22 = 0x0;
  do {
    if (uStack18 <= uStack22) {
      return;
    }
    uStack14 |= uStack14;
    if (uStack14 == 0x0) {
      return;
    }
    pass1_1028_b58e(uStack14);
    uStack26 = uStack14;
    uStack24 = uStack18;
    pass1_1038_1a30(uVar3,uVar16,CONCAT22(uStack18,uStack14));
    uStack30 = uStack14;
    uStack28 = uStack18;
    if ((uStack18 | uStack14) != 0x0) {
      sVar2 = (qword)CONCAT22(uStack18,uStack14) * 0x64;
      uVar15 = (u32)((qword)sVar2 >> 0x20);
      uVar7 = (u32)sVar2 >> 0x1;
      ppcVar1 = (code **)((int)*param_3 + 0x4);
      uStack34 = uVar7;
      (**ppcVar1)(0x1028,param_3,(char)uStack22,(int)(uStack22 >> 0x10));
      uVar6 = uVar7;
      uStack36 = uVar15;
      uStack38 = uVar6;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7 & 0xffff | uVar15 << 0x10);
      uStack40 = uVar15;
      uStack42 = uVar6;
      uStack46 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uStack40,uVar6),uVar6,uStack40);
      paVar14 = (astruct_57 *)(uVar15 & 0xffff0000);
      puStack50 = (u32*)((int)uStack46 + 0x28);
      puStack54 = NULL;
      uStack56 = ((int)puStack50 + 0x4);
      for (uStack58 = 0x0; uVar5 = uStack56, uStack58 < uStack56; uStack58 += 0x1) {
        pass1_1020_bb16(puStack50,(u32 *)CONCAT22(0x1050,&local_46),(u16 *)CONCAT13(0x10,CONCAT12(0x50,local_42)),
                        uStack58);
        if (((local_46 != 0x0) && (0xd < (int)local_42[0])) && ((int)local_42[0] < 0x1d)) {
          uVar15 = local_46;
          uVar7 = local_46;
          if (uStack34 < local_46) {
            uVar15 = uStack34 & 0xffff;
            uVar7 = uStack34;
          }
          paVar11 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | uVar7 >> 0x10);
          uVar5 = uVar15;
          uVar10 = (uVar7 >> 0x10);
          if ((local_a <= uVar10) && ((local_a < uVar10 || (local_a < uVar5)))) {
            paVar11 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)local_a);
            uVar5 = local_a;
          }
          iVar8 = (int)paVar11;
          lStack74 = CONCAT22(iVar8,uVar5);
          uStack34 = CONCAT22(((int)(uStack34 >> 0x10) - iVar8) - (uStack34 < uVar5),uStack34 - uVar5)
          ;
          local_a = CONCAT22((local_a - iVar8) - (local_a < uVar5),local_a - uVar5);
          paVar13 = paVar11;
          if (puStack54 == NULL) {
            paVar14 = paVar11;
            uVar10 = uVar5;
            mem_op_1000_179c(0xa,paVar11);
            uVar9 = paVar14 | uVar10;
            paVar13 = (astruct_57 *)((u32)paVar14 & 0xffff0000);
            paVar12 = (astruct_57 *)((u32)paVar13 | (u32)uVar9);
            if (uVar9 == 0x0) {
              uVar10 = 0x0;
            }
            else {
              pass1_1020_ba3e((astruct_172 *)CONCAT22(paVar14,uVar10),0x5,0x5);
              paVar13 = paVar12;
            }
            puStack54 = (u32 *)CONCAT22((int)paVar13,uVar10);
          }
          pass1_1020_bb8a((i32 *)puStack54,uVar5,(u32)paVar11 & 0xffff | (u32)local_42[0] << 0x10);
          uVar7 = local_46 - lStack74;
          pass1_1020_bb8a((i32 *)puStack50,uVar7,CONCAT22(local_42[0],(int)(uVar7 >> 0x10)));
          paVar14 = paVar13;
          if (local_a == 0x0) {
            pass1_1038_1b3a(uVar7,uVar3,uVar16,(u32)uStack14,puStack54,unaff_DI);
            puStack54 = NULL;
            uVar7 = ZEXT24(&local_a);
            pass1_1038_134a(&local_a,uVar3,uVar16,(u32 *)CONCAT22(0x1050,&local_a),(u32 *)CONCAT22(0x1050,&local_6),
                            param_4);
            uVar5 = uVar7;
            uStack14 = (astruct_15 *)(uVar7 & 0xffff | (long)paVar13 << 0x10);
            uVar10 = paVar13 | uVar5;
            paVar14 = (astruct_57 *)((u32)paVar13 & 0xffff0000 | (u32)uVar10);
            if (uVar10 != 0x0) {
              uVar17 = 0x64;
              uVar18 = 0x0;
              uVar6 = 0x0;
              pass1_1028_b58e((astruct_15 *)(uVar7 & 0xffff | (long)paVar13 << 0x10));
              uVar10 = paVar14;
              uStack26 = uVar5;
              uStack24 = uVar10;
              pass1_1038_1a30(uVar3,uVar16,CONCAT22(uVar10,uVar5));
              sVar2 = (qword)CONCAT22(uVar10,uVar5) * (qword)CONCAT22(uVar6,CONCAT11(uVar18,uVar17));
              paVar14 = (astruct_57 *)((qword)sVar2 >> 0x20);
              uVar7 = (u32)sVar2 >> 0x1;
              uStack34 = uVar7;
              uStack30 = uVar5;
              uStack28 = uVar10;
            }
          }
          uVar5 = uVar7;
          if ((uStack34 == 0x0) || (local_a == 0x0)) break;
        }
      }
      if (puStack54 != NULL) {
        pass1_1038_1b3a(uVar5,uVar3,uVar16,(u32)uStack14,puStack54,unaff_DI);
        puStack54 = NULL;
      }
    }
    uStack22 += 0x1;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_16f2(mut param_1: u16 ,mut param_2: u32,u32 *param_3,u32 *param_4)

{
  i32 *plVar1;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  u32 *puVar5;
  let mut iVar6: i16;
  u32 *puVar7;
  u32 *puVar8;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  astruct_57 *paVar13;
  astruct_57 *paVar14;
  astruct_57 *paVar15;
  let mut unaff_CS: u16;
  let mut uVar16: u32;
  i32 lVar17;
  let mut uVar18: u16;
  u32 *puVar19;
  i32 lStack68;
  u32 *puStack56;
  u32 *puStack52;
  i32 *plStack50;
  let mut uStack46: u16;
  let mut uStack42: u32;
  let mut uStack22: u32;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut local_a: u32;
  let mut local_6: u32;

  local_6 = 0x0;
  local_a = 0x0;
  puVar7 = &local_a;
  uVar18 = (param_2 >> 0x10);
  uVar4 = param_2;
  pass1_1038_13da(puVar7,uVar4,uVar18,(u32 *)CONCAT22(0x1050,puVar7),(u32 *)CONCAT22(0x1050,&local_6),
                  param_4);
  uStack14 = CONCAT22(param_1,puVar7);
  uVar9 = param_1 | puVar7;
  if (uVar9 != 0x0) {
    ppcVar2 = (code **)((int)*param_3 + 0x10);
    puVar19 = param_3;
    (**ppcVar2)();
    uStack18 = CONCAT22(uVar9,puVar7);
    for (uStack22 = 0x0; uStack22 < uStack18; uStack22 += 0x1) {
      ppcVar2 = (code **)((int)*param_3 + 0x4);
      uVar16 = uStack18;
      uVar10 = uVar9;
      (**ppcVar2)(unaff_CS,param_3,(char)uStack22,(int)(uStack22 >> 0x10),puVar19);
      iVar6 = (int)uVar16;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar16 & 0xffff | (u32)uVar10 << 0x10);
      unaff_CS = 0x1030;
      uVar16 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar10,iVar6),iVar6,uVar10);
      uVar11 = (uVar16 >> 0x10);
      uVar12 = uVar16;
      pass1_1038_1a30(uVar4,uVar18,CONCAT22(uVar10,iVar6));
      if ((uVar11 | uVar12) != 0x0) {
        uStack42 = (u32)(CONCAT22(uVar11,uVar12) * 0x64) >> 0x1;
        plVar1 = *(i32 **)(iVar6 + 0x22);
        uVar12 = (iVar6 + 0x24);
        paVar13 = (astruct_57 *)(u32)uVar12;
        uStack46 = plVar1;
        if ((uVar12 | uStack46) != 0x0) {
          plStack50 = NULL;
          puVar7 = (u32 *)pass1_1028_0d80(uVar16);
          puStack56 = NULL;
          puStack52 = puVar7;
          while( true ) {
            lVar17 = pass1_1020_bae6(puStack52,paVar13,uStack46,
                                     CONCAT22(puStack52,(int)((u32)plVar1 >> 0x10)));
            uVar3 = (u32)paVar13 & 0xffff0000;
            puVar8 = (u32 *)lVar17;
            uVar12 = ((u32)lVar17 >> 0x10);
            paVar13 = (astruct_57 *)(uVar3 | (uVar12 | puVar8));
            if ((uVar12 | puVar8) != 0x0) {
              paVar14 = (astruct_57 *)(uVar3 | uVar12);
              if ((uStack42 <= uVar12) && ((uStack42 < uVar12 || ((u32 *)uStack42 < puVar8)))) {
                paVar14 = (astruct_57 *)(uVar3 | uStack42);
                puVar8 = (u32 *)uStack42;
              }
              if ((local_a <= paVar14) &&
                 ((local_a < paVar14 || ((u32 *)local_a < puVar8)))) {
                paVar14 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)local_a);
                puVar8 = (u32 *)local_a;
              }
              iVar6 = (int)paVar14;
              lStack68 = CONCAT22(iVar6,puVar8);
              uStack42 = CONCAT22((uStack42 - iVar6) - ((u32 *)uStack42 < puVar8),
                                  (int)(u32 *)uStack42 - (int)puVar8);
              local_a = CONCAT22((local_a - iVar6) - ((u32 *)local_a < puVar8),
                                 (int)(u32 *)local_a - (int)puVar8);
              paVar13 = paVar14;
              if (plStack50 == NULL) {
                paVar15 = paVar14;
                puVar5 = puVar8;
                mem_op_1000_179c(0xa,paVar14);
                uVar12 = paVar15 | puVar5;
                paVar13 = (astruct_57 *)((u32)paVar15 & 0xffff0000 | (u32)uVar12);
                if (uVar12 == 0x0) {
                  puVar5 = NULL;
                  paVar13 = (astruct_57 *)((u32)paVar15 & 0xffff0000);
                }
                else {
                  pass1_1020_ba3e((astruct_172 *)CONCAT22(paVar15,puVar5),0x5,0x5);
                }
                plStack50 = (i32 *)CONCAT22((int)paVar13,puVar5);
              }
              pass1_1020_bb8a(plStack50,puVar8,(u32)paVar14 & 0xffff | ZEXT24(puStack52) << 0x10);
              pass1_1020_bb8a(plVar1,(lVar17 - lStack68),
                              CONCAT22(puStack52,(int)((u32)(lVar17 - lStack68) >> 0x10)));
              puStack56 = puStack52;
              puVar8 = puStack52;
              if (local_a == 0x0) {
                pass1_1038_1ac6((int)puStack52,uVar4,uVar18,uStack14,(u32)plStack50);
                plStack50 = NULL;
                puVar8 = &local_a;
                pass1_1038_13da(puVar8,uVar4,uVar18,(u32 *)CONCAT22(0x1050,puVar8),
                                (u32 *)CONCAT22(0x1050,&local_6),param_4);
                uStack14 = CONCAT22(paVar13,puVar8);
                uVar12 = paVar13 | puVar8;
                paVar13 = (astruct_57 *)((u32)paVar13 & 0xffff0000 | (u32)uVar12);
                if (uVar12 == 0x0) {
                  return;
                }
              }
            }
            unaff_CS = 0x1020;
            if ((uStack42 == 0x0) || (local_a == 0x0)) break;
            unaff_CS = 0x1028;
            puVar8 = (u32 *)pass1_1028_0d80(uVar16);
            if ((puVar8 == puStack56) || ((puStack52 = puVar8, puStack56 == NULL && (puVar8 == puVar7)))) break;
          }
          if (plStack50 != NULL) {
            pass1_1038_1ac6((int)puVar8,uVar4,uVar18,uStack14,(u32)plStack50);
          }
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_1940(mut param_1: u32,u32 *param_2,mut param_3: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  u8 *puVar4;
  let mut extraout_DX: u16;
  u32 *puVar5;
  u32 *puStack10;

  puVar5 = pass1_1008_c6fa(_u16_1050_06e0,0x3);
  puVar4 = (u8 *)((u32)puVar5 >> 0x10);
  uVar2 = puVar5;
  pass1_1038_4d6e(uVar2,puVar4,(astruct_691 *)param_3,puVar5);
  puStack10 = (u32 *)CONCAT22(puVar4,uVar2);
  ppcVar1 = (code **)((int)*puStack10 + 0x10);
  uVar3 = uVar2;
  (**ppcVar1)(0x1008,uVar2,puVar4);
  if ((extraout_DX | uVar3) != 0x0) {
    pass1_1038_1482(extraout_DX | uVar3,param_1,param_2,puStack10);
  }
  if (puStack10 != NULL) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(0x1008,uVar2,(char)puVar4,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_19a0(mut param_1: u32,u32 *param_2,mut param_3: u32,mut param_4: u16 ,undefined1 param_5)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 *puVar5;
  let mut extraout_DX: u16;
  u32 *puVar6;
  u32 *puStack10;

  puVar6 = pass1_1008_c6fa(_u16_1050_06e0,0x2);
  puVar5 = (u8 *)((u32)puVar6 >> 0x10);
  uVar3 = puVar6;
  pass1_1038_4d6e(uVar3,puVar5,(astruct_691 *)param_3,puVar6);
  puStack10 = (u32 *)CONCAT22(puVar5,uVar3);
  uVar2 = *puStack10;
  ppcVar1 = (code **)uVar2 + 0x8;
  uVar4 = uVar3;
  (**ppcVar1)(0x1008,uVar3,puVar5);
  if ((extraout_DX | uVar4) == 0x0) {
    vsprintf_op_1030_840a(0x0,(u32)s_mineToSmelter__no_mines_1050_59df);
    if (puStack10 != NULL) {
      ppcVar1 = (code **)uVar2;
      (**ppcVar1)(0x1030,uVar3,(char)puVar5,0x1);
      return;
    }
  }
  else {
    pass1_1038_16f2(extraout_DX | uVar4,param_1,puStack10,param_2);
    if (puStack10 != NULL) {
      ppcVar1 = (code **)*puStack10;
      (**ppcVar1)(0x1008,uVar3,(char)puVar5,0x1);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_1a30(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  u32 *puVar1;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar5: u16;
  let mut unaff_CS: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uStack18: u32;
  let mut uStack10: u32;
  let mut uStack6: u16;

  uVar5 = (param_3 >> 0x10);
  puVar1 = (u32 *)(u32)((int)param_3 + 0x1e);
  uVar7 = ((int)param_3 + 0x20);
  uStack6 = puVar1;
  uVar3 = uVar7 | uStack6;
  if (uVar3 != 0x0) {
    ppcVar2 = (code **)((int)*puVar1 + 0x10);
    uVar6 = uStack6;
    (**ppcVar2)();
    uStack10 = CONCAT22(extraout_DX,uVar3);
    for (uStack18 = 0x0; uStack18 < uStack10; uStack18 += 0x1) {
      ppcVar2 = (code **)((int)*puVar1 + 0x4);
      uVar4 = uStack10;
      (**ppcVar2)(unaff_CS,uStack6,(int)((u32)puVar1 >> 0x10),uStack18,uVar6,uVar7);
      if ((extraout_DX_00 | uVar4) != 0x0) {
        unaff_CS = 0x1028;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | (u32)extraout_DX_00 << 0x10);
      }
    }
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_1ac6(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32,mut param_5: u32)

{
  let mut extraout_DX: u16;
  u8 local_118 [0x112];
  let mut uStack6: u32;

  pass1_1028_b58e((astruct_15 *)param_4);
  uStack6 = CONCAT22(extraout_DX,param_1);
  pass1_1030_e8a0((astruct_97 *)CONCAT22(0x1050,local_118),param_5,((int)param_4 + 0xc),
                  (u32)(param_1 + 0x4));
  pass1_1028_d52c(*_u16_1050_5748,*_PTR_LOOP_1050_65e2 + 0x1,(u32 *)CONCAT22(0x1050,local_118));
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1038_1b3a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32,u32 *param_5,mut param_6: u16 )

{
  let mut in_EDX: u32;
  let mut uVar2: u16;
  astruct_57 *paVar1;
  let mut local_1a: u32;
  u16 local_16 [0x2];
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  pass1_1028_b58e((astruct_15 *)param_4);
  uStack6 = CONCAT22((int)in_EDX,param_1);
  uStack10 = param_4;
  uStack14 = pass1_1028_45e2(param_4,(int)in_EDX,param_4);
  paVar1 = (astruct_57 *)(in_EDX & 0xffff0000);
  uStack16 = ((int)param_5 + 0x4);
  for (uStack18 = 0x0; uVar2 = ((u32)paVar1 >> 0x10), uStack18 < uStack16; uStack18 += 0x1) {
    pass1_1020_bb16(param_5,(u32 *)CONCAT22(0x1050,&local_1a),(u16 *)CONCAT22(0x1050,local_16),uStack18);
    paVar1 = (astruct_57 *)CONCAT22(uVar2,uStack14);
    if (uStack14 < local_1a) {
      pass1_1030_7ddc(uStack14,paVar1,uStack6,uStack14,local_16[0]);
      uStack14 = 0x0;
    }
    else {
      uStack14 -= local_1a;
      pass1_1030_7ddc(local_1a,paVar1,uStack6,local_1a,local_16[0]);
    }
    if (uStack14 == 0x0) break;
  }
  if (param_5 != NULL) {
    fn_ptr_1020_ba7e(param_5);
    fn_ptr_1000_17ce((char *)param_5);
  }
  return;
}



StructD * pass1_1038_1c02(StructD *param_1,u8 param_2)

{
  param_1.address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_1c3e(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u16;
  u32 *puVar2;
  code **ppcVar3;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut BVar7: bool;
  u32 *puVar8;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut unaff_CS: u16;
  let mut uVar11: u32;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  astruct_419 *paStack26;
  let mut uStack14: u32;

  uVar10 = (param_2 >> 0x10);
  puVar2 = (u32 *)(u32)((int)param_2 + 0xc);
  uVar10 = ((int)param_2 + 0xe);
  ppcVar3 = (code **)((int)*puVar2 + 0x10);
  puVar8 = puVar2;
  uVar14 = (int)puVar2;
  (**ppcVar3)();
  uVar4 = (u32)puVar8 & 0xffff | (u32)extraout_DX << 0x10;
  uStack14 = 0x0;
  do {
    if (uVar4 <= uStack14) {
      return;
    }
    ppcVar3 = (code **)((int)*puVar2 + 0x4);
    uVar11 = uVar4;
    (**ppcVar3)(unaff_CS,(int)puVar2,(int)((u32)puVar2 >> 0x10),uStack14,uVar14,uVar10);
    uVar5 = uVar11;
    uVar9 = extraout_DX_00 | uVar5;
    if (uVar9 != 0x0) {
      unaff_CS = 0x1028;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar11 & 0xffff | (u32)extraout_DX_00 << 0x10);
      paStack26 = (astruct_419 *)CONCAT22(uVar9,uVar5);
      iVar6 = (uVar5 + 0x34);
      if ((iVar6 != 0x0) && (*(i32 *)(uVar5 + 0x36) != 0x0)) {
        uVar12 = param_1;
        uVar13 = (param_1 >> 0x10);
        pass1_1038_201a(iVar6,uVar9,uVar12,uVar13,(astruct_412 *)CONCAT22(uVar9,uVar5));
        if (iVar6 == 0x0) {
          uVar11 = struct_op_1030_73a8(paStack26,0x0,uVar9);
          uVar1 = ((int)uVar11 + 0xc);
          unaff_CS = 0x1008;
          BVar7 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x1);
          if (BVar7 == 0x0) {
            unaff_CS = 0x1008;
            BVar7 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x2);
            if (BVar7 == 0x0) {
              BVar7 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x5);
              if (BVar7 == 0x0) {
                unaff_CS = 0x1008;
                BVar7 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x6);
                if (BVar7 == 0x0) goto LAB_1038_1c76;
              }
              unaff_CS = 0x1008;
              pass1_1038_2306(uVar12,uVar13,paStack26);
            }
            else {
              pass1_1038_26ee(uVar12,uVar13,(u32)paStack26);
            }
          }
          else {
            pass1_1038_24e8(uVar12,uVar13,(u32)paStack26);
          }
        }
      }
    }//
LAB_1038_1c76:
    uStack14 += 0x1;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_1d68(mut param_1: u16 ,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,u32 *param_5,mut param_6: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  code **ppcVar6;
  let mut uVar7: u16;
  let mut bVar8: bool;
  u8 *puVar9;
  let mut uVar10: u32;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u32;
  let mut iVar14: i16;
  let mut unaff_CS: u16;
  u32 *puVar15;
  astruct_99 *paStack82;
  let mut uStack78: u16;
  let mut uStack52: u32;
  u8 local_30 [0x4];
  let mut uStack44: u32;
  u32 *puStack40;
  let mut uStack36: u32;
  u8 local_20 [0x4];
  u32 *puStack28;
  let mut uStack24: u16;
  let mut uStack22: u16;
  let mut uStack20: u16;
  let mut uStack18: u16;
  let mut uStack16: u32;
  let mut uStack12: u32;
  let mut uStack8: u16;
  let mut uStack6: u32;

  uStack6 = 0x64;
  uStack8 = 0x0;
  ppcVar6 = (code **)((int)*param_5 + 0x10);
  puVar15 = param_5;
  (**ppcVar6)();
  uStack12 = CONCAT22((int)param_2,param_1);
  uStack16 = 0x0;
  do {
    if (uStack12 <= uStack16) {
      return;
    }
    ppcVar6 = (code **)((int)*param_5 + 0x4);
    uVar10 = uStack12;
    (**ppcVar6)(unaff_CS,param_5,uStack16,puVar15);
    uVar11 = uVar10;
    uStack18 = param_2;
    uVar12 = uStack18 | uVar11;
    uVar13 = (u32)uVar12;
    uStack20 = uVar11;
    if (uVar12 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar10 & 0xffff | param_2 << 0x10);
      uStack22 = uVar12;
      unaff_CS = 0x1030;
      uStack24 = uVar11;
      puStack28 = (u32 *)struct_op_1030_73a8((astruct_419 *)CONCAT22(uStack22,uVar11),uVar11,uStack22);
      uVar13 = (u32)puStack28 >> 0x10;
      puVar9 = local_20;
      ppcVar6 = (code **)((int)*puStack28 + 0x40);
      (**ppcVar6)(0x1030,(int)puStack28,(int)((u32)puStack28 >> 0x10),puVar9,(int)&DAT_1050_1050);
      if (puVar9 == NULL) {
        uStack36 = pass1_1028_62c8((u32)puStack28);
        uVar13 = uStack36 >> 0x10;
        uStack8 = 0x1;
        puStack40 = (u32 *)(u32)((int)param_6 + 0x22);
        pass1_1008_5784((char *)CONCAT22(0x1050,local_30),(u32)puStack40);
        while( true ) {
          uVar12 = uVar13;
          puVar9 = local_30;
          unaff_CS = 0x1008;
          pass1_1008_5b12((char *)CONCAT22(0x1050,puVar9));
          uStack52 = CONCAT22(uVar12,puVar9);
          uVar13 = (u32)(uVar12 | puVar9);
          if ((uVar12 | puVar9) == 0x0) break;
          uVar2 = (puVar9 + 0x4);
          iVar3 = (puVar9 + 0x6);
          uVar4 = (puVar9 + 0x8);
          uVar11 = (puVar9 + 0xc);
          uVar5 = (puVar9 + 0xa);
          uVar7 = uVar11 / uVar5;
          uVar13 = (u32)uVar11 % (u32)uVar5;
          bVar8 = false;
          if (((0x0 < iVar3) && (!SBORROW2(iVar3,0x1))) && ((iVar3 == 0x5 || iVar3 + -0x1 < 0x4 || (iVar3 == 0x8)))) {
            bVar8 = true;
          }
          if (bVar8) {
            uVar10 = uStack36;
            if (uStack6 < uStack36) {
              uVar10 = uStack6 & 0xffff;
              uStack36 = uStack6;
            }
            uVar11 = uStack36 | uVar10;
            uVar13 = (u32)uVar11;
            if (uVar11 == 0x0) break;
            uStack78 = ((uVar10 & 0xffff | (u32)uStack36 << 0x10) / (u32)uVar7);
            if (uStack78 < uVar5) {
              piVar1 = (i16 *)(puVar9 + 0xc);
              *piVar1 = *piVar1 - uVar10;
              piVar1 = (i16 *)(puVar9 + 0xa);
              *piVar1 = *piVar1 - uStack78;
            }
            else {
              ppcVar6 = (code **)((int)*puStack40 + 0xc);
              (**ppcVar6)(0x1008,(int)puStack40,(int)((u32)puStack40 >> 0x10),uStack52);
              uStack44 = 0x0;
              uStack78 = uVar5;
            }
            paStack82 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
            uVar11 = ((u32)paStack82 >> 0x10);
            uVar12 = paStack82;
            if ((uVar11 | uVar12) == 0x0) {
              paStack82 = NULL;
            }
            else {
              paStack82.field0_0x0 = 0x389a;
              (uVar12 + 0x2) = 0x1008;
              (uVar12 + 0x4) = 0x0;
              (uVar12 + 0x6) = 0x0;
              (uVar12 + 0x8) = 0x0;
              (uVar12 + 0xa) = 0x0;
              (uVar12 + 0xc) = 0x0;
              paStack82.field0_0x0 = 0x56ce;
              (uVar12 + 0x2) = 0x1018;
            }
            uVar12 = ((u32)paStack82 >> 0x10);
            iVar14 = (int)paStack82;
            (iVar14 + 0xa) = uStack78;
            uVar10 = (u32)uStack78 * (u32)uVar7;
            uVar13 = uVar10 >> 0x10;
            (iVar14 + 0xc) = (int)uVar10;
            (iVar14 + 0x4) = uVar2;
            (iVar14 + 0x6) = iVar3;
            (iVar14 + 0x8) = uVar4;
            pass1_1028_6408((u32)puStack28,(u32 *)((u32)paStack82 & 0xffff | (u32)uVar12 << 0x10));
          }
        }
      }
      else {
        ppcVar6 = (code **)((int)*param_5 + 0x8);
        (**ppcVar6)(0x1030,param_5,0x0,uStack16);
      }
    }
    uStack16 += 0x1;
    param_2 = uVar13;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_1faa(mut param_1: u16 ,mut param_2: u32,mut param_3: u32,u32 *param_4,u32 *param_5)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  ppcVar1 = (code **)((int)*param_5 + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22((int)param_2,param_1);
  uStack10 = 0x0;
  while( true ) {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar1 = (code **)((int)*param_5 + 0x4);
    uVar4 = uStack6;
    (**ppcVar1)();
    uVar2 = uVar4;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | param_2 << 0x10);
    uVar4 = struct_op_1030_73a8((astruct_419 *)CONCAT22((int)param_2,uVar2),uVar2,(int)param_2);
    param_2 = param_2 & 0xffff0000 | uVar4 >> 0x10;
    uVar3 = uVar4;
    pass1_1038_1d68(uVar3,param_2,param_3,(param_3 >> 0x10),param_4,uVar4);
    if (uVar3 == 0x0) break;
    uStack10 += 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_201a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut astruct_412)

{
  let mut puVar1: *mut u16;
  let mut iVar2: i16;
  code **ppcVar3;
  i32 lVar4;
  let mut uVar6: u32;
  let mut uVar8: u16;
  let mut uVar9: u16;
  astruct_57 *paVar10;
  let mut uVar11: u32;
  u8 *puVar12;
  astruct_57 *paVar13;
  astruct_412 *iVar12;
  let mut uVar14: u16;
  u8 *puVar15;
  astruct_412 *uVar16;
  let mut uVar17: u16;
  astruct_385 *paVar18;
  i32 lStack24;
  i32 lStack20;
  let mut uStack10: u16;
  astruct_385 *uVar7;
  astruct_385 *uVar5;

  uVar17 = 0x1030;
  paVar18 = (astruct_385 *)pass1_1030_6b16(param_5);
  uVar7 = (astruct_385 *)((u32)paVar18 >> 0x10);
  uVar5 = (astruct_385 *)paVar18;
  if ((uVar7 | uVar5) == 0x0) {
    return;
  }
  uVar16 = (astruct_412 *)((u32)param_5 >> 0x10);
  iVar12 = (astruct_412 *)param_5;
  iVar2 = &iVar12.field_0x34;
  lVar4 = (long)iVar2;
  uVar6 = lVar4 * 0x64;
  puVar12 = (u8 *)(uVar6 >> 0x10);
  uVar8 = uVar6;
  uStack10 = 0x0;
  lStack20 = 0x0;
  if (uVar5.field4_0x4 == 0x0) {
    if (uVar5.field5_0x6 == 0x0) {
      if (uVar5.field6_0x8 == 0x0) goto LAB_1038_2102;
      uVar9 = pass1_1020_c42e(uVar5.field6_0x8);
      uVar11 = (u32)uVar5.field7_0xa * (u32)uVar9;
      puVar15 = (u8 *)(uVar11 >> 0x10);
      if (uVar11 + lVar4 * -0x64 != 0x0 && (long)uVar6 <= (long)uVar11) {
        uVar11 = uVar6 & 0xffff;
        puVar15 = puVar12;
      }
      uVar6 = uVar11 & 0xffff | ZEXT24(puVar15) << 0x10;
      uVar11 = (long)(uVar11 & 0xffff | ZEXT24(puVar15) << 0x10) / (long)(u32)uVar9;
      puVar1 = &uVar5.field7_0xa;
      *puVar1 = *puVar1 - (int)uVar11;
      uStack10 = ((long)uVar6 / 0x64);
      paVar13 = (astruct_57 *)((long)uVar6 % 0x64);
      paVar10 = paVar13;
      if (paVar13 != NULL) {
        uStack10 += 0x1;
        paVar10 = (astruct_57 *)(u32)uStack10;
      }
      uVar8 = paVar10;
      mem_op_1000_179c(0x2a,paVar13);
      puVar12 = (u8 *)(paVar13 | uVar8);
      if (puVar12 == NULL) goto LAB_1038_20fa;
      pass1_1038_6838((astruct_415 *)CONCAT22(paVar13,uVar8),uVar11,uVar5.field6_0x8,uStack10,
                      (u32)&iVar12.field_0x4);
    }
    else {
      uVar9 = switch_1020_c3b4(uVar5.field5_0x6);
      uVar11 = (u32)uVar5.field7_0xa * (u32)uVar9;
      puVar15 = (u8 *)(uVar11 >> 0x10);
      if (uVar11 + lVar4 * -0x64 != 0x0 && (long)uVar6 <= (long)uVar11) {
        uVar11 = uVar6 & 0xffff;
        puVar15 = puVar12;
      }
      uVar6 = uVar11 & 0xffff | ZEXT24(puVar15) << 0x10;
      uVar11 = (long)(uVar11 & 0xffff | ZEXT24(puVar15) << 0x10) / (long)(u32)uVar9;
      puVar1 = &uVar5.field7_0xa;
      *puVar1 = *puVar1 - (int)uVar11;
      uStack10 = ((long)uVar6 / 0x64);
      paVar13 = (astruct_57 *)((long)uVar6 % 0x64);
      paVar10 = paVar13;
      if (paVar13 != NULL) {
        uStack10 += 0x1;
        paVar10 = (astruct_57 *)(u32)uStack10;
      }
      uVar8 = paVar10;
      mem_op_1000_179c(0x2a,paVar13);
      puVar12 = (u8 *)(paVar13 | uVar8);
      if (puVar12 == NULL) goto LAB_1038_20fa;
      pass1_1038_675c((astruct_414 *)CONCAT22(paVar13,uVar8),uVar11,uVar5.field5_0x6,uStack10,
                      (u32)&iVar12.field_0x4);
    }
  }
  else {
    uVar14 = uVar5.field7_0xa;
    puVar15 = NULL;
    if (((int)puVar12 < 0x1) && (((u8 *)0x7fff < puVar12 || (uVar8 < uVar14)))) {
      uVar14 = uVar8;
      puVar15 = puVar12;
    }
    lStack24 = CONCAT22(puVar15,uVar14);
    puVar1 = &uVar5.field7_0xa;
    *puVar1 = *puVar1 - uVar14;
    uStack10 = (lStack24 / 0x64);
    paVar13 = (astruct_57 *)(lStack24 % 0x64);
    paVar10 = paVar13;
    if (paVar13 != NULL) {
      uStack10 += 0x1;
      paVar10 = (astruct_57 *)(u32)uStack10;
    }
    uVar8 = paVar10;
    mem_op_1000_179c(0x2a,paVar13);
    puVar12 = (u8 *)(paVar13 | uVar8);
    if (puVar12 == NULL) {//
LAB_1038_20fa:
      uVar17 = 0x1000;
      lStack20 = 0x0;
      goto LAB_1038_2102;
    }
    pass1_1038_6590((astruct_410 *)CONCAT22(paVar13,uVar8),uVar14,puVar15,uVar5.field4_0x4,uStack10,
                    (u32)&iVar12.field_0x4);
  }
  uVar17 = 0x1000;
  lStack20 = CONCAT22(puVar12,uVar8);//
LAB_1038_2102:
  if (lStack20 != 0x0) {
    pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
    uVar17 = 0x1030;
    uVar8 = uStack10;
    pass1_1030_6c4c((u32)param_5,iVar2 - uStack10);
  }
  if (uVar5.field7_0xa == 0x0) {
    if ((uVar7 | uVar5) != 0x0) {
      ppcVar3 = (code **)(u32)paVar18;
      (**ppcVar3)(uVar17,uVar5,uVar7,0x1);
    }
  }
  else {
    pass1_1030_6c66(uVar8,puVar12,(astruct_386 *)param_5,0x0,paVar18);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_2306(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut astruct_419)

{
  let mut piVar1: *mut i16;
  astruct_57 *paVar2;
  code **ppcVar3;
  qword qVar4;
  u32 *puVar5;
  let mut in_AX: u16;
  astruct_417 *uVar9;
  astruct_57 *puVar7;
  let mut in_DX: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  astruct_57 *paVar8;
  astruct_57 *paVar9;
  astruct_419 *iVar11;
  astruct_417 *iVar12;
  astruct_419 *uVar10;
  let mut uVar11: u16;
  let mut uVar12: u32;
  let mut uStack42: u32;
  let mut uStack34: u32;
  let mut uStack30: u16;
  let mut uStack24: u32;
  let mut uStack12: u32;
  let mut iStack8: i16;
  astruct_417 *uVar13;

  uVar11 = 0x1030;
  uVar12 = struct_op_1030_73a8(param_3,in_AX,in_DX);
  paVar8 = (astruct_57 *)(uVar12 >> 0x10);
  uVar10 = (astruct_419 *)((u32)param_3 >> 0x10);
  iVar11 = (astruct_419 *)param_3;
  iStack8 = iVar11.field49_0x34;
  uStack12 = 0x64;
  paVar2 = *(astruct_57 **)((int)uVar12 + 0x22);
  puVar7 = paVar2;
  while( true ) {
    uVar6 = paVar8;
    ppcVar3 = (code **)((int)(u32)paVar2 + 0x10);
    (**ppcVar3)(uVar11,(int)paVar2,(int)((u32)paVar2 >> 0x10));
    uVar9 = (astruct_417 *)puVar7;
    uVar13 = (astruct_417 *)((u32)puVar7 & 0xffff);
    puVar5 = (u32 *)((u32)uVar13 | (u32)uVar6 << 0x10);
    if ((uVar6 | uVar9) == 0x0) break;
    if (uVar9.field8_0xa == 0x0) {
      paVar8 = (astruct_57 *)(u32)(uVar6 | uVar9);
      if ((uVar6 | uVar9) != 0x0) {
        ppcVar3 = (code **)*puVar5;
        (**ppcVar3)((char)uVar11,uVar9,uVar6,0x1);
      }
    }
    else {
      uStack24 = 0x0;
      uStack30 = 0x0;
      if (uVar9.field6_0x6 == 0x0) {
        if (uVar9.field7_0x8 != 0x0) {
          uStack30 = pass1_1020_c42e(uVar9.field7_0x8);
          goto LAB_1038_2385;
        }
      }
      else {
        uStack30 = switch_1020_c3b4(uVar9.field6_0x6);//
LAB_1038_2385:
        uVar11 = 0x1020;
        uStack24 = (u32)(uVar9.field8_0xa * uStack30);
      }
      uStack12 = 0x0;
      if (uStack12 < uStack24) {
        uStack24 = uStack12 & 0xffff;
      }
      uStack34 = uStack24 | (u32)uStack12 << 0x10;
      uStack24 |= (u32)uStack12 << 0x10;
      qVar4 = (qword)uStack24 / (qword)uStack30;
      uVar12 = (u32)qVar4;
      paVar8 = (astruct_57 *)(uStack24 % (u32)uStack30);
      piVar1 = &uVar9.field8_0xa;
      *piVar1 = *piVar1 - (int)qVar4;
      if (*piVar1 == 0x0) {
        paVar8 = (astruct_57 *)(u32)(uVar6 | uVar9);
        if ((uVar6 | uVar9) != 0x0) {
          ppcVar3 = (code **)*puVar5;
          (**ppcVar3)((char)uVar11,uVar9,uVar6,0x1);
        }
      }
      else {
        ppcVar3 = (code **)((int)(u32)paVar2 + 0x8);
        (**ppcVar3)();
      }
      uStack12 -= uStack34;
      puVar7 = NULL;
      uStack42 = 0x0;
      iVar12 = (astruct_417 *)uVar13;
      if (iVar12.field6_0x6 == 0x0) {
        if (iVar12.field7_0x8 != 0x0) {
          mem_op_1000_179c(0x2a,paVar8);
          uVar7 = paVar8 | puVar7;
          paVar9 = (astruct_57 *)(u32)uVar7;
          if (uVar7 == 0x0) goto LAB_1038_244e;
          pass1_1038_6838((astruct_415 *)((u32)puVar7 & 0xffff | (long)paVar8 << 0x10),uVar12,iVar12.field7_0x8,0x1,
                          iVar11.field4_0x4);
          goto LAB_1038_24b3;
        }
      }
      else {
        mem_op_1000_179c(0x2a,paVar8);
        uVar7 = paVar8 | puVar7;
        paVar9 = (astruct_57 *)(u32)uVar7;
        if (uVar7 == 0x0) {//
LAB_1038_244e:
          uVar11 = 0x1000;
          uStack42 = 0x0;
          paVar8 = paVar9;
        }
        else {
          pass1_1038_675c((astruct_414 *)((u32)puVar7 & 0xffff | (long)paVar8 << 0x10),uVar12,iVar12.field6_0x6,0x1,
                          iVar11.field4_0x4);//
LAB_1038_24b3:
          uVar11 = 0x1000;
          uStack42 = (u32)puVar7 & 0xffff | (long)paVar9 << 0x10;
          paVar8 = paVar9;
        }
      }
      if (uStack42 != 0x0) {
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
        iStack8 += -0x1;
        if (iStack8 == 0x0) break;
        uStack12 = 0x64;
      }
    }
  }
  pass1_1030_6c4c((u32)param_3,iStack8);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_24e8(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u32;
  let mut in_AX: u16;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar6;
  astruct_57 *paVar8;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut uStack30: u16;
  u32 *puStack28;
  let mut local_18: u32;
  let mut local_14: u16;
  let mut uStack18: u16;
  let mut uStack16: u32;
  u32 *puStack12;
  let mut iStack8: i16;
  let mut uStack6: u32;
  astruct_57 *paVar7;

  uStack6 = struct_op_1030_73a8((astruct_419 *)param_3,in_AX,(int)in_EDX);
  paVar6 = (astruct_57 *)(in_EDX & 0xffff0000 | uStack6 >> 0x10);
  uVar10 = (param_3 >> 0x10);
  iVar9 = (int)param_3;
  iStack8 = (iVar9 + 0x34);
  puStack12 = (u32*)((int)uStack6 + 0x28);
  uStack16 = 0x64;
  uStack18 = ((int)puStack12 + 0x4);
  uVar3 = (u32)uStack18;
  mem_op_1000_179c(0xa,paVar6);
  uVar5 = uVar3;
  uVar4 = paVar6 | uVar5;
  paVar8 = (astruct_57 *)((u32)paVar6 & 0xffff0000);
  paVar7 = (astruct_57 *)((u32)paVar8 | (u32)uVar4);
  if (uVar4 == 0x0) {
    uVar5 = 0x0;
  }
  else {
    pass1_1020_ba3e((astruct_172 *)(uVar3 & 0xffff | (long)paVar6 << 0x10),0x5,0x5);
    paVar8 = paVar7;
  }
  puStack28 = (u32 *)CONCAT22((int)paVar8,uVar5);
  for (uStack30 = 0x0; uVar3 = (u32)uStack18, uStack30 < uStack18; uStack30 += 0x1) {
    pass1_1020_bb16(puStack12,(u32 *)CONCAT22(0x1050,&local_18),(u16 *)CONCAT22(0x1050,&local_14),uStack30);
    if (local_18 != 0x0) {
      uVar3 = local_18;
      uVar2 = local_18;
      if (uStack16 < local_18) {
        uVar3 = uStack16 & 0xffff;
        uVar2 = uStack16;
      }
      paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | uVar2 >> 0x10);
      uVar5 = uVar3;
      uVar3 = uVar3 & 0xffff | uVar2 & 0xffff0000;
      iVar1 = ((int)(local_18 >> 0x10) - (int)(uVar2 >> 0x10)) - (local_18 < uVar5);
      local_18 = CONCAT22(iVar1,local_18 - uVar5);
      pass1_1020_bb8a((i32 *)puStack12,local_18 - uVar5,CONCAT22(local_14,iVar1));
      pass1_1020_bb70((i32 *)puStack28,uVar5,uVar2 >> 0x10 | (u32)local_14 << 0x10);
      uStack16 -= uVar3;
      if (uStack16 == 0x0) {
        mem_op_1000_179c(0x2a,paVar8);
        uVar5 = paVar8 | uVar3;
        paVar6 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)uVar5);
        if (uVar5 == 0x0) {
          uVar3 = 0x0;
        }
        else {
          pass1_1038_666e((astruct_420 *)(uVar3 & 0xffff | (long)paVar8 << 0x10),(i32 *)puStack28,0x1,
                          (u32)(iVar9 + 0x4));
        }
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
        mem_op_1000_179c(0xa,paVar6);
        uVar5 = paVar6 | uVar3;
        paVar8 = (astruct_57 *)((u32)paVar6 & 0xffff0000);
        paVar7 = (astruct_57 *)((u32)paVar8 | (u32)uVar5);
        if (uVar5 == 0x0) {
          uVar3 = 0x0;
        }
        else {
          pass1_1020_ba3e((astruct_172 *)(uVar3 & 0xffff | (long)paVar6 << 0x10),0x5,0x5);
          paVar8 = paVar7;
        }
        puStack28 = (u32 *)(uVar3 & 0xffff | (long)paVar8 << 0x10);
        iStack8 += -0x1;
        if (iStack8 == 0x0) break;
        uStack16 = 0x64;
      }
    }
  }
  pass1_1020_ba94((i32 *)puStack28);
  uVar5 = paVar8 | uVar3;
  paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)uVar5);
  if (uVar5 == 0x0) {
    if (puStack28 != NULL) {
      fn_ptr_1020_ba7e(puStack28);
      fn_ptr_1000_17ce((char *)puStack28);
    }
  }
  else {
    mem_op_1000_179c(0x2a,paVar8);
    if ((paVar8 | uVar3) != 0x0) {
      pass1_1038_666e((astruct_420 *)(uVar3 & 0xffff | (long)paVar8 << 0x10),(i32 *)puStack28,0x1,
                      (u32)(iVar9 + 0x4));
    }
    pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
  }
  pass1_1030_6c4c(param_3,iStack8);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_26ee(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_AX: u16;
  let mut uVar1: u16;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut iVar5: i16;
  let mut in_EDX: u32;
  astruct_57 *paVar6;
  astruct_57 *paVar8;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut uVar11: u32;
  let mut uVar12: u32;
  let mut uStack36: u32;
  let mut uStack20: u16;
  let mut uStack18: u16;
  let mut uStack16: u32;
  let mut uStack12: u16;
  let mut uStack10: u16;
  let mut iStack8: i16;
  astruct_57 *paVar7;

  uVar11 = struct_op_1030_73a8((astruct_419 *)param_3,in_AX,(int)in_EDX);
  paVar6 = (astruct_57 *)(in_EDX & 0xffff0000 | uVar11 >> 0x10);
  uVar10 = (param_3 >> 0x10);
  iVar9 = (int)param_3;
  iStack8 = (iVar9 + 0x34);
  uStack12 = pass1_1028_0d80(uVar11);
  uVar2 = (u32)uStack12;
  uStack16 = 0x64;
  mem_op_1000_179c(0xa,paVar6);
  uVar3 = paVar6 | uVar2;
  paVar8 = (astruct_57 *)((u32)paVar6 & 0xffff0000);
  paVar7 = (astruct_57 *)((u32)paVar8 | (u32)uVar3);
  if (uVar3 == 0x0) {
    uVar2 = 0x0;
  }
  else {
    pass1_1020_ba3e((astruct_172 *)(uVar2 & 0xffff | (long)paVar6 << 0x10),0x5,0x5);
    paVar8 = paVar7;
  }
  uStack20 = uVar2;
  uStack18 = paVar8;
  uStack10 = uStack12;
  do {
    uVar12 = pass1_1030_7c28(uVar2,paVar8,param_3,uStack10);
    uVar2 = (u32)paVar8 & 0xffff0000;
    uVar3 = uVar12;
    uVar4 = ((u32)uVar12 >> 0x10);
    paVar8 = (astruct_57 *)(uVar2 | (uVar4 | uVar3));
    if ((uVar4 | uVar3) != 0x0) {
      paVar6 = (astruct_57 *)(uVar2 | uVar4);
      uVar1 = uVar3;
      if ((uStack16 <= uVar4) && ((uStack16 < uVar4 || (uStack16 < uVar3)))) {
        paVar6 = (astruct_57 *)(uVar2 | uStack16);
        uVar1 = uStack16;
      }
      iVar5 = (int)paVar6;
      uStack36 = CONCAT22(iVar5,uVar1);
      paVar8 = paVar6;
      pass1_1030_7d1c(uVar1,iVar5,(astruct_397 *)param_3,uVar3 - uVar1,
                      CONCAT22(uStack10,(uVar4 - iVar5) - (uVar3 < uVar1)));
      pass1_1020_bb70((i32 *)CONCAT22(uStack18,uStack20),uVar1,(u32)paVar6 & 0xffff | (u32)uStack10 << 0x10);
      uStack16 -= uStack36;
      if (uStack16 == 0x0) {
        mem_op_1000_179c(0x2a,paVar8);
        uStack10 = uStack36;
        uVar3 = paVar8 | uStack10;
        paVar6 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)uVar3);
        if (uVar3 == 0x0) {
          uStack10 = 0x0;
        }
        else {
          pass1_1038_666e((astruct_420 *)(uStack36 & 0xffff | (long)paVar8 << 0x10),(i32 *)CONCAT22(uStack18,uStack20),
                          0x1,(u32)(iVar9 + 0x4));
        }
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
        mem_op_1000_179c(0xa,paVar6);
        uVar3 = paVar6 | uStack10;
        paVar8 = (astruct_57 *)((u32)paVar6 & 0xffff0000);
        paVar7 = (astruct_57 *)((u32)paVar8 | (u32)uVar3);
        if (uVar3 == 0x0) {
          uStack10 = 0x0;
        }
        else {
          pass1_1020_ba3e((astruct_172 *)CONCAT22(paVar6,uStack10),0x5,0x5);
          paVar8 = paVar7;
        }
        uStack18 = paVar8;
        iStack8 += -0x1;
        uStack20 = uStack10;
        if (iStack8 == 0x0) break;
        uStack16 = 0x64;
      }
    }
    uStack10 = pass1_1028_0d80(uVar11);
    uVar2 = (u32)uStack10;
    if (uStack12 == 0x0) {
      uStack12 = uStack10;
    }
  } while (uStack12 != uStack10);
  pass1_1020_ba94((i32 *)CONCAT22(uStack18,uStack20));
  uVar3 = paVar8 | uStack10;
  paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)uVar3);
  if (uVar3 == 0x0) {
    if ((uStack18 | uStack20) != 0x0) {
      fn_ptr_1020_ba7e((u32 *)CONCAT22(uStack18,uStack20));
      fn_ptr_1000_17ce((char *)CONCAT22(uStack18,uStack20));
    }
  }
  else {
    mem_op_1000_179c(0x2a,paVar8);
    if ((paVar8 | uStack10) != 0x0) {
      pass1_1038_666e((astruct_420 *)CONCAT22(paVar8,uStack10),(i32 *)CONCAT22(uStack18,uStack20),0x1,
                      (u32)(iVar9 + 0x4));
    }
    pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
  }
  pass1_1030_6c4c(param_3,iStack8);
  return;
}



astruct_97 * pass1_1038_28d8(param_1: *mut astruct_97)

{
  struct_op_1028_d1dc(param_1,0x3a97);
  param_1.offset_0x0 = 0x29fe;
    // just 0x1038
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  unk_str_op_1000_3d3e((char *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x8)),s_SCRoboMove_1050_59f8);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1038_290e(mut param_1: u16 ,undefined1 param_2,mut param_3: u16 )

{
  let mut unaff_SI: u16;
  let mut unaff_DI: u16;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x4000001);
  if ((param_3 | param_1) != 0x0) {
    pass1_1038_4918(param_1,param_3 | param_1,CONCAT22(param_3,param_1));
  }
  pass1_1038_7a76(_PTR_LOOP_1050_5a64,unaff_SI,unaff_DI,(int)&DAT_1050_1050);
  return 0x1;
}
pub fn pass1_1038_2944(mut param_1: u16 ,u8 *param_2,mut param_3: u32)

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
    *puStack10 = 0x29fe;
    (param_1 + 0x2) = (int)&u16_1050_1038;
  }
  return;
}



StructD * pass1_1038_29d2(StructD *param_1,u8 param_2)

{
  param_1.address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1038_2a0e(param_1: *mut astruct_97,mut param_2: u32,mut param_3: u32,mut param_4: u32,mut param_5: u32)

{
  astruct_97 *iVar1;
  let mut uVar1: u16;

  struct_op_1028_d1dc(param_1,0x2af7);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_97 *)param_1;
  (u32)&iVar1.field259_0x108 = param_5;
  (u32)&iVar1.field262_0x10c = param_4;
  iVar1.field264_0x110 = param_3;
  iVar1.field265_0x114 = param_2;
  param_1.offset_0x0 = 0x309a;
  iVar1.segment_0x2 = &u16_1050_1038;
  return;
}
pub fn pass1_1038_2a5c(param_1: *mut u16)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = ((u32)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0x309a;
  (iVar4 + 0x2) = (int)&u16_1050_1038;
  puVar1 = (u32 *)(iVar4 + 0x114);
  uVar2 = (iVar4 + 0x116);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = (u32 *)(iVar4 + 0x110);
  uVar2 = (iVar4 + 0x112);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *param_1 = 0x389a;
  (iVar4 + 0x2) = 0x1008;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1038_2ac2(mut param_1: u16 ,mut param_2: u16 ,uchar param_3,mut param_4: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar2 = (param_4 >> 0x10);
  uVar1 = param_4;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(uVar1 + 0x108));
  uStack6 = CONCAT22(param_2,param_1);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(uVar1 + 0x10c));
  uStack10 = CONCAT22(param_2,param_1);
  pass1_1038_2c82(param_3,uVar1,uVar2,*(astruct_702 **)(uVar1 + 0x110),CONCAT22(param_2,param_1),uStack6);
  pass1_1038_2c82(param_3,uVar1,uVar2,*(astruct_702 **)(uVar1 + 0x114),uStack6,uStack10);
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1038_2b2e(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uStack6: u32;

  uVar2 = (param_3 >> 0x10);
  uVar1 = param_3;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(uVar1 + 0x108));
  uStack6 = CONCAT22(param_2,param_1);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(uVar1 + 0x10c));
  pass1_1038_2f92(uVar1,uVar2,(u32)(uVar1 + 0x110),CONCAT22(param_2,param_1));
  pass1_1038_2f92(uVar1,uVar2,(u32)(uVar1 + 0x114),uStack6);
  return 0x1;
}
pub fn pass1_1038_2b9a(param_1: *mut astruct_422,u8 *param_2,param_3: *mut astruct_421)

{
  u32 *puVar1;
  u32 *puVar2;
  let mut iVar3: i16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar4;
  astruct_57 *uVar4;
  astruct_421 *iVar5;
  u32 *puVar5;
  u32 *puVar6;
  let mut uVar7: u16;
  let mut puStack10: *mut u16;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x118,paVar4);
  uVar4._0_2_ = paVar4;
  puStack10 = (u16 *)CONCAT22(uVar4,param_1);
  iVar5 = (astruct_421 *)param_3;
  uVar7 = ((u32)param_3 >> 0x10);
  if ((uVar4 | param_1) != 0x0) {
    *puStack10 = 0x389a;
    param_1.field2_0x2 = 0x1008;
    param_1.field3_0x4 = iVar5.field4_0x4;
    puVar5 = &iVar5.field5_0x8;
    puVar6 = &param_1.field4_0x8;
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 += -0x1) {
      puVar2 = puVar6;
      puVar6 = puVar6 + 0x1;
      puVar1 = puVar5;
      puVar5 = puVar5 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_1.field2_0x2 = 0x1028;
    param_1.field257_0x108 = iVar5.field258_0x108;
    param_1.field258_0x10c = iVar5.field259_0x10c;
    param_1.field259_0x110 = iVar5.field260_0x110;
    param_1.field260_0x114 = iVar5.field261_0x114;
    *puStack10 = 0x309a;
    param_1.field2_0x2 = (int)&u16_1050_1038;
  }
  iVar5.field261_0x114 = 0x0;
  iVar5.field260_0x110 = 0x0;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_2c82(undefined1 param_1,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut astruct_702,mut param_5: u32,mut param_6: u32)

{
  let mut puVar1: *mut u16;
  let mut piVar2: *mut i16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  code **ppcVar6;
  let mut uVar7: u32;
  let mut uVar8: u16;
  u32 *puVar9;
  let mut iVar10: i16;
  let mut uVar11: u32;
  let mut in_EDX: u32;
  astruct_57 *paVar12;
  StructD *pSVar13;
  astruct_57 *paVar14;
  let mut uVar15: u16;
  let mut iVar16: i16;
  astruct_702 *iVar15;
  let mut uVar17: u16;
  let mut uVar18: u16;
  u8 *puVar19;
  astruct_211 *paVar20;
  u32 *puVar21;
  let mut in_stack_0000fe64: u16;
  let mut in_stack_0000fe76: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ffa0: u16;
  let mut in_stack_0000ffa4: u16;
  u8 uVar22;
  let mut in_stack_0000ffce: u16;
  let mut uStack22: u32;
  let mut local_12: u32;
  astruct_702 *paStack14;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar17 = (param_6 >> 0x10);
  uVar15 = param_6;
  uStack6 = (u32)(uVar15 + 0x200);
  uVar18 = (param_5 >> 0x10);
  iVar16 = (int)param_5;
  uStack10 = (u32)(iVar16 + 0x200);
  uVar3 = (iVar16 + 0x202);
  paVar12 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)uVar3);
  puVar19 = (u8 *)((u32)param_4 >> 0x10);
  iVar15 = (astruct_702 *)param_4;
  iVar10 = iVar15.field10_0xc;
  if (iVar10 == 0x1) {
    paStack14 = param_4;
    pass1_1038_52b8(param_6,(u32)&iVar15.field8_0x8,&iVar15.field11_0xe);
    return;
  }
  if (iVar10 == 0x2) {
    paStack14 = param_4;
    if (iVar15.field11_0xe != 0x0) {
      pass1_1038_3efc(uVar15,uVar17,param_5,iVar15.field11_0xe,(int)iVar15,puVar19);
      return;
    }
    pass1_1020_a43e(puVar19,(u16 *)CONCAT22(0x1050,&local_12));
    uStack22 = *(i32 *)((int)paStack14 + 0x8);
    while( true ) {
      uStack22 += -0x1;
      if ((uStack22 | uStack22) == 0x0) break;
      pass1_1020_a6ee(&local_12,uStack22 | uStack22,in_stack_0000fe64,
                      CONCAT13(0x10,CONCAT12(0x50,&local_12)),((int)paStack14 + 0x12));
    }
  }
  else {
    if (iVar10 == 0x3) {
      pass1_1038_3f38((u32 *)param_6,(u32 *)param_5,iVar15.field11_0xe,0x0,uVar3);
      return;
    }
    if (iVar10 == 0x4) {
      uVar7 = uStack6 >> 0x10 & 0xff;
      pSVar13 = (StructD *)(in_EDX & 0xffff0000 | uVar7);
      if (((int)uStack6 == 0x1) && ((int)uVar7 == 0x0)) {
        local_12 = (u32)(uVar15 + 0x1f6);
        pass1_1030_3694(NULL,local_12,&iVar15.field11_0xe,*(i32 *)&iVar15.field8_0x8);
        ((int)&iVar15.field11_0xe + 0x2) = (int)local_12;
        iVar15.field12_0x12 = (u8 *)pSVar13;
      }
      else {
        if (_PTR_LOOP_1050_5f2c == 0x0) {
          PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar13);
          PTR_LOOP_1050_5f2e = (u8 *)pSVar13;
        }
        else {
        }
        uVar15 = fn_ptr_op_1000_1708(0x16c,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
        ((int)&iVar15.field11_0xe + 0x2) = uVar15;
        iVar15.field12_0x12 = PTR_LOOP_1050_5f2e;
        iVar10 = &iVar15.field11_0xe;
        if (iVar10 != 0x3) {
          if (iVar10 != 0x4) {
            uVar5 = (u32)((int)&iVar15.field11_0xe + 0x2);
            (u32)((int)uVar5 + 0x28) = (u32)&iVar15.field8_0x8;
            return;
          }
          uVar5 = (u32)((int)&iVar15.field11_0xe + 0x2);
          (u32)((int)uVar5 + 0xdc) = (u32)&iVar15.field8_0x8;
          return;
        }
        uVar5 = (u32)((int)&iVar15.field11_0xe + 0x2);
        (u32)((int)uVar5 + 0x64) = (u32)&iVar15.field8_0x8;
      }
    }
    else if (iVar10 == 0x5) {
      if (&iVar15.field11_0xe == 0xc) {
        if (((int)uStack6 == 0x1) && ((uStack6 & 0xff0000) == 0x0)) {
          uVar5 = (u32)(uVar15 + 0x1f6);
          uVar3 = iVar15.field8_0x8;
          iVar10 = iVar15.field9_0xa;
          uVar8 = -uVar3;
          uVar18 = ((u32)uVar5 >> 0x10);
          iVar16 = (int)uVar5;
          puVar1 = (u16 *)(iVar16 + 0x170);
          uVar4 = *puVar1;
          *puVar1 = *puVar1 + uVar8;
          piVar2 = (i16 *)(iVar16 + 0x172);
          *piVar2 = (*piVar2 - (iVar10 + (uVar3 != 0x0))) + CARRY2(uVar4,uVar8);
        }
      }
      else {
        pass1_1038_43cc((int)iVar15,(int)puVar19,uVar15,uVar17,iVar15.field8_0x8,&iVar15.field11_0xe);
      }
    }
    else {
      iVar10 += -0x7;
      if (iVar10 == 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,iVar15.field11_0xe);
        paVar14 = paVar12;
        pass1_1038_349e((astruct_685 *)CONCAT22((int)paVar12,iVar10),(u32)(iVar16 + 0x200));
        uVar22 = (u8)((u32)paVar12 >> 0x8);
        pass1_1038_4d0e((astruct_685 *)CONCAT13(uVar22,CONCAT12((char)paVar12,iVar10)),0x258);
        pass1_1038_4d0e((astruct_685 *)CONCAT13(uVar22,CONCAT12((char)paVar12,iVar10)),0x258);
        paVar20 = (astruct_211 *)
                  mixed_1010_20ba(paVar14,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffce,0x3b),
                                  in_stack_0000fe76,in_stack_0000ff9a,in_stack_0000ffa0,in_stack_0000ffa4);
        paVar12 = (astruct_57 *)((u32)paVar14 & 0xffff0000 | (u32)paVar20 >> 0x10);
        pass1_1008_de58(paVar20,iVar15.field11_0xe,0x4000001);
        puVar21 = mixed_1010_20ba(paVar12,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffce,0x2f),
                                  in_stack_0000fe76,in_stack_0000ff9a,in_stack_0000ffa0,in_stack_0000ffa4);
        paVar12 = (astruct_57 *)((u32)paVar12 & 0xffff0000 | (u32)puVar21 >> 0x10);
        uVar11 = (u32)((int)puVar21 + 0x20);
        pass1_1030_8344(_u16_1050_5748,uVar11);
        local_12 = uVar11 & 0xffff | (long)paVar12 << 0x10;
        iVar10 = pass1_1030_5b00(uVar11 & 0xffff | (long)paVar12 << 0x10);
        paStack14 = (astruct_702 *)
                    mixed_1010_20ba(paVar12,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffce,iVar10),
                                    in_stack_0000fe76,in_stack_0000ff9a,in_stack_0000ffa0,in_stack_0000ffa4);
        puVar9 = (u32 *)((int)paStack14 + 0x20);
        ppcVar6 = (code **)((int)*puVar9 + 0x4);
        (**ppcVar6)(0x1010,puVar9,(char)((u32)paStack14 >> 0x10),0x6);
      }
    }
  }
  return;
}
pub fn pass1_1038_2f92(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  let mut puVar1: *mut u16;
  let mut piVar2: *mut i16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut uVar7: u32;
  let mut iVar8: i16;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut iStack10: i16;

  uVar10 = (param_4 >> 0x10);
  iVar8 = (int)param_4;
  uVar6 = (u32)(iVar8 + 0x200);
  uVar11 = (param_3 >> 0x10);
  iVar9 = (int)param_3;
  iVar3 = (iVar9 + 0xc);
  if (iVar3 == 0x1) {
    uVar7 = (u32)(iVar9 + 0x8);
    pass1_1038_3cc0(param_4,uVar7,(u8 *)((u32)uVar7 >> 0x10),(iVar9 + 0xe));
    return;
  }
  if (iVar3 == 0x4) {
    pass1_1030_355c((u32)(iVar8 + 0x1f6),(u32)(iVar9 + 0x10));
    return;
  }
  if (iVar3 == 0x5) {
    if ((iVar9 + 0xe) != 0xc) {
      pass1_1038_5798(param_4,*(i32 *)(iVar9 + 0x8),(iVar9 + 0xe));
      return;
    }
    iStack10 = (int)uVar6;
    if ((iStack10 == 0x1) && ((uVar6 & 0xff0000) == 0x0)) {
      uVar7 = (u32)(iVar8 + 0x1f6);
      uVar4 = (iVar9 + 0x8);
      iVar3 = (iVar9 + 0xa);
      uVar10 = ((u32)uVar7 >> 0x10);
      iVar8 = (int)uVar7;
      puVar1 = (u16 *)(iVar8 + 0x170);
      uVar5 = *puVar1;
      *puVar1 = *puVar1 + uVar4;
      piVar2 = (i16 *)(iVar8 + 0x172);
      *piVar2 = *piVar2 + iVar3 + CARRY2(uVar5,uVar4);
      return;
    }
  }
  return;
}



StructD * pass1_1038_3074(StructD *param_1,u8 param_2)

{
  pass1_1038_2a5c(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn pass1_1038_30aa(param_1: *mut astruct_180,mut param_2: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  astruct_57 *paVar3;
  astruct_180 *iVar5;
  astruct_180 *uVar5;
  let mut puVar5: *mut u16;
  astruct_57 *paVar4;

  puVar5 = struct_1030_17ce(param_1,0x0,0x0,param_2);
  paVar3 = (astruct_57 *)(param_2 & 0xffff0000 | (u32)puVar5 >> 0x10);
  uVar5 = (astruct_180 *)((u32)param_1 >> 0x10);
  iVar5 = (astruct_180 *)param_1;
  (u32)&iVar5.field12_0x10 = 0x0;
  (u32)&iVar5.field14_0x14 = 0x0;
  iVar5.field16_0x18 = 0x258;
  iVar5.field17_0x1a = 0x258;
  &iVar5.field18_0x1c = 0x0;
  (u32)((int)&iVar5.field18_0x1c + 0x2) = 0x0;
  iVar5[0x1].field1_0x2 = 0x0;
  &iVar5[0x1].field_0x4 = 0x32;
  (u32)&iVar5[0xf].field15_0x16 = 0x0;
  (u32)&iVar5[0xf].field17_0x1a = 0x0;
  ((int)&iVar5[0xf].field18_0x1c + 0x2) = 0x0;
  (u32)(iVar5 + 0x10) = 0x8000001;
  &iVar5[0x10].field_0x4 = 0x0;
  &iVar5[0x10].field_0x6 = 0x0;
  &iVar5[0x10].field_0x8 = 0x1;
  &iVar5[0x10].field_0xa = 0x0;
  iVar5[0x10].field10_0xc = 0x0;
  iVar5[0x10].field11_0xe = 0x0;
  (u32)&iVar5[0x10].field12_0x10 = 0x0;
  iVar5[0x10].field14_0x14 = 0x0;
  (u32)&iVar5[0x10].field15_0x16 = 0x0;
  (u32)&iVar5[0x10].field17_0x1a = 0x0;
  param_1.field0_0x0 = 0x6504;
  iVar5.field1_0x2 = (int)&u16_1050_1038;
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5[0x1].field_0x6)),NULL,0x94);
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5[0x5].field17_0x1a)),NULL,0x94);
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5[0xa].field11_0xe)),NULL,0x54);
  puVar1 = pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar5[0xd].field1_0x2)),NULL,0x54);
  mem_op_1000_179c(0x1b0,paVar3);
  uVar2 = paVar3 | puVar1;
  paVar4 = (astruct_57 *)((u32)paVar3 & 0xffff0000 | (u32)uVar2);
  if (uVar2 == 0x0) {
    (u32)&iVar5[0xf].field15_0x16 = 0x0;
  }
  else {
    pass1_1030_314c(uVar2,(astruct_364 *)CONCAT22(paVar3,puVar1),(u32)&iVar5.field_0x4);
    iVar5[0xf].field15_0x16 = puVar1;
    iVar5[0xf].field16_0x18 = (int)paVar4;
  }
  mem_op_1000_179c(0x1e,paVar4);
  uVar2 = paVar4 | puVar1;
  if (uVar2 == 0x0) {
    puVar1 = NULL;
    uVar2 = 0x0;
  }
  else {
    struct_1020_c444((astruct_75 *)CONCAT22(paVar4,puVar1),0x64,0xc8);
  }
  iVar5.field10_0xc = puVar1;
  iVar5.field11_0xe = uVar2;
  return;
}
pub fn pass1_1038_3222(mut param_1: u16 ,param_2: *mut astruct_57,param_3: *mut astruct_363,mut param_4: u32,mut param_5: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  astruct_57 *paVar4;
  astruct_363 *iVar5;
  let mut uVar6: u16;
  let mut puVar7: *mut u16;
  u8 local_16 [0x14];
  let mut uVar5: u32;

  puVar7 = pass1_1030_183c(param_1,param_2,(u16 *)param_3,0x0,0x0,0x4000000,param_5);
  paVar4 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)puVar7 >> 0x10);
  uVar6 = ((u32)param_3 >> 0x10);
  iVar5 = (astruct_363 *)param_3;
  iVar5.field12_0x10 = param_4;
  iVar5.field13_0x14 = 0x0;
  iVar5.field14_0x18 = 0x258;
  iVar5.field15_0x1a = 0x258;
  iVar5.field16_0x1c = 0x0;
  iVar5.field17_0x1e = 0x0;
  iVar5.field18_0x22 = 0x0;
  iVar5.field19_0x24 = 0x32;
  (u32)&iVar5.field484_0x1f6 = 0x0;
  (u32)&iVar5.field486_0x1fa = 0x0;
  iVar5.field488_0x1fe = 0x0;
  iVar5.field489_0x200 = 0x8000001;
  iVar5.field490_0x204 = 0x0;
  iVar5.field491_0x206 = 0x0;
  iVar5.field492_0x208 = 0x1;
  iVar5.field493_0x20a = 0x0;
  iVar5.field494_0x20c = 0x0;
  iVar5.field495_0x20e = 0x0;
  iVar5.field496_0x210 = 0x0;
  iVar5.field497_0x214 = 0x0;
  iVar5.field498_0x216 = 0x0;
  iVar5.field499_0x21a = 0x0;
  param_3 = 0x6504;
  iVar5.field2_0x2 = (int)&u16_1050_1038;
  pass1_1000_4906((StructD *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar5.field_0x26)),NULL,0x94);
  pass1_1000_4906((StructD *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar5.field_0xba)),NULL,0x94);
  pass1_1000_4906((StructD *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar5.field_0x14e)),NULL,0x54);
  puVar1 = pass1_1000_4906((StructD *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar5.field_0x1a2)),NULL,0x54);
  mem_op_1000_179c(0x1b0,paVar4);
  uVar2 = paVar4 | puVar1;
  uVar5 = (u32)paVar4 & 0xffff0000 | (u32)uVar2;
  if (uVar2 == 0x0) {
    (u32)&iVar5.field484_0x1f6 = 0x0;
  }
  else {
    pass1_1030_314c(uVar2,(astruct_364 *)CONCAT22(paVar4,puVar1),(u32)&iVar5.field_0x4);
    iVar5.field484_0x1f6 = puVar1;
    iVar5.field485_0x1f8 = (int)uVar5;
  }
  paVar4 = (astruct_57 *)(uVar5 & 0xffff0000 | (u32)iVar5.field5_0x6 & 0xffff00ff);
  sys_1000_3f9c((char *)CONCAT22(0x1050,local_16),(char *)0x10505a1a,(u32)&iVar5.field_0x4);
  uVar2 = str_op_1008_60e8(paVar4,(char *)CONCAT22(0x1050,local_16));
  iVar5.field486_0x1fa = uVar2;
  iVar5.field487_0x1fc = (u8 *)paVar4;
  mem_op_1000_179c(0x1e,paVar4);
  uVar3 = paVar4 | uVar2;
  if (uVar3 == 0x0) {
    (u32)&iVar5.field10_0xc = 0x0;
  }
  else {
    struct_1020_c444((astruct_75 *)CONCAT22(paVar4,uVar2),0x64,0xc8);
    iVar5.field10_0xc = uVar2;
    iVar5.field11_0xe = uVar3;
  }
  return;
}
pub fn pass1_1038_33f8(param_1: *mut u16)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = ((u32)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0x6504;
  (iVar4 + 0x2) = (int)&u16_1050_1038;
  puVar1 = (u32 *)(iVar4 + 0x14);
  uVar2 = (iVar4 + 0x16);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = (u32 *)(iVar4 + 0x1f6);
  uVar2 = (iVar4 + 0x1f8);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce(*(char **)(iVar4 + 0x1fa));
  puVar1 = (u32 *)(iVar4 + 0x210);
  uVar2 = (iVar4 + 0x212);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1000,puVar1,uVar2,0x1);
  }
  fn_ptr_1000_17ce(*(char **)(iVar4 + 0x21a));
  pass1_1030_18b2(param_1);
  return;
}
pub fn pass1_1038_349e(param_1: *mut astruct_685,mut param_2: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut extraout_DX_00: u16;
  astruct_685 *iVar7;
  let mut uVar6: u16;
  u32 *puVar7;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar6 = ((u32)param_1 >> 0x10);
  iVar7 = (astruct_685 *)param_1;
  iVar7.field509_0x200 = param_2;
  pass1_1038_4d0e(param_1,0x258);
  uVar3 = param_2;
  pass1_1038_4d0e(param_1,0x258);
  iVar7.field510_0x204 = 0x0;
  iVar7.field511_0x206 = 0x0;
  puVar7 = iVar7.field12_0xc;
  uVar8 = SUB42(puVar7,0x0);
  uVar9 = ((u32)puVar7 >> 0x10);
  ppcVar1 = (code **)((int)*iVar7.field12_0xc + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,uVar3);
  uVar5 = extraout_DX;
  for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
    puVar7 = (u32 *)pass1_1030_1d7c(uVar3,uVar5,(u32)iVar7.field12_0xc);
    uVar4 = ((u32)puVar7 >> 0x10);
    uVar2 = puVar7;
    uVar5 = uVar4 | uVar2;
    if (uVar5 != 0x0) {
      ppcVar1 = (code **)((int)*puVar7 + 0x58);
      (**ppcVar1)(0x1030,uVar2,uVar4,(char)param_1,uVar6,uVar8,uVar9);
      (u32)(uVar2 + 0x1c) = 0x0;
      uVar5 = extraout_DX_00;
    }
  }
  return;
}
pub fn pass1_1038_354a(param_1: *mut astruct_424,mut param_2: u16 ,u8 *param_3)

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar2;
  astruct_424 *iVar1;
  astruct_424 *uVar3;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_3);
  uVar3 = (astruct_424 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_424 *)param_1;
  if (*(i32 *)&iVar1.field_0x21a == 0x0) {
    mem_op_1000_179c(0xa,paVar2);
    uVar1 = paVar2 | param_2;
    if (uVar1 == 0x0) {
      (u32)&iVar1.field_0x21a = 0x0;
    }
    else {
      pass1_1030_9ecc((u32 *)CONCAT22(paVar2,param_2),param_1);
      &iVar1.field_0x21a = param_2;
      iVar1.field540_0x21c = uVar1;
    }
  }
  pass1_1030_9ef2((u32*)&iVar1.field_0x21a);
  return;
}
pub fn pass1_1038_35a8(mut param_1: u16 ,u8 *param_2,mut param_3: u16 ,param_4: *mut astruct_425)

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar2;
  astruct_425 *iVar3;
  uchar in_AF;
  let mut in_stack_00000006: u16;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  if (*(i32 *)&param_4.field_0x21a == 0x0) {
    mem_op_1000_179c(0xa,paVar2);
    uVar1 = paVar2 | param_1;
    if (uVar1 == 0x0) {
      (u32)&param_4.field_0x21a = 0x0;
    }
    else {
      pass1_1030_9ecc((u32 *)CONCAT22(paVar2,param_1),(astruct_424 *)CONCAT22(in_stack_00000006,param_4));
      &param_4.field_0x21a = param_1;
      param_4.field540_0x21c = uVar1;
    }
  }
  pass1_1030_9f40(in_AF,(u32)&param_4.field_0x21a,param_3);
  return;
}
pub fn pass1_1038_3608(mut param_1: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  fn_ptr_1000_17ce(*(char **)((int)param_1 + 0x21a));
  (u32)((int)param_1 + 0x21a) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_362e(mut param_1: u32,mut param_2: u16 ,u8 **param_3,param_4: *mut astruct_57)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  astruct_67 *paVar3;
  let mut in_stack_0000fea2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000ffd0: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  if ((iVar1 + 0x214) == 0x0) {
    pass1_1038_4f54(param_2,param_1 & 0xffff | (u32)uVar2 << 0x10,0x1f);
    if (param_2 == 0x0) {
      (iVar1 + 0x214) = 0x14;
    }
    else {
      (iVar1 + 0x214) = 0x28;
    }
    param_3 = (u8 **)((u32)param_3 & 0xffff0000 | 0x37);
    paVar3 = (astruct_67 *)
             mixed_1010_20ba(param_4,_u16_1050_0ed0,param_3,in_stack_0000fea2,in_stack_0000ffc6,in_stack_0000ffcc,
                             in_stack_0000ffd0);
    post_win_msg_1008_a0e4(paVar3,0x0,0x0,0x1,(u32)(iVar1 + 0x4),0x38);
    (u32)(iVar1 + 0x216) = 0x0;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_3698(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut piVar1: *mut i16;
  let mut puVar2: *mut u16;
  let mut uVar3: u32;
  code **ppcVar4;
  let mut uVar5: u16;
  let mut BVar6: bool;
  let mut uVar7: u16;
  let mut uVar8: u16;
  i32 lVar9;
  let mut uVar10: u32;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar14: u32;
  let mut iVar15: i16;
  let mut uVar16: u16;
  let mut uVar17: u32;
  let mut uStack32: u32;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar16 = (param_3 >> 0x10);
  iVar15 = (int)param_3;
  if ((iVar15 + 0x214) == 0x0) {
    return;
  }
  pass1_1030_38b8();
  uStack6 = CONCAT22(param_2,param_1);
  uStack6 -= *(i32 *)(iVar15 + 0x216);
  if (0x0 < uStack6) {
    uStack6 += 0x3;
    uStack10 = uStack6 / 0x5;
    uVar14 = uStack6 % 0x5;
    if (*(i32 *)(iVar15 + 0xc) == 0x0) {
      uVar5 = 0x0;
      uVar14 = 0x0;
    }
    else {
      uVar3 = (u32)(iVar15 + 0xc);
      ppcVar4 = (code **)((int)(u32)(u32)(iVar15 + 0xc) + 0x10);
      lVar9 = uStack10;
      (**ppcVar4)(0x1030,(int)uVar3,(int)((u32)uVar3 >> 0x10));
      uVar5 = lVar9;
    }
    uStack14 = CONCAT22((int)uVar14,uVar5);
    for (uStack18 = 0x0; uVar12 = uVar14, uVar10 = uStack14, uStack18 < uStack14; uStack18 += 0x1) {
      uVar17 = pass1_1030_1d7c(uVar5,uVar12,(u32)(iVar15 + 0xc));
      uVar8 = (uVar17 >> 0x10);
      uVar13 = uVar8 | uVar17;
      uVar14 = (u32)uVar13;
      if (uVar13 != 0x0) {
        BVar6 = pass1_1008_c6ae(_u16_1050_06e0,(uVar17 + 0xc),0x4);
        uVar8 = uVar14;
        uVar10 = (u32)BVar6;
        if (BVar6 != 0x0) {
          uVar7 = pass1_1028_678c(uVar17,0xf);
          uStack32 = CONCAT22(uVar8,uVar7);
          uVar14 = (u32)(uVar8 | uVar7);
          uVar10 = (u32)uVar7;
          if ((uVar8 | uVar7) != 0x0) {
            if (uStack10 < (long)uStack32) {
              uVar8 = uStack10;
              pass1_1028_6356(uVar17,0xf,uVar8,uStack10);
              uVar13 = uVar8 * 0x5;
              uVar11 = uStack10 * 0x5 + CARRY2(uVar8,uVar8) * 0x2 + CARRY2(uVar8 * 0x2,uVar8 * 0x2) +
                       CARRY2(uVar8 * 0x4,uVar8);
              uVar14 = (u32)uVar11;
              puVar2 = (u16 *)(iVar15 + 0x216);
              uVar8 = *puVar2;
              *puVar2 = *puVar2 + uVar13;
              piVar1 = (i16 *)(iVar15 + 0x218);
              *piVar1 = *piVar1 + uVar11 + CARRY2(uVar8,uVar13);
              uStack10 = 0x0;
              uVar10 = (u32)uVar13;
            }
            else {
              pass1_1028_6356(uVar17,0xf,uVar7,uVar8);
              uVar13 = uVar8 * 0x5 + CARRY2(uVar7,uVar7) * 0x2 + CARRY2(uVar7 * 0x2,uVar7 * 0x2) +
                       CARRY2(uVar7 * 0x4,uVar7);
              uVar14 = (u32)uVar13;
              puVar2 = (u16 *)(iVar15 + 0x216);
              uVar8 = *puVar2;
              *puVar2 = *puVar2 + uVar7 * 0x5;
              piVar1 = (i16 *)(iVar15 + 0x218);
              *piVar1 = *piVar1 + uVar13 + CARRY2(uVar8,uVar7 * 0x5);
              uStack10 -= uStack32;
              uVar10 = uStack32;
            }
          }
        }
        uVar12 = uVar14;
        if (uStack10 == 0x0) break;
      }
    }
    uVar5 = uVar10;
    pass1_1030_38b8();
    uStack6 = CONCAT22(uVar12,uVar5);
    uStack6 -= *(i32 *)(iVar15 + 0x216);
    uStack6 = ((u32)uStack6 >> 0x10);
    if ((uStack6 | uStack6) != 0x0) {
      uStack32 = uStack6 / (long)(iVar15 + 0x214);
      if ((long)uStack32 < 0x1) {
        uStack32 = 0x1;
      }
      pass1_1030_375a((u32)(iVar15 + 0x1f6),0x0,uStack32);
    }
  }
  piVar1 = (i16 *)(iVar15 + 0x214);
  *piVar1 = *piVar1 + -0x1;
  return;
}
pub fn pass1_1038_387e(param_1: *mut astruct_57,param_2: *mut astruct_302,mut param_3: i16,mut param_4: i16,mut param_5: u32)

{
  code **ppcVar1;
  i32 lVar2;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u32;
  let mut uVar6: u32;
  let mut uVar7: u16;
  let mut uVar8: u16;
  astruct_302 *iVar10;
  let mut uVar10: u16;
  let mut iStack22: i16;
  let mut uStack12: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;
  astruct_57 *paVar9;

  if (param_3 != param_4) {
    iVar10 = (astruct_302 *)param_2;
    uVar10 = ((u32)param_2 >> 0x10);
    if (param_3 < param_4) {
      uStack12 = param_4 - param_3;
      if ((iVar10.field525_0x210 == 0x0) || (lVar2 = iVar10.field525_0x210, *(i32 *)((int)lVar2 + 0xa) == 0x0)) {
        if (iVar10.field12_0xc == NULL) {
          uVar7 = 0x0;
          param_1 = (astruct_57 *)((u32)param_1 & 0xffff0000);
        }
        else {
          ppcVar1 = (code **)((int)*iVar10.field12_0xc + 0x10);
          uVar7 = uStack12;
          (**ppcVar1)();
        }
        uStack6 = CONCAT22((int)param_1,uVar7);
        for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
          uVar5 = uStack6;
          pass1_1030_1d58((u32)iVar10.field12_0xc);
          uVar7 = param_1 | uVar5;
          paVar9 = (astruct_57 *)((u32)param_1 & 0xffff0000 | (u32)uVar7);
          if ((uVar7 != 0x0) && (uVar3 = pass1_1030_6fa0(uVar5 & 0xffff | (long)param_1 << 0x10), uVar3 == 0xb)) {
            pass1_1030_7c50(uStack12,paVar9,
                            (astruct_305 *)CONCAT13((char)((u32)param_1 >> 0x8),CONCAT12((char)param_1,uVar5)),
                            (long)(int)uStack12,0x4);
            return;
          }
          param_1 = paVar9;
        }
      }
      else {
        lVar2 = iVar10->field525_0x210;
        uVar5 = (u32)((int)lVar2 + 0xa);
        for (uStack10 = 0x0; uVar7 = param_1, uStack10 < uVar5; uStack10 += 0x1) {
          uVar6 = uVar5;
          bad_1030_1312();
          uVar8 = uVar6;
          param_1 = (astruct_57 *)(u32)(uVar7 | uVar8);
          if ((((uVar7 | uVar8) != 0x0) && (pass1_1030_cc44(uVar8,uVar7,uStack12,param_5,0x4), uVar8 != 0x0)) &&
             (uStack12 -= uVar8, uStack12 == 0x0)) {
            return;
          }
        }
      }
    }
    else {
      iStack22 = param_3 - param_4;
      if ((iVar10->field525_0x210 == 0x0) || (lVar2 = iVar10->field525_0x210, *(i32 *)((int)lVar2 + 0xa) == 0x0)) {
        if (iVar10->field12_0xc == NULL) {
          iVar4 = 0x0;
          param_1 = NULL;
        }
        else {
          ppcVar1 = (code **)((int)*iVar10->field12_0xc + 0x10);
          iVar4 = iStack22;
          (**ppcVar1)();
        }
        uStack6 = CONCAT22((int)param_1,iVar4);
        for (uStack10 = 0x0; uVar7 = param_1, uStack10 < uStack6; uStack10 += 0x1) {
          uVar5 = uStack6;
          pass1_1030_1d58((u32)iVar10->field12_0xc);
          uVar8 = uVar7 | uVar5;
          param_1 = (astruct_57 *)(u32)uVar8;
          if ((uVar8 != 0x0) && (uVar3 = pass1_1030_6fa0(uVar5 & 0xffff | (u32)uVar7 << 0x10), uVar3 == 0xb)) {
            pass1_1030_6e9c((astruct_301 *)CONCAT13((char)(uVar7 >> 0x8),CONCAT12((char)uVar7,uVar5)),
                            (long)iStack22,0x4);
            return;
          }
        }
      }
      else {
        lVar2 = iVar10->field525_0x210;
        uVar5 = (u32)((int)lVar2 + 0xa);
        for (uStack10 = 0x0; uVar7 = param_1, uStack10 < uVar5; uStack10 += 0x1) {
          uVar6 = uVar5;
          bad_1030_1312();
          uVar8 = uVar6;
          param_1 = (astruct_57 *)(u32)(uVar7 | uVar8);
          if ((uVar7 | uVar8) != 0x0) {
            pass1_1030_ce72((u32)uVar7 << 0x10 | uVar6 & 0xffff,iStack22,param_5,0x4);
            iStack22 -= uVar8;
            if (iStack22 == 0x0) {
              return;
            }
          }
        }
      }
    }
  }
  return;
}
pub fn pass1_1038_3aa6(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uVar5: u32;
  let mut extraout_DX: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uStack12: u32;
  let mut uStack8: u32;

  uVar9 = (param_3 >> 0x10);
  iVar8 = (int)param_3;
  if ((*(i32 *)(iVar8 + 0x210) == 0x0) || (uVar2 = (u32)(iVar8 + 0x210), *(i32 *)((int)uVar2 + 0xa) == 0x0))
  {
    if (*(i32 *)(iVar8 + 0xc) == 0x0) {
      param_1 = 0x0;
      uVar6 = 0x0;
    }
    else {
      ppcVar1 = (code **)((int)(u32)(u32)(iVar8 + 0xc) + 0x10);
      (**ppcVar1)();
      uVar6 = extraout_DX;
    }
    uStack8 = CONCAT22(uVar6,param_1);
    for (uStack12 = 0x0; uStack12 < uStack8; uStack12 += 0x1) {
      uVar4 = uStack8;
      pass1_1030_1d58((u32)(iVar8 + 0xc));
      uVar7 = uVar6 | uVar4;
      if ((uVar7 != 0x0) && (uVar3 = pass1_1030_6fa0(uVar4 & 0xffff | (u32)uVar6 << 0x10), uVar3 == 0xb)) {
        pass1_1030_6b86(0xb,uVar4 & 0xffff | (u32)uVar6 << 0x10);
        return;
      }
      uVar6 = uVar7;
    }
  }
  else {
    uVar2 = (u32)(iVar8 + 0x210);
    uVar4 = (u32)((int)uVar2 + 0xa);
    for (uStack12 = 0x0; uStack12 < uVar4; uStack12 += 0x1) {
      uVar5 = uVar4;
      bad_1030_1312();
      uVar6 = param_2 | uVar5;
      if (uVar6 != 0x0) {
        pass1_1030_ce2e(uVar5,param_2,0x4);
      }
      param_2 = uVar6;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_3ba0(param_1: *mut astruct_428)

{
  u32 *puVar1;
  code **ppcVar2;
  u32 *puVar3;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut uVar6: u32;
  u8 *puVar7;
  u8 *puVar8;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar11;
  astruct_428 *iVar13;
  astruct_428 *uVar13;
  let mut uVar14: u16;
  u32 *puVar15;
  let mut uVar16: u32;
  let mut uStack20: u32;
  let mut uVar12: u32;

  uVar13 = (astruct_428 *)((u32)param_1 >> 0x10);
  iVar13 = (astruct_428 *)param_1;
  puVar1 = (u32*)&iVar13->field528_0x210;
  uVar5 = ((int)&iVar13->field528_0x210 + 0x2);
  uVar14 = ((u32)in_EDX >> 0x10);
  if ((uVar5 | puVar1) != 0x0) {
    ppcVar2 = (code **)*puVar1;
    (**ppcVar2)();
  }
  puVar15 = pass1_1008_c6fa(_u16_1050_06e0,0x1e);
  puVar7 = (u8 *)((u32)puVar15 >> 0x10);
  uVar6 = (u32)puVar15 & 0xffff;
  pass1_1038_4d6e(uVar6,puVar7,(astruct_691 *)param_1,puVar15);
  uVar4 = uVar6 & 0xffff;
  puVar3 = (u32 *)(uVar4 | ZEXT24(puVar7) << 0x10);
  ppcVar2 = (code **)((int)*puVar3 + 0x10);
  puVar8 = puVar7;
  (**ppcVar2)(0x1008,(int)uVar6,puVar7);
  uVar5 = uVar6;
  paVar11 = (astruct_57 *)CONCAT22(uVar14,puVar8);
  if ((puVar8 == NULL) && (uVar5 < 0x5)) {
    uVar5 = 0x5;
  }
  uVar5 += 0x1;
  uVar14 = 0x1000;
  uVar10 = uVar5;
  mem_op_1000_179c(0x1c,paVar11);
  uVar9 = paVar11 | uVar10;
  uVar12 = (u32)uVar9;
  if (uVar9 == 0x0) {
    iVar13->field528_0x210 = 0x0;
  }
  else {
    uVar12 = (u32)((int)uVar5 >> 0xf);
    uVar14 = 0x1030;
    struct_1030_11aa((astruct_156 *)CONCAT22(paVar11,uVar10),0x5,(long)(int)uVar5);
    &iVar13->field528_0x210 = uVar5;
    ((int)&iVar13->field528_0x210 + 0x2) = (int)uVar12;
  }
  uVar16 = iVar13->field528_0x210;
  ((int)uVar16 + 0x1a) = 0x0;
  for (uStack20 = 0x0; uStack20 < (uVar6 & 0xffff | ZEXT24(puVar8) << 0x10); uStack20 += 0x1) {
    uVar16 = pass1_1030_1d7c((int)(uVar6 & 0xffff),(int)uVar12,(u32)puVar3);
    uVar5 = (uVar16 >> 0x10);
    uVar10 = uVar5 | uVar16;
    uVar12 = (u32)uVar10;
    if (uVar10 != 0x0) {
      pass1_1030_1358((astruct_291 *)iVar13->field528_0x210,uVar16,uVar5,uStack20 + 0x1);
    }
    uVar14 = 0x1030;
  }
  if (puVar3 != NULL) {
    ppcVar2 = (code **)*puVar3;
    (**ppcVar2)(uVar14,(int)uVar4,(char)puVar7,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_3cc0(mut param_1: u32,mut param_2: u16 ,u8 *param_3,mut param_4: u16 )

{
  i32 lVar1;
  code **ppcVar2;
  u32 *puVar3;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut in_EDX: u32;
  astruct_57 *paVar10;
  u32 *puVar11;
  let mut uVar12: u16;
  let mut uVar13: u16;
  u32 *puVar14;
  astruct_15 *paVar15;
  let mut uVar16: u32;
  u8 uVar17;
  u8 uVar18;
  u8 uVar19;
  u8 uVar20;
  u32 *puStack26;
  let mut uStack22: u32;
  let mut uStack18: u32;
  let mut uStack14: u32;
  u32 *puStack10;

  uVar12 = ((u32)in_EDX >> 0x10);
  if (param_4 == 0x1e) {
    uVar13 = 0x1008;
    puVar14 = pass1_1008_c6fa(_u16_1050_06e0,0x27);
    paVar10 = (astruct_57 *)CONCAT22(uVar12,(int)((u32)puVar14 >> 0x10));
    puVar11 = (u32 *)puVar14;
    pass1_1038_4e78(puVar11,paVar10,param_1,puVar14);
    uVar12 = SUB42(paVar10,0x0);
    puStack10 = (u32 *)CONCAT22(uVar12,puVar11);
    ppcVar2 = (code **)((int)*puStack10 + 0x10);
    puVar3 = puVar11;
    (**ppcVar2)(0x1008,puVar11,uVar12);
    uStack14 = CONCAT22((int)paVar10,puVar3);
    for (uStack18 = 0x0; uStack18 < uStack14; uStack18 += 0x1) {
      paVar15 = (astruct_15 *)pass1_1030_1d7c(puVar3,(int)paVar10,(u32)puStack10);
      uVar6 = ((u32)paVar15 >> 0x10) | paVar15;
      paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)uVar6);
      if (uVar6 != 0x0) {
        uVar4 = pass1_1030_bfb8((u32)paVar15);
        puStack26 = (u32 *)CONCAT22(paVar10,uVar4);
        uVar6 = paVar10 | uVar4;
        paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)uVar6);
        if (uVar6 != 0x0) {
          pass1_1028_b58e(paVar15);
          uVar7 = SUB42(paVar10,0x0);
          paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | ZEXT24(param_3));
          if (CONCAT22(param_3,param_2) <= puStack26) {
            uVar13 = 0x1030;
            pass1_1030_7ddc(param_2,paVar10,CONCAT22(uVar7,uVar4),
                            CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,param_2)),0x1e);
            break;
          }
          pass1_1030_7ddc(param_2,paVar10,CONCAT22(uVar7,uVar4),(long)puStack26,0x1e);
          lVar1 = CONCAT22(param_3,param_2) - (long)puStack26;
          param_2 = lVar1;
          param_3 = (u8 *)((u32)lVar1 >> 0x10);
        }
      }
      uVar13 = 0x1030;
    }
    puStack26 = puStack10;
    if (puStack10 == NULL) {
      return;
    }
  }
  else {
    if (param_4 != 0x21) {
      uVar13 = 0x1008;
      puVar14 = pass1_1008_c6fa(_u16_1050_06e0,0x3);
      paVar10 = (astruct_57 *)CONCAT22(uVar12,(int)((u32)puVar14 >> 0x10));
      uVar6 = puVar14;
      pass1_1038_4e78(uVar6,paVar10,param_1,puVar14);
      puStack26 = (u32 *)CONCAT22((int)paVar10,uVar6);
      ppcVar2 = (code **)((int)*puStack26 + 0x10);
      (**ppcVar2)(0x1008,uVar6,(int)paVar10);
      uStack22 = CONCAT22((int)paVar10,uVar6);
      uStack18 = 0x0;//
LAB_1038_3e9c:
      if (uStack18 < uStack22) {
        uVar13 = 0x1030;
        paVar15 = (astruct_15 *)pass1_1030_1d7c(uVar6,(int)paVar10,(u32)puStack26);
        uVar8 = ((u32)paVar15 >> 0x10) | paVar15;
        paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)uVar8);
        if (uVar8 == 0x0) goto LAB_1038_3e98;
        uVar13 = 0x1028;
        uVar16 = pass1_1028_45e2(paVar15,uVar8,(u32)paVar15);
        uVar8 = uVar16;
        uVar9 = (uVar16 >> 0x10) | uVar8;
        paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)uVar9);
        if (uVar9 == 0x0) goto LAB_1038_3e98;
        pass1_1028_b58e(paVar15);
        uVar12 = SUB42(paVar10,0x0);
        uVar5 = CONCAT22(param_3,param_2);
        paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | ZEXT24(param_3));
        if (uVar16 < uVar5) {
          uVar13 = 0x1030;
          pass1_1030_7ddc(param_2,paVar10,CONCAT22(uVar12,uVar8),uVar16,param_4);
          lVar1 = CONCAT22(param_3,param_2) - uVar16;
          param_2 = lVar1;
          param_3 = (u8 *)((u32)lVar1 >> 0x10);
          goto LAB_1038_3e98;
        }
        uVar19 = SUB21(param_3,0x0);
        uVar20 = (u8)(param_3 >> 0x8);
        uVar17 = (u8)uVar8;
        uVar18 = (u8)(uVar8 >> 0x8);//
LAB_1038_3e67:
        uVar13 = 0x1030;
        pass1_1030_7ddc(uVar5,paVar10,CONCAT22(uVar12,CONCAT11(uVar18,uVar17)),
                        CONCAT13(uVar20,CONCAT12(uVar19,param_2)),param_4);
      }
      goto LAB_1038_3e6c;
    }
    uVar13 = 0x1008;
    puVar14 = pass1_1008_c6fa(_u16_1050_06e0,0xa);
    paVar10 = (astruct_57 *)CONCAT22(uVar12,(int)((u32)puVar14 >> 0x10));
    uVar6 = puVar14;
    pass1_1038_4e78(uVar6,paVar10,param_1,puVar14);
    puStack26 = (u32 *)CONCAT22((int)paVar10,uVar6);
    ppcVar2 = (code **)((int)*puStack26 + 0x10);
    (**ppcVar2)(0x1008,uVar6,(int)paVar10);
    uStack22 = CONCAT22((int)paVar10,uVar6);
    for (uStack18 = 0x0; uStack18 < uStack22; uStack18 += 0x1) {
      uVar13 = 0x1030;
      paVar15 = (astruct_15 *)pass1_1030_1d7c(uVar6,(int)paVar10,(u32)puStack26);
      uVar5 = (u32)paVar15 & 0xffff;
      uVar8 = ((u32)paVar15 >> 0x10) | uVar5;
      paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)uVar8);
      if (uVar8 != 0x0) {
        uVar19 = SUB21(param_3,0x0);
        uVar20 = (u8)(param_3 >> 0x8);
        pass1_1028_b58e(paVar15);
        uVar12 = SUB42(paVar10,0x0);
        uVar17 = (u8)uVar5;
        uVar18 = (u8)(uVar5 >> 0x8);
        goto LAB_1038_3e67;
      }
    }//
LAB_1038_3e6c:
    if (puStack26 == NULL) {
      return;
    }
    uVar12 = ((u32)puStack26 >> 0x10);
    puVar11 = (u32 *)puStack26;
  }
  ppcVar2 = (code **)*puVar11;
  (**ppcVar2)(uVar13,(int)puStack26,(char)uVar12,0x1);
  return;//
LAB_1038_3e98:
  uStack18 += 0x1;
  goto LAB_1038_3e9c;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_3efc(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32,mut param_5: i16,mut param_6: u16 )

{
  code **ppcVar1;
  u32 *puStack6;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4);
  puStack6 = (u32 *)CONCAT22(param_6,param_5);
  (u32)(param_5 + 0x1c) = (u32)((int)param_3 + 0x4);
  ppcVar1 = (code **)((int)*puStack6 + 0x58);
  (**ppcVar1)(0x1028,param_5,param_6,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_3f38(u32 *param_1,u32 *param_2,mut param_3: u32,mut param_4: i16,mut param_5: u16 )

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  u32 *puVar3;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut uVar6: u16;
  astruct_294 *paStack10;
  u32 *puStack6;

  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_3);
  puStack6 = (u32 *)CONCAT22(param_5,param_4);
  iVar2 = param_4;
  pass1_1028_b58e((astruct_15 *)CONCAT22(param_5,param_4));
  paStack10 = (astruct_294 *)CONCAT22(extraout_DX,iVar2);
  uVar5 = (u32)(iVar2 + 0x4);
  ppcVar1 = (code **)((int)*param_1 + 0x18);
  (**ppcVar1)(0x1028,param_1,uVar5);
  uVar6 = 0x0;
  uVar4 = 0x0;
  ppcVar1 = (code **)((int)*param_2 + 0x8);
  puVar3 = param_2;
  (**ppcVar1)();
  pass1_1030_73ee(extraout_DX_00,paStack10,(u32)((int)param_2 + 0x4));
  ppcVar1 = (code **)((int)*puStack6 + 0x58);
  (**ppcVar1)(0x1030,param_4,param_5,param_2,puVar3,uVar4,uVar5,uVar6);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_3fb0(mut param_1: u32)

{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)((int)param_1 + 0x200));
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_3fca(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u32;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_EDX: u32;
  StructD *pSVar6;
  let mut uVar7: u16;
  let mut unaff_SI: u16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u32;
  u32 *puVar14;
  let mut in_stack_0000fe6c: u16;
  let mut in_stack_0000ff90: u16;
  let mut in_stack_0000ff96: u16;
  let mut in_stack_0000ff9a: u16;
  u8 uVar15;
  u8 uVar16;
  u8 uVar17;
  u8 uVar18;
  let mut iStack38: i16;
  let mut local_24: i16;
  u8 local_22 [0x2];
  let mut piStack32: *mut i16;
  let mut uStack30: u16;
  u8 *puStack28;
  let mut uStack26: u16;
  let mut uStack24: u16;
  let mut uStack22: u32;
  let mut uStack18: u16;
  let mut uStack16: u16;
  char *pcStack14;
  char *pcStack10;
  let mut uStack6: u32;
  StructD *pSVar5;

  uVar9 = (param_2 >> 0x10);
  uVar7 = param_2;
  if (*(i32 *)(uVar7 + 0xc) == 0x0) {
    param_1 = 0x0;
    in_EDX &= 0xffff0000;
  }
  else {
    ppcVar2 = (code **)((int)(u32)(u32)(uVar7 + 0xc) + 0x10);
    (**ppcVar2)();
  }
  uStack6 = CONCAT22(in_EDX,param_1);
  uVar4 = in_EDX | param_1;
  pSVar5 = (StructD *)(in_EDX & 0xffff0000 | (u32)uVar4);
  if (uVar4 != 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar5);
    }
    else {
      pSVar5 = (StructD *)(in_EDX & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    uVar3 = fn_ptr_op_1000_1708((int)uStack6 << 0x2,0x0,0x1,PTR_LOOP_1050_5f2c,pSVar5);
    pcStack10 = (char *)CONCAT22((int)pSVar5,uVar3);
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar5);
    }
    else {
      pSVar5 = (StructD *)((u32)pSVar5 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    uVar12 = 0x1000;
    uVar3 = fn_ptr_op_1000_1708((int)uStack6 << 0x2,0x0,0x1,PTR_LOOP_1050_5f2c,pSVar5);
    pcStack14 = (char *)CONCAT22((int)pSVar5,uVar3);
    for (uStack22 = 0x0; uStack22 < uStack6; uStack22 += 0x1) {
      uVar1 = (u32)(uVar7 + 0xc);
      ppcVar2 = (code **)((int)(u32)(u32)(uVar7 + 0xc) + 0x4);
      uVar13 = uStack6;
      (**ppcVar2)(uVar12,(char)uVar1,(int)((u32)uVar1 >> 0x10),(char)uStack22,(int)(uStack22 >> 0x10));
      uVar4 = uVar13;
      uStack16 = pSVar5;
      pSVar6 = (StructD *)((u32)pSVar5 & 0xffff0000 | (u32)(uStack16 | uVar4));
      uStack18 = uVar4;
      if ((uStack16 | uVar4) != 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar13 & 0xffff | (long)pSVar5 << 0x10);
        (int)uStack22 *= 0x4;
        uVar10 = ((u32)pcStack10 >> 0x10);
        iVar8 = (int)pcStack10;
        ((int)uStack22 + iVar8) = uVar4;
        uVar11 = SUB42(pSVar6,0x0);
        ((int)uStack22 + iVar8 + 0x2) = uVar11;
        uVar12 = 0x1030;
        uVar13 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar11,((int)uStack22 + iVar8)),uVar4,uVar11
                                    );
        pSVar6 = (StructD *)((u32)pSVar6 & 0xffff0000 | uVar13 >> 0x10);
        uVar11 = ((u32)pcStack14 >> 0x10);
        ((int)pcStack14 + (int)uStack22) = (int)uVar13;
        ((int)pcStack14 + (int)uStack22 + 0x2) = (int)(uVar13 >> 0x10);
      }
      pSVar5 = pSVar6;
    }
    for (uStack22 = 0x0; uStack22 < uStack6; uStack22 += 0x1) {
      uVar12 = ((u32)pcStack14 >> 0x10);
      iVar8 = (int)pcStack14;
      if ((*(i32 *)((int)uStack22 * 0x4 + iVar8) != 0x0) &&
         (uVar1 = (u32)((int)uStack22 * 0x4 + iVar8), ((int)uVar1 + 0x1a) = 0x0,
         uVar1 = (u32)((int)uStack22 * 0x4 + iVar8), ((int)uVar1 + 0x12) == 0x5)) {
        pass1_1028_bdac(*(astruct_15 **)((int)uStack22 * 0x4 + iVar8),0x6);
      }
    }
    (uVar7 + 0x204) = 0x0;
    puVar14 = mixed_1010_20ba((astruct_57 *)pSVar5,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000fe6c
                              ,in_stack_0000ff90,in_stack_0000ff96,in_stack_0000ff9a);
    uStack30 = ((u32)puVar14 >> 0x10);
    uStack26 = SUB42(puVar14,0x0);
    puStack28 = PTR_LOOP_1050_13ae;
    if (PTR_LOOP_1050_13ae == (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1)) {
      (uVar7 + 0x204) = 0x1;
    }
    uStack24 = uStack30;
    pass1_1038_5a96(uVar7,uVar9,uStack6,(u32)pcStack14);
    pass1_1038_5cc6(param_2,uStack6,(u32)pcStack14,(u32)pcStack10,0x0,0x2);
    pass1_1038_5b3c(uVar7,uVar9,uStack6,(u32)pcStack14);
    pass1_1038_5cc6(param_2,uStack6,(u32)pcStack14,(u32)pcStack10,0x0,0x1);
    uVar11 = SUB42(&DAT_1050_1050,0x0);
    uVar17 = SUB21(local_22,0x0);
    uVar18 = (u8)(local_22 >> 0x8);
    piStack32 = &local_24;
    uVar12 = SUB42(&DAT_1050_1050,0x0);
    uVar15 = SUB21(piStack32,0x0);
    uVar16 = (u8)(piStack32 >> 0x8);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(uVar7 + 0x8));
    pass1_1030_5b1c(CONCAT22(uStack30,piStack32),(u16 *)CONCAT22(uVar12,CONCAT11(uVar16,uVar15)),
                    (u16 *)CONCAT22(uVar11,CONCAT11(uVar18,uVar17)));
    for (iStack38 = 0x1; iStack38 <= local_24; iStack38 += 0x1) {
      pass1_1038_58e6(uVar7,uVar9,uStack6,(u32)pcStack14,(u32)pcStack10,iStack38);
      pass1_1038_5cc6(param_2,uStack6,(u32)pcStack14,(u32)pcStack10,iStack38,0x3);
    }
    pass1_1038_5a16(uVar7,uVar9,uStack6,(u32)pcStack14);
    for (uStack22 = 0x0; uStack22 < uStack6; uStack22 += 0x1) {
      uVar12 = ((u32)pcStack14 >> 0x10);
      iVar8 = (int)pcStack14;
      if ((*(i32 *)((int)uStack22 * 0x4 + iVar8) != 0x0) &&
         (uVar1 = (u32)((int)uStack22 * 0x4 + iVar8), ((int)uVar1 + 0x12) != 0x5)) {
        uVar1 = (u32)((int)uStack22 * 0x4 + iVar8);
        ppcVar2 = (code **)((int)(u32)(u32)((int)uStack22 * 0x4 + iVar8) + 0x28);
        (**ppcVar2)(0x1030,(char)uVar1,(int)((u32)uVar1 >> 0x10));
      }
    }
    fn_ptr_1000_17ce(pcStack10);
    fn_ptr_1000_17ce(pcStack14);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_42cc(mut param_1: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut bVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  u8 *puVar7;
  let mut extraout_DX: u16;
  let mut uVar8: u16;
  let mut extraout_DX_00: u16;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  u32 *puVar12;
  u32 *puVar13;
  let mut uStack24: u32;
  let mut uStack18: u32;
  u32 *puStack10;

  uVar10 = (param_1 >> 0x10);
  iVar9 = (int)param_1;
  if (*(i32 *)(iVar9 + 0x1f6) == 0x0) {
    return;
  }
  uVar11 = 0x1008;
  puVar12 = pass1_1008_c6fa(_u16_1050_06e0,0x2d);
  puVar7 = (u8 *)((u32)puVar12 >> 0x10);
  uVar4 = puVar12;
  pass1_1038_4d6e(uVar4,puVar7,(astruct_691 *)param_1,puVar12);
  puStack10 = (u32 *)CONCAT22(puVar7,uVar4);
  ppcVar1 = (code **)((int)*puStack10 + 0x10);
  uVar5 = uVar4;
  (**ppcVar1)(0x1008,uVar4,puVar7);
  uStack18 = CONCAT22(extraout_DX,uVar5);
  bVar3 = false;
  uVar8 = extraout_DX;
  for (uStack24 = 0x0; uStack24 < uStack18; uStack24 += 0x1) {
    uVar11 = 0x1030;
    puVar13 = (u32 *)pass1_1030_1d7c(uVar5,uVar8,(u32)puStack10);
    uVar6 = puVar13;
    uVar8 = ((u32)puVar13 >> 0x10) | uVar6;
    if (uVar8 != 0x0) {
      ppcVar1 = (code **)((int)*puVar13 + 0x50);
      (**ppcVar1)();
      uVar8 = extraout_DX_00;
      if (uVar6 != 0x0) {
        bVar3 = true;
      }
    }
  }
  if (bVar3) {
    uVar2 = (u32)(iVar9 + 0x1f6);
    (u32)((int)uVar2 + 0x1aa) = 0x0;
  }
  else {
    uVar11 = 0x1030;
    pass1_1030_38b8();
    uVar8 |= uStack18;
    if (uVar8 != 0x0) {
      uVar11 = 0x1030;
      pass1_1030_326a(uStack18,uVar8,*(astruct_692 **)(iVar9 + 0x1f6));
    }
  }
  if (puStack10 != NULL) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(uVar11,uVar4,(char)puVar7,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_43cc(mut param_1: i16,mut param_2: i16,mut param_3: i16,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar6;
  astruct_57 *paVar7;
  let mut iVar8: i16;
  let mut iVar9: i16;
  let mut uVar10: u16;
  u32 *puVar11;
  astruct_15 *paVar12;
  let mut uStack22: u32;
  let mut uStack18: u32;
  u32 *puStack14;

  if (param_6 == 0x5) {
    pass1_1038_4900(CONCAT22(param_4,param_3));
    return;
  }
  pass1_1038_53ba(CONCAT22(param_4,param_3),param_6);
  if ((param_2 != 0x0) || (param_1 != 0x0)) {
    iVar8 = param_6 * 0x4;
    uVar2 = (param_3 + iVar8 + 0x14e);
    iVar9 = ((param_3 + iVar8 + 0x150) - ((int)param_5 >> 0xf)) - (uVar2 < param_5);
    (param_3 + iVar8 + 0x14e) = uVar2 - param_5;
    (param_3 + iVar8 + 0x150) = iVar9;
    if (iVar9 < 0x0) {
      (u32)(param_3 + iVar8 + 0x14e) = 0x0;
    }
    uVar10 = 0x1008;
    puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0x1e);
    paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,(int)((u32)puVar11 >> 0x10));
    uVar2 = puVar11;
    pass1_1038_4e78(uVar2,paVar6,CONCAT22(param_4,param_3),puVar11);
    puStack14 = (u32 *)CONCAT22((int)paVar6,uVar2);
    ppcVar1 = (code **)((int)*puStack14 + 0x10);
    paVar7 = paVar6;
    uVar3 = uVar2;
    (**ppcVar1)(0x1008,uVar2,(int)paVar6);
    uStack18 = CONCAT22((int)paVar7,uVar3);
    for (uStack22 = 0x0; uStack22 < uStack18; uStack22 += 0x1) {
      paVar12 = (astruct_15 *)pass1_1030_1d7c(uVar3,(int)paVar7,(u32)puStack14);
      paVar7 = (astruct_57 *)((u32)paVar12 >> 0x10);
      uVar5 = (u32)paVar12 & 0xffff;
      for (; uVar4 = uVar5, param_5 != 0x0; param_5 -= 0x1) {
        pass1_1030_cf78(paVar12,param_6);
        uVar5 = (u32)uVar4;
        if (uVar4 == 0x0) break;
      }
      uVar10 = 0x1030;
      if (param_5 == 0x0) break;
    }
    if (puStack14 != NULL) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(uVar10,uVar2,(char)paVar6,0x1);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_44d8(mut param_1: i16,mut param_2: i16,mut param_3: i16,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u32;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar6;
  astruct_57 *paVar7;
  astruct_697 *iVar9;
  let mut iVar8: i16;
  let mut uVar9: u16;
  u32 *puVar10;
  astruct_15 *paVar11;
  let mut uStack22: u32;
  let mut uStack18: u32;
  u32 *puStack14;

  if (param_6 == 0x5) {
    pass1_1038_4900(CONCAT22(param_4,param_3));
    return;
  }
  pass1_1038_53ba(CONCAT22(param_4,param_3),param_6);
  if ((param_2 != 0x0) || (param_1 != 0x0)) {
    iVar9 = (astruct_697 *)(param_6 * 0x4);
    uVar2 = (iVar9 + param_3 + 0x14e);
    iVar8 = ((iVar9 + param_3 + 0x150) - ((int)param_5 >> 0xf)) - (uVar2 < param_5);
    (iVar9 + param_3 + 0x14e) = uVar2 - param_5;
    (iVar9 + param_3 + 0x150) = iVar8;
    if (iVar8 < 0x0) {
      (u32)(iVar9 + param_3 + 0x14e) = 0x0;
    }
    uVar9 = 0x1008;
    puVar10 = pass1_1008_c6fa(_u16_1050_06e0,0x1e);
    paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,(int)((u32)puVar10 >> 0x10));
    uVar2 = puVar10;
    pass1_1038_4e78(uVar2,paVar6,CONCAT22(param_4,param_3),puVar10);
    puStack14 = (u32 *)CONCAT22((int)paVar6,uVar2);
    ppcVar1 = (code **)((int)*puStack14 + 0x10);
    paVar7 = paVar6;
    uVar3 = uVar2;
    (**ppcVar1)(0x1008,uVar2,(int)paVar6);
    uStack18 = CONCAT22((int)paVar7,uVar3);
    for (uStack22 = 0x0; uStack22 < uStack18; uStack22 += 0x1) {
      paVar11 = (astruct_15 *)pass1_1030_1d7c(uVar3,(int)paVar7,(u32)puStack14);
      paVar7 = (astruct_57 *)((u32)paVar11 >> 0x10);
      uVar5 = (u32)paVar11 & 0xffff;
      for (; uVar4 = uVar5, param_5 != 0x0; param_5 -= 0x1) {
        pass1_1030_d00c(paVar11,param_6);
        uVar5 = (u32)uVar4;
        if (uVar4 == 0x0) break;
      }
      uVar9 = 0x1030;
      if (param_5 == 0x0) break;
    }
    if (puStack14 != NULL) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(uVar9,uVar2,(char)paVar6,0x1);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_45e4(mut param_1: u16 ,mut param_2: i16,mut param_3: u32)

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut iVar8: i16;
  let mut iVar9: i16;
  let mut iVar10: i16;
  u8 *puVar11;
  let mut iVar12: i16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut bVar15: bool;
  u32 *puVar16;
  let mut uStack28: u16;
  u32 *puStack22;

  uVar14 = (param_3 >> 0x10);
  iVar12 = (int)param_3;
  pass1_1030_38f2((u32)(iVar12 + 0x1f6),0x2);
  iVar8 = param_2;
  uVar4 = param_1;
  pass1_1030_38f2((u32)(iVar12 + 0x1f6),0x1);
  bVar15 = param_1 < uVar4;
  uVar13 = param_1 - uVar4;
  iVar10 = param_2 - iVar8;
  pass1_1030_38f2((u32)(iVar12 + 0x1f6),0x4);
  iVar9 = iVar8;
  uVar5 = uVar4;
  pass1_1030_38f2((u32)(iVar12 + 0x1f6),0x3);
  uVar7 = (iVar12 + 0x24);
  uVar6 = uVar7 + (uVar4 - uVar5);
  iVar10 = ((int)uVar7 >> 0xf) + ((iVar8 - iVar9) - (uVar4 < uVar5)) + CARRY2(uVar7,uVar4 - uVar5) +
           (iVar10 - bVar15) + CARRY2(uVar6,uVar13);
  if ((iVar10 < 0x0) || ((iVar10 < 0x1 && (uVar6 + uVar13 == 0x0)))) {
    iVar10 = -0x1;
  }
  else {
    iVar10 = 0x1;
  }
  piVar1 = (i16 *)(iVar12 + 0x24);
  *piVar1 = *piVar1 + iVar10;
  puVar16 = pass1_1008_c6fa(_u16_1050_06e0,0x16);
  puVar11 = (u8 *)((u32)puVar16 >> 0x10);
  uVar7 = puVar16;
  pass1_1038_4d6e(uVar7,puVar11,(astruct_691 *)param_3,puVar16);
  puStack22 = (u32 *)CONCAT22(puVar11,uVar7);
  uVar3 = *puStack22;
  ppcVar2 = (code **)uVar3 + 0x8;
  uVar5 = uVar7;
  (**ppcVar2)(0x1008,uVar7,puVar11);
  if (puStack22 != NULL) {
    ppcVar2 = (code **)uVar3;
    (**ppcVar2)(0x1008,uVar7,(char)puVar11,0x1);
  }
  piVar1 = (i16 *)(iVar12 + 0x24);
  *piVar1 = *piVar1 + uVar5 * 0x2;
  iVar10 = (iVar12 + 0x24);
  if (0x64 < iVar10) {
    iVar10 = 0x64;
  }
  (iVar12 + 0x24) = iVar10;
  if (iVar10 < 0x0) {
    iVar10 = 0x0;
  }
  (iVar12 + 0x24) = iVar10;
  iVar10 /= 0xa;
  uStack28 = 0x10;
  if (iVar10 < 0xb) {
    uStack28 = 0x14;
  }
  else if (iVar10 < 0x15) {
    uStack28 = 0x13;
  }
  else if (iVar10 < 0x1f) {
    uStack28 = 0x12;
  }
  else if (iVar10 < 0x29) {
    uStack28 = 0x11;
  }
  else if (iVar10 < 0x33) {
    uStack28 = 0x10;
  }
  else if (iVar10 < 0x3d) {
    uStack28 = 0xf;
  }
  else if (iVar10 < 0x47) {
    uStack28 = 0xe;
  }
  else if (iVar10 < 0x51) {
    uStack28 = 0xd;
  }
  else if (iVar10 < 0x5b) {
    uStack28 = 0xc;
  }
  pass1_1030_3258((u32)(iVar12 + 0x1f6),uStack28);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_4760(mut param_1: u32)

{
  let mut puVar1: *mut u16;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  u8 *puVar7;
  u8 *puVar8;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut extraout_DX_01: u16;
  let mut extraout_DX_02: u16;
  let mut extraout_DX_03: u16;
  let mut uVar9: u16;
  astruct_700 *iVar10;
  let mut uVar10: u16;
  let mut uVar11: u16;
  u32 *puVar12;
  let mut uVar13: u32;
  u32 *puVar14;
  u8 uVar15;
  u8 *puVar16;
  let mut uStack26: u32;
  let mut uStack22: u32;
  u32 *puStack14;
  u32 *puStack10;

  uVar10 = (param_1 >> 0x10);
  iVar10 = (astruct_700 *)param_1;
  puVar1 = &iVar10->field33_0x22;
  *puVar1 = *puVar1 + iVar10->field521_0x20c;
  puVar12 = pass1_1008_c6fa(_u16_1050_06e0,0x26);
  puVar7 = (u8 *)((u32)puVar12 >> 0x10);
  uVar6 = puVar12;
  pass1_1038_4d6e(uVar6,puVar7,(astruct_691 *)param_1,puVar12);
  puStack10 = (u32 *)CONCAT22(puVar7,uVar6);
  uVar11 = 0x1008;
  puVar12 = pass1_1008_c6fa(_u16_1050_06e0,0x1a);
  puVar8 = (u8 *)((u32)puVar12 >> 0x10);
  uVar3 = puVar12;
  pass1_1038_4d6e(uVar3,puVar8,(astruct_691 *)param_1,puVar12);
  puStack14 = (u32 *)CONCAT22(puVar8,uVar3);
  ppcVar2 = (code **)((int)*puStack14 + 0x10);
  uVar4 = uVar3;
  (**ppcVar2)(0x1008,uVar3,puVar8);
  uVar15 = (u8)uVar6;
  puVar16 = puVar7;
  if ((extraout_DX | uVar4) == 0x0) {
    ppcVar2 = (code **)((int)*puStack10 + 0x10);
    (**ppcVar2)();
    puVar1 = &iVar10->field33_0x22;
    *puVar1 = *puVar1 + uVar4;
    uVar9 = extraout_DX_00;
  }
  else {
    ppcVar2 = (code **)((int)*puStack10 + 0x10);
    (**ppcVar2)();
    uStack22 = CONCAT22(extraout_DX_03,uVar4);
    uVar9 = extraout_DX_03;
    for (uStack26 = 0x0; uStack26 < uStack22; uStack26 += 0x1) {
      puVar14 = puStack14;
      uVar13 = pass1_1030_1d7c(uVar4,uVar9,(u32)puStack10);
      uVar9 = (uVar13 >> 0x10);
      uVar5 = uVar13;
      uVar11 = 0x1028;
      FUN_1028_5a94(uVar5,0x1030,(astruct_15 *)(uVar13 & 0xffff | (u32)uVar9 << 0x10),puVar14);
      if (uVar5 == 0x2) {
        if ((*_PTR_LOOP_1050_65e2 & 0x1) == 0x0) goto LAB_1038_485e;
      }
      else if (uVar5 != 0x3) {//
LAB_1038_485e:
        puVar1 = &iVar10->field33_0x22;
        *puVar1 = *puVar1 + 0x1;
      }
    }
  }
  if (puStack10 != NULL) {
    ppcVar2 = (code **)*puStack10;
    (**ppcVar2)(uVar11,uVar6,puVar7,0x1,uVar15,puVar16);
    uVar9 = extraout_DX_01;
  }
  if (puStack14 != NULL) {
    ppcVar2 = (code **)*puStack14;
    (**ppcVar2)(uVar11,uVar3,puVar8,0x1);
    uVar9 = extraout_DX_02;
  }
  pass1_1038_45e4((int)puStack14,uVar9,param_1);
  if (0x32 < iVar10->field34_0x24) {
    puVar1 = &iVar10->field33_0x22;
    *puVar1 = *puVar1 - 0x1;
  }
  if (iVar10->field34_0x24 < 0x32) {
    puVar1 = &iVar10->field33_0x22;
    *puVar1 = *puVar1 + 0x1;
  }
  if (iVar10->field24_0x18 < 0xfa) {
    puVar1 = &iVar10->field33_0x22;
    *puVar1 = *puVar1 + 0x2;
  }
  else if (iVar10->field24_0x18 < 0x1c2) {
    puVar1 = &iVar10->field33_0x22;
    *puVar1 = *puVar1 + 0x1;
  }
  else if (0x225 < iVar10->field24_0x18) {
    if (iVar10->field24_0x18 < 0x2ee) {
      puVar1 = &iVar10->field33_0x22;
      *puVar1 = *puVar1 - 0x1;
    }
    else {
      puVar1 = &iVar10->field33_0x22;
      *puVar1 = *puVar1 - 0x2;
    }
  }
  uVar6 = iVar10->field33_0x22;
  if (0x64 < (int)uVar6) {
    uVar6 = 0x64;
  }
  iVar10->field33_0x22 = uVar6;
  if ((int)uVar6 < 0x0) {
    uVar6 = 0x0;
  }
  iVar10->field33_0x22 = uVar6;
  return;
}
pub fn pass1_1038_48e0(mut param_1: u32,mut param_2: i16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = ((int)param_1 + 0x20e) + param_2;
  if (0xa < iVar1) {
    iVar1 = 0xa;
  }
  ((int)param_1 + 0x20e) = iVar1;
  return;
}
pub fn pass1_1038_4900(mut param_1: u32)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  piVar1 = (i16 *)((int)param_1 + 0x20e);
  *piVar1 = *piVar1 + -0x1;
  if (*piVar1 < 0x0) {
    ((int)param_1 + 0x20e) = 0x0;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_4918(mut param_1: i16,mut param_2: u16 ,mut param_3: u32)

{
  let mut piVar1: *mut i16;
  let mut iVar2: i16;
  u32 *puVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u32;
  u8 bStack347;
  u8 local_14a [0x4];
  u32 *puStack326;
  u8 local_144 [0x124];
  let mut local_20: u32;
  let mut uStack28: u16;
  let mut uStack26: u32;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar8 = (param_3 >> 0x10);
  iVar6 = (int)param_3;
  if (*(i32 *)(iVar6 + 0x4) != 0x4000001) {
    return;
  }
  pass1_1028_e1ec((u32)_PTR_LOOP_1050_65e2,(u32)(iVar6 + 0x8));
  uStack6 = CONCAT22(param_2,param_1);
  uStack10 = (u32)(param_1 + 0x10);
  uVar9 = ((u32)uStack10 >> 0x10);
  iVar7 = (int)uStack10;
  if ((iVar7 + 0x1c) == 0x0) {
    return;
  }
  uStack14 = 0x0;
  switch((iVar6 + 0x20e)) {
  case 0x1:
    uStack14._0_2_ = 0x1e;
    break;
  case 0x2:
    uStack14._0_2_ = 0x1c;
    break;
  case 0x3:
    uStack14._0_2_ = 0x1a;
    break;
  case 0x4:
    uStack14._0_2_ = 0x18;
    break;
  case 0x5:
    uStack14._0_2_ = 0x16;
    break;
  case 0x6:
    uStack14._0_2_ = 0x14;
    break;
  case 0x7:
    uStack14._0_2_ = 0x12;
    break;
  case 0x8:
    uStack14._0_2_ = 0x10;
    break;
  case 0x9:
    uStack14._0_2_ = 0xe;
    break;
  case 0xa:
    uStack14._0_2_ = 0xc;
    break;
  default:
    goto switchD_1038_49cf_caseD_a;
  }
  uStack14 = (u32)uStack14;
switchD_1038_49cf_caseD_a:
  uStack18 = *_PTR_LOOP_1050_65e2;
  if ((uStack14 != 0x0) &&
     ((int)((uStack18 & 0xffff | (u32)((int)_PTR_LOOP_1050_65e2 + 0x2) << 0x10) % uStack14) == 0x0)) {
    piVar1 = (i16 *)(iVar7 + 0x1c);
    *piVar1 = *piVar1 + -0x1;
    piVar1 = (i16 *)(iVar7 + 0x1a);
    *piVar1 = *piVar1 + 0x1;
    iVar2 = (iVar7 + 0x1a) * 0x6 + (iVar7 + 0x16);
    uVar9 = (iVar7 + 0x18);
    local_20 = (u32)(iVar2 + -0x6);
    uStack28 = (iVar2 + -0x2);
    puStack326 = &local_20;
    puVar3 = &local_20;
    pass1_1030_64ce(puVar3,uVar9,_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar3),*(i32 *)(iVar6 + 0x8),
                    (u32 *)CONCAT22(0x1050,local_14a));
    uStack26 = *puVar3;
    uVar5 = ((int)puVar3 + 0x2);
    bStack347 = (u8)(uStack26 >> 0x18);
    uVar4 = bStack347;
    if (bStack347 != 0x0) {
      pass1_1028_e1ec((u32)_PTR_LOOP_1050_65e2,uStack26 & 0xffff | (u32)uVar5 << 0x10);
      uVar10 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar5,uVar4),uVar4,uVar5);
      uVar5 = (uVar10 >> 0x10);
      if ((uVar5 | uVar10) != 0x0) {
        iVar7 = (uVar10 + 0xc);
        if (iVar7 < 0x1) {
          return;
        }
        if (SBORROW2(iVar7,0x1)) {
          return;
        }
        if (0x8 < iVar7 + -0x1) {
          return;
        }
      }
    }
    struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_144),0x0,0x0,0x10,&local_20,&DAT_1050_1050,
                        (u32)(iVar6 + 0x4),(u32)(iVar6 + 0x8));
    fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_144));
  }
  return;
}
pub fn pass1_1038_4b20(mut param_1: u16 ,mut param_2: u32,mut param_3: u32,mut param_4: u32)

{
  astruct_360 *paVar1;

  paVar1 = *(astruct_360 **)((int)param_2 + 0xc);
  pass1_1020_c4f4((astruct_361 *)paVar1,param_1,paVar1,param_3,(param_3 >> 0x10),param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_4b40(mut param_1: u16 ,mut param_2: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut extraout_DX_00: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut unaff_CS: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;

  uVar7 = (param_2 >> 0x10);
  iVar6 = (int)param_2;
  if (*(i32 *)(iVar6 + 0xc) == 0x0) {
    param_1 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar1 = (code **)((int)(u32)(u32)(iVar6 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar4 = extraout_DX;
  }
  uStack10 = CONCAT22(uVar4,param_1);
  for (uStack14 = 0x0; uStack14 < uStack10; uStack14 += 0x1) {
    ppcVar1 = (code **)((int)(u32)(u32)(iVar6 + 0xc) + 0x4);
    uVar3 = uStack10;
    (**ppcVar1)(unaff_CS,(u32)(iVar6 + 0xc));
    uVar2 = uVar3;
    uVar5 = extraout_DX_00 | uVar2;
    if (uVar5 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3 & 0xffff | (u32)extraout_DX_00 << 0x10);
      unaff_CS = 0x1030;
      struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar5,uVar2),uVar2,uVar5);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_4c1a(mut param_1: u16 ,mut param_2: u32,mut param_3: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut unaff_CS: u16;
  let mut uVar8: u32;
  let mut uStack14: u32;
  let mut uStack10: u32;

  uVar7 = (param_3 >> 0x10);
  iVar6 = (int)param_3;
  uVar8 = (u32)(iVar6 + 0xc);
  ppcVar1 = (code **)((int)(u32)(u32)(iVar6 + 0xc) + 0x10);
  (**ppcVar1)();
  uStack10 = CONCAT22((int)param_2,param_1);
  for (uStack14 = 0x0; uVar5 = param_2, uStack14 < uStack10; uStack14 += 0x1) {
    ppcVar1 = (code **)((int)(u32)(u32)(iVar6 + 0xc) + 0x4);
    uVar4 = uStack10;
    (**ppcVar1)(unaff_CS,(u32)(iVar6 + 0xc),uStack14,uVar8);
    uVar2 = uVar4;
    param_2 = (u32)(uVar5 | uVar2);
    if ((uVar5 | uVar2) != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | (u32)uVar5 << 0x10);
      uVar3 = pass1_1030_6fa0(CONCAT22((int)param_2,uVar2));
      unaff_CS = 0x1008;
      pass1_1008_c6ae(_u16_1050_06e0,uVar3,0xe);
    }
  }
  return;
}
pub fn pass1_1038_4cba(void)

{
  pass1_1030_38b8();
  return;
}
pub fn pass1_1038_4cd0(mut param_1: u32,mut param_2: u32,mut param_3: u16 )

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  ((int)param_1 + 0x1c) = param_3;
  (u32)((int)param_1 + 0x1e) = param_2;
  return;
}
pub fn pass1_1038_4cea(mut param_1: u32,u32 *param_2,param_3: *mut u16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *param_3 = ((int)param_1 + 0x1c);
  *param_2 = (u32)((int)param_1 + 0x1e);
  return;
}
pub fn pass1_1038_4d0e(param_1: *mut astruct_685,mut param_2: u16 )

{
  astruct_686 *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_686 *)param_1;
  iVar1->field25_0x1a = iVar1->field24_0x18;
  iVar1->field24_0x18 = param_2;
  return;
}



char * pass1_1038_4d28(char *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  return (char *)CONCAT22(((int)param_1 + 0x1fc),((int)param_1 + 0x1fa));
}
pub fn pass1_1038_4d3c(mut param_1: u32,char *param_2,mut param_3: u16 )

{
  let mut uVar1: u16;
  let mut iVar2: i16;
  let mut uVar3: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  fn_ptr_1000_17ce(*(char **)(iVar2 + 0x1fa));
  uVar1 = str_op_1008_60e8(param_3,param_2);
  (iVar2 + 0x1fa) = uVar1;
  (iVar2 + 0x1fc) = param_3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_4d6e(mut param_1: u16 ,u8 *param_2,param_3: *mut astruct_691,u32 *param_4)

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar6;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u32;
  let mut iStack30: i16;
  astruct_419 *paStack26;
  let mut uStack14: u32;
  let mut uStack10: u32;
  u32 *puStack6;
  let mut uVar7: u32;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x18,paVar6);
  uVar5 = paVar6 | param_1;
  uVar7 = (u32)uVar5;
  if (uVar5 == 0x0) {
    param_1 = 0x0;
    uVar7 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT22(paVar6,param_1),0x5,0x5);
  }
  puStack6 = (u32 *)CONCAT22((int)uVar7,param_1);
  uVar9 = ((u32)param_3 >> 0x10);
  iVar8 = (int)param_3;
  if (*(i32 *)(iVar8 + 0xc) == 0x0) {
    param_1 = 0x0;
    uVar7 = 0x0;
  }
  else {
    ppcVar2 = (code **)((int)(u32)(u32)(iVar8 + 0xc) + 0x10);
    (**ppcVar2)();
  }
  uStack10 = CONCAT22((int)uVar7,param_1);
  uStack14 = 0x0;
  do {
    uVar5 = uVar7;
    if (uStack10 <= uStack14) {
      return;
    }
    ppcVar2 = (code **)((int)(u32)(u32)(iVar8 + 0xc) + 0x4);
    uVar10 = uStack10;
    (**ppcVar2)();
    uVar3 = uVar10;
    uVar7 = (u32)(uVar5 | uVar3);
    if ((uVar5 | uVar3) != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar10 & 0xffff | (u32)uVar5 << 0x10);
      paStack26 = (astruct_419 *)CONCAT22((int)uVar7,uVar3);
      uVar4 = pass1_1030_6fa0(CONCAT22((int)uVar7,uVar3));
      iStack30 = 0x0;
      while( true ) {
        piVar1 = (i16 *)((int)param_4 + 0x4);
        if (*piVar1 == iStack30 || *piVar1 < iStack30) break;
        if (((int)*param_4 + iStack30 * 0x2) == uVar4) {
          uVar10 = struct_op_1030_73a8(paStack26,uVar4,(int)uVar7);
          uVar7 = uVar10 >> 0x10;
          if (((int)uVar10 + 0x12) == 0x5) {
            ppcVar2 = (code **)((int)*puStack6 + 0xc);
            (**ppcVar2)();
          }
          break;
        }
        iStack30 += 0x1;
      }
    }
    uStack14 += 0x1;
  } while( true );
}
pub fn pass1_1038_4e78(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u32,u32 *param_4)

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut iStack26: i16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  u32 *puStack6;
  let mut uVar7: u32;

  mem_op_1000_179c(0x18,param_2);
  uVar5 = param_2 | param_1;
  uVar7 = (u32)uVar5;
  if (uVar5 == 0x0) {
    param_1 = 0x0;
    uVar7 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_2,param_1),0x5,0x5);
  }
  puStack6 = (u32 *)CONCAT22((int)uVar7,param_1);
  uVar9 = (param_3 >> 0x10);
  iVar8 = (int)param_3;
  if (*(i32 *)(iVar8 + 0xc) == 0x0) {
    param_1 = 0x0;
    uVar7 = 0x0;
  }
  else {
    ppcVar2 = (code **)((int)(u32)(u32)(iVar8 + 0xc) + 0x10);
    (**ppcVar2)();
  }
  uStack10 = CONCAT22((int)uVar7,param_1);
  uStack14 = 0x0;
  do {
    uVar5 = uVar7;
    if (uStack10 <= uStack14) {
      return;
    }
    uVar4 = uStack10;
    pass1_1030_1d58((u32)(iVar8 + 0xc));
    uVar6 = uVar5 | uVar4;
    uVar7 = (u32)uVar6;
    if (uVar6 != 0x0) {
      uVar3 = pass1_1030_6fa0(uVar4 & 0xffff | (u32)uVar5 << 0x10);
      iStack26 = 0x0;
      while( true ) {
        piVar1 = (i16 *)((int)param_4 + 0x4);
        if (*piVar1 == iStack26 || *piVar1 < iStack26) break;
        if (((int)*param_4 + iStack26 * 0x2) == uVar3) {
          ppcVar2 = (code **)((int)*puStack6 + 0xc);
          (**ppcVar2)();
          break;
        }
        iStack26 += 0x1;
      }
    }
    uStack14 += 0x1;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_4f54(mut param_1: u16 ,mut param_2: u32,mut param_3: u16 )

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut uVar4: u32;
  let mut extraout_DX: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar8 = (param_2 >> 0x10);
  iVar7 = (int)param_2;
  if (*(i32 *)(iVar7 + 0xc) == 0x0) {
    param_1 = 0x0;
    uVar5 = 0x0;
  }
  else {
    ppcVar1 = (code **)((int)(u32)(u32)(iVar7 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar5 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar5,param_1);
  uStack10 = 0x0;
  do {
    if (uStack6 <= uStack10) {
      return;
    }
    uVar4 = uStack6;
    pass1_1030_1d58((u32)(iVar7 + 0xc));
    uVar6 = uVar5 | uVar4;
    if (uVar6 != 0x0) {
      uVar2 = pass1_1030_6fa0(uVar4 & 0xffff | (u32)uVar5 << 0x10);
      BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,param_3);
      if (BVar3 != 0x0) {
        return;
      }
    }
    uStack10 += 0x1;
    uVar5 = uVar6;
  } while( true );
}
pub fn pass1_1038_4fd8(mut param_1: u16 ,mut param_2: u32,mut param_3: u16 )

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar7 = (param_2 >> 0x10);
  iVar6 = (int)param_2;
  if (*(i32 *)(iVar6 + 0xc) == 0x0) {
    param_1 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar1 = (code **)((int)(u32)(u32)(iVar6 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar4 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar4,param_1);
  uStack10 = 0x0;
  do {
    if (uStack6 <= uStack10) {
      return;
    }
    uVar3 = uStack6;
    pass1_1030_1d58((u32)(iVar6 + 0xc));
    uVar5 = uVar4 | uVar3;
    if (uVar5 != 0x0) {
      uVar2 = pass1_1030_6fa0(uVar3 & 0xffff | (u32)uVar4 << 0x10);
      if (uVar2 == param_3) {
        return;
      }
    }
    uStack10 += 0x1;
    uVar4 = uVar5;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_5050(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 )

{
  code **ppcVar1;
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
  iVar6 = (int)param_3;
  if (*(i32 *)(iVar6 + 0xc) == 0x0) {
    param_1 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar1 = (code **)((int)(u32)(u32)(iVar6 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar4 = extraout_DX;
  }
  uStack10 = CONCAT22(uVar4,param_1);
  for (uStack14 = 0x0; uStack14 < uStack10; uStack14 += 0x1) {
    uVar3 = uStack10;
    pass1_1030_1d58((u32)(iVar6 + 0xc));
    uVar5 = uVar4 | uVar3;
    if (uVar5 != 0x0) {
      uVar2 = pass1_1030_6fa0(uVar3 & 0xffff | (u32)uVar4 << 0x10);
      pass1_1008_c6ae(_u16_1050_06e0,uVar2,param_4);
    }
    uVar4 = uVar5;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_50e0(mut param_1: u16 ,mut param_2: u32,mut param_3: u16 )

{
  code **ppcVar1;
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
  iVar6 = (int)param_2;
  if (*(i32 *)(iVar6 + 0xc) == 0x0) {
    param_1 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar1 = (code **)((int)(u32)(u32)(iVar6 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar4 = extraout_DX;
  }
  uStack10 = CONCAT22(uVar4,param_1);
  for (uStack14 = 0x0; uStack14 < uStack10; uStack14 += 0x1) {
    uVar8 = uStack10;
    pass1_1030_1d58((u32)(iVar6 + 0xc));
    uVar5 = uVar4 | uVar8;
    if (uVar5 != 0x0) {
      uVar2 = pass1_1030_6fa0(uVar8 & 0xffff | (u32)uVar4 << 0x10);
      BVar3 = pass1_1008_c6ae(_u16_1050_06e0,uVar2,param_3);
      if (BVar3 != 0x0) {
        uVar8 = struct_op_1030_73a8((astruct_419 *)(uVar8 & 0xffff | (u32)uVar4 << 0x10),BVar3,uVar5);
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
  code **ppcVar3;
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
  u8 unaff_CS;
  let mut bVar12: bool;
  let mut uVar13: u32;
  let mut iStack34: i16;
  let mut uStack32: u32;
  u32 *puStack28;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar10 = (param_2 >> 0x10);
  iVar7 = (int)param_2;
  if ((iVar7 + 0x206) == 0x0) {
    if (*(i32 *)(iVar7 + 0xc) == 0x0) {
      param_1 = 0x0;
      uVar11 = 0x0;
    }
    else {
      ppcVar3 = (code **)((int)(u32)(u32)(iVar7 + 0xc) + 0x10);
      (**ppcVar3)();
      uVar11 = extraout_DX;
    }
    uStack6 = CONCAT22(uVar11,param_1);
    for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
      uVar2 = (u32)(iVar7 + 0xc);
      ppcVar3 = (code **)((int)(u32)(u32)(iVar7 + 0xc) + 0x4);
      uVar5 = uStack6;
      (**ppcVar3)(unaff_CS,(char)uVar2,(int)((u32)uVar2 >> 0x10),(int)uStack10,(int)(uStack10 >> 0x10));
      uVar4 = uVar5;
      uVar6 = extraout_DX_00 | uVar4;
      if (uVar6 != 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5 & 0xffff | (u32)extraout_DX_00 << 0x10);
        unaff_CS = 0x30;
        uVar13 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar6,uVar4),uVar4,uVar6);
        uVar6 = (uVar13 >> 0x10);
        iVar8 = ((int)uVar13 + 0x12);
        uVar4 = (int)uVar13 + 0x14;
        uVar5 = (u32)uVar4;
        puStack28 = (u32 *)(uVar13 & 0xffff0000 | (u32)uVar4);
        uStack32 = 0x0;
        if ((iVar8 == 0x4) || (iVar8 == 0x5)) {
          uVar5 = *puStack28;
          uStack32 = uVar5;
        }
        if (uStack32 != 0x0) {
          for (iStack34 = 0x11; iStack34 < 0x25; iStack34 += 0x1) {
            if ((((iVar7 + 0x204) == 0x0) || (iStack34 == 0x23)) || (iStack34 == 0x24)) {
              empty_1038_540a();
              iVar8 = iStack34 * 0x4;
              uVar11 = (uStack32 >> 0x10);
              iVar9 = (int)uStack32;
              puVar1 = (u16 *)(iVar8 + iVar9 + 0x2);
              bVar12 = *puVar1 < uVar6;
              if ((bVar12 || *puVar1 == uVar6) &&
                 ((bVar12 || (puVar1 = (u16 *)(iVar8 + iVar9), *puVar1 < uVar5 || *puVar1 == uVar5)))) {
                pass1_1038_5770(param_2,*(i32 *)(iVar8 + iVar9),iStack34);
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
  u32 *puVar1;
  code **ppcVar2;
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
  astruct_397 *paStack22;
  let mut uStack14: u32;
  let mut uStack10: u32;
  astruct_601 *iVar10;

  iVar4 = -(int)param_2;
  iVar10 = (astruct_601 *)param_1;
  pass1_1038_5694(param_1,CONCAT22(-(param_2 + ((int)param_2 != 0x0)),iVar4),param_3);
  if (param_3 != 0x24) {
    uVar8 = (param_1 >> 0x10);
    if (iVar10->field12_0xc == NULL) {
      iVar4 = 0x0;
      uVar6 = 0x0;
    }
    else {
      ppcVar2 = (code **)((int)*iVar10->field12_0xc + 0x10);
      (**ppcVar2)();
      uVar6 = extraout_DX;
    }
    uStack10 = CONCAT22(uVar6,iVar4);
    for (uStack14 = 0x0; uVar3 = param_2, uStack14 < uStack10; uStack14 += 0x1) {
      puVar1 = iVar10->field12_0xc;
      ppcVar2 = (code **)((int)*iVar10->field12_0xc + 0x4);
      uVar9 = uStack10;
      (**ppcVar2)(unaff_CS,(char)puVar1,(int)((u32)puVar1 >> 0x10),(int)uStack14,(int)(uStack14 >> 0x10));
      uVar5 = uVar9;
      uVar7 = extraout_DX_00 | uVar5;
      if (uVar7 != 0x0) {
        uVar10 = param_3;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar9 & 0xffff | (u32)extraout_DX_00 << 0x10);
        paStack22 = (astruct_397 *)CONCAT22(uVar7,uVar5);
        unaff_CS = 0x1030;
        uVar9 = pass1_1030_7c28(uVar5,uVar7,CONCAT22(uVar7,uVar5),uVar10);
        uVar7 = (uVar9 >> 0x10);
        uVar5 = uVar9;
        if ((uVar7 | uVar5) != 0x0) {
          if (uVar9 < param_2) {
            param_2 -= uVar9;
            uStack26 = 0x0;
            iStack24 = 0x0;
          }
          else {
            uStack26 = uVar5 - param_2;
            iStack24 = (uVar7 - param_2) - (uVar5 < param_2);
            param_2 = 0x0;
            uVar9 = uVar3;
          }
          unaff_CS = 0x1030;
          pass1_1030_7d1c((int)uVar9,param_2,paStack22,uStack26,CONCAT22(param_3,iStack24));
          if (param_2 == 0x0) {
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
  if ((u32)((int)param_1 + 0x1a2 + param_2 * 0x4) < (u32)((int)param_1 + 0x14e + param_2 * 0x4)) {
    return;
  }
  return;
}
pub fn empty_1038_540a(void)

{
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_5464(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u32;
  code **ppcVar2;
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
  u32 *puStack34;
  let mut uStack30: u16;
  let mut uStack28: u16;
  u32 *puStack26;
  let mut uStack22: u32;
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  pass1_1038_56ba(param_2);
  pass1_1038_57c0(param_2);
  uVar8 = (param_2 >> 0x10);
  iVar6 = (int)param_2;
  if (*(i32 *)(iVar6 + 0xc) == 0x0) {
    param_1 = 0x0;
    uVar5 = 0x0;
  }
  else {
    ppcVar2 = (code **)((int)(u32)(u32)(iVar6 + 0xc) + 0x10);
    (**ppcVar2)();
    uVar5 = extraout_DX;
  }
  uStack10 = CONCAT22(uVar5,param_1);
  for (uStack14 = 0x0; uStack14 < uStack10; uStack14 += 0x1) {
    uVar1 = (u32)(iVar6 + 0xc);
    ppcVar2 = (code **)((int)(u32)(u32)(iVar6 + 0xc) + 0x4);
    uVar4 = uStack10;
    (**ppcVar2)(unaff_CS,(char)uVar1,(int)((u32)uVar1 >> 0x10),(int)uStack14,(int)(uStack14 >> 0x10));
    uVar3 = uVar4;
    uVar5 = extraout_DX_02 | uVar3;
    uStack18 = uVar3;
    uStack16 = extraout_DX_02;
    if (uVar5 != 0x0) {
      unaff_CS = 0x1028;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4 & 0xffff | (u32)extraout_DX_02 << 0x10);
      uStack22 = CONCAT22(uVar5,uVar3);
      puStack26 = (u32*)(uVar3 + 0x22);
      if (((uVar3 + 0x24) | puStack26) == 0x0) {
        uStack28 = 0x0;
      }
      else {
        uStack28 = (puStack26 + 0x4);
      }
      for (uStack30 = 0x0; uStack30 < uStack28; uStack30 += 0x1) {
        unaff_CS = 0x1020;
        pass1_1020_bb16(puStack26,(u32 *)CONCAT13(0x10,CONCAT12(0x50,&local_2e)),(u16 *)CONCAT22(0x1050,&local_2a),
                        uStack30);
        if (CONCAT22(uStack44,local_2e) != 0x0) {
          pass1_1038_5694(param_2,CONCAT22(uStack44,local_2e),local_2a);
        }
      }
      uVar9 = ((u32)uStack22 >> 0x10);
      puStack34 = (u32 *)(u32)((int)uStack22 + 0x1e);
      uVar5 = ((int)uStack22 + 0x20);
      uVar3 = uVar5 | puStack34;
      if (uVar3 == 0x0) {
        uVar3 = 0x0;
      }
      else {
        ppcVar2 = (code **)((int)*puStack34 + 0x10);
        (**ppcVar2)(unaff_CS,puStack34,uVar5);
        uVar5 = extraout_DX_00;
      }
      uStack28 = uVar3;
      for (uStack30 = 0x0; uStack30 < uStack28; uStack30 += 0x1) {
        ppcVar2 = (code **)((int)*puStack34 + 0x4);
        uVar3 = uStack28;
        (**ppcVar2)(unaff_CS,(char)puStack34,(int)((u32)puStack34 >> 0x10),uStack30,0x0);
        uVar5 = extraout_DX_01 | uVar3;
        local_2a = uVar3;
        uStack40 = extraout_DX_01;
        if (uVar5 != 0x0) {
          unaff_CS = 0x1028;
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(extraout_DX_01,uVar3));
          iVar7 = (uVar3 + 0xc) * 0x4;
          *(i32 *)(iVar6 + iVar7 + 0x14e) = *(i32 *)(iVar6 + 0x14e + iVar7) + 0x1;
        }
      }
    }
  }
  uVar4 = uStack10;
  pass1_1030_38f2((u32)(iVar6 + 0x1f6),0x3);
  uVar3 = uVar4;
  uStack6._0_2_ = uVar3;
  uStack6 = uVar5;
  pass1_1030_38f2((u32)(iVar6 + 0x1f6),0x4);
  uStack6 = CONCAT22(uStack6 + uVar5 + CARRY2(uStack6,uVar3),uStack6 + uVar3);
  if (uStack6 == 0x0) {
    pass1_1030_38f2((u32)(iVar6 + 0x1f6),0x2);
    uStack6 = CONCAT22(uVar5,uVar3);
  }
  uVar1 = (u32)(iVar6 + 0x1f6);
  uStack6 += *(i32 *)((int)uVar1 + 0x170);
  pass1_1038_5694(param_2,uStack6,0x24);
  return;
}



pub fn pass1_1038_565e(u8 *param_1,mut param_2: u32) -> u32

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u32;
  let mut in_stack_0000ffce: u16;
  u8 local_4 [0x2];

  uVar2 = (param_2 >> 0x10);
  iVar1 = (int)param_2;
  uVar3 = pass1_1030_8e3c((StructD *)CONCAT22(local_4,param_1),CONCAT22(0x1050,local_4),(u32)(iVar1 + 0x4),
                          in_stack_0000ffce);
  pass1_1038_582c(param_2,uVar3);
  return CONCAT22((iVar1 + 0x16),(iVar1 + 0x14));
}
pub fn pass1_1038_5694(mut param_1: u32,i32 param_2,mut param_3: i16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *(i32 *)((int)param_1 + param_3 * 0x4 + 0x26) = *(i32 *)((int)param_1 + 0x26 + param_3 * 0x4) + param_2;
  return;
}
pub fn pass1_1038_56ba(mut param_1: u32)

{
  pass1_1000_4906((StructD *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0x26)),NULL,0x94);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_56d6(mut param_1: u32,mut param_2: i16)

{
  code **ppcVar1;
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

  iVar2 = (int)param_1;
  uVar9 = 0x1000;
  puVar3 = pass1_1000_4906((StructD *)(param_1 & 0xffff0000 | (u32)(iVar2 + 0xba)),NULL,0x94);
  if (param_2 != 0x0) {
    uVar8 = (param_1 >> 0x10);
    if (*(i32 *)(iVar2 + 0xc) == 0x0) {
      puVar3 = NULL;
      uVar6 = 0x0;
    }
    else {
      ppcVar1 = (code **)((int)(u32)(u32)(iVar2 + 0xc) + 0x10);
      (**ppcVar1)();
      uVar6 = extraout_DX;
    }
    uStack6 = CONCAT22(uVar6,puVar3);
    for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
      ppcVar1 = (code **)((int)(u32)(u32)(iVar2 + 0xc) + 0x4);
      uVar5 = uStack6;
      (**ppcVar1)(uVar9,(u32)(iVar2 + 0xc));
      uVar4 = uVar5;
      uVar7 = extraout_DX_00 | uVar4;
      if (uVar7 != 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5 & 0xffff | (u32)extraout_DX_00 << 0x10);
        uVar9 = 0x1030;
        pass1_1030_72d0((astruct_292 *)CONCAT22(uVar7,uVar4));
      }
    }
  }
  return;
}
pub fn pass1_1038_5770(mut param_1: u32,i32 param_2,mut param_3: i16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *(i32 *)((int)param_1 + param_3 * 0x4 + 0xba) = *(i32 *)((int)param_1 + 0xba + param_3 * 0x4) + param_2;
  return;
}
pub fn pass1_1038_5798(mut param_1: u32,i32 param_2,mut param_3: i16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *(i32 *)((int)param_1 + param_3 * 0x4 + 0x14e) = *(i32 *)((int)param_1 + 0x14e + param_3 * 0x4) + param_2;
  return;
}
pub fn pass1_1038_57c0(mut param_1: u32)

{
  pass1_1000_4906((StructD *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0x14e)),NULL,0x54);
  return;
}
pub fn pass1_1038_57dc(mut param_1: u32,i32 param_2,mut param_3: i16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *(i32 *)((int)param_1 + param_3 * 0x4 + 0x1a2) = *(i32 *)((int)param_1 + 0x1a2 + param_3 * 0x4) + param_2;
  return;
}
pub fn pass1_1038_5804(mut param_1: u32,i32 param_2,mut param_3: i16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *(i32 *)((int)param_1 + param_3 * 0x4 + 0x1a2) = *(i32 *)((int)param_1 + 0x1a2 + param_3 * 0x4) - param_2;
  return;
}
pub fn pass1_1038_582c(mut param_1: u32,mut param_2: u32)

{
  u32 *puVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar1 = (u32 *)(iVar4 + 0x14);
  uVar2 = (iVar4 + 0x16);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (u32)(iVar4 + 0x14) = param_2;
  return;
}
pub fn pass1_1038_5860(mut param_1: u32,mut param_2: u16 ,mut param_3: u32,mut param_4: i16)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: i16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uStack14: u32;
  let mut iStack6: i16;
  let mut iStack4: i16;

  if (param_4 == 0x0) {
    uVar5 = (param_1 >> 0x10);
    iVar4 = (int)param_1;
    ppcVar1 = (code **)((int)(u32)(u32)(iVar4 + 0xc) + 0x10);
    uVar2 = param_3;
    (**ppcVar1)();
    uVar2 = uVar2 & 0xffff | (u32)extraout_DX << 0x10;
    for (uStack14 = 0x0; uStack14 < uVar2; uStack14 += 0x1) {
      ppcVar1 = (code **)((int)(u32)(u32)(iVar4 + 0xc) + 0x4);
      uVar3 = uVar2;
      (**ppcVar1)();
      iStack6 = (int)param_3;
      if (((int)uVar3 == iStack6) && (iStack4 = (int)(param_3 >> 0x10), extraout_DX_00 == iStack4)) {
        return;
      }
    }
    ppcVar1 = (code **)((int)(u32)(u32)(iVar4 + 0xc) + 0xc);
    (**ppcVar1)();
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_58e6(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32,mut param_5: u32,mut param_6: i16)

{
  let mut iVar1: i16;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut BVar4: bool;
  u32 *puVar5;
  let mut uVar6: u16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u32;
  let mut local_12: u32;
  let mut iStack14: i16;
  let mut iStack12: i16;
  let mut uStack6: u32;

  for (uStack6 = 0x0; uStack6 < param_3; uStack6 += 0x1) {
    uVar9 = (param_4 >> 0x10);
    iVar7 = (int)param_4;
    if ((*(i32 *)((int)uStack6 * 0x4 + iVar7) != 0x0) &&
       (uVar3 = (u32)((int)uStack6 * 0x4 + iVar7),
       BVar4 = pass1_1008_c6ae(_u16_1050_06e0,((int)uVar3 + 0xc),0x2e), BVar4 != 0x0)) {
      uVar8 = (param_5 >> 0x10);
      iVar1 = ((int)uStack6 * 0x4 + (int)param_5);
      uVar8 = ((int)uStack6 * 0x4 + (int)param_5 + 0x2);
      local_12 = (u32)(iVar1 + 0xc);
      iStack12 = (iVar1 + 0x10);
      iStack14 = iStack12;
      if (iStack12 == param_6) {
        iStack14 = iStack12 + -0x1;
        uVar10 = pass1_1028_bb24(*(astruct_15 **)((int)uStack6 * 0x4 + iVar7));
        uVar6 = (uVar10 >> 0x10);
        puVar5 = &local_12;
        pass1_1030_627e(puVar5,uVar6,_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar5),
                        uVar10 & 0xffff | (u32)uVar6 << 0x10);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(uVar6,puVar5));
        if ((uVar6 | puVar5) != 0x0) {
          uVar10 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar6,puVar5),puVar5,uVar6 | puVar5);
          uVar6 = ((int)uVar10 + 0x1a);
          if (((uVar6 & 0x2) != 0x0) && ((uVar6 & 0x1) != 0x0)) {
            uVar3 = (u32)((int)uStack6 * 0x4 + iVar7);
            ((int)uVar3 + 0x1a) = 0x3;
            ppcVar2 = (code **)((int)(u32)(u32)((int)uStack6 * 0x4 + iVar7) + 0x28);
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
  code **ppcVar1;
  let mut uVar2: u32;
  let mut BVar3: bool;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uStack6: u32;

  for (uStack6 = 0x0; uStack6 < param_3; uStack6 += 0x1) {
    uVar5 = (param_4 >> 0x10);
    iVar4 = (int)param_4;
    if ((*(i32 *)((int)uStack6 * 0x4 + iVar4) != 0x0) &&
       (uVar2 = (u32)((int)uStack6 * 0x4 + iVar4),
       BVar3 = pass1_1008_c6ae(_u16_1050_06e0,((int)uVar2 + 0xc),0x2f), BVar3 != 0x0)) {
      uVar2 = (u32)((int)uStack6 * 0x4 + iVar4);
      ((int)uVar2 + 0x1a) = 0x3;
      ppcVar1 = (code **)((int)(u32)(u32)((int)uStack6 * 0x4 + iVar4) + 0x28);
      (**ppcVar1)();
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_5a96(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut BVar3: bool;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uStack6: u32;

  for (uStack6 = 0x0; uStack6 < param_3; uStack6 += 0x1) {
    uVar5 = (param_4 >> 0x10);
    iVar4 = (int)param_4;
    if ((*(i32 *)((int)uStack6 * 0x4 + iVar4) != 0x0) &&
       (uVar2 = (u32)((int)uStack6 * 0x4 + iVar4),
       BVar3 = pass1_1008_c6ae(_u16_1050_06e0,((int)uVar2 + 0xc),0x2c), BVar3 != 0x0)) {
      ppcVar1 = (code **)((int)(u32)(u32)((int)uStack6 * 0x4 + iVar4) + 0x54);
      (**ppcVar1)();
      if (BVar3 != 0x0) {
        uVar2 = (u32)(iVar4 + (int)uStack6 * 0x4);
        ((int)uVar2 + 0x1a) = 0x3;
        ppcVar1 = (code **)((int)(u32)(u32)((int)uStack6 * 0x4 + iVar4) + 0x28);
        (**ppcVar1)();
        uVar2 = (u32)(iVar4 + (int)uStack6 * 0x4);
        ((int)uVar2 + 0x1a) = 0x2;
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_5b3c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut BVar4: bool;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut uStack6: u32;

  for (uStack6 = 0x0; uStack6 < param_3; uStack6 += 0x1) {
    uVar6 = (param_4 >> 0x10);
    iVar5 = (int)param_4;
    if (((*(i32 *)((int)uStack6 * 0x4 + iVar5) != 0x0) &&
        (uVar2 = (u32)((int)uStack6 * 0x4 + iVar5),
        BVar4 = pass1_1008_c6ae(_u16_1050_06e0,((int)uVar2 + 0xc),0x2d), BVar4 != 0x0)) &&
       (ppcVar1 = (code **)((int)(u32)(u32)((int)uStack6 * 0x4 + iVar5) + 0x50), (**ppcVar1)(),
       BVar4 != 0x0)) {
      uVar2 = (u32)((int)uStack6 * 0x4 + iVar5);
      uVar3 = (u32)((int)uStack6 * 0x4 + iVar5);
      ((int)uVar3 + 0x1a) = ((int)uVar2 + 0x1a) | 0x1;
      ppcVar1 = (code **)((int)(u32)(u32)((int)uStack6 * 0x4 + iVar5) + 0x28);
      (**ppcVar1)();
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1038_5be8(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 ,mut param_5: i16,param_6: *mut u16)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut BVar4: bool;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut iStack14: i16;
  astruct_419 *paStack10;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_6,*(i32 *)((int)param_3 + 0x8));
  uVar5 = param_2 | param_1;
  if (uVar5 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    paStack10 = (astruct_419 *)CONCAT22(uVar5,param_1);
    iVar3 = ((int)param_6 + 0x4);
    iStack14 = 0x7a;
    if (0x0 < iVar3) {
      iVar3 = param_5 + -0x7b;
      if (iVar3 == 0x0) {
        param_5 = 0x7e;
      }
      else {
        iVar3 = param_5 + -0x7c;
        if (iVar3 == 0x0) {
          param_5 = 0x7d;
        }
      }
      iStack14 = 0x7f;
    }
    uVar6 = struct_op_1030_73a8(paStack10,iVar3,uVar5);
    uVar2 = (uVar6 >> 0x10);
    iVar3 = (int)uVar6;
    if (((((iVar3 + 0x1a) & param_4) == 0x0) &&
        (((iVar1 = (iVar3 + 0xc), iVar1 == iStack14 || (iVar1 == param_5)) ||
         (BVar4 = pass1_1008_c6ae(_u16_1050_06e0,iVar1,0x2b), BVar4 != 0x0)))) && ((iVar3 + 0x12) != 0x7)) {
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

  puVar6 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,&local_a));
  uVar4 = ((u32)puVar6 >> 0x10);
  do {
    iStack4 = 0x0;
    for (uStack14 = 0x0; uStack14 < param_2; uStack14 += 0x1) {
      uVar5 = (param_4 >> 0x10);
      if (*(i32 *)((int)uStack14 * 0x4 + (int)param_4) != 0x0) {
        uVar1 = (u32)((int)uStack14 * 0x4 + (int)param_4);
        pass1_1008_3f62((u16 *)CONCAT22(0x1050,&local_a),(u16 *)(uVar1 & 0xffff0000 | (u32)((int)uVar1 + 0xc)));
        pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_a),(u16 *)CONCAT22(0x1050,&local_14),
                        (u16 *)CONCAT22(0x1050,&local_12),(u16 *)CONCAT22(0x1050,&local_10));
        if (local_14 == param_5) {
          uVar5 = (param_3 >> 0x10);
          if ((*(i32 *)((int)uStack14 * 0x4 + (int)param_3) != 0x0) &&
             (uVar2 = (u32)((int)uStack14 * 0x4 + (int)param_3),
             (((int)uVar2 + 0x1a) & param_6) != 0x0)) {
            iStack8 = local_12 + -0x1;
            uVar3 = pass1_1038_5be8(&local_a,uVar4,param_1,param_6,0x7b,(u16 *)CONCAT22(0x1050,&local_a));
            if (uVar3 != 0x0) {
              iStack4 = 0x1;
            }
            iStack8 = local_12 + 0x1;
            uVar3 = pass1_1038_5be8(&local_a,uVar4,param_1,param_6,0x7b,(u16 *)CONCAT22(0x1050,&local_a));
            if (uVar3 != 0x0) {
              iStack4 = 0x1;
            }
            iStack8 = local_12;
            local_a = local_10 + -0x1;
            uVar3 = pass1_1038_5be8(&local_a,uVar4,param_1,param_6,0x7c,(u16 *)CONCAT22(0x1050,&local_a));
            if (uVar3 != 0x0) {
              iStack4 = 0x1;
            }
            local_a = local_10 + 0x1;
            uVar3 = pass1_1038_5be8(&local_a,uVar4,param_1,param_6,0x7c,(u16 *)CONCAT22(0x1050,&local_a));
            if (uVar3 != 0x0) {
              iStack4 = 0x1;
            }
          }
        }
      }
    }
  } while (iStack4 != 0x0);
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1038_5e16(mut param_1: i16,mut param_2: u32,mut param_3: u32)

{
  let mut BVar1: bool;
  u32 *puVar2;
  let mut iVar3: i16;
  let mut uVar4: u16;
  HFILE16 in_stack_0000ffb6;
  u32 local_14 [0x2];
  let mut local_c: u32;
  u32 *puStack6;

  pass1_1030_16d6((astruct_731 *)param_2,param_3);
  if (param_1 != 0x0) {
    uVar4 = (param_2 >> 0x10);
    iVar3 = (int)param_2;
    puVar2 = (u32*)(iVar3 + 0xc);
    puStack6 = puVar2;
    pass1_1008_7898(puVar2,param_3,puVar2);
    if ((int)puVar2 != 0x0) {
      local_14[0] = (u32)(iVar3 + 0x10);
      BVar1 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_14),(char *)0x4,in_stack_0000ffb6);
      if (BVar1 != 0x0) {
        local_c._0_2_ = (iVar3 + 0x18);
        BVar1 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffb6);
        if (BVar1 != 0x0) {
          local_c._0_2_ = (iVar3 + 0x1a);
          BVar1 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffb6);
          if (BVar1 != 0x0) {
            local_c = CONCAT22(local_c,(iVar3 + 0x1c));
            BVar1 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffb6);
            if (BVar1 != 0x0) {
              local_c = (u32)(iVar3 + 0x1e);
              BVar1 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x4,in_stack_0000ffb6);
              if (BVar1 != 0x0) {
                local_c = local_c & 0xffff0000 | (u32)(iVar3 + 0x22);
                BVar1 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffb6);
                if (BVar1 != 0x0) {
                  local_c = local_c & 0xffff0000 | (u32)(iVar3 + 0x24);
                  BVar1 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffb6)
                  ;
                  if (BVar1 != 0x0) {
                    BVar1 = write_to_file_1008_7e1c
                                      ((u8 *)param_3,param_2 & 0xffff0000 | (u32)(iVar3 + 0x26),(char *)0x94,
                                       in_stack_0000ffb6);
                    if (BVar1 != 0x0) {
                      BVar1 = write_to_file_1008_7e1c
                                        ((u8 *)param_3,param_2 & 0xffff0000 | (u32)(iVar3 + 0x14e),(char *)0x54,
                                         in_stack_0000ffb6);
                      if (BVar1 != 0x0) {
                        BVar1 = write_to_file_1008_7e1c
                                          ((u8 *)param_3,param_2 & 0xffff0000 | (u32)(iVar3 + 0x1a2),(char *)0x54,
                                           in_stack_0000ffb6);
                        if (BVar1 != 0x0) {
                          write_to_file_1030_32e4((u32)(iVar3 + 0x1f6),param_3);
                          BVar1 = pass1_1008_7c2a(param_3,*(char **)(iVar3 + 0x1fa));
                          if (BVar1 != 0x0) {
                            local_c = local_c & 0xffff0000 | (u32)(iVar3 + 0x1fe);
                            BVar1 = write_to_file_1008_7e1c
                                              ((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffb6);
                            if (BVar1 != 0x0) {
                              local_c = (u32)(iVar3 + 0x200);
                              BVar1 = write_to_file_1008_7e1c
                                                ((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x4,in_stack_0000ffb6);
                              if (BVar1 != 0x0) {
                                local_c = local_c & 0xffff0000 | (u32)(iVar3 + 0x204);
                                BVar1 = write_to_file_1008_7e1c
                                                  ((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,in_stack_0000ffb6
                                                  );
                                if (BVar1 != 0x0) {
                                  local_c = local_c & 0xffff0000 | (u32)(iVar3 + 0x206);
                                  BVar1 = write_to_file_1008_7e1c
                                                    ((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,
                                                     in_stack_0000ffb6);
                                  if (BVar1 != 0x0) {
                                    local_c = local_c & 0xffff0000 | (u32)(iVar3 + 0x208);
                                    BVar1 = write_to_file_1008_7e1c
                                                      ((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,
                                                       in_stack_0000ffb6);
                                    if (BVar1 != 0x0) {
                                      local_c = local_c & 0xffff0000 | (u32)(iVar3 + 0x20a);
                                      BVar1 = write_to_file_1008_7e1c
                                                        ((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,
                                                         in_stack_0000ffb6);
                                      if (BVar1 != 0x0) {
                                        local_c = local_c & 0xffff0000 | (u32)(iVar3 + 0x20c);
                                        BVar1 = write_to_file_1008_7e1c
                                                          ((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,
                                                           in_stack_0000ffb6);
                                        if (BVar1 != 0x0) {
                                          local_c = local_c & 0xffff0000 | (u32)(iVar3 + 0x20e);
                                          BVar1 = write_to_file_1008_7e1c
                                                            ((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,
                                                             in_stack_0000ffb6);
                                          if (BVar1 != 0x0) {
                                            local_c = local_c & 0xffff0000 | (u32)(iVar3 + 0x214);
                                            BVar1 = write_to_file_1008_7e1c
                                                              ((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x2,
                                                               in_stack_0000ffb6);
                                            if (BVar1 != 0x0) {
                                              local_c = (u32)(iVar3 + 0x216);
                                              BVar1 = write_to_file_1008_7e1c
                                                                ((u8 *)param_3,CONCAT22(0x1050,&local_c),(char *)0x4,
                                                                 in_stack_0000ffb6);
                                              if (BVar1 != 0x0) {
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
pub fn file_1038_6118(mut param_1: i16,param_2: *mut astruct_57,param_3: *mut astruct_373,HFILE16 *param_4)

{
  u32 *puVar1;
  let mut BVar2: bool;
  let mut BVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u16;
  u8 *puVar6;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar8;
  astruct_373 *iVar9;
  astruct_373 *uVar10;
  let mut uVar9: u16;
  let mut uVar11: u16;
  u8 *puStack1046;
  let mut uStack1042: u16;
  u8 local_408 [0x400];
  let mut local_8: u16;
  let mut local_6: u32;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  file_1030_1730(param_3,param_4);
  if (param_1 == 0x0) {
    return;
  }
  local_6 = 0x0;
  puVar1 = &local_6;
  file_1008_7548(param_4,(i32 *)CONCAT22(0x1050,puVar1),paVar8);
  if (puVar1 != NULL) {
    uVar10 = (astruct_373 *)((u32)param_3 >> 0x10);
    iVar9 = (astruct_373 *)param_3;
    (u32)&iVar9->field_0xc = local_6;
    BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9->field13_0x10)),0x4);
    if (((((BVar2 != 0x0) &&
          (BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9->field_0x18)),0x2),
          BVar2 != 0x0)) &&
         (BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9->field19_0x1a)),0x2),
         BVar2 != 0x0)) &&
        ((BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_8),0x2), BVar2 != 0x0 &&
         (BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9[0x1].field_0x2)),0x4),
         BVar2 != 0x0)))) &&
       (BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 |
                                                  ZEXT24((u8 *)((int)&iVar9[0x1].field4_0x4 + 0x2))),0x2),
       BVar2 != 0x0)) {
      (iVar9 + 0x1) = local_8;
      BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9[0x1].field_0x8)),0x2);
      if ((BVar2 != 0x0) &&
         (BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9[0x1].field_0xa)),0x94),
         BVar2 != 0x0)) {
        if ((int)u16_1050_0312 < 0x2) {
          uVar9 = 0x54;
          uVar11 = 0x0;
          mem_op_1000_179c(0x54,paVar8);
          uVar7 = SUB42(paVar8,0x0);
          puStack1046 = (u8 *)CONCAT22(uVar7,BVar2);
          BVar3 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(uVar7,BVar2),CONCAT22(uVar11,uVar9));
          if (BVar3 == 0x0) {//
LAB_1038_626a:
            u16_1050_0310 = 0x6d2;
            fn_ptr_1000_17ce((char *)puStack1046);
            return;
          }
          uStack1042 = 0x0;
          do {
            uVar4 = switch_1008_72bc(param_4,uStack1042);
            uVar9 = (uStack1042 * 0x4 + BVar2 + 0x2);
            (&iVar9[0xb].field19_0x1a)[uVar4 * 0x2] = *(astruct_430 **)(uStack1042 * 0x4 + BVar2);
            (&iVar9[0xc].field_0x0 + uVar4 * 0x4) = uVar9;
            uStack1042 += 0x1;
          } while ((int)uStack1042 < 0x15);
          BVar3 = read_file_1008_7dee(param_4,puStack1046,0x54);
          if (BVar3 == 0x0) goto LAB_1038_626a;
          uStack1042 = 0x0;
          do {
            uVar5 = switch_1008_72bc(param_4,uStack1042);
            uVar4 = (uStack1042 * 0x4 + BVar2 + 0x2);
            (&iVar9[0xe].field19_0x1a)[uVar5 * 0x2] = *(astruct_430 **)(uStack1042 * 0x4 + BVar2);
            (&iVar9[0xf].field_0x0 + uVar5 * 0x4) = uVar4;
            uStack1042 += 0x1;
          } while ((int)uStack1042 < 0x15);
          fn_ptr_1000_17ce((char *)puStack1046);
        }
        else {
          BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9[0xb].field19_0x1a)),
                                      0x54);
          uVar4 = paVar8;
          if (BVar2 == 0x0) {
            u16_1050_0310 = 0x6d2;
            return;
          }
          BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9[0xe].field19_0x1a)),
                                      0x54);
          if (BVar2 == 0x0) {
            u16_1050_0310 = 0x6d2;
            return;
          }
        }
    // WARNING: Load size is inaccurate
        read_file_1030_33f0(iVar9[0x11].field19_0x1a,param_4);
        puVar6 = local_408;
        read_file_1008_7c6e((HFILE16)param_4,((u32)param_4 >> 0x10),(char *)CONCAT22(0x1050,puVar6));
        if (puVar6 != NULL) {
          uVar5 = str_op_1008_60e8(uVar4,(char *)CONCAT22(0x1050,local_408));
          &iVar9[0x12].field_0x2 = uVar5;
          &iVar9[0x12].field4_0x4 = uVar4;
          BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 |
                                                    ZEXT24((u8 *)((int)&iVar9[0x12].field4_0x4 + 0x2))),0x2);
          if (((((BVar2 != 0x0) &&
                (BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 |
                                                           (u32)CONCAT11((char)((u32)param_3 >> 0x8) + '\x02',
                                                                           (char)param_3)),0x4), BVar2 != 0x0)) &&
               (BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9[0x12].field_0xc))
                                            ,0x2), BVar2 != 0x0)) &&
              (((BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(&iVar9[0x12].field_0xe)
                                                           ),0x2), BVar2 != 0x0 &&
                (BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 |
                                                           ZEXT24(&iVar9[0x12].field13_0x10)),0x2), BVar2 != 0x0)) &&
               ((BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 |
                                                           ZEXT24((u8 *)((int)&iVar9[0x12].field13_0x10 + 0x2))),
                                             0x2), BVar2 != 0x0 &&
                ((BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 |
                                                            ZEXT24(&iVar9[0x12].field_0x14)),0x2), BVar2 != 0x0 &&
                 (BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 |
                                                            ZEXT24(&iVar9[0x12].field16_0x16)),0x2), BVar2 != 0x0)))))))
              ) && (((int)u16_1050_0312 < 0x2 ||
                    ((BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 | ZEXT24(iVar9 + 0x13)),0x2
                                                 ), BVar2 != 0x0 &&
                     (BVar2 = read_file_1008_7dee(param_4,(u8 *)((u32)param_3 & 0xffff0000 |
                                                                ZEXT24(&iVar9[0x13].field_0x2)),0x4), BVar2 != 0x0))))))
          {
            return;
          }
          u16_1050_0310 = 0x6d0;
          return;
        }
      }
    }
  }
  u16_1050_0310 = 0x6d2;
  return;
}



StructD * pass1_1038_64de(StructD *param_1,u8 param_2)

{
  pass1_1038_33f8(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}
pub fn struct_1038_6520(param_1: *mut astruct_308)

{
  astruct_308 *pstruct308_1;
  astruct_308 *pstruct308_2;

  pstruct308_2 = (astruct_308 *)((u32)param_1 >> 0x10);
  pstruct308_1 = (astruct_308 *)param_1;
  param_1->field0_0x0 = 0x389a;
  pstruct308_1->field1_0x2 = 0x1008;
  pstruct308_1->field2_0x4 = 0x0;
  pstruct308_1->field3_0x8 = 0x0;
  pstruct308_1->field4_0xc = 0x0;
  pstruct308_1->field5_0xe = 0x0;
  pstruct308_1->field6_0x12 = 0x0;
  pstruct308_1->field7_0x14 = 0x0;
  pstruct308_1->field8_0x16 = 0x0;
  pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&pstruct308_1->field9_0x1a)));
  pstruct308_1->field12_0x20 = 0x0;
  pstruct308_1->field13_0x24 = 0x0;
  pstruct308_1->field14_0x26 = 0x0;
  pstruct308_1->field15_0x28 = 0x0;
  param_1->field0_0x0 = 0x78de;
    // just 0x1038
  pstruct308_1->field1_0x2 = &u16_1050_1038;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_6590(param_1: *mut astruct_410,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut iVar4: i16;
  astruct_410 *iVar3;
  let mut uVar5: u16;
  let mut puVar6: *mut u16;
  let mut uVar7: u32;

  uVar5 = ((u32)param_1 >> 0x10);
  iVar3 = (astruct_410 *)param_1;
  param_1 = 0x389a;
  iVar3->field2_0x2 = 0x1008;
  (u32)&iVar3->field3_0x4 = 0x0;
  iVar3->field5_0x8 = param_6;
  iVar3->field6_0xc = param_4;
  iVar3->field7_0xe = 0x0;
  iVar3->field8_0x12 = 0x0;
  iVar3->field9_0x14 = 0x0;
  iVar3->field10_0x16 = param_2;
  iVar3->field11_0x18 = param_3;
  puVar6 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x1a)));
  uVar2 = ((u32)puVar6 >> 0x10);
  (u32)&iVar3->field18_0x20 = 0x0;
  iVar3->field20_0x24 = 0x0;
  iVar3->field21_0x26 = param_5;
  iVar3->field22_0x28 = 0x0;
  param_1 = 0x78de;
  iVar3->field2_0x2 = (int)&u16_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_6);
  uVar7 = pass1_1030_6d4e(param_5,uVar2,CONCAT22(uVar2,param_5));
  iVar4 = (int)(uVar7 >> 0x10);
  iVar3->field3_0x4 = (astruct_411 *)uVar7;
  iVar3->field4_0x6 = iVar4;
  puVar1 = (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x1a));
  pass1_1008_3f62(puVar1,(u16 *)CONCAT22(uVar2,param_5 + 0xc));
    // WARNING: Load size is inaccurate
  uVar2 = puVar1;
  pass1_1010_8fba(uVar2,iVar3->field3_0x4);
  iVar3->field18_0x20 = uVar2;
  iVar3->field19_0x22 = iVar4;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_666e(param_1: *mut astruct_420,i32 *param_2,mut param_3: u16 ,mut param_4: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  astruct_420 *iVar3;
  astruct_420 *uVar4;
  let mut puVar4: *mut u16;
  let mut uVar5: u32;

  uVar4 = (astruct_420 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_420 *)param_1;
  param_1 = 0x389a;
  iVar3->field2_0x2 = 0x1008;
  iVar3->field3_0x4 = NULL;
  iVar3->field4_0x8 = param_4;
  iVar3->field5_0xc = 0x0;
  iVar3->field6_0xe = param_2;
  iVar3->field7_0x12 = 0x0;
  iVar3->field8_0x14 = 0x0;
  iVar3->field10_0x18 = 0x0;
  iVar3->field9_0x16 = 0x0;
  puVar4 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field11_0x1a)));
  uVar2 = ((u32)puVar4 >> 0x10);
  (u32)&iVar3->field16_0x20 = 0x0;
  iVar3->field18_0x24 = 0x0;
  iVar3->field19_0x26 = param_3;
  iVar3->field20_0x28 = 0x0;
  param_1 = 0x78de;
  iVar3->field2_0x2 = (int)&u16_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4);
  uVar5 = pass1_1030_6d4e(param_3,uVar2,CONCAT22(uVar2,param_3));
  uVar3 = (uVar5 >> 0x10);
  &iVar3->field3_0x4 = (int)uVar5;
  ((int)&iVar3->field3_0x4 + 0x2) = uVar3;
  puVar1 = (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field11_0x1a));
  pass1_1008_3f62(puVar1,(u16 *)CONCAT22(uVar2,param_3 + 0xc));
  uVar2 = puVar1;
  pass1_1010_8fba(uVar2,iVar3->field3_0x4);
  iVar3->field16_0x20 = uVar2;
  iVar3->field17_0x22 = uVar3;
  pass1_1020_ba94(param_2);
  iVar3->field9_0x16 = uVar2;
  iVar3->field10_0x18 = uVar3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_675c(param_1: *mut astruct_414,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  astruct_414 *iVar3;
  astruct_414 *uVar4;
  let mut puVar4: *mut u16;
  let mut uVar5: u32;

  uVar4 = (astruct_414 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_414 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar3->field1_0x2 = 0x1008;
  iVar3->field2_0x4 = NULL;
  iVar3->field3_0x8 = param_5;
  iVar3->field4_0xc = 0x0;
  iVar3->field5_0xe = 0x0;
  iVar3->field6_0x12 = param_3;
  iVar3->field7_0x14 = 0x0;
  iVar3->field8_0x16 = param_2;
  puVar4 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field9_0x1a)));
  uVar2 = ((u32)puVar4 >> 0x10);
  (u32)&iVar3->field14_0x20 = 0x0;
  iVar3->field16_0x24 = 0x0;
  iVar3->field17_0x26 = param_4;
  iVar3->field18_0x28 = 0x0;
  param_1->field0_0x0 = 0x78de;
  iVar3->field1_0x2 = (int)&u16_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
  uVar5 = pass1_1030_6d4e(param_4,uVar2,CONCAT22(uVar2,param_4));
  uVar3 = (uVar5 >> 0x10);
  &iVar3->field2_0x4 = (int)uVar5;
  ((int)&iVar3->field2_0x4 + 0x2) = uVar3;
  puVar1 = (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field9_0x1a));
  pass1_1008_3f62(puVar1,(u16 *)CONCAT22(uVar2,param_4 + 0xc));
  uVar2 = puVar1;
  pass1_1010_8fba(uVar2,iVar3->field2_0x4);
  iVar3->field14_0x20 = uVar2;
  iVar3->field15_0x22 = uVar3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_6838(param_1: *mut astruct_415,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  astruct_415 *iVar3;
  astruct_415 *uVar4;
  let mut puVar4: *mut u16;
  let mut uVar5: u32;

  uVar4 = (astruct_415 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_415 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar3->field1_0x2 = 0x1008;
  iVar3->field2_0x4 = NULL;
  iVar3->field3_0x8 = param_5;
  iVar3->field4_0xc = 0x0;
  iVar3->field5_0xe = 0x0;
  iVar3->field6_0x12 = 0x0;
  iVar3->field7_0x14 = param_3;
  iVar3->field8_0x16 = param_2;
  puVar4 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field9_0x1a)));
  uVar2 = ((u32)puVar4 >> 0x10);
  (u32)&iVar3->field14_0x20 = 0x0;
  iVar3->field16_0x24 = 0x0;
  iVar3->field17_0x26 = param_4;
  iVar3->field18_0x28 = 0x0;
  param_1->field0_0x0 = 0x78de;
    // 0x1038
  iVar3->field1_0x2 = (int)&u16_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
  uVar5 = pass1_1030_6d4e(param_4,uVar2,CONCAT22(uVar2,param_4));
  uVar3 = (uVar5 >> 0x10);
  &iVar3->field2_0x4 = (int)uVar5;
  ((int)&iVar3->field2_0x4 + 0x2) = uVar3;
  puVar1 = (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field9_0x1a));
  pass1_1008_3f62(puVar1,(u16 *)CONCAT22(uVar2,param_4 + 0xc));
  uVar2 = puVar1;
  pass1_1010_8fba(uVar2,iVar3->field2_0x4);
  iVar3->field14_0x20 = uVar2;
  iVar3->field15_0x22 = uVar3;
  return;
}
pub fn pass1_1038_6912(param_1: *mut u16)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  code **ppcVar3;
  u32 *puVar4;
  let mut iVar5: i16;
  let mut uVar6: u16;
  char *pcStack10;

  uVar6 = ((u32)param_1 >> 0x10);
  iVar5 = (int)param_1;
  *param_1 = 0x78de;
  (iVar5 + 0x2) = (int)&u16_1050_1038;
  uVar1 = (iVar5 + 0x6);
  puVar4 = (u32 *)(u32)(iVar5 + 0x4);
  if ((uVar1 | puVar4) != 0x0) {
    ppcVar3 = (code **)*puVar4;
    (**ppcVar3)();
  }
  uVar1 = (iVar5 + 0xe);
  uVar2 = (iVar5 + 0x10);
  pcStack10 = (char *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    fn_ptr_1020_ba7e((u32 *)CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack10);
  }
  *param_1 = 0x389a;
  (iVar5 + 0x2) = 0x1008;
  return;
}
pub fn pass1_1038_6984(mut param_1: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  if ((iVar1 + 0xc) != 0x0) {
    pass1_1020_c3ae();
    return;
  }
  if (*(i32 *)(iVar1 + 0xe) != 0x0) {
    pass1_1020_ba94(*(i32 **)(iVar1 + 0xe));
    return;
  }
  if ((iVar1 + 0x12) == 0x0) {
    if ((iVar1 + 0x14) == 0x0) {
      return;
    }
    pass1_1020_c42e((iVar1 + 0x14));
  }
  else {
    switch_1020_c3b4((iVar1 + 0x12));
  }
  return;
}
pub fn pass1_1038_69fe(mut param_1: u32)

{
  ((int)param_1 + 0x28) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_6a0e(param_1: *mut astruct_419,param_2: *mut astruct_615)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  astruct_615 *uVar7;
  let mut uVar8: u16;
  let mut puVar9: *mut u16;
  let mut uVar10: u32;
  let mut uStack22: u32;
  u8 local_10 [0x4];
  u8 local_c [0x6];
  astruct_419 *paStack6;

  uVar8 = ((u32)param_2 >> 0x10);
  uVar7 = (astruct_615 *)param_2;
  if ((uVar7 + 0x1) == 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)&uVar7->field_0x20);
    paStack6 = (astruct_419 *)CONCAT22((int)param_1,(int)((u32)param_1 >> 0x10));
    piVar1 = (i16 *)&uVar7->field_0x24;
    *piVar1 = *piVar1 + 0x3c;
    puVar9 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_c));
    uVar5 = ((u32)puVar9 >> 0x10);
    while( true ) {
      uVar2 = pass1_1038_6d24(param_2,(u32 *)CONCAT22(0x1050,local_10),(u16 *)CONCAT22(0x1050,local_c),
                              (int)paStack6,((u32)paStack6 >> 0x10));
      if (uVar2 == 0x0) {
        pass1_1010_8fba(0x0,*(astruct_411 **)&uVar7->field_0x4);
        uStack22 = CONCAT22(uVar5,uVar2);
        uVar6 = uVar5 | uVar2;
        if (uVar6 == 0x0) {
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)&uVar7->field_0x20);
          pass1_1038_7356(param_2,(astruct_419 *)CONCAT22(uVar6,uVar2));
          return;
        }
        uVar10 = struct_op_1030_73a8(paStack6,uVar2,uVar6);
        BVar3 = pass1_1008_c6ae(_u16_1050_06e0,((int)uVar10 + 0xc),0x40);
        if (BVar3 != 0x0) {
          (uVar7 + 0x1) = 0x1;
          (u32)&uVar7->field_0x20 = uStack22;
          return;
        }
        (u32)&uVar7->field_0x20 = uStack22;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(uVar5,&uVar7->field_0x20));
        paStack6 = (astruct_419 *)(uStack22 & 0xffff | (u32)uVar5 << 0x10);
      }
      uVar4 = pass1_1038_6e1a(uVar7,uVar8,(i32 *)CONCAT22(0x1050,local_10));
      if (&uVar7->field_0x24 < (int)uVar4) break;
      piVar1 = (i16 *)&uVar7->field_0x24;
      *piVar1 = *piVar1 - uVar4;
      pass1_1008_3f62((u16 *)((u32)param_2 & 0xffff0000 | ZEXT24(&uVar7->field_0x1a)),
                      (u16 *)CONCAT22(0x1050,local_c));
    }
  }
  return;
}



u16 pass1_1038_6b3c(mut param_1: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (((((iVar1 + 0xc) == 0x0) && ((iVar1 + 0x12) == 0x0)) && ((iVar1 + 0x14) == 0x0)) &&
     ((*(i32 *)(iVar1 + 0xe) == 0x0 && (*(i32 *)(iVar1 + 0x16) != 0x0)))) {
    (u32)(iVar1 + 0x16) = 0x0;
  }
  if (*(i32 *)(iVar1 + 0x16) == 0x0) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_6b88(u8 *param_1,mut param_2: u16 ,mut param_3: u16 ,param_4: *mut u16,u32 *param_5)

{
  u32 *puVar1;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  u32 *puVar3;
  let mut in_stack_0000fe96: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffee: u16;

  puVar3 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000ffee,0x2f),in_stack_0000fe96,in_stack_0000ffba,
                           in_stack_0000ffc0,in_stack_0000ffc4);
  uVar2 = ((u32)puVar3 >> 0x10);
  puVar1 = (u32 *)&stack0xffee;
  pass1_1030_64ce(puVar1,uVar2,_PTR_LOOP_1050_5740,param_4,*(i32 *)((int)puVar3 + 0x20),
                  (u32 *)CONCAT22(0x1050,puVar1));
  *param_5 = *puVar1;
  return;
}
pub fn pass1_1038_6bd4(u8 *param_1,param_2: *mut astruct_615,param_3: *mut u16,u32 *param_4,mut param_5: i16)

{
  let mut uStack4: u16;

  pass1_1008_3f62(param_3,(u16 *)((u32)param_2 & 0xffff0000 | (u32)(param_2 + 0x1a)));
  if (param_5 < 0x0) {
    uStack4 = *param_3 - 0x1;
  }
  else {
    uStack4 = *param_3 + 0x1;
  }
  *param_3 = uStack4;
  pass1_1038_6b88(param_1,param_2,((u32)param_2 >> 0x10),param_3,param_4);
  return;
}
pub fn pass1_1038_6c1c(u8 *param_1,param_2: *mut astruct_615,param_3: *mut u16,u32 *param_4,mut param_5: i16)

{
  let mut uVar1: u16;
  let mut iStack4: i16;

  pass1_1008_3f62(param_3,(u16 *)((u32)param_2 & 0xffff0000 | (u32)(param_2 + 0x1a)));
  uVar1 = ((u32)param_3 >> 0x10);
  iStack4 = ((int)param_3 + 0x2);
  if (param_5 < 0x0) {
    iStack4 += -0x1;
  }
  else {
    iStack4 += 0x1;
  }
  ((int)param_3 + 0x2) = iStack4;
  pass1_1038_6b88(param_1,param_2,((u32)param_2 >> 0x10),param_3,param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_6c68(u8 *param_1,param_2: *mut astruct_615,param_3: *mut u16,u32 *param_4,mut param_5: i16)

{
  let mut iVar1: i16;
  u8 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut puVar5: *mut u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar8;
  let mut uVar9: u16;
  u32 *puVar10;
  let mut uVar11: u32;
  let mut in_stack_0000fe80: u16;
  let mut in_stack_0000ffa4: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffd8: u16;
  let mut iStack30: i16;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar3 = param_2;
  pass1_1008_3f62(param_3,(u16 *)((u32)param_2 & 0xffff0000 | (u32)(uVar3 + 0x1a)));
  puVar10 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffd8,0x2f),in_stack_0000fe80,
                            in_stack_0000ffa4,in_stack_0000ffaa,in_stack_0000ffae);
  uVar6 = ((u32)puVar10 >> 0x10);
  puVar5 = (u16 *)((u32)param_2 & 0xffff0000 | (u32)(uVar3 + 0x1a));
  pass1_1030_627e(uVar3 + 0x1a,uVar6,_PTR_LOOP_1050_5740,puVar5,*(i32 *)((int)puVar10 + 0x20));
  uVar4 = puVar5;
  uVar7 = uVar6 | uVar4;
  if (uVar7 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)puVar5 & 0xffff | (u32)uVar6 << 0x10);
    uVar11 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar7,uVar4),uVar4,uVar7);
    puVar2 = (u8 *)(uVar11 >> 0x10);
    iVar1 = ((int)uVar11 + 0xc);
    if ((iVar1 == 0x47) || (iVar1 == 0x6a)) {
      uVar9 = ((u32)param_2 >> 0x10);
      iStack30 = (uVar3 + 0x1e);
      if (param_5 < 0x0) {
        iStack30 += -0x1;
      }
      else {
        iStack30 += 0x1;
      }
      ((int)param_3 + 0x4) = iStack30;
      pass1_1038_6b88(puVar2,uVar3,uVar9,param_3,param_4);
    }
  }
  return;
}



i16 pass1_1038_6d24(param_1: *mut astruct_615,u32 *param_2,param_3: *mut u16,mut param_4: i16,mut param_5: u16 )

{
  u8 *puVar1;
  let mut local_14: i16;
  let mut local_12: i16;
  let mut local_10: i16;
  let mut local_e: i16;
  let mut local_c: i16;
  let mut local_a: i16;
  let mut local_8: u32;
  let mut uStack4: u16;

  *param_2 = 0x0;
  local_8 = (u32)(param_4 + 0xc);
  uStack4 = (param_4 + 0x10);
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,&local_e),
                  (u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_a));
  pass1_1008_3eb4((astruct_615 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x1a)),
                  (u16 *)CONCAT22(0x1050,&local_14),(u16 *)CONCAT22(0x1050,&local_12),
                  (u16 *)CONCAT22(0x1050,&local_10));
  local_c -= local_12;
  local_e -= local_14;
  puVar1 = (u8 *)(local_a - local_10);
  if (((puVar1 == NULL) && (local_c == 0x0)) && (local_e == 0x0)) {
    return 0x0;
  }
  if ((local_c != 0x0) || (puVar1 == NULL)) {
    if ((puVar1 == NULL) && (local_c != 0x0)) {
      pass1_1038_6c1c(NULL,param_1,param_3,param_2,local_c);
      return 0x1;
    }
    if (((puVar1 == NULL) && (local_c == 0x0)) && (local_e != 0x0)) {
      pass1_1038_6c68(NULL,param_1,param_3,param_2,local_e);
      if (local_c != 0x0) {
        return 0x1;
      }
      return local_c;
    }
  }
  pass1_1038_6bd4(puVar1,param_1,param_3,param_2,(int)puVar1);
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1038_6e1a(mut param_1: u16 ,mut param_2: u16 ,i32 *param_3)

{
  let mut uVar1: u16;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 bStack21;
  let mut uStack4: u16;

  uStack4 = 0x0;
  if ((*param_3 == 0x0) && (param_3 == 0x0)) {
    return 0x1;
  }
  uVar4 = ((int)param_3 + 0x2);
  bStack21 = (u8)(uVar4 >> 0x8);
  uVar1 = bStack21;
  if (bStack21 == 0x0) {
    uStack4 = param_3;
    goto switchD_1038_6eab_caseD_9;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,*param_3);
  uVar3 = pass1_1030_6fa0(CONCAT22(uVar4,uVar1));
  if ((int)uVar3 < 0xa) {
    switch(uVar3) {
    case 0x1:
      uStack4 = 0x1;
      break;
    case 0x2:
    case 0x6:
      uStack4 = 0x2;
      break;
    case 0x3:
    case 0x7:
      uStack4 = 0x3;
      break;
    case 0x4:
    case 0x8:
      uStack4 = 0x4;
      break;
    case 0x5:
    case 0x9:
      goto switchD_1038_6eab_caseD_5;
    }
  }
  else {
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar3,0x41);
    if (BVar2 != 0x0) {
      uStack4 = 0xa;
      goto switchD_1038_6eab_caseD_9;
    }
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0,uVar3,0x42);
    if ((BVar2 != 0x0) || (uVar3 == 0x3f)) {
      uStack4 = 0xb;
      goto switchD_1038_6eab_caseD_9;
    }
switchD_1038_6eab_caseD_5:
    uStack4 = 0x5;
  }
switchD_1038_6eab_caseD_9:
  switch(uStack4) {
  case 0x1:
    return 0x14;
  case 0x2:
  case 0x7:
    return 0x3c;
  case 0x3:
  case 0x8:
    return 0x78;
  case 0x4:
  case 0x9:
    return 0xf0;
  case 0x5:
  case 0x6:
    return 0xf;
  case 0xa:
    uVar3 = 0xc;
    break;
  case 0xb:
    uVar3 = 0xa;
    break;
  default:
    uVar3 = 0xffff;
  }
  return uVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_6f5a(mut param_1: u16 ,u8 *param_2,mut param_3: u32,param_4: *mut astruct_419)

{
  let mut uVar1: u32;
  i32 lVar2;
  let mut uVar4: u16;
  let mut puVar5: *mut u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar7;
  let mut iVar8: i16;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  astruct_99 *paStack16;
  let mut uStack12: u16;
  let mut local_a: u16;
  let mut uStack8: u16;
  let mut local_6: u16;
  let mut uStack4: u16;
  astruct_623 *uVar3;

  paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar10 = (param_3 >> 0x10);
  iVar8 = (int)param_3;
  if (*(i32 *)(iVar8 + 0xe) == 0x0) {
    if ((iVar8 + 0xc) != 0x0) {
      pass1_1030_7ddc(param_1,paVar7,(u32)param_4,*(i32 *)(iVar8 + 0x16),(iVar8 + 0xc));
      return;
    }
    if ((iVar8 + 0x12) != 0x0) {
      pass1_1030_7c50(param_1,paVar7,(astruct_305 *)param_4,*(i32 *)(iVar8 + 0x16),(iVar8 + 0x12));
      return;
    }
    paStack16 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
    uVar4 = ((u32)paStack16 >> 0x10);
    uVar3 = (astruct_623 *)paStack16;
    if ((uVar4 | uVar3) == 0x0) {
      paStack16 = NULL;
    }
    else {
      paStack16->field0_0x0 = 0x389a;
      uVar3->field2_0x2 = 0x1008;
      uVar3->field3_0x4 = 0x0;
      uVar3->field4_0x6 = 0x0;
      uVar3->field5_0x8 = 0x0;
      uVar3->field6_0xa = 0x0;
      uVar3->field7_0xc = 0x0;
      paStack16->field0_0x0 = 0x56ce;
      uVar3->field2_0x2 = 0x1018;
    }
    uVar11 = ((u32)paStack16 >> 0x10);
    iVar9 = (int)paStack16;
    (iVar9 + 0x8) = (iVar8 + 0x14);
    (iVar9 + 0xa) = (iVar8 + 0x16);
    uVar6 = pass1_1020_c42e((iVar8 + 0x14));
    lVar2 = (u32)uVar6 * (u32)(iVar9 + 0xa);
    uVar4 = lVar2;
    (iVar9 + 0xc) = uVar4;
    pass1_1030_6a2c(uVar4,(StructD *)((u32)lVar2 >> 0x10),(astruct_382 *)param_4,(astruct_383 *)paStack16);
  }
  else {
    uVar1 = (u32)(iVar8 + 0xe);
    uStack4 = ((int)uVar1 + 0x4);
    for (uStack12 = 0x0; uStack12 < uStack4; uStack12 += 0x1) {
      puVar5 = &local_6;
      pass1_1020_bb16((u32*)(iVar8 + 0xe),(u32 *)CONCAT22(0x1050,&local_a),(u16 *)CONCAT22(0x1050,puVar5),
                      uStack12);
      if (CONCAT22(uStack8,local_a) != 0x0) {
        pass1_1030_7ddc(puVar5,paVar7,(u32)param_4,CONCAT22(uStack8,local_a),local_6);
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_709c(u8 *param_1,param_2: *mut astruct_618,mut param_3: u32)

{
  u32 *puVar1;
  i32 lVar2;
  let mut uVar7: u16;
  let mut uVar8: u16;
  StructD *pSVar9;
  let mut in_register_0000000a: u16;
  astruct_618 *iVar8;
  let mut iVar10: i16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  astruct_99 *paStack40;
  astruct_99 *paStack16;
  let mut uStack12: u16;
  i32 local_a;
  let mut local_6: u16;
  let mut uStack4: u16;
  astruct_617 *uVar3;
  astruct_619 *uVar4;
  astruct_620 *uVar5;
  astruct_621 *uVar6;

  uVar11 = ((u32)param_2 >> 0x10);
  iVar8 = (astruct_618 *)param_2;
  if ((((int)&iVar8->field13_0xe + 0x2) | &iVar8->field13_0xe) == 0x0) {
    if (iVar8->field12_0xc == 0x0) {
      if (iVar8->field14_0x12 == 0x0) {
        if (iVar8->field15_0x14 == 0x0) {
          return;
        }
        paStack40 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar8 = ((u32)paStack40 >> 0x10);
        uVar3 = (astruct_617 *)paStack40;
        if ((uVar8 | uVar3) == 0x0) {
          paStack40 = NULL;
        }
        else {
          paStack40->field0_0x0 = 0x389a;
          uVar3->field2_0x2 = 0x1008;
          uVar3->field3_0x4 = 0x0;
          uVar3->field4_0x6 = 0x0;
          uVar3->field5_0x8 = 0x0;
          uVar3->field6_0xa = 0x0;
          uVar3->field7_0xc = 0x0;
          paStack40->field0_0x0 = 0x56ce;
          uVar3->field2_0x2 = 0x1018;
        }
        uVar12 = ((u32)paStack40 >> 0x10);
        ((int)paStack40 + 0x8) = iVar8->field15_0x14;
        ((int)paStack40 + 0xa) = &iVar8->field16_0x16;
        uVar8 = pass1_1020_c42e(iVar8->field15_0x14);
      }
      else {
        pass1_1030_7c50(0x0,(astruct_57 *)CONCAT22(in_register_0000000a,param_1),(astruct_305 *)param_3,
                        iVar8->field16_0x16,iVar8->field14_0x12);
        paStack40 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar8 = ((u32)paStack40 >> 0x10);
        uVar4 = (astruct_619 *)paStack40;
        if ((uVar8 | uVar4) == 0x0) {
          paStack40 = NULL;
        }
        else {
          paStack40->field0_0x0 = 0x389a;
          uVar4->field2_0x2 = 0x1008;
          uVar4->field3_0x4 = 0x0;
          uVar4->field4_0x6 = 0x0;
          uVar4->field5_0x8 = 0x0;
          uVar4->field6_0xa = 0x0;
          uVar4->field7_0xc = 0x0;
          paStack40->field0_0x0 = 0x56ce;
          uVar4->field2_0x2 = 0x1018;
        }
        uVar12 = ((u32)paStack40 >> 0x10);
        ((int)paStack40 + 0x6) = iVar8->field14_0x12;
        ((int)paStack40 + 0xa) = &iVar8->field16_0x16;
        uVar8 = switch_1020_c3b4(iVar8->field14_0x12);
      }
      uVar12 = ((u32)paStack40 >> 0x10);
      iVar10 = (int)paStack40;
      lVar2 = (u32)uVar8 * (u32)(iVar10 + 0xa);
      pSVar9 = (StructD *)((u32)lVar2 >> 0x10);
      uVar8 = lVar2;
    }
    else {
      paStack40 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
      uVar8 = ((u32)paStack40 >> 0x10);
      uVar5 = (astruct_620 *)paStack40;
      pSVar9 = (StructD *)(uVar8 | uVar5);
      if (pSVar9 == NULL) {
        paStack40 = NULL;
      }
      else {
        paStack40->field0_0x0 = 0x389a;
        uVar5->field2_0x2 = 0x1008;
        uVar5->field3_0x4 = 0x0;
        uVar5->field4_0x6 = 0x0;
        uVar5->field5_0x8 = 0x0;
        uVar5->field6_0xa = 0x0;
        uVar5->field7_0xc = 0x0;
        paStack40->field0_0x0 = 0x56ce;
        uVar5->field2_0x2 = 0x1018;
      }
      uVar12 = ((u32)paStack40 >> 0x10);
      iVar10 = (int)paStack40;
      (iVar10 + 0x4) = iVar8->field12_0xc;
      uVar8 = &iVar8->field16_0x16;
      (iVar10 + 0xa) = uVar8;
    }
    (iVar10 + 0xc) = uVar8;
    pass1_1030_6a2c(uVar8,pSVar9,(astruct_382 *)param_3,(astruct_383 *)CONCAT22(uVar12,iVar10));
  }
  else {
    puVar1 = iVar8->field13_0xe;
    uStack4 = ((int)puVar1 + 0x4);
    for (uStack12 = 0x0; uStack12 < uStack4; uStack12 += 0x1) {
      pass1_1020_bb16(iVar8->field13_0xe,(u32 *)CONCAT22(0x1050,&local_a),(u16 *)CONCAT22(0x1050,&local_6),uStack12
                     );
      if (local_a != 0x0) {
        paStack16 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar8 = ((u32)paStack16 >> 0x10);
        uVar6 = (astruct_621 *)paStack16;
        if ((uVar8 | uVar6) == 0x0) {
          paStack16 = NULL;
        }
        else {
          paStack16->field0_0x0 = 0x389a;
          uVar6->field2_0x2 = 0x1008;
          uVar6->field3_0x4 = 0x0;
          uVar6->field4_0x6 = 0x0;
          uVar6->field5_0x8 = 0x0;
          uVar6->field6_0xa = 0x0;
          uVar6->field7_0xc = 0x0;
          paStack16->field0_0x0 = 0x56ce;
          uVar6->field2_0x2 = 0x1018;
        }
        uVar12 = ((u32)paStack16 >> 0x10);
        iVar10 = (int)paStack16;
        (iVar10 + 0x4) = local_6;
        (iVar10 + 0xa) = local_a;
        uVar7 = pass1_1020_c3ae();
        lVar2 = (u32)uVar7 * (u32)(iVar10 + 0xa);
        uVar8 = lVar2;
        (iVar10 + 0xc) = uVar8;
        pass1_1030_6a2c(uVar8,(StructD *)((u32)lVar2 >> 0x10),(astruct_382 *)param_3,(astruct_383 *)paStack16);
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_7356(param_1: *mut astruct_615,param_2: *mut astruct_419)

{
  u8 **ppuVar1;
  let mut puVar2: *mut u16;
  char *pcVar3;
  i32 lVar4;
  let mut in_AX: u16;
  let mut BVar5: bool;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut in_DX: u16;
  u8 *puVar8;
  u8 *puVar9;
  astruct_615 *iVar9;
  let mut iVar10: i16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut bVar13: bool;
  let mut uVar14: u32;
  let mut uVar15: u32;
  astruct_99 *paStack50;
  astruct_99 *paStack26;
  astruct_616 *uVar8;
  astruct_622 *uVar10;

  uVar14 = struct_op_1030_73a8(param_2,in_AX,in_DX);
  puVar8 = (u8 *)(uVar14 >> 0x10);
  uVar6 = uVar14;
  puVar9 = puVar8;
  BVar5 = pass1_1008_c6ae(_u16_1050_06e0,(uVar6 + 0xc),0x4);
  iVar9 = (astruct_615 *)param_1;
  uVar11 = ((u32)param_1 >> 0x10);
  if (BVar5 == 0x0) {
    uVar7 = pass1_1008_c6ae(_u16_1050_06e0,(uVar6 + 0xc),0x3);
    if (uVar7 == 0x0) {
code_r0x10387545:
      pass1_1038_6f5a(uVar7,puVar9,(u32)param_1,param_2);
      goto LAB_1038_7549;
    }
    if ((iVar9->field9_0xc != 0x0) || (*(i32 *)&iVar9->field_0xe != 0x0)) {
      uVar14 = pass1_1028_45e2(uVar6,(int)puVar9,uVar14);
      puVar9 = (u8 *)(uVar14 >> 0x10);
      uVar7 = uVar14;
      ppuVar1 = (u8 **)&iVar9->field16_0x18;
      bVar13 = *ppuVar1 < puVar9;
      if ((bVar13 || *ppuVar1 == puVar9) &&
         ((bVar13 || (puVar2 = &iVar9->field15_0x16, *puVar2 < uVar7 || *puVar2 == uVar7)))) goto code_r0x10387545;
    }
  }
  else {
    uVar15 = pass1_1028_62c8(uVar14);
    puVar9 = (u8 *)(uVar15 >> 0x10);
    uVar7 = uVar15;
    ppuVar1 = (u8 **)&iVar9->field16_0x18;
    bVar13 = *ppuVar1 < puVar9;
    if ((bVar13 || *ppuVar1 == puVar9) &&
       ((bVar13 || (puVar2 = &iVar9->field15_0x16, *puVar2 < uVar7 || *puVar2 == uVar7)))) {
      if (iVar9->field13_0x12 == 0x0) {
        if (iVar9->field14_0x14 == 0x0) goto LAB_1038_74e0;
        paStack50 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar6 = ((u32)paStack50 >> 0x10);
        uVar10 = (astruct_622 *)paStack50;
        if ((uVar6 | uVar10) == 0x0) {
          paStack50 = NULL;
        }
        else {
          paStack50->field0_0x0 = 0x389a;
          uVar10->field2_0x2 = 0x1008;
          uVar10->field3_0x4 = 0x0;
          uVar10->field4_0x6 = 0x0;
          uVar10->field5_0x8 = 0x0;
          uVar10->field6_0xa = 0x0;
          uVar10->field7_0xc = 0x0;
          paStack50->field0_0x0 = 0x56ce;
          uVar10->field2_0x2 = 0x1018;
        }
        uVar12 = ((u32)paStack50 >> 0x10);
        iVar10 = (int)paStack50;
        (iVar10 + 0x8) = iVar9->field14_0x14;
        (iVar10 + 0xa) = iVar9->field15_0x16;
        uVar6 = pass1_1020_c42e(iVar9->field14_0x14);
      }
      else {
        paStack26 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
        uVar6 = ((u32)paStack26 >> 0x10);
        uVar8 = (astruct_616 *)paStack26;
        if ((uVar6 | uVar8) == 0x0) {
          paStack26 = NULL;
        }
        else {
          paStack26->field0_0x0 = 0x389a;
          uVar8->field2_0x2 = 0x1008;
          uVar8->field3_0x4 = 0x0;
          uVar8->field4_0x6 = 0x0;
          uVar8->field5_0x8 = 0x0;
          uVar8->field6_0xa = 0x0;
          uVar8->field7_0xc = 0x0;
          paStack26->field0_0x0 = 0x56ce;
          uVar8->field2_0x2 = 0x1018;
        }
        uVar12 = ((u32)paStack26 >> 0x10);
        iVar10 = (int)paStack26;
        (iVar10 + 0x6) = iVar9->field13_0x12;
        (iVar10 + 0xa) = iVar9->field15_0x16;
        uVar6 = switch_1020_c3b4(iVar9->field13_0x12);
      }
      lVar4 = (u32)uVar6 * (u32)(iVar10 + 0xa);
      puVar9 = (u8 *)((u32)lVar4 >> 0x10);
      uVar7 = lVar4;
      (iVar10 + 0xc) = uVar7;
      pass1_1028_6408(uVar14,(u32 *)CONCAT22(uVar12,iVar10));
      goto LAB_1038_7549;
    }
  }//
LAB_1038_74e0:
  pass1_1038_709c(puVar9,(astruct_618 *)param_1,(u32)param_2);//
LAB_1038_7549:
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,iVar9->field8_0x8);
  pass1_1030_6c4c(CONCAT22(puVar9,uVar7),(uVar7 + 0x34) + iVar9->field29_0x26);
  iVar9->field9_0xc = 0x0;
  iVar9->field13_0x12 = 0x0;
  iVar9->field14_0x14 = 0x0;
  (u32)&iVar9->field15_0x16 = 0x0;
  pcVar3 = *(char **)&iVar9->field_0xe;
  uVar6 = iVar9->field12_0x10;
  if ((uVar6 | pcVar3) != 0x0) {
    fn_ptr_1020_ba7e((u32 *)((u32)pcVar3 & 0xffff | (u32)uVar6 << 0x10));
    fn_ptr_1000_17ce(pcVar3);
  }
  (u32)&iVar9->field_0xe = 0x0;
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1038_75ca(mut param_1: i16,mut param_2: u32,mut param_3: u32)

{
  let mut BVar1: bool;
  let mut iVar2: i16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  HFILE16 in_stack_0000ffca;
  u32 local_10 [0x2];
  let mut local_8: u32;

  uVar4 = (param_2 >> 0x10);
  iVar3 = (int)param_2;
  pass1_1008_79f0(param_3,*(i32 *)(iVar3 + 0x4));
  if (param_1 != 0x0) {
    local_10[0] = (u32)(iVar3 + 0x8);
    BVar1 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,local_10),(char *)0x4,in_stack_0000ffca);
    if (BVar1 != 0x0) {
      write_to_file_1008_7a22(param_3,*(i32 *)(iVar3 + 0xe));
      if (BVar1 != 0x0) {
        local_8._0_2_ = (iVar3 + 0xc);
        BVar1 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_8),(char *)0x2,in_stack_0000ffca);
        if (BVar1 != 0x0) {
          local_8._0_2_ = (iVar3 + 0x12);
          BVar1 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_8),(char *)0x2,in_stack_0000ffca);
          if (BVar1 != 0x0) {
            local_8 = CONCAT22(local_8,(iVar3 + 0x14));
            BVar1 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_8),(char *)0x2,in_stack_0000ffca);
            if (BVar1 != 0x0) {
              local_8 = (u32)(iVar3 + 0x16);
              BVar1 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_8),(char *)0x4,in_stack_0000ffca);
              if (BVar1 != 0x0) {
                iVar2 = write_to_file_1008_7b4c(param_3,(astruct_615 *)(param_2 & 0xffff0000 | (u32)(iVar3 + 0x1a)));
                if (iVar2 != 0x0) {
                  local_8 = (u32)(iVar3 + 0x20);
                  BVar1 = write_to_file_1008_7e1c((u8 *)param_3,CONCAT22(0x1050,&local_8),(char *)0x4,in_stack_0000ffca)
                  ;
                  if (BVar1 != 0x0) {
                    local_8 = local_8 & 0xffff0000 | (u32)(iVar3 + 0x24);
                    BVar1 = write_to_file_1008_7e1c
                                      ((u8 *)param_3,CONCAT22(0x1050,&local_8),(char *)0x2,in_stack_0000ffca);
                    if (BVar1 != 0x0) {
                      local_8 = local_8 & 0xffff0000 | (u32)(iVar3 + 0x26);
                      BVar1 = write_to_file_1008_7e1c
                                        ((u8 *)param_3,CONCAT22(0x1050,&local_8),(char *)0x2,in_stack_0000ffca);
                      if (BVar1 != 0x0) {
                        local_8 = local_8 & 0xffff0000 | (u32)(iVar3 + 0x28);
                        BVar1 = write_to_file_1008_7e1c
                                          ((u8 *)param_3,CONCAT22(0x1050,&local_8),(char *)0x2,in_stack_0000ffca);
                        if (BVar1 != 0x0) {
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
  u16_1050_0310 = 0x6d0;
  return;
}
pub fn file_1038_774e(u8 *param_1,param_2: *mut astruct_169,mut param_3: u32)

{
  astruct_169 *iVar2;
  let mut BVar1: bool;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar6: u16;
  let mut local_8: u16;
  let mut local_6: u16;
  let mut local_4: u16;
  astruct_169 *paVar5;

  if ((int)u16_1050_0312 < 0x2) {
    return;
  }
  iVar2 = (astruct_169 *)param_2;
  iVar2 = (astruct_169 *)&iVar2->field4_0x4;
  paVar5 = (astruct_169 *)((u32)param_2 & 0xffff0000 | ZEXT24(iVar2));
  pass1_1008_766e(param_1,param_3,paVar5);
  if ((((((int)paVar5 != 0x0) &&
        (BVar1 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)((u32)param_2 & 0xffff0000 | ZEXT24(iVar2 + 0x1)),0x4),
        BVar1 != 0x0)) &&
       (iVar3 = file_1008_77cc(param_1,param_3,
                               (i32 *)((u32)param_2 & 0xffff0000 |
                                       ZEXT24((u8 *)((int)&iVar2[0x1].field4_0x4 + 0x2)))), iVar3 != 0x0)) &&
      ((((BVar1 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)CONCAT22(0x1050,&local_4),0x2), BVar1 != 0x0 &&
         (BVar1 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)CONCAT22(0x1050,&local_6),0x2), BVar1 != 0x0)) &&
        ((BVar1 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)CONCAT22(0x1050,&local_8),0x2), BVar1 != 0x0 &&
         ((BVar1 = read_file_1008_7dee((HFILE16 *)param_3,
                                       (u8 *)((u32)param_2 & 0xffff0000 |
                                             ZEXT24((u8 *)((int)&iVar2[0x2].field4_0x4 + 0x2))),0x4),
          BVar1 != 0x0 &&
          (BVar1 = read_file_1008_7bc8(param_3,(u16 *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar2[0x3].field_0x2))),
          BVar1 != 0x0)))))) &&
       (BVar1 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)((u32)param_2 & 0xffff0000 | ZEXT24(iVar2 + 0x4)),0x4),
       BVar1 != 0x0)))) &&
     (((BVar1 = read_file_1008_7dee((HFILE16 *)param_3,
                                    (u8 *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar2[0x4].field4_0x4)),0x2),
       BVar1 != 0x0 &&
       (BVar1 = read_file_1008_7dee((HFILE16 *)param_3,
                                    (u8 *)((u32)param_2 & 0xffff0000 |
                                          ZEXT24((u8 *)((int)&iVar2[0x4].field4_0x4 + 0x2))),0x2), BVar1 != 0x0))
      && (BVar1 = read_file_1008_7dee((HFILE16 *)param_3,(u8 *)((u32)param_2 & 0xffff0000 | ZEXT24(iVar2 + 0x5)),0x2),
         BVar1 != 0x0)))) {
    uVar6 = ((u32)param_2 >> 0x10);
    &iVar2[0x1].field4_0x4 = local_4;
    uVar4 = switch_1008_72bc((HFILE16 *)param_3,local_6);
    &iVar2[0x2].field_0x2 = uVar4;
    &iVar2[0x2].field4_0x4 = local_8;
    return;
  }
  u16_1050_0310 = 0x6d2;
  return;
}



StructD * pass1_1038_78b8(StructD *param_1,u8 param_2)

{
  pass1_1038_6912(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_78e2(StructD *param_1,param_2: *mut astruct_431)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  astruct_431 *iVar4;
  let mut uVar5: u16;
  astruct_57 *paVar4;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar5 = ((u32)param_2 >> 0x10);
  iVar4 = (astruct_431 *)param_2;
  uVar1 = 0x0;
  (u32)param_2 = 0x0;
  (u32)&iVar4->field2_0x4 = 0x0;
  _PTR_LOOP_1050_5a64 = param_2;
  mem_op_1000_179c(0xc,paVar3);
  uVar2 = paVar3 | uVar1;
  paVar4 = (astruct_57 *)((u32)paVar3 & 0xffff0000 | (u32)uVar2);
  if (uVar2 == 0x0) {
    (u32)param_2 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar3,uVar1));
    param_2->field0_0x0 = uVar1;
    iVar4->field1_0x2 = (u8 *)paVar4;
  }
  mem_op_1000_179c(0xc,paVar4);
  uVar2 = paVar4 | uVar1;
  if (uVar2 == 0x0) {
    uVar1 = 0x0;
    uVar2 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar4,uVar1));
  }
  iVar4->field2_0x4 = uVar1;
  iVar4->field3_0x6 = uVar2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_7964(param_1: *mut u16)

{
  let mut uVar1: u16;
  u32 *puVar2;
  code **ppcVar3;
  let mut iVar4: i16;
  let mut uVar5: u16;

  _PTR_LOOP_1050_5a64 = 0x0;
  uVar5 = ((u32)param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar1 = (iVar4 + 0x2);
  if ((uVar1 | (u32 *)*param_1) != 0x0) {
    ppcVar3 = (code **)(u32)*param_1;
    (**ppcVar3)();
  }
  puVar2 = (u32 *)(iVar4 + 0x4);
  uVar1 = (iVar4 + 0x6);
  if ((uVar1 | puVar2) != 0x0) {
    ppcVar3 = (code **)*puVar2;
    (**ppcVar3)();
  }
  return;
}
pub fn pass1_1038_79b2(mut param_1: u16 ,u8 *param_2,mut param_3: u32,mut param_4: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar5 = 0x1000;
  mem_op_1000_179c(0x14,paVar3);
  uVar2 = paVar3 | param_1;
  if (uVar2 == 0x0) {
    param_1 = 0x0;
    uVar2 = 0x0;
  }
  else {
    uVar5 = 0x1030;
    pass1_1030_aefa((astruct_400 *)CONCAT22(paVar3,param_1),param_4);
  }
  uVar4 = (param_3 >> 0x10);
  ppcVar1 = (code **)((int)(u32)(u32)((int)param_3 + 0x4) + 0x4);
  (**ppcVar1)(uVar5,(u32)((int)param_3 + 0x4),param_1,uVar2);
  return;
}
pub fn pass1_1038_79f2(mut param_1: u32,mut param_2: u32,mut param_3: u16 )

{
  code **ppcVar1;
  u8 *puVar2;
  let mut extraout_DX: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  u8 local_e [0x8];
  i32 lStack6;

  lStack6 = *(i32 *)((int)param_2 + 0x4);
  uVar4 = (param_1 >> 0x10);
  iVar3 = (int)param_1;
  pass1_1008_5784((char *)CONCAT22(0x1050,local_e),(u32)(iVar3 + 0x4));
  do {
    puVar2 = local_e;
    pass1_1008_5b12((char *)CONCAT22(0x1050,puVar2));
    if ((extraout_DX | puVar2) == 0x0) {
      return;
    }
  } while (*(i32 *)(puVar2 + 0x4) != lStack6);
  ppcVar1 = (code **)((int)(u32)(u32)(iVar3 + 0x4) + 0xc);
  (**ppcVar1)(0x1008,(u32)(iVar3 + 0x4),puVar2,extraout_DX);
  return;
}
pub fn pass1_1038_7a5a(u32 *param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)(u32)*param_1 + 0x4);
  (**ppcVar1)();
  return;
}
pub fn pass1_1038_7a76(u32 *param_1,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 )

{
  code **ppcVar1;
  let mut uVar2: u16;
  astruct_615 *paVar3;
  let mut uVar4: u32;
  u8 local_a [0x4];
  let mut uStack6: u32;

  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),*param_1);
  while( true ) {
    paVar3 = (astruct_615 *)pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    if (paVar3 == NULL) break;
    pass1_1038_6a0e((astruct_419 *)CONCAT22(paVar3,((u32)paVar3 >> 0x10) | paVar3),paVar3);
  }
  do {
    uStack6 = 0x0;
    do {
      uVar4 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
      if (uVar4 == 0x0) {
        pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)param_1 + 0x4));
        while( true ) {
          uVar4 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
          if (uVar4 == 0x0) break;
          pass1_1030_affc(uVar4);
        }
        return;
      }
      uVar2 = pass1_1038_6b3c(uVar4);
    } while (uVar2 == 0x0);
    ppcVar1 = (code **)((int)(u32)*param_1 + 0xc);
    (**ppcVar1)(0x1008);
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps

u16 pass1_1038_7b20(u32 *param_1,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut uVar4: u32;
  HFILE16 in_stack_0000ffce;
  let mut local_1c: u16;
  let mut uStack26: u16;
  let mut uStack24: u16;
  let mut uStack16: u32;
  u8 local_c [0x8];
  let mut local_4: u16;

  BVar2 = write_to_file_1008_7cac(param_2);
  if (BVar2 != 0x0) {
    local_1c = ((int)*param_1 + 0x8);
    local_4 = local_1c;
    BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_1c),(char *)0x2,in_stack_0000ffce);
    if (BVar2 != 0x0) {
      pass1_1008_5784((char *)CONCAT22(0x1050,local_c),*param_1);
      do {
        uStack16 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_c));
        if (uStack16 == 0x0) {
          uVar3 = ((u32)param_1 >> 0x10);
          uVar1 = (u32)((int)param_1 + 0x4);
          local_1c = ((int)uVar1 + 0x8);
          local_4 = local_1c;
          BVar2 = write_to_file_1008_7e1c((u8 *)param_2,CONCAT22(0x1050,&local_4),(char *)0x2,in_stack_0000ffce);
          if (BVar2 == 0x0) {
            return 0x0;
          }
          pass1_1008_5784((char *)CONCAT22(0x1050,local_c),(u32)((int)param_1 + 0x4));
          do {
            uVar4 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_c));
            uStack26 = uVar4;
            if (uVar4 == 0x0) {
              return 0x1;
            }
            pass1_1030_b768(uVar4,param_2);
            uStack24 = (uVar4 >> 0x10);
          } while ((int)uVar4 != 0x0);
          return 0x0;
        }
        pass1_1038_75ca((int)uStack16,uStack16,param_2);
        uStack16 = (uStack16 >> 0x10);
      } while ((int)uStack16 != 0x0);
    }
  }
  return 0x0;
}



u16 read_file_1038_7c02(u16_t param_1,u16_t param_2,u32 *param_3,HFILE16 *param_4)

{
  code **ppcVar1;
  let mut BVar2: bool;
  let mut uVar3: u16;
  u8 *puVar4;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar7;
  astruct_57 *paVar8;
  HFILE16 *pHVar9;
  u16 local_12 [0x2];
  let mut uStack14: u32;
  let mut local_4: u16;

  paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  if ((int)u16_1050_0312 < 0x2) {
    return 0x1;
  }
  read_file_1008_7cfe((int)param_4,(int)((u32)param_4 >> 0x10),0x17);
  if ((param_1 != 0x0) && (BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,&local_4),0x2), BVar2 != 0x0)) {
    while (local_4 != 0x0) {
      uVar3 = local_4;
      local_4 = local_4 - 0x1;
      pHVar9 = param_4;
      mem_op_1000_179c(0x2a,paVar7);
      uVar5 = paVar7;
      uVar6 = uVar5 | uVar3;
      paVar7 = (astruct_57 *)((u32)paVar7 & 0xffff0000);
      paVar8 = (astruct_57 *)((u32)paVar7 | (u32)uVar6);
      if (uVar6 == 0x0) {
        uVar3 = 0x0;
      }
      else {
        struct_1038_6520((astruct_308 *)CONCAT22(uVar5,uVar3));
        paVar7 = paVar8;
      }
      puVar4 = (u8 *)paVar7;
      uStack14 = CONCAT22(puVar4,uVar3);
      file_1038_774e(puVar4,(astruct_169 *)CONCAT22(puVar4,uVar3),(u32)pHVar9);
      if (uVar3 == 0x0) {
        return 0x0;
      }
      ppcVar1 = (code **)((int)(u32)*param_3 + 0x4);
      (**ppcVar1)();
    }
    local_4 = local_4 - 0x1;
    BVar2 = read_file_1008_7dee(param_4,(u8 *)CONCAT22(0x1050,local_12),0x2);
    if (BVar2 != 0x0) {
      while( true ) {
        if (local_12[0] == 0x0) {
          return 0x1;
        }
        uVar3 = local_12[0];
        local_12[0] = local_12[0] - 0x1;
        pHVar9 = param_4;
        mem_op_1000_179c(0x14,paVar7);
        uVar5 = paVar7;
        uVar6 = uVar5 | uVar3;
        paVar7 = (astruct_57 *)((u32)paVar7 & 0xffff0000);
        paVar8 = (astruct_57 *)((u32)paVar7 | (u32)uVar6);
        if (uVar6 == 0x0) {
          uVar3 = 0x0;
        }
        else {
          pass1_1030_ae6c((astruct_399 *)CONCAT22(uVar5,uVar3));
          paVar7 = paVar8;
        }
        file_1030_b836((u8 *)paVar7,(astruct_401 *)CONCAT22((u8 *)paVar7,uVar3),(u32)pHVar9);
        if (uVar3 == 0x0) break;
        ppcVar1 = (code **)((int)(u32)(u32)((int)param_3 + 0x4) + 0x4);
        (**ppcVar1)();
      }
      return 0x0;
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * pass1_1038_7d10(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_57 *iVar1;
  let mut unaff_BP: u16;
  astruct_57 *uVar1;
  u32 *puVar2;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  struct_1040_b082(param_2,CONCAT22(param_3,0x1853));
  uVar1 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)&iVar1[0x1].field3_0x6 = 0x0;
  param_2->field0_0x0 = 0x8876;
  iVar1->field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x40),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar1[0x1].field3_0x6 = puVar2;
  iVar1[0x1].field4_0x8 = ((u32)puVar2 >> 0x10);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_7d5c(StructD *param_1)

{
  let mut uVar1: u16;
  let mut in_stack_0000ffde: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x8876;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}
pub fn destroy_window_1038_7d88(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 )

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  pass1_1008_b544(param_3,(u32)((int)param_1 + 0x94),param_2);
  DestroyWindow16(*(HWND16 *)((int)param_1 + 0x6));
  return;
}



LRESULT pass1_1038_7dac(param_1: *mut astruct_903,mut param_2: u16 )

{
  LRESULT LVar1;

  pass1_1040_78de();
  LVar1 = send_dlg_item_msg_1038_844a(param_1);
  return LVar1;
}
pub fn pass1_1038_7dc6(u8 *param_1,astruct_903 *pstruct903_param_2,mut param_3: u16 ,mut param_4: u32)

{
  let mut bVar1: bool;
  LRESULT LVar2;
  let mut uVar3: u32;

  bVar1 = false;
  if (param_4 == 0x1854) {
    if ((int)param_4 != 0x1) goto LAB_1038_7e8c;
    send_dlg_item_msg_1038_8618s(param_1,pstruct903_param_2);
  }
  else {
    if (param_4 < 0x18550000) {
      if (param_4 == 0xeb) {
        LVar2 = send_dlg_item_msg_1038_844a(pstruct903_param_2);
        param_1 = (u8 *)((u32)LVar2 >> 0x10);
      }
      else if (param_4 == 0xfb) {
        LVar2 = send_dlg_item_msg_1038_7eac(pstruct903_param_2);
        param_1 = (u8 *)((u32)LVar2 >> 0x10);
      }
      else {
        if (param_4 != (int)s_vrpal_bmp_1050_183a + 0x7) {//
LAB_1038_7e77:
          pass1_1040_b54a(param_1,pstruct903_param_2,param_3,param_4);
          return;
        }
        msg_box_op_1038_81be(0x0,param_1,pstruct903_param_2);
      }
      goto LAB_1038_7e8c;
    }
    if (param_4 == 0x1855) {
      if ((int)param_4 != 0x1) goto LAB_1038_7e8c;
      send_dlg_item_msg_1038_87b2(param_1,pstruct903_param_2);
    }
    else if (param_4 == 0x1856) {
      if ((int)param_4 != 0x1) goto LAB_1038_7e8c;
      pass1_1038_8810((u32)pstruct903_param_2);
    }
    else if (param_4 == 0x1858) {
      send_dlg_item_msg_1038_7fae(0x0,param_1,pstruct903_param_2);
    }
    else {
      if (param_4 != 0x1859) goto LAB_1038_7e77;
      uVar3 = pass1_1038_801a(param_1,pstruct903_param_2);
      param_1 = (u8 *)(uVar3 >> 0x10);
    }
  }
  bVar1 = true;//
LAB_1038_7e8c:
  if (bVar1) {
    set_win_text_1038_8358(param_1,pstruct903_param_2);
    enable_win_1038_806a(param_1,(astruct_902 *)pstruct903_param_2);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT send_dlg_item_msg_1038_7eac(param_1: *mut astruct_903)

{
  astruct_57 *in_EDX;
  let mut iVar1: i16;
  let mut uVar2: u16;
  u32 *puVar3;
  char *l_param;
  LRESULT LVar4;
  let mut in_stack_0000fe9a: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000fff2: u16;

  puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff2,0x30),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  l_param = (char *)pass1_1010_375e((u32)puVar3);
  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  pass1_1008_b1a6((u32)(iVar1 + 0x94),l_param);
  SendDlgItemMessage16(0x0,0x0,0xb,0x1854,*(HWND16 *)(iVar1 + 0x6));
  LVar4 = SendDlgItemMessage16(0x0,0x0,0x409,0x1854,*(HWND16 *)(iVar1 + 0x6));
  if (((WPARAM16)LVar4 != 0xffff) || ((int)((u32)LVar4 >> 0x10) != -0x1)) {
    SendDlgItemMessage16(0x0,(WPARAM16)LVar4,0x403,0x1854,*(HWND16 *)(iVar1 + 0x6));
    SendDlgItemMessage16((LPARAM)l_param,0x0,0x401,0x1854,*(HWND16 *)(iVar1 + 0x6));
    SendDlgItemMessage16(0x0,0xffff,0x407,0x1854,*(HWND16 *)(iVar1 + 0x6));
    SendDlgItemMessage16(0x0,0x0,0x405,0x1855,*(HWND16 *)(iVar1 + 0x6));
    LVar4 = SendDlgItemMessage16(0x0,0x0,0x405,0x1856,*(HWND16 *)(iVar1 + 0x6));
    enable_win_1038_806a((int)((u32)LVar4 >> 0x10),(astruct_902 *)param_1);
  }
  LVar4 = SendDlgItemMessage16(0x0,0x1,0xb,0x1854,*(HWND16 *)(iVar1 + 0x6));
  return LVar4;
}
pub fn send_dlg_item_msg_1038_7fae(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  LRESULT LVar3;

  uVar2 = ((u32)param_3 >> 0x10);
  iVar1 = (int)param_3;
  pass1_1008_b146(param_1,param_2,(u32)(iVar1 + 0x94));
  SendDlgItemMessage16(0x0,0xffff,0x407,0x1855,*(HWND16 *)(iVar1 + 0x6));
  LVar3 = SendDlgItemMessage16(0x0,0xffff,0x407,0x1856,*(HWND16 *)(iVar1 + 0x6));
  pass1_1008_b61a(LVar3,(int)((u32)LVar3 >> 0x10),(u32)(iVar1 + 0x94),0x0);
  pass1_1008_b63a((u32)(iVar1 + 0x94),0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_801a(mut param_1: u16 ,param_2: *mut astruct_903) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut in_register_0000000a: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  astruct_477 *paVar6;
  char *pcVar7;
  let mut uVar8: u32;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000fff0: u16;
  let mut uVar3: u32;

  paVar6 = (astruct_477 *)
           mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000fff0,0x30),in_stack_0000fe98,in_stack_0000ffbc,
                           in_stack_0000ffc2,in_stack_0000ffc6);
  uVar5 = ((u32)param_2 >> 0x10);
  uVar4 = param_2;
  pcVar7 = (char *)pass1_1008_b340((u32)(uVar4 + 0x94));
  uVar1 = pcVar7;
  uVar2 = ((u32)pcVar7 >> 0x10) | uVar1;
  uVar3 = (u32)uVar2;
  if (uVar2 != 0x0) {
    pass1_1010_3770(uVar2,paVar6,pcVar7);
    uVar8 = pass1_1038_af40(uVar4,uVar2,_PTR_LOOP_1050_5b7c,(uVar4 + 0x6),0x3);
    uVar3 = uVar8 >> 0x10;
    uVar1 = uVar8;
  }
  return CONCAT22((int)uVar3,uVar1);
}
pub fn enable_win_1038_806a(mut param_1: u16 ,param_2: *mut astruct_902)

{
  HWND16 HVar1;
  let mut BVar2: bool;
  astruct_902 *iVar3;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut uVar5: u32;
  let mut uVar6: u32;

  uVar3 = ((u32)param_2 >> 0x10);
  iVar3 = (astruct_902 *)param_2;
  HVar1 = GetDlgItem16(0x1,iVar3->field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16(0x1858,iVar3->field6_0x6);
  EnableWindow16(0x0,HVar1);
  HVar1 = GetDlgItem16(0x1859,iVar3->field6_0x6);
  BVar2 = EnableWindow16(0x0,HVar1);
  uVar4 = pass1_1008_b820(BVar2,param_1,iVar3->field147_0x94);
  if (uVar4 != 0x0) {
    uVar4 = pass1_1008_b340(iVar3->field147_0x94);
    uVar5 = pass1_1008_b366(iVar3->field147_0x94);
    uVar6 = pass1_1008_b47a(iVar3->field147_0x94);
    if (((uVar4 != 0x0) && (uVar5 != 0x0)) && (uVar6 != 0x0)) {
      HVar1 = GetDlgItem16(0x1,iVar3->field6_0x6);
      EnableWindow16(0x1,HVar1);
      HVar1 = GetDlgItem16(0x1858,iVar3->field6_0x6);
      EnableWindow16(0x1,HVar1);
    }
    if (uVar4 != 0x0) {
      HVar1 = GetDlgItem16(0x1859,iVar3->field6_0x6);
      EnableWindow16(0x1,HVar1);
    }
  }
  return;
}



u16 send_dlg_item_msg_1038_8164(mut param_1: u16 ,mut param_2: u16 ,u8 *param_3,mut param_4: u16 )

{
  LRESULT LVar1;

  *param_3 = '\0';
  LVar1 = SendDlgItemMessage16(0x0,0x0,0x409,param_4,*(HWND16 *)(param_1 + 0x6));
  if ((LVar1 != -0x1) &&
     (LVar1 = SendDlgItemMessage16((LPARAM)param_3,(WPARAM16)LVar1,0x40a,param_4,*(HWND16 *)(param_1 + 0x6)),
     LVar1 != -0x1)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn msg_box_op_1038_81be(char *param_1,mut param_2: u16 ,param_3: *mut astruct_903)

{
  short in_buf_len_5;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  let mut uVar2: u16;
  char local_206 [0x102];
  char local_104 [0x102];

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = (short)paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_206,(short)&DAT_1050_1050);
  load_string_1010_84e0(_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  uVar2 = ((u32)param_3 >> 0x10);
  MessageBox16(0x0,(char *)CONCAT22(0x1050,local_206),(char *)CONCAT22(in_buf_len_5,param_1),
               *(HWND16 *)((int)param_3 + 0x6));
  load_string_1010_84e0(_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  MessageBox16(0x0,(char *)CONCAT22(0x1050,local_206),(char *)CONCAT22(in_buf_len_5,param_1),
               *(HWND16 *)((int)param_3 + 0x6));
  fn_ptr_1000_17ce((char *)CONCAT22(in_buf_len_5,param_1));
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn set_win_text_1038_8358(mut param_1: u16 ,param_2: *mut astruct_903)

{
  char *pcVar1;
  let mut uVar2: u16;
  let mut uVar4: u16;
  let mut uVar3: u16;
  char local_30a [0x102];
  CHAR local_208 [0x100];
  u8 local_108 [0x100];
  let mut uStack8: u32;
  HWND16 HStack4;
  let mut uVar1: u32;

  uVar3 = ((u32)param_2 >> 0x10);
  uVar4 = param_2;
  HStack4 = GetDlgItem16(0x1857,*(HWND16 *)(uVar4 + 0x6));
  uStack8 = pass1_1008_b820(HStack4,param_1,(u32)(uVar4 + 0x94));
  if (uStack8 == 0x0) {
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,local_30a,(short)&DAT_1050_1050);
    pcVar1 = local_30a;
  }
  else {
    uVar2 = send_dlg_item_msg_1038_8164(uVar4,uVar3,(u8 *)CONCAT22(0x1050,local_108),0x1854);
    if (uVar2 == 0x0) {
      load_string_1010_84e0
                (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,local_208,(short)&DAT_1050_1050);
    }
    else {
      load_string_1008_b65a((u32)(uVar4 + 0x94),local_208,CONCAT22(local_108,0x1050),&DAT_1050_1050);
    }
    pcVar1 = local_208;
  }
  SetWindowText16(CONCAT22(0x1050,pcVar1),HStack4);
  return;
}
pub fn send_dlg_item_msg_1038_8400(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 )

{
  i32 lVar1;
  u8 local_a [0x8];

  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),param_3);
  while( true ) {
    lVar1 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    if (lVar1 == 0x0) break;
    SendDlgItemMessage16(*(LPARAM *)((int)lVar1 + 0x4),0x0,0x401,param_4,*(HWND16 *)(param_1 + 0x6));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT send_dlg_item_msg_1038_844a(param_1: *mut astruct_903)

{
  HWND16 hwnd;
  let mut BVar1: bool;
  let mut uVar2: u16;
  let mut uVar3: u16;
  astruct_903 *uVar4;
  let mut uVar5: u16;
  LRESULT LVar6;
  char local_108 [0x102];
  let mut uStack6: u32;

  uVar5 = ((u32)param_1 >> 0x10);
  uVar4 = (astruct_903 *)param_1;
  SendDlgItemMessage16(0x0,0x0,0xb,0x1854,uVar4->field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0xb,0x1855,uVar4->field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0xb,0x1856,uVar4->field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0x405,0x1854,uVar4->field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0x405,0x1855,uVar4->field6_0x6);
  LVar6 = SendDlgItemMessage16(0x0,0x0,0x405,0x1856,uVar4->field6_0x6);
  uStack6 = pass1_1008_b820((int)LVar6,((u32)LVar6 >> 0x10),(u32)uVar4->field147_0x94);
  uVar2 = (uStack6 >> 0x10) | uStack6;
  if (uStack6 == 0x0) {
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,local_108,(short)&DAT_1050_1050);
    SendDlgItemMessage16(CONCAT22(0x1050,local_108),0x0,0x401,0x1854,uVar4->field6_0x6);
    SendDlgItemMessage16(0x0,0x1,0xb,0x1854,uVar4->field6_0x6);
    SendDlgItemMessage16(0x0,0x1,0xb,0x1855,uVar4->field6_0x6);
    LVar6 = SendDlgItemMessage16(0x0,0x1,0xb,0x1856,uVar4->field6_0x6);
    uVar3 = ((u32)LVar6 >> 0x10);
    hwnd = GetDlgItem16(0x1857,uVar4->field6_0x6);
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,local_108,(short)&DAT_1050_1050);
    BVar1 = SetWindowText16(CONCAT22(0x1050,local_108),hwnd);
    return CONCAT22(uVar3,BVar1);
  }
  send_dlg_item_msg_1038_8400(uVar4,uVar5,uStack6,0x1854);
  set_win_text_1038_8358(uVar2,param_1);
  SendDlgItemMessage16(0x0,0x1,0xb,0x1854,uVar4->field6_0x6);
  SendDlgItemMessage16(0x0,0x1,0xb,0x1855,uVar4->field6_0x6);
  LVar6 = SendDlgItemMessage16(0x0,0x1,0xb,0x1856,uVar4->field6_0x6);
  return LVar6;
}



// WARNING: Could not reconcile some variable overlaps

u16 send_dlg_item_msg_1038_8618s(mut param_1: u16 ,param_2: *mut astruct_903)

{
  let mut in_AX: i16;
  let mut uVar1: u16;
  u8 *puVar2;
  u8 *puVar3;
  let mut uVar4: u16;
  let mut uVar5: u16;
  LRESULT LVar6;
  let mut l_param: u32;
  let mut uVar7: u32;
  u8 local_106 [0x100];
  let mut uStack6: u32;

  uVar5 = ((u32)param_2 >> 0x10);
  uVar4 = param_2;
  uStack6 = pass1_1008_b820(in_AX,param_1,(u32)(uVar4 + 0x94));
  uVar1 = uStack6;
  if (uStack6 != 0x0) {
    uVar1 = send_dlg_item_msg_1038_8164(uVar4,uVar5,(u8 *)CONCAT22(0x1050,local_106),0x1854);
    if (uVar1 != 0x0) {
      SendDlgItemMessage16(0x0,0x0,0xb,0x1855,*(HWND16 *)(uVar4 + 0x6));
      SendDlgItemMessage16(0x0,0x0,0xb,0x1856,*(HWND16 *)(uVar4 + 0x6));
      SendDlgItemMessage16(0x0,0x0,0x405,0x1855,*(HWND16 *)(uVar4 + 0x6));
      LVar6 = SendDlgItemMessage16(0x0,0x0,0x405,0x1856,*(HWND16 *)(uVar4 + 0x6));
      puVar3 = (u8 *)((u32)LVar6 >> 0x10);
      puVar2 = local_106;
      pass1_1008_b4a0(puVar2,puVar3,(u32)(uVar4 + 0x94),CONCAT22(0x1050,puVar2));
      pass1_1008_b200(*(astruct_194 **)(uVar4 + 0x94));
      uVar7 = CONCAT22(puVar3 | puVar2,puVar2);
      if ((puVar3 | puVar2) != 0x0) {
        send_dlg_item_msg_1038_8400(uVar4,uVar5,CONCAT22(puVar3,puVar2),0x1855);
        l_param = pass1_1008_b366((u32)(uVar4 + 0x94));
        uVar7 = l_param & 0xffff | (u32)((l_param >> 0x10) | l_param) << 0x10;
        if (l_param != 0x0) {
          uVar7 = SendDlgItemMessage16(l_param,0xffff,0x40d,0x1855,*(HWND16 *)(uVar4 + 0x6));
        }
      }
      uVar7 = pass1_1008_b38c((StructD *)CONCAT22((int)uVar7,(int)(uVar7 >> 0x10)),*(astruct_196 **)(uVar4 + 0x94));
      if (uVar7 != 0x0) {
        send_dlg_item_msg_1038_8400(uVar4,uVar5,uVar7,0x1856);
        uVar7 = pass1_1008_b47a((u32)(uVar4 + 0x94));
        if (uVar7 != 0x0) {
          SendDlgItemMessage16(uVar7,0xffff,0x40d,0x1856,*(HWND16 *)(uVar4 + 0x6));
        }
      }
      SendDlgItemMessage16(0x0,0x1,0xb,0x1855,*(HWND16 *)(uVar4 + 0x6));
      LVar6 = SendDlgItemMessage16(0x0,0x1,0xb,0x1856,*(HWND16 *)(uVar4 + 0x6));
      uVar1 = LVar6;
    }
  }
  return uVar1;
}



u16 send_dlg_item_msg_1038_87b2(mut param_1: u16 ,param_2: *mut astruct_903)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  char *l_param;
  LRESULT LVar3;
  let mut uVar4: u16;
  u8 local_102 [0x100];

  uVar4 = param_2;
  uVar1 = ((u32)param_2 >> 0x10);
  uVar2 = send_dlg_item_msg_1038_8164(uVar4,uVar1,(u8 *)CONCAT22(0x1050,local_102),0x1855);
  if (uVar2 != 0x0) {
    pass1_1008_b61a(local_102,param_1,(u32)(uVar4 + 0x94),CONCAT22(0x1050,local_102));
    l_param = load_string_1008_b1f0();
    LVar3 = SendDlgItemMessage16((LPARAM)l_param,0xffff,0x40d,0x1856,*(HWND16 *)(uVar4 + 0x6));
    uVar2 = LVar3;
  }
  return uVar2;
}
pub fn pass1_1038_8810(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  u8 local_102 [0x100];

  uVar2 = (param_1 >> 0x10);
  uVar1 = send_dlg_item_msg_1038_8164(param_1,uVar2,(u8 *)CONCAT22(0x1050,local_102),0x1856);
  if (uVar1 != 0x0) {
    pass1_1008_b63a((u32)(param_1 + 0x94),CONCAT22(0x1050,local_102));
  }
  return;
}



u16 FUN_1038_8842(void)

{
  return 0x0;
}
pub fn pass1_1038_8848(void)

{
  return;
}
pub fn pass1_1038_884c(void)

{
  return;
}



StructD * pass1_1038_8850(StructD *param_1,u8 param_2)

{
  pass1_1038_7d5c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_88f2(param_1: *mut astruct_57,mut param_2: u16 )

{
  astruct_57 *iVar1;
  astruct_57 *uVar1;

  struct_1040_b082(param_1,CONCAT22(param_2,0x184c));
  uVar1 = (astruct_57 *)((u32)param_1 >> 0x10);
  iVar1 = (astruct_57 *)param_1;
  (u32)&iVar1[0x1].field3_0x6 = _u16_1050_5a68;
  iVar1[0x1].field5_0xa = 0x0;
  iVar1[0x1].field6_0xc = 0x0;
  iVar1[0x1].field7_0xe = 0x0;
  iVar1[0x1].field8_0x10 = 0x0;
  param_1->field0_0x0 = 0x8c2e;
  iVar1->field1_0x2 = &u16_1050_1038;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_893a(StructD *param_1)

{
  let mut uVar1: u16;
  let mut in_stack_0000ffde: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x8c2e;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}



u16 pass1_1038_8966(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: i16,mut param_5: u16 )

{
  let mut piVar1: *mut i16;
  let mut bVar2: bool;
  let mut iVar3: i16;
  let mut uVar4: u16;

  bVar2 = false;
  iVar3 = (int)param_1;
  uVar4 = (param_1 >> 0x10);
  if (param_4 == 0x0) {
    if ((iVar3 + 0x98) < 0x1) goto LAB_1038_89af;
    piVar1 = (i16 *)(iVar3 + 0x9a);
    *piVar1 = *piVar1 + 0x1;
    piVar1 = (i16 *)(iVar3 + 0x98);
    *piVar1 = *piVar1 + -0x1;
  }
  else {
    if (param_4 != 0x1) goto LAB_1038_89af;
    if ((iVar3 + 0x9a) < 0x1) goto LAB_1038_89af;
    piVar1 = (i16 *)(iVar3 + 0x9a);
    *piVar1 = *piVar1 + -0x1;
    piVar1 = (i16 *)(iVar3 + 0x98);
    *piVar1 = *piVar1 + 0x1;
  }
  bVar2 = true;//
LAB_1038_89af:
  if (bVar2) {
    SetDlgItemInt16(0x0,(iVar3 + 0x9a),(int)s_dibtext_bmp_1050_1844 + 0x9,*(HWND16 *)(iVar3 + 0x6));
    SetDlgItemInt16(0x0,(iVar3 + 0x98),(int)s_dibtext_bmp_1050_1844 + 0xb,*(HWND16 *)(iVar3 + 0x6));
  }
  return 0x0;
}
pub fn pass1_1038_89e8(mut param_1: u32)

{
  send_dlg_item_msg_1038_8b58((astruct_903 *)param_1);
  return;
}
pub fn pass1_1038_89f8(param_1: *mut astruct_903,mut param_2: u16 ,mut param_3: u32,u8 *param_4,mut param_5: u16 )

{
  if (param_3 == 0xeb) {
    send_dlg_item_msg_1038_8b58(param_1);
  }
  else {
    if (param_3 != (int)s_vrpal_bmp_1050_183a + 0x7) {
      pass1_1040_b54a(param_4,param_1,param_2,param_3);
      return;
    }
    msg_box_ui_op_1038_8a3a(0x0,param_4,param_1,(int)&DAT_1050_1050);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn msg_box_ui_op_1038_8a3a(char *param_1,mut param_2: u16 ,param_3: *mut astruct_903,mut param_4: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  char local_20a [0x102];
  char *pcStack264;
  short sStack262;
  char local_104 [0x102];

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  sStack262 = (short)paVar1;
  pcStack264 = param_1;
  load_string_1010_84e0(_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,param_1,sStack262);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(sStack262,pcStack264),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(sStack262,pcStack264),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x101,local_20a,(short)&DAT_1050_1050);
  MessageBox16(0x0,(char *)CONCAT22(0x1050,local_20a),(char *)CONCAT22(sStack262,pcStack264),
               *(HWND16 *)((int)param_3 + 0x6));
  fn_ptr_1000_17ce((char *)CONCAT22(sStack262,pcStack264));
  return;
}
pub fn unk_win_ui_op_1038_8afe(mut param_1: u16 ,param_2: *mut astruct_50)

{
  let mut uVar1: u32;
  let mut dlg_item: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar2;
  astruct_50 *iVar4;
  astruct_50 *uVar4;
  let mut local_4: bool;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar4 = (astruct_50 *)((u32)param_2 >> 0x10);
  iVar4 = (astruct_50 *)param_2;
  dlg_item = GetDlgItemInt16(0x0,&local_4,(INT16)&DAT_1050_1050,(int)s_dibtext_bmp_1050_1844 + 0x9);
  pass1_1030_6c1a(iVar4->field148_0x94,dlg_item);
  uVar1 = iVar4->field148_0x94;
  pass1_1038_387e(paVar2,*(astruct_302 **)((int)uVar1 + 0x2e),dlg_item,iVar4->field153_0x9c,iVar4->field148_0x94);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn send_dlg_item_msg_1038_8b58(param_1: *mut astruct_903)

{
  let mut uVar1: u32;
  u8 *puVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  astruct_57 *in_EDX;
  let mut iVar5: i16;
  let mut uVar6: u16;
  LRESULT LVar7;
  let mut in_stack_0000fd96: u16;
  let mut in_stack_0000feba: u16;
  let mut in_stack_0000fec0: u16;
  let mut in_stack_0000fec4: u16;
  let mut in_stack_0000feee: u16;
  u8 local_106 [0x100];
  u32 *puStack6;

  puStack6 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000feee,0x3),in_stack_0000fd96,
                             in_stack_0000feba,in_stack_0000fec0,in_stack_0000fec4);
  puVar2 = (u8 *)((u32)puStack6 >> 0x10);
  uVar6 = ((u32)param_1 >> 0x10);
  iVar5 = (int)param_1;
  pass1_1010_c3c2(puVar2,puStack6,puVar2,CONCAT22(0x1050,local_106),(u32)(iVar5 + 0x94));
  LVar7 = SendDlgItemMessage16
                    (CONCAT22(0x1050,local_106),0x0,0xc,(int)s_dibtext_bmp_1050_1844 + 0x2,*(HWND16 *)(iVar5 + 0x6));
  uVar4 = ((u32)LVar7 >> 0x10);
  uVar1 = (u32)(iVar5 + 0x94);
  (iVar5 + 0x9c) = ((int)uVar1 + 0x32);
  (iVar5 + 0x9a) = (iVar5 + 0x9c);
  SetDlgItemInt16(0x0,(iVar5 + 0x9c),(int)s_dibtext_bmp_1050_1844 + 0x9,*(HWND16 *)(iVar5 + 0x6));
  uVar1 = (u32)(iVar5 + 0x94);
  uVar3 = (u32)((int)uVar1 + 0x2e);
  pass1_1038_3aa6(uVar3,uVar4,uVar3);
  (iVar5 + 0x98) = uVar3;
  SetDlgItemInt16(0x0,uVar3,(int)s_dibtext_bmp_1050_1844 + 0xb,*(HWND16 *)(iVar5 + 0x6));
  return;
}



StructD * pass1_1038_8c08(StructD *param_1,u8 param_2)

{
  pass1_1038_893a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * pass1_1038_8caa(u8 *param_1,param_2: *mut astruct_57,mut param_3: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_57 *iVar1;
  let mut unaff_BP: u16;
  let mut uVar2: u16;
  u32 *puVar3;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  struct_1040_b082(param_2,CONCAT22(param_3,0x185a));
  uVar2 = ((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)&iVar1[0x1].field3_0x6 = 0x0;
  param_2->field0_0x0 = 0x90c8;
  iVar1->field1_0x2 = &u16_1050_1038;
  puVar3 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3f),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar1[0x1].field3_0x6 = puVar3;
  iVar1[0x1].field4_0x8 = ((u32)puVar3 >> 0x10);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_8cf6(StructD *param_1)

{
  let mut uVar1: u16;
  let mut in_stack_0000ffde: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x90c8;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}
pub fn send_dlg_item_msg_1038_8d22(mut param_1: u32,undefined1 param_2)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  LRESULT LVar3;
  u8 local_106 [0x100];
  WPARAM16 WStack6;
  let mut iStack4: i16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  LVar3 = SendDlgItemMessage16(0x0,0x0,0x409,0x185b,*(HWND16 *)(iVar1 + 0x6));
  WStack6 = (WPARAM16)LVar3;
  iStack4 = (int)WStack6 >> 0xf;
  if (WStack6 != 0xffff) {
    SendDlgItemMessage16(CONCAT22(0x1050,local_106),WStack6,0x40a,0x185b,*(HWND16 *)(iVar1 + 0x6));
    pass1_1008_c79a((u32)(iVar1 + 0x94),(char *)CONCAT22(0x1050,local_106));
  }
  return;
}



LRESULT pass1_1038_8d7e(param_1: *mut astruct_903)

{
  LRESULT LVar1;

  pass1_1040_78de();
  LVar1 = send_dlg_item_msg_1038_8f74(param_1);
  return LVar1;
}
pub fn pass1_1038_8d98(u8 *param_1,param_2: *mut astruct_903,mut param_3: u16 ,mut param_4: u32)

{
  if (param_4 == 0xeb) {
    send_dlg_item_msg_1038_8f74(param_2);
  }
  else {
    if (param_4 != (int)s_vrpal_bmp_1050_183a + 0x7) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4);
      return;
    }
    msg_box_op_1038_8dda(0x0,param_1,param_2);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn msg_box_op_1038_8dda(char *param_1,mut param_2: u16 ,param_3: *mut astruct_903)

{
  short in_buf_len_5;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  let mut uVar2: u16;
  char local_206 [0x102];
  char local_104 [0x102];

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x1000,paVar1);
  in_buf_len_5 = (short)paVar1;
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,local_206,(short)&DAT_1050_1050);
  load_string_1010_84e0(_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  uVar2 = ((u32)param_3 >> 0x10);
  MessageBox16(0x0,(char *)CONCAT22(0x1050,local_206),(char *)CONCAT22(in_buf_len_5,param_1),
               *(HWND16 *)((int)param_3 + 0x6));
  load_string_1010_84e0(_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,param_1,in_buf_len_5);
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,local_104,(short)&DAT_1050_1050);
  pass1_1000_3cea(CONCAT22(in_buf_len_5,param_1),(char *)CONCAT22(0x1050,local_104));
  MessageBox16(0x0,(char *)CONCAT22(0x1050,local_206),(char *)CONCAT22(in_buf_len_5,param_1),
               *(HWND16 *)((int)param_3 + 0x6));
  fn_ptr_1000_17ce((char *)CONCAT22(in_buf_len_5,param_1));
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT send_dlg_item_msg_1038_8f74(param_1: *mut astruct_903)

{
  let mut uVar1: u32;
  let mut iVar3: i16;
  HWND16 hwnd;
  astruct_903 *iVar2;
  let mut uVar4: u16;
  i32 lVar4;
  LRESULT LVar5;
  let mut enable: bool;
  WORD local_50c [0x80];
  u8 local_40c [0x8];
  let mut local_404: u32;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_903 *)param_1;
  SendDlgItemMessage16(0x0,0x0,0xb,0x185b,iVar2->field6_0x6);
  SendDlgItemMessage16(0x0,0x0,0x405,0x185b,iVar2->field6_0x6);
  iVar3 = pass1_1008_c83a(iVar2->field147_0x94);
  if (iVar3 == 0x0) {
    local_404 = pass1_1008_c85e(iVar2->field147_0x94);
    pass1_1008_5784((char *)CONCAT22(0x1050,local_40c),local_404);
    while( true ) {
      lVar4 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_40c));
      if (lVar4 == 0x0) break;
      uVar1 = (u32)((int)lVar4 + 0x4);
      wsprintf16(local_50c,(char *)0x5a6c1050,(char *)CONCAT22((int)uVar1,0x1050),(int)((u32)uVar1 >> 0x10));
      SendDlgItemMessage16(CONCAT22(0x1050,local_50c),0x0,0x401,0x185b,iVar2->field6_0x6);
    }
    hwnd = GetDlgItem16(0x1,iVar2->field6_0x6);
    enable = 0x1;
  }
  else {
    load_string_1010_84e0
              (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x3ff,(char *)&local_404,
               (short)&DAT_1050_1050);
    SendDlgItemMessage16(CONCAT22(0x1050,&local_404),0x0,0x401,0x185b,iVar2->field6_0x6);
    hwnd = GetDlgItem16(0x1,iVar2->field6_0x6);
    enable = 0x0;
  }
  EnableWindow16(enable,hwnd);
  LVar5 = SendDlgItemMessage16(0x0,0x1,0xb,0x185b,iVar2->field6_0x6);
  return LVar5;
}



StructD * pass1_1038_90a2(StructD *param_1,u8 param_2)

{
  pass1_1038_8cf6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_9144(mut param_1: u16 ,param_2: *mut u16,mut param_3: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar4;
  astruct_57 *iVar6;
  let mut iVar7: i16;
  let mut unaff_SI: u16;
  astruct_57 *uVar8;
  let mut uVar9: u16;
  u32 *puVar10;
  let mut in_stack_0000fe9c: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000ffca: u16;
  let mut piStack8: *mut i16;
  astruct_57 *paVar5;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  struct_1040_b082((astruct_57 *)param_2,CONCAT22(param_3,0xfaa));
  uVar8 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar6 = (astruct_57 *)param_2;
  iVar6[0x1].field3_0x6 = 0x0;
  iVar6[0x1].field4_0x8 = 0x0;
  (u32)&iVar6[0x1].field5_0xa = 0x0;
  *param_2 = 0x99a2;
  iVar6->field1_0x2 = &u16_1050_1038;
  iVar6->field105_0x8a = 0x27;
  puVar10 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x28),in_stack_0000fe9c,
                            in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
  paVar4 = (astruct_57 *)((u32)paVar4 & 0xffff0000 | (u32)puVar10 >> 0x10);
  uVar2 = puVar10;
  iVar6[0x1].field5_0xa = uVar2;
  iVar6[0x1].field6_0xc = ((u32)puVar10 >> 0x10);
  mem_op_1000_179c(0x18,paVar4);
  uVar3 = paVar4 | uVar2;
  paVar5 = (astruct_57 *)((u32)paVar4 & 0xffff0000 | (u32)uVar3);
  if (uVar3 == 0x0) {
    (u32)&iVar6[0x1].field1_0x2 = 0x0;
  }
  else {
    struct_1040_a598((astruct_259 *)CONCAT22(paVar4,uVar2));
    iVar6[0x1].field1_0x2 = uVar2;
    iVar6[0x1].field2_0x4 = paVar5;
  }
  *(u16*)&iVar6[0x1].field1_0x2 = 0x11;
  iVar7 = **(i16 **)&iVar6[0x1].field1_0x2;
  uVar2 = iVar7 * 0xa + 0x2;
  mem_op_1000_179c(uVar2,paVar5);
  uVar3 = paVar5;
  piStack8 = (i16 *)CONCAT22(uVar3,uVar2);
  if ((uVar3 | uVar2) == 0x0) {
    uVar1 = (u32)&iVar6[0x1].field1_0x2;
    (u32)((int)uVar1 + 0x2) = 0x0;
  }
  else {
    *piStack8 = iVar7;
    pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,iVar7,0xa,uVar2 + 0x2,uVar3);
    uVar1 = (u32)&iVar6[0x1].field1_0x2;
    uVar9 = ((u32)uVar1 >> 0x10);
    iVar7 = (int)uVar1;
    (iVar7 + 0x2) = uVar2 + 0x2;
    (iVar7 + 0x4) = uVar3;
  }
  uVar1 = (u32)&iVar6[0x1].field1_0x2;
  ((int)uVar1 + 0xa) = 0x18;
  uVar1 = (u32)&iVar6[0x1].field1_0x2;
  ((int)uVar1 + 0x12) = iVar6->field5_0xa;
  return;
}
pub fn pass1_1038_927c(u32 *param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)*param_1 + 0x74);
  (**ppcVar1)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_dlg_op_1038_9294(mut param_1: u16 ,StructB *param_2)

{
  let mut UVar1: u16;
  let mut uVar1: u16;
  StructB *struct_b_1_hi;
  let mut local_6: bool;
  let mut local_4: bool;

  unk_win_ui_op_1040_b230(param_1,param_2);
  struct_b_1_hi = (StructB *)((u32)param_2 >> 0x10);
  UVar1 = GetDlgItemInt16(0x1,&local_4,(INT16)&DAT_1050_1050,0xfa9);
  ((int)param_2 + 0x94) = UVar1;
  uVar1 = GetDlgItemInt16(0x1,&local_6,(INT16)&DAT_1050_1050,0xfa8);
  ((int)param_2 + 0x96) = uVar1;
  win_ui_dlg_op_1038_98b4((StructB *)((u32)param_2 & 0xffff | ZEXT24(struct_b_1_hi) << 0x10));
  win_1008_5c7c(uVar1,param_1,_u16_1050_02a0,0x950001);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn draw_op_1038_92f6(u8 *param_1,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut iVar4: i16;
  StructD *pSVar5;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar7;
  astruct_57 *paVar9;
  let mut unaff_SI: u16;
  let mut uVar10: u16;
  let mut in_stack_0000fe88: u16;
  let mut in_stack_0000ffac: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb6: u16;
  HFILE16 *hfile_param;
  BOOL16 local_1a [0x2];
  let mut UStack22: u16;
  StructD *pSStack20;
  StructD *pSStack16;
  let mut iStack12: i16;
  StructD *pSStack10;
  astruct_20 *paStack6;
  astruct_57 *paVar8;

  paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  if (param_5 == 0xeb) {
    paStack6 = (astruct_20 *)
               mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x3),in_stack_0000fe88,
                               in_stack_0000ffac,in_stack_0000ffb2,in_stack_0000ffb6);
    paVar7 = (astruct_57 *)((u32)paVar7 & 0xffff0000 | (u32)paStack6 >> 0x10);
    pSVar5 = *(StructD **)(param_2 + 0x90);
    if (pSVar5 != NULL) {
      pSStack10 = pSVar5;
      mem_op_1000_179c(0x18,paVar7);
      uVar3 = pSVar5;
      pSStack16 = (StructD *)((u32)pSVar5 & 0xffff | (long)paVar7 << 0x10);
      uVar6 = paVar7 | uVar3;
      paVar9 = (astruct_57 *)((u32)paVar7 & 0xffff0000);
      paVar8 = (astruct_57 *)((u32)paVar9 | (u32)uVar6);
      if (uVar6 == 0x0) {
        uVar3 = 0x0;
      }
      else {
        struct_1040_a598((astruct_259 *)((u32)pSVar5 & 0xffff | (long)paVar7 << 0x10));
        paVar9 = paVar8;
      }
      (param_2 + 0x90) = uVar3;
      (param_2 + 0x92) = (int)paVar9;
      (u32)(param_2 + 0x90) = 0x11;
      iStack12 = **(i16 **)(param_2 + 0x90);
      uVar3 = iStack12 * 0xa + 0x2;
      mem_op_1000_179c(uVar3,paVar9);
      uVar6 = paVar9;
      pSStack16 = (StructD *)CONCAT22(uVar6,uVar3);
      if ((uVar6 | uVar3) == 0x0) {
        uVar2 = (u32)(param_2 + 0x90);
        (u32)((int)uVar2 + 0x2) = 0x0;
      }
      else {
        pSStack16 = iStack12;
        pass1_1000_5586(0xa564,&PTR_LOOP_1050_1040,iStack12,0xa,uVar3 + 0x2,uVar6);
        uVar2 = (u32)(param_2 + 0x90);
        uVar10 = ((u32)uVar2 >> 0x10);
        iVar4 = (int)uVar2;
        (iVar4 + 0x2) = uVar3 + 0x2;
        (iVar4 + 0x4) = uVar6;
      }
      uVar10 = ((u32)pSStack10 >> 0x10);
      uVar2 = (u32)(param_2 + 0x90);
      (u32)((int)uVar2 + 0x6) = (u32)((int)pSStack10 + 0x6);
      uVar2 = (u32)(param_2 + 0x90);
      ((int)uVar2 + 0xa) = ((int)pSStack10 + 0xa);
      uVar2 = (u32)(param_2 + 0x90);
      ((int)uVar2 + 0x12) = (param_2 + 0xa);
      uVar10 = 0x1010;
      pass1_1010_a50c(paStack6,(u8 **)&u32_1050_5b42,*(StructD **)(param_2 + 0x90));
      pSStack20 = pSStack10;
      pSStack16 = pSStack10;
      if (pSStack10 != NULL) {
        pass1_1040_a5d0(pSStack10);
        uVar10 = 0x1000;
        fn_ptr_1000_17ce((char *)pSStack10);
      }
      ppcVar1 = (code **)((int)(u32)CONCAT22(param_3,param_2) + 0x70);
      (**ppcVar1)(uVar10,param_2,param_3);
    }
  }
  else {
    if (param_5 != 0xf9) {
      pass1_1040_b54a(param_1,(astruct_903 *)CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,param_2)),param_4,
                      param_5);
      return;
    }
    iVar4 = pass1_1038_993a(param_2,param_3,param_4);
    if (-0x1 < iVar4) {
      hfile_param = *(HFILE16 **)(param_2 + 0x6);
      UStack22 = GetDlgItemInt16(0x1,local_1a,(INT16)&DAT_1050_1050,*(HWND16 *)(iVar4 * 0xe + 0x5a72));
      if (local_1a[0] != 0x0) {
        FUN_1010_2a32(*(u8 **)(param_2 + 0x98),CONCAT22((iVar4 * 0xe + 0x5a72),UStack22),hfile_param,
                      unaff_SI);
      }
    }
  }
  return;
}



BOOL16 send_dlg_item_int_1038_94da(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16)

{
  let mut pUVar1: *mut u16;
  let mut iVar2: i16;
  HWND16 hwnd;
  i32 lVar3;
  let mut local_c: bool;
  let mut uStack10: u16;
  let mut iStack8: i16;
  let mut UStack6: u16;
  let mut iStack4: i16;

  iStack4 = 0x1;
  iStack8 = pass1_1038_993a(param_1,param_2,param_3);
  if ((-0x1 < iStack8) &&
     (UStack6 = GetDlgItemInt16(0x1,&local_c,(INT16)&DAT_1050_1050,*(HWND16 *)(iStack8 * 0xe + 0x5a72)), local_c != 0x0)
     ) {
    if (param_5 == 0x0) {
      UStack6 += 0x1;
    }
    else {
      iStack4 = -0x1;
      UStack6 -= 0x1;
    }
    uStack10 = ((int)UStack6 <= (iStack8 * 0xe + 0x5a7a));
    pUVar1 = (u16 *)(iStack8 * 0xe + 0x5a78);
    if (*pUVar1 != UStack6 && (int)UStack6 <= (int)*pUVar1) {
      uStack10 = 0x0;
    }
    iVar2 = iStack8 * 0xe;
    hwnd = GetDlgItem16(*(INT16 *)(iVar2 + 0x5a72),*(HWND16 *)(param_1 + 0x6));
    SetFocus16(hwnd);
    if ((uStack10 != 0x0) &&
       (lVar3 = unk_win_ui_op_1038_9820((StructB *)CONCAT22(param_2,param_1),0x1,iStack4,iStack8), (int)lVar3 != 0x0)) {
      SetDlgItemInt16(0x1,UStack6,*(INT16 *)(iVar2 + 0x5a72),*(HWND16 *)(param_1 + 0x6));
      SetDlgItemInt16(0x1,(param_1 + 0x94),0xfa9,*(HWND16 *)(param_1 + 0x6));
      SetDlgItemInt16(0x1,(param_1 + 0x96),0xfa8,*(HWND16 *)(param_1 + 0x6));
    }
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_msg_op_1038_95fc(mut param_1: u16 ,mut param_2: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut UVar3: u16;
  let mut UVar4: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar6;
  astruct_57 *paVar7;
  let mut iVar9: i16;
  let mut unaff_SI: u16;
  let mut uVar10: u16;
  let mut in_stack_0000fe80: u16;
  let mut in_stack_0000ffa4: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffae: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut puStack30: *mut u16;
  let mut puStack24: *mut u16;
  let mut iStack20: i16;
  let mut local_10: bool;
  u32 *puStack14;
  u32 *puStack10;
  u32 *puStack6;
  astruct_57 *paVar8;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  puStack6 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x8),in_stack_0000fe80,
                             in_stack_0000ffa4,in_stack_0000ffaa,in_stack_0000ffae);
  paVar6 = (astruct_57 *)((u32)paVar6 & 0xffff0000 | (u32)puStack6 >> 0x10);
  puStack10 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x9),in_stack_0000fe80,
                              in_stack_0000ffa4,in_stack_0000ffaa,in_stack_0000ffae);
  paVar7 = (astruct_57 *)((u32)paVar6 & 0xffff0000 | (u32)puStack10 >> 0x10);
  uVar2 = puStack10;
  mem_op_1000_179c(0xc,paVar7);
  uVar5 = paVar7 | uVar2;
  paVar6 = (astruct_57 *)((u32)paVar7 & 0xffff0000);
  paVar8 = (astruct_57 *)((u32)paVar6 | (u32)uVar5);
  if (uVar5 == 0x0) {
    uVar2 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)CONCAT22(paVar7,uVar2));
    paVar6 = paVar8;
  }
  puStack14 = (u32 *)CONCAT22((int)paVar6,uVar2);
  for (iStack20 = 0x0; iStack20 < 0xf; iStack20 += 0x1) {
    uVar13 = ((int)param_2 + 0x6);
    UVar3 = GetDlgItemInt16(0x1,&local_10,(INT16)&DAT_1050_1050,*(HWND16 *)(iStack20 * 0xe + 0x5a72));
    if (UVar3 != 0x0) {
      if ((iStack20 * 0xe + 0x5a7c) < 0x83) {
        UVar4 = UVar3;
        mem_op_1000_179c(0x8,paVar6);
        uVar2 = paVar6;
        puStack24 = (u16 *)CONCAT22(uVar2,UVar4);
        paVar6 = (astruct_57 *)((u32)paVar6 & 0xffff0000 | (u32)(uVar2 | UVar4));
        if ((uVar2 | UVar4) == 0x0) {
          puStack30 = NULL;
        }
        else {
          *puStack24 = 0x389a;
          (UVar4 + 0x2) = 0x1008;
          *puStack24 = 0xa1c4;
          (UVar4 + 0x2) = 0x1010;
          puStack30 = puStack24;
        }
        uVar10 = ((u32)puStack30 >> 0x10);
        iVar9 = (int)puStack30;
        (iVar9 + 0x6) = UVar3;
        (iVar9 + 0x4) = (iStack20 * 0xe + 0x5a7c);
        ppcVar1 = (code **)((int)*puStack14 + 0x4);
        (**ppcVar1)(0x1000,(int)puStack14,(int)((u32)puStack14 >> 0x10),iVar9,uVar10,uVar13);
      }
      else {
        if ((iStack20 * 0xe + 0x5a7c) == 0x89) {
          uVar12 = (iStack20 * 0xe + 0x5a7c);
          uVar11 = UVar3;
        }
        else {
          uVar12 = (iStack20 * 0xe + 0x5a7c);
          uVar11 = 0x0;
        }
        pass1_1010_6566((u32)puStack10,uVar11,UVar3,uVar12);
      }
    }
  }
  (u32)((int)puStack6 + 0xa) = puStack14;
  PostMessage16(0x0,0xed,0x111,HWND16_1050_0396);
  return;
}
pub fn win_ui_op_1038_977a(param_1: *mut astruct_57,mut param_2: i16,mut param_3: u16 ,mut param_4: i16)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar6: u16;
  u8 local_10 [0x4];
  u32 *puStack12;
  let mut iStack8: i16;
  let mut uStack6: u16;
  let mut local_4: bool;
  let mut uVar5: u32;

  iStack8 = 0x0;
  uVar6 = (param_2 + 0x6);
  uVar2 = GetDlgItemInt16(0x1,&local_4,(INT16)&DAT_1050_1050,0xfa8);
  uStack6 = uVar2;
  if (uVar2 != 0x0) {
    mem_op_1000_179c(0xb4,param_1);
    uVar3 = param_1 | uVar2;
    uVar5 = (u32)param_1 & 0xffff0000 | (u32)uVar3;
    if (uVar3 == 0x0) {
      iVar2 = 0x0;
      uVar4 = 0x0;
    }
    else {
      iVar2 = string_1040_8520(uVar5,(astruct_57 *)CONCAT22(param_1,uVar2),(param_2 + 0x6),0x20041,
                               0x5da05db);
      uVar4 = uVar5;
    }
    puStack12 = (u32 *)CONCAT22(uVar4,iVar2);
    pass1_1008_941a((u16 *)CONCAT22(0x1050,local_10),0x1,0xc3);
    ppcVar1 = (code **)((int)*puStack12 + 0x6c);
    iStack8 = (**ppcVar1)(0x1008,(int)puStack12,(int)((u32)puStack12 >> 0x10),local_10,(int)&DAT_1050_1050,uVar6,uVar2
                         );
  }
  if ((iStack8 == 0x1) || (uStack6 == 0x0)) {
    destroy_window_1040_b726(CONCAT22(param_3,param_2),param_4);
  }
  return;
}



i32 unk_win_ui_op_1038_9820(StructB *param_1,mut param_2: i16,mut param_3: i16,mut param_4: i16)

{
  let mut puVar1: *mut u16;
  u32 **ppuVar2;
  i32 lVar3;
  let mut UVar4: u16;
  let mut iVar5: i16;
  let mut iVar6: i16;
  let mut uVar8: u16;
  StructB *iVar7;
  StructB *uVar7;
  let mut local_6: bool;
  let mut local_4: bool;

  uVar7 = (StructB *)((u32)param_1 >> 0x10);
  iVar7 = (StructB *)param_1;
  UVar4 = GetDlgItemInt16(0x1,&local_4,(INT16)&DAT_1050_1050,*(HWND16 *)(param_4 * 0xe + 0x5a74));
  iVar5 = UVar4 * param_2 * param_3;
  UVar4 = GetDlgItemInt16(0x1,&local_6,(INT16)&DAT_1050_1050,*(HWND16 *)(param_4 * 0xe + 0x5a76));
  lVar3 = (long)(int)(UVar4 * param_2) * (long)param_3;
  uVar8 = ((u32)lVar3 >> 0x10);
  iVar6 = (int)lVar3;
  if (((int)(iVar5 - iVar7[0x7].max_count_field_0x10) < 0x1) && (-0x1 < (int)iVar7[0x7].field5_0xa - iVar6)) {
    puVar1 = &iVar7[0x7].max_count_field_0x10;
    *puVar1 = *puVar1 - iVar5;
    ppuVar2 = &iVar7[0x7].field5_0xa;
    *ppuVar2 = (u32 *)((int)*ppuVar2 - iVar6);
    return CONCAT22(uVar8,0x1);
  }
  return (u32)uVar8 << 0x10;
}
pub fn win_ui_dlg_op_1038_98b4(StructB *param_1)

{
  let mut UVar1: u16;
  StructB *iVar3;
  StructB *uVar2;
  LPVOID pvVar2;
  let mut iVar4: i16;
  let mut iStack8: i16;
  let mut local_4: bool;

  local_4 = 0x0;
  for (iStack8 = 0x0; iVar3 = (StructB *)param_1, uVar2 = (StructB *)((u32)param_1 >> 0x10), iStack8 < 0xf;
      iStack8 += 0x1) {
    iVar4 = 0x1;
    pvVar2 = iVar3->lpvoid_field_0x8;
    UVar1 = GetDlgItemInt16(0x1,&local_4,(INT16)&DAT_1050_1050,*(HWND16 *)(iStack8 * 0xe + 0x5a72));
    unk_win_ui_op_1038_9820(param_1,UVar1,(int)pvVar2,iVar4);
  }
  SetDlgItemInt16(0x1,iVar3[0x7].max_count_field_0x10,0xfa9,(HWND16)iVar3->lpvoid_field_0x8);
  SetDlgItemInt16(0x1,iVar3[0x7].field5_0xa,0xfa8,(HWND16)iVar3->lpvoid_field_0x8);
  return;
}



i16 pass1_1038_993a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16)

{
  let mut iStack6: i16;

  iStack6 = 0x0;
  while( true ) {
    if (0xe < iStack6) {
      return -0x1;
    }
    if ((iStack6 * 0xe + 0x5a70) == param_3) break;
    iStack6 += 0x1;
  }
  return iStack6;
}



StructD * pass1_1038_997c(StructD *param_1,u8 param_2)

{
  let mut in_stack_0000ffda: u16;

  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1038_9a1e(param_1: *mut astruct_57,param_2: *mut astruct_57,mut param_3: u16 ,mut param_4: u32)

{
  pass1_1040_b040((astruct_57 *)CONCAT22(param_2,param_1),CONCAT22((int)param_4,param_3),(param_4 >> 0x10));
  CONCAT22(param_2,param_1) = 0x9af6;
  param_1->field1_0x2 = &u16_1050_1038;
  return (u16 *)CONCAT22(param_2,param_1);
}
pub fn pass1_1038_9a48(StructD *param_1)

{
  let mut in_stack_0000ffde: u16;

  param_1->address_offset_field_0x0 = 0x9af6;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  unk_draw_op_1040_b0f8(in_stack_0000ffde,param_1);
  return;
}
pub fn enable_win_1038_9a66(u8 *param_1,astruct_903 *pstruct903_param_2,u16 in_b_enable_3,mut param_4: u32)

{
  let mut enable: bool;
  HWND16 hwnd;

  if (param_4 == 0xf8) {
    hwnd = GetDlgItem16(0x17d9,*(HWND16 *)((int)pstruct903_param_2 + 0x6));
    enable = 0x1;
  }
  else {
    if (param_4 != 0x17d9) {
      pass1_1040_b54a(param_1,pstruct903_param_2,in_b_enable_3,param_4);
      return;
    }
    enable = 0x0;
    SetWindowPos16(0x6,0x1a0,0x12c,0x0,0x0,0x0,*(HWND16 *)((int)pstruct903_param_2 + 0x6));
    hwnd = 0x0;
  }
  EnableWindow16(enable,hwnd);
  return;
}



StructD * pass1_1038_9ad0(StructD *param_1,u8 param_2)

{
  pass1_1038_9a48(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



pub fn pass1_1038_9b72(param_1: *mut astruct_57,param_2: *mut astruct_57,mut param_3: u16 ,mut param_4: u32) -> u32

{
  let mut iStack4: i16;

  pass1_1040_b040((astruct_57 *)CONCAT22(param_2,param_1),CONCAT22((int)param_4,param_3),(param_4 >> 0x10));
  param_1[0x2].field6_0xc = 0x0;
  CONCAT22(param_2,param_1) = 0x9efa;
  param_1->field1_0x2 = &u16_1050_1038;
  iStack4 = 0x0;
  do {
    (&param_1[0x1].field3_0x6)[iStack4] = 0x0;
    iStack4 += 0x1;
  } while (iStack4 < 0x4a);
  return CONCAT22(param_2,param_1);
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_win_ui_op_1038_9bc8(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,StructB *param_4)

{
  let mut IVar2: i16;
  let mut iVar2: i16;
  HDC16 hdc;
  INT16 IVar1;
  HWND16 HVar2;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar3;
  StructB *struct_b_7;
  let mut uVar4: u16;
  u32 *puVar5;
  let mut in_stack_0000fe60: u16;
  let mut in_stack_0000fe70: u16;
  let mut in_stack_0000ff84: u16;
  let mut in_stack_0000ff8a: u16;
  let mut in_stack_0000ff8e: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff9a: u16;
  let mut in_stack_0000ff9e: u16;
  let mut piVar6: *mut i16;
  let mut uVar7: u16;
  let mut piVar8: *mut i16;
  let mut uVar9: u16;
  let mut iStack36: i16;
  u8 local_16 [0x2];
  let mut iStack20: i16;
  let mut iStack16: i16;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut local_6: i16;
  let mut local_4: i16;
  astruct_778 *iVar3;
  let mut piVar1: *mut i16;
  let mut in_stack_0000ffc6: u32;
  let mut uVar16: u16;
  code **fn_ptr_1;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(param_4);
  if (PTR_LOOP_1050_5ef8 == (u8 *)((int)&u32_1050_0004 + 0x1)) {
    PTR_LOOP_1050_5ef8 = NULL;
  }
  piVar8 = &local_4;
  uVar9 = SUB42(&DAT_1050_1050,0x0);
  piVar6 = &local_6;
  uVar7 = SUB42(&DAT_1050_1050,0x0);
  puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(piVar6,0x48),in_stack_0000fe60,in_stack_0000ff84
                           ,in_stack_0000ff8a,in_stack_0000ff8e);
  uVar4 = ((u32)paVar3 >> 0x10);
  uStack10._0_2_ = (int)puVar5;
  uStack10 = ((u32)puVar5 >> 0x10);
  pass1_1008_3e94((u16 *)((u32)puVar5 & 0xffff0000 | (u32)((int)uStack10 + 0xe)),(u16 *)CONCAT22(uVar7,piVar6),
                  (char *)CONCAT22(uVar9,piVar8));
  IVar2 = GetSystemMetrics16(SM_CYCAPTION);
  paVar3 = (astruct_57 *)CONCAT22(uVar4,(int)((u32)((long)IVar2 * (long)(int)PTR_LOOP_1050_5ef8) >> 0x10));
  iVar2 = (int)((long)IVar2 * (long)(int)PTR_LOOP_1050_5ef8) + 0xa;
  PTR_LOOP_1050_5ef8 = PTR_LOOP_1050_5ef8 + 0x1;
  uStack14 = CONCAT22(iVar2 + local_4,iVar2 + local_6);
  uVar4 = ((u32)param_4 >> 0x10);
  struct_b_7 = (StructB *)param_4;
  GetWindowRect16((RECT16 *)CONCAT22(0x1050,local_16),(HWND16)struct_b_7->lpvoid_field_0x8);
  hdc = GetDC16(0x0);
  IVar1 = GetDeviceCaps16(VERTRES,hdc);
  ReleaseDC16(hdc,0x0);
  if (IVar1 < iStack16) {
    uStack14 = uStack14 & 0xffff0000 | (u32)((iStack20 - (iStack16 - IVar1)) + 0x1);
  }
  SetWindowPos16(0x1,0x0,0x0,(INT16)uStack14,(INT16)(uStack14 >> 0x10),0x0,(HWND16)struct_b_7->lpvoid_field_0x8);
  _param_3 = (u8 **)CONCAT22(param_2,0x3);
  uVar9 = 0x1010;
  puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,_param_3,in_stack_0000fe70,in_stack_0000ff94,in_stack_0000ff9a,
                           in_stack_0000ff9e);
  uVar7 = ((u32)puVar5 >> 0x10);
  iStack36 = 0x0;
  while (iVar3 = (astruct_778 *)(iStack36 * 0x2), (&iVar3[0x52].field_0x0 + (int)puVar5) != 0x0) {
    _param_3 = (u8 **)((u32)_param_3 & 0xffff0000);
    uVar9 = SUB42(s_tile2_bmp_1050_1538,0x0);
    HVar2 = GetDlgItem16(*(INT16 *)(&iVar3[0x52].field_0x0 + (int)puVar5),(HWND16)struct_b_7->lpvoid_field_0x8);
    *(HWND16 *)(&iVar3[0x4a].field_0x0 + (int)&struct_b_7->field0_0x0) = HVar2;
    iStack36 += 0x1;
    piVar1 = (i16 *)&struct_b_7[0xe].field8_0x10;
    *piVar1 = *piVar1 + 0x1;
  }
  fn_ptr_1 = (code **)((int)(u32)param_4 + 0x6c);
  (**fn_ptr_1)(uVar9,param_4,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn enable_window_1038_9cec(u8 *param_1,param_2: *mut astruct_903,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16)

{
  let mut piVar1: *mut i16;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar4;
  let mut unaff_SI: u16;
  u32 *puVar5;
  let mut in_stack_0000fe90: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffbe: u16;
  let mut enable: bool;
  HWND16 HVar6;
  let mut iStack12: i16;
  astruct_905 *iVar2;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  if (param_5 == 0xeb) {
    pass1_1040_b54a(param_1,param_2,param_3,CONCAT22(0xeb,param_4));
    puVar5 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x3),in_stack_0000fe90,
                             in_stack_0000ffb4,in_stack_0000ffba,in_stack_0000ffbe);
    iVar3 = (int)puVar5 + 0xa4;
    uVar2 = ((u32)puVar5 >> 0x10);
    iStack12 = 0x0;
    while (iVar2 = (astruct_905 *)(iStack12 * 0x2), (iVar2 + iVar3) != 0x0) {
      HVar6 = GetDlgItem16(*(INT16 *)(iVar2 + iVar3),*(HWND16 *)((int)param_2 + 0x6));
      *(HWND16 *)(iVar2 + (int)param_2 + 0x94) = HVar6;
      iStack12 += 0x1;
      piVar1 = (i16 *)((int)param_2 + 0x128);
      *piVar1 = *piVar1 + 0x1;
    }
  }
  else {
    if (param_5 == 0xf8) {
      HVar6 = GetDlgItem16(0x17d8,*(HWND16 *)((int)param_2 + 0x6));
      enable = 0x1;
    }
    else {
      if (param_5 != 0x17d8) {
        pass1_1040_b54a(param_1,param_2,param_3,CONCAT22(param_5,param_4));
        return;
      }
      SetWindowPos16(0x6,0xed,0x237,0x0,0x0,0x0,*(HWND16 *)((int)param_2 + 0x6));
      enable = (BOOL16)s_tile2_bmp_1050_1538;
      GetDlgItem16(0x17d8,*(HWND16 *)((int)param_2 + 0x6));
      HVar6 = 0x0;
    }
    EnableWindow16(enable,HVar6);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn draw_op_1038_9dcc(astruct_10 *in_struct_1,mut param_2: i16,mut param_3: u16 ,u16 in_hdc_param_4,mut param_5: u16 )

{
  let mut bVar1: bool;
  HBRUSH16 local_brush_handle;
  astruct_10 *struct10_5;
  astruct_10 *struct10_5_hi;
  let mut uVar3: u32;
  let mut uStack14: u16;
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  astruct_749 *iVar2;

  struct10_5_hi = (astruct_10 *)((u32)in_struct_1 >> 0x10);
  struct10_5 = (astruct_10 *)in_struct_1;
  if (struct10_5->brush_handle_field_0x8e == 0x0) {
    local_brush_handle = CreateSolidBrush16(WHITE_BRUSH);
    struct10_5->brush_handle_field_0x8e = local_brush_handle;
  }
  if (_u16_1050_5b64 == 0x0) {
    uVar3 = pass1_1008_4d72((u32)((int)_PTR_LOOP_1050_4230 + 0xe));
    uVar2 = (uVar3 >> 0x10);
    iVar2 = (astruct_749 *)uVar3;
    _u16_1050_5b64 = (u32)CONCAT12(iVar2->field_0x94,CONCAT11(iVar2->field_0x95,iVar2->field_0x96));
    u16_1050_5b68 = CONCAT11(iVar2->field_0x3e5,iVar2->field_0x3e6);
    u16_1050_5b6a = iVar2->field996_0x3e4;
  }
  if (0x5 < param_3) {
    if (param_3 != 0x6) {
      return;
    }
    bVar1 = false;
    for (uStack14 = 0x0; puVar1 = &struct10_5->field295_0x128, uStack14 <= *puVar1 && *puVar1 != uStack14;
        uStack14 += 0x1) {
      if ((&struct10_5->field_0x94 + uStack14 * 0x2) == param_2) {
        bVar1 = true;
        break;
      }
    }
    if (bVar1) {
      u16_1050_5b64 = u16_1050_5b68;
      u16_1050_5b66 = u16_1050_5b6a;
    }
  }
  SetTextColor16(CONCAT22(u16_1050_5b66,u16_1050_5b64),in_hdc_param_4);
  SetBkColor16(0x1000000,in_hdc_param_4);
  return;
}



StructD * pass1_1038_9ed4(StructD *param_1,u8 param_2)

{
  let mut in_stack_0000ffda: u16;

  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_57 * pass1_1038_9f76(param_1: *mut astruct_57,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfba,param_5);
  param_1->field0_0x0 = 0xa0b6;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_9fa4(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xa0b6;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn show_win_1038_9fd0(StructB *param_1)

{
  dialog_ui_fn_1040_78e2(param_1);
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(0x5,*(HWND16 *)((int)param_1 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 call_fn_ptr_1038_9ffa(mut param_1: u32,astruct_733 *pstruct_param_2,mut param_3: u16 )

{
  code **ppcVar1;
  astruct_43 *struct_3;
  astruct_43 *struct_2;
  u32 *puStack8;
  HDC16 hdc;
  let mut var_5: u16;

  hdc = GetDC16(pstruct_param_2->hwnd_0x6);
  struct_2._0_2_ = FUN_1010_830a(hdc,param_1,(int)s_tile2_bmp_1050_1538,_u16_1050_14cc,0x3);
  puStack8 = (u32 *)CONCAT22((int)param_1,struct_2._0_2_);
  struct_3 = (astruct_43 *)*puStack8;
  ppcVar1 = (code **)&struct_3->fn_ptr_field_0x8;
  (**ppcVar1)(0x1010,struct_2._0_2_,(int)param_1,&hdc,(int)&DAT_1050_1050);
  ppcVar1 = (code **)&struct_3->fn_ptr_field_0x4;
  (**ppcVar1)(0x1010,puStack8,0x50005,&hdc,(int)&DAT_1050_1050);
  ppcVar1 = (code **)&struct_3->fn_ptr_field_0xc;
  (**ppcVar1)(0x1010,puStack8,&hdc,(int)&DAT_1050_1050);
  ReleaseDC16(hdc,pstruct_param_2->hwnd_0x6);
  return 0x0;
}
pub fn destroy_window_1038_a072(param_1: *mut astruct_880,mut param_2: u16 ,mut param_3: i16)

{
  if (param_3 != 0x0) {
    DestroyWindow16(param_1->field6_0x6);
  }
  return;
}
pub fn FUN_1038_a08c(void)

{
  return;
}



StructD * pass1_1038_a090(StructD *param_1,u8 param_2)

{
  pass1_1038_9fa4(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_57 * pass1_1038_a122(param_1: *mut astruct_57,param_2: *mut astruct_57,mut param_3: u16 ,mut param_4: u32,mut param_5: u32)

{
  get_sys_metrics_1040_7728
            ((astruct_57 *)CONCAT22(param_2,param_1),param_3,param_4,param_5,(param_5 >> 0x10));
  (param_1 + 0x1)->field0_0x0 = 0x0;
  ((astruct_57 *)CONCAT22(param_2,param_1))->field0_0x0 = 0xa2d0;
  param_1->field1_0x2 = &u16_1050_1038;
  return (astruct_57 *)CONCAT22(param_2,param_1);
}
pub fn pass1_1038_a156(StructD *param_1)

{
  param_1->address_offset_field_0x0 = 0xa2d0;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn pass1_1038_a174(mut param_1: u32,mut param_2: i16)

{
  if (param_2 == 0x1) {
    ((int)param_1 + 0x8e) = 0x0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_win_ui_op_1038_a18c(param_1: *mut astruct_57,StructB *param_2,mut param_3: u16 )

{
  code **ppcVar1;
  u8 *puVar2;
  let mut uVar3: u16;
  INT16 IVar4;
  let mut uVar5: u32;
  let mut puVar6: *mut u16;
  let mut in_stack_0000fe56: u16;
  let mut in_stack_0000fe5c: u16;
  let mut in_stack_0000ff7a: u16;
  let mut in_stack_0000ff80: u16;
  let mut in_stack_0000ff84: u16;
  let mut in_stack_0000ff86: u16;
  let mut in_stack_0000ff8a: u16;
  let mut piVar7: *mut i16;
  u8 uVar8;
  u8 uVar9;
  i16 local_2c [0x2];
  let mut iStack40: i16;
  u32 *puStack36;
  let mut iStack32: i16;
  let mut uStack30: u16;
  let mut local_1c: i16;
  u8 local_1a [0x2];
  let mut uStack24: u32;
  u32 *puStack20;
  let mut local_10: i16;
  let mut local_e: bool;
  u8 local_c [0x6];
  u32 *puStack6;

  puStack6 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x27),in_stack_0000fe5c,
                             in_stack_0000ff80,in_stack_0000ff86,in_stack_0000ff8a);
  uVar5 = (u32)param_1 & 0xffff0000;
  puVar6 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_c));
  uVar5 = uVar5 & 0xffff0000 | (u32)puVar6 >> 0x10;
  pass1_1008_3f62((u16 *)CONCAT22(0x1050,local_c),
                  (u16 *)((u32)puStack6 & 0xffff0000 | (u32)((int)puStack6 + 0x52)));
  puVar2 = local_c;
  pass1_1008_3e94((u16 *)CONCAT22(0x1050,puVar2),(u16 *)CONCAT22(0x1050,&local_10),(char *)CONCAT22(0x1050,&local_e));
  uVar3 = FUN_1010_830a(puVar2,uVar5,0x1008,_u16_1050_14cc,0x1c0);
  puStack20 = (u32 *)CONCAT22((int)uVar5,uVar3);
  uStack24 = pass1_1008_4772((astruct_76 *)CONCAT22((int)uVar5,uVar3));
  puVar2 = local_1a;
  piVar7 = &local_1c;
  uVar8 = 0x50;
  uVar9 = 0x10;
  puStack36 = mixed_1010_20ba((astruct_57 *)(uVar5 & 0xffff0000 | uStack24 >> 0x10),_u16_1050_0ed0,
                              (u8 **)CONCAT22(piVar7,0x48),in_stack_0000fe56,in_stack_0000ff7a,in_stack_0000ff80,
                              in_stack_0000ff84);
  pass1_1008_3e94((u16 *)((u32)puStack36 & 0xffff0000 | (u32)((int)puStack36 + 0xe)),
                  (u16 *)CONCAT13(uVar9,CONCAT12(uVar8,piVar7)),(char *)CONCAT22(0x1050,puVar2));
  uVar3 = ((u32)puStack36 >> 0x10);
  uStack30 = ((int)puStack36 + 0xa);
  iStack32 = ((int)puStack36 + 0xc);
  local_10 += (iStack32 * 0xa) / 0x258 + ((int)uStack24 + 0x8) + local_1c;
  GetWindowRect16((RECT16 *)CONCAT22(0x1050,local_2c),*(HWND16 *)((int)param_2 + 0x6));
  IVar4 = GetSystemMetrics16(SM_CXSCREEN);
  local_e = (IVar4 - (iStack40 - local_2c[0])) / 0x2;
  move_win_1040_826c(param_2,local_10,local_e);
  if (puStack20 != NULL) {
    uVar3 = ((u32)puStack20 >> 0x10);
    ppcVar1 = (code **)*puStack20;
    (**ppcVar1)((int)&PTR_LOOP_1050_1040,(int)puStack20,uVar3,0x1,uVar3);
  }
  return;
}



u16 FUN_1038_a2a4(void)

{
  return 0x0;
}



StructD * pass1_1038_a2aa(StructD *param_1,u8 param_2)

{
  pass1_1038_a156(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1038_a33c(param_1: *mut u16,mut param_2: u16 )

{
  astruct_57 *paVar1;

  paVar1 = (astruct_57 *)((u32)param_1 >> 0x10);
  pass1_1038_a122((astruct_57 *)param_1,paVar1,0x1,0x0,CONCAT22(param_2,0xfc7));
  *param_1 = 0xa428;
  ((astruct_57 *)param_1)->field1_0x2 = &u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_a36a(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xa428;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn show_win_1038_a396(mut param_1: u16 ,mut param_2: u16 ,StructB *param_3)

{
  let mut in_register_0000000a: u16;
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut in_stack_0000ffaa: u16;

  uVar1 = CONCAT22(in_register_0000000a,param_2);
  dialog_ui_fn_1040_78e2(param_3);
  unk_win_ui_op_1038_a18c(uVar1,param_3,in_stack_0000ffaa);
  win_1008_5c7c(param_1,uVar1,_u16_1050_02a0,0x10001);
  uVar2 = ((u32)param_3 >> 0x10);
  ((int)param_3 + 0x8c) = param_1;
  ShowWindow16(0x5,*(HWND16 *)((int)param_3 + 0x6));
  return;
}
pub fn destroy_win_1038_a3d2(mut param_1: u32)

{
  WORD hwnd;

  hwnd = GetWindowWord16(-0x8,*(HWND16 *)((int)param_1 + 0x6));
  PostMessage16(0x0,0x105,0x111,hwnd);
  destroy_win_1040_7b98(param_1);
  return;
}



StructD * pass1_1038_a402(StructD *param_1,u8 param_2)

{
  pass1_1038_a36a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1038_a494(param_1: *mut u16,mut param_2: u16 )

{
  astruct_57 *paVar1;

  paVar1 = (astruct_57 *)((u32)param_1 >> 0x10);
  pass1_1038_a122((astruct_57 *)param_1,paVar1,0x1,0x0,CONCAT22(param_2,0xfc8));
  *param_1 = 0xa62e;
  ((astruct_57 *)param_1)->field1_0x2 = &u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_a4c2(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xa62e;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1038_a4ee(mut param_1: u16 ,mut param_2: u16 ,StructB *struct_b_param_1)

{
  let mut lp_string: u32;
  HWND16 hwnd;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  let mut uVar2: u16;
  StructB *struct_b_1;
  let mut uVar3: u16;
  u32 *puVar4;
  LRESULT LVar5;
  let mut in_stack_0000fe94: u16;
  let mut in_stack_0000ff9e: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffec: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  dialog_ui_fn_1040_78e2(struct_b_param_1);
  win_1008_5c7c(param_1,paVar1,_u16_1050_02a0,0x20001);
  uVar3 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_1 = (StructB *)struct_b_param_1;
  (struct_b_1 + 0x7)->field0_0x0 = param_1;
  puVar4 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffec,0x2),in_stack_0000fe94,
                           in_stack_0000ffb8,in_stack_0000ffbe,in_stack_0000ffc2);
  uVar2 = ((u32)paVar1 >> 0x10);
  lp_string = (u32)((int)puVar4 + 0x6c);
  hwnd = GetDlgItem16(0x114,(HWND16)struct_b_1->lpvoid_field_0x8);
  SetWindowText16(lp_string,hwnd);
  SetFocus16(hwnd);
  LVar5 = SendMessage16(-0x10000,0x0,0x401,hwnd);
  unk_win_ui_op_1038_a18c(CONCAT22(uVar2,(int)((u32)LVar5 >> 0x10)),struct_b_param_1,in_stack_0000ff9e);
  ShowWindow16(0x5,(HWND16)struct_b_1->lpvoid_field_0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1038_a584(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16,mut param_4: i16)

{
  HWND16 hwnd;
  let mut uVar1: u16;
  WORD hwnd_00;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar2;
  astruct_486 *paVar3;
  let mut in_stack_0000fe4c: u16;
  let mut in_stack_0000ff70: u16;
  let mut in_stack_0000ff76: u16;
  let mut in_stack_0000ff7a: u16;
  u8 *puVar4;
  let mut uVar5: u16;
  u8 local_52 [0x50];

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  if (param_4 != 0x0) {
    hwnd = GetDlgItem16(0x114,*(HWND16 *)(param_2 + 0x6));
    GetWindowText16(0x50,CONCAT22(0x1050,local_52),hwnd);
    uVar1 = str_op_1000_3da4((char *)CONCAT22(0x1050,local_52));
    if (uVar1 != 0x0) {
      puVar4 = local_52;
      uVar5 = SUB42(&DAT_1050_1050,0x0);
      paVar3 = (astruct_486 *)
               mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(puVar4,0x2),in_stack_0000fe4c,
                               in_stack_0000ff70,in_stack_0000ff76,in_stack_0000ff7a);
      pass1_1010_6006(((u32)paVar3 >> 0x10),paVar3,(char *)CONCAT22(uVar5,puVar4));
      hwnd_00 = GetWindowWord16(-0x8,*(HWND16 *)(param_2 + 0x6));
      PostMessage16(0x0,0x105,0x111,hwnd_00);
      destroy_win_1040_7b98(CONCAT22(param_3,param_2));
    }
  }
  return;
}



StructD * pass1_1038_a608(StructD *param_1,u8 param_2)

{
  pass1_1038_a4c2(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1038_a69a(param_1: *mut u16,mut param_2: u16 )

{
  astruct_57 *paVar1;

  paVar1 = (astruct_57 *)((u32)param_1 >> 0x10);
  pass1_1038_a122((astruct_57 *)param_1,paVar1,0x1,0x0,CONCAT22(param_2,0xfc9));
  *param_1 = 0xa832;
  ((astruct_57 *)param_1)->field1_0x2 = &u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_a6c8(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xa832;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1038_a6f4(mut param_1: u16 ,StructB *param_2)

{
  let mut lp_string: u32;
  HWND16 hwnd;
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar2;
  let mut uVar3: u32;
  StructB *struct_b_3;
  let mut uVar5: u16;
  u32 *puVar6;
  LRESULT LVar7;
  let mut in_stack_0000fe94: u16;
  let mut in_stack_0000ff9e: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffec: u16;
  let mut uVar4: u16;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(param_2);
  puVar6 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffec,0x2),in_stack_0000fe94,
                           in_stack_0000ffb8,in_stack_0000ffbe,in_stack_0000ffc2);
  uVar4 = ((u32)paVar2 >> 0x10);
  lp_string = (u32)((int)puVar6 + 0x68);
  uVar5 = ((u32)param_2 >> 0x10);
  struct_b_3 = (StructB *)param_2;
  hwnd = GetDlgItem16(0x115,(HWND16)struct_b_3->lpvoid_field_0x8);
  SetWindowText16(lp_string,hwnd);
  SetFocus16(hwnd);
  LVar7 = SendMessage16(-0x10000,0x0,0x401,hwnd);
  uVar3 = CONCAT22(uVar4,(int)((u32)LVar7 >> 0x10));
  uVar1 = LVar7;
  unk_win_ui_op_1038_a18c(uVar3,param_2,in_stack_0000ff9e);
  win_1008_5c7c(uVar1,uVar3,_u16_1050_02a0,0x30001);
  (struct_b_3 + 0x7)->field0_0x0 = uVar1;
  ShowWindow16(0x5,(HWND16)struct_b_3->lpvoid_field_0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1038_a788(mut param_1: u16 ,mut param_2: u32,mut param_3: i16)

{
  HWND16 hwnd;
  let mut uVar1: u16;
  WORD hwnd_00;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar2;
  let mut uVar3: u16;
  let mut pUVar2: *mut u16;
  let mut in_stack_0000fe4c: u16;
  let mut in_stack_0000ff70: u16;
  let mut in_stack_0000ff76: u16;
  let mut in_stack_0000ff7a: u16;
  u8 *puVar4;
  let mut uVar5: u16;
  u8 local_52 [0x50];
  u8 *puVar3;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  if (param_3 != 0x0) {
    uVar3 = (param_2 >> 0x10);
    hwnd = GetDlgItem16(0x115,*(HWND16 *)((int)param_2 + 0x6));
    GetWindowText16(0x50,CONCAT22(0x1050,local_52),hwnd);
    uVar1 = str_op_1000_3da4((char *)CONCAT22(0x1050,local_52));
    if (uVar1 != 0x0) {
      puVar4 = local_52;
      uVar5 = SUB42(&DAT_1050_1050,0x0);
      pUVar2 = (u16 *)
               mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(puVar4,0x2),in_stack_0000fe4c,
                               in_stack_0000ff70,in_stack_0000ff76,in_stack_0000ff7a);
      pass1_1010_5fd8(((u32)pUVar2 >> 0x10),(astruct_485 *)pUVar2,(char *)CONCAT22(uVar5,puVar4));
      hwnd_00 = GetWindowWord16(-0x8,*(HWND16 *)((int)param_2 + 0x6));
      PostMessage16(0x0,0x105,0x111,hwnd_00);
      destroy_win_1040_7b98(param_2);
    }
  }
  return;
}



StructD * pass1_1038_a80c(StructD *param_1,u8 param_2)

{
  pass1_1038_a6c8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1038_a89e(param_1: *mut u16,mut param_2: u16 )

{
  astruct_57 *paVar1;

  paVar1 = (astruct_57 *)((u32)param_1 >> 0x10);
  pass1_1038_a122((astruct_57 *)param_1,paVar1,0x1,0x0,CONCAT22(param_2,0xfca));
  *param_1 = 0xab16;
  ((astruct_57 *)param_1)->field1_0x2 = &u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_a8cc(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xab16;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}
pub fn enable_win_1038_a8f8(StructC *param_1,mut param_2: u16 ,TwoWords param_3)

{
  HWND16 hwnd;
  let mut enable: bool;

  if (param_3.b_0x2 == 0x116) {
    SendDlgItemMessage16(0x0,0x1,0x401,0x11a,*(HWND16 *)((int)param_1 + 0x6));
    hwnd = GetDlgItem16(0x11a,*(HWND16 *)((int)param_1 + 0x6));
    enable = 0x0;
  }
  else {
    if ((param_3.b_0x2 == 0x116) || (0x2 < param_3.b_0x2 - 0x117)) {
      post_win_msg_1040_7b3c(param_1,param_2,param_3,param_3.b_0x2);
      return;
    }
    hwnd = GetDlgItem16(0x11a,*(HWND16 *)((int)param_1 + 0x6));
    enable = 0x1;
  }
  EnableWindow16(enable,hwnd);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1038_a972(StructB *struct_b_param_1)

{
  HWND16 hwnd;
  let mut BVar1: bool;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut uVar2: u32;
  StructB *struct_b_3;
  let mut uVar4: u16;
  LRESULT LVar5;
  let mut in_stack_0000ffaa: u16;

  uVar3 = ((u32)in_EDX >> 0x10);
  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar4 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_3 = (StructB *)struct_b_param_1;
  SendDlgItemMessage16(0x0,0x1,0x401,0x116,(HWND16)struct_b_3->lpvoid_field_0x8);
  LVar5 = SendDlgItemMessage16(0x0,0x1,0x401,0x11a,(HWND16)struct_b_3->lpvoid_field_0x8);
  uVar2 = CONCAT22(uVar3,(int)((u32)LVar5 >> 0x10));
  hwnd = GetDlgItem16(0x11a,(HWND16)struct_b_3->lpvoid_field_0x8);
  BVar1 = EnableWindow16(0x0,hwnd);
  win_1008_5c7c(BVar1,uVar2,_u16_1050_02a0,0x40001);
  (struct_b_3 + 0x7)->field0_0x0 = BVar1;
  unk_win_ui_op_1038_a18c(uVar2,struct_b_param_1,in_stack_0000ffaa);
  ShowWindow16(0x5,(HWND16)struct_b_3->lpvoid_field_0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_sys_op_1038_a9fa(mut param_1: u32,mut param_2: i16)

{
  WORD hwnd;
  astruct_57 *in_EDX;
  let mut iVar1: i16;
  let mut uVar2: u16;
  u32 *puVar3;
  LRESULT LVar4;
  let mut in_stack_0000fe98: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc6: u16;
  let mut in_stack_0000fff0: u16;

  if (param_2 != 0x0) {
    puVar3 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff0,0x2),in_stack_0000fe98,
                             in_stack_0000ffbc,in_stack_0000ffc2,in_stack_0000ffc6);
    uVar2 = (param_1 >> 0x10);
    iVar1 = (int)param_1;
    LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x116,*(HWND16 *)(iVar1 + 0x6));
    if ((((u32)LVar4 >> 0x10) | LVar4) == 0x0) {
      LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x117,*(HWND16 *)(iVar1 + 0x6));
      if ((((u32)LVar4 >> 0x10) | LVar4) == 0x0) {
        LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x118,*(HWND16 *)(iVar1 + 0x6));
        if ((((u32)LVar4 >> 0x10) | LVar4) == 0x0) {
          LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x119,*(HWND16 *)(iVar1 + 0x6));
          if ((((u32)LVar4 >> 0x10) | LVar4) != 0x0) {
            PTR_LOOP_1050_13ae = (u8 *)&u32_1050_0004;
          }
        }
        else {
          PTR_LOOP_1050_13ae = (u8 *)((int)&u16_1050_0002 + 0x1);
        }
      }
      else {
        PTR_LOOP_1050_13ae = (u8 *)&u16_1050_0002;
      }
    }
    else {
      PTR_LOOP_1050_13ae = (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1);
    }
    LVar4 = SendDlgItemMessage16(0x0,0x0,0x400,0x11a,*(HWND16 *)(iVar1 + 0x6));
    ((int)puVar3 + 0x82) = (int)LVar4;
    hwnd = GetWindowWord16(-0x8,*(HWND16 *)(iVar1 + 0x6));
    PostMessage16(0x0,0x105,0x111,hwnd);
    destroy_win_1040_7b98(param_1);
  }
  return;
}



StructD * pass1_1038_aaf0(StructD *param_1,u8 param_2)

{
  pass1_1038_a8cc(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_57 * pass1_1038_ab82(param_1: *mut astruct_57,mut param_2: u16 )

{
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfd3,param_2);
  param_1->field0_0x0 = 0xad72;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_abb0(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xad72;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn set_win_pos_1038_abdc(param_1: *mut astruct_940)

{
  HWND16 hwnd;
  astruct_940 *iVar1;
  let mut uVar1: u16;
  let mut in_stack_0000fff0: i16;
  let mut local_a: i16;
  let mut iStack8: i16;
  let mut iStack6: i16;
  let mut iStack4: i16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_940 *)param_1;
  GetWindowRect16((RECT16 *)CONCAT22(0x1050,&local_a),iVar1->field6_0x6);
  hwnd = GetDlgItem16(0xfd7,iVar1->field6_0x6);
  GetWindowRect16((RECT16 *)CONCAT22(0x1050,&stack0xffee),hwnd);
  iStack6 -= local_a;
  iStack4 = (in_stack_0000fff0 - iStack8) + -0x2;
  SetWindowPos16(0x6,iStack4,iStack6,0x0,0x0,0x0,iVar1->field6_0x6);
  return;
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Unable to use type for symbol uVar5
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_win_ui_op_1038_ac38(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,HDC16 hdc_param_5)

{
  let mut IVar1: i16;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut uVar6: u32;
  UCHAR uVar4;
  astruct_46 *iVar1;
  astruct_786 *iVar2;
  let mut uVar2: u16;
  let mut uVar5: u16;
  let mut uVar1: u16;

  GetStockObject16(BLACK_BRUSH);
  if (_u16_1050_5b78 == 0x0) {
    uVar6 = pass1_1008_4d72((u32)((int)_PTR_LOOP_1050_4230 + 0xe));
    uVar1 = (uVar6 >> 0x10);
    iVar2 = (astruct_786 *)uVar6;
    _u16_1050_5b6c = (u32)CONCAT12(iVar2->field_0x3ec,CONCAT11(iVar2->field_0x3ed,iVar2->field_0x3ee));
    _u16_1050_5b70 = (u32)CONCAT12(iVar2->field_0x3e4,CONCAT11(iVar2->field_0x3e5,iVar2->field_0x3e6));
    _u16_1050_5b74 = (u32)CONCAT12(iVar2->field_0x3f8,CONCAT11(iVar2->field_0x3f9,iVar2->field_0x3fa));
    _u16_1050_5b78 = (u32)CONCAT12(iVar2->field_0x94,CONCAT11(iVar2->field_0x95,iVar2->field_0x96));
  }
  if (param_4 < 0x4) {//
LAB_1038_acf0:
    IVar1 = GetDlgCtrlID16(param_3);
    if (IVar1 == 0xfd4) {
      uVar2 = _u16_1050_5b70;
      uVar5 = (_u16_1050_5b70 >> 0x10);
      goto LAB_1038_ad0e;
    }
    if (IVar1 != 0xfd5) {
      if (IVar1 == 0xfd6) {
        uVar2 = _u16_1050_5b6c;
        uVar5 = (_u16_1050_5b6c >> 0x10);
        goto LAB_1038_ad0e;
      }
      if (IVar1 == 0xfd7) {
        uVar2 = _u16_1050_5b74;
        uVar5 = (_u16_1050_5b74 >> 0x10);
        goto LAB_1038_ad0e;
      }
    }
  }
  else if (param_4 != 0x4) {
    if ((param_4 == 0x4) || (0x1 < param_4 - 0x5)) {
      return;
    }
    goto LAB_1038_acf0;
  }
  uVar2 = _u16_1050_5b78;
  uVar5 = (_u16_1050_5b78 >> 0x10);//
LAB_1038_ad0e:
  SetTextColor16(CONCAT22(uVar5,uVar2),hdc_param_5);
  SetBkColor16(0x1000000,hdc_param_5);
  return;
}



StructD * pass1_1038_ad4c(StructD *param_1,u8 param_2)

{
  pass1_1038_abb0(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1038_adde(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  pass1_1038_9b72((astruct_57 *)param_1,(astruct_57 *)param_2,param_3,param_4);
  CONCAT22(param_2,param_1) = 0xae4e;
  (param_1 + 0x2) = (int)&u16_1050_1038;
  return (u16 *)CONCAT22(param_2,param_1);
}
pub fn pass1_1038_ae08(StructD *param_1)

{
  let mut in_stack_0000ffda: u16;

  param_1->address_offset_field_0x0 = 0xae4e;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  unk_draw_op_1040_b0f8(in_stack_0000ffda,param_1);
  return;
}



StructD * pass1_1038_ae28(StructD *param_1,u8 param_2)

{
  pass1_1038_ae08(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

StructD * pass1_1038_aeca(StructD *param_1)

{
  let mut uVar1: u16;
  let mut addr_offset_b6: u16;
  let mut seg_addr_180: u16;
  u8 u8_array_5c [0x5a];

  uVar1 = ((u32)param_1 >> 0x10);
  ((int)param_1 + 0xac) = 0x0;
  ((int)param_1 + 0xae) = 0x0;
  if (_PTR_LOOP_1050_5b7c == NULL) {
    _PTR_LOOP_1050_5b7c = param_1;
  }
  pass1_1000_4906(param_1,NULL,0xac);
  unk_draw_op_1008_80ee((astruct_23 *)CONCAT22(0x1050,u8_array_5c));
  unk_win_ui_op_1040_9854((astruct_787 *)CONCAT22(0x1050,&addr_offset_b6));
  addr_offset_b6 = 0x389a;
  seg_addr_180 = 0x1008;
  pass1_1008_8168((char *)CONCAT22(0x1050,u8_array_5c));
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_af34(void)

{
  _PTR_LOOP_1050_5b7c = 0x0;
  return;
}



pub fn pass1_1038_af40(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 ,mut param_5: i16) -> u32

{
  code **ppcVar1;
  let mut uVar2: u32;
  u8 *puVar3;
  let mut uVar4: u16;
  u8 *puVar5;
  StructD *pSVar6;
  let mut in_register_0000000a: u16;
  StructD *pSVar7;
  let mut uVar8: u32;
  let mut iVar9: i16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  astruct_57 *paVar12;
  let mut puVar13: *mut u16;
  let mut in_stack_0000fe74: u16;
  let mut in_stack_0000fe86: u16;
  let mut in_stack_0000ff98: u16;
  let mut in_stack_0000ff9e: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffcc: u16;
  let mut in_stack_0000ffd4: u32;
  let mut in_stack_0000ffde: u16;

  paVar12 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  puVar3 = (u8 *)bring_win_to_top_1038_b72e(param_3,param_5);
  iVar9 = (int)param_3;
  uVar10 = (param_3 >> 0x10);
  if (puVar3 != NULL) goto LAB_1038_b61f;
  uVar11 = SUB42(&u16_1050_1038,0x0);
  PTR_LOOP_1050_5b82 = puVar3;
  switch(param_5) {
  case 0x1:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x8e,paVar12);
    if ((paVar12 | param_1) == 0x0) {//
LAB_1038_afa0:
      uVar11 = 0x1000;
      param_1 = 0x0;
      pSVar6 = NULL;
    }
    else {
      paVar12 = pass1_1038_9f76((astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
      pSVar6 = (StructD *)((u32)paVar12 >> 0x10);
      param_1 = paVar12;
    }
    break;
  case 0x2:
    mem_op_1000_179c(0x96,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_181c(pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x3:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x92,paVar12);
    puVar5 = (u8 *)(paVar12 | param_1);
    if (puVar5 == NULL) goto LAB_1038_afa0;
    paVar12 = pass1_1038_e99a(puVar5,(astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (StructD *)((u32)paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x4:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x92,paVar12);
    puVar5 = (u8 *)(paVar12 | param_1);
    if (puVar5 == NULL) goto LAB_1038_afa0;
    paVar12 = pass1_1038_c7b8(puVar5,(astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (StructD *)((u32)paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x5:
    mem_op_1000_179c(0x96,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_23ea(pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4,in_stack_0000ffd4);
    break;
  case 0x6:
    mem_op_1000_179c(0x92,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    paVar12 = pass1_1040_06e8(pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (StructD *)((u32)paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x7:
    mem_op_1000_179c(0x9c,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_4068((u8 *)pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x8:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    pass1_1038_b772((u8 *)pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),param_4);
    break;
  case 0x9:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x8e,paVar12);
    if ((paVar12 | param_1) == 0x0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_e140((astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (StructD *)((u32)paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0xa:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x90,paVar12);
    if ((paVar12 | param_1) == 0x0) goto LAB_1038_afa0;
    puVar13 = pass1_1038_a33c((u16 *)CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (StructD *)((u32)puVar13 >> 0x10);
    param_1 = puVar13;
    break;
  case 0xb:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x90,paVar12);
    if ((paVar12 | param_1) == 0x0) goto LAB_1038_afa0;
    puVar13 = pass1_1038_a494((u16 *)CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (StructD *)((u32)puVar13 >> 0x10);
    param_1 = puVar13;
    break;
  case 0xc:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x90,paVar12);
    if ((paVar12 | param_1) == 0x0) goto LAB_1038_afa0;
    puVar13 = pass1_1038_a69a((u16 *)CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (StructD *)((u32)puVar13 >> 0x10);
    param_1 = puVar13;
    break;
  case 0xd:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x90,paVar12);
    if ((paVar12 | param_1) == 0x0) goto LAB_1038_afa0;
    puVar13 = pass1_1038_a89e((u16 *)CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (StructD *)((u32)puVar13 >> 0x10);
    param_1 = puVar13;
    break;
  case 0xe:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x94,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    pass1_1038_e69a(pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0xf:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x94,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    pass1_1038_cd06(pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x10:
    mem_op_1000_179c(0x92,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    paVar12 = pass1_1040_0bfc(pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (StructD *)((u32)paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x11:
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_0e1c(pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,param_4);
    break;
  case 0x12:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    paVar12 = pass1_1038_d756(pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (StructD *)((u32)paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x13:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x92,paVar12);
    puVar5 = (u8 *)(paVar12 | param_1);
    if (puVar5 == NULL) goto LAB_1038_afa0;
    paVar12 = pass1_1038_cad8(puVar5,(astruct_57 *)CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (StructD *)((u32)paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x14:
    mem_op_1000_179c(0xaa,paVar12);
    uVar4 = paVar12 | param_1;
    uVar8 = (u32)paVar12 & 0xffff0000 | (u32)uVar4;
    if (uVar4 == 0x0) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_1f5a((astruct_57 *)CONCAT22(paVar12,param_1),param_4,uVar8);
    pSVar6 = (StructD *)uVar8;
    break;
  case 0x15:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x8e,paVar12);
    if ((paVar12 | param_1) == 0x0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_d242((astruct_57 *)CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (StructD *)((u32)paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x16:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    pass1_1038_eeda(pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),param_4);
    break;
  case 0x17:
    mem_op_1000_179c(0x96,paVar12);
    if ((paVar12 | param_1) == 0x0) goto LAB_1038_afa0;
    uVar11 = 0x1018;
    paVar12 = pass1_1018_5e26((astruct_57 *)CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (StructD *)((u32)paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  default:
    goto switchD_1038_b581_caseD_18;
  case 0x19:
    mem_op_1000_179c(0x96,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_1cb4(pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x1a:
    mem_op_1000_179c(0x92,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    paVar12 = pass1_1040_123e(pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    pSVar6 = (StructD *)((u32)paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x1b:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x8e,paVar12);
    if ((paVar12 | param_1) == 0x0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_ab82((astruct_57 *)CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (StructD *)((u32)paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x1c:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x92,paVar12);
    if ((paVar12 | param_1) == 0x0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_e2d0((astruct_57 *)CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (StructD *)((u32)paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x1d:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x92,paVar12);
    if ((paVar12 | param_1) == 0x0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_eb9e((astruct_57 *)CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (StructD *)((u32)paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x1e:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x29e,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    pass1_1038_bddc(pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x1f:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    pass1_1038_c4a2(pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x20:
    mem_op_1000_179c(0x29a,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_2ea2(pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x21:
    mem_op_1000_179c(0xa6,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_3966((u8 *)pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x22:
    mem_op_1000_179c(0x9a,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_34a2(pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),0x0,0x0,0x0,param_4);
    break;
  case 0x23:
    mem_op_1000_179c(0x9c,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_ac84((u8 *)pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),param_4);
    break;
  case 0x25:
    mem_op_1000_179c(0xa0,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_ca16((u8 *)pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),param_4);
    break;
  case 0x26:
    mem_op_1000_179c(0xa2,paVar12);
    uVar4 = paVar12 | param_1;
    pSVar7 = (StructD *)((u32)paVar12 & 0xffff0000 | (u32)uVar4);
    if (uVar4 == 0x0) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_d0f8((astruct_57 *)CONCAT22(paVar12,param_1),param_4,in_stack_0000ffde,pSVar7,in_stack_0000fe74,
                    in_stack_0000fe86,in_stack_0000ff98,in_stack_0000ff9e,in_stack_0000ffa2,in_stack_0000ffaa,
                    in_stack_0000ffb0,in_stack_0000ffb4,in_stack_0000ffcc);
    pSVar6 = (StructD *)pSVar7;
    break;
  case 0x27:
    uVar11 = 0x1000;
    mem_op_1000_179c(0xa0,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    pass1_1038_88f2((astruct_57 *)CONCAT22(paVar12,param_1),param_4);
    break;
  case 0x28:
    mem_op_1000_179c(0x96,paVar12);
    pSVar6 = (StructD *)(paVar12 | param_1);
    if (pSVar6 == NULL) goto LAB_1038_afa0;
    uVar11 = SUB42(&PTR_LOOP_1050_1040,0x0);
    pass1_1040_6402(pSVar6,(astruct_57 *)CONCAT22(paVar12,param_1),param_4);
    break;
  case 0x29:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x98,paVar12);
    uVar4 = paVar12 | param_1;
    if (uVar4 == 0x0) goto LAB_1038_afa0;
    paVar12 = pass1_1038_7d10(uVar4,(astruct_57 *)CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (StructD *)((u32)paVar12 >> 0x10);
    param_1 = paVar12;
    break;
  case 0x2a:
    uVar11 = 0x1000;
    mem_op_1000_179c(0x98,paVar12);
    puVar5 = (u8 *)(paVar12 | param_1);
    if (puVar5 == NULL) goto LAB_1038_afa0;
    paVar12 = pass1_1038_8caa(puVar5,(astruct_57 *)CONCAT22(paVar12,param_1),param_4);
    pSVar6 = (StructD *)((u32)paVar12 >> 0x10);
    param_1 = paVar12;
  }
  (param_5 * 0x4 + iVar9) = param_1;
  *(StructD **)(param_5 * 0x4 + iVar9 + 0x2) = pSVar6;
switchD_1038_b581_caseD_18:
  if (*(i32 *)(param_5 * 0x4 + iVar9) != 0x0) {
    if ((iVar9 + 0xae) != 0x0) {
      uVar2 = (u32)(param_5 * 0x4 + iVar9);
      ((int)uVar2 + 0x6e) = (iVar9 + 0xae);
    }
    (iVar9 + 0xae) = 0x0;
    uVar2 = (u32)(param_5 * 0x4 + iVar9);
    ppcVar1 = (code **)((int)(u32)(u32)(param_5 * 0x4 + iVar9) + 0x8);
    (**ppcVar1)(uVar11,(int)uVar2,(int)((u32)uVar2 >> 0x10));
  }//
LAB_1038_b61f:
  return CONCAT22((param_5 * 0x4 + iVar9 + 0x2),(param_5 * 0x4 + iVar9));
}
pub fn show_win_1038_b634(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uStack4: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((iVar2 + 0xac) == 0x0) {
    (iVar2 + 0xac) = 0x1;
    for (uStack4 = 0x1; uStack4 < 0x2b; uStack4 += 0x1) {
      if (((uStack4 * 0x4 + iVar2 + 0x2) | (uStack4 * 0x4 + iVar2)) != 0x0) {
        uVar1 = (u32)(uStack4 * 0x4 + iVar2);
        ShowWindow16(0x0,*(HWND16 *)((int)uVar1 + 0x6));
      }
    }
  }
  return;
}
pub fn show_win_1038_b68a(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uStack4: u16;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((iVar2 + 0xac) != 0x0) {
    (iVar2 + 0xac) = 0x0;
    for (uStack4 = 0x1; uStack4 < 0x2b; uStack4 += 0x1) {
      if (((uStack4 * 0x4 + iVar2 + 0x2) | (uStack4 * 0x4 + iVar2)) != 0x0) {
        uVar1 = (u32)(uStack4 * 0x4 + iVar2);
        ShowWindow16(0x1,*(HWND16 *)((int)uVar1 + 0x6));
      }
    }
  }
  return;
}
pub fn pass1_1038_b6e0(mut param_1: u32,mut param_2: i16)

{
  let mut uVar1: u32;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut uStack4: u16;

  uStack4 = 0x1;
  while( true ) {
    if (0x2a < uStack4) {
      return;
    }
    uVar3 = (param_1 >> 0x10);
    iVar2 = (int)param_1;
    if ((((uStack4 * 0x4 + iVar2 + 0x2) | (uStack4 * 0x4 + iVar2)) != 0x0) &&
       (uVar1 = (u32)(uStack4 * 0x4 + iVar2), ((int)uVar1 + 0x6) == param_2)) break;
    uStack4 += 0x1;
  }
  (u32)(uStack4 * 0x4 + iVar2) = 0x0;
  return;
}



BOOL16 bring_win_to_top_1038_b72e(mut param_1: u32,mut param_2: i16)

{
  HWND16 hwnd;
  let mut uVar1: u32;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)(param_2 * 0x4 + (int)param_1) != 0x0) {
    uVar1 = (u32)(param_2 * 0x4 + (int)param_1);
    hwnd = *(HWND16 *)((int)uVar1 + 0x6);
    SetFocus16(hwnd);
    BringWindowToTop16(hwnd);
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_b772(u8 *param_1,param_2: *mut astruct_57,mut param_3: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_57 *iVar2;
  let mut unaff_BP: u16;
  astruct_57 *uVar2;
  u32 *puVar2;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;
  u8 **ppuVar3;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x9a,0x0,0xfbf,param_3);
  uVar2 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar2 = (astruct_57 *)param_2;
  (u32)(iVar2 + 0x1) = 0x0;
  (u32)&iVar2[0x1].field2_0x4 = 0x0;
  iVar2[0x1].field4_0x8 = 0x1;
  iVar2[0x1].field5_0xa = 0x0;
  param_2->field0_0x0 = 0xbd70;
  iVar2->field1_0x2 = &u16_1050_1038;
  ppuVar3 = (u8 **)CONCAT22(unaff_BP,0x36);
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,ppuVar3,in_stack_0000fea6,in_stack_0000ffca,in_stack_0000ffd0,
                           in_stack_0000ffd4);
  (iVar2 + 0x1)->field0_0x0 = puVar2;
  iVar2[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  puVar2 = mixed_1010_20ba((astruct_57 *)((u32)paVar1 & 0xffff0000 | (u32)puVar2 >> 0x10),_u16_1050_0ed0,
                           (u8 **)CONCAT22((int)((u32)ppuVar3 >> 0x10),0x6),in_stack_0000fea6,in_stack_0000ffca
                           ,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar2[0x1].field2_0x4 = puVar2;
  iVar2[0x1].field3_0x6 = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_b7f0(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xbd70;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1038_b81c(mut param_1: u16 ,StructB *struct_b_param_1)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  code **ppcVar3;
  let mut uVar4: u16;
  HWND16 HVar5;
  astruct_909 *win_enabled;
  let mut extraout_DX: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar7;
  let mut uVar9: u32;
  StructB *struct_b_8;
  let mut unaff_SI: u16;
  let mut uVar7: u16;
  u32 *puVar10;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut piStack16: *mut i16;
  let mut iStack12: i16;
  let mut iStack10: i16;
  StructB *iVar7;
  let mut uVar8: u16;
  let mut piVar6: *mut i16;

  paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(struct_b_param_1);
  puVar10 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x6),in_stack_0000fe8e,
                            in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  uVar9 = (u32)puVar10 >> 0x10;
  uVar7 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_8 = (StructB *)struct_b_param_1;
  struct_b_8[0x7].lpvoid_field_0x8 = (LPVOID)puVar10;
  struct_b_8[0x7].max_count_field_0x10 = ((u32)puVar10 >> 0x10);
  uVar1 = (u32)&struct_b_8[0x7].lpvoid_field_0x8;
  uVar4 = (int)uVar1 + 0x4e;
  uVar1 &= 0xffff0000;
  piVar6 = (i16 *)(uVar1 | uVar4);
  iStack10 = 0x0;
  for (iStack12 = 0x1a0; extraout_DX = uVar9, iStack12 < 0x1b5; iStack12 += 0x1) {
    if ((iStack10 * 0x2 + uVar4) == iStack12) {
      iStack10 += 0x1;
    }
    else {
      CheckDlgButton16(0x2,iStack12,(HWND16)struct_b_8->lpvoid_field_0x8);
    }
  }
  HVar5 = GetDlgItem16(0xfb1,(HWND16)struct_b_8->lpvoid_field_0x8);
  win_enabled = (astruct_909 *)EnableWindow16(0x0,HVar5);
  uVar2 = (u32)&struct_b_8[0x7].lpvoid_field_0x8;
  ppcVar3 = (code **)((int)*(u32*)&struct_b_8[0x7].lpvoid_field_0x8 + 0x10);
  (**ppcVar3)((int)s_tile2_bmp_1050_1538,(int)uVar2,(int)((u32)uVar2 >> 0x10));
  piStack16 = (i16 *)CONCAT22(extraout_DX,win_enabled);
  move_win_1040_826c(struct_b_param_1,win_enabled->field1_0x2 + -0x2,win_enabled->field2_0x4 + *piStack16 + 0x3);
  ShowWindow16(0x5,(HWND16)struct_b_8->lpvoid_field_0x8);
  pass1_1018_1c9a(*(astruct_263 **)&struct_b_8[0x7].lpvoid_field_0x8,*piVar6);
  HVar5 = GetDlgItem16(*piVar6,(HWND16)struct_b_8->lpvoid_field_0x8);
  SetFocus16(HVar5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_op_1038_b922(StructD *param_1,StructC *param_2,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ) -> u32

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  let mut UVar3: u16;
  HWND16 HVar4;
  let mut BVar5: bool;
  let mut uVar6: u16;
  let mut uVar7: u16;
  StructC *iVar8;
  StructC *uVar8;
  let mut uVar9: u16;
  LRESULT LVar10;
  char *pcVar11;
  astruct_57 *paVar12;
  let mut in_stack_0000fa38: u16;
  let mut in_stack_0000fb5c: u16;
  let mut in_stack_0000fb62: u16;
  let mut in_stack_0000fb66: u16;
  u8 uVar13;
  WORD *pWVar14;
  let mut uVar15: u16;
  u32 *puStack1128;
  char local_464 [0x50];
  WORD local_414 [0x200];
  let mut puStack20: *mut u16;
  u8 *puStack16;
  u32 *puStack14;
  let mut uStack10: u16;
  HWND16 HStack8;
  let mut BStack6: bool;
  let mut uStack4: u16;

  BStack6 = 0x0;
  uStack4 = 0x0;
  iVar8 = (StructC *)param_2;
  uVar8 = (StructC *)((u32)param_2 >> 0x10);
  if (param_4 < 0x1b5) {
    if (param_4 < 0x1a0) {
      if (param_4 != 0x2) goto LAB_1038_bbbf;
    }
    else {
      HStack8 = GetDlgItem16(param_4,iVar8->field6_0x6);
      LVar10 = SendMessage16(0x0,0x0,0x400,HStack8);
      uStack10 = LVar10;
      if (uStack10 == 0x2) {
        BStack6 = 0x0;
        uStack4 = 0x0;
        goto LAB_1038_bc26;
      }
      SendMessage16(0x0,(uStack10 == 0x0),0x401,HStack8);
      UVar3 = IsDlgButtonChecked(param_4,iVar8->field6_0x6);
      if (UVar3 == 0x0) {
        piVar1 = (i16 *)&iVar8->field_0x96;
        *piVar1 = *piVar1 + 0x1;
        if (&iVar8->field_0x96 == 0x1) {
          HVar4 = GetDlgItem16(0xfb1,iVar8->field6_0x6);
          EnableWindow16(0x0,HVar4);
        }
      }
      else {
        piVar1 = (i16 *)&iVar8->field_0x96;
        *piVar1 = *piVar1 + -0x1;
        HVar4 = GetDlgItem16(0xfb1,iVar8->field6_0x6);
        BVar5 = IsWindowEnabled16(HVar4);
        if (BVar5 == 0x0) {
          HVar4 = GetDlgItem16(0xfb1,iVar8->field6_0x6);
          EnableWindow16(0x1,HVar4);
        }
        if (&iVar8->field_0x96 < 0x0) {
          CheckDlgButton16(0x0,iVar8->field145_0x98,iVar8->field6_0x6);
          &iVar8->field_0x96 = 0x0;
        }
        iVar8->field145_0x98 = param_4;
        pass1_1018_1c9a((astruct_263 *)iVar8->field142_0x92,param_4);
        puStack14 = (u32 *)pass1_1018_1e78(iVar8->field142_0x92,-0x1);
        uVar6 = ((u32)puStack14 >> 0x10);
        uVar7 = uVar6 | puStack14;
        if (uVar7 == 0x0) {
          puStack16 = NULL;
        }
        else {
          puStack16 = (u8 *)(puStack14 + 0x1c);
        }
        win_1008_5c7c(puStack16,uVar7,_u16_1050_02a0,CONCAT22(puStack16,0x1));
      }
    }
    BStack6 = 0x1;
    uStack4 = 0x0;
  }
  else {
    if (param_4 == 0xfb1) {
      for (uVar6 = 0x1a0; uVar6 < 0x1b5; uVar6 += 0x1) {
        UVar3 = IsDlgButtonChecked(uVar6,iVar8->field6_0x6);
        if (UVar3 == 0x1) {
          pass1_1008_d818(iVar8->field141_0x8e,uVar6);
          goto LAB_1038_bba2;
        }
      }
    }
    else {
      if (param_4 != 0xfbe) goto LAB_1038_bbbf;
      puStack14 = mixed_1010_20ba((astruct_57 *)param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_5,0x2),
                                  in_stack_0000fa38,in_stack_0000fb5c,in_stack_0000fb62,in_stack_0000fb66);
      uVar9 = ((u32)param_1 >> 0x10);
      puStack16 = PTR_LOOP_1050_13ae;
      if (PTR_LOOP_1050_13ae == (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1)) {
        puStack16 = (u8 *)&u16_1050_0002;
      }
      uStack10 = ((int)puStack16 * 0xc + 0x5b84) - 0x1;
      pass1_1008_612e(uStack10,0x0,uStack10);
      puStack20 = (u16 *)pass1_1018_1e78(iVar8->field142_0x92,(((int)puStack16 * 0x6 + uStack10) * 0x2 + 0x5b86)
                                        );
      load_string_1010_84e0
                (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x50,local_464,(short)&DAT_1050_1050);
      pcVar11 = load_string_1010_847e(_u16_1050_14cc,*puStack20);
      uVar15 = ((u32)pcVar11 >> 0x10);
      paVar12 = (astruct_57 *)CONCAT22(uVar9,uVar15);
      uVar13 = SUB21(local_464,0x0);
      uVar6 = wsprintf16(local_414,(char *)0x5bc01050,
                         (char *)CONCAT13((char)(local_464 >> 0x8),CONCAT12(uVar13,0x1050)),uVar13,
                         (int)&DAT_1050_1050,(int)pcVar11,uVar15);
      uVar9 = 0x1000;
      mem_op_1000_179c(0xb4,paVar12);
      uVar7 = paVar12;
      if ((uVar7 | uVar6) == 0x0) {
        uVar6 = 0x0;
        paVar12 = NULL;
      }
      else {
        pWVar14 = local_414;
        uVar15 = SUB42(&DAT_1050_1050,0x0);
        HVar4 = HWND16_1050_0396;
        pcVar11 = load_string_1010_847e(_u16_1050_14cc,0x57b);
        uVar9 = SUB42(&PTR_LOOP_1050_1040,0x0);
        paVar12 = pass1_1040_8478(((u32)pcVar11 >> 0x10),(astruct_57 *)CONCAT22(uVar7,uVar6),0x41,pcVar11,
                                  (char *)CONCAT22(uVar15,pWVar14),HVar4);
        uVar6 = paVar12;
      }
      param_1 = (StructD *)((u32)paVar12 >> 0x10);
      puStack1128 = (u32 *)((u32)paVar12 & 0xffff0000 | (u32)uVar6);
      ppcVar2 = (code **)((int)*puStack1128 + 0x74);
      HStack8 = (**ppcVar2)(uVar9,uVar6,(int)((u32)paVar12 >> 0x10));
      if (HStack8 != 0x1) goto LAB_1038_bc26;
      pass1_1008_d818(iVar8->field141_0x8e,((int)puStack20 + 0x1a));//
LAB_1038_bba2:
      win_ui_cursor_op_1038_bc30(param_2);
    }
    PostMessage16(0x0,0xce,0x111,HWND16_1050_0396);
    param_4 = 0x1;//
LAB_1038_bbbf:
    uVar9 = SUB42(param_1,0x0);
    BStack6 = post_win_msg_1040_7b3c(param_2,param_3,(param_3 >> 0x10),param_4);
    uStack4 = uVar9;
  }//
LAB_1038_bc26:
  return CONCAT22(uStack4,BStack6);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_cursor_op_1038_bc30(StructC *param_1)

{
  let mut uVar1: u32;
  let mut local_112: u16;
  let mut uStack272: u16;
  HCURSOR16 HStack6;
  HCURSOR16 HStack4;

  HStack4 = LoadCursor16((char *)0x7f02,0x0);
  HStack6 = SetCursor16(HStack4);
  uVar1 = (u32)((int)param_1 + 0x8e);
  pass1_1030_532e((astruct_97 *)CONCAT22(0x1050,&local_112),(long)((int)uVar1 + 0xe) + 0x1000000);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,&local_112));
  pass1_1030_838e((u32 *)_u16_1050_5748);
  local_112 = 0x389a;
  uStack272 = 0x1008;
  pass1_1030_8334();
  SetCursor16(HStack6);
  return;
}
pub fn pass1_1038_bca8(mut param_1: u32)

{
  let mut uVar1: u16;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut uVar4: u32;
  astruct_394 *paVar5;
  u32 *puVar6;
  let mut uVar7: u16;
  u8 *puVar8;
  let mut in_EDX: u32;
  astruct_57 *paVar9;
  let mut iVar10: i16;
  let mut iVar11: i16;
  let mut uVar12: u16;
  let mut uVar13: u16;

  uVar12 = (param_1 >> 0x10);
  iVar10 = (int)param_1;
  uVar3 = (u32)(iVar10 + 0x8e);
  uVar13 = ((u32)uVar3 >> 0x10);
  iVar11 = (int)uVar3;
  puVar6 = (u32 *)(u32)(iVar11 + 0xa);
  paVar9 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)(iVar11 + 0xc));
  ppcVar2 = (code **)((int)*puVar6 + 0x14);
  (**ppcVar2)();
  paVar5 = (astruct_394 *)puVar6;
  uVar4 = (long)paVar9 << 0x10;
  if (*(i32 *)(iVar10 + 0x70) != 0x0) {
    paVar5 = *(astruct_394 **)(iVar10 + 0x70);
    uVar1 = (iVar10 + 0x72);
    uVar7 = uVar1 | paVar5;
    paVar9 = (astruct_57 *)((u32)paVar9 & 0xffff0000 | (u32)uVar7);
    if (uVar7 != 0x0) {
      ppcVar2 = (code **)(u32)paVar5;
      (**ppcVar2)();
    }
  }
  mem_op_1000_179c(0x14,paVar9);
  puVar8 = (u8 *)(paVar9 | paVar5);
  if (puVar8 == NULL) {
    paVar5 = NULL;
    puVar8 = NULL;
  }
  else {
    struct_1008_4c58(paVar5);
  }
  *(astruct_394 **)(iVar10 + 0x70) = paVar5;
  *(u8 **)(iVar10 + 0x72) = puVar8;
  pass1_1008_4d84(puVar8,*(astruct_90 **)(iVar10 + 0x70),(u32)puVar6 & 0xffff | uVar4);
  return;
}



StructD * pass1_1038_bd4a(StructD *param_1,u8 param_2)

{
  pass1_1038_b7f0(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_bddc(StructD *param_1,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_57 *iVar1;
  let mut unaff_BP: u16;
  astruct_57 *uVar1;
  u32 *puVar2;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0x176,param_6);
  uVar1 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  iVar1[0x1].field2_0x4 = 0x0;
  iVar1[0x1].field3_0x6 = 0x0;
  iVar1[0x1].field4_0x8 = 0x0;
  iVar1[0x1].field5_0xa = 0x0;
  iVar1[0x1].field6_0xc = 0x0;
  iVar1[0x1].field7_0xe = 0x0;
  param_2->field0_0x0 = 0xc436;
  iVar1->field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3b),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_be4a(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xc436;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_be76(u8 *param_1,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_register_0000000a: u16;
  astruct_27 *paVar1;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut iVar2: i16;

  if (param_3 == 0x0) {
    iVar2 = 0x0;
    paVar1 = (astruct_27 *)
             mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,(u8 **)0x2b,
                             in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
    pass1_1010_038e(paVar1,iVar2);
  }
  destroy_win_1040_7b98(CONCAT22((int)param_3,param_2));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_dlg_op_1038_bea4(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  HWND16 HVar3;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar4;
  let mut iVar5: i16;
  let mut uVar6: u16;
  u32 *puVar7;
  let mut uVar8: u32;
  char *lparam;
  LRESULT LVar9;
  let mut in_stack_0000fd7a: u16;
  let mut in_stack_0000fd7c: u16;
  let mut in_stack_0000fe9e: u16;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000fea4: u16;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000fea8: u16;
  let mut in_stack_0000feaa: u16;
  let mut in_stack_0000fed2: u16;
  let mut in_stack_0000fed4: u16;
  u32 *local_116;
  u32 *local_112;
  WORD local_10e [0x41];
  u8 local_8c [0x82];
  let mut uStack10: u32;
  u32 *puStack6;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  puStack6 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fed2,0x2),in_stack_0000fd7a,
                             in_stack_0000fe9e,in_stack_0000fea4,in_stack_0000fea8);
  paVar4 = (astruct_57 *)((u32)paVar4 & 0xffff0000 | (u32)puStack6 >> 0x10);
  uStack10 = (u32)((int)puStack6 + 0x68);
  uVar6 = (param_2 >> 0x10);
  iVar5 = (int)param_2;
  GetWindowText16(0x80,CONCAT22(0x1050,local_8c),*(HWND16 *)(iVar5 + 0x6));
  wsprintf16(local_10e,(char *)CONCAT22(local_8c,0x1050),(char *)CONCAT22((int)uStack10,0x1050),
             (int)((u32)uStack10 >> 0x10));
  SetWindowText16(CONCAT22(0x1050,local_10e),*(HWND16 *)(iVar5 + 0x6));
  HVar3 = GetDlgItem16(0x179,*(HWND16 *)(iVar5 + 0x6));
  *(HWND16 *)(iVar5 + 0x92) = HVar3;
  pass1_1008_e3ec(*(astruct_218 **)(iVar5 + 0x8e),(u32 *)CONCAT22(0x1050,&local_116),
                  (u32 *)CONCAT22(0x1050,&local_112));
  send_msg_1038_c374(param_2,local_112,(iVar5 + 0x92));
  puVar7 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fed4,0x2f),in_stack_0000fd7c,
                           in_stack_0000fea0,in_stack_0000fea6,in_stack_0000feaa);
  uVar2 = ((u32)puVar7 >> 0x10);
  uVar8 = (u32)((int)puVar7 + 0x24);
  uVar1 = (u32)(iVar5 + 0x8e);
  uVar8 = string_1008_e586(uVar1,((u32)uVar1 >> 0x10),uVar8,uVar8,uVar2);
  SendMessage16(uVar8,0xffff,0x40d,*(HWND16 *)(iVar5 + 0x92));
  HVar3 = GetDlgItem16(0x17a,*(HWND16 *)(iVar5 + 0x6));
  *(HWND16 *)(iVar5 + 0x94) = HVar3;
  send_msg_1038_c374(param_2,local_116,HVar3);
  lparam = load_string_1010_847e(_u16_1050_14cc,0x531);
  LVar9 = SendMessage16((LPARAM)lparam,0x0,0x403,*(HWND16 *)(iVar5 + 0x94));
  (iVar5 + 0x9c) = (int)LVar9;
  SendMessage16((LPARAM)lparam,0xffff,0x40d,*(HWND16 *)(iVar5 + 0x94));
  HVar3 = GetDlgItem16(0x178,*(HWND16 *)(iVar5 + 0x6));
  *(HWND16 *)(iVar5 + 0x96) = HVar3;
  HVar3 = GetDlgItem16(0x177,*(HWND16 *)(iVar5 + 0x6));
  *(HWND16 *)(iVar5 + 0x98) = HVar3;
  HVar3 = GetDlgItem16(0x184,*(HWND16 *)(iVar5 + 0x6));
  *(HWND16 *)(iVar5 + 0x9a) = HVar3;
  return;
}
pub fn show_win_1038_c044(StructB *struct_b_param_1)

{
  let mut uVar1: u16;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  uVar1 = ((u32)struct_b_param_1 >> 0x10);
  ShowWindow16(0x5,*(HWND16 *)((int)struct_b_param_1 + 0x6));
  SetFocus16(*(HWND16 *)((int)struct_b_param_1 + 0x6));
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn msg_box_op_1038_c07a(mut param_1: i16,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  u8 local_70c [0x200];
  char local_50c [0x100];
  char local_40c [0x402];
  let mut uStack10: u32;
  let mut uStack6: u32;

  send_msg_1038_c228(CONCAT22(param_2,param_1));
  uStack6 = load_string_1010_847e(_u16_1050_14cc,0x531);
  if (param_4 == 0x177) {
    pass1_1008_e05e(*(astruct_102 **)(param_1 + 0x8e),0x2,(char *)CONCAT22(param_2,param_1 + 0x19eU),
                    (char *)CONCAT22(param_2,param_1 + 0x9e));
    load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x200,local_40c,(short)&DAT_1050_1050)
    ;
    sys_1000_3f9c((char *)CONCAT22(0x1050,local_70c),(char *)CONCAT22(0x1050,local_40c),param_1 + 0x19eU);
    load_string_1010_84e0(_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x100,local_50c,(short)&DAT_1050_1050)
    ;
    MessageBox16(0x30,(char *)CONCAT22(0x1050,local_50c),(char *)CONCAT22(0x1050,local_70c),*(HWND16 *)(param_1 + 0x6));
  }
  else {
    if (param_4 != 0x178) {
      if ((param_4 != 0x178) && (param_4 - 0x179U < 0x2)) {
        set_win_pos_1038_c31a(CONCAT22(param_2,param_1),param_3,param_4);
        return;
      }
      post_win_msg_1040_7b3c((StructC *)CONCAT22(param_2,param_1),param_3,param_4,param_4);
      return;
    }
    uStack10 = CONCAT22(param_2,param_1 + 0x9e);
    uVar2 = param_2;
    iVar1 = pass1_1008_e10c(*(astruct_102 **)(param_1 + 0x8e),(char *)CONCAT22(param_2,param_1 + 0x19e),
                            (char *)CONCAT22(param_2,param_1 + 0x9e),param_2,&DAT_1050_1050);
    if (iVar1 == 0x0) {
      load_string_1010_84e0
                (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_40c,(short)&DAT_1050_1050);
      load_string_1010_84e0
                (_u16_1050_14cc,(_u16_1050_14cc >> 0x10),0x3ff,local_50c,(short)&DAT_1050_1050);
      MessageBox16(0x30,(char *)CONCAT22(0x1050,local_50c),(char *)CONCAT22(0x1050,local_40c),*(HWND16 *)(param_1 + 0x6)
                  );
      return;
    }
    pass1_1008_e01c((u32)(param_1 + 0x8e),CONCAT22(param_2,param_1 + 0x19e),uStack10);
    pass1_1038_af40(param_1,uVar2,_PTR_LOOP_1050_5b7c,(param_1 + 0x8),0x1f);
  }
  PostMessage16(0x0,0x2,0x111,*(HWND16 *)(param_1 + 0x6));
  return;
}



LRESULT send_msg_1038_c228(mut param_1: u32)

{
  WPARAM16 wparam;
  let mut iVar1: i16;
  let mut uVar2: u16;
  LRESULT LVar3;
  WPARAM16 wparam_00;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  LVar3 = SendMessage16(0x0,0x0,0x407,*(HWND16 *)(iVar1 + 0x92));
  wparam = (WPARAM16)LVar3;
  SendMessage16(0x0,0x0,0x407,*(HWND16 *)(iVar1 + 0x94));
  wparam_00 = 0x408;
  SendMessage16(param_1 & 0xffff0000 | (u32)(iVar1 + 0x9e),wparam,0x408,*(HWND16 *)(iVar1 + 0x92));
  LVar3 = SendMessage16(param_1 & 0xffff0000 | (u32)(iVar1 + 0x19e),wparam_00,0x408,*(HWND16 *)(iVar1 + 0x94));
  return LVar3;
}
pub fn enable_win_1038_c294(mut param_1: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  char *pcStack12;

  uVar1 = (int)param_1 + 0x9e;
  pcStack12 = (char *)(param_1 & 0xffff0000 | (u32)uVar1);
  uVar3 = param_1;
  pass1_1008_e320(*(astruct_102 **)((int)param_1 + 0x8e),(char *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0x19eU))
                  ,(char *)(param_1 & 0xffff0000 | (u32)uVar1));
  SetWindowText16(CONCAT22(uVar3,uVar1),*(HWND16 *)((int)param_1 + 0x9a));
  uVar2 = pass1_1008_e2a4(*(astruct_102 **)((int)param_1 + 0x8e),
                          (char *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0x19eU)),pcStack12);
  EnableWindow16(uVar2 & 0x1,*(HWND16 *)((int)param_1 + 0x96));
  EnableWindow16(uVar2 & 0x2,*(HWND16 *)((int)param_1 + 0x98));
  return;
}



BOOL16 set_win_pos_1038_c31a(mut param_1: u32,mut param_2: u16 ,mut param_3: i16)

{
  let mut iStack10: i16;

  if (param_3 == 0x1) {
    enable_win_1038_c294(param_1);
  }
  else {
    if (param_3 != 0x7) {
      return 0x0;
    }
    GetWindowRect16((RECT16 *)CONCAT22(0x1050,&stack0xfff2),param_2);
    SetWindowPos16(0x2,0x50,iStack10 - param_2,0x0,0x0,0x0,param_2);
  }
  return 0x1;
}
pub fn send_msg_1038_c374(mut param_1: u32,u32 *param_2,mut param_3: u16 )

{
  let mut uVar1: u32;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut uVar4: u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar5: u16;
  LRESULT LVar6;
  char *lparam;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uVar5 = SUB42(s_tile2_bmp_1050_1538,0x0);
  LVar6 = SendMessage16(0x0,0x0,0x40b,param_3);
  uVar3 = LVar6;
  ppcVar2 = (code **)((int)*param_2 + 0x10);
  (**ppcVar2)((int)s_tile2_bmp_1050_1538,param_2);
  uStack6 = CONCAT22(extraout_DX,uVar3);
  uStack10 = 0x0;
  while( true ) {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar2 = (code **)((int)*param_2 + 0x4);
    uVar4 = uStack6;
    (**ppcVar2)(uVar5,param_2,(char)uStack10,(int)(uStack10 >> 0x10));
    uVar1 = (u32)((int)param_1 + 0x8e);
    lparam = (char *)string_1008_e586(uVar1,((u32)uVar1 >> 0x10),
                                      CONCAT13((char)(extraout_DX_00 >> 0x8),CONCAT12((char)extraout_DX_00,uVar4))
                                      ,uVar4,extraout_DX_00);
    LVar6 = SendMessage16((LPARAM)lparam,0x0,0x403,param_3);
    uVar5 = 0x1000;
    fn_ptr_1000_17ce(lparam);
    if (LVar6 == -0x1) break;
    if (LVar6 == -0x2) {
      return;
    }
    uStack10 += 0x1;
  }
  return;
}



StructD * pass1_1038_c410(StructD *param_1,u8 param_2)

{
  pass1_1038_be4a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_c4a2(StructD *param_1,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_57 *iVar1;
  let mut unaff_BP: u16;
  let mut uVar2: u16;
  u32 *puVar3;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0x17c,param_6);
  uVar2 = ((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  (u32)&iVar1[0x1].field2_0x4 = 0x0;
  (u32)&iVar1[0x1].field4_0x8 = 0x0;
  param_2->field0_0x0 = 0xc74c;
  iVar1->field1_0x2 = &u16_1050_1038;
  puVar3 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3b),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar3;
  iVar1[0x1].field1_0x2 = ((u32)puVar3 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_c4fe(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xc74c;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_c52a(u8 *param_1,mut param_2: u16 ,mut param_3: u32)

{
  let mut in_register_0000000a: u16;
  astruct_27 *paVar1;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut iVar2: i16;

  if (param_3 == 0x0) {
    iVar2 = 0x0;
    paVar1 = (astruct_27 *)
             mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,(u8 **)0x2b,
                             in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
    pass1_1010_038e(paVar1,iVar2);
  }
  destroy_win_1040_7b98(CONCAT22((int)param_3,param_2));
  return;
}
pub fn show_win_1038_c558(StructB *struct_b_param_1)

{
  let mut uVar1: u16;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  uVar1 = ((u32)struct_b_param_1 >> 0x10);
  ShowWindow16(0x5,*(HWND16 *)((int)struct_b_param_1 + 0x6));
  SetFocus16(*(HWND16 *)((int)struct_b_param_1 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_dlg_op_1038_c58e(mut param_1: u16 ,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut in_register_0000000a: u16;
  let mut iVar2: i16;
  let mut unaff_SI: u16;
  let mut uVar3: u16;
  let mut in_stack_0000f68e: u16;
  let mut in_stack_0000f7b2: u16;
  let mut in_stack_0000f7b8: u16;
  let mut in_stack_0000f7bc: u16;
  u32 *puStack2070;
  WORD local_80e [0x201];
  WORD local_40c [0x201];
  let mut uStack10: u32;
  u32 *puStack6;

  puStack6 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                             (u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000f68e,in_stack_0000f7b2,in_stack_0000f7b8,
                             in_stack_0000f7bc);
  uStack10 = (u32)((int)puStack6 + 0x68);
  uVar3 = (param_2 >> 0x10);
  iVar2 = (int)param_2;
  GetWindowText16(0x80,CONCAT22(0x1050,local_40c),*(HWND16 *)(iVar2 + 0x6));
  wsprintf16(local_80e,(char *)CONCAT22(local_40c,0x1050),(char *)CONCAT22((int)uStack10,0x1050),
             (int)((u32)uStack10 >> 0x10));
  SetWindowText16(CONCAT22(0x1050,local_80e),*(HWND16 *)(iVar2 + 0x6));
  puStack2070 = (u32 *)(param_2 & 0xffff0000 | (u32)(iVar2 + 0x96U));
  pass1_1008_e038((u32)(iVar2 + 0x8e),(u32 *)(param_2 & 0xffff0000 | ZEXT24((u32 *)(iVar2 + 0x92))),
                  (u32 *)(param_2 & 0xffff0000 | (u32)(iVar2 + 0x96U)));
  load_string_1010_84e0
            (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x400,(char *)local_80e,
             (short)&DAT_1050_1050);
  uVar1 = (u32)(iVar2 + 0x92);
  wsprintf16(local_40c,(char *)CONCAT22(local_80e,0x1050),(char *)CONCAT22((int)*puStack2070,0x1050),
             (int)((u32)*puStack2070 >> 0x10),(int)uVar1,(int)((u32)uVar1 >> 0x10));
  SetDlgItemText16(CONCAT22(0x1050,local_40c),0x17f,*(HWND16 *)(iVar2 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn message_box_op_1038_c672(undefined1 param_1,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  let mut uVar1: u16;
  char local_404 [0x402];

  uVar1 = ((u32)_u16_1050_14cc >> 0x10);
  if (param_5 == 0x17d) {
    load_string_1010_84e0(_u16_1050_14cc,uVar1,0x3ff,local_404,(short)&DAT_1050_1050);
    MessageBox16(0x30,*(char **)(param_2 + 0x92),(char *)CONCAT22(0x1050,local_404),*(HWND16 *)(param_2 + 0x6));
  }
  else {
    if (param_5 != 0x17e) {
      post_win_msg_1040_7b3c((StructC *)CONCAT22(param_3,param_2),param_4,param_5,param_5);
      return;
    }
    load_string_1010_84e0(_u16_1050_14cc,uVar1,0x3ff,local_404,(short)&DAT_1050_1050);
    MessageBox16(0x30,*(char **)(param_2 + 0x92),(char *)CONCAT22(0x1050,local_404),*(HWND16 *)(param_2 + 0x6));
    pass1_1008_e164(*(astruct_102 **)(param_2 + 0x8e));
  }
  PostMessage16(0x0,0x2,0x111,*(HWND16 *)(param_2 + 0x6));
  return;
}



StructD * pass1_1038_c726(StructD *StructD_32,u8 param_2)

{
  pass1_1038_c4fe(StructD_32);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)StructD_32);
  }
  return StructD_32;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 *
pass1_1038_c7b8(u8 *param_1,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_57 *iVar1;
  let mut unaff_BP: u16;
  astruct_57 *uVar1;
  u32 *puVar2;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfb8,param_6);
  uVar1 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  param_2->field0_0x0 = 0xca6c;
  iVar1->field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x5),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_c80a(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xca6c;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn destroy_window_1038_c836(param_1: *mut astruct_881,mut param_2: u32,mut param_3: u32)

{
  let mut puVar1: *mut u16;
  u8 local_6 [0x4];
  let mut uVar1: u32;

  if (param_3 == 0xfce) {
    puVar1 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0xac);
    win_1008_5c9e(local_6,(int)((u32)puVar1 >> 0x10),_u16_1050_02a0,(u32 *)CONCAT22(0x1050,local_6));
    uVar1 = param_1->field141_0x8e;
    ((int)uVar1 + 0xa) = 0x6;
    DestroyWindow16(param_1->field6_0x6);
    PTR_LOOP_1050_5b80 = NULL;
    return;
  }
  post_win_msg_1040_7b3c
            ((StructC *)CONCAT22(param_2,param_1),(param_2 >> 0x10),param_3,param_3);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2
pub fn win_ui_op_1038_c89c(StructB *struct_b_param_1)

{
  HWND16 HVar1;
  StructB *struct_b_4;
  let mut uVar3: u16;
  let mut enable: bool;
  astruct_910 *iVar1;
  let mut uVar1: u32;
  let mut uVar2: u32;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar3 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_4 = (StructB *)struct_b_param_1;
  CheckRadioButton16(0xfac,0xfad,0xfac,(HWND16)struct_b_4->lpvoid_field_0x8);
  uVar1 = (u32)&struct_b_4[0x7].field1_0x2;
  ((int)uVar1 + 0xa) = 0x1;
  uVar2 = (u32)&struct_b_4[0x7].field1_0x2;
  iVar1 = *(astruct_910 **)((int)uVar2 + 0x12);
  if (iVar1 == (astruct_910 *)&u32_1050_0004) {//
LAB_1038_c8da:
    HVar1 = GetDlgItem16(0xfce,(HWND16)struct_b_4->lpvoid_field_0x8);
    if (HVar1 != 0x0) {
      EnableWindow16(0x1,HVar1);
    }
    HVar1 = GetDlgItem16(0x1,(HWND16)struct_b_4->lpvoid_field_0x8);
    if (HVar1 == 0x0) goto LAB_1038_c93c;
    enable = 0x0;
  }
  else {
    if (((int)(iVar1 + -0x5) < 0x1) || (SBORROW2((int)(iVar1 + -0x5),0x1))) goto LAB_1038_c93c;
    if (iVar1 != (astruct_910 *)&u16_1050_0008 && 0x0 < (int)(iVar1 + -0x7)) {
      if (iVar1 != (astruct_910 *)((int)&u16_1050_0008 + 0x1)) goto LAB_1038_c93c;
      goto LAB_1038_c8da;
    }
    HVar1 = GetDlgItem16(0xfce,(HWND16)struct_b_4->lpvoid_field_0x8);
    if (HVar1 == 0x0) goto LAB_1038_c93c;
    enable = 0x1;
  }
  EnableWindow16(enable,HVar1);//
LAB_1038_c93c:
  move_win_1040_826c(struct_b_param_1,0xc8,0x0);
  ShowWindow16(0x5,(HWND16)struct_b_4->lpvoid_field_0x8);
  return;
}



// WARNING: Unable to use type for symbol uVar1
// WARNING: Unable to use type for symbol uVar2
pub fn win_dlg_op_1038_c95e(astruct_882 *struct_param_1,mut param_2: i16)

{
  let mut uVar3: u32;
  let mut UVar4: u16;
  let mut UVar5: u16;
  let mut UVar6: u16;
  astruct_882 *iVar3;
  let mut uVar7: u16;
  let mut uVar1: u32;
  let mut uVar2: u32;

  iVar3 = (astruct_882 *)struct_param_1;
  uVar7 = ((u32)struct_param_1 >> 0x10);
  if (param_2 == 0x0) {
    uVar3 = iVar3->field141_0x8e;
    ((int)uVar3 + 0xa) = 0x0;
  }
  else {
    UVar4 = IsDlgButtonChecked(0xfac,iVar3->field6_0x6);
    if (UVar4 == 0x0) {
      UVar5 = IsDlgButtonChecked(0xfad,iVar3->field6_0x6);
      if (UVar5 == 0x0) {
        UVar6 = IsDlgButtonChecked(0xfae,iVar3->field6_0x6);
        if (UVar6 == 0x0) {
          UVar6 = IsDlgButtonChecked(0xfaf,iVar3->field6_0x6);
          if (UVar6 == 0x0) {
            UVar6 = IsDlgButtonChecked(0xfb0,iVar3->field6_0x6);
            if (UVar6 != 0x0) {
              uVar3 = iVar3->field141_0x8e;
              ((int)uVar3 + 0xa) = 0x5;
            }
          }
          else {
            uVar3 = iVar3->field141_0x8e;
            ((int)uVar3 + 0xa) = 0x4;
          }
        }
        else {
          uVar3 = iVar3->field141_0x8e;
          ((int)uVar3 + 0xa) = 0x3;
        }
      }
      else {
        uVar2 = iVar3->field141_0x8e;
        ((int)uVar2 + 0xa) = 0x2;
      }
    }
    else {
      uVar1 = iVar3->field141_0x8e;
      ((int)uVar1 + 0xa) = 0x1;
    }
  }
  DestroyWindow16(iVar3->field6_0x6);
  PTR_LOOP_1050_5b80 = NULL;
  return;
}
pub fn FUN_1038_ca42(void)

{
  return;
}



StructD * pass1_1038_ca46(StructD *param_1,u8 param_2)

{
  pass1_1038_c80a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * pass1_1038_cad8(u8 *param_1,param_2: *mut astruct_57,mut param_3: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_57 *iVar1;
  let mut unaff_BP: u16;
  astruct_57 *uVar1;
  u32 *puVar2;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,0x0,0x1cb,param_3);
  uVar1 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  param_2->field0_0x0 = 0xcc9a;
  iVar1->field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x2c),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  iVar1->field86_0x74 = 0x0;
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_cb30(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xcc9a;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn show_win_1038_cb5c(mut param_1: u32,StructB *struct_b_param_1,mut param_3: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  astruct_57 *paVar4;
  astruct_57 *paVar5;
  let mut uVar6: u16;
  astruct_57 *paVar7;
  StructB *struct_b_5;
  let mut uVar8: u16;
  let mut puVar9: *mut u16;
  let mut in_stack_0000fe48: u16;
  let mut in_stack_0000fe4c: u16;
  let mut in_stack_0000ff72: u16;
  let mut in_stack_0000ff76: u16;
  let mut in_stack_0000ff7a: u16;
  let mut iStack10: i16;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uVar8 = ((u32)struct_b_param_1 >> 0x10);
  struct_b_5 = (StructB *)struct_b_param_1;
  uVar3 = pass1_1008_eb6e();
  for (iStack10 = 0x0; iStack10 < (int)uVar3; iStack10 += 0x1) {
    uVar1 = (u32)&struct_b_5[0x7].field1_0x2;
    puVar9 = (u16 *)pass1_1008_eb5c(uVar1,((u32)uVar1 >> 0x10),iStack10);
    paVar7 = (astruct_57 *)(param_1 & 0xffff0000 | (u32)puVar9 >> 0x10);
    paVar4 = (astruct_57 *)puVar9;
    uVar2 = ((u32)puVar9 >> 0x10);
    paVar5 = paVar4;
    mem_op_1000_179c(0x42,paVar7);
    uVar6 = (astruct_57 *)paVar7 | paVar5;
    param_1 = (u32)paVar7 & 0xffff0000 | (u32)uVar6;
    if (uVar6 != 0x0) {
      pass1_1008_3bd6(param_1,paVar5,(astruct_57 *)paVar7,0x0,CONCAT22(*puVar9,paVar4->field1_0x2),0x101,0xff0100,
                      CONCAT22(struct_b_5->lpvoid_field_0x8,paVar4->field2_0x4),param_3,in_stack_0000fe48,
                      in_stack_0000fe4c,in_stack_0000ff72,in_stack_0000ff76,in_stack_0000ff7a);
    }
  }
  win_1008_5c7c(uVar3,param_1,_u16_1050_02a0,0x90001);
  ShowWindow16(0x5,(HWND16)struct_b_5->lpvoid_field_0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn destroy_window_1038_cc00(StructC *param_1,mut param_2: u16 ,mut param_3: u32,u8 *param_4)

{
  let mut uVar1: u16;
  let mut iVar2: i16;

  uVar1 = param_3 - 0x1cd;
  if (uVar1 == 0x0) {
    iVar2 = 0x1;
  }
  else {
    uVar1 = param_3 - 0x1ce;
    if (uVar1 == 0x0) {
      iVar2 = 0x2;
    }
    else {
      uVar1 = param_3 - 0x1cf;
      if (uVar1 == 0x0) {
        iVar2 = 0x3;
      }
      else {
        uVar1 = param_3 - 0x1d0;
        if (uVar1 == 0x0) {
          iVar2 = 0x4;
        }
        else {
          uVar1 = param_3 - 0x1d1;
          if (uVar1 != 0x0) {
            post_win_msg_1040_7b3c(param_1,param_2,param_3,param_3);
            return;
          }
          iVar2 = 0x5;
        }
      }
    }
  }
  pass1_1008_eb74(param_4,(u32)((int)param_1 + 0x8e),iVar2);
  if (uVar1 != 0x0) {
    win_1008_5c7c(uVar1,param_4,_u16_1050_02a0,CONCAT22(uVar1,0x1));
    DestroyWindow16(*(HWND16 *)((int)param_1 + 0x6));
  }
  return;
}



StructD * pass1_1038_cc74(StructD *param_1,u8 param_2)

{
  pass1_1038_cb30(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_cd06(StructD *param_1,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_57 *iVar1;
  let mut unaff_BP: u16;
  astruct_57 *uVar1;
  u32 *puVar2;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfcc,param_6);
  uVar1 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  iVar1[0x1].field2_0x4 = 0x0;
  param_2->field0_0x0 = 0xcf00;
  iVar1->field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x42),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_cd5c(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xcf00;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn destroy_window_1038_cd88(StructB *struct_b_param_1)

{
  StructB *struct_1;
  let mut uVar1: u16;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  uVar1 = ((u32)struct_b_param_1 >> 0x10);
  struct_1 = (StructB *)struct_b_param_1;
  ShowWindow16(0x5,(HWND16)struct_1->lpvoid_field_0x8);
  struct_1[0x7].lpvoid_field_0x8 = (LPVOID)((int)&PTR_LOOP_1050_0000 + 0x1);
  unk_win_msg_op_1008_9510((i16 *)((u32)struct_b_param_1 & 0xffff0000 | ZEXT24(&struct_1[0x7].lpvoid_field_0x8)));
  DestroyWindow16((HWND16)struct_1->lpvoid_field_0x8);
  return;
}
pub fn check_dlg_btn_checked_1038_cdd6(param_1: *mut astruct_61,mut param_2: i16)

{
  let mut uVar1: u32;
  let mut UVar2: u16;
  astruct_61 *iVar3;
  let mut uVar3: u16;

  iVar3 = (astruct_61 *)param_1;
  uVar3 = ((u32)param_1 >> 0x10);
  if (param_2 == 0x0) {
    uVar1 = iVar3->field142_0x8e;
    ((int)uVar1 + 0xa) = 0x0;
  }
  else {
    UVar2 = IsDlgButtonChecked(0x182e,*(HWND16 *)&iVar3->field_0x6);
    if (UVar2 == 0x0) {
      UVar2 = IsDlgButtonChecked(0x182f,*(HWND16 *)&iVar3->field_0x6);
      if (UVar2 == 0x0) {
        UVar2 = IsDlgButtonChecked(0x1829,*(HWND16 *)&iVar3->field_0x6);
        if (UVar2 == 0x0) {
          UVar2 = IsDlgButtonChecked(0x182a,*(HWND16 *)&iVar3->field_0x6);
          if (UVar2 == 0x0) {
            UVar2 = IsDlgButtonChecked(0x182c,*(HWND16 *)&iVar3->field_0x6);
            if (UVar2 == 0x0) {
              UVar2 = IsDlgButtonChecked(0x182d,*(HWND16 *)&iVar3->field_0x6);
              if (UVar2 != 0x0) {
                uVar1 = iVar3->field142_0x8e;
                ((int)uVar1 + 0xa) = 0x7;
              }
            }
            else {
              uVar1 = iVar3->field142_0x8e;
              ((int)uVar1 + 0xa) = 0x6;
            }
          }
          else {
            uVar1 = iVar3->field142_0x8e;
            ((int)uVar1 + 0xa) = 0x4;
          }
        }
        else {
          uVar1 = iVar3->field142_0x8e;
          ((int)uVar1 + 0xa) = 0x3;
        }
      }
      else {
        uVar1 = iVar3->field142_0x8e;
        ((int)uVar1 + 0xa) = 0x2;
      }
    }
    else {
      uVar1 = iVar3->field142_0x8e;
      ((int)uVar1 + 0xa) = 0x1;
    }
  }
  iVar3->field143_0x92 = 0x0;
  return;
}
pub fn FUN_1038_ced6(void)

{
  return;
}



StructD * pass1_1038_ceda(StructD *param_1,u8 param_2)

{
  pass1_1038_cd5c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn make_proc_inst_1038_cf6c(u8 *param_1,param_2: *mut astruct_831)

{
  astruct_831 *iVar1;
  let mut uVar1: u16;
pub fn *pvVar1;

  uVar1 = ((u32)param_2 >> 0x10);
  iVar1 = (astruct_831 *)param_2;
  param_2 = 0x389a;
  iVar1->field2_0x2 = 0x1008;
  (u32)&iVar1->field3_0x4 = 0x0;
  iVar1->field5_0x8 = 0x0;
  param_2 = 0xd23e;
  iVar1->field2_0x2 = (int)&u16_1050_1038;
  _u16_1050_5bc8 = param_2;
  pvVar1 = MakeProcInstance16(HINSTANCE16_1050_038c,&u16_1038_d116);
  iVar1->field3_0x4 = (int)pvVar1;
  iVar1->field4_0x6 = (int)((u32)pvVar1 >> 0x10);
  pvVar1 = MakeProcInstance16(HINSTANCE16_1050_038c,&PTR_LAB_1038_d08b_1_1038_d01e);
  u16_1050_5bcc = pvVar1;
  u16_1050_5bce = ((u32)pvVar1 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn free_proc_inst_1038_cfda(StructD *param_1)

{
  StructD *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0xd23e;
  iVar1->address_offset_field_0x2 = &u16_1050_1038;
  FreeProcInstance16(*(void **)&iVar1->hfile_0x4);
  FreeProcInstance16(_u16_1050_5bcc);
  (u32)&iVar1->hfile_0x4 = 0x0;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  return;
}



i32 call_win_proc_1038_d020
               (u16 base_param_6,HWND16 win_handle_1,mut param_3: u32,HWND16 l_param,HWND16 hwnd_param_4,
               HWND16 win_handle_2)

{
  HANDLE16 handle_1;
  HANDLE16 handle_2;
  HANDLE16 handle_3;
  let mut var1: u16;
  LRESULT lresult;
  i32 var5;
  u32 *var6;
  i32 var7;
  let mut var8: u16;
  code **fn_ptr_1;

  handle_1 = GetProp16((LPCSTR)CONCAT22(base_param_6,0x5bd7),hwnd_param_4);
  handle_2 = GetProp16((LPCSTR)CONCAT13((char)(base_param_6 >> 0x8),CONCAT12((char)base_param_6,0x5bd0)),
                       hwnd_param_4);
  var7 = CONCAT22(handle_1,handle_2);
  handle_1 = GetProp16((LPCSTR)CONCAT22(base_param_6,0x5be5),hwnd_param_4);
  handle_3 = GetProp16((LPCSTR)CONCAT22(base_param_6,0x5bde),hwnd_param_4);
  var6 = (u32 *)CONCAT22(handle_1,handle_3);
  if ((handle_1 | handle_3) != 0x0) {
    var5 = 0x0;
    if (l_param == 0x19) {
      fn_ptr_1 = (code **)((int)*var6 + 0x34);
      var5 = (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,(char)handle_3,handle_1,win_handle_1,param_3);
    }
    else {
      if (l_param == 0x86) {
        fn_ptr_1 = (code **)((int)*var6 + 0x20);
        var1 = (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,handle_3,handle_1,param_3);
        goto LAB_1038_d10e;
      }
      if ((l_param == 0x112) && ((param_3 & 0xfff0) == 0xf140)) {
        lresult = SendMessage16(0x0,0xf140,0x112,*(HWND16 *)&HWND16_1050_0396);
        var1 = ((int)lresult == 0x0);
        goto LAB_1038_d10e;
      }
    }
    if (var5 != 0x0) {
      return var5;
    }
  }
  if (var7 != 0x0) {
    lresult = CallWindowProc16(CONCAT22((int)param_3,win_handle_1),param_3,l_param,hwnd_param_4,(LPVOID)handle_2);
    return lresult;
  }
  var1 = 0x0;//
LAB_1038_d10e:
  return (long)(int)var1;
}



// WARNING: Unable to use type for symbol uVar2
pub fn win_prop_op_1038_d118(u16 base_addr_param_4,mut param_2: u32,mut param_3: u32,HWND16 hwnd_param_3)

{
  let mut uVar1: u32;
  char cVar2;
  HANDLE16 HVar3;
  HANDLE16 HVar4;
  u32 *puStack6;
  let mut uVar2: u32;
  code **fn_ptr_1;

  HVar3 = GetProp16((LPCSTR)CONCAT22(base_addr_param_4,0x5bf3),hwnd_param_3);
  HVar4 = GetProp16((LPCSTR)CONCAT22(base_addr_param_4,0x5bec),hwnd_param_3);
  puStack6 = (u32 *)CONCAT22(HVar3,HVar4);
  if (param_3 == 0x30) {
    if ((HANDLE16)param_3 == 0x0) {
      return;
    }
    SetProp16((HANDLE16)param_3,(char *)CONCAT22(base_addr_param_4,0x5c06),hwnd_param_3);
    return;
  }
  if (param_3 < 0x310000) {
    cVar2 = (char)(param_3 >> 0x10);
    if (cVar2 == '\x02') {
      if ((HVar3 | HVar4) != 0x0) {
        uVar1 = *puStack6;
        fn_ptr_1 = (code **)uVar1 + 0x6;
        (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,HVar4,(char)HVar3,param_2,param_3);
        if (puStack6 != NULL) {
          fn_ptr_1 = (code **)uVar1;
          (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,HVar4,HVar3,0x1);
        }
      }
      HVar3 = GetProp16((LPCSTR)CONCAT22(base_addr_param_4,0x5bfa),hwnd_param_3);
      if (HVar3 == 0x0) {
        return;
      }
      DeleteObject16(HVar3);
      RemoveProp16((LPCSTR)CONCAT22(base_addr_param_4,0x5c00),hwnd_param_3);
      return;
    }
    if (cVar2 == '\x06') {
      if (((HANDLE16)param_3 != 0x1) && ((HANDLE16)param_3 != 0x2)) {
        uVar1 = (u32)&u16_1050_5bc8;
        ((int)uVar1 + 0x8) = 0x0;
        return;
      }
      uVar2 = (u32)&u16_1050_5bc8;
      *(HWND16 *)((int)uVar2 + 0x8) = hwnd_param_3;
      return;
    }
  }
  if ((HVar3 | HVar4) != 0x0) {
    fn_ptr_1 = (code **)((int)*puStack6 + 0xc);
    (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,HVar4,(char)HVar3,param_2,param_3);
  }
  return;
}



StructD * pass1_1038_d218(StructD *param_1,u8 param_2)

{
  free_proc_inst_1038_cfda(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_57 * pass1_1038_d242(param_1: *mut astruct_57,mut param_2: u16 )

{
  astruct_57 *uVar1;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x13e,param_2);
  uVar1 = (astruct_57 *)((u32)param_1 >> 0x10);
  param_1->field0_0x0 = 0xd6ea;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  ((int)param_1 + 0x74) = 0x1;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_d276(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xd6ea;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1038_d2a2(param_1: *mut astruct_57,StructB *struct_b_param_1,mut param_3: u16 )

{
  astruct_57 *rect;
  let mut iVar1: i16;
  HWND16 hwnd_2;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut uVar4: u16;
  astruct_912 *iVar5;
  StructB *struct_b_6;
  let mut uVar6: u16;
  u32 *puVar7;
  char *l_param;
  LRESULT LVar8;
  let mut in_stack_0000fe2e: u16;
  let mut in_stack_0000fe32: u16;
  let mut in_stack_0000fe80: u16;
  let mut in_stack_0000ff58: u16;
  let mut in_stack_0000ff5c: u16;
  let mut in_stack_0000ff60: u16;
  let mut in_stack_0000ffa4: u16;
  let mut in_stack_0000ffaa: u16;
  let mut in_stack_0000ffae: u16;
  WPARAM16 w_param;
  let mut msg: u16;
  INT16 id;
  let mut uVar9: u16;
  LPVOID hwnd;
  let mut local_16: u16;
  let mut uStack20: u16;
  let mut uStack18: u16;
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut uStack12: u16;
  let mut uStack10: u16;
  let mut uStack8: u32;
  let mut uStack4: u16;
  astruct_57 *paVar5;

  dialog_ui_fn_1040_78e2(struct_b_param_1);
  uStack4 = 0x7;
  for (uStack10 = 0x0; struct_b_6 = (StructB *)struct_b_param_1, uVar6 = ((u32)struct_b_param_1 >> 0x10),
      (int)uStack10 < (int)uStack4; uStack10 += 0x1) {
    iVar5 = (astruct_912 *)(uStack10 * 0xc);
    local_16 = (iVar5 + 0x5c0c);
    uStack20 = (iVar5 + 0x5c0e);
    uStack18 = 0x1;
    uStack16 = 0x1;
    rect = (astruct_57 *)&local_16;
    MapDialogRect16((RECT16 *)rect,(HWND16)&DAT_1050_1050);
    mem_op_1000_179c(0x42,param_1);
    uVar3 = (astruct_57 *)param_1 | rect;
    paVar5 = (astruct_57 *)((u32)param_1 & 0xffff0000 | (u32)uVar3);
    if (uVar3 == 0x0) {
      rect = NULL;
      param_1 = (astruct_57 *)((u32)param_1 & 0xffff0000);
    }
    else {
      pass1_1008_3bd6((u32)paVar5,rect,(astruct_57 *)param_1,0x1,CONCAT22(local_16,uStack20),0x104,0x1020103,
                      CONCAT22(struct_b_6->lpvoid_field_0x8,(iVar5 + 0x5c10)),param_3,in_stack_0000fe2e,
                      in_stack_0000fe32,in_stack_0000ff58,in_stack_0000ff5c,in_stack_0000ff60);
      param_1 = paVar5;
    }
    uStack8 = CONCAT22((int)param_1,rect);
    if ((uStack10 * 0xc + 0x5c12) == 0x0) {
      EnableWindow16(0x0,*(HWND16 *)&rect->field11_0x18);
    }
  }
  uVar9 = 0x86;
  puVar7 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)0x860009,in_stack_0000fe80,in_stack_0000ffa4,
                           in_stack_0000ffaa,in_stack_0000ffae);
  uVar4 = ((u32)puVar7 >> 0x10);
  uStack14 = puVar7;
  uStack12 = uVar4;
  iVar1 = pass1_1010_659a((u32)puVar7,uVar9);
  if (iVar1 == 0x0) {
    hwnd_2 = GetDlgItem16(0x14a,(HWND16)struct_b_6->lpvoid_field_0x8);
    EnableWindow16(0x0,hwnd_2);
    hwnd = struct_b_6->lpvoid_field_0x8;
    msg = 0xc;
    id = 0x144;
    w_param = 0x0;
    l_param = load_string_1010_847e(_u16_1050_14cc,0x531);
    LVar8 = SendDlgItemMessage16((LPARAM)l_param,w_param,msg,id,(HWND16)hwnd);
    uVar4 = ((u32)LVar8 >> 0x10);
  }
  move_win_1040_826c(struct_b_param_1,-0x1,0xffff);
  BVar2 = ShowWindow16(0x5,(HWND16)struct_b_6->lpvoid_field_0x8);
  win_1008_5c7c(BVar2,uVar4,_u16_1050_02a0,0x9a0001);
  (struct_b_6 + 0x7)->field0_0x0 = BVar2;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_win_ui_op_1038_d400
               (mut param_1: u16 ,param_2: *mut astruct_885,u8 param_3,u8 param_4,mut param_5: u16 ,mut param_6: u16 ,
               mut param_7: u32)

{
  HWND16 HVar1;
  let mut iVar2: i16;
  let mut uVar2: u16;
  let mut BVar2: bool;
  let mut in_DX: u16;
  let mut in_register_0000000a: u16;
  let mut uVar4: u16;
  let mut puVar4: *mut u16;
  u32 *puVar5;
  LRESULT LVar6;
  char *pcVar7;
  let mut in_stack_0000fe8c: u16;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffbc: u16;
  WPARAM16 WVar8;
  let mut UVar9: u16;
  INT16 IVar10;
  let mut uVar11: u16;
  let mut in_stack_0000ffe6: u16;
  u8 local_c [0x4];
  WPARAM16 WStack8;
  let mut uStack6: u32;
  astruct_57 *paVar3;

  uStack6 = 0x0;
  WStack8 = 0x0;
  switch((int)param_7) {
  case 0x145:
    HVar1 = GetDlgItem16(0x146,param_2->field6_0x6);
    uVar2 = EnableWindow16(0x1,HVar1);
    uStack6 = 0x13f0647;
    uVar11 = 0x1f1;
    goto LAB_1038_d490;
  case 0x146:
    uStack6 = 0x1400648;
    puVar4 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_c),0x1,0xc4);
    puVar4 = ((u32)puVar4 >> 0x10);
    paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,puVar4);
    win_1008_5c9e(local_c,puVar4,_u16_1050_02a0,(u32 *)CONCAT22(0x1050,local_c));
    uVar11 = 0x86;
    puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)0x860009,in_stack_0000fe8c,in_stack_0000ffb0,
                             in_stack_0000ffb6,in_stack_0000ffba);
    uVar4 = ((u32)paVar3 >> 0x10);
    pass1_1010_6604((u32)puVar5,uVar11);
    HVar1 = GetDlgItem16(0x145,param_2->field6_0x6);
    EnableWindow16(0x0,HVar1);
    HVar1 = param_2->field6_0x6;
    UVar9 = 0xc;
    IVar10 = 0x13f;
    WVar8 = 0x0;
    pcVar7 = load_string_1010_847e(_u16_1050_14cc,0x649);
    LVar6 = SendDlgItemMessage16((LPARAM)pcVar7,WVar8,UVar9,IVar10,HVar1);
    paVar3 = (astruct_57 *)CONCAT22(uVar4,(int)((u32)LVar6 >> 0x10));
    HVar1 = GetDlgItem16(0x146,param_2->field6_0x6);
    EnableWindow16(0x0,HVar1);
    iVar2 = pass1_1010_659a((u32)puVar5,0x86);
    if (iVar2 == 0x0) {
      HVar1 = GetDlgItem16(0x14a,param_2->field6_0x6);
      uVar4 = ((u32)paVar3 >> 0x10);
      EnableWindow16(0x0,HVar1);
      HVar1 = param_2->field6_0x6;
      UVar9 = 0xc;
      IVar10 = 0x144;
      WVar8 = 0x0;
      pcVar7 = load_string_1010_847e(_u16_1050_14cc,0x531);
      LVar6 = SendDlgItemMessage16((LPARAM)pcVar7,WVar8,UVar9,IVar10,HVar1);
      paVar3 = (astruct_57 *)CONCAT22(uVar4,(int)((u32)LVar6 >> 0x10));
    }
    puVar5 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe6,0x2),in_stack_0000fe8e,
                             in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
    if (((int)puVar5 + 0x20) != 0x0) {
      PostMessage16(0x0,0xaf,0x111,HWND16_1050_0396);
    }
    break;
  case 0x147:
    HVar1 = GetDlgItem16(0x148,param_2->field6_0x6);
    uVar2 = EnableWindow16(0x1,HVar1);
    uStack6 = 0x1410647;
    uVar11 = 0x1f5;
    goto LAB_1038_d490;
  case 0x148:
    HVar1 = GetDlgItem16(0x149,param_2->field6_0x6);
    uVar2 = EnableWindow16(0x1,HVar1);
    uStack6 = 0x1420647;
    uVar11 = 0x1f2;//
LAB_1038_d490:
    win_1008_5c5c(uVar2,param_1,_u16_1050_02a0,uVar11);
    break;
  case 0x149:
    uStack6 = 0x1430648;
    PostMessage16(0x0,0xb8,0x111,HWND16_1050_0396);
    DestroyWindow16(param_2->field6_0x6);
    break;
  case 0x14a:
    HVar1 = GetDlgItem16(0x145,param_2->field6_0x6);
    EnableWindow16(0x1,HVar1);
    HVar1 = param_2->field6_0x6;
    UVar9 = 0xc;
    IVar10 = 0x140;
    WVar8 = 0x0;
    pcVar7 = load_string_1010_847e(_u16_1050_14cc,0x649);
    SendDlgItemMessage16((LPARAM)pcVar7,WVar8,UVar9,IVar10,HVar1);
    break;
  case 0x14b:
    HVar1 = GetDlgItem16(0x147,param_2->field6_0x6);
    EnableWindow16(0x1,HVar1);
    break;
  default:
    post_win_msg_1040_7b3c((StructC *)CONCAT22(CONCAT11(param_4,param_3),param_2),param_5,param_6,(int)param_7);
    return;
  }
  if (((uStack6 != 0x0) && (uStack6 != 0x0)) && (BVar2 = IsWindow16(param_2->field6_0x6), BVar2 != 0x0)) {
    HVar1 = param_2->field6_0x6;
    WVar8 = 0x0;
    UVar9 = 0xc;
    pcVar7 = load_string_1010_847e(_u16_1050_14cc,uStack6);
    SendDlgItemMessage16((LPARAM)pcVar7,WVar8,UVar9,uStack6,HVar1);
  }
  if (WStack8 != 0x0) {
    PostMessage16(0x0,WStack8,0x111,HWND16_1050_0396);
  }
  return;
}



StructD * pass1_1038_d6c4(StructD *param_1,u8 param_2)

{
  pass1_1038_d276(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * pass1_1038_d756(StructD *param_1,param_2: *mut astruct_57,mut param_3: u16 )

{
  code **ppcVar1;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar2;
  astruct_57 *iVar2;
  let mut unaff_BP: u16;
  astruct_57 *uVar2;
  u32 *puVar3;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,0x0,0x11b,param_3);
  uVar2 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar2 = (astruct_57 *)param_2;
  (iVar2 + 0x1)->field0_0x0 = 0x0;
  iVar2[0x1].field1_0x2 = 0x0;
  (u32)&iVar2[0x1].field2_0x4 = 0x0;
  (u32)&iVar2[0x1].field4_0x8 = 0x0;
  param_2->field0_0x0 = 0xe0d4;
  iVar2->field1_0x2 = &u16_1050_1038;
  puVar3 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x2b),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  iVar2[0x1].field2_0x4 = puVar3;
  iVar2[0x1].field3_0x6 = ((u32)puVar3 >> 0x10);
  ppcVar1 = (code **)((int)*(u32*)&iVar2[0x1].field2_0x4 + 0x4);
  (**ppcVar1)();
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_d7d0(StructD *param_1)

{
  StructD *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0xe0d4;
  iVar1->address_offset_field_0x2 = &u16_1050_1038;
  if (&iVar1->field_0x90 != 0x0) {
    pass1_1010_1ea6(_u16_1050_02a0,param_1);
  }
  if (*(i32 *)&iVar1->field_0x92 != 0x0) {
    pass1_1010_1ea6((u32)&iVar1->field_0x92,param_1);
  }
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar1->field_0x6);
  fn_ptr_1000_17ce(*(char **)&iVar1->field_0x96);
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn post_win_msg_1038_d840(param_1: *mut astruct_70,mut param_2: u16 )

{
  astruct_70 *iVar1;
  astruct_70 *uVar1;

  iVar1 = (astruct_70 *)param_1;
  uVar1 = (astruct_70 *)((u32)param_1 >> 0x10);
  if (param_2 == 0x10) {
    if (iVar1->field142_0x8e != 0x0) {
      PostMessage16(0x0,iVar1->field142_0x8e,0x111,*(HWND16 *)&iVar1->field_0x6);
      iVar1->field142_0x8e = 0x0;
      return;
    }
  }
  else if (param_2 < 0x11) {
    if ((char)param_2 == '\x01') {
      iVar1->field143_0x90 = 0x0;
      iVar1->field144_0x92 = 0x0;
      return;
    }
    if ((char)param_2 == '\x03') {
      pass1_1038_e03e((u32)param_1);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn FUN_1038_d8ae(StructD *param_1,mut param_2: u16 ,StructB *struct_b_param_2,mut param_4: u16 ,mut param_5: u16 )

{
  LPVOID pvVar1;
  let mut uVar2: u32;
  u32 *puVar3;
  astruct_57 *rect;
  let mut uVar4: u16;
  astruct_57 *paVar5;
  StructB *struct_b_1;
  let mut iVar6: i16;
  let mut uVar7: u16;
  let mut uVar9: u16;
  let mut uVar8: u16;
  let mut in_stack_0000fe24: u16;
  let mut in_stack_0000fe28: u16;
  let mut in_stack_0000fe78: u16;
  let mut in_stack_0000ff4e: u16;
  let mut in_stack_0000ff52: u16;
  let mut in_stack_0000ff56: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffa6: u16;
  let mut local_26: u16;
  let mut uStack36: u16;
  let mut uStack34: u16;
  let mut uStack32: u16;
  let mut puStack30: *mut u16;
  u32 *puStack14;
  let mut iStack10: i16;
  let mut uStack8: u16;
  HCURSOR16 HStack6;
  HCURSOR16 HStack4;

  HStack4 = LoadCursor16((char *)0x7f02,0x0);
  HStack6 = SetCursor16(HStack4);
  dialog_ui_fn_1040_78e2(struct_b_param_2);
  uVar7 = ((u32)struct_b_param_2 >> 0x10);
  struct_b_1 = (StructB *)struct_b_param_2;
  uStack8 = pass1_1010_0886();
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_1);
  }
  else {
    param_1 = (StructD *)((u32)param_1 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
  }
  puStack30 = (u16 *)CONCAT22(param_1,PTR_LOOP_1050_5f2c);
  puVar3 = (u32 *)fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4,0x0,0x1,PTR_LOOP_1050_5f2c,param_1);
  struct_b_1[0x7].field5_0xa = puVar3;
  struct_b_1[0x7].field6_0xc = (u8 *)param_1;
  for (iStack10 = 0x1; iStack10 <= (int)uStack8; iStack10 += 0x1) {
    uVar2 = (u32)&struct_b_1[0x7].lpvoid_field_0x8;
    puStack30 = (u16 *)pass1_1010_08e2(uVar2,((u32)uVar2 >> 0x10),iStack10);
    paVar5 = (astruct_57 *)((u32)param_1 & 0xffff0000 | (u32)puStack30 >> 0x10);
    local_26 = *puStack30;
    uStack36 = ((int)puStack30 + 0x2);
    uStack34 = 0x1;
    uStack32 = 0x1;
    rect = (astruct_57 *)&local_26;
    MapDialogRect16((RECT16 *)rect,(HWND16)&DAT_1050_1050);
    mem_op_1000_179c(0x42,paVar5);
    uVar4 = (astruct_57 *)paVar5 | rect;
    param_1 = (StructD *)((u32)paVar5 & 0xffff0000 | (u32)uVar4);
    if (uVar4 == 0x0) {
      uVar2 = (u32)&struct_b_1[0x7].field5_0xa;
      (u32)((int)uVar2 + iStack10 * 0x4) = 0x0;
    }
    else {
      pvVar1 = struct_b_1->lpvoid_field_0x8;
      pass1_1008_3bd6((u32)param_1,rect,(astruct_57 *)paVar5,0x0,CONCAT22(local_26,uStack36),0x101,0xff0100,
                      CONCAT13((char)(pvVar1 >> 0x8),CONCAT12((char)pvVar1,((int)puStack30 + 0x4)))
                      ,param_4,in_stack_0000fe24,in_stack_0000fe28,in_stack_0000ff4e,in_stack_0000ff52,in_stack_0000ff56
                     );
      uVar2 = (u32)&struct_b_1[0x7].field5_0xa;
      uVar8 = ((u32)uVar2 >> 0x10);
      iVar6 = (int)uVar2;
      *(astruct_57 **)(iVar6 + iStack10 * 0x4) = rect;
      (iVar6 + iStack10 * 0x4 + 0x2) = (int)param_1;
    }
    uVar2 = (u32)&struct_b_1[0x7].field5_0xa;
    uVar9 = ((u32)uVar2 >> 0x10);
    iVar6 = (int)uVar2;
    if (*(i32 *)(iVar6 + iStack10 * 0x4) != 0x0) {
      uVar2 = (u32)(iVar6 + iStack10 * 0x4);
      ((int)uVar2 + 0x3e) = 0x1;
      uVar2 = (u32)&struct_b_1[0x7].field5_0xa;
      enable_win_1040_9234((u32)((int)uVar2 + iStack10 * 0x4),*(BOOL16 *)((int)puStack30 + 0x6));
    }
  }
  puStack14 = mixed_1010_20ba((astruct_57 *)param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_5,0x2),in_stack_0000fe78
                              ,in_stack_0000ff9c,in_stack_0000ffa2,in_stack_0000ffa6);
  SetWindowText16((u32)((int)puStack14 + 0x68),(HWND16)struct_b_1->lpvoid_field_0x8);
  ShowWindow16(0x5,(HWND16)struct_b_1->lpvoid_field_0x8);
  SetCursor16(HStack6);
  return;
}



// WARNING: Removing unreachable block (ram,0x1038dad3)
// WARNING: Removing unreachable block (ram,0x1038daea)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn unk_win_sys_op_1038_da68(StructD *param_1,mut param_2: i16,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar5;
  let mut in_BX: u16;
  let mut uVar6: u16;
  let mut uVar7: u32;
  u8 uVar8;
  let mut iVar9: i16;
  u32 *puStack14;
  let mut uStack8: u16;
  let mut iStack4: i16;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar8 = (u8)(param_3 >> 0x8);
  if (param_4 == 0x204) {
    pass1_1038_de20(in_BX,param_1,CONCAT13(uVar8,CONCAT12((char)param_3,param_2)),0x204,param_5,
                    param_5);
    return;
  }
  iStack4 = 0x0;
  uStack8 = 0x0;
  if (param_5 == 0x121) {
    iStack4 = 0x6ec;
    uStack8 = 0x15;
    goto LAB_1038_dac3;
  }
  if (param_5 < 0x1220000) {
    uVar2 = param_5 - 0x100;
    if (uVar2 == 0x0) {
      param_5 = uVar2;
      if ((param_2 + 0x8e) == 0x0) {
        pass1_1010_1ea6((u32)_u16_1050_02a0,(StructD *)CONCAT22(param_3,param_2));
        (param_2 + 0x90) = 0x0;
      }
      iStack4 = 0x72c;
      uStack8 = 0x48;
      goto LAB_1038_dac3;
    }
    if (param_5 - 0x11c == 0x0) {
      param_5 = param_5 - 0x11c;
      pass1_1038_df86(param_1,CONCAT22(param_3,param_2));
      goto LAB_1038_dac3;
    }
    if (param_5 == 0x11d) {
      uVar7 = pass1_1038_df5c(param_1,CONCAT22(param_3,param_2));
      paVar5 = (astruct_57 *)((u32)paVar5 & 0xffff0000 | uVar7 >> 0x10);
      param_5 = uVar7;
      goto LAB_1038_dac3;
    }
    if (param_5 == 0x11e) {
      iVar9 = 0x1d;
    }
    else {
      if (param_5 != 0x120) {//
LAB_1038_dc20:
        post_win_msg_1040_7b3c
                  ((StructC *)CONCAT13(uVar8,CONCAT12((char)param_3,param_2)),param_4,param_5,param_5);
        return;
      }
      iVar9 = 0x1c;
    }
  }
  else if (param_5 == 0x122) {
    iVar9 = 0x14;
  }
  else {
    if (param_5 != 0x123) {
      if (param_5 - 0x125 == 0x0) {
        ppcVar1 = (code **)((int)*_u16_1050_02a0 + 0x4);
        param_5 = param_5 - 0x125;
        (**ppcVar1)();
        (param_2 + 0x90) = 0x1;
        win_1008_5c5c(param_5,paVar5,(u32)_u16_1050_02a0,0x1db);
        (param_2 + 0x8e) = 0x100;
      }
      else {
        iVar9 = param_5 - 0x126;
        if (iVar9 == 0x0) {
          (param_2 + 0x8e) = 0x0;
          win_1008_5c7c(0x0,param_1,(u32)_u16_1050_02a0,0xcb0001);
          uVar3 = FUN_1010_830a(iVar9,paVar5,0x1008,_u16_1050_14cc,0x1f8);
          param_5 = WinHelp16(0x0,0x3,(char *)CONCAT22((int)paVar5,uVar3),*(HWND16 *)(param_2 + 0x6));
        }
        else {
          if (param_5 - 0x127 != 0x0) goto LAB_1038_dc20;
          param_5 = param_5 - 0x127;
          post_win_msg_1038_dcb0(0x0,paVar5,CONCAT22(param_3,param_2));
        }
      }
      goto LAB_1038_dac3;
    }
    iVar9 = 0x28;
  }
  uVar7 = pass1_1038_af40(param_2,param_1,_PTR_LOOP_1050_5b7c,(param_2 + 0x8),iVar9);
  paVar5 = (astruct_57 *)((u32)paVar5 & 0xffff0000 | uVar7 >> 0x10);
  param_5 = uVar7;//
LAB_1038_dac3:
  if (iStack4 != 0x0) {
    mem_op_1000_179c(0xb4,paVar5);
    uVar4 = paVar5 | param_5;
    uVar7 = (u32)paVar5 & 0xffff0000 | (u32)uVar4;
    if (uVar4 == 0x0) {
      uVar6 = 0x1000;
      iVar9 = 0x0;
      uVar3 = 0x0;
    }
    else {
      uVar6 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar9 = string_1040_8520(uVar7,(astruct_57 *)
                                     CONCAT13((char)((u32)paVar5 >> 0x8),CONCAT12((char)paVar5,param_5)),
                               (param_2 + 0x6),0x20000,CONCAT22(iStack4,0x634));
      uVar3 = uVar7;
    }
    puStack14 = (u32 *)CONCAT22(uVar3,iVar9);
    if (uStack8 == 0x0) {
      ppcVar1 = (code **)((int)*puStack14 + 0x74);
      (**ppcVar1)(uVar6,iVar9,uVar3);
    }
    else {
      pass1_1008_941a((u16 *)CONCAT22(0x1050,&stack0xffea),0x1,uStack8);
      ppcVar1 = (code **)((int)*puStack14 + 0x6c);
      (**ppcVar1)(0x1008,(char)iVar9,uVar3,&stack0xffea,(int)&DAT_1050_1050);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn post_win_msg_1038_dcb0(mut param_1: u16 ,param_2: *mut astruct_57,mut param_3: u32)

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  astruct_57 *paVar7;
  let mut uVar8: u32;
  let mut puVar9: *mut u16;
  astruct_67 *paVar10;
  let mut in_stack_0000fe72: u16;
  let mut in_stack_0000ff96: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ffa0: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut uVar13: u16;
  u8 uVar14;
  u8 uVar15;
  let mut local_18: u16;
  let mut uStack22: u16;
  u8 local_14 [0x4];
  let mut uStack16: u16;
  let mut uStack14: u16;
  let mut uStack12: u16;
  u8 local_a [0x4];
  u32 *puStack6;

  mem_op_1000_179c(0xb4,param_2);
  uStack14 = param_2;
  uVar8 = (u32)param_2 & 0xffff0000;
  uVar6 = uVar8 | (uStack14 | param_1);
  iVar4 = (int)param_3;
  uVar5 = (param_3 >> 0x10);
  uStack16 = param_1;
  if ((uStack14 | param_1) == 0x0) {
    iVar2 = 0x0;
  }
  else {
    iVar2 = string_1040_8520(uVar6,(astruct_57 *)CONCAT22(uStack14,param_1),(iVar4 + 0x6),0x30004,0x7260634);
    uVar8 = uVar6;
  }
  puStack6 = (u32 *)CONCAT22((int)uVar8,iVar2);
  puVar9 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_a),0x1,0x49);
  paVar7 = (astruct_57 *)(uVar8 & 0xffff0000 | (u32)puVar9 >> 0x10);
  ppcVar1 = (code **)((int)*puStack6 + 0x6c);
  uVar3 = (**ppcVar1)(0x1008,(int)puStack6,(int)((u32)puStack6 >> 0x10),local_a,(int)&DAT_1050_1050);
  uStack12 = uVar3;
  if (uVar3 == 0x6) {
    mem_op_1000_179c(0xb4,paVar7);
    uStack14 = paVar7;
    uVar8 = (u32)paVar7 & 0xffff0000;
    uVar6 = uVar8 | (uStack14 | uVar3);
    uStack16 = uVar3;
    if ((uStack14 | uVar3) == 0x0) {
      iVar4 = 0x0;
    }
    else {
      iVar4 = string_1040_8520(uVar6,(astruct_57 *)CONCAT13((char)((u32)paVar7 >> 0x8),CONCAT12((char)paVar7,uVar3)),
                               (iVar4 + 0x6),0x20000,0x7280634);
      uVar8 = uVar6;
    }
    puStack6 = (u32 *)CONCAT22((int)uVar8,iVar4);
    puVar9 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_14),0x1,0x4a);
    paVar7 = (astruct_57 *)(uVar8 & 0xffff0000 | (u32)puVar9 >> 0x10);
    ppcVar1 = (code **)((int)*puStack6 + 0x6c);
    (**ppcVar1)(0x1008,(int)puStack6,(int)((u32)puStack6 >> 0x10),(char)local_14,(int)&DAT_1050_1050);
    uVar14 = 0x0;
    uVar15 = 0x0;
    iVar2 = 0x15;
    uVar12 = 0x1;
    uVar13 = 0x0;
    uVar11 = 0x0;
    iVar4 = 0x0;
    uVar5 = 0x0;
    paVar10 = (astruct_67 *)
              mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)0x37,in_stack_0000fe72,in_stack_0000ff96,
                              in_stack_0000ff9c,in_stack_0000ffa0);
    uStack22 = ((u32)paVar10 >> 0x10);
    local_18 = SUB42(paVar10,0x0);
    post_win_msg_1008_a0e4(paVar10,CONCAT22(uVar11,uVar5),iVar4,uVar12,CONCAT13(uVar15,CONCAT12(uVar14,uVar13)),iVar2);
    PostMessage16(0x0,0xfc,0x111,HWND16_1050_0396);
    return;
  }
  mem_op_1000_179c(0xb4,paVar7);
  uStack14 = paVar7;
  uVar8 = (u32)paVar7 & 0xffff0000 | (u32)(uStack14 | uVar3);
  uStack16 = uVar3;
  if ((uStack14 | uVar3) == 0x0) {
    iVar4 = 0x0;
    uVar5 = 0x0;
  }
  else {
    iVar4 = string_1040_8520(uVar8,(astruct_57 *)CONCAT13((char)((u32)paVar7 >> 0x8),CONCAT12((char)paVar7,uVar3)),
                             (iVar4 + 0x6),0x20000,0x7290634);
    uVar5 = uVar8;
  }
  puStack6 = (u32 *)CONCAT22(uVar5,iVar4);
  pass1_1008_941a((u16 *)CONCAT22(0x1050,&local_18),0x1,0x4b);
  ppcVar1 = (code **)((int)*puStack6 + 0x6c);
  (**ppcVar1)(0x1008,(int)puStack6,(int)((u32)puStack6 >> 0x10),(char)&local_18,(int)&DAT_1050_1050);
  return;
}
pub fn pass1_1038_de20(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: i16)

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar4;
  let mut uVar5: u32;
  let mut uVar6: u16;
  u8 local_12 [0x4];
  let mut uStack14: u16;
  let mut uStack12: u16;
  u32 *puStack10;
  let mut uStack6: u16;
  let mut iStack4: i16;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  iStack4 = 0x644;
  uStack6 = 0x0;
  switch(param_6 + -0x11c) {
  case 0x0:
    iStack4 = 0x635;
    uStack6 = 0x3a;
    break;
  case 0x1:
    iStack4 = 0x636;
    uStack6 = 0x3b;
    break;
  case 0x2:
    iStack4 = 0x637;
    uStack6 = 0x3c;
    break;
  case 0x4:
    iStack4 = 0x639;
    uStack6 = 0x3e;
    break;
  case 0x5:
    iStack4 = 0x63a;
    uStack6 = 0x3f;
    break;
  case 0x6:
    iStack4 = 0x63b;
    uStack6 = 0x40;
    break;
  case 0x7:
    iStack4 = 0x640;
    uStack6 = 0x45;
    break;
  case 0x9:
    iStack4 = 0x642;
    uStack6 = 0x47;
    break;
  case 0xa:
    iStack4 = 0x641;
    uStack6 = 0x46;
    break;
  case 0xb:
    iStack4 = 0x63f;
    uStack6 = 0x44;
  }
  if (iStack4 != 0x0) {
    uVar6 = 0x1000;
    mem_op_1000_179c(0xb4,paVar4);
    uStack12 = paVar4;
    uVar5 = (u32)paVar4 & 0xffff0000 | (u32)(uStack12 | param_1);
    uStack14 = param_1;
    if ((uStack12 | param_1) == 0x0) {
      iVar2 = 0x0;
      uVar3 = 0x0;
    }
    else {
      uVar6 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar2 = string_1040_8520(uVar5,(astruct_57 *)CONCAT22(uStack12,param_1),((int)param_3 + 0x6),0x20000,
                               CONCAT22(iStack4,0x634));
      uVar3 = uVar5;
    }
    puStack10 = (u32 *)CONCAT22(uVar3,iVar2);
    if (uStack6 == 0x0) {
      ppcVar1 = (code **)((int)*puStack10 + 0x74);
      (**ppcVar1)(uVar6,iVar2,uVar3);
    }
    else {
      pass1_1008_941a((u16 *)CONCAT22(0x1050,local_12),0x1,uStack6);
      ppcVar1 = (code **)((int)*puStack10 + 0x6c);
      (**ppcVar1)(0x1008,(char)puStack10,(int)((u32)puStack10 >> 0x10),local_12,(int)&DAT_1050_1050);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1038_df5c(mut param_1: u16 ,mut param_2: u32) -> u32

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u32;

  uVar2 = (param_2 >> 0x10);
  uVar1 = param_2;
  pass1_1010_038e(*(astruct_27 **)(uVar1 + 0x92),0x1);
  uVar3 = pass1_1038_af40(uVar1,param_1,_PTR_LOOP_1050_5b7c,(uVar1 + 0x8),0x20);
  return uVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_df86(StructD *param_1,mut param_2: u32)

{
  char *pcVar1;
  code **ppcVar2;
  let mut BVar3: bool;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  let mut uVar7: u32;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  u32 *puVar11;
  char *pcVar12;
  astruct_57 *paVar13;
  let mut in_stack_0000fe8e: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbc: u16;
  let mut in_stack_0000ffe6: u16;
  u32 *puStack22;

  paVar13 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  puVar11 = mixed_1010_20ba(paVar13,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe6,0x2),in_stack_0000fe8e,
                            in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  uVar7 = (u32)paVar13 & 0xffff0000 | (u32)puVar11 >> 0x10;
  pcVar1 = *(char **)((int)puVar11 + 0x68);
  uVar9 = (param_2 >> 0x10);
  uVar8 = param_2;
  BVar3 = pass1_1010_041a();
  if (BVar3 != 0x0) {
    pass1_1010_038e(*(astruct_27 **)(uVar8 + 0x92),0x1);
    pass1_1038_af40(uVar8,uVar7,_PTR_LOOP_1050_5b7c,(uVar8 + 0x8),0x1e);
    return;
  }
  pcVar12 = load_string_1010_847e(_u16_1050_14cc,0x7d5);
  paVar13 = (astruct_57 *)(uVar7 & 0xffff0000 | (u32)pcVar12 >> 0x10);
  uVar4 = pcVar12;
  uVar10 = 0x1000;
  mem_op_1000_179c(0xb4,paVar13);
  uVar5 = paVar13 | uVar4;
  if (uVar5 == 0x0) {
    uVar9 = 0x0;
    uVar6 = 0x0;
  }
  else {
    uVar10 = SUB42(&PTR_LOOP_1050_1040,0x0);
    paVar13 = pass1_1040_8478(uVar5,(astruct_57 *)CONCAT22(paVar13,uVar4),0x20,pcVar1,pcVar12,
                              (uVar8 + 0x6));
    uVar6 = ((u32)paVar13 >> 0x10);
    uVar9 = SUB42(paVar13,0x0);
  }
  puStack22 = (u32 *)CONCAT22(uVar6,uVar9);
  ppcVar2 = (code **)((int)*puStack22 + 0x74);
  (**ppcVar2)(uVar10,uVar9,uVar6);
  return;
}
pub fn pass1_1038_e03e(mut param_1: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  let mut iVar3: i16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut uVar6: u32;
  let mut iStack6: i16;

  uVar4 = (param_1 >> 0x10);
  uVar2 = pass1_1010_0886();
  for (iStack6 = 0x1; iStack6 <= (int)uVar2; iStack6 += 0x1) {
    uVar1 = (u32)((int)param_1 + 0x92);
    uVar6 = pass1_1010_08e2(uVar1,((u32)uVar1 >> 0x10),iStack6);
    uVar1 = (u32)((int)param_1 + 0x96);
    uVar5 = ((u32)uVar1 >> 0x10);
    iVar3 = (int)uVar1;
    if (*(i32 *)(iVar3 + iStack6 * 0x4) != 0x0) {
      enable_win_1040_9234((u32)(iVar3 + iStack6 * 0x4),*(BOOL16 *)((int)uVar6 + 0x6));
    }
  }
  return;
}



StructD * pass1_1038_e0ae(StructD *param_1,u8 param_2)

{
  pass1_1038_d7d0(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_57 * pass1_1038_e140(param_1: *mut astruct_57,mut param_2: u32,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfc2,param_5);
  param_1->field0_0x0 = 0xe264;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_e16e(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xe264;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn check_radio_btn_show_win_1038_e19a(StructB *param_1)

{
  let mut uVar1: u16;

  dialog_ui_fn_1040_78e2(param_1);
  uVar1 = ((u32)param_1 >> 0x10);
  CheckRadioButton16(0x1807,0x1807,0x1807,*(HWND16 *)((int)param_1 + 0x6));
  move_win_1040_826c(param_1,0xc8,0xc8);
  ShowWindow16(0x5,*(HWND16 *)((int)param_1 + 0x6));
  return;
}
pub fn destroy_win_1038_e1dc(param_1: *mut astruct_886,mut param_2: u16 ,mut param_3: i16)

{
  let mut UVar1: u16;
  let mut uVar2: u32;

  if (param_3 != 0x0) {
    UVar1 = IsDlgButtonChecked(0x1807,param_1->field6_0x6);
    if (UVar1 == 0x0) {
      UVar1 = IsDlgButtonChecked(0x1806,param_1->field6_0x6);
      if (UVar1 == 0x0) goto LAB_1038_e229;
      uVar2 = 0x1110130;
    }
    else {
      uVar2 = 0x111012f;
    }
    SendMessage16(0x0,(WPARAM16)uVar2,((u32)uVar2 >> 0x10),HWND16_1050_0396);
  }//
LAB_1038_e229:
  DestroyWindow16(param_1->field6_0x6);
  return;
}
pub fn FUN_1038_e23a(void)

{
  return;
}



StructD * pass1_1038_e23e(StructD *param_1,u8 param_2)

{
  pass1_1038_e16e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_57 * pass1_1038_e2d0(param_1: *mut astruct_57,mut param_2: u16 )

{
  astruct_57 *uVar1;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x1c3,param_2);
  uVar1 = (astruct_57 *)((u32)param_1 >> 0x10);
  (u32)((int)param_1 + 0x8e) = 0x0;
  param_1->field0_0x0 = 0xe62e;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_e308(StructD *param_1)

{
  StructD *iVar1;
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0xe62e;
  iVar1->address_offset_field_0x2 = &u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar1->field_0x6);
  fn_ptr_1000_17ce(*(char **)&iVar1->field_0x8e);
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn win_ui_op_1038_e348(StructB *param_1,u8 param_2,StructD *param_3,mut param_4: u16 )

{
  LPVOID pvVar1;
  let mut uVar3: u32;
  let mut uVar5: u16;
  let mut uVar4: u16;
  astruct_57 *rect;
  let mut uVar7: u16;
  StructD *uVar6;
  let mut uVar11: u16;
  StructB *struct_b_5;
  let mut iVar12: i16;
  let mut unaff_SI: u16;
  let mut uVar8: u16;
  let mut uVar10: u16;
  let mut uVar9: u16;
  let mut in_stack_0000fe2a: u16;
  let mut in_stack_0000fe2e: u16;
  let mut in_stack_0000fe7e: u16;
  let mut in_stack_0000ff54: u16;
  let mut in_stack_0000ff58: u16;
  let mut in_stack_0000ff5c: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffa8: u16;
  let mut in_stack_0000ffac: u16;
  let mut local_22: u16;
  let mut uStack32: u16;
  let mut uStack30: u16;
  let mut uStack28: u16;
  let mut puStack26: *mut u16;
  let mut iStack10: i16;
  let mut uStack8: u16;
  u32 *puStack6;
  let mut uVar2: u32;
  astruct_57 *paVar8;

  dialog_ui_fn_1040_78e2(param_1);
  uVar6 = param_3;
  puStack6 = mixed_1010_20ba((astruct_57 *)param_3,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2b),
                             in_stack_0000fe7e,in_stack_0000ffa2,in_stack_0000ffa8,in_stack_0000ffac);
  uVar6 = (StructD *)((u32)uVar6 & 0xffff0000 | (u32)puStack6 >> 0x10);
  uStack8 = pass1_1010_088c();
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(uVar6);
  }
  else {
    uVar6 = (StructD *)((u32)uVar6 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
  }
  puStack26 = (u16 *)CONCAT22(uVar6,PTR_LOOP_1050_5f2c);
  uVar4 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4,0x0,0x1,PTR_LOOP_1050_5f2c,uVar6);
  uVar8 = ((u32)param_1 >> 0x10);
  struct_b_5 = (StructB *)param_1;
  struct_b_5[0x7].field1_0x2 = uVar4;
  struct_b_5[0x7].hwnd_0x6 = (HWND16)uVar6;
  for (iStack10 = 0x1; uVar11 = ((u32)uVar6 >> 0x10), iStack10 <= (int)uStack8; iStack10 += 0x1) {
    puStack26 = (u16 *)pass1_1010_091e(puStack6,((u32)puStack6 >> 0x10),iStack10);
    uVar5 = ((u32)puStack26 >> 0x10);
    paVar8 = (astruct_57 *)CONCAT22(uVar11,uVar5);
    local_22 = *puStack26;
    uStack32 = ((int)puStack26 + 0x2);
    uStack30 = 0x1;
    uStack28 = 0x1;
    rect = (astruct_57 *)&local_22;
    MapDialogRect16((RECT16 *)rect,(HWND16)&DAT_1050_1050);
    mem_op_1000_179c(0x42,paVar8);
    uVar7 = (astruct_57 *)paVar8 | rect;
    uVar6 = (StructD *)((u32)paVar8 & 0xffff0000 | (u32)uVar7);
    if (uVar7 == 0x0) {
      uVar3 = (u32)&struct_b_5[0x7].field1_0x2;
      (u32)((int)uVar3 + iStack10 * 0x4) = 0x0;
    }
    else {
      pvVar1 = struct_b_5->lpvoid_field_0x8;
      pass1_1008_3bd6((u32)uVar6,rect,(astruct_57 *)paVar8,0x0,CONCAT22(local_22,uStack32),0x101,0xff0100,
                      CONCAT13((char)(pvVar1 >> 0x8),CONCAT12((char)pvVar1,((int)puStack26 + 0x4)))
                      ,param_4,in_stack_0000fe2a,in_stack_0000fe2e,in_stack_0000ff54,in_stack_0000ff58,in_stack_0000ff5c
                     );
      uVar2 = (u32)&struct_b_5[0x7].field1_0x2;
      uVar9 = ((u32)uVar2 >> 0x10);
      iVar12 = (int)uVar2;
      *(astruct_57 **)(iVar12 + iStack10 * 0x4) = rect;
      (iVar12 + iStack10 * 0x4 + 0x2) = (int)uVar6;
    }
    uVar3 = (u32)&struct_b_5[0x7].field1_0x2;
    uVar10 = ((u32)uVar3 >> 0x10);
    iVar12 = (int)uVar3;
    if (*(i32 *)(iVar12 + iStack10 * 0x4) != 0x0) {
      enable_win_1040_9234((u32)(iVar12 + iStack10 * 0x4),*(BOOL16 *)((int)puStack26 + 0x6));
    }
  }
  move_win_1040_826c(param_1,-0x1,0xffff);
  ShowWindow16(0x5,(HWND16)struct_b_5->lpvoid_field_0x8);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_e4bc(u8 *param_1,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar6;
  let mut uVar7: u32;
  let mut uVar8: u32;
  let mut unaff_SI: u16;
  u32 *puVar9;
  u32 *puVar10;
  astruct_57 *paVar11;
  let mut in_stack_0000fe66: u16;
  let mut in_stack_0000fe68: u16;
  let mut in_stack_0000fe72: u16;
  let mut in_stack_0000ff8a: u16;
  let mut in_stack_0000ff8c: u16;
  let mut in_stack_0000ff90: u16;
  let mut in_stack_0000ff92: u16;
  let mut in_stack_0000ff94: u16;
  let mut in_stack_0000ff96: u16;
  let mut in_stack_0000ff9c: u16;
  let mut in_stack_0000ffa0: u16;
  let mut uVar12: u16;
  u8 uVar13;
  u8 uVar14;
  let mut uVar15: u16;
  let mut uVar16: u16;
  let mut uVar17: u16;
  u32 *puStack22;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  if (param_4 == 0x1c4) {
    puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2f),in_stack_0000fe72,
                             in_stack_0000ff96,in_stack_0000ff9c,in_stack_0000ffa0);
    uVar15 = ((u32)puVar9 >> 0x10);
    uVar4 = ((int)puVar9 + 0x24);
    uVar5 = ((int)puVar9 + 0x26);
    uVar7 = (u32)paVar6 & 0xffff0000 | (u32)uVar5;
    uVar3 = uVar5 | uVar4;
    if (uVar3 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(uVar5,uVar4));
      uVar8 = uVar7 & 0xffff0000;
      if ((uVar7 | uVar3) != 0x0) {
        puVar10 = pass1_1008_c6fa(_u16_1050_06e0,0x20);
        paVar11 = (astruct_57 *)(uVar8 & 0xffff0000 | (u32)puVar10 >> 0x10);
        uVar4 = puVar10;
        pass1_1038_4e78(uVar4,paVar11,CONCAT22(uVar7,uVar3),puVar10);
        uVar15 = SUB42(paVar11,0x0);
        puStack22 = (u32 *)CONCAT22(uVar15,uVar4);
        uVar2 = *puStack22;
        ppcVar1 = (code **)uVar2 + 0x8;
        paVar6 = paVar11;
        uVar5 = uVar4;
        (**ppcVar1)(0x1008,uVar4,uVar15);
        uVar3 = paVar6 | uVar5;
        paVar6 = (astruct_57 *)((u32)paVar6 & 0xffff0000 | (u32)uVar3);
        if (uVar3 == 0x0) {
          if (puStack22 != NULL) {
            ppcVar1 = (code **)uVar2;
            (**ppcVar1)(0x1008,uVar4,(char)paVar11,0x1);
          }
        }
        else {
          ppcVar1 = (code **)((int)*puStack22 + 0x4);
          uVar3 = uVar4;
          (**ppcVar1)(0x8,uVar4,uVar15,0x0,0x0);
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT13((char)((u32)paVar6 >> 0x8),CONCAT12((char)paVar6,uVar5)));
          puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(uVar3,0x32),in_stack_0000fe66,
                                   in_stack_0000ff8a,in_stack_0000ff90,in_stack_0000ff94);
          pass1_1010_71d6(uVar5 + 0xc,((u32)puVar9 >> 0x10),(u32)puVar9,0x1,
                          (u16 *)(((u32)paVar6 & 0xff00) << 0x10 | (u32)CONCAT12((char)paVar6,uVar5 + 0xc)),
                          &DAT_1050_1050);
          if (puStack22 != NULL) {
            ppcVar1 = (code **)*puStack22;
            (**ppcVar1)(0x1010,uVar4,(char)paVar11,0x1);
          }
        }
      }
    }
  }
  else {
    if (param_4 == 0x1c5) {
      uVar15 = 0xe;
    }
    else {
      if (param_4 != 0x1c6) {
        post_win_msg_1040_7b3c
                  ((StructC *)CONCAT13((char)(param_3 >> 0x8),CONCAT12((char)param_3,param_2)),(param_3 >> 0x10)
                   ,param_4,param_4);
        return;
      }
      uVar15 = 0xd;
    }
    uVar17 = 0x0;
    uVar16 = 0x0;
    uVar12 = 0x0;
    uVar13 = 0x0;
    uVar14 = 0x0;
    paVar11 = (astruct_57 *)
              mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)0x32,in_stack_0000fe68,in_stack_0000ff8c,
                              in_stack_0000ff92,in_stack_0000ff96);
    unk_win_op_1010_7300
              ((u32)paVar6 & 0xffff0000 | (u32)paVar11 >> 0x10,paVar11,CONCAT13(uVar14,CONCAT12(uVar13,uVar12)),
               uVar15,CONCAT22(uVar17,uVar16));
  }
  return;
}



StructD * pass1_1038_e608(StructD *param_1,u8 param_2)

{
  pass1_1038_e308(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_e69a(StructD *param_1,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_57 *iVar1;
  let mut unaff_BP: u16;
  astruct_57 *uVar1;
  u32 *puVar2;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfcb,param_6);
  uVar1 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  iVar1[0x1].field2_0x4 = 0x0;
  param_2->field0_0x0 = 0xe92e;
  iVar1->field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x43),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_e6f0(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xe92e;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn unk_win_ui_op_1038_e71c(mut param_1: u16 ,StructB *param_2)

{
  let mut extraout_DX: u16;
  StructB *struct_1;
  let mut struct_1_lo: u16;
  char *pcStack6;

  dialog_ui_fn_1040_78e2(param_2);
  struct_1_lo = ((u32)param_2 >> 0x10);
  struct_1 = (StructB *)param_2;
  unk_load_str_op_1010_2c34((u32 *)(u32)&struct_1[0x7].field1_0x2);
  pcStack6 = (char *)CONCAT22(extraout_DX,param_1);
  unk_str_op_1000_3d3e
            ((char *)((u32)param_2 & 0xffff0000 | ZEXT24(&struct_1->field8_0x10)),
             (char *)CONCAT22(extraout_DX,param_1));
  fn_ptr_1000_17ce(pcStack6);
  move_win_1040_826c(param_2,-0x1,0xffff);
  ShowWindow16(0x5,(HWND16)struct_1->lpvoid_field_0x8);
  struct_1[0x7].lpvoid_field_0x8 = (LPVOID)((int)&PTR_LOOP_1050_0000 + 0x1);
  unk_win_msg_op_1008_9510((i16 *)((u32)param_2 & 0xffff0000 | ZEXT24(&struct_1[0x7].lpvoid_field_0x8)));
  DestroyWindow16((HWND16)struct_1->lpvoid_field_0x8);
  return;
}
pub fn chk_is_dlg_btn_checked_1038_e7a0(param_1: *mut astruct_62,mut param_2: i16)

{
  let mut uVar1: u32;
  let mut UVar2: u16;
  astruct_62 *iVar3;
  let mut uVar3: u16;

  iVar3 = (astruct_62 *)param_1;
  uVar3 = ((u32)param_1 >> 0x10);
  if (param_2 == 0x0) {
    uVar1 = iVar3->field142_0x8e;
    ((int)uVar1 + 0x10) = 0x1;
    uVar1 = iVar3->field142_0x8e;
    ((int)uVar1 + 0xa) = 0x0;
    uVar1 = iVar3->field142_0x8e;
    ((int)uVar1 + 0xc) = 0x0;
    uVar1 = iVar3->field142_0x8e;
    ((int)uVar1 + 0xe) = 0x0;
  }
  else {
    UVar2 = IsDlgButtonChecked(0x1827,*(HWND16 *)&iVar3->field_0x6);
    if (UVar2 == 0x0) {
      UVar2 = IsDlgButtonChecked(0x1828,*(HWND16 *)&iVar3->field_0x6);
      if (UVar2 == 0x0) {
        uVar1 = iVar3->field142_0x8e;
        ((int)uVar1 + 0xa) = 0x0;
      }
      else {
        uVar1 = iVar3->field142_0x8e;
        ((int)uVar1 + 0xa) = 0x2;
      }
    }
    else {
      uVar1 = iVar3->field142_0x8e;
      ((int)uVar1 + 0xa) = 0x1;
    }
    UVar2 = IsDlgButtonChecked(s_vrpal_bmp_1050_183a,*(HWND16 *)&iVar3->field_0x6);
    if (UVar2 == 0x0) {
      UVar2 = IsDlgButtonChecked((int)s_vrpal_bmp_1050_183a + 0x1,*(HWND16 *)&iVar3->field_0x6);
      if (UVar2 == 0x0) {
        uVar1 = iVar3->field142_0x8e;
        ((int)uVar1 + 0xc) = 0x0;
      }
      else {
        uVar1 = iVar3->field142_0x8e;
        ((int)uVar1 + 0xc) = 0x2;
      }
    }
    else {
      uVar1 = iVar3->field142_0x8e;
      ((int)uVar1 + 0xc) = 0x1;
    }
    UVar2 = IsDlgButtonChecked((int)s_vrpal_bmp_1050_183a + 0x2,*(HWND16 *)&iVar3->field_0x6);
    if (UVar2 == 0x0) {
      UVar2 = IsDlgButtonChecked((int)s_vrpal_bmp_1050_183a + 0x3,*(HWND16 *)&iVar3->field_0x6);
      if (UVar2 == 0x0) {
        uVar1 = iVar3->field142_0x8e;
        ((int)uVar1 + 0xe) = 0x0;
      }
      else {
        uVar1 = iVar3->field142_0x8e;
        ((int)uVar1 + 0xe) = 0x2;
      }
    }
    else {
      uVar1 = iVar3->field142_0x8e;
      ((int)uVar1 + 0xe) = 0x1;
    }
    uVar1 = iVar3->field142_0x8e;
    ((int)uVar1 + 0x10) = 0x0;
  }
  iVar3->field143_0x92 = 0x0;
  return;
}
pub fn FUN_1038_e904(void)

{
  return;
}



StructD * pass1_1038_e908(StructD *param_1,u8 param_2)

{
  pass1_1038_e6f0(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 *
pass1_1038_e99a(u8 *param_1,param_2: *mut astruct_57,mut param_3: u32,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_57 *iVar1;
  let mut unaff_BP: u16;
  astruct_57 *uVar2;
  u32 *puVar2;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,param_3,0xfb9,param_6);
  uVar2 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  param_2->field0_0x0 = 0xeb32;
  iVar1->field1_0x2 = &u16_1050_1038;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x30),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_e9ec(StructD *param_1)

{
  let mut uVar1: u16;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0xeb32;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1);
  return;
}
pub fn win_ui_op_1038_ea18(StructB *param_1)

{
  HWND16 hwnd;
  INT16 IVar1;
  StructB *iVar2;
  let mut uVar2: u16;
  let mut lparam: u32;
  let mut in_stack_0000fff0: bool;
  let mut iStack14: i16;

  dialog_ui_fn_1040_78e2(param_1);
  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (StructB *)param_1;
  lparam = pass1_1010_375e((u32)&iVar2[0x7].field1_0x2);
  hwnd = GetDlgItem16(0xfa5,(HWND16)iVar2->lpvoid_field_0x8);
  SendMessage16(lparam,0x0,0xc,hwnd);
  GetWindowRect16((RECT16 *)CONCAT22(0x1050,&stack0xfff0),iVar2->max_count_field_0x10);
  IVar1 = GetSystemMetrics16(SM_CYCAPTION);
  move_win_1040_826c(param_1,IVar1 + iStack14 + 0x5,in_stack_0000fff0);
  ShowWindow16(0x5,(HWND16)iVar2->lpvoid_field_0x8);
  return;
}
pub fn win_ui_op_1038_eaa2(param_1: *mut astruct_888,mut param_2: i16)

{
  HWND16 hwnd;
  astruct_888 *struct_1;
  let mut struct_1_lo: u16;
  LRESULT LVar1;

  struct_1 = (astruct_888 *)param_1;
  struct_1_lo = ((u32)param_1 >> 0x10);
  if (param_2 != 0x0) {
    hwnd = GetDlgItem16(0xfa5,struct_1->field6_0x6);
    LVar1 = SendMessage16(CONCAT22(0x1050,&stack0xffac),0x50,0xd,hwnd);
    pass1_1010_3770(((u32)LVar1 >> 0x10),(astruct_477 *)struct_1->field140_0x8e,
                    (char *)CONCAT22(0x1050,&stack0xffac));
    PostMessage16(0x0,0xfb,0x111,struct_1->field7_0x8);
  }
  DestroyWindow16(struct_1->field6_0x6);
  return;
}
pub fn FUN_1038_eb08(void)

{
  return;
}



StructD * pass1_1038_eb0c(StructD *param_1,u8 param_2)

{
  pass1_1038_e9ec(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_57 * pass1_1038_eb9e(param_1: *mut astruct_57,mut param_2: u16 )

{
  astruct_57 *uVar1;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x1c7,param_2);
  uVar1 = (astruct_57 *)((u32)param_1 >> 0x10);
  (u32)((int)param_1 + 0x8e) = 0x0;
  param_1->field0_0x0 = 0xee6e;
  ((int)param_1 + 0x2) = (int)&u16_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_ebd6(StructD *param_1)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  param_1->address_offset_field_0x0 = 0xee6e;
  (iVar1 + 0x2) = (int)&u16_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(iVar1 + 0x6));
  fn_ptr_1000_17ce(*(char **)(iVar1 + 0x8e));
  ui_cleanup_op_1040_782c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn FUN_1038_ec16(mut param_1: u16 ,StructB *param_2,param_3: *mut astruct_57,mut param_4: u16 )

{
  let mut uVar1: u32;
  let mut uVar2: u16;
  astruct_57 *rect;
  let mut uVar3: u16;
  let mut uVar4: u16;
  StructD *pSVar5;
  let mut iVar7: i16;
  let mut iVar8: i16;
  let mut unaff_SI: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut in_stack_0000fe2a: u16;
  let mut in_stack_0000fe2e: u16;
  let mut in_stack_0000fe7e: u16;
  let mut in_stack_0000ff54: u16;
  let mut in_stack_0000ff58: u16;
  let mut in_stack_0000ff5c: u16;
  let mut in_stack_0000ffa2: u16;
  let mut in_stack_0000ffa8: u16;
  let mut in_stack_0000ffac: u16;
  let mut local_22: u16;
  let mut uStack32: u16;
  let mut uStack30: u16;
  let mut uStack28: u16;
  let mut puStack26: *mut u16;
  let mut iStack10: i16;
  let mut uStack8: u16;
  u32 *puStack6;
  astruct_57 *paVar6;

  dialog_ui_fn_1040_78e2(param_2);
  puStack6 = mixed_1010_20ba(param_3,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2b),in_stack_0000fe7e,
                             in_stack_0000ffa2,in_stack_0000ffa8,in_stack_0000ffac);
  pSVar5 = (StructD *)((u32)param_3 & 0xffff0000 | (u32)puStack6 >> 0x10);
  uStack8 = pass1_1010_0892();
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar5);
  }
  else {
    pSVar5 = (StructD *)((u32)pSVar5 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
  }
  puStack26 = (u16 *)CONCAT22(pSVar5,PTR_LOOP_1050_5f2c);
  uVar2 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4,0x0,0x1,PTR_LOOP_1050_5f2c,pSVar5);
  uVar9 = ((u32)param_2 >> 0x10);
  iVar7 = (int)param_2;
  (iVar7 + 0x8e) = uVar2;
  (iVar7 + 0x90) = (int)pSVar5;
  for (iStack10 = 0x1; uVar10 = ((u32)pSVar5 >> 0x10), iStack10 <= (int)uStack8; iStack10 += 0x1) {
    puStack26 = (u16 *)pass1_1010_0932(puStack6,((u32)puStack6 >> 0x10),iStack10);
    uVar3 = ((u32)puStack26 >> 0x10);
    paVar6 = (astruct_57 *)CONCAT22(uVar10,uVar3);
    local_22 = *puStack26;
    uStack32 = ((int)puStack26 + 0x2);
    uStack30 = 0x1;
    uStack28 = 0x1;
    rect = (astruct_57 *)&local_22;
    MapDialogRect16((RECT16 *)rect,(HWND16)&DAT_1050_1050);
    mem_op_1000_179c(0x42,paVar6);
    uVar4 = (astruct_57 *)paVar6 | rect;
    pSVar5 = (StructD *)((u32)paVar6 & 0xffff0000 | (u32)uVar4);
    if (uVar4 == 0x0) {
      uVar1 = (u32)(iVar7 + 0x8e);
      (u32)((int)uVar1 + iStack10 * 0x4) = 0x0;
    }
    else {
      uVar10 = (iVar7 + 0x6);
      pass1_1008_3bd6((u32)pSVar5,rect,(astruct_57 *)paVar6,0x0,CONCAT22(local_22,uStack32),0x101,0xff0100,
                      CONCAT13((char)(uVar10 >> 0x8),CONCAT12((char)uVar10,((int)puStack26 + 0x4)))
                      ,param_4,in_stack_0000fe2a,in_stack_0000fe2e,in_stack_0000ff54,in_stack_0000ff58,in_stack_0000ff5c
                     );
      uVar1 = (u32)(iVar7 + 0x8e);
      uVar10 = ((u32)uVar1 >> 0x10);
      iVar8 = (int)uVar1;
      *(astruct_57 **)(iVar8 + iStack10 * 0x4) = rect;
      (iVar8 + iStack10 * 0x4 + 0x2) = (int)pSVar5;
    }
    uVar1 = (u32)(iVar7 + 0x8e);
    uVar10 = ((u32)uVar1 >> 0x10);
    iVar8 = (int)uVar1;
    if (*(i32 *)(iVar8 + iStack10 * 0x4) != 0x0) {
      enable_win_1040_9234((u32)(iVar8 + iStack10 * 0x4),*(BOOL16 *)((int)puStack26 + 0x6));
    }
  }
  move_win_1040_826c(param_2,-0x1,0xffff);
  ShowWindow16(0x5,*(HWND16 *)(iVar7 + 0x6));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn send_msg_1038_ed8a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,mut param_4: u32)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  astruct_57 *paVar6;
  u32 *puVar7;
  let mut uVar8: u32;
  let mut in_stack_0000fe8a: u16;
  let mut in_stack_0000ffae: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffb8: u16;
  HWND16 hwnd;
  let mut in_stack_0000ffe2: u16;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  hwnd = HWND16_1050_0396;
  if (param_4 != 0x1c8) {
    if (param_4 == 0x1c9) {
      puVar7 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe2,0x2f),in_stack_0000fe8a,
                               in_stack_0000ffae,in_stack_0000ffb4,in_stack_0000ffb8);
      uVar2 = ((u32)puVar7 >> 0x10);
      uVar5 = ((int)puVar7 + 0x20);
      uVar1 = ((int)puVar7 + 0x22);
      uVar8 = (u32)paVar6 & 0xffff0000 | (u32)uVar1;
      uVar3 = uVar1 | uVar5;
      if (uVar3 == 0x0) {
        return;
      }
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(uVar1,uVar5));
      uVar5 = uVar8 | uVar3;
      paVar6 = (astruct_57 *)(uVar8 & 0xffff0000 | (u32)uVar5);
      if (uVar5 == 0x0) {
        return;
      }
      iVar4 = pass1_1030_5b00(CONCAT22(uVar8,uVar3));
      puVar7 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe2,iVar4),in_stack_0000fe8a,
                               in_stack_0000ffae,in_stack_0000ffb4,in_stack_0000ffb8);
      if ((((u32)puVar7 >> 0x10) | puVar7) == 0x0) {
        return;
      }
      uVar8 = pass1_1018_0ad4((u32)puVar7);
      uVar5 = (uVar8 >> 0x10);
      if ((uVar5 | uVar8) == 0x0) {
        return;
      }
      param_4 = 0x72;
      hwnd = *(HWND16 *)(uVar8 + 0x8);
    }
    else if (param_4 != 0x1ca) {
      post_win_msg_1040_7b3c
                ((StructC *)CONCAT22((int)param_3,param_2),(param_3 >> 0x10),param_4,param_4);
      return;
    }
  }
  SendMessage16(0x0,param_4,0x111,hwnd);
  return;
}



StructD * pass1_1038_ee48(StructD *param_1,u8 param_2)

{
  pass1_1038_ebd6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1038_eeda(StructD *param_1,param_2: *mut astruct_57,mut param_3: u16 )

{
  let mut in_register_0000000a: u16;
  astruct_57 *paVar1;
  astruct_57 *iVar1;
  let mut unaff_BP: u16;
  astruct_57 *uVar1;
  u32 *puVar2;
  let mut in_stack_0000fea6: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffd0: u16;
  let mut in_stack_0000ffd4: u16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  get_sys_metrics_1040_7728(param_2,0x1,0x0,0x166,param_3);
  uVar1 = (astruct_57 *)((u32)param_2 >> 0x10);
  iVar1 = (astruct_57 *)param_2;
  (u32)(iVar1 + 0x1) = 0x0;
  iVar1[0x1].field2_0x4 = 0x0;
  iVar1[0x1].field3_0x6 = 0x0;
  param_2->field0_0x0 = 0x67c;
  iVar1->field1_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x9),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  (iVar1 + 0x1)->field0_0x0 = puVar2;
  iVar1[0x1].field1_0x2 = ((u32)puVar2 >> 0x10);
  iVar1->field86_0x74 = 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn destroy_win_1038_ef3a(StructD *param_1)

{
  let mut uVar2: u32;
  StructD *iVar1;
  StructD *uVar1;

  uVar1 = (StructD *)((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x67c;
  iVar1->address_offset_field_0x2 = &PTR_LOOP_1050_1040;
  if (*(i32 *)&iVar1->field_0x96 != 0x0) {
    uVar2 = (u32)&iVar1->field_0x96;
    DestroyWindow16(*(HWND16 *)((int)uVar2 + 0x6));
    (u32)&iVar1->field_0x96 = 0x0;
  }
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,&iVar1->field_0x6);
  ui_cleanup_op_1040_782c(param_1);
  return;
}

