//
// Created by cyrex on 2022-05-22.
//

#include "block_1018.h"

void pass1_1018_0000(i16 param_1,u8 *param_2,u32 param_3,u32 param_4)

{
  i16 *piVar1;
  i16 iVar2;
  u32 uVar3;
  i16 iVar4;
  BOOL16 BVar5;
  u16 uVar6;
  u16 in_register_0000000a;
  StructD *pSVar7;
  u16 uVar8;
  u8 local_20 [0x10];
  i16 iStack16;

    // Segment:    4
    // Offset:     00024460
    // Length:     ee6a
    // Min Alloc:  ee6a
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
  pSVar7 = (StructD *)CONCAT22(in_register_0000000a,param_2);
  read_file_1008_7cfe((int)param_4,(int)(param_4 >> 0x10),0x2);
  if (param_1 == 0x0) {
    u16_1050_0310 = 0x6d4;
  }
  else {
    iVar4 = (int)param_3;
    BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(param_3 & 0xffff0000 | (u32)(iVar4 + 0x16)),0x4);
    if ((((BVar5 != 0x0) &&
         (BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(param_3 & 0xffff0000 | (u32)(iVar4 + 0x1a)),0x4),
         BVar5 != 0x0)) &&
        (BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(param_3 & 0xffff0000 | (u32)(iVar4 + 0x20)),0x4),
        BVar5 != 0x0)) &&
       (((BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(param_3 & 0xffff0000 | (u32)(iVar4 + 0x24)),0x4),
         BVar5 != 0x0 &&
         (BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(param_3 & 0xffff0000 | (u32)(iVar4 + 0x30)),0x2),
         BVar5 != 0x0)) &&
        (BVar5 = read_file_1008_7dee((HFILE16 *)param_4,(u8 *)(param_3 & 0xffff0000 | (u32)(iVar4 + 0x32)),0x2),
        BVar5 != 0x0)))) {
      uVar8 = (param_3 >> 0x10);
      if ((iVar4 + 0x30) != 0x0) {
        iVar2 = (iVar4 + 0x32);
        if (_PTR_LOOP_1050_5f2c == 0x0) {
          PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar7);
          PTR_LOOP_1050_5f2e = (u8 *)pSVar7;
        }
        else {
        }
        uVar6 = fn_ptr_op_1000_1708(iVar2 * 0x6,0x0,0x1,PTR_LOOP_1050_5f2c,PTR_LOOP_1050_5f2e);
        (iVar4 + 0x2c) = uVar6;
        (iVar4 + 0x2e) = PTR_LOOP_1050_5f2e;
        pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_20));
        for (iStack16 = 0x0; piVar1 = (i16 *)(iVar4 + 0x30), *piVar1 != iStack16 && iStack16 <= *piVar1; iStack16 += 0x1
            ) {
          BVar5 = read_file_1008_7bc8(param_4,(u16 *)CONCAT22(0x1050,local_20));
          if (BVar5 == 0x0) {
            u16_1050_0310 = 0x6d0;
            return;
          }
          uVar3 = (u32)(iVar4 + 0x2c);
          pass1_1008_3f62((u16 *)(uVar3 & 0xffff0000 | (u32)((int)uVar3 + iStack16 * 0x6)),
                          (u16 *)CONCAT22(0x1050,local_20));
        }
      }
      return;
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}



void pass1_1018_017c(u32 param_1,u16 param_2)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  ((int)param_1 + 0x1e) = param_2;
  pass1_1010_1f62((astruct_27 *)(param_1 & 0xffff | (u32)uVar1 << 0x10),0x4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_0196(u16 param_1,u8 *param_2,u32 param_3,u32 param_4,u32 param_5)

{
  i16 *piVar1;
  i16 iVar2;
  u32 uVar3;
  u16 uVar4;
  u32 uVar5;
  u16 uVar6;
  u16 in_register_0000000a;
  StructD *pSVar7;
  i16 iVar8;
  u16 uVar9;
  u32 uVar10;

  pSVar7 = (StructD *)CONCAT22(in_register_0000000a,param_2);
  pass1_1030_8344(_u16_1050_5748,param_5);
  uVar9 = (param_3 >> 0x10);
  iVar8 = (int)param_3;
  if (*(i32 *)(iVar8 + 0x2c) == 0x0) {
    (iVar8 + 0x32) = 0x5;
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar7);
    }
    else {
      pSVar7 = (StructD *)(_PTR_LOOP_1050_5f2c >> 0x10);
    }
    uVar4 = fn_ptr_op_1000_1708(0x1e,0x0,0x1,PTR_LOOP_1050_5f2c,pSVar7);
  }
  else {
    uVar4 = (iVar8 + 0x30) + 0x1;
    if ((int)uVar4 < (iVar8 + 0x32)) goto LAB_1018_022a;
    piVar1 = (i16 *)(iVar8 + 0x32);
    *piVar1 = *piVar1 + 0x5;
    uVar3 = (u32)(iVar8 + 0x2c);
    uVar10 = pass1_1000_0ed4(0x1,(iVar8 + 0x32) * 0x6,0x0,(astruct_172 *)uVar3,
                             (astruct_172 *)((u32)uVar3 >> 0x10));
    pSVar7 = (StructD *)(uVar10 >> 0x10);
    uVar4 = uVar10;
  }
  (iVar8 + 0x2c) = uVar4;
  (iVar8 + 0x2e) = (int)pSVar7;
LAB_1018_022a:
  uVar6 = SUB42(pSVar7,0x0);
  pass1_1030_8344(_u16_1050_5748,param_4);
  uVar5 = (u32)(uVar4 + 0x10);
  pass1_1030_8344(_u16_1050_5748,uVar5);
  iVar2 = (iVar8 + 0x30);
  piVar1 = (i16 *)(iVar8 + 0x30);
  *piVar1 = *piVar1 + 0x1;
  uVar10 = (u32)(iVar8 + 0x2c);
  pass1_1008_3f62((u16 *)(uVar10 & 0xffff0000 | (u32)((int)uVar10 + iVar2 * 0x6)),
                  (u16 *)CONCAT22(uVar6,(int)uVar5 + 0xc));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_028c(u16 param_1,StructD *param_2,u32 param_3,u32 param_4,u16 param_5)

{
  u32 uVar1;
  code **ppcVar2;
  u8 *puVar3;
  u16 uVar4;
  u16 uVar5;
  i16 iVar6;
  u32 uVar7;
  u16 uVar8;
  u16 uVar9;
  u16 uVar13;
  astruct_57 *paVar10;
  u32 uVar12;
  i16 iVar14;
  u16 uVar15;
  u16 in_stack_0000fe32;
  u16 in_stack_0000fe76;
  u16 in_stack_0000ff56;
  u16 in_stack_0000ff5c;
  u16 in_stack_0000ff60;
  u16 in_stack_0000ff8a;
  u16 in_stack_0000ff9a;
  u16 in_stack_0000ffa0;
  u16 in_stack_0000ffa4;
  i16 iStack36;
  u32 *puStack28;
  u8 local_18 [0x4];
  u16 uStack20;
  astruct_74 *paStack12;
  i16 iStack8;
  u16 uStack6;
  u16 uStack4;
  u32 uVar11;

  pass1_1030_8344(_u16_1050_5748,param_4);
  uStack4 = SUB42(param_2,0x0);
  uStack6 = param_1;
  iStack8 = pass1_1030_5b00(CONCAT22(uStack4,param_1));
  paStack12 = (astruct_74 *)
              mixed_1010_20ba((astruct_57 *)param_2,_u16_1050_0ed0,(u8 **)CONCAT22(param_5,iStack8),
                              in_stack_0000fe76,in_stack_0000ff9a,in_stack_0000ffa0,in_stack_0000ffa4);
  uVar13 = ((u32)param_2 >> 0x10);
  pass1_1008_6c90((u16 *)CONCAT22(0x1050,local_18));
  pass1_1018_0b1e(paStack12,(u16 *)CONCAT22(0x1050,local_18));
  paVar10 = (astruct_57 *)CONCAT22(uVar13,(int)uStack20 >> 0xf);
  if (((int)uStack20 >> 0xf | uStack20) == 0x0) {
    puVar3 = local_18;
    pass1_1030_6522(_PTR_LOOP_1050_5740,CONCAT22(0x1050,puVar3),param_4);
  }
  else {
    puVar3 = local_18;
    pass1_1030_62e4(_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar3),param_4);
  }
  uVar8 = paVar10;
  puStack28 = (u32 *)CONCAT22(uVar8,puVar3);
  uVar4 = uVar8 | puVar3;
  if (uVar4 == 0x0) {
    return;
  }
  pass1_1018_04f2((StructD *)param_3);
  uVar15 = 0x1000;
  mem_op_1000_179c(0x1c,paVar10);
  uVar9 = paVar10 | uVar4;
  uVar11 = (u32)paVar10 & 0xffff0000 | (u32)uVar9;
  iVar14 = (int)param_3;
  uVar13 = (param_3 >> 0x10);
  uVar5 = uVar4;
  if (uVar9 == 0x0) {
    (u32)(iVar14 + 0x12) = 0x0;
  }
  else {
    uVar15 = 0x1008;
    struct_op_1008_8e9e((astruct_78 *)CONCAT22(paVar10,uVar4),0x6,0x24);
    (iVar14 + 0x12) = uVar5;
    (iVar14 + 0x14) = (int)uVar11;
  }
  ppcVar2 = (code **)((int)*puStack28 + 0x10);
  (**ppcVar2)(uVar15,puVar3,uVar8,uVar4);
  for (iStack36 = 0x0; iStack36 < (int)uVar5; iStack36 += 0x1) {
    uVar7 = (u32)iStack36;
    ppcVar2 = (code **)((int)*puStack28 + 0x4);
    (**ppcVar2)();
    uVar4 = uVar11 | uVar7;
    uVar12 = uVar11 & 0xffff0000 | (u32)uVar4;
    if (uVar4 != 0x0) {
      iVar6 = iStack36 / 0x6;
      uVar12 = uVar11 & 0xffff0000 | (long)iStack36 % 0x6 & 0xffffU;
      uVar1 = (u32)(iVar14 + 0xe);
      pass1_1018_dd7c(uVar12,uVar1,((u32)uVar1 >> 0x10),CONCAT22(iStack36 % 0x6,iVar6),
                      uVar7 & 0xffff | uVar11 << 0x10,in_stack_0000ff8a,in_stack_0000ff5c,in_stack_0000ff60,
                      in_stack_0000ff56,in_stack_0000fe32);
      pass1_1008_8faa(*(astruct_78 **)(iVar14 + 0x12),CONCAT22((int)uVar12,iVar6));
    }
    uVar11 = uVar12;
  }
  return;
}



void pass1_1018_03ea(u32 param_1,i16 param_2)

{
  if (param_2 != 0x5) {
    return;
  }
  pass1_1010_1f62((astruct_27 *)(param_1 & 0xffff0000 | (u32)((int)param_1 - 0xa)),0x5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_0412(astruct_27 *param_1,u16 param_2,u32 param_3,u16 param_4,u32 param_5)

{
  astruct_97 *pstruct97_1;
  astruct_97 struct97_128;
  u16 uStack4;

  uStack4 = 0x0;
  if (((0x72 < (int)param_4) && (!SBORROW2(param_4,0x73))) &&
     ((param_4 == 0x75 || (int)(param_4 - 0x74) < 0x1 ||
      ((0x0 < (int)(param_4 - 0x76) && ((int)(param_4 - 0x77) < 0x2)))))) {
    uStack4 = 0x1;
  }
  struct_op_1028_933c((astruct_97 *)CONCAT22(0x1050,&struct97_128),param_2,uStack4,param_4,(u32 *)param_3,
                      (param_3 >> 0x10),(u32)((int)param_1 + 0x24),param_5);
  pstruct97_1 = &struct97_128;
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,pstruct97_1));
  if (pstruct97_1 != NULL) {
    pass1_1010_1f62(param_1,0x6);
  }
  return;
}



void pass1_1018_04a4(u32 param_1,u32 param_2)

{
  (u32)((int)param_1 + 0x16) = param_2;
  return;
}



u32 pass1_1018_04b8(u32 param_1)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22(((int)param_1 + 0x18),((int)param_1 + 0x16));
}



void pass1_1018_04ca(u32 param_1,u32 param_2)

{
  (u32)((int)param_1 + 0x1a) = param_2;
  return;
}



void pass1_1018_04de(u32 param_1,u32 param_2)

{
  (u32)((int)param_1 + 0x20) = param_2;
  return;
}



void pass1_1018_04f2(StructD *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;

  uVar5 = ((u32)param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar1 = (u32 *)(iVar4 + 0x12);
  uVar2 = (iVar4 + 0x14);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (u32)(iVar4 + 0x12) = 0x0;
  return;
}



u16 * pass1_1018_0526(u16 *param_1,u8 param_2)

{
  param_1 = (u16 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0xa));
  pass1_1010_eb66((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * FUN_1018_0532(u16 param_1,StructD *param_2,u8 param_3)

{
  pass1_1010_eb66(param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void struct_1018_0570(astruct_19 *param_1,u16 param_2,u16 param_3)

{
  u32 uVar1;
  code **ppcVar2;
  u16 *puVar3;
  u16 uVar4;
  u32 in_EDX;
  u16 uVar6;
  astruct_57 *paVar5;
  u16 *puVar7;
  u32 *puVar8;
  u16 in_stack_0000fe8e;
  u16 in_stack_0000fe98;
  u16 in_stack_0000ffb2;
  u16 in_stack_0000ffb8;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc6;
  u16 uVar10;
  astruct_19 *uVar9;
  astruct_19 *uVar11;

  uVar6 = ((u32)in_EDX >> 0x10);
  uVar9 = (astruct_19 *)param_1;
  uVar10 = ((u32)param_1 >> 0x10);
  get_sys_metrics_1018_4b1e(param_1,0x0,param_2);
  uVar9->field17_0x20 = 0x389a;
  uVar9->field18_0x22 = 0x1008;
  uVar9->field17_0x20 = 0x3aa8;
  uVar9->field18_0x22 = 0x1008;
  (u32)&uVar9->field19_0x24 = 0x0;
  (u32)&uVar9->field22_0x2c = 0x0;
  pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&uVar9->field24_0x30)));
  puVar7 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&uVar9->field26_0x36)));
  paVar5 = (astruct_57 *)CONCAT22(uVar6,(int)((u32)puVar7 >> 0x10));
  (u32)&uVar9->field_0x3c = 0x0;
  pass1_1008_6c90((u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&uVar9->field34_0x40)));
  uVar6 = 0x0;
  (u32)&uVar9->field_0x4c = 0x0;
  (u32)&uVar9->field_0x5a = 0x0;
  uVar9->field53_0x5e = 0x0;
  (u32)(uVar9 + 0x1) = 0x0;
  &uVar9[0x1].field2_0x4 = 0xff00;
  ((int)&uVar9[0x1].field2_0x4 + 0x2) = 0x0;
  (u32)&uVar9[0x1].field3_0x8 = 0x10000fb;
  (u32)&uVar9[0x1].ver_res_0xc = 0x10000f9;
  (u32)&uVar9[0x1].field8_0x10 = 0x10000ff;
  (u32)&uVar9[0x1].field10_0x14 = 0x10000fe;
  (u32)&uVar9[0x1].field12_0x18 = 0x10000fc;
  (u32)&uVar9[0x1].field15_0x1c = 0x0;
  (u32)&uVar9[0x1].field17_0x20 = 0x0;
  uVar9[0x1].field19_0x24 = 0x1;
  &uVar9[0x1].field20_0x26 = 0x0;
  (u32)((int)&uVar9[0x1].field20_0x26 + 0x2) = 0x0;
  uVar9[0x1].field22_0x2c = 0x0;
  (u32)&uVar9[0x1].field23_0x2e = 0x0;
  &uVar9[0x1].field25_0x32 = 0x0;
  (u32)((int)&uVar9[0x1].field25_0x32 + 0x2) = 0x0;
  uVar9[0x1].field27_0x38 = 0x0;
  (u32)&uVar9[0x1].field_0x3a = 0x0;
  (u32)((int)&uVar9[0x1].field34_0x40 + 0x2) = 0x0;
  &uVar9[0x1].field36_0x46 = 0xffff;
  ((int)&uVar9[0x1].field36_0x46 + 0x2) = 0x0;
  param_1->offset_0x0 = 0x1874;
  uVar9->segment_0x2 = 0x1018;
  uVar9->field17_0x20 = 0x18b0;
  uVar9->field18_0x22 = 0x1018;
  if ((PTR_LOOP_1050_3960 == NULL) && (_PTR_LOOP_1050_3962 == 0x0)) {
    mem_op_1000_179c(0x8,paVar5);
    _PTR_LOOP_1050_3962 = CONCAT22((int)paVar5,uVar6);
    pass1_1000_4906((StructD *)CONCAT22((int)paVar5,uVar6),NULL,0x8);
  }
  PTR_LOOP_1050_3960 = PTR_LOOP_1050_3960 + 0x1;
  puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x2f),in_stack_0000fe98,
                           in_stack_0000ffbc,in_stack_0000ffc2,in_stack_0000ffc6);
  paVar5 = (astruct_57 *)((u32)paVar5 & 0xffff0000);
  uVar9->field22_0x2c = puVar8;
  uVar9->field23_0x2e = ((u32)puVar8 >> 0x10);
  if (param_1 == NULL) {
    puVar3 = NULL;
  }
  else {
    paVar5 = (astruct_57 *)((u32)paVar5 | (u32)uVar10);
    puVar3 = &uVar9->field17_0x20;
  }
  uVar1 = (u32)&uVar9->field22_0x2c;
  uVar6 = uVar1;
  ppcVar2 = (code **)((int)*(u32*)&uVar9->field22_0x2c + 0x4);
  (**ppcVar2)(0x1010,uVar6,(int)((u32)uVar1 >> 0x10),0x0,puVar3,(int)paVar5);
  puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(uVar6,0x2),in_stack_0000fe8e,in_stack_0000ffb2,
                           in_stack_0000ffb8,in_stack_0000ffbc);
  paVar5 = (astruct_57 *)((u32)paVar5 & 0xffff0000 | (u32)puVar8 >> 0x10);
  if (((int)puVar8 + 0x80) != 0x0) {
    uVar9[0x1].field19_0x24 = 0x2;
  }
  puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(uVar6,0x9),in_stack_0000fe8e,in_stack_0000ffb2,
                           in_stack_0000ffb8,in_stack_0000ffbc);
  paVar5 = (astruct_57 *)((u32)paVar5 & 0xffff0000 | (u32)puVar8 >> 0x10);
  &uVar9[0x1].field_0x3e = (int)puVar8;
  &uVar9[0x1].field34_0x40 = (int)((u32)puVar8 >> 0x10);
  uVar4 = pass1_1010_65d0((u32)puVar8 & 0xffff0000 | (u32)&uVar9[0x1].field_0x3e,0x88);
  if (uVar4 != 0x0) {
    ((int)&uVar9[0x1].field36_0x46 + 0x2) = 0x1;
  }
  puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(uVar6,0x3b),in_stack_0000fe8e,in_stack_0000ffb2,
                           in_stack_0000ffb8,in_stack_0000ffbc);
  ((int)&uVar9[0x1].field34_0x40 + 0x2) = (int)puVar8;
  uVar9[0x1].field35_0x44 = ((u32)puVar8 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_078e(StructD *param_1)

{
  u16 uVar1;
  u16 uVar2;
  u16 *puVar3;
  StructD *pSVar4;
  StructD *pstruct_5;
  StructD *uVar6;
  u16 *puStack26;
  char *pcStack6;

  uVar6 = (StructD *)((u32)param_1 >> 0x10);
  pstruct_5 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x1874;
  pstruct_5->address_offset_field_0x2 = 0x1018;
  pstruct_5->field19_0x20 = 0x18b0;
  pstruct_5->field20_0x22 = 0x1018;
  PTR_LOOP_1050_3960 = PTR_LOOP_1050_3960 + -0x1;
  ((int)_PTR_LOOP_1050_3962 + pstruct_5->field11_0x12 * 0x2 + -0x4) = 0x0;
  if (PTR_LOOP_1050_3960 == NULL) {
    fn_ptr_1000_17ce(_PTR_LOOP_1050_3962);
    _PTR_LOOP_1050_3962 = NULL;
  }
  fn_ptr_1000_17ce(*(char **)&pstruct_5->field_0x94);
  fn_ptr_1000_17ce(*(char **)&pstruct_5->field_0x9a);
  fn_ptr_1000_17ce(*(char **)&pstruct_5->field_0x88);
  fn_ptr_1000_17ce(*(char **)&pstruct_5->field_0x8e);
  if (pstruct_5->field29_0x2c != NULL) {
    if (param_1 == NULL) {
      puVar3 = NULL;
      pSVar4 = NULL;
    }
    else {
      puVar3 = &pstruct_5->field19_0x20;
      pSVar4 = uVar6;
    }
    pass1_1010_1ea6((u32)pstruct_5->field29_0x2c,(StructD *)CONCAT22(pSVar4,puVar3));
  }
  if (*(i32 *)&pstruct_5->field_0x9e != 0x0) {
    if (param_1 == NULL) {
      puVar3 = NULL;
      pSVar4 = NULL;
    }
    else {
      puVar3 = &pstruct_5->field19_0x20;
      pSVar4 = uVar6;
    }
    pass1_1010_1ea6((u32)&pstruct_5->field_0x9e,(StructD *)CONCAT22(pSVar4,puVar3));
  }
  uVar1 = &pstruct_5->field_0x60;
  uVar2 = &pstruct_5->field_0x62;
  pcStack6 = (char *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1008_5118(CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(pcStack6);
  }
  (u32)&pstruct_5->field_0x4c = 0x0;
  if (param_1 == NULL) {
    puVar3 = NULL;
    uVar6 = NULL;
  }
  else {
    puVar3 = &pstruct_5->field19_0x20;
  }
  puStack26 = (u16 *)CONCAT22(uVar6,puVar3);
  *puStack26 = 0x389a;
  puVar3[0x1] = 0x1008;
  pass1_1010_1d80(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_0902(u32 *param_1,u32 param_2)

{
  u32 uVar1;
  code **ppcVar2;
  astruct_76 **ppaVar3;
  astruct_76 **ppaVar4;
  i16 iVar5;
  u16 uVar6;
  u32 uVar7;
  u32 uVar8;
  u32 *puVar9;
  u32 *puVar10;

  puVar10 = (u32 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x28));
  ppaVar3 = (astruct_76 **)((int)param_1 + 0x24);
  puVar9 = (u32 *)((u32)param_1 & 0xffff0000 | ZEXT24(ppaVar3));
  uVar6 = param_1;
  ppaVar4 = ppaVar3;
  pass1_1030_8344(_u16_1050_5748,param_2);
  pass1_1030_5a52(CONCAT22(uVar6,ppaVar4),puVar9,puVar10);
  uVar7 = pass1_1008_4772(*ppaVar3);
  ((int)param_1 + 0x5a) = (int)uVar7;
  ((int)param_1 + 0x5c) = (int)(uVar7 >> 0x10);
  iVar5 = pass1_1018_17f0();
  ((int)param_1 + 0x12) = iVar5 + 0x2;
  (iVar5 * 0x2 + (int)_PTR_LOOP_1050_3962) = 0x1;
  ppcVar2 = (code **)((int)*param_1 + 0x18);
  (**ppcVar2)();
  (u32)((int)param_1 + 0x3c) = param_2;
  uVar1 = (u32)((int)param_1 + 0x2c);
  uVar8 = pass1_1010_ec18((int)param_2,param_2,uVar1,((u32)uVar1 >> 0x10),
                          param_2 & 0xffff0000 | (u32)((int)param_1 + 0x3c));
  ((int)param_1 + 0x7c) = (int)uVar8;
  ((int)param_1 + 0x7e) = (int)(uVar8 >> 0x10);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void get_sys_metrics_1018_09a8(u16 param_1,u32 param_2)

{
  u32 uVar1;
  INT16 IVar2;
  INT16 IVar3;
  u16 in_register_0000000a;
  astruct_57 *paVar4;
  i16 iVar5;
  u16 uVar6;
  u32 *puVar7;
  u16 in_stack_0000fe7c;
  u16 in_stack_0000ffa0;
  u16 in_stack_0000ffa6;
  u16 in_stack_0000ffaa;
  i16 *piVar8;
  u16 uVar9;
  char *pcVar10;
  i16 local_a;
  i16 local_8;
  i16 iStack6;
  INT16 IStack4;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  IStack4 = GetSystemMetrics16(SM_CYCAPTION);
  uVar6 = (param_2 >> 0x10);
  iVar5 = (int)param_2;
  iStack6 = (iVar5 + 0x12) + -0x2;
  pcVar10 = (char *)CONCAT22(0x1050,&local_8);
  piVar8 = &local_a;
  uVar9 = SUB42(&DAT_1050_1050,0x0);
  puVar7 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(piVar8,0x48),in_stack_0000fe7c,in_stack_0000ffa0
                           ,in_stack_0000ffa6,in_stack_0000ffaa);
  pass1_1008_3e94((u16 *)((u32)puVar7 & 0xffff0000 | (u32)((int)puVar7 + 0xe)),(u16 *)CONCAT22(uVar9,piVar8),pcVar10
                 );
  (iVar5 + 0x18) = iStack6 * IStack4 + local_8 + 0x146;
  (iVar5 + 0x1a) = iStack6 * IStack4 + local_a + 0x9;
  IVar2 = GetSystemMetrics16(SM_CXBORDER);
  uVar1 = (u32)(iVar5 + 0x5a);
  (iVar5 + 0x1c) = IVar2 * 0x2 + ((int)uVar1 + 0x4);
  IVar2 = GetSystemMetrics16(SM_CYCAPTION);
  IVar3 = GetSystemMetrics16(SM_CYBORDER);
  uVar1 = (u32)(iVar5 + 0x5a);
  (iVar5 + 0x1e) = IVar3 + IVar2 + ((int)uVar1 + 0x8);
  return;
}



u32 pass1_1018_0a50(u32 param_1)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  if ((iVar1 + 0x84) == 0x2) {
    return CONCAT22((iVar1 + 0x2a),(iVar1 + 0x28));
  }
  return CONCAT22((iVar1 + 0x26),(iVar1 + 0x24));
}



void pass1_1018_0a76(u32 param_1)

{
  u16 uVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  if (((int)param_1 + 0x84) == 0x1) {
    uVar1 = 0x2;
  }
  else {
    uVar1 = 0x1;
  }
  ((int)param_1 + 0x84) = uVar1;
  pass1_1010_1f62((astruct_27 *)(param_1 & 0xffff | (u32)uVar2 << 0x10),0x4);
  return;
}



void pass1_1018_0aa0(u32 param_1,u16 param_2)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  (iVar1 + 0x14) = param_2;
  pass1_1018_04de((u32)(iVar1 + 0x2c),(u32)(iVar1 + 0x3c));
  return;
}



void pass1_1018_0ac0(u32 param_1,StructA *param_2)

{
  *(StructA **)((int)param_1 + 0x80) = param_2;
  return;
}



u32 pass1_1018_0ad4(u32 param_1)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22(((int)param_1 + 0x82),((int)param_1 + 0x80));
}



void pass1_1018_0ae8(u32 param_1,u16 param_2)

{
  ((int)param_1 + 0x5e) = param_2;
  return;
}



u16 pass1_1018_0afa(u32 param_1)

{
  return ((int)param_1 + 0x5e);
}



u32 pass1_1018_0b08(u32 param_1)

{
  u32 uVar1;
  i16 iVar2;
  u16 uVar3;

  uVar1 = (u32)((int)param_1 + 0x7c);
  uVar3 = ((u32)uVar1 >> 0x10);
  iVar2 = (int)uVar1;
  return CONCAT22((iVar2 + 0x6),(iVar2 + 0x4));
}



void pass1_1018_0b1e(astruct_74 *param_1,u16 *param_2)

{
  i16 iVar1;
  u32 uVar2;
  astruct_74 *iVar3;
  u16 uVar3;
  u16 local_8;
  i16 local_6;
  i16 local_4;

  iVar3 = (astruct_74 *)param_1;
  iVar3 = (astruct_74 *)&iVar3->field_0x30;
  pass1_1008_3eb4((astruct_615 *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar3)),(u16 *)CONCAT22(0x1050,&local_8),
                  (u16 *)CONCAT22(0x1050,&local_6),(u16 *)CONCAT22(0x1050,&local_4));
  if (local_4 + -0x3 < 0x1) {
    local_4 = 0x3;
  }
  if (local_6 + -0x3 < 0x1) {
    local_6 = 0x3;
  }
  uVar3 = ((u32)param_1 >> 0x10);
  uVar2 = iVar3->field90_0x5a;
  iVar1 = ((int)uVar2 + 0x4);
  if (iVar1 <= local_4 + 0x2) {
    local_4 = iVar1 + -0x3;
  }
  uVar2 = iVar3->field90_0x5a;
  iVar1 = ((int)uVar2 + 0x8);
  if (iVar1 <= local_6 + 0x2) {
    local_6 = iVar1 + -0x3;
  }
  pass1_1008_6cec((u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x40)),local_8,
                  CONCAT22(local_4 + 0x2,local_6 + 0x2),local_8,CONCAT22(local_4 + -0x3,local_6 + -0x3));
  pass1_1008_3f62(param_2,(u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x40)));
  pass1_1008_3f62((u16 *)((u32)param_2 & 0xffff0000 | (u32)((int)param_2 + 0x6)),
                  (u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x46)));
  return;
}



void pass1_1018_0bf4(i16 param_1,u32 param_2,i16 param_3,u32 param_4,u16 param_5)

{
  u32 uVar1;
  u16 uVar2;
  u16 uVar3;
  i32 lVar4;
  StructD *pSVar5;
  u32 uVar6;
  u16 uVar7;
  u8 local_14 [0xc];
  u16 local_8;
  u32 local_6;

  uVar7 = (param_4 >> 0x10);
  switch(param_3) {
  case 0x1:
    (u32)((int)param_2 + 0xc) = 0x0;
    (u32)((int)param_2 + 0x7e) = 0x0;
    return;
  case 0x4:
    pass1_1008_3eb4((astruct_615 *)(param_2 & 0xffff0000 | (u32)((int)param_2 + 0x10)),
                    (u16 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,&local_6),
                    (u16 *)CONCAT22(0x1050,(int)&local_6 + 0x2));
    uVar1 = (u32)((int)param_2 + 0xc);
    local_8 = ((int)uVar1 + 0x1e);
    pass1_1008_3e76((u16 *)(param_2 & 0xffff0000 | (u32)((int)param_2 + 0x10)),local_8,local_6,
                    ((u32)local_6 >> 0x10));
    pass1_1008_6c90((u16 *)CONCAT22(0x1050,local_14));
    pass1_1018_0b1e((astruct_74 *)(param_2 & 0xffff0000 | (u32)((int)param_2 - 0x20)),
                    (u16 *)CONCAT22(0x1050,local_14));
    goto LAB_1018_0c71;
  case 0x5:
  case 0x6:
    uVar6 = param_4 & 0xffff0000 | (u32)param_2;
    uVar2 = (int)param_2 - 0x20;
    pass1_1018_0dc6(uVar6,(astruct_91 *)(param_2 & 0xffff0000 | (u32)uVar2));
    pass1_1018_10c4(uVar6,param_2 & 0xffff0000 | (u32)uVar2);
    pass1_1018_1346(uVar6,(astruct_93 *)(param_2 & 0xffff0000 | (u32)uVar2));
    uVar7 = (uVar6 >> 0x10);
LAB_1018_0c71:
    (u32)((int)param_2 + 0x2c) = 0x0;
    lVar4 = *(i32 *)((int)param_2 + 0x1c);
    pSVar5 = (StructD *)CONCAT22(uVar7,((int)param_2 + 0x1e));
    uVar1 = (u32)((int)param_2 + 0xc);
    if (*(i32 *)((int)uVar1 + 0x20) == lVar4) {
      pass1_1018_028c(lVar4,pSVar5,(u32)((int)param_2 + 0xc),(u32)((int)param_2 + 0x1c),param_5);
      ((int)param_2 + 0x2c) = (int)lVar4;
      ((int)param_2 + 0x2e) = (int)pSVar5;
      pass1_1010_1f62((astruct_27 *)(param_2 & 0xffff0000 | (u32)((int)param_2 - 0x20)),param_3);
      return;
    }
    break;
  case 0x14:
    uVar1 = (u32)((int)param_2 + 0xc);
    if (*(i32 *)((int)uVar1 + 0x20) != *(i32 *)((int)param_2 + 0x1c)) {
      post_win_msg_1020_291a((u32)((int)param_2 + 0x60));
      return;
    }
    break;
  case 0x15:
    uVar3 = pass1_1010_65d0((u32)((int)param_2 + 0x7e),0x88);
    if (uVar3 != 0x0) {
      ((int)param_2 + 0x88) = 0x1;
      return;
    }
  }
  return;
}



void pass1_1018_0d76(u32 param_1)

{
  u32 uVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  uVar1 = (u32)((int)param_1 + 0x2c);
  if (*(i32 *)((int)uVar1 + 0x20) == *(i32 *)((int)param_1 + 0x3c)) {
    return;
  }
  return;
}



void pass1_1018_0d9a(u32 param_1,u16 *param_2,u32 *param_3)

{
  u32 uVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  uVar1 = (u32)((int)param_1 + 0x7c);
  *param_3 = (u32)((int)uVar1 + 0x16);
  uVar1 = (u32)((int)param_1 + 0x7c);
  *param_2 = ((int)uVar1 + 0x1a);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_0dc6(astruct_57 *param_1,astruct_91 *param_2)

{
  u16 *puVar1;
  astruct_92 *paVar2;
  i16 iVar3;
  u32 *puVar4;
  char *pcVar5;
  u16 uVar6;
  u16 uVar7;
  u32 uVar8;
  astruct_91 *iVar13;
  u16 uVar9;
  u32 local_32;
  u16 uStack46;
  u32 uStack44;
  char *pcStack40;
  char *pcStack36;
  char *pcStack32;
  u32 uStack28;
  u32 uStack24;
  astruct_92 local_14;

  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_14),0x1,0x0,0x400);
  uVar9 = ((u32)param_2 >> 0x10);
  iVar13 = (astruct_91 *)param_2;
  pcStack36 = (char *)iVar13->field147_0x94;
  fn_ptr_1000_17ce(pcStack36);
  pcStack40 = (char *)iVar13->field149_0x9a;
  pcStack32 = pcStack40;
  fn_ptr_1000_17ce(pcStack40);
  iVar13->field147_0x94 = 0x0;
  iVar13->field149_0x9a = 0x0;
  iVar13->field146_0x92 = 0x0;
  iVar13->field148_0x98 = 0x0;
  while( true ) {
    paVar2 = &local_14;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar2));
    uVar6 = param_1;
    uStack24 = CONCAT22(uVar6,paVar2);
    param_1 = (astruct_57 *)((u32)param_1 & 0xffff0000 | (u32)(uVar6 | paVar2));
    if ((uVar6 | paVar2) == 0x0) break;
    pcVar5 = (char *)paVar2[0x1c].field4_0x8;
    pcStack40 = pcVar5;
    if (pcVar5 == (char *)0x8000001) {
      puVar1 = &iVar13->field146_0x92;
      *puVar1 = *puVar1 + 0x1;
    }
    else if ((iVar13->field157_0xa8 != 0x0) ||
            (pass1_1008_dfa6(iVar13->field154_0xa2,paVar2->field3_0x4,0x4000001), (int)pcVar5 != 0x0)) {
      puVar1 = &iVar13->field148_0x98;
      *puVar1 = *puVar1 + 0x1;
    }
  }
  if (iVar13->field146_0x92 != 0x0) {
    uVar6 = iVar13->field146_0x92;
    uStack44 = uStack44 & 0xffff0000 | (u32)uVar6;
    uVar6 *= 0x6;
    mem_op_1000_179c(uVar6,param_1);
    uVar7 = param_1;
    pcStack32 = (char *)CONCAT22(uVar7,uVar6);
    param_1 = (astruct_57 *)((u32)param_1 & 0xffff0000 | (u32)(uVar7 | uVar6));
    if ((uVar7 | uVar6) == 0x0) {
      iVar13->field147_0x94 = 0x0;
    }
    else {
      pass1_1000_5586(0x3e38,0x1008,(int)uStack44,0x6,uVar6,uVar7);
      iVar13->field147_0x94 = (u32)pcStack32;
    }
  }
  if (iVar13->field148_0x98 != 0x0) {
    uVar6 = iVar13->field148_0x98;
    uStack44 = uStack44 & 0xffff0000 | (u32)uVar6;
    uVar6 *= 0x6;
    mem_op_1000_179c(uVar6,param_1);
    uVar7 = param_1;
    pcStack32 = (char *)CONCAT22(uVar7,uVar6);
    if ((uVar7 | uVar6) == 0x0) {
      iVar13->field149_0x9a = 0x0;
    }
    else {
      pass1_1000_5586(0x3e38,0x1008,(int)uStack44,0x6,uVar6,uVar7);
      iVar13->field149_0x9a = (u32)pcStack32;
    }
  }
  if (local_14.field6_0x10 != 0x0) {
    local_14.field5_0xc._0_2_ = 0x1;
    local_14.field5_0xc = 0x0;
  }
  uVar8 = (u32)local_14.field5_0xc;
  uStack28 = 0x0;
  local_14.field4_0x8._0_2_ = local_14.field5_0xc;
  local_14.field4_0x8 = local_14.field5_0xc;
LAB_1018_0f74:
  uVar6 = uVar8;
  paVar2 = &local_14;
  pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar2));
  uStack24 = CONCAT22(uVar6,paVar2);
  uVar8 = (u32)(uVar6 | paVar2);
  if ((uVar6 | paVar2) == 0x0) {
    return;
  }
  uStack44 = paVar2[0x1c].field4_0x8;
  pcVar5 = *(char **)&paVar2->field6_0x10;
  pcStack40 = pcVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)pcVar5);
  pcStack36 = (char *)((u32)pcVar5 & 0xffff | uVar8 << 0x10);
  local_32 = (u32)((int)pcVar5 + 0xc);
  uStack46 = ((int)pcVar5 + 0x10);
  puVar4 = &local_32;
  if (uStack44 != 0x8000001) goto LAB_1018_0ffc;
  iVar3 = &iVar13->field147_0x94;
  uVar6 = ((int)&iVar13->field147_0x94 + 0x2);
  uStack28 = uStack28 & 0xffff | (u32)(uStack28 + 0x1) << 0x10;
  goto LAB_1018_0fe8;
LAB_1018_0ffc:
  if ((iVar13->field157_0xa8 != 0x0) ||
     (pass1_1008_dfa6(iVar13->field154_0xa2,*(i32 *)((int)uStack24 + 0x4),0x4000001), puVar4 != NULL)) {
    iVar3 = &iVar13->field149_0x9a;
    uVar6 = ((int)&iVar13->field149_0x9a + 0x2);
    uStack28 = uStack28 & 0xffff0000 | (u32)((int)uStack28 + 0x1);
    uStack28 = (int)uStack28;
LAB_1018_0fe8:
    uVar8 = (u32)uVar6;
    pass1_1008_3f62((u16 *)CONCAT22(uVar6,iVar3 + uStack28 * 0x6),(u16 *)CONCAT22(0x1050,&local_32));
  }
  goto LAB_1018_0f74;
}



void pass1_1018_1054(u32 param_1,u16 *param_2,u32 *param_3)

{
  u32 in_EDX;
  i16 iVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(i32 *)(iVar1 + 0x94) == 0x0) {
    pass1_1018_0dc6(in_EDX,(astruct_91 *)(param_1 & 0xffff | (u32)uVar2 << 0x10));
  }
  *param_3 = (u32)(iVar1 + 0x94);
  *param_2 = (iVar1 + 0x92);
  return;
}



void pass1_1018_108c(u32 param_1,u16 *param_2,u32 *param_3)

{
  u32 in_EDX;
  i16 iVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(i32 *)(iVar1 + 0x9a) == 0x0) {
    pass1_1018_0dc6(in_EDX,(astruct_91 *)(param_1 & 0xffff | (u32)uVar2 << 0x10));
  }
  *param_3 = (u32)(iVar1 + 0x9a);
  *param_2 = (iVar1 + 0x98);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_10c4(u16 param_1,u32 param_2)

{
  i16 iVar1;
  code **ppcVar2;
  u32 uVar3;
  i16 iVar4;
  astruct_92 *paVar5;
  u32 uVar6;
  u16 uVar7;
  u16 uVar8;
  u16 uVar9;
  u16 in_register_0000000a;
  astruct_57 *paVar10;
  u32 uVar11;
  i16 iVar12;
  u16 uVar13;
  u8 uVar14;
  bool bVar15;
  u32 *puVar16;
  u32 uStack60;
  u32 uStack56;
  u32 uStack52;
  u32 *puStack48;
  u32 *puStack40;
  u16 uStack30;
  u16 uStack28;
  astruct_92 local_16;

  paVar10 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar13 = (param_2 >> 0x10);
  iVar12 = (int)param_2;
  iVar1 = (iVar12 + 0x86);
  fn_ptr_1000_17ce(*(char **)(iVar12 + 0x88));
  (iVar12 + 0x86) = 0x0;
  (u32)(iVar12 + 0x88) = 0x0;
  pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_16)),0x1,0x0,0x400);
  uStack30 = 0x0;
  uStack28 = 0x0;
  while( true ) {
    paVar5 = &local_16;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar5));
    uVar7 = paVar10;
    paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)(uVar7 | paVar5));
    if ((uVar7 | paVar5) == 0x0) break;
    if (*(i32 *)(iVar12 + 0x3c) == paVar5->field4_0x8) {
      puVar16 = pass1_1008_c6fa(_u16_1050_06e0,0x2);
      paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)puVar16 >> 0x10);
      uVar9 = puVar16;
      pass1_1038_4e78(uVar9,paVar10,CONCAT22(uVar7,paVar5),puVar16);
      uVar8 = SUB42(paVar10,0x0);
      puStack48 = (u32 *)CONCAT22(uVar8,uVar9);
      uVar3 = *puStack48;
      ppcVar2 = (code **)uVar3 + 0x8;
      uVar7 = uVar9;
      (**ppcVar2)((int)&u16_1050_1038,uVar9,uVar8);
      bVar15 = CARRY2(uStack30,uVar7);
      uStack30 += uVar7;
      uStack28 = uStack28 + (int)paVar10 + bVar15;
      if (puStack48 != NULL) {
        ppcVar2 = (code **)uVar3;
        (**ppcVar2)(0x38,uVar9,uVar8,0x1);
      }
    }
  }
  if ((uStack28 | uStack30) != 0x0) {
    (iVar12 + 0x86) = uStack30;
    uVar7 = uStack30 * 0x6;
    mem_op_1000_179c(uVar7,paVar10);
    uVar9 = paVar10;
    uStack52 = CONCAT22(uVar9,uVar7);
    uVar11 = (u32)paVar10 & 0xffff0000;
    if ((uVar9 | uVar7) == 0x0) {
      (u32)(iVar12 + 0x88) = 0x0;
    }
    else {
      pass1_1000_5586(0x3e38,0x1008,uStack30,0x6,uVar7,uVar9);
      (u32)(iVar12 + 0x88) = uStack52;
    }
    if (local_16.field6_0x10 == 0x0) {
      paVar10 = (astruct_57 *)(uVar11 & 0xffff0000 | (u32)local_16.field5_0xc);
    }
    else {
      local_16.field5_0xc._0_2_ = 0x1;
      paVar10 = (astruct_57 *)(uVar11 & 0xffff0000);
    }
    local_16.field4_0x8 = SUB42(paVar10,0x0);
    iVar4 = 0x0;
    local_16.field4_0x8._0_2_ = local_16.field5_0xc;
    while( true ) {
      paVar5 = &local_16;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,paVar5));
      uVar7 = paVar10;
      paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)(uVar7 | paVar5));
      if ((uVar7 | paVar5) == 0x0) break;
      if (*(i32 *)(iVar12 + 0x3c) == paVar5->field4_0x8) {
        puVar16 = pass1_1008_c6fa(_u16_1050_06e0,0x2);
        paVar10 = (astruct_57 *)((u32)paVar10 & 0xffff0000 | (u32)puVar16 >> 0x10);
        uVar9 = puVar16;
        uVar14 = 0x38;
        pass1_1038_4e78(uVar9,paVar10,CONCAT22(uVar7,paVar5),puVar16);
        uVar8 = SUB42(paVar10,0x0);
        puStack40 = (u32 *)CONCAT22(uVar8,uVar9);
        ppcVar2 = (code **)((int)*puStack40 + 0x10);
        uVar7 = uVar9;
        (**ppcVar2)((int)&u16_1050_1038,uVar9,uVar8);
        uStack56 = CONCAT22((int)paVar10,uVar7);
        for (uStack60 = 0x0; uStack60 < uStack56; uStack60 += 0x1) {
          uVar6 = uStack56;
          pass1_1030_1d58((u32)puStack40);
          uVar11 = (u32)(iVar12 + 0x88);
          uVar14 = 0x8;
          pass1_1008_3f62((u16 *)(uVar11 & 0xffff0000 | (u32)((int)uVar11 + iVar4 * 0x6)),
                          (u16 *)CONCAT22((int)paVar10,(int)uVar6 + 0xc));
          iVar4 += 0x1;
        }
        if (puStack40 != NULL) {
          ppcVar2 = (code **)*puStack40;
          (**ppcVar2)(uVar14,uVar9,uVar8,0x1);
        }
      }
    }
    if ((iVar12 + 0x86) != iVar1) {
      pass1_1010_1f62((astruct_27 *)param_2,0x6);
    }
  }
  return;
}



void pass1_1018_1320(u32 param_1,u16 *param_2,u32 *param_3)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  *param_3 = (u32)((int)param_1 + 0x88);
  *param_2 = ((int)param_1 + 0x86);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_1346(u16 param_1,astruct_93 *param_2)

{
  code **ppcVar1;
  i16 iVar2;
  astruct_92 **ppaVar3;
  u32 uVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  u16 in_register_0000000a;
  astruct_57 *paVar8;
  astruct_93 *iVar9;
  u16 uVar9;
  u8 uVar10;
  u32 *puVar11;
  u32 uVar12;
  u32 uVar13;
  u32 uStack70;
  u32 *puStack56;
  u32 uStack52;
  u32 *puStack48;
  u32 uStack30;
  astruct_92 *local_16;
  u16 uStack14;
  u16 uStack12;
  u16 uStack10;
  u16 uStack8;
  i16 iStack6;
  u16 uStack4;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar9 = ((u32)param_2 >> 0x10);
  iVar9 = (astruct_93 *)param_2;
  uStack4 = iVar9->field137_0x8c;
  fn_ptr_1000_17ce((char *)iVar9->field138_0x8e);
  iVar9->field137_0x8c = 0x0;
  iVar9->field138_0x8e = 0x0;
  pass1_1028_dc52((astruct_92 *)CONCAT13(0x10,CONCAT12(0x50,&local_16)),0x1,0x0,0x400);
  uStack30 = 0x0;
  while( true ) {
    ppaVar3 = &local_16;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,ppaVar3));
    uVar5 = paVar8;
    paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)(uVar5 | ppaVar3));
    if ((uVar5 | ppaVar3) == 0x0) break;
    if (iVar9->field60_0x3c == *(i32 *)(ppaVar3 + 0x4)) {
      puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0x2);
      paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)puVar11 >> 0x10);
      uVar7 = puVar11;
      uVar10 = 0x38;
      pass1_1038_4e78(uVar7,paVar8,CONCAT22(uVar5,ppaVar3),puVar11);
      uVar6 = SUB42(paVar8,0x0);
      puStack48 = (u32 *)CONCAT22(uVar6,uVar7);
      ppcVar1 = (code **)((int)*puStack48 + 0x10);
      uVar5 = uVar7;
      (**ppcVar1)((int)&u16_1050_1038,uVar7,uVar6);
      uStack52 = CONCAT22((int)paVar8,uVar5);
      for (puStack56 = NULL; puStack56 < uStack52; puStack56 = (u32 *)((long)puStack56 + 0x1)) {
        uVar10 = 0x30;
        uVar12 = pass1_1030_1d7c(uVar5,(int)paVar8,(u32)puStack48);
        paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | uVar12 >> 0x10);
        if (((int)uVar12 + 0x12) == 0x9) {
          uStack30 += 0x1;
        }
      }
      if (puStack48 != NULL) {
        ppcVar1 = (code **)*puStack48;
        (**ppcVar1)(uVar10,uVar7,uVar6,0x1);
      }
    }
  }
  if ((uStack30 | uStack30) != 0x0) {
    iVar9->field137_0x8c = uStack30;
    uVar5 = uStack30 * 0x6;
    mem_op_1000_179c(uVar5,paVar8);
    uVar7 = paVar8;
    uStack70 = CONCAT22(uVar7,uVar5);
    uVar12 = (u32)paVar8 & 0xffff0000;
    if ((uVar7 | uVar5) == 0x0) {
      iVar9->field138_0x8e = 0x0;
    }
    else {
      pass1_1000_5586(0x3e38,0x1008,uStack30,0x6,uVar5,uVar7);
      iVar9->field138_0x8e = uStack70;
    }
    if (iStack6 == 0x0) {
      paVar8 = (astruct_57 *)(uVar12 & 0xffff0000 | (u32)uStack8);
    }
    else {
      uStack10 = 0x1;
      paVar8 = (astruct_57 *)(uVar12 & 0xffff0000);
    }
    uStack12 = SUB42(paVar8,0x0);
    iVar2 = 0x0;
    uStack14 = uStack10;
    while( true ) {
      ppaVar3 = &local_16;
      pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,ppaVar3));
      uVar5 = paVar8;
      paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)(uVar5 | ppaVar3));
      if ((uVar5 | ppaVar3) == 0x0) break;
      if (iVar9->field60_0x3c == *(i32 *)(ppaVar3 + 0x4)) {
        puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0x2);
        paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)puVar11 >> 0x10);
        uVar7 = puVar11;
        uVar10 = 0x38;
        pass1_1038_4e78(uVar7,paVar8,CONCAT22(uVar5,ppaVar3),puVar11);
        uVar6 = SUB42(paVar8,0x0);
        puStack56 = (u32 *)CONCAT22(uVar6,uVar7);
        ppcVar1 = (code **)((int)*puStack56 + 0x10);
        uVar5 = uVar7;
        (**ppcVar1)((int)&u16_1050_1038,uVar7,uVar6);
        uStack52 = CONCAT22((int)paVar8,uVar5);
        for (puStack48 = NULL; puStack48 < uStack52; puStack48 = (u32 *)((long)puStack48 + 0x1)) {
          uVar4 = uStack52;
          pass1_1030_1d58((u32)puStack56);
          uVar12 = (long)paVar8 << 0x10;
          uVar10 = 0x30;
          uVar13 = struct_op_1030_73a8((astruct_419 *)(uVar4 & 0xffff | (long)paVar8 << 0x10),(int)uVar4,(int)paVar8);
          paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | uVar13 >> 0x10);
          if (((int)uVar13 + 0x12) == 0x9) {
            uVar13 = iVar9->field138_0x8e;
            uVar10 = 0x8;
            pass1_1008_3f62((u16 *)(uVar13 & 0xffff0000 | (u32)((int)uVar13 + iVar2 * 0x6)),
                            (u16 *)(uVar12 | (int)uVar4 + 0xc));
            iVar2 += 0x1;
          }
        }
        if (puStack56 != NULL) {
          ppcVar1 = (code **)*puStack56;
          (**ppcVar1)(uVar10,uVar7,uVar6,0x1);
        }
      }
    }
    if (iVar9->field137_0x8c != uStack4) {
      pass1_1010_1f62((astruct_27 *)param_2,0x6);
    }
  }
  return;
}



void pass1_1018_15f6(u32 param_1,u16 *param_2,u32 *param_3)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  *param_3 = (u32)((int)param_1 + 0x8e);
  *param_2 = ((int)param_1 + 0x8c);
  return;
}



void pass1_1018_161c(u32 param_1,u16 *param_2,i16 param_3,i16 param_4)

{
  u16 uVar1;
  u16 uVar2;
  u32 local_6;

  pass1_1008_3e94((u16 *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0x36)),(u16 *)CONCAT22(0x1050,&local_6),
                  (char *)CONCAT22(0x1050,(int)&local_6 + 0x2));
  uVar1 = local_6 + param_4 + -0x3;
  uVar2 = (int)local_6 + param_3 + -0x3;
  local_6 = CONCAT22(uVar1,uVar2);
  pass1_1008_3e76(param_2,((int)param_1 + 0x44),uVar2,uVar1);
  return;
}



void pass1_1018_1662(u32 param_1,i16 param_2,i16 param_3)

{
  i16 local_6;
  i16 local_4;

  pass1_1008_3e94((u16 *)(param_1 & 0xffff0000 | (u32)((int)param_1 + 0x36)),(u16 *)CONCAT22(0x1050,&local_6),
                  (char *)CONCAT22(0x1050,&local_4));
  pass1_1018_16b8(param_1,((int)param_1 + 0x44),CONCAT22(local_4 + param_3,local_6 + param_2));
  return;
}



void pass1_1018_169e(u32 param_1,u32 param_2)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  pass1_1018_16b8(param_1 & 0xffff | (u32)uVar1 << 0x10,((int)param_1 + 0x44),param_2);
  return;
}



// WARNING: Unable to use type for symbol uVar2

void pass1_1018_16b8(u32 param_1,u16 param_2,u32 param_3)

{
  i16 iVar1;
  u32 uVar3;
  i32 lVar4;
  u32 in_EDX;
  u16 uVar6;
  StructD *pSVar5;
  i16 iVar7;
  u16 uVar8;
  u16 in_stack_0000ffb0;
  u16 uVar9;
  u8 local_6 [0x2];
  u8 local_4 [0x2];
  u32 uVar2;

  uVar6 = ((u32)in_EDX >> 0x10);
  if (param_3 + -0x3 < 0x1) {
    param_3 = CONCAT22(0x3,(int)param_3);
  }
  if ((int)param_3 + -0x3 < 0x1) {
    param_3 = CONCAT22(param_3,0x3);
  }
  uVar8 = (param_1 >> 0x10);
  iVar7 = (int)param_1;
  uVar2 = (u32)(iVar7 + 0x5a);
  iVar1 = ((int)uVar2 + 0x4);
  if (iVar1 <= param_3 + 0x2) {
    param_3 = param_3 & 0xffff | (u32)(iVar1 - 0x3) << 0x10;
  }
  uVar3 = (u32)(iVar7 + 0x5a);
  iVar1 = ((int)uVar3 + 0x8);
  if (iVar1 <= (int)param_3 + 0x2) {
    param_3 = param_3 & 0xffff0000 | (u32)(iVar1 - 0x3);
  }
  uVar9 = (param_3 >> 0x10);
  pass1_1008_3e76((u16 *)(param_1 & 0xffff0000 | (u32)(iVar7 + 0x30)),param_2,param_3,uVar9);
  pSVar5 = (StructD *)CONCAT22(uVar6,uVar8);
  pass1_1008_3e94((u16 *)(param_1 & 0xffff0000 | (u32)(iVar7 + 0x36U)),(u16 *)CONCAT22(0x1050,local_6),
                  (char *)CONCAT22(0x1050,local_4));
  pass1_1008_3e76((u16 *)(param_1 & 0xffff0000 | (u32)(iVar7 + 0x36U)),0x0,param_3,uVar9);
  (u32)(iVar7 + 0x4c) = 0x0;
  lVar4 = *(i32 *)(iVar7 + 0x3c);
  uVar3 = (u32)(iVar7 + 0x2c);
  if (*(i32 *)((int)uVar3 + 0x20) == lVar4) {
    pass1_1018_028c(lVar4,pSVar5,(u32)(iVar7 + 0x2c),(u32)(iVar7 + 0x3c),in_stack_0000ffb0);
    (iVar7 + 0x4c) = (int)lVar4;
    (iVar7 + 0x4e) = (int)pSVar5;
    pass1_1010_1f62((astruct_27 *)param_1,0x4);
  }
  return;
}



void pass1_1018_179e(u32 param_1,u32 param_2)

{
  u16 local_8;
  u32 local_6;

  pass1_1008_3eb4((astruct_615 *)param_2,(u16 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,&local_6),
                  (u16 *)CONCAT22(0x1050,(int)&local_6 + 0x2));
  pass1_1018_16b8(param_1,local_8,local_6);
  return;
}



void pass1_1018_17ce(u32 param_1,u32 param_2,u32 param_3)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  pass1_1018_0412(*(astruct_27 **)((int)param_1 + 0x2c),param_2,CONCAT22((int)param_3,(int)(param_2 >> 0x10)),
                  (param_3 >> 0x10),(u32)((int)param_1 + 0x3c));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 pass1_1018_17f0(void)

{
  i16 iStack4;

  iStack4 = 0x0;
  while ((iStack4 < 0x4 && ((iStack4 * 0x2 + (int)_PTR_LOOP_1050_3962) != 0x0))) {
    iStack4 += 0x1;
  }
  return iStack4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_181c(astruct_610 *param_1,u32 param_2,char *param_3)

{
  pass1_1030_8344(_u16_1050_5748,(u32)((int)param_2 + 0x3c));
  pass1_1030_5b6c(param_1,(astruct_610 *)CONCAT22(param_1,(int)((u32)param_1 >> 0x10)),param_3);
  return;
}



u16 * pass1_1018_1842(u16 *param_1,u8 param_2)

{
  param_1 = (u16 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0x20));
  pass1_1018_078e((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * FUN_1018_184e(u16 param_1,StructD *param_2,u8 param_3)

{
  pass1_1018_078e(param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_18b8(astruct_19 *param_1,u16 param_2)

{
  u16 uVar2;
  u16 uVar4;
  u32 in_EDX;
  astruct_57 *paVar5;
  u32 uVar6;
  astruct_19 *iVar3;
  astruct_19 *uVar3;
  u16 *puVar7;
  u32 *puVar8;
  u32 uVar9;
  u16 in_stack_0000fe7e;
  u16 in_stack_0000ffa2;
  u16 in_stack_0000ffa8;
  u16 in_stack_0000ffac;
  i16 *piVar10;
  u16 uVar11;
  i16 *piVar12;
  u16 uVar13;
  i16 local_6;
  i16 local_4;
  u16 uVar1;

  uVar11 = ((u32)in_EDX >> 0x10);
  get_sys_metrics_1018_4b1e(param_1,0x0,param_2);
  uVar3 = (astruct_19 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_19 *)param_1;
  iVar3->field17_0x20 = 0x389a;
  iVar3->field18_0x22 = 0x1008;
  iVar3->field17_0x20 = 0x3aa8;
  iVar3->field18_0x22 = 0x1008;
  (u32)&iVar3->field19_0x24 = 0x0;
  ((int)&iVar3->field20_0x26 + 0x2) = 0x4;
  puVar7 = pass1_1008_3e38((astruct_19 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x3a)));
  paVar5 = (astruct_57 *)CONCAT22(uVar11,(int)((u32)puVar7 >> 0x10));
  iVar3->field34_0x40 = 0x0;
  iVar3->field35_0x44 = 0x0;
  iVar3->field36_0x46 = 0x0;
  iVar3->field37_0x4a = 0x0;
  iVar3->field47_0x56 = 0x0;
  param_1->offset_0x0 = 0x1fb0;
  iVar3->segment_0x2 = 0x1018;
  iVar3->field17_0x20 = 0x1fec;
  iVar3->field18_0x22 = 0x1018;
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x4e)),NULL,0x8);
  piVar12 = &local_4;
  uVar13 = SUB42(&DAT_1050_1050,0x0);
  piVar10 = &local_6;
  uVar11 = SUB42(&DAT_1050_1050,0x0);
  puVar8 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(piVar10,0x48),in_stack_0000fe7e,
                           in_stack_0000ffa2,in_stack_0000ffa8,in_stack_0000ffac);
  uVar6 = (u32)paVar5 & 0xffff0000 | (u32)puVar8 >> 0x10;
  uVar2 = (int)puVar8 + 0xe;
  pass1_1008_3e94((u16 *)((u32)puVar8 & 0xffff0000 | (u32)uVar2),(u16 *)CONCAT22(uVar11,piVar10),
                  (char *)CONCAT22(uVar13,piVar12));
  uVar4 = FUN_1010_830a(uVar2,uVar6,0x1008,_u16_1050_14cc,0x9a);
  iVar3->field19_0x24 = uVar4;
  &iVar3->field20_0x26 = (int)uVar6;
  uVar9 = pass1_1008_4772((astruct_76 *)CONCAT22((int)uVar6,iVar3->field19_0x24));
  uVar1 = (uVar9 >> 0x10);
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field25_0x32)),NULL,0x8);
  iVar3->field26_0x36 = ((int)uVar9 + 0x4);
  iVar3->field27_0x38 = ((int)uVar9 + 0x8);
  iVar3->field21_0x2a = local_4 + 0x14;
  iVar3->field22_0x2c = local_6 + 0x14;
  get_sys_metrics_1018_1ea0(param_1);
  pass1_1008_3e76((u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(&iVar3->field_0x3a)),0x0,0x88,0x99);
  return;
}



void pass1_1018_1a04(StructD *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  u16 *puVar4;
  StructD *iVar5;
  StructD *uVar5;
  u16 *puStack14;

  uVar5 = (StructD *)((u32)param_1 >> 0x10);
  iVar5 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x1fb0;
  iVar5->address_offset_field_0x2 = 0x1018;
  iVar5->field19_0x20 = 0x1fec;
  iVar5->field20_0x22 = 0x1018;
  puVar1 = (u32*)&iVar5->field_0x24;
  uVar2 = &iVar5->field_0x26;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce(*(char **)&iVar5->field_0x40);
  if (param_1 == NULL) {
    puVar4 = NULL;
    uVar5 = NULL;
  }
  else {
    puVar4 = &iVar5->field19_0x20;
  }
  puStack14 = (u16 *)CONCAT22(uVar5,puVar4);
  *puStack14 = 0x389a;
  puVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_1a8e(u8 *param_1,astruct_653 *param_2)

{
  i16 *piVar1;
  i32 lVar2;
  u16 in_register_0000000a;
  astruct_653 *iVar2;
  u16 uVar3;
  u32 *puVar4;
  u16 in_stack_0000fe98;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc6;
  i16 *piVar5;
  u16 uVar6;
  i16 local_8;
  u32 uStack6;

  uVar3 = ((u32)param_2 >> 0x10);
  iVar2 = (astruct_653 *)param_2;
  if (iVar2->field65_0x44 != 0x0) {
    if (iVar2->field66_0x46 != 0x0) {
      lVar2 = iVar2->field66_0x46;
      ((int)lVar2 + 0xe) = 0x0;
      iVar2->field66_0x46 = 0x0;
    }
    piVar1 = &iVar2->field67_0x4a;
    *piVar1 = *piVar1 + 0x1;
    return;
  }
  piVar5 = &local_8;
  uVar6 = SUB42(&DAT_1050_1050,0x0);
  puVar4 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(piVar5,0x3),in_stack_0000fe98,in_stack_0000ffbc,in_stack_0000ffc2,
                           in_stack_0000ffc6);
  pass1_1010_bf1e((int)puVar4,(u8 *)((u32)puVar4 >> 0x10),(u32)puVar4,(i16 *)CONCAT22(uVar6,piVar5));
  iVar2->field65_0x44 = local_8;
  iVar2->field64_0x40 = uStack6;
  pass1_1018_1ce8((u32)param_2);
  return;
}



// WARNING: Unable to use type for symbol uVar2

void pass1_1018_1b02(astruct_95 *param_1,i16 param_2)

{
  i16 *piVar1;
  u32 uVar3;
  astruct_96 *pstruct96_4;
  astruct_95 *pstruct95_5;
  u16 uVar4;
  i16 iStack12;
  u16 local_6;
  u8 local_4 [0x2];
  u32 uVar2;
  astruct_96 *pstruct96_2;

  iStack12 = 0x0;
  while( true ) {
    uVar4 = ((u32)param_1 >> 0x10);
    pstruct95_5 = (astruct_95 *)param_1;
    piVar1 = &pstruct95_5->field65_0x44;
    if (*piVar1 == iStack12 || *piVar1 < iStack12) break;
    pstruct96_2 = pstruct95_5->field64_0x40;
    pstruct96_4 = (astruct_96 *)pstruct96_2;
    pstruct96_4 = pstruct96_4 + iStack12;
    uVar2 = (u32)pstruct96_2 & 0xffff0000;
    uVar3 = ZEXT24(pstruct96_4);
    piVar1 = &pstruct96_4->field6_0x6;
    *piVar1 = *piVar1 + param_2 * 0x2 + -0x1;
    uVar4 = (uVar2 >> 0x10);
    if (0x23 < pstruct96_4->field6_0x6) {
      pstruct96_4->field6_0x6 = 0x0;
    }
    if (pstruct96_4->field6_0x6 < 0x0) {
      pstruct96_4->field6_0x6 = 0x23;
    }
    pass1_1008_3f62((u16 *)(uVar2 | ZEXT24(&pstruct96_4->field_0x10)),(u16 *)(uVar2 | uVar3));
    pstruct96_4->field19_0x16 = pstruct96_4->field8_0xa;
    pass1_1008_3e94((u16 *)(uVar2 | uVar3),(u16 *)CONCAT22(0x1050,&local_6),(char *)CONCAT22(0x1050,local_4));
    pass1_1008_3e76((u16 *)(uVar2 | uVar3),0x0,local_6,
                    ((pstruct96_4->field7_0x8 * 0x24 + pstruct96_4->field6_0x6) * 0x2 + 0x3c20));
    pstruct96_4->field8_0xa = (pstruct96_4->field6_0x6 * 0x2 + 0x3966);
    iStack12 += 0x1;
  }
  pass1_1010_1f62((astruct_27 *)param_1,0xd);
  return;
}



// WARNING: Could not reconcile some variable overlaps

void pt_in_rect_1018_1bda(u32 param_1,u16 param_2,u16 param_3)

{
  i16 *piVar1;
  u16 uVar2;
  i16 iVar3;
  BOOL16 BVar4;
  i16 iVar5;
  u16 uVar6;
  i16 iStack26;
  POINT16 PStack24;
  i16 local_14;
  i16 local_12;
  u16 uStack16;
  u32 uStack14;
  u32 local_a;
  i16 iStack6;
  i16 iStack4;

  uStack14 = 0x0;
  iVar3 = (int)param_1;
  pass1_1008_3e94((u16 *)(param_1 & 0xffff0000 | (u32)(iVar3 + 0x3a)),(u16 *)CONCAT22(0x1050,&local_14),
                  (char *)CONCAT22(0x1050,&local_12));
  PStack24 = (POINT16)CONCAT22(param_2,param_3);
  uStack16 = 0x0;
  iStack26 = 0x0;
  while( true ) {
    uVar6 = (param_1 >> 0x10);
    piVar1 = (i16 *)(iVar3 + 0x44);
    if (*piVar1 == iStack26 || *piVar1 < iStack26) {
      return;
    }
    uVar2 = (iVar3 + 0x42);
    iVar5 = (iVar3 + 0x40) + iStack26 * 0x18;
    uStack14 = CONCAT22(uVar2,iVar5);
    pass1_1008_3e94((u16 *)CONCAT22(uVar2,iVar5),(u16 *)CONCAT22(0x1050,(int)&local_a + 0x2),
                    (char *)CONCAT22(0x1050,&local_a));
    (int)local_a += local_12 + -0x6;
    iStack6 = (int)local_a + 0xc;
    local_a += local_14 + -0x6;
    iStack4 = local_a + 0xc;
    BVar4 = PtInRect16(PStack24,(RECT16 *)&local_a);
    if (BVar4 != 0x0) break;
    iStack26 += 0x1;
  }
  pass1_1018_1eda(param_1,uStack14);
  return;
}



u16 pass1_1018_1c9a(astruct_263 *param_1,i16 param_2)

{
  i16 *piVar1;
  u32 uVar2;
  u16 uVar3;
  u16 uVar4;
  i16 iStack10;

  iStack10 = 0x0;
  while( true ) {
    uVar4 = ((u32)param_1 >> 0x10);
    piVar1 = (i16 *)((int)param_1 + 0x44);
    if (*piVar1 == iStack10 || *piVar1 < iStack10) {
      return 0x0;
    }
    uVar2 = (u32)((int)param_1 + 0x40);
    uVar3 = (int)uVar2 + iStack10 * 0x18;
    if (((uVar3 + 0xc) * 0x1e + 0x3c32) == param_2) break;
    iStack10 += 0x1;
  }
  pass1_1018_1eda((u32)param_1,uVar2 & 0xffff0000 | (u32)uVar3);
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps

void pass1_1018_1ce8(u32 param_1)

{
  i16 *piVar1;
  i16 iVar2;
  i16 iVar3;
  u16 uVar4;
  u16 uVar5;
  i16 iStack26;
  u8 local_18 [0x2];
  u8 local_16 [0x2];
  i16 iStack20;
  i16 iStack18;
  i16 iStack16;
  u16 local_e;
  i16 local_c;
  i16 local_a;
  i16 iStack8;
  u32 uStack6;

  uVar5 = (param_1 >> 0x10);
  iVar3 = (int)param_1;
  uStack6 = (u32)(iVar3 + 0x40);
  iStack8 = 0x0;
  do {
    piVar1 = (i16 *)(iVar3 + 0x44);
    if (*piVar1 == iStack8 || *piVar1 < iStack8) {
      return;
    }
    pass1_1008_3eb4((astruct_615 *)(uStack6 & 0xffff0000 | (u32)(iStack8 * 0x18 + (int)uStack6)),
                    (u16 *)CONCAT22(0x1050,&local_e),(u16 *)CONCAT22(0x1050,&local_c),
                    (u16 *)CONCAT22(0x1050,&local_a));
    local_a /= 0xa;
    iStack16 = local_c % 0xa;
    if (iStack16 != 0x0) {
      if (iStack16 < 0x6) {
        local_c -= iStack16;
      }
      else {
        local_c += 0xa - iStack16;
      }
    }
    iStack18 = pass1_1000_49b2(local_e);
    iStack18 /= 0x5;
    if (0x14 < iStack18) {
      iStack18 = 0x14;
      iVar2 = pass1_1000_49b2(local_e);
      local_e = ((int)local_e / iVar2) * 0x64;
    }
    iStack16 = pass1_1000_49b2(local_e);
    iStack16 %= 0x5;
    if (iStack16 != 0x0) {
      if ((int)local_e < 0x0) {
        iVar2 = iStack16;
        if (0x2 < iStack16) {
          iVar2 = iStack16 + -0x5;
        }
        local_e += iVar2;
      }
      else if (iStack16 < 0x3) {
        local_e -= iStack16;
      }
      else {
        local_e += 0x5 - iStack16;
      }
    }
    iStack20 = (iStack18 * 0x48 + 0x3c20);
    if (local_c < iStack20) {
      for (iStack26 = iStack18; iStack26 < 0x15; iStack26 += 0x1) {
        piVar1 = (i16 *)(iStack26 * 0x48 + 0x3c20);
        if (*piVar1 == local_c || *piVar1 < local_c) {
          iStack18 = iStack26;
          break;
        }
      }
    }
    pass1_1008_3e94((u16 *)(param_1 & 0xffff0000 | (u32)(iVar3 + 0x3a)),(u16 *)CONCAT22(0x1050,local_18),
                    (char *)CONCAT22(0x1050,local_16));
    uVar4 = iStack8 * 0x18 + (int)uStack6;
    (uVar4 + 0x6) = local_a;
    (uVar4 + 0x8) = iStack18;
    pass1_1008_3e76((u16 *)(uStack6 & 0xffff0000 | (u32)uVar4),0x0,local_e,
                    ((iStack18 * 0x24 + local_a) * 0x2 + 0x3c20));
    (uVar4 + 0xa) = (local_a * 0x2 + 0x3966);
    iStack8 += 0x1;
  } while( true );
}



u32 pass1_1018_1e78(u32 param_1,i16 param_2)

{
  u32 uVar1;

  if (param_2 == -0x1) {
    uVar1 = (u32)((int)param_1 + 0x46);
    param_2 = ((int)uVar1 + 0xc);
  }
  return CONCAT22(0x1050,param_2 * 0x1e + 0x3c18);
}



void get_sys_metrics_1018_1ea0(astruct_19 *param_1)

{
  INT16 IVar1;
  INT16 IVar2;
  astruct_19 *iVar3;
  astruct_19 *uVar3;

  IVar1 = GetSystemMetrics16(SM_CXBORDER);
  uVar3 = (astruct_19 *)((u32)param_1 >> 0x10);
  iVar3 = (astruct_19 *)param_1;
  iVar3->field23_0x2e = IVar1 * 0x2 + iVar3->field26_0x36;
  IVar1 = GetSystemMetrics16(SM_CYCAPTION);
  IVar2 = GetSystemMetrics16(SM_CYBORDER);
  iVar3->field24_0x30 = IVar1 + iVar3->field27_0x38 + IVar2;
  return;
}



void pass1_1018_1eda(u32 param_1,u32 param_2)

{
  u32 uVar1;
  i16 iVar2;
  u16 uVar3;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (*(i32 *)(iVar2 + 0x46) != 0x0) {
    uVar1 = (u32)(iVar2 + 0x46);
    ((int)uVar1 + 0xe) = 0x2;
  }
  (u32)(iVar2 + 0x46) = param_2;
  ((int)param_2 + 0xe) = 0x1;
  pass1_1010_1f62((astruct_27 *)param_1,0xd);
  return;
}



u16 pass1_1018_1f1a(u32 param_1,i16 param_2)

{
  i16 *piVar1;
  i16 iVar2;
  u16 uVar3;
  i16 iStack6;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((iVar2 + 0x56) == 0x0) {
    return 0x0;
  }
  iStack6 = 0x0;
  while( true ) {
    piVar1 = (i16 *)(iVar2 + 0x56);
    if (*piVar1 == iStack6 || *piVar1 < iStack6) {
      return 0x0;
    }
    if ((iVar2 + 0x4e + iStack6 * 0x2) == param_2) break;
    iStack6 += 0x1;
  }
  return 0x1;
}



StructD * pass1_1018_1f6a(u16 param_1,StructD *param_2,u8 param_3)

{
  param_2 = (StructD *)((u32)param_2 & 0xffff0000 | (u32)((int)param_2 - 0x20));
  pass1_1018_1a04(param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



void FUN_1018_1f76(void)

{
  return;
}



u32 pass1_1018_1f7a(i16 param_1,u16 param_2)

{
  return CONCAT22(param_2,param_1 + 0x2a);
}



u16 * pass1_1018_1f8a(StructD *param_1,u8 param_2)

{
  pass1_1018_1a04(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return &param_1->address_offset_field_0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 pass1_1018_1ff4(astruct_19 *param_1,u16 param_2)

{
  i16 *piVar1;
  u32 in_EDX;
  u16 uVar2;
  astruct_19 *paVar3;
  u32 *puVar4;
  u16 in_stack_0000fe82;
  u16 in_stack_0000ffa6;
  u16 in_stack_0000ffac;
  u16 in_stack_0000ffb0;
  i16 *piVar5;
  u16 uVar6;
  i16 *piVar7;
  u16 uVar8;
  i16 local_a;
  i16 local_8;
  i16 iStack6;
  u16 uStack4;

  uVar2 = ((u32)in_EDX >> 0x10);
  paVar3 = struct_op_1010_1d48(param_1,param_2);
  (u32)((int)param_1 + 0xa) = 0xb9010b;
  (u32)((int)param_1 + 0xe) = 0x170035;
  param_1->offset_0x0 = 0x21e8;
  ((int)param_1 + 0x2) = 0x1018;
  piVar7 = &local_8;
  uVar8 = SUB42(&DAT_1050_1050,0x0);
  piVar5 = &local_a;
  uVar6 = SUB42(&DAT_1050_1050,0x0);
  puVar4 = mixed_1010_20ba((astruct_57 *)CONCAT22(uVar2,(int)((u32)paVar3 >> 0x10)),_u16_1050_0ed0,
                           (u8 **)CONCAT22(piVar5,0x48),in_stack_0000fe82,in_stack_0000ffa6,in_stack_0000ffac,
                           in_stack_0000ffb0);
  uStack4 = ((u32)puVar4 >> 0x10);
  iStack6 = (int)puVar4;
  pass1_1008_3e94((u16 *)((u32)puVar4 & 0xffff0000 | (u32)(iStack6 + 0xe)),(u16 *)CONCAT22(uVar6,piVar5),
                  (char *)CONCAT22(uVar8,piVar7));
  piVar1 = (i16 *)((int)param_1 + 0xa);
  *piVar1 = *piVar1 + local_8;
  piVar1 = (i16 *)((int)param_1 + 0xc);
  *piVar1 = *piVar1 + local_a;
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x12)),NULL,0x7f4);
  return (u32)param_1;
}



void pass1_1018_2076(u16 *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  *param_1 = 0x21e8;
  ((int)param_1 + 0x2) = 0x1018;
  pass1_1018_209c((u32)param_1 & 0xffff | (u32)uVar1 << 0x10);
  pass1_1010_1d80((StructD *)param_1);
  return;
}



void pass1_1018_209c(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;
  i16 iStack4;

  iStack4 = 0x0;
  do {
    uVar5 = (param_1 >> 0x10);
    iVar4 = (int)param_1 + 0x12;
    puVar1 = (u32 *)(iVar4 + iStack4 * 0x4);
    uVar2 = (iVar4 + iStack4 * 0x4 + 0x2);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    (u32)((int)param_1 + iStack4 * 0x4 + 0x12) = 0x0;
    iStack4 += 0x1;
  } while (iStack4 < 0x1fd);
  return;
}



void pass1_1018_20ee(u32 param_1,i16 *param_2)

{
  BOOL16 BVar1;
  u16 in_DX;
  u16 uVar2;

  BVar1 = pass1_1008_aed8((u32)param_2);
  if (BVar1 == 0x0) {
    return;
  }
  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + *param_2 * 0x4 + 0x12) == 0x0) {
    pass1_1018_216e(in_DX,param_1 & 0xffff | (u32)uVar2 << 0x10,(u32)param_2);
  }
  pass1_1008_ae26(param_2);
  return;
}



void pass1_1018_214e(u16 param_1,u16 param_2,u16 *param_3,i16 param_4)

{
  pass1_1008_3e76(param_3,0x0,(param_4 * 0x4 + 0x3e90),(param_4 * 0x4 + 0x3e8e));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_216e(u16 param_1,u32 param_2,u32 param_3)

{
  u16 uVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uStack8;

  uStack8 = pass1_1008_adf2(param_3);
  uVar1 = pass1_1008_ae0c(param_3);
  for (; (int)uStack8 <= (int)uVar1; uStack8 += 0x1) {
    uVar2 = uVar1;
    pass1_1010_8018((u8 *)param_1,_u16_1050_14cc,uStack8);
    uVar3 = (param_2 >> 0x10);
    ((int)param_2 + uStack8 * 0x4 + 0x12) = uVar2;
    *(u8 **)((int)param_2 + uStack8 * 0x4 + 0x14) = (u8 *)param_1;
  }
  return;
}



u32 pass1_1018_21c2(u32 param_1,u8 param_2)

{
  pass1_1018_2076((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 pass1_1018_21f8(void)

{
  u16 *puVar1;

  pass1_1008_3e54(&u16_1048_4210,0x0,0x195,0x1);
  pass1_1008_3e54(&u16_1050_65ca,0x0,0xe0,0x1b1);
  pass1_1008_3e54(&u16_1050_65d0,0x0,0x17a,0x72);
  pass1_1008_3e54(&u16_1050_65d6,0x0,0xde,0x93);
  pass1_1008_3e54(&u16_1050_65dc,0x0,0x177,0x1da);
  pass1_1008_3e54(&u16_1048_4216,0x0,0x195,0x21c);
  pass1_1008_3e54((u16 *)&u32_1048_421c,0x0,0x1b6,0x22c);
  pass1_1008_3e54((u16 *)((long)&u32_1048_4220 + 0x2),0x0,0x109,0x5);
  puVar1 = pass1_1008_3e54((u16 *)&u32_1048_4228,0x0,0xfd,0x1fd);
  return puVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void struct_1018_229c(u8 *param_1,astruct_19 *param_2,u16 param_3)

{
  u16 uVar1;
  u16 *puVar2;
  astruct_57 *paVar3;
  u16 in_register_0000000a;
  u32 uVar4;
  astruct_57 *paVar5;
  u16 unaff_CS;
  i16 iVar6;

  uVar4 = CONCAT22(in_register_0000000a,param_1);
  struct_op_1018_4cda(param_2,param_3);
  ((int)param_2 + 0x1c) = 0x389a;
  ((int)param_2 + 0x1e) = 0x1008;
  ((int)param_2 + 0x1c) = 0x3aa8;
  ((int)param_2 + 0x1e) = 0x1008;
  uVar1 = 0x0;
  (u32)((int)param_2 + 0x20) = 0x0;
  ((int)param_2 + 0x24) = 0x0;
  (u32)((int)param_2 + 0x26) = 0x0;
  (u32)((int)param_2 + 0x2a) = 0x0;
  ((int)param_2 + 0x3e) = 0x0;
  ((int)param_2 + 0x40) = 0x0;
  ((int)param_2 + 0x42) = 0x0;
  ((int)param_2 + 0x44) = 0x0;
  (u32)((int)param_2 + 0x6e) = 0x0;
  param_2->offset_0x0 = 0x2ada;
  ((int)param_2 + 0x2) = 0x1018;
  ((int)param_2 + 0x1c) = (int)s_fem132_wav_1050_2aec + 0x6;
  ((int)param_2 + 0x1e) = 0x1018;
  _PTR_LOOP_1050_4230 = param_2;
  pass1_1018_4dce((u8 *)uVar4,param_2,0x105);
  uVar1 = FUN_1010_830a(uVar1,uVar4,unaff_CS,_u16_1050_14cc,0x1a8);
  ((int)param_2 + 0x2a) = uVar1;
  ((int)param_2 + 0x2c) = (int)uVar4;
  pass1_1000_4906((StructD *)((u32)param_2 & 0xffff0000 | ZEXT24((u16 *)((int)param_2 + 0x2eU))),NULL,0x10);
  puVar2 = pass1_1000_4906((StructD *)((u32)param_2 & 0xffff0000 | (u32)((int)param_2 + 0x46)),NULL,0x28);
  uVar1 = FUN_1010_830a(puVar2,uVar4,0x1000,_u16_1050_14cc,0x6c);
  ((int)param_2 + 0x2eU) = uVar1;
  ((int)param_2 + 0x30) = (int)uVar4;
  uVar1 = FUN_1010_830a(uVar1,uVar4,0x1010,_u16_1050_14cc,0x68);
  ((int)param_2 + 0x32) = uVar1;
  ((int)param_2 + 0x34) = (int)uVar4;
  uVar1 = FUN_1010_830a(uVar1,uVar4,0x1010,_u16_1050_14cc,0x66);
  ((int)param_2 + 0x36) = uVar1;
  ((int)param_2 + 0x38) = (int)uVar4;
  uVar1 = FUN_1010_830a(uVar1,uVar4,0x1010,_u16_1050_14cc,0x6a);
  ((int)param_2 + 0x3a) = uVar1;
  ((int)param_2 + 0x3c) = (int)uVar4;
  uVar1 = FUN_1010_830a(uVar1,uVar4,0x1010,_u16_1050_14cc,0x1cd);
  ((int)param_2 + 0x6e) = uVar1;
  ((int)param_2 + 0x70) = (int)uVar4;
  iVar6 = 0x0;
  do {
    uVar1 = FUN_1010_830a(iVar6 + 0x8f,uVar4,0x1010,_u16_1050_14cc,iVar6 + 0x8f);
    ((int)param_2 + iVar6 * 0x4 + 0x46) = uVar1;
    ((int)param_2 + iVar6 * 0x4 + 0x48) = (int)uVar4;
    iVar6 += 0x1;
  } while (iVar6 < 0xa);
  if (param_2 == NULL) {
    paVar3 = NULL;
    paVar5 = (astruct_57 *)(uVar4 & 0xffff0000);
  }
  else {
    paVar5 = (astruct_57 *)(uVar4 & 0xffff0000 | (u32)param_2);
    paVar3 = (astruct_57 *)((int)param_2 + 0x1c);
  }
  pass1_1008_9262(paVar3,paVar5,(int)_PTR_LOOP_1050_0388,((u32)_PTR_LOOP_1050_0388 >> 0x10),0x73,
                  CONCAT22((int)paVar5,paVar3));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_2440(StructD *param_1)

{
  u32 *puVar2;
  u16 uVar3;
  code **ppcVar4;
  u8 *puVar5;
  u8 *puVar4;
  u16 uVar6;
  StructD *uVar5;
  u16 uVar7;
  u16 unaff_CS;
  u16 *puStack6;
  u16 uVar2;
  u32 *puVar1;

  uVar7 = ((u32)param_1 >> 0x10);
  uVar5 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x2ada;
  uVar5->address_offset_field_0x2 = 0x1018;
  &uVar5->field_0x1c = (int)s_fem132_wav_1050_2aec + 0x6;
  &uVar5->field_0x1e = 0x1018;
  if (_PTR_LOOP_1050_0388 != 0x0) {
    if (param_1 == NULL) {
      puVar5 = NULL;
      uVar6 = 0x0;
    }
    else {
      puVar5 = &uVar5->field_0x1c;
      uVar6 = uVar7;
    }
    unaff_CS = 0x1008;
    pass1_1008_92b2(_PTR_LOOP_1050_0388,0x73,CONCAT22(uVar6,puVar5));
  }
  puVar1 = (u32*)&uVar5->field_0x2a;
  uVar2 = &uVar5->field29_0x2c;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar4 = (code **)*puVar1;
    (**ppcVar4)(unaff_CS,puVar1,uVar2,0x1);
  }
  puVar2 = (u32*)&uVar5->field_0x6e;
  uVar3 = &uVar5->field_0x70;
  if ((uVar3 | puVar2) != 0x0) {
    ppcVar4 = (code **)*puVar2;
    (**ppcVar4)(unaff_CS,puVar2,uVar3,0x1);
  }
  if (param_1 == NULL) {
    puVar4 = NULL;
    uVar7 = 0x0;
  }
  else {
    puVar4 = &uVar5->field_0x1c;
  }
  puStack6 = (u16 *)CONCAT22(uVar7,puVar4);
  *puStack6 = 0x389a;
  (puVar4 + 0x2) = 0x1008;
  clenaup_win_ui_1018_4d22(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_2504(u16 param_1,u16 param_2)

{
  u16 uVar1;

  pass1_1030_8344((u32)_u16_1050_5748,0x4000001);
  if ((param_2 | param_1) != 0x0) {
    uVar1 = pass1_1028_d69e(**_u16_1050_5748);
    if (uVar1 == 0x0) {
      return;
    }
  }
  return;
}



void pass1_1018_2548(u16 param_1,u16 param_2,u16 *param_3)

{
  pass1_1008_3f62(param_3,(u16 *)&u32_1048_4228);
  return;
}



u16 pass1_1018_255e(u32 param_1)

{
  u32 uVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x26) != 0x0) {
    uVar1 = (u32)((int)param_1 + 0x26);
    return ((int)uVar1 + 0xa);
  }
  return 0x0;
}



u8 * pass1_1018_2580(undefined1 param_1,u32 param_2,u16 param_3,u32 param_4,u16 param_5)

{
  u8 *puVar1;
  i16 iVar2;
  u16 uVar3;
  uchar local_8 [0x6];

  uVar3 = (param_2 >> 0x10);
  iVar2 = (int)param_2;
  if (*(i32 *)(iVar2 + 0x20) == 0x0) {
    return (u8 *)0x6ad;
  }
  pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_8));
  pass1_1018_161c((u32)(iVar2 + 0x20),(u16 *)CONCAT22(0x1050,local_8),(int)param_4,(int)(param_4 >> 0x10));
  puVar1 = local_8;
  pass1_1018_17ce((u32)(iVar2 + 0x20),CONCAT22(puVar1,param_3),CONCAT22(param_5,0x1050));
  return puVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1018_25d2(u32 param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;
  u32 in_EDX;
  u16 uVar3;
  astruct_57 *paVar2;
  u16 uVar4;
  u16 *puVar5;
  u32 *puVar6;
  u16 in_stack_0000fe96;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc4;
  u16 *puVar7;
  u8 local_8 [0x6];

  uVar3 = ((u32)in_EDX >> 0x10);
  uVar4 = (param_1 >> 0x10);
  if (*(i32 *)((int)param_1 + 0x20) == 0x0) {
    return 0x0;
  }
  puVar5 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_8));
  paVar2 = (astruct_57 *)CONCAT22(uVar3,(int)((u32)puVar5 >> 0x10));
  pass1_1018_161c((u32)((int)param_1 + 0x20),(u16 *)CONCAT22(0x1050,local_8),(int)param_3,(int)(param_3 >> 0x10)
                 );
  puVar7 = (u16 *)CONCAT22(0x1050,local_8);
  puVar6 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x32),in_stack_0000fe96,
                           in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
  uVar1 = puVar6;
  pass1_1010_71d6(uVar1,((u32)puVar6 >> 0x10),(u32)puVar6,param_2,puVar7,&DAT_1050_1050);
  return uVar1;
}



void pass1_1018_262e(u32 param_1)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  ((int)param_1 + 0x44) = 0x1;
  (u32)((int)param_1 + 0x3e) = 0x0;
  return;
}



void pass1_1018_2646(u16 param_1,u16 param_2,u16 *param_3)

{
  pass1_1008_3f62(param_3,(u16 *)((long)&u32_1048_4220 + 0x2));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 pass1_1018_265c(void)

{
  u32 uVar1;

  uVar1 = pass1_1030_8326();
  return uVar1;
}



u16 pass1_1018_266a(u32 param_1)

{
  return ((int)param_1 + 0x44);
}



void pass1_1018_2678(u16 param_1,u16 param_2,u16 *param_3)

{
  pass1_1008_3f62(param_3,&u16_1048_4216);
  return;
}



u32 pass1_1018_268e(astruct_287 *param_1)

{
  astruct_287 *iVar1;
  i16 iVar2;
  u16 uVar3;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_287 *)param_1;
  if (iVar1->field65_0x42 != 0x0) {
    (u32)&iVar1->field_0x40 = 0x0;
    iVar1->field66_0x44 = 0x1;
  }
  iVar2 = iVar1->field62_0x3e * 0x4;
  return CONCAT22((&iVar1[0x1].field_0x2 + iVar2),(&iVar1[0x1].field_0x0 + iVar2));
}



void pass1_1018_26c2(u16 param_1,u16 param_2,u16 *param_3)

{
  pass1_1008_3f62(param_3,(u16 *)&u32_1048_421c);
  return;
}



void pass1_1018_26d8(u16 param_1,u16 param_2,i16 param_3,u16 *param_4)

{
  pass1_1008_3f62(param_4,(u16 *)CONCAT22(0x1050,(int)&u16_1050_65ca + param_3 * 0x6));
  return;
}



void pass1_1018_26f8(u16 param_1,u16 param_2,u16 *param_3)

{
  pass1_1008_3f62(param_3,&u16_1048_4210);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_270e(u8 *param_1,astruct_27 *param_2,i16 param_3,u16 param_4)

{
  code **ppcVar1;
  u32 uVar2;
  u8 *puVar3;
  u16 uVar4;
  u16 in_register_0000000a;
  astruct_57 *paVar5;
  astruct_27 *iVar5;
  u16 uVar6;
  u32 *puVar7;
  u16 in_stack_0000fe9c;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000fff4;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  iVar5 = (astruct_27 *)param_2;
  uVar6 = ((u32)param_2 >> 0x10);
  if (param_3 == 0x0) {
    if ((*(i32 *)&iVar5->field_0x20 == 0x0) ||
       (uVar2 = (u32)&iVar5->field_0x20, ((int)uVar2 + 0x8) != param_4)) {
      puVar7 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff4,param_4),in_stack_0000fe9c
                               ,in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
      if (*(i32 *)&iVar5->field_0x20 != 0x0) {
        if (param_2 == NULL) {
          puVar3 = NULL;
          uVar4 = 0x0;
        }
        else {
          puVar3 = &iVar5->field_0x1c;
          uVar4 = uVar6;
        }
        pass1_1010_1ea6((u32)&iVar5->field_0x20,(StructD *)CONCAT22(uVar4,puVar3));
      }
      (u32*)&iVar5->field_0x20 = puVar7;
      if (param_2 == NULL) {
        param_4 = 0x0;
        uVar4 = 0x0;
      }
      else {
        param_4 = &iVar5->field_0x1c;
        uVar4 = uVar6;
      }
      paVar5 = (astruct_57 *)(u32)uVar4;
      uVar2 = (u32)&iVar5->field_0x20;
      ppcVar1 = (code **)((int)*(u32*)&iVar5->field_0x20 + 0x4);
      (**ppcVar1)(0x1010,(int)uVar2,(int)((u32)uVar2 >> 0x10),0x0,param_4,uVar4);
    }
    uVar4 = paVar5;
    pass1_1018_2862((astruct_654 *)param_2);
    if ((uVar4 | param_4) != 0x0) {
      ((int)&iVar5->field30_0x22 + 0x2) = 0x1;
    }
    pass1_1010_1f62(param_2,0x7);
  }
  else if ((&iVar5->field30_0x22 | &iVar5->field_0x20) != 0x0) {
    if (param_2 == NULL) {
      puVar3 = NULL;
      uVar4 = 0x0;
    }
    else {
      puVar3 = &iVar5->field_0x1c;
      uVar4 = uVar6;
    }
    pass1_1010_1ea6((u32)&iVar5->field_0x20,(StructD *)CONCAT22(uVar4,puVar3));
    (u32)&iVar5->field_0x20 = 0x0;
    return;
  }
  return;
}



void pass1_1018_280c(u32 param_1)

{
  u32 uVar1;
  i16 iVar2;
  u16 uVar3;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if ((iVar2 + 0x24) == 0x0) {
    return;
  }
  (iVar2 + 0x24) = 0x0;
  if (*(i32 *)(iVar2 + 0x20) == 0x0) {
    (u32)(iVar2 + 0x26) = 0x0;
  }
  else {
    uVar1 = (u32)(iVar2 + 0x20);
    (u32)(iVar2 + 0x26) = (u32)((int)uVar1 + 0x4c);
  }
  return;
}



void pass1_1018_2862(astruct_654 *param_1)

{
  i32 lVar1;
  astruct_654 *iVar2;
  u16 uVar2;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar2 = (astruct_654 *)param_1;
  if (iVar2->field32_0x20 == 0x0) {
    iVar2->field35_0x26 = 0x0;
  }
  else {
    lVar1 = iVar2->field32_0x20;
    iVar2->field35_0x26 = (u32)((int)lVar1 + 0x4c);
  }
  return;
}



void pass1_1018_289c(u16 param_1,u32 param_2,i16 param_3)

{
  u16 uVar1;

  if (param_3 == 0x1) {
    (u32)((int)param_2 + 0x4) = 0x0;
    return;
  }
  if (param_3 == 0x2) {
    pass1_1018_2922(param_2 & 0xffff0000 | (u32)((int)param_2 - 0x1c));
  }
  else {
    if ((((param_3 + -0x3 < 0x1) || (SBORROW2(param_3 + -0x3,0x1))) || (0x1 < param_3 + -0x5)) ||
       (*(i32 *)((int)param_2 + 0x4) == 0x0)) {
      return;
    }
    uVar1 = (int)param_2 - 0x1c;
    pass1_1018_2862((astruct_654 *)(param_2 & 0xffff0000 | (u32)uVar1));
    if ((param_1 | uVar1) != 0x0) {
      ((int)param_2 + 0x8) = 0x1;
    }
  }
  pass1_1010_1f62((astruct_27 *)(param_2 & 0xffff0000 | (u32)((int)param_2 - 0x1c)),param_3);
  return;
}



void pass1_1018_2922(u32 param_1)

{
  i16 *piVar1;
  i16 iVar2;
  u16 uVar3;

  uVar3 = (param_1 >> 0x10);
  iVar2 = (int)param_1;
  if (((iVar2 + 0x40) != 0x0) &&
     (piVar1 = (i16 *)(iVar2 + 0x3e), *piVar1 = *piVar1 + 0x1, 0x9 < (iVar2 + 0x3e))) {
    (iVar2 + 0x3e) = 0x0;
    (iVar2 + 0x42) = 0x1;
  }
  return;
}



void win_op_1018_294a(u16 param_1,u16 in_string_6,astruct_8 *param_3,u16 param_4,u32 param_5)

{
  if ((((int)param_3 + 0x18) != 0x0) && (param_5 == 0x280)) {
    ((int)param_3 + 0x18) = 0x0;
  }
  create_dc_1018_4e04(param_1,in_string_6,param_3,param_4,(int)param_5,param_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void mixed_sys_op_1018_2978(u16 param_1,u16 param_2,astruct_931 *param_3)

{
  code **ppcVar1;
  u8 *puVar2;
  astruct_394 *paVar2;
  RECT16 *rect;
  i16 iVar4;
  u16 uVar5;
  u16 uVar3;
  u16 uVar4;
  u16 uVar6;
  u16 in_register_0000000a;
  astruct_57 *paVar5;
  astruct_57 *paVar7;
  astruct_931 *iVar9;
  u16 uVar8;
  u16 uVar7;
  u16 uVar9;
  HWND16 HVar8;
  astruct_394 *paVar9;
  RECT16 local_3a;
  i16 iStack54;
  i16 iStack52;
  u32 uStack50;
  u32 *puStack46;
  astruct_394 local_2a;
  u16 uStack6;
  u16 uStack4;
  u8 uVar10;
  astruct_57 *paVar6;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  pass1_1010_8096(_u16_1050_14cc,0x1);
  uStack4 = SUB42(paVar5,0x0);
  puVar2 = (u8 *)&local_2a;
  uStack6 = param_1;
  struct_op_1008_48fe(paVar5,(astruct_81 *)CONCAT13(0x10,CONCAT12(0x50,puVar2)),0x1,(char *)CONCAT22(uStack4,param_1));
  uVar7 = 0x1000;
  mem_op_1000_179c(0x1e,paVar5);
  uVar5 = paVar5 | puVar2;
  paVar7 = (astruct_57 *)((u32)paVar5 & 0xffff0000);
  paVar6 = (astruct_57 *)((u32)paVar7 | (u32)uVar5);
  if (uVar5 == 0x0) {
    paVar2 = NULL;
  }
  else {
    paVar2 = &local_2a;
    uVar7 = 0x1008;
    struct_op_1008_3f92((astruct_76 *)CONCAT22(paVar5,puVar2),(char *)CONCAT22(0x1050,paVar2));
    paVar7 = paVar6;
  }
  uVar3 = SUB42(paVar7,0x0);
  puStack46 = (u32 *)CONCAT22(uVar3,paVar2);
  ppcVar1 = (code **)((int)*puStack46 + 0x14);
  paVar9 = paVar2;
  (**ppcVar1)(uVar7,paVar2,uVar3);
  uStack50 = CONCAT22((int)paVar7,paVar2);
  mem_op_1000_179c(0x14,paVar7);
  uVar4 = paVar7 | paVar2;
  paVar7 = (astruct_57 *)((u32)paVar7 & 0xffff0000);
  paVar5 = (astruct_57 *)((u32)paVar7 | (u32)uVar4);
  if (uVar4 == 0x0) {
    paVar2 = NULL;
  }
  else {
    struct_1008_4c58(paVar2);
    paVar7 = paVar5;
  }
  uVar8 = ((u32)param_3 >> 0x10);
  iVar9 = (astruct_931 *)param_3;
  *(astruct_394 **)&iVar9->field12_0xe = paVar2;
  *(u8 **)((int)&iVar9->field12_0xe + 0x2) = (u8 *)paVar7;
  pass1_1008_4d84((u8 *)paVar7,iVar9->field12_0xe,uStack50);
  rect = &local_3a;
  HVar8 = HWND16_1050_0396;
  GetClientRect16(rect,(HWND16)&DAT_1050_1050);
  uVar9 = 0x1000;
  mem_op_1000_179c(0x1e,paVar7);
  uVar6 = paVar7 | rect;
  if (uVar6 == 0x0) {
    (u32)&iVar9->field10_0xa = 0x0;
  }
  else {
    iVar4 = (iStack52 - local_3a.y) + 0x1;
    uVar9 = 0x1008;
    pass1_1008_405c((astruct_76 *)CONCAT22(paVar7,rect),(u32)iVar9->field12_0xe,iVar4,
                    (iStack54 - local_3a.x) + 0x1);
    iVar9->field10_0xa = iVar4;
    iVar9->field11_0xc = uVar6;
  }
  if (puStack46 != NULL) {
    ppcVar1 = (code **)*puStack46;
    (**ppcVar1)(uVar9,(int)puStack46,(int)((u32)puStack46 >> 0x10),0x1,HVar8,paVar9,uVar3,puStack46,puStack46);
  }
  close_file_1008_496c((astruct_803 *)CONCAT13(0x10,CONCAT12(0x50,&local_2a)));
  return;
}



void pass1_1018_2aa3(void)

{
  pass1_1018_21f8();
  return;
}



StructD * pass1_1018_2aa8(StructD *param_1,u8 param_2)

{
  param_1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0x1c));
  pass1_1018_2440(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void pass1_1018_2afa(u32 *param_1)

{
  fn_ptr_1000_17ce((char *)*param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void struct_1018_2b10(astruct_19 *param_1,u16 param_2)

{
  u32 uVar1;
  code **ppcVar2;
  u16 *puVar3;
  u16 uVar4;
  u32 in_EDX;
  u16 uVar7;
  astruct_57 *paVar5;
  u32 uVar6;
  u16 *puVar8;
  u32 *puVar9;
  u32 uVar10;
  u16 in_stack_0000fe9a;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffc8;
  u16 uVar12;
  u32 in_stack_0000fff0;
  u16 uVar14;
  u8 **ppuVar13;
  astruct_19 *uVar11;
  astruct_19 *uVar9;

  uVar7 = ((u32)in_EDX >> 0x10);
  uVar14 = ((u32)in_stack_0000fff0 >> 0x10);
  uVar9 = (astruct_19 *)param_1;
  uVar12 = ((u32)param_1 >> 0x10);
  puVar8 = get_sys_metrics_1018_4b1e(param_1,0x1,param_2);
  paVar5 = (astruct_57 *)CONCAT22(uVar7,(int)((u32)puVar8 >> 0x10));
  uVar9->field17_0x20 = 0x389a;
  uVar9->field18_0x22 = 0x1008;
  uVar9->field17_0x20 = 0x3aa8;
  uVar9->field18_0x22 = 0x1008;
  (u32)&uVar9->field19_0x24 = 0x0;
  &uVar9[0x3].field_0x54 = 0x0;
  uVar9[0x3].field47_0x56 = 0x0;
  &uVar9[0x3].field_0x58 = 0x0;
  (u32)&uVar9[0x3].field_0x5a = 0x0;
  (u32)&uVar9[0x3].field53_0x5e = 0x0;
  (u32)&uVar9[0x4].segment_0x2 = 0x0;
  (u32)((int)&uVar9[0x4].field2_0x4 + 0x2) = 0x0;
  param_1->offset_0x0 = 0x32d8;
  uVar9->segment_0x2 = 0x1018;
  uVar9->field17_0x20 = 0x3314;
  uVar9->field18_0x22 = 0x1018;
  ppuVar13 = (u8 **)CONCAT22(uVar14,0x2f);
  puVar9 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,ppuVar13,in_stack_0000fe9a,in_stack_0000ffbe,in_stack_0000ffc4,
                           in_stack_0000ffc8);
  uVar6 = (u32)paVar5 & 0xffff0000;
  uVar9[0x4].segment_0x2 = puVar9;
  &uVar9[0x4].field2_0x4 = (int)((u32)puVar9 >> 0x10);
  if (param_1 == NULL) {
    puVar3 = NULL;
  }
  else {
    uVar6 |= uVar12;
    puVar3 = &uVar9->field17_0x20;
  }
  uVar1 = (u32)&uVar9[0x4].segment_0x2;
  ppcVar2 = (code **)((int)*(u32*)&uVar9[0x4].segment_0x2 + 0x4);
  (**ppcVar2)(0x1010,(int)uVar1,(int)((u32)uVar1 >> 0x10),0x0,puVar3,(int)uVar6,(int)((u32)ppuVar13 >> 0x10));
  uVar1 = (u32)&uVar9[0x4].segment_0x2;
  uVar1 = (u32)((int)uVar1 + 0x24);
  (u32)&uVar9[0x3].field_0x5a = uVar1;
  uVar4 = FUN_1010_830a((int)uVar1,uVar6,0x1010,_u16_1050_14cc,0x6e);
  uVar9->field19_0x24 = uVar4;
  &uVar9->field20_0x26 = (int)uVar6;
  (u32)((int)&uVar9->field20_0x26 + 0x2) = 0x0;
  uVar10 = pass1_1008_4772(*(astruct_76 **)&uVar9->field19_0x24);
  uVar14 = (uVar10 >> 0x10);
  pass1_1018_4b78(param_1);
  &uVar9[0x3].field_0x4c = 0x1;
  &uVar9[0x3].field_0x4e = 0x1;
  &uVar9[0x3].field_0x50 = ((int)uVar10 + 0x4) + &uVar9[0x3].field_0x4c;
  uVar9[0x3].field44_0x52 = ((int)uVar10 + 0x8) - 0x19;
  return;
}



void pass1_1018_2c60(StructD *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  u16 *puVar4;
  u16 uVar5;
  StructD *uVar6;
  u16 uVar7;
  u16 *puStack6;

  uVar7 = ((u32)param_1 >> 0x10);
  uVar6 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x32d8;
  uVar6->address_offset_field_0x2 = 0x1018;
  uVar6->field19_0x20 = 0x3314;
  uVar6->field20_0x22 = 0x1018;
  if (*(i32 *)&uVar6[0x1].field_0x94 != 0x0) {
    if (param_1 == NULL) {
      puVar4 = NULL;
      uVar5 = 0x0;
    }
    else {
      puVar4 = &uVar6->field19_0x20;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6((u32)&uVar6[0x1].field_0x94,(StructD *)CONCAT22(uVar5,puVar4));
  }
  fn_ptr_1000_17ce(*(char **)&uVar6[0x1].field_0x98);
  puVar1 = (u32*)&uVar6->field_0x24;
  uVar2 = &uVar6->field_0x26;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1000,puVar1,uVar2,0x1);
  }
  if (param_1 == NULL) {
    puVar4 = NULL;
    uVar7 = 0x0;
  }
  else {
    puVar4 = &uVar6->field19_0x20;
  }
  puStack6 = (u16 *)CONCAT22(uVar7,puVar4);
  *puStack6 = 0x389a;
  puVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1);
  return;
}



void pass1_1018_2d22(u32 param_1,i16 *param_2,u16 *param_3,i16 param_4)

{
  u32 uVar1;

  *param_3 = 0x0;
  *param_2 = 0x0;
  uVar1 = pass1_1008_4772(*(astruct_76 **)((int)param_1 + 0x24));
  *param_2 = ((int)uVar1 + 0x8) + -0x14;
  if (param_4 == 0xbb8) {
    *param_3 = 0x5;
  }
  if (param_4 == 0xbba) {
    *param_3 = 0x23;
  }
  if (param_4 == 0xbb9) {
    *param_3 = 0x75;
  }
  return;
}



void pass1_1018_2d84(u16 param_1,astruct_126 *param_2)

{
  pass1_1018_2e28(param_2);
  pass1_1020_bd80(param_1);
  return;
}



void pass1_1018_2d9a(astruct_126 *param_1)

{
  i16 *piVar1;
  u32 uVar2;
  u16 uVar3;
  astruct_126 *iVar4;
  u16 uVar4;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_126 *)param_1;
  uVar3 = iVar4->field375_0x180 | iVar4->field374_0x17e;
  if (uVar3 != 0x0) {
    piVar1 = &iVar4->field369_0x174;
    *piVar1 = *piVar1 + -0x1;
    if (*piVar1 < 0x0) {
      uVar2 = (u32)&iVar4->field374_0x17e;
      uVar3 = ((int)uVar2 + 0xa) - 0x1;
      iVar4->field369_0x174 = uVar3;
    }
    pass1_1018_2e28(param_1);
    iVar4->field370_0x176 = uVar3;
  }
  return;
}



void pass1_1018_2dde(astruct_126 *param_1)

{
  i16 *piVar1;
  u32 uVar2;
  i16 iVar3;
  astruct_126 *iVar4;
  u16 uVar4;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar4 = (astruct_126 *)param_1;
  if ((iVar4->field375_0x180 | iVar4->field374_0x17e) != 0x0) {
    piVar1 = &iVar4->field369_0x174;
    *piVar1 = *piVar1 + 0x1;
    iVar3 = iVar4->field369_0x174;
    uVar2 = (u32)&iVar4->field374_0x17e;
    piVar1 = (i16 *)((int)uVar2 + 0xa);
    if (*piVar1 == iVar3 || *piVar1 < iVar3) {
      iVar4->field369_0x174 = 0x0;
    }
    pass1_1018_2e28(param_1);
    iVar4->field370_0x176 = iVar3;
  }
  return;
}



void pass1_1018_2e28(astruct_126 *param_1)

{
  u16 uVar1;
  u16 extraout_DX;

  uVar1 = ((int)param_1 + 0x174);
  empty_1008_8fc4();
  if ((extraout_DX | uVar1) != 0x0) {
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_2e5e(u16 param_1,u16 param_2,astruct_126 *param_3)

{
  u32 uVar3;
  u16 uVar1;
  i16 iVar5;
  astruct_126 *iVar4;
  astruct_126 *uVar2;

  uVar2 = (astruct_126 *)((u32)param_3 >> 0x10);
  iVar4 = (astruct_126 *)param_3;
  if (*(i32 *)&iVar4->field374_0x17e == 0x0) {
    pass1_1030_82f0(_u16_1050_5748,iVar4->field373_0x17a);
    iVar4->field374_0x17e = param_1;
    iVar4->field375_0x180 = param_2;
  }
  if ((*(i32 *)&iVar4->field374_0x17e != 0x0) &&
     (uVar3 = (u32)&iVar4->field374_0x17e, ((int)uVar3 + 0xa) != 0x0)) {
    iVar5 = iVar4->field369_0x174;
    empty_1008_8fc4();
    pass1_1018_2e28(param_3);
    iVar4->field370_0x176 = iVar5;
    return;
  }
  return;
}



void pass1_1018_2ee4(astruct_126 *param_1,u16 param_2)

{
  u32 uVar1;
  char cVar2;
  u16 uVar3;

  if (param_2 != 0x12) {
    if (param_2 < 0x13) {
      cVar2 = (char)param_2;
      if (cVar2 == '\x01') {
        (u32)((int)param_1 + 0x162) = 0x0;
        return;
      }
      if (('\x02' < (char)(cVar2 + -0x1)) && ((char)(cVar2 + -0x4) < '\x03')) goto LAB_1018_2f06;
    }
    return;
  }
  uVar1 = (u32)((int)param_1 + 0x162);
  (u32)((int)param_1 + 0x15a) = (u32)((int)uVar1 + 0x24);
LAB_1018_2f06:
  uVar3 = (int)param_1 - 0x20;
  pass1_1018_31fa(uVar3,param_1,(astruct_126 *)((u32)param_1 & 0xffff0000 | (u32)uVar3));
  pass1_1010_1f62((astruct_27 *)((u32)param_1 & 0xffff0000 | (u32)uVar3),param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void get_sys_metrics_1018_2f56(u32 param_1)

{
  INT16 IVar1;
  INT16 IVar2;
  astruct_57 *in_EDX;
  i16 iVar3;
  u16 uVar4;
  u32 *puVar5;
  u32 uVar6;
  u16 in_stack_0000fe7c;
  u16 in_stack_0000ffa0;
  u16 in_stack_0000ffa6;
  u16 in_stack_0000ffaa;
  i16 *piVar7;
  u16 uVar8;
  char *pcVar9;
  i16 local_6;
  i16 local_4;

  pcVar9 = (char *)CONCAT22(0x1050,&local_4);
  piVar7 = &local_6;
  uVar8 = SUB42(&DAT_1050_1050,0x0);
  puVar5 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(piVar7,0x48),in_stack_0000fe7c,in_stack_0000ffa0
                           ,in_stack_0000ffa6,in_stack_0000ffaa);
  pass1_1008_3e94((u16 *)((u32)puVar5 & 0xffff0000 | (u32)((int)puVar5 + 0xe)),(u16 *)CONCAT22(uVar8,piVar7),pcVar9)
  ;
  uVar4 = (param_1 >> 0x10);
  iVar3 = (int)param_1;
  uVar6 = pass1_1008_4772(*(astruct_76 **)(iVar3 + 0x24));
  uVar8 = (uVar6 >> 0x10);
  (iVar3 + 0x18) = local_4 + 0xb5;
  (iVar3 + 0x1a) = local_6 + 0x9;
  IVar1 = GetSystemMetrics16(SM_CXBORDER);
  (iVar3 + 0x1c) = IVar1 * 0x2 + ((int)uVar6 + 0x4);
  IVar1 = GetSystemMetrics16(SM_CYCAPTION);
  IVar2 = GetSystemMetrics16(SM_CYBORDER);
  (iVar3 + 0x1e) = IVar2 + IVar1 + ((int)uVar6 + 0x8);
  return;
}



void pass1_1018_2fe8(astruct_126 *param_1,u16_t param_2,u16_t param_3)

{
  i16 *piVar1;
  u32 uVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  i16 iVar6;
  i16 iVar7;
  u16 extraout_DX;
  u16 uVar8;
  astruct_126 *pstruct126_9;
  astruct_126 *uVar9;

  uVar9 = (astruct_126 *)((u32)param_1 >> 0x10);
  pstruct126_9 = (astruct_126 *)param_1;
  iVar6 = pstruct126_9->field369_0x174;
  uVar2 = (u32)&pstruct126_9->field374_0x17e;
  iVar7 = ((int)uVar2 + 0xa);
  if (iVar7 != 0x0) {
    if (pstruct126_9[0x1].field4_0x4 != NULL) {
      uVar3 = str_op_1000_3da4(pstruct126_9[0x1].field4_0x4);
      pstruct126_9->field369_0x174 = 0x0;
      while( true ) {
        if (iVar7 <= pstruct126_9->field369_0x174) break;
        uVar4 = pstruct126_9->field369_0x174;
        empty_1008_8fc4();
        uVar8 = extraout_DX;
        pass1_1018_2e28(param_1);
        uVar4 = pass1_1020_bd80(uVar4);
        uVar5 = pass1_1000_3de8((char *)CONCAT22(uVar8,uVar4),pstruct126_9[0x1].field4_0x4,uVar3,param_2,param_3);
        if (uVar5 == 0x0) break;
        piVar1 = &pstruct126_9->field369_0x174;
        *piVar1 = *piVar1 + 0x1;
      }
      if (pstruct126_9->field369_0x174 < iVar7) {
        pass1_1018_2e28(param_1);
        pstruct126_9->field370_0x176 = iVar7;
        return;
      }
      pstruct126_9->field369_0x174 = iVar6;
      pass1_1018_2e28(param_1);
      pstruct126_9->field370_0x176 = iVar6;
    }
  }
  return;
}



void pass1_1018_30ca(u16 param_1,astruct_504 *param_2,char *param_3)

{
  u16 uVar1;
  astruct_504 *iVar3;
  u16 uVar2;

  uVar2 = ((u32)param_2 >> 0x10);
  iVar3 = (astruct_504 *)param_2;
  fn_ptr_1000_17ce(*(char **)&iVar3->field390_0x186);
  uVar1 = str_op_1008_60e8(param_1,param_3);
  iVar3->field390_0x186 = uVar1;
  iVar3->field391_0x188 = param_1;
  return;
}



void pass1_1018_30fc(u16 param_1,u32 param_2,u16 **param_3)

{
  u16 uVar1;
  u32 uVar2;
  u16 *puVar3;
  u16 uVar4;
  u16 uVar5;
  i16 iVar6;
  u16 uVar7;
  u16 in_register_0000000a;
  astruct_57 *paVar8;
  u32 *puStack18;
  i16 iStack6;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  *param_3 = NULL;
  uVar2 = (u32)((int)param_2 + 0x17e);
  uVar1 = ((int)uVar2 + 0xa);
  if (uVar1 != 0x0) {
    uVar4 = uVar1;
    mem_op_1000_179c(0x6,paVar8);
    uVar7 = paVar8;
    puStack18 = (u32 *)CONCAT22(uVar7,uVar4);
    paVar8 = (astruct_57 *)((u32)paVar8 & 0xffff0000 | (u32)(uVar7 | uVar4));
    if ((uVar7 | uVar4) == 0x0) {
      *param_3 = NULL;
    }
    else {
      *puStack18 = 0x0;
      (uVar4 + 0x4) = 0x0;
      *param_3 = (u16 *)puStack18;
    }
    uVar5 = uVar1 * 0x2;
    mem_op_1000_179c(uVar5,paVar8);
    puVar3 = *param_3;
    *puVar3 = uVar5;
    ((int)puVar3 + 0x2) = (int)paVar8;
    ((int)*param_3 + 0x4) = uVar1;
    for (iStack6 = 0x0; iStack6 < (int)uVar1; iStack6 += 0x1) {
      iVar6 = iStack6;
      empty_1008_8fc4();
      ((int)(u32)*param_3 + iStack6 * 0x2) = (iVar6 + 0x2e);
    }
  }
  return;
}



u16 pass1_1018_31d0(astruct_126 *param_1)

{
  u32 uVar1;
  u16 uVar2;

  uVar2 = ((u32)param_1 >> 0x10);
  if ((*(i32 *)((int)param_1 + 0x17e) != 0x0) &&
     (uVar1 = (u32)((int)param_1 + 0x17e), *(i32 *)((int)uVar1 + 0xa) != 0x0)) {
    return 0x1;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_31fa(u16 param_1,u16 param_2,astruct_126 *param_3)

{
  i16 *piVar1;
  u32 uVar2;
  i16 iVar3;
  i16 iVar4;
  astruct_126 *pstruct126_5;
  u16 uVar5;

  uVar5 = ((u32)param_3 >> 0x10);
  pstruct126_5 = (astruct_126 *)param_3;
  pass1_1030_82f0(_u16_1050_5748,pstruct126_5->field373_0x17a);
  pstruct126_5->field374_0x17e = param_1;
  pstruct126_5->field375_0x180 = param_2;
  if (((param_2 | pstruct126_5->field374_0x17e) != 0x0) &&
     (uVar2 = (u32)&pstruct126_5->field374_0x17e, iVar4 = ((int)uVar2 + 0xa), iVar4 != 0x0)) {
    pstruct126_5->field369_0x174 = 0x0;
    while( true ) {
      if (iVar4 <= pstruct126_5->field369_0x174) break;
      iVar3 = pstruct126_5->field369_0x174;
      empty_1008_8fc4();
      pass1_1018_2e28(param_3);
      if (pstruct126_5->field370_0x176 == iVar3) break;
      piVar1 = &pstruct126_5->field369_0x174;
      *piVar1 = *piVar1 + 0x1;
    }
    if (iVar4 <= pstruct126_5->field369_0x174) {
      pstruct126_5->field369_0x174 = 0x0;
    }
    pass1_1018_2e28(param_3);
    pstruct126_5->field370_0x176 = iVar4;
  }
  return;
}



u16 * pass1_1018_32a6(u16 *param_1,u8 param_2)

{
  param_1 = (u16 *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0x20));
  pass1_1018_2c60((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * FUN_1018_32b2(u16 param_1,StructD *param_2,u8 param_3)

{
  pass1_1018_2c60(param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_331c(u8 *param_1,astruct_638 *param_2,u16 param_3,u16 param_4)

{
  u16 uVar1;
  u16 in_register_0000000a;
  astruct_57 *paVar2;
  u16 unaff_BP;
  u32 *puVar3;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  pass1_1008_ca5a((astruct_19 *)CONCAT22(param_3,param_2),param_4);
  (u32)&param_2->field271_0x122 = 0x0;
  param_2->field273_0x126 = 0x0;
  param_2->field274_0x12a = 0x0;
  param_2->field275_0x12e = 0x0;
  param_2->field276_0x130 = 0x0;
  param_2->field277_0x132 = 0x0;
  param_2->field278_0x136 = 0x0;
  param_2->field279_0x13a = 0x0;
  param_2->field280_0x13c = 0x0;
  param_2->field281_0x13e = 0x0;
  param_2->field282_0x142 = 0x0;
  CONCAT22(param_3,param_2) = (int)&PTR_LOOP_1050_470c;
  param_2->field2_0x2 = 0x1018;
  puVar3 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x3b),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  uVar1 = puVar3;
  param_2->field271_0x122 = uVar1;
  param_2->field272_0x124 = (int)((u32)puVar3 >> 0x10);
  param_2->field_0x22 = 0x0;
  pass1_1008_612e(uVar1,0x8,0xc);
  param_2->field280_0x13c = uVar1;
  return;
}



void pass1_1018_33b4(astruct_455 *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  astruct_455 *iVar5;
  u16 uVar4;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar5 = (astruct_455 *)param_1;
  param_1->field0_0x0 = &PTR_LOOP_1050_470c;
  iVar5->field1_0x2 = 0x1018;
  puVar1 = (u32 *)iVar5[0x26].field3_0x6;
  uVar2 = (iVar5 + 0x27)->field0_0x0;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (u32)&iVar5[0x26].field3_0x6 = 0x0;
  fn_ptr_1000_17ce(*(char **)&iVar5[0x24].field3_0x6);
  fn_ptr_1000_17ce(*(char **)&iVar5[0x25].field1_0x2);
  pass1_1008_caa0(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_3424(i16 param_1,u16 param_2,u32 param_3)

{
  u32 uVar1;
  u16 uVar2;
  i16 iVar3;
  u16 uVar4;
  u32 uStack10;
  u32 uStack6;

  uVar4 = (param_3 >> 0x10);
  iVar3 = (int)param_3;
  uVar1 = (u32)(iVar3 + 0x122);
  pass1_1008_e852(param_2,uVar1,((u32)uVar1 >> 0x10),*(char **)(iVar3 + 0x126));
  uStack6 = CONCAT22(param_2,param_1);
  uVar1 = (u32)(iVar3 + 0x122);
  pass1_1008_e852(param_2,uVar1,((u32)uVar1 >> 0x10),*(char **)(iVar3 + 0x12a));
  uStack10 = CONCAT22(param_2,param_1);
  pass1_1030_8344(_u16_1050_5748,uStack6);
  uVar2 = param_2;
  iVar3 = param_1;
  pass1_1030_8344(_u16_1050_5748,uStack10);
  if (*(i32 *)(iVar3 + 0x200) == *(i32 *)(param_1 + 0x200)) {
    return;
  }
  return;
}



void pass1_1018_34a6(astruct_679 *param_1)

{
  pass1_1018_3d6c(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void sprintf_op_1018_34b6(uchar param_1,u16 param_2,astruct_263 *param_3)

{
  i16 iVar1;
  u32 uVar2;
  char *pcVar3;
  u32 in_register_00000001;
  u32 uVar4;
  u16 uVar5;
  i16 iVar6;
  short in_buf_len_5;
  u32 uVar7;
  i32 lVar8;
  u16 uVar9;
  u16 uVar10;
  u16 uVar11;

  in_buf_len_5 = (short)((u32)param_3 >> 0x10);
  iVar6 = (int)param_3;
  uVar7 = switch_1018_3b9e(CONCAT31(in_register_00000001,param_1),param_2,param_3,(iVar6 + 0x12e));
  uVar4 = (u32)param_3 & 0xffff0000 | (u32)(iVar6 + 0x22);
  iVar1 = (iVar6 + 0x12e);
  if (iVar1 == 0x188) {
    lVar8 = pass1_1008_57f0(uVar7,(iVar6 + 0x130));
    uVar5 = ((u32)lVar8 >> 0x10);
    pcVar3 = string_1020_c0d8(((int)lVar8 + 0xe));
    uVar2 = (u32)(iVar6 + 0x132);
    uVar10 = uVar2;
    uVar11 = ((u32)uVar2 >> 0x10);
    uVar9 = SUB42(s__ld__s_1050_4150,0x0);
  }
  else if (iVar1 == 0x18b) {
    lVar8 = pass1_1008_57f0(uVar7,(iVar6 + 0x130));
    uVar2 = (u32)((int)lVar8 + 0x4);
    pcVar3 = (char *)uVar2;
    uVar5 = ((u32)uVar2 >> 0x10);
    uVar2 = (u32)(iVar6 + 0x132);
    uVar10 = uVar2;
    uVar11 = ((u32)uVar2 >> 0x10);
    uVar9 = SUB42(s__ld__s_1050_415e,0x0);
  }
  else {
    if (iVar1 != 0x18c) {
      load_string_1010_84e0
                (_u16_1050_14cc,((u32)_u16_1050_14cc >> 0x10),0x100,(char *)(iVar6 + 0x22),
                 in_buf_len_5);
      return;
    }
    lVar8 = pass1_1008_57f0(uVar7,(iVar6 + 0x130));
    uVar2 = (u32)((int)lVar8 + 0x4);
    pcVar3 = (char *)uVar2;
    uVar5 = ((u32)uVar2 >> 0x10);
    uVar2 = (u32)(iVar6 + 0x132);
    uVar10 = uVar2;
    uVar11 = ((u32)uVar2 >> 0x10);
    uVar9 = SUB42(s__ld__s_1050_4157,0x0);
  }
  wsprintf16((WORD *)(iVar6 + 0x22),(char *)CONCAT22(uVar9,in_buf_len_5),(char *)CONCAT22(uVar10,0x1050),uVar11,pcVar3,
             uVar5,lVar8,uVar4);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_str_op_1018_35b0(u16 param_1,astruct_263 *param_2)

{
  i32 *plVar1;
  i16 *piVar2;
  u16 uVar3;
  u16 uVar4;
  u32 uVar5;
  code **ppcVar6;
  u32 *puVar7;
  u16 uVar8;
  u16 uVar9;
  u16 extraout_DX;
  astruct_263 *pstruct263_1;
  i16 iVar10;
  astruct_263 *pstruct263_2;
  u16 uVar11;
  bool bVar12;
  u32 uVar13;
  u32 uVar14;
  i16 local_12;
  i16 local_10;
  i32 lStack14;
  u16 uStack10;
  u16 uStack8;
  u16 uStack6;
  u16 uStack4;

  uVar13 = pass1_1030_8326();
  uStack4 = (uVar13 >> 0x10);
  uStack6 = uVar13;
  pstruct263_2 = (astruct_263 *)((u32)param_2 >> 0x10);
  pstruct263_1 = (astruct_263 *)param_2;
  plVar1 = &pstruct263_1[0x1].field15_0x16;
  bVar12 = plVar1 < uStack4;
  if ((bVar12) || ((bVar12 || plVar1 == uStack4 && (pstruct263_1[0x1].field14_0x14 < uStack6)))) {
    uVar3 = pstruct263_1[0x1].field13_0x12;
    if (&pstruct263_1[0x1].field_0x10 < (int)uVar3) {
      uVar14 = switch_1018_3b9e(uVar3,uStack4,param_2,pstruct263_1[0x1].field4_0x4);
      uVar8 = ((u32)uVar14 >> 0x10);
      uVar11 = uVar14;
      uStack10 = uVar11;
      uStack8 = uVar8;
      pass1_1018_427c(param_2,uVar11,uVar8);
      lStack14 = CONCAT22(uVar8,uVar11);
      pass1_1018_3e8c(pstruct263_1,pstruct263_2,(u16 *)CONCAT22(0x1050,&local_12),
                      (u16 *)CONCAT22(0x1050,&local_10));
      if (lStack14 < local_12) {
        local_12 = (int)lStack14;
      }
      uVar4 = pstruct263_1[0x1].field10_0xe;
      puVar7 = (u32 *)(u32)&pstruct263_1[0x1].field9_0xc;
      uVar9 = uVar4 | puVar7;
      if (uVar9 != 0x0) {
        ppcVar6 = (code **)*puVar7;
        (**ppcVar6)(0x30,puVar7,uVar4,0x1);
        uVar9 = extraout_DX;
      }
      pass1_1018_435e(uVar9,(u32)param_2,lStack14,local_12,local_10);
      pstruct263_1[0x1].field9_0xc = puVar7;
      pstruct263_1[0x1].field10_0xe = uVar9;
      piVar2 = (i16 *)&pstruct263_1[0x1].field_0x10;
      *piVar2 = *piVar2 + 0x1;
      uVar14 = (u32)&pstruct263_1[0x1].field9_0xc;
      uVar11 = ((u32)uVar14 >> 0x10);
      iVar10 = (int)uVar14;
      uVar14 = (u32)(iVar10 + 0x4);
      uVar5 = (u32)(iVar10 + 0x8);
      wsprintf16((WORD *)&pstruct263_1->field_0x22,(char *)CONCAT22(0x4165,pstruct263_2),0x50,
                 CONCAT13((char)((u32)uVar5 >> 0x8),CONCAT12((char)uVar5,0x1050)),(char)uVar5,
                 (int)((u32)uVar5 >> 0x10),(int)uVar14,(int)((u32)uVar14 >> 0x10));
      return;
    }
    pstruct263_1[0x1].field14_0x14 = uStack6;
    &pstruct263_1[0x1].field15_0x16 = uStack4;
    &pstruct263_1[0x1].field_0x10 = 0x0;
    pass1_1008_612e(uStack6,0x8,0xc);
    pstruct263_1[0x1].field13_0x12 = uStack6;
  }
  return;
}



void pass1_1018_36e6(u32 param_1,u16 param_2,u16 param_3,u16 param_4)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  (iVar1 + 0x12e) = param_4;
  (iVar1 + 0x130) = param_3;
  (iVar1 + 0x132) = param_2;
  (iVar1 + 0x134) = 0x0;
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_3710(u16 param_1,u16 param_2,astruct_263 *param_3)

{
  u32 uVar1;
  code **ppcVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  u16 in_register_0000000a;
  astruct_57 *paVar7;
  astruct_263 *iVar8;
  u16 uVar8;
  i32 lVar9;
  u8 local_12a [0x118];
  u32 uStack18;
  astruct_203 *paStack14;
  u32 uStack10;
  astruct_203 *paStack6;

  paStack6 = NULL;
  uVar8 = ((u32)param_3 >> 0x10);
  iVar8 = (astruct_263 *)param_3;
  uStack10 = switch_1018_3b9e(param_1,param_2,param_3,iVar8[0x1].field4_0x4);
  uVar4 = iVar8[0x1].field4_0x4 - 0x188;
  uStack18 = (astruct_203 *)(uStack10 & 0xffff0000 | (u32)uVar4);
  switch(uVar4) {
  case 0x0:
    lVar9 = pass1_1008_57f0(uStack10,iVar8[0x1].field5_0x6);
    uVar3 = ((u32)lVar9 >> 0x10);
    paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,uVar3);
    uVar4 = lVar9;
    mem_op_1000_179c(0x10,paVar7);
    uVar6 = paVar7 | uVar4;
    if (uVar6 != 0x0) {
      uVar3 = struct_1018_4790(CONCAT22(paVar7,uVar4),(u32)&iVar8[0x1].field_0x8,0x0,
                               (lVar9 + 0xe));
      uStack18 = (astruct_203 *)CONCAT22(uVar6,uVar3);
      paStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x1:
    lVar9 = pass1_1008_57f0(uStack10,iVar8[0x1].field5_0x6);
    uVar3 = ((u32)lVar9 >> 0x10);
    paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,uVar3);
    uVar6 = lVar9;
    uVar4 = uVar6;
    mem_op_1000_179c(0x14,paVar7);
    uVar5 = paVar7 | uVar4;
    if (uVar5 != 0x0) {
      struct_1018_47c8((astruct_203 *)CONCAT22(paVar7,uVar4),(u32)&iVar8[0x1].field_0x8,0x0,
                       (uVar6 + 0x12),(u32)(uVar6 + 0xe));
      uStack18 = (astruct_203 *)CONCAT22(uVar5,uVar4);
      paStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x2:
    lVar9 = pass1_1008_57f0(uStack10,iVar8[0x1].field5_0x6);
    uVar3 = ((u32)lVar9 >> 0x10);
    paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,uVar3);
    uVar4 = lVar9;
    mem_op_1000_179c(0x12,paVar7);
    uVar6 = paVar7 | uVar4;
    if (uVar6 != 0x0) {
      pass1_1018_4808((astruct_203 *)CONCAT22(paVar7,uVar4),(u32)&iVar8[0x1].field_0x8,0x0,
                      (u32)(lVar9 + 0xe));
      uStack18 = (astruct_203 *)CONCAT22(uVar6,uVar4);
      paStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x3:
    lVar9 = pass1_1008_57f0(uStack10,iVar8[0x1].field5_0x6);
    uVar3 = ((u32)lVar9 >> 0x10);
    paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,uVar3);
    uVar4 = lVar9;
    mem_op_1000_179c(0x14,paVar7);
    if ((paVar7 | uVar4) != 0x0) {
      uStack18 = struct_1018_4842((astruct_203 *)CONCAT22(paVar7,uVar4),(u32)&iVar8[0x1].field_0x8,0x0,
                                  (lVar9 + 0xe));
      paStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x4:
    lVar9 = pass1_1008_57f0(uStack10,iVar8[0x1].field5_0x6);
    uVar3 = ((u32)lVar9 >> 0x10);
    paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,uVar3);
    uVar4 = lVar9;
    mem_op_1000_179c(0x10,paVar7);
    if ((paVar7 | uVar4) != 0x0) {
      uStack18 = struct_1018_48b0((astruct_203 *)CONCAT22(paVar7,uVar4),(u32)&iVar8[0x1].field_0x8,0x0,
                                  (lVar9 + 0xe));
      paStack6 = uStack18;
      goto switchD_1018_393f_caseD_6;
    }
    break;
  case 0x5:
    lVar9 = pass1_1008_57f0(uStack10,iVar8[0x1].field5_0x6);
    uVar3 = ((u32)lVar9 >> 0x10);
    paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,uVar3);
    uVar4 = lVar9;
    mem_op_1000_179c(0x12,paVar7);
    uVar6 = paVar7 | uVar4;
    if (uVar6 != 0x0) {
      uVar3 = struct_1018_4920((astruct_203 *)CONCAT22(paVar7,uVar4),(u32)&iVar8[0x1].field_0x8,0x0,
                               (u32)(lVar9 + 0xe));
      uStack18 = (astruct_203 *)CONCAT22(uVar6,uVar3);
      paStack6 = uStack18;
    }
    break;
  default:
    goto switchD_1018_393f_caseD_6;
  }
  uStack18 = NULL;
  paStack6 = uStack18;
switchD_1018_393f_caseD_6:
  uVar1 = iVar8->field274_0x122;
  pass1_1008_e852(((u32)uStack18 >> 0x10),uVar1,((u32)uVar1 >> 0x10),
                  (char *)iVar8->field275_0x126);
  uVar1 = iVar8->field274_0x122;
  paStack14 = uStack18;
  pass1_1008_e852(((u32)uStack18 >> 0x10),uVar1,((u32)uVar1 >> 0x10),*(char **)(iVar8 + 0x1));
  pass1_1038_2a0e((astruct_97 *)CONCAT22(0x1050,local_12a),(u32)&iVar8[0x1].field9_0xc,(u32)paStack6,
                  (u32)uStack18,(u32)paStack14);
  fn_ptr_1030_835a(_u16_1050_5748,(char *)CONCAT22(0x1050,local_12a));
  (u32)&iVar8[0x1].field9_0xc = 0x0;
  ppcVar2 = (code **)((int)(u32)param_3 + 0x10);
  (**ppcVar2)(0x1030,param_3);
  pass1_1038_2a5c((u16 *)CONCAT22(0x1050,local_12a));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 string_1018_39d8(u32 param_1,char *param_2,char *param_3)

{
  i16 iVar1;
  char *pcVar2;
  i32 lVar3;
  char *pcVar4;

  pcVar4 = param_2;
  pcVar2 = load_string_1010_847e(_u16_1050_14cc,0x531);
  iVar1 = pass1_1000_3d7a(pcVar2,pcVar4);
  if (iVar1 != 0x0) {
    iVar1 = pass1_1000_3d7a(param_3,param_2);
    if (iVar1 != 0x0) {
      lVar3 = pass1_1018_4608(param_1,param_2,param_3);
      if ((lVar3 != 0x0) && (((int)lVar3 + 0xc) == 0x1)) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



void pass1_1018_3a42(u16 param_1,u32 param_2,u32 param_3)

{
  u32 uVar1;

  uVar1 = (u32)((int)param_2 + 0x122);
  pass1_1008_e852(param_1,uVar1,((u32)uVar1 >> 0x10),(char *)param_3);
  return;
}



void pass1_1018_3a5c(u32 param_1,char *param_2,char *param_3)

{
  pass1_1008_e320(*(astruct_102 **)((int)param_1 + 0x122),param_2,param_3);
  return;
}



u32 pass1_1018_3a7a(u16 param_1,u16 param_2,u32 param_3,u32 param_4)

{
  u32 uVar1;
  u32 uVar2;

  uVar1 = (u32)((int)param_3 + 0x122);
  uVar2 = string_1008_e586(uVar1,((u32)uVar1 >> 0x10),param_4,param_1,param_2);
  return uVar2;
}



void pass1_1018_3a94(u32 param_1,u32 *param_2,u32 *param_3)

{
  pass1_1008_e3ec(*(astruct_218 **)((int)param_1 + 0x122),param_2,param_3);
  return;
}



u16 pass1_1018_3ab2(u32 param_1,i16 param_2,i16 param_3)

{
  u16 uVar1;
  u16 uVar2;
  i16 iVar3;
  i32 lVar4;
  u16 uStack22;
  u8 local_10 [0x8];
  i16 iStack8;
  u32 uStack6;

  if (0x5 < param_3 - 0x188U) {
    return 0x0;
  }
  iVar3 = (int)param_1;
  uVar2 = (param_1 >> 0x10);
  switch(param_3) {
  case 0x188:
    uVar1 = (iVar3 + 0xa);
    uVar2 = (iVar3 + 0xc);
    break;
  case 0x189:
    uVar1 = (iVar3 + 0xe);
    uVar2 = (iVar3 + 0x10);
    break;
  case 0x18a:
    uVar1 = (iVar3 + 0x12);
    uVar2 = (iVar3 + 0x14);
    break;
  case 0x18b:
    uVar1 = (iVar3 + 0x16);
    uVar2 = (iVar3 + 0x18);
    break;
  case 0x18c:
    uVar1 = (iVar3 + 0x1a);
    uVar2 = (iVar3 + 0x1c);
    break;
  case 0x18d:
    uVar1 = (iVar3 + 0x1e);
    uVar2 = (iVar3 + 0x20);
  }
  uStack6 = CONCAT22(uVar2,uVar1);
  iStack8 = 0x0;
  pass1_1008_5784((char *)CONCAT22(0x1050,local_10),uStack6);
  while( true ) {
    lVar4 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_10));
    uVar2 = ((u32)lVar4 >> 0x10);
    if ((lVar4 == 0x0) || (iStack8 == param_2)) break;
    iStack8 += 0x1;
  }
  uStack22 = 0x0;
  if (lVar4 != 0x0) {
    if (((int)lVar4 + 0xa) == 0x0) {
      uStack22 = ((int)lVar4 + 0x8);
    }
    else {
      uStack22 = 0xffff;
    }
  }
  return uStack22;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 switch_1018_3b9e(u16 param_1,u16 param_2,astruct_263 *param_3,u16 param_4)

{
  u32 uVar1;
  astruct_263 *pstruct_1_1;
  astruct_263 *pstruct_1_2;
  astruct_6 *paStack14;
  u16 uStack6;
  u16 uStack4;

  uStack6 = 0x0;
  uStack4 = 0x0;
  pstruct_1_2 = (astruct_263 *)((u32)param_3 >> 0x10);
  pstruct_1_1 = (astruct_263 *)param_3;
  uVar1 = pstruct_1_1->field274_0x122;
  pass1_1008_e852(param_2,uVar1,((u32)uVar1 >> 0x10),(char *)pstruct_1_1->field275_0x126);
  pass1_1030_8344(_u16_1050_5748,CONCAT22(param_2,param_1));
  paStack14 = (astruct_6 *)CONCAT22(param_2,param_1);
  switch(param_4) {
  case 0x188:
    if (*(i32 *)&pstruct_1_1->field8_0xa == 0x0) {
      pass1_1008_d3ae((astruct_263 *)((u32)param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10));
    }
    uStack6 = pstruct_1_1->field8_0xa;
    uStack4 = pstruct_1_1->field9_0xc;
    break;
  case 0x189:
    if (*(i32 *)&pstruct_1_1->field10_0xe == 0x0) {
      unk_str_op_1008_d4f6((astruct_263 *)((u32)param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10),paStack14);
    }
    uStack6 = pstruct_1_1->field10_0xe;
    uStack4 = &pstruct_1_1->field_0x10;
    break;
  case 0x18a:
    if (*(i32 *)&pstruct_1_1->field13_0x12 == 0x0) {
      unk_str_op_1008_d1c6((astruct_263 *)((u32)param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10),(u32)paStack14);
    }
    uStack6 = pstruct_1_1->field13_0x12;
    uStack4 = pstruct_1_1->field14_0x14;
    break;
  case 0x18b:
    if (pstruct_1_1->field15_0x16 == 0x0) {
      pass1_1008_cfa0((astruct_263 *)((u32)param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10),(u32)paStack14);
    }
    uStack6 = &pstruct_1_1->field15_0x16;
    uStack4 = ((int)&pstruct_1_1->field15_0x16 + 0x2);
    break;
  case 0x18c:
    if (pstruct_1_1->field16_0x1a == 0x0) {
      pass1_1008_cda2((astruct_263 *)((u32)param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10),(u32)paStack14);
    }
    uStack6 = &pstruct_1_1->field16_0x1a;
    uStack4 = ((int)&pstruct_1_1->field16_0x1a + 0x2);
    break;
  case 0x18d:
    if (pstruct_1_1->field17_0x1e == 0x0) {
      pass1_1008_cbc4((astruct_263 *)((u32)param_3 & 0xffff | ZEXT24(pstruct_1_2) << 0x10),(u32)paStack14);
    }
    uStack6 = &pstruct_1_1->field17_0x1e;
    uStack4 = ((int)&pstruct_1_1->field17_0x1e + 0x2);
  }
  return CONCAT22(uStack4,uStack6);
}



void pass1_1018_3cda(astruct_506 *param_1,char *param_2,char *param_3)

{
  code **ppcVar1;
  u16 uVar2;
  u16 extraout_DX;
  u16 uVar3;
  astruct_506 *iVar5;
  u16 uVar4;

  uVar4 = ((u32)param_1 >> 0x10);
  iVar5 = (astruct_506 *)param_1;
  ppcVar1 = (code **)((int)(u32)param_1 + 0x10);
  (**ppcVar1)();
  uVar3 = extraout_DX;
  fn_ptr_1000_17ce(*(char **)&iVar5->field294_0x126);
  fn_ptr_1000_17ce(*(char **)&iVar5->field296_0x12a);
  uVar2 = str_op_1008_60e8(uVar3,param_3);
  iVar5->field294_0x126 = uVar2;
  iVar5->field295_0x128 = uVar3;
  uVar2 = str_op_1008_60e8(uVar3,param_2);
  iVar5->field296_0x12a = uVar2;
  iVar5->field297_0x12c = uVar3;
  return;
}



void pass1_1018_3d44(u32 param_1,u32 *param_2,u32 *param_3)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  *param_3 = (u32)((int)param_1 + 0x126);
  *param_2 = (u32)((int)param_1 + 0x12a);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_3d6c(astruct_679 *param_1)

{
  u8 bVar1;
  u16 uVar2;
  u8 *puVar3;
  u16 uVar4;
  astruct_679 *iVar6;
  astruct_679 *uVar5;
  u32 uVar6;
  u32 uVar7;

  uVar5 = (astruct_679 *)((u32)param_1 >> 0x10);
  iVar6 = (astruct_679 *)param_1;
  uVar4 = iVar6->field322_0x142;
  uVar2 = uVar4 + 0x1e;
  if (iVar6->field323_0x144 + 0x1U == (uVar4 < 0xffe2)) {
    if (uVar2 != 0x3c) {
      if (0x3c < uVar2) {
        return;
      }
      bVar1 = (u8)uVar2;
      if (bVar1 == 0x14) {
        iVar6->field322_0x142 = 0xffec;
LAB_1018_3e3d:
        iVar6->field323_0x144 = -0x1;
        return;
      }
      if (0x14 < bVar1) {
        if (bVar1 == 0x1e) {
          if ((int)PTR_LOOP_1050_13ae < 0x1) {
            return;
          }
          if (SBORROW2((int)PTR_LOOP_1050_13ae,0x1)) {
            return;
          }
          if (PTR_LOOP_1050_13ae != (u8 *)&u16_1050_0002 && 0x0 < (int)(PTR_LOOP_1050_13ae + -0x1)) {
            puVar3 = PTR_LOOP_1050_13ae + -0x3;
            if (puVar3 == NULL) {
              pass1_1008_612e(0x0,0x1,0x64);
              if ((int)puVar3 < 0x32) {
                uVar4 = 0xa;
              }
              else {
                uVar4 = 0xfff6;
              }
              iVar6->field322_0x142 = uVar4;
              iVar6->field323_0x144 = (int)uVar4 >> 0xf;
              return;
            }
            if (puVar3 != (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1)) {
              return;
            }
            iVar6->field322_0x142 = 0xfff6;
            goto LAB_1018_3e3d;
          }
          iVar6->field322_0x142 = 0xa;
        }
        else if (bVar1 == 0x28) {
          iVar6->field322_0x142 = 0x14;
        }
        else {
          if (bVar1 != 0x32) {
            return;
          }
          iVar6->field322_0x142 = 0x1e;
        }
        iVar6->field323_0x144 = 0x0;
        return;
      }
      if (bVar1 != 0x0) {
        if (bVar1 != 0xa) {
          return;
        }
        iVar6->field322_0x142 = 0xffe2;
        goto LAB_1018_3e3d;
      }
    }
    uVar7 = 0x5;
    uVar6 = pass1_1030_8326();
    if (uVar6 % uVar7 == 0x0) {
      (u32)&iVar6->field322_0x142 = 0x0;
      return;
    }
  }
  return;
}



void pass1_1018_3e8c(astruct_263 *param_1,astruct_263 *param_2,u16 *param_3,u16 *param_4)

{
  *param_4 = 0x1;
  *param_3 = 0x19;
  return;
}



void pass1_1018_3ea4(astruct_455 *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  astruct_455 *iVar4;
  astruct_455 *uVar4;

  pass1_1008_cac6(param_1);
  uVar4 = (astruct_455 *)((u32)param_1 >> 0x10);
  iVar4 = (astruct_455 *)param_1;
  puVar1 = (u32 *)iVar4[0x26].field3_0x6;
  uVar2 = (iVar4 + 0x27)->field0_0x0;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1008,puVar1,uVar2,0x1);
  }
  (u32)&iVar4[0x26].field3_0x6 = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void switch_1018_3ee6(u16 param_1,u32 param_2,i32 param_3,i16 param_4,u16 param_5)

{
  i16 iVar1;
  char *pcVar2;
  u16 uVar3;
  u16 uVar4;
  u16 uVar5;
  u32 uVar6;
  u16 uVar7;
  u16 in_register_0000000a;
  astruct_57 *paVar8;
  astruct_203 *paVar9;
  i32 lVar10;
  i16 iVar11;
  astruct_263 *paVar12;
  INT16 IVar13;
  astruct_263 *paVar14;
  u16 uVar15;
  u32 uStack26;
  astruct_203 *paStack22;
  i32 lStack18;
  i32 lStack14;
  i16 iStack10;
  u16 uStack8;
  i16 *piStack6;

  paVar8 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  switch(param_5) {
  case 0x1:
    iVar1 = param_4 * 0x4 + 0x40b6;
    break;
  default:
    iVar1 = param_4 * 0x4 + 0x40ce;
    break;
  case 0x3:
    iVar1 = param_4 * 0x4 + 0x40e2;
    break;
  case 0x4:
    iVar1 = param_4 * 0x4 + 0x40ee;
    break;
  case 0x8:
    iVar1 = param_4 * 0x4 + 0x40f2;
    break;
  case 0x9:
    iVar1 = param_4 * 0x4 + 0x4106;
    break;
  case 0xa:
    iVar1 = param_4 * 0x4 + 0x410a;
    break;
  case 0x14:
    iVar1 = param_4 * 0x4 + 0x410e;
    break;
  case 0x16:
    iVar1 = param_4 * 0x4 + 0x4112;
    break;
  case 0x17:
    iVar1 = param_4 * 0x4 + 0x4116;
    break;
  case 0x19:
    iVar1 = param_4 * 0x4 + 0x411a;
  }
  piStack6 = (i16 *)CONCAT22(0x1050,iVar1);
  if (piStack6 == NULL) {
    return;
  }
  iStack10 = 0x0;
  uStack8 = 0x0;
  iVar11 = *piStack6;
  paVar12 = (astruct_263 *)param_2;
  paVar14 = (astruct_263 *)(param_2 >> 0x10);
  if (iVar11 == 0x1) {
    uVar15 = pass1_1018_456a(paVar12,paVar14,(iVar1 + 0x2));
    lStack14 = CONCAT22((int)paVar8,uVar15);
    pcVar2 = string_1020_c0d8((iVar1 + 0x2));
    uVar3 = str_op_1008_60e8(paVar8,(char *)CONCAT22(paVar8,pcVar2));
    uVar7 = SUB42(paVar8,0x0);
    uVar15 = uVar3;
    mem_op_1000_179c(0x10,paVar8);
    paStack22 = (astruct_203 *)CONCAT22(paVar8,uVar15);
    if ((paVar8 | uVar15) != 0x0) {
      lVar10 = param_3 / lStack14;
      uStack8 = (param_3 % lStack14);
      struct_1018_4790(paStack22,lVar10,CONCAT22(uVar7,uVar3),(iVar1 + 0x2));
      iStack10 = (int)lVar10;
      goto LAB_1018_425e;
    }
  }
  else if (iVar11 == 0x2) {
    uVar15 = pass1_1018_451e(paVar12,paVar14,(iVar1 + 0x2));
    lStack18 = CONCAT22((int)paVar8,uVar15);
    pcVar2 = string_op_1020_c222((iVar1 + 0x2));
    uVar3 = str_op_1008_60e8(paVar8,(char *)CONCAT22(paVar8,pcVar2));
    uVar7 = SUB42(paVar8,0x0);
    uVar15 = uVar3;
    mem_op_1000_179c(0x10,paVar8);
    paStack22 = (astruct_203 *)CONCAT22(paVar8,uVar15);
    if ((paVar8 | uVar15) != 0x0) {
      paVar9 = struct_1018_48b0(paStack22,param_3 / lStack18,CONCAT22(uVar7,uVar3),(iVar1 + 0x2));
      uStack8 = ((u32)paVar9 >> 0x10);
      iStack10 = (int)paVar9;
      goto LAB_1018_425e;
    }
  }
  else if (iVar11 == 0x3) {
    uVar15 = pass1_1008_c646(_u16_1050_06e0,
                             CONCAT22((iVar1 + 0x2),(int)((u32)_u16_1050_06e0 >> 0x10)));
    if (uVar15 == 0x0) {
      uVar15 = 0x4f;
    }
    uVar3 = switch_1018_43ec(paVar12,paVar14,uVar15);
    lStack14 = CONCAT22((int)paVar8,uVar3);
    uVar3 = pass1_1020_bd80(uVar15);
    uVar4 = str_op_1008_60e8(paVar8,(char *)CONCAT22(paVar8,uVar3));
    uStack26 = CONCAT22((int)paVar8,uVar4);
    mem_op_1000_179c(0x14,paVar8);
    paStack22 = (astruct_203 *)CONCAT22(paVar8,uVar4);
    if ((paVar8 | uVar4) != 0x0) {
      uVar6 = param_3 / lStack14;
      uStack8 = (param_3 % lStack14);
      struct_1018_47c8(paStack22,uVar6,uStack26,uVar15,0x0);
      iStack10 = (int)uVar6;
      goto LAB_1018_425e;
    }
  }
  else {
    if (iVar11 != 0x4) goto LAB_1018_425e;
    iVar1 = (iVar1 + 0x2);
    uVar4 = iVar1 - 0x1;
    iVar11 = (int)_u16_1050_14cc;
    IVar13 = (INT16)((u32)_u16_1050_14cc >> 0x10);
    if (uVar4 == 0x0) {
      load_string_1010_84ac(iVar11,IVar13,0x430);
      uVar7 = SUB42(paVar8,0x0);
      uVar5 = uVar4;
      mem_op_1000_179c(0x14,paVar8);
      paStack22 = (astruct_203 *)CONCAT22(paVar8,uVar5);
      if ((paVar8 | uVar5) != 0x0) {
        uVar15 = 0x2;
        lVar10 = 0x14;
LAB_1018_4230:
        paVar9 = struct_1018_4842(paStack22,param_3 / lVar10,CONCAT22(uVar7,uVar4),uVar15);
        uStack8 = ((u32)paVar9 >> 0x10);
        iStack10 = (int)paVar9;
        goto LAB_1018_425e;
      }
    }
    else {
      uVar4 = iVar1 - 0x2;
      if (uVar4 == 0x0) {
        load_string_1010_84ac(iVar11,IVar13,0x431);
        uVar7 = SUB42(paVar8,0x0);
        uVar5 = uVar4;
        mem_op_1000_179c(0x14,paVar8);
        paStack22 = (astruct_203 *)CONCAT22(paVar8,uVar5);
        if ((paVar8 | uVar5) != 0x0) {
          uVar15 = 0x3;
          lVar10 = 0x16;
          goto LAB_1018_4230;
        }
      }
      else {
        uVar4 = iVar1 - 0x3;
        if (uVar4 == 0x0) {
          load_string_1010_84ac(iVar11,IVar13,0x432);
          uVar7 = SUB42(paVar8,0x0);
          uVar5 = uVar4;
          mem_op_1000_179c(0x14,paVar8);
          paStack22 = (astruct_203 *)CONCAT22(paVar8,uVar5);
          if ((paVar8 | uVar5) != 0x0) {
            uVar15 = 0x4;
            lVar10 = 0x17;
            goto LAB_1018_4230;
          }
        }
        else {
          uVar4 = iVar1 - 0x4;
          if (uVar4 != 0x0) goto LAB_1018_425e;
          load_string_1010_84ac(iVar11,IVar13,0x433);
          uVar7 = SUB42(paVar8,0x0);
          uVar5 = uVar4;
          mem_op_1000_179c(0x14,paVar8);
          paStack22 = (astruct_203 *)CONCAT22(paVar8,uVar5);
          if ((paVar8 | uVar5) != 0x0) {
            uVar15 = 0x4;
            lVar10 = 0xa;
            goto LAB_1018_4230;
          }
        }
      }
    }
  }
  iStack10 = 0x0;
  uStack8 = 0x0;
LAB_1018_425e:
  if (*(i32 *)(iStack10 + 0x8) == 0x0) {
    (u32)(iStack10 + 0x8) = 0x1;
  }
  return;
}



void pass1_1018_427c(astruct_263 *param_1,u16 param_2,u16 param_3)

{
  u16 uVar1;
  astruct_263 *pstruct263_1;
  astruct_263 *pstruct263_2;
  u32 uVar2;
  i32 lVar3;

  pstruct263_2 = (astruct_263 *)((u32)param_1 >> 0x10);
  pstruct263_1 = (astruct_263 *)param_1;
  uVar2 = switch_1018_3b9e(param_2,param_3,param_1,pstruct263_1[0x1].field4_0x4);
  uVar1 = pstruct263_1[0x1].field4_0x4;
  if (uVar1 == 0x188) {
    lVar3 = pass1_1008_57f0(uVar2,pstruct263_1[0x1].field5_0x6);
    pass1_1018_456a(pstruct263_1,pstruct263_2,((int)lVar3 + 0xe));
  }
  else if (uVar1 == 0x18b) {
    lVar3 = pass1_1008_57f0(uVar2,pstruct263_1[0x1].field5_0x6);
    pass1_1018_45d4(pstruct263_1,pstruct263_2,((int)lVar3 + 0xe));
  }
  else if (uVar1 == 0x18c) {
    lVar3 = pass1_1008_57f0(uVar2,pstruct263_1[0x1].field5_0x6);
    pass1_1018_451e(pstruct263_1,pstruct263_2,((int)lVar3 + 0xe));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_435e(u16 param_1,u32 param_2,i32 param_3,i16 param_4,i16 param_5)

{
  u32 uVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;

  if (param_4 < param_5) {
    param_5 = param_4;
  }
  uVar2 = 0x0;
  uVar4 = (param_2 >> 0x10);
  uVar1 = (u32)((int)param_2 + 0x122);
  pass1_1008_e852(param_1,uVar1,((u32)uVar1 >> 0x10),*(char **)((int)param_2 + 0x126));
  pass1_1030_8344(_u16_1050_5748,CONCAT22(param_1,uVar2));
  do {
    do {
      uVar3 = uVar2;
      pass1_1008_612e(uVar3,param_5,param_4);
      uVar2 = (uVar3 * 0x2 + 0x411c);
    } while (uVar2 == 0x0);
    if (uVar2 != 0x1) {
      pass1_1008_612e(uVar2,0x1,uVar2);
    }
    uVar2 -= 0x1;
    switch_1018_3ee6(param_1,param_2,param_3,uVar2,uVar3);
    param_1 |= uVar2;
  } while (param_1 == 0x0);
  return;
}



u16 switch_1018_43ec(u16 param_1,u16 param_2,u16 param_3)

{
  u16 uStack6;

  switch(param_3) {
  case 0xf:
  case 0x35:
  case 0x36:
    uStack6 = 0x7;
    break;
  default:
    uStack6 = 0x1;
    break;
  case 0x11:
  case 0x13:
  case 0x14:
  case 0x15:
  case 0x2d:
  case 0x2e:
  case 0x6e:
    uStack6 = 0x9;
    break;
  case 0x12:
  case 0x31:
  case 0x32:
  case 0x52:
  case 0x53:
  case 0x54:
  case 0x55:
  case 0x56:
  case 0x5a:
  case 0x5b:
  case 0x5c:
  case 0x5d:
  case 0x5e:
  case 0x5f:
    uStack6 = 0x4;
    break;
  case 0x1b:
  case 0x1c:
  case 0x1d:
  case 0x28:
  case 0x29:
  case 0x2c:
  case 0x2f:
  case 0x30:
  case 0x68:
  case 0x69:
    uStack6 = 0x5;
    break;
  case 0x1e:
  case 0x1f:
  case 0x20:
  case 0x33:
  case 0x34:
    uStack6 = 0x6;
    break;
  case 0x22:
  case 0x23:
  case 0x24:
    uStack6 = 0x8;
    break;
  case 0x25:
  case 0x26:
  case 0x27:
    uStack6 = 0x2;
    break;
  case 0x38:
  case 0x39:
  case 0x4f:
  case 0x50:
  case 0x51:
  case 0x57:
  case 0x58:
  case 0x59:
  case 0x66:
  case 0x67:
  case 0x6c:
  case 0x6d:
    uStack6 = 0x3;
  }
  return uStack6;
}



u16 pass1_1018_451e(astruct_263 *param_1,astruct_263 *param_2,i16 param_3)

{
  u16 uStack6;

  if (param_3 == 0x7) {
    uStack6 = 0x9;
  }
  else if (param_3 == 0x8) {
    uStack6 = 0xa;
  }
  else if (param_3 == 0xc) {
    uStack6 = 0x19;
  }
  else if (param_3 == 0xd) {
    uStack6 = 0x3;
  }
  else {
    uStack6 = 0x8;
  }
  return uStack6;
}



u16 pass1_1018_456a(astruct_263 *param_1,astruct_263 *param_2,u16 param_3)

{
  u16 uStack6;

  switch(param_3) {
  case 0x11:
  case 0x12:
  case 0x13:
  case 0x14:
  case 0x15:
    uStack6 = 0x2;
    break;
  case 0x16:
  case 0x1e:
    uStack6 = 0x3;
    break;
  default:
    uStack6 = 0x1;
    break;
  case 0x1d:
  case 0x21:
    uStack6 = 0x4;
  }
  return uStack6;
}



u16 pass1_1018_45d4(astruct_263 *param_1,astruct_263 *param_2,i16 param_3)

{
  u16 uStack6;

  if (param_3 == 0x3) {
    uStack6 = 0x16;
  }
  else if (param_3 == 0x4) {
    uStack6 = 0x17;
  }
  else {
    uStack6 = 0x14;
  }
  return uStack6;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i32 pass1_1018_4608(u32 param_1,char *param_2,char *param_3)

{
  u32 uVar1;
  u16 uVar2;
  u16 uVar3;
  i16 iVar4;
  u16 uVar5;
  u16 uVar6;
  i32 lVar7;
  char *pcVar8;
  char *pcVar9;
  char *pcStack26;
  char *pcStack22;
  char local_a [0x8];

  uVar1 = (u32)((int)param_1 + 0x122);
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)((int)uVar1 + 0xa));
  while( true ) {
    lVar7 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    uVar5 = ((u32)lVar7 >> 0x10);
    uVar2 = lVar7;
    uVar6 = uVar5 | uVar2;
    if (lVar7 == 0x0) {
      return 0x0;
    }
    uVar3 = uVar2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(uVar2 + 0x4));
    pcStack22 = (char *)CONCAT22(uVar6,uVar3);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,(u32)(uVar2 + 0x8));
    pcStack26 = (char *)CONCAT22(uVar6,uVar3);
    pcVar8 = pass1_1038_4d28(pcStack22);
    pcVar9 = pass1_1038_4d28(pcStack26);
    iVar4 = pass1_1000_3d7a(param_3,pcVar8);
    if ((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_2,pcVar9), iVar4 == 0x0)) break;
    iVar4 = pass1_1000_3d7a(param_2,pcVar8);
    if ((iVar4 == 0x0) && (iVar4 = pass1_1000_3d7a(param_3,pcVar9), iVar4 == 0x0)) {
      return lVar7;
    }
  }
  return lVar7;
}



u16 * pass1_1018_46e6(u16 *param_1,u8 param_2)

{
  pass1_1018_33b4((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void struct_1018_4720(astruct_203 *param_1,u32 param_2,u32 param_3)

{
  astruct_203 *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_203 *)param_1;
  param_1->field0_0x0 = 0x389a;
  iVar1->field1_0x2 = 0x1008;
  (u32)&iVar1->field2_0x4 = param_3;
  (u32)&iVar1->field4_0x8 = param_2;
  iVar1->field6_0xc = 0x0;
  param_1->field0_0x0 = &PTR_LOOP_1050_4aa6;
  iVar1->field1_0x2 = 0x1018;
  return;
}



void pass1_1018_4760(StructD *param_1)

{
  StructD *iVar2;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar2 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = &PTR_LOOP_1050_4aa6;
  iVar2->address_offset_field_0x2 = 0x1018;
  fn_ptr_1000_17ce(*(char **)&iVar2->hfile_0x4);
  param_1->address_offset_field_0x0 = 0x389a;
  iVar2->address_offset_field_0x2 = 0x1008;
  return;
}



astruct_203 * struct_1018_4790(astruct_203 *param_1,u32 param_2,u32 param_3,u16 param_4)

{
  astruct_203 *iVar1;
  u16 uVar1;

  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_203 *)param_1;
  &iVar1->field7_0xe = param_4;
  param_1->field0_0x0 = 0x4a92;
  iVar1->field1_0x2 = 0x1018;
  iVar1->field6_0xc = 0x1;
  return param_1;
}



void struct_1018_47c8(astruct_203 *param_1,u32 param_2,u32 param_3,u16 param_4,u32 param_5)

{
  astruct_203 *iVar1;
  u16 uVar1;

  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_203 *)param_1;
  iVar1->field7_0xe = param_5;
  (iVar1 + 0x1)->field0_0x0 = param_4;
  param_1->field0_0x0 = &PTR_LOOP_1050_4a9a;
  iVar1->field1_0x2 = 0x1018;
  iVar1->field6_0xc = 0x2;
  return;
}



void pass1_1018_4808(astruct_203 *param_1,u32 param_2,u32 param_3,u32 param_4)

{
  astruct_203 *iVar1;
  u16 uVar1;

  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_203 *)param_1;
  iVar1->field7_0xe = param_4;
  param_1->field0_0x0 = &PTR_LOOP_1050_4aa2;
  iVar1->field1_0x2 = 0x1018;
  iVar1->field6_0xc = 0x3;
  return;
}



astruct_203 * struct_1018_4842(astruct_203 *param_1,u32 param_2,u32 param_3,u16 param_4)

{
  astruct_203 *iVar1;
  u16 uVar1;

  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_203 *)param_1;
  &iVar1->field7_0xe = param_4;
  (u32)((int)&iVar1->field7_0xe + 0x2) = 0x0;
  param_1->field0_0x0 = &PTR_LOOP_1050_4a8e;
  iVar1->field1_0x2 = 0x1018;
  iVar1->field6_0xc = 0x4;
  return param_1;
}



void pass1_1018_4882(StructD *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = &PTR_LOOP_1050_4a8e;
  ((int)param_1 + 0x2) = 0x1018;
  fn_ptr_1000_17ce(*(char **)((int)param_1 + 0x10));
  pass1_1018_4760(param_1);
  return;
}



astruct_203 * struct_1018_48b0(astruct_203 *param_1,u32 param_2,u32 param_3,u16 param_4)

{
  astruct_203 *iVar1;
  u16 uVar1;

  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_203 *)param_1;
  &iVar1->field7_0xe = param_4;
  param_1->field0_0x0 = &PTR_LOOP_1050_4a96;
  iVar1->field1_0x2 = 0x1018;
  iVar1->field6_0xc = 0x5;
  return param_1;
}



u16 * struct_1018_48e8(astruct_203 *param_1,u32 param_2,u32 param_3,u16 param_4)

{
  astruct_203 *iVar1;
  u16 uVar1;

  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_203 *)param_1;
  &iVar1->field7_0xe = param_4;
  param_1->field0_0x0 = 0x4a9e;
  iVar1->field1_0x2 = 0x1018;
  iVar1->field6_0xc = 0x6;
  return &param_1->field0_0x0;
}



void struct_1018_4920(astruct_203 *param_1,u32 param_2,u32 param_3,u32 param_4)

{
  astruct_203 *iVar1;
  u16 uVar1;

  struct_1018_4720(param_1,param_2,param_3);
  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (astruct_203 *)param_1;
  iVar1->field7_0xe = param_4;
    // just 0x4a8a
  param_1->field0_0x0 = &PTR_LOOP_1050_4a8a;
  iVar1->field1_0x2 = 0x1018;
  iVar1->field6_0xc = 0x7;
  return;
}



StructD * pass1_1018_495a(StructD *param_1,u8 param_2)

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * pass1_1018_4980(StructD *param_1,u8 param_2)

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * pass1_1018_49a6(StructD *param_1,u8 param_2)

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * pass1_1018_49cc(StructD *param_1,u8 param_2)

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * pass1_1018_49f2(StructD *param_1,u8 param_2)

{
  pass1_1018_4882(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * pass1_1018_4a18(StructD *param_1,u8 param_2)

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * pass1_1018_4a3e(StructD *param_1,u8 param_2)

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * pass1_1018_4a64(StructD *param_1,u8 param_2)

{
  pass1_1018_4760(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_4aaa(u8 *param_1,astruct_19 *param_2,u16 param_3)

{
  struct_op_1018_4cda(param_2,param_3);
  param_2->offset_0x0 = 0x4b06;
  ((int)param_2 + 0x2) = 0x1018;
  pass1_1018_4dce(param_1,param_2,0x9a);
  _PTR_LOOP_1050_4230 = param_2;
  return;
}



struct * pass1_1018_4ae0(StructD *param_1,u8 param_2)

{
  clenaup_win_ui_1018_4d22(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return (struct *)param_1;
}



u16 * get_sys_metrics_1018_4b1e(astruct_19 *param_1,u16 param_2,u16 param_3)

{
  astruct_19 *pstruct19_1;
  astruct_19 *pstruct19_param_1_hi;

  struct_op_1010_1d48(param_1,param_3);
  pstruct19_param_1_hi = (astruct_19 *)((u32)param_1 >> 0x10);
  pstruct19_1 = (astruct_19 *)param_1;
  pstruct19_1->field9_0x12 = param_2;
  pstruct19_1->field10_0x14 = 0x0;
    // 0x4c9e val
  param_1->offset_0x0 = &PTR_LOOP_1050_4c9e;
  pstruct19_1->segment_0x2 = 0x1018;
  if (G_SM_CYCAPTION_1050_416c == 0x0) {
    G_SM_CYCAPTION_1050_416c = GetSystemMetrics16(SM_CYCAPTION);
    G_SM_CXBORDER_1050_416e = GetSystemMetrics16(SM_CXBORDER);
    G_SM_CYBORDER_1050_4170 = GetSystemMetrics16(SM_CYBORDER);
  }
  return &param_1->offset_0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_4b78(astruct_19 *param_1)

{
  code **ppcVar1;
  u16 uVar2;
  u32 in_EDX;
  astruct_57 *paVar3;
  u16 unaff_SI;
  u32 *puVar4;
  u32 *puVar5;
  u16 in_stack_0000fe9a;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffc8;

  paVar3 = (astruct_57 *)(in_EDX & 0xffff0000 | (u32)param_1);
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24((u32 *)((int)param_1 + 0xa))),NULL,0x8);
  pass1_1000_4906((StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 + 0x18)),NULL,0x8);
  puVar4 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  puVar5 = (u32 *)pass1_1010_5f7a((int)puVar4,((u32)puVar4 >> 0x10),0x0,((int)param_1 + 0x12));
  uVar2 = ((u32)puVar5 >> 0x10);
  if ((uVar2 | puVar5) != 0x0) {
    (u32)((int)param_1 + 0xa) = *puVar5;
    (u32)((int)param_1 + 0xe) = (u32)(puVar5 + 0x4);
  }
  ppcVar1 = (code **)((int)(u32)param_1 + 0x20);
  (**ppcVar1)(0x1010,param_1);
  if ((((int)param_1 + 0xe) == 0x0) && (((int)param_1 + 0x10) == 0x0)) {
    ((int)param_1 + 0xa) = ((int)param_1 + 0x18);
    ((int)param_1 + 0xc) = ((int)param_1 + 0x1a);
  }
  ((int)param_1 + 0xe) = ((int)param_1 + 0x1c);
  ((int)param_1 + 0x10) = ((int)param_1 + 0x1e);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_4c2c(u32 param_1,u32 *param_2,u16 param_3)

{
  u32 in_EDX;
  u16 unaff_SI;
  u32 *puVar1;
  u16 in_stack_0000fe9e;
  u16 in_stack_0000ffc2;
  u16 in_stack_0000ffc8;
  u16 in_stack_0000ffcc;

  (u32)((int)param_1 + 0xa) = *param_2;
  (u32)((int)param_1 + 0xe) = param_2[0x1];
  puVar1 = mixed_1010_20ba((astruct_57 *)(in_EDX & 0xffff0000 | (u32)param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000fe9e,in_stack_0000ffc2,in_stack_0000ffc8,
                           in_stack_0000ffcc);
  pass1_1010_5fb0((u32)puVar1,0x0,(u32 *)((int)param_1 + 0xa),param_1,((int)param_1 + 0x12));
  return;
}



u16 * pass1_1018_4c78(u16 *param_1,u8 param_2)

{
  pass1_1010_1d80((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void struct_op_1018_4cda(astruct_19 *param_1,u16 param_2)

{
  struct_op_1010_1d48(param_1,param_2);
  (u32)((int)param_1 + 0xa) = 0x0;
  (u32)((int)param_1 + 0xe) = 0x0;
  ((int)param_1 + 0x12) = 0x0;
  ((int)param_1 + 0x14) = 0x0;
  ((int)param_1 + 0x16) = 0x0;
  ((int)param_1 + 0x18) = 0x1;
  ((int)param_1 + 0x1a) = 0x0;
  param_1->offset_0x0 = (int)s_SCInternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12;
  ((int)param_1 + 0x2) = 0x1018;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void clenaup_win_ui_1018_4d22(StructD *in_struct_1)

{
  HPALETTE16 obj;
  StructD *local_struct_1;
  StructD *uVar4;
  u16 uVar3;
  u16 unaff_SS;
  u16 uVar1;
  u16 uVar2;
  u32 *puVar2;
  u32 *puVar1;
  code **fn_ptr_1;

  uVar3 = 0x1018;
  uVar4 = (StructD *)((u32)in_struct_1 >> 0x10);
  local_struct_1 = (StructD *)in_struct_1;
    // just 0x5058
  in_struct_1->address_offset_field_0x0 = (int)s_SCInternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12;
  local_struct_1->address_offset_field_0x2 = 0x1018;
  if (local_struct_1->field11_0x12 != 0x0) {
    obj = SelectPalette16(0x0,local_struct_1->field14_0x1a,local_struct_1->field11_0x12);
    DeleteObject16(obj);
    uVar3 = SUB42(s_tile2_bmp_1050_1538,0x0);
    DeleteDC16(local_struct_1->field11_0x12);
  }
  puVar1 = (u32 *)local_struct_1->field6_0xa;
  uVar1 = local_struct_1->field7_0xc;
  if ((uVar1 | puVar1) != 0x0) {
    fn_ptr_1 = (code **)*puVar1;
    (**fn_ptr_1)(uVar3,puVar1,uVar1,0x1);
  }
  puVar2 = (u32 *)local_struct_1->field8_0xe;
  uVar2 = &local_struct_1->field_0x10;
  if ((uVar2 | puVar2) != 0x0) {
    fn_ptr_1 = (code **)*puVar2;
    (**fn_ptr_1)(uVar3,puVar2,uVar2,0x1);
  }
  _PTR_LOOP_1050_4230 = 0x0;
  pass1_1010_1d80(in_struct_1);
  return;
}



void get_dc_1018_4db0(astruct_126 *param_1,u16 param_2)

{
  HDC16 HVar1;
  u16 uVar2;

  uVar2 = ((u32)param_1 >> 0x10);
  ((int)param_1 + 0x16) = param_2;
  HVar1 = GetDC16(param_2);
  *(HDC16 *)((int)param_1 + 0x14) = HVar1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_4dce(u8 *param_1,astruct_19 *param_2,u16 param_3)

{
  code **ppcVar1;
  u16 uVar2;
  u16 in_register_0000000a;
  u32 *puVar3;
  u16 in_stack_0000fe8a;
  u16 in_stack_0000ffae;
  u16 in_stack_0000ffb4;
  u16 in_stack_0000ffb8;
  u32 in_stack_0000ffe0;
  u8 **ppuVar4;

  ppuVar4 = (u8 **)CONCAT22((int)((u32)in_stack_0000ffe0 >> 0x10),0x48);
  puVar3 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,ppuVar4,in_stack_0000fe8a
                           ,in_stack_0000ffae,in_stack_0000ffb4,in_stack_0000ffb8);
  uVar2 = ((u32)puVar3 >> 0x10);
  ppcVar1 = (code **)((int)(u32)param_2 + 0x10);
  (**ppcVar1)(0x1010,param_2,param_3,((int)puVar3 + 0xc),((int)puVar3 + 0xa),
              (int)((u32)ppuVar4 >> 0x10));
  return;
}



void create_dc_1018_4e04(u16 in_string_6,u16 in_string_5,astruct_8 *param_3,u16 param_4,i16 param_5,i16 param_6)

{
  HDC16 HVar1;
  HDC16 *pHVar2;
  astruct_8 *iVar4;
  u16 uVar3;
  DEVMODEA *devmodea_init_data;
  i16 iStack16;
  code **fn_ptr_1;

  uVar3 = ((u32)param_3 >> 0x10);
  iVar4 = (astruct_8 *)param_3;
  fn_ptr_1 = (code **)((int)(u32)param_3 + 0x14);
  (**fn_ptr_1)();
  devmodea_init_data = (DEVMODEA *)pass1_1008_4772(iVar4->field10_0xa);
  pass1_1008_43cc(iVar4->field10_0xa);
  HVar1 = CreateDC16(devmodea_init_data,NULL,NULL,s_dib_1050_4234);
  iVar4->field15_0x12 = HVar1;
  pHVar2 = &iVar4->field15_0x12;
  fn_ptr_1 = (code **)((int)(u32)iVar4->field10_0xa + 0x8);
  (**fn_ptr_1)();
  iVar4->field22_0x1a = (int)pHVar2;
  if ((DAT_1050_422e != 0x0) && (0x280 < param_6)) {
    for (iStack16 = 0x0; iStack16 < DAT_1050_4216 + 0x1; iStack16 += 0x1) {
      ((int)&u16_1050_4172 + iStack16 * 0x2) =
           (int)(((long)((int)&u16_1050_4172 + iStack16 * 0x2) * ((long)param_6 + 0x1)) / 0x280);
    }
    for (iStack16 = 0x0; iStack16 < DAT_1050_422c + 0x1; iStack16 += 0x1) {
      ((int)&u16_1050_419a + iStack16 * 0x2) =
           (int)(((long)((int)&u16_1050_419a + iStack16 * 0x2) * ((long)param_5 + 0x1)) / 0x1e0);
    }
  }
  DAT_1050_422e = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_win_ui_op_1018_4f18(u32 param_1,astruct_57 *param_2,astruct_39 *param_3,u16 param_4)

{
  code **ppcVar1;
  u16 uVar2;
  astruct_394 *paVar3;
  RECT16 *rect;
  i16 iVar4;
  u16 uVar6;
  u16 uVar7;
  astruct_57 *paVar9;
  astruct_39 *iVar6;
  u16 uVar10;
  u8 uVar11;
  u16 unaff_CS;
  HWND16 HVar12;
  RECT16 local_12;
  i16 iStack14;
  i16 iStack12;
  u32 uStack10;
  u32 *puStack6;
  u32 uVar5;
  astruct_57 *paVar8;

  uVar2 = FUN_1010_830a((int)param_1,param_2,unaff_CS,_u16_1050_14cc,param_4);
  uVar5 = (u32)uVar2;
  uVar6 = SUB42(param_2,0x0);
  puStack6 = (u32 *)CONCAT22(uVar6,uVar2);
  ppcVar1 = (code **)((int)*puStack6 + 0x14);
  (**ppcVar1)();
  paVar3 = (astruct_394 *)uVar5;
  uStack10 = uVar5 & 0xffff | (long)param_2 << 0x10;
  uVar10 = ((u32)param_3 >> 0x10);
  iVar6 = (astruct_39 *)param_3;
  if (*(i32 *)&iVar6->field12_0xe != 0x0) {
    uVar7 = iVar6->field13_0x10;
    paVar3 = (astruct_394 *)(u32)&iVar6->field12_0xe;
    param_2 = (astruct_57 *)((u32)param_2 & 0xffff0000 | (u32)(uVar7 | paVar3));
    if ((uVar7 | paVar3) != 0x0) {
      ppcVar1 = (code **)(u32)paVar3;
      (**ppcVar1)(0x1010,paVar3,uVar7,0x1,uVar2,uVar6);
    }
  }
  mem_op_1000_179c(0x14,param_2);
  uVar7 = param_2 | paVar3;
  paVar9 = (astruct_57 *)((u32)param_2 & 0xffff0000);
  paVar8 = (astruct_57 *)((u32)paVar9 | (u32)uVar7);
  if (uVar7 == 0x0) {
    paVar3 = NULL;
  }
  else {
    struct_1008_4c58(paVar3);
    paVar9 = paVar8;
  }
  iVar6->field12_0xe = (u32 *)paVar3;
  iVar6->field13_0x10 = (u8 *)paVar9;
  pass1_1008_4d84((u8 *)paVar9,*(astruct_90 **)&iVar6->field12_0xe,uStack10);
  rect = &local_12;
  HVar12 = HWND16_1050_0396;
  GetClientRect16(rect,(HWND16)&DAT_1050_1050);
  uVar11 = 0x0;
  mem_op_1000_179c(0x1e,paVar9);
  uVar7 = paVar9 | rect;
  if (uVar7 == 0x0) {
    (u32)&iVar6->field10_0xa = 0x0;
  }
  else {
    iVar4 = (iStack12 - local_12.y) + 0x1;
    uVar11 = 0x8;
    pass1_1008_405c((astruct_76 *)CONCAT22(paVar9,rect),(u32)&iVar6->field12_0xe,iVar4,
                    (iStack14 - local_12.x) + 0x1);
    iVar6->field10_0xa = iVar4;
    iVar6->field11_0xc = uVar7;
  }
  if (puStack6 != NULL) {
    ppcVar1 = (code **)*puStack6;
    (**ppcVar1)(uVar11,(int)puStack6,(int)((u32)puStack6 >> 0x10),0x1,HVar12,uVar2,uVar6,puStack6,puStack6);
  }
  return;
}



StructD * pass1_1018_5032(StructD *param_1,u8 param_2)

{
  clenaup_win_ui_1018_4d22(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void pass1_1018_5070(astruct_19 *param_1,u16 param_2)

{
  struct_op_1010_1d48(param_1,param_2);
  (u32)((int)param_1 + 0xa) = 0x0;
  (u32)((int)param_1 + 0xe) = 0x0;
  (u32)((int)param_1 + 0x12) = 0x0;
  ((int)param_1 + 0x16) = 0x0;
  param_1->offset_0x0 = 0x56d2;
  ((int)param_1 + 0x2) = 0x1018;
  return;
}



void pass1_1018_50ac(u16 *param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;

  uVar5 = ((u32)param_1 >> 0x10);
  iVar4 = (int)param_1;
  *param_1 = 0x56d2;
  (iVar4 + 0x2) = 0x1018;
  puVar1 = (u32 *)(iVar4 + 0xe);
  uVar2 = (iVar4 + 0x10);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80((StructD *)param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_50ea(u32 param_1,u16 param_2,u32 param_3)

{
  i16 iVar1;
  code **ppcVar2;
  u32 uVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  i16 iVar8;
  u16 uVar9;
  u16 uVar10;
  astruct_99 *paStack6;

  paStack6 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
  uVar6 = ((u32)paStack6 >> 0x10);
  uVar4 = paStack6;
  if ((u8 *)(uVar6 | uVar4) == NULL) {
    paStack6 = NULL;
  }
  else {
    paStack6->field0_0x0 = 0x389a;
    (uVar4 + 0x2) = 0x1008;
    (uVar4 + 0x4) = 0x0;
    (uVar4 + 0x6) = 0x0;
    (uVar4 + 0x8) = 0x0;
    (uVar4 + 0xa) = 0x0;
    (uVar4 + 0xc) = 0x0;
    paStack6->field0_0x0 = 0x56ce;
    (uVar4 + 0x2) = 0x1018;
  }
  uVar9 = ((u32)paStack6 >> 0x10);
  uVar7 = paStack6;
  (uVar7 + 0xa) = param_2;
  uVar10 = (param_1 >> 0x10);
  iVar8 = (int)param_1;
  uVar3 = (u32)(iVar8 + 0xa);
  iVar1 = ((int)uVar3 + 0xc);
  if (iVar1 == 0x1) {
    uVar3 = (u32)(iVar8 + 0xa);
    uVar5 = ((int)uVar3 + 0xe);
    (uVar7 + 0x4) = uVar5;
  }
  else if (iVar1 == 0x5) {
    uVar3 = (u32)(iVar8 + 0xa);
    uVar5 = ((int)uVar3 + 0xe);
    (uVar7 + 0x6) = uVar5;
  }
  else {
    if (iVar1 != 0x6) {
      if ((uVar9 | uVar7) == 0x0) {
        return;
      }
      ppcVar2 = (code **)(u32)paStack6;
      (**ppcVar2)();
      return;
    }
    uVar3 = (u32)(iVar8 + 0xa);
    uVar5 = ((int)uVar3 + 0xe);
    (uVar7 + 0x8) = uVar5;
  }
  pass1_1030_6c66(uVar5,(u8 *)(uVar6 | uVar4),(astruct_386 *)param_3,0x1,(astruct_385 *)paStack6);
  return;
}



void pass1_1018_51d2(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar1 = (u32 *)(iVar4 + 0xe);
  uVar2 = (iVar4 + 0x10);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (u32)(iVar4 + 0xe) = 0x0;
  return;
}



u32 pass1_1018_5206(u32 param_1,char *param_2)

{
  i16 iVar1;
  u16 uVar2;
  i16 iVar3;
  u16 uVar4;
  u32 uVar5;
  char local_a [0x8];

  uVar4 = (param_1 >> 0x10);
  iVar3 = (int)param_1;
  (u32)(iVar3 + 0xa) = 0x0;
  pass1_1008_5784((char *)CONCAT22(0x1050,local_a),(u32)(iVar3 + 0xe));
  do {
    uVar5 = pass1_1008_5b12((char *)CONCAT22(0x1050,local_a));
    uVar2 = ((u32)uVar5 >> 0x10);
    (iVar3 + 0xa) = (int)uVar5;
    (iVar3 + 0xc) = uVar2;
    if ((uVar2 | (iVar3 + 0xa)) == 0x0) break;
    uVar5 = (u32)(iVar3 + 0xa);
    iVar1 = pass1_1000_3d7a(*(char **)((int)uVar5 + 0x4),param_2);
  } while (iVar1 != 0x0);
  return CONCAT22((iVar3 + 0xc),(iVar3 + 0xa));
}



u32 pass1_1018_526a(u32 param_1,u32 param_2)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = (param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(i32 *)(iVar1 + 0xe) == 0x0) {
    pass1_1018_5292((astruct_9 *)(param_1 & 0xffff | (u32)uVar2 << 0x10),param_2);
  }
  return CONCAT22((iVar1 + 0x10),(iVar1 + 0xe));
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_5292(astruct_9 *param_1,u32 param_2)

{
  u32 uVar1;
  u32 uVar3;
  u16 uVar11;
  u16 uVar4;
  BOOL16 BVar4;
  char *puVar5;
  astruct_11 *paVar5;
  char *pcVar6;
  u16 uVar7;
  u16 uVar8;
  char *pcVar9;
  u16 uVar10;
  u32 uVar9;
  u16 uVar12;
  u32 in_EDX;
  astruct_57 *paVar13;
  astruct_57 *paVar14;
  astruct_9 *pstruct9_v18;
  astruct_9 *pstruct9_v16;
  u16 uVar16;
  u16 uVar17;
  astruct_203 *paVar18;
  u16 *puVar19;
  u16 uStack50;
  char local_26 [0x8];
  u32 uStack30;
  u32 uStack26;
  u32 uStack22;
  u16 puStack18;
  StructD *pSStack16;
  u16 puStack14;
  StructD *pSStack12;
  u16 uStack10;
  u32 uStack8;
  u16 uStack4;
  u32 uVar2;
  astruct_11 *iVar1;
  astruct_57 *paVar15;
  code **fn_ptr_2;

  pstruct9_v16 = (astruct_9 *)((u32)param_1 >> 0x10);
  pstruct9_v18 = (astruct_9 *)param_1;
  puStack18 = pstruct9_v18->field14_0xe;
  uVar9 = (u32)puStack18;
  pSStack16 = pstruct9_v18->field15_0x10;
  paVar13 = (astruct_57 *)(in_EDX & 0xffff0000 | ZEXT24(pSStack16));
  puStack14 = puStack18;
  pSStack12 = pSStack16;
  if ((pSStack16 | puStack18) != 0x0) {
    fn_ptr_2 = (code **)(u32)puStack18;
    (**fn_ptr_2)();
  }
  mem_op_1000_179c(0xc,paVar13);
  puStack18 = uVar9;
  pSStack16 = (StructD *)paVar13;
  paVar15 = (astruct_57 *)((u32)paVar13 & 0xffff0000);
  paVar14 = (astruct_57 *)((u32)paVar15 | (u32)(pSStack16 | puStack18));
  if ((pSStack16 | puStack18) == 0x0) {
    uVar9 = 0x0;
  }
  else {
    set_struct_1008_574a((astruct_57 *)(uVar9 & 0xffff | (long)paVar13 << 0x10));
    paVar15 = paVar14;
  }
  pstruct9_v18->field14_0xe = uVar9;
  pstruct9_v18->field15_0x10 = (StructD *)paVar15;
  uStack4 = 0x21;
  while( true ) {
    if ((int)uStack4 < 0x0) break;
    uStack22 = pass1_1030_7c28(uVar9,paVar15,param_2,uStack4);
    uVar9 = uStack22 & 0xffff;
    uVar11 = uVar9;
    uVar12 = (uStack22 >> 0x10) | uVar11;
    paVar15 = (astruct_57 *)((u32)paVar15 & 0xffff0000 | (u32)uVar12);
    if (uVar12 != 0x0) {
      string_1020_c0ca(uStack4);
      uVar4 = str_op_1008_60e8(paVar15,(char *)CONCAT22(paVar15,uVar11));
      uVar9 = (u32)uVar4;
      uStack26 = CONCAT22((int)paVar15,uVar4);
      mem_op_1000_179c(0x10,paVar15);
      puStack14 = uVar9;
      pSStack12 = (StructD *)paVar15;
      paVar13 = (astruct_57 *)((u32)paVar15 & 0xffff0000 | (u32)(pSStack12 | puStack14));
      if ((pSStack12 | puStack14) == 0x0) {
        uVar9 = 0x0;
        paVar15 = (astruct_57 *)((u32)paVar15 & 0xffff0000);
      }
      else {
        struct_1018_4790(uVar9 & 0xffff | (long)paVar15 << 0x10,uStack22,uStack26,uStack4);
        paVar15 = paVar13;
      }
      uStack30 = uVar9 & 0xffff | (long)paVar15 << 0x10;
      uVar2 = (u32)&pstruct9_v18->field14_0xe;
      fn_ptr_2 = (code **)((int)*(u32*)&pstruct9_v18->field14_0xe + 0x4);
      (**fn_ptr_2)(0x0,(int)uVar2,(int)((u32)uVar2 >> 0x10),(int)uVar9,(int)paVar15);
    }
    uStack4 -= 0x1;
  }
  uStack8 = struct_op_1030_73a8((astruct_419 *)param_2,uVar9,paVar15);
  paVar13 = (astruct_57 *)((u32)paVar15 & 0xffff0000 | uStack8 >> 0x10);
  uStack10 = ((int)uStack8 + 0xc);
  BVar4 = pass1_1008_c6ae(_u16_1050_06e0,uStack10,0x4);
  if (BVar4 != 0x0) {
    uStack30 = uStack8;
    uStack26 = (u32)((int)uStack8 + 0x20);
    pass1_1008_5784((char *)CONCAT22(0x1050,local_26),uStack26);
    while( true ) {
      puVar5 = local_26;
      pass1_1008_5b12((char *)CONCAT22(0x1050,puVar5));
      uVar12 = paVar13;
      uStack22 = CONCAT22(uVar12,puVar5);
      paVar13 = (astruct_57 *)((u32)paVar13 & 0xffff0000 | (u32)(uVar12 | puVar5));
      if ((uVar12 | puVar5) == 0x0) break;
      iVar1 = *(astruct_11 **)(puVar5 + 0x6);
      paVar5 = iVar1 + -0x7;
      if (paVar5 == NULL) {
LAB_1018_53f0:
        pcVar6 = string_op_1020_c222((puVar5 + 0x6));
        uVar7 = str_op_1008_60e8(paVar13,(char *)CONCAT22(paVar13,pcVar6));
        uVar17 = SUB42(paVar13,0x0);
        uVar8 = uVar7;
        mem_op_1000_179c(0x10,paVar13);
        pSStack16 = (StructD *)paVar13;
        paVar13 = (astruct_57 *)((u32)paVar13 & 0xffff0000);
        puStack18 = uVar8;
        if ((pSStack16 | uVar8) == 0x0) {
          uVar17 = 0x0;
        }
        else {
          uVar16 = (uStack22 >> 0x10);
          paVar18 = struct_1018_48b0((astruct_203 *)CONCAT22(pSStack16,uVar8),(u32)((int)uStack22 + 0xa),
                                     CONCAT22(uVar17,uVar7),((int)uStack22 + 0x6));
          paVar13 = (astruct_57 *)((u32)paVar13 & 0xffff0000 | (u32)paVar18 >> 0x10);
          uVar17 = SUB42(paVar18,0x0);
        }
        uVar1 = (u32)&pstruct9_v18->field14_0xe;
        fn_ptr_2 = (code **)((int)*(u32*)&pstruct9_v18->field14_0xe + 0x4);
        (**fn_ptr_2)(0x0,(int)uVar1,(int)((u32)uVar1 >> 0x10),uVar17,(int)paVar13);
      }
      else if (((0x5 < (int)paVar5) && (!SBORROW2((int)paVar5,0x6))) && ((int)(iVar1 + -0xd) < 0x2)) goto LAB_1018_53f0;
      uVar17 = (uStack22 >> 0x10);
      if (((int)uStack22 + 0x8) != 0x0) {
        pcVar6 = string_op_1020_c2f8(((int)uStack22 + 0x8));
        uVar7 = str_op_1008_60e8(paVar13,(char *)CONCAT22(paVar13,pcVar6));
        uVar17 = SUB42(paVar13,0x0);
        uVar8 = uVar7;
        mem_op_1000_179c(0x10,paVar13);
        pSStack12 = (StructD *)paVar13;
        paVar13 = (astruct_57 *)((u32)paVar13 & 0xffff0000);
        puStack14 = uVar8;
        if ((pSStack12 | uVar8) == 0x0) {
          uVar17 = 0x0;
        }
        else {
          uVar16 = (uStack22 >> 0x10);
          puVar19 = struct_1018_48e8((astruct_203 *)CONCAT22(pSStack12,uVar8),(u32)((int)uStack22 + 0xa),
                                     CONCAT22(uVar17,uVar7),((int)uStack22 + 0x8));
          paVar13 = (astruct_57 *)((u32)paVar13 & 0xffff0000 | (u32)puVar19 >> 0x10);
          uVar17 = SUB42(puVar19,0x0);
        }
        uVar1 = (u32)&pstruct9_v18->field14_0xe;
        fn_ptr_2 = (code **)((int)*(u32*)&pstruct9_v18->field14_0xe + 0x4);
        (**fn_ptr_2)(0x0,(int)uVar1,(int)((u32)uVar1 >> 0x10),uVar17,(int)paVar13);
      }
    }
  }
  uVar17 = (param_2 >> 0x10);
  uVar3 = (u32)((int)param_2 + 0x3e);
  uVar12 = ((int)param_2 + 0x40);
  paVar13 = (astruct_57 *)((u32)paVar13 & 0xffff0000 | (u32)uVar12);
  uStack50 = uVar3;
  if ((uVar12 | uStack50) != 0x0) {
    pass1_1008_5784((char *)CONCAT22(0x1050,local_26),uVar3 & 0xffff | (u32)uVar12 << 0x10);
    while( true ) {
      pcVar6 = local_26;
      pass1_1008_5b12((char *)CONCAT22(0x1050,pcVar6));
      uVar12 = paVar13;
      paVar13 = (astruct_57 *)((u32)paVar13 & 0xffff0000 | (u32)(uVar12 | pcVar6));
      if ((uVar12 | pcVar6) == 0x0) break;
      if ((pcVar6 + 0x4) != 0x0) {
        pcVar9 = string_1020_c0d8((pcVar6 + 0x4));
        uVar10 = str_op_1008_60e8(paVar13,(char *)CONCAT22(paVar13,pcVar9));
        uStack30 = CONCAT22((int)paVar13,uVar10);
        mem_op_1000_179c(0x10,paVar13);
        pSStack16 = (StructD *)paVar13;
        paVar13 = (astruct_57 *)((u32)paVar13 & 0xffff0000);
        paVar15 = (astruct_57 *)((u32)paVar13 | (u32)(pSStack16 | uVar10));
        puStack18 = uVar10;
        if ((pSStack16 | uVar10) == 0x0) {
          uVar10 = 0x0;
        }
        else {
          struct_1018_4790(CONCAT22(pSStack16,uVar10),(u32)(pcVar6 + 0xa),uStack30,(pcVar6 + 0x4))
          ;
          paVar13 = paVar15;
        }
        uStack26 = CONCAT22((int)paVar13,uVar10);
        uVar1 = (u32)&pstruct9_v18->field14_0xe;
        fn_ptr_2 = (code **)((int)*(u32*)&pstruct9_v18->field14_0xe + 0x4);
        (**fn_ptr_2)(0x0,(int)uVar1,(int)((u32)uVar1 >> 0x10),uVar10,(int)paVar13);
      }
      if ((pcVar6 + 0x6) != 0x0) {
        pcVar9 = string_op_1020_c222((pcVar6 + 0x6));
        uVar10 = str_op_1008_60e8(paVar13,(char *)CONCAT22(paVar13,pcVar9));
        uStack30 = CONCAT22((int)paVar13,uVar10);
        mem_op_1000_179c(0x10,paVar13);
        pSStack12 = (StructD *)paVar13;
        paVar13 = (astruct_57 *)((u32)paVar13 & 0xffff0000);
        puStack14 = uVar10;
        if ((pSStack12 | uVar10) == 0x0) {
          uVar17 = 0x0;
        }
        else {
          paVar18 = struct_1018_48b0((astruct_203 *)CONCAT22(pSStack12,uVar10),(u32)(pcVar6 + 0xa),uStack30,
                                     (pcVar6 + 0x6));
          paVar13 = (astruct_57 *)((u32)paVar13 & 0xffff0000 | (u32)paVar18 >> 0x10);
          uVar17 = SUB42(paVar18,0x0);
        }
        uStack26 = CONCAT22((int)paVar13,uVar17);
        uVar1 = (u32)&pstruct9_v18->field14_0xe;
        fn_ptr_2 = (code **)((int)*(u32*)&pstruct9_v18->field14_0xe + 0x4);
        (**fn_ptr_2)(0x0,(int)uVar1,(int)((u32)uVar1 >> 0x10),uVar17,(int)paVar13);
      }
      if ((pcVar6 + 0x8) != 0x0) {
        pcVar9 = string_op_1020_c2f8((pcVar6 + 0x8));
        uVar10 = str_op_1008_60e8(paVar13,(char *)CONCAT22(paVar13,pcVar9));
        uStack30 = CONCAT22((int)paVar13,uVar10);
        mem_op_1000_179c(0x10,paVar13);
        pSStack16 = (StructD *)paVar13;
        paVar13 = (astruct_57 *)((u32)paVar13 & 0xffff0000);
        puStack18 = uVar10;
        if ((pSStack16 | uVar10) == 0x0) {
          uVar17 = 0x0;
        }
        else {
          puVar19 = struct_1018_48e8((astruct_203 *)CONCAT22(pSStack16,uVar10),(u32)(pcVar6 + 0xa),uStack30,
                                     (pcVar6 + 0x8));
          paVar13 = (astruct_57 *)((u32)paVar13 & 0xffff0000 | (u32)puVar19 >> 0x10);
          uVar17 = SUB42(puVar19,0x0);
        }
        uStack26 = CONCAT22((int)paVar13,uVar17);
        uVar1 = (u32)&pstruct9_v18->field14_0xe;
        fn_ptr_2 = (code **)((int)*(u32*)&pstruct9_v18->field14_0xe + 0x4);
        (**fn_ptr_2)(0x0,(int)uVar1,(int)((u32)uVar1 >> 0x10),uVar17,(int)paVar13);
      }
    }
  }
  return;
}



u16 * pass1_1018_567c(u16 *param_1,u8 param_2)

{
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    pass1_1000_093a((astruct_611 *)param_1);
  }
  return param_1;
}



u32 pass1_1018_56a8(u32 param_1,u8 param_2)

{
  pass1_1018_50ac((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1018_56e6(astruct_19 *param_1,u16 param_2)

{
  struct_op_1010_1d48(param_1,param_2);
  (u32)((int)param_1 + 0xa) = 0x0;
  param_1->offset_0x0 = 0x5830;
  ((int)param_1 + 0x2) = 0x1018;
  return &param_1->offset_0x0;
}



void pass1_1018_5714(u16 *param_1)

{
  *param_1 = 0x5830;
  ((int)param_1 + 0x2) = 0x1018;
  pass1_1010_1d80((StructD *)param_1);
  return;
}



u32 pass1_1018_5732(u16 param_1,u16 param_2,u16 param_3,u16 param_4,u32 param_5)

{
  u32 uVar1;

  uVar1 = pass1_1030_6d4e(param_1,param_2,param_5);
  return uVar1;
}



void pass1_1018_5742(u16 param_1,u16 param_2,u32 *param_3,astruct_299 *param_4)

{
  u32 *puVar1;
  code **ppcVar2;
  u32 uVar3;
  bool bVar4;
  u32 *puVar5;
  u32 uVar6;
  u16 extraout_DX;
  u16 extraout_DX_00;
  u32 uStack16;

  bVar4 = false;
  puVar1 = (u32 *)(u32)((int)param_3 + 0x4);
  ppcVar2 = (code **)((int)*puVar1 + 0x10);
  puVar5 = puVar1;
  (**ppcVar2)();
  uVar3 = (u32)puVar5 & 0xffff | (u32)extraout_DX << 0x10;
  uStack16 = 0x0;
  do {
    if (uVar3 <= uStack16) {
LAB_1018_579f:
      if (!bVar4) {
        if (param_3 != NULL) {
          ppcVar2 = (code **)*param_3;
          (**ppcVar2)();
        }
        param_3 = NULL;
      }
      pass1_1030_6d80(param_4,(u32)param_3);
      return;
    }
    ppcVar2 = (code **)((int)*puVar1 + 0x4);
    uVar6 = uVar3;
    (**ppcVar2)();
    if ((extraout_DX_00 | uVar6) != 0x0) {
      bVar4 = true;
      goto LAB_1018_579f;
    }
    uStack16 += 0x1;
  } while( true );
}



void pass1_1018_57d2(u32 param_1,u32 param_2)

{
  (u32)((int)param_1 + 0xa) = param_2;
  return;
}



void pass1_1018_57e6(u32 param_1,i32 param_2,u16 param_3,u16 param_4)

{
  u16 uVar1;

  uVar1 = (param_1 >> 0x10);
  send_dlg_item_msg_1040_d20c(param_3,param_4,*(astruct_929 **)((int)param_1 + 0xa),param_2);
  (u32)((int)param_1 + 0xa) = 0x0;
  return;
}



u16 * pass1_1018_580a(u16 *param_1,u8 param_2)

{
  pass1_1018_5714(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void struct_1018_5840(astruct_57 *param_1,astruct_20 *param_2,u16 param_3,u16 param_4,u16 param_5,
                     u16 param_6,u16 param_7,u16 param_8,u16 param_9)

{
  u16 uVar1;
  astruct_20 *iVar2;
  u16 unaff_BP;
  astruct_20 *uVar2;
  u32 *puVar2;

  unk_draw_op_1020_7f7a(param_2,0x6,CONCAT22(param_4,param_3),param_5);
  uVar2 = (astruct_20 *)((u32)param_2 >> 0x10);
  iVar2 = (astruct_20 *)param_2;
  (u32)&iVar2[0x1].field5_0xc = 0x0;
  iVar2[0x1].field7_0x10 = NULL;
  iVar2[0x1].field8_0x14 = 0x0;
  param_2->offset_0x0 = (int)s_Alloc__s_1050_5a5b + 0x7;
  iVar2->base_0x2 = 0x1018;
  (iVar2 + 0x1)->offset_0x0 = 0x5afe;
  iVar2[0x1].base_0x2 = 0x1018;
  puVar2 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x27),param_6,param_7,param_8,param_9)
  ;
  uVar1 = ((u32)puVar2 >> 0x10);
  &iVar2[0x1].field7_0x10 = (int)puVar2;
  ((int)&iVar2[0x1].field7_0x10 + 0x2) = uVar1;
  &iVar2[0x1].field2_0x4 = &iVar2[0x1].field7_0x10;
  ((int)&iVar2[0x1].field2_0x4 + 0x2) = uVar1;
  return;
}



void pass1_1018_58b6(u16 *param_1)

{
  i16 iVar1;
  u16 uVar2;

  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *param_1 = (int)s_Alloc__s_1050_5a5b + 0x7;
  (iVar1 + 0x2) = 0x1018;
  (iVar1 + 0xe2) = 0x5afe;
  (iVar1 + 0xe4) = 0x1018;
  pass1_1020_808e((StructD *)param_1);
  return;
}



void invalidate_rect_1018_58e2(astruct_58 *param_1,i16 param_2)

{
  i16 *piVar1;
  astruct_58 *iVar2;
  u16 uVar2;

  if (param_2 == 0x105) {
    uVar2 = ((u32)param_1 >> 0x10);
    iVar2 = (astruct_58 *)param_1;
    piVar1 = &iVar2->field245_0xf6;
    *piVar1 = *piVar1 + 0x1;
    if ((int)u16_1050_4240 <= iVar2->field245_0xf6) {
      PostMessage16(0x0,0xca,0x111,HWND16_1050_0396);
      return;
    }
    iVar2->field234_0xea = 0x0;
    InvalidateRect16(0x0,NULL,0x0);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1018_5932(u16 param_1,u32 param_2)

{
  code **ppcVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;
  u32 uVar5;

  uVar4 = (param_2 >> 0x10);
  uVar3 = param_2;
  uVar2 = (uVar3 + 0xf0) | (uVar3 + 0xee);
  if (uVar2 != 0x0) {
    ppcVar1 = (code **)((int)(u32)(u32)(uVar3 + 0xee) + 0x8);
    uVar5 = (**ppcVar1)();
    param_1 = ((u32)uVar5 >> 0x10);
    uVar2 = uVar5;
  }
  if ((uVar3 + 0xea) == 0x0) {
    (uVar3 + 0xea) = 0x1;
    uVar5 = pass1_1038_af40(uVar3,param_1,_PTR_LOOP_1050_5b7c,(uVar3 + 0x8),
                            ((uVar3 + 0xf6) * 0x2 + 0x4238));
    uVar2 = uVar5;
  }
  return uVar2;
}



void win_1018_598c(u16 param_1,astruct_57 *param_2,StructA *struct_param_1,u16 param_4,u16 param_5)

{
  u16 uVar1;
  StructA *struct_1;
  u16 uVar3;
  u16 in_stack_0000fe68;
  u16 in_stack_0000ff8c;
  u16 in_stack_0000ff92;
  u16 in_stack_0000ff96;
  u32 uVar2;

  create_window_ex_1008_9760(struct_param_1);
  uVar3 = ((u32)struct_param_1 >> 0x10);
  struct_1 = (StructA *)struct_param_1;
  get_dc_1018_4db0(*(astruct_126 **)&struct_1[0x1].field20_0x26,struct_1->field4_0x8);
  mem_op_1000_179c(0x2a,param_2);
  uVar1 = param_2 | param_1;
  uVar2 = (u32)param_2 & 0xffff0000 | (u32)uVar1;
  if (uVar1 != 0x0) {
    pass1_1018_5b06(uVar2,(StructA *)CONCAT22(param_2,param_1),struct_1->field4_0x8,param_4,param_5,
                    in_stack_0000fe68,in_stack_0000ff8c,in_stack_0000ff92,in_stack_0000ff96);
    struct_1[0x1].field18_0x22 = (astruct_666 *)param_1;
    struct_1[0x1].field19_0x24 = uVar2;
    return;
  }
  (u32)&struct_1[0x1].field18_0x22 = 0x0;
  return;
}



void FUN_1018_59f0(u16 param_1,u32 param_2)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;

  uVar5 = (param_2 >> 0x10);
  iVar4 = (int)param_2;
  puVar1 = (u32 *)(iVar4 + 0xee);
  uVar2 = (iVar4 + 0xf0);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (u32)(iVar4 + 0xee) = 0x0;
  destroy_win_1008_628e(param_2 & 0xffff | (u32)uVar5 << 0x10);
  return;
}



StructD * pass1_1018_5a2e(StructD *param_1,u8 param_2)

{
  param_1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0xe2));
  pass1_1018_58b6((u16 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * FUN_1018_5a3c(u16 param_1,StructD *param_2,u8 param_3)

{
  pass1_1018_58b6(&param_2->address_offset_field_0x0);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



// WARNING: Unable to use type for symbol uVar4
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_5b06(u32 param_1,StructA *param_2,u16 param_3,u16 param_4,u16 param_5,
                    u16 param_6,u16 param_7,u16 param_8,u16 param_9)

{
  astruct_76 *paVar1;
  code **ppcVar2;
  u32 uVar3;
  i16 iVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  u16 uVar10;
  astruct_57 *paVar8;
  u32 uVar9;
  u16 unaff_SI;
  u16 *puVar11;
  u32 *puVar12;
  astruct_76 *paVar13;
  u32 uVar14;
  u8 local_c [0x6];
  u32 *puStack6;
  u16 uVar4;

  set_struct_op_1020_921c(param_1,param_2,param_3,CONCAT22(param_5,param_4));
  uVar10 = ((u32)param_1 >> 0x10);
  (u32)((int)param_2 + 0x14) = 0x0;
  (u32)((int)param_2 + 0x18) = 0x0;
  puVar11 = pass1_1008_3e38((astruct_19 *)((u32)param_2 & 0xffff0000 | (u32)((int)param_2 + 0x1c)));
  paVar8 = (astruct_57 *)CONCAT22(uVar10,(int)((u32)puVar11 >> 0x10));
  param_2->field0_0x0 = &u16_1050_5e1a;
  ((int)param_2 + 0x2) = 0x1018;
  puStack6 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x48),param_6,param_7,param_8,param_9
                            );
  uVar9 = (u32)paVar8 & 0xffff0000;
  puVar11 = pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_c));
  paVar8 = (astruct_57 *)(uVar9 & 0xffff0000 | (u32)puVar11 >> 0x10);
  pass1_1008_3f62((u16 *)CONCAT22(0x1050,local_c),
                  (u16 *)((u32)puStack6 & 0xffff0000 | (u32)((int)puStack6 + 0xe)));
  puVar12 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x27),param_6,param_7,param_8,param_9)
  ;
  uVar10 = ((u32)paVar8 >> 0x10);
  ((int)param_2 + 0x14) = (int)puVar12;
  ((int)param_2 + 0x16) = (int)((u32)puVar12 >> 0x10);
  ppcVar2 = (code **)((int)(u32)(u32)((int)param_2 + 0x14) + 0x4);
  (**ppcVar2)();
  (u32)((int)param_2 + 0x6) = (u32)((int)param_2 + 0x14);
  uVar3 = (u32)((int)param_2 + 0x14);
  paVar1 = *(astruct_76 **)((int)uVar3 + 0xa);
  iVar4 = (int)param_2 + 0xa;
  ppcVar2 = (code **)((int)(u32)paVar1 + 0x8);
  (**ppcVar2)();
  ((int)param_2 + 0x12) = iVar4;
  draw_op_1020_9364((StructA *)((u32)param_2 & 0xffff | (u32)param_2 << 0x10));
  uVar9 = (u32)((int)param_2 + 0x14);
  pass1_1008_3f62((u16 *)((u32)param_2 & 0xffff0000 | (u32)((int)param_2 + 0x1c)),
                  (u16 *)(uVar9 & 0xffff0000 | (u32)((int)uVar9 + 0x52)));
  pass1_1008_3f32((i16 *)((u32)param_2 & 0xffff0000 | (u32)((int)param_2 + 0x1cU)),(i16 *)CONCAT22(0x1050,local_c));
  paVar13 = pass1_1008_9f48(*(astruct_134 **)((int)param_2 + 0x14));
  uVar14 = pass1_1008_4772(paVar13);
  uVar6 = (uVar14 >> 0x10);
  paVar8 = (astruct_57 *)CONCAT22(uVar10,uVar6);
  uVar4 = uVar14;
  uVar5 = uVar4;
  mem_op_1000_179c(0x14,paVar8);
  uVar7 = paVar8 | uVar5;
  if (uVar7 == 0x0) {
    (u32)((int)param_2 + 0x18) = 0x0;
  }
  else {
    pass1_1008_50c2((astruct_110 *)CONCAT22(paVar8,uVar5),(u32)(uVar4 + 0x8),(u32)(uVar4 + 0x4),
                    (u16 *)((u32)param_2 & 0xffff0000 | (u32)((int)param_2 + 0x1cU)),paVar1);
    ((int)param_2 + 0x18) = uVar5;
    ((int)param_2 + 0x1a) = uVar7;
  }
  pass1_1008_5134((u32)((int)param_2 + 0x18));
  ((int)param_2 + 0x22) = ((int)param_2 + 0x1c);
  ((int)param_2 + 0x24) = ((int)param_2 + 0x1e);
  ((int)param_2 + 0x26) = (uVar4 + 0x4) + ((int)param_2 + 0x22) + 0x1;
  ((int)param_2 + 0x28) = (uVar4 + 0x8) + ((int)param_2 + 0x24) + 0x1;
  return;
}



void pass1_1018_5cc8(StructD *param_1)

{
  u16 uVar1;
  char *pcVar2;
  StructD *iVar3;
  u16 uVar3;

  uVar3 = ((u32)param_1 >> 0x10);
  iVar3 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = &u16_1050_5e1a;
  iVar3->address_offset_field_0x2 = 0x1018;
  pcVar2 = *(char **)&iVar3->field13_0x18;
  uVar1 = iVar3->field14_0x1a;
  if ((uVar1 | pcVar2) != 0x0) {
    pass1_1008_5118((u32)pcVar2 & 0xffff | (u32)uVar1 << 0x10);
    fn_ptr_1000_17ce(pcVar2);
  }
  if (iVar3->field12_0x14 != 0x0) {
    pass1_1010_1ea6(iVar3->field12_0x14,(StructD *)((u32)param_1 & 0xffff | (u32)uVar3 << 0x10));
    pass1_1010_1dda(iVar3->field12_0x14);
  }
  palette_op_1020_92c4(param_1);
  return;
}



void invalidate_rect_1018_5d32(u32 param_1,i16 param_2)

{
  HWND16 hwnd;

  hwnd = (HWND16)(param_1 >> 0x10);
  if (param_2 == 0x1) {
    (u32)((int)param_1 + 0x14) = 0x0;
    return;
  }
  if (param_2 != 0x2) {
    return;
  }
  InvalidateRect16(0x0,(RECT16 *)((int)param_1 + 0x22),hwnd);
  return;
}



// WARNING: Unable to use type for symbol uVar4

void misc_draw_op_1018_5d6c(astruct_839 *param_1)

{
  astruct_76 *paVar1;
  astruct_839 *struct_4;
  u16 uVar5;
  astruct_76 *paVar2;
  PAINTSTRUCT16 local_22;
  u32 *puVar1;
  astruct_134 *uVar4;
  code **fn_ptr_1;

  uVar5 = ((u32)param_1 >> 0x10);
  struct_4 = (astruct_839 *)param_1;
  BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&local_22),struct_4->field4_0x4);
  uVar4 = struct_4->pstruct134_0x14;
  paVar1 = *(astruct_76 **)((int)uVar4 + 0xa);
  paVar2 = pass1_1008_9f48(struct_4->pstruct134_0x14);
  pass1_1008_5236(struct_4->field20_0x18);
  pass1_1008_4480(paVar1,(u16 *)((u32)param_1 & 0xffff0000 | ZEXT24(struct_4 + 0x1)),paVar2);
  fn_ptr_1 = (code **)((int)(u32)paVar1 + 0x4);
  (**fn_ptr_1)(0x1008,(int)paVar1,(int)((u32)paVar1 >> 0x10),0x0,
               (u32)param_1 & 0xffff0000 | ZEXT24(&struct_4->field_0xa));
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&local_22),struct_4->field4_0x4);
  return;
}



u16 * pass1_1018_5df4(u16 *param_1,u8 param_2)

{
  pass1_1018_5cc8((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_57 * pass1_1018_5e26(astruct_57 *param_1,u16 param_2)

{
  astruct_57 *uVar1;

  get_sys_metrics_1040_7728(param_1,0x1,0x0,0xfd0,param_2);
  uVar1 = (astruct_57 *)((u32)param_1 >> 0x10);
  param_1->field0_0x0 = 0x6128;
  ((int)param_1 + 0x2) = 0x1018;
  ((int)param_1 + 0x74) = 0x1;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_5e5a(u16 *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  *param_1 = 0x6128;
  ((int)param_1 + 0x2) = 0x1018;
  pass1_1038_b6e0(_PTR_LOOP_1050_5b7c,((int)param_1 + 0x6));
  ui_cleanup_op_1040_782c((StructD *)param_1);
  return;
}



void pass1_1018_5e86(u32 *param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)*param_1 + 0x6c);
  (**ppcVar1)();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void win_ui_op_1018_5e9a(u16 param_1,StructB *structb_param_1)

{
  char **ppcVar1;
  LPVOID pvVar2;
  INT16 IVar3;
  astruct_92 **ppaVar4;
  u8 *puVar5;
  u16 uVar6;
  u16 in_register_0000000a;
  astruct_57 *paVar7;
  u32 uVar9;
  StructB *structb_9;
  i16 iVar10;
  u16 uVar11;
  u16 uVar12;
  u32 *puVar13;
  u16 in_stack_0000fe5a;
  u16 in_stack_0000ff7e;
  u16 in_stack_0000ff84;
  u16 in_stack_0000ff88;
  u16 in_stack_0000ffb2;
  astruct_92 *local_28;
  i16 iStack22;
  u16 uStack20;
  i16 iStack18;
  i16 iStack16;
  RECT16 local_e;
  i16 iStack8;
  INT16 *pIStack6;
  astruct_57 *paVar8;

  paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  dialog_ui_fn_1040_78e2(structb_param_1);
  puVar13 = mixed_1010_20ba(paVar7,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffb2,0x39),in_stack_0000fe5a,
                            in_stack_0000ff7e,in_stack_0000ff84,in_stack_0000ff88);
  paVar7 = (astruct_57 *)((u32)paVar7 & 0xffff0000 | (u32)puVar13 >> 0x10);
  pvVar2 = (LPVOID)puVar13;
  uVar11 = ((u32)structb_param_1 >> 0x10);
  structb_9 = (StructB *)structb_param_1;
  structb_9[0x7].field1_0x2 = pvVar2;
  structb_9[0x7].hwnd_0x6 = (HWND16)((u32)puVar13 >> 0x10);
  mem_op_1000_179c(0x12,paVar7);
  puVar5 = (u8 *)(paVar7 | pvVar2);
  paVar8 = (astruct_57 *)((u32)paVar7 & 0xffff0000 | ZEXT24(puVar5));
  if (puVar5 == NULL) {
    (u32)&structb_9[0x7].lpvoid_field_0x8 = 0x0;
  }
  else {
    pass1_1018_6198(puVar5,(astruct_657 *)CONCAT22(paVar7,pvVar2),structb_param_1,
                    structb_9->lpvoid_field_0x8);
    structb_9[0x7].lpvoid_field_0x8 = pvVar2;
    structb_9[0x7].max_count_field_0x10 = paVar8;
  }
  uVar9 = (u32)&structb_9[0x7].field1_0x2;
  pIStack6 = (INT16 *)(uVar9 & 0xffff0000 | (u32)((int)uVar9 + 0xa));
  GetClientRect16(&local_e,(HWND16)&DAT_1050_1050);
  IVar3 = GetSystemMetrics16(SM_CYCAPTION);
  ((int)pIStack6 + 0x6) = IVar3 + iStack8;
  puVar13 = mixed_1010_20ba(paVar8,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffb2,0x48),in_stack_0000fe5a,
                            in_stack_0000ff7e,in_stack_0000ff84,in_stack_0000ff88);
  uStack20 = ((u32)puVar13 >> 0x10);
  iStack22 = (int)puVar13;
  iStack16 = (iStack22 + 0xa);
  iStack18 = (iStack22 + 0xc);
  uVar12 = ((u32)pIStack6 >> 0x10);
  ((int)pIStack6 + 0x2) = (iStack18 - ((int)pIStack6 + 0x6)) / 0x2;
  uVar9 = (u32)(iStack16 >> 0xf);
  *pIStack6 = iStack16 / 0x2 + 0x3;
  pass1_1028_dc52((astruct_92 *)CONCAT22(0x1050,&local_28),0x1,0x0,0x100);
  while( true ) {
    uVar6 = uVar9;
    ppaVar4 = &local_28;
    pass1_1028_e4ec((astruct_92 *)CONCAT22(0x1050,ppaVar4));
    uVar9 = (u32)(uVar6 | ppaVar4);
    if ((uVar6 | ppaVar4) == 0x0) break;
    ppcVar1 = *(char ***)(ppaVar4 + 0x8);
    if (ppcVar1 != NULL) {
      pass1_1000_3cea((u32)structb_param_1 & 0xffff0000 | ZEXT24(&structb_9->field8_0x10),*ppcVar1);
    }
  }
  uVar12 = ((u32)pIStack6 >> 0x10);
  iVar10 = (int)pIStack6;
  SetWindowPos16(0x44,*(INT16 *)(iVar10 + 0x6),*(INT16 *)(iVar10 + 0x4),*(INT16 *)(iVar10 + 0x2),*pIStack6,0x0,
                 (HWND16)structb_9->lpvoid_field_0x8);
  return;
}



void pass1_1018_5ffa(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar1 = (u32 *)(iVar4 + 0x92);
  uVar2 = (iVar4 + 0x94);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (u32)(iVar4 + 0x92) = 0x0;
  pass1_1010_1dda((u32)(iVar4 + 0x8e));
  (u32)(iVar4 + 0x8e) = 0x0;
  return;
}



u16 pass1_1018_6048(u32 param_1)

{
  code **ppcVar1;

  ppcVar1 = (code **)((int)(u32)(u32)((int)param_1 + 0x92) + 0x8);
  (**ppcVar1)();
  return 0x0;
}



void set_window_text_1018_6066(astruct_937 *param_1,u16 param_2,char *in_win_text_3,INT16 dialog_id_5)

{
  HWND16 hwnd;

  hwnd = GetDlgItem16(dialog_id_5,param_1->hwnd_field_0x6);
  SetWindowText16((u32)in_win_text_3,hwnd);
  return;
}



void set_window_text_1018_6086(u32 param_1,u16 param_2,u16 param_3)

{
  HWND16 hwnd_1;
  u16 uVar2;

  wsprintf16((WORD *)&stack0xfff4,(char *)0x42421050,(char *)CONCAT22(param_3,0x1050));
  uVar2 = (param_1 >> 0x10);
  hwnd_1 = GetDlgItem16(0x1be,*(HWND16 *)((int)param_1 + 0x6));
  SetWindowText16(CONCAT22(0x1050,&stack0xfff4),hwnd_1);
  wsprintf16((WORD *)&stack0xfff4,(char *)0x42451050,(char *)CONCAT22(param_2,0x1050));
  hwnd_1 = GetDlgItem16(0x1bf,*(HWND16 *)((int)param_1 + 0x6));
  SetWindowText16(CONCAT22(0x1050,&stack0xfff4),hwnd_1);
  return;
}



void FUN_1018_60ea(void)

{
  return;
}



u16 FUN_1018_60ee(void)

{
  return 0x0;
}



u16 FUN_1018_60f4(void)

{
  return 0x0;
}



void FUN_1018_60fa(void)

{
  return;
}



void FUN_1018_60fe(void)

{
  return;
}



StructD * pass1_1018_6102(StructD *param_1,u8 param_2)

{
  pass1_1018_5e5a(&param_1->address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_6198(u8 *param_1,astruct_657 *param_2,StructB *param_3,u16 param_4)

{
  u16 in_register_0000000a;
  astruct_657 *iVar1;
  u16 uVar1;
  u32 *puVar2;
  u16 in_stack_0000fe96;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc4;
  u32 in_stack_0000ffec;

  uVar1 = ((u32)param_2 >> 0x10);
  iVar1 = (astruct_657 *)param_2;
  param_2 = 0x389a;
  iVar1->field2_0x2 = 0x1008;
  param_2 = 0x3aa8;
  iVar1->field2_0x2 = 0x1008;
  iVar1->field3_0x4 = param_4;
  param_2 = 0x3ab0;
  iVar1->field2_0x2 = 0x1008;
  (u32)&iVar1->field4_0x6 = 0x0;
  iVar1->field6_0xa = param_3;
  param_2 = 0x66c0;
  iVar1->field2_0x2 = 0x1018;
  puVar2 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22((int)((u32)in_stack_0000ffec >> 0x10),0x39),in_stack_0000fe96,
                           in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
  iVar1->field4_0x6 = (int)puVar2;
  iVar1->field5_0x8 = (int)((u32)puVar2 >> 0x10);
  return;
}



void pass1_1018_620c(StructD *struct_param_1)

{
  StructD *struct_1;
  u16 struct_1_lo;

  struct_1_lo = ((u32)struct_param_1 >> 0x10);
  struct_1 = (StructD *)struct_param_1;
  struct_param_1->address_offset_field_0x0 = 0x66c0;
  struct_1->address_offset_field_0x2 = 0x1018;
  struct_param_1->address_offset_field_0x0 = 0x3ab0;
  struct_1->address_offset_field_0x2 = 0x1008;
  struct_param_1->address_offset_field_0x0 = 0x389a;
  struct_1->address_offset_field_0x2 = 0x1008;
  return;
}



// WARNING: Inlined function: struct_1010_4d5c
// WARNING: Unable to use type for symbol uVar2_01
// WARNING: Unable to use type for symbol uVar15
// WARNING: Unable to use type for symbol uVar5
// WARNING: Unable to use type for symbol uVar8
// WARNING: Unable to use type for symbol uVar9
// WARNING: Unable to use type for symbol uVar10
// WARNING: Unable to use type for symbol uVar11
// WARNING: Unable to use type for symbol puVar4
// WARNING: Unable to use type for symbol uVar4
// WARNING: Unable to use type for symbol uVar3
// WARNING: Unable to use type for symbol uVar2_00
// WARNING: Unable to use type for symbol uVar3_00
// WARNING: Unable to use type for symbol uVar23

void unk_draw_op_1018_623e(astruct_742 *param_1)

{
  i16 *piVar2;
  u32 uVar3_01;
  u16 uVar6;
  HDC16 *pHVar14;
  i16 iVar7;
  i16 iVar11;
  HPEN16 handle;
  HBRUSH16 hgdiobj16_00;
  u16 uVar7;
  HPALETTE16 obj;
  u8 *puVar7;
  HGDIOBJ16 hgdiobj16_var7;
  u32 in_EDX;
  astruct_57 *paVar25;
  u16 uVar27;
  u32 uVar26;
  astruct_742 *struct742_var8;
  astruct_755 *iVar9;
  astruct_756 *iVar10;
  astruct_734 *puVar11;
  i16 iVar12;
  u16 uVar12;
  u16 uVar14;
  u16 uVar13;
  u32 uVar28;
  i16 *piVar16;
  i16 iVar29;
  u8 local_38 [0x6];
  u16 local_32;
  u16 uStack48;
  u32 uStack46;
  u16 uStack42;
  u32 *puStack40;
  HDC16 local_24;
  u8 paintstruct16_22 [0x20];
  u32 uVar2_01;
  i16 *piVar1;
  u32 uVar15;
  u32 uVar5;
  u32 uVar8;
  u32 uVar9;
  u32 uVar10;
  astruct_758 *iVar13;
  u32 uVar11;
  u32 uVar2;
  u32 *puVar4;
  u32 uVar4;
  u8 uVar16;
  u8 uVar17;
  u16 uVar18;
  astruct_757 *iVar16;
  u16 uVar19;
  u8 uVar20;
  u8 uVar21;
  u16 uVar22;
  u32 uVar3;
  u32 uVar2_00;
  u32 uVar3_00;
  u32 uVar23;
  u32 uVar24;
  code **fn_ptr_1;

  uVar22 = ((u32)in_EDX >> 0x10);
  puVar11 = (astruct_734 *)&stack0xfffe;
  uVar12 = ((u32)param_1 >> 0x10);
  struct742_var8 = (astruct_742 *)param_1;
  local_24 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct16_22),struct742_var8->field4_0x4);
  puStack40 = (u32 *)pass1_1010_4c2c(struct742_var8->field5_0x6);
  pHVar14 = &local_24;
  fn_ptr_1 = (code **)((int)*puStack40 + 0x8);
  (**fn_ptr_1)(0x1010,(int)puStack40,(int)((u32)puStack40 >> 0x10),pHVar14,(int)&DAT_1050_1050);
  struct742_var8->field12_0x10 = pHVar14;
  uVar2 = struct742_var8->field5_0x6;
  uStack42 = ((int)uVar2 + 0x30);
  uVar28 = struct742_var8->field5_0x6;
  uStack46 = *(u32*)((int)uVar28 + 0x12);
  uStack48 = 0x14;
  local_32 = 0x0;
  uVar13 = 0x1008;
  pass1_1008_3e38((astruct_19 *)CONCAT22(0x1050,local_38));
  paVar25 = (astruct_57 *)((u32)uVar22 << 0x10);
  while (uVar27 = ((u32)paVar25 >> 0x10), &puVar11[-0x6].field_0x4 < (puVar11 + -0x4)) {
    iVar9 = (astruct_755 *)(&puVar11[-0x6].field_0x4 * 0x4);
    uVar8 = puVar11[-0x5].field6_0x6;
    uVar28 = pass1_1008_4772(*(astruct_76 **)(iVar9 + (int)uVar8));
    puVar7 = (u8 *)(uVar28 >> 0x10);
    paVar25 = (astruct_57 *)CONCAT22(uVar27,puVar7);
    &puVar11[-0x7].field_0x2 = (int)uVar28;
    *(u8 **)&puVar11[-0x7].field_0x4 = puVar7;
    uVar3 = puVar11->field6_0x6;
    pass1_1018_642e(uVar3,(uVar3 >> 0x10),(i16 *)CONCAT13(0x10,CONCAT12(0x50,&puVar11[-0x5].field_0x2)),
                    ((int)uVar28 + 0x8));
    uVar9 = (u32)&puVar11[-0x5].field_0x2;
    pass1_1008_3e76((u16 *)CONCAT22(0x1050,&puVar11[-0x6].field6_0x6),0x0,uVar9,((u32)uVar9 >> 0x10));
    uVar23 = puVar11[-0x5].field6_0x6;
    pass1_1008_4480(*(astruct_76 **)&puVar11[-0x4].field_0x2,(u16 *)CONCAT22(0x1050,&puVar11[-0x6].field6_0x6),
                    *(astruct_76 **)(iVar9 + (int)uVar23));
    iVar29 = &puVar11[-0x6].field_0x4;
    uVar10 = (u32)&puVar11[-0x5].field_0x2;
    uVar19 = uVar10;
    uVar20 = (u8)((u32)uVar10 >> 0x10);
    uVar21 = (u8)((u32)uVar10 >> 0x18);
    uVar11 = (u32)&puVar11[-0x7].field_0x2;
    uVar14 = ((u32)uVar11 >> 0x10);
    iVar10 = (astruct_756 *)uVar11;
    iVar7 = iVar10->field4_0x4 + &puVar11[-0x5].field_0x4;
    iVar11 = iVar10->field7_0x8 + &puVar11[-0x5].field_0x2;
    uVar26 = puVar11->field6_0x6;
    uVar2_00 = (u32)((int)uVar26 + 0x6);
    iVar16 = (astruct_757 *)uVar2_00;
    uVar18 = ((u32)uVar2_00 >> 0x10);
    uVar16 = '\b';
    uVar17 = '\x10';
    if (*(i32 *)&iVar16->field_0x1a == 0x0) {
      uVar6 = iVar16->field47_0x30 << 0x3;
      mem_op_1000_179c(uVar6,paVar25);
      &iVar16->field_0x1a = uVar6;
      iVar16->field28_0x1c = (u8 *)paVar25;
    }
    uVar3_00 = (u32)&iVar16->field_0x1a;
    iVar13 = (astruct_758 *)(iVar29 * 0x8);
    (iVar13 + (int)uVar3_00) = CONCAT11(uVar21,uVar20);
    uVar3_01 = (u32)&iVar16->field_0x1a;
    (iVar13 + (int)uVar3_01 + 0x2) = uVar19;
    uVar3_01 = (u32)&iVar16->field_0x1a;
    (iVar13 + (int)uVar3_01 + 0x4) = iVar7;
    uVar3_01 = (u32)&iVar16->field_0x1a;
    (iVar13 + (int)uVar3_01 + 0x6) = iVar11;
    uVar13 = CONCAT11(uVar17,uVar16);
    uVar2_01 = (u32)&puVar11[-0x7].field_0x2;
    piVar2 = (i16 *)&puVar11[-0x5].field_0x4;
    *piVar2 = *piVar2 + (-(&puVar11[-0x6].field_0x4 == 0x0) & 0x5) + 0x14 + ((int)uVar2_01 + 0x4);
    piVar2 = (i16 *)&puVar11[-0x6].field_0x4;
    *piVar2 = *piVar2 + 0x1;
  }
  puVar4 = (u32*)&puVar11[-0x4].field_0x2;
  fn_ptr_1 = (code **)((int)*puVar4 + 0x4);
  (**fn_ptr_1)(uVar13,(int)puVar4,(int)((u32)puVar4 >> 0x10),0x0,0x0,(char)puVar11 + -0x22,(int)&DAT_1050_1050);
  handle = CreatePen16(0x1000025,0x1,0x0);
  *(HPEN16 *)&puVar11[-0x6].field_0x2 = handle;
  hgdiobj16_var7 = SelectObject16(handle,*(HDC16 *)&puVar11[-0x4].field6_0x6);
  *(HGDIOBJ16 *)(puVar11 + -0x6) = hgdiobj16_var7;
  hgdiobj16_00 = CreateSolidBrush16(0x1000025);
  *(HBRUSH16 *)((int)&puVar11[-0x7].field6_0x6 + 0x2) = hgdiobj16_00;
  hgdiobj16_var7 = SelectObject16(hgdiobj16_00,*(HDC16 *)&puVar11[-0x4].field6_0x6);
  *(HGDIOBJ16 *)&puVar11[-0x7].field6_0x6 = hgdiobj16_var7;
  draw_line_1018_6444(puVar11->field6_0x6,*(HDC16 *)&puVar11[-0x4].field6_0x6);
  uVar4 = puVar11->field6_0x6;
  piVar16 = (i16 *)pass1_1010_4dc8((u32)((int)uVar4 + 0x6));
  uVar26 = (u32)piVar16 >> 0x10;
  uVar24 = (u32)piVar16 & 0xffff;
  draw_op_1018_6544(puVar11->field6_0x6,
                    (i16 *)((u32)piVar16 & 0xff000000 | (u32)CONCAT12((char)((u32)piVar16 >> 0x10),(int)uVar24)));
  pass1_1018_6630((char *)(uVar26 & 0xffff | uVar24 << 0x10),(astruct_2 *)puVar11->field6_0x6);
  uVar5 = puVar11->field6_0x6;
  obj = SelectPalette16(0x0,*(HPALETTE16 *)((int)uVar5 + 0x10),*(HDC16 *)&puVar11[-0x4].field6_0x6);
  DeleteObject16(obj);
  hgdiobj16_var7 = SelectObject16(*(HGDIOBJ16 *)(puVar11 + -0x6),*(HDC16 *)&puVar11[-0x4].field6_0x6);
  DeleteObject16(hgdiobj16_var7);
  hgdiobj16_var7 = SelectObject16(*(HGDIOBJ16 *)&puVar11[-0x7].field6_0x6,*(HDC16 *)&puVar11[-0x4].field6_0x6);
  DeleteObject16(hgdiobj16_var7);
  uVar15 = puVar11->field6_0x6;
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,(u8 *)((int)&puVar11[-0x4].field6_0x6 + 0x2)),
             *(HWND16 *)((int)uVar15 + 0x4));
  return;
}



void pass1_1018_642e(u16 param_1,u16 param_2,i16 *param_3,i16 param_4)

{
  *param_3 = 0x64 - param_4 >> 0x1;
  return;
}



// WARNING: Unable to use type for symbol uVar3
// WARNING: Unable to use type for symbol x1
// WARNING: Unable to use type for symbol uVar2

void draw_line_1018_6444(u32 param_1,HDC16 hdc_param_2)

{
  i16 iVar1;
  i16 x;
  astruct_796 *iVar4;
  i16 *piVar5;
  i16 *piVar4;
  u16 uVar6;
  u16 uVar7;
  HDC16 x_00;
  i16 iStack10;
  u32 uVar3;
  INT16 *x1;
  u32 uVar2;

  uVar6 = (param_1 >> 0x10);
  uVar2 = (u32)((int)param_1 + 0x6);
  iVar1 = ((int)uVar2 + 0x30);
  uVar3 = (u32)((int)param_1 + 0x6);
  x1 = *(INT16 **)((int)uVar3 + 0x1a);
  MoveTo16(0x5,*x1,hdc_param_2);
  uVar7 = ((u32)x1 >> 0x10);
  iVar4 = (astruct_796 *)x1;
  LineTo16(0x5,*(INT16 *)(iVar4 + iVar1 * 0x8 + -0x4),hdc_param_2);
  for (iStack10 = 0x0; iStack10 < iVar1; iStack10 += 0x1) {
    piVar5 = (i16 *)(iVar4 + iStack10 * 0x8);
    x = (piVar5[0x2] - *piVar5 >> 0x1) + *piVar5;
    MoveTo16(0x5,x,hdc_param_2);
    LineTo16(0xa,x,hdc_param_2);
  }
  MoveTo16(0x5f,*x1,hdc_param_2);
  LineTo16(0x5f,*(INT16 *)(iVar4 + iVar1 * 0x8 + -0x4),hdc_param_2);
  for (iStack10 = 0x0; iStack10 < iVar1; iStack10 += 0x1) {
    piVar4 = (i16 *)(iVar4 + iStack10 * 0x8);
    x_00 = hdc_param_2;
    MoveTo16(0x5f,(piVar4[0x2] - *piVar4 >> 0x1) + *piVar4,hdc_param_2);
    LineTo16(0x5a,x_00,hdc_param_2);
  }
  return;
}



void draw_op_1018_6544(u32 param_1,i16 *param_2)

{
  u16 *puVar1;
  u32 uVar2;
  u16 uVar3;
  u8 local_a [0x6];
  u16 uStack4;

  if (param_2 != NULL) {
    uStack4 = (((int)param_2 + 0x4) - *param_2 >> 0x1) + *param_2;
    puVar1 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,local_a),0x0,0x57,uStack4);
    uVar3 = (param_1 >> 0x10);
    uVar2 = pass1_1018_659a((u8 *)((u32)puVar1 >> 0x10),param_1,uVar3,(u16 *)CONCAT22(0x1050,local_a));
    draw_polygon_1018_661c(param_1,uVar3,0x3,uVar2,(uVar2 >> 0x10));
  }
  return;
}



u32 pass1_1018_659a(u8 *param_1,u16 param_2,u16 param_3,u16 *param_4)

{
  i16 *piVar1;
  u16 uVar2;
  u16 in_register_0000000a;
  astruct_57 *paVar3;
  i16 iStack18;
  i16 local_6;
  i16 local_4;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  piVar1 = &local_6;
  pass1_1008_3e94(param_4,(u16 *)CONCAT22(0x1050,piVar1),(char *)CONCAT22(0x1050,&local_4));
  mem_op_1000_179c(0xc,paVar3);
  uVar2 = SUB42(paVar3,0x0);
  for (iStack18 = 0x0; iStack18 < 0x3; iStack18 += 0x1) {
    piVar1[iStack18 * 0x2] = (iStack18 * 0x4 + 0x4248) + local_4;
    piVar1[iStack18 * 0x2 + 0x1] = (iStack18 * 0x4 + 0x424a) + local_6;
  }
  return CONCAT22(uVar2,piVar1);
}



void draw_polygon_1018_661c(u16 param_1,u16 param_2,u16 count_param_3,u16 param_4,u16 param_5)

{
  Polygon16(count_param_3,(POINT16 *)param_4,param_5);
  return;
}



void pass1_1018_6630(char *param_1,astruct_2 *pstruct_param_3)

{
  astruct_812 *paVar1;
  u32 uVar2;
  u16 dialog_id_5;
  u16 uVar3;
  astruct_2 *pstruct_2_1;
  u16 uVar4;
  i16 iStack12;
  i16 iStack10;
  u16 uStack8;

  uVar4 = ((u32)pstruct_param_3 >> 0x10);
  pstruct_2_1 = (astruct_2 *)pstruct_param_3;
  find_n_load_rsrc_1010_4e9e(pstruct_2_1->field6_0x6);
  if (param_1 != NULL) {
    for (iStack12 = 0x0; iStack10 = (int)((u32)param_1 >> 0x10), uStack8 = SUB42(param_1,0x0), iStack12 < 0x9;
        iStack12 += 0x1) {
      paVar1 = pstruct_2_1->field6_0x6;
      dialog_id_5 = pass1_1010_4f20(paVar1,((u32)paVar1 >> 0x10),iStack12);
      uVar2 = pstruct_2_1->field7_0xa;
      set_window_text_1018_6066
                ((astruct_937 *)uVar2,((u32)uVar2 >> 0x10),(char *)CONCAT22(uStack8,iStack10),dialog_id_5);
      uVar3 = str_op_1000_3da4((char *)CONCAT22(uStack8,iStack10));
      param_1 = (char *)((u32)param_1 & 0xffff | (u32)(iStack10 + uVar3 + 0x1) << 0x10);
    }
  }
  return;
}



StructD * pass1_1018_669a(StructD *param_1,u8 param_2)

{
  pass1_1018_620c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void struct_1018_66cc(u16 param_1,astruct_20 *param_2,u16 param_3,u16 param_4,u16 param_5)

{
  u16 uVar1;
  u16 in_register_0000000a;
  astruct_57 *paVar2;
  astruct_20 *iVar2;
  u16 unaff_BP;
  astruct_20 *uVar2;
  u32 *puVar3;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  unk_draw_op_1020_7f7a(param_2,0xa,CONCAT22(param_4,param_3),param_5);
  uVar2 = (astruct_20 *)((u32)param_2 >> 0x10);
  iVar2 = (astruct_20 *)param_2;
  (u32)&iVar2[0x1].field5_0xc = 0x0;
  iVar2[0x1].field7_0x10 = NULL;
  param_2->offset_0x0 = 0x6880;
  iVar2->base_0x2 = 0x1018;
  (iVar2 + 0x1)->offset_0x0 = 0x691c;
  iVar2[0x1].base_0x2 = 0x1018;
  puVar3 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0xb),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  uVar1 = ((u32)puVar3 >> 0x10);
  &iVar2[0x1].field7_0x10 = (int)puVar3;
  ((int)&iVar2[0x1].field7_0x10 + 0x2) = uVar1;
  &iVar2[0x1].field2_0x4 = &iVar2[0x1].field7_0x10;
  ((int)&iVar2[0x1].field2_0x4 + 0x2) = uVar1;
  return;
}



void pass1_1018_673c(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x6880;
  iVar1->address_offset_field_0x2 = 0x1018;
  &iVar1->field_0xe2 = 0x691c;
  &iVar1->field_0xe4 = 0x1018;
  pass1_1020_808e(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1018_6768(u16 param_1,u16 param_2,u32 param_3)

{
  code **ppcVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;
  u32 uVar5;

  uVar4 = (param_3 >> 0x10);
  uVar3 = param_3;
  uVar2 = (uVar3 + 0xf0) | (uVar3 + 0xee);
  if (uVar2 != 0x0) {
    ppcVar1 = (code **)((int)(u32)(u32)(uVar3 + 0xee) + 0x8);
    uVar5 = (**ppcVar1)();
    param_1 = ((u32)uVar5 >> 0x10);
    uVar2 = uVar5;
  }
  if ((uVar3 + 0xea) == 0x0) {
    (uVar3 + 0xea) = 0x1;
    uVar5 = pass1_1038_af40(uVar3,param_1,_PTR_LOOP_1050_5b7c,(uVar3 + 0x8),0x16);
    uVar2 = uVar5;
  }
  return uVar2;
}



void window_op_1018_67b6(StructD *param_1,StructA *param_2)

{
  astruct_666 *paVar1;
  u16 uVar3;
  astruct_57 *paVar4;
  StructA *struct_1;
  u16 uVar2;

  paVar1 = (astruct_666 *)((u32)param_1 >> 0x10);
  paVar4 = (astruct_57 *)(param_1 & 0xffff0000 | (u32)param_1 & 0xffff);
  create_window_ex_1008_9760(param_2);
  uVar2 = ((u32)param_2 >> 0x10);
  struct_1 = (StructA *)param_2;
  get_dc_1018_4db0(*(astruct_126 **)&struct_1[0x1].field20_0x26,struct_1->field4_0x8);
  mem_op_1000_179c(0x18,paVar4);
  uVar3 = paVar4 | paVar1;
  if (uVar3 != 0x0) {
    pass1_1018_6924(uVar3,(StructA *)CONCAT22(paVar4,paVar1),struct_1->field4_0x8);
    struct_1[0x1].field18_0x22 = paVar1;
    struct_1[0x1].field19_0x24 = uVar3;
    return;
  }
  (u32)&struct_1[0x1].field18_0x22 = 0x0;
  return;
}



void pass1_1018_681a(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  u16 uVar4;

  uVar4 = (param_1 >> 0x10);
  puVar1 = (u32 *)((int)param_1 + 0xee);
  uVar2 = ((int)param_1 + 0xf0);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  destroy_win_1008_628e(param_1);
  return;
}



StructD * pass1_1018_684c(StructD *param_1,u8 param_2)

{
  param_1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0xe2));
  pass1_1018_673c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * FUN_1018_685a(u16 param_1,StructD *param_2,u8 param_3)

{
  pass1_1018_673c(param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_6924(u16 param_1,StructA *param_2,u16 param_3)

{
  code **ppcVar1;
  u16 uVar3;
  i16 iVar4;
  u16 in_register_0000000a;
  astruct_57 *paVar5;
  u32 *puVar6;
  u16 in_stack_0000fe9a;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffc8;
  u32 in_stack_0000ffca;
  u16 in_stack_0000fff2;
  u32 uVar2;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  set_struct_op_1020_921c(param_1,param_2,param_3,in_stack_0000ffca);
  (u32)((int)param_2 + 0x14) = 0x0;
  param_2->field0_0x0 = 0x6a02;
  ((int)param_2 + 0x2) = 0x1018;
  puVar6 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff2,0xb),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  uVar3 = ((u32)puVar6 >> 0x10);
  ((int)param_2 + 0x14) = (int)puVar6;
  ((int)param_2 + 0x16) = uVar3;
  ((int)param_2 + 0x6) = ((int)param_2 + 0x14);
  ((int)param_2 + 0x8) = uVar3;
  uVar2 = (u32)((int)param_2 + 0x14);
  iVar4 = (int)param_2 + 0xa;
  ppcVar1 = (code **)((int)(u32)(u32)((int)uVar2 + 0xa) + 0x8);
  (**ppcVar1)();
  ((int)param_2 + 0x12) = iVar4;
  draw_op_1020_9364((StructA *)((u32)param_2 & 0xffff | (u32)param_2 << 0x10));
  return;
}



void pass1_1018_69ac(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x6a02;
  iVar1->address_offset_field_0x2 = 0x1018;
  if (iVar1->field12_0x14 != 0x0) {
    pass1_1010_1dda(iVar1->field12_0x14);
  }
  palette_op_1020_92c4(param_1);
  return;
}



StructD * pass1_1018_69dc(StructD *param_1,u8 param_2)

{
  pass1_1018_69ac(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_20 *
struct_op_1018_6a0e(astruct_20 *param_1,u16 param_2,u16 param_3,u16 param_4,u16 param_5,u16 param_6,
                   u32 param_7)

{
  i16 iVar1;
  u16 uVar2;
  u16 in_stack_0000ffd6;

  unk_draw_op_1008_61b2(in_stack_0000ffd6,(StructA *)param_1,param_3,param_6,param_7);
  uVar2 = ((u32)param_1 >> 0x10);
  iVar1 = (int)param_1;
  (iVar1 + 0xea) = param_5;
  (iVar1 + 0xec) = param_4;
  (iVar1 + 0xee) = param_2;
  (iVar1 + 0xf0) = 0x0;
  param_1->offset_0x0 = 0x6c66;
  (iVar1 + 0x2) = 0x1018;
  (iVar1 + 0xe0) = 0x1;
  (iVar1 + 0xe2) = 0x0;
  (iVar1 + 0xe4) = 0x0;
  (u32)(iVar1 + 0xe6) = 0x1df027f;
  return param_1;
}



void FUN_1018_6a76(void)

{
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void mixed_draw_op_1018_6a7a(u16 param_1,u16 param_2,astruct_28 *param_3)

{
  u16 uVar2;
  u16 in_register_0000000a;
  astruct_57 *paVar3;
  u16 uVar1;
  u32 *puVar4;
  u16 in_stack_0000fe80;
  u16 in_stack_0000ffa4;
  u16 in_stack_0000ffaa;
  u16 in_stack_0000ffae;
  u16 in_stack_0000ffb0;
  PAINTSTRUCT16 local_22;
  u16 in_stack_0000ffd8;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar1 = ((u32)param_3 >> 0x10);
  BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&local_22),*(HWND16 *)((int)param_3 + 0x8));
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&local_22),*(HWND16 *)((int)param_3 + 0x8));
  puVar4 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x2),in_stack_0000fe80,in_stack_0000ffa4
                           ,in_stack_0000ffaa,in_stack_0000ffae);
  uVar2 = ((u32)puVar4 >> 0x10);
  if (((int)puVar4 + 0x20) == 0x0) {
    unk_destroy_window_op_1018_6bb6(param_3);
    return;
  }
  mix_ui_op_1018_6adc(param_3,uVar2,in_stack_0000ffae,in_stack_0000ffb0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void mix_ui_op_1018_6adc(astruct_28 *param_1,u16 param_2,u16 param_3,u16 param_4)

{
  i16 iVar1;
  i16 iVar2;
  u16 uVar3;
  u16 in_register_0000000a;
  astruct_57 *paVar4;
  u32 uVar6;
  astruct_28 *pstruct28_1;
  astruct_28 *uVar5;
  u16 uVar7;
  u32 *puVar8;
  u16 in_stack_0000fe86;
  u16 in_stack_0000ffaa;
  u16 in_stack_0000ffb0;
  u16 in_stack_0000ffb4;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  puVar8 = mixed_1010_20ba(paVar4,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x48),in_stack_0000fe86,
                           in_stack_0000ffaa,in_stack_0000ffb0,in_stack_0000ffb4);
  uVar6 = (u32)puVar8 >> 0x10;
  uVar7 = ((u32)puVar8 >> 0x10);
  iVar1 = ((int)puVar8 + 0xa);
  iVar2 = ((int)puVar8 + 0xc);
  uVar5 = (astruct_28 *)((u32)param_1 >> 0x10);
  pstruct28_1 = (astruct_28 *)param_1;
  if (0x1 < iVar1 - pstruct28_1->field227_0xe6) {
    uVar6 = (u32)(iVar1 >> 0xf);
    pstruct28_1->field225_0xe2 = iVar1 / 0x2 - (pstruct28_1->field227_0xe6 + 0x1) / 0x2;
  }
  if (0x1 < iVar2 - pstruct28_1->field228_0xe8) {
    uVar6 = (u32)(iVar2 >> 0xf);
    pstruct28_1->field226_0xe4 = iVar2 / 0x2 - (pstruct28_1->field228_0xe8 + 0x1) / 0x2;
  }
  uVar6 = (u32)paVar4 & 0xffff0000 | uVar6;
  uVar7 = SUB42(s_tile2_bmp_1050_1538,0x0);
  uVar3 = ShowCursor16(0x0);
  if (pstruct28_1->field231_0xee != 0x0) {
    uVar7 = 0x1008;
    win_1008_5c5c(uVar3,uVar6,_u16_1050_02a0,pstruct28_1->field231_0xee);
    pstruct28_1->hwnd_0xf0 = uVar3;
  }
  uVar7 = FUN_1010_830a(uVar3,uVar6,uVar7,_u16_1050_14cc,pstruct28_1->field230_0xec);
  mci_send_command_1008_53ae(CONCAT22((int)uVar6,uVar7),pstruct28_1->field8_0x8);
  ShowCursor16(0x1);
  unk_destroy_window_op_1018_6bb6(param_1);
  return;
}



void unk_destroy_window_op_1018_6bb6(astruct_28 *param_1)

{
  BOOL16 BVar1;
  astruct_28 *struct_1;
  astruct_28 *uVar2;

  uVar2 = (astruct_28 *)((u32)param_1 >> 0x10);
  struct_1 = (astruct_28 *)param_1;
  if (struct_1->field229_0xea != 0x0) {
    PostMessage16(0x0,struct_1->field229_0xea,0x111,HWND16_1050_0396);
  }
  PostMessage16(0x0,0x79,0x111,HWND16_1050_0396);
  if (struct_1->hwnd_0xf0 != 0x0) {
    BVar1 = IsWindow16(struct_1->hwnd_0xf0);
    if (BVar1 != 0x0) {
      DestroyWindow16(struct_1->hwnd_0xf0);
      struct_1->hwnd_0xf0 = 0x0;
    }
  }
  return;
}



void pass1_1018_6c1e(StructD *param_1,u8 param_2)

{
  StructD *uVar1;
  u16 uVar2;

  uVar1 = (StructD *)param_1;
  uVar1 = (StructD *)&uVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(uVar1)));
  uVar2 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  uVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  uVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



astruct_20 * struct_1018_6d02(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0xb,0x9c,0x8b,param_2,param_3);
  param_1->offset_0x0 = 0xa27e;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_6d38(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0xc,0x9d,0xd0,param_2,param_3);
  param_1->offset_0x0 = 0xb562;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_6d6e(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0xd,0x9e,0xd1,param_2,param_3);
  param_1->offset_0x0 = 0x9822;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_6da4(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0xe,0x9f,0xd2,param_2,param_3);
  param_1->offset_0x0 = 0xab06;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_6dda(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0xf,0xa0,0xd4,param_2,param_3);
  param_1->offset_0x0 = 0xbdea;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_6e10(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x10,0xa1,0xda,param_2,param_3);
  param_1->offset_0x0 = 0xa0aa;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_6e46(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x11,0xa2,0xdc,param_2,param_3);
  param_1->offset_0x0 = 0xb38e;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_6e7c(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x12,0xa3,0xd3,param_2,param_3);
  param_1->offset_0x0 = 0x964e;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_6eb2(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x13,0xa4,0xdb,param_2,param_3);
  param_1->offset_0x0 = 0xa932;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_6ee8(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x14,0xa5,0xa5,param_2,param_3);
  param_1->offset_0x0 = 0xbc16;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_6f1e(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x15,0xa7,0xb2,param_2,param_3);
  param_1->offset_0x0 = 0x9e3a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_6f54(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x16,0xa8,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xb11e;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_6f8a(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x17,0xaf,0xc0,param_2,param_3);
  param_1->offset_0x0 = 0x93de;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_6fc0(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x18,0xb0,0xc1,param_2,param_3);
  param_1->offset_0x0 = 0xa6c2;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_6ff6(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x19,0xb1,0x80,param_2,param_3);
  param_1->offset_0x0 = 0xb9a6;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_702c(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x1ec,0x1a,0xb2,0xc3,param_2,param_3);
  param_1->offset_0x0 = 0x9c66;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7062(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x1b,0xb3,0xc4,param_2,param_3);
  param_1->offset_0x0 = 0xaf4a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7098(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x1c,0xb4,0xd8,param_2,param_3);
  param_1->offset_0x0 = 0xc22e;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_70ce(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x1d,0xb5,0x7b,param_2,param_3);
  param_1->offset_0x0 = 0xa4ee;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7104(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x1e,0xb6,0xd9,param_2,param_3);
  param_1->offset_0x0 = 0xb7d2;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_713a(astruct_20 *param_1,u16 param_2,u32 param_3,u16 param_4)

{
  struct_op_1018_6a0e(param_1,0x0,0x1f,0xb7,0x7d,param_2,param_3);
  param_1->offset_0x0 = 0x9a92;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7170(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x21,0xb9,0xdd,param_2,param_3);
  param_1->offset_0x0 = 0xad76;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_71a6(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x23,0xd3,0xd6,param_2,param_3);
  param_1->offset_0x0 = 0xb69a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_71dc(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x1ed,0x24,0xd4,0xd7,param_2,param_3);
  param_1->offset_0x0 = 0x995a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7212(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x25,0xe9,0xee,param_2,param_3);
  param_1->offset_0x0 = 0xa452;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7248(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x63,0xa6,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xc05a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_727e(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x64,0xa9,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xa31a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_72b4(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x65,0xaa,0xbb,param_2,param_3);
  param_1->offset_0x0 = 0xb5fe;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_72ea(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x66,0xab,0x0,param_2,param_3);
  param_1->offset_0x0 = 0x98be;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7320(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x67,0xac,0xbd,param_2,param_3);
  param_1->offset_0x0 = 0xaba2;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7356(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x68,0xad,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xbe86;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_738c(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x69,0xae,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xac3e;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_73c2(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x35,0xba,0x81,param_2,param_3);
  param_1->offset_0x0 = 0xbf22;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_73f8(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x39,0xbb,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xa146;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void FUN_1018_742e(u16 param_1,u16 param_2,u16 param_3,u16 param_4,astruct_28 *param_5)

{
  mixed_draw_op_1018_6a7a(param_2,param_3,param_5);
  if (PTR_LOOP_1050_4254 == NULL) {
    win_1008_5c5c(param_1,param_2,_u16_1050_02a0,0x1e9);
    if (param_1 != 0x0) {
      PTR_LOOP_1050_4254 = (u8 *)((int)&PTR_LOOP_1050_0000 + 0x1);
    }
  }
  return;
}



astruct_20 * struct_1018_745e(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x22,0xbc,0xd5,param_2,param_3);
  param_1->offset_0x0 = 0xb42a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7494(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x36,0xbd,0xcd,param_2,param_3);
  param_1->offset_0x0 = 0x96ea;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_74ca(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x37,0xbe,0x83,param_2,param_3);
  param_1->offset_0x0 = 0xa9ce;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7500(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x38,0xbf,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xbcb2;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7536(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x3a,0xc0,0x85,param_2,param_3);
  param_1->offset_0x0 = 0x9f72;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_756c(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x1e2,0x3b,0xc1,0x86,param_2,param_3);
  param_1->offset_0x0 = 0xb256;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_75a2(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x3c,0xc2,0x87,param_2,param_3);
  param_1->offset_0x0 = 0x9516;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_75d8(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x3d,0xc3,0x88,param_2,param_3);
  param_1->offset_0x0 = 0xa7fa;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_760e(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x3e,0xc4,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xbade;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7644(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x3f,0xc5,0x0,param_2,param_3);
  param_1->offset_0x0 = 0x9d02;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_767a(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x40,0xc6,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xafe6;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_76b0(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x41,0xc7,0x8d,param_2,param_3);
  param_1->offset_0x0 = 0xc2ca;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_76e6(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x42,0xc8,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xa58a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_771c(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x43,0xc9,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xb86e;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7752(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x44,0xcc,0x0,param_2,param_3);
  param_1->offset_0x0 = 0x9b2e;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7788(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x45,0xcd,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xae12;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_77be(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x46,0xd1,0x92,param_2,param_3);
  param_1->offset_0x0 = 0xc0f6;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_77f4(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x47,0xd2,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xa3b6;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_782a(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x48,0xd5,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xacda;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7860(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x49,0xd6,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xbfbe;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7896(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x1f4,0x4a,0xd7,0x98,param_2,param_3);
  param_1->offset_0x0 = 0xa1e2;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_78cc(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x4b,0xd8,0x99,param_2,param_3);
  param_1->offset_0x0 = 0xb4c6;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7902(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x4c,0xd9,0xee,param_2,param_3);
  param_1->offset_0x0 = 0x9786;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7938(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x4d,0xda,0x9c,param_2,param_3);
  param_1->offset_0x0 = 0xaa6a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_796e(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x4e,0xdb,0x9d,param_2,param_3);
  param_1->offset_0x0 = 0xbd4e;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_79a4(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x4f,0xdc,0x9e,param_2,param_3);
  param_1->offset_0x0 = 0xa00e;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_79da(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x50,0xdd,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xb2f2;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7a10(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x1d9,0x51,0xde,0x0,param_2,param_3);
  param_1->offset_0x0 = 0x95b2;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7a46(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x52,0xdf,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xa896;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7a7c(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x53,0xe0,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xbb7a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7ab2(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x1e4,0x55,0xe2,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xb082;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7ae8(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x1e4,0x56,0xe3,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xc366;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7b1e(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x1da,0x57,0xe4,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xa626;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7b54(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x1d8,0x58,0xe5,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xb90a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7b8a(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x59,0xe6,0x0,param_2,param_3);
  param_1->offset_0x0 = 0x9bca;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7bc0(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x1ef,0x5a,0xe7,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xaeae;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7bf6(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x5b,0xe8,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xc192;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7c2c(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x5c,0xea,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xb736;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7c62(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x5d,0xeb,0x0,param_2,param_3);
  param_1->offset_0x0 = 0x99f6;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7c98(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x1e6,0x5e,0xec,0xee,param_2,param_3);
  param_1->offset_0x0 = 0xba42;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7cce(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x1da,0x5f,0xed,0x0,param_2,param_3);
  param_1->offset_0x0 = 0x9ed6;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7d04(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x0,0x60,0xee,0x0,param_2,param_3);
  param_1->offset_0x0 = 0xb1ba;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7d3a(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x1f0,0x61,0xef,0x0,param_2,param_3);
  param_1->offset_0x0 = 0x947a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * struct_1018_7d70(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  struct_op_1018_6a0e(param_1,0x1f7,0x62,0xf0,0xcc,param_2,param_3);
  param_1->offset_0x0 = 0xa75e;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



void pass1_1018_7da6(u16 *param_1,u8 param_2)

{
  i16 iVar1;
  u16 uVar2;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_7dee(u16 *param_1,u8 param_2)

{
  i16 iVar1;
  u16 uVar2;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_7e36(u16 *param_1,u8 param_2)

{
  i16 iVar1;
  u16 uVar2;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_7e7e(u16 *param_1,u8 param_2)

{
  i16 iVar1;
  u16 uVar2;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_7ec6(u16 *param_1,u8 param_2)

{
  i16 iVar1;
  u16 uVar2;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_7f0e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_7f56(u16 *param_1,u8 param_2)

{
  i16 iVar1;
  u16 uVar2;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_7f9e(u16 *param_1,u8 param_2)

{
  i16 iVar1;
  u16 uVar2;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_7fe6(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_802e(u16 *param_1,u8 param_2)

{
  i16 iVar1;
  u16 uVar2;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8076(u16 *param_1,u8 param_2)

{
  i16 iVar1;
  u16 uVar2;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_80be(u16 *param_1,u8 param_2)

{
  i16 iVar1;
  u16 uVar2;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8106(u16 *param_1,u8 param_2)

{
  i16 iVar1;
  u16 uVar2;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_814e(u16 *param_1,u8 param_2)

{
  i16 iVar1;
  u16 uVar2;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8196(u16 *param_1,u8 param_2)

{
  i16 iVar1;
  u16 uVar2;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_81de(u16 *param_1,u8 param_2)

{
  i16 iVar1;
  u16 uVar2;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8226(u16 *param_1,u8 param_2)

{
  i16 iVar1;
  u16 uVar2;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_826e(u16 *param_1,u8 param_2)

{
  i16 iVar1;
  u16 uVar2;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_82b6(u16 *param_1,u8 param_2)

{
  i16 iVar1;
  u16 uVar2;

  iVar1 = (int)param_1;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | (u32)(iVar1 + 0xd2)));
  uVar2 = ((u32)param_1 >> 0x10);
  *param_1 = 0x380a;
  (iVar1 + 0x2) = 0x1008;
  *param_1 = 0x389a;
  (iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_82fe(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8346(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_838e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_83d6(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_841e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8466(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_84ae(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_84f6(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_853e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8586(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_85ce(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8616(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_865e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_86a6(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_86ee(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8736(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_877e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_87c6(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_880e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8856(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_889e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_88e6(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_892e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8976(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_89be(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8a06(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8a4e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8a96(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8ade(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8b26(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8b6e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8bb6(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8bfe(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8c46(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8c8e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8cd6(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8d1e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8d66(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8dae(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8df6(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8e3e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8e86(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8ece(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8f16(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8f5e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8fa6(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1._0_2_ = (int)param_1;
  iVar1._0_2_ = (int)iVar1 + 0xd2;
  iVar1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)iVar1);
  pass1_1008_57c4(iVar1);
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  ((int)iVar1 + 0x2) = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_8fee(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1._0_2_ = (int)param_1;
  iVar1._0_2_ = (int)iVar1 + 0xd2;
  iVar1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)iVar1);
  pass1_1008_57c4(iVar1);
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  ((int)iVar1 + 0x2) = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_9036(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1._0_2_ = (int)param_1;
  iVar1._0_2_ = (int)iVar1 + 0xd2;
  iVar1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)iVar1);
  pass1_1008_57c4(iVar1);
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  ((int)iVar1 + 0x2) = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)iVar1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_907e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_90c6(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_910e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_9156(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_919e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_91e6(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_922e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_9276(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_92be(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_9306(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_934e(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



void pass1_1018_9396(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_c402(u16 param_1,astruct_20 *param_2,u16 param_3,u16 param_4,u16 param_5,u32 param_6,
                    u32 param_7,u32 param_8,u32 param_9)

{
  i16 iVar1;
  u16 *puVar2;
  u16 in_register_0000000a;
  astruct_57 *paVar3;
  astruct_20 *iVar4;
  astruct_20 *uVar4;
  u32 *puVar4;
  u16 in_stack_0000fe8e;
  u16 in_stack_0000ffb2;
  u16 in_stack_0000ffb8;
  u16 in_stack_0000ffbc;
  u16 in_stack_0000ffe6;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  struct_1020_0762(param_2,CONCAT22((int)param_6,param_5),(u32 *)CONCAT22((int)param_7,(int)(param_6 >> 0x10)),
                   (param_7 >> 0x10),param_8,param_9);
  uVar4 = (astruct_20 *)((u32)param_2 >> 0x10);
  iVar4 = (astruct_20 *)param_2;
  iVar4[0x1].field8_0x14 = 0x0;
  iVar4[0x1].field9_0x16 = NULL;
  iVar4[0x1].field10_0x18 = 0x0;
  iVar4[0x1].field11_0x1a = 0x0;
  iVar4[0x1].field12_0x1c = 0x2;
  iVar4[0x1].field15_0x26 = 0x0;
  iVar4[0x1].field16_0x2a = 0x0;
  iVar4[0x1].field17_0x2c = 0x1e0190;
  iVar4[0x1].field18_0x30 = 0x0;
  param_2->offset_0x0 = 0xc8bc;
  iVar4->base_0x2 = 0x1018;
  puVar2 = pass1_1000_4906((StructD *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar4[0x1].field13_0x1e)),NULL,0x8);
  if ((param_4 == 0x0) || (param_3 != 0x0)) {
    if ((param_3 & param_4) == 0x0) goto LAB_1018_c4bb;
    puVar2 = (u16 *)pass1_1008_5fd8((u8 *)paVar3);
  }
  else {
    load_string_1010_84ac((int)_u16_1050_14cc,(INT16)((u32)_u16_1050_14cc >> 0x10),param_4);
  }
  (u16*)&iVar4[0x1].field15_0x26 = puVar2;
  ((int)&iVar4[0x1].field15_0x26 + 0x2) = (int)paVar3;
LAB_1018_c4bb:
  puVar4 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000ffe6,0x48),in_stack_0000fe8e,
                           in_stack_0000ffb2,in_stack_0000ffb8,in_stack_0000ffbc);
  iVar1 = (int)puVar4;
  iVar4[0x1].field8_0x14 = (iVar1 + 0xa);
  iVar4[0x1].field9_0x16 = *(astruct_19 **)(iVar1 + 0xc);
  pass1_1008_3e94((u16 *)((u32)puVar4 & 0xffff0000 | (u32)(iVar1 + 0xe)),
                  (u16 *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar4[0x1].field11_0x1a)),
                  (char *)((u32)param_2 & 0xffff0000 | ZEXT24(&iVar4[0x1].field10_0x18)));
  return;
}



void destroy_window_1018_c518(astruct_29 *param_1)

{
  BOOL16 is_window;
  astruct_29 *pstruct_29_1;
  astruct_29 *pstruct_29_hi;

  pstruct_29_hi = (astruct_29 *)((u32)param_1 >> 0x10);
  pstruct_29_1 = (astruct_29 *)param_1;
  param_1->field0_0x0 = 0xc8bc;
  pstruct_29_1->field1_0x2 = 0x1018;
  fn_ptr_1000_17ce(pstruct_29_1->field259_0x108);
  if (pstruct_29_1->hwnd_0x112 != NULL) {
    is_window = IsWindow16((HWND16)pstruct_29_1->hwnd_0x112);
    if (is_window != 0x0) {
      DestroyWindow16((HWND16)pstruct_29_1->hwnd_0x112);
      pstruct_29_1->hwnd_0x112 = NULL;
    }
  }
  pass1_1020_022c(param_1);
  return;
}



// WARNING: Unable to use type for symbol iVar2
// WARNING: Unable to use type for symbol uVar3
// WARNING: Unable to use type for symbol puVar3
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_draw_op_1018_c578(astruct_57 *param_1,astruct_36 *param_2)

{
  astruct_76 *paVar1;
  u16 uVar2;
  u16 uVar5;
  HDC16 *hpal;
  i16 iVar5;
  i16 iVar3;
  u16 uVar6;
  u16 uVar9;
  u16 uVar7;
  u16 extraout_DX;
  HPALETTE16 obj;
  astruct_36 *iVar4;
  u16 uVar10;
  u16 unaff_SI;
  astruct_36 *uVar8;
  u16 uVar11;
  u32 uVar12;
  u16 in_stack_0000fe56;
  u16 in_stack_0000ff7a;
  u16 in_stack_0000ff80;
  u16 in_stack_0000ff84;
  RECT16 rect_34;
  i16 iStack48;
  i16 iStack46;
  HBRUSH16 hbrush_44;
  HDC16 hdc_2a;
  u16 uStack40;
  u32 *puStack38;
  PAINTSTRUCT16 paintstruct_22;
  u16 uVar1;
  i16 iVar1;
  i16 iVar2;
  u32 uVar3;
  u32 uVar4;
  u8 *puVar3;
  code **fn_ptr_1;

  puStack38 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x2),in_stack_0000fe56,
                              in_stack_0000ff7a,in_stack_0000ff80,in_stack_0000ff84);
  uVar9 = ((u32)puStack38 >> 0x10);
  uVar5 = ((int)puStack38 + 0x20);
  iVar4 = (astruct_36 *)param_2;
  uVar8 = (astruct_36 *)((u32)param_2 >> 0x10);
  uStack40 = uVar5;
  if (uVar5 == 0x0) {
    BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar4->hwnd_0x8);
    EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar4->hwnd_0x8);
    PostMessage16(0x0,iVar4->wparam_0xea,0x111,HWND16_1050_0396);
    return;
  }
  if ((iVar4->field235_0xf0 == 0x0) && (iVar4->field238_0xf4 != 0x0)) {
    iVar4->field235_0xf0 = 0x1;
    puVar3 = &iVar4->field_0xf2;
    win_1008_5c9e(puVar3,uVar9,_u16_1050_02a0,(u32 *)((u32)param_2 & 0xffff0000 | ZEXT24(puVar3)));
    uVar5 = puVar3;
  }
  if (((int)_u16_1050_02a0 + 0x12) == 0x0) {
    win_1008_5c5c(uVar5,uVar9,_u16_1050_02a0,0x1d3);
  }
  hdc_2a = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar4->hwnd_0x8);
  hbrush_44 = CreateSolidBrush16(0x2000000);
  rect_34 = (RECT16)0x0;
  iStack48 = iVar4->field239_0xf6 + -0x1;
  iStack46 = iVar4->field240_0xf8 + -0x1;
  FillRect16(hbrush_44,&rect_34,(HDC16)&DAT_1050_1050);
  DeleteObject16(hbrush_44);
  uVar3 = iVar4->field225_0xe2;
  paVar1 = *(astruct_76 **)((int)uVar3 + 0xe);
  hpal = &hdc_2a;
  uVar11 = ((u32)paVar1 >> 0x10);
  uVar10 = paVar1;
  uVar4 = (u32)paVar1;
  fn_ptr_1 = (code **)((int)uVar4 + 0x8);
  (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,uVar10,uVar11,hpal,(int)&DAT_1050_1050);
  uVar12 = pass1_1008_4772(paVar1);
  uVar2 = (uVar12 >> 0x10);
  iVar1 = ((int)uVar12 + 0x4);
  iVar2 = ((int)uVar12 + 0x8);
  iVar5 = 0x1e0 - iVar2;
  extraout_DX = iVar5 >> 0xf;
  iVar3 = iVar5 / 0x2;
  iVar4->field249_0x10c = iVar3 + iVar2 + iVar4->field251_0x110;
  fn_ptr_1 = (code **)((int)uVar4 + 0x4);
  (**fn_ptr_1)(0x1008,uVar10,uVar11,iVar4->field242_0xfc + iVar4->field243_0xfe + iVar3,
               iVar4->field241_0xfa + (0x280 - iVar1) / 0x2,(char)&hdc_2a,(int)&DAT_1050_1050);
  draw_text_1018_c742(extraout_DX,param_2,(HDC16)&hdc_2a,(i16)&DAT_1050_1050,uVar10);
  obj = SelectPalette16(0x0,(HPALETTE16)hpal,hdc_2a);
  DeleteObject16(obj);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,&paintstruct_22),iVar4->hwnd_0x8);
  return;
}



// WARNING: Variable defined which should be unmapped: iStack22
// WARNING: Variable defined which should be unmapped: iStack20

void draw_text_1018_c742(u16 param_1,astruct_36 *struct36_param_1,HDC16 hdc_2,i16 count_param_3,u16 param_5)

{
  i16 *piVar2;
  i16 iVar3;
  u8 extraout_AH;
  u8 uVar3;
  i16 iVar5;
  i16 iVar1;
  astruct_36 *pstruct36_4;
  astruct_36 *pstruct36_hi;
  COLORREF color;
  i16 iStack22;
  i16 iStack20;
  RECT16 rect_12;
  i16 iStack14;
  i16 iStack12;
  i16 *piVar1;

  pstruct36_hi = (astruct_36 *)((u32)struct36_param_1 >> 0x10);
  pstruct36_4 = (astruct_36 *)struct36_param_1;
  if ((pstruct36_4->string_0x108 != NULL) && (*pstruct36_4->string_0x108 != '\0')) {
    uVar3 = SetTextColor16(0x8000,*_hdc_2);
    color = SetBkColor16(0x2000000,*_hdc_2);
    if (pstruct36_4->field247_0x106 == 0x0) {
      iVar3 = pstruct36_4->field250_0x10e;
      DrawText16(0x410,(RECT16 *)CONCAT22(0x1050,&stack0xffe6),-0x1,pstruct36_4->string_0x108,*_hdc_2);
      pstruct36_4->field244_0x100 = (0x280 - iVar3) / 0x2;
      pstruct36_4->field245_0x102 = pstruct36_4->field249_0x10c;
      pstruct36_4->field246_0x104 = pstruct36_4->field244_0x100 + iVar3;
      iVar3 = pstruct36_4->field245_0x102;
      pstruct36_4->field247_0x106 = iVar3;
      piVar1 = &pstruct36_4->field240_0xf8;
      if (*piVar1 == iVar3 || *piVar1 < iVar3) {
        iVar1 = iVar3 - pstruct36_4->field240_0xf8;
        piVar2 = &pstruct36_4->field245_0x102;
        *piVar2 = *piVar2 - iVar1;
        piVar2 = &pstruct36_4->field247_0x106;
        *piVar2 = *piVar2 - iVar1;
      }
    }
    rect_12.x = pstruct36_4->field241_0xfa + pstruct36_4->field244_0x100;
    rect_12.y = pstruct36_4->field242_0xfc + pstruct36_4->field245_0x102;
    DrawText16(0x10,(RECT16 *)CONCAT22(0x1050,&rect_12),-0x1,pstruct36_4->string_0x108,*_hdc_2);
    SetTextColor16(CONCAT22(param_1,CONCAT11(extraout_AH,uVar3)),*_hdc_2);
    SetBkColor16(color,*_hdc_2);
  }
  return;
}



astruct_29 * pass1_1018_c896(astruct_29 *param_1,u8 param_2)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_20 * pass1_1018_c958(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;
  u16 *puVar2;
  u16 uVar3;
  u16 uVar4;
  u8 local_6 [0x4];

  uVar3 = 0xf1;
  uVar4 = 0x9a;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x8d);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x732,0x26,CONCAT22((int)puVar2,0x1f40),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xdc5a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_c9a6(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;
  u16 *puVar2;
  u16 uVar3;
  u16 uVar4;
  u8 local_6 [0x4];

  uVar3 = 0xf2;
  uVar4 = 0xa0;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x8e);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x733,0x27,CONCAT22((int)puVar2,0x1b58),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xd6de;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_c9f4(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  i16 *piVar1;
  u16 uVar2;
  u16 uVar3;
  u16 *puVar4;
  u16 uVar5;
  u8 local_6 [0x4];

  uVar3 = 0xf3;
  uVar5 = 0xa6;
  puVar4 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x8f);
  uVar2 = ((u32)puVar4 >> 0x10);
  pass1_1018_c402(uVar2,param_1,0x0,0x734,0x28,CONCAT22((int)puVar4,0x32c8),CONCAT22(uVar3,uVar2),
                  CONCAT22(param_2,uVar5),param_3);
  uVar3 = ((u32)param_1 >> 0x10);
  param_1->offset_0x0 = 0xda86;
  ((int)param_1 + 0x2) = 0x1018;
  piVar1 = (i16 *)((int)param_1 + 0x10e);
  *piVar1 = *piVar1 + -0x19;
  return param_1;
}



astruct_20 * pass1_1018_ca48(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;
  u16 *puVar2;
  u16 uVar3;
  u16 uVar4;
  u8 local_6 [0x4];

  uVar3 = 0xf4;
  uVar4 = 0xa1;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x90);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x735,0x29,CONCAT22((int)puVar2,0x36b0),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xd50a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_ca96(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  i16 *piVar1;
  u16 uVar2;
  u16 uVar3;
  u16 *puVar4;
  u16 uVar5;
  u8 local_6 [0x4];

  uVar3 = 0xf5;
  uVar5 = 0xbf;
  puVar4 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x92);
  uVar2 = ((u32)puVar4 >> 0x10);
  pass1_1018_c402(uVar2,param_1,0x737,0x736,0x2a,CONCAT22((int)puVar4,0x6590),CONCAT22(uVar3,uVar2),
                  CONCAT22(param_2,uVar5),param_3);
  uVar3 = ((u32)param_1 >> 0x10);
  param_1->offset_0x0 = 0xd8b2;
  ((int)param_1 + 0x2) = 0x1018;
  piVar1 = (i16 *)((int)param_1 + 0x10e);
  *piVar1 = *piVar1 + 0x64;
  return param_1;
}



astruct_20 * pass1_1018_caea(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;
  u16 *puVar2;
  u16 uVar3;
  u16 uVar4;
  u8 local_6 [0x4];

  uVar3 = 0xf6;
  uVar4 = 0x93;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x93);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x738,0x2b,CONCAT22((int)puVar2,0x2328),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xdbbe;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_cb38(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;
  u16 *puVar2;
  u16 uVar3;
  u16 uVar4;
  u8 local_6 [0x4];

  uVar3 = 0xf7;
  uVar4 = 0x94;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x94);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x739,0x2c,CONCAT22((int)puVar2,0x32c8),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xd642;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_cb86(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  i16 *piVar1;
  u16 uVar2;
  u16 uVar3;
  u16 *puVar4;
  u16 uVar5;
  u8 local_6 [0x4];

  uVar3 = 0xf8;
  uVar5 = 0xc2;
  puVar4 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x96);
  uVar2 = ((u32)puVar4 >> 0x10);
  pass1_1018_c402(uVar2,param_1,0x0,0x73a,0x2d,CONCAT22((int)puVar4,0x2328),CONCAT22(uVar3,uVar2),
                  CONCAT22(param_2,uVar5),param_3);
  uVar3 = ((u32)param_1 >> 0x10);
  param_1->offset_0x0 = 0xd9ea;
  ((int)param_1 + 0x2) = 0x1018;
  piVar1 = (i16 *)((int)param_1 + 0x10e);
  *piVar1 = *piVar1 + 0x64;
  return param_1;
}



astruct_20 * pass1_1018_cbda(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;
  u16 *puVar2;
  u16 uVar3;
  u16 uVar4;
  u8 local_6 [0x4];

  uVar3 = 0xf9;
  uVar4 = 0xc5;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x97);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x73b,0x2e,CONCAT22((int)puVar2,0x2af8),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xd46e;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_cc28(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;
  u16 *puVar2;
  u8 local_6 [0x4];
  u16 uVar3;
  u16 uVar4;

  uVar3 = 0xfa;
  uVar4 = 0xa3;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x98);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x73c,0x2f,CONCAT22((int)puVar2,0x2710),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xd816;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_cc76(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;
  u16 *puVar2;
  u16 uVar3;
  u16 uVar4;
  u8 local_6 [0x4];

  uVar3 = 0xfb;
  uVar4 = 0xa8;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x99);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x73e,0x73d,0x30,CONCAT22((int)puVar2,0x61a8),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xdb22;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_ccc4(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;
  u16 *puVar2;
  u16 uVar3;
  u16 uVar4;
  u8 local_6 [0x4];

  uVar3 = 0xfc;
  uVar4 = 0xa9;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x9b);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x740,0x73f,0x31,CONCAT22((int)puVar2,0x59d8),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xd5a6;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_cd12(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;
  u16 *puVar2;
  u16 uVar3;
  u16 uVar4;
  u8 local_6 [0x4];

  uVar3 = 0xfd;
  uVar4 = 0x7c;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x9c);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x741,0x32,CONCAT22((int)puVar2,0x2710),CONCAT22(uVar3,uVar1),
                  CONCAT22(param_2,uVar4),param_3);
  param_1->offset_0x0 = 0xd94e;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



astruct_20 * pass1_1018_cd60(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;
  u16 *puVar2;
  u16 uVar3;
  u16 uVar4;
  u8 local_6 [0x4];

  uVar3 = 0xfe;
  uVar4 = 0xc9;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x0);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x0,0x33,CONCAT22((int)puVar2,0x2710),CONCAT22(uVar3,uVar1),CONCAT22(param_2,uVar4),
                  param_3);
  param_1->offset_0x0 = 0xd3d2;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



// WARNING: Unable to use type for symbol iVar1
// WARNING: Unable to use type for symbol uVar2
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_draw_op_1018_cda8(astruct_57 *param_1,u16 param_2,u16 param_3,astruct_36 *struct36_param_1)

{
  astruct_76 *paVar1;
  u16 uVar9;
  u16 uVar4;
  u16 uVar11;
  u16 uVar7;
  u16 uVar3;
  HDC16 *hpalette_var1;
  i16 iVar4;
  i16 iVar10;
  i16 iVar2;
  HPALETTE16 selected_obj;
  u16 uVar12;
  astruct_36 *struct36_var3;
  u16 uVar13;
  u16 uVar5;
  u16 uVar6;
  u16 uVar8;
  u32 uVar14;
  u16 in_stack_0000fe5a;
  u16 in_stack_0000ff7e;
  u16 in_stack_0000ff84;
  u16 in_stack_0000ff88;
  RECT16 rect_var34;
  i16 iStack48;
  i16 iStack46;
  HBRUSH16 brush_handle_var44;
  HDC16 hdc_2a;
  u16 uStack40;
  u32 *puStack38;
  u8 paintstruct_var_22 [0x20];
  i16 *piVar1;
  i16 iVar1;
  u32 uVar2;
  u32 in_stack_0000ffb0;
  u16 uVar10;
  code **fn_ptr_2;

  puStack38 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x2),in_stack_0000fe5a,
                              in_stack_0000ff7e,in_stack_0000ff84,in_stack_0000ff88);
  uVar11 = ((u32)puStack38 >> 0x10);
  uVar3 = ((int)puStack38 + 0x20);
  struct36_var3 = (astruct_36 *)struct36_param_1;
  uVar5 = ((u32)struct36_param_1 >> 0x10);
  uStack40 = uVar3;
  if (uVar3 == 0x0) {
    BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_var_22),struct36_var3->hwnd_0x8);
    EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_var_22),struct36_var3->hwnd_0x8);
    PostMessage16(0x0,struct36_var3->wparam_0xea,0x111,HWND16_1050_0396);
    return;
  }
  if (struct36_var3->field235_0xf0 == 0x0) {
    struct36_var3->field235_0xf0 = 0x1;
    win_1008_5c5c(uVar3,uVar11,_u16_1050_02a0,0x1f3);
    uVar6 = (_u16_1050_02a0 >> 0x10);
    if (((int)_u16_1050_02a0 + 0x12) == 0x0) {
      win_1008_5c5c(uVar3,uVar11,_u16_1050_02a0,0x1d3);
    }
  }
  hdc_2a = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_var_22),struct36_var3->hwnd_0x8);
  brush_handle_var44 = CreateSolidBrush16(0x2000000);
  rect_var34 = (RECT16)0x0;
  iStack48 = struct36_var3->field239_0xf6 + -0x1;
  iStack46 = struct36_var3->field240_0xf8 + -0x1;
  FillRect16(brush_handle_var44,&rect_var34,(HDC16)&DAT_1050_1050);
  DeleteObject16(brush_handle_var44);
  uVar2 = struct36_var3->field225_0xe2;
  paVar1 = *(astruct_76 **)((int)uVar2 + 0xe);
  hpalette_var1 = &hdc_2a;
  uVar8 = ((u32)paVar1 >> 0x10);
  uVar13 = paVar1;
  fn_ptr_2 = (code **)((int)(u32)paVar1 + 0x8);
  (**fn_ptr_2)((int)s_tile2_bmp_1050_1538,uVar13,uVar8,hpalette_var1,(int)&DAT_1050_1050);
  uVar14 = pass1_1008_4772(paVar1);
  uVar9 = (uVar14 >> 0x10);
  iVar4 = (0x280 - ((int)uVar14 + 0x4)) / 0x2;
  iVar1 = ((int)uVar14 + 0x8);
  iVar10 = 0x1e0 - iVar1;
  uVar12 = iVar10 >> 0xf;
  iVar2 = iVar10 / 0x2;
  struct36_var3->field249_0x10c = iVar2 + iVar1 + struct36_var3->field251_0x110;
  if ((struct36_var3->field241_0xfa == 0x0) && (iVar4 == 0x0)) {
    piVar1 = &struct36_var3->field241_0xfa;
    *piVar1 = *piVar1 + 0x2;
  }
  fn_ptr_2 = (code **)((int)(u32)paVar1 + 0x4);
  (**fn_ptr_2)(0x1008,uVar13,uVar8,struct36_var3->field242_0xfc + struct36_var3->field243_0xfe + iVar2,
               struct36_var3->field241_0xfa + iVar4,(char)&hdc_2a,(int)&DAT_1050_1050);
  draw_text_1018_c742(uVar12,struct36_param_1,(HDC16)&hdc_2a,(i16)&DAT_1050_1050,uVar13);
  selected_obj = SelectPalette16(0x0,(HPALETTE16)hpalette_var1,hdc_2a);
  DeleteObject16(selected_obj);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_var_22),struct36_var3->hwnd_0x8);
  return;
}



astruct_20 * pass1_1018_cf74(astruct_20 *param_1,u16 param_2,u32 param_3)

{
  u16 uVar1;
  u16 *puVar2;
  u16 uVar3;
  u16 uVar4;
  u8 local_6 [0x4];

  uVar3 = 0xfe;
  uVar4 = 0xcf;
  puVar2 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x80);
  uVar1 = ((u32)puVar2 >> 0x10);
  pass1_1018_c402(uVar1,param_1,0x0,0x0,0x34,CONCAT22((int)puVar2,0x2710),CONCAT22(uVar3,uVar1),CONCAT22(param_2,uVar4),
                  param_3);
  param_1->offset_0x0 = 0xd77a;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



// WARNING: Unable to use type for symbol iVar1
// WARNING: Unable to use type for symbol uVar3
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void unk_draw_op_1018_cfc0(astruct_57 *param_1,u16 param_2,astruct_36 *struct36_param_1)

{
  u16 uVar1;
  u16 uVar4;
  HDC16 *hpal;
  i16 iVar3;
  i16 iVar2;
  i16 iVar4;
  u16 uVar5;
  u16 uVar9;
  u16 uVar6;
  HPALETTE16 obj;
  u16 uVar10;
  astruct_36 *struct36_var5;
  u16 uVar11;
  u16 uVar7;
  u16 uVar8;
  u32 uVar12;
  u16 in_stack_0000fe58;
  u16 in_stack_0000ff7c;
  u16 in_stack_0000ff82;
  u16 in_stack_0000ff86;
  RECT16 rect_34;
  i16 iStack48;
  i16 iStack46;
  HBRUSH16 hbrush_44;
  HDC16 local_2a;
  i16 iStack40;
  u32 *puStack38;
  u8 paintstruct_22 [0x20];
  i16 *piVar1;
  i16 iVar1;
  u32 uVar3;
  u16 in_stack_0000ffb0;
  code **fn_ptr_2;
  astruct_76 *struct76_var1;

  puStack38 = mixed_1010_20ba(param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_2,0x2),in_stack_0000fe58,
                              in_stack_0000ff7c,in_stack_0000ff82,in_stack_0000ff86);
  uVar9 = ((u32)puStack38 >> 0x10);
  iStack40 = ((int)puStack38 + 0x20);
  struct36_var5 = (astruct_36 *)struct36_param_1;
  uVar7 = ((u32)struct36_param_1 >> 0x10);
  if (iStack40 == 0x0) {
    BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),struct36_var5->hwnd_0x8);
    EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),struct36_var5->hwnd_0x8);
    PostMessage16(0x0,struct36_var5->wparam_0xea,0x111,HWND16_1050_0396);
    return;
  }
  if ((struct36_var5->field235_0xf0 == 0x0) && (struct36_var5->field238_0xf4 != 0x0)) {
    struct36_var5->field235_0xf0 = 0x1;
    uVar4 = &struct36_var5->field_0xf2;
    win_1008_5c9e(uVar4,uVar9,_u16_1050_02a0,(u32 *)((u32)struct36_param_1 & 0xffff0000 | (u32)uVar4));
    if (((int)_u16_1050_02a0 + 0x12) == 0x0) {
      win_1008_5c5c(uVar4,uVar9,_u16_1050_02a0,0x1d3);
    }
  }
  local_2a = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),struct36_var5->hwnd_0x8);
  hbrush_44 = CreateSolidBrush16(0x2000000);
  rect_34 = (RECT16)0x0;
  iStack48 = struct36_var5->field239_0xf6 + -0x1;
  iStack46 = struct36_var5->field240_0xf8 + -0x1;
  FillRect16(hbrush_44,&rect_34,(HDC16)&DAT_1050_1050);
  DeleteObject16(hbrush_44);
  uVar3 = struct36_var5->field225_0xe2;
  struct76_var1 = *(astruct_76 **)((int)uVar3 + 0xe);
  hpal = &local_2a;
  uVar8 = ((u32)struct76_var1 >> 0x10);
  uVar11 = struct76_var1;
  fn_ptr_2 = (code **)((int)(u32)struct76_var1 + 0x8);
  (**fn_ptr_2)((int)s_tile2_bmp_1050_1538,uVar11,uVar8,hpal,(int)&DAT_1050_1050);
  uVar12 = pass1_1008_4772(struct76_var1);
  uVar1 = (uVar12 >> 0x10);
  iVar3 = (0x280 - ((int)uVar12 + 0x4)) / 0x2;
  iVar1 = ((int)uVar12 + 0x8);
  iVar2 = 0x1e0 - iVar1;
  uVar10 = iVar2 >> 0xf;
  iVar4 = iVar2 / 0x2;
  struct36_var5->field249_0x10c = iVar4 + iVar1 + struct36_var5->field251_0x110;
  if ((struct36_var5->field241_0xfa == 0x0) && (iVar3 == 0x0)) {
    piVar1 = &struct36_var5->field241_0xfa;
    *piVar1 = *piVar1 + 0x2;
  }
  fn_ptr_2 = (code **)((int)(u32)struct76_var1 + 0x4);
  (**fn_ptr_2)(0x1008,uVar11,uVar8,struct36_var5->field242_0xfc + struct36_var5->field243_0xfe + iVar4,
               struct36_var5->field241_0xfa + iVar3,(char)&local_2a,(int)&DAT_1050_1050);
  draw_text_1018_c742(uVar10,struct36_param_1,(HDC16)&local_2a,(i16)&DAT_1050_1050,uVar11);
  obj = SelectPalette16(0x0,(HPALETTE16)hpal,local_2a);
  DeleteObject16(obj);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),struct36_var5->hwnd_0x8);
  return;
}



astruct_29 * pass1_1018_d198(astruct_29 *param_1,u8 param_2)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d1be(astruct_29 *param_1,u8 param_2)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d1e4(astruct_29 *param_1,u8 param_2)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d20a(astruct_29 *param_1,u8 param_2)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d230(astruct_29 *param_1,u8 param_2)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d256(astruct_29 *param_1,u8 param_2)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d27c(astruct_29 *param_1,u8 param_2)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d2a2(astruct_29 *param_1,u8 param_2)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d2c8(astruct_29 *param_1,u8 param_2)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d2ee(astruct_29 *param_1,u8 param_2)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d314(astruct_29 *param_1,u8 param_2)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d33a(astruct_29 *param_1,u8 param_2)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d360(astruct_29 *param_1,u8 param_2)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d386(astruct_29 *param_1,u8 param_2)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



astruct_29 * pass1_1018_d3ac(astruct_29 *param_1,u8 param_2)

{
  destroy_window_1018_c518(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



u16 * pass1_1018_dcf6(u16 *param_1)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  *param_1 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  *param_1 = 0xdf3c;
  ((int)param_1 + 0x2) = 0x1018;
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 pass1_1018_dd1e(u16 param_1,astruct_57 *param_2,u16 param_3,u16 param_4,i16 param_5,u32 param_6)

{
  u16 uVar1;
  u16 uVar2;
  u16 uVar3;
  u32 uVar4;

  pass1_1010_81f6(_u16_1050_14cc,0x0,param_6);
  uVar2 = SUB42(param_2,0x0);
  uVar1 = param_1;
  mem_op_1000_179c(0x46,param_2);
  uVar3 = param_2 | uVar1;
  uVar4 = (u32)param_2 & 0xffff0000 | (u32)uVar3;
  if (uVar3 == 0x0) {
    uVar1 = 0x0;
    uVar3 = 0x0;
  }
  else {
    pass1_1008_87cc((astruct_86 *)CONCAT22(param_2,uVar1),param_5,(int)param_6,param_6,
                    (astruct_76 *)CONCAT22(uVar2,param_1),0x0,uVar4);
    uVar3 = uVar4;
  }
  pass1_1008_8bc6(uVar3,CONCAT22(uVar3,uVar1));
  return CONCAT22(uVar3,uVar1);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_dd7c(u32 param_1,u16 param_2,u16 param_3,u32 param_4,u32 param_5,u16 param_6,
                    u16 param_7,u16 param_8,u16 param_9,u16 param_10)

{
  u16 uVar1;
  u32 uVar2;
  code **ppcVar3;
  u16 uVar4;
  u16 uVar5;
  u16 uVar6;
  u16 uVar7;
  u16 uVar8;
  u16 uVar9;
  u16 uVar10;
  astruct_57 *paVar11;
  u32 uVar12;
  u32 *puVar13;
  u32 *puVar14;
  i16 iVar15;
  i32 lStack32;
  u16 uStack20;
  u16 uStack12;

  uVar4 = param_5._3_1_;
  iVar15 = (int)(param_4 >> 0x10);
  if (param_5._3_1_ == 0x0) {
    puVar13 = mixed_1010_20ba((astruct_57 *)param_1,_u16_1050_0ed0,(u8 **)CONCAT22(param_6,0x2f),param_10,param_9
                              ,param_7,param_8);
    paVar11 = (astruct_57 *)(param_1 & 0xffff0000 | (u32)puVar13 >> 0x10);
    if (((int)puVar13 + 0x1e) == 0x0) {
      uStack20 = param_5;
      uVar4 = param_5;
    }
    else {
      if (param_5 - 0x7 == 0x0) {
        uStack20 = 0x6;
        param_5._0_2_ = param_5 - 0x7;
      }
      else if (param_5 - 0x8 == 0x0) {
        uStack20 = 0x7;
        param_5._0_2_ = param_5 - 0x8;
      }
      else {
        uStack20 = 0x8;
        param_5._0_2_ = param_5 - 0x9;
      }
      uVar4 = 0x6;
    }
    pass1_1010_81f6(_u16_1050_14cc,0x0,uVar4);
    uVar8 = paVar11;
    uVar4 = uVar8 | param_5;
    if (uVar4 != 0x0) {
      mem_op_1000_179c(0x46,paVar11);
      uVar9 = paVar11 | uVar4;
      if (uVar9 != 0x0) {
        pass1_1008_87cc((astruct_86 *)CONCAT22(paVar11,uVar4),(int)param_4,iVar15,uStack20,
                        (astruct_76 *)CONCAT22(uVar8,param_5),param_5,(u32)paVar11 & 0xffff0000 | (u32)uVar9);
      }
    }
  }
  else {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_5);
    uVar10 = param_1;
    puVar14 = (u32 *)struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar10,uVar4),uVar4,uVar10);
    uVar8 = puVar14;
    uVar9 = ((u32)puVar14 >> 0x10);
    if ((uVar9 | uVar8) != 0x0) {
      uVar2 = (u32)(uVar4 + 0x2e);
      uStack12 = uVar2;
      if (((uVar4 + 0x30) | uStack12) == 0x0) {
        lStack32 = 0x0;
      }
      else {
        lStack32 = *(i32 *)(uStack12 + 0x200);
      }
      uVar4 = (uVar8 + 0x1c);
      uVar1 = (uVar8 + 0x1e);
      paVar11 = (astruct_57 *)(param_1 & 0xffff0000 | (u32)uVar1);
      uVar5 = uVar1 | uVar4;
      if ((uVar1 | uVar4) != 0x0) {
        lStack32 = CONCAT22(uVar1,uVar4);
        uVar5 = uVar4;
      }
      ppcVar3 = (code **)((int)*puVar14 + 0x14);
      (**ppcVar3)(0x1030,uVar8,uVar9);
      uVar6 = uVar5;
      pass1_1010_81f6(_u16_1050_14cc,lStack32,uVar5);
      uVar10 = SUB42(paVar11,0x0);
      uVar7 = uVar6;
      mem_op_1000_179c(0x46,paVar11);
      uVar4 = paVar11 | uVar7;
      uVar12 = (u32)paVar11 & 0xffff0000 | (u32)uVar4;
      if (uVar4 == 0x0) {
        uVar7 = 0x0;
        uVar4 = 0x0;
      }
      else {
        pass1_1008_87cc((astruct_86 *)CONCAT22(paVar11,uVar7),(int)param_4,iVar15,uVar5,
                        (astruct_76 *)CONCAT22(uVar10,uVar6),param_5,uVar12);
        uVar4 = uVar12;
      }
      pass1_1008_8bc6(uVar4,CONCAT22(uVar4,uVar7));
    }
  }
  return;
}



StructD * pass1_1018_df10(StructD *param_1,u8 param_2)

{
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



void win_1018_df40(u16 param_1,u8 *param_2,StructA *param_3)

{
  u8 *puVar2;
  u16 in_register_0000000a;
  astruct_57 *paVar3;
  StructA *struct_1;
  u16 struct_1_hi;
  u16 *puVar1;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  create_window_ex_1008_9760(param_3);
  mem_op_1000_179c(0xa,paVar3);
  puVar2 = (u8 *)(paVar3 | param_1);
  struct_1 = (StructA *)param_3;
  struct_1_hi = ((u32)param_3 >> 0x10);
  if (puVar2 != NULL) {
    puVar1 = struct_1018_e100(puVar2,(u16 *)CONCAT22(paVar3,param_1),struct_1->field4_0x8);
    struct_1[0x1].field11_0x16 = (i16)puVar1;
    struct_1[0x1].field12_0x18 = (i16)((u32)puVar1 >> 0x10);
    return;
  }
  (u32)&struct_1[0x1].field11_0x16 = 0x0;
  return;
}



void pass1_1018_df92(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;

  destroy_win_1008_628e(param_1);
  uVar5 = (param_1 >> 0x10);
  iVar4 = (int)param_1;
  puVar1 = (u32 *)(iVar4 + 0xe2);
  uVar2 = (iVar4 + 0xe4);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)(0x1008,puVar1,uVar2,0x1);
  }
  (u32)(iVar4 + 0xe2) = 0x0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_dfd4(u8 *param_1,u32 param_2)

{
  u16 in_register_0000000a;
  astruct_57 *paVar1;
  u16 uVar2;
  u16 uVar3;
  u32 *puVar4;
  u16 in_stack_0000fe9c;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000fff4;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar3 = ((u32)param_2 >> 0x10);
  uVar2 = param_2;
  delete_palette_1018_e16c(*(astruct_795 **)(uVar2 + 0xe2));
  if ((uVar2 + 0xe6) == 0x0) {
    (uVar2 + 0xe6) = 0x1;
    puVar4 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff4,0x36),in_stack_0000fe9c,
                             in_stack_0000ffc0,in_stack_0000ffc6,in_stack_0000ffca);
    pass1_1038_af40(uVar2,((u32)puVar4 >> 0x10),_PTR_LOOP_1050_5b7c,(uVar2 + 0x8),0x8);
  }
  return;
}



void pass1_1018_e01c(StructD *param_1,u8 param_2)

{
  StructD *iVar1;
  u16 uVar1;

  iVar1 = (StructD *)param_1;
  iVar1 = (StructD *)&iVar1->field192_0xd2;
  pass1_1008_57c4((StructD *)((u32)param_1 & 0xffff0000 | ZEXT24(iVar1)));
  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x380a;
  iVar1->address_offset_field_0x2 = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  iVar1->address_offset_field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 * struct_1018_e100(u8 *param_1,u16 *param_2,u16 param_3)

{
  u16 in_register_0000000a;
  astruct_268 *iVar1;
  u16 uVar1;
  u32 *puVar2;
  u16 in_stack_0000fe96;
  u16 in_stack_0000ffba;
  u16 in_stack_0000ffc0;
  u16 in_stack_0000ffc4;
  u32 in_stack_0000ffec;

  uVar1 = ((u32)param_2 >> 0x10);
  iVar1 = (astruct_268 *)param_2;
  *param_2 = 0x389a;
  iVar1->field2_0x2 = 0x1008;
  *param_2 = 0x3aa8;
  iVar1->field2_0x2 = 0x1008;
  iVar1->field3_0x4 = param_3;
  *param_2 = 0x3ab0;
  iVar1->field2_0x2 = 0x1008;
  (u32)&iVar1->field4_0x6 = 0x0;
  *param_2 = 0xe228;
  iVar1->field2_0x2 = 0x1018;
  puVar2 = mixed_1010_20ba((astruct_57 *)CONCAT22(in_register_0000000a,param_1),_u16_1050_0ed0,
                           (u8 **)CONCAT22((int)((u32)in_stack_0000ffec >> 0x10),0x36),in_stack_0000fe96,
                           in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
  iVar1->field4_0x6 = (int)puVar2;
  iVar1->field5_0x8 = (int)((u32)puVar2 >> 0x10);
  return param_2;
}



// WARNING: Unable to use type for symbol uVar3

void delete_palette_1018_e16c(astruct_795 *param_1)

{
  u32 *puVar2;
  HDC16 *hpal;
  HPALETTE16 hpalette_a;
  astruct_795 *iVar5;
  u16 uVar5;
  u16 uVar6;
  u16 hwnd16_ss;
  HDC16 hdc_var24;
  u8 paintstruct_22 [0x20];
  u32 uVar4;
  u32 uVar3;
  u32 *puVar3;
  u32 *puVar1;
  code **fn_ptr_1;

  uVar5 = ((u32)param_1 >> 0x10);
  iVar5 = (astruct_795 *)param_1;
  hdc_var24 = BeginPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),iVar5->hwnd_0x4);
  uVar3 = iVar5->field5_0x6;
  puVar2 = (u32*)((int)uVar3 + 0xa);
  hpal = &hdc_var24;
  uVar6 = ((u32)puVar2 >> 0x10);
  uVar4 = *puVar2;
  fn_ptr_1 = (code **)((int)uVar4 + 0x8);
  (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,(int)puVar2,uVar6,hpal,(int)&DAT_1050_1050);
  fn_ptr_1 = (code **)((int)uVar4 + 0x4);
  (**fn_ptr_1)((int)s_tile2_bmp_1050_1538,(int)puVar2,uVar6,0x0,&hdc_var24,(int)&DAT_1050_1050);
  hpalette_a = SelectPalette16(0x0,(HPALETTE16)hpal,hdc_var24);
  DeleteObject16(hpalette_a);
  EndPaint16((PAINTSTRUCT16 *)CONCAT22(0x1050,paintstruct_22),iVar5->hwnd_0x4);
  return;
}



StructD * pass1_1018_e1ee(StructD *param_1,u8 param_2)

{
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  param_1->address_offset_field_0x0 = 0x3ab0;
  ((int)param_1 + 0x2) = 0x1008;
  param_1->address_offset_field_0x0 = 0x389a;
  ((int)param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_e230(u16 param_1,astruct_20 *param_2,u16 param_3,u16 param_4,u16 param_5)

{
  u16 uVar1;
  u16 in_register_0000000a;
  astruct_57 *paVar2;
  astruct_20 *iVar2;
  u16 unaff_BP;
  u16 uVar3;
  u32 *puVar4;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  unk_draw_op_1020_7f7a(param_2,0x4,CONCAT22(param_4,param_3),param_5);
  uVar3 = ((u32)param_2 >> 0x10);
  iVar2 = (astruct_20 *)param_2;
  (u32)&iVar2[0x1].field5_0xc = 0x0;
  iVar2[0x1].field7_0x10 = NULL;
  param_2->offset_0x0 = 0xe44e;
  iVar2->base_0x2 = 0x1018;
  (iVar2 + 0x1)->offset_0x0 = 0xe4ea;
  iVar2[0x1].base_0x2 = 0x1018;
  puVar4 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0x26),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  uVar1 = ((u32)puVar4 >> 0x10);
  &iVar2[0x1].field7_0x10 = (int)puVar4;
  ((int)&iVar2[0x1].field7_0x10 + 0x2) = uVar1;
  &iVar2[0x1].field2_0x4 = &iVar2[0x1].field7_0x10;
  ((int)&iVar2[0x1].field2_0x4 + 0x2) = uVar1;
  return;
}



void pass1_1018_e2a0(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0xe44e;
  iVar1->address_offset_field_0x2 = 0x1018;
  &iVar1->field_0xe2 = 0xe4ea;
  &iVar1->field_0xe4 = 0x1018;
  pass1_1020_808e(param_1);
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_e2cc(u32 param_1)

{
  i16 *piVar1;
  code **ppcVar2;
  u32 uVar3;
  u16 uVar4;
  u16 uVar5;
  u32 in_EDX;
  u16 uVar8;
  astruct_269 *iVar7;
  u16 uVar9;
  u16 *puVar10;
  u16 in_stack_0000ff4c;
  u16 in_stack_0000ff62;
  u32 uStack10;
  u8 local_6 [0x4];
  astruct_57 *paVar6;
  u32 uVar7;

  uVar8 = ((u32)in_EDX >> 0x10);
  uVar9 = (param_1 >> 0x10);
  iVar7 = (astruct_269 *)param_1;
  if (iVar7->field235_0xee != NULL) {
    ppcVar2 = (code **)((int)*iVar7->field235_0xee + 0x8);
    (**ppcVar2)();
  }
  if (iVar7->field233_0xea == 0x0) {
    iVar7->field233_0xea = 0x1;
    puVar10 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x7a);
    uVar4 = ((u32)puVar10 >> 0x10);
    paVar6 = (astruct_57 *)CONCAT22(uVar8,uVar4);
    uVar3 = ZEXT24(local_6);
    win_1008_5c9e(local_6,uVar4,_u16_1050_02a0,(u32 *)CONCAT22(0x1050,local_6));
    iVar7->field234_0xec = (int)uVar3;
    mem_op_1000_179c(0x112,paVar6);
    uVar5 = paVar6 | uVar3;
    uVar7 = (u32)paVar6 & 0xffff0000 | (u32)uVar5;
    if (uVar5 == 0x0) {
      uVar9 = 0x0;
      uStack10 = NULL;
    }
    else {
      piVar1 = &iVar7->field204_0xcc;
      *piVar1 = *piVar1 + 0x1;
      struct_1020_3644(uVar7,(StructA *)CONCAT13((char)((u32)paVar6 >> 0x8),CONCAT12((char)paVar6,uVar3)),
                       iVar7->field204_0xcc,param_1 & 0xffff | (u32)uVar9 << 0x10,in_stack_0000ff4c,in_stack_0000ff62)
      ;
      uVar9 = uVar3;
      uStack10 = (u32 *)(uVar3 & 0xffff | uVar7 << 0x10);
    }
    pass1_1008_6978(uVar9,(u8 *)uVar7,param_1,0x0,(u32)uStack10 & 0xffff0000 | (u32)uVar9);
    ppcVar2 = (code **)((int)*uStack10 + 0xc);
    (**ppcVar2)(0x8,(int)uStack10,uStack10,0x5);
  }
  return;
}



void window_op_1018_e384(astruct_659 *param_1,astruct_57 *param_2,StructA *param_3)

{
  u16 uVar1;
  astruct_57 *struct_1;
  astruct_57 *uVar2;

  create_window_ex_1008_9760(param_3);
  uVar2 = (astruct_57 *)((u32)param_3 >> 0x10);
  struct_1 = (astruct_57 *)param_3;
  get_dc_1018_4db0(*(astruct_126 **)&struct_1[0x1].field80_0x64,struct_1->field4_0x8);
  mem_op_1000_179c(0x18,param_2);
  uVar1 = param_2 | param_1;
  if (uVar1 != 0x0) {
    pass1_1018_e4f2(param_1,(astruct_57 *)((u32)param_2 & 0xffff | (u32)struct_1->field4_0x8 << 0x10));
    struct_1[0x1].field78_0x60 = param_1;
    struct_1[0x1].field79_0x62 = uVar1;
    return;
  }
  (u32)&struct_1[0x1].field78_0x60 = 0x0;
  return;
}



void pass1_1018_e3e8(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  u16 uVar4;

  uVar4 = (param_1 >> 0x10);
  puVar1 = (u32 *)((int)param_1 + 0xee);
  uVar2 = ((int)param_1 + 0xf0);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  destroy_win_1008_628e(param_1);
  return;
}



StructD * pass1_1018_e41a(StructD *param_1,u8 param_2)

{
  param_1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0xe2));
  pass1_1018_e2a0(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_e4f2(astruct_659 *param_1,astruct_57 *param_2)

{
  code **ppcVar1;
  u32 uVar2;
  u16 uVar3;
  u8 *puVar4;
  astruct_57 *in_EDX;
  u32 *puVar5;
  u16 in_stack_0000fe9a;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffc8;
  u32 in_stack_0000ffca;
  u16 uVar6;
  u16 in_stack_0000fff2;

  uVar6 = SUB42(param_2,0x0);
  set_struct_op_1020_921c
            (in_EDX,(StructA *)CONCAT22(uVar6,param_1),((u32)param_2 >> 0x10),in_stack_0000ffca);
  (u32)&param_1->field16_0x14 = 0x0;
  CONCAT22(uVar6,param_1) = 0xe5d0;
  param_1->field2_0x2 = 0x1018;
  puVar5 = mixed_1010_20ba(in_EDX,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff2,0x26),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  uVar3 = ((u32)puVar5 >> 0x10);
  param_1->field16_0x14 = (int)puVar5;
  param_1->field17_0x16 = uVar3;
  param_1->field5_0x6 = param_1->field16_0x14;
  param_1->field6_0x8 = uVar3;
  uVar2 = (u32)&param_1->field16_0x14;
  puVar4 = &param_1->field_0xa;
  ppcVar1 = (code **)((int)(u32)(u32)((int)uVar2 + 0xa) + 0x8);
  (**ppcVar1)();
  param_1->field15_0x12 = (int)puVar4;
  draw_op_1020_9364((StructA *)CONCAT22(uVar6,param_1));
  return;
}



void pass1_1018_e57a(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0xe5d0;
  iVar1->address_offset_field_0x2 = 0x1018;
  if (iVar1->field12_0x14 != 0x0) {
    pass1_1010_1dda(iVar1->field12_0x14);
  }
  palette_op_1020_92c4(param_1);
  return;
}



StructD * pass1_1018_e5aa(StructD *param_1,u8 param_2)

{
  pass1_1018_e57a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_e5dc(u16 param_1,astruct_20 *param_2,u16 param_3,u16 param_4,u16 param_5)

{
  u16 uVar1;
  u16 in_register_0000000a;
  astruct_57 *paVar2;
  astruct_20 *iVar2;
  u16 unaff_BP;
  u16 uVar3;
  u32 *puVar4;
  u16 in_stack_0000fea6;
  u16 in_stack_0000ffca;
  u16 in_stack_0000ffd0;
  u16 in_stack_0000ffd4;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  unk_draw_op_1020_7f7a(param_2,0x9,CONCAT22(param_4,param_3),param_5);
  uVar3 = ((u32)param_2 >> 0x10);
  iVar2 = (astruct_20 *)param_2;
  (u32)&iVar2[0x1].field5_0xc = 0x0;
  iVar2[0x1].field7_0x10 = NULL;
  param_2->offset_0x0 = 0xe790;
  iVar2->base_0x2 = 0x1018;
  (iVar2 + 0x1)->offset_0x0 = 0xe82c;
  iVar2[0x1].base_0x2 = 0x1018;
  puVar4 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_BP,0xa),in_stack_0000fea6,
                           in_stack_0000ffca,in_stack_0000ffd0,in_stack_0000ffd4);
  uVar1 = ((u32)puVar4 >> 0x10);
  &iVar2[0x1].field7_0x10 = (int)puVar4;
  ((int)&iVar2[0x1].field7_0x10 + 0x2) = uVar1;
  &iVar2[0x1].field2_0x4 = &iVar2[0x1].field7_0x10;
  ((int)&iVar2[0x1].field2_0x4 + 0x2) = uVar1;
  return;
}



void pass1_1018_e64c(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0xe790;
  iVar1->address_offset_field_0x2 = 0x1018;
  &iVar1->field_0xe2 = 0xe82c;
  &iVar1->field_0xe4 = 0x1018;
  pass1_1020_808e(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1018_e678(u16 param_1,u16 param_2,u32 param_3)

{
  code **ppcVar1;
  u16 uVar2;
  u16 uVar3;
  u16 uVar4;
  u32 uVar5;

  uVar4 = (param_3 >> 0x10);
  uVar3 = param_3;
  uVar2 = (uVar3 + 0xf0) | (uVar3 + 0xee);
  if (uVar2 != 0x0) {
    ppcVar1 = (code **)((int)(u32)(u32)(uVar3 + 0xee) + 0x8);
    uVar5 = (**ppcVar1)();
    param_1 = ((u32)uVar5 >> 0x10);
    uVar2 = uVar5;
  }
  if ((uVar3 + 0xea) == 0x0) {
    (uVar3 + 0xea) = 0x1;
    uVar5 = pass1_1038_af40(uVar3,param_1,_PTR_LOOP_1050_5b7c,(uVar3 + 0x8),0x15);
    uVar2 = uVar5;
  }
  return uVar2;
}



void window_op_1018_e6c6(astruct_666 *param_1,u16 param_2,StructA *param_3)

{
  u16 uVar1;
  u16 in_register_0000000a;
  astruct_57 *paVar2;
  StructA *iVar2;
  u16 uVar4;
  u32 uVar3;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  create_window_ex_1008_9760(param_3);
  uVar4 = ((u32)param_3 >> 0x10);
  iVar2 = (StructA *)param_3;
  get_dc_1018_4db0(*(astruct_126 **)&iVar2[0x1].field20_0x26,iVar2->field4_0x8);
  mem_op_1000_179c(0x18,paVar2);
  uVar1 = paVar2 | param_1;
  uVar3 = (u32)paVar2 & 0xffff0000 | (u32)uVar1;
  if (uVar1 != 0x0) {
    pass1_1018_e834((StructA *)CONCAT22(paVar2,param_1),iVar2->field4_0x8,uVar3);
    iVar2[0x1].field18_0x22 = param_1;
    iVar2[0x1].field19_0x24 = uVar3;
    return;
  }
  (u32)&iVar2[0x1].field18_0x22 = 0x0;
  return;
}



void pass1_1018_e72a(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  u16 uVar4;

  uVar4 = (param_1 >> 0x10);
  puVar1 = (u32 *)((int)param_1 + 0xee);
  uVar2 = ((int)param_1 + 0xf0);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  destroy_win_1008_628e(param_1);
  return;
}



StructD * pass1_1018_e75c(StructD *param_1,u8 param_2)

{
  param_1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0xe2));
  pass1_1018_e64c(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * FUN_1018_e76a(u16 param_1,StructD *param_2,u8 param_3)

{
  pass1_1018_e64c(param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_e834(StructA *param_1,u16 param_2,astruct_57 *param_3)

{
  code **ppcVar1;
  u32 uVar2;
  u16 uVar3;
  i16 iVar4;
  u32 *puVar5;
  u16 in_stack_0000fe9a;
  u16 in_stack_0000ffbe;
  u16 in_stack_0000ffc4;
  u16 in_stack_0000ffc8;
  u32 in_stack_0000ffca;
  u16 in_stack_0000fff2;

  set_struct_op_1020_921c(param_3,param_1,param_2,in_stack_0000ffca);
  (u32)((int)param_1 + 0x14) = 0x0;
  param_1->field0_0x0 = 0xe912;
  ((int)param_1 + 0x2) = 0x1018;
  puVar5 = mixed_1010_20ba(param_3,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fff2,0xa),in_stack_0000fe9a,
                           in_stack_0000ffbe,in_stack_0000ffc4,in_stack_0000ffc8);
  uVar3 = ((u32)puVar5 >> 0x10);
  ((int)param_1 + 0x14) = (int)puVar5;
  ((int)param_1 + 0x16) = uVar3;
  ((int)param_1 + 0x6) = ((int)param_1 + 0x14);
  ((int)param_1 + 0x8) = uVar3;
  uVar2 = (u32)((int)param_1 + 0x14);
  iVar4 = (int)param_1 + 0xa;
  ppcVar1 = (code **)((int)(u32)(u32)((int)uVar2 + 0xa) + 0x8);
  (**ppcVar1)();
  ((int)param_1 + 0x12) = iVar4;
  draw_op_1020_9364((StructA *)((u32)param_1 & 0xffff | (u32)param_1 << 0x10));
  return;
}



void pass1_1018_e8bc(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0xe912;
  iVar1->address_offset_field_0x2 = 0x1018;
  if (iVar1->field12_0x14 != 0x0) {
    pass1_1010_1dda(iVar1->field12_0x14);
  }
  palette_op_1020_92c4(param_1);
  return;
}



StructD * pass1_1018_e8ec(StructD *param_1,u8 param_2)

{
  pass1_1018_e8bc(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_e91e(u16 param_1,astruct_20 *param_2,u16 param_3,u16 param_4,u16 param_5)

{
  u32 uVar1;
  code **ppcVar2;
  astruct_20 *paVar3;
  u16 uVar4;
  u16 in_register_0000000a;
  astruct_57 *paVar5;
  u32 *puVar6;
  u16 in_stack_0000fea2;
  u16 in_stack_0000ffc6;
  u16 in_stack_0000ffcc;
  u16 in_stack_0000ffd0;
  u16 uVar7;
  u16 in_stack_0000fffa;
  astruct_20 *iVar7;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  iVar7 = (astruct_20 *)param_2;
  uVar7 = ((u32)param_2 >> 0x10);
  unk_draw_op_1020_7f7a(param_2,0x3,CONCAT22(param_4,param_3),param_5);
  (u32)&iVar7[0x1].field5_0xc = 0x0;
  iVar7[0x1].field7_0x10 = NULL;
  (u32)&iVar7[0x1].field8_0x14 = 0x0;
  param_2->offset_0x0 = 0xebd0;
  iVar7->base_0x2 = 0x1018;
  (iVar7 + 0x1)->offset_0x0 = 0xec6c;
  iVar7[0x1].base_0x2 = 0x1018;
  puVar6 = mixed_1010_20ba(paVar5,_u16_1050_0ed0,(u8 **)CONCAT22(in_stack_0000fffa,0x28),in_stack_0000fea2,
                           in_stack_0000ffc6,in_stack_0000ffcc,in_stack_0000ffd0);
  &iVar7[0x1].field7_0x10 = (int)puVar6;
  uVar4 = ((u32)puVar6 >> 0x10);
  ((int)&iVar7[0x1].field7_0x10 + 0x2) = uVar4;
  &iVar7[0x1].field2_0x4 = &iVar7[0x1].field7_0x10;
  ((int)&iVar7[0x1].field2_0x4 + 0x2) = uVar4;
  puVar6 = mixed_1010_20ba((astruct_57 *)((u32)paVar5 & 0xffff0000 | (u32)puVar6 >> 0x10),_u16_1050_0ed0,
                           (u8 **)CONCAT22(in_stack_0000fffa,0x32),in_stack_0000fea2,in_stack_0000ffc6,
                           in_stack_0000ffcc,in_stack_0000ffd0);
  iVar7[0x1].field8_0x14 = puVar6;
  iVar7[0x1].field9_0x16 = (astruct_19 *)((u32)puVar6 >> 0x10);
  if (param_2 == NULL) {
    paVar3 = NULL;
    uVar4 = 0x0;
  }
  else {
    paVar3 = iVar7 + 0x1;
    uVar4 = uVar7;
  }
  uVar1 = (u32)&iVar7[0x1].field8_0x14;
  ppcVar2 = (code **)((int)*(u32*)&iVar7[0x1].field8_0x14 + 0x4);
  (**ppcVar2)(0x1010,(int)uVar1,(int)((u32)uVar1 >> 0x10),0xb,paVar3,uVar4);
  return;
}



void pass1_1018_e9de(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0xebd0;
  iVar1->address_offset_field_0x2 = 0x1018;
  &iVar1->field_0xe2 = 0xec6c;
  &iVar1->field_0xe4 = 0x1018;
  pass1_1020_808e(param_1);
  return;
}



void post_win_msg_1018_ea0a(u16 param_1,u16 param_2,i16 param_3)

{
  if (param_3 == 0xed) {
    PostMessage16(0x0,0xc6,0x111,HWND16_1050_0396);
  }
  return;
}



void FUN_1018_ea2c(u16 param_1,u32 param_2,i16 param_3)

{
  u32 uVar1;
  i16 iVar2;
  u16 uVar3;

  iVar2 = (int)param_2;
  uVar3 = ((u32)param_2 >> 0x10);
  if (param_3 == 0x1) {
    (u32)(iVar2 + 0x14) = 0x0;
    return;
  }
  if (param_3 != 0xb) {
    return;
  }
  uVar1 = (u32)(iVar2 + 0x14);
  ((int)uVar1 + 0xe) = (iVar2 + -0xda);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_ea66(u32 param_1)

{
  code **ppcVar1;
  u8 *puVar2;
  u16 uVar3;
  u32 in_EDX;
  u16 uVar5;
  i16 iVar6;
  u16 uVar7;
  u16 *puVar8;
  u8 local_6 [0x4];
  u32 uVar4;

  uVar5 = ((u32)in_EDX >> 0x10);
  uVar7 = (param_1 >> 0x10);
  iVar6 = (int)param_1;
  if (*(i32 *)(iVar6 + 0xee) != 0x0) {
    ppcVar1 = (code **)((int)(u32)(u32)(iVar6 + 0xee) + 0x8);
    (**ppcVar1)();
  }
  if ((iVar6 + 0xea) == 0x0) {
    (iVar6 + 0xea) = 0x1;
    puVar8 = pass1_1008_941a((u16 *)CONCAT22(0x1050,local_6),0x1,0x95);
    uVar3 = ((u32)puVar8 >> 0x10);
    uVar4 = CONCAT22(uVar5,uVar3);
    puVar2 = local_6;
    win_1008_5c9e(puVar2,uVar3,_u16_1050_02a0,(u32 *)CONCAT22(0x1050,puVar2));
    (iVar6 + 0xec) = (int)puVar2;
    unk_win_op_1010_7300(uVar4,*(astruct_57 **)(iVar6 + 0xf6),0x0,0x8,0x0);
  }
  return;
}



void window_op_1018_eada(astruct_661 *param_1,astruct_57 *param_2,StructA *param_3)

{
  u16 uVar1;
  StructA *struct_1;
  u16 uVar2;

  create_window_ex_1008_9760(param_3);
  uVar2 = ((u32)param_3 >> 0x10);
  struct_1 = (StructA *)param_3;
  get_dc_1018_4db0(*(astruct_126 **)&struct_1[0x1].field20_0x26,struct_1->field4_0x8);
  mem_op_1000_179c(0x28,param_2);
  uVar1 = param_2 | param_1;
  if (uVar1 != 0x0) {
    pass1_1018_ec74(uVar1,param_1,param_2,struct_1->field4_0x8);
    struct_1[0x1].field18_0x22 = (astruct_666 *)param_1;
    struct_1[0x1].field19_0x24 = uVar1;
    return;
  }
  (u32)&struct_1[0x1].field18_0x22 = 0x0;
  return;
}



void pass1_1018_eb3e(u32 param_1)

{
  u32 *puVar1;
  u16 uVar2;
  code **ppcVar3;
  i16 iVar4;
  u16 uVar5;
  i16 iVar6;
  u16 uVar7;

  uVar7 = (param_1 >> 0x10);
  iVar6 = (int)param_1;
  puVar1 = (u32 *)(iVar6 + 0xee);
  uVar2 = (iVar6 + 0xf0);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  if (*(i32 *)(iVar6 + 0xf6) != 0x0) {
    if (param_1 == 0x0) {
      iVar4 = 0x0;
      uVar5 = 0x0;
    }
    else {
      iVar4 = iVar6 + 0xe2;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6((u32)(iVar6 + 0xf6),(StructD *)CONCAT22(uVar5,iVar4));
  }
  destroy_win_1008_628e(param_1);
  return;
}



StructD * pass1_1018_eb9c(StructD *param_1,u8 param_2)

{
  param_1 = (StructD *)((u32)param_1 & 0xffff0000 | (u32)((int)param_1 - 0xe2));
  pass1_1018_e9de(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_1);
  }
  return param_1;
}



StructD * FUN_1018_ebaa(u16 param_1,StructD *param_2,u8 param_3)

{
  pass1_1018_e9de(param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((char *)param_2);
  }
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void pass1_1018_ec74(u16 param_1,astruct_661 *param_2,i16 param_3,u16 param_4)

{
  u32 *puVar1;
  code **ppcVar2;
  u32 uVar3;
  u16 uVar4;
  i16 iVar5;
  u16 in_register_0000000a;
  astruct_57 *paVar6;
  u16 uVar7;
  u16 unaff_SI;
  u16 *puVar8;
  u32 *puVar9;
  u32 uVar10;
  u16 in_stack_0000fe78;
  u16 in_stack_0000fe8a;
  u16 in_stack_0000ff9c;
  u16 in_stack_0000ffa2;
  u16 in_stack_0000ffa6;
  u16 in_stack_0000ffae;
  u16 in_stack_0000ffb4;
  u16 in_stack_0000ffb8;
  u32 in_stack_0000ffba;
  u16 uVar11;
  u16 uVar12;
  astruct_661 *paVar13;
  i16 iVar14;

  set_struct_op_1020_921c(param_1,(StructA *)CONCAT22(param_3,param_2),param_4,in_stack_0000ffba);
  param_2->field15_0x14 = NULL;
  pass1_1008_3e38((astruct_19 *)CONCAT22(param_3,&param_2->field_0x18));
  puVar8 = pass1_1008_3e38((astruct_19 *)CONCAT22(param_3,&param_2->field_0x1e));
  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,(int)((u32)puVar8 >> 0x10));
  (u32)&param_2->field28_0x24 = 0x0;
  CONCAT22(param_3,param_2) = 0x1cc;
  param_2->field2_0x2 = 0x1020;
  puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(unaff_SI,0x28),in_stack_0000fe8a,
                           in_stack_0000ffae,in_stack_0000ffb4,in_stack_0000ffb8);
  uVar7 = ((u32)paVar6 >> 0x10);
  &param_2->field15_0x14 = (int)puVar9;
  uVar4 = ((u32)puVar9 >> 0x10);
  ((int)&param_2->field15_0x14 + 0x2) = uVar4;
  uVar12 = 0x0;
  uVar11 = &param_2->field15_0x14;
  ppcVar2 = (code **)((int)*param_2->field15_0x14 + 0x4);
  paVar13 = param_2;
  iVar14 = param_3;
  (**ppcVar2)();
  param_2->field5_0x6 = param_2->field15_0x14;
  puVar1 = param_2->field15_0x14;
  pass1_1010_2b50(puVar1,((u32)puVar1 >> 0x10),(u16 *)CONCAT22(param_3,&param_2->field_0x18));
  uVar10 = pass1_1010_2b66((u32)param_2->field15_0x14);
  iVar5 = (int)(uVar10 >> 0x10);
  paVar6 = (astruct_57 *)CONCAT22(uVar7,iVar5);
  param_2->field28_0x24 = (int)uVar10;
  param_2->field29_0x26 = iVar5;
  puVar1 = param_2->field15_0x14;
  puVar1 = (u32 *)(u32)((int)puVar1 + 0xa);
  uVar3 = CONCAT22(param_3,&param_2->field_0xa);
  uVar7 = SUB42(puVar1,0x0);
  ppcVar2 = (code **)((int)*puVar1 + 0x8);
  (**ppcVar2)(0x1010,uVar7,(int)((u32)puVar1 >> 0x10),uVar3,uVar11,uVar4,uVar12,paVar13,iVar14);
  param_2->field14_0x12 = (int)uVar3;
  draw_op_1020_9364((StructA *)CONCAT22(param_3,param_2));
  puVar9 = mixed_1010_20ba(paVar6,_u16_1050_0ed0,(u8 **)CONCAT22(uVar7,0x48),in_stack_0000fe78,in_stack_0000ff9c,
                           in_stack_0000ffa2,in_stack_0000ffa6);
  pass1_1008_3f62((u16 *)CONCAT22(param_3,&param_2->field_0x1e),
                  (u16 *)((u32)puVar9 & 0xffff0000 | (u32)((int)puVar9 + 0xe)));
  pass1_1008_3f32((i16 *)CONCAT22(param_3,&param_2->field_0x18),(i16 *)CONCAT22(param_3,&param_2->field_0x1e));
  return;
}



void pass1_1018_ed98(StructD *param_1)

{
  StructD *iVar1;
  u16 uVar1;

  uVar1 = ((u32)param_1 >> 0x10);
  iVar1 = (StructD *)param_1;
  param_1->address_offset_field_0x0 = 0x1cc;
  iVar1->address_offset_field_0x2 = 0x1020;
  if (iVar1->field12_0x14 != 0x0) {
    pass1_1010_1ea6(iVar1->field12_0x14,(StructD *)((u32)param_1 & 0xffff | (u32)uVar1 << 0x10));
    pass1_1010_1dda(iVar1->field12_0x14);
  }
  palette_op_1020_92c4(param_1);
  return;
}



void invalidate_rect_1018_edd8(u32 param_1,i16 param_2)

{
  i16 iVar1;
  u16 uVar2;
  u32 uVar3;
  RECT16 local_16;
  i16 iStack18;
  i16 iStack16;
  u32 uStack14;
  u16 uStack10;
  u16 uStack8;
  i16 local_6;
  i16 local_4;

  iVar1 = (int)param_1;
  uVar2 = (param_1 >> 0x10);
  if (param_2 == 0x1) {
    (u32)(iVar1 + 0x14) = 0x0;
    return;
  }
  if (param_2 != 0xc) {
    return;
  }
  pass1_1008_3e94((u16 *)(param_1 & 0xffff0000 | (u32)(iVar1 + 0x18)),(u16 *)CONCAT22(0x1050,&local_6),
                  (char *)CONCAT22(0x1050,&local_4));
  uVar3 = pass1_1010_2b66((u32)(iVar1 + 0x14));
  uStack8 = (uVar3 >> 0x10);
  uStack10 = uVar3;
  uStack14 = pass1_1008_4772((astruct_76 *)(uVar3 & 0xffff | (u32)uStack8 << 0x10));
  uVar2 = (uStack14 >> 0x10);
  local_16.x = local_4;
  local_16.y = local_6;
  iStack18 = local_4 + ((int)uStack14 + 0x4);
  iStack16 = local_6 + ((int)uStack14 + 0x8);
  InvalidateRect16(0x0,&local_16,(HWND16)&DAT_1050_1050);
  return;
}
