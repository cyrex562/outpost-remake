


void __stdcall16far pass1_1038_0000(ulong param_1,uint param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  int iVar4;
  undefined4 *puVar5;
  undefined2 uVar6;
  undefined2 *puStack10;
  
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
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    *(undefined2 *)(param_2 + 0x2) = 0x1008;
    uVar6 = (undefined2)(param_1 >> 0x10);
    *(undefined4 *)(param_2 + 0x4) = *(undefined4 *)((int)param_1 + 0x4);
    puVar3 = (undefined4 *)((int)param_1 + 0x8);
    puVar5 = (undefined4 *)(param_2 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
      puVar2 = puVar5;
      puVar5 = puVar5 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
    *puStack10 = 0xb96;
    *(undefined2 *)(param_2 + 0x2) = (int)&PTR_LOOP_1050_1038;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_008e(ushort param_1,ushort param_2,ulong param_3,uchar *param_4,int param_5,ushort param_6)

{
  int iVar1;
  undefined4 uVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  uchar *puVar7;
  uchar *puVar8;
  int iVar9;
  undefined2 uVar10;
  ushort *puVar11;
  ushort *puVar12;
  int iStack32;
  int iStack12;
  undefined4 uVar6;
  
  uVar10 = (undefined2)(param_3 >> 0x10);
  iVar9 = (int)param_3;
  if (*(long *)(iVar9 + 0x4) != 0x4000001) {
    return;
  }
  puVar11 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2c,param_6,param_4,param_5);
  puVar7 = (uchar *)((ulong)puVar11 >> 0x10);
  uVar3 = (uint)puVar11;
  puVar8 = puVar7;
  uVar4 = uVar3;
  pass1_1008_612e(0x1,0x64,uVar3);
  iStack12 = 0x0;
  iVar1 = *(int *)(uVar3 + 0xa);
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
          if ((int)uVar4 < 0x32) {
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
    puVar8 = (uchar *)((ulong)puVar12 >> 0x10);
    pass1_1010_043a((ulong)puVar12 & 0xffff | ZEXT24(puVar8) << 0x10,*(long *)(iVar9 + 0x4),iStack12,param_6);
  }
  pass1_1008_eb74(puVar11,0x0,puVar8,param_5,param_6);
  if (((*(uint *)(uVar3 + 0xe) | *(uint *)(uVar3 + 0xc)) == 0x0) && (*(int *)(iVar9 + 0x18) < 0xc9)) {
    uVar2 = *_PTR_LOOP_1050_65e2;
    uVar4 = (uint)uVar2;
    uVar6 = uVar2;
    pass1_1008_612e(0x0,0x8,uVar4);
    uVar5 = (uint)uVar6;
    iStack32 = (int)((ulong)uVar2 >> 0x10);
    *(int *)(uVar3 + 0xc) = uVar5 + uVar4 + 0x1e;
    *(int *)(uVar3 + 0xe) = ((int)uVar5 >> 0xf) + iStack32 + (uint)CARRY2(uVar5,uVar4) + (uint)(0xffe1 < uVar5 + uVar4);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_01c0(ushort param_1,ushort param_2,ulong param_3,ushort param_4)

{
  int iVar1;
  undefined4 *puVar2;
  code **ppcVar3;
  ulong uVar4;
  ulong uVar5;
  BOOL16 BVar6;
  undefined *puVar7;
  undefined4 *puVar8;
  ulong uVar9;
  uchar *puVar10;
  undefined2 extraout_DX;
  uint extraout_DX_00;
  uint uVar11;
  uint uVar12;
  undefined2 uVar13;
  undefined2 uVar14;
  ulong *puVar15;
  ulong uVar16;
  ulong uVar17;
  undefined uVar18;
  ulong uStack50;
  ulong uStack30;
  ulong uStack18;
  undefined local_e [0x2];
  undefined4 *puStack12;
  uint uStack8;
  uchar *puStack6;
  int iStack4;
  
  iStack4 = 0x0;
  puVar15 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x29);
  puVar10 = (uchar *)((ulong)puVar15 >> 0x10);
  uVar12 = (uint)puVar15;
  uStack8 = uVar12;
  puStack6 = puVar10;
  pass1_1038_4e78(uVar12,puVar10,param_3,puVar15);
  puStack12 = (undefined4 *)CONCAT22(puVar10,uVar12);
  uVar14 = 0x1030;
  uVar16 = pass1_1030_bcae((ushort)local_e,param_4);
  uVar13 = (undefined2)uVar16;
  ppcVar3 = (code **)((int)*puStack12 + 0x10);
  (**ppcVar3)(0x1030,(int)puStack12,(int)((ulong)puStack12 >> 0x10));
  uStack18 = CONCAT22(extraout_DX,uVar13);
  uVar13 = (undefined2)(param_3 >> 0x10);
  puVar2 = (undefined4 *)*(ulong *)((int)param_3 + 0xc);
  uVar13 = *(undefined2 *)((int)param_3 + 0xe);
  uVar18 = SUB41(puVar2,0x0);
  ppcVar3 = (code **)((int)*puVar2 + 0x10);
  puVar8 = puVar2;
  (**ppcVar3)();
  uVar16 = (ulong)puVar8 & 0xffff | (ulong)extraout_DX_00 << 0x10;
  uStack30 = 0x0;
  uVar12 = extraout_DX_00;
  do {
    if (uStack18 <= uStack30) {
      if (puStack12 != (undefined4 *)0x0) {
        ppcVar3 = (code **)*puStack12;
        (**ppcVar3)(uVar14,(int)puStack12,(int)((ulong)puStack12 >> 0x10),0x1,uVar18,uVar13);
      }
      return;
    }
    uVar14 = 0x1030;
    uVar9 = uStack18;
    pass1_1030_1d58((ulong)puStack12);
    uVar5 = (ulong)uVar12;
    iVar1 = *(int *)((int)uVar9 + 0x10);
    uVar11 = uVar12;
    for (uStack50 = 0x0; uVar12 = uVar11, uStack50 < uVar16; uStack50 = uStack50 + 0x1) {
      uVar14 = 0x1030;
      uVar17 = uVar16;
      pass1_1030_1d58((ulong)puVar2);
      uVar4 = uVar17 & 0xffff | (ulong)uVar11 << 0x10;
      uVar12 = uVar11 | (uint)uVar17;
      if ((uVar12 != 0x0) && (uVar12 = uVar11, *(int *)((uint)uVar17 + 0x10) == iVar1)) {
        uVar17 = struct_op_1030_73a8(uVar4);
        uVar12 = (uint)(uVar17 >> 0x10);
        uVar14 = 0x1008;
        BVar6 = pass1_1008_c6ae((ulong)_PTR_LOOP_1050_06e0,*(undefined2 *)((int)uVar17 + 0xc),0x30);
        if (BVar6 == 0x0) {
          puVar7 = local_e;
          uVar14 = 0x1030;
          pass1_1030_bd74((ushort)puVar7,param_4,uVar4,uVar9 & 0xffff | uVar5 << 0x10,param_4);
          if ((int)puVar7 < 0x6) {
            iStack4 = iStack4 + 0x1;
            break;
          }
        }
      }
      uVar11 = uVar12;
    }
    uStack30 = uStack30 + 0x1;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_0340(ushort param_1,ushort param_2,int param_3,ulong param_4,uint param_5,ushort param_6,uchar param_7)

{
  uint uVar1;
  ulong uVar2;
  int iVar3;
  undefined2 uVar4;
  undefined local_13a [0x11c];
  undefined4 uStack30;
  ulong uStack26;
  ulong uStack22;
  uint local_12;
  uint uStack16;
  int local_e;
  ushort uStack12;
  undefined4 uStack10;
  ulong uStack6;
  
  uStack6 = *_PTR_LOOP_1050_65e2;
  uStack10 = 0x0;
  uStack12 = 0x0;
  iVar3 = (int)param_4;
  uVar4 = (undefined2)(param_4 >> 0x10);
  pass1_1038_4cea(param_4,(ulong *)CONCAT22(param_6,&local_12),(ushort *)CONCAT22(param_6,&local_e));
  uVar2 = *(ulong *)(iVar3 + 0x1f6);
  uStack22 = uVar2;
  pass1_1030_38b8();
  uVar1 = (uint)uVar2;
  uStack26 = uVar2 & 0xffff | (ulong)param_5 << 0x10;
  if (param_3 == 0x0) {
    if (local_e != 0x8) {
      uStack10 = (long)(uVar2 & 0xffff | (ulong)param_5 << 0x10) / 0x4;
      uStack12 = 0x8;
      goto LAB_1038_054b;
    }
  }
  else {
    if (param_3 < 0xb) {
      if (local_e != 0x7) {
        uStack10 = (long)(uVar2 & 0xffff | (ulong)param_5 << 0x10) / 0xa;
        uStack12 = 0x7;
        goto LAB_1038_054b;
      }
    }
    else {
      if (param_3 < 0x1a) {
        if (local_e != 0x6) {
          uStack10 = (long)(uVar2 & 0xffff | (ulong)param_5 << 0x10) / 0x14;
          uStack12 = 0x6;
          goto LAB_1038_054b;
        }
      }
      else {
        if (param_3 < 0x33) {
          if (local_e != 0x5) {
            uStack10 = (long)(uVar2 & 0xffff | (ulong)param_5 << 0x10) / 0x64;
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
  if (((int)uStack16 <= (int)param_5) && (((int)uStack16 < (int)param_5 || (local_12 < uVar1)))) {
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
  if ((uStack10._2_2_ | (uint)uStack10) != 0x0) {
    if (*(long *)(iVar3 + 0x200) == 0x8000001) {
      uStack30._0_2_ = 0x2;
    }
    else {
      uStack30._0_2_ = 0x1;
    }
    uStack30 = CONCAT22(0x400,(undefined2)uStack30);
    pass1_1028_9944((astruct_100 *)CONCAT22(param_6,local_13a),uStack10,CONCAT22(0x400,(undefined2)uStack30),
                    *(ulong *)(iVar3 + 0x4),param_6,param_7);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_6,local_13a));
    pass1_1028_9992((ushort *)CONCAT22(param_6,local_13a));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_05d8(ushort param_1,ushort param_2,int param_3,ulong param_4,ulong param_5,ushort param_6,uchar param_7)

{
  undefined2 *puVar1;
  ulong uVar2;
  uint uVar3;
  uint uVar4;
  undefined2 uVar5;
  undefined local_158 [0x118];
  ulong uStack64;
  undefined2 local_34;
  undefined2 uStack50;
  undefined4 uStack34;
  undefined4 uStack30;
  ulong uStack26;
  undefined4 uStack22;
  undefined4 local_12;
  int local_e;
  ushort uStack12;
  undefined4 uStack10;
  ulong uStack6;
  
  uStack6 = *_PTR_LOOP_1050_65e2;
  uStack10 = 0x0;
  uStack12 = 0x0;
  pass1_1038_4cea(param_4,(ulong *)CONCAT22(param_6,&local_12),(ushort *)CONCAT22(param_6,&local_e));
  uStack22 = 0x0;
  uStack26 = 0x0;
  uStack30 = 0x0;
  pass1_1028_dc52((astruct_92 *)CONCAT13((char)(param_6 >> 0x8),CONCAT12((char)param_6,&local_34)),0x1,0x0,0x400);
  do {
    do {
      uVar3 = (uint)param_5;
      puVar1 = &local_34;
      pass1_1028_e4ec(CONCAT22(param_6,puVar1));
      uStack34 = CONCAT22(uVar3,puVar1);
      uVar4 = uVar3 | (uint)puVar1;
      param_5 = (ulong)uVar4;
      if (uVar4 == 0x0) goto LAB_1038_0668;
    } while (*(long *)(puVar1 + 0x100) != 0x8000002);
    uStack22 = CONCAT22(uVar3,puVar1);
    uVar2 = *(ulong *)(puVar1 + 0xfb);
    uStack26 = uVar2;
    pass1_1030_38b8();
    uStack30 = uVar2 & 0xffff | (ulong)uVar4 << 0x10;
    uVar4 = uVar4 | (uint)uVar2;
    param_5 = (ulong)uVar4;
  } while (uVar4 == 0x0);
LAB_1038_0668:
  local_34 = 0x389a;
  uStack50 = 0x1008;
  if ((uStack22._2_2_ | (uint)uStack22) == 0x0) {
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
  uStack10 = uVar2 & 0xffff | (ulong)uStack30._2_2_ << 0x10;
LAB_1038_0841:
  if (uStack12 != 0x0) {
    if ((uStack30 != 0x0) && (uStack10 == 0x0)) {
      uStack10 = 0x1;
    }
    pass1_1038_4cd0(param_4,uStack10,uStack12);
  }
  if ((uStack10._2_2_ | (uint)uStack10) != 0x0) {
    uVar5 = (undefined2)(param_4 >> 0x10);
    if (*(long *)((int)param_4 + 0x200) == 0x8000001) {
      uStack64 = *(ulong *)((int)uStack22 + 0x4);
    }
    else {
      uStack64 = 0x4000001;
    }
    pass1_1028_9944((astruct_100 *)CONCAT22(param_6,local_158),uStack10,*(ulong *)((int)param_4 + 0x4),uStack64,param_6,
                    param_7);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_6,local_158));
    pass1_1028_9992((ushort *)CONCAT22(param_6,local_158));
  }
  return;
}



void __stdcall16far
pass1_1038_08d4(ushort param_1,long param_2,ulong param_3,ulong param_4,ushort param_5,uchar param_6)

{
  undefined2 *puVar1;
  uint uVar2;
  uint uVar3;
  undefined2 local_16;
  undefined2 uStack20;
  int iStack4;
  
  iStack4 = 0x0;
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_5,&local_16),0x1,0x0,0x400);
  do {
    puVar1 = &local_16;
    pass1_1028_e4ec(CONCAT22(param_5,puVar1));
    uVar2 = (uint)param_4;
    uVar3 = uVar2 | (uint)puVar1;
    param_4 = param_4 & 0xffff0000 | (ulong)uVar3;
    if (uVar3 == 0x0) goto LAB_1038_0917;
  } while (*(long *)(puVar1 + 0x100) != 0x8000002);
  iStack4 = 0x1;
LAB_1038_0917:
  local_16 = 0x389a;
  uStack20 = 0x1008;
  if (iStack4 != 0x0) {
    if (param_2 < 0xc90000) {
      pass1_1038_0340(param_1,(ushort)param_2,param_2._2_2_,param_3,uVar3,param_5,param_6);
      return;
    }
    if (0x31fffff < param_2) {
      pass1_1038_05d8(param_1,(ushort)param_2,param_2._2_2_,param_3,param_4,param_5,param_6);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_095e(ushort param_1,ushort param_2,int param_3,ulong param_4,uchar *param_5,int param_6,ushort param_7)

{
  code **ppcVar1;
  bool bVar2;
  uint uVar3;
  undefined *puVar4;
  ulong uVar5;
  ulong uVar6;
  uchar *puVar7;
  undefined2 uVar8;
  undefined uVar9;
  ulong *puVar10;
  ulong uVar11;
  int iVar12;
  ulong uStack58;
  ulong uStack54;
  undefined local_28 [0x2];
  ulong uStack38;
  ulong uStack34;
  undefined4 *puStack30;
  uint uStack26;
  uchar *puStack24;
  undefined4 *puStack22;
  ulong uStack18;
  int iStack14;
  int iStack12;
  ulong uStack10;
  astruct_67 *paStack6;
  
  paStack6 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_7,param_5,param_6);
  uStack10 = *_PTR_LOOP_1050_65e2;
  uVar8 = (undefined2)(param_4 >> 0x10);
  if (uStack10 % 0xa == 0x0) {
    if (param_3 < 0xc9) {
      iVar12 = 0x3f;
    }
    else {
      if (param_3 < 0x320) goto LAB_1038_09c3;
      iVar12 = 0x3e;
    }
    post_win_msg_1008_a0e4(paStack6,0x0,0x0,0x1,*(ulong *)((int)param_4 + 0x4),iVar12,0x1008,param_7);
  }
LAB_1038_09c3:
  iStack12 = *(int *)((int)param_4 + 0x22);
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
  if ((uStack18 & 0xffff | (ulong)*(uint *)((int)_PTR_LOOP_1050_65e2 + 0x2) << 0x10) % (ulong)uVar3 == 0x0) {
    iStack14 = 0x1;
  }
LAB_1038_0a1c:
  if (iStack14 != 0x0) {
    puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0xf);
    puVar7 = (uchar *)((ulong)puVar10 >> 0x10);
    uVar3 = (uint)puVar10;
    pass1_1038_4e78(uVar3,puVar7,param_4,puVar10);
    puStack22 = (undefined4 *)CONCAT22(puVar7,uVar3);
    puVar10 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1a);
    puVar7 = (uchar *)((ulong)puVar10 >> 0x10);
    uVar3 = (uint)puVar10;
    uStack26 = uVar3;
    puStack24 = puVar7;
    pass1_1038_4d6e(param_4,puVar10,uVar3,puVar7);
    puStack30 = (undefined4 *)CONCAT22(puVar7,uVar3);
    ppcVar1 = (code **)((int)*puStack22 + 0x10);
    (**ppcVar1)(0x1008,(int)puStack22,(int)((ulong)puStack22 >> 0x10));
    uStack34 = CONCAT22(puVar7,uVar3);
    ppcVar1 = (code **)((int)*puStack30 + 0x10);
    (**ppcVar1)(0x1008,(char)puStack30,(int)((ulong)puStack30 >> 0x10));
    uStack38 = CONCAT22(puVar7,uVar3);
    uVar11 = pass1_1030_bcae((ushort)local_28,param_7);
    uStack54 = 0x0;
    while( true ) {
      uVar11 = uVar11 >> 0x10;
      uVar9 = 0x30;
      if (uStack34 <= uStack54) break;
      uVar6 = uStack34;
      pass1_1030_1d58((ulong)puStack22);
      uVar6 = uVar6 & 0xffff | uVar11 << 0x10;
      bVar2 = false;
      for (uStack58 = 0x0; uStack58 < uStack38; uStack58 = uStack58 + 0x1) {
        uVar5 = uStack38;
        pass1_1030_1d58((ulong)puStack30);
        puVar4 = local_28;
        pass1_1030_bd74((ushort)puVar4,param_7,uVar6,uVar5 & 0xffff | uVar11 << 0x10,param_7);
        if ((int)puVar4 < 0x6) {
          bVar2 = true;
          break;
        }
      }
      uVar11 = struct_op_1030_73a8(uVar6);
      if (!bVar2) {
        uVar9 = 0x28;
        func_0x10285ca0(0x1030,(char)uVar11,(int)(uVar11 >> 0x10));
        break;
      }
      uStack54 = uStack54 + 0x1;
    }
    if (puStack22 != (undefined4 *)0x0) {
      ppcVar1 = (code **)*puStack22;
      (**ppcVar1)(uVar9,(int)puStack22,(char)((ulong)puStack22 >> 0x10),0x1);
    }
    if (puStack30 != (undefined4 *)0x0) {
      ppcVar1 = (code **)*puStack30;
      (**ppcVar1)(uVar9,(int)puStack30,(char)((ulong)puStack30 >> 0x10),0x1);
    }
  }
  return;
}



astruct_18 * __stdcall16far pass1_1038_0b6a(astruct_18 *param_1,byte param_2)

{
  param_1->field_0x0 = 0x389a;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_100 * __stdcall16far pass1_1038_0ba6(astruct_100 *param_1,int param_2,ushort param_3,uchar param_4)

{
  uchar *puVar1;
  astruct_701 *iVar2;
  undefined2 uVar2;
  astruct_100 *paVar3;
  ushort *puVar4;
  
  paVar3 = struct_op_1028_d1dc(param_3,param_4,param_1,0x270f);
  puVar1 = (uchar *)((ulong)paVar3 >> 0x10);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_701 *)param_1;
  *(undefined4 *)&iVar2->field_0x108 = 0x0;
  param_1->field_0x0 = 0x1c2e;
  iVar2->field_0x2 = (int)&PTR_LOOP_1050_1038;
  unk_str_op_1000_3d3e((char *)((ulong)param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x8),s_SCMove_1050_59d8);
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_3,puVar1,param_2);
  iVar2->field_0x108 = (int)puVar4;
  iVar2->field_0x10a = (int)((ulong)puVar4 >> 0x10);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_0c00(ulong param_1,ulong param_2,ushort param_3,ushort param_4,ushort param_5,uchar param_6)

{
  code **ppcVar1;
  undefined4 uVar2;
  undefined *puVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  uchar *puVar7;
  uchar *puVar8;
  ulong uVar9;
  undefined2 uVar10;
  ulong *puVar11;
  undefined4 *puStack32;
  ulong uStack24;
  undefined local_14 [0x12];
  
  pass1_1028_dc52((astruct_92 *)CONCAT13((char)(param_5 >> 0x8),CONCAT12((char)param_5,local_14)),0x1,0x0,0x400);
  while( true ) {
    puVar3 = local_14;
    pass1_1028_e4ec(CONCAT22(param_5,puVar3));
    uVar6 = (uint)param_2;
    uStack24 = CONCAT22(uVar6,puVar3);
    uVar9 = param_2 & 0xffff0000 | (ulong)(uVar6 | (uint)puVar3);
    if ((uVar6 | (uint)puVar3) == 0x0) break;
    pass1_1038_0e78(param_1,CONCAT22(uVar6,puVar3),param_5);
    pass1_1038_1220(param_1,CONCAT22(uVar6,puVar3),uVar9,param_5);
    uVar10 = (undefined2)(uVar9 >> 0x10);
    puVar11 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1);
    puVar7 = (uchar *)((ulong)puVar11 >> 0x10);
    uVar4 = (uint)puVar11;
    pass1_1038_4d6e(CONCAT22(uVar6,puVar3),puVar11,uVar4,puVar7);
    puStack32 = (undefined4 *)CONCAT22(puVar7,uVar4);
    ppcVar1 = (code **)((int)*puStack32 + 0x10);
    uVar5 = uVar4;
    puVar8 = puVar7;
    (**ppcVar1)(0x1008,uVar4,puVar7);
    param_2 = CONCAT22(uVar10,(uint)puVar8 | uVar5);
    if (((uint)puVar8 | uVar5) != 0x0) {
      uVar2 = *(undefined4 *)((int)param_1 + 0x108);
      if (*(int *)((int)uVar2 + 0x82) != 0x0) {
        pass1_1038_19a0(param_1,(ulong *)CONCAT22(puVar7,uVar4),CONCAT22(uVar6,puVar3),param_5,param_6);
      }
      pass1_1038_1940(param_1,(ulong *)CONCAT22(puVar7,uVar4),uStack24,param_3,param_4,param_5);
    }
    if (puStack32 != (undefined4 *)0x0) {
      ppcVar1 = (code **)*puStack32;
      (**ppcVar1)(0x8,uVar4,puVar7,0x1);
    }
    pass1_1038_1c3e(param_1,uStack24,param_3,param_4,0x1008,param_5);
  }
  return;
}



void __stdcall16far pass1_1038_0cf0(ulong param_1,uint param_2,uchar *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  int iVar4;
  int iVar5;
  undefined4 *puVar6;
  undefined2 uVar7;
  undefined2 *puStack10;
  
  mem_op_1000_179c(0x10c,param_3,0x1000);
  puStack10 = (undefined2 *)CONCAT22(param_3,param_2);
  if (((uint)param_3 | param_2) != 0x0) {
    *puStack10 = 0x389a;
    *(undefined2 *)(param_2 + 0x2) = 0x1008;
    uVar7 = (undefined2)(param_1 >> 0x10);
    iVar5 = (int)param_1;
    *(undefined4 *)(param_2 + 0x4) = *(undefined4 *)(iVar5 + 0x4);
    puVar3 = (undefined4 *)(iVar5 + 0x8);
    puVar6 = (undefined4 *)(param_2 + 0x8);
    for (iVar4 = 0x40; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
      puVar2 = puVar6;
      puVar6 = puVar6 + 0x1;
      puVar1 = puVar3;
      puVar3 = puVar3 + 0x1;
      *puVar2 = *puVar1;
    }
    *puStack10 = 0x6ad2;
    *(undefined2 *)(param_2 + 0x2) = (int)&USHORT_1050_1028;
    *(undefined4 *)(param_2 + 0x108) = *(undefined4 *)(iVar5 + 0x108);
    *puStack10 = 0x1c2e;
    *(undefined2 *)(param_2 + 0x2) = (int)&PTR_LOOP_1050_1038;
  }
  return;
}



void __stdcall16far pass1_1038_0d8e(ushort param_1,ushort param_2,ulong param_3,ulong param_4,ushort param_5)

{
  ushort uVar1;
  uint uVar2;
  ushort uVar3;
  long lStack10;
  ushort uStack4;
  
  uVar1 = pass1_1030_d0a8(param_4);
  uVar2 = pass1_1030_d144(param_4);
  lStack10 = (long)(int)uVar2;
  uVar2 = (int)uVar2 >> 0xf | uVar2;
  uStack4 = uVar1;
  if (uVar2 != 0x0) {
    do {
      uVar3 = pass1_1028_6744(param_5,param_3,uStack4);
      uVar2 = uVar2 | uVar3;
      if (uVar2 != 0x0) {
        pass1_1028_6228(param_3,0x1,0x0,uStack4,param_5);
        lStack10 = lStack10 + -0x1;
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

void __stdcall16far pass1_1038_0e00(ulong param_1,ulong *param_2,ulong param_3,ushort param_4,ushort param_5)

{
  code **ppcVar1;
  undefined2 extraout_DX;
  uint extraout_DX_00;
  uint uVar2;
  uint uVar3;
  ulong uVar4;
  ulong uStack10;
  ulong uStack6;
  
  ppcVar1 = (code **)((int)*param_2 + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,param_4);
  for (uStack10 = 0x0; uStack10 < uStack6; uStack10 = uStack10 + 0x1) {
    ppcVar1 = (code **)((int)*param_2 + 0x4);
    uVar4 = uStack6;
    (**ppcVar1)();
    uVar3 = (uint)uVar4;
    uVar2 = extraout_DX_00 | uVar3;
    if (uVar2 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,extraout_DX_00);
      uVar4 = struct_op_1030_73a8(CONCAT22(uVar2,uVar3));
      uVar3 = (uint)(uVar4 >> 0x10);
      if ((uVar3 | (uint)uVar4) != 0x0) {
        pass1_1038_0d8e((ushort)param_1,(ushort)(param_1 >> 0x10),uVar4 & 0xffff | (ulong)uVar3 << 0x10,param_3,param_5)
        ;
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_0e78(ulong param_1,ulong param_2,ushort param_3)

{
  code **ppcVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  uchar *puVar5;
  uint extraout_DX;
  uchar *puVar6;
  undefined2 extraout_DX_00;
  uint extraout_DX_01;
  uint uVar7;
  undefined2 uVar8;
  ulong *puVar9;
  ulong uVar10;
  ulong uStack22;
  ulong uStack18;
  undefined4 *puStack14;
  ulong *puStack10;
  
  puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x4);
  puVar5 = (uchar *)((ulong)puVar9 >> 0x10);
  uVar2 = (uint)puVar9;
  pass1_1038_4d6e(param_2,puVar9,uVar2,puVar5);
  puStack10 = (ulong *)CONCAT22(puVar5,uVar2);
  uVar10 = *puStack10;
  ppcVar1 = (code **)uVar10 + 0x8;
  uVar3 = uVar2;
  (**ppcVar1)(0x1008,uVar2,puVar5);
  if ((extraout_DX | uVar3) == 0x0) {
    if (puStack10 != (ulong *)0x0) {
      ppcVar1 = (code **)uVar10;
      (**ppcVar1)(0x8,uVar2,(char)puVar5,0x1);
      return;
    }
  }
  else {
    uVar8 = 0x1008;
    puVar9 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x1e);
    puVar6 = (uchar *)((ulong)puVar9 >> 0x10);
    uVar3 = (uint)puVar9;
    pass1_1038_4d6e(param_2,puVar9,uVar3,puVar6);
    puStack14 = (undefined4 *)CONCAT22(puVar6,uVar3);
    ppcVar1 = (code **)((int)*puStack14 + 0x10);
    uVar4 = uVar3;
    (**ppcVar1)(0x1008,(char)uVar3,puVar6);
    uStack18 = CONCAT22(extraout_DX_00,uVar4);
    for (uStack22 = 0x0; uStack22 < uStack18; uStack22 = uStack22 + 0x1) {
      ppcVar1 = (code **)((int)*puStack14 + 0x4);
      uVar10 = uStack18;
      (**ppcVar1)();
      uVar4 = (uint)uVar10;
      uVar7 = extraout_DX_01 | uVar4;
      if (uVar7 != 0x0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar4,extraout_DX_01);
        uVar8 = 0x1030;
        uVar10 = struct_op_1030_73a8(CONCAT22(uVar7,uVar4));
        if (((uint)(uVar10 >> 0x10) | (uint)uVar10) != 0x0) {
          pass1_1038_0e00(param_1,puStack10,uVar10,(uint)uVar10,param_3);
        }
      }
    }
    if (puStack10 != (ulong *)0x0) {
      ppcVar1 = (code **)*puStack10;
      (**ppcVar1)(uVar8,uVar2,(char)puVar5,0x1);
    }
    if (puStack14 != (undefined4 *)0x0) {
      ppcVar1 = (code **)*puStack14;
      (**ppcVar1)(uVar8,uVar3,(char)puVar6,0x1);
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1038_0f8c(ushort param_1,ushort param_2,ulong *param_3,ulong param_4,ushort param_5,ulong param_6,ushort param_7,
               ushort param_8)

{
  int *piVar1;
  undefined2 uVar2;
  undefined2 uVar3;
  undefined2 uVar4;
  code **ppcVar5;
  ulong uVar6;
  qword qVar7;
  undefined *puVar8;
  ulong uVar9;
  uint uVar10;
  uint uVar11;
  uint uVar12;
  int iVar13;
  undefined2 uVar14;
  astruct_99 *paStack80;
  uint uStack76;
  undefined local_30 [0x4];
  undefined4 uStack44;
  undefined4 *puStack40;
  undefined4 uStack36;
  undefined local_20 [0x4];
  undefined4 *puStack28;
  uint uStack24;
  uint uStack22;
  uint uStack20;
  uint uStack18;
  ulong uStack16;
  ulong uStack12;
  undefined2 uStack8;
  undefined4 uStack6;
  
  uStack6 = 0x64;
  uStack8 = 0x0;
  ppcVar5 = (code **)((int)*param_3 + 0x10);
  (**ppcVar5)(param_7,param_3);
  uStack12 = CONCAT22((int)param_6,param_5);
  uStack16 = 0x0;
  do {
    if (uStack12 <= uStack16) {
      return;
    }
    ppcVar5 = (code **)((int)*param_3 + 0x4);
    uVar9 = uStack12;
    uVar11 = (uint)param_6;
    (**ppcVar5)(param_7,(char)param_3,(int)((ulong)param_3 >> 0x10),(char)uStack16,(int)(uStack16 >> 0x10));
    uStack18 = uVar11;
    uVar12 = (uint)uVar9;
    uVar11 = uStack18 | uVar12;
    param_6 = (ulong)uVar11;
    uStack20 = uVar12;
    if (uVar11 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar12,uStack18);
      uStack22 = uVar11;
      param_7 = 0x1030;
      uStack24 = uVar12;
      puStack28 = (undefined4 *)struct_op_1030_73a8(CONCAT22(uStack22,uVar12));
      param_6 = (ulong)puStack28 >> 0x10;
      puVar8 = local_20;
      ppcVar5 = (code **)((int)*puStack28 + 0x40);
      (**ppcVar5)(0x1030,(char)puStack28,(int)((ulong)puStack28 >> 0x10),(char)puVar8,param_8);
      if (puVar8 == (undefined *)0x0) {
        uStack36 = pass1_1028_62c8((ulong)puStack28,param_8);
        uVar9 = uStack36 >> 0x10;
        uStack8 = 0x1;
        puStack40 = (undefined4 *)*(ulong *)((int)param_4 + 0x22);
        pass1_1008_5784((ulong *)CONCAT22(param_8,local_30),(ulong)puStack40);
        while( true ) {
          uVar11 = (uint)uVar9;
          puVar8 = local_30;
          param_7 = 0x1008;
          pass1_1008_5b12((char)puVar8,param_8);
          param_6 = (ulong)(uVar11 | (uint)puVar8);
          if ((uVar11 | (uint)puVar8) == 0x0) break;
          uVar2 = *(undefined2 *)(puVar8 + 0x4);
          uVar3 = *(undefined2 *)(puVar8 + 0x6);
          uVar4 = *(undefined2 *)(puVar8 + 0x8);
          uVar12 = *(uint *)(puVar8 + 0xa);
          uVar6 = (ulong)*(uint *)(puVar8 + 0xc) / (ulong)uVar12;
          uVar9 = uStack36;
          if (uStack6 < uStack36) {
            uVar9 = uStack6 & 0xffff;
            uStack36._2_2_ = uStack6._2_2_;
          }
          uVar10 = uStack36._2_2_ | (uint)uVar9;
          param_6 = (ulong)uVar10;
          if (uVar10 == 0x0) break;
          qVar7 = (qword)(uVar9 & 0xffff | (ulong)uStack36._2_2_ << 0x10) / (qword)uVar6;
          param_6 = (ulong)qVar7 >> 0x10;
          uStack76 = (uint)qVar7;
          if (uStack76 == 0x0) break;
          if (uStack76 < uVar12) {
            piVar1 = (int *)(puVar8 + 0xc);
            *piVar1 = *piVar1 - (uint)uVar9;
            piVar1 = (int *)(puVar8 + 0xa);
            *piVar1 = *piVar1 - uStack76;
          }
          else {
            ppcVar5 = (code **)((int)*puStack40 + 0xc);
            (**ppcVar5)(0x1008,(char)puStack40,(int)((ulong)puStack40 >> 0x10),(char)puVar8,uVar11);
            uStack44 = 0x0;
            uStack76 = uVar12;
          }
          paStack80 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
          uVar12 = (uint)((ulong)paStack80 >> 0x10);
          uVar11 = (uint)paStack80;
          if ((uVar12 | uVar11) == 0x0) {
            paStack80 = (astruct_99 *)0x0;
          }
          else {
            paStack80->field_0x0 = 0x389a;
            *(undefined2 *)(uVar11 + 0x2) = 0x1008;
            *(undefined2 *)(uVar11 + 0x4) = 0x0;
            *(undefined2 *)(uVar11 + 0x6) = 0x0;
            *(undefined2 *)(uVar11 + 0x8) = 0x0;
            *(undefined2 *)(uVar11 + 0xa) = 0x0;
            *(undefined2 *)(uVar11 + 0xc) = 0x0;
            paStack80->field_0x0 = 0x56ce;
            *(undefined2 *)(uVar11 + 0x2) = 0x1018;
          }
          uVar14 = (undefined2)((ulong)paStack80 >> 0x10);
          iVar13 = (int)paStack80;
          *(uint *)(iVar13 + 0xa) = uStack76;
          uVar6 = uStack76 * uVar6;
          uVar9 = uVar6 >> 0x10;
          *(undefined2 *)(iVar13 + 0xc) = (int)uVar6;
          *(undefined2 *)(iVar13 + 0x4) = uVar2;
          *(undefined2 *)(iVar13 + 0x6) = uVar3;
          *(undefined2 *)(iVar13 + 0x8) = uVar4;
          pass1_1028_6408((ulong)puStack28,(ulong *)paStack80,param_8);
        }
      }
      else {
        ppcVar5 = (code **)((int)*param_3 + 0x8);
        (**ppcVar5)(0x1030,param_3,0x0,0x0,(char)uStack16,(int)(uStack16 >> 0x10));
      }
    }
    uStack16 = uStack16 + 0x1;
  } while( true );
}
