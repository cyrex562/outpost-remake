


// WARNING: Could not reconcile some variable overlaps

void __stdcall16far pass1_1038_75ca(ulong param_1,ulong param_2,int param_3,uint16_t param_4)

{
  BOOL16 BVar1;
  int iVar2;
  int iVar3;
  undefined2 uVar4;
  ushort uVar5;
  ushort uVar6;
  undefined4 local_10 [0x2];
  undefined4 local_8;
  
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  pass1_1008_79f0(param_2,*(long *)(iVar3 + 0x4),0x1008,param_4);
  if (param_3 != 0x0) {
    local_10[0] = *(undefined4 *)(iVar3 + 0x8);
    uVar5 = (ushort)param_2;
    uVar6 = (ushort)(param_2 >> 0x10);
    BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)local_10,param_4,(char *)0x4,0x1008);
    if (BVar1 != 0x0) {
      write_to_file_1008_7a22(param_2,*(long *)(iVar3 + 0xe),0x1008,param_4);
      if (BVar1 != 0x0) {
        local_8._0_2_ = *(undefined2 *)(iVar3 + 0xc);
        BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_8,param_4,(char *)0x2,0x1008);
        if (BVar1 != 0x0) {
          local_8._0_2_ = *(undefined2 *)(iVar3 + 0x12);
          BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_8,param_4,(char *)0x2,0x1008);
          if (BVar1 != 0x0) {
            local_8 = CONCAT22(local_8._2_2_,*(undefined2 *)(iVar3 + 0x14));
            BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_8,param_4,(char *)0x2,0x1008);
            if (BVar1 != 0x0) {
              local_8 = *(undefined4 *)(iVar3 + 0x16);
              BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_8,param_4,(char *)0x4,0x1008);
              if (BVar1 != 0x0) {
                iVar2 = write_to_file_1008_7b4c(param_2,param_1 & 0xffff0000 | (ulong)(iVar3 + 0x1a),0x1008,param_4);
                if (iVar2 != 0x0) {
                  local_8 = *(ulong *)(iVar3 + 0x20);
                  BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_8,param_4,(char *)0x4,0x1008);
                  if (BVar1 != 0x0) {
                    local_8 = local_8 & 0xffff0000 | (ulong)*(uint *)(iVar3 + 0x24);
                    BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_8,param_4,(char *)0x2,0x1008);
                    if (BVar1 != 0x0) {
                      local_8 = local_8 & 0xffff0000 | (ulong)*(uint *)(iVar3 + 0x26);
                      BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_8,param_4,(char *)0x2,0x1008);
                      if (BVar1 != 0x0) {
                        local_8 = local_8 & 0xffff0000 | (ulong)*(uint *)(iVar3 + 0x28);
                        BVar1 = write_to_file_1008_7e1c(uVar5,uVar6,(ushort)&local_8,param_4,(char *)0x2,0x1008);
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
  PTR_LOOP_1050_0310 = (undefined *)0x6d0;
  return;
}



void __stdcall16far file_1038_774e(ulong param_1,ulong param_2,uchar *param_3,ushort param_4)

{
  ushort uVar1;
  astruct_307 *iVar2;
  BOOL16 BVar2;
  int iVar3;
  ushort uVar4;
  ushort uVar6;
  undefined2 local_8;
  ushort local_6;
  undefined2 local_4;
  ulong *puVar5;
  
  if ((int)PTR_LOOP_1050_0312 < 0x2) {
    return;
  }
  iVar2 = (astruct_307 *)param_1;
  iVar2 = (astruct_307 *)&iVar2->field_0x4;
  puVar5 = (ulong *)(param_1 & 0xffff0000 | ZEXT24(iVar2));
  pass1_1008_766e(param_2,puVar5,param_4,0x1008,param_3);
  if ((int)puVar5 != 0x0) {
    uVar1 = (ushort)(param_1 >> 0x10);
    uVar4 = (ushort)param_2;
    uVar6 = (ushort)(param_2 >> 0x10);
    BVar2 = read_file_1008_7dee(uVar4,uVar6,&iVar2->field_0x8,0x0,uVar1,0x4,0x1008);
    if ((((((BVar2 != 0x0) &&
           (iVar3 = file_1008_77cc(param_2,(long *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0xe),param_3,
                                   0x1008,param_4), iVar3 != 0x0)) &&
          (BVar2 = read_file_1008_7dee(uVar4,uVar6,(ushort)&local_4,0x0,param_4,0x2,0x1008), BVar2 != 0x0)) &&
         ((BVar2 = read_file_1008_7dee(uVar4,uVar6,(ushort)&local_6,0x0,param_4,0x2,0x1008), BVar2 != 0x0 &&
          (BVar2 = read_file_1008_7dee(uVar4,uVar6,(ushort)&local_8,0x0,param_4,0x2,0x1008), BVar2 != 0x0)))) &&
        ((BVar2 = read_file_1008_7dee(uVar4,uVar6,&iVar2->field_0x16,0x0,uVar1,0x4,0x1008), BVar2 != 0x0 &&
         ((BVar2 = read_file_1008_7bc8(param_2,(ushort *)(param_1 & 0xffff0000 | (ulong)(uint)&iVar2->field_0x1a),0x1008
                                       ,param_4), BVar2 != 0x0 &&
          (BVar2 = read_file_1008_7dee(uVar4,uVar6,&iVar2->field_0x20,0x0,uVar1,0x4,0x1008), BVar2 != 0x0)))))) &&
       ((BVar2 = read_file_1008_7dee(uVar4,uVar6,&iVar2->field_0x24,0x0,uVar1,0x2,0x1008), BVar2 != 0x0 &&
        ((BVar2 = read_file_1008_7dee(uVar4,uVar6,&iVar2->field_0x26,0x0,uVar1,0x2,0x1008), BVar2 != 0x0 &&
         (BVar2 = read_file_1008_7dee(uVar4,uVar6,&iVar2->field_0x28,0x0,uVar1,0x2,0x1008), BVar2 != 0x0)))))) {
      iVar2->field_0xc = local_4;
      uVar4 = switch_1008_72bc(uVar4,uVar6,local_6);
      iVar2->field_0x12 = uVar4;
      iVar2->field_0x14 = local_8;
      return;
    }
  }
  PTR_LOOP_1050_0310 = (undefined *)0x6d2;
  return;
}



astruct_18 * __stdcall16far pass1_1038_78b8(astruct_18 *param_1,byte param_2)

{
  pass1_1038_6912(&param_1->field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_78e2(ulong *param_1,uchar *param_2)

{
  uint uVar1;
  uchar *puVar2;
  uchar *extraout_DX;
  undefined2 extraout_DX_00;
  undefined2 uVar3;
  astruct_431 *iVar4;
  undefined2 uVar4;
  
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (astruct_431 *)param_1;
  uVar1 = 0x0;
  *param_1 = 0x0;
  *(undefined4 *)&iVar4->field_0x4 = 0x0;
  _PTR_LOOP_1050_5a64 = param_1;
  mem_op_1000_179c(0xc,param_2,0x1000);
  puVar2 = (uchar *)((uint)param_2 | uVar1);
  if (puVar2 == (uchar *)0x0) {
    *param_1 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_21 *)CONCAT22(param_2,uVar1));
    *(uint *)param_1 = uVar1;
    iVar4->field_0x2 = extraout_DX;
    puVar2 = extraout_DX;
  }
  mem_op_1000_179c(0xc,puVar2,0x1000);
  if (((uint)puVar2 | uVar1) == 0x0) {
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

void __stdcall16far pass1_1038_7964(uint *param_1)

{
  uint uVar1;
  undefined4 *puVar2;
  code **ppcVar3;
  int iVar4;
  undefined2 uVar5;
  
  _PTR_LOOP_1050_5a64 = 0x0;
  uVar5 = (undefined2)((ulong)param_1 >> 0x10);
  iVar4 = (int)param_1;
  uVar1 = *(uint *)(iVar4 + 0x2);
  if ((uVar1 | (uint)(undefined4 *)*param_1) != 0x0) {
    ppcVar3 = (code **)*(undefined4 *)*param_1;
    (**ppcVar3)();
  }
  puVar2 = (undefined4 *)*(uint *)(iVar4 + 0x4);
  uVar1 = *(uint *)(iVar4 + 0x6);
  if ((uVar1 | (uint)puVar2) != 0x0) {
    ppcVar3 = (code **)*puVar2;
    (**ppcVar3)();
  }
  return;
}



void __stdcall16far pass1_1038_79b2(ulong param_1,ulong param_2,uint param_3,uchar *param_4)

{
  code **ppcVar1;
  uint uVar2;
  undefined2 uVar3;
  undefined2 uVar4;
  
  uVar4 = 0x1000;
  mem_op_1000_179c(0x14,param_4,0x1000);
  uVar2 = (uint)param_4 | param_3;
  if (uVar2 == 0x0) {
    param_3 = 0x0;
    uVar2 = 0x0;
  }
  else {
    uVar4 = 0x1030;
    pass1_1030_aefa((ushort *)CONCAT22(param_4,param_3),param_2);
  }
  uVar3 = (undefined2)(param_1 >> 0x10);
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0x4);
  (**ppcVar1)(uVar4,*(undefined4 *)((int)param_1 + 0x4),param_3,uVar2);
  return;
}



void __stdcall16far pass1_1038_79f2(ulong param_1,ulong param_2,ushort param_3)

{
  code **ppcVar1;
  undefined *puVar2;
  uint extraout_DX;
  int iVar3;
  undefined2 uVar4;
  undefined local_e [0x8];
  long lStack6;
  
  lStack6 = *(long *)((int)param_2 + 0x4);
  uVar4 = (undefined2)(param_1 >> 0x10);
  iVar3 = (int)param_1;
  pass1_1008_5784((ulong *)CONCAT22(param_3,local_e),*(ulong *)(iVar3 + 0x4));
  do {
    puVar2 = local_e;
    pass1_1008_5b12(puVar2,param_3);
    if ((extraout_DX | (uint)puVar2) == 0x0) {
      return;
    }
  } while (*(long *)(puVar2 + 0x4) != lStack6);
  ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)(iVar3 + 0x4) + 0xc);
  (**ppcVar1)(0x1008,*(undefined4 *)(iVar3 + 0x4),puVar2,extraout_DX);
  return;
}



void __stdcall16far pass1_1038_7a5a(ulong *param_1)

{
  code **ppcVar1;
  
  ppcVar1 = (code **)((int)*(undefined4 *)*param_1 + 0x4);
  (**ppcVar1)();
  return;
}



void __stdcall16far pass1_1038_7a76(ulong *param_1,undefined2 param_2,int param_3,ushort param_4)

{
  code **ppcVar1;
  ushort uVar2;
  ulong uVar3;
  undefined local_a [0x4];
  undefined4 uStack6;
  
  pass1_1008_5784((ulong *)CONCAT22(param_4,local_a),*param_1);
  while( true ) {
    uVar3 = pass1_1008_5b12(local_a,param_4);
    if (uVar3 == 0x0) break;
    pass1_1038_6a0e(uVar3,(uint)uVar3,(uint)(uVar3 >> 0x10) | (uint)uVar3,param_2,param_3,param_4);
  }
  do {
    uStack6 = 0x0;
    do {
      uVar3 = pass1_1008_5b12(local_a,param_4);
      if (uVar3 == 0x0) {
        pass1_1008_5784((ulong *)CONCAT22(param_4,local_a),*(ulong *)((int)param_1 + 0x4));
        while( true ) {
          uVar3 = pass1_1008_5b12(local_a,param_4);
          if (uVar3 == 0x0) break;
          pass1_1030_affc(uVar3,param_3,param_4);
        }
        return;
      }
      uVar2 = pass1_1038_6b3c(uVar3);
    } while (uVar2 == 0x0);
    ppcVar1 = (code **)((int)*(undefined4 *)*param_1 + 0xc);
    (**ppcVar1)(0x1008);
  } while( true );
}



// WARNING: Could not reconcile some variable overlaps

ushort __stdcall16far pass1_1038_7b20(ulong *param_1,ulong param_2,uint16_t param_3)

{
  undefined4 uVar1;
  BOOL16 BVar2;
  undefined2 uVar3;
  ulong uVar4;
  ushort uVar5;
  undefined2 local_1c;
  undefined2 uStack26;
  undefined2 uStack24;
  undefined4 uStack16;
  undefined local_c [0x8];
  undefined2 local_4;
  
  BVar2 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar2 != 0x0) {
    local_1c = *(undefined2 *)((int)*param_1 + 0x8);
    uVar5 = (ushort)(param_2 >> 0x10);
    local_4 = local_1c;
    BVar2 = write_to_file_1008_7e1c((ushort)param_2,uVar5,(ushort)&local_1c,param_3,(char *)0x2,0x1008);
    if (BVar2 != 0x0) {
      pass1_1008_5784((ulong *)CONCAT22(param_3,local_c),*param_1);
      do {
        uStack16 = pass1_1008_5b12(local_c,param_3);
        if (uStack16 == 0x0) {
          uVar3 = (undefined2)((ulong)param_1 >> 0x10);
          uVar1 = *(undefined4 *)((int)param_1 + 0x4);
          local_1c = *(undefined2 *)((int)uVar1 + 0x8);
          local_4 = local_1c;
          BVar2 = write_to_file_1008_7e1c((ushort)param_2,uVar5,(ushort)&local_4,param_3,(char *)0x2,0x1008);
          if (BVar2 == 0x0) {
            return 0x0;
          }
          pass1_1008_5784((ulong *)CONCAT22(param_3,local_c),*(ulong *)((int)param_1 + 0x4));
          do {
            uVar4 = pass1_1008_5b12(local_c,param_3);
            uStack26 = (undefined2)uVar4;
            if (uVar4 == 0x0) {
              return 0x1;
            }
            pass1_1030_b768(uVar4,param_2,param_3);
            uStack24 = (undefined2)(uVar4 >> 0x10);
          } while ((int)uVar4 != 0x0);
          return 0x0;
        }
        pass1_1038_75ca(uStack16,param_2,(int)uStack16,param_3);
        uStack16._2_2_ = (undefined2)(uStack16 >> 0x10);
      } while ((int)uStack16 != 0x0);
    }
  }
  return 0x0;
}



undefined2 __stdcall16far read_file_1038_7c02(undefined4 *param_1,undefined4 param_2,uint16_t param_3,uint16_t param_4)

{
  code **ppcVar1;
  BOOL16 BVar2;
  uint uVar3;
  uint uVar4;
  uchar *extraout_DX;
  uchar *puVar5;
  uchar *extraout_DX_00;
  uint16_t unaff_SS;
  ushort uVar6;
  undefined2 uVar7;
  ushort uVar8;
  undefined4 uVar9;
  undefined2 uVar10;
  uint local_12 [0x2];
  undefined4 uStack14;
  uint local_4;
  
  if ((int)PTR_LOOP_1050_0312 < 0x2) {
    return 0x1;
  }
  uVar6 = (ushort)param_2;
  uVar8 = (ushort)((ulong)param_2 >> 0x10);
  read_file_1008_7cfe(uVar6,uVar8,0x17,0x1008,unaff_SS);
  if ((param_3 != 0x0) &&
     (BVar2 = read_file_1008_7dee(uVar6,uVar8,(ushort)&local_4,0x0,unaff_SS,0x2,0x1008), BVar2 != 0x0)) {
    while (local_4 != 0x0) {
      uVar7 = 0x2a;
      uVar3 = local_4;
      local_4 = local_4 - 0x1;
      uVar9 = param_2;
      mem_op_1000_179c(0x2a,(uchar *)param_4,0x1000);
      puVar5 = (uchar *)(param_4 | uVar3);
      if (puVar5 == (uchar *)0x0) {
        uVar3 = 0x0;
        puVar5 = (uchar *)0x0;
      }
      else {
        struct_1038_6520((ushort *)CONCAT22(param_4,uVar3));
      }
      uVar10 = (undefined2)((ulong)uVar9 >> 0x10);
      uStack14 = CONCAT22(puVar5,uVar3);
      file_1038_774e(CONCAT22(puVar5,uVar3),CONCAT22((int)uVar9,uVar7),puVar5,unaff_SS);
      if (uVar3 == 0x0) {
        return 0x0;
      }
      ppcVar1 = (code **)((int)*(undefined4 *)*param_1 + 0x4);
      (**ppcVar1)(0x1000,(int)*param_1,(int)((ulong)*param_1 >> 0x10),(int)uStack14,(int)((ulong)uStack14 >> 0x10),
                  uVar10);
      param_4 = (uint16_t)extraout_DX;
    }
    local_4 = local_4 - 0x1;
    BVar2 = read_file_1008_7dee(uVar6,uVar8,(ushort)local_12,0x0,unaff_SS,0x2,0x1008);
    if (BVar2 != 0x0) {
      while( true ) {
        if (local_12[0] == 0x0) {
          return 0x1;
        }
        uVar7 = 0x14;
        uVar3 = local_12[0];
        local_12[0] = local_12[0] - 0x1;
        uVar9 = param_2;
        mem_op_1000_179c(0x14,(uchar *)param_4,0x1000);
        puVar5 = (uchar *)(param_4 | uVar3);
        if (puVar5 == (uchar *)0x0) {
          uVar3 = 0x0;
          puVar5 = (uchar *)0x0;
        }
        else {
          pass1_1030_ae6c((ushort *)CONCAT22(param_4,uVar3));
        }
        uVar10 = (undefined2)((ulong)uVar9 >> 0x10);
        uVar4 = uVar3;
        file_1030_b836(CONCAT22(puVar5,uVar3),CONCAT22((int)uVar9,uVar7),puVar5,unaff_SS);
        if (uVar4 == 0x0) break;
        uVar7 = (undefined2)((ulong)param_1 >> 0x10);
        uVar9 = *(undefined4 *)((int)param_1 + 0x4);
        ppcVar1 = (code **)((int)*(undefined4 *)*(undefined4 *)((int)param_1 + 0x4) + 0x4);
        (**ppcVar1)(0x1030,(int)uVar9,(int)((ulong)uVar9 >> 0x10),uVar3,puVar5,uVar10);
        param_4 = (uint16_t)extraout_DX_00;
      }
      return 0x0;
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

astruct_57 * __stdcall16far
pass1_1038_7d10(astruct_57 *param_1,ushort param_2,uchar *param_3,int param_4,ushort param_5)

{
  astruct_703 *iVar1;
  undefined2 uVar1;
  ushort *puVar2;
  
  struct_1040_b082(param_1,CONCAT22(param_2,0x1853));
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_703 *)param_1;
  *(undefined4 *)&iVar1->field_0x94 = 0x0;
  *(undefined2 *)param_1 = 0x8876;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1038;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x40,param_5,param_3,param_4);
  iVar1->field_0x94 = (int)puVar2;
  iVar1->field_0x96 = (int)((ulong)puVar2 >> 0x10);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far pass1_1038_7d5c(astruct_18 *param_1)

{
  undefined2 uVar1;
  
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  param_1->field_0x0 = 0x8876;
  *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,*(int *)((int)param_1 + 0x6));
  unk_draw_op_1040_b0f8(param_1);
  return;
}



void __stdcall16far destroy_window_1038_7d88(ULONG param_1,UINT16 param_2)

{
  ushort in_DX;
  
  pass1_1008_b544(*(ulong *)((int)param_1 + 0x94),param_2,in_DX,0x1008);
  DestroyWindow16(0x1008);
  return;
}



LRESULT __stdcall16far pass1_1038_7dac(ulong param_1,ushort param_2)

{
  LRESULT LVar1;
  
  pass1_1040_78de(param_1);
  LVar1 = send_dlg_item_msg_1038_844a(param_1,(int)&PTR_LOOP_1050_1040,param_2);
  return LVar1;
}



void __stdcall16far
pass1_1038_7dc6(int param_1,ushort param_2,ushort param_3,ulong param_4,uchar *param_5,ushort param_6,ushort param_7,
               ushort param_8)

{
  bool bVar1;
  
  bVar1 = false;
  if (param_4._2_2_ == 0x1854) {
    if ((int)param_4 != 0x1) goto LAB_1038_7e8c;
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
          if (param_4._2_2_ != (int)s_vrpal_bmp_1050_183a + 0x7) {
LAB_1038_7e77:
            pass1_1040_b54a(param_1,param_2,param_3,param_4,param_5,(ushort)&PTR_LOOP_1050_1040,param_8);
            return;
          }
          msg_box_op_1038_81be(CONCAT22(param_2,param_1),0x0,param_5,param_8);
        }
      }
      goto LAB_1038_7e8c;
    }
    if (param_4._2_2_ == 0x1855) {
      if ((int)param_4 != 0x1) goto LAB_1038_7e8c;
      send_dlg_item_msg_1038_87b2(CONCAT22(param_2,param_1),param_7,param_8);
    }
    else {
      if (param_4._2_2_ == 0x1856) {
        if ((int)param_4 != 0x1) goto LAB_1038_7e8c;
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT __stdcall16far send_dlg_item_msg_1038_7eac(ulong param_1)

{
  uchar *in_DX;
  int unaff_DI;
  ushort unaff_SS;
  ushort *puVar1;
  char *pcVar2;
  LRESULT LVar3;
  
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x30,unaff_SS,in_DX,unaff_DI);
  pcVar2 = (char *)pass1_1010_375e((ulong)puVar1);
  pass1_1008_b1a6(*(ulong *)((int)param_1 + 0x94),pcVar2);
  SendDlgItemMessage16(0x1008,0x0,0x0,0x0,0x1854000b);
  LVar3 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18540409);
  if (LVar3 != -0x1) {
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,(WPARAM16)LVar3,0x18540403);
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,(INT16)pcVar2,(UINT16)((ulong)pcVar2 >> 0x10),0x0,0x18540401);
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xffff,0x18540407);
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18550405);
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18560405);
    enable_win_1038_806a(param_1,(int)s_tile2_bmp_1050_1538);
  }
  LVar3 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x1854000b);
  return LVar3;
}



void __stdcall16far send_dlg_item_msg_1038_7fae(undefined4 param_1)

{
  ushort in_AX;
  ushort in_DX;
  int iVar1;
  undefined2 uVar2;
  undefined2 unaff_SS;
  LRESULT LVar3;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  pass1_1008_b146(*(ulong *)(iVar1 + 0x94),in_AX,in_DX);
  SendDlgItemMessage16(0x1008,0x0,0x0,0xffff,0x18550407);
  LVar3 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0xffff,0x18560407);
  pass1_1008_b61a(*(ulong *)(iVar1 + 0x94),0x0,(ushort)LVar3,(int)((ulong)LVar3 >> 0x10),unaff_SS);
  pass1_1008_b63a(*(ulong *)(iVar1 + 0x94),0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

ulong __stdcall16far pass1_1038_801a(ulong param_1,uchar *param_2,int param_3,ushort param_4)

{
  ushort uVar1;
  ushort uVar2;
  undefined2 uVar3;
  ushort *puVar4;
  char *pcVar5;
  ulong uVar6;
  
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x30,param_4,param_2,param_3);
  uVar3 = (undefined2)(param_1 >> 0x10);
  uVar2 = (ushort)param_1;
  pcVar5 = (char *)pass1_1008_b340(*(ulong *)(uVar2 + 0x94));
  uVar1 = (uint)((ulong)pcVar5 >> 0x10) | (uint)pcVar5;
  uVar6 = (ulong)pcVar5 & 0xffff | (ulong)uVar1 << 0x10;
  if (pcVar5 != (char *)0x0) {
    pass1_1010_3770((ulong)puVar4,pcVar5,uVar1);
    uVar6 = pass1_1038_af40(_PTR_LOOP_1050_5b7c,*(ushort *)(uVar2 + 0x6),0x3,uVar1,uVar2,0x1010,param_4);
  }
  return uVar6;
}



void __stdcall16far enable_win_1038_806a(ulong param_1,HWND16 param_2)

{
  BOOL16 BVar1;
  ushort in_DX;
  int iVar2;
  undefined2 uVar3;
  HWND16 hwnd_dlg;
  ulong uVar4;
  ulong uVar5;
  ulong uVar6;
  
  uVar3 = (undefined2)(param_1 >> 0x10);
  iVar2 = (int)param_1;
  GetDlgItem16(param_2,0x1);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1858);
  EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1859);
  BVar1 = EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x0);
  uVar4 = pass1_1008_b820(*(ulong *)(iVar2 + 0x94),BVar1,in_DX);
  if (uVar4 != 0x0) {
    uVar4 = pass1_1008_b340(*(ulong *)(iVar2 + 0x94));
    uVar5 = pass1_1008_b366(*(ulong *)(iVar2 + 0x94));
    hwnd_dlg = 0x1008;
    uVar6 = pass1_1008_b47a(*(ulong *)(iVar2 + 0x94));
    if (((uVar4 != 0x0) && (uVar5 != 0x0)) && (uVar6 != 0x0)) {
      GetDlgItem16(0x1008,0x1);
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
      GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1858);
      hwnd_dlg = (HWND16)s_tile2_bmp_1050_1538;
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    }
    if (uVar4 != 0x0) {
      GetDlgItem16(hwnd_dlg,0x1859);
      EnableWindow16((HWND16)s_tile2_bmp_1050_1538,0x1);
    }
  }
  return;
}



ushort __stdcall16far
send_dlg_item_msg_1038_8164(ushort param_1,ushort param_2,uchar *param_3,ushort param_4,HWND16 param_5)

{
  LRESULT LVar1;
  
  *param_3 = '\0';
  LVar1 = SendDlgItemMessage16(param_5,0x0,0x0,0x0,CONCAT22(param_4,0x409));
  if ((LVar1 != -0x1) &&
     (LVar1 = SendDlgItemMessage16
                        ((HWND16)s_tile2_bmp_1050_1538,(INT16)param_3,(UINT16)((ulong)param_3 >> 0x10),(WPARAM16)LVar1,
                         CONCAT22(param_4,0x40a)), LVar1 != -0x1)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far msg_box_op_1038_81be(ulong param_1,char *param_2,uchar *param_3,ushort param_4)

{
  char local_206 [0x102];
  char local_104 [0x102];
  
  mem_op_1000_179c(0x1000,param_3,0x1000);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_206,param_4);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,param_2,
             (short)param_3);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,(LPCSTR)0x0,local_206,param_4);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,param_2,
             (short)param_3);
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  load_string_1010_84e0
            (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x3ff,local_104,param_4);
  pass1_1000_3cea(CONCAT22(param_3,param_2),CONCAT22(param_4,local_104));
  MessageBox16(0x1000,(LPCSTR)0x0,local_206,param_4);
  fn_ptr_1000_17ce((astruct_18 *)CONCAT22(param_3,param_2),0x1000);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __stdcall16far set_win_text_1038_8358(ulong param_1,HWND16 param_2,UINT16 param_3)

{
  char *lp_string;
  ushort uVar2;
  ushort in_DX;
  ushort uVar4;
  ushort uVar3;
  HWND16 hwnd;
  char local_30a [0x102];
  CHAR local_208 [0x100];
  undefined local_108 [0x100];
  undefined4 uStack8;
  HWND16 HStack4;
  ulong uVar1;
  
  uVar3 = (ushort)(param_1 >> 0x10);
  uVar4 = (ushort)param_1;
  HStack4 = GetDlgItem16(param_2,0x1857);
  uStack8 = pass1_1008_b820(*(ulong *)(uVar4 + 0x94),HStack4,in_DX);
  if (uStack8 == 0x0) {
    hwnd = 0x1010;
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,local_30a,param_3);
    lp_string = local_30a;
  }
  else {
    uVar2 = send_dlg_item_msg_1038_8164(uVar4,uVar3,(uchar *)CONCAT22(param_3,local_108),0x1854,0x1008);
    if (uVar2 == 0x0) {
      hwnd = 0x1010;
      load_string_1010_84e0
                (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,local_208,param_3
                );
    }
    else {
      hwnd = 0x1008;
      load_string_1008_b65a(*(ULONG *)(uVar4 + 0x94),local_208,CONCAT22(local_108,param_3),param_3);
    }
    lp_string = local_208;
  }
  SetWindowText16(hwnd,(SEGPTR)lp_string);
  return;
}



void __stdcall16far
send_dlg_item_msg_1038_8400(ushort param_1,ushort param_2,ulong param_3,ushort param_4,ushort param_5)

{
  undefined4 uVar1;
  long lVar2;
  undefined local_a [0x8];
  
  pass1_1008_5784((ulong *)CONCAT22(param_5,local_a),param_3);
  while( true ) {
    lVar2 = pass1_1008_5b12(local_a,param_5);
    if (lVar2 == 0x0) break;
    uVar1 = *(undefined4 *)((int)lVar2 + 0x4);
    SendDlgItemMessage16(0x1008,(INT16)uVar1,(UINT16)((ulong)uVar1 >> 0x10),0x0,CONCAT22(param_4,0x401));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT __stdcall16far send_dlg_item_msg_1038_844a(ulong param_1,HWND16 param_2,ushort param_3)

{
  BOOL16 BVar1;
  undefined2 uVar2;
  ushort uVar3;
  LRESULT LVar4;
  char local_108 [0x102];
  undefined4 uStack6;
  
  uVar3 = (ushort)(param_1 >> 0x10);
  SendDlgItemMessage16(param_2,0x0,0x0,0x0,0x1854000b);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1855000b);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1856000b);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18540405);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18550405);
  LVar4 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18560405);
  uStack6 = pass1_1008_b820(*(ulong *)((ushort)param_1 + 0x94),(int)LVar4,(ushort)((ulong)LVar4 >> 0x10));
  if (uStack6 == 0x0) {
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,local_108,param_3);
    SendDlgItemMessage16(0x1010,(INT16)local_108,param_3,0x0,0x18540401);
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x1854000b);
    SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x1855000b);
    LVar4 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x1856000b);
    uVar2 = (undefined2)((ulong)LVar4 >> 0x10);
    GetDlgItem16((HWND16)s_tile2_bmp_1050_1538,0x1857);
    load_string_1010_84e0
              (0x1010,(ushort)_PTR_LOOP_1050_14cc,(ushort)((ulong)_PTR_LOOP_1050_14cc >> 0x10),0x100,local_108,param_3);
    BVar1 = SetWindowText16(0x1010,(SEGPTR)local_108);
    return CONCAT22(uVar2,BVar1);
  }
  send_dlg_item_msg_1038_8400((ushort)param_1,uVar3,uStack6,0x1854,param_3);
  set_win_text_1038_8358(param_1,0x1008,param_3);
  SendDlgItemMessage16(0x1008,0x0,0x0,0x1,0x1854000b);
  SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x1855000b);
  LVar4 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x1856000b);
  return LVar4;
}



// WARNING: Could not reconcile some variable overlaps

ushort __stdcall16far send_dlg_item_msg_1038_8618(ulong param_1,ushort param_2)

{
  int in_AX;
  ushort uVar1;
  undefined *puVar2;
  ushort in_DX;
  uchar *puVar3;
  uint msg;
  ushort uVar4;
  ushort uVar5;
  HWND16 hwnd;
  LRESULT LVar6;
  ulong uVar7;
  ulong uVar8;
  undefined local_106 [0x100];
  undefined4 uStack6;
  
  uVar5 = (ushort)(param_1 >> 0x10);
  uVar4 = (ushort)param_1;
  uStack6 = pass1_1008_b820(*(ulong *)(uVar4 + 0x94),in_AX,in_DX);
  uVar1 = (ushort)uStack6;
  if (uStack6 != 0x0) {
    uVar1 = send_dlg_item_msg_1038_8164(uVar4,uVar5,(uchar *)CONCAT22(param_2,local_106),0x1854,0x1008);
    if (uVar1 != 0x0) {
      SendDlgItemMessage16(0x1008,0x0,0x0,0x0,0x1855000b);
      SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x1856000b);
      SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18550405);
      LVar6 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x0,0x18560405);
      puVar3 = (uchar *)((ulong)LVar6 >> 0x10);
      puVar2 = local_106;
      pass1_1008_b4a0(*(ulong *)(uVar4 + 0x94),CONCAT22(param_2,puVar2),(uint)puVar2,puVar3,param_2);
      pass1_1008_b200(*(ulong *)(uVar4 + 0x94),param_2);
      uVar8 = CONCAT22((uint)puVar3 | (uint)puVar2,puVar2);
      if (((uint)puVar3 | (uint)puVar2) != 0x0) {
        send_dlg_item_msg_1038_8400(uVar4,uVar5,CONCAT22(puVar3,puVar2),0x1855,param_2);
        uVar7 = pass1_1008_b366(*(ulong *)(uVar4 + 0x94));
        msg = (uint)(uVar7 >> 0x10);
        uVar8 = uVar7 & 0xffff | (ulong)(msg | (uint)uVar7) << 0x10;
        if (uVar7 != 0x0) {
          uVar8 = SendDlgItemMessage16(0x1008,(uint)uVar7,msg,0xffff,0x1855040d);
        }
      }
      hwnd = 0x1008;
      uVar8 = pass1_1008_b38c(*(ulong *)(uVar4 + 0x94),(uint)uVar8,(uchar *)(uVar8 >> 0x10));
      if (uVar8 != 0x0) {
        send_dlg_item_msg_1038_8400(uVar4,uVar5,uVar8,0x1856,param_2);
        hwnd = 0x1008;
        uVar8 = pass1_1008_b47a(*(ulong *)(uVar4 + 0x94));
        if (uVar8 != 0x0) {
          hwnd = (HWND16)s_tile2_bmp_1050_1538;
          SendDlgItemMessage16(0x1008,(INT16)uVar8,(UINT16)(uVar8 >> 0x10),0xffff,0x1856040d);
        }
      }
      SendDlgItemMessage16(hwnd,0x0,0x0,0x1,0x1855000b);
      LVar6 = SendDlgItemMessage16((HWND16)s_tile2_bmp_1050_1538,0x0,0x0,0x1,0x1856000b);
      uVar1 = (ushort)LVar6;
    }
  }
  return uVar1;
}



ushort __stdcall16far send_dlg_item_msg_1038_87b2(ulong param_1,ushort param_2,ushort param_3)

{
  ushort uVar1;
  ushort uVar2;
  undefined2 in_DX;
  undefined4 uVar3;
  LRESULT LVar4;
  ushort uVar5;
  undefined local_102 [0x100];
  
  uVar5 = (ushort)param_1;
  uVar1 = (ushort)(param_1 >> 0x10);
  uVar2 = send_dlg_item_msg_1038_8164(uVar5,uVar1,(uchar *)CONCAT22(param_3,local_102),0x1855,param_2);
  if (uVar2 != 0x0) {
    pass1_1008_b61a(*(ulong *)(uVar5 + 0x94),CONCAT22(param_3,local_102),(ushort)local_102,in_DX,param_3);
    uVar3 = *(undefined4 *)(uVar5 + 0x94);
    uVar3 = load_string_1008_b1f0((int)uVar3,(int)((ulong)uVar3 >> 0x10));
    LVar4 = SendDlgItemMessage16(0x1008,(INT16)uVar3,(UINT16)((ulong)uVar3 >> 0x10),0xffff,0x1856040d);
    uVar2 = (ushort)LVar4;
  }
  return uVar2;
}



void __stdcall16far pass1_1038_8810(ulong param_1,ushort param_2,ushort param_3)

{
  ushort uVar1;
  ushort uVar2;
  undefined local_102 [0x100];
  
  uVar2 = (ushort)(param_1 >> 0x10);
  uVar1 = send_dlg_item_msg_1038_8164((ushort)param_1,uVar2,(uchar *)CONCAT22(param_3,local_102),0x1856,param_2);
  if (uVar1 != 0x0) {
    pass1_1008_b63a(*(ulong *)((ushort)param_1 + 0x94),CONCAT22(param_3,local_102));
  }
  return;
}



void __stdcall16far pass1_1038_8848(void)

{
  return;
}



void __stdcall16far pass1_1038_884c(void)

{
  return;
}



astruct_18 * __stdcall16far pass1_1038_8850(astruct_18 *param_1,byte param_2)

{
  pass1_1038_7d5c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1,0x1000);
  }
  return param_1;
}
