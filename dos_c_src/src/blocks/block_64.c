
ushort * __stdcall16far pass1_1028_2bfe(astruct_179 *param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_0982(param_1,param_2,param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0x341c;
  param_1->field_0x2 = (int)&USHORT_1050_1028;
  return (ushort *)CONCAT22(param_2,param_1);
}



ushort __stdcall16far
pass1_1028_2c28(ushort param_1,ushort param_2,ushort param_3,ulong param_4,ulong *param_5,ulong param_6,ulong param_7)

{
  ulong *puVar1;
  ushort *puVar2;
  ushort uVar3;
  ushort uVar4;
  ushort local_e;
  ushort local_c;
  ushort local_a;
  ulong local_8;
  undefined2 uStack4;
  
  pass1_1028_09d4(param_1,param_2,param_3,param_4,(ushort *)param_5,param_6,param_7);
  if (param_2 != 0x0) {
    local_8 = *param_5;
    uStack4 = *(undefined2 *)((int)param_5 + 0x4);
    puVar2 = &local_e;
    pass1_1008_3eb4((ushort *)CONCAT22(param_1,&local_8),(ushort *)CONCAT22(param_1,puVar2),
                    (ushort *)CONCAT22(param_1,&local_c),(ushort *)CONCAT22(param_1,&local_a));
    pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_8),local_e,local_c - 0x1,local_a - 0x1);
    puVar1 = &local_8;
    uVar3 = (ushort)param_4;
    uVar4 = (ushort)(param_4 >> 0x10);
    pass1_1028_c7b6(param_1,(ushort)puVar2,uVar3,uVar4,(ushort *)CONCAT22(param_1,puVar1),param_7);
    if (puVar1 != (ulong *)0x0) {
      pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_8),local_e,local_c - 0x1,local_a);
      puVar1 = &local_8;
      pass1_1028_c7b6(param_1,(ushort)puVar2,uVar3,uVar4,(ushort *)CONCAT22(param_1,puVar1),param_7);
      if (puVar1 != (ulong *)0x0) {
        pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_8),local_e,local_c - 0x1,local_a + 0x1);
        puVar1 = &local_8;
        pass1_1028_c7b6(param_1,(ushort)puVar2,uVar3,uVar4,(ushort *)CONCAT22(param_1,puVar1),param_7);
        if (puVar1 != (ulong *)0x0) {
          pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_8),local_e,local_c,local_a - 0x1);
          puVar1 = &local_8;
          pass1_1028_c7b6(param_1,(ushort)puVar2,uVar3,uVar4,(ushort *)CONCAT22(param_1,puVar1),param_7);
          if (puVar1 != (ulong *)0x0) {
            pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_8),local_e,local_c,local_a);
            puVar1 = &local_8;
            pass1_1028_c7b6(param_1,(ushort)puVar2,uVar3,uVar4,(ushort *)CONCAT22(param_1,puVar1),param_7);
            if (puVar1 != (ulong *)0x0) {
              pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_8),local_e,local_c,local_a + 0x1);
              puVar1 = &local_8;
              pass1_1028_c7b6(param_1,(ushort)puVar2,uVar3,uVar4,(ushort *)CONCAT22(param_1,puVar1),param_7);
              if (puVar1 != (ulong *)0x0) {
                pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_8),local_e,local_c + 0x1,local_a - 0x1);
                puVar1 = &local_8;
                pass1_1028_c7b6(param_1,(ushort)puVar2,uVar3,uVar4,(ushort *)CONCAT22(param_1,puVar1),param_7);
                if (puVar1 != (ulong *)0x0) {
                  pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_8),local_e,local_c + 0x1,local_a);
                  puVar1 = &local_8;
                  pass1_1028_c7b6(param_1,(ushort)puVar2,uVar3,uVar4,(ushort *)CONCAT22(param_1,puVar1),param_7);
                  if (puVar1 != (ulong *)0x0) {
                    pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_8),local_e,local_c + 0x1,local_a + 0x1);
                    puVar1 = &local_8;
                    pass1_1028_c7b6(param_1,(ushort)puVar2,uVar3,uVar4,(ushort *)CONCAT22(param_1,puVar1),param_7);
                    if (puVar1 != (ulong *)0x0) {
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
    PTR_LOOP_1050_50ca = (undefined *)0x6a8;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_2e40(ulong *param_1,int param_2,uchar *param_3,ushort param_4,ushort param_5,ushort param_6,ushort param_7,
               uchar param_8)

{
  uint uVar1;
  ushort *puVar2;
  undefined4 uVar3;
  
  pass1_1028_be9e(param_1,param_4,param_5,param_6,param_7);
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  if (*(int *)((int)param_1 + 0x12) == 0x5) {
    pass1_1028_2f18(param_7,param_2,param_8,(ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
    pass1_1028_3246((ulong)param_1,param_2,(ushort)param_3,param_4,param_5,param_7);
    uVar3 = 0x50001;
    puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_7,param_3,param_5);
    pass1_1010_089e(param_7,(ulong)puVar2,(ushort)uVar3,(int)((ulong)uVar3 >> 0x10));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_2e84(ushort param_1,ulong param_2,uchar *param_3,ushort param_4,ushort param_5,uchar param_6)

{
  uchar *puVar1;
  astruct_67 *paVar2;
  ushort *puVar3;
  undefined2 uVar4;
  undefined2 uVar5;
  int iVar6;
  ushort uVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  int iVar10;
  
  pass1_1028_09b8(CONCAT22((int)param_2,param_1));
  if ((int)(param_2 >> 0x10) == 0x0) {
    uVar9 = 0x0;
    iVar10 = 0x8;
    uVar7 = 0x1;
    uVar8 = 0x0;
    uVar5 = 0x0;
    iVar6 = 0x0;
    uVar4 = 0x0;
    paVar2 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_5,param_3,param_4);
    puVar1 = (uchar *)((ulong)paVar2 >> 0x10);
    post_win_msg_1008_a0e4(paVar2,CONCAT22(uVar5,uVar4),iVar6,uVar7,CONCAT22(uVar9,uVar8),iVar10,0x1008,param_5);
    uVar5 = 0x400;
    iVar6 = 0x3;
    uVar4 = 0x1;
    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_5,puVar1,param_4);
    puVar1 = (uchar *)((ulong)puVar3 >> 0x10);
    pass1_1010_043a((ulong)puVar3,CONCAT22(uVar5,uVar4),iVar6,param_5);
    pass1_1010_043a((ulong)puVar3,0x4000001,0x4,param_5);
    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_5,puVar1,param_4);
    puVar1 = (uchar *)((ulong)puVar3 >> 0x10);
    pass1_1010_ec84((ulong)puVar3,param_5,param_6);
    puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x8,param_5,puVar1,param_4);
    pass1_1010_9766((ulong)puVar3,(ushort)((ulong)puVar3 >> 0x10),param_4,param_5);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1028_2f18(ushort param_1,int param_2,uchar param_3,ulong param_4)

{
  int iVar1;
  ulong *puVar2;
  undefined2 extraout_DX;
  uint uVar3;
  undefined2 uVar4;
  ushort *puVar5;
  undefined local_944 [0x124];
  undefined local_820 [0x124];
  undefined local_6fc [0x124];
  undefined local_5d8 [0x124];
  undefined local_4b4 [0x124];
  ulong local_390;
  undefined local_38a [0x124];
  undefined local_266 [0x124];
  undefined local_142 [0x124];
  undefined4 local_1e;
  ushort local_1a;
  ulong local_18;
  undefined2 uStack20;
  ulong uStack18;
  undefined4 uStack14;
  undefined4 uStack10;
  ulong uStack6;
  
  uStack6 = pass1_1028_bb24(param_4);
  iVar1 = (int)uStack6;
  pass1_1028_b58e(param_4);
  uStack10 = CONCAT22(extraout_DX,iVar1);
  uStack14 = *(undefined4 *)(iVar1 + 0x2e);
  uStack18 = *(ulong *)((int)uStack14 + 0x4);
  local_18 = *(ulong *)(iVar1 + 0xc);
  uStack20 = *(undefined2 *)(iVar1 + 0x10);
  pass1_1008_3eb4((ushort *)CONCAT22(param_1,&local_18),(ushort *)CONCAT22(param_1,&local_1e),
                  (ushort *)CONCAT22(param_1,(int)&local_1e + 0x2),(ushort *)CONCAT22(param_1,&local_1a));
  pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_18),(ushort)local_1e,local_1e._2_2_ - 0x1,local_1a - 0x1);
  struct_op_1028_87f0(param_1,param_3,(astruct_97 *)CONCAT22(param_1,local_142),0x0,0x0,0xd,&local_18,param_1,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,local_142));
  pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_18),(ushort)local_1e,local_1e._2_2_ + 0x1,local_1a + 0x1);
  struct_op_1028_87f0(param_1,param_3,(astruct_97 *)CONCAT22(param_1,local_266),0x0,0x0,0xc,&local_18,param_1,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,local_266));
  pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_18),(ushort)local_1e,local_1e._2_2_ + 0x1,local_1a - 0x1);
  struct_op_1028_87f0(param_1,param_3,(astruct_97 *)CONCAT22(param_1,local_38a),0x0,0x0,0xe,&local_18,param_1,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,local_38a));
  puVar5 = pass1_1008_3e54((ushort *)CONCAT22(param_1,&local_390),(ushort)local_1e,local_1e._2_2_ - 0x1,local_1a + 0x1);
  uVar3 = (uint)((ulong)puVar5 >> 0x10);
  struct_op_1028_87f0(param_1,param_3,(astruct_97 *)CONCAT22(param_1,local_4b4),0x0,0x0,0xb,&local_390,param_1,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,local_4b4));
  pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_18),(ushort)local_1e,local_1e._2_2_ - 0x1,local_1a);
  struct_op_1028_87f0(param_1,param_3,(astruct_97 *)CONCAT22(param_1,local_5d8),0x0,0x0,0x7a,&local_18,param_1,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,local_5d8));
  pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_18),(ushort)local_1e,(ushort)((ulong)local_1e >> 0x10),
                  local_1a + 0x1);
  struct_op_1028_87f0(param_1,param_3,(astruct_97 *)CONCAT22(param_1,local_6fc),0x0,0x0,0x7a,&local_18,param_1,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,local_6fc));
  pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_18),(ushort)local_1e,local_1e._2_2_ + 0x1,local_1a);
  struct_op_1028_87f0(param_1,param_3,(astruct_97 *)CONCAT22(param_1,local_820),0x0,0x0,0x7a,&local_18,param_1,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,local_820));
  pass1_1008_3e76((ushort *)CONCAT22(param_1,&local_18),(ushort)local_1e,(ushort)((ulong)local_1e >> 0x10),
                  local_1a - 0x1);
  struct_op_1028_87f0(param_1,param_3,(astruct_97 *)CONCAT22(param_1,local_944),0x0,0x0,0x7a,&local_18,param_1,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,(ulong *)CONCAT22(param_1,local_944));
  puVar2 = &local_390;
  pass1_1030_627e(param_1,(uint)puVar2,uVar3,_PTR_LOOP_1050_5740,(ushort *)CONCAT22(param_1,puVar2),uStack6);
  uVar4 = (undefined2)((ulong)uStack14 >> 0x10);
  *(ulong **)((int)uStack14 + 0x10) = puVar2;
  *(uint *)((int)uStack14 + 0x12) = uVar3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_3246(ulong param_1,int param_2,ushort param_3,ushort param_4,ushort param_5,ushort param_6)

{
  uint uVar1;
  ulong uVar2;
  uchar *extraout_DX;
  uchar *puVar3;
  undefined2 uVar4;
  undefined2 uVar5;
  int iVar6;
  undefined local_20 [0x6];
  ushort *puStack26;
  uint uStack18;
  uchar *puStack16;
  ulong uStack14;
  undefined4 uStack10;
  undefined4 uStack6;
  
  pass1_1028_b58e(param_1);
  uStack6 = CONCAT22(extraout_DX,param_2);
  uStack10 = *(undefined4 *)(param_2 + 0x2e);
  uVar2 = *(ulong *)((int)uStack10 + 0x10);
  uVar5 = 0x0;
  iVar6 = 0x1;
  uVar4 = 0x1;
  puVar3 = extraout_DX;
  uStack14 = uVar2;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(ushort)uVar2,(uint)(uVar2 >> 0x10));
  uVar1 = (uint)uVar2;
  uStack18 = uVar1;
  puStack16 = puVar3;
  pass1_1030_7c50(uVar2 & 0xffff | ZEXT24(puVar3) << 0x10,CONCAT22(uVar5,uVar4),iVar6,uVar1,puVar3);
  pass1_1030_7c50(CONCAT22(puStack16,uStack18),0x1,0x2,uVar1,puVar3);
  pass1_1030_7c50(CONCAT22(puStack16,uStack18),0x1,0x3,uVar1,puVar3);
  pass1_1030_7c50(CONCAT22(puStack16,uStack18),0x1,0x5,uVar1,puVar3);
  puStack26 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_6,puVar3,param_5);
  puVar3 = (uchar *)((ulong)puStack26 >> 0x10);
  uVar1 = (uint)puStack26;
  if (*(int *)(uVar1 + 0x82) == 0x0) {
    pass1_1030_7c50(CONCAT22(puStack16,uStack18),0x4,0x4,uVar1,puVar3);
  }
  pass1_1030_7ddc(CONCAT22(puStack16,uStack18),0xc8,0x11,uVar1,puVar3,param_4,param_5,param_6);
  pass1_1030_7ddc(CONCAT22(puStack16,uStack18),0xc8,0x12,uVar1,puVar3,param_4,param_5,param_6);
  pass1_1030_7ddc(CONCAT22(puStack16,uStack18),0xc8,0x13,uVar1,puVar3,param_4,param_5,param_6);
  pass1_1030_7ddc(CONCAT22(puStack16,uStack18),0xc8,0x14,uVar1,puVar3,param_4,param_5,param_6);
  pass1_1030_7ddc(CONCAT22(puStack16,uStack18),0x14,0x15,uVar1,puVar3,param_4,param_5,param_6);
  pass1_1030_7ddc(CONCAT22(puStack16,uStack18),0x14,0x16,uVar1,puVar3,param_4,param_5,param_6);
  pass1_1030_7ddc(CONCAT22(puStack16,uStack18),0xc8,0x17,uVar1,puVar3,param_4,param_5,param_6);
  pass1_1030_7ddc(CONCAT22(puStack16,uStack18),0xc8,0x18,uVar1,puVar3,param_4,param_5,param_6);
  pass1_1030_7ddc(CONCAT22(puStack16,uStack18),0xc8,0x19,uVar1,puVar3,param_4,param_5,param_6);
  pass1_1030_7ddc(CONCAT22(puStack16,uStack18),0x14,0x1a,uVar1,puVar3,param_4,param_5,param_6);
  pass1_1030_7ddc(CONCAT22(puStack16,uStack18),0x14,0x1b,uVar1,puVar3,param_4,param_5,param_6);
  pass1_1030_7ddc(CONCAT22(puStack16,uStack18),0x14,0x1c,uVar1,puVar3,param_4,param_5,param_6);
  if (*(long *)((int)uStack10 + 0x200) == 0x8000002) {
    pass1_1020_a43e(param_6,puVar3,(ushort *)CONCAT22(param_6,local_20));
    pass1_1020_a89e(param_6,CONCAT22(param_6,local_20),(ulong *)((int)uStack6 + 0xc),(ushort)((ulong)uStack6 >> 0x10));
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_33f6(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_3484(ushort *param_1)

{
  uchar *in_DX;
  
  struct_1028_0068(param_1,in_DX);
  *param_1 = 0x34f6;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



ushort * __stdcall16far pass1_1028_34a6(int param_1,ushort param_2,int param_3,ulong param_4,uchar *param_5)

{
  pass1_1028_00cc(param_1,param_2,param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0x34f6;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  return (ushort *)CONCAT22(param_2,param_1);
}



astruct_18 * __stdcall16far pass1_1028_34d0(astruct_18 *param_1,byte param_2)

{
  pass1_1028_0138(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_355e(ushort *param_1)

{
  struct_1028_b354(param_1);
  *param_1 = 0x3608;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



ushort * __stdcall16far pass1_1028_3580(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(ushort *)CONCAT22(param_2,param_1) = 0x3608;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  return (ushort *)CONCAT22(param_2,param_1);
}



ushort __stdcall16far pass1_1028_35aa(void)

{
  return 0x1;
}



void __stdcall16far pass1_1028_35b0(ulong param_1,int param_2,ushort param_3,ushort param_4,ushort param_5)

{
  ulong uVar1;
  uint uVar2;
  
  uVar1 = pass1_1028_b58e(param_1);
  if (param_2 == 0x0) {
    uVar2 = 0x0;
  }
  else {
    uVar2 = 0x32;
  }
  pass1_1030_7d1c(uVar1,uVar2,0x230000,(int)uVar1,(int)(uVar1 >> 0x10),param_3,param_4,param_5);
  return;
}



astruct_18 * __stdcall16far pass1_1028_35e2(astruct_18 *param_1,byte param_2)

{
  pass1_1028_b418(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_3670(ushort *param_1,uchar *param_2,ushort param_3,ushort param_4)

{
  struct_1028_37a6(param_1,param_2,param_3,param_4);
  *param_1 = 0x373e;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  return param_1;
}



ushort * __stdcall16far
pass1_1028_3692(int param_1,ushort param_2,int param_3,ulong param_4,uchar *param_5,ushort param_6,ushort param_7)

{
  pass1_1028_3816(param_1,param_2,param_3,param_4,param_5,param_6,param_7);
  *(ushort *)CONCAT22(param_2,param_1) = 0x373e;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  return (ushort *)CONCAT22(param_2,param_1);
}



ushort __stdcall16far pass1_1028_36bc(ulong param_1,ulong *param_2,ushort param_3,ushort param_4,ushort param_5)

{
  int *piVar1;
  uint uVar2;
  undefined4 uVar3;
  undefined2 uVar4;
  undefined4 uVar5;
  int iStack4;
  
  uVar5 = CONCAT22(param_4,param_3);
  *param_2 = 0x0;
  uVar4 = (undefined2)(param_1 >> 0x10);
  if (*(long *)((int)param_1 + 0x28) != 0x0) {
    iStack4 = 0x4;
    while( true ) {
      if (0x1c < iStack4) break;
      uVar3 = *(undefined4 *)((int)param_1 + 0x28);
      uVar5 = pass1_1020_bae6((ushort)uVar3,CONCAT22(iStack4,(int)((ulong)uVar3 >> 0x10)),(uint)uVar5,
                              (uint)((ulong)uVar5 >> 0x10),param_5);
      uVar2 = *(uint *)param_2;
      *(uint *)param_2 = *(int *)param_2 + (uint)uVar5;
      piVar1 = (int *)((int)param_2 + 0x2);
      *piVar1 = *piVar1 + (int)((ulong)uVar5 >> 0x10) + (uint)CARRY2(uVar2,(uint)uVar5);
      if (0xf9 < *param_2) {
        return 0x1;
      }
      iStack4 = iStack4 + 0x1;
    }
  }
  return 0x0;
}



astruct_18 * __stdcall16far pass1_1028_3718(astruct_18 *param_1,byte param_2)

{
  pass1_1028_388e(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



void __stdcall16far struct_1028_37a6(ushort *param_1,uchar *param_2,ushort param_3,ushort param_4)

{
  uint uVar1;
  uint uVar2;
  astruct_189 *iVar3;
  undefined2 uVar3;
  
  struct_1028_b354(param_1);
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (astruct_189 *)param_1;
  uVar1 = 0x0;
  iVar3->field_0x20 = 0x0;
  iVar3->field_0x24 = 0x0;
  *(undefined4 *)&iVar3->field_0x28 = 0x0;
  *param_1 = 0x3e2c;
  iVar3->field_0x2 = (int)&USHORT_1050_1028;
  mem_op_1000_179c(0xa,param_2,0x1000);
  uVar2 = (uint)param_2 | uVar1;
  if (uVar2 == 0x0) {
    *(undefined4 *)&iVar3->field_0x28 = 0x0;
  }
  else {
    pass1_1020_ba3e((long *)CONCAT22(param_2,uVar1),0x5,0x5,param_4,param_3);
    iVar3->field_0x28 = uVar1;
    iVar3->field_0x2a = uVar2;
  }
  return;
}



void __stdcall16far
pass1_1028_3816(int param_1,ushort param_2,int param_3,ulong param_4,uchar *param_5,ushort param_6,ushort param_7)

{
  uint uVar1;
  uint uVar2;
  
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,(ushort)param_5);
  uVar1 = 0x0;
  *(undefined4 *)(param_1 + 0x20) = 0x0;
  *(undefined4 *)(param_1 + 0x24) = 0x0;
  *(undefined4 *)(param_1 + 0x28) = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x3e2c;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  mem_op_1000_179c(0xa,param_5,0x1000);
  uVar2 = (uint)param_5 | uVar1;
  if (uVar2 == 0x0) {
    *(undefined4 *)(param_1 + 0x28) = 0x0;
  }
  else {
    pass1_1020_ba3e((long *)CONCAT22(param_5,uVar1),0x5,0x5,param_7,param_6);
    *(uint *)(param_1 + 0x28) = uVar1;
    *(uint *)(param_1 + 0x2a) = uVar2;
  }
  return;
}



void __stdcall16far pass1_1028_388e(ushort *param_1)

{
  uint uVar1;
  astruct_18 *paVar2;
  int iVar3;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  *param_1 = 0x3e2c;
  *(undefined2 *)(iVar3 + 0x2) = (int)&USHORT_1050_1028;
  paVar2 = *(astruct_18 **)(iVar3 + 0x28);
  uVar1 = *(uint *)(iVar3 + 0x2a);
  if ((uVar1 | (uint)paVar2) != 0x0) {
    fn_ptr_1020_ba7e((ulong *)((ulong)paVar2 & 0xffff | (ulong)uVar1 << 0x10));
    fn_ptr_1000_17ce(paVar2,0x1000);
  }
  pass1_1028_b418(param_1);
  return;
}



ushort __stdcall16far
pass1_1028_38d4(ulong *param_1,ushort *param_2,ulong param_3,ulong param_4,int param_5,ushort param_6,ushort param_7)

{
  code **ppcVar1;
  BOOL16 BVar2;
  undefined4 uVar3;
  ushort uVar4;
  ushort uVar5;
  
  uVar4 = (ushort)param_1;
  uVar5 = (ushort)((ulong)param_1 >> 0x10);
  pass1_1028_c7b6(param_7,param_6,uVar4,uVar5,param_2,param_4);
  if ((param_5 == 0x5) || (param_5 == 0x6)) {
    ppcVar1 = (code **)((int)*param_1 + 0x60);
    uVar3 = (**ppcVar1)();
    if ((uint)uVar3 != 0x0) {
      pass1_1028_c23e(uVar4,uVar5,param_2,param_3,param_4,(uint)uVar3,(uint)((ulong)uVar3 >> 0x10),param_7);
      if ((int)uVar3 != 0x0) {
        BVar2 = pass1_1028_c314(param_7,(int)uVar3,(ushort)((ulong)uVar3 >> 0x10),uVar4,uVar5,param_2,(ushort)param_3,
                                (ushort)(param_3 >> 0x10),param_4);
        if (BVar2 != 0x0) {
          return 0x1;
        }
      }
    }
  }
  else {
    PTR_LOOP_1050_50ca = (undefined *)0x6a8;
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far
pass1_1028_3958(ulong param_1,int param_2,undefined2 param_3,int param_4,ushort param_5,ushort param_6)

{
  long *plVar1;
  qword qVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  ulong uVar6;
  int iVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  undefined4 uStack52;
  uint local_2c [0x2];
  undefined4 local_28;
  int iStack36;
  ulong uStack34;
  undefined4 uStack30;
  uint uStack22;
  uint uStack20;
  undefined4 uStack18;
  ulong uStack14;
  ulong *puStack10;
  int iStack6;
  undefined2 uStack4;
  
  pass1_1028_b58e(param_1);
  puStack10 = *(ulong **)(param_2 + 0x22);
  uVar5 = *(uint *)(param_2 + 0x24);
  uVar6 = (ulong)uVar5;
  if ((uVar5 | (uint)puStack10) != 0x0) {
    iStack6 = param_2;
    uStack4 = param_3;
    if (PTR_LOOP_1050_574c != (undefined *)0x0) {
      uStack30 = (ulong)*(uint *)((uint)puStack10 + 0x4);
      for (uStack34 = 0x0; uStack34 < uStack30; uStack34 = uStack34 + 0x1) {
        pass1_1020_bb16(puStack10,(ulong *)CONCAT22(param_6,local_2c),(ushort *)CONCAT22(param_6,&local_28),
                        (uint)uStack34);
      }
    }
    uStack14 = *(ulong *)(iStack6 + 0x2e);
    uStack18 = *_PTR_LOOP_1050_65e2;
    uStack20 = (uint)uStack18 & 0x1;
    for (uStack22 = 0x4; (int)uStack22 < 0xe; uStack22 = uStack22 + 0x1) {
      local_2c[0] = uStack22;
      local_28 = pass1_1020_bae6((ushort)puStack10,CONCAT22(uStack22,(int)((ulong)puStack10 >> 0x10)),uStack22,
                                 (uint)uVar6,param_6);
      uVar5 = (uint)(local_28 >> 0x10) | (uint)local_28;
      uVar6 = (ulong)uVar5;
      if (uVar5 != 0x0) {
        pass1_1020_bb8a((long *)puStack10,0x0,(ulong)local_2c[0] << 0x10,param_5,param_6);
        uVar5 = -(local_28._2_2_ + (uint)((int)local_28 != 0x0));
        uVar6 = (ulong)uVar5;
        uStack34 = CONCAT22(uVar5,-(int)local_28);
        pass1_1038_5694(uStack14,CONCAT22(uVar5,-(int)local_28),local_2c[0]);
        uStack30 = 0x0;
        iStack36 = 0x0;
        iVar7 = (int)param_1;
        uVar8 = (undefined2)(param_1 >> 0x10);
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
          plVar1 = (long *)(iVar7 + 0x20);
          *plVar1 = *plVar1 + local_28;
          uVar5 = *(uint *)(iVar7 + 0x20);
          uVar3 = *(uint *)(iVar7 + 0x22);
          uVar4 = uVar5 >> 0x1 | (uint)((uVar3 & 0x1) != 0x0) << 0xf;
          uStack52 = CONCAT22(uVar3 >> 0x1,uVar4);
          uVar4 = (uVar3 & 0xfffe) + (uint)CARRY2(uVar4,uVar4);
          uVar6 = (ulong)uVar4;
          param_4 = (uVar3 - uVar4) - (uint)(uVar5 < (uVar5 & 0xfffe));
          *(int *)(iVar7 + 0x20) = uVar5 - (uVar5 & 0xfffe);
          *(int *)(iVar7 + 0x22) = param_4;
          if (uStack52 != 0x0) {
            uVar9 = 0x15;
LAB_1028_3b14:
            uStack30 = local_28;
            pass1_1020_bb8a(*(long **)(iVar7 + 0x28),(uint)uStack52,CONCAT22(uVar9,(int)((ulong)uStack52 >> 0x10)),
                            param_5,param_6);
          }
          break;
        case 0xd:
          iStack36 = 0x19;
          uStack30 = local_28;
          plVar1 = (long *)(iVar7 + 0x24);
          *plVar1 = *plVar1 + local_28;
          uVar5 = *(uint *)(iVar7 + 0x24);
          qVar2 = (qword)(local_28 & 0xffff0000 | (ulong)uVar5) / 0x3;
          uStack52 = (long)qVar2;
          uStack52._2_2_ = (int)(qVar2 >> 0x10);
          uVar3 = (uint)qVar2;
          uVar4 = uStack52._2_2_ * 0x3 + (uint)CARRY2(uVar3,uVar3) + (uint)CARRY2(uVar3 * 0x2,uVar3);
          uVar6 = (ulong)uVar4;
          param_4 = uVar5 + uVar3 * -0x3;
          param_5 = (*(int *)(iVar7 + 0x26) - uVar4) - (uint)(uVar5 < uVar3 * 0x3);
          *(int *)(iVar7 + 0x24) = param_4;
          *(ushort *)(iVar7 + 0x26) = param_5;
          if (uStack52 != 0x0) {
            uVar9 = 0x16;
            goto LAB_1028_3b14;
          }
        }
        if (((uStack30._2_2_ | (uint)uStack30) != 0x0) && (iStack36 != 0x0)) {
          pass1_1020_bb70(*(long **)(iVar7 + 0x28),(uint)uStack30,CONCAT22(iStack36,uStack30._2_2_),param_5,param_4,
                          param_6);
        }
      }
    }
  }
  return;
}



ulong __stdcall16far pass1_1028_3c32(ulong *param_1)

{
  code **ppcVar1;
  int iVar2;
  uint local_6;
  int iStack4;
  
  ppcVar1 = (code **)((int)*param_1 + 0x40);
  iVar2 = (**ppcVar1)();
  if (iVar2 != 0x0) {
    return 0x0;
  }
  return CONCAT22(-(uint)(0x3e8 < local_6) - iStack4,0x3e8 - local_6);
}



void __stdcall16far pass1_1028_3c60(undefined4 param_1,ulong *param_2,ushort param_3,ushort param_4,ushort param_5)

{
  int *piVar1;
  uint uVar2;
  undefined4 uVar3;
  int iVar4;
  undefined2 uVar5;
  undefined4 uVar6;
  long local_10;
  undefined local_c [0x4];
  int iStack8;
  uint uStack6;
  uint uStack4;
  
  uVar6 = CONCAT22(param_4,param_3);
  *param_2 = 0x0;
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(long *)(iVar4 + 0x28) != 0x0) {
    iStack8 = 0x4;
    while( true ) {
      if (0x1c < iStack8) break;
      uVar3 = *(undefined4 *)(iVar4 + 0x28);
      uVar6 = pass1_1020_bae6((ushort)uVar3,CONCAT22(iStack8,(int)((ulong)uVar3 >> 0x10)),(uint)uVar6,
                              (uint)((ulong)uVar6 >> 0x10),param_5);
      uVar2 = *(uint *)param_2;
      *(uint *)param_2 = *(int *)param_2 + (uint)uVar6;
      piVar1 = (int *)((int)param_2 + 0x2);
      *piVar1 = *piVar1 + (int)((ulong)uVar6 >> 0x10) + (uint)CARRY2(uVar2,(uint)uVar6);
      if (0x3e7 < *param_2) {
        return;
      }
      iStack8 = iStack8 + 0x1;
    }
  }
  uVar6 = *(undefined4 *)(iVar4 + 0x28);
  uStack4 = *(uint *)((int)uVar6 + 0x4);
  uStack6 = 0x0;
  while( true ) {
    if (uStack4 <= uStack6) {
      return;
    }
    pass1_1020_bb16(*(ulong **)(iVar4 + 0x28),(ulong *)CONCAT22(param_5,&local_10),(ushort *)CONCAT22(param_5,local_c),
                    uStack6);
    *param_2 = *param_2 + local_10;
    if (0x3e7 < *param_2) break;
    uStack6 = uStack6 + 0x1;
  }
  return;
}



void __stdcall16far write_to_file_1028_3d0e(ulong param_1,ulong param_2,ushort param_3,undefined2 param_4)

{
  BOOL16 BVar1;
  int iVar2;
  undefined2 uVar3;
  ushort uVar4;
  undefined4 local_10 [0x2];
  undefined4 local_8;
  
  BVar1 = write_to_file_1028_b5ec(param_1,param_2,param_3);
  if (BVar1 != 0x0) {
    uVar3 = (undefined2)(param_1 >> 0x10);
    iVar2 = (int)param_1;
    local_10[0] = *(undefined4 *)(iVar2 + 0x20);
    uVar4 = (ushort)(param_2 >> 0x10);
    BVar1 = write_to_file_1008_7e1c((ushort)param_2,uVar4,(ushort)local_10,param_3,(char *)0x4,0x1008);
    if (BVar1 != 0x0) {
      local_8 = *(undefined4 *)(iVar2 + 0x24);
      BVar1 = write_to_file_1008_7e1c((ushort)param_2,uVar4,(ushort)&local_8,param_3,(char *)0x4,0x1008);
      if (BVar1 != 0x0) {
        write_to_file_1008_7a22(param_2,*(long *)(iVar2 + 0x28),0x1008,param_3);
        if (BVar1 != 0x0) {
          return;
        }
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  }
  return;
}



void __stdcall16far
pass1_1028_3d92(ulong param_1,ulong param_2,int param_3,uchar *param_4,ushort param_5,ushort param_6)

{
  int iVar1;
  BOOL16 BVar2;
  ushort uVar3;
  ushort uVar4;
  
  file_1028_b81a(param_1,param_2,param_3,param_6,param_4);
  if (param_3 != 0x0) {
    iVar1 = (int)param_1;
    uVar3 = (ushort)(param_1 >> 0x10);
    uVar4 = (ushort)(param_2 >> 0x10);
    BVar2 = read_file_1008_7dee((ushort)param_2,uVar4,iVar1 + 0x20,0x0,uVar3,0x4,0x1008);
    if (BVar2 != 0x0) {
      BVar2 = read_file_1008_7dee((ushort)param_2,uVar4,iVar1 + 0x24,0x0,uVar3,0x4,0x1008);
      if (BVar2 != 0x0) {
        uVar3 = pass1_1008_7ad4(param_2,*(long **)(iVar1 + 0x28),param_5,0x1008,param_6);
        if (uVar3 != 0x0) {
          return;
        }
      }
    }
    PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  }
  return;
}



astruct_18 * __stdcall16far pass1_1028_3e06(astruct_18 *param_1,byte param_2)

{
  pass1_1028_388e(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



ushort * __stdcall16far struct_1028_3e94(ushort *param_1)

{
  uint uVar1;
  
  struct_1028_b354(param_1);
  uVar1 = (uint)((ulong)param_1 >> 0x10);
  *(undefined4 *)((int)param_1 + 0x20) = 0x0;
  *param_1 = 0x4004;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&USHORT_1050_1028;
  pass1_1028_3fa2((ulong)param_1 & 0xffff | (ulong)uVar1 << 0x10);
  return param_1;
}



ulong __stdcall16far pass1_1028_3ec8(int param_1,ushort param_2,int param_3,ulong param_4,ushort param_5)

{
  pass1_1028_b39e((ushort *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  *(undefined4 *)(param_1 + 0x20) = 0x0;
  *(undefined2 *)CONCAT22(param_2,param_1) = 0x4004;
  *(undefined2 *)(param_1 + 0x2) = (int)&USHORT_1050_1028;
  pass1_1028_3fa2(CONCAT22(param_2,param_1));
  return CONCAT22(param_2,param_1);
}



void __stdcall16far
pass1_1028_3f04(ulong param_1,uint param_2,uint param_3,ushort param_4,ushort param_5,ushort param_6)

{
  uint uVar1;
  ulong uVar2;
  uchar *puVar3;
  int iVar4;
  undefined2 uVar5;
  ushort uVar6;
  ulong uStack14;
  ulong uStack10;
  ulong uStack6;
  
  uVar6 = 0x1f;
  pass1_1028_b58e(param_1);
  uStack6 = CONCAT22(param_3,param_2);
  uStack10 = pass1_1030_7c28(CONCAT22(param_3,param_2),uVar6,param_2,param_3,param_6);
  puVar3 = (uchar *)(uStack10 >> 0x10);
  uVar2 = uStack10 & 0xffff;
  pass1_1030_7d1c(uStack6,0x0,0x1f0000,(int)uVar2,puVar3,param_4,param_5,param_6);
  uVar5 = (undefined2)(param_1 >> 0x10);
  iVar4 = (int)param_1;
  if (*(int *)(iVar4 + 0xc) != 0x22) {
    if (*(int *)(iVar4 + 0xc) == 0x23) {
      uVar1 = 0xa;
    }
    else {
      uVar1 = 0x5;
    }
    uStack14 = (ulong)uVar1;
    uStack10 = uStack10 + *(long *)(iVar4 + 0x20);
    *(ulong *)(iVar4 + 0x20) = uStack10 % (ulong)uVar1;
    uVar2 = uStack10 / uStack14;
    puVar3 = (uchar *)(uStack10 % uStack14);
    uStack10 = uStack10 + uVar2;
  }
  pass1_1030_7ddc(uStack6,uStack10,0x21,(uint)uVar2,puVar3,param_4,param_5,param_6);
  return;
}



void __stdcall16far pass1_1028_3fa2(ulong param_1)

{
  uint uVar1;
  int iVar2;
  undefined2 uVar3;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(int *)(iVar2 + 0xc) != 0x22) {
    if (*(int *)(iVar2 + 0xc) == 0x23) {
      uVar1 = 0xa;
    }
    else {
      uVar1 = 0x5;
    }
    uVar1 = uVar1 >> 0x1;
    pass1_1008_612e(0x0,uVar1,uVar1);
    *(uint *)(iVar2 + 0x20) = uVar1;
    *(int *)(iVar2 + 0x22) = (int)uVar1 >> 0xf;
  }
  return;
}
