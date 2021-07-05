
fn pass1_1038_0000(param_1: u32,param_2: u16,uchar *param_3)
{
  let puVar1: u32;
  let puVar2: u32;
  let puVar3: u32;
  let iVar4: i16;
  let puVar5: u32;
  let uVar6: u16;
  let puStack10: *mut u16;
  
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
  mem_op_1000_179c(0x108,param_3,0x1000);
  puStack10 = CONCAT22(param_3,param_2);
  if ((param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    (param_2 + 0x2) = 0x1008;
    uVar6 = (param_1 >> 0x10);
    (param_2 + 0x4) = (param_1 + 0x4);
    puVar3 = (param_1 + 0x8);
    puVar5 = (param_2 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_2 + 0x2) = &USHORT_1050_1028;
    *puStack10 = 0xb96;
    (param_2 + 0x2) = &PTR_LOOP_1050_1038;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_008e(param_1: u16,param_2: u16,param_3: u32,uchar *param_4,param_5: i16,
               param_6: u16)

{
  let iVar1: i16;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let puVar7: *mut u8
  let puVar8: *mut u8
  let iVar9: i16;
  let uVar10: u16;
  let puVar11: *mut u16;
  let puVar12: *mut u16;
  let iStack32: i16;
  let iStack12: i16;
  let uVar6: u32;
  
  uVar10 = (param_3 >> 0x10);
  iVar9 = param_3;
  if (*(long *)(iVar9 + 0x4) != 0x4000001) {
    return;
  }
  puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2c,param_6,param_4,param_5);
  puVar7 = (puVar11 >> 0x10);
  uVar3 = puVar11;
  puVar8 = puVar7;
  uVar4 = uVar3;
  pass1_1008_612e(0x1,0x64,uVar3);
  iStack12 = 0x0;
  iVar1 = (uVar3 + 0xa);
  if (iVar1 == 0x1) {
    iStack12 = 0x15;
  }
  else {
    if (iVar1 != 0x2) {
      if (iVar1 == 0x3) {
        iStack12 = 0x16;
      }
      else {
        if (iVar1 == 0x4) {
          if (uVar4 < 0x32) {
            iStack12 = 0x17;
          }
          else {
            iStack12 = 0x18;
          }
        }
        else {
          if (iVar1 == 0x5) {
            iStack12 = 0x19;
          }
        }
      }
    }
  }
  if (iStack12 != 0x0) {
    puVar12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_6,puVar8,param_5);
    puVar8 = (puVar12 >> 0x10);
    pass1_1010_043a(puVar12 & 0xffff | ZEXT24(puVar8) << 0x10,
                    *(long *)(iVar9 + 0x4),iStack12,param_6);
  }
  pass1_1008_eb74(puVar11,0x0,puVar8,param_5,param_6);
  if ((((uVar3 + 0xe) | (uVar3 + 0xc)) == 0x0) &&
     ((iVar9 + 0x18) < 0xc9)) {
    uVar2 = *_PTR_LOOP_1050_65e2;
    uVar4 = uVar2;
    uVar6 = uVar2;
    pass1_1008_612e(0x0,0x8,uVar4);
    uVar5 = uVar6;
    iStack32 = (uVar2 >> 0x10);
    (uVar3 + 0xc) = uVar5 + uVar4 + 0x1e;
    (uVar3 + 0xe) =
         (uVar5 >> 0xf) + iStack32 + CARRY2(uVar5,uVar4) +
         (0xffe1 < uVar5 + uVar4);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_01c0(param_1: u16,param_2: u16,param_3: u32,param_4: u16)
{
  let iVar1: i16;
  let puVar2: u32;
  code **ppcVar3;
  let uVar4: u32;
  let uVar5: u32;
  let BVar6: bool;
  let puVar7: *mut u8;
  let puVar8: u32;
  let uVar9: u32;
  let puVar10: *mut u8
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar11: u16;
  let uVar12: u16;
  let uVar13: u16;
  let uVar14: u16;
  let puVar15: u32;
  let uVar16: u32;
  let uVar17: u32;
  let uVar18: u8;
  let uStack50: u32;
  let uStack30: u32;
  let uStack18: u32;
  let local_e: [u8;2];
  let puStack12: u32;
  let uStack8: u16;
  let puStack6: *mut u8
  let iStack4: i16;
  
  iStack4 = 0x0;
  puVar15 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x29);
  puVar10 = (puVar15 >> 0x10);
  uVar12 = puVar15;
  uStack8 = uVar12;
  puStack6 = puVar10;
  pass1_1038_4e78(uVar12,puVar10,param_3,puVar15);
  puStack12 = CONCAT22(puVar10,uVar12);
  uVar14 = 0x1030;
  uVar16 = pass1_1030_bcae(local_e,param_4);
  uVar13 = uVar16;
  ppcVar3 = (code **)(*puStack12 + 0x10);
  (**ppcVar3)(0x1030,puStack12,(puStack12 >> 0x10));
  uStack18 = CONCAT22(extraout_DX,uVar13);
  uVar13 = (param_3 >> 0x10);
  puVar2 = (param_3 + 0xc);
  uVar13 = (param_3 + 0xe);
  uVar18 = SUB41(puVar2,0x0);
  ppcVar3 = (code **)(*puVar2 + 0x10);
  puVar8 = puVar2;
  (**ppcVar3)();
  uVar16 = puVar8 & 0xffff | extraout_DX_00 << 0x10;
  uStack30 = 0x0;
  uVar12 = extraout_DX_00;
  do {
    if (uStack18 <= uStack30) {
      if (puStack12 != 0x0) {
        ppcVar3 = (code **)*puStack12;
        (**ppcVar3)(uVar14,puStack12,(puStack12 >> 0x10),0x1,uVar18,
                    uVar13);
      }
      return;
    }
    uVar14 = 0x1030;
    uVar9 = uStack18;
    pass1_1030_1d58(puStack12);
    uVar5 = uVar12;
    iVar1 = (uVar9 + 0x10);
    uVar11 = uVar12;
    for (uStack50 = 0x0; uVar12 = uVar11, uStack50 < uVar16; uStack50 += 0x1) {
      uVar14 = 0x1030;
      uVar17 = uVar16;
      pass1_1030_1d58(puVar2);
      uVar4 = uVar17 & 0xffff | uVar11 << 0x10;
      uVar12 = uVar11 | uVar17;
      if ((uVar12 != 0x0) && (uVar12 = uVar11, (uVar17 + 0x10) == iVar1)) {
        uVar17 = struct_op_1030_73a8(uVar4);
        uVar12 = (uVar17 >> 0x10);
        uVar14 = 0x1008;
        BVar6 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,
                                (uVar17 + 0xc),0x30);
        if (BVar6 == 0x0) {
          puVar7 = local_e;
          uVar14 = 0x1030;
          pass1_1030_bd74(puVar7,param_4,uVar4,uVar9 & 0xffff | uVar5 << 0x10,
                          param_4);
          if (puVar7 < 0x6) {
            iStack4 += 0x1;
            break;
          }
        }
      }
      uVar11 = uVar12;
    }
    uStack30 += 0x1;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_0340(param_1: u16,param_2: u16,param_3: i16,param_4: u32,param_5: u16,
               param_6: u16,param_7: u8)

{
  let uVar1: u16;
  let uVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  u8 local_13a [0x11c];
  let uStack30: u32;
  let uStack26: u32;
  let uStack22: u32;
  let local_12: u16;
  let uStack16: u16;
  let local_e: i16;
  let uStack12: u16;
  let uStack10: u32;
  let uStack6: u32;
  
  uStack6 = *_PTR_LOOP_1050_65e2;
  uStack10 = 0x0;
  uStack12 = 0x0;
  iVar3 = param_4;
  uVar4 = (param_4 >> 0x10);
  pass1_1038_4cea(param_4,CONCAT22(param_6,&local_12),
                  CONCAT22(param_6,&local_e));
  uVar2 = (iVar3 + 0x1f6);
  uStack22 = uVar2;
  pass1_1030_38b8();
  uVar1 = uVar2;
  uStack26 = uVar2 & 0xffff | param_5 << 0x10;
  if (param_3 == 0x0) {
    if (local_e != 0x8) {
      uStack10 = (long)(uVar2 & 0xffff | param_5 << 0x10) / 0x4;
      uStack12 = 0x8;
      goto LAB_1038_054b;
    }
  }
  else {
    if (param_3 < 0xb) {
      if (local_e != 0x7) {
        uStack10 = (long)(uVar2 & 0xffff | param_5 << 0x10) / 0xa;
        uStack12 = 0x7;
        goto LAB_1038_054b;
      }
    }
    else {
      if (param_3 < 0x1a) {
        if (local_e != 0x6) {
          uStack10 = (long)(uVar2 & 0xffff | param_5 << 0x10) / 0x14;
          uStack12 = 0x6;
          goto LAB_1038_054b;
        }
      }
      else {
        if (param_3 < 0x33) {
          if (local_e != 0x5) {
            uStack10 = (long)(uVar2 & 0xffff | param_5 << 0x10) / 0x64;
            uStack12 = 0x5;
            goto LAB_1038_054b;
          }
        }
        else {
          if (param_3 < 0x4c) {
            if (uStack6 % 0x3 != 0x0) goto LAB_1038_054b;
            if (local_e != 0x4) {
              uStack10 = (long)uStack26 / 0x64;
              uStack12 = 0x4;
              goto LAB_1038_054b;
            }
          }
          else {
            if (param_3 < 0x65) {
              if (uStack6 % 0x5 != 0x0) goto LAB_1038_054b;
              if (local_e != 0x3) {
                uStack10 = (long)uStack26 / 0x64;
                uStack12 = 0x3;
                goto LAB_1038_054b;
              }
            }
            else {
              if (param_3 < 0x97) {
                if (uStack6 % 0xa != 0x0) goto LAB_1038_054b;
                if (local_e != 0x2) {
                  uStack10 = (long)uStack26 / 0x64;
                  uStack12 = 0x2;
                  goto LAB_1038_054b;
                }
              }
              else {
                if ((0xc8 < param_3) || (uStack6 % 0x14 != 0x0)) goto LAB_1038_054b;
                if (local_e != 0x1) {
                  uStack10 = (long)uStack26 / 0x64;
                  uStack12 = 0x1;
                  goto LAB_1038_054b;
                }
              }
            }
          }
        }
      }
    }
  }
  if ((uStack16 <= param_5) &&
     ((uStack16 < param_5 || (local_12 < uVar1)))) {
    uVar1 = local_12;
    param_5 = uStack16;
  }
  uStack10 = CONCAT22(param_5,uVar1);
LAB_1038_054b:
  if (uStack12 != 0x0) {
    if ((uStack26 != 0x0) && (uStack10 == 0x0)) {
      uStack10 = 0x1;
    }
    pass1_1038_4cd0(param_4,uStack10,uStack12);
  }
  if ((uStack10._2_2_ | uStack10) != 0x0) {
    if (*(long *)(iVar3 + 0x200) == 0x8000001) {
      uStack30._0_2_ = 0x2;
    }
    else {
      uStack30._0_2_ = 0x1;
    }
    uStack30 = CONCAT22(0x400,uStack30);
    pass1_1028_9944((astruct_100 *)CONCAT22(param_6,local_13a),uStack10,
                    CONCAT22(0x400,uStack30),(iVar3 + 0x4),param_6,
                    param_7);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_6,local_13a));
    pass1_1028_9992((u16 *)CONCAT22(param_6,local_13a));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_05d8(param_1: u16,param_2: u16,param_3: i16,param_4: u32,param_5: u32,
               param_6: u16,param_7: u8)

{
  let puVar1: *mut u16;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let local_158: [u8;118];
  let uStack64: u32;
  let local_34: u16;
  let uStack50: u16;
  let uStack34: u32;
  let uStack30: u32;
  let uStack26: u32;
  let uStack22: u32;
  let local_12: u32;
  let local_e: i16;
  let uStack12: u16;
  let uStack10: u32;
  let uStack6: u32;
  
  uStack6 = *_PTR_LOOP_1050_65e2;
  uStack10 = 0x0;
  uStack12 = 0x0;
  pass1_1038_4cea(param_4,CONCAT22(param_6,&local_12),
                  CONCAT22(param_6,&local_e));
  uStack22 = 0x0;
  uStack26 = 0x0;
  uStack30 = 0x0;
  pass1_1028_dc52((astruct_92 *)
                  CONCAT13((char)(param_6 >> 0x8),CONCAT12((char)param_6,&local_34)),0x1,
                  0x0,0x400);
  do {
    do {
      uVar3 = param_5;
      puVar1 = &local_34;
      pass1_1028_e4ec(CONCAT22(param_6,puVar1));
      uStack34 = CONCAT22(uVar3,puVar1);
      uVar4 = uVar3 | puVar1;
      param_5 = uVar4;
      if (uVar4 == 0x0) goto LAB_1038_0668;
    } while (*(long *)(puVar1 + 0x100) != 0x8000002);
    uStack22 = CONCAT22(uVar3,puVar1);
    uVar2 = (puVar1 + 0xfb);
    uStack26 = uVar2;
    pass1_1030_38b8();
    uStack30 = uVar2 & 0xffff | uVar4 << 0x10;
    uVar4 |= uVar2;
    param_5 = uVar4;
  } while (uVar4 == 0x0);
LAB_1038_0668:
  local_34 = 0x389a;
  uStack50 = 0x1008;
  if ((uStack22._2_2_ | uStack22) == 0x0) {
    return;
  }
  if (param_3 == 0x3e8) {
    if (local_e != 0x10) {
      uStack10 = (long)uStack30 / 0x4;
      uStack12 = 0x10;
      goto LAB_1038_0841;
    }
  }
  else {
    if (param_3 < 0x3de) {
      if (param_3 < 0x3cf) {
        if (param_3 < 0x3b6) {
          if (param_3 < 0x39d) {
            if (param_3 < 0x384) {
              if (param_3 < 0x352) {
                if ((param_3 < 0x320) || (uStack6 % 0x14 != 0x0)) goto LAB_1038_0841;
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
        else {
          if (local_e != 0xd) {
            uStack10 = (long)uStack30 / 0x64;
            uStack12 = 0xd;
            goto LAB_1038_0841;
          }
        }
      }
      else {
        if (local_e != 0xe) {
          uStack10 = (long)uStack30 / 0x14;
          uStack12 = 0xe;
          goto LAB_1038_0841;
        }
      }
    }
    else {
      if (local_e != 0xf) {
        uStack10 = (long)uStack30 / 0xa;
        uStack12 = 0xf;
        goto LAB_1038_0841;
      }
    }
  }
  uVar2 = uStack30;
  if ((long)local_12 < (long)uStack30) {
    uVar2 = local_12 & 0xffff;
    uStack30._2_2_ = local_12._2_2_;
  }
  uStack10 = uVar2 & 0xffff | uStack30._2_2_ << 0x10;
LAB_1038_0841:
  if (uStack12 != 0x0) {
    if ((uStack30 != 0x0) && (uStack10 == 0x0)) {
      uStack10 = 0x1;
    }
    pass1_1038_4cd0(param_4,uStack10,uStack12);
  }
  if ((uStack10._2_2_ | uStack10) != 0x0) {
    uVar5 = (param_4 >> 0x10);
    if (*(long *)(param_4 + 0x200) == 0x8000001) {
      uStack64 = (uStack22 + 0x4);
    }
    else {
      uStack64 = 0x4000001;
    }
    pass1_1028_9944((astruct_100 *)CONCAT22(param_6,local_158),uStack10,
                    (param_4 + 0x4),uStack64,param_6,param_7);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_6,local_158));
    pass1_1028_9992((u16 *)CONCAT22(param_6,local_158));
  }
  return;
}



void 
pass1_1038_08d4(param_1: u16,param_2: i32,param_3: u32,param_4: u32,param_5: u16,
               param_6: u8)

{
  let puVar1: *mut u16;
  let uVar2: u16;
  let uVar3: u16;
  let local_16: u16;
  let uStack20: u16;
  let iStack4: i16;
  
  iStack4 = 0x0;
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_5,&local_16),0x1,0x0,0x400);
  do {
    puVar1 = &local_16;
    pass1_1028_e4ec(CONCAT22(param_5,puVar1));
    uVar2 = param_4;
    uVar3 = uVar2 | puVar1;
    param_4 = param_4 & 0xffff0000 | uVar3;
    if (uVar3 == 0x0) goto LAB_1038_0917;
  } while (*(long *)(puVar1 + 0x100) != 0x8000002);
  iStack4 = 0x1;
LAB_1038_0917:
  local_16 = 0x389a;
  uStack20 = 0x1008;
  if (iStack4 != 0x0) {
    if (param_2 < 0xc90000) {
      pass1_1038_0340(param_1,param_2,param_2._2_2_,param_3,uVar3,param_5,param_6)
      ;
      return;
    }
    if (0x31fffff < param_2) {
      pass1_1038_05d8(param_1,param_2,param_2._2_2_,param_3,param_4,param_5,
                      param_6);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_095e(param_1: u16,param_2: u16,param_3: i16,param_4: u32,uchar *param_5,
               param_6: i16,param_7: u16)

{
  code **ppcVar1;
  let bVar2: bool;
  let uVar3: u16;
  let puVar4: *mut u8;
  let uVar5: u32;
  let uVar6: u32;
  let puVar7: *mut u8
  let uVar8: u16;
  let uVar9: u8;
  let puVar10: u32;
  let uVar11: u32;
  let iVar12: i16;
  let uStack58: u32;
  let uStack54: u32;
  let local_28: [u8;2];
  let uStack38: u32;
  let uStack34: u32;
  let puStack30: u32;
  let uStack26: u16;
  let puStack24: *mut u8
  let puStack22: u32;
  let uStack18: u32;
  let iStack14: i16;
  let iStack12: i16;
  let uStack10: u32;
  astruct_67 *paStack6;
  
  paStack6 = (astruct_67 *)
             mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_7,param_5,param_6);
  uStack10 = *_PTR_LOOP_1050_65e2;
  uVar8 = (param_4 >> 0x10);
  if (uStack10 % 0xa == 0x0) {
    if (param_3 < 0xc9) {
      iVar12 = 0x3f;
    }
    else {
      if (param_3 < 0x320) goto LAB_1038_09c3;
      iVar12 = 0x3e;
    }
    post_win_msg_1008_a0e4
              (paStack6,0x0,0x0,0x1,(param_4 + 0x4),iVar12,0x1008,param_7);
  }
LAB_1038_09c3:
  iStack12 = (param_4 + 0x22);
  iStack14 = 0x0;
  uStack18 = *_PTR_LOOP_1050_65e2;
  if (iStack12 < 0x4b) {
    if (iStack12 < 0x3c) {
      if (iStack12 < 0x32) goto LAB_1038_0a1c;
      uVar3 = 0x1e;
    }
    else {
      uVar3 = 0xf;
    }
  }
  else {
    uVar3 = 0x5;
  }
  if ((uStack18 & 0xffff | (_PTR_LOOP_1050_65e2 + 0x2) << 0x10) %
      uVar3 == 0x0) {
    iStack14 = 0x1;
  }
LAB_1038_0a1c:
  if (iStack14 != 0x0) {
    puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0xf);
    puVar7 = (puVar10 >> 0x10);
    uVar3 = puVar10;
    pass1_1038_4e78(uVar3,puVar7,param_4,puVar10);
    puStack22 = CONCAT22(puVar7,uVar3);
    puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1a);
    puVar7 = (puVar10 >> 0x10);
    uVar3 = puVar10;
    uStack26 = uVar3;
    puStack24 = puVar7;
    pass1_1038_4d6e(param_4,puVar10,uVar3,puVar7);
    puStack30 = CONCAT22(puVar7,uVar3);
    ppcVar1 = (code **)(*puStack22 + 0x10);
    (**ppcVar1)(0x1008,puStack22,(puStack22 >> 0x10));
    uStack34 = CONCAT22(puVar7,uVar3);
    ppcVar1 = (code **)(*puStack30 + 0x10);
    (**ppcVar1)(0x1008,puStack30,(puStack30 >> 0x10));
    uStack38 = CONCAT22(puVar7,uVar3);
    uVar11 = pass1_1030_bcae(local_28,param_7);
    uStack54 = 0x0;
    while( true ) {
      uVar11 >>= 0x10;
      uVar9 = 0x30;
      if (uStack34 <= uStack54) break;
      uVar6 = uStack34;
      pass1_1030_1d58(puStack22);
      uVar6 = uVar6 & 0xffff | uVar11 << 0x10;
      bVar2 = false;
      for (uStack58 = 0x0; uStack58 < uStack38; uStack58 += 0x1) {
        uVar5 = uStack38;
        pass1_1030_1d58(puStack30);
        puVar4 = local_28;
        pass1_1030_bd74(puVar4,param_7,uVar6,uVar5 & 0xffff | uVar11 << 0x10,
                        param_7);
        if (puVar4 < 0x6) {
          bVar2 = true;
          break;
        }
      }
      uVar11 = struct_op_1030_73a8(uVar6);
      if (!bVar2) {
        uVar9 = 0x28;
        func_0x10285ca0(0x1030,uVar11,(uVar11 >> 0x10));
        break;
      }
      uStack54 += 0x1;
    }
    if (puStack22 != 0x0) {
      ppcVar1 = (code **)*puStack22;
      (**ppcVar1)(uVar9,puStack22,(puStack22 >> 0x10),0x1);
    }
    if (puStack30 != 0x0) {
      ppcVar1 = (code **)*puStack30;
      (**ppcVar1)(uVar9,puStack30,(puStack30 >> 0x10),0x1);
    }
  }
  return;
}



astruct_18 *  pass1_1038_0b6a(astruct_18 *param_1,param_2: u8)

{
  param_1->field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_100 * 
pass1_1038_0ba6(astruct_100 *param_1,param_2: i16,param_3: u16,param_4: u8)

{
  let puVar1: *mut u8
  astruct_701 *iVar2;
  let uVar2: u16;
  astruct_100 *paVar3;
  let puVar4: *mut u16;
  
  paVar3 = struct_op_1028_d1dc(param_3,param_4,param_1,0x270f);
  puVar1 = (paVar3 >> 0x10);
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_701 *)param_1;
  &iVar2->field_0x108 = 0x0;
  param_1->field_0x0 = 0x1c2e;
  iVar2->field_0x2 = &PTR_LOOP_1050_1038;
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | &iVar2->field_0x8),
             s_SCMove_1050_59d8);
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_3,puVar1,param_2);
  iVar2->field_0x108 = puVar4;
  iVar2->field_0x10a = (puVar4 >> 0x10);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_0c00(param_1: u32,param_2: u32,param_3: u16,param_4: u16,param_5: u16,
               param_6: u8)

{
  code **ppcVar1;
  let uVar2: u32;
  let puVar3: *mut u8;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let puVar7: *mut u8
  let puVar8: *mut u8
  let uVar9: u32;
  let uVar10: u16;
  let puVar11: u32;
  let puStack32: u32;
  let uStack24: u32;
  let local_14: [u8;12];
  
  pass1_1028_dc52((astruct_92 *)
                  CONCAT13((char)(param_5 >> 0x8),CONCAT12((char)param_5,local_14)),0x1,
                  0x0,0x400);
  while( true ) {
    puVar3 = local_14;
    pass1_1028_e4ec(CONCAT22(param_5,puVar3));
    uVar6 = param_2;
    uStack24 = CONCAT22(uVar6,puVar3);
    uVar9 = param_2 & 0xffff0000 | (uVar6 | puVar3);
    if ((uVar6 | puVar3) == 0x0) break;
    pass1_1038_0e78(param_1,CONCAT22(uVar6,puVar3),param_5);
    pass1_1038_1220(param_1,CONCAT22(uVar6,puVar3),uVar9,param_5);
    uVar10 = (uVar9 >> 0x10);
    puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1);
    puVar7 = (puVar11 >> 0x10);
    uVar4 = puVar11;
    pass1_1038_4d6e(CONCAT22(uVar6,puVar3),puVar11,uVar4,puVar7);
    puStack32 = CONCAT22(puVar7,uVar4);
    ppcVar1 = (code **)(*puStack32 + 0x10);
    uVar5 = uVar4;
    puVar8 = puVar7;
    (**ppcVar1)(0x1008,uVar4,puVar7);
    param_2 = CONCAT22(uVar10,puVar8 | uVar5);
    if ((puVar8 | uVar5) != 0x0) {
      uVar2 = (param_1 + 0x108);
      if ((uVar2 + 0x82) != 0x0) {
        pass1_1038_19a0(param_1,CONCAT22(puVar7,uVar4),CONCAT22(uVar6,puVar3),
                        param_5,param_6);
      }
      pass1_1038_1940(param_1,CONCAT22(puVar7,uVar4),uStack24,param_3,param_4,
                      param_5);
    }
    if (puStack32 != 0x0) {
      ppcVar1 = (code **)*puStack32;
      (**ppcVar1)(0x8,uVar4,puVar7,0x1);
    }
    pass1_1038_1c3e(param_1,uStack24,param_3,param_4,0x1008,param_5);
  }
  return;
}



fn pass1_1038_0cf0(param_1: u32,param_2: u16,uchar *param_3)
{
  let puVar1: u32;
  let puVar2: u32;
  let puVar3: u32;
  let iVar4: i16;
  let iVar5: i16;
  let puVar6: u32;
  let uVar7: u16;
  let puStack10: *mut u16;
  
  mem_op_1000_179c(0x10c,param_3,0x1000);
  puStack10 = CONCAT22(param_3,param_2);
  if ((param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    (param_2 + 0x2) = 0x1008;
    uVar7 = (param_1 >> 0x10);
    iVar5 = param_1;
    (param_2 + 0x4) = (iVar5 + 0x4);
    puVar3 = (iVar5 + 0x8);
    puVar6 = (param_2 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar6;
      puVar6 = puVar6 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_2 + 0x2) = &USHORT_1050_1028;
    (param_2 + 0x108) = (iVar5 + 0x108);
    *puStack10 = 0x1c2e;
    (param_2 + 0x2) = &PTR_LOOP_1050_1038;
  }
  return;
}



fn pass1_1038_0d8e(param_1: u16,param_2: u16,param_3: u32,param_4: u32,param_5: u16)
{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let lStack10: i32;
  let uStack4: u16;
  
  uVar1 = pass1_1030_d0a8(param_4);
  uVar2 = pass1_1030_d144(param_4);
  lStack10 = (long)uVar2;
  uVar2 = uVar2 >> 0xf | uVar2;
  uStack4 = uVar1;
  if (uVar2 != 0x0) {
    do {
      uVar3 = pass1_1028_6744(param_5,param_3,uStack4);
      uVar2 |= uVar3;
      if (uVar2 != 0x0) {
        pass1_1028_6228(param_3,0x1,0x0,uStack4,param_5);
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

fn pass1_1038_0e00(param_1: u32,param_2: *mut u32,param_3: u32,param_4: u16,param_5: u16)
{
  code **ppcVar1;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u32;
  let uStack10: u32;
  let uStack6: u32;
  
  ppcVar1 = (code **)(*param_2 + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,param_4);
  for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
    ppcVar1 = (code **)(*param_2 + 0x4);
    uVar4 = uStack6;
    (**ppcVar1)();
    uVar3 = uVar4;
    uVar2 = extraout_DX_00 | uVar3;
    if (uVar2 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,extraout_DX_00);
      uVar4 = struct_op_1030_73a8(CONCAT22(uVar2,uVar3));
      uVar3 = (uVar4 >> 0x10);
      if ((uVar3 | uVar4) != 0x0) {
        pass1_1038_0d8e(param_1,(param_1 >> 0x10),
                        uVar4 & 0xffff | uVar3 << 0x10,param_3,param_5);
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_0e78(param_1: u32,param_2: u32,param_3: u16)
{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let puVar5: *mut u8
  let extraout_DX: u16;
  let puVar6: *mut u8
  let extraout_DX_00: u16;
  let extraout_DX_01: u16;
  let uVar7: u16;
  let uVar8: u16;
  let puVar9: u32;
  let uVar10: u32;
  let uStack22: u32;
  let uStack18: u32;
  let puStack14: u32;
  let puStack10: u32;
  
  puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x4);
  puVar5 = (puVar9 >> 0x10);
  uVar2 = puVar9;
  pass1_1038_4d6e(param_2,puVar9,uVar2,puVar5);
  puStack10 = CONCAT22(puVar5,uVar2);
  uVar10 = *puStack10;
  ppcVar1 = (code **)uVar10 + 0x8;
  uVar3 = uVar2;
  (**ppcVar1)(0x1008,uVar2,puVar5);
  if ((extraout_DX | uVar3) == 0x0) {
    if (puStack10 != 0x0) {
      ppcVar1 = (code **)uVar10;
      (**ppcVar1)(0x8,uVar2,puVar5,0x1);
      return;
    }
  }
  else {
    uVar8 = 0x1008;
    puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1e);
    puVar6 = (puVar9 >> 0x10);
    uVar3 = puVar9;
    pass1_1038_4d6e(param_2,puVar9,uVar3,puVar6);
    puStack14 = CONCAT22(puVar6,uVar3);
    ppcVar1 = (code **)(*puStack14 + 0x10);
    uVar4 = uVar3;
    (**ppcVar1)(0x1008,uVar3,puVar6);
    uStack18 = CONCAT22(extraout_DX_00,uVar4);
    for (uStack22 = 0x0; uStack22 < uStack18; uStack22 += 0x1) {
      ppcVar1 = (code **)(*puStack14 + 0x4);
      uVar10 = uStack18;
      (**ppcVar1)();
      uVar4 = uVar10;
      uVar7 = extraout_DX_01 | uVar4;
      if (uVar7 != 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4,extraout_DX_01);
        uVar8 = 0x1030;
        uVar10 = struct_op_1030_73a8(CONCAT22(uVar7,uVar4));
        if (((uVar10 >> 0x10) | uVar10) != 0x0) {
          pass1_1038_0e00(param_1,puStack10,uVar10,uVar10,param_3);
        }
      }
    }
    if (puStack10 != 0x0) {
      ppcVar1 = (code **)*puStack10;
      (**ppcVar1)(uVar8,uVar2,puVar5,0x1);
    }
    if (puStack14 != 0x0) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(uVar8,uVar3,puVar6,0x1);
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_0f8c(param_1: u16,param_2: u16,param_3: *mut u32,param_4: u32,param_5: u16,
               param_6: u32,param_7: u16,param_8: u16)

{
  let piVar1: *mut i16;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  code **ppcVar5;
  let uVar6: u32;
  qword qVar7;
  let puVar8: *mut u8;
  let uVar9: u32;
  let uVar10: u16;
  let uVar11: u16;
  let uVar12: u16;
  let iVar13: i16;
  let uVar14: u16;
  astruct_99 *paStack80;
  let uStack76: u16;
  let local_30: [u8;4];
  let uStack44: u32;
  let puStack40: u32;
  let uStack36: u32;
  let local_20: [u8;4];
  let puStack28: u32;
  let uStack24: u16;
  let uStack22: u16;
  let uStack20: u16;
  let uStack18: u16;
  let uStack16: u32;
  let uStack12: u32;
  let uStack8: u16;
  let uStack6: u32;
  
  uStack6 = 0x64;
  uStack8 = 0x0;
  ppcVar5 = (code **)(*param_3 + 0x10);
  (**ppcVar5)(param_7,param_3);
  uStack12 = CONCAT22(param_6,param_5);
  uStack16 = 0x0;
  do {
    if (uStack12 <= uStack16) {
      return;
    }
    ppcVar5 = (code **)(*param_3 + 0x4);
    uVar9 = uStack12;
    uVar11 = param_6;
    (**ppcVar5)(param_7,param_3,(param_3 >> 0x10),uStack16,
                (uStack16 >> 0x10));
    uStack18 = uVar11;
    uVar12 = uVar9;
    uVar11 = uStack18 | uVar12;
    param_6 = uVar11;
    uStack20 = uVar12;
    if (uVar11 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar12,uStack18);
      uStack22 = uVar11;
      param_7 = 0x1030;
      uStack24 = uVar12;
      puStack28 = struct_op_1030_73a8(CONCAT22(uStack22,uVar12));
      param_6 = puStack28 >> 0x10;
      puVar8 = local_20;
      ppcVar5 = (code **)(*puStack28 + 0x40);
      (**ppcVar5)(0x1030,puStack28,(puStack28 >> 0x10),puVar8,
                  param_8);
      if (puVar8 == 0x0) {
        uStack36 = pass1_1028_62c8(puStack28,param_8);
        uVar9 = uStack36 >> 0x10;
        uStack8 = 0x1;
        puStack40 = (param_4 + 0x22);
        pass1_1008_5784(CONCAT22(param_8,local_30),puStack40);
        while( true ) {
          uVar11 = uVar9;
          puVar8 = local_30;
          param_7 = 0x1008;
          pass1_1008_5b12((char)puVar8,param_8);
          param_6 = (uVar11 | puVar8);
          if ((uVar11 | puVar8) == 0x0) break;
          uVar2 = (puVar8 + 0x4);
          uVar3 = (puVar8 + 0x6);
          uVar4 = (puVar8 + 0x8);
          uVar12 = (puVar8 + 0xa);
          uVar6 = (puVar8 + 0xc) / uVar12;
          uVar9 = uStack36;
          if (uStack6 < uStack36) {
            uVar9 = uStack6 & 0xffff;
            uStack36._2_2_ = uStack6._2_2_;
          }
          uVar10 = uStack36._2_2_ | uVar9;
          param_6 = uVar10;
          if (uVar10 == 0x0) break;
          qVar7 = (qword)(uVar9 & 0xffff | uStack36._2_2_ << 0x10) / (qword)uVar6;
          param_6 = qVar7 >> 0x10;
          uStack76 = qVar7;
          if (uStack76 == 0x0) break;
          if (uStack76 < uVar12) {
            piVar1 = (i16 *)(puVar8 + 0xc);
            *piVar1 = *piVar1 - uVar9;
            piVar1 = (i16 *)(puVar8 + 0xa);
            *piVar1 = *piVar1 - uStack76;
          }
          else {
            ppcVar5 = (code **)(*puStack40 + 0xc);
            (**ppcVar5)(0x1008,puStack40,(puStack40 >> 0x10),
                        puVar8,uVar11);
            uStack44 = 0x0;
            uStack76 = uVar12;
          }
          paStack80 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
          uVar12 = (paStack80 >> 0x10);
          uVar11 = paStack80;
          if ((uVar12 | uVar11) == 0x0) {
            paStack80 = (astruct_99 *)0x0;
          }
          else {
            paStack80->field_0x0 = 0x389a;
            (uVar11 + 0x2) = 0x1008;
            (uVar11 + 0x4) = 0x0;
            (uVar11 + 0x6) = 0x0;
            (uVar11 + 0x8) = 0x0;
            (uVar11 + 0xa) = 0x0;
            (uVar11 + 0xc) = 0x0;
            paStack80->field_0x0 = 0x56ce;
            (uVar11 + 0x2) = 0x1018;
          }
          uVar14 = (paStack80 >> 0x10);
          iVar13 = paStack80;
          (iVar13 + 0xa) = uStack76;
          uVar6 = uStack76 * uVar6;
          uVar9 = uVar6 >> 0x10;
          (iVar13 + 0xc) = uVar6;
          (iVar13 + 0x4) = uVar2;
          (iVar13 + 0x6) = uVar3;
          (iVar13 + 0x8) = uVar4;
          pass1_1028_6408(puStack28,paStack80,param_8);
        }
      }
      else {
        ppcVar5 = (code **)(*param_3 + 0x8);
        (**ppcVar5)(0x1030,param_3,0x0,0x0,uStack16,(uStack16 >> 0x10));
      }
    }
    uStack16 += 0x1;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_11b0(param_1: u32,param_2: *mut u32,param_3: *mut u32,param_4: u16,param_5: u32,
               param_6: u16)

{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u32;
  let uStack10: u32;
  let uStack6: u32;
  
  ppcVar1 = (code **)(*param_3 + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22(param_5,param_4);
  uStack10 = 0x0;
  while( true ) {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar1 = (code **)(*param_3 + 0x4);
    uVar3 = uStack6;
    (**ppcVar1)();
    uVar2 = uVar3;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,param_5);
    uVar3 = struct_op_1030_73a8(CONCAT22(param_5,uVar2));
    param_5 = param_5 & 0xffff0000 | uVar3 >> 0x10;
    uVar2 = uVar3;
    pass1_1038_0f8c(param_1,(param_1 >> 0x10),param_2,uVar3,uVar2,param_5,
                    0x1030,param_6);
    if (uVar2 == 0x0) break;
    uStack10 += 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_1220(param_1: u32,param_2: u32,param_3: u32,param_4: u16)
{
  code **ppcVar1;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let puVar6: *mut u8
  let puVar7: *mut u8
  let puVar8: *mut u8
  let uVar10: u16;
  let uVar9: u32;
  let puVar11: u32;
  let uVar12: u8;
  let puStack14: u32;
  let puStack10: u32;
  
  uVar10 = (param_3 >> 0x10);
  puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x4);
  puVar6 = (puVar11 >> 0x10);
  uVar3 = puVar11;
  pass1_1038_4d6e(param_2,puVar11,uVar3,puVar6);
  puStack10 = CONCAT22(puVar6,uVar3);
  ppcVar1 = (code **)(*puStack10 + 0x10);
  puVar7 = puVar6;
  uVar4 = uVar3;
  (**ppcVar1)(0x1008,uVar3,puVar6);
  if ((puVar7 != 0x0) || (uVar4 != 0x0)) {
    puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x5);
    puVar8 = (puVar11 >> 0x10);
    uVar4 = puVar11;
    pass1_1038_4d6e(param_2,puVar11,uVar4,puVar8);
    puStack14 = CONCAT22(puVar8,uVar4);
    uVar12 = (u8)uVar4;
    uVar2 = *puStack14;
    ppcVar1 = (code **)uVar2 + 0x8;
    puVar7 = puVar8;
    uVar5 = uVar4;
    (**ppcVar1)(0x1008,uVar12,puVar8);
    uVar9 = CONCAT22(uVar10,puVar7);
    if (((puVar7 != 0x0) || (uVar5 != 0x0)) &&
       (pass1_1038_11b0(param_1,CONCAT13((char)(puVar6 >> 0x8),
                                                  CONCAT12((char)puVar6,uVar3)),
                        CONCAT22(puVar8,uVar4),uVar5,uVar9,param_4), uVar5 == 0x0
       )) {
      if (puStack14 == 0x0) {
        return;
      }
      ppcVar1 = (code **)uVar2;
      (**ppcVar1)(0x8,uVar12,puVar8,0x1);
      return;
    }
    uVar10 = (uVar9 >> 0x10);
    if (puStack14 != 0x0) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(0x8,uVar12,puVar8,0x1);
    }
    puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x6);
    puVar8 = (puVar11 >> 0x10);
    uVar4 = puVar11;
    pass1_1038_4d6e(param_2,puVar11,uVar4,puVar8);
    puStack14 = CONCAT22(puVar8,uVar4);
    ppcVar1 = (code **)(*puStack14 + 0x10);
    puVar7 = puVar8;
    uVar5 = uVar4;
    (**ppcVar1)(0x8,uVar4,puVar8);
    if ((puVar7 != 0x0) || (uVar5 != 0x0)) {
      pass1_1038_11b0(param_1,CONCAT22(puVar6,uVar3),
                      CONCAT22(puVar8,uVar4),uVar5,CONCAT22(uVar10,puVar7),
                      param_4);
    }
    if (puStack14 != 0x0) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(0x8,uVar4,puVar8,0x1);
    }
  }
  if (puStack10 != 0x0) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(0x8,uVar3,puVar6,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_134a(param_1: u16,param_2: u16,param_3: *mut u32,param_4: *mut u32,param_5: *mut u32
               ,param_6: u16,param_7: u16)

{
  code **ppcVar1;
  let uVar2: u16;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar3: u16;
  let unaff_SS: u16;
  let uVar4: u32;
  let puVar5: u32;
  let uStack6: u32;
  
  ppcVar1 = (code **)(*param_5 + 0x10);
  puVar5 = param_5;
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,param_6);
  *param_3 = 0x0;
  do {
    if (uStack6 <= *param_4) {
      return;
    }
    uVar4 = *param_4;
    *param_4 = *param_4 + 0x1;
    ppcVar1 = (code **)(*param_5 + 0x4);
    (**ppcVar1)(param_7,param_5,uVar4,puVar5);
    uVar2 = uVar4;
    uVar3 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,extraout_DX_00);
    uVar4 = struct_op_1030_73a8(CONCAT22(uVar3,uVar2));
    uVar3 = (uVar4 >> 0x10);
    param_7 = &USHORT_1050_1028;
    uVar4 = pass1_1028_45e2(uVar4 & 0xffff | uVar3 << 0x10,uVar4,uVar3,
                            unaff_SS);
    uVar3 = (uVar4 >> 0x10);
    param_3 = uVar4;
    (param_3 + 0x2) = uVar3;
  } while ((uVar3 | param_3) == 0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_13da(param_1: u16,param_2: u16,param_3: *mut u32,param_4: *mut u32,param_5: *mut u32
               ,param_6: u16,param_7: u16)

{
  code **ppcVar1;
  let uVar2: u16;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar3: u16;
  let uVar4: u32;
  let puVar5: u32;
  let uStack6: u32;
  
  ppcVar1 = (code **)(*param_5 + 0x10);
  puVar5 = param_5;
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,param_6);
  *param_3 = 0x0;
  do {
    if (uStack6 <= *param_4) {
      return;
    }
    uVar4 = *param_4;
    *param_4 = *param_4 + 0x1;
    ppcVar1 = (code **)(*param_5 + 0x4);
    (**ppcVar1)(param_7,param_5,uVar4,puVar5);
    uVar2 = uVar4;
    uVar3 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,extraout_DX_00);
    if ((uVar3 | uVar2) == 0x0) {
      return;
    }
    uVar4 = struct_op_1030_73a8(CONCAT22(uVar3,uVar2));
    uVar3 = (uVar4 >> 0x10);
    if ((uVar3 | uVar4) == 0x0) {
      return;
    }
    param_7 = &USHORT_1050_1028;
    uVar4 = pass1_1028_3c32((uVar4 & 0xffff | uVar3 << 0x10));
    uVar3 = (uVar4 >> 0x10);
    param_3 = uVar4;
    (param_3 + 0x2) = uVar3;
  } while ((uVar3 | param_3) == 0x0);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_1482(param_1: u32,param_2: *mut u32,param_3: *mut u32,param_4: u16,param_5: u16,
               param_6: u16,param_7: u16,param_8: u16)

{
  code **ppcVar1;
  sqword sVar2;
  let uVar3: u16;
  let puVar4: u32;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u32;
  let puVar8: *mut u8
  let puVar9: *mut u8
  let uVar10: u16;
  let uVar11: u16;
  let uVar12: u8;
  let uVar13: u8;
  let uVar14: u16;
  let lStack74: i32;
  let local_46: u32;
  i16 local_42 [0x4];
  let uStack58: u16;
  let uStack56: u16;
  let puStack54: u32;
  let puStack50: u32;
  let uStack46: u32;
  let uStack42: u16;
  let uStack40: u16;
  let uStack38: u16;
  let uStack36: u16;
  let uStack34: u32;
  let uStack30: u16;
  let uStack28: u16;
  let uStack26: u16;
  let uStack24: u16;
  let uStack22: u32;
  let uStack18: u32;
  let uStack14: u32;
  let local_a: u32;
  let local_6: u32;
  
  local_6 = 0x0;
  local_a = 0x0;
  puVar4 = &local_a;
  uVar11 = (param_1 >> 0x10);
  uVar3 = param_1;
  pass1_1038_134a(uVar3,uVar11,CONCAT22(param_6,puVar4),
                  CONCAT22(param_6,&local_6),param_3,puVar4,param_4);
  uStack14 = CONCAT22(param_5,puVar4);
  ppcVar1 = (code **)(*param_2 + 0x10);
  (**ppcVar1)(param_4,param_2);
  uStack18 = CONCAT22(param_5,puVar4);
  uStack22 = 0x0;
  do {
    if (uStack18 <= uStack22) {
      return;
    }
    uStack14._2_2_ |= uStack14;
    if (uStack14._2_2_ == 0x0) {
      return;
    }
    pass1_1028_b58e(uStack14);
    uStack26 = uStack14._2_2_;
    uStack24 = uStack18._2_2_;
    pass1_1038_1a30(uVar3,uVar11,CONCAT22(uStack18._2_2_,uStack14._2_2_),
                    &USHORT_1050_1028);
    uStack30 = uStack14._2_2_;
    uStack28 = uStack18._2_2_;
    if ((uStack18._2_2_ | uStack14._2_2_) != 0x0) {
      sVar2 = (qword)CONCAT22(uStack18._2_2_,uStack14._2_2_) * 0x64;
      uVar5 = ((qword)sVar2 >> 0x20);
      uVar7 = sVar2 >> 0x1;
      ppcVar1 = (code **)(*param_2 + 0x4);
      uStack34 = uVar7;
      (**ppcVar1)(&USHORT_1050_1028,param_2,uStack22,(uStack22 >> 0x10));
      uVar6 = uVar7;
      uStack38 = uVar6;
      uStack36 = uVar5;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar6,uVar5);
      uStack42 = uVar6;
      uStack40 = uVar5;
      uStack46 = struct_op_1030_73a8(CONCAT22(uVar5,uVar6));
      puStack50 = *(u32 **)(uStack46 + 0x28);
      puStack54 = 0x0;
      uStack56 = (puStack50 + 0x4);
      for (uStack58 = 0x0; uVar5 = uStack56, uStack58 < uStack56; uStack58 += 0x1) {
        pass1_1020_bb16(puStack50,CONCAT22(param_6,&local_46),

                        CONCAT13((char)(param_6 >> 0x8),CONCAT12((char)param_6,local_42)),
                        uStack58);
        if (((local_46 != 0x0) && (0xd < local_42[0])) && (local_42[0] < 0x1d)) {
          uVar7 = local_46;
          if (uStack34 < local_46) {
            uVar7 = uStack34 & 0xffff;
            local_46._2_2_ = uStack34._2_2_;
          }
          uVar5 = uVar7;
          if ((local_a._2_2_ <= local_46._2_2_) &&
             ((local_a._2_2_ < local_46._2_2_ || (local_a < uVar5)))) {
            uVar5 = local_a;
            local_46._2_2_ = local_a._2_2_;
          }
          lStack74 = CONCAT22(local_46._2_2_,uVar5);
          uStack34 = CONCAT22(uStack34._2_2_ +
                              (-(uStack34 < uVar5) - local_46._2_2_),
                              uStack34 - uVar5);
          local_a = CONCAT22(local_a._2_2_ +
                             (-(local_a < uVar5) - local_46._2_2_),
                             local_a - uVar5);
          puVar9 = local_46._2_2_;
          if (puStack54 == 0x0) {
            puVar8 = local_46._2_2_;
            uVar10 = uVar5;
            mem_op_1000_179c(0xa,local_46._2_2_,0x1000);
            puVar9 = (puVar8 | uVar10);
            if (puVar9 == 0x0) {
              uVar10 = 0x0;
              puVar9 = 0x0;
            }
            else {
              pass1_1020_ba3e((long *)CONCAT22(puVar8,uVar10),0x5,0x5,param_8,param_7);
            }
            puStack54 = CONCAT22(puVar9,uVar10);
          }
          pass1_1020_bb8a((long *)puStack54,uVar5,CONCAT22(local_42[0],local_46._2_2_),
                          param_8,param_6);
          uVar7 = local_46 - lStack74;
          pass1_1020_bb8a((long *)puStack50,uVar7,
                          CONCAT22(local_42[0],(uVar7 >> 0x10)),param_8,param_6);
          if (local_a == 0x0) {
            pass1_1038_1b3a(uVar3,uVar11,uStack14,puStack54,param_6,uVar7,param_7,
                            param_8);
            puStack54 = 0x0;
            uVar7 = ZEXT24(&local_a);
            pass1_1038_134a(uVar3,uVar11,CONCAT22(param_6,&local_a),
                            CONCAT22(param_6,&local_6),param_3,&local_a,0x1020);
            uVar5 = uVar7;
            uStack14 = uVar7 & 0xffff | ZEXT24(puVar9) << 0x10;
            uVar10 = puVar9 | uVar5;
            if (uVar10 != 0x0) {
              uVar12 = 0x64;
              uVar13 = 0x0;
              uVar14 = 0x0;
              pass1_1028_b58e(uVar7 & 0xffff | ZEXT24(puVar9) << 0x10);
              uStack26 = uVar5;
              uStack24 = uVar10;
              pass1_1038_1a30(uVar3,uVar11,CONCAT22(uVar10,uVar5),
                              &USHORT_1050_1028);
              uVar7 = (CONCAT22(uVar10,uVar5) *
                             CONCAT22(uVar14,CONCAT11(uVar13,uVar12))) >> 0x1;
              uStack34 = uVar7;
              uStack30 = uVar5;
              uStack28 = uVar10;
            }
          }
          uVar5 = uVar7;
          if ((uStack34 == 0x0) || (local_a == 0x0)) break;
        }
      }
      if (puStack54 != 0x0) {
        pass1_1038_1b3a(uVar3,uVar11,uStack14,puStack54,param_6,uVar5,param_7,param_8);
        puStack54 = 0x0;
      }
    }
    uStack22 += 0x1;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_16f2(param_1: u32,param_2: *mut u32,param_3: *mut u32,param_4: u16,param_5: u16,
               param_6: u16,param_7: u16,param_8: u16,param_9: u8)

{
  long *plVar1;
  code **ppcVar2;
  let uVar3: u16;
  let puVar4: u32;
  let uVar5: u16;
  let puVar6: u32;
  let puVar7: u32;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  let puVar12: *mut u8
  let uVar13: u32;
  let uVar14: u32;
  let uVar15: u32;
  let lVar16: i32;
  let uVar17: u16;
  let lStack68: i32;
  let puStack56: u32;
  let puStack52: u32;
  long *plStack50;
  let uStack46: u16;
  let uStack42: u32;
  let uStack22: u32;
  let uStack18: u32;
  let uStack14: u32;
  let local_a: u32;
  let local_6: u32;
  
  local_6 = 0x0;
  local_a = 0x0;
  puVar6 = &local_a;
  uVar17 = (param_1 >> 0x10);
  uVar3 = param_1;
  pass1_1038_13da(uVar3,uVar17,CONCAT22(param_8,puVar6),
                  CONCAT22(param_8,&local_6),param_3,puVar6,param_7);
  uStack14 = CONCAT22(param_4,puVar6);
  uVar8 = param_4 | puVar6;
  if (uVar8 != 0x0) {
    ppcVar2 = (code **)(*param_2 + 0x10);
    (**ppcVar2)(param_7,param_2);
    uStack18 = CONCAT22(uVar8,puVar6);
    for (uStack22 = 0x0; uStack22 < uStack18; uStack22 += 0x1) {
      ppcVar2 = (code **)(*param_2 + 0x4);
      uVar15 = uStack18;
      uVar10 = uVar8;
      (**ppcVar2)(param_7,param_2,uStack22,(uStack22 >> 0x10));
      uVar5 = uVar15;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5,uVar10);
      param_7 = 0x1030;
      uVar15 = struct_op_1030_73a8(CONCAT22(uVar10,uVar5));
      uVar11 = (uVar15 >> 0x10);
      uVar9 = uVar15;
      pass1_1038_1a30(uVar3,uVar17,CONCAT22(uVar10,uVar5),0x1030);
      if ((uVar11 | uVar9) != 0x0) {
        uStack42 = (CONCAT22(uVar11,uVar9) * 0x64) >> 0x1;
        plVar1 = *(long **)(uVar5 + 0x22);
        uVar9 = (uVar5 + 0x24);
        uVar13 = uVar9;
        uStack46 = plVar1;
        if ((uVar9 | uStack46) != 0x0) {
          plStack50 = (long *)0x0;
          puVar6 = pass1_1028_0d80(uVar15);
          puStack56 = 0x0;
          puStack52 = puVar6;
          while( true ) {
            lVar16 = pass1_1020_bae6(uStack46,CONCAT22(puStack52,
                                                       (plVar1 >> 0x10)),
                                     puStack52,uVar13,param_8);
            uVar9 = (lVar16 >> 0x10);
            puVar7 = lVar16;
            uVar13 = (uVar9 | puVar7);
            if ((uVar9 | puVar7) != 0x0) {
              uVar14 = uVar9;
              if ((uStack42._2_2_ <= uVar9) &&
                 ((uStack42._2_2_ < uVar9 || (uStack42 < puVar7)))) {
                uVar14 = uStack42._2_2_;
                puVar7 = uStack42;
              }
              if ((local_a._2_2_ <= uVar14) &&
                 ((local_a._2_2_ < uVar14 || (local_a < puVar7)))) {
                uVar14 = local_a._2_2_;
                puVar7 = local_a;
              }
              puVar12 = uVar14;
              lStack68 = CONCAT22(puVar12,puVar7);
              uStack42 = CONCAT22((uStack42._2_2_ - puVar12) -
                                  (uStack42 < puVar7),
                                  
                                  (uStack42 - puVar7));
              local_a = CONCAT22((local_a._2_2_ - puVar12) -
                                 (local_a < puVar7),
                                 (local_a - puVar7))
              ;
              uVar13 = uVar14;
              if (plStack50 == (long *)0x0) {
                puVar4 = puVar7;
                mem_op_1000_179c(0xa,puVar12,0x1000);
                uVar13 = (puVar12 | puVar4);
                if ((puVar12 | puVar4) == 0x0) {
                  puVar4 = 0x0;
                  uVar13 = 0x0;
                }
                else {
                  pass1_1020_ba3e((long *)CONCAT22(puVar12,puVar4),0x5,0x5,param_6,param_5
                                 );
                }
                plStack50 = (long *)CONCAT22(uVar13,puVar4);
              }
              pass1_1020_bb8a(plStack50,puVar7,uVar14 | ZEXT24(puStack52) << 0x10,
                              param_6,param_8);
              pass1_1020_bb8a(plVar1,(lVar16 - lStack68),
                              CONCAT22(puStack52,((lVar16 - lStack68) >> 0x10)
                                      ),param_6,param_8);
              uVar9 = uVar13;
              puStack56 = puStack52;
              puVar7 = puStack52;
              if (local_a == 0x0) {
                pass1_1038_1ac6(uVar3,uVar17,uStack14,plStack50,puStack52,
                                param_8,param_9);
                plStack50 = (long *)0x0;
                puVar7 = &local_a;
                pass1_1038_13da(uVar3,uVar17,CONCAT22(param_8,puVar7),
                                CONCAT22(param_8,&local_6),param_3,puVar7
                                ,0x1020);
                uStack14 = CONCAT22(uVar9,puVar7);
                uVar13 = (uVar9 | puVar7);
                if ((uVar9 | puVar7) == 0x0) {
                  return;
                }
              }
            }
            param_7 = 0x1020;
            if ((uStack42 == 0x0) || (local_a == 0x0)) break;
            param_7 = &USHORT_1050_1028;
            puVar7 = pass1_1028_0d80(uVar15);
            if ((puVar7 == puStack56) ||
               ((puStack52 = puVar7, puStack56 == 0x0 && (puVar7 == puVar6))
               )) break;
          }
          if (plStack50 != (long *)0x0) {
            pass1_1038_1ac6(uVar3,uVar17,uStack14,plStack50,puVar7,param_8,
                            param_9);
          }
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_1940(param_1: u32,param_2: *mut u32,param_3: u32,param_4: u16,param_5: u16,
               param_6: u16)

{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u16;
  let puVar4: *mut u8
  let extraout_DX: u16;
  let puVar5: u32;
  let puStack10: u32;
  
  puVar5 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x3);
  puVar4 = (puVar5 >> 0x10);
  uVar2 = puVar5;
  pass1_1038_4d6e(param_3,puVar5,uVar2,puVar4);
  puStack10 = CONCAT22(puVar4,uVar2);
  ppcVar1 = (code **)(*puStack10 + 0x10);
  uVar3 = uVar2;
  (**ppcVar1)(0x1008,uVar2,puVar4);
  if ((extraout_DX | uVar3) != 0x0) {
    pass1_1038_1482(param_1,param_2,puStack10,0x1008,extraout_DX | uVar3,param_6,param_4,
                    param_5);
  }
  if (puStack10 != 0x0) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(0x1008,uVar2,puVar4,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_19a0(param_1: u32,param_2: *mut u32,param_3: u32,param_4: u16,param_5: u8)
{
  code **ppcVar1;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let puVar5: *mut u8
  let extraout_DX: u16;
  code **ppcVar6;
  let puVar7: u32;
  let puStack10: u32;
  
  puVar7 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x2);
  puVar5 = (puVar7 >> 0x10);
  uVar3 = puVar7;
  pass1_1038_4d6e(param_3,puVar7,uVar3,puVar5);
  puStack10 = CONCAT22(puVar5,uVar3);
  uVar2 = *puStack10;
  ppcVar6 = (code **)uVar2;
  ppcVar1 = ppcVar6 + 0x8;
  uVar4 = uVar3;
  (**ppcVar1)(0x1008,uVar3,puVar5);
  if ((extraout_DX | uVar4) == 0x0) {
    vsprintf_op_1030_840a(s_mineToSmelter__no_mines_1050_59df,0x1030,param_4,0x0);
    if (puStack10 != 0x0) {
      ppcVar1 = ppcVar6;
      (**ppcVar1)(0x1030,uVar3,puVar5,0x1);
      return;
    }
  }
  else {
    pass1_1038_16f2(param_1,puStack10,param_2,extraout_DX | uVar4,ppcVar6,
                    (uVar2 >> 0x10),0x1008,param_4,param_5);
    if (puStack10 != 0x0) {
      ppcVar1 = (code **)*puStack10;
      (**ppcVar1)(0x1008,uVar3,puVar5,0x1);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_1a30(param_1: u16,param_2: u16,param_3: u32,param_4: u16)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u16;
  let uVar4: u32;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uStack18: u32;
  let uStack10: u32;
  let uStack6: u16;
  
  uVar5 = (param_3 >> 0x10);
  puVar1 = (param_3 + 0x1e);
  uVar7 = (param_3 + 0x20);
  uStack6 = puVar1;
  uVar3 = uVar7 | uStack6;
  if (uVar3 != 0x0) {
    ppcVar2 = (code **)(*puVar1 + 0x10);
    uVar6 = uStack6;
    (**ppcVar2)();
    uStack10 = CONCAT22(extraout_DX,uVar3);
    for (uStack18 = 0x0; uStack18 < uStack10; uStack18 += 0x1) {
      ppcVar2 = (code **)(*puVar1 + 0x4);
      uVar4 = uStack10;
      (**ppcVar2)(param_4,uStack6,(puVar1 >> 0x10),uStack18,uVar6,uVar7);
      if ((extraout_DX_00 | uVar4) != 0x0) {
        param_4 = &USHORT_1050_1028;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4,extraout_DX_00);
      }
    }
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_1ac6(param_1: u16,param_2: u16,param_3: u32,param_4: u32,param_5: i16,
               param_6: u16,param_7: u8)

{
  let extraout_DX: u16;
  let local_118: [u8;112];
  let uStack6: u32;
  
  pass1_1028_b58e(param_3);
  uStack6 = CONCAT22(extraout_DX,param_5);
  pass1_1030_e8a0((astruct_100 *)CONCAT22(param_6,local_118),param_4,
                  (param_3 + 0xc),(param_5 + 0x4),param_6,
                  param_7);
  pass1_1028_d52c(*_PTR_LOOP_1050_5748,*_PTR_LOOP_1050_65e2 + 0x1,
                  CONCAT22(param_6,local_118));
  return;
}



// WARNING: Could not reconcile some variable overlaps

void 
pass1_1038_1b3a(param_1: u16,param_2: u16,param_3: u32,param_4: *mut u32,param_5: u16,
               param_6: u16,param_7: u16,param_8: u16)

{
  let extraout_DX: i16;
  let local_1a: u32;
  let local_16: [u16;0x2];
  let uStack18: u16;
  let uStack16: u16;
  let uStack14: u32;
  let uStack10: u32;
  let uStack6: u32;
  
  pass1_1028_b58e(param_3);
  uStack6 = CONCAT22(extraout_DX,param_6);
  uStack10 = param_3;
  uStack14 = pass1_1028_45e2(param_3,param_3,extraout_DX,param_5);
  uStack16 = (param_4 + 0x4);
  for (uStack18 = 0x0; uStack18 < uStack16; uStack18 += 0x1) {
    pass1_1020_bb16(param_4,CONCAT22(param_5,&local_1a),
                    CONCAT22(param_5,local_16),uStack18);
    if (uStack14 < local_1a) {
      pass1_1030_7ddc(uStack6,uStack14,local_16[0],uStack14,uStack14._2_2_,param_7,
                      param_8,param_5);
      uStack14 = 0x0;
    }
    else {
      uStack14 -= local_1a;
      pass1_1030_7ddc(uStack6,local_1a,local_16[0],local_1a,uStack14._2_2_,param_7,
                      param_8,param_5);
    }
    if (uStack14 == 0x0) break;
  }
  if (param_4 != 0x0) {
    fn_ptr_1020_ba7e(param_4);
    fn_ptr_1000_17ce((astruct_18 *)param_4,0x1000);
  }
  return;
}



astruct_18 *  pass1_1038_1c02(astruct_18 *param_1,param_2: u8)

{
  param_1->field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_1c3e(param_1: u32,param_2: u32,param_3: u16,param_4: u16,param_5: u16,
               param_6: u16)

{
  let uVar1: u16;
  let puVar2: u32;
  code **ppcVar3;
  let uVar4: u32;
  let uVar5: u16;
  let iVar6: i16;
  let BVar7: bool;
  let puVar8: u32;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u32;
  let uVar12: u16;
  let uVar13: u16;
  let uVar14: u16;
  let uStack26: u32;
  let uStack14: u32;
  
  uVar10 = (param_2 >> 0x10);
  puVar2 = (param_2 + 0xc);
  uVar10 = (param_2 + 0xe);
  ppcVar3 = (code **)(*puVar2 + 0x10);
  puVar8 = puVar2;
  uVar14 = puVar2;
  (**ppcVar3)();
  uVar4 = puVar8 & 0xffff | extraout_DX << 0x10;
  uStack14 = 0x0;
  do {
    if (uVar4 <= uStack14) {
      return;
    }
    ppcVar3 = (code **)(*puVar2 + 0x4);
    uVar11 = uVar4;
    (**ppcVar3)(param_5,puVar2,(puVar2 >> 0x10),uStack14,uVar14,uVar10);
    uVar5 = uVar11;
    uVar9 = extraout_DX_00 | uVar5;
    if (uVar9 != 0x0) {
      param_5 = &USHORT_1050_1028;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5,extraout_DX_00);
      uStack26 = CONCAT22(uVar9,uVar5);
      iVar6 = (uVar5 + 0x34);
      if ((iVar6 != 0x0) && (*(long *)(uVar5 + 0x36) != 0x0)) {
        uVar12 = param_1;
        uVar13 = (param_1 >> 0x10);
        pass1_1038_201a(uVar12,uVar13,CONCAT22(uVar9,uVar5),iVar6,uVar9);
        if (iVar6 == 0x0) {
          uVar11 = struct_op_1030_73a8(uStack26);
          uVar1 = (uVar11 + 0xc);
          param_5 = 0x1008;
          BVar7 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1);
          if (BVar7 == 0x0) {
            param_5 = 0x1008;
            BVar7 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x2);
            if (BVar7 == 0x0) {
              BVar7 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x5);
              if (BVar7 == 0x0) {
                param_5 = 0x1008;
                BVar7 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x6);
                if (BVar7 == 0x0) goto LAB_1038_1c76;
              }
              param_5 = 0x1008;
              pass1_1038_2306(uVar12,uVar13,uStack26);
            }
            else {
              pass1_1038_26ee(uVar12,uVar13,uStack26,param_3,param_4,param_6);
            }
          }
          else {
            pass1_1038_24e8(uVar12,uVar13,uStack26,param_3,param_4,param_6);
          }
        }
      }
    }
LAB_1038_1c76:
    uStack14 += 0x1;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_1d68(param_1: u16,param_2: u16,param_3: *mut u32,param_4: u32,param_5: u16,
               param_6: u16,param_7: u16,param_8: u32)

{
  let piVar1: *mut i16;
  let uVar2: u16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  code **ppcVar6;
  let uVar7: u32;
  let uVar8: u16;
  let bVar9: bool;
  let puVar10: *mut u8;
  let uVar11: u32;
  let uVar12: u16;
  let uVar13: u16;
  let iVar14: i16;
  let puVar15: u32;
  astruct_99 *paStack82;
  let uStack78: u16;
  let uStack52: u32;
  let local_30: [u8;4];
  let uStack44: u32;
  let puStack40: u32;
  let uStack36: u32;
  let local_20: [u8;4];
  let puStack28: u32;
  let uStack24: u16;
  let uStack22: u16;
  let uStack20: u16;
  let uStack18: u16;
  let uStack16: u32;
  let uStack12: u32;
  let uStack8: u16;
  let uStack6: u32;
  
  uStack6 = 0x64;
  uStack8 = 0x0;
  ppcVar6 = (code **)(*param_3 + 0x10);
  puVar15 = param_3;
  (**ppcVar6)();
  uStack12 = CONCAT22(param_8,param_5);
  uStack16 = 0x0;
  do {
    if (uStack12 <= uStack16) {
      return;
    }
    ppcVar6 = (code **)(*param_3 + 0x4);
    uVar11 = uStack12;
    uVar13 = param_8;
    (**ppcVar6)(param_6,param_3,uStack16,puVar15);
    uStack18 = uVar13;
    uVar12 = uVar11;
    uVar13 = uStack18 | uVar12;
    param_8 = uVar13;
    uStack20 = uVar12;
    if (uVar13 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar12,uStack18);
      uStack22 = uVar13;
      param_6 = 0x1030;
      uStack24 = uVar12;
      puStack28 = struct_op_1030_73a8(CONCAT22(uStack22,uVar12));
      param_8 = puStack28 >> 0x10;
      puVar10 = local_20;
      ppcVar6 = (code **)(*puStack28 + 0x40);
      (**ppcVar6)(0x1030,puStack28,(puStack28 >> 0x10),puVar10,param_7);
      if (puVar10 == 0x0) {
        uStack36 = pass1_1028_62c8(puStack28,param_7);
        uVar11 = uStack36 >> 0x10;
        uStack8 = 0x1;
        puStack40 = (param_4 + 0x22);
        pass1_1008_5784(CONCAT22(param_7,local_30),puStack40);
        while( true ) {
          uVar13 = uVar11;
          puVar10 = local_30;
          param_6 = 0x1008;
          pass1_1008_5b12(puVar10,param_7);
          uStack52 = CONCAT22(uVar13,puVar10);
          param_8 = (uVar13 | puVar10);
          if ((uVar13 | puVar10) == 0x0) break;
          uVar2 = (puVar10 + 0x4);
          iVar3 = (puVar10 + 0x6);
          uVar4 = (puVar10 + 0x8);
          uVar12 = (puVar10 + 0xc);
          uVar5 = (puVar10 + 0xa);
          uVar8 = uVar12 / uVar5;
          uVar11 = uVar12 % uVar5;
          bVar9 = false;
          if (((0x0 < iVar3) && (!SBORROW2(iVar3,0x1))) &&
             ((iVar3 == 0x5 || iVar3 + -0x1 < 0x4 || (iVar3 == 0x8)))) {
            bVar9 = true;
          }
          if (bVar9) {
            uVar11 = uStack36;
            if (uStack6 < uStack36) {
              uVar11 = uStack6 & 0xffff;
              uStack36._2_2_ = uStack6._2_2_;
            }
            uVar12 = uStack36._2_2_ | uVar11;
            param_8 = uVar12;
            if (uVar12 == 0x0) break;
            uStack78 = ((uVar11 & 0xffff | uStack36._2_2_ << 0x10) /
                             uVar8);
            if (uStack78 < uVar5) {
              piVar1 = (i16 *)(puVar10 + 0xc);
              *piVar1 = *piVar1 - uVar11;
              piVar1 = (i16 *)(puVar10 + 0xa);
              *piVar1 = *piVar1 - uStack78;
            }
            else {
              ppcVar6 = (code **)(*puStack40 + 0xc);
              (**ppcVar6)(0x1008,puStack40,(puStack40 >> 0x10),uStack52);
              uStack44 = 0x0;
              uStack78 = uVar5;
            }
            paStack82 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
            uVar12 = (paStack82 >> 0x10);
            uVar13 = paStack82;
            if ((uVar12 | uVar13) == 0x0) {
              paStack82 = (astruct_99 *)0x0;
            }
            else {
              paStack82->field_0x0 = 0x389a;
              (uVar13 + 0x2) = 0x1008;
              (uVar13 + 0x4) = 0x0;
              (uVar13 + 0x6) = 0x0;
              (uVar13 + 0x8) = 0x0;
              (uVar13 + 0xa) = 0x0;
              (uVar13 + 0xc) = 0x0;
              paStack82->field_0x0 = 0x56ce;
              (uVar13 + 0x2) = 0x1018;
            }
            uVar13 = (paStack82 >> 0x10);
            iVar14 = paStack82;
            (iVar14 + 0xa) = uStack78;
            uVar7 = uStack78 * uVar8;
            uVar11 = uVar7 >> 0x10;
            (iVar14 + 0xc) = uVar7;
            (iVar14 + 0x4) = uVar2;
            (iVar14 + 0x6) = iVar3;
            (iVar14 + 0x8) = uVar4;
            pass1_1028_6408(puStack28,
                            (paStack82 & 0xffff | uVar13 << 0x10),
                            param_7);
          }
        }
      }
      else {
        ppcVar6 = (code **)(*param_3 + 0x8);
        (**ppcVar6)(0x1030,param_3,0x0,uStack16);
      }
    }
    uStack16 += 0x1;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_1faa(param_1: u32,param_2: *mut u32,param_3: *mut u32,param_4: u16,param_5: u32,
               param_6: u16)

{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u32;
  let uStack10: u32;
  let uStack6: u32;
  
  ppcVar1 = (code **)(*param_3 + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22(param_5,param_4);
  uStack10 = 0x0;
  while( true ) {
    if (uStack6 <= uStack10) {
      return;
    }
    ppcVar1 = (code **)(*param_3 + 0x4);
    uVar3 = uStack6;
    (**ppcVar1)();
    uVar2 = uVar3;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,param_5);
    uVar3 = struct_op_1030_73a8(CONCAT22(param_5,uVar2));
    param_5 = param_5 & 0xffff0000 | uVar3 >> 0x10;
    uVar2 = uVar3;
    pass1_1038_1d68(param_1,(param_1 >> 0x10),param_2,uVar3,uVar2,0x1030,
                    param_6,param_5);
    if (uVar2 == 0x0) break;
    uStack10 += 0x1;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_201a(param_1: u16,param_2: u16,param_3: u32,param_4: u16,
               param_5: u16)

{
  let puVar1: *mut u16;
  let iVar2: i16;
  code **ppcVar3;
  let lVar4: i32;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u32;
  let puVar10: *mut u8
  let uVar11: u32;
  let uVar12: u32;
  astruct_416 *iVar12;
  let uVar13: u16;
  let puVar14: *mut u8
  let uVar15: u16;
  let puVar16: u32;
  let uVar17: u16;
  let lStack24: i32;
  let lStack20: i32;
  let uStack10: u16;
  astruct_413 *uVar5;
  
  uVar17 = (param_3 >> 0x10);
  uVar15 = 0x1030;
  puVar16 = pass1_1030_6b16(param_3);
  uVar6 = (puVar16 >> 0x10);
  uVar5 = (astruct_413 *)puVar16;
  if ((uVar6 | uVar5) == 0x0) {
    return;
  }
  iVar12 = (astruct_416 *)param_3;
  iVar2 = iVar12->field_0x34;
  lVar4 = (long)iVar2;
  uVar12 = lVar4 * 0x64;
  puVar10 = (uVar12 >> 0x10);
  uVar7 = uVar12;
  uStack10 = 0x0;
  lStack20 = 0x0;
  if (uVar5->field_0x4 == 0x0) {
    if (uVar5->field_0x6 == 0x0) {
      if (uVar5->field_0x8 == 0x0) goto LAB_1038_2102;
      uVar8 = pass1_1020_c42e(uVar5->field_0x8);
      uVar11 = uVar5->field_0xa * uVar8;
      puVar14 = (uVar11 >> 0x10);
      if (uVar11 + lVar4 * -0x64 != 0x0 && (long)uVar12 <= (long)uVar11) {
        uVar11 = uVar12 & 0xffff;
        puVar14 = puVar10;
      }
      uVar12 = uVar11 & 0xffff | ZEXT24(puVar14) << 0x10;
      uVar9 = (long)(uVar11 & 0xffff | ZEXT24(puVar14) << 0x10) / (long)uVar8;
      puVar1 = &uVar5->field_0xa;
      *puVar1 = *puVar1 - uVar9;
      uStack10 = ((long)uVar12 / 0x64);
      uVar12 = (long)uVar12 % 0x64;
      uVar11 = uVar12;
      if (uVar12 != 0x0) {
        uStack10 += 0x1;
        uVar11 = uStack10;
      }
      uVar7 = uVar11;
      mem_op_1000_179c(0x2a,uVar12,0x1000);
      puVar10 = (uVar12 | uVar7);
      if (puVar10 == 0x0) goto LAB_1038_20fa;
      pass1_1038_6838((u16 *)CONCAT22(uVar12,uVar7),uVar9,uVar5->field_0x8,
                      uStack10,iVar12->field_0x4);
    }
    else {
      uVar8 = switch_1020_c3b4(uVar5->field_0x6);
      uVar11 = uVar5->field_0xa * uVar8;
      puVar14 = (uVar11 >> 0x10);
      if (uVar11 + lVar4 * -0x64 != 0x0 && (long)uVar12 <= (long)uVar11) {
        uVar11 = uVar12 & 0xffff;
        puVar14 = puVar10;
      }
      uVar12 = uVar11 & 0xffff | ZEXT24(puVar14) << 0x10;
      uVar9 = (long)(uVar11 & 0xffff | ZEXT24(puVar14) << 0x10) / (long)uVar8;
      puVar1 = &uVar5->field_0xa;
      *puVar1 = *puVar1 - uVar9;
      uStack10 = ((long)uVar12 / 0x64);
      uVar12 = (long)uVar12 % 0x64;
      uVar11 = uVar12;
      if (uVar12 != 0x0) {
        uStack10 += 0x1;
        uVar11 = uStack10;
      }
      uVar7 = uVar11;
      mem_op_1000_179c(0x2a,uVar12,0x1000);
      puVar10 = (uVar12 | uVar7);
      if (puVar10 == 0x0) goto LAB_1038_20fa;
      pass1_1038_675c((u16 *)CONCAT22(uVar12,uVar7),uVar9,uVar5->field_0x6,
                      uStack10,iVar12->field_0x4);
    }
  }
  else {
    uVar13 = uVar5->field_0xa;
    puVar14 = 0x0;
    if ((puVar10 < 0x1) && (((uchar *)0x7fff < puVar10 || (uVar7 < uVar13)))) {
      uVar13 = uVar7;
      puVar14 = puVar10;
    }
    lStack24 = CONCAT22(puVar14,uVar13);
    puVar1 = &uVar5->field_0xa;
    *puVar1 = *puVar1 - uVar13;
    uStack10 = (lStack24 / 0x64);
    uVar11 = lStack24 % 0x64;
    uVar12 = uVar11;
    if (uVar11 != 0x0) {
      uStack10 += 0x1;
      uVar12 = uStack10;
    }
    uVar7 = uVar12;
    mem_op_1000_179c(0x2a,uVar11,0x1000);
    puVar10 = (uVar11 | uVar7);
    if (puVar10 == 0x0) {
LAB_1038_20fa:
      uVar15 = 0x1000;
      lStack20 = 0x0;
      goto LAB_1038_2102;
    }
    pass1_1038_6590((u16 *)CONCAT22(uVar11,uVar7),uVar13,puVar14,
                    uVar5->field_0x4,uStack10,iVar12->field_0x4);
  }
  uVar15 = 0x1000;
  lStack20 = CONCAT22(puVar10,uVar7);
LAB_1038_2102:
  if (lStack20 != 0x0) {
    pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
    uVar15 = 0x1030;
    uVar7 = uStack10;
    pass1_1030_6c4c(param_3,iVar2 - uStack10);
  }
  if (uVar5->field_0xa == 0x0) {
    if ((uVar6 | uVar5) != 0x0) {
      ppcVar3 = (code **)*puVar16;
      (**ppcVar3)(uVar15,uVar5,uVar6,0x1);
    }
  }
  else {
    pass1_1030_6c66(param_3,0x0,puVar16,uVar7,puVar10,0x1030);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_2306(param_1: u16,param_2: u16,param_3: u32)
{
  let piVar1: *mut i16;
  let puVar2: u32;
  code **ppcVar3;
  qword qVar4;
  let puVar5: u32;
  astruct_417 *uVar9;
  let uVar6: u32;
  let puVar7: u32;
  let uVar8: u16;
  let uVar10: u16;
  astruct_419 *iVar11;
  astruct_418 *iVar12;
  let uVar11: u16;
  let uVar12: u16;
  let uVar13: u16;
  let uVar14: u32;
  let uStack42: u32;
  let uStack34: u32;
  let uStack30: u16;
  let uStack24: u32;
  let uStack12: u32;
  let iStack8: i16;
  
  uVar13 = 0x1030;
  uVar14 = struct_op_1030_73a8(param_3);
  uStack24 = uVar14 >> 0x10;
  uVar11 = (param_3 >> 0x10);
  iVar11 = (astruct_419 *)param_3;
  iStack8 = iVar11->field_0x34;
  uStack12 = 0x64;
  puVar2 = (uVar14 + 0x22);
  puVar7 = puVar2;
  while( true ) {
    uVar8 = uStack24;
    uVar12 = (puVar2 >> 0x10);
    ppcVar3 = (code **)(*puVar2 + 0x10);
    (**ppcVar3)(uVar13,puVar2,uVar12);
    uVar9 = (astruct_417 *)puVar7;
    uVar14 = puVar7 & 0xffff;
    puVar5 = (uVar14 | uVar8 << 0x10);
    if ((uVar8 | uVar9) == 0x0) break;
    if (uVar9->field_0xa == 0x0) {
      uStack24 = (uVar8 | uVar9);
      if ((uVar8 | uVar9) != 0x0) {
        ppcVar3 = (code **)*puVar5;
        (**ppcVar3)((char)uVar13,uVar9,uVar8,0x1);
      }
    }
    else {
      uStack24 = 0x0;
      uStack30 = 0x0;
      if (uVar9->field_0x6 == 0x0) {
        if (uVar9->field_0x8 != 0x0) {
          uStack30 = pass1_1020_c42e(uVar9->field_0x8);
          goto LAB_1038_2385;
        }
      }
      else {
        uStack30 = switch_1020_c3b4(uVar9->field_0x6);
LAB_1038_2385:
        uVar13 = 0x1020;
        uStack24 = (uVar9->field_0xa * uStack30);
      }
      uStack12._2_2_ = 0x0;
      if (uStack12 < uStack24) {
        uStack24 = uStack12 & 0xffff;
      }
      uStack34 = uStack24 | uStack12._2_2_ << 0x10;
      uStack24 |= uStack12._2_2_ << 0x10;
      qVar4 = (qword)uStack24 / (qword)uStack30;
      uVar6 = qVar4;
      uStack24 %= uStack30;
      piVar1 = &uVar9->field_0xa;
      *piVar1 = *piVar1 - qVar4;
      if (*piVar1 == 0x0) {
        uStack24 = (uVar8 | uVar9);
        if ((uVar8 | uVar9) != 0x0) {
          ppcVar3 = (code **)*puVar5;
          (**ppcVar3)((char)uVar13,uVar9,uVar8,0x1);
        }
      }
      else {
        ppcVar3 = (code **)(*puVar2 + 0x8);
        (**ppcVar3)(uVar13,puVar2,uVar12,uVar9,uVar8);
      }
      uStack12 -= uStack34;
      puVar7 = 0x0;
      uStack42 = 0x0;
      iVar12 = (astruct_418 *)uVar14;
      if (iVar12->field_0x6 == 0x0) {
        if (iVar12->field_0x8 != 0x0) {
          mem_op_1000_179c(0x2a,uStack24,0x1000);
          uVar10 = uStack24 | puVar7;
          uVar14 = uVar10;
          if (uVar10 == 0x0) goto LAB_1038_244e;
          pass1_1038_6838((u16 *)(puVar7 & 0xffff | uStack24 << 0x10),uVar6,
                          iVar12->field_0x8,0x1,iVar11->field_0x4);
          goto LAB_1038_24b3;
        }
      }
      else {
        mem_op_1000_179c(0x2a,uStack24,0x1000);
        uVar10 = uStack24 | puVar7;
        uVar14 = uVar10;
        if (uVar10 == 0x0) {
LAB_1038_244e:
          uVar13 = 0x1000;
          uStack42 = 0x0;
          uStack24 = uVar14;
        }
        else {
          pass1_1038_675c((u16 *)(puVar7 & 0xffff | uStack24 << 0x10),uVar6,
                          iVar12->field_0x6,0x1,iVar11->field_0x4);
LAB_1038_24b3:
          uVar13 = 0x1000;
          uStack42 = puVar7 & 0xffff | uVar14 << 0x10;
          uStack24 = uVar14;
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
  pass1_1030_6c4c(param_3,iStack8);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_24e8(param_1: u16,param_2: u16,param_3: u32,param_4: u16,param_5: u16,
               param_6: u16)

{
  let uVar1: u16;
  let uVar2: u32;
  let puVar3: *mut u8
  let puVar4: *mut u8
  let iVar5: i16;
  let uVar6: u16;
  let uStack30: u16;
  astruct_18 *paStack28;
  let local_18: u32;
  let local_14: u16;
  let uStack18: u16;
  let uStack16: u32;
  let puStack12: u32;
  let iStack8: i16;
  let uStack6: u32;
  
  uStack6 = struct_op_1030_73a8(param_3);
  puVar4 = (uStack6 >> 0x10);
  uVar6 = (param_3 >> 0x10);
  iVar5 = param_3;
  iStack8 = (iVar5 + 0x34);
  puStack12 = *(u32 **)(uStack6 + 0x28);
  uStack16 = 0x64;
  uStack18 = (puStack12 + 0x4);
  uVar2 = uStack18;
  mem_op_1000_179c(0xa,puVar4,0x1000);
  uVar1 = uVar2;
  puVar3 = (puVar4 | uVar1);
  if (puVar3 == 0x0) {
    uVar1 = 0x0;
    puVar3 = 0x0;
  }
  else {
    pass1_1020_ba3e((long *)(uVar2 & 0xffff | ZEXT24(puVar4) << 0x10),0x5,0x5,param_5,
                    param_4);
  }
  paStack28 = (astruct_18 *)CONCAT22(puVar3,uVar1);
  for (uStack30 = 0x0; uVar2 = uStack18, uStack30 < uStack18; uStack30 += 0x1) {
    pass1_1020_bb16(puStack12,CONCAT22(param_6,&local_18),
                    CONCAT22(param_6,&local_14),uStack30);
    if (local_18 != 0x0) {
      uVar2 = local_18;
      uStack16._2_2_ = local_18._2_2_;
      if (uStack16 < local_18) {
        uVar2 = uStack16 & 0xffff;
      }
      uVar1 = uVar2;
      uVar2 = uVar2 & 0xffff | ZEXT24(uStack16._2_2_) << 0x10;
      local_18 = CONCAT22(local_18._2_2_ +
                          (-(local_18 < uVar1) - uStack16._2_2_),
                          local_18 - uVar1);
      puVar3 = uStack16._2_2_;
      pass1_1020_bb8a((long *)puStack12,local_18 - uVar1,
                      CONCAT22(local_14,local_18._2_2_ +
                                        (-(local_18 < uVar1) -
                                        uStack16._2_2_)),param_5,param_6);
      pass1_1020_bb70((long *)paStack28,uVar1,CONCAT22(local_14,uStack16._2_2_),param_5,
                      param_4,param_6);
      uStack16 -= uVar2;
      if (uStack16 == 0x0) {
        mem_op_1000_179c(0x2a,puVar3,0x1000);
        puVar4 = (puVar3 | uVar2);
        if (puVar4 == 0x0) {
          uVar2 = 0x0;
        }
        else {
          pass1_1038_666e((u16 *)(uVar2 & 0xffff | ZEXT24(puVar3) << 0x10),
                          (long *)paStack28,0x1,(iVar5 + 0x4));
        }
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
        mem_op_1000_179c(0xa,puVar4,0x1000);
        puVar3 = (puVar4 | uVar2);
        if (puVar3 == 0x0) {
          uVar2 = 0x0;
          puVar3 = 0x0;
        }
        else {
          pass1_1020_ba3e((long *)(uVar2 & 0xffff | ZEXT24(puVar4) << 0x10),0x5,0x5,
                          param_5,param_4);
        }
        paStack28 = (astruct_18 *)(uVar2 & 0xffff | ZEXT24(puVar3) << 0x10);
        iStack8 += -0x1;
        if (iStack8 == 0x0) break;
        uStack16 = 0x64;
      }
    }
  }
  pass1_1020_ba94((long *)paStack28);
  puVar3 = (puVar3 | uVar2);
  if (puVar3 == 0x0) {
    if (paStack28 != (astruct_18 *)0x0) {
      fn_ptr_1020_ba7e(paStack28);
      fn_ptr_1000_17ce(paStack28,0x1000);
    }
  }
  else {
    mem_op_1000_179c(0x2a,puVar3,0x1000);
    if ((puVar3 | uVar2) != 0x0) {
      pass1_1038_666e((u16 *)(uVar2 & 0xffff | ZEXT24(puVar3) << 0x10),
                      (long *)paStack28,0x1,(iVar5 + 0x4));
    }
    pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
  }
  pass1_1030_6c4c(param_3,iStack8);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_26ee(param_1: u16,param_2: u16,param_3: u32,param_4: u16,param_5: u16,
               param_6: u16)

{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u32;
  let puVar4: *mut u8
  let puVar5: *mut u8
  let puVar6: *mut u8
  let iVar7: i16;
  let uVar8: u16;
  let uVar9: u32;
  let uVar10: u32;
  let uStack36: u32;
  let uStack20: u16;
  let puStack18: *mut u8
  let uStack16: u32;
  let uStack12: u16;
  let uStack10: u16;
  let iStack8: i16;
  
  uVar9 = struct_op_1030_73a8(param_3);
  puVar6 = (uVar9 >> 0x10);
  uVar8 = (param_3 >> 0x10);
  iVar7 = param_3;
  iStack8 = (iVar7 + 0x34);
  uStack12 = pass1_1028_0d80(uVar9);
  uVar3 = uStack12;
  uStack16 = 0x64;
  mem_op_1000_179c(0xa,puVar6,0x1000);
  puVar4 = (puVar6 | uVar3);
  if (puVar4 == 0x0) {
    uVar3 = 0x0;
    puVar4 = 0x0;
  }
  else {
    pass1_1020_ba3e((long *)(uVar3 & 0xffff | ZEXT24(puVar6) << 0x10),0x5,0x5,param_5,
                    param_4);
  }
  uStack20 = uVar3;
  uStack10 = uStack12;
  puStack18 = puVar4;
  do {
    uVar10 = pass1_1030_7c28(param_3,uStack10,uVar3,puVar4,param_6);
    puVar6 = (uVar10 >> 0x10);
    uVar1 = uVar10;
    puVar4 = (puVar6 | uVar1);
    if (puVar4 != 0x0) {
      puVar5 = puVar6;
      uVar2 = uVar1;
      if ((uStack16._2_2_ <= puVar6) &&
         ((uStack16._2_2_ < puVar6 || (uStack16 < uVar1)))) {
        puVar5 = uStack16._2_2_;
        uVar2 = uStack16;
      }
      uStack36 = CONCAT22(puVar5,uVar2);
      puVar4 = puVar5;
      pass1_1030_7d1c(param_3,uVar1 - uVar2,
                      CONCAT22(uStack10,puVar6 + (-(uVar1 < uVar2) - puVar5)),
                      uVar2,puVar5,param_4,param_5,param_6);
      pass1_1020_bb70((long *)CONCAT22(puStack18,uStack20),uVar2,CONCAT22(uStack10,puVar5)
                      ,param_5,param_4,param_6);
      uStack16 -= uStack36;
      if (uStack16 == 0x0) {
        mem_op_1000_179c(0x2a,puVar4,0x1000);
        uStack10 = uStack36;
        puVar6 = (puVar4 | uStack10);
        if (puVar6 == 0x0) {
          uStack10 = 0x0;
        }
        else {
          pass1_1038_666e((u16 *)(uStack36 & 0xffff | ZEXT24(puVar4) << 0x10),
                          (long *)CONCAT22(puStack18,uStack20),0x1,(iVar7 + 0x4)
                         );
        }
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
        mem_op_1000_179c(0xa,puVar6,0x1000);
        puVar4 = (puVar6 | uStack10);
        if (puVar4 == 0x0) {
          uStack10 = 0x0;
          puVar4 = 0x0;
        }
        else {
          pass1_1020_ba3e((long *)CONCAT22(puVar6,uStack10),0x5,0x5,param_5,param_4);
        }
        iStack8 += -0x1;
        uStack20 = uStack10;
        puStack18 = puVar4;
        if (iStack8 == 0x0) break;
        uStack16 = 0x64;
      }
    }
    uStack10 = pass1_1028_0d80(uVar9);
    uVar3 = uStack10;
    if (uStack12 == 0x0) {
      uStack12 = uStack10;
    }
  } while (uStack12 != uStack10);
  pass1_1020_ba94((long *)CONCAT22(puStack18,uStack20));
  puVar4 = (puVar4 | uStack10);
  if (puVar4 == 0x0) {
    if ((puStack18 | uStack20) != 0x0) {
      fn_ptr_1020_ba7e(CONCAT22(puStack18,uStack20));
      fn_ptr_1000_17ce((astruct_18 *)CONCAT22(puStack18,uStack20),0x1000);
    }
  }
  else {
    mem_op_1000_179c(0x2a,puVar4,0x1000);
    if ((puVar4 | uStack10) != 0x0) {
      pass1_1038_666e((u16 *)CONCAT22(puVar4,uStack10),
                      (long *)CONCAT22(puStack18,uStack20),0x1,(iVar7 + 0x4));
    }
    pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
  }
  pass1_1030_6c4c(param_3,iStack8);
  return;
}



astruct_100 * 
pass1_1038_28d8(astruct_100 *param_1,param_2: u16,param_3: u8)

{
  struct_op_1028_d1dc(param_2,param_3,param_1,0x3a97);
  param_1->field_0x0 = 0x29fe;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  unk_str_op_1000_3d3e
            ((param_1 & 0xffff0000 | (param_1 + 0x8)),
             s_SCRoboMove_1050_59f8);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_290e(param_1: u16,param_2: u16) -> u16

{
  let unaff_SI: u16;
  let unaff_DI: u16;
  let unaff_SS: u16;
  ulet in_AF: u8;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,0x1,0x400);
  if ((param_2 | param_1) != 0x0) {
    pass1_1038_4918(CONCAT22(param_2,param_1),param_1,param_2 | param_1,unaff_SS,in_AF);
  }
  pass1_1038_7a76(_PTR_LOOP_1050_5a64,unaff_SI,unaff_DI,unaff_SS);
  return 0x1;
}



fn pass1_1038_2944(param_1: u32,param_2: u16,uchar *param_3)
{
  let puVar1: u32;
  let puVar2: u32;
  let puVar3: u32;
  let iVar4: i16;
  let puVar5: u32;
  let uVar6: u16;
  let puStack10: *mut u16;
  
  mem_op_1000_179c(0x108,param_3,0x1000);
  puStack10 = CONCAT22(param_3,param_2);
  if ((param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    (param_2 + 0x2) = 0x1008;
    uVar6 = (param_1 >> 0x10);
    (param_2 + 0x4) = (param_1 + 0x4);
    puVar3 = (param_1 + 0x8);
    puVar5 = (param_2 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 += -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    (param_2 + 0x2) = &USHORT_1050_1028;
    *puStack10 = 0x29fe;
    (param_2 + 0x2) = &PTR_LOOP_1050_1038;
  }
  return;
}



astruct_18 *  pass1_1038_29d2(astruct_18 *param_1,param_2: u8)

{
  param_1->field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void 
pass1_1038_2a0e(astruct_100 *param_1,param_2: u32,param_3: u32,param_4: u32,
               param_5: u32,param_6: u16,param_7: u8)

{
  let iVar1: i16;
  let uVar2: u16;
  
  struct_op_1028_d1dc(param_6,param_7,param_1,0x2af7);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x108) = param_5;
  (iVar1 + 0x10c) = param_4;
  (iVar1 + 0x110) = param_3;
  (iVar1 + 0x114) = param_2;
  param_1->field_0x0 = 0x309a;
  (iVar1 + 0x2) = &PTR_LOOP_1050_1038;
  return;
}



fn pass1_1038_2a5c(param_1: *mut u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0x309a;
  (iVar4 + 0x2) = &PTR_LOOP_1050_1038;
  puVar1 = (iVar4 + 0x114);
  uVar2 = (iVar4 + 0x116);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = (iVar4 + 0x110);
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

u16 
pass1_1038_2ac2(param_1: u32,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
               param_6: u16,param_7: u8)

{
  let uVar1: u32;
  let uVar2: u16;
  let uVar3: u16;
  let uStack10: u32;
  let uStack6: u32;
  
  uVar3 = (param_1 >> 0x10);
  uVar2 = param_1;
  uVar1 = (uVar2 + 0x108);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
  uStack6 = CONCAT22(param_3,param_2);
  uVar1 = (uVar2 + 0x10c);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
  uStack10 = CONCAT22(param_3,param_2);
  pass1_1038_2c82(uVar2,uVar3,(uVar2 + 0x110),CONCAT22(param_3,param_2),uStack6,
                  param_4,param_5,&USHORT_1050_1028,param_6,param_7);
  pass1_1038_2c82(uVar2,uVar3,(uVar2 + 0x114),uStack6,uStack10,param_4,param_5,
                  &USHORT_1050_1028,param_6,param_7);
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_2b2e(param_1: u32,param_2: u16,param_3: u16) -> u16

{
  let uVar1: u32;
  let uVar2: u16;
  let unaff_SI: u16;
  let unaff_DI: u16;
  let uVar3: u16;
  let unaff_SS: u16;
  let uStack6: u32;
  
  uVar3 = (param_1 >> 0x10);
  uVar2 = param_1;
  uVar1 = (uVar2 + 0x108);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
  uStack6 = CONCAT22(param_3,param_2);
  uVar1 = (uVar2 + 0x10c);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
  pass1_1038_2f92(uVar2,uVar3,(uVar2 + 0x110),CONCAT22(param_3,param_2),unaff_SI
                  ,unaff_DI,unaff_SS);
  pass1_1038_2f92(uVar2,uVar3,(uVar2 + 0x114),uStack6,unaff_SI,unaff_DI,unaff_SS
                 );
  return 0x1;
}



fn pass1_1038_2b9a(param_1: u32,astruct_422 *param_2,uchar *param_3)
{
  let puVar1: u32;
  let puVar2: u32;
  let iVar3: i16;
  astruct_421 *iVar5;
  let puVar4: u32;
  let puVar5: u32;
  let uVar6: u16;
  let puStack10: *mut u16;
  
  mem_op_1000_179c(0x118,param_3,0x1000);
  puStack10 = CONCAT22(param_3,param_2);
  iVar5 = (astruct_421 *)param_1;
  uVar6 = (param_1 >> 0x10);
  if ((param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    param_2->field_0x2 = 0x1008;
    param_2->field_0x4 = iVar5->field_0x4;
    puVar4 = &iVar5->field_0x8;
    puVar5 = &param_2->field_0x8;
    for (iVar3 = 0x40; iVar3 != 0x0; iVar3 += -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar4;
      puVar4 = puVar4 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    param_2->field_0x2 = &USHORT_1050_1028;
    param_2->field_0x108 = iVar5->field_0x108;
    param_2->field_0x10c = iVar5->field_0x10c;
    param_2->field_0x110 = iVar5->field_0x110;
    param_2->field_0x114 = iVar5->field_0x114;
    *puStack10 = 0x309a;
    param_2->field_0x2 = &PTR_LOOP_1050_1038;
  }
  iVar5->field_0x114 = 0x0;
  iVar5->field_0x110 = 0x0;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_2c82(param_1: u16,param_2: u16,param_3: u32,param_4: u32,param_5: u32,
               param_6: u16,param_7: u16,param_8: u16,param_9: u16,param_10: u8)

{
  let puVar1: *mut u16;
  let piVar2: *mut i16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u32;
  let lVar6: i32;
  code **ppcVar7;
  let uVar8: u16;
  let puVar9: u32;
  let iVar10: i16;
  let uVar11: u32;
  let puVar12: *mut u8
  let puVar13: *mut u8
  let uVar14: u16;
  let iVar16: i16;
  astruct_702 *iVar15;
  let uVar17: u16;
  let uVar18: u16;
  let puVar19: *mut u8
  let puVar20: *mut u16;
  let uVar21: u8;
  let uStack22: u32;
  let local_12: u32;
  let puStack14: *mut u16;
  let uStack10: u32;
  let uStack6: u32;
  
  uVar17 = (param_5 >> 0x10);
  uVar14 = param_5;
  uStack6 = (uVar14 + 0x200);
  uVar18 = (param_4 >> 0x10);
  iVar16 = param_4;
  uStack10 = (iVar16 + 0x200);
  puVar13 = *(uchar **)(iVar16 + 0x202);
  puVar19 = (param_3 >> 0x10);
  iVar15 = (astruct_702 *)param_3;
  iVar10 = iVar15->field_0xc;
  if (iVar10 == 0x1) {
    puStack14 = param_3;
    pass1_1038_52b8(param_5,&iVar15->field_0x8,&iVar15->field_0xe,
                    param_6,param_7,param_8,param_9);
    return;
  }
  if (iVar10 == 0x2) {
    puStack14 = param_3;
    if (iVar15->field_0xe != 0x0) {
      pass1_1038_3efc(uVar14,uVar17,param_4,iVar15->field_0xe,iVar15,puVar19)
      ;
      return;
    }
    pass1_1020_a43e(param_9,puVar19,CONCAT22(param_9,&local_12));
    uStack22 = *(long *)(puStack14 + 0x8);
    while( true ) {
      uStack22 += -0x1;
      if ((uStack22._2_2_ | uStack22) == 0x0) break;
      pass1_1020_a6ee(CONCAT13((char)(param_9 >> 0x8),CONCAT12((char)param_9,&local_12)),
                      (puStack14 + 0x12),&local_12,
                      uStack22._2_2_ | uStack22,param_7,param_9,param_10);
    }
  }
  else {
    if (iVar10 == 0x3) {
      pass1_1038_3f38(param_5,param_4,iVar15->field_0xe,0x0,
                      puVar13);
      return;
    }
    uStack6._2_2_ = (uStack6 >> 0x10);
    if (iVar10 == 0x4) {
      PTR_LOOP_1050_5f2e = (uStack6._2_2_ & 0xff);
      if ((uStack6 == 0x1) && ((uStack6 & 0xff0000) == 0x0)) {
        local_12 = (uVar14 + 0x1f6);
        pass1_1030_3694(local_12,&iVar15->field_0xe,*(long *)&iVar15->field_0x8,
                        0x0,0x1030,param_9);
        (&iVar15->field_0xe + 0x2) = local_12;
        iVar15->field_0x12 = PTR_LOOP_1050_5f2e;
      }
      else {
        if (_PTR_LOOP_1050_5f2c == 0x0) {
          PTR_LOOP_1050_5f2c = mem_op_1000_160a(PTR_LOOP_1050_5f2e,0x1000);
        }
        else {
        }
        uVar14 = fn_ptr_op_1000_1708(0x16c,0x0,0x1,PTR_LOOP_1050_5f2c,
                                     PTR_LOOP_1050_5f2e,0x1000);
        (&iVar15->field_0xe + 0x2) = uVar14;
        iVar15->field_0x12 = PTR_LOOP_1050_5f2e;
        iVar10 = &iVar15->field_0xe;
        if (iVar10 != 0x3) {
          if (iVar10 != 0x4) {
            uVar5 = (&iVar15->field_0xe + 0x2);
            (uVar5 + 0x28) = &iVar15->field_0x8;
            return;
          }
          uVar5 = (&iVar15->field_0xe + 0x2);
          (uVar5 + 0xdc) = &iVar15->field_0x8;
          return;
        }
        uVar5 = (&iVar15->field_0xe + 0x2);
        (uVar5 + 0x64) = &iVar15->field_0x8;
      }
    }
    else {
      if (iVar10 == 0x5) {
        if (&iVar15->field_0xe == 0xc) {
          if ((uStack6 == 0x1) && ((uStack6 & 0xff0000) == 0x0)) {
            uVar5 = (uVar14 + 0x1f6);
            uVar3 = iVar15->field_0x8;
            iVar10 = iVar15->field_0xa;
            uVar8 = -uVar3;
            uVar18 = (uVar5 >> 0x10);
            iVar16 = uVar5;
            puVar1 = (iVar16 + 0x170);
            uVar4 = *puVar1;
            *puVar1 = *puVar1 + uVar8;
            piVar2 = (i16 *)(iVar16 + 0x172);
            *piVar2 = (*piVar2 - (iVar10 + (uVar3 != 0x0))) +
                      CARRY2(uVar4,uVar8);
          }
        }
        else {
          pass1_1038_43cc(uVar14,uVar17,iVar15->field_0x8,&iVar15->field_0xe,
                          iVar15,puVar19);
        }
      }
      else {
        iVar10 += -0x7;
        if (iVar10 == 0x0) {
          lVar6 = iVar15->field_0xe;
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,lVar6,(lVar6 >> 0x10));
          puVar12 = puVar13;
          pass1_1038_349e(CONCAT22(puVar13,iVar10),(iVar16 + 0x200));
          uVar21 = (u8)(puVar13 >> 0x8);
          pass1_1038_4d0e(CONCAT13(uVar21,CONCAT12((char)puVar13,iVar10)),0x258);
          pass1_1038_4d0e(CONCAT13(uVar21,CONCAT12((char)puVar13,iVar10)),0x258);
          puVar20 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3b,param_9,puVar12,param_7);
          puVar13 = (puVar20 >> 0x10);
          pass1_1008_de58(puVar20,iVar15->field_0xe,0x4000001,param_9);
          puVar20 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_9,puVar13,param_7);
          puVar13 = (puVar20 >> 0x10);
          uVar11 = (puVar20 + 0x20);
          pass1_1030_8344(_PTR_LOOP_1050_5748,
                          (_PTR_LOOP_1050_5748 >> 0x10),uVar11);
          local_12 = uVar11 & 0xffff | ZEXT24(puVar13) << 0x10;
          uVar14 = pass1_1030_5b00(uVar11 & 0xffff | ZEXT24(puVar13) << 0x10);
          puStack14 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,uVar14,param_9,puVar13,param_7);
          puVar9 = (puStack14 + 0x20);
          ppcVar7 = (code **)(*puVar9 + 0x4);
          (**ppcVar7)(0x1010,puVar9,(puStack14 >> 0x10),0x6);
        }
      }
    }
  }
  return;
}



void 
pass1_1038_2f92(param_1: u16,param_2: u16,param_3: u32,param_4: u32,param_5: u16,
               param_6: u16,param_7: u16)

{
  let puVar1: *mut u16;
  let piVar2: *mut i16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u32;
  let uVar7: u32;
  let iVar8: i16;
  let iVar9: i16;
  let uVar10: u16;
  let uVar11: u16;
  let uStack10: i16;
  
  uVar10 = (param_4 >> 0x10);
  iVar8 = param_4;
  uVar6 = (iVar8 + 0x200);
  uVar11 = (param_3 >> 0x10);
  iVar9 = param_3;
  iVar3 = (iVar9 + 0xc);
  if (iVar3 == 0x1) {
    uVar7 = (iVar9 + 0x8);
    pass1_1038_3cc0(param_4,uVar7,(uVar7 >> 0x10),
                    (iVar9 + 0xe),param_5,param_6,param_7);
    return;
  }
  if (iVar3 == 0x4) {
    pass1_1030_355c((iVar8 + 0x1f6),(iVar9 + 0x10));
    return;
  }
  if (iVar3 == 0x5) {
    if ((iVar9 + 0xe) != 0xc) {
      pass1_1038_5798(param_4,*(long *)(iVar9 + 0x8),(iVar9 + 0xe));
      return;
    }
    iStack10 = uVar6;
    if ((iStack10 == 0x1) && ((uVar6 & 0xff0000) == 0x0)) {
      uVar7 = (iVar8 + 0x1f6);
      uVar4 = (iVar9 + 0x8);
      iVar3 = (iVar9 + 0xa);
      uVar10 = (uVar7 >> 0x10);
      iVar8 = uVar7;
      puVar1 = (iVar8 + 0x170);
      uVar5 = *puVar1;
      *puVar1 = *puVar1 + uVar4;
      piVar2 = (i16 *)(iVar8 + 0x172);
      *piVar2 = *piVar2 + iVar3 + CARRY2(uVar5,uVar4);
      return;
    }
  }
  return;
}



astruct_18 *  pass1_1038_3074(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_2a5c((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1038_30aa(param_1: *mut u16,param_2: u16)
{
  let puVar1: *mut u16;
  let puVar2: *mut u8
  let puVar3: *mut u8
  let uVar4: u16;
  astruct_423 *iVar5;
  let uVar5: u16;
  let puVar6: *mut u16;
  
  puVar6 = struct_1030_17ce(param_1,0x0,0x0);
  puVar2 = (puVar6 >> 0x10);
  uVar5 = (param_1 >> 0x10);
  iVar5 = (astruct_423 *)param_1;
  iVar5->field_0x10 = 0x0;
  iVar5->field_0x14 = 0x0;
  iVar5->field_0x18 = 0x258;
  iVar5->field_0x1a = 0x258;
  iVar5->field_0x1c = 0x0;
  iVar5->field_0x1e = 0x0;
  iVar5->field_0x22 = 0x0;
  iVar5->field_0x24 = 0x32;
  &iVar5->field_0x1f6 = 0x0;
  iVar5->field_0x1fa = 0x0;
  iVar5->field_0x1fe = 0x0;
  iVar5->field_0x200 = 0x8000001;
  iVar5->field_0x204 = 0x0;
  iVar5->field_0x206 = 0x0;
  iVar5->field_0x208 = 0x1;
  iVar5->field_0x20a = 0x0;
  iVar5->field_0x20c = 0x0;
  iVar5->field_0x20e = 0x0;
  iVar5->field_0x210 = 0x0;
  iVar5->field_0x214 = 0x0;
  iVar5->field_0x216 = 0x0;
  iVar5->field_0x21a = 0x0;
  *param_1 = 0x6504;
  iVar5->field_0x2 = &PTR_LOOP_1050_1038;
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &iVar5->field_0x26),
                  (WNDCLASS16 *)0x0,0x94);
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &iVar5->field_0xba),
                  (WNDCLASS16 *)0x0,0x94);
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &iVar5->field_0x14e),
                  (WNDCLASS16 *)0x0,0x54);
  puVar1 = pass1_1000_4906((astruct_20 *)
                           (param_1 & 0xffff0000 | &iVar5->field_0x1a2
                           ),(WNDCLASS16 *)0x0,0x54);
  mem_op_1000_179c(0x1b0,puVar2,0x1000);
  puVar3 = (puVar2 | puVar1);
  if (puVar3 == 0x0) {
    &iVar5->field_0x1f6 = 0x0;
  }
  else {
    pass1_1030_314c((u16 *)CONCAT22(puVar2,puVar1),iVar5->field_0x4,puVar3,param_2);
    iVar5->field_0x1f6 = puVar1;
    iVar5->field_0x1f8 = puVar3;
  }
  mem_op_1000_179c(0x1e,puVar3,0x1000);
  uVar4 = puVar3 | puVar1;
  if (uVar4 == 0x0) {
    puVar1 = 0x0;
    uVar4 = 0x0;
  }
  else {
    struct_1020_c444((astruct_75 *)CONCAT22(puVar3,puVar1),0x64,0xc8);
  }
  iVar5->field_0xc = puVar1;
  iVar5->field_0xe = uVar4;
  return;
}



void 
pass1_1038_3222(param_1: *mut u16,param_2: u32,param_3: u32,param_4: u16,uchar *param_5,
               param_6: u8,uchar *param_7)

{
  let puVar1: *mut u16;
  let puVar2: *mut u8
  let uVar3: u16;
  let uVar4: u16;
  astruct_363 *iVar5;
  let uVar5: u16;
  let puVar6: *mut u16;
  uchar local_16 [0x14];
  
  puVar6 = pass1_1030_183c(param_1,0x0,0x0,0x4000000,param_3,param_4,param_5);
  puVar2 = (puVar6 >> 0x10);
  uVar5 = (param_1 >> 0x10);
  iVar5 = (astruct_363 *)param_1;
  iVar5->field_0x10 = param_2;
  iVar5->field_0x14 = 0x0;
  iVar5->field_0x18 = 0x258;
  iVar5->field_0x1a = 0x258;
  iVar5->field_0x1c = 0x0;
  iVar5->field_0x1e = 0x0;
  iVar5->field_0x22 = 0x0;
  iVar5->field_0x24 = 0x32;
  &iVar5->field_0x1f6 = 0x0;
  &iVar5->field_0x1fa = 0x0;
  iVar5->field_0x1fe = 0x0;
  iVar5->field_0x200 = 0x8000001;
  iVar5->field_0x204 = 0x0;
  iVar5->field_0x206 = 0x0;
  iVar5->field_0x208 = 0x1;
  iVar5->field_0x20a = 0x0;
  iVar5->field_0x20c = 0x0;
  iVar5->field_0x20e = 0x0;
  iVar5->field_0x210 = 0x0;
  iVar5->field_0x214 = 0x0;
  iVar5->field_0x216 = 0x0;
  iVar5->field_0x21a = 0x0;
  *param_1 = 0x6504;
  iVar5->field_0x2 = &PTR_LOOP_1050_1038;
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &iVar5->field_0x26),
                  (WNDCLASS16 *)0x0,0x94);
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &iVar5->field_0xba),
                  (WNDCLASS16 *)0x0,0x94);
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &iVar5->field_0x14e),
                  (WNDCLASS16 *)0x0,0x54);
  puVar1 = pass1_1000_4906((astruct_20 *)
                           (param_1 & 0xffff0000 | &iVar5->field_0x1a2
                           ),(WNDCLASS16 *)0x0,0x54);
  mem_op_1000_179c(0x1b0,puVar2,0x1000);
  uVar3 = puVar2 | puVar1;
  if (uVar3 == 0x0) {
    &iVar5->field_0x1f6 = 0x0;
  }
  else {
    pass1_1030_314c((u16 *)CONCAT22(puVar2,puVar1),&iVar5->field_0x4,uVar3,
                    param_7);
    iVar5->field_0x1f6 = puVar1;
    iVar5->field_0x1f8 = uVar3;
  }
  puVar2 = (iVar5->field_0x6 & 0xff);
  sys_1000_3f9c(local_16,param_7,0x5a1a,&USHORT_1050_1050,
                &iVar5->field_0x4,&stack0xfffe,uVar5,0x1000,param_7
                ,param_6);
  uVar3 = str_op_1008_60e8(CONCAT22(param_7,local_16),puVar2);
  iVar5->field_0x1fa = uVar3;
  iVar5->field_0x1fc = puVar2;
  mem_op_1000_179c(0x1e,puVar2,0x1000);
  uVar4 = puVar2 | uVar3;
  if (uVar4 == 0x0) {
    &iVar5->field_0xc = 0x0;
  }
  else {
    struct_1020_c444((astruct_75 *)CONCAT22(puVar2,uVar3),0x64,0xc8);
    iVar5->field_0xc = uVar3;
    iVar5->field_0xe = uVar4;
  }
  return;
}



fn pass1_1038_33f8(param_1: *mut u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  *param_1 = 0x6504;
  (iVar4 + 0x2) = &PTR_LOOP_1050_1038;
  puVar1 = (iVar4 + 0x14);
  uVar2 = (iVar4 + 0x16);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = (iVar4 + 0x1f6);
  uVar2 = (iVar4 + 0x1f8);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar4 + 0x1fa),0x1000);
  puVar1 = (iVar4 + 0x210);
  uVar2 = (iVar4 + 0x212);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1000,puVar1,uVar2,0x1);
  }
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar4 + 0x21a),0x1000);
  pass1_1030_18b2(param_1);
  return;
}



fn pass1_1038_349e(param_1: u32,param_2: u32)
{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u16;
  let extraout_DX: u16;
  let uVar4: u16;
  let uVar5: u16;
  let extraout_DX_00: u16;
  astruct_685 *iVar7;
  let uVar6: u16;
  let puVar7: u32;
  let uVar8: u16;
  let uVar9: u16;
  let uStack10: u32;
  let uStack6: u32;
  
  uVar6 = (param_1 >> 0x10);
  iVar7 = (astruct_685 *)param_1;
  iVar7->field_0x200 = param_2;
  pass1_1038_4d0e(param_1,0x258);
  uVar3 = param_2;
  pass1_1038_4d0e(param_1,0x258);
  iVar7->field_0x204 = 0x0;
  iVar7->field_0x206 = 0x0;
  puVar7 = iVar7->field_0xc;
  uVar8 = SUB42(puVar7,0x0);
  uVar9 = (puVar7 >> 0x10);
  ppcVar1 = (code **)(*iVar7->field_0xc + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,uVar3);
  uVar5 = extraout_DX;
  for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
    puVar7 = pass1_1030_1d7c(uVar3,uVar5,iVar7->field_0xc);
    uVar4 = (puVar7 >> 0x10);
    uVar2 = puVar7;
    uVar5 = uVar4 | uVar2;
    if (uVar5 != 0x0) {
      ppcVar1 = (code **)(*puVar7 + 0x58);
      (**ppcVar1)(0x1030,uVar2,uVar4,param_1,uVar6,uVar8,uVar9);
      (uVar2 + 0x1c) = 0x0;
      uVar5 = extraout_DX_00;
    }
  }
  return;
}



fn pass1_1038_354a(param_1: u32,param_2: u16,uchar *param_3)
{
  let uVar1: u16;
  astruct_424 *iVar1;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = (astruct_424 *)param_1;
  if (*(long *)&iVar1->field_0x21a == 0x0) {
    mem_op_1000_179c(0xa,param_3,0x1000);
    uVar1 = param_3 | param_2;
    if (uVar1 == 0x0) {
      &iVar1->field_0x21a = 0x0;
    }
    else {
      pass1_1030_9ecc(CONCAT22(param_3,param_2),param_1);
      &iVar1->field_0x21a = param_2;
      iVar1->field_0x21c = uVar1;
    }
  }
  pass1_1030_9ef2(*(u32 **)&iVar1->field_0x21a);
  return;
}



fn pass1_1038_35a8(param_1: u32,param_2: u16,param_3: u16,uchar *param_4)
{
  let uVar1: u16;
  astruct_425 *iVar3;
  let uVar2: u16;
  let unaff_SS: u16;
  ulet in_AF: u8;
  
  uVar2 = (param_1 >> 0x10);
  iVar3 = (astruct_425 *)param_1;
  if (*(long *)&iVar3->field_0x21a == 0x0) {
    mem_op_1000_179c(0xa,param_4,0x1000);
    uVar1 = param_4 | param_3;
    if (uVar1 == 0x0) {
      &iVar3->field_0x21a = 0x0;
    }
    else {
      pass1_1030_9ecc(CONCAT22(param_4,param_3),param_1);
      &iVar3->field_0x21a = param_3;
      iVar3->field_0x21c = uVar1;
    }
  }
  pass1_1030_9f40(&iVar3->field_0x21a,param_2,unaff_SS,in_AF);
  return;
}



fn pass1_1038_3608(param_1: u32)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0x21a),0x1000);
  (param_1 + 0x21a) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_362e(param_1: u32)
{
  let in_AX: u16;
  let in_DX: *mut u8
  let iVar1: i16;
  let unaff_DI: i16;
  let uVar2: u16;
  let unaff_SS: u16;
  astruct_67 *paVar3;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((iVar1 + 0x214) == 0x0) {
    pass1_1038_4f54(param_1 & 0xffff | uVar2 << 0x10,0x1f,in_AX);
    if (in_AX == 0x0) {
      (iVar1 + 0x214) = 0x14;
    }
    else {
      (iVar1 + 0x214) = 0x28;
    }
    paVar3 = (astruct_67 *)
             mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,unaff_SS,in_DX,unaff_DI);
    post_win_msg_1008_a0e4
              (paVar3,0x0,0x0,0x1,(iVar1 + 0x4),0x38,0x1008,unaff_SS);
    (iVar1 + 0x216) = 0x0;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_3698(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  let piVar1: *mut i16;
  let puVar2: *mut u16;
  let uVar3: u32;
  code **ppcVar4;
  let uVar5: u16;
  let BVar6: bool;
  let uVar7: u16;
  let uVar8: u16;
  let lVar9: i32;
  let uVar10: u32;
  let uVar11: u16;
  let uVar12: u16;
  let uVar13: u16;
  let uVar14: u32;
  let iVar15: i16;
  let uVar16: u16;
  let uVar17: u32;
  let uStack32: u32;
  let uStack18: u32;
  let uStack14: u32;
  let uStack10: u32;
  let uStack6: u32;
  
  uVar16 = (param_1 >> 0x10);
  iVar15 = param_1;
  if ((iVar15 + 0x214) == 0x0) {
    return;
  }
  pass1_1030_38b8();
  uStack6 = CONCAT22(param_3,param_2);
  uStack6 -= *(long *)(iVar15 + 0x216);
  if (0x0 < uStack6) {
    uStack6 += 0x3;
    uStack10 = uStack6 / 0x5;
    uVar14 = uStack6 % 0x5;
    if (*(long *)(iVar15 + 0xc) == 0x0) {
      uVar5 = 0x0;
      uVar14 = 0x0;
    }
    else {
      uVar3 = (iVar15 + 0xc);
      ppcVar4 = (code **)((iVar15 + 0xc) + 0x10);
      lVar9 = uStack10;
      (**ppcVar4)(0x1030,uVar3,(uVar3 >> 0x10));
      uVar5 = lVar9;
    }
    uStack14 = CONCAT22(uVar14,uVar5);
    for (uStack18 = 0x0; uVar12 = uVar14, uVar10 = uStack14,
        uStack18 < uStack14; uStack18 += 0x1) {
      uVar17 = pass1_1030_1d7c(uVar5,uVar12,(iVar15 + 0xc));
      uVar8 = (uVar17 >> 0x10);
      uVar13 = uVar8 | uVar17;
      uVar14 = uVar13;
      if (uVar13 != 0x0) {
        BVar6 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,(uVar17 + 0xc),
                                0x4);
        uVar8 = uVar14;
        uVar10 = BVar6;
        if (BVar6 != 0x0) {
          uVar7 = pass1_1028_678c(uVar17,0xf,param_4);
          uStack32 = CONCAT22(uVar8,uVar7);
          uVar14 = (uVar8 | uVar7);
          uVar10 = uVar7;
          if ((uVar8 | uVar7) != 0x0) {
            if (uStack10 < (long)uStack32) {
              uVar8 = uStack10;
              pass1_1028_6356(uVar17,0xf,uVar8,uStack10._2_2_,param_4);
              uVar13 = uVar8 * 0x5;
              uVar11 = uStack10._2_2_ * 0x5 + CARRY2(uVar8,uVar8) * 0x2 +
                       CARRY2(uVar8 * 0x2,uVar8 * 0x2) +
                       CARRY2(uVar8 * 0x4,uVar8);
              uVar14 = uVar11;
              puVar2 = (iVar15 + 0x216);
              uVar8 = *puVar2;
              *puVar2 = *puVar2 + uVar13;
              piVar1 = (i16 *)(iVar15 + 0x218);
              *piVar1 = *piVar1 + uVar11 + CARRY2(uVar8,uVar13);
              uStack10 = 0x0;
              uVar10 = uVar13;
            }
            else {
              pass1_1028_6356(uVar17,0xf,uVar7,uVar8,param_4);
              uVar13 = uVar8 * 0x5 + CARRY2(uVar7,uVar7) * 0x2 +
                       CARRY2(uVar7 * 0x2,uVar7 * 0x2) +
                       CARRY2(uVar7 * 0x4,uVar7);
              uVar14 = uVar13;
              puVar2 = (iVar15 + 0x216);
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
    uStack6 -= *(long *)(iVar15 + 0x216);
    uStack6._2_2_ = (uStack6 >> 0x10);
    if ((uStack6._2_2_ | uStack6) != 0x0) {
      uStack32 = uStack6 / (long)(iVar15 + 0x214);
      if ((long)uStack32 < 0x1) {
        uStack32 = 0x1;
      }
      pass1_1030_375a((iVar15 + 0x1f6),0x0,uStack32,param_4);
    }
  }
  piVar1 = (i16 *)(iVar15 + 0x214);
  *piVar1 = *piVar1 + -0x1;
  return;
}



fn pass1_1038_387e(param_1: u32,param_2: i16,param_3: i16,param_4: u32,param_5: u16)
{
  code **ppcVar1;
  let lVar2: i32;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u32;
  let uVar6: u32;
  let uVar7: u32;
  let extraout_DX: *mut u8
  let puVar8: *mut u8
  let puVar9: *mut u8
  let uVar10: u16;
  let extraout_DX_00: u16;
  let uVar11: u16;
  astruct_302 *iVar10;
  let uVar12: u16;
  let iStack22: i16;
  let uStack12: u16;
  let uStack10: u32;
  let uStack6: u32;
  
  if (param_2 != param_3) {
    iVar10 = (astruct_302 *)param_1;
    uVar12 = (param_1 >> 0x10);
    if (param_2 < param_3) {
      uStack12 = param_3 - param_2;
      if ((iVar10->field_0x210 == 0x0) ||
         (lVar2 = iVar10->field_0x210, *(long *)(lVar2 + 0xa) == 0x0)) {
        if (iVar10->field_0xc == 0x0) {
          uVar11 = 0x0;
          puVar8 = 0x0;
        }
        else {
          ppcVar1 = (code **)(*iVar10->field_0xc + 0x10);
          uVar11 = uStack12;
          (**ppcVar1)();
          puVar8 = extraout_DX;
        }
        uStack6 = CONCAT22(puVar8,uVar11);
        for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
          uVar6 = uStack6;
          pass1_1030_1d58(iVar10->field_0xc);
          puVar9 = (puVar8 | uVar6);
          if ((puVar9 != 0x0) &&
             (uVar3 = pass1_1030_6fa0(uVar6 & 0xffff | ZEXT24(puVar8) << 0x10),
             uVar3 == 0xb)) {
            pass1_1030_7c50(CONCAT13((char)(puVar8 >> 0x8),
                                     CONCAT12((char)puVar8,uVar6)),
                            (long)uStack12,0x4,uStack12,puVar9);
            return;
          }
          puVar8 = puVar9;
        }
      }
      else {
        lVar2 = iVar10->field_0x210;
        uVar6 = (lVar2 + 0xa);
        for (uStack10 = 0x0; uStack10 < uVar6; uStack10 += 0x1) {
          uVar5 = uVar6;
          bad_1030_1312();
          uVar11 = uVar5;
          uVar10 = param_5 | uVar11;
          if (((uVar10 != 0x0) &&
              (pass1_1030_cc44(uVar11,param_5,uStack12,param_4,0x4), uVar11 != 0x0)) &&
             (uStack12 -= uVar11, uStack12 == 0x0)) {
            return;
          }
          param_5 = uVar10;
        }
      }
    }
    else {
      iStack22 = param_2 - param_3;
      if ((iVar10->field_0x210 == 0x0) ||
         (lVar2 = iVar10->field_0x210, *(long *)(lVar2 + 0xa) == 0x0)) {
        if (iVar10->field_0xc == 0x0) {
          iVar4 = 0x0;
          uVar11 = 0x0;
        }
        else {
          ppcVar1 = (code **)(*iVar10->field_0xc + 0x10);
          iVar4 = iStack22;
          (**ppcVar1)();
          uVar11 = extraout_DX_00;
        }
        uStack6 = CONCAT22(uVar11,iVar4);
        for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
          uVar6 = uStack6;
          pass1_1030_1d58(iVar10->field_0xc);
          uVar10 = uVar11 | uVar6;
          if ((uVar10 != 0x0) &&
             (uVar3 = pass1_1030_6fa0(uVar6 & 0xffff | uVar11 << 0x10),
             uVar3 == 0xb)) {
            pass1_1030_6e9c(CONCAT13((char)(uVar11 >> 0x8),
                                     CONCAT12((char)uVar11,uVar6)),(long)iStack22,
                            0x4);
            return;
          }
          uVar11 = uVar10;
        }
      }
      else {
        lVar2 = iVar10->field_0x210;
        uVar6 = (lVar2 + 0xa);
        for (uStack10 = 0x0; uStack10 < uVar6; uStack10 += 0x1) {
          uVar7 = uVar6;
          bad_1030_1312();
          uVar5 = param_5;
          uVar11 = uVar7;
          param_5 |= uVar11;
          if (param_5 != 0x0) {
            pass1_1030_ce72(uVar5 << 0x10 | uVar7 & 0xffff,iStack22,param_4,0x4);
            iStack22 -= uVar11;
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



fn pass1_1038_3aa6(param_1: u32,param_2: u16,param_3: u16)
{
  code **ppcVar1;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u32;
  let uVar5: u32;
  let extraout_DX: u16;
  let uVar6: u16;
  let uVar7: u16;
  let iVar8: i16;
  let uVar9: u16;
  let uStack12: u32;
  let uStack8: u32;
  
  uVar9 = (param_1 >> 0x10);
  iVar8 = param_1;
  if ((*(long *)(iVar8 + 0x210) == 0x0) ||
     (uVar2 = (iVar8 + 0x210), *(long *)(uVar2 + 0xa) == 0x0)) {
    if (*(long *)(iVar8 + 0xc) == 0x0) {
      param_2 = 0x0;
      uVar6 = 0x0;
    }
    else {
      ppcVar1 = (code **)((iVar8 + 0xc) + 0x10);
      (**ppcVar1)();
      uVar6 = extraout_DX;
    }
    uStack8 = CONCAT22(uVar6,param_2);
    for (uStack12 = 0x0; uStack12 < uStack8; uStack12 += 0x1) {
      uVar4 = uStack8;
      pass1_1030_1d58((iVar8 + 0xc));
      uVar7 = uVar6 | uVar4;
      if ((uVar7 != 0x0) &&
         (uVar3 = pass1_1030_6fa0(uVar4 & 0xffff | uVar6 << 0x10), uVar3 == 0xb)) {
        pass1_1030_6b86(uVar4 & 0xffff | uVar6 << 0x10,0xb,0x1030);
        return;
      }
      uVar6 = uVar7;
    }
  }
  else {
    uVar2 = (iVar8 + 0x210);
    uVar4 = (uVar2 + 0xa);
    for (uStack12 = 0x0; uStack12 < uVar4; uStack12 += 0x1) {
      uVar5 = uVar4;
      bad_1030_1312();
      uVar6 = param_3 | uVar5;
      if (uVar6 != 0x0) {
        pass1_1030_ce2e(uVar5,param_3,0x4);
      }
      param_3 = uVar6;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_3ba0(param_1: u32)
{
  let puVar1: u32;
  code **ppcVar2;
  let cVar3: u8;
  let puVar4: u32;
  let uVar5: u32;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u32;
  let puVar9: *mut u8
  let extraout_DX: *mut u8
  let puVar10: *mut u8
  let uVar11: u16;
  astruct_428 *iVar13;
  let uVar12: u16;
  let uVar13: u16;
  let unaff_SS: u16;
  let puVar14: u32;
  let uVar15: u32;
  let uStack20: u32;
  
  uVar12 = (param_1 >> 0x10);
  iVar13 = (astruct_428 *)param_1;
  puVar1 = *(u32 **)&iVar13->field_0x210;
  uVar6 = (&iVar13->field_0x210 + 0x2);
  if ((uVar6 | puVar1) != 0x0) {
    ppcVar2 = (code **)*puVar1;
    (**ppcVar2)();
  }
  puVar14 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1e);
  puVar9 = (puVar14 >> 0x10);
  uVar8 = puVar14 & 0xffff;
  pass1_1038_4d6e(param_1,puVar14,uVar8,puVar9);
  uVar5 = uVar8 & 0xffff;
  puVar4 = (uVar5 | ZEXT24(puVar9) << 0x10);
  ppcVar2 = (code **)(*puVar4 + 0x10);
  (**ppcVar2)(0x1008,uVar8,puVar9);
  uVar6 = uVar8;
  if ((extraout_DX == 0x0) && ((false || (uVar6 < 0x5)))) {
    uVar6 = 0x5;
  }
  uVar6 += 0x1;
  uVar13 = 0x1000;
  puVar10 = extraout_DX;
  uVar7 = uVar6;
  mem_op_1000_179c(0x1c,extraout_DX,0x1000);
  uVar11 = puVar10 | uVar7;
  if (uVar11 == 0x0) {
    iVar13->field_0x210 = 0x0;
  }
  else {
    uVar11 = uVar6 >> 0xf;
    cVar3 = (uVar6 >> 0x8);
    uVar13 = 0x1030;
    struct_1030_11aa((u16 *)CONCAT22(puVar10,uVar7),0x5,
                     CONCAT13(cVar3 >> 0xf,CONCAT12(cVar3 >> 0x7,uVar6)),unaff_SS);
    &iVar13->field_0x210 = uVar6;
    (&iVar13->field_0x210 + 0x2) = uVar11;
  }
  uVar15 = iVar13->field_0x210;
  (uVar15 + 0x1a) = 0x0;
  for (uStack20 = 0x0; uStack20 < (uVar8 & 0xffff | ZEXT24(extraout_DX) << 0x10);
      uStack20 += 0x1) {
    uVar15 = pass1_1030_1d7c((uVar8 & 0xffff),uVar11,puVar4);
    uVar6 = (uVar15 >> 0x10);
    uVar11 = uVar6 | uVar15;
    if (uVar11 != 0x0) {
      pass1_1030_1358(iVar13->field_0x210,uVar15,uVar6,uStack20 + 0x1,unaff_SS);
    }
    uVar13 = 0x1030;
  }
  if (puVar4 != 0x0) {
    ppcVar2 = (code **)*puVar4;
    (**ppcVar2)(uVar13,uVar5,puVar9,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_3cc0(param_1: u32,param_2: u16,uchar *param_3,param_4: u16,param_5: u16,
               param_6: u16,param_7: u16)

{
  let lVar1: i32;
  code **ppcVar2;
  let uVar3: u16;
  let puVar4: u32;
  let uVar5: u16;
  let extraout_DX: *mut u8
  let extraout_DX_00: *mut u8
  let extraout_DX_01: u16;
  let extraout_DX_02: u16;
  let uVar6: u16;
  let extraout_DX_03: *mut u8
  let puVar7: *mut u8
  let extraout_DX_04: *mut u8
  let puVar8: u32;
  let puVar9: *mut u8
  let uVar10: u16;
  let puVar11: u32;
  let uVar12: u32;
  let uVar13: u32;
  let uVar14: u8;
  let uVar15: u8;
  let uVar16: u8;
  let uVar17: u8;
  let puStack26: u32;
  let uStack22: u32;
  let uStack18: u32;
  let uStack14: u32;
  let puStack10: u32;
  
  if (param_4 == 0x1e) {
    uVar10 = 0x1008;
    puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x27);
    puVar9 = (puVar11 >> 0x10);
    puVar8 = puVar11;
    pass1_1038_4e78(puVar8,puVar9,param_1,puVar11);
    puStack10 = CONCAT22(puVar9,puVar8);
    ppcVar2 = (code **)(*puStack10 + 0x10);
    puVar4 = puVar8;
    (**ppcVar2)(0x1008,puVar8,puVar9);
    uStack14 = CONCAT22(extraout_DX_00,puVar4);
    puVar7 = extraout_DX_00;
    for (uStack18 = 0x0; uStack18 < uStack14; uStack18 += 0x1) {
      uVar12 = pass1_1030_1d7c(puVar4,puVar7,puStack10);
      puVar7 = ((uVar12 >> 0x10) | uVar12);
      if (puVar7 != 0x0) {
        uVar5 = pass1_1030_bfb8(uVar12,param_7);
        puStack26 = CONCAT22(puVar7,uVar5);
        puVar7 = (puVar7 | uVar5);
        if (puVar7 != 0x0) {
          pass1_1028_b58e(uVar12);
          if (CONCAT22(param_3,param_2) <= puStack26) {
            uVar10 = 0x1030;
            pass1_1030_7ddc(CONCAT22(extraout_DX_01,uVar5),
                            CONCAT13((char)(param_3 >> 0x8),
                                     CONCAT12((char)param_3,param_2)),0x1e,param_2,param_3
                            ,param_5,param_6,param_7);
            break;
          }
          puVar7 = param_3;
          pass1_1030_7ddc(CONCAT22(extraout_DX_01,uVar5),(long)puStack26,0x1e,param_2,
                          param_3,param_5,param_6,param_7);
          lVar1 = CONCAT22(param_3,param_2) - (long)puStack26;
          param_2 = lVar1;
          param_3 = (lVar1 >> 0x10);
        }
      }
      uVar10 = 0x1030;
    }
    puStack26 = puStack10;
    if (puStack10 == 0x0) {
      return;
    }
  }
  else {
    if (param_4 != 0x21) {
      uVar10 = 0x1008;
      puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x3);
      puVar7 = (puVar11 >> 0x10);
      uVar3 = puVar11;
      pass1_1038_4e78(uVar3,puVar7,param_1,puVar11);
      puStack26 = CONCAT22(puVar7,uVar3);
      ppcVar2 = (code **)(*puStack26 + 0x10);
      (**ppcVar2)(0x1008,uVar3,puVar7);
      uStack22 = CONCAT22(extraout_DX,uVar3);
      uStack18 = 0x0;
      puVar7 = extraout_DX;
LAB_1038_3e9c:
      if (uStack18 < uStack22) {
        uVar10 = 0x1030;
        uVar12 = pass1_1030_1d7c(uVar3,puVar7,puStack26);
        puVar7 = ((uVar12 >> 0x10) | uVar12);
        if (puVar7 == 0x0) goto LAB_1038_3e98;
        uVar10 = SUB42(&USHORT_1050_1028,0x0);
        uVar13 = pass1_1028_45e2(uVar12,uVar12,puVar7,param_7);
        uVar6 = uVar13;
        puVar7 = ((uVar13 >> 0x10) | uVar6);
        if (puVar7 == 0x0) goto LAB_1038_3e98;
        pass1_1028_b58e(uVar12);
        uVar12 = CONCAT22(param_3,param_2);
        if (uVar13 < uVar12) {
          uVar10 = 0x1030;
          puVar7 = param_3;
          pass1_1030_7ddc(CONCAT22(extraout_DX_04,uVar6),uVar13,param_4,param_2,param_3,
                          param_5,param_6,param_7);
          lVar1 = CONCAT22(param_3,param_2) - uVar13;
          param_2 = lVar1;
          param_3 = (lVar1 >> 0x10);
          goto LAB_1038_3e98;
        }
        uVar16 = SUB21(param_3,0x0);
        uVar17 = (u8)(param_3 >> 0x8);
        uVar14 = (u8)uVar6;
        uVar15 = (u8)(uVar6 >> 0x8);
        puVar7 = extraout_DX_04;
LAB_1038_3e67:
        uVar10 = 0x1030;
        pass1_1030_7ddc(CONCAT22(puVar7,CONCAT11(uVar15,uVar14)),
                        CONCAT13(uVar17,CONCAT12(uVar16,param_2)),param_4,uVar12,
                        param_3,param_5,param_6,param_7);
      }
      goto LAB_1038_3e6c;
    }
    uVar10 = 0x1008;
    puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0xa);
    puVar7 = (puVar11 >> 0x10);
    uVar3 = puVar11;
    pass1_1038_4e78(uVar3,puVar7,param_1,puVar11);
    puStack26 = CONCAT22(puVar7,uVar3);
    ppcVar2 = (code **)(*puStack26 + 0x10);
    (**ppcVar2)(0x1008,uVar3,puVar7);
    uStack22 = CONCAT22(extraout_DX_02,uVar3);
    uVar6 = extraout_DX_02;
    for (uStack18 = 0x0; uStack18 < uStack22; uStack18 += 0x1) {
      uVar10 = 0x1030;
      uVar13 = pass1_1030_1d7c(uVar3,uVar6,puStack26);
      uVar12 = uVar13 & 0xffff;
      uVar6 = (uVar13 >> 0x10) | uVar12;
      if (uVar6 != 0x0) {
        uVar16 = SUB21(param_3,0x0);
        uVar17 = (u8)(param_3 >> 0x8);
        pass1_1028_b58e(uVar13);
        uVar14 = (u8)uVar12;
        uVar15 = (u8)(uVar12 >> 0x8);
        param_3 = extraout_DX_03;
        puVar7 = extraout_DX_03;
        goto LAB_1038_3e67;
      }
    }
LAB_1038_3e6c:
    if (puStack26 == 0x0) {
      return;
    }
    puVar9 = (puStack26 >> 0x10);
    puVar8 = puStack26;
  }
  ppcVar2 = (code **)*puVar8;
  (**ppcVar2)(uVar10,puStack26,puVar9,0x1);
  return;
LAB_1038_3e98:
  uStack18 += 0x1;
  goto LAB_1038_3e9c;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_3efc(param_1: u16,param_2: u16,param_3: u32,param_4: u32,param_5: i16,
               param_6: u16)

{
  code **ppcVar1;
  let puStack6: u32;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4,(param_4 >> 0x10));
  puStack6 = CONCAT22(param_6,param_5);
  (param_5 + 0x1c) = (param_3 + 0x4);
  ppcVar1 = (code **)(*puStack6 + 0x58);
  (**ppcVar1)(&USHORT_1050_1028,param_5,param_6,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_3f38(param_1: *mut u32,param_2: *mut u32,param_3: u32,param_4: i16,param_5: u16)
{
  code **ppcVar1;
  let iVar2: i16;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let puVar3: u32;
  let uVar4: u16;
  let uVar5: u32;
  let uVar6: u16;
  let uStack10: u32;
  let puStack6: u32;
  
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_3,(param_3 >> 0x10));
  puStack6 = CONCAT22(param_5,param_4);
  iVar2 = param_4;
  pass1_1028_b58e(CONCAT22(param_5,param_4));
  uStack10 = CONCAT22(extraout_DX,iVar2);
  uVar5 = (iVar2 + 0x4);
  ppcVar1 = (code **)(*param_1 + 0x18);
  (**ppcVar1)(&USHORT_1050_1028,param_1,uVar5);
  uVar6 = 0x0;
  uVar4 = 0x0;
  ppcVar1 = (code **)(*param_2 + 0x8);
  puVar3 = param_2;
  (**ppcVar1)();
  pass1_1030_73ee(uStack10,(param_2 + 0x4),extraout_DX_00);
  ppcVar1 = (code **)(*puStack6 + 0x58);
  (**ppcVar1)(0x1030,param_4,param_5,param_2,puVar3,uVar4,uVar5,uVar6);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_3fb0(param_1: u32)
{
  let uVar1: u32;
  
  uVar1 = (param_1 + 0x200);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_3fca(param_1: u32,param_2: u16,param_3: u16)
{
  let uVar1: u32;
  code **ppcVar2;
  let uVar3: u16;
  let extraout_DX: u16;
  let uVar4: u16;
  let extraout_DX_00: u16;
  let uVar5: u16;
  let iVar6: i16;
  let unaff_DI: i16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u32;
  let puVar11: *mut u16;
  let uVar12: u8;
  let uVar13: u8;
  let uVar14: u8;
  let uVar15: u8;
  let uVar16: u16;
  let iStack38: i16;
  let local_24: i16;
  let local_22: [u8;2];
  let piStack32: *mut i16;
  let uStack30: u16;
  let puStack28: *mut u8;
  let uStack26: u16;
  let uStack24: u16;
  let uStack22: u32;
  let uStack18: u16;
  let uStack16: u16;
  astruct_18 *paStack14;
  astruct_18 *paStack10;
  let uStack6: u32;
  
  uVar7 = (param_1 >> 0x10);
  uVar5 = param_1;
  if (*(long *)(uVar5 + 0xc) == 0x0) {
    param_2 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar2 = (code **)((uVar5 + 0xc) + 0x10);
    (**ppcVar2)();
    uVar4 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar4,param_2);
  PTR_LOOP_1050_5f2e = (uVar4 | param_2);
  if (PTR_LOOP_1050_5f2e != 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
    }
    uVar3 = fn_ptr_op_1000_1708(uStack6 << 0x2,0x0,0x1,PTR_LOOP_1050_5f2c,
                                PTR_LOOP_1050_5f2e,0x1000);
    paStack10 = (astruct_18 *)CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(PTR_LOOP_1050_5f2e,0x1000);
    }
    else {
    }
    uVar9 = 0x1000;
    uVar3 = fn_ptr_op_1000_1708(uStack6 << 0x2,0x0,0x1,PTR_LOOP_1050_5f2c,
                                PTR_LOOP_1050_5f2e,0x1000);
    paStack14 = (astruct_18 *)CONCAT22(PTR_LOOP_1050_5f2e,uVar3);
    for (uStack22 = 0x0; uStack22 < uStack6; uStack22 += 0x1) {
      uVar1 = (uVar5 + 0xc);
      ppcVar2 = (code **)((uVar5 + 0xc) + 0x4);
      uVar10 = uStack6;
      (**ppcVar2)(uVar9,uVar1,(uVar1 >> 0x10),uStack22,
                  (uStack22 >> 0x10));
      uVar4 = uVar10;
      PTR_LOOP_1050_5f2e = (extraout_DX_00 | uVar4);
      uStack18 = uVar4;
      uStack16 = extraout_DX_00;
      if (PTR_LOOP_1050_5f2e != 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4,extraout_DX_00);
        uStack22 *= 0x4;
        uVar8 = (paStack10 >> 0x10);
        iVar6 = paStack10;
        (uStack22 + iVar6) = uVar4;
        *(uchar **)(uStack22 + iVar6 + 0x2) = PTR_LOOP_1050_5f2e;
        uVar9 = 0x1030;
        uVar10 = struct_op_1030_73a8(CONCAT22(PTR_LOOP_1050_5f2e,
                                              (uStack22 + iVar6)));
        PTR_LOOP_1050_5f2e = (uVar10 >> 0x10);
        uVar8 = (paStack14 >> 0x10);
        (paStack14 + uStack22) = uVar10;
        *(uchar **)(paStack14 + uStack22 + 0x2) = PTR_LOOP_1050_5f2e;
      }
    }
    for (uStack22 = 0x0; uStack22 < uStack6; uStack22 += 0x1) {
      uVar9 = (paStack14 >> 0x10);
      iVar6 = paStack14;
      if ((*(long *)(uStack22 * 0x4 + iVar6) != 0x0) &&
         (uVar1 = (uStack22 * 0x4 + iVar6),
         (uVar1 + 0x1a) = 0x0,
         uVar1 = (uStack22 * 0x4 + iVar6),
         (uVar1 + 0x12) == 0x5)) {
        pass1_1028_bdac(*(u32 **)(uStack22 * 0x4 + iVar6),0x6,
                        &USHORT_1050_1028);
      }
    }
    (uVar5 + 0x204) = 0x0;
    puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_3,PTR_LOOP_1050_5f2e,unaff_DI)
    ;
    uStack30 = (puVar11 >> 0x10);
    uStack26 = SUB42(puVar11,0x0);
    puStack28 = PTR_LOOP_1050_13ae;
    if (PTR_LOOP_1050_13ae == (&PTR_LOOP_1050_0000 + 0x1)) {
      (uVar5 + 0x204) = 0x1;
    }
    uStack24 = uStack30;
    pass1_1038_5a96(uVar5,uVar7,uStack6,paStack14);
    pass1_1038_5cc6(param_1,uStack6,paStack14,paStack10,0x0,0x2);
    pass1_1038_5b3c(uVar5,uVar7,uStack6,paStack14);
    pass1_1038_5cc6(param_1,uStack6,paStack14,paStack10,0x0,0x1);
    uVar14 = SUB21(local_22,0x0);
    uVar15 = (u8)(local_22 >> 0x8);
    piStack32 = &local_24;
    uVar12 = SUB21(piStack32,0x0);
    uVar13 = (u8)(piStack32 >> 0x8);
    uVar1 = (uVar5 + 0x8);
    uVar3 = param_3;
    uVar16 = param_3;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
    pass1_1030_5b1c(CONCAT22(uStack30,piStack32),
                    CONCAT22(uVar3,CONCAT11(uVar13,uVar12)),
                    CONCAT22(uVar16,CONCAT11(uVar15,uVar14)));
    for (iStack38 = 0x1; iStack38 <= local_24; iStack38 += 0x1) {
      pass1_1038_58e6(uVar5,uVar7,uStack6,paStack14,paStack10,iStack38,
                      param_3);
      pass1_1038_5cc6(param_1,uStack6,paStack14,paStack10,iStack38,0x3);
    }
    pass1_1038_5a16(uVar5,uVar7,uStack6,paStack14);
    for (uStack22 = 0x0; uStack22 < uStack6; uStack22 += 0x1) {
      uVar9 = (paStack14 >> 0x10);
      iVar6 = paStack14;
      if ((*(long *)(uStack22 * 0x4 + iVar6) != 0x0) &&
         (uVar1 = (uStack22 * 0x4 + iVar6),
         (uVar1 + 0x12) != 0x5)) {
        uVar1 = (uStack22 * 0x4 + iVar6);
        ppcVar2 = (code **)(
                                  (uStack22 * 0x4 + iVar6) + 0x28);
        (**ppcVar2)(0x1030,uVar1,(uVar1 >> 0x10));
      }
    }
    fn_ptr_1000_17ce(paStack10,0x1000);
    fn_ptr_1000_17ce(paStack14,0x1000);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_42cc(param_1: u32,param_2: u16)
{
  code **ppcVar1;
  let uVar2: u32;
  let bVar3: bool;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let puVar7: *mut u8
  let extraout_DX: u16;
  let uVar8: u16;
  let extraout_DX_00: u16;
  let iVar9: i16;
  let uVar10: u16;
  let uVar11: u16;
  let puVar12: u32;
  let puVar13: u32;
  let uStack24: u32;
  let uStack18: u32;
  let puStack10: u32;
  
  uVar10 = (param_1 >> 0x10);
  iVar9 = param_1;
  if (*(long *)(iVar9 + 0x1f6) == 0x0) {
    return;
  }
  uVar11 = 0x1008;
  puVar12 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x2d);
  puVar7 = (puVar12 >> 0x10);
  uVar4 = puVar12;
  pass1_1038_4d6e(param_1,puVar12,uVar4,puVar7);
  puStack10 = CONCAT22(puVar7,uVar4);
  ppcVar1 = (code **)(*puStack10 + 0x10);
  uVar5 = uVar4;
  (**ppcVar1)(0x1008,uVar4,puVar7);
  uStack18 = CONCAT22(extraout_DX,uVar5);
  bVar3 = false;
  uVar8 = extraout_DX;
  for (uStack24 = 0x0; uStack24 < uStack18; uStack24 += 0x1) {
    uVar11 = 0x1030;
    puVar13 = pass1_1030_1d7c(uVar5,uVar8,puStack10);
    uVar6 = puVar13;
    uVar8 = (puVar13 >> 0x10) | uVar6;
    if (uVar8 != 0x0) {
      ppcVar1 = (code **)(*puVar13 + 0x50);
      (**ppcVar1)();
      uVar8 = extraout_DX_00;
      if (uVar6 != 0x0) {
        bVar3 = true;
      }
    }
  }
  if (bVar3) {
    uVar2 = (iVar9 + 0x1f6);
    (uVar2 + 0x1aa) = 0x0;
  }
  else {
    uVar11 = 0x1030;
    pass1_1030_38b8();
    uVar8 |= uStack18;
    if (uVar8 != 0x0) {
      uVar11 = 0x1030;
      pass1_1030_326a((iVar9 + 0x1f6),uStack18,uVar8,param_2);
    }
  }
  if (puStack10 != 0x0) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(uVar11,uVar4,puVar7,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_43cc(param_1: i16,param_2: u16,param_3: u16,param_4: u16,param_5: i16,
               param_6: i16)

{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u32;
  let puVar6: *mut u8
  let extraout_DX: u16;
  let uVar7: u16;
  let iVar8: i16;
  let iVar9: i16;
  let uVar10: u16;
  let puVar11: u32;
  let uVar12: u32;
  let uStack22: u32;
  let uStack18: u32;
  let puStack14: u32;
  
  if (param_4 == 0x5) {
    pass1_1038_4900(CONCAT22(param_2,param_1));
    return;
  }
  pass1_1038_53ba(CONCAT22(param_2,param_1),param_4);
  if ((param_6 != 0x0) || (param_5 != 0x0)) {
    iVar8 = param_4 * 0x4;
    uVar2 = (param_1 + iVar8 + 0x14e);
    iVar9 = ((param_1 + iVar8 + 0x150) - (param_3 >> 0xf)) -
            (uVar2 < param_3);
    (param_1 + iVar8 + 0x14e) = uVar2 - param_3;
    (param_1 + iVar8 + 0x150) = iVar9;
    if (iVar9 < 0x0) {
      (param_1 + iVar8 + 0x14e) = 0x0;
    }
    uVar10 = 0x1008;
    puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1e);
    puVar6 = (puVar11 >> 0x10);
    uVar2 = puVar11;
    pass1_1038_4e78(uVar2,puVar6,CONCAT22(param_2,param_1),puVar11);
    puStack14 = CONCAT22(puVar6,uVar2);
    ppcVar1 = (code **)(*puStack14 + 0x10);
    uVar3 = uVar2;
    (**ppcVar1)(0x1008,uVar2,puVar6);
    uStack18 = CONCAT22(extraout_DX,uVar3);
    uVar7 = extraout_DX;
    for (uStack22 = 0x0; uStack22 < uStack18; uStack22 += 0x1) {
      uVar12 = pass1_1030_1d7c(uVar3,uVar7,puStack14);
      uVar7 = (uVar12 >> 0x10);
      uVar5 = uVar12 & 0xffff;
      for (; uVar4 = uVar5, param_3 != 0x0; param_3 -= 0x1) {
        pass1_1030_cf78(uVar12,param_4);
        uVar5 = uVar4;
        if (uVar4 == 0x0) break;
      }
      uVar10 = 0x1030;
      if (param_3 == 0x0) break;
    }
    if (puStack14 != 0x0) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(uVar10,uVar2,puVar6,0x1);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_44d8(param_1: i16,param_2: u16,param_3: u16,param_4: u16,param_5: i16,
               param_6: i16)

{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u32;
  let puVar6: *mut u8
  let extraout_DX: u16;
  let uVar7: u16;
  let uVar8: u16;
  astruct_697 *iVar9;
  let iVar10: i16;
  let uVar11: u16;
  let puVar12: u32;
  let uVar13: u32;
  let uStack22: u32;
  let uStack18: u32;
  let puStack14: u32;
  
  if (param_4 == 0x5) {
    pass1_1038_4900(CONCAT22(param_2,param_1));
    return;
  }
  pass1_1038_53ba(CONCAT22(param_2,param_1),param_4);
  if ((param_6 != 0x0) || (param_5 != 0x0)) {
    iVar9 = (astruct_697 *)(param_4 * 0x4);
    uVar2 = (iVar9 + param_1 + 0x14e);
    iVar10 = ((iVar9 + param_1 + 0x150) - (param_3 >> 0xf)) -
             (uVar2 < param_3);
    (iVar9 + param_1 + 0x14e) = uVar2 - param_3;
    (iVar9 + param_1 + 0x150) = iVar10;
    if (iVar10 < 0x0) {
      (iVar9 + param_1 + 0x14e) = 0x0;
    }
    uVar11 = 0x1008;
    puVar12 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1e);
    puVar6 = (puVar12 >> 0x10);
    uVar2 = puVar12;
    pass1_1038_4e78(uVar2,puVar6,CONCAT22(param_2,param_1),puVar12);
    puStack14 = CONCAT22(puVar6,uVar2);
    ppcVar1 = (code **)(*puStack14 + 0x10);
    uVar3 = uVar2;
    (**ppcVar1)(0x1008,uVar2,puVar6);
    uStack18 = CONCAT22(extraout_DX,uVar3);
    uVar7 = extraout_DX;
    for (uStack22 = 0x0; uStack22 < uStack18; uStack22 += 0x1) {
      uVar13 = pass1_1030_1d7c(uVar3,uVar7,puStack14);
      uVar8 = (uVar13 >> 0x10);
      uVar5 = uVar13 & 0xffff;
      uVar7 = uVar8;
      for (; uVar4 = uVar5, param_3 != 0x0; param_3 -= 0x1) {
        pass1_1030_d00c(uVar13,uVar8,param_4);
        uVar5 = uVar4;
        if (uVar4 == 0x0) break;
      }
      uVar11 = 0x1030;
      if (param_3 == 0x0) break;
    }
    if (puStack14 != 0x0) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(uVar11,uVar2,puVar6,0x1);
      return;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_45e4(param_1: u32,param_2: u16,param_3: i16,param_4: u16)
{
  let piVar1: *mut i16;
  code **ppcVar2;
  let uVar3: u32;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let iVar8: i16;
  let iVar9: i16;
  let iVar10: i16;
  let puVar11: *mut u8
  let iVar12: i16;
  let uVar13: u16;
  let uVar14: u16;
  let bVar15: bool;
  let puVar16: u32;
  let uStack28: u16;
  let puStack22: u32;
  
  uVar14 = (param_1 >> 0x10);
  iVar12 = param_1;
  pass1_1030_38f2((iVar12 + 0x1f6),0x2,param_4);
  iVar8 = param_3;
  uVar4 = param_2;
  pass1_1030_38f2((iVar12 + 0x1f6),0x1,param_4);
  bVar15 = param_2 < uVar4;
  uVar13 = param_2 - uVar4;
  iVar10 = param_3 - iVar8;
  pass1_1030_38f2((iVar12 + 0x1f6),0x4,param_4);
  iVar9 = iVar8;
  uVar5 = uVar4;
  pass1_1030_38f2((iVar12 + 0x1f6),0x3,param_4);
  uVar7 = (iVar12 + 0x24);
  uVar6 = uVar7 + (uVar4 - uVar5);
  iVar10 = (uVar7 >> 0xf) + ((iVar8 - iVar9) - (uVar4 < uVar5)) +
           CARRY2(uVar7,uVar4 - uVar5) + (iVar10 - bVar15) +
           CARRY2(uVar6,uVar13);
  if ((iVar10 < 0x0) || ((iVar10 < 0x1 && (uVar6 + uVar13 == 0x0)))) {
    iVar10 = -0x1;
  }
  else {
    iVar10 = 0x1;
  }
  piVar1 = (i16 *)(iVar12 + 0x24);
  *piVar1 = *piVar1 + iVar10;
  puVar16 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x16);
  puVar11 = (puVar16 >> 0x10);
  uVar7 = puVar16;
  pass1_1038_4d6e(param_1,puVar16,uVar7,puVar11);
  puStack22 = CONCAT22(puVar11,uVar7);
  uVar3 = *puStack22;
  ppcVar2 = (code **)uVar3 + 0x8;
  uVar5 = uVar7;
  (**ppcVar2)(0x1008,uVar7,puVar11);
  if (puStack22 != 0x0) {
    ppcVar2 = (code **)uVar3;
    (**ppcVar2)(0x1008,uVar7,puVar11,0x1);
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
  else {
    if (iVar10 < 0x15) {
      uStack28 = 0x13;
    }
    else {
      if (iVar10 < 0x1f) {
        uStack28 = 0x12;
      }
      else {
        if (iVar10 < 0x29) {
          uStack28 = 0x11;
        }
        else {
          if (iVar10 < 0x33) {
            uStack28 = 0x10;
          }
          else {
            if (iVar10 < 0x3d) {
              uStack28 = 0xf;
            }
            else {
              if (iVar10 < 0x47) {
                uStack28 = 0xe;
              }
              else {
                if (iVar10 < 0x51) {
                  uStack28 = 0xd;
                }
                else {
                  if (iVar10 < 0x5b) {
                    uStack28 = 0xc;
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  pass1_1030_3258((iVar12 + 0x1f6),uStack28);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_4760(param_1: u32)
{
  let puVar1: *mut u16;
  code **ppcVar2;
  let uVar3: u16;
  let uVar4: u16;
  let iVar5: i16;
  let uVar6: u16;
  let puVar7: *mut u8
  let puVar8: *mut u8
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let extraout_DX_01: u16;
  let extraout_DX_02: u16;
  let uVar9: u16;
  let extraout_DX_03: u16;
  let extraout_DX_04: u16;
  astruct_700 *iVar10;
  let uVar10: u16;
  let uVar11: u16;
  let unaff_SS: u16;
  let puVar12: u32;
  let uVar13: u32;
  let uVar14: u8;
  let puVar15: *mut u8
  let uStack26: u32;
  let uStack22: u32;
  let puStack14: u32;
  let puStack10: u32;
  
  uVar10 = (param_1 >> 0x10);
  iVar10 = (astruct_700 *)param_1;
  puVar1 = &iVar10->field_0x22;
  *puVar1 = *puVar1 + iVar10->field_0x20c;
  puVar12 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x26);
  puVar7 = (puVar12 >> 0x10);
  uVar6 = puVar12;
  pass1_1038_4d6e(param_1,puVar12,uVar6,puVar7);
  puStack10 = CONCAT22(puVar7,uVar6);
  uVar11 = 0x1008;
  puVar12 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1a);
  puVar8 = (puVar12 >> 0x10);
  uVar3 = puVar12;
  pass1_1038_4d6e(param_1,puVar12,uVar3,puVar8);
  puStack14 = CONCAT22(puVar8,uVar3);
  ppcVar2 = (code **)(*puStack14 + 0x10);
  uVar4 = uVar3;
  (**ppcVar2)(0x1008,uVar3,puVar8);
  uVar14 = (u8)uVar6;
  puVar15 = puVar7;
  if ((extraout_DX | uVar4) == 0x0) {
    ppcVar2 = (code **)(*puStack10 + 0x10);
    (**ppcVar2)();
    puVar1 = &iVar10->field_0x22;
    *puVar1 = *puVar1 + uVar4;
    uVar9 = extraout_DX_00;
  }
  else {
    ppcVar2 = (code **)(*puStack10 + 0x10);
    (**ppcVar2)();
    uStack22 = CONCAT22(extraout_DX_03,uVar4);
    uVar9 = extraout_DX_03;
    for (uStack26 = 0x0; uStack26 < uStack22; uStack26 += 0x1) {
      uVar13 = pass1_1030_1d7c(uVar4,uVar9,puStack10);
      iVar5 = uVar13;
      uVar11 = SUB42(&USHORT_1050_1028,0x0);
      func_0x10285a94();
      if (iVar5 == 0x2) {
        if ((*_PTR_LOOP_1050_65e2 & 0x1) == 0x0) goto LAB_1038_485e;
      }
      else {
        if (iVar5 != 0x3) {
LAB_1038_485e:
          puVar1 = &iVar10->field_0x22;
          *puVar1 = *puVar1 + 0x1;
        }
      }
      uVar9 = extraout_DX_04;
    }
  }
  if (puStack10 != 0x0) {
    ppcVar2 = (code **)*puStack10;
    (**ppcVar2)(uVar11,uVar6,puVar7,0x1,uVar14,puVar15);
    uVar9 = extraout_DX_01;
  }
  if (puStack14 != 0x0) {
    ppcVar2 = (code **)*puStack14;
    (**ppcVar2)(uVar11,uVar3,puVar8,0x1);
    uVar9 = extraout_DX_02;
  }
  pass1_1038_45e4(param_1,puStack14,uVar9,unaff_SS);
  if (0x32 < iVar10->field_0x24) {
    puVar1 = &iVar10->field_0x22;
    *puVar1 = *puVar1 - 0x1;
  }
  if (iVar10->field_0x24 < 0x32) {
    puVar1 = &iVar10->field_0x22;
    *puVar1 = *puVar1 + 0x1;
  }
  if (iVar10->field_0x18 < 0xfa) {
    puVar1 = &iVar10->field_0x22;
    *puVar1 = *puVar1 + 0x2;
  }
  else {
    if (iVar10->field_0x18 < 0x1c2) {
      puVar1 = &iVar10->field_0x22;
      *puVar1 = *puVar1 + 0x1;
    }
    else {
      if (0x225 < iVar10->field_0x18) {
        if (iVar10->field_0x18 < 0x2ee) {
          puVar1 = &iVar10->field_0x22;
          *puVar1 = *puVar1 - 0x1;
        }
        else {
          puVar1 = &iVar10->field_0x22;
          *puVar1 = *puVar1 - 0x2;
        }
      }
    }
  }
  uVar6 = iVar10->field_0x22;
  if (0x64 < uVar6) {
    uVar6 = 0x64;
  }
  iVar10->field_0x22 = uVar6;
  if (uVar6 < 0x0) {
    uVar6 = 0x0;
  }
  iVar10->field_0x22 = uVar6;
  return;
}



fn pass1_1038_48e0(param_1: u32,param_2: i16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = (param_1 + 0x20e) + param_2;
  if (0xa < iVar1) {
    iVar1 = 0xa;
  }
  (param_1 + 0x20e) = iVar1;
  return;
}



fn pass1_1038_4900(param_1: u32)
{
  let piVar1: *mut i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  piVar1 = (i16 *)(param_1 + 0x20e);
  *piVar1 = *piVar1 + -0x1;
  if (*piVar1 < 0x0) {
    (param_1 + 0x20e) = 0x0;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_4918(param_1: u32,param_2: i16,param_3: u16,param_4: u16,param_5: u8)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let iVar3: i16;
  let puVar4: u32;
  let uVar5: u16;
  let uVar6: u16;
  let iVar7: i16;
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u32;
  let bStack347: u8;
  let local_14a: [u8;4];
  let puStack326: u32;
  let local_144: [u8;124];
  let local_20: u32;
  let uStack28: u16;
  let uStack26: u32;
  let uStack18: u32;
  let uStack14: u32;
  let uStack10: u32;
  let uStack6: u32;
  
  uVar9 = (param_1 >> 0x10);
  iVar7 = param_1;
  if (*(long *)(iVar7 + 0x4) != 0x4000001) {
    return;
  }
  uVar2 = (iVar7 + 0x8);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,(uVar2 >> 0x10));
  uStack6 = CONCAT22(param_3,param_2);
  uStack10 = (param_2 + 0x10);
  uVar10 = (uStack10 >> 0x10);
  iVar8 = uStack10;
  if ((iVar8 + 0x1c) == 0x0) {
    return;
  }
  uStack14 = 0x0;
  switch((iVar7 + 0x20e)) {
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
  uStack14 = uStack14;
switchD_1038_49cf_caseD_a:
  uStack18 = *_PTR_LOOP_1050_65e2;
  if ((uStack14 != 0x0) &&
     (((uStack18 & 0xffff | (_PTR_LOOP_1050_65e2 + 0x2) << 0x10)
           % uStack14) == 0x0)) {
    piVar1 = (i16 *)(iVar8 + 0x1c);
    *piVar1 = *piVar1 + -0x1;
    piVar1 = (i16 *)(iVar8 + 0x1a);
    *piVar1 = *piVar1 + 0x1;
    iVar3 = (iVar8 + 0x1a) * 0x6 + (iVar8 + 0x16);
    uVar10 = (iVar8 + 0x18);
    local_20 = (iVar3 + -0x6);
    uStack28 = (iVar3 + -0x2);
    puStack326 = &local_20;
    puVar4 = &local_20;
    pass1_1030_64ce(param_4,puVar4,uVar10,_PTR_LOOP_1050_5740,
                    CONCAT22(param_4,puVar4),*(long *)(iVar7 + 0x8),
                    CONCAT22(param_4,local_14a));
    uStack26 = *puVar4;
    uVar6 = (puVar4 + 0x2);
    bStack347 = (byte)(uStack26 >> 0x18);
    uVar5 = bStack347;
    if (bStack347 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uStack26,uVar6);
      uVar11 = struct_op_1030_73a8(CONCAT22(uVar6,uVar5));
      uVar6 = (uVar11 >> 0x10);
      if ((uVar6 | uVar11) != 0x0) {
        iVar8 = (uVar11 + 0xc);
        if (iVar8 < 0x1) {
          return;
        }
        if (SBORROW2(iVar8,0x1)) {
          return;
        }
        if (0x8 < iVar8 + -0x1) {
          return;
        }
      }
    }
    struct_op_1028_87f0(param_4,param_5,(astruct_97 *)CONCAT22(param_4,local_144),0x0,0x0,
                        0x10,&local_20,param_4,(iVar7 + 0x4),
                        (iVar7 + 0x8));
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_4,local_144));
  }
  return;
}



fn pass1_1038_4b20(param_1: u32,param_2: u32,param_3: u32,param_4: u16)
{
  let uVar1: u32;
  
  uVar1 = (param_1 + 0xc);
  pass1_1020_c4f4(uVar1,param_2,(param_2 >> 0x10),param_3,
                  (astruct_361 *)uVar1,param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_4b40(param_1: u32,param_2: u16,param_3: u16)
{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u32;
  let extraout_DX: u16;
  let uVar4: u16;
  let extraout_DX_00: u16;
  let uVar5: u16;
  let iVar6: i16;
  let uVar7: u16;
  let uStack14: u32;
  let uStack10: u32;
  
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  if (*(long *)(iVar6 + 0xc) == 0x0) {
    param_2 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar1 = (code **)((iVar6 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar4 = extraout_DX;
  }
  uStack10 = CONCAT22(uVar4,param_2);
  for (uStack14 = 0x0; uStack14 < uStack10; uStack14 += 0x1) {
    ppcVar1 = (code **)((iVar6 + 0xc) + 0x4);
    uVar3 = uStack10;
    (**ppcVar1)(param_3,(iVar6 + 0xc));
    uVar2 = uVar3;
    uVar5 = extraout_DX_00 | uVar2;
    if (uVar5 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,extraout_DX_00);
      param_3 = 0x1030;
      struct_op_1030_73a8(CONCAT22(uVar5,uVar2));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_4c1a(param_1: u32,param_2: u16,param_3: u32,param_4: u16)
{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u32;
  let uVar5: u16;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u32;
  let uStack14: u32;
  let uStack10: u32;
  
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  uVar8 = (iVar6 + 0xc);
  ppcVar1 = (code **)((iVar6 + 0xc) + 0x10);
  (**ppcVar1)();
  uStack10 = CONCAT22(param_3,param_2);
  for (uStack14 = 0x0; uVar5 = param_3, uStack14 < uStack10; uStack14 += 0x1) {
    ppcVar1 = (code **)((iVar6 + 0xc) + 0x4);
    uVar4 = uStack10;
    (**ppcVar1)(param_4,(iVar6 + 0xc),uStack14,uVar8);
    uVar2 = uVar4;
    param_3 = (uVar5 | uVar2);
    if ((uVar5 | uVar2) != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,uVar5);
      uVar3 = pass1_1030_6fa0(CONCAT22(param_3,uVar2));
      param_4 = 0x1008;
      pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar3,0xe);
    }
  }
  return;
}



fn pass1_1038_4cba(void)
{
  pass1_1030_38b8();
  return;
}



fn pass1_1038_4cd0(param_1: u32,param_2: u32,param_3: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x1c) = param_3;
  (param_1 + 0x1e) = param_2;
  return;
}



fn pass1_1038_4cea(param_1: u32,param_2: *mut u32,param_3: *mut u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_3 = (param_1 + 0x1c);
  *param_2 = (param_1 + 0x1e);
  return;
}



fn pass1_1038_4d0e(param_1: u32,param_2: u16)
{
  astruct_686 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_686 *)param_1;
  iVar1->field_0x1a = iVar1->field_0x18;
  iVar1->field_0x18 = param_2;
  return;
}



fn pass1_1038_4d28(param_1: u32) -> *mut u8

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x1fc),
                          (param_1 + 0x1fa));
}



fn pass1_1038_4d3c(param_1: u32,char *param_2,param_3: u16)
{
  let uVar1: u16;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar2 + 0x1fa),0x1000);
  uVar1 = str_op_1008_60e8(param_2,param_3);
  (iVar2 + 0x1fa) = uVar1;
  (iVar2 + 0x1fc) = param_3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_4d6e(param_1: u32,param_2: *mut u32,param_3: u16,uchar *param_4)
{
  let piVar1: *mut i16;
  code **ppcVar2;
  let uVar3: u16;
  let uVar4: u16;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar5: u16;
  let extraout_DX_01: u16;
  let uVar6: u16;
  let iVar7: i16;
  let uVar8: u16;
  let uVar9: u32;
  let iStack30: i16;
  let uStack26: u32;
  let uStack14: u32;
  let uStack10: u32;
  let puStack6: u32;
  
  mem_op_1000_179c(0x18,param_4,0x1000);
  if ((param_4 | param_3) == 0x0) {
    param_3 = 0x0;
    uVar8 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_4,param_3),0x5,0x5);
    uVar8 = extraout_DX;
  }
  puStack6 = CONCAT22(uVar8,param_3);
  uVar8 = (param_1 >> 0x10);
  iVar7 = param_1;
  if (*(long *)(iVar7 + 0xc) == 0x0) {
    param_3 = 0x0;
    uVar5 = 0x0;
  }
  else {
    ppcVar2 = (code **)((iVar7 + 0xc) + 0x10);
    (**ppcVar2)();
    uVar5 = extraout_DX_00;
  }
  uStack10 = CONCAT22(uVar5,param_3);
  uStack14 = 0x0;
  do {
    if (uStack10 <= uStack14) {
      return;
    }
    ppcVar2 = (code **)((iVar7 + 0xc) + 0x4);
    uVar9 = uStack10;
    (**ppcVar2)();
    uVar3 = uVar9;
    uVar6 = extraout_DX_01 | uVar3;
    if (uVar6 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,extraout_DX_01);
      uStack26 = CONCAT22(uVar6,uVar3);
      uVar4 = pass1_1030_6fa0(CONCAT22(uVar6,uVar3));
      iStack30 = 0x0;
      while( true ) {
        piVar1 = (i16 *)(param_2 + 0x4);
        if (*piVar1 == iStack30 || *piVar1 < iStack30) break;
        if ((*param_2 + iStack30 * 0x2) == uVar4) {
          uVar9 = struct_op_1030_73a8(uStack26);
          if ((uVar9 + 0x12) == 0x5) {
            ppcVar2 = (code **)(*puStack6 + 0xc);
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



fn pass1_1038_4e78(param_1: u16,uchar *param_2,param_3: u32,param_4: *mut u32)
{
  let piVar1: *mut i16;
  code **ppcVar2;
  let uVar3: u16;
  let uVar4: u32;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar5: u16;
  let extraout_DX_01: u16;
  let uVar6: u16;
  let iVar7: i16;
  let uVar8: u16;
  let iStack26: i16;
  let uStack14: u32;
  let uStack10: u32;
  let puStack6: u32;
  
  mem_op_1000_179c(0x18,param_2,0x1000);
  if ((param_2 | param_1) == 0x0) {
    param_1 = 0x0;
    uVar8 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_2,param_1),0x5,0x5);
    uVar8 = extraout_DX;
  }
  puStack6 = CONCAT22(uVar8,param_1);
  uVar8 = (param_3 >> 0x10);
  iVar7 = param_3;
  if (*(long *)(iVar7 + 0xc) == 0x0) {
    param_1 = 0x0;
    uVar5 = 0x0;
  }
  else {
    ppcVar2 = (code **)((iVar7 + 0xc) + 0x10);
    (**ppcVar2)();
    uVar5 = extraout_DX_00;
  }
  uStack10 = CONCAT22(uVar5,param_1);
  uStack14 = 0x0;
  do {
    if (uStack10 <= uStack14) {
      return;
    }
    uVar4 = uStack10;
    pass1_1030_1d58((iVar7 + 0xc));
    uVar6 = uVar5 | uVar4;
    if (uVar6 != 0x0) {
      uVar3 = pass1_1030_6fa0(uVar4 & 0xffff | uVar5 << 0x10);
      iStack26 = 0x0;
      while( true ) {
        piVar1 = (i16 *)(param_4 + 0x4);
        if (*piVar1 == iStack26 || *piVar1 < iStack26) break;
        if ((*param_4 + iStack26 * 0x2) == uVar3) {
          ppcVar2 = (code **)(*puStack6 + 0xc);
          (**ppcVar2)();
          uVar6 = extraout_DX_01;
          break;
        }
        iStack26 += 0x1;
      }
    }
    uStack14 += 0x1;
    uVar5 = uVar6;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_4f54(param_1: u32,param_2: u16,param_3: u16)
{
  code **ppcVar1;
  let uVar2: u16;
  let BVar3: bool;
  let uVar4: u32;
  let extraout_DX: u16;
  let uVar5: u16;
  let uVar6: u16;
  let iVar7: i16;
  let uVar8: u16;
  let uStack10: u32;
  let uStack6: u32;
  
  uVar8 = (param_1 >> 0x10);
  iVar7 = param_1;
  if (*(long *)(iVar7 + 0xc) == 0x0) {
    param_3 = 0x0;
    uVar5 = 0x0;
  }
  else {
    ppcVar1 = (code **)((iVar7 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar5 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar5,param_3);
  uStack10 = 0x0;
  do {
    if (uStack6 <= uStack10) {
      return;
    }
    uVar4 = uStack6;
    pass1_1030_1d58((iVar7 + 0xc));
    uVar6 = uVar5 | uVar4;
    if (uVar6 != 0x0) {
      uVar2 = pass1_1030_6fa0(uVar4 & 0xffff | uVar5 << 0x10);
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,param_2);
      if (BVar3 != 0x0) {
        return;
      }
    }
    uStack10 += 0x1;
    uVar5 = uVar6;
  } while( true );
}



fn pass1_1038_4fd8(param_1: u16,param_2: u32,param_3: u16)
{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u32;
  let extraout_DX: u16;
  let uVar4: u16;
  let uVar5: u16;
  let iVar6: i16;
  let uVar7: u16;
  let uStack10: u32;
  let uStack6: u32;
  
  uVar7 = (param_2 >> 0x10);
  iVar6 = param_2;
  if (*(long *)(iVar6 + 0xc) == 0x0) {
    param_1 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar1 = (code **)((iVar6 + 0xc) + 0x10);
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
    pass1_1030_1d58((iVar6 + 0xc));
    uVar5 = uVar4 | uVar3;
    if (uVar5 != 0x0) {
      uVar2 = pass1_1030_6fa0(uVar3 & 0xffff | uVar4 << 0x10);
      if (uVar2 == param_3) {
        return;
      }
    }
    uStack10 += 0x1;
    uVar4 = uVar5;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_5050(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u32;
  let extraout_DX: u16;
  let uVar4: u16;
  let uVar5: u16;
  let iVar6: i16;
  let uVar7: u16;
  let uStack14: u32;
  let uStack10: u32;
  
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  if (*(long *)(iVar6 + 0xc) == 0x0) {
    param_3 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar1 = (code **)((iVar6 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar4 = extraout_DX;
  }
  uStack10 = CONCAT22(uVar4,param_3);
  for (uStack14 = 0x0; uStack14 < uStack10; uStack14 += 0x1) {
    uVar3 = uStack10;
    pass1_1030_1d58((iVar6 + 0xc));
    uVar5 = uVar4 | uVar3;
    if (uVar5 != 0x0) {
      uVar2 = pass1_1030_6fa0(uVar3 & 0xffff | uVar4 << 0x10);
      pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,param_2);
    }
    uVar4 = uVar5;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_50e0(param_1: u32,param_2: u16,param_3: u16)
{
  code **ppcVar1;
  let uVar2: u16;
  let BVar3: bool;
  let extraout_DX: u16;
  let uVar4: u16;
  let uVar5: u16;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u32;
  let uStack14: u32;
  let uStack10: u32;
  
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  if (*(long *)(iVar6 + 0xc) == 0x0) {
    param_3 = 0x0;
    uVar4 = 0x0;
  }
  else {
    ppcVar1 = (code **)((iVar6 + 0xc) + 0x10);
    (**ppcVar1)();
    uVar4 = extraout_DX;
  }
  uStack10 = CONCAT22(uVar4,param_3);
  for (uStack14 = 0x0; uStack14 < uStack10; uStack14 += 0x1) {
    uVar8 = uStack10;
    pass1_1030_1d58((iVar6 + 0xc));
    uVar5 = uVar4 | uVar8;
    if (uVar5 != 0x0) {
      uVar2 = pass1_1030_6fa0(uVar8 & 0xffff | uVar4 << 0x10);
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,param_2);
      if (BVar3 != 0x0) {
        uVar8 = struct_op_1030_73a8(uVar8 & 0xffff | uVar4 << 0x10);
        uVar5 = (uVar8 >> 0x10);
      }
    }
    uVar4 = uVar5;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_518c(param_1: u32,param_2: u16,param_3: u16)
{
  let puVar1: *mut u16;
  let uVar2: u32;
  code **ppcVar3;
  let uVar4: u16;
  let uVar5: u32;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar6: u16;
  let iVar7: i16;
  let iVar8: i16;
  let iVar9: i16;
  let uVar10: u16;
  let uVar11: u16;
  let bVar12: bool;
  let uVar13: u32;
  let iStack34: i16;
  let uStack32: u32;
  let puStack28: u32;
  let uStack10: u32;
  let uStack6: u32;
  
  uVar10 = (param_1 >> 0x10);
  iVar7 = param_1;
  if ((iVar7 + 0x206) == 0x0) {
    if (*(long *)(iVar7 + 0xc) == 0x0) {
      param_2 = 0x0;
      uVar11 = 0x0;
    }
    else {
      uVar2 = (iVar7 + 0xc);
      ppcVar3 = (code **)((iVar7 + 0xc) + 0x10);
      (**ppcVar3)(param_3,uVar2,(uVar2 >> 0x10));
      uVar11 = extraout_DX;
    }
    uStack6 = CONCAT22(uVar11,param_2);
    for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
      uVar2 = (iVar7 + 0xc);
      ppcVar3 = (code **)((iVar7 + 0xc) + 0x4);
      uVar5 = uStack6;
      (**ppcVar3)((char)param_3,uVar2,(uVar2 >> 0x10),uStack10,
                  (uStack10 >> 0x10));
      uVar4 = uVar5;
      uVar6 = extraout_DX_00 | uVar4;
      if (uVar6 != 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4,extraout_DX_00);
        param_3 = 0x1030;
        uVar13 = struct_op_1030_73a8(CONCAT22(uVar6,uVar4));
        uVar6 = (uVar13 >> 0x10);
        iVar8 = (uVar13 + 0x12);
        uVar4 = uVar13 + 0x14;
        uVar5 = uVar4;
        puStack28 = (uVar13 & 0xffff0000 | uVar4);
        uStack32 = 0x0;
        if ((iVar8 == 0x4) || (iVar8 == 0x5)) {
          uVar5 = *puStack28;
          uStack32 = uVar5;
        }
        if (uStack32 != 0x0) {
          for (iStack34 = 0x11; iStack34 < 0x25; iStack34 += 0x1) {
            if ((((iVar7 + 0x204) == 0x0) || (iStack34 == 0x23)) ||
               (iStack34 == 0x24)) {
              empty_1038_540a();
              iVar8 = iStack34 * 0x4;
              uVar11 = (uStack32 >> 0x10);
              iVar9 = uStack32;
              puVar1 = (iVar8 + iVar9 + 0x2);
              bVar12 = *puVar1 < uVar6;
              if ((bVar12 || *puVar1 == uVar6) &&
                 ((bVar12 ||
                  (puVar1 = (iVar8 + iVar9),
                  *puVar1 < uVar5 || *puVar1 == uVar5)))) {
                pass1_1038_5770(param_1,*(long *)(iVar8 + iVar9),iStack34);
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

void 
pass1_1038_52b8(param_1: u32,param_2: u32,param_3: u16,param_4: u16,param_5: u16,
               param_6: u16,param_7: u16)

{
  let uVar1: u32;
  code **ppcVar2;
  let uVar3: u32;
  let iVar4: i16;
  let uVar5: u16;
  let extraout_DX: u16;
  let uVar6: u16;
  let extraout_DX_00: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u32;
  let iVar11: i16;
  let uVar12: u16;
  let uStack26: u16;
  let iStack24: i16;
  let uStack22: u32;
  let uStack14: u32;
  let uStack10: u32;
  astruct_601 *iVar10;
  
  iVar4 = -param_2;
  iVar11 = param_1;
  pass1_1038_5694(param_1,CONCAT22(-(param_2._2_2_ + (param_2 != 0x0)),iVar4),
                  param_3);
  if (param_3 != 0x24) {
    uVar8 = (param_1 >> 0x10);
    if (*(long *)(iVar11 + 0xc) == 0x0) {
      iVar4 = 0x0;
      uVar6 = 0x0;
    }
    else {
      uVar1 = (iVar11 + 0xc);
      ppcVar2 = (code **)((iVar11 + 0xc) + 0x10);
      (**ppcVar2)(param_6,uVar1,(uVar1 >> 0x10));
      uVar6 = extraout_DX;
    }
    uStack10 = CONCAT22(uVar6,iVar4);
    for (uStack14 = 0x0; uVar3 = param_2, uStack14 < uStack10; uStack14 += 0x1) {
      uVar1 = (iVar11 + 0xc);
      ppcVar2 = (code **)((iVar11 + 0xc) + 0x4);
      uVar9 = uStack10;
      (**ppcVar2)(param_6,uVar1,(uVar1 >> 0x10),uStack14,
                  (uStack14 >> 0x10));
      uVar5 = uVar9;
      uVar7 = extraout_DX_00 | uVar5;
      if (uVar7 != 0x0) {
        uVar12 = param_3;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5,extraout_DX_00);
        uStack22 = CONCAT22(uVar7,uVar5);
        param_6 = 0x1030;
        uVar9 = pass1_1030_7c28(CONCAT22(uVar7,uVar5),uVar12,uVar5,uVar7,param_7);
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
            iStack24 = (uVar7 - param_2._2_2_) - (uVar5 < param_2);
            param_2 = 0x0;
            uVar9 = uVar3;
          }
          param_6 = 0x1030;
          pass1_1030_7d1c(uStack22,uStack26,CONCAT22(param_3,iStack24),uVar9,
                          param_2._2_2_,param_4,param_5,param_7);
          if (param_2 == 0x0) {
            return;
          }
        }
      }
    }
  }
  return;
}



fn pass1_1038_53ba(param_1: u32,param_2: i16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x1a2 + param_2 * 0x4) <
      (param_1 + 0x14e + param_2 * 0x4)) {
    return;
  }
  return;
}



fn empty_1038_540a(void)
{
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_5464(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  let uVar1: u32;
  code **ppcVar2;
  let uVar3: u16;
  let uVar4: u32;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let extraout_DX_01: u16;
  let extraout_DX_02: u16;
  let uVar5: u16;
  let iVar6: i16;
  let iVar7: i16;
  let uVar8: u16;
  let uVar9: u16;
  let local_2e: u16;
  let uStack44: u16;
  let local_2a: u16;
  let uStack40: u16;
  let puStack34: u32;
  let uStack30: u16;
  let uStack28: u16;
  let puStack26: u32;
  let uStack22: u32;
  let uStack18: u16;
  let uStack16: u16;
  let uStack14: u32;
  let uStack10: u32;
  let uStack6: u32;
  
  pass1_1038_56ba(param_1);
  pass1_1038_57c0(param_1);
  uVar8 = (param_1 >> 0x10);
  iVar6 = param_1;
  if (*(long *)(iVar6 + 0xc) == 0x0) {
    param_2 = 0x0;
    uVar5 = 0x0;
  }
  else {
    uVar1 = (iVar6 + 0xc);
    ppcVar2 = (code **)((iVar6 + 0xc) + 0x10);
    (**ppcVar2)(param_3,uVar1,(uVar1 >> 0x10));
    uVar5 = extraout_DX;
  }
  uStack10 = CONCAT22(uVar5,param_2);
  for (uStack14 = 0x0; uStack14 < uStack10; uStack14 += 0x1) {
    uVar1 = (iVar6 + 0xc);
    ppcVar2 = (code **)((iVar6 + 0xc) + 0x4);
    uVar4 = uStack10;
    (**ppcVar2)(param_3,uVar1,(uVar1 >> 0x10),uStack14,
                (uStack14 >> 0x10));
    uVar3 = uVar4;
    uVar5 = extraout_DX_02 | uVar3;
    uStack18 = uVar3;
    uStack16 = extraout_DX_02;
    if (uVar5 != 0x0) {
      param_3 = &USHORT_1050_1028;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,extraout_DX_02);
      uStack22 = CONCAT22(uVar5,uVar3);
      puStack26 = *(u32 **)(uVar3 + 0x22);
      if (((uVar3 + 0x24) | puStack26) == 0x0) {
        uStack28 = 0x0;
      }
      else {
        uStack28 = (puStack26 + 0x4);
      }
      for (uStack30 = 0x0; uStack30 < uStack28; uStack30 += 0x1) {
        param_3 = 0x1020;
        pass1_1020_bb16(puStack26,
                        CONCAT13((char)(param_4 >> 0x8),
                                          CONCAT12((char)param_4,&local_2e)),
                        CONCAT22(param_4,&local_2a),uStack30);
        if (CONCAT22(uStack44,local_2e) != 0x0) {
          pass1_1038_5694(param_1,CONCAT22(uStack44,local_2e),local_2a);
        }
      }
      uVar9 = (uStack22 >> 0x10);
      puStack34 = (uStack22 + 0x1e);
      uVar5 = (uStack22 + 0x20);
      uVar3 = uVar5 | puStack34;
      if (uVar3 == 0x0) {
        uVar3 = 0x0;
      }
      else {
        ppcVar2 = (code **)(*puStack34 + 0x10);
        (**ppcVar2)(param_3,puStack34,uVar5);
        uVar5 = extraout_DX_00;
      }
      uStack28 = uVar3;
      for (uStack30 = 0x0; uStack30 < uStack28; uStack30 += 0x1) {
        ppcVar2 = (code **)(*puStack34 + 0x4);
        uVar3 = uStack28;
        (**ppcVar2)(param_3,puStack34,(puStack34 >> 0x10),uStack30,0x0);
        uVar5 = extraout_DX_01 | uVar3;
        local_2a = uVar3;
        uStack40 = extraout_DX_01;
        if (uVar5 != 0x0) {
          param_3 = &USHORT_1050_1028;
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,extraout_DX_01);
          iVar7 = (uVar3 + 0xc) * 0x4;
          *(long *)(iVar6 + iVar7 + 0x14e) = *(long *)(iVar6 + 0x14e + iVar7) + 0x1;
        }
      }
    }
  }
  uVar4 = uStack10;
  pass1_1030_38f2((iVar6 + 0x1f6),0x3,param_4);
  uVar3 = uVar4;
  uStack6._0_2_ = uVar3;
  uStack6._2_2_ = uVar5;
  pass1_1030_38f2((iVar6 + 0x1f6),0x4,param_4);
  uStack6 = CONCAT22(uStack6._2_2_ + uVar5 + CARRY2(uStack6,uVar3),
                     uStack6 + uVar3);
  if (uStack6 == 0x0) {
    pass1_1030_38f2((iVar6 + 0x1f6),0x2,param_4);
    uStack6 = CONCAT22(uVar5,uVar3);
  }
  uVar1 = (iVar6 + 0x1f6);
  uStack6 += *(long *)(uVar1 + 0x170);
  pass1_1038_5694(param_1,uStack6,0x24);
  return;
}



fn pass1_1038_565e(param_1: u16,uchar *param_2,param_3: u32) -> u32

{
  let iVar1: i16;
  let uVar2: u16;
  let uVar3: u32;
  let local_4: [u8;2];
  
  uVar2 = (param_3 >> 0x10);
  iVar1 = param_3;
  uVar3 = pass1_1030_8e3c(param_1,local_4,param_2,CONCAT22(param_1,local_4),
                          (iVar1 + 0x4));
  pass1_1038_582c(param_3,uVar3);
  return CONCAT22((iVar1 + 0x16),(iVar1 + 0x14));
}



fn pass1_1038_5694(param_1: u32,param_2: i32,param_3: i16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *(long *)(param_1 + param_3 * 0x4 + 0x26) =
       *(long *)(param_1 + 0x26 + param_3 * 0x4) + param_2;
  return;
}



fn pass1_1038_56ba(param_1: u32)
{
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (param_1 + 0x26)),
                  (WNDCLASS16 *)0x0,0x94);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_56d6(param_1: u32,param_2: i16)
{
  code **ppcVar1;
  let iVar2: i16;
  let puVar3: *mut u16;
  let uVar4: u16;
  let uVar5: u32;
  let extraout_DX: u16;
  let uVar6: u16;
  let extraout_DX_00: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let uStack10: u32;
  let uStack6: u32;
  
  iVar2 = param_1;
  uVar9 = 0x1000;
  puVar3 = pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (iVar2 + 0xba)),
                           (WNDCLASS16 *)0x0,0x94);
  if (param_2 != 0x0) {
    uVar8 = (param_1 >> 0x10);
    if (*(long *)(iVar2 + 0xc) == 0x0) {
      puVar3 = 0x0;
      uVar6 = 0x0;
    }
    else {
      ppcVar1 = (code **)((iVar2 + 0xc) + 0x10);
      (**ppcVar1)();
      uVar6 = extraout_DX;
    }
    uStack6 = CONCAT22(uVar6,puVar3);
    for (uStack10 = 0x0; uStack10 < uStack6; uStack10 += 0x1) {
      ppcVar1 = (code **)((iVar2 + 0xc) + 0x4);
      uVar5 = uStack6;
      (**ppcVar1)(uVar9,(iVar2 + 0xc));
      uVar4 = uVar5;
      uVar7 = extraout_DX_00 | uVar4;
      if (uVar7 != 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4,extraout_DX_00);
        uVar9 = 0x1030;
        pass1_1030_72d0(CONCAT22(uVar7,uVar4));
      }
    }
  }
  return;
}



fn pass1_1038_5770(param_1: u32,param_2: i32,param_3: i16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *(long *)(param_1 + param_3 * 0x4 + 0xba) =
       *(long *)(param_1 + 0xba + param_3 * 0x4) + param_2;
  return;
}



fn pass1_1038_5798(param_1: u32,param_2: i32,param_3: i16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *(long *)(param_1 + param_3 * 0x4 + 0x14e) =
       *(long *)(param_1 + 0x14e + param_3 * 0x4) + param_2;
  return;
}



fn pass1_1038_57c0(param_1: u32)
{
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (param_1 + 0x14e)),
                  (WNDCLASS16 *)0x0,0x54);
  return;
}



fn pass1_1038_57dc(param_1: u32,param_2: i32,param_3: i16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *(long *)(param_1 + param_3 * 0x4 + 0x1a2) =
       *(long *)(param_1 + 0x1a2 + param_3 * 0x4) + param_2;
  return;
}



fn pass1_1038_5804(param_1: u32,param_2: i32,param_3: i16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *(long *)(param_1 + param_3 * 0x4 + 0x1a2) =
       *(long *)(param_1 + 0x1a2 + param_3 * 0x4) - param_2;
  return;
}



fn pass1_1038_582c(param_1: u32,param_2: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0x14);
  uVar2 = (iVar4 + 0x16);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (iVar4 + 0x14) = param_2;
  return;
}



fn pass1_1038_5860(param_1: u32,param_2: u16,param_3: u32,param_4: i16)
{
  code **ppcVar1;
  let uVar2: u32;
  let uVar3: u32;
  let extraout_DX: u16;
  let extraout_DX_00: i16;
  let iVar4: i16;
  let uVar5: u16;
  let uStack14: u32;
  let iStack6: i16;
  let iStack4: i16;
  
  if (param_4 == 0x0) {
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    ppcVar1 = (code **)((iVar4 + 0xc) + 0x10);
    uVar2 = param_3;
    (**ppcVar1)();
    uVar2 = uVar2 & 0xffff | extraout_DX << 0x10;
    for (uStack14 = 0x0; uStack14 < uVar2; uStack14 += 0x1) {
      ppcVar1 = (code **)((iVar4 + 0xc) + 0x4);
      uVar3 = uVar2;
      (**ppcVar1)();
      iStack6 = param_3;
      if ((uVar3 == iStack6) &&
         (iStack4 = (param_3 >> 0x10), extraout_DX_00 == iStack4)) {
        return;
      }
    }
    ppcVar1 = (code **)((iVar4 + 0xc) + 0xc);
    (**ppcVar1)();
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_58e6(param_1: u16,param_2: u16,param_3: u32,param_4: u32,param_5: u32,
               param_6: i16,param_7: u16)

{
  let iVar1: i16;
  code **ppcVar2;
  let uVar3: u32;
  let Bvar4: bool;
  let puVar5: u32;
  let uVar6: u16;
  let iVar7: i16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u32;
  let local_12: u32;
  let iStack14: i16;
  let iStack12: i16;
  let uStack6: u32;
  
  for (uStack6 = 0x0; uStack6 < param_3; uStack6 += 0x1) {
    uVar9 = (param_4 >> 0x10);
    iVar7 = param_4;
    if ((*(long *)(uStack6 * 0x4 + iVar7) != 0x0) &&
       (uVar3 = (uStack6 * 0x4 + iVar7),
       BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,(uVar3 + 0xc),0x2e)
       , BVar4 != 0x0)) {
      uVar8 = (param_5 >> 0x10);
      iVar1 = (uStack6 * 0x4 + param_5);
      uVar8 = (uStack6 * 0x4 + param_5 + 0x2);
      local_12 = (iVar1 + 0xc);
      iStack12 = (iVar1 + 0x10);
      iStack14 = iStack12;
      if (iStack12 == param_6) {
        iStack14 = iStack12 + -0x1;
        uVar10 = pass1_1028_bb24((uStack6 * 0x4 + iVar7));
        uVar6 = (uVar10 >> 0x10);
        puVar5 = &local_12;
        pass1_1030_627e(param_7,puVar5,uVar6,_PTR_LOOP_1050_5740,
                        CONCAT22(param_7,puVar5),
                        uVar10 & 0xffff | uVar6 << 0x10);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,puVar5,uVar6);
        if ((uVar6 | puVar5) != 0x0) {
          uVar10 = struct_op_1030_73a8(CONCAT22(uVar6,puVar5));
          uVar6 = (uVar10 + 0x1a);
          if (((uVar6 & 0x2) != 0x0) && ((uVar6 & 0x1) != 0x0)) {
            uVar3 = (uStack6 * 0x4 + iVar7);
            (uVar3 + 0x1a) = 0x3;
            ppcVar2 = (code **)(
                                      (uStack6 * 0x4 + iVar7) + 0x28);
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

fn pass1_1038_5a16(param_1: u16,param_2: u16,param_3: u32,param_4: u32)
{
  code **ppcVar1;
  let uVar2: u32;
  let BVar3: bool;
  let iVar4: i16;
  let uVar5: u16;
  let uStack6: u32;
  
  for (uStack6 = 0x0; uStack6 < param_3; uStack6 += 0x1) {
    uVar5 = (param_4 >> 0x10);
    iVar4 = param_4;
    if ((*(long *)(uStack6 * 0x4 + iVar4) != 0x0) &&
       (uVar2 = (uStack6 * 0x4 + iVar4),
       BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,(uVar2 + 0xc),0x2f)
       , BVar3 != 0x0)) {
      uVar2 = (uStack6 * 0x4 + iVar4);
      (uVar2 + 0x1a) = 0x3;
      ppcVar1 = (code **)((uStack6 * 0x4 + iVar4)
                         + 0x28);
      (**ppcVar1)();
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_5a96(param_1: u16,param_2: u16,param_3: u32,param_4: u32)
{
  code **ppcVar1;
  let uVar2: u32;
  let BVar3: bool;
  let iVar4: i16;
  let uVar5: u16;
  let uStack6: u32;
  
  for (uStack6 = 0x0; uStack6 < param_3; uStack6 += 0x1) {
    uVar5 = (param_4 >> 0x10);
    iVar4 = param_4;
    if ((*(long *)(uStack6 * 0x4 + iVar4) != 0x0) &&
       (uVar2 = (uStack6 * 0x4 + iVar4),
       BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,(uVar2 + 0xc),0x2c)
       , BVar3 != 0x0)) {
      ppcVar1 = (code **)((uStack6 * 0x4 + iVar4)
                         + 0x54);
      (**ppcVar1)();
      if (BVar3 != 0x0) {
        uVar2 = (iVar4 + uStack6 * 0x4);
        (uVar2 + 0x1a) = 0x3;
        ppcVar1 = (code **)(
                                  (uStack6 * 0x4 + iVar4) + 0x28);
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

fn pass1_1038_5b3c(param_1: u16,param_2: u16,param_3: u32,param_4: u32)
{
  code **ppcVar1;
  let uVar2: u32;
  let uVar3: u32;
  let Bvar4: bool;
  let iVar5: i16;
  let uVar6: u16;
  let uStack6: u32;
  
  for (uStack6 = 0x0; uStack6 < param_3; uStack6 += 0x1) {
    uVar6 = (param_4 >> 0x10);
    iVar5 = param_4;
    if (((*(long *)(uStack6 * 0x4 + iVar5) != 0x0) &&
        (uVar2 = (uStack6 * 0x4 + iVar5),
        BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,(uVar2 + 0xc),0x2d
                               ), BVar4 != 0x0)) &&
       (ppcVar1 = (code **)(
                                  (uStack6 * 0x4 + iVar5) + 0x50),
       (**ppcVar1)(), BVar4 != 0x0)) {
      uVar2 = (uStack6 * 0x4 + iVar5);
      uVar3 = (uStack6 * 0x4 + iVar5);
      (uVar3 + 0x1a) = (uVar2 + 0x1a) | 0x1;
      ppcVar1 = (code **)((uStack6 * 0x4 + iVar5)
                         + 0x28);
      (**ppcVar1)();
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 
pass1_1038_5be8(param_1: u32,param_2: u16,param_3: i16,param_4: *mut u16,param_5: u16,
               param_6: u16,param_7: u16)

{
  let iVar1: i16;
  let uVar2: u16;
  let iVar3: i16;
  let Bvar4: bool;
  let uVar5: u16;
  let uVar6: u32;
  let iStack14: i16;
  let uStack10: u32;
  
  pass1_1030_627e(param_7,param_5,param_6,_PTR_LOOP_1050_5740,param_4,
                  *(long *)(param_1 + 0x8));
  uVar5 = param_6 | param_5;
  if (uVar5 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5,param_6);
    uStack10 = CONCAT22(uVar5,param_5);
    iStack14 = 0x7a;
    if (0x0 < (param_4 + 0x4)) {
      if (param_3 == 0x7b) {
        param_3 = 0x7e;
      }
      else {
        if (param_3 == 0x7c) {
          param_3 = 0x7d;
        }
      }
      iStack14 = 0x7f;
    }
    uVar6 = struct_op_1030_73a8(uStack10);
    uVar2 = (uVar6 >> 0x10);
    iVar3 = uVar6;
    if (((((iVar3 + 0x1a) & param_2) == 0x0) &&
        (((iVar1 = (iVar3 + 0xc), iVar1 == iStack14 || (iVar1 == param_3)) ||
         (BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,iVar1,0x2b), BVar4 != 0x0)))) &&
       ((iVar3 + 0x12) != 0x7)) {
      (iVar3 + 0x1a) = (iVar3 + 0x1a) | param_2;
      return 0x1;
    }
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps

void 
pass1_1038_5cc6(param_1: u32,param_2: u32,param_3: u32,param_4: u32,param_5: i16,
               param_6: u16)

{
  let uVar1: u32;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let unaff_SS: u16;
  let puVar6: *mut u16;
  let local_14: i16;
  let local_12: i16;
  let local_10: i16;
  let uStack14: u32;
  let local_a: i16;
  let iStack8: i16;
  let iStack4: i16;
  
  puVar6 = pass1_1008_3e38((u16 *)CONCAT22(unaff_SS,&local_a));
  uVar4 = (puVar6 >> 0x10);
  do {
    iStack4 = 0x0;
    for (uStack14 = 0x0; uStack14 < param_2; uStack14 += 0x1) {
      uVar5 = (param_4 >> 0x10);
      if (*(long *)(uStack14 * 0x4 + param_4) != 0x0) {
        uVar1 = (uStack14 * 0x4 + param_4);
        pass1_1008_3f62((u16 *)CONCAT22(unaff_SS,&local_a),
                        (uVar1 & 0xffff0000 | (uVar1 + 0xc)));
        pass1_1008_3eb4((u16 *)CONCAT22(unaff_SS,&local_a),
                        CONCAT22(unaff_SS,&local_14),
                        CONCAT22(unaff_SS,&local_12),
                        CONCAT22(unaff_SS,&local_10));
        if (local_14 == param_5) {
          uVar5 = (param_3 >> 0x10);
          if ((*(long *)(uStack14 * 0x4 + param_3) != 0x0) &&
             (uVar2 = (uStack14 * 0x4 + param_3),
             ((uVar2 + 0x1a) & param_6) != 0x0)) {
            iStack8 = local_12 + -0x1;
            uVar3 = pass1_1038_5be8(param_1,param_6,0x7b,
                                    CONCAT22(unaff_SS,&local_a),&local_a,
                                    uVar4,unaff_SS);
            if (uVar3 != 0x0) {
              iStack4 = 0x1;
            }
            iStack8 = local_12 + 0x1;
            uVar3 = pass1_1038_5be8(param_1,param_6,0x7b,
                                    CONCAT22(unaff_SS,&local_a),&local_a,
                                    uVar4,unaff_SS);
            if (uVar3 != 0x0) {
              iStack4 = 0x1;
            }
            iStack8 = local_12;
            local_a = local_10 + -0x1;
            uVar3 = pass1_1038_5be8(param_1,param_6,0x7c,
                                    CONCAT22(unaff_SS,&local_a),&local_a,
                                    uVar4,unaff_SS);
            if (uVar3 != 0x0) {
              iStack4 = 0x1;
            }
            local_a = local_10 + 0x1;
            uVar3 = pass1_1038_5be8(param_1,param_6,0x7c,
                                    CONCAT22(unaff_SS,&local_a),&local_a,
                                    uVar4,unaff_SS);
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

fn pass1_1038_5e16(param_1: u32,param_2: u32,param_3: i16,param_4: u16,param_5: u16)
{
  let BVar1: bool;
  let puVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let local_14: [u32;0x2];
  let local_c: u32;
  let puStack6: u32;
  
  pass1_1030_16d6(param_1,param_2,param_5);
  if (param_3 != 0x0) {
    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    puVar2 = *(u32 **)(iVar3 + 0xc);
    puStack6 = puVar2;
    pass1_1008_7898(param_2,puVar2,puVar2,param_4,0x1008,param_5);
    if (puVar2 != 0x0) {
      local_14[0] = (iVar3 + 0x10);
      uVar5 = param_2;
      uVar6 = (param_2 >> 0x10);
      BVar1 = write_to_file_1008_7e1c
                        (uVar5,uVar6,local_14,param_5,0x4,0x1008);
      if (BVar1 != 0x0) {
        local_c._0_2_ = (iVar3 + 0x18);
        BVar1 = write_to_file_1008_7e1c
                          (uVar5,uVar6,&local_c,param_5,0x2,0x1008);
        if (BVar1 != 0x0) {
          local_c._0_2_ = (iVar3 + 0x1a);
          BVar1 = write_to_file_1008_7e1c
                            (uVar5,uVar6,&local_c,param_5,0x2,0x1008);
          if (BVar1 != 0x0) {
            local_c = CONCAT22(local_c._2_2_,(iVar3 + 0x1c));
            BVar1 = write_to_file_1008_7e1c
                              (uVar5,uVar6,&local_c,param_5,0x2,0x1008);
            if (BVar1 != 0x0) {
              local_c = (iVar3 + 0x1e);
              BVar1 = write_to_file_1008_7e1c
                                (uVar5,uVar6,&local_c,param_5,0x4,0x1008);
              if (BVar1 != 0x0) {
                local_c = local_c & 0xffff0000 | (iVar3 + 0x22);
                BVar1 = write_to_file_1008_7e1c
                                  (uVar5,uVar6,&local_c,param_5,0x2,0x1008
                                  );
                if (BVar1 != 0x0) {
                  local_c = local_c & 0xffff0000 | (iVar3 + 0x24);
                  BVar1 = write_to_file_1008_7e1c
                                    (uVar5,uVar6,&local_c,param_5,0x2,
                                     0x1008);
                  if (BVar1 != 0x0) {
                    BVar1 = write_to_file_1008_7e1c
                                      (uVar5,uVar6,iVar3 + 0x26,uVar4,0x94,0x1008)
                    ;
                    if (BVar1 != 0x0) {
                      BVar1 = write_to_file_1008_7e1c
                                        (uVar5,uVar6,iVar3 + 0x14e,uVar4,0x54,
                                         0x1008);
                      if (BVar1 != 0x0) {
                        BVar1 = write_to_file_1008_7e1c
                                          (uVar5,uVar6,iVar3 + 0x1a2,uVar4,0x54,
                                           0x1008);
                        if (BVar1 != 0x0) {
                          write_to_file_1030_32e4
                                    ((iVar3 + 0x1f6),param_2,param_5);
                          BVar1 = pass1_1008_7c2a(param_2,*(char **)(iVar3 + 0x1fa),0x1008
                                                 );
                          if (BVar1 != 0x0) {
                            local_c = local_c & 0xffff0000 |
                                      (iVar3 + 0x1fe);
                            BVar1 = write_to_file_1008_7e1c
                                              (uVar5,uVar6,&local_c,param_5,
                                               0x2,0x1008);
                            if (BVar1 != 0x0) {
                              local_c = (iVar3 + 0x200);
                              BVar1 = write_to_file_1008_7e1c
                                                (uVar5,uVar6,&local_c,param_5,
                                                 0x4,0x1008);
                              if (BVar1 != 0x0) {
                                local_c = local_c & 0xffff0000 |
                                          (iVar3 + 0x204);
                                BVar1 = write_to_file_1008_7e1c
                                                  (uVar5,uVar6,&local_c,param_5,
                                                   0x2,0x1008);
                                if (BVar1 != 0x0) {
                                  local_c = local_c & 0xffff0000 |
                                            (iVar3 + 0x206);
                                  BVar1 = write_to_file_1008_7e1c
                                                    (uVar5,uVar6,&local_c,param_5,
                                                     0x2,0x1008);
                                  if (BVar1 != 0x0) {
                                    local_c = local_c & 0xffff0000 |
                                              (iVar3 + 0x208);
                                    BVar1 = write_to_file_1008_7e1c
                                                      (uVar5,uVar6,&local_c,
                                                       param_5,0x2,0x1008);
                                    if (BVar1 != 0x0) {
                                      local_c = local_c & 0xffff0000 |
                                                (iVar3 + 0x20a);
                                      BVar1 = write_to_file_1008_7e1c
                                                        (uVar5,uVar6,&local_c,
                                                         param_5,0x2,0x1008);
                                      if (BVar1 != 0x0) {
                                        local_c = local_c & 0xffff0000 |
                                                  (iVar3 + 0x20c);
                                        BVar1 = write_to_file_1008_7e1c
                                                          (uVar5,uVar6,&local_c,
                                                           param_5,0x2,0x1008);
                                        if (BVar1 != 0x0) {
                                          local_c = local_c & 0xffff0000 |
                                                    (iVar3 + 0x20e);
                                          BVar1 = write_to_file_1008_7e1c
                                                            (uVar5,uVar6,&local_c,
                                                             param_5,0x2,0x1008);
                                          if (BVar1 != 0x0) {
                                            local_c = local_c & 0xffff0000 |
                                                      (iVar3 + 0x214);
                                            BVar1 = write_to_file_1008_7e1c
                                                              (uVar5,uVar6,
                                                               &local_c,param_5,
                                                               0x2,0x1008);
                                            if (BVar1 != 0x0) {
                                              local_c = (iVar3 + 0x216);
                                              BVar1 = write_to_file_1008_7e1c
                                                                (uVar5,uVar6,
                                                                 &local_c,param_5,
                                                                 0x4,0x1008);
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
    PTR_LOOP_1050_0310 = 0x6d0;
  }
  return;
}



fn file_1038_6118(param_1: u32,param_2: u32,param_3: i16,uchar *param_4,param_5: u16)
{
  let uVar1: u16;
  let puVar2: u32;
  let BVar3: bool;
  let uVar4: u16;
  let uVar5: u16;
  let puVar6: *mut u8;
  let puVar7: *mut u8
  astruct_429 *iVar9;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  SEGPTR SVar11;
  astruct_18 *paStack1046;
  let uStack1042: u16;
  let local_408: [u8;400];
  let local_8: u16;
  let local_6: u32;
  
  file_1030_1730(param_1,param_2);
  if (param_3 == 0x0) {
    return;
  }
  local_6 = 0x0;
  puVar2 = &local_6;
  file_1008_7548(param_2,(long *)CONCAT22(param_5,puVar2),0x1008,param_5);
  if (puVar2 != 0x0) {
    uVar8 = (param_1 >> 0x10);
    iVar9 = (astruct_429 *)param_1;
    iVar9->field_0xc = local_6;
    uVar9 = param_2;
    uVar10 = (param_2 >> 0x10);
    BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x10,0x0,uVar8,0x4,0x1008);
    if (((((BVar3 != 0x0) &&
          (BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x18,0x0,uVar8,0x2,
                                       0x1008), BVar3 != 0x0)) &&
         (BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x1a,0x0,uVar8,0x2,0x1008
                                     ), BVar3 != 0x0)) &&
        ((BVar3 = read_file_1008_7dee(uVar9,uVar10,&local_8,0x0,param_5,0x2,0x1008
                                     ), BVar3 != 0x0 &&
         (BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x1e,0x0,uVar8,0x4,0x1008
                                     ), BVar3 != 0x0)))) &&
       (BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x22,0x0,uVar8,0x2,0x1008),
       BVar3 != 0x0)) {
      iVar9->field_0x1c = local_8;
      BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x24,0x0,uVar8,0x2,0x1008);
      if ((BVar3 != 0x0) &&
         (uVar4 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x26,0x0,uVar8,0x94,
                                      0x1008), uVar4 != 0x0)) {
        if (PTR_LOOP_1050_0312 < 0x2) {
          uVar5 = 0x54;
          SVar11 = 0x54;
          mem_op_1000_179c(0x54,param_4,0x1000);
          paStack1046 = (astruct_18 *)CONCAT22(param_4,uVar4);
          BVar3 = read_file_1008_7dee(uVar9,uVar10,uVar4,uVar5,param_4,SVar11,
                                      0x1008);
          if (BVar3 == 0x0) {
LAB_1038_626a:
            PTR_LOOP_1050_0310 = 0x6d2;
            fn_ptr_1000_17ce(paStack1046,0x1000);
            return;
          }
          uStack1042 = 0x0;
          do {
            uVar5 = switch_1008_72bc(uVar9,uVar10,uStack1042);
            uVar1 = (uStack1042 * 0x4 + uVar4 + 0x2);
            (&iVar9->field_0x14e + uVar5 * 0x4) =
                 (uStack1042 * 0x4 + uVar4);
            (&iVar9->field_0x150 + uVar5 * 0x4) = uVar1;
            uStack1042 += 0x1;
          } while (uStack1042 < 0x15);
          BVar3 = read_file_1008_7dee(uVar9,uVar10,uVar4,0x0,param_4,0x54,0x1008);
          if (BVar3 == 0x0) goto LAB_1038_626a;
          uStack1042 = 0x0;
          do {
            uVar5 = switch_1008_72bc(uVar9,uVar10,uStack1042);
            puVar7 = *(uchar **)(uStack1042 * 0x4 + uVar4 + 0x2);
            (&iVar9->field_0x1a2 + uVar5 * 0x4) =
                 (uStack1042 * 0x4 + uVar4);
            *(uchar **)(&iVar9->field_0x1a4 + uVar5 * 0x4) = puVar7;
            uStack1042 += 0x1;
          } while (uStack1042 < 0x15);
          fn_ptr_1000_17ce(paStack1046,0x1000);
          param_4 = puVar7;
        }
        else {
          BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x14e,0x0,uVar8,0x54,
                                      0x1008);
          if (BVar3 == 0x0) {
            PTR_LOOP_1050_0310 = 0x6d2;
            return;
          }
          BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x1a2,0x0,uVar8,0x54,
                                      0x1008);
          if (BVar3 == 0x0) {
            PTR_LOOP_1050_0310 = 0x6d2;
            return;
          }
        }
        read_file_1030_33f0(iVar9->field_0x1f6,param_2);
        puVar6 = local_408;
        read_file_1008_7c6e(uVar9,uVar10,CONCAT22(param_5,puVar6),0x1008);
        if (puVar6 != 0x0) {
          uVar4 = str_op_1008_60e8(CONCAT22(param_5,local_408),param_4);
          iVar9->field_0x1fa = uVar4;
          iVar9->field_0x1fc = param_4;
          BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x1fe,0x0,uVar8,0x2,
                                      0x1008);
          if (((((BVar3 != 0x0) &&
                (BVar3 = read_file_1008_7dee(uVar9,uVar10,
                                             CONCAT11((char)(param_1 >> 0x8) + '\x02',
                                                      param_1),0x0,uVar8,0x4,0x1008)
                , BVar3 != 0x0)) &&
               (BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x204,0x0,uVar8,0x2
                                            ,0x1008), BVar3 != 0x0)) &&
              (((BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x206,0x0,uVar8,
                                             0x2,0x1008), BVar3 != 0x0 &&
                (BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x208,0x0,uVar8,
                                             0x2,0x1008), BVar3 != 0x0)) &&
               ((BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x20a,0x0,uVar8,
                                             0x2,0x1008), BVar3 != 0x0 &&
                ((BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x20c,0x0,uVar8,
                                              0x2,0x1008), BVar3 != 0x0 &&
                 (BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x20e,0x0,uVar8,
                                              0x2,0x1008), BVar3 != 0x0)))))))) &&
             ((PTR_LOOP_1050_0312 < 0x2 ||
              ((BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x214,0x0,uVar8,0x2
                                            ,0x1008), BVar3 != 0x0 &&
               (BVar3 = read_file_1008_7dee(uVar9,uVar10,&iVar9->field_0x216,0x0,uVar8,0x4
                                            ,0x1008), BVar3 != 0x0)))))) {
            return;
          }
          PTR_LOOP_1050_0310 = 0x6d0;
          return;
        }
      }
    }
  }
  PTR_LOOP_1050_0310 = 0x6d2;
  return;
}



astruct_18 *  pass1_1038_64de(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_33f8((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn struct_1038_6520(param_1: *mut u16)
{
  astruct_308 *iVar1;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_308 *)param_1;
  *param_1 = 0x389a;
  iVar1->field_0x2 = 0x1008;
  iVar1->field_0x4 = 0x0;
  iVar1->field_0x8 = 0x0;
  iVar1->field_0xc = 0x0;
  iVar1->field_0xe = 0x0;
  iVar1->field_0x12 = 0x0;
  iVar1->field_0x14 = 0x0;
  iVar1->field_0x16 = 0x0;
  pass1_1008_3e38((u16 *)
                  (param_1 & 0xffff0000 | &iVar1->field_0x1a));
  iVar1->field_0x20 = 0x0;
  iVar1->field_0x24 = 0x0;
  iVar1->field_0x26 = 0x0;
  iVar1->field_0x28 = 0x0;
  *param_1 = 0x78de;
  iVar1->field_0x2 = &PTR_LOOP_1050_1038;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_6590(param_1: *mut u16,param_2: u16,param_3: u16,param_4: u16,
               param_5: u16,param_6: u32)

{
  let uVar1: u16;
  let iVar2: i16;
  astruct_410 *iVar3;
  let uVar3: u16;
  let unaff_SS: u16;
  let puVar4: *mut u16;
  let uVar5: u32;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_410 *)param_1;
  *param_1 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  &iVar3->field_0x4 = 0x0;
  iVar3->field_0x8 = param_6;
  iVar3->field_0xc = param_4;
  iVar3->field_0xe = 0x0;
  iVar3->field_0x12 = 0x0;
  iVar3->field_0x14 = 0x0;
  iVar3->field_0x16 = param_2;
  iVar3->field_0x18 = param_3;
  puVar4 = pass1_1008_3e38((u16 *)
                           (param_1 & 0xffff0000 | &iVar3->field_0x1a)
                          );
  uVar1 = (puVar4 >> 0x10);
  &iVar3->field_0x20 = 0x0;
  iVar3->field_0x24 = 0x0;
  iVar3->field_0x26 = param_5;
  iVar3->field_0x28 = 0x0;
  *param_1 = 0x78de;
  iVar3->field_0x2 = &PTR_LOOP_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_6,(param_6 >> 0x10));
  uVar5 = pass1_1030_6d4e(CONCAT22(uVar1,param_5),param_5,uVar1,unaff_SS);
  iVar2 = (uVar5 >> 0x10);
  iVar3->field_0x4 = uVar5;
  iVar3->field_0x6 = iVar2;
  puVar4 = (param_1 & 0xffff0000 | &iVar3->field_0x1a);
  pass1_1008_3f62(puVar4,CONCAT22(uVar1,param_5 + 0xc));
  uVar1 = puVar4;
  pass1_1010_8fba(&iVar3->field_0x4,uVar1);
  iVar3->field_0x20 = uVar1;
  iVar3->field_0x22 = iVar2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_666e(param_1: *mut u16,long *param_2,param_3: u16,param_4: u32)
{
  let uVar1: u16;
  let uVar2: u16;
  astruct_420 *iVar3;
  let uVar3: u16;
  let unaff_SS: u16;
  let puVar4: *mut u16;
  let uVar5: u32;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_420 *)param_1;
  *param_1 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  iVar3->field_0x4 = 0x0;
  iVar3->field_0x8 = param_4;
  iVar3->field_0xc = 0x0;
  iVar3->field_0xe = param_2;
  iVar3->field_0x12 = 0x0;
  iVar3->field_0x14 = 0x0;
  iVar3->field_0x18 = 0x0;
  iVar3->field_0x16 = 0x0;
  puVar4 = pass1_1008_3e38((u16 *)
                           (param_1 & 0xffff0000 | &iVar3->field_0x1a)
                          );
  uVar1 = (puVar4 >> 0x10);
  &iVar3->field_0x20 = 0x0;
  iVar3->field_0x24 = 0x0;
  iVar3->field_0x26 = param_3;
  iVar3->field_0x28 = 0x0;
  *param_1 = 0x78de;
  iVar3->field_0x2 = &PTR_LOOP_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4,(param_4 >> 0x10));
  uVar5 = pass1_1030_6d4e(CONCAT22(uVar1,param_3),param_3,uVar1,unaff_SS);
  uVar2 = (uVar5 >> 0x10);
  &iVar3->field_0x4 = uVar5;
  (&iVar3->field_0x4 + 0x2) = uVar2;
  puVar4 = (param_1 & 0xffff0000 | &iVar3->field_0x1a);
  pass1_1008_3f62(puVar4,CONCAT22(uVar1,param_3 + 0xc));
  uVar1 = puVar4;
  pass1_1010_8fba(iVar3->field_0x4,uVar1);
  iVar3->field_0x20 = uVar1;
  iVar3->field_0x22 = uVar2;
  pass1_1020_ba94(param_2);
  iVar3->field_0x16 = uVar1;
  iVar3->field_0x18 = uVar2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_675c(param_1: *mut u16,param_2: u32,param_3: u16,param_4: u16,param_5: u32)
{
  let uVar1: u16;
  let uVar2: u16;
  astruct_414 *iVar3;
  let uVar3: u16;
  let unaff_SS: u16;
  let puVar4: *mut u16;
  let uVar5: u32;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_414 *)param_1;
  *param_1 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  iVar3->field_0x4 = 0x0;
  iVar3->field_0x8 = param_5;
  iVar3->field_0xc = 0x0;
  iVar3->field_0xe = 0x0;
  iVar3->field_0x12 = param_3;
  iVar3->field_0x14 = 0x0;
  iVar3->field_0x16 = param_2;
  puVar4 = pass1_1008_3e38((u16 *)
                           (param_1 & 0xffff0000 | &iVar3->field_0x1a)
                          );
  uVar1 = (puVar4 >> 0x10);
  &iVar3->field_0x20 = 0x0;
  iVar3->field_0x24 = 0x0;
  iVar3->field_0x26 = param_4;
  iVar3->field_0x28 = 0x0;
  *param_1 = 0x78de;
  iVar3->field_0x2 = &PTR_LOOP_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5,(param_5 >> 0x10));
  uVar5 = pass1_1030_6d4e(CONCAT22(uVar1,param_4),param_4,uVar1,unaff_SS);
  uVar2 = (uVar5 >> 0x10);
  &iVar3->field_0x4 = uVar5;
  (&iVar3->field_0x4 + 0x2) = uVar2;
  puVar4 = (param_1 & 0xffff0000 | &iVar3->field_0x1a);
  pass1_1008_3f62(puVar4,CONCAT22(uVar1,param_4 + 0xc));
  uVar1 = puVar4;
  pass1_1010_8fba(iVar3->field_0x4,uVar1);
  iVar3->field_0x20 = uVar1;
  iVar3->field_0x22 = uVar2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_6838(param_1: *mut u16,param_2: u32,param_3: u16,param_4: u16,param_5: u32)
{
  let uVar1: u16;
  let uVar2: u16;
  astruct_415 *iVar3;
  let uVar3: u16;
  let unaff_SS: u16;
  let puVar4: *mut u16;
  let uVar5: u32;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_415 *)param_1;
  *param_1 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  iVar3->field_0x4 = 0x0;
  iVar3->field_0x8 = param_5;
  iVar3->field_0xc = 0x0;
  iVar3->field_0xe = 0x0;
  iVar3->field_0x12 = 0x0;
  iVar3->field_0x14 = param_3;
  iVar3->field_0x16 = param_2;
  puVar4 = pass1_1008_3e38((u16 *)
                           (param_1 & 0xffff0000 | &iVar3->field_0x1a)
                          );
  uVar1 = (puVar4 >> 0x10);
  &iVar3->field_0x20 = 0x0;
  iVar3->field_0x24 = 0x0;
  iVar3->field_0x26 = param_4;
  iVar3->field_0x28 = 0x0;
  *param_1 = 0x78de;
  iVar3->field_0x2 = &PTR_LOOP_1050_1038;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5,(param_5 >> 0x10));
  uVar5 = pass1_1030_6d4e(CONCAT22(uVar1,param_4),param_4,uVar1,unaff_SS);
  uVar2 = (uVar5 >> 0x10);
  &iVar3->field_0x4 = uVar5;
  (&iVar3->field_0x4 + 0x2) = uVar2;
  puVar4 = (param_1 & 0xffff0000 | &iVar3->field_0x1a);
  pass1_1008_3f62(puVar4,CONCAT22(uVar1,param_4 + 0xc));
  uVar1 = puVar4;
  pass1_1010_8fba(iVar3->field_0x4,uVar1);
  iVar3->field_0x20 = uVar1;
  iVar3->field_0x22 = uVar2;
  return;
}



fn pass1_1038_6912(param_1: *mut u16)
{
  let uVar1: u16;
  let uVar2: u16;
  code **ppcVar3;
  let puVar4: u32;
  let iVar5: i16;
  let uVar6: u16;
  astruct_18 *paStack10;
  
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  *param_1 = 0x78de;
  (iVar5 + 0x2) = &PTR_LOOP_1050_1038;
  uVar1 = (iVar5 + 0x6);
  puVar4 = (iVar5 + 0x4);
  if ((uVar1 | puVar4) != 0x0) {
    ppcVar3 = (code **)*puVar4;
    (**ppcVar3)();
  }
  uVar1 = (iVar5 + 0xe);
  uVar2 = (iVar5 + 0x10);
  paStack10 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    fn_ptr_1020_ba7e(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(paStack10,0x1000);
  }
  *param_1 = 0x389a;
  (iVar5 + 0x2) = 0x1008;
  return;
}



fn pass1_1038_6984(param_1: u32)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((iVar1 + 0xc) != 0x0) {
    pass1_1020_c3ae();
    return;
  }
  if (*(long *)(iVar1 + 0xe) != 0x0) {
    pass1_1020_ba94(*(long **)(iVar1 + 0xe));
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



fn pass1_1038_69fe(param_1: u32)
{
  (param_1 + 0x28) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_6a0e(param_1: u32,param_2: u16,param_3: u16,param_4: u16,
               param_5: u16,param_6: u16)

{
  let piVar1: *mut i16;
  let uVar2: u32;
  let uVar3: u16;
  let Bvar4: bool;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let puVar10: *mut u16;
  let uVar11: u32;
  let uStack22: u32;
  let local_10: [u8;4];
  let local_c: [u8;6];
  let uStack6: u32;
  
  uVar9 = (param_1 >> 0x10);
  uVar8 = param_1;
  if ((uVar8 + 0x28) == 0x0) {
    uVar2 = (uVar8 + 0x20);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,(uVar2 >> 0x10));
    uStack6 = CONCAT22(param_3,param_2);
    piVar1 = (i16 *)(uVar8 + 0x24);
    *piVar1 = *piVar1 + 0x3c;
    puVar10 = pass1_1008_3e38((u16 *)CONCAT22(param_6,local_c));
    uVar6 = (puVar10 >> 0x10);
    while( true ) {
      uVar3 = pass1_1038_6d24(param_1,CONCAT22(param_6,local_10),
                              CONCAT22(param_6,local_c),uStack6,
                              (uStack6 >> 0x10),param_6);
      if (uVar3 == 0x0) {
        pass1_1010_8fba((uVar8 + 0x4),0x0);
        uStack22 = CONCAT22(uVar6,uVar3);
        uVar7 = uVar6 | uVar3;
        if (uVar7 == 0x0) {
          uVar2 = (uVar8 + 0x20);
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,(uVar2 >> 0x10));
          pass1_1038_7356(param_1,CONCAT22(uVar7,uVar3),param_6,param_4,param_5);
          return;
        }
        uVar11 = struct_op_1030_73a8(uStack6);
        BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,(uVar11 + 0xc),
                                0x40);
        if (BVar4 != 0x0) {
          (uVar8 + 0x28) = 0x1;
          (uVar8 + 0x20) = uStack22;
          return;
        }
        (uVar8 + 0x20) = uStack22;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(uVar8 + 0x20),uVar6);
        uStack6 = uStack22 & 0xffff | uVar6 << 0x10;
      }
      uVar5 = pass1_1038_6e1a(uVar8,uVar9,(long *)CONCAT22(param_6,local_10));
      if ((uVar8 + 0x24) < uVar5) break;
      piVar1 = (i16 *)(uVar8 + 0x24);
      *piVar1 = *piVar1 - uVar5;
      pass1_1008_3f62((u16 *)(param_1 & 0xffff0000 | (uVar8 + 0x1a)),
                      CONCAT22(param_6,local_c));
    }
  }
  return;
}



fn pass1_1038_6b3c(param_1: u32) -> u16

{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if (((((iVar1 + 0xc) == 0x0) && ((iVar1 + 0x12) == 0x0)) &&
      ((iVar1 + 0x14) == 0x0)) &&
     ((*(long *)(iVar1 + 0xe) == 0x0 && (*(long *)(iVar1 + 0x16) != 0x0)))) {
    (iVar1 + 0x16) = 0x0;
  }
  if (*(long *)(iVar1 + 0x16) == 0x0) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_6b88(param_1: u16,param_2: u16,param_3: *mut u16,param_4: *mut u32,
               uchar *param_5,param_6: i16,param_7: u16)

{
  let puVar1: u32;
  let uVar2: u16;
  let local_12: [u32;0x2];
  let lStack10: i32;
  let puStack6: *mut u16;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_7,param_5,param_6);
  uVar2 = (puStack6 >> 0x10);
  lStack10 = *(long *)(puStack6 + 0x20);
  puVar1 = local_12;
  pass1_1030_64ce(param_7,puVar1,uVar2,_PTR_LOOP_1050_5740,param_3,lStack10,
                  CONCAT22(param_7,puVar1));
  *param_4 = *puVar1;
  return;
}



void 
pass1_1038_6bd4(param_1: u32,param_2: *mut u16,param_3: *mut u32,param_4: i16,uchar *param_5,
               param_6: i16,param_7: u16)

{
  let uStack4: u16;
  
  pass1_1008_3f62(param_2,
                          (param_1 & 0xffff0000 | (param_1 + 0x1a)));
  if (param_4 < 0x0) {
    uStack4 = *param_2 - 0x1;
  }
  else {
    uStack4 = *param_2 + 0x1;
  }
  *param_2 = uStack4;
  pass1_1038_6b88(param_1,(param_1 >> 0x10),param_2,param_3,param_5,
                  param_6,param_7);
  return;
}



void 
pass1_1038_6c1c(param_1: u32,param_2: *mut u16,param_3: *mut u32,param_4: i16,uchar *param_5,
               param_6: i16,param_7: u16)

{
  let uVar1: u16;
  let iStack4: i16;
  
  pass1_1008_3f62(param_2,
                          (param_1 & 0xffff0000 | (param_1 + 0x1a)));
  uVar1 = (param_2 >> 0x10);
  iStack4 = (param_2 + 0x2);
  if (param_4 < 0x0) {
    iStack4 += -0x1;
  }
  else {
    iStack4 += 0x1;
  }
  (param_2 + 0x2) = iStack4;
  pass1_1038_6b88(param_1,(param_1 >> 0x10),param_2,param_3,param_5,
                  param_6,param_7);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_6c68(param_1: u32,param_2: *mut u16,param_3: *mut u32,param_4: i16,uchar *param_5,
               param_6: i16,param_7: u16)

{
  let iVar1: i16;
  let uVar2: u16;
  let uVar3: u16;
  let puVar4: *mut u16;
  let uVar5: u16;
  let uVar6: u16;
  let puVar7: *mut u8
  let uVar8: u16;
  let puVar9: *mut u16;
  let uVar10: u32;
  let iStack30: i16;
  
  uVar2 = param_1;
  pass1_1008_3f62(param_2,(param_1 & 0xffff0000 | (uVar2 + 0x1a)));
  puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_7,param_5,param_6);
  uVar5 = (puVar9 >> 0x10);
  puVar4 = (param_1 & 0xffff0000 | (uVar2 + 0x1a));
  pass1_1030_627e(param_7,uVar2 + 0x1a,uVar5,_PTR_LOOP_1050_5740,puVar4,
                  *(long *)(puVar9 + 0x20));
  uVar3 = puVar4;
  uVar6 = uVar5 | uVar3;
  if (uVar6 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,uVar5);
    uVar10 = struct_op_1030_73a8(CONCAT22(uVar6,uVar3));
    puVar7 = (uVar10 >> 0x10);
    iVar1 = (uVar10 + 0xc);
    if ((iVar1 == 0x47) || (iVar1 == 0x6a)) {
      uVar8 = (param_1 >> 0x10);
      iStack30 = (uVar2 + 0x1e);
      if (param_4 < 0x0) {
        iStack30 += -0x1;
      }
      else {
        iStack30 += 0x1;
      }
      (param_2 + 0x4) = iStack30;
      pass1_1038_6b88(uVar2,uVar8,param_2,param_3,puVar7,param_6,param_7);
    }
  }
  return;
}



i16 
pass1_1038_6d24(param_1: u32,param_2: *mut u32,param_3: *mut u16,param_4: i16,param_5: u16,
               param_6: u16)

{
  let local_14: i16;
  let local_12: i16;
  let local_10: i16;
  let local_e: i16;
  let local_c: i16;
  let local_a: i16;
  let local_8: u32;
  let uStack4: u16;
  
  *param_2 = 0x0;
  local_8 = (param_4 + 0xc);
  uStack4 = (param_4 + 0x10);
  pass1_1008_3eb4((u16 *)CONCAT22(param_6,&local_8),
                  CONCAT22(param_6,&local_e),
                  CONCAT22(param_6,&local_c),
                  CONCAT22(param_6,&local_a));
  pass1_1008_3eb4((u16 *)(param_1 & 0xffff0000 | (param_1 + 0x1a)),
                  CONCAT22(param_6,&local_14),
                  CONCAT22(param_6,&local_12),
                  CONCAT22(param_6,&local_10));
  local_c -= local_12;
  local_e -= local_14;
  local_a -= local_10;
  if (((local_a == 0x0) && (local_c == 0x0)) && (local_e == 0x0)) {
    return 0x0;
  }
  if ((local_c != 0x0) || (local_a == 0x0)) {
    if ((local_a == 0x0) && (local_c != 0x0)) {
      pass1_1038_6c1c(param_1,param_3,param_2,local_c,0x0,&stack0xfffe,
                      param_6);
      return 0x1;
    }
    if (((local_a == 0x0) && (local_c == 0x0)) && (local_e != 0x0)) {
      pass1_1038_6c68(param_1,param_3,param_2,local_e,0x0,&stack0xfffe,
                      param_6);
      if (local_c != 0x0) {
        return 0x1;
      }
      return local_c;
    }
  }
  pass1_1038_6bd4(param_1,param_3,param_2,local_a,local_a,&stack0xfffe,
                  param_6);
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_6e1a(param_1: u16,param_2: u16,long *param_3) -> u16

{
  let uVar1: u16;
  let BVar2: bool;
  let uVar3: u16;
  let uVar4: u16;
  let bStack21: u8;
  let uStack4: u16;
  
  uStack4 = 0x0;
  if ((*param_3 == 0x0) && (param_3 == 0x0)) {
    return 0x1;
  }
  uVar4 = (param_3 + 0x2);
  bStack21 = (byte)(uVar4 >> 0x8);
  uVar1 = bStack21;
  if (bStack21 == 0x0) {
    uStack4 = param_3;
    goto switchD_1038_6eab_caseD_9;
  }
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,*param_3,(*param_3 >> 0x10));
  uVar3 = pass1_1030_6fa0(CONCAT22(uVar4,uVar1));
  if (uVar3 < 0xa) {
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
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar3,0x41);
    if (BVar2 != 0x0) {
      uStack4 = 0xa;
      goto switchD_1038_6eab_caseD_9;
    }
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar3,0x42);
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

void 
pass1_1038_6f5a(param_1: u32,param_2: u32,param_3: u16,uchar *param_4,param_5: u16,
               param_6: u16,param_7: u16)

{
  let uVar1: u32;
  let lVar2: i32;
  let puVar3: *mut u16;
  let uVar4: u16;
  let uVar5: u16;
  let iVar6: i16;
  let iVar7: i16;
  let uVar8: u16;
  let uVar9: u16;
  astruct_99 *paStack16;
  let uStack12: u16;
  let local_a: u16;
  let uStack8: u16;
  let local_6: u16;
  let uStack4: u16;
  astruct_623 *uVar3;
  
  uVar8 = (param_1 >> 0x10);
  iVar6 = param_1;
  if (*(long *)(iVar6 + 0xe) == 0x0) {
    if ((iVar6 + 0xc) != 0x0) {
      pass1_1030_7ddc(param_2,*(long *)(iVar6 + 0x16),(iVar6 + 0xc),param_3,
                      param_4,param_5,param_6,param_7);
      return;
    }
    if ((iVar6 + 0x12) != 0x0) {
      pass1_1030_7c50(param_2,*(long *)(iVar6 + 0x16),(iVar6 + 0x12),param_3,
                      param_4);
      return;
    }
    paStack16 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
    uVar5 = (paStack16 >> 0x10);
    uVar3 = (astruct_623 *)paStack16;
    if ((uVar5 | uVar3) == 0x0) {
      paStack16 = (astruct_99 *)0x0;
    }
    else {
      paStack16->field_0x0 = 0x389a;
      uVar3->field_0x2 = 0x1008;
      uVar3->field_0x4 = 0x0;
      uVar3->field_0x6 = 0x0;
      uVar3->field_0x8 = 0x0;
      uVar3->field_0xa = 0x0;
      uVar3->field_0xc = 0x0;
      paStack16->field_0x0 = 0x56ce;
      uVar3->field_0x2 = 0x1018;
    }
    uVar9 = (paStack16 >> 0x10);
    iVar7 = paStack16;
    (iVar7 + 0x8) = (iVar6 + 0x14);
    (iVar7 + 0xa) = (iVar6 + 0x16);
    uVar4 = pass1_1020_c42e((iVar6 + 0x14));
    lVar2 = uVar4 * (iVar7 + 0xa);
    uVar5 = lVar2;
    (iVar7 + 0xc) = uVar5;
    pass1_1030_6a2c(param_2,(long)paStack16,uVar5,(lVar2 >> 0x10),param_7)
    ;
  }
  else {
    uVar1 = (iVar6 + 0xe);
    uStack4 = (uVar1 + 0x4);
    for (uStack12 = 0x0; uStack12 < uStack4; uStack12 += 0x1) {
      puVar3 = &local_6;
      pass1_1020_bb16(*(u32 **)(iVar6 + 0xe),CONCAT22(param_7,&local_a),
                      CONCAT22(param_7,puVar3),uStack12);
      if (CONCAT22(uStack8,local_a) != 0x0) {
        pass1_1030_7ddc(param_2,CONCAT22(uStack8,local_a),local_6,puVar3,param_4,
                        param_5,param_6,param_7);
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_709c(param_1: u32,param_2: u32,uchar *param_3,param_4: u16)
{
  let puVar1: u32;
  let lVar2: i32;
  let uVar7: u16;
  let uVar8: u16;
  let puVar9: *mut u8
  astruct_618 *iVar8;
  let iVar10: i16;
  let uVar11: u16;
  let uVar12: u16;
  astruct_99 *paStack40;
  astruct_99 *paStack16;
  let uStack12: u16;
  let local_a: i32;
  let local_6: u16;
  let uStack4: u16;
  astruct_617 *uVar3;
  astruct_619 *uVar4;
  astruct_620 *uVar5;
  astruct_621 *uVar6;
  
  uVar11 = (param_1 >> 0x10);
  iVar8 = (astruct_618 *)param_1;
  if (((&iVar8->field_0xe + 0x2) | &iVar8->field_0xe) == 0x0) {
    if (iVar8->field_0xc == 0x0) {
      if (iVar8->field_0x12 == 0x0) {
        if (iVar8->field_0x14 == 0x0) {
          return;
        }
        paStack40 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
        uVar8 = (paStack40 >> 0x10);
        uVar3 = (astruct_617 *)paStack40;
        if ((uVar8 | uVar3) == 0x0) {
          paStack40 = (astruct_99 *)0x0;
        }
        else {
          paStack40->field_0x0 = 0x389a;
          uVar3->field_0x2 = 0x1008;
          uVar3->field_0x4 = 0x0;
          uVar3->field_0x6 = 0x0;
          uVar3->field_0x8 = 0x0;
          uVar3->field_0xa = 0x0;
          uVar3->field_0xc = 0x0;
          paStack40->field_0x0 = 0x56ce;
          uVar3->field_0x2 = 0x1018;
        }
        uVar12 = (paStack40 >> 0x10);
        (paStack40 + 0x8) = iVar8->field_0x14;
        (paStack40 + 0xa) = &iVar8->field_0x16;
        uVar8 = pass1_1020_c42e(iVar8->field_0x14);
      }
      else {
        pass1_1030_7c50(param_2,iVar8->field_0x16,iVar8->field_0x12,0x0,param_3);
        paStack40 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
        uVar8 = (paStack40 >> 0x10);
        uVar4 = (astruct_619 *)paStack40;
        if ((uVar8 | uVar4) == 0x0) {
          paStack40 = (astruct_99 *)0x0;
        }
        else {
          paStack40->field_0x0 = 0x389a;
          uVar4->field_0x2 = 0x1008;
          uVar4->field_0x4 = 0x0;
          uVar4->field_0x6 = 0x0;
          uVar4->field_0x8 = 0x0;
          uVar4->field_0xa = 0x0;
          uVar4->field_0xc = 0x0;
          paStack40->field_0x0 = 0x56ce;
          uVar4->field_0x2 = 0x1018;
        }
        uVar12 = (paStack40 >> 0x10);
        (paStack40 + 0x6) = iVar8->field_0x12;
        (paStack40 + 0xa) = &iVar8->field_0x16;
        uVar8 = switch_1020_c3b4(iVar8->field_0x12);
      }
      uVar12 = (paStack40 >> 0x10);
      iVar10 = paStack40;
      lVar2 = uVar8 * (iVar10 + 0xa);
      puVar9 = (lVar2 >> 0x10);
      uVar8 = lVar2;
    }
    else {
      paStack40 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
      uVar8 = (paStack40 >> 0x10);
      uVar5 = (astruct_620 *)paStack40;
      puVar9 = (uVar8 | uVar5);
      if (puVar9 == 0x0) {
        paStack40 = (astruct_99 *)0x0;
      }
      else {
        paStack40->field_0x0 = 0x389a;
        uVar5->field_0x2 = 0x1008;
        uVar5->field_0x4 = 0x0;
        uVar5->field_0x6 = 0x0;
        uVar5->field_0x8 = 0x0;
        uVar5->field_0xa = 0x0;
        uVar5->field_0xc = 0x0;
        paStack40->field_0x0 = 0x56ce;
        uVar5->field_0x2 = 0x1018;
      }
      uVar12 = (paStack40 >> 0x10);
      iVar10 = paStack40;
      (iVar10 + 0x4) = iVar8->field_0xc;
      uVar8 = &iVar8->field_0x16;
      (iVar10 + 0xa) = uVar8;
    }
    (iVar10 + 0xc) = uVar8;
    pass1_1030_6a2c(param_2,CONCAT22(uVar12,iVar10),uVar8,puVar9,param_4);
  }
  else {
    puVar1 = iVar8->field_0xe;
    uStack4 = (puVar1 + 0x4);
    for (uStack12 = 0x0; uStack12 < uStack4; uStack12 += 0x1) {
      pass1_1020_bb16(iVar8->field_0xe,CONCAT22(param_4,&local_a),
                      CONCAT22(param_4,&local_6),uStack12);
      if (local_a != 0x0) {
        paStack16 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
        uVar8 = (paStack16 >> 0x10);
        uVar6 = (astruct_621 *)paStack16;
        if ((uVar8 | uVar6) == 0x0) {
          paStack16 = (astruct_99 *)0x0;
        }
        else {
          paStack16->field_0x0 = 0x389a;
          uVar6->field_0x2 = 0x1008;
          uVar6->field_0x4 = 0x0;
          uVar6->field_0x6 = 0x0;
          uVar6->field_0x8 = 0x0;
          uVar6->field_0xa = 0x0;
          uVar6->field_0xc = 0x0;
          paStack16->field_0x0 = 0x56ce;
          uVar6->field_0x2 = 0x1018;
        }
        uVar12 = (paStack16 >> 0x10);
        iVar10 = paStack16;
        (iVar10 + 0x4) = local_6;
        (iVar10 + 0xa) = local_a;
        uVar7 = pass1_1020_c3ae();
        lVar2 = uVar7 * (iVar10 + 0xa);
        uVar8 = lVar2;
        (iVar10 + 0xc) = uVar8;
        pass1_1030_6a2c(param_2,(long)paStack16,uVar8,(lVar2 >> 0x10),
                        param_4);
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_7356(param_1: u32,param_2: u32,param_3: u16,param_4: u16,param_5: u16)
{
  uchar **ppuVar1;
  let puVar2: *mut u16;
  let uVar3: u32;
  astruct_18 *paVar4;
  let lVar5: i32;
  let BVar6: bool;
  let uVar7: u16;
  let uVar9: u16;
  let puVar10: *mut u8
  let puVar11: *mut u8
  astruct_615 *iVar9;
  let iVar12: i16;
  let uVar13: u16;
  let uVar14: u16;
  let bVar15: bool;
  let uVar16: u32;
  let uVar17: u32;
  astruct_99 *paStack50;
  astruct_99 *paStack26;
  astruct_616 *uVar8;
  astruct_622 *uVar10;
  
  uVar16 = struct_op_1030_73a8(param_2);
  puVar10 = (uVar16 >> 0x10);
  uVar7 = uVar16;
  puVar11 = puVar10;
  BVar6 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,(uVar7 + 0xc),0x4);
  iVar9 = (astruct_615 *)param_1;
  uVar13 = (param_1 >> 0x10);
  if (BVar6 == 0x0) {
    uVar9 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,(uVar7 + 0xc),0x3);
    if (uVar9 == 0x0) {
code_r0x10387545:
      pass1_1038_6f5a(param_1,param_2,uVar9,puVar11,param_4,param_5,param_3);
      goto LAB_1038_7549;
    }
    if ((iVar9->field_0xc != 0x0) || (*(long *)&iVar9->field_0xe != 0x0)) {
      uVar16 = pass1_1028_45e2(uVar16,uVar7,puVar11,param_3);
      puVar11 = (uVar16 >> 0x10);
      uVar9 = uVar16;
      ppuVar1 = (uchar **)&iVar9->field_0x18;
      bVar15 = *ppuVar1 < puVar11;
      if ((bVar15 || *ppuVar1 == puVar11) &&
         ((bVar15 || (puVar2 = &iVar9->field_0x16, *puVar2 < uVar9 || *puVar2 == uVar9))))
      goto code_r0x10387545;
    }
  }
  else {
    uVar17 = pass1_1028_62c8(uVar16,param_3);
    puVar11 = (uVar17 >> 0x10);
    uVar9 = uVar17;
    ppuVar1 = (uchar **)&iVar9->field_0x18;
    bVar15 = *ppuVar1 < puVar11;
    if ((bVar15 || *ppuVar1 == puVar11) &&
       ((bVar15 || (puVar2 = &iVar9->field_0x16, *puVar2 < uVar9 || *puVar2 == uVar9)))) {
      if (iVar9->field_0x12 == 0x0) {
        if (iVar9->field_0x14 == 0x0) goto LAB_1038_74e0;
        paStack50 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
        uVar7 = (paStack50 >> 0x10);
        uVar10 = (astruct_622 *)paStack50;
        if ((uVar7 | uVar10) == 0x0) {
          paStack50 = (astruct_99 *)0x0;
        }
        else {
          paStack50->field_0x0 = 0x389a;
          uVar10->field_0x2 = 0x1008;
          uVar10->field_0x4 = 0x0;
          uVar10->field_0x6 = 0x0;
          uVar10->field_0x8 = 0x0;
          uVar10->field_0xa = 0x0;
          uVar10->field_0xc = 0x0;
          paStack50->field_0x0 = 0x56ce;
          uVar10->field_0x2 = 0x1018;
        }
        uVar14 = (paStack50 >> 0x10);
        iVar12 = paStack50;
        (iVar12 + 0x8) = iVar9->field_0x14;
        (iVar12 + 0xa) = iVar9->field_0x16;
        uVar7 = pass1_1020_c42e(iVar9->field_0x14);
      }
      else {
        paStack26 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
        uVar7 = (paStack26 >> 0x10);
        uVar8 = (astruct_616 *)paStack26;
        if ((uVar7 | uVar8) == 0x0) {
          paStack26 = (astruct_99 *)0x0;
        }
        else {
          paStack26->field_0x0 = 0x389a;
          uVar8->field_0x2 = 0x1008;
          uVar8->field_0x4 = 0x0;
          uVar8->field_0x6 = 0x0;
          uVar8->field_0x8 = 0x0;
          uVar8->field_0xa = 0x0;
          uVar8->field_0xc = 0x0;
          paStack26->field_0x0 = 0x56ce;
          uVar8->field_0x2 = 0x1018;
        }
        uVar14 = (paStack26 >> 0x10);
        iVar12 = paStack26;
        (iVar12 + 0x6) = iVar9->field_0x12;
        (iVar12 + 0xa) = iVar9->field_0x16;
        uVar7 = switch_1020_c3b4(iVar9->field_0x12);
      }
      lVar5 = uVar7 * (iVar12 + 0xa);
      puVar11 = (lVar5 >> 0x10);
      uVar9 = lVar5;
      (iVar12 + 0xc) = uVar9;
      pass1_1028_6408(uVar16,CONCAT22(uVar14,iVar12),param_3);
      goto LAB_1038_7549;
    }
  }
LAB_1038_74e0:
  pass1_1038_709c(param_1,param_2,puVar11,param_3);
LAB_1038_7549:
  uVar3 = iVar9->field_0x8;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,(uVar3 >> 0x10));
  pass1_1030_6c4c(CONCAT22(puVar11,uVar9),(uVar9 + 0x34) + iVar9->field_0x26);
  iVar9->field_0xc = 0x0;
  iVar9->field_0x12 = 0x0;
  iVar9->field_0x14 = 0x0;
  &iVar9->field_0x16 = 0x0;
  paVar4 = *(astruct_18 **)&iVar9->field_0xe;
  uVar7 = iVar9->field_0x10;
  if ((uVar7 | paVar4) != 0x0) {
    fn_ptr_1020_ba7e((paVar4 & 0xffff | uVar7 << 0x10));
    fn_ptr_1000_17ce(paVar4,0x1000);
  }
  &iVar9->field_0xe = 0x0;
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1038_75ca(param_1: u32,param_2: u32,param_3: i16,param_4: u16)
{
  let BVar1: bool;
  let iVar2: i16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let local_10: [u32;0x2];
  let local_8: u32;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  pass1_1008_79f0(param_2,*(long *)(iVar3 + 0x4),0x1008,param_4);
  if (param_3 != 0x0) {
    local_10[0] = (iVar3 + 0x8);
    uVar5 = param_2;
    uVar6 = (param_2 >> 0x10);
    BVar1 = write_to_file_1008_7e1c
                      (uVar5,uVar6,local_10,param_4,0x4,0x1008);
    if (BVar1 != 0x0) {
      write_to_file_1008_7a22(param_2,*(long *)(iVar3 + 0xe),0x1008,param_4);
      if (BVar1 != 0x0) {
        local_8._0_2_ = (iVar3 + 0xc);
        BVar1 = write_to_file_1008_7e1c
                          (uVar5,uVar6,&local_8,param_4,0x2,0x1008);
        if (BVar1 != 0x0) {
          local_8._0_2_ = (iVar3 + 0x12);
          BVar1 = write_to_file_1008_7e1c
                            (uVar5,uVar6,&local_8,param_4,0x2,0x1008);
          if (BVar1 != 0x0) {
            local_8 = CONCAT22(local_8._2_2_,(iVar3 + 0x14));
            BVar1 = write_to_file_1008_7e1c
                              (uVar5,uVar6,&local_8,param_4,0x2,0x1008);
            if (BVar1 != 0x0) {
              local_8 = (iVar3 + 0x16);
              BVar1 = write_to_file_1008_7e1c
                                (uVar5,uVar6,&local_8,param_4,0x4,0x1008);
              if (BVar1 != 0x0) {
                iVar2 = write_to_file_1008_7b4c
                                  (param_2,param_1 & 0xffff0000 | (iVar3 + 0x1a),
                                   0x1008,param_4);
                if (iVar2 != 0x0) {
                  local_8 = (iVar3 + 0x20);
                  BVar1 = write_to_file_1008_7e1c
                                    (uVar5,uVar6,&local_8,param_4,0x4,
                                     0x1008);
                  if (BVar1 != 0x0) {
                    local_8 = local_8 & 0xffff0000 | (iVar3 + 0x24);
                    BVar1 = write_to_file_1008_7e1c
                                      (uVar5,uVar6,&local_8,param_4,0x2,
                                       0x1008);
                    if (BVar1 != 0x0) {
                      local_8 = local_8 & 0xffff0000 | (iVar3 + 0x26);
                      BVar1 = write_to_file_1008_7e1c
                                        (uVar5,uVar6,&local_8,param_4,0x2,
                                         0x1008);
                      if (BVar1 != 0x0) {
                        local_8 = local_8 & 0xffff0000 | (iVar3 + 0x28);
                        BVar1 = write_to_file_1008_7e1c
                                          (uVar5,uVar6,&local_8,param_4,
                                           0x2,0x1008);
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
  PTR_LOOP_1050_0310 = 0x6d0;
  return;
}


astruct_18 *  pass1_1038_78b8(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_6912((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_78e2(param_1: *mut u32,uchar *param_2)
{
  let uVar1: u16;
  let puVar2: *mut u8
  let extraout_DX: *mut u8
  let extraout_DX_00: u16;
  let uVar3: u16;
  astruct_431 *iVar4;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_431 *)param_1;
  uVar1 = 0x0;
  *param_1 = 0x0;
  &iVar4->field_0x4 = 0x0;
  _PTR_LOOP_1050_5a64 = param_1;
  mem_op_1000_179c(0xc,param_2,0x1000);
  puVar2 = (param_2 | uVar1);
  if (puVar2 == 0x0) {
    *param_1 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(param_2,uVar1));
    param_1 = uVar1;
    iVar4->field_0x2 = extraout_DX;
    puVar2 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar2,0x1000);
  if ((puVar2 | uVar1) == 0x0) {
    uVar1 = 0x0;
    uVar3 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(puVar2,uVar1));
    uVar3 = extraout_DX_00;
  }
  iVar4->field_0x4 = uVar1;
  iVar4->field_0x6 = uVar3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_7964(param_1: *mut u16)
{
  let uVar1: u16;
  let puVar2: u32;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  _PTR_LOOP_1050_5a64 = 0x0;
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  uVar1 = (iVar4 + 0x2);
  if ((uVar1 | *param_1) != 0x0) {
    ppcVar3 = (code **)*param_1;
    (**ppcVar3)();
  }
  puVar2 = (iVar4 + 0x4);
  uVar1 = (iVar4 + 0x6);
  if ((uVar1 | puVar2) != 0x0) {
    ppcVar3 = (code **)*puVar2;
    (**ppcVar3)();
  }
  return;
}



fn pass1_1038_79b2(param_1: u32,param_2: u32,param_3: u16,uchar *param_4)
{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  
  uVar4 = 0x1000;
  mem_op_1000_179c(0x14,param_4,0x1000);
  uVar2 = param_4 | param_3;
  if (uVar2 == 0x0) {
    param_3 = 0x0;
    uVar2 = 0x0;
  }
  else {
    uVar4 = 0x1030;
    pass1_1030_aefa((u16 *)CONCAT22(param_4,param_3),param_2);
  }
  uVar3 = (param_1 >> 0x10);
  ppcVar1 = (code **)((param_1 + 0x4) + 0x4);
  (**ppcVar1)(uVar4,(param_1 + 0x4),param_3,uVar2);
  return;
}



fn pass1_1038_79f2(param_1: u32,param_2: u32,param_3: u16)
{
  code **ppcVar1;
  let puVar2: *mut u8;
  let extraout_DX: u16;
  let iVar3: i16;
  let uVar4: u16;
  let local_e: [u8;8];
  let lStack6: i32;
  
  lStack6 = *(long *)(param_2 + 0x4);
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  pass1_1008_5784(CONCAT22(param_3,local_e),(iVar3 + 0x4));
  do {
    puVar2 = local_e;
    pass1_1008_5b12(puVar2,param_3);
    if ((extraout_DX | puVar2) == 0x0) {
      return;
    }
  } while (*(long *)(puVar2 + 0x4) != lStack6);
  ppcVar1 = (code **)((iVar3 + 0x4) + 0xc);
  (**ppcVar1)(0x1008,(iVar3 + 0x4),puVar2,extraout_DX);
  return;
}



fn pass1_1038_7a5a(param_1: *mut u32)
{
  code **ppcVar1;
  
  ppcVar1 = (code **)(*param_1 + 0x4);
  (**ppcVar1)();
  return;
}



fn pass1_1038_7a76(param_1: *mut u32,param_2: u16,param_3: i16,param_4: u16)
{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u32;
  let local_a: [u8;4];
  let uStack6: u32;
  
  pass1_1008_5784(CONCAT22(param_4,local_a),*param_1);
  while( true ) {
    uVar3 = pass1_1008_5b12(local_a,param_4);
    if (uVar3 == 0x0) break;
    pass1_1038_6a0e(uVar3,uVar3,(uVar3 >> 0x10) | uVar3,param_2,param_3,
                    param_4);
  }
  do {
    uStack6 = 0x0;
    do {
      uVar3 = pass1_1008_5b12(local_a,param_4);
      if (uVar3 == 0x0) {
        pass1_1008_5784(CONCAT22(param_4,local_a),(param_1 + 0x4))
        ;
        while( true ) {
          uVar3 = pass1_1008_5b12(local_a,param_4);
          if (uVar3 == 0x0) break;
          pass1_1030_affc(uVar3,param_3,param_4);
        }
        return;
      }
      uVar2 = pass1_1038_6b3c(uVar3);
    } while (uVar2 == 0x0);
    ppcVar1 = (code **)(*param_1 + 0xc);
    (**ppcVar1)(0x1008);
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1038_7b20(param_1: *mut u32,param_2: u32,param_3: u16) -> u16

{
  let uVar1: u32;
  let BVar2: bool;
  let uVar3: u16;
  let uVar4: u32;
  let uVar5: u16;
  let local_1c: u16;
  let uStack26: u16;
  let uStack24: u16;
  let uStack16: u32;
  let local_c: [u8;8];
  let local_4: u16;
  
  BVar2 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar2 != 0x0) {
    local_1c = (*param_1 + 0x8);
    uVar5 = (param_2 >> 0x10);
    local_4 = local_1c;
    BVar2 = write_to_file_1008_7e1c
                      (param_2,uVar5,&local_1c,param_3,0x2,0x1008)
    ;
    if (BVar2 != 0x0) {
      pass1_1008_5784(CONCAT22(param_3,local_c),*param_1);
      do {
        uStack16 = pass1_1008_5b12(local_c,param_3);
        if (uStack16 == 0x0) {
          uVar3 = (param_1 >> 0x10);
          uVar1 = (param_1 + 0x4);
          local_1c = (uVar1 + 0x8);
          local_4 = local_1c;
          BVar2 = write_to_file_1008_7e1c
                            (param_2,uVar5,&local_4,param_3,0x2,
                             0x1008);
          if (BVar2 == 0x0) {
            return 0x0;
          }
          pass1_1008_5784(CONCAT22(param_3,local_c),
                          (param_1 + 0x4));
          do {
            uVar4 = pass1_1008_5b12(local_c,param_3);
            uStack26 = uVar4;
            if (uVar4 == 0x0) {
              return 0x1;
            }
            pass1_1030_b768(uVar4,param_2,param_3);
            uStack24 = (uVar4 >> 0x10);
          } while (uVar4 != 0x0);
          return 0x0;
        }
        pass1_1038_75ca(uStack16,param_2,uStack16,param_3);
        uStack16._2_2_ = (uStack16 >> 0x10);
      } while (uStack16 != 0x0);
    }
  }
  return 0x0;
}


astruct_57 * 
pass1_1038_7d10(astruct_57 *param_1,param_2: u16,uchar *param_3,param_4: i16,
               param_5: u16)

{
  astruct_703 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  struct_1040_b082(param_1,CONCAT22(param_2,0x1853));
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_703 *)param_1;
  &iVar1->field_0x94 = 0x0;
  param_1 = 0x8876;
  iVar1->field_0x2 = &PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x40,param_5,param_3,param_4);
  iVar1->field_0x94 = puVar2;
  iVar1->field_0x96 = (puVar2 >> 0x10);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_7d5c(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x8876;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  unk_draw_op_1040_b0f8(param_1);
  return;
}


LRESULT  pass1_1038_7dac(param_1: u32,param_2: u16)

{
  LRESULT LVar1;
  
  pass1_1040_78de(param_1);
  LVar1 = send_dlg_item_msg_1038_844a(param_1,&PTR_LOOP_1050_1040,param_2);
  return LVar1;
}



void 
pass1_1038_7dc6(param_1: i16,param_2: u16,param_3: u16,param_4: u32,uchar *param_5,
               param_6: u16,param_7: u16,param_8: u16)

{
  let bVar1: bool;
  
  bVar1 = false;
  if (param_4._2_2_ == 0x1854) {
    if (param_4 != 0x1) goto LAB_1038_7e8c;
    send_dlg_item_msg_1038_8618(CONCAT22(param_2,param_1),param_8);
  }
  else {
    if (param_4 < 0x18550000) {
      if (param_4._2_2_ == 0xeb) {
        send_dlg_item_msg_1038_844a(CONCAT22(param_2,param_1),param_7,param_8);
      }
      else {
        if (param_4._2_2_ == 0xfb) {
          send_dlg_item_msg_1038_7eac(CONCAT22(param_2,param_1));
        }
        else {
          if (param_4._2_2_ != (s_vrpal_bmp_1050_183a + 0x7)) {
LAB_1038_7e77:
            pass1_1040_b54a(param_1,param_2,param_3,param_4,param_5,
                            &PTR_LOOP_1050_1040,param_8);
            return;
          }
          msg_box_op_1038_81be(CONCAT22(param_2,param_1),0x0,param_5,param_8);
        }
      }
      goto LAB_1038_7e8c;
    }
    if (param_4._2_2_ == 0x1855) {
      if (param_4 != 0x1) goto LAB_1038_7e8c;
      send_dlg_item_msg_1038_87b2(CONCAT22(param_2,param_1),param_7,param_8);
    }
    else {
      if (param_4._2_2_ == 0x1856) {
        if (param_4 != 0x1) goto LAB_1038_7e8c;
        pass1_1038_8810(CONCAT22(param_2,param_1),param_7,param_8);
      }
      else {
        if (param_4._2_2_ == 0x1858) {
          send_dlg_item_msg_1038_7fae(CONCAT22(param_2,param_1));
        }
        else {
          if (param_4._2_2_ != 0x1859) goto LAB_1038_7e77;
          pass1_1038_801a(CONCAT22(param_2,param_1),param_5,param_6,param_8);
        }
      }
    }
  }
  bVar1 = true;
LAB_1038_7e8c:
  if (bVar1) {
    set_win_text_1038_8358(CONCAT22(param_2,param_1),param_7,param_8);
    enable_win_1038_806a(CONCAT22(param_2,param_1),param_7);
  }
  return;
}


fn pass1_1038_801a(param_1: u32,uchar *param_2,param_3: i16,param_4: u16) -> u32

{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let puVar4: *mut u16;
  char *pcVar5;
  let uVar6: u32;
  
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x30,param_4,param_2,param_3);
  uVar3 = (param_1 >> 0x10);
  uVar2 = param_1;
  pcVar5 = pass1_1008_b340((uVar2 + 0x94));
  uVar1 = (pcVar5 >> 0x10) | pcVar5;
  uVar6 = pcVar5 & 0xffff | uVar1 << 0x10;
  if (pcVar5 != 0x0) {
    pass1_1010_3770(puVar4,pcVar5,uVar1);
    uVar6 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,(uVar2 + 0x6),0x3,uVar1,uVar2,
                            0x1010,param_4);
  }
  return uVar6;
}


fn pass1_1038_8810(param_1: u32,param_2: u16,param_3: u16)
{
  let uVar1: u16;
  let uVar2: u16;
  let local_102: [u8;100];
  
  uVar2 = (param_1 >> 0x10);
  uVar1 = send_dlg_item_msg_1038_8164
                    (param_1,uVar2,CONCAT22(param_3,local_102),0x1856,
                     param_2);
  if (uVar1 != 0x0) {
    pass1_1008_b63a((param_1 + 0x94),CONCAT22(param_3,local_102));
  }
  return;
}



fn pass1_1038_8848(void)
{
  return;
}



fn pass1_1038_884c(void)
{
  return;
}



astruct_18 *  pass1_1038_8850(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_7d5c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_88f2(astruct_57 *param_1,param_2: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  struct_1040_b082(param_1,CONCAT22(param_2,0x184c));
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x94) = _PTR_LOOP_1050_5a68;
  (iVar1 + 0x98) = 0x0;
  (iVar1 + 0x9a) = 0x0;
  (iVar1 + 0x9c) = 0x0;
  (iVar1 + 0x9e) = 0x0;
  param_1 = 0x8c2e;
  (iVar1 + 0x2) = &PTR_LOOP_1050_1038;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_893a(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x8c2e;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  unk_draw_op_1040_b0f8(param_1);
  return;
}



fn pass1_1038_8966(param_1: u32,param_2: u16,param_3: u16,param_4: i16,HWND16 param_5) -> u16

{
  let piVar1: *mut i16;
  let bVar2: bool;
  let iVar3: i16;
  let uVar4: u16;
  
  bVar2 = false;
  iVar3 = param_1;
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
  bVar2 = true;
LAB_1038_89af:
  if (bVar2) {
    SetDlgItemInt16(param_5,0x0,(iVar3 + 0x9a),
                    (bool)(s_dibtext_bmp_1050_1844 + 0x9));
    SetDlgItemInt16((HWND16)s_tile2_bmp_1050_1538,0x0,(iVar3 + 0x98),
                    (bool)(s_dibtext_bmp_1050_1844 + 0xb));
  }
  return 0x0;
}



fn pass1_1038_89e8(param_1: u32,param_2: u16)
{
  send_dlg_item_msg_1038_8b58(param_1,param_2);
  return;
}



void 
pass1_1038_89f8(param_1: i16,param_2: u16,param_3: u16,param_4: u32,uchar *param_5,
               param_6: u16)

{
  if (param_4._2_2_ == 0xeb) {
    send_dlg_item_msg_1038_8b58(CONCAT22(param_2,param_1),param_6);
  }
  else {
    if (param_4._2_2_ != (s_vrpal_bmp_1050_183a + 0x7)) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4,param_5,&PTR_LOOP_1050_1040,
                      param_6);
      return;
    }
    msg_box_ui_op_1038_8a3a(CONCAT22(param_2,param_1),0x0,param_5,param_6);
  }
  return;
}


astruct_18 *  pass1_1038_8c08(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_893a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * 
pass1_1038_8caa(astruct_57 *param_1,param_2: u16,uchar *param_3,param_4: i16,
               param_5: u16)

{
  astruct_704 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  struct_1040_b082(param_1,CONCAT22(param_2,0x185a));
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_704 *)param_1;
  &iVar1->field_0x94 = 0x0;
  param_1 = 0x90c8;
  iVar1->field_0x2 = &PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3f,param_5,param_3,param_4);
  iVar1->field_0x94 = puVar2;
  iVar1->field_0x96 = (puVar2 >> 0x10);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_8cf6(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0x90c8;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  unk_draw_op_1040_b0f8(param_1);
  return;
}


LRESULT  pass1_1038_8d7e(param_1: u32,param_2: u16)

{
  LRESULT LVar1;
  
  pass1_1040_78de(param_1);
  LVar1 = send_dlg_item_msg_1038_8f74(param_1,&PTR_LOOP_1050_1040,param_2);
  return LVar1;
}



void 
pass1_1038_8d98(param_1: i16,param_2: u16,param_3: u16,param_4: u32,uchar *param_5,
               param_6: u16,param_7: u16)

{
  if (param_4._2_2_ == 0xeb) {
    send_dlg_item_msg_1038_8f74(CONCAT22(param_2,param_1),param_6,param_7);
  }
  else {
    if (param_4._2_2_ != (s_vrpal_bmp_1050_183a + 0x7)) {
      pass1_1040_b54a(param_1,param_2,param_3,param_4,param_5,&PTR_LOOP_1050_1040,
                      param_7);
      return;
    }
    msg_box_op_1038_8dda(CONCAT22(param_2,param_1),0x0,param_5,param_7);
  }
  return;
}


astruct_18 *  pass1_1038_90a2(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_8cf6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_9144(param_1: *mut u16,param_2: u16,param_3: u16)
{
  let uVar1: u32;
  let uVar2: u16;
  let in_DX: *mut u8
  let puVar3: *mut u8
  let puVar4: *mut u8
  let iVar5: i16;
  let iVar6: i16;
  let unaff_DI: i16;
  let uVar7: u16;
  let uVar8: u16;
  let puVar9: *mut u16;
  let piStack8: *mut i16;
  astruct_432 *iVar8;
  
  struct_1040_b082((astruct_57 *)param_1,CONCAT22(param_2,0xfaa));
  uVar7 = (param_1 >> 0x10);
  iVar5 = param_1;
  (iVar5 + 0x94) = 0x0;
  (iVar5 + 0x96) = 0x0;
  (iVar5 + 0x98) = 0x0;
  *param_1 = 0x99a2;
  (iVar5 + 0x2) = &PTR_LOOP_1050_1038;
  (iVar5 + 0x8a) = 0x27;
  puVar9 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x28,param_3,in_DX,unaff_DI);
  puVar3 = (puVar9 >> 0x10);
  uVar2 = puVar9;
  (iVar5 + 0x98) = uVar2;
  *(uchar **)(iVar5 + 0x9a) = puVar3;
  mem_op_1000_179c(0x18,puVar3,0x1000);
  puVar4 = (puVar3 | uVar2);
  if (puVar4 == 0x0) {
    (iVar5 + 0x90) = 0x0;
  }
  else {
    struct_1040_a598((u16 *)CONCAT22(puVar3,uVar2));
    (iVar5 + 0x90) = uVar2;
    *(uchar **)(iVar5 + 0x92) = puVar4;
  }
  (iVar5 + 0x90) = 0x11;
  iVar6 = **(i16 **)(iVar5 + 0x90);
  uVar2 = iVar6 * 0xa + 0x2;
  mem_op_1000_179c(uVar2,puVar4,0x1000);
  piStack8 = (i16 *)CONCAT22(puVar4,uVar2);
  if ((puVar4 | uVar2) == 0x0) {
    uVar1 = (iVar5 + 0x90);
    (uVar1 + 0x2) = 0x0;
  }
  else {
    *piStack8 = iVar6;
    pass1_1000_5586((uchar *)0xa564,&PTR_LOOP_1050_1040,iVar6,0xa,uVar2 + 0x2,
                    puVar4);
    uVar1 = (iVar5 + 0x90);
    uVar8 = (uVar1 >> 0x10);
    iVar6 = uVar1;
    (iVar6 + 0x2) = uVar2 + 0x2;
    *(uchar **)(iVar6 + 0x4) = puVar4;
  }
  uVar1 = (iVar5 + 0x90);
  (uVar1 + 0xa) = 0x18;
  uVar1 = (iVar5 + 0x90);
  (uVar1 + 0x12) = (iVar5 + 0xa);
  return;
}



fn pass1_1038_927c(param_1: *mut u32)
{
  code **ppcVar1;
  
  ppcVar1 = (code **)(*param_1 + 0x74);
  (**ppcVar1)();
  return;
}


i16  pass1_1038_993a(param_1: u16,param_2: u16,param_3: i16)

{
  let iStack6: i16;
  
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



astruct_18 *  pass1_1038_997c(astruct_18 *param_1,param_2: u8)

{
  unk_draw_op_1040_b0f8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1038_9a1e(param_1: i16,param_2: u16,param_3: u16,param_4: u32) -> u16

{
  pass1_1040_b040((astruct_57 *)CONCAT22(param_2,param_1),CONCAT22(param_4,param_3),
                  (param_4 >> 0x10));
  CONCAT22(param_2,param_1) = 0x9af6;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  return CONCAT22(param_2,param_1);
}



fn pass1_1038_9a48(astruct_18 *param_1)
{
  param_1->field_0x0 = 0x9af6;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  unk_draw_op_1040_b0f8(param_1);
  return;
}


ULONG  pass1_1038_9ad0(Uparam_1: i32,param_2: u8)

{
  pass1_1038_9a48((astruct_18 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1038_9b72(param_1: i16,param_2: u16,param_3: u16,param_4: u32) -> u32

{
  let iStack4: i16;
  
  pass1_1040_b040((astruct_57 *)CONCAT22(param_2,param_1),CONCAT22(param_4,param_3),
                  (param_4 >> 0x10));
  (param_1 + 0x128) = 0x0;
  CONCAT22(param_2,param_1) = 0x9efa;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  iStack4 = 0x0;
  do {
    (param_1 + iStack4 * 0x2 + 0x94) = 0x0;
    iStack4 += 0x1;
  } while (iStack4 < 0x4a);
  return CONCAT22(param_2,param_1);
}


astruct_18 *  pass1_1038_9ed4(astruct_18 *param_1,param_2: u8)

{
  unk_draw_op_1040_b0f8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_57 * 
pass1_1038_9f76(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16)

{
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfba,param_5);
  param_1 = 0xa0b6;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_9fa4(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xa0b6;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&PTR_LOOP_1050_1040);
  return;
}


astruct_18 *  pass1_1038_a090(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_9fa4(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1038_a122(param_1: i16,param_2: u16,param_3: u16,param_4: u32,param_5: u32) -> u16

{
  get_sys_metrics_1040_7728
            ((astruct_57 *)CONCAT22(param_2,param_1),param_3,param_4,param_5,
             (param_5 >> 0x10));
  (param_1 + 0x8e) = 0x0;
  CONCAT22(param_2,param_1) = 0xa2d0;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  return CONCAT22(param_2,param_1);
}



fn pass1_1038_a156(astruct_18 *param_1)
{
  param_1->field_0x0 = 0xa2d0;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  ui_cleanup_op_1040_782c(param_1,&PTR_LOOP_1050_1040);
  return;
}



fn pass1_1038_a174(param_1: u32,param_2: i16)
{
  if (param_2 == 0x1) {
    (param_1 + 0x8e) = 0x0;
  }
  return;
}


astruct_18 *  pass1_1038_a2aa(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_a156(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1038_a33c(param_1: *mut u16,param_2: u16) -> u16

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  pass1_1038_a122(param_1,uVar1,0x1,0x0,CONCAT22(param_2,0xfc7));
  *param_1 = 0xa428;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_a36a(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xa428;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}


astruct_18 *  pass1_1038_a402(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_a36a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1038_a494(param_1: *mut u16,param_2: u16) -> u16

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  pass1_1038_a122(param_1,uVar1,0x1,0x0,CONCAT22(param_2,0xfc8));
  *param_1 = 0xa62e;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_a4c2(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xa62e;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}


astruct_18 *  pass1_1038_a608(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_a4c2(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1038_a69a(param_1: *mut u16,param_2: u16) -> u16

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  pass1_1038_a122(param_1,uVar1,0x1,0x0,CONCAT22(param_2,0xfc9));
  *param_1 = 0xa832;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_a6c8(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xa832;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}


astruct_18 *  pass1_1038_a80c(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_a6c8(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1038_a89e(param_1: *mut u16,param_2: u16) -> u16

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  pass1_1038_a122(param_1,uVar1,0x1,0x0,CONCAT22(param_2,0xfca));
  *param_1 = 0xab16;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_a8cc(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xab16;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  pass1_1038_a156(param_1);
  return;
}


astruct_18 *  pass1_1038_aaf0(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_a8cc(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_57 *  pass1_1038_ab82(astruct_57 *param_1,param_2: u16)

{
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfd3,param_2);
  param_1 = 0xad72;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_abb0(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xad72;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&PTR_LOOP_1050_1040);
  return;
}


astruct_18 *  pass1_1038_ad4c(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_abb0(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



fn pass1_1038_adde(param_1: i16,param_2: u16,param_3: u16,param_4: u32) -> u16

{
  pass1_1038_9b72(param_1,param_2,param_3,param_4);
  CONCAT22(param_2,param_1) = 0xae4e;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  return CONCAT22(param_2,param_1);
}



fn pass1_1038_ae08(astruct_18 *param_1)
{
  param_1->field_0x0 = 0xae4e;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  unk_draw_op_1040_b0f8(param_1);
  return;
}



astruct_18 *  pass1_1038_ae28(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_ae08(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_20 *  pass1_1038_aeca(astruct_20 *param_1,param_2: u16)

{
  let uVar1: u16;
  let local_b6: u16;
  let uStack180: u16;
  u8 local_5c [0x5a];
  
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0xac) = 0x0;
  (param_1 + 0xae) = 0x0;
  if (_PTR_LOOP_1050_5b7c == (astruct_20 *)0x0) {
    _PTR_LOOP_1050_5b7c = param_1;
  }
  pass1_1000_4906(param_1,(WNDCLASS16 *)0x0,0xac);
  unk_draw_op_1008_80ee((astruct_23 *)CONCAT22(param_2,local_5c),param_2);
  unk_win_ui_op_1040_9854((u16 *)CONCAT22(param_2,&local_b6),param_2);
  local_b6 = 0x389a;
  uStack180 = 0x1008;
  pass1_1008_8168((u16 *)CONCAT22(param_2,local_5c));
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_af34(void)
{
  _PTR_LOOP_1050_5b7c = 0x0;
  return;
}



u32 
pass1_1038_af40(param_1: u32,param_2: u16,param_3: i16,param_4: u16,param_5: u16,
               param_6: u16,param_7: u16)

{
  code **ppcVar1;
  let uVar2: u32;
  let puVar3: *mut u8;
  let puVar4: *mut u8
  let uVar5: u16;
  let iVar6: i16;
  let unaff_DI: i16;
  let uVar7: u16;
  astruct_57 *paVar8;
  
  puVar3 = bring_win_to_top_1038_b72e(param_1,param_3,param_6);
  iVar6 = param_1;
  uVar7 = (param_1 >> 0x10);
  if (puVar3 != 0x0) goto LAB_1038_b61f;
  PTR_LOOP_1050_5b82 = puVar3;
  if (true) {
    param_6 = &PTR_LOOP_1050_1038;
    switch(param_3) {
    case 0x1:
      param_6 = 0x1000;
      mem_op_1000_179c(0x8e,param_4,0x1000);
      if ((param_4 | param_5) == 0x0) {
LAB_1038_afa0:
        param_6 = 0x1000;
        paVar8 = (astruct_57 *)0x0;
      }
      else {
        paVar8 = pass1_1038_9f76((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,
                                 param_2);
      }
      break;
    case 0x2:
      mem_op_1000_179c(0x96,param_4,0x1000);
      uVar5 = param_4 | param_5;
      if (uVar5 == 0x0) goto LAB_1038_afa0;
      param_6 = &PTR_LOOP_1050_1040;
      pass1_1040_181c((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,uVar5,
                      param_7);
      paVar8 = (astruct_57 *)CONCAT22(uVar5,param_5);
      break;
    case 0x3:
      param_6 = 0x1000;
      mem_op_1000_179c(0x92,param_4,0x1000);
      if ((uchar *)(param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      paVar8 = pass1_1038_e99a((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2
                               ,(param_4 | param_5),param_7);
      break;
    case 0x4:
      param_6 = 0x1000;
      mem_op_1000_179c(0x92,param_4,0x1000);
      if ((uchar *)(param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      paVar8 = pass1_1038_c7b8((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2
                               ,(param_4 | param_5),param_7);
      break;
    case 0x5:
      mem_op_1000_179c(0x96,param_4,0x1000);
      uVar5 = param_4 | param_5;
      if (uVar5 == 0x0) goto LAB_1038_afa0;
      param_6 = &PTR_LOOP_1050_1040;
      pass1_1040_23ea((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,param_7,
                      uVar5);
      paVar8 = (astruct_57 *)CONCAT22(uVar5,param_5);
      break;
    case 0x6:
      mem_op_1000_179c(0x92,param_4,0x1000);
      if ((uchar *)(param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      param_6 = &PTR_LOOP_1050_1040;
      paVar8 = pass1_1040_06e8((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2
                               ,(param_4 | param_5),param_7);
      break;
    case 0x7:
      mem_op_1000_179c(0x9c,param_4,0x1000);
      puVar4 = (param_4 | param_5);
      if (puVar4 == 0x0) goto LAB_1038_afa0;
      param_6 = &PTR_LOOP_1050_1040;
      pass1_1040_4068((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,
                      unaff_DI,param_7);
      paVar8 = (astruct_57 *)CONCAT22(puVar4,param_5);
      break;
    case 0x8:
      param_6 = 0x1000;
      mem_op_1000_179c(0x9a,param_4,0x1000);
      puVar4 = (param_4 | param_5);
      if (puVar4 == 0x0) goto LAB_1038_afa0;
      pass1_1038_b772((astruct_57 *)CONCAT22(param_4,param_5),puVar4,unaff_DI,param_7,
                      param_2);
      paVar8 = (astruct_57 *)CONCAT22(puVar4,param_5);
      break;
    case 0x9:
      param_6 = 0x1000;
      mem_op_1000_179c(0x8e,param_4,0x1000);
      if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      paVar8 = pass1_1038_e140((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2
                              );
      break;
    case 0xa:
      param_6 = 0x1000;
      mem_op_1000_179c(0x90,param_4,0x1000);
      if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      paVar8 = (astruct_57 *)pass1_1038_a33c((u16 *)CONCAT22(param_4,param_5),param_2);
      break;
    case 0xb:
      param_6 = 0x1000;
      mem_op_1000_179c(0x90,param_4,0x1000);
      if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      paVar8 = (astruct_57 *)pass1_1038_a494((u16 *)CONCAT22(param_4,param_5),param_2);
      break;
    case 0xc:
      param_6 = 0x1000;
      mem_op_1000_179c(0x90,param_4,0x1000);
      if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      paVar8 = (astruct_57 *)pass1_1038_a69a((u16 *)CONCAT22(param_4,param_5),param_2);
      break;
    case 0xd:
      param_6 = 0x1000;
      mem_op_1000_179c(0x90,param_4,0x1000);
      if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      paVar8 = (astruct_57 *)pass1_1038_a89e((u16 *)CONCAT22(param_4,param_5),param_2);
      break;
    case 0xe:
      param_6 = 0x1000;
      mem_op_1000_179c(0x94,param_4,0x1000);
      puVar4 = (param_4 | param_5);
      if (puVar4 == 0x0) goto LAB_1038_afa0;
      pass1_1038_e69a((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,
                      unaff_DI,param_7);
      paVar8 = (astruct_57 *)CONCAT22(puVar4,param_5);
      break;
    case 0xf:
      param_6 = 0x1000;
      mem_op_1000_179c(0x94,param_4,0x1000);
      puVar4 = (param_4 | param_5);
      if (puVar4 == 0x0) goto LAB_1038_afa0;
      pass1_1038_cd06((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,
                      unaff_DI,param_7);
      paVar8 = (astruct_57 *)CONCAT22(puVar4,param_5);
      break;
    case 0x10:
      mem_op_1000_179c(0x92,param_4,0x1000);
      if ((uchar *)(param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      param_6 = &PTR_LOOP_1050_1040;
      paVar8 = pass1_1040_0bfc((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2
                               ,(param_4 | param_5),unaff_DI,param_7);
      break;
    case 0x11:
      mem_op_1000_179c(0x9a,param_4,0x1000);
      puVar4 = (param_4 | param_5);
      if (puVar4 == 0x0) goto LAB_1038_afa0;
      param_6 = &PTR_LOOP_1050_1040;
      pass1_1040_0e1c((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,param_2,puVar4,
                      unaff_DI,param_7);
      paVar8 = (astruct_57 *)CONCAT22(puVar4,param_5);
      break;
    case 0x12:
      param_6 = 0x1000;
      mem_op_1000_179c(0x9a,param_4,0x1000);
      if ((uchar *)(param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      paVar8 = pass1_1038_d756((astruct_57 *)CONCAT22(param_4,param_5),param_2,
                               (param_4 | param_5),unaff_DI,param_7);
      break;
    case 0x13:
      param_6 = 0x1000;
      mem_op_1000_179c(0x92,param_4,0x1000);
      if ((uchar *)(param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      paVar8 = pass1_1038_cad8((astruct_57 *)CONCAT22(param_4,param_5),param_2,
                               (param_4 | param_5),unaff_DI,param_7);
      break;
    case 0x14:
      mem_op_1000_179c(0xaa,param_4,0x1000);
      uVar5 = param_4 | param_5;
      if (uVar5 == 0x0) goto LAB_1038_afa0;
      param_6 = &PTR_LOOP_1050_1040;
      pass1_1040_1f5a((astruct_57 *)CONCAT22(param_4,param_5),param_2,unaff_DI,param_7);
      paVar8 = (astruct_57 *)CONCAT22(uVar5,param_5);
      break;
    case 0x15:
      param_6 = 0x1000;
      mem_op_1000_179c(0x8e,param_4,0x1000);
      if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      paVar8 = pass1_1038_d242((astruct_57 *)CONCAT22(param_4,param_5),param_2);
      break;
    case 0x16:
      param_6 = 0x1000;
      mem_op_1000_179c(0x9a,param_4,0x1000);
      puVar4 = (param_4 | param_5);
      if (puVar4 == 0x0) goto LAB_1038_afa0;
      pass1_1038_eeda((astruct_57 *)CONCAT22(param_4,param_5),param_2,puVar4,unaff_DI,
                      param_7);
      paVar8 = (astruct_57 *)CONCAT22(puVar4,param_5);
      break;
    case 0x17:
      mem_op_1000_179c(0x96,param_4,0x1000);
      if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      param_6 = 0x1018;
      paVar8 = pass1_1018_5e26((astruct_57 *)CONCAT22(param_4,param_5),param_2);
      break;
    default:
      goto switchD_1038_b581_caseD_18;
    case 0x19:
      mem_op_1000_179c(0x96,param_4,0x1000);
      puVar4 = (param_4 | param_5);
      if (puVar4 == 0x0) goto LAB_1038_afa0;
      param_6 = &PTR_LOOP_1050_1040;
      pass1_1040_1cb4((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,
                      unaff_DI,param_7);
      paVar8 = (astruct_57 *)CONCAT22(puVar4,param_5);
      break;
    case 0x1a:
      mem_op_1000_179c(0x92,param_4,0x1000);
      if ((uchar *)(param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      param_6 = &PTR_LOOP_1050_1040;
      paVar8 = pass1_1040_123e((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2
                               ,(param_4 | param_5),unaff_DI,param_7);
      break;
    case 0x1b:
      param_6 = 0x1000;
      mem_op_1000_179c(0x8e,param_4,0x1000);
      if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      paVar8 = pass1_1038_ab82((astruct_57 *)CONCAT22(param_4,param_5),param_2);
      break;
    case 0x1c:
      param_6 = 0x1000;
      mem_op_1000_179c(0x92,param_4,0x1000);
      if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      paVar8 = pass1_1038_e2d0((astruct_57 *)CONCAT22(param_4,param_5),param_2);
      break;
    case 0x1d:
      param_6 = 0x1000;
      mem_op_1000_179c(0x92,param_4,0x1000);
      if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      paVar8 = pass1_1038_eb9e((astruct_57 *)CONCAT22(param_4,param_5),param_2);
      break;
    case 0x1e:
      param_6 = 0x1000;
      mem_op_1000_179c(0x29e,param_4,0x1000);
      puVar4 = (param_4 | param_5);
      if (puVar4 == 0x0) goto LAB_1038_afa0;
      pass1_1038_bddc((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,
                      unaff_DI,param_7);
      paVar8 = (astruct_57 *)CONCAT22(puVar4,param_5);
      break;
    case 0x1f:
      param_6 = 0x1000;
      mem_op_1000_179c(0x9a,param_4,0x1000);
      puVar4 = (param_4 | param_5);
      if (puVar4 == 0x0) goto LAB_1038_afa0;
      pass1_1038_c4a2((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,
                      unaff_DI,param_7);
      paVar8 = (astruct_57 *)CONCAT22(puVar4,param_5);
      break;
    case 0x20:
      mem_op_1000_179c(0x29a,param_4,0x1000);
      puVar4 = (param_4 | param_5);
      if (puVar4 == 0x0) goto LAB_1038_afa0;
      param_6 = &PTR_LOOP_1050_1040;
      pass1_1040_2ea2((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,
                      unaff_DI,param_7);
      paVar8 = (astruct_57 *)CONCAT22(puVar4,param_5);
      break;
    case 0x21:
      mem_op_1000_179c(0xa6,param_4,0x1000);
      puVar4 = (param_4 | param_5);
      if (puVar4 == 0x0) goto LAB_1038_afa0;
      param_6 = &PTR_LOOP_1050_1040;
      pass1_1040_3966((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,
                      unaff_DI,param_7);
      paVar8 = (astruct_57 *)CONCAT22(puVar4,param_5);
      break;
    case 0x22:
      mem_op_1000_179c(0x9a,param_4,0x1000);
      puVar4 = (param_4 | param_5);
      if (puVar4 == 0x0) goto LAB_1038_afa0;
      param_6 = &PTR_LOOP_1050_1040;
      pass1_1040_34a2((astruct_57 *)CONCAT22(param_4,param_5),0x0,0x0,0x0,param_2,puVar4,
                      unaff_DI,param_7);
      paVar8 = (astruct_57 *)CONCAT22(puVar4,param_5);
      break;
    case 0x23:
      mem_op_1000_179c(0x9c,param_4,0x1000);
      puVar4 = (param_4 | param_5);
      if (puVar4 == 0x0) goto LAB_1038_afa0;
      param_6 = &PTR_LOOP_1050_1040;
      pass1_1040_ac84((astruct_57 *)CONCAT22(param_4,param_5),param_2,puVar4,unaff_DI,
                      param_7);
      paVar8 = (astruct_57 *)CONCAT22(puVar4,param_5);
      break;
    case 0x25:
      mem_op_1000_179c(0xa0,param_4,0x1000);
      puVar4 = (param_4 | param_5);
      if (puVar4 == 0x0) goto LAB_1038_afa0;
      param_6 = &PTR_LOOP_1050_1040;
      pass1_1040_ca16((astruct_57 *)CONCAT22(param_4,param_5),param_2,puVar4,unaff_DI,
                      param_7);
      paVar8 = (astruct_57 *)CONCAT22(puVar4,param_5);
      break;
    case 0x26:
      mem_op_1000_179c(0xa2,param_4,0x1000);
      uVar5 = param_4 | param_5;
      if (uVar5 == 0x0) goto LAB_1038_afa0;
      param_6 = &PTR_LOOP_1050_1040;
      pass1_1040_d0f8((astruct_57 *)CONCAT22(param_4,param_5),param_2);
      paVar8 = (astruct_57 *)CONCAT22(uVar5,param_5);
      break;
    case 0x27:
      param_6 = 0x1000;
      mem_op_1000_179c(0xa0,param_4,0x1000);
      uVar5 = param_4 | param_5;
      if (uVar5 == 0x0) goto LAB_1038_afa0;
      pass1_1038_88f2((astruct_57 *)CONCAT22(param_4,param_5),param_2);
      paVar8 = (astruct_57 *)CONCAT22(uVar5,param_5);
      break;
    case 0x28:
      mem_op_1000_179c(0x96,param_4,0x1000);
      puVar4 = (param_4 | param_5);
      if (puVar4 == 0x0) goto LAB_1038_afa0;
      param_6 = &PTR_LOOP_1050_1040;
      pass1_1040_6402((astruct_57 *)CONCAT22(param_4,param_5),param_2,puVar4,unaff_DI,
                      param_7);
      paVar8 = (astruct_57 *)CONCAT22(puVar4,param_5);
      break;
    case 0x29:
      param_6 = 0x1000;
      mem_op_1000_179c(0x98,param_4,0x1000);
      if ((param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      paVar8 = pass1_1038_7d10((astruct_57 *)CONCAT22(param_4,param_5),param_2,
                               param_4 | param_5,unaff_DI,param_7);
      break;
    case 0x2a:
      param_6 = 0x1000;
      mem_op_1000_179c(0x98,param_4,0x1000);
      if ((uchar *)(param_4 | param_5) == 0x0) goto LAB_1038_afa0;
      paVar8 = pass1_1038_8caa((astruct_57 *)CONCAT22(param_4,param_5),param_2,
                               (param_4 | param_5),unaff_DI,param_7);
    }
    (param_3 * 0x4 + iVar6) = paVar8;
    (param_3 * 0x4 + iVar6 + 0x2) = (paVar8 >> 0x10);
  }
switchD_1038_b581_caseD_18:
  if (*(long *)(param_3 * 0x4 + iVar6) != 0x0) {
    if ((iVar6 + 0xae) != 0x0) {
      uVar2 = (param_3 * 0x4 + iVar6);
      (uVar2 + 0x6e) = (iVar6 + 0xae);
    }
    (iVar6 + 0xae) = 0x0;
    uVar2 = (param_3 * 0x4 + iVar6);
    ppcVar1 = (code **)((param_3 * 0x4 + iVar6) + 0x8);
    (**ppcVar1)(param_6,uVar2,(uVar2 >> 0x10));
  }
LAB_1038_b61f:
  return CONCAT22((param_3 * 0x4 + iVar6 + 0x2),
                  (param_3 * 0x4 + iVar6));
}


fn pass1_1038_b6e0(param_1: u32,param_2: i16)
{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  let uStack4: u16;
  
  uStack4 = 0x1;
  while( true ) {
    if (0x2a < uStack4) {
      return;
    }
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((((uStack4 * 0x4 + iVar2 + 0x2) | (uStack4 * 0x4 + iVar2)) !=
         0x0) && (uVar1 = (uStack4 * 0x4 + iVar2),
                 (uVar1 + 0x6) == param_2)) break;
    uStack4 += 0x1;
  }
  (uStack4 * 0x4 + iVar2) = 0x0;
  return;
}


void 
pass1_1038_b772(astruct_57 *param_1,uchar *param_2,param_3: i16,param_4: u16,
               param_5: u16)

{
  let puVar1: *mut u8
  astruct_705 *iVar2;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x9a,0x0,0xfbf,param_5);
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_705 *)param_1;
  &iVar2->field_0x8e = 0x0;
  &iVar2->field_0x92 = 0x0;
  iVar2->field_0x96 = 0x1;
  iVar2->field_0x98 = 0x0;
  param_1 = 0xbd70;
  iVar2->field_0x2 = &PTR_LOOP_1050_1038;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x36,param_4,param_2,param_3);
  puVar1 = (puVar3 >> 0x10);
  iVar2->field_0x8e = puVar3;
  iVar2->field_0x90 = puVar1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x6,param_4,puVar1,param_3);
  iVar2->field_0x92 = puVar3;
  iVar2->field_0x94 = (puVar3 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_b7f0(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xbd70;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&PTR_LOOP_1050_1040);
  return;
}


fn pass1_1038_bca8(param_1: u32)
{
  let uVar1: u16;
  code **ppcVar2;
  let uVar3: u32;
  let puVar4: u32;
  let puVar5: u32;
  let extraout_DX: *mut u8
  let puVar6: *mut u8
  let extraout_DX_00: *mut u8
  let puVar7: *mut u8
  let iVar8: i16;
  let uVar9: u16;
  
  uVar9 = (param_1 >> 0x10);
  iVar8 = param_1;
  uVar3 = (iVar8 + 0x8e);
  puVar5 = (uVar3 + 0xa);
  ppcVar2 = (code **)(*puVar5 + 0x14);
  (**ppcVar2)();
  puVar4 = puVar5;
  puVar6 = extraout_DX;
  if (*(long *)(iVar8 + 0x70) != 0x0) {
    puVar4 = (iVar8 + 0x70);
    uVar1 = (iVar8 + 0x72);
    puVar6 = (uVar1 | puVar4);
    if (puVar6 != 0x0) {
      ppcVar2 = (code **)*puVar4;
      (**ppcVar2)();
      puVar6 = extraout_DX_00;
    }
  }
  mem_op_1000_179c(0x14,puVar6,0x1000);
  puVar7 = (puVar6 | puVar4);
  if (puVar7 == 0x0) {
    puVar4 = 0x0;
    puVar7 = 0x0;
  }
  else {
    struct_1008_4c58((u16 *)CONCAT22(puVar6,puVar4));
  }
  (iVar8 + 0x70) = puVar4;
  *(uchar **)(iVar8 + 0x72) = puVar7;
  pass1_1008_4d84(*(astruct_90 **)(iVar8 + 0x70),
                  puVar5 & 0xffff | ZEXT24(extraout_DX) << 0x10,puVar7);
  return;
}



astruct_18 *  pass1_1038_bd4a(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_b7f0(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_bddc(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16,uchar *param_6,param_7: i16,param_8: u16)

{
  astruct_706 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0x176,param_5);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_706 *)param_1;
  &iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x94 = 0x0;
  iVar1->field_0x96 = 0x0;
  iVar1->field_0x98 = 0x0;
  iVar1->field_0x9a = 0x0;
  iVar1->field_0x9c = 0x0;
  param_1 = 0xc436;
  iVar1->field_0x2 = &PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3b,param_8,param_6,param_7);
  iVar1->field_0x8e = puVar2;
  iVar1->field_0x90 = (puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_be4a(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xc436;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&PTR_LOOP_1050_1040);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_be76(param_1: u16,param_2: u32,uchar *param_3,param_4: i16,param_5: u16)
{
  let puVar1: *mut u16;
  let iVar2: i16;
  
  if (param_2._2_2_ == 0x0) {
    iVar2 = 0x0;
    puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,param_3,param_4);
    pass1_1010_038e(puVar1,iVar2,param_5);
  }
  destroy_win_1040_7b98(CONCAT22(param_2,param_1),&PTR_LOOP_1050_1040);
  return;
}


astruct_18 *  pass1_1038_c410(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_be4a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_c4a2(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16,uchar *param_6,param_7: i16,param_8: u16)

{
  astruct_708 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0x17c,param_5);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_708 *)param_1;
  &iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x96 = 0x0;
  param_1 = 0xc74c;
  iVar1->field_0x2 = &PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3b,param_8,param_6,param_7);
  iVar1->field_0x8e = puVar2;
  iVar1->field_0x90 = (puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_c4fe(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xc74c;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&PTR_LOOP_1050_1040);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_c52a(param_1: u16,param_2: u32,uchar *param_3,param_4: i16,param_5: u16)
{
  let puVar1: *mut u16;
  let iVar2: i16;
  
  if (param_2._2_2_ == 0x0) {
    iVar2 = 0x0;
    puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,param_3,param_4);
    pass1_1010_038e(puVar1,iVar2,param_5);
  }
  destroy_win_1040_7b98(CONCAT22(param_2,param_1),&PTR_LOOP_1050_1040);
  return;
}


astruct_18 *  pass1_1038_c726(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_c4fe(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * 
pass1_1038_c7b8(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16,uchar *param_6,param_7: u16)

{
  astruct_435 *iVar1;
  let unaff_DI: i16;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfb8,param_5);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_435 *)param_1;
  &iVar1->field_0x8e = 0x0;
  param_1 = 0xca6c;
  iVar1->field_0x2 = &PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x5,param_7,param_6,unaff_DI);
  iVar1->field_0x8e = puVar2;
  iVar1->field_0x90 = (puVar2 >> 0x10);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_c80a(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xca6c;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&PTR_LOOP_1050_1040);
  return;
}


astruct_18 *  pass1_1038_ca46(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_c80a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * 
pass1_1038_cad8(astruct_57 *param_1,param_2: u16,uchar *param_3,param_4: i16,
               param_5: u16)

{
  astruct_709 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x1cb,param_2);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_709 *)param_1;
  &iVar1->field_0x8e = 0x0;
  param_1 = 0xcc9a;
  iVar1->field_0x2 = &PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2c,param_5,param_3,param_4);
  iVar1->field_0x8e = puVar2;
  iVar1->field_0x90 = (puVar2 >> 0x10);
  iVar1->field_0x74 = 0x0;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_cb30(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xcc9a;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&PTR_LOOP_1050_1040);
  return;
}


astruct_18 *  pass1_1038_cc74(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_cb30(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_cd06(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16,uchar *param_6,param_7: i16,param_8: u16)

{
  astruct_710 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfcc,param_5);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_710 *)param_1;
  &iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  param_1 = 0xcf00;
  iVar1->field_0x2 = &PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x42,param_8,param_6,param_7);
  iVar1->field_0x8e = puVar2;
  iVar1->field_0x90 = (puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_cd5c(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xcf00;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&PTR_LOOP_1050_1040);
  return;
}


astruct_18 *  pass1_1038_ceda(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_cd5c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}


astruct_18 * 
pass1_1038_d218(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  free_proc_inst_1038_cfda((u16 *)param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_57 *  pass1_1038_d242(astruct_57 *param_1,param_2: u16)

{
  let uVar1: u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x13e,param_2);
  uVar1 = (param_1 >> 0x10);
  param_1 = 0xd6ea;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  (param_1 + 0x74) = 0x1;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_d276(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xd6ea;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&PTR_LOOP_1050_1040);
  return;
}


astruct_18 *  pass1_1038_d6c4(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_d276(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * 
pass1_1038_d756(astruct_57 *param_1,param_2: u16,uchar *param_3,param_4: i16,
               param_5: u16)

{
  code **ppcVar1;
  astruct_711 *iVar2;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x11b,param_2);
  uVar2 = (param_1 >> 0x10);
  iVar2 = (astruct_711 *)param_1;
  iVar2->field_0x8e = 0x0;
  iVar2->field_0x90 = 0x0;
  iVar2->field_0x92 = 0x0;
  iVar2->field_0x96 = 0x0;
  param_1 = 0xe0d4;
  iVar2->field_0x2 = &PTR_LOOP_1050_1038;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,param_3,param_4);
  &iVar2->field_0x92 = puVar3;
  (&iVar2->field_0x92 + 0x2) = (puVar3 >> 0x10);
  ppcVar1 = (code **)(*iVar2->field_0x92 + 0x4);
  (**ppcVar1)();
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_d7d0(astruct_18 *param_1,param_2: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1->field_0x0 = 0xe0d4;
  (iVar1 + 0x2) = &PTR_LOOP_1050_1038;
  if ((iVar1 + 0x90) != 0x0) {
    pass1_1010_1ea6(_PTR_LOOP_1050_02a0,(long)param_1,param_2);
  }
  if (*(long *)(iVar1 + 0x92) != 0x0) {
    pass1_1010_1ea6((iVar1 + 0x92),(long)param_1,param_2);
  }
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(iVar1 + 0x6));
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x96),0x1000);
  ui_cleanup_op_1040_782c(param_1,&PTR_LOOP_1050_1040);
  return;
}


void 
pass1_1038_de20(param_1: u32,param_2: u16,param_3: u16,param_4: i16,uchar *param_5,
               param_6: u16,param_7: u16)

{
  code **ppcVar1;
  let iVar2: i16;
  let puVar3: *mut u8
  let uVar4: u16;
  let local_12: [u8;4];
  let uStack14: u16;
  let puStack12: *mut u8
  let puStack10: u32;
  let uStack6: u16;
  let iStack4: i16;
  
  iStack4 = 0x644;
  uStack6 = 0x0;
  uStack14 = param_4 - 0x11cU;
  if (true) {
    uStack14 = param_6;
    switch(param_4 - 0x11cU) {
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
  }
  if (iStack4 != 0x0) {
    uVar4 = 0x1000;
    mem_op_1000_179c(0xb4,param_5,0x1000);
    puVar3 = (param_5 | uStack14);
    puStack12 = param_5;
    if (puVar3 == 0x0) {
      iVar2 = 0x0;
      puVar3 = 0x0;
    }
    else {
      uVar4 = SUB42(&PTR_LOOP_1050_1040,0x0);
      iVar2 = string_1040_8520((astruct_57 *)CONCAT22(param_5,uStack14),
                               (param_1 + 0x6),0x0,0x2,0x634,iStack4,
                               puVar3,param_7);
    }
    puStack10 = CONCAT22(puVar3,iVar2);
    if (uStack6 == 0x0) {
      ppcVar1 = (code **)(*puStack10 + 0x74);
      (**ppcVar1)(uVar4,iVar2,puVar3);
    }
    else {
      pass1_1008_941a((u16 *)CONCAT22(param_7,local_12),0x1,uStack6);
      ppcVar1 = (code **)(*puStack10 + 0x6c);
      (**ppcVar1)(0x1008,puStack10,(puStack10 >> 0x10),local_12,param_7)
      ;
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_df5c(param_1: u32,param_2: u16,param_3: u16) -> u32

{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u32;
  
  uVar2 = (param_1 >> 0x10);
  uVar1 = param_1;
  pass1_1010_038e((uVar1 + 0x92),0x1,param_3);
  uVar3 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,(uVar1 + 0x8),0x20,param_2,uVar1,
                          0x1010,param_3);
  return uVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_df86(param_1: u32,uchar *param_2,param_3: i16,param_4: u16)
{
  char *pcVar1;
  code **ppcVar2;
  let BVar3: bool;
  let uVar4: u16;
  let uVar5: u16;
  let puVar6: *mut u8
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u8;
  let puVar11: *mut u16;
  char *pcVar12;
  astruct_57 *paVar13;
  let puStack22: u32;
  
  puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,param_2,param_3);
  uVar5 = (puVar11 >> 0x10);
  pcVar1 = *(char **)(puVar11 + 0x68);
  uVar9 = (param_1 >> 0x10);
  uVar8 = param_1;
  BVar3 = pass1_1010_041a();
  if (BVar3 != 0x0) {
    pass1_1010_038e((uVar8 + 0x92),0x1,param_4);
    pass1_1038_af40(_PTR_LOOP_1050_5b7c,(uVar8 + 0x8),0x1e,uVar5,uVar8,0x1010,
                    param_4);
    return;
  }
  pcVar12 = load_string_1010_847e
                      (_PTR_LOOP_1050_14cc,
                       (_PTR_LOOP_1050_14cc >> 0x10),0x1010);
  puVar6 = (pcVar12 >> 0x10);
  uVar4 = pcVar12;
  uVar10 = 0x0;
  mem_op_1000_179c(0xb4,puVar6,0x1000);
  if ((puVar6 | uVar4) == 0x0) {
    uVar9 = 0x0;
    uVar7 = 0x0;
  }
  else {
    uVar10 = 0x40;
    paVar13 = pass1_1040_8478((astruct_57 *)CONCAT22(puVar6,uVar4),0x20,pcVar1,pcVar12,
                              (uVar8 + 0x6),puVar6 | uVar4);
    uVar7 = (paVar13 >> 0x10);
    uVar9 = SUB42(paVar13,0x0);
  }
  puStack22 = CONCAT22(uVar7,uVar9);
  ppcVar2 = (code **)(*puStack22 + 0x74);
  (**ppcVar2)(uVar10,uVar9,uVar7);
  return;
}



fn pass1_1038_e03e(param_1: u32)
{
  let uVar1: u32;
  let uVar2: u16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u32;
  let iStack6: i16;
  
  uVar4 = (param_1 >> 0x10);
  uVar2 = pass1_1010_0886();
  for (iStack6 = 0x1; iStack6 <= uVar2; iStack6 += 0x1) {
    uVar1 = (param_1 + 0x92);
    uVar6 = pass1_1010_08e2(uVar1,(uVar1 >> 0x10),iStack6);
    uVar1 = (param_1 + 0x96);
    uVar5 = (uVar1 >> 0x10);
    iVar3 = uVar1;
    if (*(long *)(iVar3 + iStack6 * 0x4) != 0x0) {
      enable_win_1040_9234
                ((iVar3 + iStack6 * 0x4),*(bool *)(uVar6 + 0x6),
                 &PTR_LOOP_1050_1040);
    }
  }
  return;
}



astruct_18 * 
pass1_1038_e0ae(astruct_18 *param_1,param_2: u8,param_3: u16)

{
  pass1_1038_d7d0(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_57 * 
pass1_1038_e140(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16)

{
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfc2,param_5);
  param_1 = 0xe264;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_e16e(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xe264;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&PTR_LOOP_1050_1040);
  return;
}



astruct_18 *  pass1_1038_e23e(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_e16e(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_57 *  pass1_1038_e2d0(astruct_57 *param_1,param_2: u16)

{
  let uVar1: u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x1c3,param_2);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x8e) = 0x0;
  param_1 = 0xe62e;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_e308(astruct_18 *param_1)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1->field_0x0 = 0xe62e;
  (iVar1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(iVar1 + 0x6));
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x8e),0x1000);
  ui_cleanup_op_1040_782c(param_1,&PTR_LOOP_1050_1040);
  return;
}


void 
pass1_1038_e4bc(param_1: u16,param_2: u32,param_3: u32,uchar *param_4,param_5: i16,
               param_6: u16)

{
  code **ppcVar1;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let puVar6: *mut u8
  let extraout_DX: u16;
  let extraout_DX_00: *mut u8
  let puVar7: *mut u8
  code **ppcVar8;
  let puVar9: u32;
  let puVar10: *mut u16;
  let uVar11: u16;
  let uVar12: u8;
  let uVar13: u8;
  let uVar14: u16;
  let uVar15: u16;
  let uVar16: u16;
  let puStack22: u32;
  
  if (param_3._2_2_ == 0x1c4) {
    puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,param_4,param_5);
    uVar14 = (puVar10 >> 0x10);
    uVar4 = (puVar10 + 0x24);
    uVar5 = (puVar10 + 0x26);
    uVar3 = uVar5 | uVar4;
    if (uVar3 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4,uVar5);
      if ((uVar5 | uVar3) != 0x0) {
        puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x20);
        puVar6 = (puVar9 >> 0x10);
        uVar4 = puVar9;
        pass1_1038_4e78(uVar4,puVar6,CONCAT22(uVar5,uVar3),puVar9);
        puStack22 = CONCAT22(puVar6,uVar4);
        uVar2 = *puStack22;
        ppcVar8 = (code **)uVar2;
        ppcVar1 = ppcVar8 + 0x8;
        uVar5 = uVar4;
        (**ppcVar1)(0x1008,uVar4,puVar6);
        if ((extraout_DX | uVar5) == 0x0) {
          if (puStack22 != 0x0) {
            ppcVar1 = ppcVar8;
            (**ppcVar1)(0x1008,uVar4,puVar6,0x1);
          }
        }
        else {
          ppcVar1 = (code **)(*puStack22 + 0x4);
          (**ppcVar1)(0x8,uVar4,puVar6,0x0,0x0);
          puVar7 = extraout_DX_00;
          pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar5,extraout_DX_00);
          puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_6,puVar7,
                                    (uVar2 >> 0x10));
          pass1_1010_71d6(puVar10,0x1,

                          ((ZEXT24(puVar7) & 0xff00) << 0x10 |
                          CONCAT12((char)puVar7,uVar5 + 0xc)),uVar5 + 0xc,
                          (puVar10 >> 0x10),param_6);
          if (puStack22 != 0x0) {
            ppcVar1 = (code **)*puStack22;
            (**ppcVar1)(0x1010,uVar4,puVar6,0x1);
          }
        }
      }
    }
  }
  else {
    if (param_3._2_2_ == 0x1c5) {
      uVar14 = 0xe;
    }
    else {
      if (param_3._2_2_ != 0x1c6) {
        post_win_msg_1040_7b3c
                  (CONCAT13((char)(param_2 >> 0x8),
                                     CONCAT12((char)param_2,param_1)),
                   (param_2 >> 0x10),param_3,param_3._2_2_,
                   &PTR_LOOP_1050_1040);
        return;
      }
      uVar14 = 0xd;
    }
    uVar16 = 0x0;
    uVar15 = 0x0;
    uVar11 = 0x0;
    uVar12 = 0x0;
    uVar13 = 0x0;
    puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_6,param_4,param_5);
    unk_win_op_1010_7300
              (puVar10,CONCAT13(uVar13,CONCAT12(uVar12,uVar11)),uVar14,
               CONCAT22(uVar16,uVar15));
  }
  return;
}



astruct_18 *  pass1_1038_e608(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_e308(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_e69a(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16,uchar *param_6,param_7: i16,param_8: u16)

{
  astruct_713 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfcb,param_5);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_713 *)param_1;
  &iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  param_1 = 0xe92e;
  iVar1->field_0x2 = &PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x43,param_8,param_6,param_7);
  iVar1->field_0x8e = puVar2;
  iVar1->field_0x90 = (puVar2 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_e6f0(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xe92e;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&PTR_LOOP_1050_1040);
  return;
}



astruct_18 *  pass1_1038_e908(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_e6f0(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * 
pass1_1038_e99a(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16,uchar *param_6,param_7: u16)

{
  astruct_434 *iVar1;
  let unaff_DI: i16;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfb9,param_5);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_434 *)param_1;
  &iVar1->field_0x8e = 0x0;
  param_1 = 0xeb32;
  iVar1->field_0x2 = &PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x30,param_7,param_6,unaff_DI);
  iVar1->field_0x8e = puVar2;
  iVar1->field_0x90 = (puVar2 >> 0x10);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_e9ec(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xeb32;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&PTR_LOOP_1050_1040);
  return;
}


astruct_18 *  pass1_1038_eb0c(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_e9ec(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



astruct_57 *  pass1_1038_eb9e(astruct_57 *param_1,param_2: u16)

{
  let uVar1: u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x1c7,param_2);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x8e) = 0x0;
  param_1 = 0xee6e;
  (param_1 + 0x2) = &PTR_LOOP_1050_1038;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1038_ebd6(astruct_18 *param_1)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  param_1->field_0x0 = 0xee6e;
  (iVar1 + 0x2) = &PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(iVar1 + 0x6));
  fn_ptr_1000_17ce(*(astruct_18 **)(iVar1 + 0x8e),0x1000);
  ui_cleanup_op_1040_782c(param_1,&PTR_LOOP_1050_1040);
  return;
}


astruct_18 *  pass1_1038_ee48(astruct_18 *param_1,param_2: u8)

{
  pass1_1038_ebd6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1038_eeda(astruct_57 *param_1,param_2: u16,uchar *param_3,param_4: i16,
               param_5: u16)

{
  astruct_714 *iVar1;
  let uVar1: u16;
  let puVar2: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,0x166,param_2);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_714 *)param_1;
  &iVar1->field_0x8e = 0x0;
  iVar1->field_0x92 = 0x0;
  iVar1->field_0x94 = 0x0;
  param_1 = 0x67c;
  iVar1->field_0x2 = &PTR_LOOP_1050_1040;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x9,param_5,param_3,param_4);
  iVar1->field_0x8e = puVar2;
  iVar1->field_0x90 = (puVar2 >> 0x10);
  iVar1->field_0x74 = 0x1;
  return;
}


astruct_31 *  pass1_1040_0656(astruct_31 *param_1,param_2: u8)

{
  destroy_win_1038_ef3a(param_1,&PTR_LOOP_1050_1038);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * 
pass1_1040_06e8(astruct_57 *param_1,param_2: u32,param_3: u16,param_4: u16,
               param_5: u16,uchar *param_6,param_7: u16)

{
  let iVar1: i16;
  let unaff_DI: i16;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  get_sys_metrics_1040_7728(param_1,0x1,param_2,0xfbc,param_5);
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x8e) = 0x0;
  param_1 = 0xb90;
  (iVar1 + 0x2) = &PTR_LOOP_1050_1040;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x7,param_7,param_6,unaff_DI);
  (iVar1 + 0x8e) = puVar3;
  (iVar1 + 0x90) = (puVar3 >> 0x10);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1040_073a(astruct_18 *param_1)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  param_1->field_0x0 = 0xb90;
  (param_1 + 0x2) = &PTR_LOOP_1050_1040;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,(param_1 + 0x6));
  ui_cleanup_op_1040_782c(param_1,&PTR_LOOP_1050_1038);
  return;
}








