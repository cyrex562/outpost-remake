


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1030_9adc(ushort param_1,ushort param_2,ulong *param_3,ulong param_4,uint param_5,uint param_6)

{
  code **ppcVar1;
  astruct_99 *paVar2;
  uint uVar4;
  uint extraout_DX;
  uint extraout_DX_00;
  astruct_121 *iVar7;
  astruct_119 *iVar6;
  astruct_99 *paStack6;
  astruct_120 *uVar3;
  
  pass1_1038_53ba(param_4,0x1);
  uVar4 = param_6 | param_5;
  if (uVar4 != 0x0) {
    paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    uVar4 = (uint)((ulong)paStack6 >> 0x10);
    paVar2 = (astruct_99 *)((ulong)paStack6 & 0xffff);
    if ((uVar4 | (uint)paVar2) == 0x0) {
      paStack6 = (astruct_99 *)0x0;
    }
    else {
      iVar7 = (astruct_121 *)paStack6;
      paStack6->field_0x0 = 0x389a;
      iVar7->field_0x2 = 0x1008;
      iVar7->field_0x4 = 0x77;
      paStack6->field_0x0 = 0x9ec8;
      iVar7->field_0x2 = 0x1030;
      paVar2 = paStack6;
    }
    param_5 = (uint)paVar2;
    ppcVar1 = (code **)((int)*param_3 + 0x4);
    (**ppcVar1)(0x1000,param_3,(int)paStack6,(int)((ulong)paStack6 >> 0x10));
    uVar4 = extraout_DX;
  }
  pass1_1038_53ba(param_4,0x2);
  uVar4 = uVar4 | param_5;
  if (uVar4 != 0x0) {
    paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    uVar4 = (uint)((ulong)paStack6 >> 0x10);
    paVar2 = (astruct_99 *)((ulong)paStack6 & 0xffff);
    if ((uVar4 | (uint)paVar2) == 0x0) {
      paStack6 = (astruct_99 *)0x0;
    }
    else {
      iVar6 = (astruct_119 *)paStack6;
      paStack6->field_0x0 = 0x389a;
      iVar6->field_0x2 = 0x1008;
      iVar6->field_0x4 = 0x78;
      paStack6->field_0x0 = 0x9ec8;
      iVar6->field_0x2 = 0x1030;
      paVar2 = paStack6;
    }
    param_5 = (uint)paVar2;
    ppcVar1 = (code **)((int)*param_3 + 0x8);
    (**ppcVar1)(0x1000,param_3,(int)paStack6,(int)((ulong)paStack6 >> 0x10));
    uVar4 = extraout_DX_00;
  }
  pass1_1038_53ba(param_4,0x3);
  if ((uVar4 | param_5) != 0x0) {
    paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
    uVar4 = (uint)((ulong)paStack6 >> 0x10);
    uVar3 = (astruct_120 *)paStack6;
    if ((uVar4 | (uint)uVar3) == 0x0) {
      paStack6 = (astruct_99 *)0x0;
    }
    else {
      paStack6->field_0x0 = 0x389a;
      uVar3->field_0x2 = 0x1008;
      uVar3->field_0x4 = 0x75;
      paStack6->field_0x0 = 0x9ec8;
      uVar3->field_0x2 = 0x1030;
    }
    ppcVar1 = (code **)((int)*param_3 + 0x8);
    (**ppcVar1)(0x1000,param_3,(int)paStack6,(int)((ulong)paStack6 >> 0x10));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_9c1c(ulong param_1,ulong *param_2,ulong param_3)

{
  code **ppcVar1;
  uint uVar2;
  undefined2 uVar3;
  int iVar4;
  int iVar5;
  uchar *in_DX;
  uint uVar6;
  int unaff_DI;
  ushort unaff_SS;
  ushort *puVar7;
  int iStack24;
  int iStack16;
  astruct_99 *paStack6;
  
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x35,unaff_SS,in_DX,unaff_DI);
  iVar4 = (int)puVar7 + 0xa;
  uVar3 = (undefined2)((ulong)puVar7 >> 0x10);
  iVar5 = iVar4;
  pass1_1030_9048(unaff_SS,param_1,0x1,param_3);
  if (iVar5 != 0x0) {
    for (iStack24 = 0x4f; iStack24 < 0x70; iStack24 = iStack24 + 0x1) {
      if (*(int *)(iStack24 * 0x2 + iVar4) != 0x0) {
        paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
        uVar6 = (uint)((ulong)paStack6 >> 0x10);
        uVar2 = (uint)paStack6;
        if ((uVar6 | uVar2) == 0x0) {
          paStack6 = (astruct_99 *)0x0;
        }
        else {
          paStack6->field_0x0 = 0x389a;
          *(undefined2 *)(uVar2 + 0x2) = 0x1008;
          *(int *)(uVar2 + 0x4) = iStack24;
          paStack6->field_0x0 = 0x9ec8;
          *(undefined2 *)(uVar2 + 0x2) = 0x1030;
        }
        ppcVar1 = (code **)((int)*param_2 + 0x8);
        (**ppcVar1)(0x1000,param_2,(int)paStack6,(int)((ulong)paStack6 >> 0x10));
      }
    }
  }
  for (iStack16 = 0x7d; iStack16 < 0x80; iStack16 = iStack16 + 0x1) {
    if (*(int *)(iStack16 * 0x2 + iVar4) != 0x0) {
      paStack6 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_5768);
      uVar6 = (uint)((ulong)paStack6 >> 0x10);
      uVar2 = (uint)paStack6;
      if ((uVar6 | uVar2) == 0x0) {
        paStack6 = (astruct_99 *)0x0;
      }
      else {
        paStack6->field_0x0 = 0x389a;
        *(undefined2 *)(uVar2 + 0x2) = 0x1008;
        *(int *)(uVar2 + 0x4) = iStack16;
        paStack6->field_0x0 = 0x9ec8;
        *(undefined2 *)(uVar2 + 0x2) = 0x1030;
      }
      ppcVar1 = (code **)((int)*param_2 + 0x8);
      (**ppcVar1)(0x1000,param_2,(int)paStack6,(int)((ulong)paStack6 >> 0x10));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1030_9d42(ushort param_1,ushort param_2,ushort param_3,ushort param_4,ulong *param_5,ulong param_6)

{
  ulong *puVar1;
  ulong uVar2;
  code **ppcVar3;
  uint *puVar4;
  undefined *puVar5;
  undefined *puVar6;
  uint extraout_DX;
  uint uVar8;
  int iVar9;
  undefined2 uVar10;
  undefined local_a6 [0x4];
  undefined4 uStack162;
  ulong uStack158;
  int iStack154;
  undefined4 local_98;
  ulong uStack12;
  ulong uStack8;
  int iStack4;
  ulong uVar7;
  
  uVar10 = (undefined2)(param_6 >> 0x10);
  if (*(int *)((int)param_6 + 0x206) == 0x0) {
    iStack4 = *(int *)((int)param_6 + 0x204);
    puVar4 = pass1_1000_4906((astruct_20 *)CONCAT22(param_1,&local_98),(WNDCLASS16 *)0x0,0x94);
    uVar7 = ZEXT24(puVar4);
    iStack154 = 0x11;
    do {
      empty_1038_540a();
      uVar10 = (undefined2)uVar7;
      *(undefined2 *)(&local_98 + iStack154) = uVar10;
      *(ushort *)((int)&local_98 + iStack154 * 0x4 + 0x2) = param_2;
      iStack154 = iStack154 + 0x1;
    } while (iStack154 < 0x25);
    empty_1038_540a();
    uStack158 = CONCAT22(param_2,uVar10);
    pass1_1008_5784((ulong *)CONCAT22(param_1,local_a6),(ulong)param_5);
    uVar7 = *(ulong *)((int)_PTR_LOOP_1050_65e2 + 0x52);
    while( true ) {
      puVar5 = local_a6;
      pass1_1008_5b12(puVar5,param_1);
      uVar8 = extraout_DX | (uint)puVar5;
      if (uVar8 == 0x0) break;
      puVar6 = puVar5;
      pass1_1030_4bbe(param_1,uVar8,uVar7,*(int *)(puVar5 + 0x4));
      if (iStack4 == 0x0) {
        for (iStack154 = 0x11; iStack154 < 0x25; iStack154 = iStack154 + 0x1) {
          iVar9 = iStack154 * 0x4;
          if ((*(long *)(puVar6 + iVar9) != 0x0) &&
             (uVar2 = (&local_98)[iStack154], puVar1 = (ulong *)(puVar6 + iVar9), uVar2 <= *puVar1 && *puVar1 != uVar2))
          {
            puVar1 = (ulong *)(puVar6 + iVar9);
            if (uStack158 <= *puVar1 && *puVar1 != uStack158) goto LAB_1030_9e17;
            uStack158 = uStack158 - *(long *)(puVar6 + iVar9);
          }
        }
      }
      else {
        puVar1 = (ulong *)(puVar6 + 0x8c);
        if ((uStack12 <= *puVar1 && *puVar1 != uStack12) ||
           (puVar1 = (ulong *)(puVar6 + 0x90), uStack8 <= *puVar1 && *puVar1 != uStack8)) {
LAB_1030_9e17:
          ppcVar3 = (code **)((int)*param_5 + 0xc);
          (**ppcVar3)(0x1008,(char)param_5,(int)((ulong)param_5 >> 0x10),puVar5,extraout_DX);
          uStack162 = 0x0;
        }
      }
    }
  }
  return;
}



ushort * __stdcall16far pass1_1030_9e9c(ushort *param_1,byte param_2)

{
  ushort uVar1;
  
  uVar1 = (ushort)((ulong)param_1 >> 0x10);
  *param_1 = 0x389a;
  ((int *)param_1)[0x1] = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a((int *)param_1,uVar1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1030_9ecc(ulong *param_1,ulong param_2)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *param_1 = 0x0;
  *(ulong *)((int)param_1 + 0x4) = param_2;
  *(undefined2 *)((int)param_1 + 0x8) = 0x0;
  return;
}



ushort __stdcall16far pass1_1030_9ef2(ulong *param_1)

{
  int iVar1;
  undefined2 uVar2;
  ulong uVar3;
  
  if (*param_1 != 0x0) {
    uVar3 = struct_op_1030_73a8(*param_1);
    uVar2 = (undefined2)(uVar3 >> 0x10);
    iVar1 = *(int *)((int)uVar3 + 0xc);
    if (((iVar1 != 0x5) && (iVar1 != 0x9)) && (*(int *)((int)uVar3 + 0x12) < 0x5)) {
      return 0x0;
    }
    pass1_1030_9f64(param_1);
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_9f40(ulong param_1,ushort param_2,ushort param_3,uchar param_4)

{
  ushort uVar1;
  
  uVar1 = pass1_1008_c646((ushort)_PTR_LOOP_1050_06e0,CONCAT22(param_2,(int)((ulong)_PTR_LOOP_1050_06e0 >> 0x10)),
                          param_3);
  *(ushort *)((int)param_1 + 0x8) = uVar1;
  pass1_1030_9f7a((ushort *)param_1,uVar1,param_3,param_4);
  return;
}



void __stdcall16far pass1_1030_9f64(ulong *param_1)

{
  *(undefined2 *)((int)param_1 + 0x8) = 0x0;
  *param_1 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_9f7a(ushort *param_1,ushort param_2,ushort param_3,uchar param_4)

{
  undefined4 uVar1;
  BOOL16 BVar2;
  ulong *puVar3;
  ushort extraout_DX;
  ushort uVar4;
  ushort uVar5;
  undefined local_130 [0x120];
  ulong uStack16;
  ulong uStack12;
  ulong local_8;
  int iStack4;
  
  pass1_1008_3e38((ushort *)CONCAT22(param_3,&local_8));
  BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,param_2,0x28);
  if (BVar2 != 0x0) {
    iStack4 = 0x1;
  }
  puVar3 = &local_8;
  pass1_1030_a278(param_1,(ushort *)CONCAT22(param_3,puVar3),(ushort)puVar3,param_3,param_4);
  if (puVar3 != (ulong *)0x0) {
    uVar5 = (ushort)((ulong)param_1 >> 0x10);
    uVar4 = (ushort)param_1;
    uVar1 = *(undefined4 *)(uVar4 + 0x4);
    uStack12 = *(ulong *)((int)uVar1 + 0x8);
    uVar1 = *(undefined4 *)(uVar4 + 0x4);
    struct_op_1028_87f0(param_3,param_4,(astruct_97 *)CONCAT22(param_3,local_130),0x0,0x0,param_2,&local_8,param_3,
                        *(ulong *)((int)uVar1 + 0x4),uStack12);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_3,local_130));
    pass1_1028_b58e(uStack16);
    *param_1 = (ushort)uStack16;
    *(ushort *)(uVar4 + 0x2) = extraout_DX;
    if (0x0 < iStack4) {
      pass1_1030_a044(param_3,extraout_DX,param_4,uVar4,uVar5,(ushort *)CONCAT22(param_3,&local_8),uStack12);
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1030_a044(ushort param_1,ushort param_2,uchar param_3,ushort param_4,ushort param_5,ushort *param_6,ulong param_7)

{
  code **ppcVar1;
  ushort *puVar2;
  undefined *puVar3;
  int iVar4;
  undefined4 uVar5;
  uint uVar6;
  undefined2 extraout_DX;
  undefined2 uVar7;
  undefined4 *puVar8;
  ushort uVar9;
  undefined2 uVar10;
  undefined2 uVar11;
  undefined2 local_17e;
  undefined2 uStack380;
  int iStack90;
  undefined4 *puStack78;
  ushort uStack70;
  int iStack68;
  undefined4 uStack66;
  undefined4 *puStack62;
  undefined local_3a [0xc];
  undefined4 local_2e;
  undefined2 uStack42;
  int iStack40;
  ushort uStack38;
  int local_24;
  int local_22;
  undefined4 uStack32;
  undefined4 uStack28;
  undefined4 uStack24;
  ushort *puStack20;
  uint uStack18;
  int iStack16;
  int iStack14;
  undefined4 uStack12;
  ushort local_8;
  int local_6;
  int local_4;
  
  puVar2 = &local_8;
  pass1_1008_3eb4(param_6,(ushort *)CONCAT22(param_1,puVar2),(ushort *)CONCAT22(param_1,&local_6),
                  (ushort *)CONCAT22(param_1,&local_4));
  pass1_1030_627e(param_1,(uint)puVar2,param_2,(ulong)_PTR_LOOP_1050_5740,param_6,param_7);
  puStack20 = puVar2;
  uStack18 = param_2;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)puVar2,param_2);
  uStack24 = CONCAT22(param_2,puVar2);
  uStack28 = *(undefined4 *)(puVar2 + 0x17);
  uVar5 = *(undefined4 *)((int)uStack28 + 0x4);
  uStack32 = uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)param_7,(uint)(param_7 >> 0x10));
  iStack40 = (int)uVar5;
  uStack38 = param_2;
  puVar8 = (undefined4 *)pass1_1030_5b5c(iStack40,param_2);
  uVar6 = (uint)((ulong)puVar8 >> 0x10);
  local_2e = *puVar8;
  uStack42 = *(undefined2 *)((int)puVar8 + 0x4);
  puStack78 = &local_2e;
  pass1_1008_3e94((ushort *)CONCAT22(param_1,&local_2e),(ushort *)CONCAT22(param_1,&local_24),
                  (ushort *)CONCAT22(param_1,&local_22));
  iStack14 = local_4 + 0x1;
  uStack12 = CONCAT22(local_4 + -0x1,local_6 - 0x1U);
  iStack16 = local_6 + 0x1;
  if (local_4 + -0x1 < 0x0) {
    uStack12 = (ulong)(local_6 - 0x1U);
  }
  if (local_22 <= iStack14) {
    iStack14 = local_22 + -0x1;
  }
  if ((int)(ushort)uStack12 < 0x0) {
    uStack12 = uStack12 & 0xffff0000;
  }
  if (local_24 <= iStack16) {
    iStack16 = local_24 + -0x1;
  }
  pass1_1008_6c90((ushort *)CONCAT22(param_1,local_3a));
  uVar7 = 0x1008;
  pass1_1008_6cec((ushort *)CONCAT22(param_1,local_3a),local_8,CONCAT22(iStack14,iStack16),local_8,uStack12);
  puVar3 = local_3a;
  pass1_1030_6522(_PTR_LOOP_1050_5740,CONCAT22(param_1,puVar3),param_7,param_1);
  puStack62 = (undefined4 *)CONCAT22(uVar6,puVar3);
  if ((uVar6 | (uint)puVar3) != 0x0) {
    uStack66 = 0x0;
    iStack68 = 0x0;
    for (uStack70 = (ushort)uStack12; (int)uStack70 <= iStack16; uStack70 = uStack70 + 0x1) {
      for (puStack78 = (undefined4 *)uStack12._2_2_; (int)puStack78 <= iStack14;
          puStack78 = (undefined4 *)((int)puStack78 + 0x1)) {
        ppcVar1 = (code **)((int)*puStack62 + 0x4);
        iVar4 = iStack68;
        iStack68 = iStack68 + 0x1;
        (**ppcVar1)(uVar7,(int)puStack62,(int)((ulong)puStack62 >> 0x10));
        uStack66 = CONCAT22(extraout_DX,iVar4);
        uStack66._3_1_ = (char)((uint)extraout_DX >> 0x8);
        if (uStack66._3_1_ == '\0') {
          iStack90 = iVar4;
          if (iVar4 == 0x7) {
            pass1_1008_3e76(param_6,local_8,uStack70,(ushort)puStack78);
            uVar10 = (undefined2)uStack32;
            uVar11 = (undefined2)((ulong)uStack32 >> 0x10);
            uVar9 = 0x6;
          }
          else {
            if (iVar4 == 0x8) {
              pass1_1008_3e76(param_6,local_8,uStack70,(ushort)puStack78);
              uVar10 = (undefined2)uStack32;
              uVar11 = (undefined2)((ulong)uStack32 >> 0x10);
              uVar9 = 0x7;
            }
            else {
              if (iVar4 != 0x9) goto LAB_1030_a1d0;
              pass1_1008_3e76(param_6,local_8,uStack70,(ushort)puStack78);
              uVar10 = (undefined2)uStack32;
              uVar11 = (undefined2)((ulong)uStack32 >> 0x10);
              uVar9 = 0x8;
            }
          }
          uVar7 = SUB42(&USHORT_1050_1028,0x0);
          struct_op_1028_87f0(param_1,param_3,(astruct_97 *)CONCAT22(param_1,&local_17e),0x0,0x0,uVar9,(ulong *)param_6,
                              (ushort)((ulong)param_6 >> 0x10),CONCAT22(uVar11,uVar10),param_7);
          fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,&local_17e));
          local_17e = 0x389a;
          uStack380 = 0x1008;
        }
LAB_1030_a1d0:
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_a278(ushort *param_1,ushort *param_2,ushort param_3,ushort param_4,uchar param_5)

{
  int iVar1;
  undefined4 uVar2;
  int in_DX;
  ushort extraout_DX;
  ulong *puVar3;
  ushort uVar4;
  uint uVar5;
  ushort uVar6;
  undefined local_134 [0x120];
  ulong uStack20;
  ulong uStack16;
  ulong uStack12;
  ushort uStack6;
  undefined2 uStack4;
  
  uStack4 = 0x1;
  pass1_1030_a39a((ulong)param_1,param_2,param_4);
  if (param_3 != 0x0) {
    return;
  }
  uStack6 = param_3;
  pass1_1030_a3ae((ulong)param_1,param_2,param_4);
  puVar3 = (ulong *)param_2;
  uVar5 = (uint)((ulong)param_2 >> 0x10);
  if (param_3 == 0x0) {
    pass1_1030_a57e((ulong)param_1,param_2,0x0,in_DX,param_4);
    if (param_3 == 0x0) {
      pass1_1030_a844((ulong)param_1,param_2,0x0,in_DX,param_4);
      if (param_3 == 0x0) {
        uStack4 = 0x0;
        goto LAB_1030_a305;
      }
      iVar1 = *(int *)(puVar3 + 0x1);
    }
    else {
      iVar1 = *(int *)(puVar3 + 0x1);
    }
    if (iVar1 < 0x1) {
      uStack6 = 0x73;
    }
    else {
      uStack6 = 0x77;
    }
  }
  else {
    if (*(int *)(puVar3 + 0x1) < 0x1) {
      uStack6 = 0x7a;
    }
    else {
      uStack6 = 0x7f;
    }
  }
LAB_1030_a305:
  if (uStack6 != 0x0) {
    uVar6 = (ushort)((ulong)param_1 >> 0x10);
    uVar4 = (ushort)param_1;
    uVar2 = *(undefined4 *)(uVar4 + 0x4);
    uStack16 = *(ulong *)((int)uVar2 + 0x8);
    uVar2 = *(undefined4 *)(uVar4 + 0x4);
    struct_op_1028_87f0(param_4,param_5,(astruct_97 *)CONCAT22(param_4,local_134),0x0,0x0,uStack6,puVar3,uVar5,
                        *(ulong *)((int)uVar2 + 0x4),uStack16);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_4,local_134));
    uStack12 = uStack20;
    pass1_1028_b58e(uStack20);
    *param_1 = (ushort)uStack20;
    *(ushort *)(uVar4 + 0x2) = extraout_DX;
    if (0x0 < *(int *)(puVar3 + 0x1)) {
      pass1_1030_a044(param_4,extraout_DX,param_5,uVar4,uVar6,(ushort *)((ulong)param_2 & 0xffff | (ulong)uVar5 << 0x10)
                      ,uStack16);
    }
  }
  return;
}



void __stdcall16far pass1_1030_a39a(ulong param_1,ushort *param_2,ushort param_3)

{
  pass1_1030_aa18(param_1,param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_a3ae(ulong param_1,ushort *param_2,ushort param_3)

{
  code **ppcVar1;
  undefined4 uVar2;
  uint uVar3;
  ushort uVar4;
  BOOL16 BVar5;
  ulong uVar6;
  uchar *puVar7;
  undefined2 extraout_DX;
  ushort uVar8;
  ushort uVar9;
  ushort uVar10;
  int iVar11;
  ushort uVar12;
  undefined2 uVar13;
  ulong *puVar14;
  ushort *puVar15;
  uint uVar16;
  ulong uStack44;
  int local_28;
  int local_26;
  int local_24;
  undefined local_22 [0x6];
  int local_1c;
  int iStack26;
  long lStack22;
  ulong uStack18;
  undefined4 *puStack14;
  uint uStack10;
  uchar *puStack8;
  int iStack6;
  undefined2 uStack4;
  
  uStack4 = 0x0;
  iStack6 = *(int *)((int)param_2 + 0x4);
  puVar14 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,0x45);
  puVar7 = (uchar *)((ulong)puVar14 >> 0x10);
  uVar3 = (uint)puVar14;
  uVar12 = (ushort)(param_1 >> 0x10);
  uVar10 = (ushort)param_1;
  uStack10 = uVar3;
  puStack8 = puVar7;
  pass1_1038_4e78(uVar3,puVar7,*(ulong *)(uVar10 + 0x4),puVar14);
  puStack14 = (undefined4 *)CONCAT22(puVar7,uVar3);
  ppcVar1 = (code **)((int)*puStack14 + 0x10);
  uVar16 = uVar3;
  (**ppcVar1)((int)&PTR_LOOP_1050_1038,uVar3,puVar7);
  uStack18 = CONCAT22(extraout_DX,uVar3);
  uVar2 = *(undefined4 *)(uVar10 + 0x4);
  lStack22 = *(long *)((int)uVar2 + 0x8);
  pass1_1008_3e38((ushort *)CONCAT22(param_3,&local_1c));
  puVar15 = pass1_1008_3e38((ushort *)CONCAT22(param_3,local_22));
  uStack44 = 0x0;
  uVar8 = (ushort)((ulong)puVar15 >> 0x10);
  do {
    if (uStack18 <= uStack44) {
LAB_1030_a4e7:
      if (puStack14 != (undefined4 *)0x0) {
        ppcVar1 = (code **)*puStack14;
        (**ppcVar1)(0x1008,(int)puStack14,(char)((ulong)puStack14 >> 0x10),0x1,uVar16,puVar7,puStack14,puStack14);
      }
      return;
    }
    uVar6 = uStack18;
    pass1_1030_1d58((ulong)puStack14);
    uVar9 = uVar8 | (uint)uVar6;
    if (uVar9 != 0x0) {
      pass1_1008_3f62((ushort *)CONCAT22(param_3,&local_1c),(ushort *)CONCAT22(uVar8,(uint)uVar6 + 0xc));
      pass1_1008_3eb4((ushort *)CONCAT22(param_3,&local_1c),(ushort *)CONCAT22(param_3,&local_28),
                      (ushort *)CONCAT22(param_3,&local_26),(ushort *)CONCAT22(param_3,&local_24));
      uVar9 = uVar8;
      if ((local_28 == iStack6) &&
         (uVar2 = *(undefined4 *)(uVar10 + 0x4), uVar13 = (undefined2)((ulong)uVar2 >> 0x10), iVar11 = (int)uVar2,
         uVar2 = *(undefined4 *)(iVar11 + 0x4),
         uVar4 = pass1_1030_addc(uVar10,uVar12,(ushort *)CONCAT22(param_3,&local_1c),(ushort)uVar2,
                                 (ushort)((ulong)uVar2 >> 0x10),*(ulong *)(iVar11 + 0x8),(int)&local_1c,uVar8,param_3),
         uVar9 = uVar8, uVar4 != 0x0)) {
        pass1_1008_3f62((ushort *)CONCAT22(param_3,local_22),(ushort *)CONCAT22(param_3,&local_1c));
        iStack26 = local_26 + -0x1;
        BVar5 = pass1_1030_ad22(uVar10,uVar12,(ushort *)CONCAT22(param_3,&local_1c),lStack22,&local_1c,uVar8,param_3);
        if (BVar5 == 0x0) {
          iStack26 = local_26 + 0x1;
          BVar5 = pass1_1030_ad22(uVar10,uVar12,(ushort *)CONCAT22(param_3,&local_1c),lStack22,&local_1c,uVar8,param_3);
          if (BVar5 == 0x0) {
            iStack26 = local_26;
            local_1c = local_24 + -0x1;
            BVar5 = pass1_1030_ad22(uVar10,uVar12,(ushort *)CONCAT22(param_3,&local_1c),lStack22,&local_1c,uVar8,param_3
                                   );
            if (BVar5 == 0x0) {
              local_1c = local_24 + 0x1;
              BVar5 = pass1_1030_ad22(uVar10,uVar12,(ushort *)CONCAT22(param_3,&local_1c),lStack22,&local_1c,uVar8,
                                      param_3);
              uVar9 = uVar8;
              if (BVar5 == 0x0) goto LAB_1030_a45b;
            }
          }
        }
        pass1_1008_3f62(param_2,(ushort *)CONCAT22(param_3,local_22));
        uStack4 = 0x1;
        goto LAB_1030_a4e7;
      }
    }
LAB_1030_a45b:
    uStack44 = uStack44 + 0x1;
    uVar8 = uVar9;
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_a57e(ulong param_1,ushort *param_2,int param_3,int param_4,ushort param_5)

{
  ulong uVar1;
  code **ppcVar2;
  undefined4 uVar3;
  ushort uVar4;
  int *piVar5;
  ulong uVar6;
  uchar *puVar7;
  uint extraout_DX;
  uint uVar8;
  ushort uVar9;
  ushort uVar10;
  undefined2 uVar11;
  int iVar12;
  undefined4 *puVar13;
  ushort uVar14;
  undefined2 uVar15;
  undefined2 uVar16;
  undefined2 uVar17;
  ulong *puVar18;
  ulong uVar19;
  undefined uVar20;
  ulong uStack40;
  undefined local_1c [0x2];
  int local_1a;
  int local_18;
  int local_16;
  int iStack20;
  undefined4 uStack16;
  uint uStack12;
  uchar *puStack10;
  int iStack8;
  int iStack6;
  undefined2 uStack4;
  
  uStack4 = 0x0;
  uVar14 = (ushort)(param_1 >> 0x10);
  uVar10 = (ushort)param_1;
  pass1_1038_53ba(*(ulong *)(uVar10 + 0x4),0x1);
  if ((param_4 != 0x0) || (param_3 != 0x0)) {
    iStack6 = *(int *)((int)param_2 + 0x4);
    iStack8 = 0x8 - (uint)(iStack6 == 0x0);
    puVar18 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,iStack8);
    puVar7 = (uchar *)((ulong)puVar18 >> 0x10);
    uVar8 = (uint)puVar18;
    uStack12 = uVar8;
    puStack10 = puVar7;
    pass1_1038_4e78(uVar8,puVar7,*(ulong *)(uVar10 + 0x4),puVar18);
    uStack16 = (undefined4 *)CONCAT22(puVar7,uVar8);
    uVar17 = 0x1008;
    pass1_1008_3e38((ushort *)CONCAT22(param_5,&local_16));
    uVar3 = *(undefined4 *)(uVar10 + 0x4);
    uVar1 = *(ulong *)((int)uVar3 + 0x8);
    uVar15 = (undefined2)((ulong)uStack16 >> 0x10);
    uVar11 = SUB42(uStack16,0x0);
    ppcVar2 = (code **)((int)*uStack16 + 0x10);
    uVar6 = uVar1;
    (**ppcVar2)(0x1008,uVar11,uVar15);
    uVar6 = uVar6 & 0xffff | (ulong)extraout_DX << 0x10;
    uVar8 = extraout_DX;
    for (uStack40 = 0x0; uStack40 < uVar6; uStack40 = uStack40 + 0x1) {
      uVar19 = uVar6;
      pass1_1030_1d58((ulong)uStack16);
      uVar9 = uVar8 | (uint)uVar19;
      if (uVar9 != 0x0) {
        uVar9 = uVar8;
        pass1_1008_3f62((ushort *)CONCAT22(param_5,&local_16),(ushort *)CONCAT22(uVar8,(uint)uVar19 + 0xc));
        uVar17 = 0x1008;
        pass1_1008_3eb4((ushort *)CONCAT22(param_5,&local_16),(ushort *)CONCAT22(param_5,local_1c),
                        (ushort *)CONCAT22(param_5,&local_1a),(ushort *)CONCAT22(param_5,&local_18));
        uVar3 = *(undefined4 *)(uVar10 + 0x4);
        uVar16 = (undefined2)((ulong)uVar3 >> 0x10);
        iVar12 = (int)uVar3;
        uVar3 = *(undefined4 *)(iVar12 + 0x4);
        uVar4 = pass1_1030_addc(uVar10,uVar14,(ushort *)CONCAT22(param_5,&local_16),(ushort)uVar3,
                                (ushort)((ulong)uVar3 >> 0x10),*(ulong *)(iVar12 + 0x8),(int)&local_16,uVar9,param_5);
        if (uVar4 == 0x0) goto LAB_1030_a660;
        uVar19 = struct_op_1030_73a8(uVar19 & 0xffff | (ulong)uVar8 << 0x10);
        uVar9 = (ushort)(uVar19 >> 0x10);
        iVar12 = *(int *)((int)uVar19 + 0xc);
        if (0x5 < iVar12 - 0x7aU) goto LAB_1030_a660;
        uVar17 = 0x1030;
        switch(iVar12) {
        default:
          iStack20 = local_1a + -0x1;
          piVar5 = &local_16;
          pass1_1030_ad86(uVar10,uVar14,(ushort *)CONCAT22(param_5,piVar5),uVar1,param_5,uVar9);
          if (piVar5 != (int *)0x0) goto LAB_1030_a7df;
          iStack20 = local_1a + 0x1;
          piVar5 = &local_16;
          pass1_1030_ad86(uVar10,uVar14,(ushort *)CONCAT22(param_5,piVar5),uVar1,param_5,uVar9);
          if (piVar5 == (int *)0x0) {
            iStack20 = local_1a;
            local_16 = local_18 + -0x1;
            piVar5 = &local_16;
            pass1_1030_ad86(uVar10,uVar14,(ushort *)CONCAT22(param_5,piVar5),uVar1,param_5,uVar9);
            goto joined_r0x1030a722;
          }
LAB_1030_a748:
          pass1_1008_3f62(param_2,(ushort *)CONCAT22(param_5,&local_16));
          break;
        case 0x7b:
        case 0x7e:
          iStack20 = local_1a + -0x1;
          piVar5 = &local_16;
          pass1_1030_ad86(uVar10,uVar14,(ushort *)CONCAT22(param_5,piVar5),uVar1,param_5,uVar9);
          if (piVar5 == (int *)0x0) {
            iStack20 = local_1a + 0x1;
            goto LAB_1030_a730;
          }
          pass1_1008_3f62(param_2,(ushort *)CONCAT22(param_5,&local_16));
          if (uStack16 == (undefined4 *)0x0) {
            return;
          }
          uVar17 = (undefined2)((ulong)uStack16 >> 0x10);
          puVar13 = (undefined4 *)uStack16;
          uVar20 = (undefined)((ulong)uStack16 >> 0x10);
          goto LAB_1030_a6ea;
        case 0x7c:
        case 0x7d:
          local_16 = local_18 + -0x1;
          piVar5 = &local_16;
          pass1_1030_ad86(uVar10,uVar14,(ushort *)CONCAT22(param_5,piVar5),uVar1,param_5,uVar9);
joined_r0x1030a722:
          if (piVar5 == (int *)0x0) {
            local_16 = local_18 + 0x1;
LAB_1030_a730:
            piVar5 = &local_16;
            pass1_1030_ad86(uVar10,uVar14,(ushort *)CONCAT22(param_5,piVar5),uVar1,param_5,uVar9);
            if (piVar5 != (int *)0x0) goto LAB_1030_a748;
            goto LAB_1030_a660;
          }
LAB_1030_a7df:
          pass1_1008_3f62(param_2,(ushort *)CONCAT22(param_5,&local_16));
        }
        puVar13 = (undefined4 *)uStack16;
        if ((uStack16._2_2_ | (uint)puVar13) != 0x0) {
          uVar17 = (undefined2)((ulong)uStack16 >> 0x10);
          uVar20 = (undefined)((ulong)uStack16 >> 0x10);
LAB_1030_a6ea:
          ppcVar2 = (code **)*puVar13;
          (**ppcVar2)(0x1008,puVar13,uVar20,0x1,uVar11,uVar15,uStack16,uStack16);
        }
        return;
      }
LAB_1030_a660:
      uVar8 = uVar9;
    }
    if (uStack16 != (undefined4 *)0x0) {
      ppcVar2 = (code **)*uStack16;
      (**ppcVar2)(uVar17,(int)uStack16,(char)((ulong)uStack16 >> 0x10),0x1,uVar11,uVar15,uStack16,uStack16);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_a844(ulong param_1,ushort *param_2,int param_3,int param_4,ushort param_5)

{
  int iVar1;
  undefined4 uVar2;
  code **ppcVar3;
  uint uVar4;
  ushort uVar5;
  int *piVar6;
  undefined4 *puVar7;
  uint extraout_DX;
  uint uVar9;
  uint uVar10;
  astruct_426 *uVar8;
  astruct_427 *iVar8;
  int iVar11;
  ushort uVar12;
  undefined2 uVar13;
  ushort *puVar14;
  ulong uVar15;
  ulong uStack34;
  int local_1c;
  int local_1a;
  int local_18;
  int local_16;
  int iStack20;
  undefined2 uStack16;
  long lStack14;
  ulong uStack10;
  undefined4 *puStack6;
  
  uVar12 = (ushort)(param_1 >> 0x10);
  uVar8 = (astruct_426 *)param_1;
  pass1_1038_53ba(uVar8->field_0x4,0x1);
  if ((param_4 != 0x0) || (param_3 != 0x0)) {
    uVar15 = uVar8->field_0x4;
    uVar13 = (undefined2)(uVar15 >> 0x10);
    iVar8 = (astruct_427 *)uVar15;
    puVar7 = iVar8->field_0xc;
    ppcVar3 = (code **)((int)*puVar7 + 0x10);
    puStack6 = puVar7;
    (**ppcVar3)((int)&PTR_LOOP_1050_1038,(int)puVar7,*(undefined2 *)((int)&iVar8->field_0xc + 0x2));
    uStack10 = (ulong)puVar7 & 0xffff | (ulong)extraout_DX << 0x10;
    uVar15 = uVar8->field_0x4;
    lStack14 = *(long *)((int)uVar15 + 0x8);
    uStack16 = 0x0;
    puVar14 = pass1_1008_3e38((ushort *)CONCAT22(param_5,&local_16));
    uVar9 = (uint)((ulong)puVar14 >> 0x10);
    iVar1 = *(int *)((int)param_2 + 0x4);
    for (uStack34 = 0x0; uStack34 < uStack10; uStack34 = uStack34 + 0x1) {
      uVar15 = pass1_1030_1d7c((int)uStack10,uVar9,(ulong)puStack6);
      uVar4 = (uint)(uVar15 >> 0x10);
      uVar10 = uVar4 | (uint)uVar15;
      uVar9 = uVar10;
      if ((uVar10 != 0x0) &&
         (uVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,*(undefined2 *)((uint)uVar15 + 0xc),0x46), uVar9 = uVar10,
         uVar4 != 0x0)) {
        pass1_1030_1d58((ulong)puStack6);
        uVar9 = uVar10 | uVar4;
        if ((uVar10 | uVar4) != 0x0) {
          pass1_1008_3f62((ushort *)CONCAT22(param_5,&local_16),(ushort *)CONCAT22(uVar10,uVar4 + 0xc));
          pass1_1008_3eb4((ushort *)CONCAT22(param_5,&local_16),(ushort *)CONCAT22(param_5,&local_1c),
                          (ushort *)CONCAT22(param_5,&local_1a),(ushort *)CONCAT22(param_5,&local_18));
          uVar9 = uVar10;
          if ((iVar1 == local_1c) &&
             (uVar15 = uVar8->field_0x4, uVar13 = (undefined2)(uVar15 >> 0x10), iVar11 = (int)uVar15,
             uVar2 = *(undefined4 *)(iVar11 + 0x4),
             uVar5 = pass1_1030_addc((ushort)uVar8,uVar12,(ushort *)CONCAT22(param_5,&local_16),(ushort)uVar2,
                                     (ushort)((ulong)uVar2 >> 0x10),*(ulong *)(iVar11 + 0x8),(int)&local_16,uVar10,
                                     param_5), uVar9 = uVar10, uVar5 != 0x0)) {
            iStack20 = local_1a + -0x1;
            piVar6 = &local_16;
            pass1_1030_ad86((ushort)uVar8,uVar12,(ushort *)CONCAT22(param_5,piVar6),lStack14,param_5,uVar10);
            if (piVar6 != (int *)0x0) {
LAB_1030_a98e:
              pass1_1008_3f62(param_2,(ushort *)CONCAT22(param_5,&local_16));
              return;
            }
            iStack20 = local_1a + 0x1;
            piVar6 = &local_16;
            pass1_1030_ad86((ushort)uVar8,uVar12,(ushort *)CONCAT22(param_5,piVar6),lStack14,param_5,uVar10);
            if (piVar6 != (int *)0x0) goto LAB_1030_a98e;
            iStack20 = local_1a;
            local_16 = local_18 + -0x1;
            piVar6 = &local_16;
            pass1_1030_ad86((ushort)uVar8,uVar12,(ushort *)CONCAT22(param_5,piVar6),lStack14,param_5,uVar10);
            if (piVar6 != (int *)0x0) goto LAB_1030_a98e;
            local_16 = local_18 + 0x1;
            piVar6 = &local_16;
            pass1_1030_ad86((ushort)uVar8,uVar12,(ushort *)CONCAT22(param_5,piVar6),lStack14,param_5,uVar10);
            uVar9 = uVar10;
            if (piVar6 != (int *)0x0) goto LAB_1030_a98e;
          }
        }
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1030_aa18(ulong param_1,ushort *param_2,ushort param_3)

{
  ulong uVar1;
  code **ppcVar2;
  undefined4 uVar3;
  ushort uVar4;
  BOOL16 BVar5;
  ulong uVar6;
  uchar *puVar7;
  uint extraout_DX;
  uint uVar8;
  uint uVar9;
  ushort uVar10;
  undefined2 uVar11;
  int iVar12;
  undefined4 *puVar13;
  ushort uVar14;
  undefined2 uVar15;
  undefined2 uVar16;
  undefined2 uVar17;
  ulong *puVar18;
  ulong uVar19;
  undefined uVar20;
  ulong uStack38;
  undefined local_1a [0x2];
  int local_18;
  int local_16;
  int local_14;
  int iStack18;
  undefined4 uStack14;
  uint uStack10;
  uchar *puStack8;
  int iStack6;
  int iStack4;
  
  iStack4 = *(int *)((int)param_2 + 0x4);
  iStack6 = 0x8 - (uint)(iStack4 == 0x0);
  puVar18 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,iStack6);
  puVar7 = (uchar *)((ulong)puVar18 >> 0x10);
  uVar8 = (uint)puVar18;
  uVar14 = (ushort)(param_1 >> 0x10);
  uVar10 = (ushort)param_1;
  uStack10 = uVar8;
  puStack8 = puVar7;
  pass1_1038_4e78(uVar8,puVar7,*(ulong *)(uVar10 + 0x4),puVar18);
  uStack14 = (undefined4 *)CONCAT22(puVar7,uVar8);
  uVar17 = 0x1008;
  pass1_1008_3e38((ushort *)CONCAT22(param_3,&local_14));
  uVar3 = *(undefined4 *)(uVar10 + 0x4);
  uVar1 = *(ulong *)((int)uVar3 + 0x8);
  uVar15 = (undefined2)((ulong)uStack14 >> 0x10);
  uVar11 = SUB42(uStack14,0x0);
  ppcVar2 = (code **)((int)*uStack14 + 0x10);
  uVar6 = uVar1;
  (**ppcVar2)(0x1008,uVar11,uVar15);
  uVar6 = uVar6 & 0xffff | (ulong)extraout_DX << 0x10;
  uStack38 = 0x0;
  uVar8 = extraout_DX;
  while( true ) {
    if (uVar6 <= uStack38) {
      if (uStack14 != (undefined4 *)0x0) {
        ppcVar2 = (code **)*uStack14;
        (**ppcVar2)(uVar17,(int)uStack14,(char)((ulong)uStack14 >> 0x10),0x1,uVar11,uVar15,uStack14,uStack14);
      }
      return;
    }
    uVar19 = uVar6;
    pass1_1030_1d58((ulong)uStack14);
    uVar9 = uVar8 | (uint)uVar19;
    if (uVar9 != 0x0) break;
LAB_1030_aadc:
    uStack38 = uStack38 + 0x1;
    uVar8 = uVar9;
  }
  uVar9 = uVar8;
  pass1_1008_3f62((ushort *)CONCAT22(param_3,&local_14),(ushort *)CONCAT22(uVar8,(uint)uVar19 + 0xc));
  uVar17 = 0x1008;
  pass1_1008_3eb4((ushort *)CONCAT22(param_3,&local_14),(ushort *)CONCAT22(param_3,local_1a),
                  (ushort *)CONCAT22(param_3,&local_18),(ushort *)CONCAT22(param_3,&local_16));
  uVar3 = *(undefined4 *)(uVar10 + 0x4);
  uVar16 = (undefined2)((ulong)uVar3 >> 0x10);
  iVar12 = (int)uVar3;
  uVar3 = *(undefined4 *)(iVar12 + 0x4);
  uVar4 = pass1_1030_addc(uVar10,uVar14,(ushort *)CONCAT22(param_3,&local_14),(ushort)uVar3,
                          (ushort)((ulong)uVar3 >> 0x10),*(ulong *)(iVar12 + 0x8),(int)&local_14,uVar9,param_3);
  if (uVar4 == 0x0) goto LAB_1030_aadc;
  uVar19 = struct_op_1030_73a8(uVar19 & 0xffff | (ulong)uVar8 << 0x10);
  uVar9 = (uint)(uVar19 >> 0x10);
  iVar12 = *(int *)((int)uVar19 + 0xc);
  if (0x5 < iVar12 - 0x7aU) goto LAB_1030_aadc;
  uVar17 = 0x1030;
  switch(iVar12) {
  default:
    iStack18 = local_18 + -0x1;
    BVar5 = pass1_1030_acbe(uVar10,uVar14,(ushort *)CONCAT22(param_3,&local_14),uVar1,(uint)&local_14,uVar9,param_3);
    if (BVar5 != 0x0) goto LAB_1030_ac5b;
    iStack18 = local_18 + 0x1;
    BVar5 = pass1_1030_acbe(uVar10,uVar14,(ushort *)CONCAT22(param_3,&local_14),uVar1,(uint)&local_14,uVar9,param_3);
    if (BVar5 == 0x0) {
      iStack18 = local_18;
      local_14 = local_16 + -0x1;
      BVar5 = pass1_1030_acbe(uVar10,uVar14,(ushort *)CONCAT22(param_3,&local_14),uVar1,(uint)&local_14,uVar9,param_3);
      goto joined_r0x1030ab9e;
    }
LAB_1030_abc4:
    pass1_1008_3f62(param_2,(ushort *)CONCAT22(param_3,&local_14));
    break;
  case 0x7b:
  case 0x7e:
    iStack18 = local_18 + -0x1;
    BVar5 = pass1_1030_acbe(uVar10,uVar14,(ushort *)CONCAT22(param_3,&local_14),uVar1,(uint)&local_14,uVar9,param_3);
    if (BVar5 == 0x0) {
      iStack18 = local_18 + 0x1;
      goto LAB_1030_abac;
    }
    pass1_1008_3f62(param_2,(ushort *)CONCAT22(param_3,&local_14));
    if (uStack14 == (undefined4 *)0x0) {
      return;
    }
    uVar17 = (undefined2)((ulong)uStack14 >> 0x10);
    puVar13 = (undefined4 *)uStack14;
    uVar20 = (undefined)((ulong)uStack14 >> 0x10);
    goto LAB_1030_ab66;
  case 0x7c:
  case 0x7d:
    local_14 = local_16 + -0x1;
    BVar5 = pass1_1030_acbe(uVar10,uVar14,(ushort *)CONCAT22(param_3,&local_14),uVar1,(uint)&local_14,uVar9,param_3);
joined_r0x1030ab9e:
    if (BVar5 == 0x0) {
      local_14 = local_16 + 0x1;
LAB_1030_abac:
      BVar5 = pass1_1030_acbe(uVar10,uVar14,(ushort *)CONCAT22(param_3,&local_14),uVar1,(uint)&local_14,uVar9,param_3);
      if (BVar5 != 0x0) goto LAB_1030_abc4;
      goto LAB_1030_aadc;
    }
LAB_1030_ac5b:
    pass1_1008_3f62(param_2,(ushort *)CONCAT22(param_3,&local_14));
  }
  puVar13 = (undefined4 *)uStack14;
  if ((uStack14._2_2_ | (uint)puVar13) != 0x0) {
    uVar17 = (undefined2)((ulong)uStack14 >> 0x10);
    uVar20 = (undefined)((ulong)uStack14 >> 0x10);
LAB_1030_ab66:
    ppcVar2 = (code **)*puVar13;
    (**ppcVar2)(0x1008,puVar13,uVar20,0x1,uVar11,uVar15,uStack14,uStack14);
  }
  return;
}
