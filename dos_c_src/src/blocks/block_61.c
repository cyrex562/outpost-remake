
astruct_18 * __stdcall16far pass1_1020_e868(astruct_18 *param_1,byte param_2)

{
  pass1_1020_e846(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1020_e8f6(ushort *param_1)

{
  undefined2 uVar1;
  
  struct_1030_dc96(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  *(undefined2 *)((int)param_1 + 0x24) = 0x0;
  *param_1 = 0xeef6;
  *(undefined2 *)((int)param_1 + 0x2) = 0x1020;
  return param_1;
}



ushort * __stdcall16far pass1_1020_e91e(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1030_dcc2(param_1,param_2,param_3,param_4,param_5);
  *(undefined2 *)(param_1 + 0x24) = 0x0;
  *(ushort *)CONCAT22(param_2,param_1) = 0xeef6;
  *(undefined2 *)(param_1 + 0x2) = 0x1020;
  return (ushort *)CONCAT22(param_2,param_1);
}



BOOL16 __stdcall16far pass1_1020_e94e(ulong param_1,ulong param_2,ushort param_3)

{
  BOOL16 in_AX;
  BOOL16 BVar1;
  undefined2 local_c [0x5];
  
  pass1_1030_de7c(param_1,param_2,param_3);
  if (in_AX != 0x0) {
    local_c[0] = *(undefined2 *)((int)param_1 + 0x24);
    BVar1 = write_to_file_1008_7e1c
                      ((ushort)param_2,(ushort)(param_2 >> 0x10),(ushort)local_c,param_3,(char *)0x2,0x1008);
    if (BVar1 == 0x0) {
      PTR_LOOP_1050_0310 = (undefined *)0x6d0;
      return BVar1;
    }
    in_AX = 0x1;
  }
  return in_AX;
}



void __stdcall16far pass1_1020_e994(ulong param_1,ulong param_2,int param_3,uchar *param_4,ushort param_5)

{
  BOOL16 BVar1;
  
  pass1_1030_dec4(param_1,param_2,param_3,param_4,param_5);
  if ((param_3 != 0x0) &&
     (BVar1 = read_file_1008_7dee((ushort)param_2,(ushort)(param_2 >> 0x10),(int)param_1 + 0x24,0x0,
                                  (ushort)(param_1 >> 0x10),0x2,0x1008), BVar1 == 0x0)) {
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
    return;
  }
  return;
}



void __stdcall16far pass1_1020_e9d4(ushort param_1,ushort param_2,ushort param_3)

{
  undefined2 extraout_DX;
  
  pass1_1030_df0c(CONCAT22(param_2,param_1),param_3);
  pass1_1028_b58e(CONCAT22(param_2,param_1));
  pass1_1038_57dc(*(ulong *)(param_3 + 0x2e),0x1,0x1);
  return;
}



void __stdcall16far pass1_1020_ea0e(ulong *param_1)

{
  pass1_1028_bdac(param_1,0x1,(ushort)&USHORT_1050_1028);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1020_ea20(ulong param_1,ushort *param_2,ulong param_3,ulong param_4,uint param_5,ushort param_6,uchar param_7,
               ushort param_8)

{
  uint uVar1;
  code **ppcVar2;
  ushort uVar3;
  char cVar4;
  ulong *puVar5;
  ushort uVar6;
  uchar *puVar7;
  uchar *extraout_DX;
  int unaff_DI;
  undefined2 uVar8;
  ushort uVar9;
  undefined local_146 [0x10c];
  uint uStack58;
  uchar *puStack56;
  ulong uStack50;
  ushort *puStack46;
  undefined4 *puStack42;
  undefined4 uStack38;
  undefined4 uStack34;
  ulong uStack28;
  ulong local_12;
  int iStack14;
  undefined4 *puStack12;
  undefined4 uStack8;
  BOOL16 BStack4;
  
  uVar9 = (ushort)param_1;
  uVar3 = (ushort)(param_1 >> 0x10);
  pass1_1028_c3aa(uVar9,uVar3,param_2,param_3,param_4,param_6);
  if (param_5 == 0x0) {
    return;
  }
  pass1_1028_c23e(uVar9,uVar3,param_2,param_3,param_4,param_5,param_8,param_6);
  if (param_5 == 0x0) {
    return;
  }
  BStack4 = pass1_1028_c314(param_6,param_5,param_8,uVar9,uVar3,param_2,(ushort)param_3,(ushort)(param_3 >> 0x10),
                            param_4);
  if (BStack4 == 0x0) {
    return;
  }
  pass1_1028_c7b6(param_6,param_8,uVar9,uVar3,param_2,param_4);
  if ((((BStack4 == 0x5) || (BStack4 == 0x4)) || (BStack4 == 0x6)) || (BStack4 == 0x9)) {
    PTR_LOOP_1050_50ca = (undefined *)0x6a8;
    return;
  }
  if (BStack4 != 0x0) {
    return;
  }
  puVar5 = &local_12;
  pass1_1030_64ce(param_6,puVar5,param_8,_PTR_LOOP_1050_5740,param_2,param_4,(ulong *)CONCAT22(param_6,puVar5));
  uStack38 = *puVar5;
  puStack56 = *(uchar **)((int)puVar5 + 0x2);
  uStack38._3_1_ = (byte)(uStack38 >> 0x18);
  uStack58 = (uint)uStack38._3_1_;
  uStack28 = uStack38;
  uStack8 = uStack38;
  if (uStack38._3_1_ == 0x0) goto LAB_1020_eb4e;
  uStack8._0_2_ = (ushort)uStack38;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack8,(uint)puStack56);
  uStack38 = CONCAT22(puStack56,uStack58);
  uStack34 = *(undefined4 *)(uStack58 + 0x2e);
  if (*(ulong *)((int)uStack34 + 0x4) != param_3) {
    PTR_LOOP_1050_50ca = (undefined *)0x6b7;
    return;
  }
  uStack28 = struct_op_1030_73a8(CONCAT22(puStack56,uStack58));
  puStack56 = (uchar *)(uStack28 >> 0x10);
  uVar1 = *(uint *)((int)uStack28 + 0xc);
  uStack58 = uVar1;
  if (uVar1 != 0x41) {
    if (0x41 < (int)uVar1) {
      if (uVar1 == 0x6b) {
        PTR_LOOP_1050_50ca = (undefined *)0x6b1;
        return;
      }
      if ((int)uVar1 < 0x6c) {
        if (uVar1 == 0x42) {
          PTR_LOOP_1050_50ca = (undefined *)0x6b1;
          return;
        }
        uStack58 = uVar1 - 0x4b;
        if (uStack58 == 0x0) {
          PTR_LOOP_1050_50ca = (undefined *)0x6b1;
          return;
        }
      }
      else {
        if (uVar1 == 0x6e) {
          return;
        }
        uStack58 = uVar1 - 0x73;
        if ((0x4 < (int)(uVar1 - 0x6e)) && (uStack58 = uVar1 - 0x79, uStack58 == 0x0 || (int)(uVar1 - 0x73) < 0x6)) {
          PTR_LOOP_1050_50ca = (undefined *)0x6b0;
          return;
        }
      }
      goto LAB_1020_eb4e;
    }
    if (uVar1 != 0x3e) {
      if (uVar1 < 0x3f) {
        cVar4 = (char)uVar1;
        if (cVar4 != '\v') {
          if (cVar4 == '\x10') {
            return;
          }
          uStack58 = uVar1 & 0xff00 | (uint)(byte)(cVar4 - 0x37U);
          if ((byte)(cVar4 - 0x37U) != 0x0) goto LAB_1020_eb4e;
        }
        PTR_LOOP_1050_50ca = (undefined *)0x6b4;
        return;
      }
      goto LAB_1020_eb4e;
    }
  }
  if (*(int *)((int)uStack28 + 0x12) == 0x4) {
    PTR_LOOP_1050_50ca = (undefined *)0x6b1;
    return;
  }
LAB_1020_eb4e:
  uVar8 = 0x1000;
  mem_op_1000_179c(0xb4,puStack56,0x1000);
  puVar7 = (uchar *)((uint)puStack56 | uStack58);
  if (puVar7 == (uchar *)0x0) {
    iStack14 = 0x0;
    puVar7 = (uchar *)0x0;
  }
  else {
    uVar8 = SUB42(&PTR_LOOP_1050_1040,0x0);
    iStack14 = string_1040_8520((astruct_57 *)
                                CONCAT13((char)((uint)puStack56 >> 0x8),CONCAT12((char)puStack56,uStack58)),
                                (ushort)PTR_LOOP_1050_0396,0x24,0x2,0x57b,0x5e8,puVar7,param_6);
  }
  puStack12 = (undefined4 *)CONCAT22(puVar7,iStack14);
  ppcVar2 = (code **)((int)*puStack12 + 0x74);
  (**ppcVar2)(uVar8,iStack14,puVar7);
  if (iStack14 != 0x7) {
    puStack46 = (ushort *)uStack8;
    uStack34 = uStack8;
    uStack34._3_1_ = (byte)(uStack8 >> 0x18);
    uVar6 = (ushort)uStack34._3_1_;
    if (uStack34._3_1_ != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uStack8,(uint)uStack8._2_2_);
      uStack50 = CONCAT22(uStack8._2_2_,uVar6);
      fn_ptr_1030_7296(CONCAT22(uStack8._2_2_,uVar6));
      pass1_1030_730a(uStack50,uVar6,0x1030,param_6);
      puStack46 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_6,uStack8._2_2_,unaff_DI);
      pass1_1010_ec68((ulong)puStack46,uStack50,param_6);
      puStack42 = (undefined4 *)struct_op_1030_73a8(uStack50);
      puVar5 = (ulong *)puStack42;
      ppcVar2 = (code **)((int)*puStack42 + 0x24);
      (**ppcVar2)(0x1030,(char)puStack42,(int)((ulong)puStack42 >> 0x10));
      uVar6 = pass1_1028_bc4a((ulong)puStack42,puVar5,extraout_DX,param_6);
      *(ushort *)(uVar9 + 0x24) = uVar6;
      struct_1030_e4fa((astruct_100 *)CONCAT22(param_6,local_146),*(ulong *)((int)uStack50 + 0x16),param_6,param_7);
      fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_6,local_146));
    }
    return;
  }
  PTR_LOOP_1050_50ca = (undefined *)((int)&PTR_LOOP_1050_0000 + 0x1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_ecb0(ulong param_1,int param_2,ushort param_3)

{
  undefined4 uVar1;
  int iVar2;
  uint uVar3;
  ushort unaff_SS;
  undefined2 uStack8;
  
  uVar3 = (uint)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  uVar1 = *(undefined4 *)(iVar2 + 0x8);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar1,(uint)((ulong)uVar1 >> 0x10));
  if (*(int *)(iVar2 + 0x12) == 0x1) {
    switch(*(undefined2 *)(param_2 + 0x14)) {
    case 0x2:
    case 0x7:
      uStack8 = 0x2;
      break;
    case 0x3:
    case 0x8:
      uStack8 = 0x3;
      break;
    default:
      uStack8 = *(undefined2 *)(param_2 + 0x14);
      break;
    case 0x5:
    case 0x6:
      uStack8 = 0x1;
    }
    *(undefined2 *)(iVar2 + 0x14) = uStack8;
    return;
  }
  pass1_1028_bf22(param_1 & 0xffff | (ulong)uVar3 << 0x10,(uchar *)param_3,unaff_SS);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_ed3c(ulong param_1,int param_2,ushort param_3,uchar param_4)

{
  int *piVar1;
  ushort uVar2;
  undefined2 extraout_DX;
  undefined2 extraout_DX_00;
  int iVar3;
  undefined2 uVar4;
  undefined local_138 [0x112];
  ulong uStack38;
  undefined4 *puStack30;
  ulong uStack28;
  undefined4 uStack24;
  ushort uStack20;
  int local_12;
  undefined local_10 [0x2];
  undefined local_e [0x2];
  undefined4 local_c;
  undefined2 uStack8;
  int iStack6;
  undefined2 uStack4;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  piVar1 = (int *)(iVar3 + 0x14);
  *piVar1 = *piVar1 + -0x1;
  if (*piVar1 == 0x0) {
    *(undefined2 *)(iVar3 + 0x12) = 0x0;
    pass1_1028_b58e(param_1);
    local_c = *(undefined4 *)(param_2 + 0xc);
    uStack8 = *(undefined2 *)(param_2 + 0x10);
    puStack30 = &local_c;
    iStack6 = param_2;
    uStack4 = extraout_DX;
    pass1_1008_3eb4((ushort *)CONCAT22(param_3,&local_c),(ushort *)CONCAT22(param_3,&local_12),
                    (ushort *)CONCAT22(param_3,local_10),(ushort *)CONCAT22(param_3,local_e));
    if (local_12 < 0x1) {
      puStack30 = (undefined4 *)0x5;
    }
    else {
      puStack30 = (undefined4 *)0x6;
    }
    *(undefined2 *)(iStack6 + 0x14) = puStack30;
    if (local_12 < 0x1) {
      uVar2 = 0x5;
    }
    else {
      uVar2 = 0x9;
    }
    uStack20 = uVar2;
    pass1_1020_ee3a(param_1,uVar2,uVar2,param_3,param_4);
    pass1_1028_b58e(param_1);
    uStack24 = CONCAT22(extraout_DX_00,uVar2);
    uStack28 = *(ulong *)(uVar2 + 0x2e);
    pass1_1038_5804(uStack28,0x1,0x1);
    if (0x0 < *(int *)(iVar3 + 0x24)) {
      uStack38 = *(ulong *)((int)uStack28 + 0x4);
      pass1_1028_68de((astruct_100 *)CONCAT22(param_3,local_138),*(ushort *)(iVar3 + 0x24),uStack38,param_4,param_3);
      fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_3,local_138));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1020_ee3a(ulong param_1,ushort param_2,int param_3,ushort param_4,uchar param_5)

{
  undefined2 extraout_DX;
  undefined local_13c [0x124];
  ulong uStack24;
  undefined4 uStack20;
  ulong uStack16;
  ulong local_c;
  undefined2 uStack8;
  int iStack6;
  undefined2 uStack4;
  
  pass1_1028_b58e(param_1);
  local_c = *(ulong *)(param_3 + 0xc);
  uStack8 = *(undefined2 *)(param_3 + 0x10);
  iStack6 = param_3;
  uStack4 = extraout_DX;
  uStack16 = pass1_1028_bb24(param_1);
  uStack20 = *(undefined4 *)(iStack6 + 0x2e);
  uStack24 = *(ulong *)((int)uStack20 + 0x4);
  struct_op_1028_87f0(param_4,param_5,(astruct_97 *)CONCAT22(param_4,local_13c),0x0,0x1,param_2,&local_c,param_4,
                      uStack24,uStack16);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_4,local_13c));
  return;
}



astruct_18 * __stdcall16far pass1_1020_eed0(astruct_18 *param_1,byte param_2,uint param_3)

{
  pass1_1030_dcf4(&param_1->field_0x0,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far pass1_1020_ef5e(ushort *param_1)

{
  *param_1 = 0x0;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  pass1_1028_b418(param_1);
  return;
}



astruct_18 * __stdcall16far pass1_1020_ef94(astruct_18 *param_1,byte param_2)

{
  pass1_1020_ef5e(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1028_0068(ushort *param_1,uchar *param_2)

{
  uint uVar1;
  undefined2 extraout_DX;
  astruct_183 *iVar2;
  undefined2 uVar2;
  
  struct_1028_b354(param_1);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (astruct_183 *)param_1;
  uVar1 = 0x0;
  iVar2->field_0x20 = 0x0;
  *(undefined4 *)&iVar2->field_0x22 = 0x0;
  *param_1 = 0x8ec;
  iVar2->field_0x2 = (int)&USHORT_1050_1028;
  mem_op_1000_179c(0xc,param_2,0x1000);
  if (((uint)param_2 | uVar1) == 0x0) {
    *(undefined4 *)&iVar2->field_0x22 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(param_2,uVar1));
    iVar2->field_0x22 = uVar1;
    iVar2->field_0x24 = extraout_DX;
  }
  return;
}



void __stdcall16far pass1_1028_00cc(int param_1,ushort param_2,int param_3,ulong param_4,uchar *param_5)

{
  uint uVar1;
  undefined2 extraout_DX;
  
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,(ushort)param_5);
  uVar1 = 0x0;
  *(undefined2 *)(param_1 + 0x20) = 0x0;
  *(undefined4 *)(param_1 + 0x22) = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x8ec;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  mem_op_1000_179c(0xc,param_5,0x1000);
  if (((uint)param_5 | uVar1) == 0x0) {
    *(undefined4 *)(param_1 + 0x22) = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(param_5,uVar1));
    *(uint *)(param_1 + 0x22) = uVar1;
    *(undefined2 *)(param_1 + 0x24) = extraout_DX;
  }
  return;
}



void __stdcall16far pass1_1028_0138(ushort *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0x8ec;
  *(undefined2 *)(iVar4 + 0x2) = (int)&USHORT_1050_1028;
  puVar1 = (undefined4 *)*(uint *)(iVar4 + 0x22);
  uVar2 = *(uint *)(iVar4 + 0x24);
  if ((uVar2 | (uint)puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1028_b418(param_1);
  return;
}



void __stdcall16far pass1_1028_0176(ulong param_1,ulong param_2,ushort param_3,ushort param_4)

{
  undefined4 *puVar1;
  uint uVar2;
  code **ppcVar3;
  ulong uVar4;
  uint uVar5;
  astruct_21 *paVar6;
  undefined4 uVar7;
  undefined2 uVar8;
  ushort uVar9;
  astruct_306 *iVar9;
  astruct_298 *iVar8;
  
  iVar9 = (astruct_306 *)param_1;
  uVar8 = (undefined2)(param_1 >> 0x10);
  pass1_1028_b46e(param_1,param_2,param_4);
  puVar1 = iVar9->field_0x22;
  uVar2 = iVar9->field_0x24;
  uVar5 = uVar2 | (uint)puVar1;
  paVar6 = (astruct_21 *)CONCAT22(uVar5,puVar1);
  if (uVar5 != 0x0) {
    ppcVar3 = (code **)*puVar1;
    paVar6 = (astruct_21 *)(**ppcVar3)();
  }
  mem_op_1000_179c(0xc,(uchar *)((ulong)paVar6 >> 0x10),0x1000);
  if (paVar6 == (astruct_21 *)0x0) {
    uVar7 = 0x0;
  }
  else {
    uVar7 = set_struct_1008_574a(paVar6);
  }
  iVar9->field_0x22 = (undefined4 *)uVar7;
  iVar9->field_0x24 = (uint)((ulong)uVar7 >> 0x10);
  uVar9 = 0x14;
  uVar4 = pass1_1028_b58e(param_1);
  pass1_1030_7f1a(uVar4,uVar9,param_3);
  return;
}



void __stdcall16far pass1_1028_01ec(ulong *param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  undefined4 uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((*(int *)(iVar2 + 0x12) == 0x6) || (*(int *)(iVar2 + 0x12) == 0x5)) {
    uVar1 = *(undefined4 *)(iVar2 + 0x14);
    uVar3 = (undefined2)((ulong)uVar1 >> 0x10);
    iVar2 = (int)uVar1;
    if ((*(int *)(iVar2 + 0xa6) == 0x14) || (*(int *)(iVar2 + 0xa8) == 0x10)) {
      pass1_1028_bdac(param_1,0x6,param_4);
      return;
    }
    pass1_1028_be2a(param_1,param_2,param_3,param_4,param_5);
  }
  return;
}



ushort __stdcall16far write_to_file_1028_0234(ulong param_1,ulong param_2,uint16_t param_3)

{
  undefined4 uVar1;
  BOOL16 BVar2;
  int iVar3;
  undefined2 uVar4;
  ushort uVar5;
  ushort uVar6;
  undefined2 local_1a [0x3];
  undefined2 local_14 [0x2];
  undefined2 uStack16;
  long lStack14;
  undefined local_a [0x8];
  
  BVar2 = write_to_file_1028_b5ec(param_1,param_2,param_3);
  if (BVar2 != 0x0) {
    uVar4 = (undefined2)(param_1 >> 0x10);
    iVar3 = (int)param_1;
    local_1a[0] = *(undefined2 *)(iVar3 + 0x20);
    uVar5 = (ushort)param_2;
    uVar6 = (ushort)(param_2 >> 0x10);
    BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_1a,param_3,(char *)0x2,0x1008);
    if (BVar2 != 0x0) {
      pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)(iVar3 + 0x22));
      uVar1 = *(undefined4 *)(iVar3 + 0x22);
      local_14[0] = *(undefined2 *)((int)uVar1 + 0x8);
      uStack16 = local_14[0];
      BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_14,param_3,(char *)0x2,0x1008);
      while (BVar2 != 0x0) {
        lStack14 = pass1_1008_5b12(local_a,param_3);
        if (lStack14 == 0x0) {
          return 0x1;
        }
        local_14[0] = *(undefined2 *)((int)lStack14 + 0x4);
        BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_14,param_3,(char *)0x2,0x1008);
        if (BVar2 == 0x0) break;
        local_14[0] = *(undefined2 *)((int)lStack14 + 0x6);
        BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_14,param_3,(char *)0x2,0x1008);
        if (BVar2 == 0x0) break;
        local_14[0] = *(undefined2 *)((int)lStack14 + 0x8);
        BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_14,param_3,(char *)0x2,0x1008);
        if (BVar2 == 0x0) break;
        local_14[0] = *(undefined2 *)((int)lStack14 + 0xa);
        BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_14,param_3,(char *)0x2,0x1008);
        if (BVar2 == 0x0) break;
        local_14[0] = *(undefined2 *)((int)lStack14 + 0xc);
        BVar2 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_14,param_3,(char *)0x2,0x1008);
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_0374(ulong param_1,ulong param_2,int param_3,uchar *param_4,ushort param_5)

{
  code **ppcVar1;
  ushort uVar3;
  BOOL16 BVar4;
  ushort uVar5;
  uint uVar6;
  undefined2 uVar7;
  ushort uVar8;
  ushort uVar9;
  undefined2 local_18 [0x2];
  astruct_99 *paStack20;
  undefined2 local_10 [0x2];
  ushort local_c [0x3];
  uint uStack6;
  uint local_4;
  astruct_728 *uVar2;
  
  file_1028_b81a(param_1,param_2,param_3,param_5,param_4);
  if (param_3 != 0x0) {
    uVar3 = (ushort)(param_1 >> 0x10);
    uVar8 = (ushort)param_2;
    uVar9 = (ushort)(param_2 >> 0x10);
    BVar4 = read_file_1008_7dee(uVar8,uVar9,(int)param_1 + 0x20,0x0,uVar3,0x2,0x1008);
    if (BVar4 != 0x0) {
      BVar4 = read_file_1008_7dee(uVar8,uVar9,(ushort)&local_4,0x0,param_5,0x2,0x1008);
      if (BVar4 != 0x0) {
        uStack6 = 0x0;
        while( true ) {
          if (local_4 <= uStack6) {
            return;
          }
          paStack20 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
          uVar6 = (uint)((ulong)paStack20 >> 0x10);
          uVar2 = (astruct_728 *)paStack20;
          if ((uVar6 | (uint)uVar2) == 0x0) {
            paStack20 = (astruct_99 *)0x0;
          }
          else {
            paStack20->field_0x0 = 0x389a;
            uVar2->field_0x2 = 0x1008;
            uVar2->field_0x4 = 0x0;
            uVar2->field_0x6 = 0x0;
            uVar2->field_0x8 = 0x0;
            uVar2->field_0xa = 0x0;
            uVar2->field_0xc = 0x0;
            paStack20->field_0x0 = 0x56ce;
            uVar2->field_0x2 = 0x1018;
          }
          BVar4 = read_file_1008_7dee(uVar8,uVar9,(ushort)local_10,0x0,param_5,0x2,0x1008);
          if (BVar4 == 0x0) break;
          BVar4 = read_file_1008_7dee(uVar8,uVar9,(ushort)local_c,0x0,param_5,0x2,0x1008);
          if (BVar4 == 0x0) break;
          BVar4 = read_file_1008_7dee(uVar8,uVar9,(ushort)local_18,0x0,param_5,0x2,0x1008);
          if (BVar4 == 0x0) break;
          BVar4 = read_file_1008_7dee(uVar8,uVar9,(int)paStack20 + 0xa,0x0,(ushort)((ulong)paStack20 >> 0x10),0x2,0x1008
                                     );
          if (BVar4 == 0x0) break;
          BVar4 = read_file_1008_7dee(uVar8,uVar9,(int)paStack20 + 0xc,0x0,(ushort)((ulong)paStack20 >> 0x10),0x2,0x1008
                                     );
          if (BVar4 == 0x0) break;
          *(undefined2 *)((int)paStack20 + 0x4) = local_10[0];
          uVar5 = switch_1008_72bc(uVar8,uVar9,local_c[0]);
          uVar7 = (undefined2)((ulong)paStack20 >> 0x10);
          *(ushort *)((int)paStack20 + 0x6) = uVar5;
          *(undefined2 *)((int)paStack20 + 0x8) = local_18[0];
          ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x22) + 0x8);
          (**ppcVar1)();
          uStack6 = uStack6 + 0x1;
        }
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return;
}



ushort __stdcall16far pass1_1028_04ee(ulong param_1,ulong *param_2,ushort param_3)

{
  int *piVar1;
  uint uVar2;
  uint uVar3;
  undefined2 uVar4;
  long lVar5;
  undefined local_a [0x8];
  
  *param_2 = 0x0;
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_a),*(ulong *)((int)param_1 + 0x22));
  do {
    lVar5 = pass1_1008_5b12(local_a,param_3);
    if (lVar5 == 0x0) {
      return 0x0;
    }
    uVar2 = *(uint *)((int)lVar5 + 0xc);
    uVar4 = (undefined2)((ulong)param_2 >> 0x10);
    uVar3 = *(uint *)param_2;
    *(uint *)param_2 = *(int *)param_2 + uVar2;
    piVar1 = (int *)((int)param_2 + 0x2);
    *piVar1 = *piVar1 + (uint)CARRY2(uVar3,uVar2);
  } while ((*(int *)((int)param_2 + 0x2) == 0x0) && (*(uint *)param_2 < 0x1e));
  return 0x1;
}



void __stdcall16far pass1_1028_0550(ulong *param_1,ushort param_2,ushort param_3,ushort param_4,ushort param_5)

{
  uint uVar1;
  ulong uVar2;
  undefined2 uVar3;
  undefined2 uVar4;
  int iVar5;
  
  pass1_1028_be9e(param_1,param_2,param_3,param_4,param_5);
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x12) == 0x5) {
    uVar4 = 0x0;
    iVar5 = 0x4;
    uVar3 = 0x1;
    uVar2 = pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
    pass1_1030_7c50(uVar2,CONCAT22(uVar4,uVar3),iVar5,(uint)uVar2,(uchar *)(uVar2 >> 0x10));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_0582(ulong *param_1,ulong *param_2,ushort param_3,ushort param_4,uchar param_5,ushort param_6)

{
  ulong **ppuVar1;
  long *plVar2;
  undefined4 uVar3;
  code **ppcVar4;
  undefined *puVar5;
  uint uVar6;
  ushort uVar7;
  ulong uVar8;
  ushort extraout_DX;
  uint uVar9;
  ushort extraout_DX_00;
  undefined2 extraout_DX_01;
  int iVar10;
  int iVar11;
  uint uVar12;
  undefined2 uVar13;
  undefined2 uVar14;
  undefined local_138 [0x10e];
  ulong local_2a;
  astruct_99 *paStack38;
  astruct_99 *paStack34;
  ulong uStack30;
  ulong uStack18;
  ulong uStack14;
  undefined local_a [0x4];
  ulong uStack6;
  
  uVar12 = (uint)((ulong)param_1 >> 0x10);
  iVar10 = (int)param_1;
  uVar8 = *(ulong *)(iVar10 + 0x14);
  uVar13 = (undefined2)(uVar8 >> 0x10);
  iVar11 = (int)uVar8;
  uStack6 = uVar8 & 0xffff0000 | (ulong)(iVar11 + 0xa4);
  if ((*(int *)(iVar11 + 0xa6) != 0x0) && (*(int *)(iVar11 + 0xac) != 0x0)) {
    pass1_1028_081e((ulong)param_1,(int)param_2,param_6);
    param_2 = *(ulong **)(iVar10 + 0x20);
    ppuVar1 = (ulong **)((int)uStack6 + 0x8);
    if (*ppuVar1 < param_2 || *ppuVar1 == param_2) {
      puVar5 = local_a;
      ppcVar4 = (code **)((int)*param_1 + 0x40);
      (**ppcVar4)(param_3,param_1);
      uVar8 = ZEXT24(puVar5);
      param_6 = extraout_DX;
      if (puVar5 == (undefined *)0x0) {
        if (*(int *)((int)uStack6 + 0x2) == 0xc) {
          uStack14 = pass1_1028_b4f2((ulong)param_1);
          param_6 = (ushort)(uStack14 >> 0x10);
          uVar8 = *(ulong *)((int)uStack14 + 0x1f6);
          plVar2 = (long *)((int)uVar8 + 0x170);
          *plVar2 = *plVar2 + 0x1;
          uStack18 = uVar8;
        }
        else {
          uStack18 = _PTR_LOOP_1050_68a2;
          paStack38 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
          uVar9 = (uint)((ulong)paStack38 >> 0x10);
          uVar6 = (uint)paStack38;
          if ((uVar9 | uVar6) == 0x0) {
            paStack34 = (astruct_99 *)0x0;
          }
          else {
            paStack38->field_0x0 = 0x389a;
            *(undefined2 *)(uVar6 + 0x2) = 0x1008;
            *(undefined2 *)(uVar6 + 0x4) = 0x0;
            *(undefined2 *)(uVar6 + 0x6) = 0x0;
            *(undefined2 *)(uVar6 + 0x8) = 0x0;
            *(undefined2 *)(uVar6 + 0xa) = 0x0;
            *(undefined2 *)(uVar6 + 0xc) = 0x0;
            paStack38->field_0x0 = 0x56ce;
            *(undefined2 *)(uVar6 + 0x2) = 0x1018;
            paStack34 = paStack38;
          }
          uVar13 = (undefined2)(uStack6 >> 0x10);
          iVar11 = (int)uStack6;
          uVar14 = (undefined2)((ulong)paStack34 >> 0x10);
          *(undefined2 *)((int)paStack34 + 0x6) = *(undefined2 *)(iVar11 + 0x2);
          *(undefined2 *)((int)paStack34 + 0xa) = *(undefined2 *)(iVar11 + 0x6);
          param_3 = 0x1020;
          uVar7 = switch_1020_c3b4(*(ushort *)(iVar11 + 0x2));
          uVar6 = uVar7 * *(int *)((int)uStack6 + 0x6);
          uVar8 = (ulong)uVar6;
          *(uint *)((int)paStack34 + 0xc) = uVar6;
          uVar3 = *(undefined4 *)(iVar10 + 0x22);
          ppcVar4 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar10 + 0x22) + 0x4);
          (**ppcVar4)(0x1020,(int)uVar3,(int)((ulong)uVar3 >> 0x10));
          param_6 = extraout_DX_00;
        }
      }
      param_2 = (ulong *)uVar8;
      *(undefined2 *)(iVar10 + 0x20) = 0x0;
    }
  }
  uVar13 = (undefined2)(uStack6 >> 0x10);
  if ((*(int *)((int)uStack6 + 0x4) != 0x0) && (*(int *)((int)uStack6 + 0x8) != 0x0)) {
    pass1_1028_081e((ulong)param_1,(int)param_2,param_6);
    param_2 = *(ulong **)(iVar10 + 0x20);
    ppuVar1 = (ulong **)((int)uStack6 + 0x8);
    if (*ppuVar1 < param_2 || *ppuVar1 == param_2) {
      param_2 = &local_2a;
      ppcVar4 = (code **)((int)*param_1 + 0x40);
      (**ppcVar4)(param_3,param_1);
      if (param_2 == (ulong *)0x0) {
        uStack18 = _PTR_LOOP_1050_68a2;
        paStack38 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
        uVar9 = (uint)((ulong)paStack38 >> 0x10);
        uVar6 = (uint)paStack38;
        if ((uVar9 | uVar6) == 0x0) {
          paStack34 = (astruct_99 *)0x0;
        }
        else {
          paStack38->field_0x0 = 0x389a;
          *(undefined2 *)(uVar6 + 0x2) = 0x1008;
          *(undefined2 *)(uVar6 + 0x4) = 0x0;
          *(undefined2 *)(uVar6 + 0x6) = 0x0;
          *(undefined2 *)(uVar6 + 0x8) = 0x0;
          *(undefined2 *)(uVar6 + 0xa) = 0x0;
          *(undefined2 *)(uVar6 + 0xc) = 0x0;
          paStack38->field_0x0 = 0x56ce;
          *(undefined2 *)(uVar6 + 0x2) = 0x1018;
          paStack34 = paStack38;
        }
        uVar13 = (undefined2)(uStack6 >> 0x10);
        iVar11 = (int)uStack6;
        uVar14 = (undefined2)((ulong)paStack34 >> 0x10);
        *(undefined2 *)((int)paStack34 + 0x8) = *(undefined2 *)(iVar11 + 0x4);
        *(undefined2 *)((int)paStack34 + 0xa) = *(undefined2 *)(iVar11 + 0x6);
        uVar7 = pass1_1020_c42e(*(int *)(iVar11 + 0x4));
        param_2 = (ulong *)(uVar7 * *(int *)((int)uStack6 + 0x6));
        *(ulong **)((int)paStack34 + 0xc) = param_2;
        uVar3 = *(undefined4 *)(iVar10 + 0x22);
        ppcVar4 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar10 + 0x22) + 0x4);
        (**ppcVar4)(0x1020,(int)uVar3,(int)((ulong)uVar3 >> 0x10));
      }
      *(undefined2 *)(iVar10 + 0x20) = 0x0;
    }
  }
  if (*(int *)(iVar10 + 0xc) != 0xe) {
    pass1_1028_b58e((ulong)param_1 & 0xffff | (ulong)uVar12 << 0x10);
    local_2a = CONCAT22(extraout_DX_01,param_2);
    paStack34 = (astruct_99 *)*(undefined4 *)((int)param_2 + 0x2e);
    uStack30 = *(ulong *)((int)paStack34 + 0x4);
    pass1_1028_68de((astruct_100 *)CONCAT22(param_4,local_138),0x1,uStack30,param_5,param_4);
    fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_4,local_138));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_081e(ulong param_1,int param_2,ushort param_3)

{
  int *piVar1;
  int iVar2;
  undefined2 uVar3;
  undefined4 uVar4;
  uint uVar5;
  int iVar6;
  undefined2 uVar7;
  
  pass1_1028_b58e(param_1);
  uVar4 = *(undefined4 *)(param_2 + 0x2e);
  iVar2 = *(int *)((int)uVar4 + 0x18);
  uVar7 = (undefined2)(param_1 >> 0x10);
  iVar6 = (int)param_1;
  piVar1 = (int *)(iVar6 + 0x20);
  *piVar1 = *piVar1 + 0x1;
  uVar5 = *_PTR_LOOP_1050_65e2;
  uVar3 = *(undefined2 *)((int)_PTR_LOOP_1050_65e2 + 0x2);
  if (iVar2 < 0xfa) {
    uVar5 = uVar5 & 0x1;
  }
  else {
    if (0x1c1 < iVar2) {
      if (iVar2 < 0x226) {
        return;
      }
      if ((iVar2 < 0x2ee) && (CONCAT22(uVar3,uVar5) % 0x3 != 0x0)) {
        return;
      }
      piVar1 = (int *)(iVar6 + 0x20);
      *piVar1 = *piVar1 + 0x1;
      return;
    }
    uVar5 = (uint)((qword)CONCAT22(uVar3,uVar5) % 0x3);
  }
  if (uVar5 != 0x0) {
    return;
  }
  piVar1 = (int *)(iVar6 + 0x20);
  *piVar1 = *piVar1 + -0x1;
  return;
}



astruct_18 * __stdcall16far pass1_1028_08c6(astruct_18 *param_1,byte param_2)

{
  pass1_1028_0138(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_0954(ushort *param_1)

{
  astruct_185 *iVar1;
  undefined2 uVar1;
  
  struct_1028_b354(param_1);
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_185 *)param_1;
  iVar1->field_0x20 = 0x0;
  *param_1 = 0xada;
  iVar1->field_0x2 = (int)&USHORT_1050_1028;
  iVar1->field_0xe = 0x4b;
  return param_1;
}
