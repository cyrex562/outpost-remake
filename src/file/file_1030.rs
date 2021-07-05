
fn file_1030_1730(param_1: u32,param_2: u32)
{
  let uVar1: u16;
  let BVar2: bool;
  let uVar3: u16;
  
  uVar1 = (param_1 >> 0x10);
  uVar3 = (param_2 >> 0x10);
  BVar2 = read_file_1008_7dee(param_2,uVar3,param_1 + 0x4,0x0,uVar1,0x4,
                              0x1008);
  if (BVar2 != 0x0) {
    BVar2 = read_file_1008_7dee(param_2,uVar3,param_1 + 0x8,0x0,uVar1,0x4,
                                0x1008);
    if (BVar2 != 0x0) {
      return;
    }
  }
  ctx.PTR_LOOP_1050_0310 = 0x6d2;
  return;
}



fn file_1030_19b4(param_1: u32,param_2: u32,param_3: i16,param_4: u16,param_5: u16)
{
  long *plVar1;
  
  file_1030_1730(param_1,param_2);
  if (param_3 != 0x0) {
    plVar1 = (long *)(param_1 & 0xffff0000 | (param_1 + 0xc));
    file_1008_76e4(param_2,plVar1,0x1008,param_5,param_4);
    if (plVar1 == 0x0) {
      ctx.PTR_LOOP_1050_0310 = 0x6d2;
      return;
    }
  }
  return;
}


u16 
file_1030_1b18(param_1: u32,param_2: u32,param_3: i16,param_4: *mut u8,
              param_5: u16)

{
  let uVar1: u32;
  let piVar2: *mut i16;
  let uVar3: u16;
  let uVar4: u16;
  let BVar5: bool;
  let uVar6: u16;
  let puVar7: *mut u8
  astruct_368 *iVar7;
  let uVar8: u16;
  let uVar9: u16;
  astruct_370 *iVar10;
  astruct_369 *iVar9;
  
  iVar10 = (astruct_370 *)param_1;
  uVar9 = (param_1 >> 0x10);
  file_1030_19b4(param_1,param_2,param_3,param_4,param_5);
  if (param_3 != 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_4,0x1000);
      ctx.PTR_LOOP_1050_5f2e = param_4;
    }
    else {
    }
    uVar4 = fn_ptr_op_1000_1708(0x6,0x0,0x1,PTR_LOOP_1050_5f2c,
                                ctx.PTR_LOOP_1050_5f2e,0x1000);
    &iVar10->field_0x10 = uVar4;
    (&iVar10->field_0x10 + 0x2) = ctx.PTR_LOOP_1050_5f2e;
    puVar7 = *(uchar **)(&iVar10->field_0x10 + 0x2);
    uVar4 = (param_2 >> 0x10);
    BVar5 = read_file_1008_7dee(param_2,uVar4,&iVar10->field_0x10,0x0,
                                puVar7,0x2,0x1008);
    if (BVar5 != 0x0) {
      piVar2 = iVar10->field_0x10;
      if (*piVar2 == 0x0) {
        return 0x1;
      }
      uVar3 = *piVar2 * 0x2;
      uVar6 = uVar3;
      mem_op_1000_179c(uVar3,puVar7,0x1000);
      piVar2 = iVar10->field_0x10;
      uVar8 = (piVar2 >> 0x10);
      iVar7 = (astruct_368 *)piVar2;
      iVar7->field_0x2 = uVar6;
      iVar7->field_0x4 = puVar7;
      piVar2 = iVar10->field_0x10;
      uVar1 = (piVar2 + 0x2);
      BVar5 = read_file_1008_7dee(param_2,uVar4,uVar1,0x0,
                                  (uVar1 >> 0x10),uVar3,0x1008);
      if (BVar5 != 0x0) {
        return 0x1;
      }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
  }
  return 0x0;
}


fn write_to_file_1030_32e4(param_1: u32,param_2: u32,param_3: u16)
{
  let uVar1: u16;
  let iVar2: i16;
  let BVar3: bool;
  let uVar4: u16;
  let uVar5: u16;
  let local_16: [u32;0x2];
  let local_c: u16;
  let local_a: [u32;0x2];
  
  iVar2 = param_1;
  uVar1 = (param_1 >> 0x10);
  uVar4 = param_2;
  uVar5 = (param_2 >> 0x10);
  BVar3 = write_to_file_1008_7e1c(uVar4,uVar5,iVar2 + 0x4,uVar1,0x16c,0x1008);
  if (BVar3 != 0x0) {
    BVar3 = write_to_file_1008_7e1c(uVar4,uVar5,iVar2 + 0x174,uVar1,&DAT_0000_000c,0x1008)
    ;
    if (BVar3 != 0x0) {
      BVar3 = write_to_file_1008_7e1c
                        (uVar4,uVar5,iVar2 + 0x180,uVar1,&DAT_0000_000c,0x1008);
      if (BVar3 != 0x0) {
        BVar3 = write_to_file_1008_7e1c
                          (uVar4,uVar5,iVar2 + 0x18c,uVar1,0x18,0x1008);
        if (BVar3 != 0x0) {
          local_c = (iVar2 + 0x1a8);
          BVar3 = write_to_file_1008_7e1c
                            (uVar4,uVar5,&local_c,param_3,0x2,0x1008);
          if (BVar3 != 0x0) {
            local_16[0] = (iVar2 + 0x1aa);
            BVar3 = write_to_file_1008_7e1c
                              (uVar4,uVar5,local_16,param_3,0x4,0x1008);
            if (BVar3 != 0x0) {
              local_a[0] = (iVar2 + 0x170);
              BVar3 = write_to_file_1008_7e1c
                                (uVar4,uVar5,local_a,param_3,0x4,0x1008);
              if (BVar3 != 0x0) {
                local_c = (iVar2 + 0x1ae);
                BVar3 = write_to_file_1008_7e1c
                                  (uVar4,uVar5,&local_c,param_3,0x2,0x1008
                                  );
                if (BVar3 != 0x0) {
                  return;
                }
              }
            }
          }
        }
      }
    }
  }
  ctx.PTR_LOOP_1050_0310 = 0x6d0;
  return;
}



fn read_file_1030_33f0(param_1: u32,param_2: u32)
{
  let uVar1: u16;
  astruct_430 *iVar2;
  let BVar2: bool;
  let uVar3: u16;
  let uVar4: u16;
  
  iVar2 = (astruct_430 *)param_1;
  iVar2 = (astruct_430 *)&iVar2->field_0x4;
  uVar1 = (param_1 >> 0x10);
  uVar3 = param_2;
  uVar4 = (param_2 >> 0x10);
  BVar2 = read_file_1008_7dee(uVar3,uVar4,iVar2,0x0,uVar1,0x16c,0x1008);
  if (((((BVar2 != 0x0) &&
        (BVar2 = read_file_1008_7dee(uVar3,uVar4,&iVar2->field_0x174,0x0,uVar1,0xc,0x1008)
        , BVar2 != 0x0)) &&
       (BVar2 = read_file_1008_7dee(uVar3,uVar4,&iVar2->field_0x180,0x0,uVar1,0xc,0x1008),
       BVar2 != 0x0)) &&
      ((BVar2 = read_file_1008_7dee(uVar3,uVar4,&iVar2->field_0x18c,0x0,uVar1,0x18,0x1008)
       , BVar2 != 0x0 &&
       (BVar2 = read_file_1008_7dee(uVar3,uVar4,&iVar2->field_0x1a8,0x0,uVar1,0x2,0x1008),
       BVar2 != 0x0)))) &&
     (BVar2 = read_file_1008_7dee(uVar3,uVar4,&iVar2->field_0x1aa,0x0,uVar1,0x4,0x1008),
     BVar2 != 0x0)) {
    if (ctx.PTR_LOOP_1050_0312 < 0x2) {
      return;
    }
    BVar2 = read_file_1008_7dee(uVar3,uVar4,&iVar2->field_0x170,0x0,uVar1,0x4,0x1008);
    if ((BVar2 != 0x0) &&
       (BVar2 = read_file_1008_7dee(uVar3,uVar4,&iVar2->field_0x1ae,0x0,uVar1,0x2,0x1008),
       BVar2 != 0x0)) {
      return;
    }
  }
  ctx.PTR_LOOP_1050_0310 = 0x6d2;
  return;
}


u16_t 
read_file_1030_4e70(param_1: u32,param_2: *mut u32,byte **param_3,param_4: i32,
                   param_5: u16)

{
  let uVar1: u16;
  HFILE16 HVar2;
  u16_t uVar3;
  let unaff_SS: u16;
  let uVar4: u32;
  let lVar5: i32;
  byte *pbStack60;
  let lStack56: i32;
  let uStack20: u32;
  
  *param_3 = 0x0;
  *param_2 = 0x0;
  if (param_4 != 0x0) {
    uVar4 = pass1_1030_5164(param_1,param_4,unaff_SS);
    param_5 = (u16_t)(uVar4 >> 0x10);
    uVar1 = dos3_call_1000_51aa(&stack0xfffe);
    if (uVar1 == 0x0) {
      *param_2 = uStack20;
      HVar2 = _lopen16((LPCSTR)&ctx.PTR_LOOP_1050_1000,0x0);
      if (HVar2 != 0xffff) {
        lVar5 = mem_op_1000_0a48(0x1,*param_2,(*param_2 >> 0x10),
                                 _PTR_LOOP_1050_5f2c,0x1000);
        param_5 = (u16_t)(lVar5 >> 0x10);
        param_3 = lVar5;
        (param_3 + 0x2) = param_5;
        if ((param_5 | param_3) != 0x0) {
          lStack56 = WIN16_hread(0x1000,(SEGPTR)*param_2,
                                 CONCAT22(*param_3,(*param_2 >> 0x10)));
          uVar3 = (u16_t)(lStack56 >> 0x10);
          _lclose16((HFILE16)s_tile2_bmp_1050_1538);
          pbStack60 = *param_3;
          while (lStack56 != 0x0) {
            if (((*pbStack60 + 0x608b) & 0x20) == 0x0) {
              *pbStack60 = *pbStack60 + 0x80;
            }
            pbStack60 = (pbStack60 & 0xffff0000 |
                                (pbStack60 + 0x1));
            lStack56 = lStack56 + -0x1;
          }
          return uVar3;
        }
      }
    }
  }
  return param_5;
}


fn file_1030_581e(param_1: u32,param_2: u32,param_3: i16,uchar *param_4,param_5: u16)
{
  let piVar1: *mut i16;
  let iVar2: i16;
  let uVar3: u32;
  let uVar4: u16;
  let BVar5: bool;
  let puVar6: *mut u8;
  let uVar7: u16;
  let uVar8: u32;
  let puVar9: *mut u8
  astruct_380 *iVar9;
  let uVar10: u16;
  ulet in_AF: u8;
  let uVar11: u16;
  let uVar12: u16;
  let uStack1040: u32;
  let iStack1036: i16;
  let local_408: [u8;400];
  let uStack8: u32;
  let local_4: i16;
  astruct_381 *iVar12;
  
  iVar12 = (astruct_381 *)param_1;
  uVar12 = (param_1 >> 0x10);
  file_1030_19b4(param_1,param_2,param_3,param_4,param_5);
  if (param_3 != 0x0) {
    if (_PTR_LOOP_1050_5f2c == 0x0) {
      ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_4,0x1000);
      ctx.PTR_LOOP_1050_5f2e = param_4;
    }
    else {
    }
    uVar4 = fn_ptr_op_1000_1708(0x20,0x0,0x1,PTR_LOOP_1050_5f2c,
                                ctx.PTR_LOOP_1050_5f2e,0x1000);
    puVar9 = (ctx.PTR_LOOP_1050_5f2e | uVar4);
    if (puVar9 == 0x0) {
      uVar4 = 0x0;
      puVar9 = 0x0;
    }
    else {
      pass1_1030_84ae(CONCAT22(ctx.PTR_LOOP_1050_5f2e,uVar4));
    }
    iVar12->field_0x10 = uVar4;
    iVar12->field_0x12 = puVar9;
    uVar4 = param_2;
    uVar11 = (param_2 >> 0x10);
    BVar5 = read_file_1008_7dee(uVar4,uVar11,&local_4,0x0,param_5,0x2,0x1008);
    if (BVar5 != 0x0) {
      uVar8 = (_PTR_LOOP_1050_65e2 + 0x52);
      uStack8 = uVar8;
      pass1_1030_4782(param_5,in_AF,puVar9,uVar8,(uVar8 >> 0x10),
                      0x0,0x1,local_4);
      iVar12->field_0x10 = uVar8;
      iVar12->field_0x12 = puVar9;
      BVar5 = read_file_1008_7dee(uVar4,uVar11,iVar12->field_0x10 + 0x2,0x0,puVar9
                                  ,0x2,0x1008);
      if (BVar5 != 0x0) {
        puVar6 = local_408;
        read_file_1008_7c6e(uVar4,uVar11,CONCAT22(param_5,puVar6),0x1008);
        if (puVar6 != 0x0) {
          uVar8 = &iVar12->field_0x10;
          fn_ptr_1000_17ce(*(astruct_18 **)(uVar8 + 0x4),0x1000);
          uVar7 = str_op_1008_60e8(CONCAT22(param_5,local_408),puVar9);
          uVar8 = &iVar12->field_0x10;
          uVar10 = (uVar8 >> 0x10);
          iVar9 = (astruct_380 *)uVar8;
          iVar9->field_0x4 = uVar7;
          iVar9->field_0x6 = puVar9;
          uVar8 = &iVar12->field_0x10;
          BVar5 = read_file_1008_7dee(uVar4,uVar11,uVar8 + 0x1a,0x0,
                                      (uVar8 >> 0x10),0x2,0x1008);
          if (BVar5 != 0x0) {
            uVar8 = &iVar12->field_0x10;
            iVar2 = (uVar8 + 0x1a);
            uVar7 = iVar2 * 0x6;
            mem_op_1000_179c(uVar7,puVar9,0x1000);
            uStack1040 = CONCAT22(puVar9,uVar7);
            if ((puVar9 | uVar7) == 0x0) {
              uVar8 = &iVar12->field_0x10;
              (uVar8 + 0x16) = 0x0;
            }
            else {
              pass1_1000_5586((uchar *)0x3e38,0x1008,iVar2,0x6,uVar7,puVar9);
              uVar8 = &iVar12->field_0x10;
              (uVar8 + 0x16) = uStack1040;
            }
            for (iStack1036 = 0x0; uVar8 = &iVar12->field_0x10,
                piVar1 = (uVar8 + 0x1a),
                *piVar1 != iStack1036 && iStack1036 <= *piVar1; iStack1036 += 0x1) {
              uVar8 = &iVar12->field_0x10;
              uVar3 = (uVar8 + 0x16);
              BVar5 = read_file_1008_7bc8(param_2,
                                                  (uVar3 & 0xffff0000 |
                                                  (uVar3 +
                                                               iStack1036 * 0x6)),0x1008,
                                          param_5);
              if (BVar5 == 0x0) goto LAB_1030_58a7;
            }
            BVar5 = read_file_1008_7bc8(param_2,
                                                (param_1 & 0xffff0000 |
                                                &iVar12->field_0x14),0x1008,
                                        param_5);
            if ((BVar5 != 0x0) &&
               (BVar5 = read_file_1008_7dee(uVar4,uVar11,&iVar12->field_0x1c,0x0,uVar12,
                                            0x2,0x1008), BVar5 != 0x0)) {
              return;
            }
          }
        }
      }
    }
//LAB_1030_58a7:
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
  }
  return;
}


bool 
read_file_1030_5c52(param_1: u32,param_2: u32,param_3: u16,
                   param_4: u16)

{
  let BVar1: bool;
  let uVar2: u16;
  
  uVar2 = (param_2 >> 0x10);
  read_file_1008_7cfe(param_2,uVar2,0x9,0x1008,param_4);
  if (param_3 != 0x0) {
    BVar1 = read_file_1008_7dee(param_2,uVar2,param_1,0x0,
                                (param_1 >> 0x10),0x24,0x1008);
    if (BVar1 == 0x0) {
      ctx.PTR_LOOP_1050_0310 = 0x6d2;
      return BVar1;
    }
    param_3 = 0x1;
  }
  return param_3;
}


fn file_1030_778c(param_1: u32,param_2: u32,param_3: i16,uchar *param_4,param_5: u16)
{
  let lVar1: i32;
  code **ppcVar2;
  astruct_387 *iVar3;
  let BVar3: bool;
  let iVar6: i16;
  long *plVar7;
  let puVar8: u32;
  let extraout_DX: u16;
  let uVar9: u16;
  let puVar10: *mut u8
  let extraout_DX_00: u16;
  astruct_389 *iVar4;
  astruct_391 *iVar5;
  let uVar11: u16;
  let uVar12: u16;
  let uVar13: u16;
  let uVar14: u16;
  let local_56: [u16;0x2];
  let uStack82: u16;
  astruct_99 *paStack74;
  let local_46: [u16;0x2];
  let local_42: [u16;0x2];
  let local_3e: [u32;0x3];
  astruct_99 *paStack50;
  let local_2e: u16;
  astruct_99 *paStack44;
  let local_28: [u16;0x2];
  let local_24: [u16;0x2];
  let local_20: [u16;0x9];
  let uStack14: u16;
  let local_4: u16;
  astruct_388 *uVar5;
  astruct_390 *uVar8;
  
  file_1030_1730(param_1,param_2);
  if (param_3 != 0x0) {
    iVar3 = (astruct_387 *)param_1;
    iVar3 = (astruct_387 *)&iVar3->field_0xc;
    BVar3 = read_file_1008_7bc8(param_2,(param_1 & 0xffff0000 | ZEXT24(iVar3)),
                                0x1008,param_5);
    if (BVar3 != 0x0) {
      uVar13 = param_2;
      uVar14 = (param_2 >> 0x10);
      BVar3 = read_file_1008_7dee(uVar13,uVar14,&local_4,0x0,param_5,0x2,0x1008);
      if (BVar3 != 0x0) {
        uVar11 = (param_1 >> 0x10);
        iVar3->field_0x12 = local_4;
        BVar3 = read_file_1008_7dee(uVar13,uVar14,&local_4,0x0,param_5,0x2,0x1008)
        ;
        if (BVar3 != 0x0) {
          iVar3->field_0x14 = local_4;
          BVar3 = read_file_1008_7dee(uVar13,uVar14,&iVar3->field_0x16,0x0,uVar11,0x4,
                                      0x1008);
          if (BVar3 != 0x0) {
            plVar7 = (long *)(param_1 & 0xffff0000 | &iVar3->field_0x1e);
            file_1008_76e4(param_2,plVar7,0x1008,param_5,param_4);
            if ((((plVar7 != 0x0) &&
                 (iVar6 = file_1008_77cc(param_2,(long *)(param_1 & 0xffff0000 |
                                                         &iVar3->field_0x22),
                                         param_4,0x1008,param_5), iVar6 != 0x0)) &&
                (iVar6 = file_1008_77cc(param_2,(long *)(param_1 & 0xffff0000 |
                                                        &iVar3->field_0x26),
                                        param_4,0x1008,param_5), iVar6 != 0x0)) &&
               (BVar3 = read_file_1008_7dee(uVar13,uVar14,&iVar3->field_0x2a,0x0,
                                            uVar11,0x4,0x1008), BVar3 != 0x0)) {
              if (iVar3->field_0x2a != 0x0) {
                lVar1 = iVar3->field_0x2a;
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2,lVar1,
                                (lVar1 >> 0x10));
                iVar3->field_0x2e = BVar3;
                iVar3->field_0x30 = param_4;
              }
              if (ctx.PTR_LOOP_1050_0312 < 0x2) {
                return;
              }
              BVar3 = read_file_1008_7dee(uVar13,uVar14,&iVar3->field_0x32,0x0,uVar11,0x2,
                                          0x1008);
              if ((BVar3 != 0x0) &&
                 (BVar3 = read_file_1008_7dee(uVar13,uVar14,&iVar3->field_0x34,0x0,uVar11,
                                              0x2,0x1008), BVar3 != 0x0)) {
                puVar8 = (param_1 & 0xffff0000 | &iVar3->field_0x36)
                ;
                pass1_1008_766e(param_2,puVar8,param_5,0x1008,param_4);
                if ((puVar8 != 0x0) &&
                   (BVar3 = read_file_1008_7dee(uVar13,uVar14,local_20,0x0,param_5
                                                ,0x2,0x1008), BVar3 != 0x0)) {
                  for (uStack14 = 0x0; uStack14 < local_20[0]; uStack14 += 0x1) {
                    local_3e[0] = _PTR_LOOP_1050_68a2;
                    paStack50 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
                    uVar9 = (paStack50 >> 0x10);
                    uVar5 = (astruct_388 *)paStack50;
                    puVar10 = (uVar9 | uVar5);
                    if (puVar10 == 0x0) {
                      paStack44 = 0x0;
                    }
                    else {
                      paStack50->field_0x0 = 0x389a;
                      uVar5->field_0x2 = 0x1008;
                      uVar5->field_0x4 = 0x0;
                      uVar5->field_0x6 = 0x0;
                      uVar5->field_0x8 = 0x0;
                      uVar5->field_0xa = 0x0;
                      uVar5->field_0xc = 0x0;
                      paStack50->field_0x0 = 0x56ce;
                      uVar5->field_0x2 = 0x1018;
                      paStack44 = paStack50;
                    }
                    BVar3 = read_file_1008_7dee(uVar13,uVar14,local_28,0x0,param_5
                                                ,0x2,0x1008);
                    if (((BVar3 == 0x0) ||
                        (BVar3 = read_file_1008_7dee(uVar13,uVar14,local_24,0x0,
                                                     param_5,0x2,0x1008), BVar3 == 0x0))
                       || ((BVar3 = read_file_1008_7dee(uVar13,uVar14,&local_2e,
                                                        0x0,param_5,0x2,0x1008),
                           BVar3 == 0x0 ||
                           ((BVar3 = read_file_1008_7dee(uVar13,uVar14,
                                                         paStack44 + 0xa,0x0,
                                                         (paStack44 >> 0x10
                                                                 ),0x2,0x1008),
                            BVar3 == 0x0 ||
                            (BVar3 = read_file_1008_7dee(uVar13,uVar14,
                                                         paStack44 + 0xc,0x0,
                                                         (paStack44 >> 0x10
                                                                 ),0x2,0x1008),
                            BVar3 == 0x0)))))) goto LAB_1030_77be;
                    uVar12 = (paStack44 >> 0x10);
                    iVar4 = (astruct_389 *)paStack44;
                    iVar4->field_0x4 = local_28[0];
                    iVar4->field_0x6 = local_24[0];
                    iVar4->field_0x8 = local_2e;
                    if (iVar3->field_0x3a == 0x0) {
                      uVar9 = local_2e;
                      mem_op_1000_179c(0xc,puVar10,0x1000);
                      paStack50 = CONCAT22(puVar10,uVar9);
                      if ((puVar10 | uVar9) == 0x0) {
                        iVar3->field_0x3a = 0x0;
                      }
                      else {
                        set_struct_1008_574a((astruct_21 *)CONCAT22(puVar10,uVar9));
                        &iVar3->field_0x3a = uVar9;
                        (&iVar3->field_0x3a + 0x2) = extraout_DX;
                      }
                    }
                    ppcVar2 = (code **)(*iVar3->field_0x3a + 0x8);
                    (**ppcVar2)();
                  }
                  BVar3 = read_file_1008_7dee(uVar13,uVar14,local_56,0x0,param_5,
                                              0x2,0x1008);
                  if (BVar3 != 0x0) {
                    uStack82 = 0x0;
                    while( true ) {
                      if (local_56[0] <= uStack82) {
                        return;
                      }
                      paStack44 = _PTR_LOOP_1050_68a2;
                      paStack50 = pass1_1000_07fc(0x1000,_PTR_LOOP_1050_68a2);
                      uVar9 = (paStack50 >> 0x10);
                      uVar8 = (astruct_390 *)paStack50;
                      puVar10 = (uVar9 | uVar8);
                      if (puVar10 == 0x0) {
                        paStack74 = 0x0;
                      }
                      else {
                        paStack50->field_0x0 = 0x389a;
                        uVar8->field_0x2 = 0x1008;
                        uVar8->field_0x4 = 0x0;
                        uVar8->field_0x6 = 0x0;
                        uVar8->field_0x8 = 0x0;
                        uVar8->field_0xa = 0x0;
                        uVar8->field_0xc = 0x0;
                        paStack50->field_0x0 = 0x56ce;
                        uVar8->field_0x2 = 0x1018;
                        paStack74 = paStack50;
                      }
                      BVar3 = read_file_1008_7dee(uVar13,uVar14,local_46,0x0,
                                                  param_5,0x2,0x1008);
                      if ((((BVar3 == 0x0) ||
                           (BVar3 = read_file_1008_7dee(uVar13,uVar14,local_42,0x0
                                                        ,param_5,0x2,0x1008), BVar3 == 0x0
                           )) || (BVar3 = read_file_1008_7dee(uVar13,uVar14,
                                                              local_3e,0x0,param_5
                                                              ,0x2,0x1008), BVar3 == 0x0))
                         || ((BVar3 = read_file_1008_7dee(uVar13,uVar14,
                                                          paStack74 + 0xa,0x0,
                                                          (paStack74 >>
                                                                  0x10),0x2,0x1008),
                             BVar3 == 0x0 ||
                             (BVar3 = read_file_1008_7dee(uVar13,uVar14,
                                                          paStack74 + 0xc,0x0,
                                                          (paStack74 >>
                                                                  0x10),0x2,0x1008),
                             BVar3 == 0x0)))) break;
                      uVar12 = (paStack74 >> 0x10);
                      iVar5 = (astruct_391 *)paStack74;
                      iVar5->field_0x4 = local_46[0];
                      iVar5->field_0x6 = local_42[0];
                      iVar5->field_0x8 = local_3e[0];
                      if (iVar3->field_0x3e == 0x0) {
                        mem_op_1000_179c(0xc,puVar10,0x1000);
                        paStack50 = CONCAT22(puVar10,local_3e[0]);
                        if ((puVar10 | local_3e[0]) == 0x0) {
                          iVar3->field_0x3e = 0x0;
                        }
                        else {
                          set_struct_1008_574a
                                    ((astruct_21 *)CONCAT22(puVar10,local_3e[0]));
                          &iVar3->field_0x3e = local_3e[0];
                          (&iVar3->field_0x3e + 0x2) = extraout_DX_00;
                        }
                      }
                      ppcVar2 = (code **)(*iVar3->field_0x3e + 0x8);
                      (**ppcVar2)();
                      uStack82 += 0x1;
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
//LAB_1030_77be:
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
  }
  return;
}
