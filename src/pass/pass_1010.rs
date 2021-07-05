
fn pass1_1010_0000(astruct_645 *param_1,param_2: u16,param_3: u16,param_4: u16) -> u32

{
  let unaff_DI: i16;
  astruct_79 *paVar1;
  let puVar2: *mut u16;
  let puVar3: *mut u16;
  let uVar4: u16;
  let puVar5: *mut u16;
  let uVar6: u16;
  
                    // Segment:    3
                    // Offset:     00015420
                    // Length:     ee9f
                    // Min Alloc:  ee9f
                    // Flags:      0d50
                    //     Code
                    //     Moveable
                    //     Preload
                    //     Impure (Non-shareable)
                    // 
  paVar1 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x0;
  param_1->field_0xc = 0x0;
  CONCAT22(param_2,param_1) = 0x2c8;
  param_1->field_0x2 = 0x1010;
  puVar5 = &param_1->field_0xa;
  puVar3 = &param_1->field_0xc;
  uVar4 = param_2;
  uVar6 = param_2;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x48,param_4,
                           (paVar1 >> 0x10),unaff_DI);
  pass1_1008_3e94((u16 *)(puVar2 & 0xffff0000 | (puVar2 + 0xe)),
                  CONCAT22(uVar4,puVar3),CONCAT22(uVar6,puVar5));
  return CONCAT22(param_2,param_1);
}



fn pass1_1010_0052(param_1: *mut u16,param_2: u16)
{
  *param_1 = 0x2c8;
  (param_1 + 0x2) = 0x1010;
  pass1_1010_1d80(param_1,param_2);
  return;
}



fn pass1_1010_01f8(param_1: u32,param_2: u32,param_3: i16)
{
  let iVar1: i16;
  let iVar2: i16;
  let uVar3: u16;
  let uVar4: u16;
  
  iVar2 = (param_3 * 0x4 + 0xe02) * 0x4;
  iVar1 = (iVar2 + 0xdfc);
  uVar3 = (param_1 >> 0x10);
  uVar4 = (param_2 >> 0x10);
  (param_2 + 0x6) =
       (param_3 * 0x4 + 0xe04) * 0x28 + (iVar2 + 0xdfa) +
       (param_1 + 0xa);
  (param_2 + 0x8) = (param_1 + 0xc) + iVar1;
  return;
}



fn pass1_1010_02a2(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_0052(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1010_0350(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_474 *iVar4;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_474 *)param_1;
  *param_1 = 0xe98;
  iVar4->field_0x2 = 0x1010;
  puVar1 = iVar4->field_0xa;
  uVar2 = iVar4->field_0xc;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}



fn pass1_1010_038e(param_1: u32,param_2: i16,param_3: u16)
{
  let bVar1: bool;
  astruct_707 *iVar2;
  let uVar2: u16;
  
  bVar1 = false;
  iVar2 = (astruct_707 *)param_1;
  uVar2 = (param_1 >> 0x10);
  if (param_2 != 0x0) {
    if (iVar2->field_0x18 == 0x0) {
      iVar2->field_0x12 = DAT_1050_0e28;
      iVar2->field_0x14 = ctx.PTR_LOOP_1050_0e30;
      iVar2->field_0x16 = ctx.PTR_LOOP_1050_0ea8;
      DAT_1050_0e28 = 0x0;
      ctx.PTR_LOOP_1050_0e30 = 0x0;
      ctx.PTR_LOOP_1050_0ea8 = 0x0;
      iVar2->field_0x18 = 0x1;
      bVar1 = true;
      goto LAB_1010_0404;
    }
  }
  if (param_2 == 0x0) {
    if (iVar2->field_0x18 != 0x0) {
      DAT_1050_0e28 = iVar2->field_0x12;
      ctx.PTR_LOOP_1050_0e30 = iVar2->field_0x14;
      ctx.PTR_LOOP_1050_0ea8 = iVar2->field_0x16;
      iVar2->field_0x18 = 0x0;
      bVar1 = true;
    }
  }
//LAB_1010_0404:
  if (bVar1) {
    pass1_1010_1f62(param_3,param_1,0x3);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_041a(void) -> bool

{
  let uVar1: u32;
  
  uVar1 = pass1_1030_8326();
  if (((uVar1 >> 0x10) == 0x0) && (uVar1 < 0x64)) {
    return 0x0;
  }
  return 0x1;
}



fn pass1_1010_043a(param_1: u32,param_2: i32,param_3: i16,param_4: u16)
{
  let puVar1: u32;
  code **ppcVar2;
  astruct_225 *puVar3;
  let extraout_DX: u16;
  let uVar3: u16;
  astruct_226 *iVar4;
  astruct_227 *iVar5;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let puStack18: *mut u16;
  let puStack14: *mut u16;
  let local_a: [u8;8];
  
  iVar4 = (astruct_226 *)param_1;
  uVar4 = (param_1 >> 0x10);
  if (param_3 == 0xc) {
    if (iVar4->field_0xe != 0x0) {
      return;
    }
    iVar4->field_0xe = 0x1;
  }
  else {
    if (param_3 == 0xd) {
      if (iVar4->field_0x10 != 0x0) {
        return;
      }
      iVar4->field_0x10 = 0x1;
    }
    else {
      if (param_3 == 0x12) {
        return;
      }
    }
  }
  pass1_1010_089e(param_4,param_1,0x1,0x6);
  pass1_1008_5784(CONCAT22(param_4,local_a),iVar4->field_0xa);
  do {
    puVar3 = (astruct_225 *)local_a;
    pass1_1008_5b12(puVar3,param_4);
    uVar3 = extraout_DX | puVar3;
    if (uVar3 == 0x0) {
      uVar6 = 0xa;
      mem_op_1000_179c(0xa,0x0,0x1000);
      puStack18 = CONCAT22(uVar3,puVar3);
      if ((uVar3 | puVar3) == 0x0) {
        puStack14 = 0x0;
      }
      else {
        *puStack18 = 0x389a;
        (&puVar3->field_0x0 + 0x2) = 0x1008;
        *puStack18 = 0xea8;
        (&puVar3->field_0x0 + 0x2) = 0x1010;
        puStack14 = puStack18;
      }
      uVar5 = (puStack14 >> 0x10);
      iVar5 = (astruct_227 *)puStack14;
      iVar5->field_0x4 = param_3;
      iVar5->field_0x6 = param_2;
      puVar1 = iVar4->field_0xa;
      ppcVar2 = (code **)(*iVar4->field_0xa + 0x8);
      (**ppcVar2)(0x1000,puVar1,(puVar1 >> 0x10),iVar5,uVar5,uVar6);
      return;
    }
  } while ((puVar3->field_0x4 != param_3) || (puVar3->field_0x6 != param_2));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_0538(param_1: u32,char **param_2,char **param_3,param_4: u16,param_5: u16)
{
  let iVar1: i16;
  let uVar2: u32;
  code **ppcVar3;
  let uVar4: u16;
  let iVar5: i16;
  char *pcVar6;
  let puVar7: *mut u8
  let extraout_DX: u16;
  let puVar8: *mut u8
  let extraout_DX_00: *mut u8
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  let uVar12: u32;
  let puStack6: u32;
  
  uVar4 = 0x0;
  *param_3 = 0x0;
  *param_2 = 0x0;
  uVar10 = (param_1 >> 0x10);
  uVar9 = param_1;
  uVar12 = (uVar9 + 0xa);
  ppcVar3 = (code **)((uVar9 + 0xa) + 0x10);
  (**ppcVar3)();
  puStack6 = CONCAT22(extraout_DX,uVar4);
  puVar8 = (extraout_DX | uVar4);
  if (puVar8 == 0x0) {
    return;
  }
  iVar1 = (uVar4 + 0x4);
  uVar2 = (uVar4 + 0x6);
  if ((extraout_DX | uVar4) != 0x0) {
    ppcVar3 = (code **)*puStack6;
    (**ppcVar3)(param_4,uVar4,extraout_DX,0x1,uVar12);
    puVar8 = extraout_DX_00;
  }
  uVar12 = (uVar9 + 0xa);
  if ((uVar12 + 0x8) == 0x0) {
    pass1_1010_089e(param_5,param_1,0x0,0x6);
    pass1_1010_1f62(param_5,param_1,0x3);
  }
  iVar5 = iVar1 + 0x757;
  load_string_1010_84ac
            (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10),param_4)
  ;
  param_3 = iVar5;
  *(uchar **)(param_3 + 0x2) = puVar8;
  while (pcVar6 = pass1_1000_472c(*param_3,'%'),
        (puVar8 | pcVar6) != 0x0) {
    pass1_1010_09b4(uVar9,uVar10,CONCAT22(puVar8,pcVar6),param_3,uVar2,puVar8,
                    param_5);
  }
  if (0x1e < iVar1 - 0x1) goto LAB_1010_0850;
  uVar11 = (param_2 >> 0x10);
  switch(iVar1) {
  case 0x1:
    goto LAB_1010_0619;
  case 0x2:
    goto LAB_1010_0619;
  case 0x3:
    break;
  case 0x4:
    goto LAB_1010_0619;
  case 0x5:
    goto LAB_1010_0619;
  case 0x6:
    break;
  case 0x7:
    goto LAB_1010_0619;
  case 0x8:
    goto LAB_1010_0619;
  case 0x9:
    break;
  case 0xa:
    goto LAB_1010_0619;
  case 0xb:
    goto LAB_1010_0619;
  case 0xc:
    break;
  case 0xd:
    goto LAB_1010_0619;
  case 0xe:
    break;
  case 0xf:
    goto LAB_1010_0619;
  case 0x10:
    break;
  case 0x11:
    break;
  case 0x12:
    break;
  case 0x13:
    break;
  case 0x14:
    break;
  case 0x15:
    break;
  case 0x16:
//LAB_1010_0619:
    puVar7 = pass1_1008_5fd8(param_5,puVar8);
    goto LAB_1010_0621;
  case 0x17:
    break;
  case 0x18:
    break;
  case 0x19:
  case 0x1f:
//LAB_1010_0785:
    puVar7 = pass1_1008_5fd8(param_5,puVar8);
    goto LAB_1010_0621;
  case 0x1a:
    goto LAB_1010_0785;
  case 0x1b:
    goto LAB_1010_0785;
  case 0x1c:
    break;
  case 0x1d:
    break;
  case 0x1e:
    puVar7 = pass1_1008_5fd8(param_5,puVar8);
    *(uchar **)param_2 = puVar7;
    *(uchar **)(param_2 + 0x2) = puVar8;
    goto LAB_1010_0785;
  }
  puVar7 = pass1_1008_5fd8(param_5,puVar8);
//LAB_1010_0621:
  *(uchar **)param_2 = puVar7;
  *(uchar **)(param_2 + 0x2) = puVar8;
//LAB_1010_0850:
  while (pcVar6 = pass1_1000_472c(*param_2,'%'),
        (puVar8 | pcVar6) != 0x0) {
    pass1_1010_09b4(uVar9,uVar10,CONCAT22(puVar8,pcVar6),param_2,uVar2,puVar8,
                    param_5);
  }
  return;
}



fn pass1_1010_0886(void) -> u16

{
  return 0xa;
}



fn pass1_1010_088c(void) -> u16

{
  return 0x3;
}



fn pass1_1010_0892(void) -> u16

{
  return 0x3;
}



fn pass1_1010_0898(void) -> u16

{
  return 0x3;
}



fn pass1_1010_089e(param_1: u16,param_2: u32,param_3: u16,param_4: i16)
{
  ((param_4 + -0x1) * 0x8 + 0xe28) = param_3;
  pass1_1010_1f62(param_1,param_2,0x3);
  return;
}



fn pass1_1010_08c0(param_1: u32,param_2: u16,param_3: i16,param_4: u16)
{
  ((param_3 + -0x1) * 0x8 + 0xea8) = param_2;
  pass1_1010_1f62(param_4,param_1,0x3);
  return;
}



fn pass1_1010_08e2(param_1: u16,param_2: u16,param_3: i16) -> u32

{
  if (ctx.PTR_LOOP_1050_4fe8 != 0x0) {
    DAT_1050_0e28 = 0x0;
    ctx.PTR_LOOP_1050_0e30 = 0x0;
    ctx.PTR_LOOP_1050_0e38 = 0x0;
    ctx.PTR_LOOP_1050_0e40 = 0x0;
    ctx.PTR_LOOP_1050_0e48 = 0x0;
    DAT_1050_0e58 = 0x0;
    DAT_1050_0e60 = 0x0;
    ctx.PTR_LOOP_1050_0e70 = 0x0;
  }
  return CONCAT22(0x1050,(param_3 + -0x1) * 0x8 + 0xe22);
}



fn pass1_1010_091e(param_1: u16,param_2: u16,param_3: i16) -> u32

{
  return CONCAT22(0x1050,(param_3 + -0x1) * 0x8 + 0xe72);
}



fn pass1_1010_0932(param_1: u16,param_2: u16,param_3: i16) -> u32

{
  return CONCAT22(0x1050,(param_3 + -0x1) * 0x8 + 0xe8a);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 
pass1_1010_0946(param_1: u16,param_2: u16,param_3: i16,uchar *param_4,
               param_5: i16,param_6: u16)

{
  let puVar1: *mut u16;
  let lVar2: i32;
  let lVar3: i32;
  
  ctx.PTR_LOOP_1050_0ea8 = 0x0;
  lVar3 = 0x4000001;
  lVar2 = 0x4000002;
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3b,param_6,param_4,param_5);
  pass1_1008_dfa6(puVar1,lVar2,lVar3,param_6);
  if (puVar1 != 0x0) {
    pass1_1030_8344(_PTR_LOOP_1050_5748,
                    (_PTR_LOOP_1050_5748 >> 0x10),0x4000002);
    if (*(long *)(puVar1 + 0x200) == 0x8000002) {
      ctx.PTR_LOOP_1050_0ea8 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
    }
  }
  return CONCAT22(0x1050,(param_3 + -0x1) * 0x8 + 0xea2);
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1010_09b4(param_1: u16,param_2: u16,uchar *param_3,char **param_4,param_5: u32,
               uchar *param_6,param_7: u16)

{
  let bVar1: u8;
  let bVar2: bool;
  let bVar3: bool;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let unaff_DI: i16;
  let puVar8: *mut u16;
  char *pcStack18;
  let uStack10: u32;
  
  bVar3 = false;
  bVar2 = false;
  bVar1 = (param_3 + 0x1);
  if (bVar1 == 0x70) {
//LAB_1010_0a06:
    bVar3 = false;
    bVar2 = true;
  }
  else {
    if (bVar1 < 0x71) {
      if (bVar1 != 0x43) {
        if (bVar1 == 0x50) goto LAB_1010_0a06;
        if (bVar1 != 0x63) goto LAB_1010_09db;
      }
      bVar3 = true;
      bVar2 = false;
    }
  }
//LAB_1010_09db:
  uVar4 = 0x0;
  uStack10 = 0x0;
  if (bVar2) {
    puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_7,param_6,unaff_DI);
    uVar4 = (puVar8 >> 0x10);
    uStack10 = CONCAT22((puVar8 + 0x6e),
                                (puVar8 + 0x6c));
  }
  else {
    if (!bVar3) goto LAB_1010_0a36;
    pass1_1030_8344(_PTR_LOOP_1050_5748,
                    (_PTR_LOOP_1050_5748 >> 0x10),param_5);
    uStack10 = pass1_1038_4d28(CONCAT22(param_6,uVar4));
  }
  param_6 = (uStack10 >> 0x10);
//LAB_1010_0a36:
  if ((uStack10._2_2_ | uStack10) != 0x0) {
    uVar5 = str_op_1000_3da4(*param_4);
    uVar6 = str_op_1000_3da4(uStack10);
    uVar7 = uVar6 + 0xa + uVar5;
    mem_op_1000_179c(uVar7,param_6,0x1000);
    pcStack18 = CONCAT22(param_6,uVar7);
    *param_3 = '\0';
    unk_str_op_1000_3d3e(CONCAT22(param_6,uVar7),*param_4);
    pass1_1000_3cea(CONCAT22(param_6,uVar7),(ULONG)uStack10);
    pass1_1000_3cea(CONCAT22(param_6,uVar7),
                    param_3 & 0xffff0000 | (param_3 + 0x2));
    fn_ptr_1000_17ce((astruct_18 *)*param_4,0x1000);
    *param_4 = pcStack18;
  }
  return;
}



fn pass1_1010_0ad2(param_1: u32,param_2: u32,param_3: u16)
{
  let uVar1: u32;
  let BVar2: bool;
  let puVar3: *mut u8;
  let extraout_DX: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let local_2a: [u32;0x2];
  let local_22: [u16;0x2];
  let local_1e: [u16;0x3];
  let local_18: [u16;0x3];
  let uStack18: u32;
  let local_e: [u8;8];
  let uStack6: u16;
  let iStack4: i16;
  
  BVar2 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar2 == 0x0) {
    return;
  }
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (*(long *)(iVar4 + 0xa) == 0x0) {
    uStack6 = 0x0;
  }
  else {
    uVar1 = (iVar4 + 0xa);
    uStack6 = (uVar1 + 0x8);
  }
  local_1e[0] = uStack6;
  uVar6 = param_2;
  uVar7 = (param_2 >> 0x10);
  BVar2 = write_to_file_1008_7e1c(uVar6,uVar7,local_1e,param_3,0x2,0x1008)
  ;
  if (BVar2 != 0x0) {
    pass1_1008_5784(CONCAT22(param_3,local_e),(iVar4 + 0xa));
    do {
      puVar3 = local_e;
      pass1_1008_5b12(puVar3,param_3);
      uStack18 = CONCAT22(extraout_DX,puVar3);
      if ((extraout_DX | puVar3) == 0x0) {
        local_22[0] = (iVar4 + 0xe);
        BVar2 = write_to_file_1008_7e1c
                          (uVar6,uVar7,local_22,param_3,0x2,0x1008);
        if (BVar2 == 0x0) {
          ctx.PTR_LOOP_1050_0310 = 0x6d0;
          return;
        }
        local_22[0] = (iVar4 + 0x10);
        BVar2 = write_to_file_1008_7e1c
                          (uVar6,uVar7,local_22,param_3,0x2,0x1008);
        if (BVar2 == 0x0) {
          ctx.PTR_LOOP_1050_0310 = 0x6d0;
          return;
        }
        if ((iVar4 + 0x18) != 0x0) {
          DAT_1050_0e28 = (iVar4 + 0x12);
          ctx.PTR_LOOP_1050_0e30 = (iVar4 + 0x14);
          ctx.PTR_LOOP_1050_0ea8 = (iVar4 + 0x16);
        }
        iStack4 = 0x0;
        while( true ) {
          if (0x9 < iStack4) {
            iStack4 = 0x0;
            while( true ) {
              if (0x2 < iStack4) {
                if ((iVar4 + 0x18) != 0x0) {
                  DAT_1050_0e28 = 0x0;
                  ctx.PTR_LOOP_1050_0e30 = 0x0;
                  ctx.PTR_LOOP_1050_0ea8 = 0x0;
                }
                return;
              }
              local_1e[0] = (iStack4 * 0x8 + 0xea8);
              BVar2 = write_to_file_1008_7e1c
                                (uVar6,uVar7,local_1e,param_3,0x2,0x1008);
              if (BVar2 == 0x0) break;
              iStack4 += 0x1;
            }
            ctx.PTR_LOOP_1050_0310 = 0x6d0;
            return;
          }
          local_1e[0] = (iStack4 * 0x8 + 0xe28);
          BVar2 = write_to_file_1008_7e1c
                            (uVar6,uVar7,local_1e,param_3,0x2,0x1008);
          if (BVar2 == 0x0) break;
          iStack4 += 0x1;
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return;
      }
      local_18[0] = (puVar3 + 0x4);
      BVar2 = write_to_file_1008_7e1c
                        (uVar6,uVar7,local_18,param_3,0x2,0x1008);
      if (BVar2 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return;
      }
      local_2a[0] = (uStack18 + 0x6);
      BVar2 = write_to_file_1008_7e1c
                        (uVar6,uVar7,local_2a,param_3,0x4,0x1008);
    } while (BVar2 != 0x0);
  }
  ctx.PTR_LOOP_1050_0310 = 0x6d0;
  return;
}


fn pass1_1010_0e46(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_0350(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1010_0e6c(param_1: *mut u16,param_2: u8) -> u16

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



u32 
pass1_1010_0eac(uchar *param_1,uchar *param_2,param_3: u16,uchar *param_4,param_5: u16
               )

{
  struct_op_1018_4cda(param_1,param_2,param_3);
  CONCAT22(param_2,param_1) = 0xf0c;
  (param_1 + 0x2) = 0x1010;
  ctx.PTR_LOOP_1050_4230 = param_1;
  ctx.PTR_LOOP_1050_4232 = param_2;
  pass1_1018_4dce(CONCAT22(param_2,param_1),0xff,param_4,param_5);
  return CONCAT22(param_2,param_1);
}



astruct_11 *  pass1_1010_0ee6(astruct_11 *param_1,param_2: u8)

{
  clenaup_win_ui_1018_4d22(param_1,0x1018);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1010_0f24(astruct_79 *param_1,astruct_79 *param_2,param_3: u16,uchar *param_4,
               param_5: u16)

{
  let unaff_DI: i16;
  let puVar1: *mut u16;
  
  struct_1010_2cd2(param_1,param_2,param_3,param_5);
  (&param_1[0x9].field_0x4 + 0x2) = 0x0;
  (param_1 + 0xa) = 0x0;
  &param_1[0xa].field_0x4 = 0x0;
  (&param_1[0xa].field_0x4 + 0x2) = 0x0;
  CONCAT22(param_2,param_1) = (s_648_bmp_1050_1919 + 0x1);
  param_1->field_0x2 = 0x1010;
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_5,param_4,unaff_DI);
  (&param_1[0xa].field_0x4 + 0x2) = puVar1;
  param_1[0xa].field_0x8 = (puVar1 >> 0x10);
  return;
}



fn pass1_1010_0f76(param_1: *mut u16,param_2: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = (s_648_bmp_1050_1919 + 0x1);
  (param_1 + 0x2) = 0x1010;
  pass1_1010_17c0(param_1 & 0xffff | uVar1 << 0x10);
  pass1_1010_2db2(param_1,param_2);
  return;
}



fn pass1_1010_1146(param_1: u32,param_2: u16,param_3: i16,param_4: u16)
{
  let uVar1: u32;
  let uVar2: u16;
  
  DAT_1050_0ecc = param_2;
  uVar2 = (param_1 >> 0x10);
  uVar1 = (param_1 + 0x66);
  pass1_1000_4aea((param_1 + 0x64),uVar1,(uVar1 >> 0x10),
                  0x4,(s_dibtext_bmp_1050_1844 + 0x6),&stack0xfffe,param_3,
                  uVar2,0x1000,param_4);
  return;
}



fn pass1_1010_116c(param_1: *mut u32,param_2: i16,param_3: u16)
{
  code **ppcVar1;
  let iVar2: i16;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u32;
  let uStack4: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (*(long *)(iVar4 + 0x56) != 0x0) {
    ppcVar1 = (code **)(*param_1 + 0x34);
    (**ppcVar1)();
  }
  ppcVar1 = (code **)(*param_1 + 0x28);
  uVar6 = (**ppcVar1)();
  uVar3 = (uVar6 >> 0x10);
  if (uVar6 != 0x0) {
    uStack4 = DAT_1050_0ecc;
    iVar2 = DAT_1050_0ecc + 0x1;
    if (iVar2 == 0x0) {
      uStack4 = 0x0;
    }
    pass1_1010_1146(param_1,uStack4,param_2,param_3);
    pass1_1010_11c6(param_1,iVar2,uVar3);
    (iVar4 + 0x56) = iVar2;
    (iVar4 + 0x58) = uVar3;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1010_11c6(param_1: u32,param_2: u16,uchar *param_3)
{
  let piVar1: *mut i16;
  let puVar2: *mut u16;
  code **ppcVar3;
  let uVar4: u32;
  let uVar5: u32;
  astruct_239 *iVar6;
  let iVar7: i16;
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  let puVar11: *mut u8
  let puVar12: *mut u8
  let puVar13: *mut u8
  let puVar14: *mut u8
  let extraout_DX: *mut u8
  let extraout_DX_00: *mut u8
  let uVar15: u16;
  let extraout_DX_01: *mut u8
  let puVar16: *mut u8
  let puVar17: u32;
  astruct_234 *iVar18;
  let iVar19: i16;
  let iVar21: i16;
  astruct_238 *iVar20;
  let uVar22: u16;
  let uVar23: u16;
  let puVar24: *mut u16;
  let puStack50: u32;
  let iStack42: i16;
  let iStack40: i16;
  astruct_20 *paStack38;
  let iStack28: i16;
  let puStack26: u32;
  let puStack22: u32;
  let uStack14: u32;
  let uStack10: u32;
  
  if (DAT_1050_0ecc == -0x1) {
    return;
  }
  mem_op_1000_179c(0x1a,param_3,0x1000);
  if ((param_3 | param_2) == 0x0) {
    iVar6 = (astruct_239 *)0x0;
    puVar11 = 0x0;
  }
  else {
    puVar24 = pass1_1010_37d4((u16 *)CONCAT22(param_3,param_2));
    puVar11 = (puVar24 >> 0x10);
    iVar6 = (astruct_239 *)puVar24;
  }
  uStack10 = 0x10500ece;
  uStack14 = 0x0;
  puVar12 = puVar11;
  while( true ) {
    uVar22 = (param_1 >> 0x10);
    iVar18 = (astruct_234 *)param_1;
    piVar1 = &iVar18->field_0x68;
    if (*piVar1 == uStack14 || *piVar1 < uStack14) break;
    uVar5 = iVar18->field_0x64;
    uVar4 = (uVar5 + uStack14 * 0x4);
    puVar17 = (uVar4 + DAT_1050_0ecc * 0x8);
    puStack50 = (uVar4 & 0xffff0000 | ZEXT24(puVar17));
    iVar7 = pass1_1000_475e(uStack10,*puVar17);
    if (iVar7 != 0x0) {
      uStack10 = *puStack50;
      uStack14 = uStack14 & 0xffff | (uStack14._2_2_ + 0x1) << 0x10;
    }
    uStack14 = uStack14 & 0xffff0000 | (uStack14 + 0x1);
  }
  iVar6->field_0x10 = uStack14._2_2_;
  puVar24 = struct_1010_38f8(CONCAT22(puVar11,iVar6),uStack14._2_2_,uStack14._2_2_,puVar12
                            );
  puVar13 = (puVar24 >> 0x10);
  iVar8 = 0x0;
  mem_op_1000_179c(0x400,puVar13,0x1000);
  puVar12 = puVar13;
  iVar7 = iVar8;
  mem_op_1000_179c(0x400,puVar13,0x1000);
  paStack38 = (astruct_20 *)CONCAT22(puVar12,iVar7);
  iStack28 = 0x0;
  pass1_1000_4906((astruct_20 *)CONCAT22(puVar13,iVar8),(WNDCLASS16 *)0x0,0x400);
  pass1_1000_4906((astruct_20 *)CONCAT22(puVar12,iVar7),(WNDCLASS16 *)0x0,0x400);
  iStack42 = 0x0;
  uVar10 = 0x0;
  do {
    puVar2 = &iVar6->field_0x10;
    if (*puVar2 == uVar10 || *puVar2 < uVar10) {
      return;
    }
    uVar5 = iVar18->field_0x64;
    uVar23 = (uVar5 >> 0x10);
    iVar19 = uVar5;
    iVar21 = (iVar19 + iStack28 * 0x4);
    puVar16 = *(uchar **)(iVar19 + iStack28 * 0x4 + 0x2);
    iVar19 = iVar21 + (DAT_1050_0ecc * 0x6 + 0xeba) * 0x8;
    puStack22 = CONCAT22(puVar16,iVar19);
    uVar9 = iVar21 + (DAT_1050_0ecc * 0x6 + 0xebc) * 0x8;
    puVar14 = puVar16;
    mem_op_1000_179c(0x1a,puVar16,0x1000);
    if ((puVar14 | uVar9) == 0x0) {
      uVar5 = iVar6->field_0x8;
      (uVar5 + uVar10 * 0x4) = 0x0;
    }
    else {
      puVar24 = pass1_1010_37d4((u16 *)CONCAT22(puVar14,uVar9));
      uVar5 = iVar6->field_0x8;
      uVar23 = (uVar5 >> 0x10);
      iVar21 = uVar5;
      (iVar21 + uVar10 * 0x4) = puVar24;
      (iVar21 + uVar10 * 0x4 + 0x2) = (puVar24 >> 0x10);
    }
    iStack42 += 0x1;
    uVar5 = iVar6->field_0x8;
    uVar23 = (uVar5 >> 0x10);
    iVar21 = uVar5;
    uVar5 = (iVar21 + uVar10 * 0x4);
    ppcVar3 = (code **)((iVar21 + uVar10 * 0x4) + 0x1c)
    ;
    (**ppcVar3)(0x1000,uVar5,(uVar5 >> 0x10),iStack42,iVar19,puVar16);
    uStack14 = uVar10;
    puVar16 = extraout_DX;
    while( true ) {
      piVar1 = &iVar18->field_0x68;
      if (*piVar1 == iStack28 || *piVar1 < iStack28) break;
      iVar19 = iStack28 * 0x4;
      uVar5 = iVar18->field_0x64;
      uVar5 = (uVar5 + iVar19);
      iVar21 = pass1_1000_475e(*puStack22,
                               (uVar5 +
                                         (DAT_1050_0ecc * 0x6 + 0xeba) * 0x8));
      if (iVar21 != 0x0) break;
      uVar5 = iVar18->field_0x64;
      uVar23 = (uVar5 >> 0x10);
      iVar21 = uVar5;
      puVar16 = *(uchar **)(iVar21 + iVar19 + 0x2);
      uVar10 = (iVar21 + iVar19) + (DAT_1050_0ecc * 0x6 + 0xebc) * 0x8;
      puStack26 = CONCAT22(puVar16,uVar10);
      mem_op_1000_179c(0x1a,puVar16,0x1000);
      if ((puVar16 | uVar10) == 0x0) {
        uVar23 = 0x0;
        uVar15 = 0x0;
      }
      else {
        puVar24 = pass1_1010_37d4((u16 *)CONCAT22(puVar16,uVar10));
        uVar15 = (puVar24 >> 0x10);
        uVar23 = SUB42(puVar24,0x0);
      }
      (uStack14._2_2_ * 0x4 + iVar8) = uVar23;
      (uStack14._2_2_ * 0x4 + iVar8 + 0x2) = uVar15;
      uVar5 = iVar18->field_0x64;
      uVar23 = (uVar5 >> 0x10);
      iVar21 = uVar5;
      iStack42 += 0x1;
      uVar5 = (uStack14._2_2_ * 0x4 + iVar8);
      ppcVar3 = (code **)(
                                (uStack14._2_2_ * 0x4 + iVar8) + 0x1c);
      (**ppcVar3)(0x1000,uVar5,(uVar5 >> 0x10),iStack42,
                  (iVar21 + iStack28 * 0x4) +
                  (DAT_1050_0ecc * 0x6 + 0xebc) * 0x8,
                  (iVar21 + iStack28 * 0x4 + 0x2));
      iStack40 = 0x0;
      puVar16 = extraout_DX_00;
      while( true ) {
        piVar1 = &iVar18->field_0x68;
        if (*piVar1 == iStack28 || *piVar1 < iStack28) break;
        uVar5 = iVar18->field_0x64;
        uVar5 = (uVar5 + iStack28 * 0x4);
        iVar21 = pass1_1000_475e(*puStack26,
                                 (uVar5 +
                                           (DAT_1050_0ecc * 0x6 + 0xebc) * 0x8));
        if (iVar21 != 0x0) break;
        uVar5 = iVar18->field_0x64;
        uVar5 = (uVar5 + iStack28 * 0x4);
        uVar10 = pass1_1000_475e(*puStack22,
                                 (uVar5 +
                                           (DAT_1050_0ecc * 0x6 + 0xeba) * 0x8));
        if (uVar10 != 0x0) break;
        mem_op_1000_179c(0x1a,puVar16,0x1000);
        if ((puVar16 | uVar10) == 0x0) {
          uVar23 = 0x0;
          uVar15 = 0x0;
        }
        else {
          puVar24 = pass1_1010_37d4((u16 *)CONCAT22(puVar16,uVar10));
          uVar15 = (puVar24 >> 0x10);
          uVar23 = SUB42(puVar24,0x0);
        }
        (iStack40 * 0x4 + iVar7) = uVar23;
        (iStack40 * 0x4 + iVar7 + 0x2) = uVar15;
        uVar5 = iVar18->field_0x64;
        uVar23 = (uVar5 >> 0x10);
        iVar20 = (astruct_238 *)uVar5;
        iStack42 += 0x1;
        uVar5 = (iStack40 * 0x4 + iVar7);
        ppcVar3 = (code **)((iStack40 * 0x4 + iVar7) +
                           0x1c);
        (**ppcVar3)(0x1000,uVar5,(uVar5 >> 0x10),iStack42,
                    (iVar20 + iStack28 * 0x4) +
                    (DAT_1050_0ecc * 0x6 + 0xebe) * 0x8,
                    (iVar20 + iStack28 * 0x4 + 0x2));
        iStack28 += 0x1;
        iStack40 += 0x1;
        puVar16 = extraout_DX_01;
      }
      uVar5 = (uStack14._2_2_ * 0x4 + iVar8);
      (uVar5 + 0x10) = iStack40;
      uVar10 = iStack40 << 0x2;
      iVar21 = iVar7;
      puVar14 = puVar12;
      puVar24 = struct_1010_38f8((uStack14._2_2_ * 0x4 + iVar8),iStack40,uVar10,
                                 puVar16);
      puVar16 = (puVar24 >> 0x10);
      pass1_1000_48a8(puVar24,CONCAT22(puVar14,iVar21),uVar10);
      pass1_1000_4906(paStack38,(WNDCLASS16 *)0x0,0x400);
      uStack14 = uStack14 & 0xffff | (uStack14._2_2_ + 0x1) << 0x10;
    }
    uVar5 = iVar6->field_0x8;
    uVar5 = (uVar5 + uStack14 * 0x4);
    (uVar5 + 0x10) = uStack14._2_2_;
    uVar10 = uStack14._2_2_ << 0x2;
    uVar5 = iVar6->field_0x8;
    iVar21 = iVar8;
    puVar14 = puVar13;
    puVar24 = struct_1010_38f8((uVar5 + uStack14 * 0x4),uStack14._2_2_
                               ,uVar10,puVar16);
    pass1_1000_48a8(puVar24,CONCAT22(puVar14,iVar21),uVar10);
    pass1_1000_4906((astruct_20 *)CONCAT22(puVar13,iVar8),(WNDCLASS16 *)0x0,0x400);
    uVar10 = uStack14 + 0x1;
  } while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1010_1656(param_1: i16,param_2: u16,param_3: u16,param_4: u32,param_5: u16,
               uchar *param_6,param_7: i16,param_8: u16)

{
  let uVar1: u32;
  let uVar2: u16;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let puVar6: *mut u16;
  let uVar7: u32;
  
  unk_destroy_win_op_1010_305a
            ((astruct_27 *)CONCAT22(param_2,param_1),param_3,param_4,param_5);
  if ((param_1 + 0x16) == 0x3) {
    puVar6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_8,param_6,param_7);
    uVar1 = (param_1 + 0x32);
    uVar1 = (uVar1 + 0x42);
    uVar5 = (uVar1 >> 0x10);
    iVar4 = uVar1;
    uVar1 = (iVar4 + 0x16);
    uVar7 = struct_op_1030_73a8((uVar1 + 0x4));
    uVar2 = pass1_1010_7818(puVar6,uVar7);
    uVar1 = (iVar4 + 0x16);
    uVar3 = uVar2;
    ui_op_1010_79aa(puVar6,0x0,*(long *)(uVar1 + 0x4),param_8);
    if (uVar3 == 0x0) {
      uVar1 = (iVar4 + 0x16);
      unk_win_op_1010_7300(puVar6,0x0,uVar2,(uVar1 + 0x4));
    }
  }
  return;
}



void 
pass1_1010_16ee(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
               uchar *param_6)

{
  let unaff_SS: u16;
  
  mem_op_1000_179c(0x4a,param_6,0x1000);
  if ((param_6 | param_5) != 0x0) {
    pass1_1040_c54a((u16 *)CONCAT22(param_6,param_5),0x0,
                    CONCAT22(param_4,param_3),&ctx.PTR_LOOP_1050_1040,
                    unaff_SS);
    return;
  }
  return;
}


void 
pass1_1010_1788(param_1: u16,param_2: u16,param_3: u32,uchar *param_4,param_5: i16,
               param_6: u16)

{
  char *pcVar1;
  let uVar2: u16;
  let puVar3: *mut u16;
  let uVar4: u32;
  let puVar5: *mut u8
  let in_stack_0000fff6: i16;
  
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_4,param_5);
  puVar5 = 0x1778;
  uVar4 = pass1_1028_b58e(param_3);
  uVar2 = (uVar4 >> 0x10);
  pcVar1 = pass1_1010_b038((uchar *)puVar3,uVar4,uVar2,puVar5,in_stack_0000fff6);
  str_op_1008_60e8(CONCAT22(uVar2,pcVar1),uVar2);
  return;
}



fn pass1_1010_17c0(Uparam_1: i32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_475 *iVar5;
  let uVar4: u16;
  let unaff_CS: u16;
  
  unk_destroy_win_op_1010_2fa0(param_1,unaff_CS);
  uVar4 = (param_1 >> 0x10);
  iVar5 = (astruct_475 *)param_1;
  puVar1 = iVar5->field_0x56;
  uVar2 = iVar5->field_0x58;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  &iVar5->field_0x56 = 0x0;
  fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x60,0x1000);
  pass1_1000_4906(iVar5->field_0x64,(WNDCLASS16 *)0x0,iVar5->field_0x68 << 0x2);
  fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x64,0x1000);
  iVar5->field_0x60 = 0x0;
  iVar5->field_0x64 = (astruct_20 *)0x0;
  return;
}



fn pass1_1010_184a(param_1: *mut u32,param_2: *mut u32)
{
  let iVar1: i16;
  let iVar2: i16;
  
  iVar2 = DAT_1050_0ecc;
  iVar1 = (DAT_1050_0ecc * 0x6 + 0xeba) * 0x8;
  iVar1 = pass1_1000_475e((iVar1 + *param_1),
                          (iVar1 + *param_2));
  if (iVar1 == 0x0) {
    iVar1 = (iVar2 * 0x6 + 0xebc) * 0x8;
    iVar1 = pass1_1000_475e((iVar1 + *param_1),
                            (iVar1 + *param_2));
    if (iVar1 == 0x0) {
      iVar2 = (iVar2 * 0x6 + 0xebe) * 0x8;
      pass1_1000_475e((iVar2 + *param_1),(iVar2 + *param_2))
      ;
    }
  }
  return;
}



fn pass1_1010_18f4(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_0f76(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_195e(astruct_79 *param_1,astruct_79 *param_2,param_3: u16) -> u32

{
  let in_DX: *mut u8
  let unaff_DI: i16;
  let unaff_SS: u16;
  let puVar1: *mut u16;
  
  pass1_1010_0f24(param_1,param_2,param_3,in_DX,unaff_SS);
  (param_1 + 0xb) = 0x0;
  CONCAT22(param_2,param_1) = 0x1b2a;
  param_1->field_0x2 = 0x1010;
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,unaff_SS,in_DX,unaff_DI);
  ((astruct_79 *)(param_1 + 0xb))->field_0x0 = puVar1;
  param_1[0xb].field_0x2 = (puVar1 >> 0x10);
  return CONCAT22(param_2,param_1);
}



fn pass1_1010_19a4(param_1: *mut u32,param_2: u16,param_3: u16)
{
  code **ppcVar1;
  let puVar2: *mut u8;
  let extraout_DX: u16;
  let local_14: [u8;12];
  
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_3,local_14),0x1,0x0,0x700);
  while( true ) {
    puVar2 = local_14;
    pass1_1028_e4ec(CONCAT22(param_3,puVar2));
    if ((param_2 | puVar2) == 0x0) break;
    ppcVar1 = (code **)(*param_1 + 0x40);
    (**ppcVar1)(&USHORT_1050_1028,param_1);
    param_2 = extraout_DX;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_1a06(param_1: u32,param_2: u32,param_3: i16,param_4: u16)
{
  char *pcVar1;
  let iVar2: i16;
  let uVar3: u16;
  let puVar4: *mut u8
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u32;
  let puVar8: *mut u16;
  char *pcVar9;
  let in_stack_0000ffee: i16;
  
  uVar7 = pass1_1028_b58e(param_2);
  puVar4 = (uVar7 >> 0x10);
  uVar6 = (param_1 >> 0x10);
  pcVar1 = pass1_1010_b038(*(uchar **)(param_1 + 0x6e),uVar7,puVar4,
                           0x1770,in_stack_0000ffee);
  iVar2 = pass1_1000_3e2c(CONCAT22(puVar4,pcVar1));
  puVar8 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x32,param_4,puVar4,param_3);
  uVar5 = (puVar8 >> 0x10);
  uVar3 = pass1_1010_7818(puVar8,param_2);
  uVar7 = (param_1 + 0x6e);
  pcVar9 = string_op_1010_ada6(0x1000,uVar5,uVar7,(uVar7 >> 0x10),
                               iVar2,uVar3);
  str_op_1008_60e8(pcVar9,(pcVar9 >> 0x10));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uchar  pass1_1010_1a66(param_1: u32,param_2: u32)

{
  let uVar1: u32;
  ulet uVar2: u8;
  let uVar3: u16;
  let Bvar4: bool;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u32;
  
  uVar5 = (param_2 >> 0x10);
  if (((param_2 + 0x1c) != 0x2) ||
     (((param_2 + 0x1e) & 0xff) != 0x0)) {
    uVar7 = pass1_1028_b58e(param_2 & 0xffff | uVar5 << 0x10);
    uVar6 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x6e);
    pass1_1010_c2d8(uVar1,(uVar1 >> 0x10),uVar7);
    if ((uVar7 != 0x2) || ((uVar7 & 0xff0000) != 0x0)) {
      uVar1 = (param_1 + 0x6e);
      uVar3 = pass1_1010_b028(uVar1,(uVar1 >> 0x10),param_2);
      BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar3,0x5);
      if ((BVar4 == 0x0) &&
         (BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar3,0x6), BVar4 == 0x0)) {
        uVar2 = '\0';
      }
      else {
        uVar2 = '\x01';
      }
      return uVar2;
    }
  }
  return '\0';
}



fn pass1_1010_1b04(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_0f76(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 
pass1_1010_1b6e(astruct_79 *param_1,astruct_79 *param_2,param_3: u16,param_4: u16,
               uchar *param_5)

{
  let unaff_DI: i16;
  let puVar1: *mut u16;
  
  pass1_1010_0f24(param_1,param_2,param_3,param_5,param_4);
  (param_1 + 0xb) = 0x0;
  CONCAT22(param_2,param_1) = 0x1d04;
  param_1->field_0x2 = 0x1010;
  puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_4,param_5,unaff_DI);
  ((astruct_79 *)(param_1 + 0xb))->field_0x0 = puVar1;
  param_1[0xb].field_0x2 = (puVar1 >> 0x10);
  return CONCAT22(param_2,param_1);
}



fn pass1_1010_1bb4(param_1: *mut u32,param_2: u16,param_3: u16)
{
  code **ppcVar1;
  let puVar2: *mut u8;
  let extraout_DX: u16;
  let local_14: [u8;12];
  
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_3,local_14),0x1,0x0,0x700);
  while( true ) {
    puVar2 = local_14;
    pass1_1028_e4ec(CONCAT22(param_3,puVar2));
    if ((param_2 | puVar2) == 0x0) break;
    ppcVar1 = (code **)(*param_1 + 0x40);
    (**ppcVar1)(&USHORT_1050_1028,param_1);
    param_2 = extraout_DX;
  }
  return;
}



fn pass1_1010_1c16(param_1: u32,param_2: u32,param_3: i16)
{
  char *pcVar1;
  let uVar2: u16;
  let uVar3: u32;
  
  uVar3 = pass1_1028_b58e(param_2);
  uVar2 = (uVar3 >> 0x10);
  pcVar1 = pass1_1010_b038(*(uchar **)(param_1 + 0x6e),uVar3,uVar2,
                           0x178a,param_3);
  str_op_1008_60e8(CONCAT22(uVar2,pcVar1),uVar2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uchar  pass1_1010_1c40(param_1: u32,param_2: u32)

{
  let uVar1: u32;
  ulet uVar2: u8;
  let uVar3: u16;
  let Bvar4: bool;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u32;
  
  uVar5 = (param_2 >> 0x10);
  if (((param_2 + 0x1c) != 0x2) ||
     (((param_2 + 0x1e) & 0xff) != 0x0)) {
    uVar7 = pass1_1028_b58e(param_2 & 0xffff | uVar5 << 0x10);
    uVar6 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x6e);
    pass1_1010_c2d8(uVar1,(uVar1 >> 0x10),uVar7);
    if ((uVar7 != 0x2) || ((uVar7 & 0xff0000) != 0x0)) {
      uVar1 = (param_1 + 0x6e);
      uVar3 = pass1_1010_b028(uVar1,(uVar1 >> 0x10),param_2);
      BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar3,0x11);
      if ((BVar4 == 0x0) &&
         (BVar4 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar3,0x12), BVar4 == 0x0)) {
        uVar2 = '\0';
      }
      else {
        uVar2 = '\x01';
      }
      return uVar2;
    }
  }
  return '\0';
}



fn pass1_1010_1cde(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_0f76(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1010_1d80(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_455 *iVar4;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_455 *)param_1;
  *param_1 = 0x2014;
  iVar4->field_0x2 = 0x1010;
  pass1_1010_1f62(param_2,param_1 & 0xffff | uVar4 << 0x10,0x1);
  puVar1 = iVar4->field_0x4;
  uVar2 = iVar4->field_0x6;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *param_1 = 0x389a;
  iVar4->field_0x2 = 0x1008;
  return;
}



fn pass1_1010_1dce(void) -> u16

{
  return 0x0;
}



fn pass1_1010_1dd4(void) -> u16

{
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_1dda(param_1: u32)
{
  pass1_1010_209e(_PTR_LOOP_1050_0ed0,(param_1 + 0x8));
  return;
}



fn pass1_1010_1df2(param_1: u32,param_2: u16,param_3: u32,param_4: u16,param_5: u16)
{
  code **ppcVar1;
  astruct_241 *in_AX;
  let puVar2: *mut u8
  let extraout_DX: *mut u8
  astruct_242 *iVar3;
  let uVar3: u16;
  let puStack10: *mut u16;
  let puStack6: *mut u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_242 *)param_1;
  puVar2 = param_5;
  if (iVar3->field_0x4 == 0x0) {
    mem_op_1000_179c(0xc,param_5,0x1000);
    puVar2 = (param_5 | in_AX);
    if (puVar2 == 0x0) {
      iVar3->field_0x4 = 0x0;
    }
    else {
      set_struct_1008_574a((astruct_21 *)CONCAT22(param_5,in_AX));
      *(astruct_241 **)&iVar3->field_0x4 = in_AX;
      *(uchar **)(&iVar3->field_0x4 + 0x2) = extraout_DX;
      puVar2 = extraout_DX;
    }
  }
  mem_op_1000_179c(0xa,puVar2,0x1000);
  puStack10 = CONCAT22(puVar2,in_AX);
  if ((puVar2 | in_AX) == 0x0) {
    puStack6 = 0x0;
  }
  else {
    *puStack10 = 0x389a;
    in_AX->field_0x2 = 0x1008;
    in_AX->field_0x4 = param_3;
    in_AX->field_0x8 = param_2;
    *puStack10 = 0x2010;
    in_AX->field_0x2 = 0x1010;
    puStack6 = puStack10;
  }
  ppcVar1 = (code **)(*iVar3->field_0x4 + 0x4);
  (**ppcVar1)(0x1000,iVar3->field_0x4,puStack6,(puStack6 >> 0x10));
  return;
}



fn pass1_1010_1ea6(param_1: u32,param_2: i32,param_3: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let puVar4: u32;
  let puVar5: *mut u8;
  let extraout_DX: u16;
  astruct_498 *iVar6;
  let uVar6: u16;
  let local_c: [u8;4];
  let uStack8: u32;
  let uStack4: u16;
  
  uVar6 = (param_1 >> 0x10);
  iVar6 = (astruct_498 *)param_1;
  if (iVar6->field_0x4 == 0x0) {
    return;
  }
  uStack4 = 0x0;
  pass1_1008_5784(CONCAT22(param_3,local_c),iVar6->field_0x4);
  while( true ) {
    puVar5 = local_c;
    pass1_1008_5b12(puVar5,param_3);
    if ((extraout_DX | puVar5) == 0x0) break;
    if (*(long *)(puVar5 + 0x4) == param_2) {
      uStack4 = 0x1;
      ppcVar3 = (code **)(*iVar6->field_0x4 + 0xc);
      (**ppcVar3)(0x1008);
      uStack8 = 0x0;
    }
  }
  puVar4 = iVar6->field_0x4;
  if ((puVar4 + 0x8) == 0x0) {
                    // WARNING: Load size is inaccurate
    puVar1 = iVar6->field_0x4;
    uVar2 = (&iVar6->field_0x4 + 0x2);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)(0x1008,puVar1,uVar2,0x1,puVar1,uVar2,puVar1,uVar2);
    }
    iVar6->field_0x4 = 0x0;
  }
  return;
}



fn pass1_1010_1f62(param_1: u16,param_2: u32,param_3: i16)
{
  let uVar1: u32;
  code **ppcVar2;
  let iVar3: i16;
  let uVar4: u16;
  let lVar5: i32;
  let local_a: [u8;8];
  
  pass1_1008_5784(CONCAT22(param_1,local_a),(param_2 + 0x4));
  while( true ) {
    lVar5 = pass1_1008_5b12(local_a,param_1);
    uVar4 = (lVar5 >> 0x10);
    iVar3 = lVar5;
    if (lVar5 == 0x0) break;
    if ((((iVar3 + 0x8) == 0x0) || (param_3 == 0x0)) ||
       ((iVar3 + 0x8) == param_3)) {
      uVar1 = (iVar3 + 0x4);
      ppcVar2 = (code **)((iVar3 + 0x4) + 0x4);
      (**ppcVar2)(0x8,uVar1,(uVar1 >> 0x10),param_3);
    }
  }
  return;
}



fn pass1_1010_1fbe(param_1: *mut u16,param_2: u8) -> u16

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1010_1fea(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_2024(param_1: u32) -> u32

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x124) = 0x0;
  _PTR_LOOP_1050_0ed0 = param_1;
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff | uVar1 << 0x10),
                  (WNDCLASS16 *)0x0,0x124);
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_2050(param_1: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let uVar4: u16;
  let iStack4: i16;
  
  pass1_1010_2816(param_1);
  iStack4 = 0x0;
  do {
    uVar4 = (param_1 >> 0x10);
    puVar1 = (iStack4 * 0x4 + param_1);
    uVar2 = (iStack4 * 0x4 + param_1 + 0x2);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    iStack4 += 0x1;
  } while (iStack4 < 0x49);
  _PTR_LOOP_1050_0ed0 = 0x0;
  return;
}



fn pass1_1010_209e(param_1: u32,param_2: u16)
{
  pass1_1010_2816(param_1);
  (param_1 + 0x124) = param_2;
  return;
}


fn pass1_1010_2816(param_1: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let iVar5: i16;
  let uVar6: u16;
  
  uVar6 = (param_1 >> 0x10);
  iVar4 = param_1;
  if ((iVar4 + 0x124) != 0x0) {
    iVar5 = (iVar4 + 0x124) * 0x4;
    puVar1 = (iVar5 + iVar4);
    uVar2 = (iVar5 + iVar4 + 0x2);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    ((iVar4 + 0x124) * 0x4 + iVar4) = 0x0;
    (iVar4 + 0x124) = 0x0;
  }
  return;
}



fn pass1_1010_286c(void) -> u16

{
  let puVar1: *mut u16;
  
  pass1_1008_3e54((u16 *)&ctx.PTR_LOOP_1048_0000,0x0,0x5,0x12c);
  pass1_1008_3e54((u16 *)0x105065a6,0x0,0x9b,0x20);
  pass1_1008_3e54((u16 *)0x105065ac,0x0,0xf5,0x3f);
  pass1_1008_3e54((u16 *)0x105065b2,0x0,0x114,0x4a);
  pass1_1008_3e54((u16 *)0x105065b8,0x0,0x135,0x45);
  pass1_1008_3e54((u16 *)0x105065be,0x0,0xf5,0x7b);
  puVar1 = pass1_1008_3e54((u16 *)0x105065c4,0x0,0x117,0x91);
  return puVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1010_28e6(astruct_631 *param_1,uchar *param_2,param_3: u16,uchar *param_4,
               param_5: u16)

{
  let uVar1: u32;
  let uVar2: u16;
  let iVar3: i16;
  let uVar4: u16;
  astruct_43 *paVar5;
  let iStack6: i16;
  
  struct_op_1018_4cda(param_1,param_2,param_3);
  &param_1->field_0x1c = 0x0;
  param_1->field_0x20 = 0x0;
  param_1->field_0x22 = 0x0;
  param_1->field_0x24 = 0x0;
  param_1->field_0x26 = 0x0;
  CONCAT22(param_2,param_1) = (s_add16_wav_1050_2bdc + 0x8);
  param_1->field_0x2 = 0x1010;
  ctx.PTR_LOOP_1050_4230 = param_1;
  ctx.PTR_LOOP_1050_4232 = param_2;
  pass1_1018_4dce(CONCAT13((param_2 >> 0x8),
                                    CONCAT12(param_2,param_1)),0x56,param_4,param_5)
  ;
  paVar5 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x4,param_5);
  ctx.PTR_LOOP_1050_5f2e = (paVar5 >> 0x10);
  param_1->field_0x1c = paVar5;
  param_1->field_0x1e = ctx.PTR_LOOP_1050_5f2e;
  if (_PTR_LOOP_1050_5f2c == 0x0) {
    ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e,0x1000);
  }
  else {
  }
  uVar2 = fn_ptr_op_1000_1708(0x40,0x0,0x1,PTR_LOOP_1050_5f2c,
                              ctx.PTR_LOOP_1050_5f2e,0x1000);
  param_1->field_0x28 = uVar2;
  param_1->field_0x2a = ctx.PTR_LOOP_1050_5f2e;
  for (iStack6 = 0x0; iStack6 < 0x10; iStack6 += 0x1) {
    paVar5 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,iStack6 + 0x56,param_5);
    uVar1 = &param_1->field_0x28;
    uVar4 = (uVar1 >> 0x10);
    iVar3 = uVar1;
    (iVar3 + iStack6 * 0x4) = paVar5;
    (iVar3 + iStack6 * 0x4 + 0x2) = (paVar5 >> 0x10);
  }
  return;
}



fn pass1_1010_29c6(astruct_11 *param_1)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_476 *iVar5;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar5 = (astruct_476 *)param_1;
  param_1 = (s_add16_wav_1050_2bdc + 0x8);
  iVar5->field_0x2 = 0x1010;
  if (*(long *)&iVar5->field_0x1c != 0x0) {
    puVar1 = &iVar5->field_0x1c;
    uVar2 = iVar5->field_0x1e;
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    &iVar5->field_0x1c = 0x0;
    fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x28,0x1000);
    iVar5->field_0x28 = 0x0;
  }
  clenaup_win_ui_1018_4d22(param_1,0x1018);
  return;
}


fn pass1_1010_2b50(param_1: u16,param_2: u16,param_3: *mut u16)
{
  pass1_1008_3f62(param_3,&ctx.PTR_LOOP_1048_0000);
  return;
}



fn pass1_1010_2b66(param_1: u32) -> u32

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x1e),
                  (param_1 + 0x1c));
}



fn pass1_1010_2b78(param_1: u16,param_2: u16,param_3: i16,param_4: u32)
{
  let puVar1: u32;
  let puVar2: u32;
  let puVar3: u32;
  let iVar4: i16;
  let puVar5: u32;
  
  puVar3 = (param_3 * 0x7c + 0xed4);
  puVar5 = param_4;
  for (iVar4 = 0x1f; iVar4 != 0x0; iVar4 += -0x1) {
    puVar2 = puVar5;
    puVar5 = puVar5 + 0x1;
    puVar1 = puVar3;
    puVar3 = puVar3 + 0x1;
    *puVar2 = *puVar1;
  }
  return;
}



fn pass1_1010_2b98(param_1: u32,param_2: i16) -> u32

{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar1 = (param_1 + 0x28);
  uVar3 = (uVar1 >> 0x10);
  iVar2 = uVar1;
  return CONCAT22((param_2 * 0x4 + iVar2 + -0x156),
                  (param_2 * 0x4 + iVar2 + -0x158));
}



fn pass1_1010_2bb9(void)
{
  pass1_1010_286c();
  return;
}



astruct_11 *  pass1_1010_2bbe(astruct_11 *param_1,param_2: u8)

{
  pass1_1010_29c6(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1010_2bfc(astruct_644 *param_1,param_2: u16,param_3: u16) -> u16

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x0;
  param_1->field_0xc = 0x0;
  param_1->field_0xe = 0x0;
  param_1->field_0x10 = 0x0;
  CONCAT22(param_2,param_1) = 0x2cc2;
  param_1->field_0x2 = 0x1010;
  return CONCAT22(param_2,param_1);
}


fn pass1_1010_2c9c(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1010_2db2(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_473 *iVar5;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar5 = (astruct_473 *)param_1;
  *param_1 = 0x36da;
  iVar5->field_0x2 = 0x1010;
  puVar1 = iVar5->field_0x56;
  uVar2 = iVar5->field_0x58;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x5c,0x1000);
  pass1_1010_1d80(param_1,param_2);
  return;
}



fn pass1_1010_2e02(param_1: u32,param_2: i16) -> u32

{
  let uVar1: u32;
  astruct_163 *iVar2;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x5c) != 0x0) {
    uVar1 = (param_1 + 0x5c);
    uVar2 = (uVar1 >> 0x10);
    iVar2 = (astruct_163 *)uVar1;
    return CONCAT22((iVar2 + param_2 * 0x4 + 0x2),
                    (iVar2 + param_2 * 0x4));
  }
  return 0x0;
}



fn pass1_1010_2e30(param_1: u32,param_2: u16,param_3: u16,param_4: i16)
{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x5c) != 0x0) {
    uVar1 = (param_1 + 0x5c);
    uVar3 = (uVar1 >> 0x10);
    iVar2 = uVar1;
    (iVar2 + param_4 * 0x4) = param_2;
    (iVar2 + param_4 * 0x4 + 0x2) = param_3;
  }
  return;
}



fn pass1_1010_2e5c(param_1: u16,param_2: u16,param_3: u32)
{
  let uStack12: u32;
  
  uStack12 = param_3;
  if (param_3 == 0x0) {
    return;
  }
  for (; (uStack12 & 0xf) != 0x0; uStack12 >>= 0x4) {
  }
  return;
}



fn pass1_1010_2ee2(param_1: *mut u32,param_2: u16,param_3: u16)
{
  let uVar1: u32;
  code **ppcVar2;
  let iVar3: i16;
  let extraout_DX: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u32;
  let puStack6: u32;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  if (*(long *)(iVar4 + 0x52) != 0x0) {
    return;
  }
  iVar3 = 0x0;
  (iVar4 + 0x28) = 0x0;
  uVar6 = *param_1;
  ppcVar2 = (code **)(uVar6 + 0x20);
  (**ppcVar2)(param_3,param_1,(iVar4 + 0x12));
  if (iVar3 == 0x0) {
    puStack6 = *(u32 **)(iVar4 + 0x56);
  }
  else {
    uVar1 = (iVar4 + 0x12);
    ppcVar2 = (code **)(uVar6 + 0x14);
    (**ppcVar2)(param_3,param_1,uVar1,(uVar1 >> 0x10));
    puStack6 = CONCAT22(extraout_DX,iVar3);
    uVar6 = pass1_1010_2e02(param_1,(iVar3 + 0x12));
    pass1_1010_35a4(param_1,uVar6,(uVar6 >> 0x10));
  }
  pass1_1010_32f4(param_1,puStack6,param_2,param_3);
  pass1_1010_1f62(param_2,param_1,0x8);
  if (*(long *)(iVar4 + 0x52) != 0x0) {
    return;
  }
  return;
}



fn pass1_1010_32c0(param_1: u32,param_2: u32)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x28) = 0x0;
  (param_1 + 0x12) = param_2;
  return;
}



fn pass1_1010_32da(param_1: *mut u32,param_2: u32,param_3: u16,param_4: u16)
{
  pass1_1010_32f4(param_1,*(u32 **)(param_2 + 0x42),param_4,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_32f4(param_1: *mut u32,param_2: *mut u32,param_3: u16,param_4: u16)
{
  let puVar1: *mut u16;
  let puVar2: u32;
  let uVar3: u32;
  let uVar4: u32;
  code **ppcVar5;
  astruct_65 *paVar6;
  let uVar7: u32;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let iVar11: i16;
  let extraout_DX: *mut u8;
  let extraout_DX_00: u16;
  astruct_166 *iVar10;
  let iVar12: i16;
  let iVar13: i16;
  let uVar14: u16;
  let uVar15: u16;
  let uVar16: u16;
  let puStack48: *mut u16;
  let uStack16: u16;
  let iStack12: i16;
  
  uVar14 = (param_1 >> 0x10);
  iVar10 = (astruct_166 *)param_1;
  if (iVar10->field_0x52 != (astruct_65 *)0x0) {
    param_4 = 0x1000;
    fn_ptr_1000_17ce((astruct_18 *)iVar10->field_0x52,0x1000);
    iVar10->field_0x52 = (astruct_65 *)0x0;
    iVar10->field_0x18 = 0x0;
  }
  uVar8 = param_2._2_2_ | param_2;
  if ((param_2 != 0x0) &&
     (ppcVar5 = (code **)(*param_1 + 0x24), (**ppcVar5)(param_4,param_1,param_2),
     uVar8 != 0x0)) {
    ppcVar5 = (code **)(*param_2 + 0x4);
    (**ppcVar5)(param_4,param_2);
    iVar10->field_0x18 = uVar8;
    if (uVar8 != 0x0) {
      iVar10->field_0x24 = 0x0;
      iVar10->field_0x26 = 0x0;
      puVar1 = &iVar10->field_0x18;
      *puVar1 = *puVar1 - iVar10->field_0x28;
      if (0xa < iVar10->field_0x18) {
        iVar10->field_0x26 = 0x1;
        iVar10->field_0x18 = 0xa;
      }
      if (0x1 < iVar10->field_0x28) {
        iVar10->field_0x24 = 0x1;
      }
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        ctx.PTR_LOOP_1050_5f2e = extraout_DX;
        ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(extraout_DX,0x1000);
      }
      else {
      }
      uVar16 = 0x1000;
      uVar9 = fn_ptr_op_1000_1708(0x28,0x0,0x1,PTR_LOOP_1050_5f2c,
                                  ctx.PTR_LOOP_1050_5f2e,0x1000);
      &iVar10->field_0x52 = uVar9;
      (&iVar10->field_0x52 + 0x2) = ctx.PTR_LOOP_1050_5f2e;
      if (((&iVar10->field_0x52 + 0x2) | &iVar10->field_0x52) !=
          0x0) {
        uVar3 = (param_2 + 0x8);
        iVar11 = iVar10->field_0x10;
        iStack12 = 0x0;
        for (uStack16 = 0x0; puVar1 = &iVar10->field_0x18,
            *puVar1 != uStack16 && uStack16 <= *puVar1; uStack16 += 0x1) {
          paVar6 = iVar10->field_0x52;
          uVar8 = paVar6 + uStack16 * 0x4;
          uVar7 = paVar6 & 0xffff0000;
          puStack48 = (uVar7 | uVar8);
          uVar4 = ((iVar10->field_0x28 + uStack16) * 0x4 + uVar3);
          ppcVar5 = (code **)(*param_1 + 0x1c);
          uVar10 = uStack16;
          (**ppcVar5)(uVar16,param_1,uVar4,(uVar4 >> 0x10),
                      iVar10->field_0x22);
          *puStack48 = uVar10;
          (uVar8 + 0x2) = extraout_DX_00;
          paVar6 = iVar10->field_0x52;
          uVar4 = (paVar6 + uStack16 * 0x4);
          iStack12 += (uVar4 + 0x24) + 0x8;
          if (iVar11 + -0xa < iStack12) {
            uVar16 = 0x1008;
            debug_print_1008_6048(s_overflow_on_node__d_1050_11ca,0x1008,param_3);
            iVar10->field_0x18 = uStack16 - 0x1;
            iVar10->field_0x26 = 0x1;
            paVar6 = iVar10->field_0x52;
            uVar15 = (paVar6 >> 0x10);
            iVar13 = paVar6;
            puVar2 = (iVar13 + uStack16 * 0x4);
            uVar8 = (iVar13 + uStack16 * 0x4 + 0x2);
            if ((uVar8 | puVar2) != 0x0) {
              ppcVar5 = (code **)*puVar2;
              (**ppcVar5)(0x1008,puVar2,uVar8,0x1);
            }
            paVar6 = iVar10->field_0x52;
            iVar13 = uStack16 * 0x4;
            (paVar6 + iVar13) = 0x0;
            if (0x0 < uStack16) {
              paVar6 = iVar10->field_0x52;
              uVar15 = (paVar6 >> 0x10);
              iVar12 = paVar6;
              puVar2 = (iVar12 + iVar13 + -0x4);
              uVar8 = (iVar12 + iVar13 + -0x2);
              if ((uVar8 | puVar2) != 0x0) {
                ppcVar5 = (code **)*puVar2;
                (**ppcVar5)(0x1008,puVar2,uVar8,0x1);
              }
              paVar6 = iVar10->field_0x52;
              (uStack16 * 0x4 + paVar6 + -0x4) = 0x0;
            }
          }
        }
        iVar10->field_0x20 = 0xa;
        uVar9 = iVar10->field_0x1e;
        mov_update_win_1040_93aa
                  (*(astruct_65 **)iVar10->field_0x52,0xa,uVar9,&ctx.PTR_LOOP_1050_1040);
        for (uStack16 = 0x1; puVar1 = &iVar10->field_0x18,
            *puVar1 != uStack16 && uStack16 <= *puVar1; uStack16 += 0x1) {
          paVar6 = iVar10->field_0x52;
          uVar3 = (uStack16 * 0x4 + paVar6 + -0x4);
          iVar11 = uVar3;
          uVar16 = (uVar3 >> 0x10);
          paVar6 = iVar10->field_0x52;
          mov_update_win_1040_93aa
                    (*(astruct_65 **)(paVar6 + uStack16 * 0x4),
                     (iVar11 + 0x20) + (iVar11 + 0x24) + 0x8,uVar9,
                     &ctx.PTR_LOOP_1050_1040);
        }
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1010_35a4(param_1: *mut u32,param_2: u32,uchar *param_3)
{
  code **ppcVar1;
  let uVar2: u32;
  let uVar3: u16;
  let puVar4: u32;
  let uVar5: u16;
  let extraout_DX: *mut u8
  let puVar6: *mut u8
  let extraout_DX_00: *mut u8
  let uVar7: u16;
  let unaff_SS: u16;
  let uStack12: u32;
  let puStack8: u32;
  
  uVar7 = (param_1 >> 0x10);
  uVar2 = (param_1 + 0x56);
  uVar2 = (uVar2 + 0x8);
  puStack8 = *(u32 **)(uVar2 + (param_1 + 0x5a) * 0x4);
  uStack12 = param_2;
  if (param_2 != 0x0) {
    uVar7 = 0x1000;
    mem_op_1000_179c(0x4a,param_3,0x1000);
    uVar3 = param_2;
    uVar5 = param_3 | uVar3;
    if (uVar5 == 0x0) {
      uVar3 = 0x0;
      uVar5 = 0x0;
    }
    else {
      uVar7 = SUB42(&ctx.PTR_LOOP_1050_1040,0x0);
      pass1_1040_c54a((u16 *)(param_2 & 0xffff | ZEXT24(param_3) << 0x10),0x1,puStack8,
                      &ctx.PTR_LOOP_1050_1040,unaff_SS);
    }
    ppcVar1 = (code **)(*param_1 + 0x18);
    (**ppcVar1)(uVar7,param_1,0x1,uVar3,uVar5);
    puVar6 = extraout_DX;
    for (; (uStack12 & 0xf) != 0x0; uStack12 >>= 0x4) {
      uVar2 = (puStack8 + 0x8);
      puStack8 = *(u32 **)(((uStack12 & 0xf) - 0x1) * 0x4 + uVar2);
      uVar7 = 0x1000;
      puVar4 = puStack8;
      mem_op_1000_179c(0x4a,puVar6,0x1000);
      uVar3 = puVar4;
      uVar5 = puVar6 | uVar3;
      if (uVar5 == 0x0) {
        uVar3 = 0x0;
        uVar5 = 0x0;
      }
      else {
        uVar7 = SUB42(&ctx.PTR_LOOP_1050_1040,0x0);
        pass1_1040_c54a((u16 *)(puVar4 & 0xffff | ZEXT24(puVar6) << 0x10),0x1,
                        puStack8,&ctx.PTR_LOOP_1050_1040,unaff_SS);
      }
      ppcVar1 = (code **)(*param_1 + 0x18);
      (**ppcVar1)(uVar7,param_1,0x1,uVar3,uVar5);
      puVar6 = extraout_DX_00;
    }
  }
  return;
}



void 
pass1_1010_3680(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
               uchar *param_6,param_7: u16)

{
  mem_op_1000_179c(0x4a,param_6,0x1000);
  if ((param_6 | param_5) != 0x0) {
    pass1_1040_c54a((u16 *)CONCAT22(param_6,param_5),0x1,
                    CONCAT22(param_4,param_3),&ctx.PTR_LOOP_1050_1040,param_7
                   );
    return;
  }
  return;
}



fn pass1_1010_36b4(param_1: *mut u16,param_2: u8) -> u16

{
  let unaff_SS: u16;
  
  pass1_1010_2db2(param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1010_3702(param_1: i16,param_2: u16,param_3: u16) -> u16

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  (param_1 + 0xa) = 0x0;
  CONCAT22(param_2,param_1) = 0x37c4;
  (param_1 + 0x2) = 0x1010;
  return CONCAT22(param_2,param_1);
}



fn pass1_1010_3730(param_1: *mut u16,param_2: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x37c4;
  (param_1 + 0x2) = 0x1010;
  fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0xa),0x1000);
  pass1_1010_1d80(param_1,param_2);
  return;
}



fn pass1_1010_375e(param_1: u32) -> u32

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0xc),(param_1 + 0xa))
  ;
}



fn pass1_1010_3770(param_1: u32,char *param_2,param_3: u16)
{
  let uVar1: u16;
  astruct_477 *iVar3;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar3 = (astruct_477 *)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0xa,0x1000);
  uVar1 = str_op_1008_60e8(param_2,param_3);
  iVar3->field_0xa = uVar1;
  iVar3->field_0xc = param_3;
  return;
}



fn pass1_1010_379e(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_3730(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1010_37d4(param_1: *mut u16) -> u16

{
  let uVar1: u16;
  
  struct_1010_383a(param_1);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x16) = 0x0;
  *param_1 = 0x3b3e;
  (param_1 + 0x2) = 0x1010;
  return param_1;
}



fn pass1_1010_3800(param_1: *mut u16)
{
  astruct_478 *iVar2;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  iVar2 = (astruct_478 *)param_1;
  *param_1 = 0x3b3e;
  iVar2->field_0x2 = 0x1010;
  if (iVar2->field_0x16 != 0x0) {
    fn_ptr_1000_17ce(*(astruct_18 **)iVar2->field_0x16,0x1000);
  }
  pass1_1010_3880(param_1);
  return;
}


fn pass1_1010_3880(param_1: *mut u16)
{
  let piVar1: *mut i16;
  let puVar2: u32;
  let uVar3: u16;
  code **ppcVar4;
  let lVar5: i32;
  astruct_472 *iVar6;
  let iVar7: i16;
  let uVar8: u16;
  let uVar9: u16;
  let iStack4: i16;
  
  uVar8 = (param_1 >> 0x10);
  iVar6 = (astruct_472 *)param_1;
  *param_1 = 0x3b5e;
  iVar6->field_0x2 = 0x1010;
  if (iVar6->field_0x8 != 0x0) {
    iStack4 = 0x0;
    while( true ) {
      piVar1 = &iVar6->field_0x10;
      if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
      lVar5 = iVar6->field_0x8;
      uVar9 = (lVar5 >> 0x10);
      iVar7 = lVar5;
      puVar2 = (iVar7 + iStack4 * 0x4);
      uVar3 = (iVar7 + iStack4 * 0x4 + 0x2);
      if ((uVar3 | puVar2) != 0x0) {
        ppcVar4 = (code **)*puVar2;
        (**ppcVar4)();
      }
      iStack4 += 0x1;
    }
    fn_ptr_1000_17ce((astruct_18 *)iVar6->field_0x8,0x1000);
  }
  *param_1 = 0x389a;
  iVar6->field_0x2 = 0x1008;
  return;
}


fn pass1_1010_394a(param_1: u16,param_2: u16,param_3: i16,param_4: u16,uchar *param_5)
{
  if (param_3 != 0x0) {
    mem_op_1000_179c(param_3 << 0x2,param_5,0x1000);
    return;
  }
  mem_op_1000_179c(0x16,param_5,0x1000);
  if ((param_5 | param_4) != 0x0) {
    struct_1010_383a((u16 *)CONCAT22(param_5,param_4));
    return;
  }
  return;
}



fn pass1_1010_398e(param_1: *mut u32,param_2: u16,param_3: u16,param_4: u32,param_5: u16)
{
  let piVar1: *mut i16;
  code **ppcVar2;
  let uVar3: u32;
  let uVar4: u32;
  let iVar5: i16;
  let uVar6: u16;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let iVar7: i16;
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  let uStack12: u16;
  let puStack6: u32;
  
  uVar9 = (param_1 >> 0x10);
  uVar3 = *param_1;
  ppcVar2 = (code **)(uVar3 + 0x8);
  (**ppcVar2)();
  puStack6 = CONCAT22(extraout_DX,param_5);
  if ((extraout_DX | param_5) == 0x0) {
    return;
  }
  (param_5 + 0xc) = param_4;
  iVar7 = *puStack6;
  ppcVar2 = (code **)(iVar7 + 0xc);
  (**ppcVar2)();
  iVar5 = (param_1 + 0x14);
  piVar1 = (param_1 + 0x14);
  *piVar1 = *piVar1 + 0x1;
  ppcVar2 = (code **)(iVar7 + 0x10);
  (**ppcVar2)();
  ppcVar2 = (code **)(iVar7 + 0x4);
  (**ppcVar2)();
  if (iVar5 != 0x0) {
    ppcVar2 = (code **)(uVar3 + 0x8);
    iVar7 = iVar5;
    (**ppcVar2)();
    (param_5 + 0x8) = iVar7;
    (param_5 + 0xa) = extraout_DX_00;
    ctx.PTR_LOOP_1050_11de = ctx.PTR_LOOP_1050_11de + 0x1;
    uVar9 = extraout_DX_00;
    for (uStack12 = 0x0; uStack12 < iVar5; uStack12 += 0x1) {
      uVar6 = uStack12;
      pass1_1010_398e(param_1,uStack12,uStack12 >> 0xf,puStack6,uStack12);
      uVar4 = (param_5 + 0x8);
      uVar10 = (uVar4 >> 0x10);
      iVar7 = uVar4;
      iVar8 = uStack12 * 0x4;
      (iVar7 + iVar8) = uVar6;
      (iVar7 + iVar8 + 0x2) = uVar9;
      uVar4 = (param_5 + 0x8);
      if (*(long *)(uVar4 + iVar8) == 0x0) break;
    }
    ctx.PTR_LOOP_1050_11de = ctx.PTR_LOOP_1050_11de + -0x1;
  }
  return;
}



fn pass1_1010_3a86(param_1: u32) -> u16

{
  return (param_1 + 0x10);
}



fn pass1_1010_3a94(param_1: u32,param_2: u16)
{
  (param_1 + 0x12) = param_2;
  return;
}



fn pass1_1010_3aaa(param_1: u32) -> u32

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x6),(param_1 + 0x4))
  ;
}



fn pass1_1010_3ac2(param_1: u32,param_2: u16,param_3: u32)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x16) = param_3;
  (param_1 + 0x12) = param_2;
  return;
}



fn pass1_1010_3adc(param_1: u32) -> u32

{
  let puVar1: *mut u16;
  
  puVar1 = (param_1 + 0x16);
  return CONCAT22((puVar1 + 0x2),*puVar1);
}



fn pass1_1010_3af2(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1010_3800(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1010_3b18(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1010_3880(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1010_3bde(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let puVar4: *mut u16;
  astruct_479 *iVar4;
  let uVar5: u16;
  let puStack14: *mut u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = (astruct_479 *)param_1;
  *param_1 = 0x3d6a;
  iVar4->field_0x2 = 0x1010;
  iVar4->field_0xa = 0x3d7a;
  iVar4->field_0xc = 0x1010;
  puVar1 = iVar4->field_0xe;
  uVar2 = iVar4->field_0x10;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  if (param_1 == 0x0) {
    puVar4 = 0x0;
    uVar5 = 0x0;
  }
  else {
    puVar4 = &iVar4->field_0xa;
  }
  puStack14 = CONCAT22(uVar5,puVar4);
  *puStack14 = 0x389a;
  puVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1,param_2);
  return;
}


fn pass1_1010_3c9e(param_1: i32)
{
  let uVar1: u16;
  let puVar2: *mut u8
  
  if (param_1 == 0x0) {
    uVar1 = 0x0;
    puVar2 = 0x0;
  }
  else {
    uVar1 = param_1 + 0xa;
    puVar2 = param_1._2_2_;
  }
  pass1_1008_9262(_PTR_LOOP_1050_0388,(_PTR_LOOP_1050_0388 >> 0x10),
                  (param_1 + 0x12),CONCAT22(puVar2,uVar1),uVar1,
                  puVar2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_3cd0(param_1: i32)
{
  let iVar1: i16;
  let uVar2: u16;
  
  if (_PTR_LOOP_1050_0388 != 0x0) {
    if (param_1 == 0x0) {
      iVar1 = 0x0;
      uVar2 = 0x0;
    }
    else {
      iVar1 = param_1 + 0xa;
      uVar2 = param_1._2_2_;
    }
    pass1_1008_92b2(_PTR_LOOP_1050_0388,(param_1 + 0x12),
                    CONCAT22(uVar2,iVar1));
  }
  return;
}



fn pass1_1010_3d0a(param_1: i16,param_2: u16,param_3: i16,param_4: u16)
{
  if (param_3 == 0x2) {
    pass1_1010_3cd0(CONCAT22(param_2,param_1 + -0xa));
    pass1_1010_1f62(param_4,CONCAT22(param_2,param_1 + -0xa),0x2);
  }
  return;
}



fn pass1_1010_3d38(param_1: *mut u16,param_2: u8) -> u16

{
  let unaff_SS: u16;
  
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0xa));
  pass1_1010_3bde(param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_3d82(astruct_628 *param_1,param_2: u16,param_3: u16,param_4: u16) -> u32

{
  astruct_43 *paVar1;
  
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  &param_1->field_0xa = 0x0;
  CONCAT22(param_2,param_1) = 0x3e2c;
  param_1->field_0x2 = 0x1010;
  paVar1 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x99,param_4);
  param_1->field_0xa = paVar1;
  param_1->field_0xc = (paVar1 >> 0x10);
  return CONCAT22(param_2,param_1);
}



fn pass1_1010_3dc8(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_480 *iVar4;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_480 *)param_1;
  *param_1 = 0x3e2c;
  iVar4->field_0x2 = 0x1010;
  puVar1 = iVar4->field_0xa;
  uVar2 = iVar4->field_0xc;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}



fn pass1_1010_3e06(param_1: *mut u16,param_2: u8) -> u16

{
  let unaff_SS: u16;
  
  pass1_1010_3dc8(param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_3e3c(astruct_55 *param_1,param_2: u16,param_3: u16)
{
  astruct_633 *iVar1;
  let uVar1: u16;
  astruct_43 *paVar2;
  
  get_sys_metrics_1018_4b1e(param_1,0x6,param_2);
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_633 *)param_1;
  iVar1->field_0x20 = 0x389a;
  iVar1->field_0x22 = 0x1008;
  iVar1->field_0x20 = 0x3aa8;
  iVar1->field_0x22 = 0x1008;
  iVar1->field_0x24 = 0x0;
  &iVar1->field_0x66 = 0x0;
  iVar1->field_0x6a = 0x4;
  iVar1->field_0x6c = 0x0;
  iVar1->field_0x70 = 0x0;
  iVar1->field_0x74 = 0x0;
  pass1_1008_3e54((u16 *)
                  (param_1 & 0xffff0000 | &iVar1->field_0x76),0x0,0x3,
                  0x5);
  iVar1->field_0x7c = 0x0;
  param_1->field_0x0 = &ctx.PTR_LOOP_1050_4a46;
  iVar1->field_0x2 = 0x1010;
  iVar1->field_0x20 = &ctx.PTR_LOOP_1050_4a82;
  iVar1->field_0x22 = 0x1010;
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &iVar1->field_0x26),
                  (WNDCLASS16 *)0x0,0x40);
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1a1,param_3);
  iVar1->field_0x66 = paVar2;
  iVar1->field_0x68 = (paVar2 >> 0x10);
  pass1_1018_4b78(param_1,param_3);
  return;
}



fn pass1_1010_3f00(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let piVar4: *mut i16;
  astruct_481 *iVar5;
  let uVar5: u16;
  let puStack16: *mut u16;
  let iStack4: i16;
  
  uVar5 = (param_1 >> 0x10);
  iVar5 = (astruct_481 *)param_1;
  *param_1 = &ctx.PTR_LOOP_1050_4a46;
  iVar5->field_0x2 = 0x1010;
  iVar5->field_0x20 = &ctx.PTR_LOOP_1050_4a82;
  iVar5->field_0x22 = 0x1010;
  iStack4 = 0x0;
  do {
    puVar1 = (&iVar5->field_0x26 + iStack4 * 0x4);
    uVar2 = (&iVar5->field_0x26 + iStack4 * 0x4 + 0x2);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    iStack4 += 0x1;
  } while (iStack4 < 0x10);
  puVar1 = iVar5->field_0x66;
  uVar2 = iVar5->field_0x68;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x70,0x1000);
  if (param_1 == 0x0) {
    piVar4 = 0x0;
    uVar5 = 0x0;
  }
  else {
    piVar4 = &iVar5->field_0x20;
  }
  puStack16 = CONCAT22(uVar5,piVar4);
  *puStack16 = 0x389a;
  piVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1,param_2);
  return;
}



fn pass1_1010_404a(param_1: u32,param_2: u32,param_3: i16,param_4: u16)
{
  let uVar1: u16;
  let iVar2: i16;
  let BVar3: bool;
  let uVar4: u16;
  let uVar5: u16;
  let local_4: u16;
  
  uVar4 = param_2;
  uVar5 = (param_2 >> 0x10);
  read_file_1008_7cfe(uVar4,uVar5,0x5,0x1008,param_4);
  if (param_3 == 0x0) {
    ctx.PTR_LOOP_1050_0310 = 0x6d4;
  }
  else {
    iVar2 = param_1;
    uVar1 = (param_1 >> 0x10);
    BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x24,0x0,uVar1,0x2,0x1008);
    if (BVar3 != 0x0) {
      BVar3 = read_file_1008_7dee(uVar4,uVar5,&local_4,0x0,param_4,0x2,0x1008);
      if (BVar3 != 0x0) {
        BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x7e,0x0,uVar1,0x2,0x1008);
        if (BVar3 != 0x0) {
          (iVar2 + 0x6a) = local_4;
          return;
        }
      }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d0;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_40cc(param_1: u32,param_2: i16,param_3: u16) -> u32

{
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,(param_1 + 0x6c));
  return CONCAT22((param_2 + 0xe),(param_2 + 0xc));
}


fn pass1_1010_41d6(param_1: u32,param_2: u32,uchar *param_3,param_4: u16,param_5: u8)
{
  let puVar1: *mut u16;
  let piVar2: *mut i16;
  let uVar3: u32;
  let uVar4: u16;
  let uVar5: u16;
  let iVar6: i16;
  let iVar7: i16;
  let puVar8: *mut u8
  let puVar9: *mut u8
  astruct_243 *iVar9;
  astruct_244 *iVar10;
  let unaff_DI: i16;
  let uVar10: u16;
  let uVar11: u16;
  let puVar12: *mut u16;
  let iStack50: i16;
  let local_30: i16;
  astruct_18 *local_2e;
  let iStack42: i16;
  astruct_18 *paStack40;
  astruct_18 *paStack34;
  astruct_18 *paStack30;
  let iStack26: i16;
  let uStack24: u16;
  let iStack22: i16;
  let uStack20: u32;
  let uStack16: u16;
  let uStack14: u32;
  let uStack10: u32;
  let uStack6: u16;
  let uStack4: u16;
  
  uVar10 = (param_1 >> 0x10);
  iVar9 = (astruct_243 *)param_1;
  iVar9->field_0x6c = param_2;
  puVar12 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_4,param_3,unaff_DI);
  uStack4 = (puVar12 >> 0x10);
  uStack6 = puVar12;
  uStack10 = pass1_1010_ec40(uStack6,uStack4,iVar9->field_0x6c,uStack6,uStack4);
  puVar8 = (uStack10 >> 0x10);
  iVar9->field_0x74 = (uStack10 + 0x22);
  if (*(long *)&iVar9->field_0x70 != 0x0) {
    paStack34 = *(astruct_18 **)&iVar9->field_0x70;
    paStack30 = paStack34;
    fn_ptr_1000_17ce(paStack34,0x1000);
    &iVar9->field_0x70 = 0x0;
  }
  uVar4 = iVar9->field_0x74 << 0x7;
  mem_op_1000_179c(uVar4,puVar8,0x1000);
  &iVar9->field_0x70 = uVar4;
  iVar9->field_0x72 = puVar8;
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,iVar9->field_0x6c);
  uStack14 = CONCAT22(puVar8,uVar4);
  uStack16 = (*(uVar4 + 0x10) == 0x9);
  iStack22 = (uStack10 + 0x22);
  uVar4 = iStack22 * 0x6;
  mem_op_1000_179c(uVar4,puVar8,0x1000);
  paStack30 = (astruct_18 *)CONCAT22(puVar8,uVar4);
  puVar9 = (puVar8 | uVar4);
  if (puVar9 == 0x0) {
    uStack20 = (astruct_18 *)0x0;
  }
  else {
    pass1_1000_5586((uchar *)0x3e38,0x1008,iStack22,0x6,uVar4,puVar8);
    uStack20 = paStack30;
  }
  uStack24 = 0x0;
  while( true ) {
    uVar11 = (uStack10 >> 0x10);
    puVar1 = (uStack10 + 0x22);
    if (*puVar1 < uStack24 || *puVar1 == uStack24) break;
    uVar3 = (uStack10 + 0x24);
    uVar5 = uStack24;
    pass1_1028_e0a0(_PTR_LOOP_1050_65e2,
                    (uVar3 + uStack24 * 0x2) << 0x10,puVar9,param_4,
                    param_5);
    paStack34 = (astruct_18 *)CONCAT22(puVar9,uVar5);
    pass1_1008_3f62((u16 *)
                    (uStack20 & 0xffff0000 |
                    (uStack24 * 0x6 + uStack20)),
                    CONCAT22(puVar9,uVar5 + 0x8));
    paStack40 = paStack34;
    paStack30 = paStack34;
    if (paStack34 != (astruct_18 *)0x0) {
      fn_ptr_1030_84d0(paStack34);
      fn_ptr_1000_17ce(paStack34,0x1000);
    }
    uStack24 += 0x1;
    puVar9 = uStack20._2_2_;
  }
  for (iStack26 = 0x0; piVar2 = &iVar9->field_0x74,
      *piVar2 != iStack26 && iStack26 <= *piVar2; iStack26 += 0x1) {
    pass1_1008_3e94((u16 *)
                    (uStack20 & 0xffff0000 |
                    (iStack26 * 0x6 + uStack20)),
                    CONCAT22(param_4,&local_2e),
                    CONCAT22(param_4,&local_30));
    iStack42 = pass1_1000_49b2(local_2e);
    iStack42 /= 0x5;
    if (0xc < iStack42) {
      iStack42 = 0xc;
      iVar6 = pass1_1000_49b2(local_2e);
      local_2e = (astruct_18 *)
                 (local_2e & 0xffff0000 |
                 ((local_2e / iVar6) * 0x3c));
    }
    iVar7 = pass1_1000_49b2(local_2e);
    iVar6 = ((long)iVar7 % 0x5);
    paStack34 = (astruct_18 *)
                (paStack34 & 0xffff0000 | (long)iVar7 % 0x5 & 0xffff);
    if (local_2e < 0x0) {
      if (0x2 < iVar6) {
        iVar6 += -0x5;
      }
      local_2e = (astruct_18 *)
                 (local_2e & 0xffff0000 | (local_2e + iVar6));
    }
    else {
      if (iVar6 < 0x3) {
        local_2e = (astruct_18 *)
                   (local_2e & 0xffff0000 | (local_2e - iVar6));
      }
      else {
        local_2e = (astruct_18 *)
                   (local_2e & 0xffff0000 | (local_2e + (0x5 - iVar6))
                   );
      }
    }
    iStack50 = local_30 / 0x16;
    for (iVar6 = 0x0; iVar6 < 0x10; iVar6 += 0x1) {
      if (0xf < iStack50) {
        iStack50 = 0x0;
      }
      if (((uStack16 != 0x0) < iStack50) && (iStack50 < 0x8)) {
        iVar7 = ((iStack42 * 0x10 + iStack50) * 0x2 + 0x11e0);
        iVar10 = (astruct_244 *)((iStack26 * 0x10 + iVar6) * 0x8);
        uVar3 = &iVar9->field_0x70;
        (iVar10 + uVar3) = iVar7 + 0x49;
        uVar3 = &iVar9->field_0x70;
        (iVar10 + uVar3 + 0x2) = local_2e + 0x49;
        uVar3 = &iVar9->field_0x70;
        (iVar10 + uVar3 + 0x4) = iVar7 + 0x4e;
        uVar3 = &iVar9->field_0x70;
        (iVar10 + uVar3 + 0x6) = local_2e + 0x4e;
      }
      else {
        iVar7 = (iStack26 * 0x10 + iVar6) * 0x8;
        uVar3 = &iVar9->field_0x70;
        (iVar7 + uVar3) = 0x0;
        uVar3 = &iVar9->field_0x70;
        (uVar3 + iVar7 + 0x2) = 0x0;
        uVar3 = &iVar9->field_0x70;
        (uVar3 + iVar7 + 0x4) = 0x1;
        uVar3 = &iVar9->field_0x70;
        (uVar3 + iVar7 + 0x6) = 0x1;
      }
      iStack50 += 0x1;
    }
  }
  paStack40 = uStack20;
  local_2e = uStack20;
  fn_ptr_1000_17ce(uStack20,0x1000);
  draw_1010_47ae(param_1,0x1000,param_4);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_451a(param_1: u32,uchar *param_2,param_3: i16,param_4: u16) -> u32

{
  let uVar1: u16;
  let uVar2: u16;
  let puVar3: *mut u16;
  let uVar4: u32;
  
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_4,param_2,param_3);
  uVar1 = (puVar3 >> 0x10);
  uVar4 = pass1_1010_ec40(puVar3,uVar1,(param_1 + 0x6c),
                          puVar3,uVar1);
  uVar2 = (uVar4 >> 0x10);
  return CONCAT22((uVar4 + 0x4),(uVar4 + 0x2));
}



fn pass1_1010_454a(param_1: u32) -> u32

{
  let iVar1: i16;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar2 = (iVar1 + 0x24) * 0x4;
  return CONCAT22((iVar1 + iVar2 + 0x28),
                  (iVar1 + iVar2 + 0x26));
}



fn pass1_1010_4566(param_1: i16,param_2: u16,param_3: i16,param_4: u16)
{
  if (param_3 != 0x2) {
    return;
  }
  pass1_1010_4956(CONCAT22(param_2,param_1 + -0x20));
  pass1_1010_1f62(param_4,CONCAT22(param_2,param_1 + -0x20),0x2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_459e(param_1: i32)
{
  let uVar1: u16;
  let puVar2: *mut u8
  
  if (param_1 == 0x0) {
    uVar1 = 0x0;
    puVar2 = 0x0;
  }
  else {
    uVar1 = param_1 + 0x20;
    puVar2 = param_1._2_2_;
  }
  pass1_1008_9262(_PTR_LOOP_1050_0388,(_PTR_LOOP_1050_0388 >> 0x10),
                  0x1f4,CONCAT22(puVar2,uVar1),uVar1,puVar2);
  (param_1 + 0x7e) = 0x1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_45d6(param_1: i32,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  let iVar6: i16;
  let uVar7: u16;
  let iStack4: i16;
  
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  if ((iVar6 + 0x7e) != 0x0) {
    if (_PTR_LOOP_1050_0388 != 0x0) {
      if (param_1 == 0x0) {
        iVar4 = 0x0;
        uVar5 = 0x0;
      }
      else {
        iVar4 = iVar6 + 0x20;
        uVar5 = uVar7;
      }
      param_2 = 0x1008;
      pass1_1008_92b2(_PTR_LOOP_1050_0388,0x1f4,CONCAT22(uVar5,iVar4));
    }
    for (iStack4 = 0x0; iStack4 < 0x10; iStack4 += 0x1) {
      if ((iVar6 + 0x24) != iStack4) {
        puVar1 = (iVar6 + 0x26 + iStack4 * 0x4);
        uVar2 = (iVar6 + 0x26 + iStack4 * 0x4 + 0x2);
        if ((uVar2 | puVar1) != 0x0) {
          ppcVar3 = (code **)*puVar1;
          (**ppcVar3)(param_2,puVar1,uVar2,0x1);
        }
        (iVar6 + iStack4 * 0x4 + 0x26) = 0x0;
      }
    }
    (iVar6 + 0x7e) = 0x0;
  }
  return;
}



fn pass1_1010_4674(param_1: u32,param_2: i16,param_3: u16,param_4: u16)
{
  let piVar1: *mut i16;
  let UVar2: u32;
  let UVar3: u16;
  
  UVar2 = param_1;
  UVar3 = (param_1 >> 0x10);
  if (param_2 == 0x1) {
    piVar1 = (UVar2 + 0x24);
    *piVar1 = *piVar1 + 0x1;
    if (0xf < (UVar2 + 0x24)) {
      (UVar2 + 0x24) = 0x0;
    }
//LAB_1010_469a:
    draw_op_1010_47d0(UVar2,UVar3,(UVar2 + 0x24),param_3,param_4);
  }
  else {
    if (param_2 != 0x2) {
      if (param_2 != 0x3) {
        if (((UVar2 + 0x6a) != 0x0) && ((UVar2 + 0x6a) != 0x4)) {
          pass1_1010_459e(param_1);
        }
        goto LAB_1010_46e8;
      }
      piVar1 = (UVar2 + 0x24);
      *piVar1 = *piVar1 + -0x1;
      if (*piVar1 < 0x0) {
        (UVar2 + 0x24) = 0xf;
      }
      goto LAB_1010_469a;
    }
  }
  pass1_1010_1f62(param_4,param_1,0x2);
  pass1_1010_45d6(param_1,param_3);
//LAB_1010_46e8:
  (UVar2 + 0x6a) = param_2;
  return;
}


fn pass1_1010_4788(param_1: u32,char *param_2,param_3: u16,param_4: u16)
{
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,(param_1 + 0x6c));
  pass1_1030_301a(CONCAT22(param_4,param_3),param_2,param_4);
  return;
}


fn pass1_1010_4956(param_1: u32)
{
  let piVar1: *mut i16;
  let iVar2: i16;
  let iVar3: i16;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  iVar2 = (iVar3 + 0x6a);
  if (iVar2 == 0x0) {
    piVar1 = (iVar3 + 0x24);
    *piVar1 = *piVar1 + 0x1;
    if (0xf < (iVar3 + 0x24)) {
      (iVar3 + 0x24) = 0x0;
      return;
    }
  }
  else {
    if (iVar2 != 0x4) {
      return;
    }
    piVar1 = (iVar3 + 0x24);
    *piVar1 = *piVar1 + -0x1;
    if (*piVar1 < 0x0) {
      (iVar3 + 0x24) = 0xf;
    }
  }
  return;
}



astruct_18 *
pass1_1010_4994(param_1: u16,astruct_18 *param_2,param_3: u8,param_4: u16)

{
  param_2 = (astruct_18 *)(param_2 & 0xffff0000 | (param_2 - 0x20));
  pass1_1010_3f00((u16 *)param_2,param_4);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_2,0x1000);
  }
  return param_2;
}



fn pass1_1010_49a0(param_1: i16,param_2: u16) -> u32

{
  return CONCAT22(param_2,param_1 + 0xa);
}



fn pass1_1010_49b0(param_1: i16,param_2: u16) -> u32

{
  return CONCAT22(param_2,param_1 + 0x18);
}



fn pass1_1010_49c0(param_1: u32) -> u16

{
  return (param_1 + 0x14);
}



fn pass1_1010_49ce(param_1: u32,param_2: u16)
{
  (param_1 + 0x14) = param_2;
  return;
}



fn pass1_1010_49e0(param_1: u32) -> u16

{
  return (param_1 + 0x16);
}



fn pass1_1010_49ee(param_1: u32,param_2: u16)
{
  (param_1 + 0x16) = param_2;
  return;
}



fn pass1_1010_4a00(param_1: u32,param_2: u16)
{
  (param_1 + 0x12) = param_2;
  return;
}



fn pass1_1010_4a12(param_1: u32) -> u16

{
  return (param_1 + 0x12);
}



fn pass1_1010_4a20(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_3f00(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_4a8a(astruct_637 *param_1,param_2: u16,param_3: u16,param_4: u16)
{
  let puVar1: *mut u8
  let unaff_DI: i16;
  astruct_43 *paVar2;
  let puVar3: *mut u16;
  
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0x16 = (astruct_76 *)0x0;
  param_1->field_0x1a = 0x0;
  param_1->field_0x1e = 0x0;
  param_1->field_0x20 = 0x1;
  param_1->field_0x22 = 0x0;
  param_1->field_0x24 = 0x0;
  &param_1->field_0x26 = 0x0;
  param_1->field_0x2a = 0x0;
  param_1->field_0x2c = 0x1;
  param_1->field_0x2e = 0x0;
  param_1->field_0x30 = 0x0;
  param_1->field_0x32 = 0x0;
  CONCAT22(param_2,param_1) =
       (s_SCForceMorale__s_for_colony__08l_1050_5024 + 0x6);
  param_1->field_0x2 = 0x1010;
  paVar2 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1b3,param_4);
  puVar1 = (paVar2 >> 0x10);
  &param_1->field_0x16 = paVar2;
  *(uchar **)(&param_1->field_0x16 + 0x2) = puVar1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_4,puVar1,unaff_DI);
  param_1->field_0x26 = puVar3;
  param_1->field_0x28 = (puVar3 >> 0x10);
  pass1_1008_4772(param_1->field_0x16);
  param_1->field_0xe = 0x13c;
  param_1->field_0xa = 0x0;
  param_1->field_0x10 = 0x0;
  param_1->field_0xc = 0x0;
  return;
}


fn pass1_1010_4c2c(param_1: u32) -> u32

{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x18),
                  (param_1 + 0x16));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_4c3e(param_1: u32,param_2: i16,param_3: i16,uchar *param_4,param_5: u16)
{
  let piVar1: *mut i16;
  let uVar2: u32;
  let iVar3: i16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  astruct_43 *paVar7;
  let uVar8: u32;
  let iStack14: i16;
  let local_c: [u8;6];
  let uStack6: u16;
  let iStack4: i16;
  
  uVar5 = (param_1 >> 0x10);
  iVar3 = param_1;
  pass1_1010_bffa((iVar3 + 0x26),param_3,param_4,param_5);
  (iVar3 + 0x12) = param_3;
  *(uchar **)(iVar3 + 0x14) = param_4;
  if ((param_4 | (iVar3 + 0x12)) != 0x0) {
    if (param_2 == 0x0) {
      uVar2 = (iVar3 + 0x12);
      (iVar3 + 0x30) = (uVar2 + 0x8);
    }
    else {
      (iVar3 + 0x2e) = 0x1;
      uVar2 = (iVar3 + 0x12);
      uVar2 = (uVar2 + 0x4);
      iVar4 = (uVar2 + 0x2);
      if ((iVar4 == 0x5) || (iVar4 == 0x6)) {
        (iVar3 + 0x30) = 0x1;
        (iVar3 + 0x20) = 0x0;
      }
      else {
        (iVar3 + 0x30) = 0x2;
        uVar2 = (iVar3 + 0x12);
        (iVar3 + 0x32) = (uVar2 + 0x4);
        paVar7 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1bf,param_5);
        uVar2 = (iVar3 + 0x12);
        uVar6 = (uVar2 >> 0x10);
        iVar4 = uVar2;
        (iVar4 + 0x4) = paVar7;
        (iVar4 + 0x6) = (paVar7 >> 0x10);
      }
    }
    iStack4 = 0x14;
    pass1_1008_3e38((u16 *)CONCAT22(param_5,local_c));
    uStack6 = 0x0;
    iStack14 = 0x0;
    while( true ) {
      piVar1 = (iVar3 + 0x30);
      if (*piVar1 == iStack14 || *piVar1 < iStack14) break;
      uVar2 = (iVar3 + 0x12);
      uVar8 = pass1_1008_4772(*(astruct_76 **)(uVar2 + iStack14 * 0x4));
      iStack4 += (-(iStack14 == 0x0) & 0x5) + 0x14 + (uVar8 + 0x4);
      iStack14 += 0x1;
    }
    if ((iVar3 + 0xe) < iStack4) {
      (iVar3 + 0xe) = iStack4;
    }
  }
  return;
}


fn pass1_1010_4dc8(param_1: u32) -> u32

{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((iVar1 + 0x20) == 0x0) {
    return 0x0;
  }
  return CONCAT22((iVar1 + 0x1c),
                  (iVar1 + 0x20) * 0x8 + (iVar1 + 0x1a));
}



fn pass1_1010_4df0(param_1: u32,param_2: u16,param_3: u16)
{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  uVar1 = (param_1 + 0x26);
  pass1_1010_c1ba(uVar1,(uVar1 >> 0x10),
                  (param_1 + 0x20),param_2,param_3);
  return;
}


fn pass1_1010_4e8c(param_1: u32,param_2: u16)
{
  pass1_1010_1f62(param_2,param_1,0xd);
  return;
}



fn pass1_1010_4f20(param_1: u16,param_2: u16,param_3: i16) -> u16

{
  return (param_3 * 0x2 + 0x139a);
}



fn pass1_1010_4f30(param_1: u16,param_2: u16,param_3: *mut u16,param_4: *mut u16)
{
  *param_4 = 0xa;
  *param_3 = 0x73;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_4f48(param_1: u32,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let puVar4: u32;
  let uVar5: u32;
  astruct_482 *iVar6;
  astruct_483 *iVar7;
  let uVar6: u16;
  let uVar7: u16;
  astruct_43 *paVar8;
  
  uVar6 = (param_1 >> 0x10);
  iVar6 = (astruct_482 *)param_1;
  puVar4 = iVar6->field_0x12;
  iVar6->field_0x30 = (puVar4 + 0x8);
  if (iVar6->field_0x32 != 0x0) {
    uVar5 = *iVar6->field_0x12;
    uVar7 = (uVar5 >> 0x10);
    iVar7 = (astruct_483 *)uVar5;
    puVar4 = iVar7->field_0x4;
    iVar7->field_0x4 = iVar6->field_0x32;
    if (puVar4 != 0x0) {
      ppcVar3 = (code **)*puVar4;
      (**ppcVar3)();
    }
    iVar6->field_0x32 = 0x0;
  }
  puVar1 = iVar6->field_0x16;
  uVar2 = iVar6->field_0x18;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  paVar8 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1b3,param_2);
  iVar6->field_0x16 = paVar8;
  iVar6->field_0x18 = (paVar8 >> 0x10);
  fn_ptr_1000_17ce((astruct_18 *)iVar6->field_0x1a,0x1000);
  iVar6->field_0x1a = 0x0;
  iVar6->field_0x2e = 0x0;
  return;
}



fn pass1_1010_5004(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  free_rsrc_1010_4b3e(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_503e(param_1: i16,param_2: u16,param_3: u16,uchar *param_4,param_5: u16)
{
  struct_op_1018_4cda(param_1,param_2,param_3);
  CONCAT22(param_2,param_1) =
       (s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x1);
  (param_1 + 0x2) = 0x1010;
  pass1_1018_4dce(CONCAT22(param_2,param_1),0x1b3,param_4,param_5);
  _PTR_LOOP_1050_4230 = CONCAT22(param_2,param_1);
  return;
}



astruct_11 *  pass1_1010_5074(astruct_11 *param_1,param_2: u8)

{
  clenaup_win_ui_1018_4d22(param_1,0x1018);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1010_50f2(param_1: *mut u16,param_2: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x53f4;
  (param_1 + 0x2) = 0x1010;
  fn_ptr_1000_17ce(*(astruct_18 **)(param_1 + 0xc),0x1000);
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_5120(param_1: u32,param_2: u16,param_3: u16,param_4: u16,param_5: u16)
{
  let uVar1: u32;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u32;
  let uVar6: u16;
  let uVar7: u16;
  let iVar8: i16;
  let iVar9: i16;
  let uVar10: u16;
  
  uVar10 = (param_1 >> 0x10);
  iVar9 = param_1;
  if (*(long *)(iVar9 + 0x16) != 0x0) {
    uVar1 = (iVar9 + 0x16);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
    uVar6 = param_4 | param_3;
    if (uVar6 != 0x0) {
      uVar2 = (param_3 + 0x1f6);
      uVar5 = uVar2;
      pass1_1030_38f2(uVar2,0x3,param_5);
      uVar3 = uVar5;
      uVar7 = uVar6;
      uVar4 = uVar3;
      pass1_1030_38f2(uVar2,0x4,param_5);
      iVar8 = uVar7 + uVar6 + CARRY2(uVar4,uVar3);
      if ((0x0 < iVar8) || ((-0x1 < iVar8 && (param_2 <= uVar4 + uVar3)))) {
        (iVar9 + 0xa) = param_2;
        return;
      }
    }
  }
  return;
}



fn pass1_1010_519a(param_1: u32,i16 *param_2,uchar *param_3,param_4: u16)
{
  let uVar1: u32;
  let uVar2: u16;
  let puVar3: *mut u8;
  let puVar4: *mut u8
  astruct_246 *iVar5;
  astruct_247 *iVar6;
  let iVar7: i16;
  let uVar8: u16;
  let uVar9: u16;
  let piStack44: *mut i16;
  u8 local_18 [0xc];
  let iStack12: i16;
  let uStack6: u32;
  
  uStack6 = 0x0;
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_4,local_18),0x1,0x0,0x400);
  uVar8 = (param_1 >> 0x10);
  iVar5 = (astruct_246 *)param_1;
  iVar5->field_0x10 = iStack12;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar5->field_0xc,0x1000);
  uVar2 = iVar5->field_0x10 << 0x2;
  mem_op_1000_179c(uVar2,param_3,0x1000);
  iVar5->field_0xc = uVar2;
  iVar5->field_0xe = param_3;
  iVar5->field_0x10 = 0x0;
  while( true ) {
    puVar4 = param_3;
    puVar3 = local_18;
    pass1_1028_e4ec(CONCAT22(param_4,puVar3));
    uStack6 = CONCAT22(puVar4,puVar3);
    if ((uchar *)(puVar4 | puVar3) == 0x0) break;
    param_3 = (puVar4 | puVar3);
    if (*(long *)(puVar3 + 0x200) != 0x8000002) {
      param_3 = *(uchar **)(puVar3 + 0x6);
      uVar1 = &iVar5->field_0xc;
      uVar9 = (uVar1 >> 0x10);
      iVar7 = uVar1;
      iVar6 = (astruct_247 *)(iVar5->field_0x10 * 0x4);
      piStack44 = (param_1 & 0xffff0000 | ZEXT24(&iVar5->field_0x10));
      (iVar6 + iVar7) = (puVar3 + 0x4);
      *(uchar **)(iVar6 + iVar7 + 0x2) = param_3;
      *piStack44 = *piStack44 + 0x1;
    }
  }
  *param_2 = iVar5->field_0x10;
  return;
}


fn pass1_1010_52fc(param_1: u32,param_2: u32,param_3: u16,uchar *param_4,param_5: u16)
{
  let uVar1: u16;
  
  pass1_1010_533c(param_1,param_2,param_4,param_5);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x12) = param_3;
  *(uchar **)(param_1 + 0x14) = param_4;
  return;
}



fn pass1_1010_531c(param_1: u32,param_2: u32,param_3: u16,uchar *param_4,param_5: u16)
{
  let uVar1: u16;
  
  pass1_1010_533c(param_1,param_2,param_4,param_5);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x16) = param_3;
  *(uchar **)(param_1 + 0x18) = param_4;
  return;
}



fn pass1_1010_533c(param_1: u32,param_2: u32,uchar *param_3,param_4: u16)
{
  let puVar1: *mut u16;
  let uVar2: u32;
  let uVar3: u32;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  astruct_18 *paVar7;
  let uStack6: u16;
  let local_4: [u8;2];
  
  pass1_1010_519a(param_1,CONCAT22(param_4,local_4),param_3,param_4);
  uStack6 = 0x0;
  while( true ) {
    uVar6 = (param_1 >> 0x10);
    uVar5 = param_1;
    puVar1 = (uVar5 + 0x10);
    if (*puVar1 < uStack6 || *puVar1 == uStack6) {
      return;
    }
    uVar3 = (uVar5 + 0xc);
    uVar2 = (uVar3 + uStack6 * 0x4);
    paVar7 = (astruct_18 *)string_1010_5286(uVar5,uVar6,uVar2,uVar2,param_3)
    ;
    param_3 = (paVar7 >> 0x10);
    iVar4 = pass1_1000_3d7a(param_2,paVar7 & 0xffff | ZEXT24(param_3) << 0x10);
    if (iVar4 == 0x0) break;
    fn_ptr_1000_17ce(paVar7,0x1000);
    uStack6 += 0x1;
  }
  return;
}



fn pass1_1010_53ce(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_50f2(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1010_5d9c(param_1: u32,param_2: i16,uchar *param_3,param_4: i16,param_5: u16)
{
  let puVar1: *mut u16;
  
  (param_1 + 0x1e) = param_2;
  if (param_2 == 0x0) {
    puVar1 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2e,param_5,param_3,param_4);
    pass1_1018_209c(puVar1);
  }
  return;
}



fn pass1_1010_5dc6(param_1: u32,param_2: u32,param_3: u16) -> u16

{
  let BVar1: bool;
  let iVar2: i16;
  let uVar3: u16;
  let uVar4: u16;
  u8 *local_c [0x3];
  let local_6: [u16;0x2];
  
  BVar1 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar1 != 0x0) {
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    BVar1 = pass1_1008_7c2a(param_2,(iVar2 + 0x68),0x1008);
    if (BVar1 != 0x0) {
      BVar1 = pass1_1008_7c2a(param_2,(iVar2 + 0x6c),0x1008);
      if (BVar1 != 0x0) {
        local_c[0] = ctx.PTR_LOOP_1050_13ae;
        uVar4 = (param_2 >> 0x10);
        BVar1 = write_to_file_1008_7e1c
                          (param_2,uVar4,local_c,param_3,0x2,
                           0x1008);
        if (BVar1 != 0x0) {
          local_6[0] = (iVar2 + 0x82);
          BVar1 = write_to_file_1008_7e1c
                            (param_2,uVar4,local_6,param_3,0x2,
                             0x1008);
          if (BVar1 != 0x0) {
            return 0x1;
          }
        }
      }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d0;
  }
  return 0x0;
}



fn pass1_1010_5e56(param_1: u32,param_2: u32,param_3: i16,param_4: u16,param_5: u16)
{
  let puVar1: *mut u8;
  let uVar2: u16;
  let BVar3: bool;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let local_404: *mut u8;
  let local_402: [u8;400];
  
  uVar6 = param_2;
  uVar7 = (param_2 >> 0x10);
  read_file_1008_7cfe(uVar6,uVar7,0x4,0x1008,param_5);
  if (param_3 == 0x0) {
    ctx.PTR_LOOP_1050_0310 = 0x6d4;
  }
  else {
    puVar1 = local_402;
    read_file_1008_7c6e(uVar6,uVar7,CONCAT22(param_5,puVar1),0x1008);
    if (puVar1 != 0x0) {
      uVar2 = str_op_1008_60e8(CONCAT22(param_5,local_402),param_4);
      uVar5 = (param_1 >> 0x10);
      iVar4 = param_1;
      (iVar4 + 0x68) = uVar2;
      (iVar4 + 0x6a) = param_4;
      puVar1 = local_402;
      read_file_1008_7c6e(uVar6,uVar7,CONCAT22(param_5,puVar1),0x1008);
      if (puVar1 != 0x0) {
        uVar2 = str_op_1008_60e8(CONCAT22(param_5,local_402),param_4);
        (iVar4 + 0x6c) = uVar2;
        (iVar4 + 0x6e) = param_4;
        BVar3 = read_file_1008_7dee(uVar6,uVar7,&local_404,0x0,param_5,0x2,0x1008)
        ;
        if (BVar3 != 0x0) {
          ctx.PTR_LOOP_1050_13ae = local_404;
          if (ctx.PTR_LOOP_1050_0312 < 0x2) {
            return;
          }
          BVar3 = read_file_1008_7dee(uVar6,uVar7,iVar4 + 0x82,0x0,uVar5,0x2,0x1008);
          if (BVar3 != 0x0) {
            return;
          }
        }
      }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
  }
  return;
}


fn pass1_1010_5f4c(param_1: u32,char *param_2,param_3: u16)
{
  let uVar1: u16;
  astruct_484 *iVar3;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar3 = (astruct_484 *)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0x12,0x1000);
  uVar1 = str_op_1008_60e8(param_2,param_3);
  iVar3->field_0x12 = uVar1;
  iVar3->field_0x14 = param_3;
  return;
}



fn pass1_1010_5f7a(param_1: i16,param_2: u16,param_3: u16,param_4: i16) -> u32

{
  let iVar1: i16;
  
  iVar1 = param_4 * 0x8 + param_1;
  if (((iVar1 + 0x26) == 0x0) && ((iVar1 + 0x28) == 0x0)) {
    return 0x0;
  }
  return CONCAT22(param_2,param_4 * 0x8 + param_1 + 0x22);
}



fn pass1_1010_5fb0(param_1: u32,param_2: u16,param_3: *mut u32,param_4: u16,param_5: i16)
{
  let uVar1: u16;
  astruct_656 *iVar1;
  
  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_656 *)(param_1 + param_5 * 0x8);
  iVar1->field_0x22 = *param_3;
  iVar1->field_0x26 = param_3[0x1];
  return;
}



fn pass1_1010_5fd8(param_1: u32,char *param_2,param_3: u16)
{
  let uVar1: u16;
  astruct_485 *iVar3;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar3 = (astruct_485 *)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0x68,0x1000);
  uVar1 = str_op_1008_60e8(param_2,param_3);
  iVar3->field_0x68 = uVar1;
  iVar3->field_0x6a = param_3;
  return;
}



fn pass1_1010_6006(param_1: u32,char *param_2,param_3: u16)
{
  let uVar1: u16;
  astruct_486 *iVar3;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar3 = (astruct_486 *)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0x6c,0x1000);
  uVar1 = str_op_1008_60e8(param_2,param_3);
  iVar3->field_0x6c = uVar1;
  iVar3->field_0x6e = param_3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_6034(param_1: u32,param_2: u16)
{
  let puVar1: *mut u16;
  let iVar2: i16;
  let uVar3: u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar2 = param_1;
  (iVar2 + 0x1e) = 0x1;
  (iVar2 + 0x20) = 0x1;
  (iVar2 + 0x72) = 0x1;
  (iVar2 + 0x74) = 0x1;
  pass1_1010_60a0(param_1);
  puVar1 = pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (iVar2 + 0x22)),
                           (WNDCLASS16 *)0x0,0x40);
  load_string_1010_84ac
            (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10),0x1000);
  (iVar2 + 0x68) = puVar1;
  (iVar2 + 0x6a) = param_2;
  load_string_1010_84ac
            (_PTR_LOOP_1050_14cc,(_PTR_LOOP_1050_14cc >> 0x10),0x1000);
  (iVar2 + 0x6c) = puVar1;
  (iVar2 + 0x6e) = param_2;
  return;
}



fn pass1_1010_60a0(param_1: u32)
{
  (param_1 + 0x76) = 0x5;
  return;
}



fn pass1_1010_60b4(void) -> u16

{
  return 0x1;
}



fn pass1_1010_60ba(void) -> u16

{
  return 0x1;
}



fn pass1_1010_60c0(void) -> u16

{
  return 0x1;
}



fn pass1_1010_60c6(void) -> u16

{
  return 0x1;
}



fn pass1_1010_60cc(param_1: u32,char *param_2,param_3: u16)
{
  let uVar1: u16;
  astruct_487 *iVar3;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar3 = (astruct_487 *)param_1;
  fn_ptr_1000_17ce(*(astruct_18 **)&iVar3->field_0x1a,0x1000);
  uVar1 = str_op_1008_60e8(param_2,param_3);
  iVar3->field_0x1a = uVar1;
  iVar3->field_0x1c = param_3;
  return;
}



fn pass1_1010_60fa(param_1: u32)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x7e) = 0x1;
  (iVar1 + 0x7c) = (iVar1 + 0x20);
  (iVar1 + 0x20) = 0x1;
  return;
}



fn pass1_1010_6118(param_1: u32)
{
  let iVar1: i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((iVar1 + 0x7e) != 0x0) {
    (iVar1 + 0x20) = (iVar1 + 0x7c);
  }
  return;
}



fn pass1_1010_62a4(param_1: *mut u16,param_2: u8)
{
  astruct_488 *uVar2;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  uVar2 = (astruct_488 *)param_1;
  *param_1 = 0x6322;
  uVar2->field_0x2 = 0x1010;
  fn_ptr_1000_17ce((astruct_18 *)uVar2->field_0x4,0x1000);
  *param_1 = 0x389a;
  uVar2->field_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return;
}



fn pass1_1010_62ec(param_1: *mut u16,param_2: u8) -> u16

{
  write_private_profile_str_1010_5b10(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1010_648a(param_1: u32,param_2: u32,param_3: i16,param_4: u16)
{
  let uVar1: u16;
  let iVar2: i16;
  let BVar3: bool;
  let uVar4: u16;
  let uVar5: u16;
  
  uVar4 = param_2;
  uVar5 = (param_2 >> 0x10);
  read_file_1008_7cfe(uVar4,uVar5,0x7,0x1008,param_4);
  if (param_3 != 0x0) {
    iVar2 = param_1;
    uVar1 = (param_1 >> 0x10);
    BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0xa,0x0,uVar1,0x4,0x1008);
    if (BVar3 != 0x0) {
      BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0xe,0x0,uVar1,0x4,0x1008);
      if (BVar3 != 0x0) {
        BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x12,0x0,uVar1,0x4,0x1008);
        if (BVar3 != 0x0) {
          BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x16,0x0,uVar1,0x4,0x1008);
          if (BVar3 != 0x0) {
            BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x1a,0x0,uVar1,0x4,0x1008);
            if (BVar3 != 0x0) {
              BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x1e,0x0,uVar1,0x4,0x1008);
              if (BVar3 != 0x0) {
                BVar3 = read_file_1008_7dee(uVar4,uVar5,iVar2 + 0x22,0x0,uVar1,0x4,0x1008)
                ;
                if (BVar3 != 0x0) {
                  return;
                }
              }
            }
          }
        }
      }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
  }
  return;
}



fn pass1_1010_6566(param_1: u32,param_2: u16,param_3: u16,param_4: u16,param_5: u16)
{
  let uVar1: u16;
  let uVar2: u16;
  let local_4: i16;
  
  uVar1 = param_1;
  uVar2 = (param_1 >> 0x10);
  switch_1010_6646(uVar1,uVar2,CONCAT22(param_5,&local_4),param_4);
  if (local_4 != 0x0) {
    (uVar1 + local_4) = param_3;
    (uVar1 + local_4 + 0x2) = param_2;
  }
  return;
}



i16  pass1_1010_659a(param_1: u32,param_2: u16,param_3: u16)

{
  let uVar1: u16;
  let uVar2: u16;
  let local_4: i16;
  
  uVar1 = param_1;
  uVar2 = (param_1 >> 0x10);
  switch_1010_6646(uVar1,uVar2,CONCAT22(param_3,&local_4),param_2);
  if (local_4 == 0x0) {
    return 0x0;
  }
  return (uVar1 + local_4) - (uVar1 + local_4 + 0x2);
}



fn pass1_1010_65d0(param_1: u16,param_2: u32,param_3: u16) -> u16

{
  let uVar1: u16;
  let local_4: i16;
  
  uVar1 = (param_2 >> 0x10);
  switch_1010_6646(param_2,uVar1,CONCAT22(param_1,&local_4),param_3);
  if (local_4 == 0x0) {
    return 0x0;
  }
  return (param_2 + local_4 + 0x2);
}



fn pass1_1010_6604(param_1: u32,param_2: u16,param_3: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  let uVar3: u16;
  let local_4: i16;
  
  uVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  switch_1010_6646(uVar2,uVar3,CONCAT22(param_3,&local_4),param_2);
  if (local_4 != 0x0) {
    iVar1 = (uVar2 + local_4 + 0x2);
    (uVar2 + local_4) = (uVar2 + local_4);
    (uVar2 + local_4 + 0x2) = iVar1 + 0x1;
    pass1_1010_1f62(param_3,param_1 & 0xffff | uVar3 << 0x10,0x15);
  }
  return;
}


fn pass1_1010_66ca(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1010_6700(astruct_636 *param_1,param_2: u16,param_3: u16) -> u32

{
  struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0x148 = 0x33;
  CONCAT22(param_2,param_1) = 0x6aac;
  param_1->field_0x2 = 0x1010;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,&param_1->field_0xa),(WNDCLASS16 *)0x0,
                  0x114);
  param_1->field_0x32 = 0x1;
  param_1->field_0x40 = 0x1;
  param_1->field_0x46 = 0x1;
  param_1->field_0x4e = 0x1;
  param_1->field_0x54 = 0x1;
  param_1->field_0x5e = 0x1;
  param_1->field_0x68 = 0x1;
  param_1->field_0x6c = 0x1;
  param_1->field_0x74 = 0x1;
  param_1->field_0x78 = 0x1;
  param_1->field_0x7a = 0x1;
  param_1->field_0x7e = 0x1;
  param_1->field_0x82 = 0x1;
  param_1->field_0xa2 = 0x1;
  param_1->field_0xa4 = 0x1;
  param_1->field_0xa6 = 0x1;
  param_1->field_0xa8 = 0x1;
  param_1->field_0xae = 0x1;
  param_1->field_0xb2 = 0x1;
  param_1->field_0xb8 = 0x1;
  param_1->field_0xbe = 0x1;
  param_1->field_0xc0 = 0x1;
  param_1->field_0xc4 = 0x1;
  param_1->field_0xd4 = 0x1;
  param_1->field_0xda = 0x1;
  param_1->field_0xe2 = 0x1;
  param_1->field_0xfe = 0x1;
  param_1->field_0x100 = 0x1;
  param_1->field_0x102 = 0x1;
  param_1->field_0x104 = 0x1;
  param_1->field_0x106 = 0x1;
  param_1->field_0x108 = 0x1;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_2,&param_1->field_0x11e),(WNDCLASS16 *)0x0,
                  0x2a);
  param_1->field_0x120 = 0x1;
  param_1->field_0x122 = 0x1;
  param_1->field_0x124 = 0x1;
  param_1->field_0x126 = 0x1;
  param_1->field_0x128 = 0x1;
  param_1->field_0x12c = 0x1;
  param_1->field_0x138 = 0x1;
  return CONCAT22(param_2,param_1);
}



fn pass1_1010_6814(param_1: u32,param_2: u16,param_3: i16)
{
  (param_1 + param_3 * 0x2 + 0x11e) = param_2;
  return;
}



fn pass1_1010_682e(param_1: u32,param_2: u16,param_3: i16)
{
  (param_1 + param_3 * 0x2 + 0xa) = param_2;
  return;
}


fn pass1_1010_68c6(param_1: u32,param_2: u32,param_3: u16,uchar *param_4,param_5: u16)
{
  astruct_248 *iVar2;
  let BVar1: bool;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let puVar6: *mut u8
  let uVar7: u16;
  let uVar8: u16;
  SEGPTR SVar9;
  let uVar10: u16;
  astruct_18 *paStack18;
  astruct_18 *paStack10;
  let local_6: i16;
  let uStack4: u16;
  
  uVar8 = param_2;
  uVar10 = (param_2 >> 0x10);
  read_file_1008_7cfe(uVar8,uVar10,0x3,0x1008,param_5);
  if (param_3 == 0x0) {
    ctx.PTR_LOOP_1050_0310 = 0x6d4;
    return;
  }
  iVar2 = (astruct_248 *)param_1;
  uVar7 = (param_1 >> 0x10);
  if (ctx.PTR_LOOP_1050_0312 < 0x2) {
    uVar4 = 0x102;
    SVar9 = 0x102;
    mem_op_1000_179c(0x102,param_4,0x1000);
    paStack10 = (astruct_18 *)CONCAT22(param_4,param_3);
    puVar6 = param_4;
    BVar1 = read_file_1008_7dee(uVar8,uVar10,param_3,uVar4,param_4,SVar9,0x1008);
    paStack18 = paStack10;
    if (BVar1 == 0x0) goto LAB_1010_692c;
    uStack4 = 0x1;
    do {
      iVar3 = switch_1008_73ea(uVar8,uVar10,uStack4);
      (&iVar2->field_0xa + iVar3 * 0x2) =
           (uStack4 * 0x2 + param_3);
      uStack4 += 0x1;
    } while (uStack4 < 0x81);
    fn_ptr_1000_17ce(paStack10,0x1000);
    uVar4 = paStack10;
    param_4 = puVar6;
  }
  else {
    uVar4 = read_file_1008_7dee(uVar8,uVar10,&iVar2->field_0xa,0x0,uVar7,0x114,0x1008);
    if (uVar4 == 0x0) {
      ctx.PTR_LOOP_1050_0310 = 0x6d2;
      return;
    }
  }
  if (ctx.PTR_LOOP_1050_0312 < 0x2) {
    uVar5 = 0x2a;
    SVar9 = 0x2a;
    mem_op_1000_179c(0x2a,param_4,0x1000);
    paStack18 = (astruct_18 *)CONCAT22(param_4,uVar4);
    BVar1 = read_file_1008_7dee(uVar8,uVar10,uVar4,uVar5,param_4,SVar9,0x1008);
    if (BVar1 == 0x0) {
//LAB_1010_692c:
      ctx.PTR_LOOP_1050_0310 = 0x6d2;
      fn_ptr_1000_17ce((astruct_18 *)(paStack18 & 0xffff | ZEXT24(param_4) << 0x10)
                       ,0x1000);
      return;
    }
    uStack4 = 0x0;
    do {
      uVar5 = switch_1008_72bc(uVar8,uVar10,uStack4);
      (&iVar2->field_0x11e + uVar5 * 0x2) =
           (uStack4 * 0x2 + uVar4);
      uStack4 += 0x1;
    } while (uStack4 < 0x15);
    fn_ptr_1000_17ce(paStack18,0x1000);
  }
  else {
    BVar1 = read_file_1008_7dee(uVar8,uVar10,&iVar2->field_0x11e,0x0,uVar7,0x2a,0x1008);
    if (BVar1 == 0x0) {
      ctx.PTR_LOOP_1050_0310 = 0x6d2;
      return;
    }
  }
  BVar1 = read_file_1008_7dee(uVar8,uVar10,&local_6,0x0,param_5,0x2,0x1008);
  if (BVar1 == 0x0) {
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
    return;
  }
  BVar1 = switch_1008_73ea(uVar8,uVar10,local_6);
  iVar2->field_0x148 = BVar1;
  return;
}



fn pass1_1010_6a86(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_1d80(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_6abc(astruct_635 *param_1,param_2: u16,param_3: u16)
{
  code **ppcVar1;
  let extraout_DX: *mut u8
  let unaff_DI: i16;
  let unaff_SS: u16;
  astruct_79 *paVar2;
  let puVar3: *mut u16;
  
  paVar2 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  param_1->field_0xa = 0x389a;
  param_1->field_0xc = 0x1008;
  param_1->field_0xa = 0x3aa8;
  param_1->field_0xc = 0x1008;
  param_1->field_0xe = 0x0;
  param_1->field_0x10 = 0x0;
  param_1->field_0x14 = 0x0;
  param_1->field_0x1c = 0x0;
  param_1->field_0x20 = 0x0;
  param_1->field_0x22 = 0x0;
  CONCAT22(param_2,param_1) = 0x7e28;
  param_1->field_0x2 = 0x1010;
  param_1->field_0xa = 0x7e38;
  param_1->field_0xc = 0x1010;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,unaff_SS,
                           (paVar2 >> 0x10),unaff_DI);
  &param_1->field_0x14 = puVar3;
  (&param_1->field_0x14 + 0x2) = (puVar3 >> 0x10);
  ppcVar1 = (code **)(*param_1->field_0x14 + 0x4);
  (**ppcVar1)();
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,unaff_SS,extraout_DX,unaff_DI);
  &param_1->field_0x22 = puVar3;
  (&param_1->field_0x22 + 0x2) = (puVar3 >> 0x10);
  ppcVar1 = (code **)(*param_1->field_0x22 + 0x4);
  (**ppcVar1)();
  return;
}



fn pass1_1010_6bb2(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  code **ppcVar2;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let puStack14: *mut u16;
  
  uVar7 = (param_1 >> 0x10);
  uVar6 = param_1;
  *param_1 = 0x7e28;
  (uVar6 + 0x2) = 0x1010;
  (uVar6 + 0xa) = 0x7e38;
  (uVar6 + 0xc) = 0x1010;
  puVar1 = (uVar6 + 0x1c);
  uVar3 = (uVar6 + 0x1e);
  if ((uVar3 | puVar1) != 0x0) {
    ppcVar2 = (code **)*puVar1;
    (**ppcVar2)();
  }
  (uVar6 + 0x1c) = 0x0;
  if (*(long *)(uVar6 + 0x14) != 0x0) {
    uVar3 = uVar7 | uVar6;
    if (param_1 == 0x0) {
      uVar5 = 0x0;
    }
    else {
      uVar3 = uVar6 + 0xa;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6((uVar6 + 0x14),CONCAT22(uVar5,uVar3),param_2);
  }
  if (*(long *)(uVar6 + 0x22) != 0x0) {
    uVar3 = uVar7 | uVar6;
    if (param_1 == 0x0) {
      uVar5 = 0x0;
    }
    else {
      uVar3 = uVar6 + 0xa;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6((uVar6 + 0x22),CONCAT22(uVar5,uVar3),param_2);
  }
  (uVar6 + 0x14) = 0x0;
  (uVar6 + 0x22) = 0x0;
  if (param_1 == 0x0) {
    iVar4 = 0x0;
    uVar7 = 0x0;
  }
  else {
    iVar4 = uVar6 + 0xa;
  }
  puStack14 = CONCAT22(uVar7,iVar4);
  *puStack14 = 0x389a;
  (iVar4 + 0x2) = 0x1008;
  pass1_1010_1d80(param_1,param_2);
  return;
}



fn pass1_1010_6ca2(param_1: u32,param_2: i16,param_3: u16,param_4: u16) -> u16

{
  let uVar1: u32;
  let puVar2: *mut u16;
  let uVar3: u16;
  let uStack10: i16;
  let iStack8: i16;
  let uStack4: u16;
  
  uStack4 = 0x1;
  _iStack8 = CONCAT22(param_4,&stack0x000a);
  iStack10 = param_2;
  do {
    puVar2 = _iStack8;
    if (iStack10 == 0x0) {
      return uStack4;
    }
    _iStack8 = (_iStack8 & 0xffff0000 | (iStack8 + 0x2));
    uVar3 = *puVar2;
    uVar1 = (param_1 + 0x14);
    iStack10 = iStack10 + -0x1;
    pass1_1010_a5ca(uVar1,(uVar1 >> 0x10),uVar3,uVar3,param_3);
  } while (uVar3 == 0x0);
  return 0x0;
}



u16 pass1_1010_6cf8(param_1: u16,param_2: u32,param_3: i16,param_4: u16,
                      param_5: u16,param_6: u16,param_7: u16)

{
  let uVar1: u16;
  
  if (false) {
code_r0x1010703e:
    return 0x0;
  }
  switch(param_3) {
  case 0x1:
    pass1_1010_715c(param_2,0x1,param_6,param_5,param_7,param_4);
    send_msg_1010_7c9e(param_2,0x12,param_4);
    return 0x1;
  default:
    goto code_r0x1010703e;
  case 0x4:
    uVar1 = 0x2;
    break;
  case 0x5:
    uVar1 = 0x3;
    break;
  case 0x6:
    uVar1 = 0x4;
    break;
  case 0x7:
    uVar1 = 0x5;
    break;
  case 0x9:
    pass1_1010_715c(param_2,0x6,param_6,param_5,param_7,param_4);
  case 0x2e:
    uVar1 = 0x38;
    break;
  case 0xa:
  case 0x80:
    uVar1 = 0x2d;
    break;
  case 0xb:
    uVar1 = 0x7;
    break;
  case 0xc:
  case 0x17:
  case 0x18:
  case 0x19:
  case 0x21:
  case 0x75:
  case 0x81:
    if (param_3 == 0x75) {
      pass1_1010_715c(param_2,0x8,param_6,param_5,param_7,param_4);
      pass1_1010_715c(param_2,0x9,param_6,param_5,param_7,param_4);
    }
    uVar1 = pass1_1010_6ca2(param_2,0x7,param_5,param_4);
    if (uVar1 != 0x0) {
      pass1_1010_715c(param_2,0x10,uVar1,param_5,param_7,param_4);
    }
    param_6 = pass1_1010_6ca2(param_2,0x3,param_5,param_4);
    if (param_6 != 0x0) {
      pass1_1010_715c(param_2,0x11,param_6,param_5,param_7,param_4);
    }
    if (param_3 == 0x21) {
      pass1_1010_715c(param_2,0x14,param_6,param_5,param_7,param_4);
    }
    if (param_3 != 0xc) {
      return 0x1;
    }
    uVar1 = 0x9;
    goto code_r0x10106d4c;
  case 0xe:
    uVar1 = 0xc;
    goto code_r0x10106d4c;
  case 0x10:
  case 0x11:
  case 0x13:
    uVar1 = 0xd;
    break;
  case 0x12:
    uVar1 = 0xe;
    break;
  case 0x1b:
  case 0x1f:
  case 0x5b:
  case 0x78:
  case 0x7e:
  case 0x7f:
    if (param_3 == 0x7e) {
      pass1_1010_715c(param_2,0x2c,param_6,param_5,param_7,param_4);
    }
    if (param_3 == 0x5b) {
      pass1_1010_715c(param_2,0x38,param_6,param_5,param_7,param_4);
    }
    if (param_3 == 0x1f) {
      pass1_1010_715c(param_2,0x3f,param_6,param_5,param_7,param_4);
    }
    if (param_3 == 0x7f) {
      pass1_1010_715c(param_2,0x42,param_6,param_5,param_7,param_4);
    }
    param_6 = pass1_1010_6ca2(param_2,0x5,param_5,param_4);
    if ((param_6 == 0x0) &&
       (param_6 = pass1_1010_6ca2(param_2,0x5,param_5,param_4), param_6 == 0x0)) {
      return 0x1;
    }
    uVar1 = 0x37;
    break;
  case 0x1d:
  case 0x2a:
  case 0x2c:
  case 0x3c:
  case 0x3d:
  case 0x4b:
  case 0x53:
  case 0x54:
  case 0x55:
  case 0x5a:
    uVar1 = pass1_1010_6ca2(param_2,0x2,param_5,param_4);
    if (uVar1 != 0x0) {
      pass1_1010_715c(param_2,0x12,uVar1,param_5,param_7,param_4);
    }
    uVar1 = pass1_1010_6ca2(param_2,0x8,param_5,param_4);
    if (uVar1 != 0x0) {
      pass1_1010_715c(param_2,0x1a,uVar1,param_5,param_7,param_4);
    }
    if (param_3 == 0x2c) {
      pass1_1010_715c(param_2,0x1d,uVar1,param_5,param_7,param_4);
    }
    param_6 = pass1_1010_6ca2(param_2,0x2,param_5,param_4);
    if (param_6 == 0x0) {
      return 0x1;
    }
    uVar1 = 0x1c;
    break;
  case 0x22:
    uVar1 = 0x15;
    break;
  case 0x25:
    uVar1 = 0x16;
    break;
  case 0x26:
    pass1_1010_715c(param_2,0x17,param_6,param_5,param_7,param_4);
  case 0x1e:
    uVar1 = 0x13;
    break;
  case 0x27:
    uVar1 = 0x18;
    break;
  case 0x29:
    uVar1 = 0x19;
    break;
  case 0x2b:
    uVar1 = 0x1b;
    break;
  case 0x2f:
  case 0x36:
    param_6 = pass1_1010_6ca2(param_2,0x2,param_5,param_4);
    if (param_6 == 0x0) {
      return 0x0;
    }
    uVar1 = 0x1e;
    break;
  case 0x30:
    uVar1 = 0x1f;
    break;
  case 0x31:
    uVar1 = 0x35;
    break;
  case 0x33:
    uVar1 = 0x21;
    break;
  case 0x34:
    uVar1 = 0x22;
    break;
  case 0x35:
    pass1_1010_715c(param_2,0x23,param_6,param_5,param_7,param_4);
  case 0x65:
  case 0x66:
  case 0x6b:
  case 0x6c:
  case 0x6d:
  case 0x6e:
  case 0x6f:
    uVar1 = 0x34;
    break;
  case 0x38:
    pass1_1010_715c(param_2,0x24,param_6,param_5,param_7,param_4);
    uVar1 = 0x3d;
    break;
  case 0x39:
    uVar1 = 0x25;
    break;
  case 0x3e:
    pass1_1010_715c(param_2,0x26,param_6,param_5,param_7,param_4);
    pass1_1010_715c(param_2,0x28,param_6,param_5,param_7,param_4);
    uVar1 = 0x27;
    break;
  case 0x40:
    uVar1 = 0x2a;
    break;
  case 0x41:
    uVar1 = 0x39;
    break;
  case 0x42:
    uVar1 = 0x3a;
    break;
  case 0x44:
    uVar1 = 0x36;
    break;
  case 0x45:
    uVar1 = 0x3b;
    break;
  case 0x49:
    uVar1 = 0x29;
    break;
  case 0x50:
    uVar1 = 0x2b;
    break;
  case 0x56:
    pass1_1010_715c(param_2,0x3c,param_6,param_5,param_7,param_4);
    uVar1 = 0x3e;
    break;
  case 0x5d:
    pass1_1010_715c(param_2,0x2f,param_6,param_5,param_7,param_4);
    uVar1 = 0x40;
    break;
  case 0x5e:
  case 0x60:
    uVar1 = 0x2f;
    break;
  case 0x5f:
    pass1_1010_715c(param_2,0x34,param_6,param_5,param_7,param_4);
    uVar1 = 0x41;
    break;
  case 0x61:
    uVar1 = 0x30;
    break;
  case 0x63:
    uVar1 = 0x31;
    break;
  case 0x64:
    uVar1 = 0x24;
    break;
  case 0x68:
    uVar1 = 0x32;
    break;
  case 0x69:
    uVar1 = 0x33;
    break;
  case 0x76:
    uVar1 = 0xa;
code_r0x10106d4c:
    pass1_1010_715c(param_2,uVar1,param_6,param_5,param_7,param_4);
    uVar1 = 0xb;
  }
  pass1_1010_715c(param_2,uVar1,param_6,param_5,param_7,param_4);
  return 0x1;
}



void 
pass1_1010_715c(param_1: u32,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
               param_6: u16)

{
  pass1_1010_a69c((param_1 + 0x14),param_2,param_3,param_4,param_5,param_6)
  ;
  return;
}



fn pass1_1010_71b0(param_1: i16,param_2: u16)
{
  let uVar1: u32;
  
  uVar1 = (param_1 + 0x6);
  send_msg_1010_7c42(uVar1 & 0xffff0000 | (uVar1 - 0xa),param_2);
  return;
}



fn pass1_1010_71c2(param_1: u16,param_2: u16,param_3: i16,param_4: u16)
{
  let uVar1: u32;
  let uVar2: u32;
  let iVar3: i16;
  let uVar4: u16;
  
  if (param_1 == 0x13) {
    uVar2 = (param_3 + 0x6);
    uVar2 = (uVar2 + 0x18);
    uVar1 = (param_3 + 0x6);
    destroy_window_1010_7b26
              (uVar1 & 0xffff0000 | (uVar1 - 0xa),*(long *)(uVar2 + 0x28)
               ,param_4,param_2);
    return;
  }
  if (param_1 < 0x14) {
    if (param_1 == '\x01') {
      uVar2 = (param_3 + 0x6);
      uVar4 = (uVar2 >> 0x10);
      iVar3 = uVar2;
      (iVar3 + 0xa) = 0x0;
      (iVar3 + 0x18) = 0x0;
      return;
    }
    if (param_1 == '\x05') {
      uVar1 = (param_3 + 0x6);
      send_msg_1010_7c42(uVar1 & 0xffff0000 | (uVar1 - 0xa),param_4);
      return;
    }
  }
  return;
}



void 
pass1_1010_71d6(param_1: u32,param_2: i16,param_3: *mut u16,param_4: u16,param_5: u16,
               param_6: u16)

{
  let iVar1: i16;
  let uVar2: u32;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u32;
  let uStack20: u16;
  let uStack14: u16;
  let uStack6: u32;
  
  uVar8 = (param_1 >> 0x10);
  uVar2 = (param_1 + 0x14);
  pass1_1010_ad22(uVar2,param_5,param_6,(uVar2 >> 0x10),*param_3);
  uStack6 = CONCAT22(param_5,param_4);
  if ((param_5 | param_4) == 0x0) {
    return;
  }
  uVar9 = struct_op_1030_73a8(CONCAT22(param_5,param_4));
  uVar7 = (uVar9 >> 0x10);
  uVar3 = uVar9;
  if (((uVar7 | uVar3) != 0x0) && (*(long *)(uVar3 + 0x1c) == 0x8000002)) {
    return;
  }
  uVar2 = (param_4 + 0x2e);
  uStack14 = uVar2;
  if ((((param_4 + 0x30) | uStack14) != 0x0) &&
     (*(long *)(uStack14 + 0x200) == 0x8000002)) {
    return;
  }
  uVar2 = (param_1 + 0x14);
  uVar5 = pass1_1010_b028(uVar2,(uVar2 >> 0x10),uVar9);
  iVar1 = (uVar3 + 0x12);
  iVar4 = iVar1;
  if ((iVar1 != 0x4) && (iVar4 = param_2, iVar1 == 0x7)) {
    param_2 = 0x5;
    iVar4 = param_2;
  }
  param_2 = iVar4;
  uVar6 = param_2 - 0x2;
  if (uVar6 != 0x0) {
    if (param_2 == 0x3) {
      uVar6 = uVar5 - 0xb;
      if ((uVar6 == 0x0) || (uVar6 = uVar5 - 0x37, uVar6 == 0x0)) {
        uStack20 = 0xb;
      }
      else {
        uStack20 = 0xa;
      }
      goto LAB_1010_72a7;
    }
    uVar6 = param_2 - 0x4;
    if (uVar6 == 0x0) {
      uStack20 = 0x17;
      goto LAB_1010_72a7;
    }
    uVar6 = param_2 - 0x5;
    if (uVar6 != 0x0) {
      uVar6 = pass1_1010_7818(param_1,uVar9);
      uStack20 = uVar6;
      goto LAB_1010_72a7;
    }
  }
  uStack20 = 0xc;
//LAB_1010_72a7:
  if (uStack20 == 0x0) {
    return;
  }
  ui_op_1010_79aa(param_1,0x0,uStack6,param_6);
  if (uVar6 == 0x0) {
    unk_win_op_1010_7300(param_1,0x0,uStack20,uStack6);
  }
  return;
}


/*
Unable to decompile 'unk_win_op_1010_7300'
Cause: Exception while decompiling 1010:7300: The pipe is being closed

*/


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_7818(param_1: u32,param_2: u32) -> u16

{
  let uVar1: u32;
  let uVar2: u16;
  let BVar3: bool;
  let uVar4: u16;
  let uStack6: u16;
  
  uVar4 = (param_1 >> 0x10);
  uVar1 = (param_1 + 0x14);
  uVar2 = pass1_1010_b028(uVar1,(uVar1 >> 0x10),param_2);
  BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x1e);
  if (BVar3 == 0x0) {
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0xb);
    if (((BVar3 == 0x0) &&
        (BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x20), BVar3 == 0x0)) &&
       (BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x1c), BVar3 == 0x0)) {
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x2);
      if ((BVar3 != 0x0) ||
         (BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x13), BVar3 != 0x0)) {
        return 0x5;
      }
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x11);
      if ((BVar3 != 0x0) ||
         (BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x12), BVar3 != 0x0)) {
        return 0x4;
      }
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x5);
      if (BVar3 != 0x0) {
        return 0x6;
      }
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x6);
      if (BVar3 != 0x0) {
        return 0x7;
      }
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x4);
      if (BVar3 != 0x0) {
        return 0x10;
      }
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x3);
      if (BVar3 != 0x0) {
        return 0x11;
      }
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x19);
      if (BVar3 != 0x0) {
        return 0x15;
      }
      BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar2,0x1d);
      if (BVar3 != 0x0) {
        return 0x16;
      }
      uVar2 = pass1_1010_7d7e(param_1,uVar4,0x1,uVar2);
      if (uVar2 == 0x0) {
        return 0x0;
      }
      return 0xc;
    }
    uStack6 = 0x1;
  }
  else {
    uStack6 = 0x18;
  }
  return uStack6;
}


fn pass1_1010_7b8c(param_1: u32,param_2: i16,param_3: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let uVar4: u32;
  let puVar5: *mut u8;
  let extraout_DX: u16;
  let iVar6: i16;
  let uVar7: u16;
  let uStack14: u32;
  let local_a: [u8;8];
  
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  if (((iVar6 + 0x1e) | (iVar6 + 0x1c)) != 0x0) {
    pass1_1008_5784(CONCAT22(param_3,local_a),(iVar6 + 0x1c));
    do {
      puVar5 = local_a;
      pass1_1008_5b12(puVar5,param_3);
      uStack14 = CONCAT22(extraout_DX,puVar5);
      if ((extraout_DX | puVar5) == 0x0) break;
      uVar4 = (puVar5 + 0x8);
    } while ((uVar4 + 0x6) != param_2);
    if ((extraout_DX | puVar5) != 0x0) {
      ppcVar3 = (code **)((iVar6 + 0x1c) + 0xc);
      (**ppcVar3)(0x1008,(iVar6 + 0x1c),uStack14);
    }
    uVar4 = (iVar6 + 0x1c);
    if ((uVar4 + 0x8) == 0x0) {
      puVar1 = (iVar6 + 0x1c);
      uVar2 = (iVar6 + 0x1e);
      if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = (code **)*puVar1;
        (**ppcVar3)(0x1008,puVar1,uVar2,0x1,puVar1,uVar2,puVar1,uVar2);
      }
      (iVar6 + 0x1c) = 0x0;
    }
  }
  return;
}


fn pass1_1010_7d38(param_1: u16,param_2: u16,param_3: i16,param_4: u16,param_5: u16) -> u16

{
  let local_e: u32;
  let uStack10: u16;
  let local_8: u16;
  let local_6: [u8;2];
  let local_4: [u8;2];
  
  local_e = (param_3 + 0xc);
  uStack10 = (param_3 + 0x10);
  pass1_1008_3eb4((u16 *)CONCAT22(param_5,&local_e),
                  CONCAT22(param_5,&local_8),CONCAT22(param_5,local_6)
                  ,CONCAT22(param_5,local_4));
  return local_8;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_7d7e(param_1: u16,param_2: u16,param_3: i16,param_4: i16) -> u16

{
  let BVar1: bool;
  
  if (param_3 != 0x3) {
    if ((param_4 < 0xa) || (0x7f < param_4)) {
      return 0x0;
    }
    BVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,param_4,0x3c);
    if (BVar1 != 0x0) {
      return 0x0;
    }
    if (((param_4 == 0x6a) && (param_3 != 0x4)) && (param_3 != 0x5)) {
      return 0x0;
    }
  }
  return 0x1;
}



fn pass1_1010_7dc6(param_1: u32,param_2: u8) -> u32

{
  let unaff_SS: u16;
  
  param_1 = param_1 & 0xffff0000 | (param_1 - 0xa);
  pass1_1010_6bb2((u16 *)param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1010_7dd2(param_1: *mut u16,param_2: u8) -> u16

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_7e40(param_1: *mut u32,uchar *param_2,param_3: i16,param_4: u16)
{
  let uVar1: u32;
  astruct_652 *puVar2;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  uVar2 = (param_1 >> 0x10);
  puVar2 = (astruct_652 *)param_1;
  *param_1 = 0x0;
  puVar2->field_0x67c = 0x0;
  puVar2->field_0x680 = 0x0;
  puVar2->field_0xe82 = 0x0;
  puVar2->field_0xe84 = 0x0;
  &puVar2->field_0xe88 = 0x0;
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &puVar2->field_0x4),
                  (WNDCLASS16 *)0x0,0x228);
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &puVar2->field_0x22c),
                  (WNDCLASS16 *)0x0,0x228);
  pass1_1000_4906((astruct_20 *)
                  (param_1 & 0xffff0000 | &puVar2->field_0x454),
                  (WNDCLASS16 *)0x0,0x228);
  *&puVar2->field_0x682 = 0x0;
  *&puVar2->field_0xa82 = 0x0;
  _PTR_LOOP_1050_14cc = param_1;
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,param_2,param_3);
  puVar2->field_0xe88 = puVar3;
  puVar2->field_0xe8a = (puVar3 >> 0x10);
  uVar1 = &puVar2->field_0xe88;
  puVar2->field_0xe84 = (uVar1 + 0x64);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_7efc(param_1: *mut u32,param_2: u16)
{
  let uVar1: u16;
  let uVar2: u16;
  let puVar3: u32;
  code **ppcVar4;
  astruct_448 *iVar5;
  let uVar5: u16;
  astruct_18 *paStack8;
  let iStack4: i16;
  
  uVar5 = (param_1 >> 0x10);
  iVar5 = (astruct_448 *)param_1;
  uVar1 = iVar5->field_0x67c;
  uVar2 = iVar5->field_0x67e;
  paStack8 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1008_64a2((u16 *)CONCAT22(uVar2,uVar1));
    param_2 = 0x1000;
    fn_ptr_1000_17ce(paStack8,0x1000);
  }
  for (iStack4 = 0x0; iStack4 < 0x8a; iStack4 += 0x1) {
    puVar3 = (&iVar5->field_0x4 + iStack4 * 0x4);
    uVar1 = (&iVar5->field_0x4 + iStack4 * 0x4 + 0x2);
    if ((uVar1 | puVar3) != 0x0) {
      ppcVar4 = (code **)*puVar3;
      (**ppcVar4)(param_2,puVar3,uVar1,0x1);
    }
    puVar3 = (&iVar5->field_0x22c + iStack4 * 0x4);
    uVar1 = (&iVar5->field_0x22c + iStack4 * 0x4 + 0x2);
    if ((uVar1 | puVar3) != 0x0) {
      ppcVar4 = (code **)*puVar3;
      (**ppcVar4)(param_2,puVar3);
    }
    puVar3 = (&iVar5->field_0x454 + iStack4 * 0x4);
    uVar1 = (&iVar5->field_0x454 + iStack4 * 0x4 + 0x2);
    if ((uVar1 | puVar3) != 0x0) {
      ppcVar4 = (code **)*puVar3;
      (**ppcVar4)(param_2,puVar3);
    }
  }
  fn_ptr_1000_17ce((astruct_18 *)*param_1,0x1000);
  _PTR_LOOP_1050_14cc = 0x0;
  return;
}



fn pass1_1010_7fd6(param_1: u32)
{
  let uVar1: u16;
  let uVar2: u16;
  astruct_489 *iVar3;
  let uVar3: u16;
  astruct_18 *paStack6;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_489 *)param_1;
  uVar1 = iVar3->field_0x67c;
  uVar2 = iVar3->field_0x67e;
  paStack6 = (astruct_18 *)CONCAT22(uVar2,uVar1);
  if ((uVar2 | uVar1) != 0x0) {
    pass1_1008_64a2((u16 *)CONCAT22(uVar2,uVar1));
    fn_ptr_1000_17ce(paStack6,0x1000);
  }
  &iVar3->field_0x67c = 0x0;
  iVar3->field_0x680 = 0x0;
  return;
}



fn pass1_1010_8018(param_1: u32,param_2: u16,uchar *param_3,param_4: u16)
{
  let iVar1: i16;
  let uVar2: *mut u16;
  
  if ((param_2 * 0xa + 0x1fa0) != 0x0) {
    pass1_1010_878c((astruct_87 **)param_1,(param_2 * 0xa + 0x1fa0),param_4);
    uVar2 = (param_1 >> 0x10);
    if (*(long *)(param_1 + 0x67c) != 0x0) {
      iVar1 = param_2 * 0xa;
      pass1_1008_64c8(*(u32 **)(param_1 + 0x67c),
                      CONCAT22((iVar1 + 0x1fa6),
                               (iVar1 + 0x1fa8)),(iVar1 + 0x1fa4),
                      param_2,param_3);
      return;
    }
  }
  return;
}



fn pass1_1010_8096(param_1: *mut u32,param_2: i16)
{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let unaff_SS: u16;
  char *pcVar5;
  let puVar6: *mut u16;
  let local_306: [u8;100];
  let local_206: [u8;100];
  let local_106: [u8;104];
  
  uVar4 = (param_1 >> 0x10);
  uVar3 = param_1;
  str_1000_4d58(((uVar3 + 0xe82) * 0x4 + 0x2526),0x0,0x0,
                CONCAT22(unaff_SS,local_206),(WNDCLASS16 *)CONCAT22(unaff_SS,local_306));
  unk_str_op_1000_3d3e
            (CONCAT22(unaff_SS,local_106),CONCAT22(unaff_SS,local_206));
  if (param_2 == 0x2) {
    puVar6 = &USHORT_1050_3194;
  }
  else {
    puVar6 = &USHORT_1050_3196;
  }
  pass1_1000_3cea(CONCAT22(unaff_SS,local_106),(ULONG)puVar6);
  pass1_1000_3cea(CONCAT22(unaff_SS,local_106),CONCAT22(unaff_SS,local_306));
  pcVar5 = set_err_mode_1010_8b14
                             (param_1,CONCAT22(unaff_SS,local_106),unaff_SS);
  uVar2 = (pcVar5 >> 0x10);
  if ((pcVar5 == local_106) && (uVar2 == unaff_SS)) {
    msg_box_op_1010_8bb4
              (uVar3,uVar4,pcVar5 & 0xffff | uVar2 << 0x10,0x1000,unaff_SS);
  }
  fn_ptr_1000_17ce((astruct_18 *)*param_1,0x1000);
  uVar1 = str_op_1008_60e8(pcVar5,uVar2);
  param_1 = uVar1;
  (uVar3 + 0x2) = uVar2;
  return;
}



fn pass1_1010_8170(astruct_87 **param_1,param_2: i16,uchar *param_3,param_4: u16)
{
  let uVar1: u16;
  let iVar2: i16;
  let iVar3: i16;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar2 = param_1;
  uVar1 = (iVar2 + 0x680);
  iVar3 = param_2 * 0x10;
  if ((iVar3 + 0x16) != uVar1) {
    pass1_1010_8096(param_1,(iVar3 + 0x16));
    pass1_1010_878c(param_1,(iVar3 + 0x16),param_4);
    if (*(long *)(iVar2 + 0x67c) == 0x0) {
      return;
    }
  }
  iVar3 = param_2 * 0x10;
  pass1_1008_6562(*(u32 **)(iVar2 + 0x67c),
                  CONCAT22((iVar3 + 0x1c),(iVar3 + 0x1e)),
                  (iVar3 + 0x1a),uVar1,param_3);
  return;
}



void 
pass1_1010_81f6(HINSTANCE16 param_1,param_2: u16,astruct_87 **param_3,param_4: i32,
               param_5: i16)

{
  let uVar1: u16;
  let uVar2: u16;
  let uStack12: u16;
  let uStack10: u32;
  
  if (param_4 == 0x8000001) {
    uStack10 = param_3 & 0xffff0000 | (param_3 + 0x22c);
    uStack12 = 0xfa;
  }
  else {
    if (param_4 == 0x8000002) {
      uStack10 = param_3 & 0xffff0000 | (param_3 + 0x454);
      uStack12 = 0xfc;
    }
    else {
      uStack10 = param_3 & 0xffff0000 | (param_3 + 0x4);
      uStack12 = 0x2;
    }
  }
  uVar2 = (uStack10 >> 0x10);
  uVar1 = param_3._2_2_;
  if (*(long *)(param_5 * 0x4 + uStack10) == 0x0) {
    if ((0x0 < param_5) && (param_5 < 0xa)) {
      pass1_1010_89f0(param_3,param_3._2_2_,uStack12,uStack10,param_1,param_2);
      return;
    }
    if (param_5 < 0xa) {
      return;
    }
    if (0x7f < param_5) {
      return;
    }
    if (*(long *)(uStack10 + 0x14) == 0x0) {
      pass1_1010_89f0(param_3,param_3._2_2_,uStack12,uStack10,param_1,param_2);
    }
    pass1_1010_887a(param_3,uStack10,param_5,param_1,param_2);
  }
  pass1_1010_866c(uVar1,param_3,param_3._2_2_,uStack10,param_5);
  return;
}



fn pass1_1010_82f8(param_1: u32,param_2: u16)
{
  (param_1 + 0xe82) = param_2;
  return;
}


fn pass1_1010_84f8(param_1: u32,param_2: i16,param_3: u16)
{
  let uVar1: u32;
  let uStack780: u16;
  char local_308 [0x100];
  let local_208: [u8;100];
  let local_108: [u8;104];
  let iStack4: i16;
  
  if ((param_2 * 0x10 + 0x10) != 0x3) {
    return;
  }
  uVar1 = (param_1 + 0xe88);
  iStack4 = (uVar1 + 0x70);
  str_1000_4d58((param_2 * 0x10 + 0x12),0x0,0x0,
                CONCAT22(param_3,local_208),(WNDCLASS16 *)CONCAT22(param_3,local_308));
  unk_str_op_1000_3d3e
            (CONCAT22(param_3,local_108),CONCAT22(param_3,local_208));
  if (local_308[0] == '\0') {
    if (iStack4 == 0x0) {
      uStack780 = 0x14c0;
    }
    else {
      uStack780 = 0x14ba;
    }
    _uStack780 = CONCAT22(0x1050,uStack780);
  }
  else {
    _uStack780 = CONCAT22(param_3,local_308);
  }
  pass1_1000_3cea(CONCAT22(param_3,local_108),_uStack780);
  set_err_mode_1010_8b14(param_1,CONCAT22(param_3,local_108),param_3);
  return;
}



fn pass1_1010_85be(param_1: u32,param_2: i16,param_3: i16,param_4: u16)
{
  let uVar1: u32;
  let local_30a: [u8;100];
  let local_20a: [u8;100];
  let local_10a: [u8;108];
  
  if (param_2 == 0x2) {
    uVar1 = (param_3 * 0x4 + 0x2e34);
    str_1000_4d58((uVar1 & 0xffff0000 | (uVar1 + 0x3)),0x0,0x0
                  ,CONCAT22(param_4,local_20a),(WNDCLASS16 *)CONCAT22(param_4,local_30a));
    unk_str_op_1000_3d3e(CONCAT22(param_4,local_10a),s_male_1050_14c6);
    pass1_1000_3cea(CONCAT22(param_4,local_10a),CONCAT22(param_4,local_20a));
    pass1_1000_3cea(CONCAT22(param_4,local_10a),CONCAT22(param_4,local_30a));
    set_err_mode_1010_8b14(param_1,CONCAT22(param_4,local_10a),param_4);
    return;
  }
  set_err_mode_1010_8b14(param_1,*(ULONG *)(param_3 * 0x4 + 0x2e34),param_4);
  return;
}



fn pass1_1010_866c(param_1: u16,param_2: u16,param_3: u16,param_4: u32,param_5: u16)
{
  let uVar1: u32;
  let cVar2: u8;
  let iVar3: i16;
  let bVar4: bool;
  
  if (param_5 < 0x28) {
    if ((param_5 < 0x25) && (param_5 != 0x23)) {
      if (0x23 < param_5) {
        return;
      }
      cVar2 = param_5;
      if (((cVar2 != '\v') && (cVar2 != '\x0f')) && (cVar2 != '!')) {
        return;
      }
    }
  }
  else {
    if (param_5 != 0x37) {
      if (param_5 < 0x38) {
        if (param_5 < 0x33) {
          return;
        }
        bVar4 = SBORROW2(param_5 - 0x33,0x1);
        iVar3 = param_5 - 0x34;
      }
      else {
        if (param_5 == 0x49) goto LAB_1010_8691;
        if ((param_5 - 0x49) < 0x2a) {
          return;
        }
        bVar4 = SBORROW2(param_5 - 0x73,0x5);
        iVar3 = param_5 - 0x78;
      }
      if (iVar3 != 0x0 && bVar4 == iVar3 < 0x0) {
        return;
      }
    }
  }
//LAB_1010_8691:
  uVar1 = (param_5 * 0x4 + param_4);
  memcpy_op_1008_676e(uVar1,uVar1,param_1);
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1010_86de(param_1: u16,param_2: u16,param_3: u8,param_4: u32)
{
  long *plVar1;
  let iVar2: i16;
  let bVar3: bool;
  let uVar4: u16;
  let lVar5: i32;
  let uVar6: u32;
  let lStack20: i32;
  let uStack10: u32;
  
  uVar6 = pass1_1008_4772((astruct_76 *)param_4);
  uVar4 = (uVar6 >> 0x10);
  uStack10 = 0x0;
  do {
    plVar1 = (long *)(uVar6 + 0x8);
    if (*plVar1 == uStack10 || *plVar1 < uStack10) {
      return;
    }
    lVar5 = uStack10;
    pass1_1008_4544(param_4);
    iVar2 = lVar5;
    bVar3 = false;
    for (lStack20 = 0x0; plVar1 = (long *)(uVar6 + 0x4),
        *plVar1 != lStack20 && lStack20 <= *plVar1; lStack20 += 0x1) {
      if (bVar3) {
//LAB_1010_86fc:
        if (bVar3) {
          if (*(lStack20 + iVar2) == -0x1) {
            *(lStack20 + iVar2) = param_3;
            break;
          }
        }
      }
      else {
        if (*(lStack20 + iVar2) == -0x1) goto LAB_1010_86fc;
        *(lStack20 + iVar2 + -0x1) = param_3;
        bVar3 = true;
      }
    }
    uStack10 += 0x1;
  } while( true );
}



fn pass1_1010_878c(astruct_87 **param_1,param_2: i16,HINSTANCE16 param_3)
{
  let uVar1: u16;
  let uVar2: u16;
  let uVar4: u16;
  let puVar3: *mut u8
  let puVar4: *mut u8
  astruct_87 *uVar6;
  let iVar5: i16;
  let uVar7: u16;
  let unaff_SS: u16;
  astruct_87 *paVar8;
  astruct_87 *paVar9;
  
  uVar7 = (param_1 >> 0x10);
  uVar6 = (astruct_87 *)param_1;
  if (uVar6->field_0x680 == param_2) {
    return;
  }
  uVar1 = uVar6->field_0x67c;
  puVar4 = uVar6->field_0x67e;
  puVar3 = (puVar4 | uVar1);
  uVar2 = uVar1;
  if (puVar3 != 0x0) {
    pass1_1008_64a2((u16 *)CONCAT22(puVar4,uVar1));
    param_3 = 0x1000;
    fn_ptr_1000_17ce((astruct_18 *)CONCAT22(puVar4,uVar1),0x1000);
  }
  if ((param_2 == 0x1) || (param_2 == 0x2)) {
    mem_op_1000_179c(0x8,puVar3,0x1000);
    puVar4 = (puVar3 | uVar2);
    if (puVar4 == 0x0) {
      &uVar6->field_0x67c = 0x0;
      goto LAB_1010_8869;
    }
    paVar8 = *param_1;
    paVar9 = (astruct_87 *)CONCAT22(puVar3,uVar2);
//LAB_1010_8853:
    uVar4 = paVar9;
    file_1008_6414(paVar9,paVar8,unaff_SS,puVar4);
  }
  else {
    iVar5 = param_2 * 0x4;
    paVar8 = (astruct_87 *)
             set_err_mode_1010_8b14(param_1,*(ULONG *)(iVar5 + 0x172a),unaff_SS);
    paVar9 = paVar8;
    if (((iVar5 + 0x172a) == paVar8) &&
       ((iVar5 + 0x172c) == (paVar8 >> 0x10))) {
      msg_box_op_1010_8bb4(uVar6,uVar7,paVar8,param_3,unaff_SS);
    }
    mem_op_1000_179c(0x8,(paVar9 >> 0x10),0x1000);
    puVar4 = ((paVar9 >> 0x10) | paVar9);
    if (paVar9 != (astruct_87 *)0x0) goto LAB_1010_8853;
    uVar4 = 0x0;
    puVar4 = 0x0;
  }
  uVar6->field_0x67c = uVar4;
  uVar6->field_0x67e = puVar4;
//LAB_1010_8869:
  uVar6->field_0x680 = param_2;
  return;
}



// WARNING: Could not reconcile some variable overlaps

void 
pass1_1010_887a(astruct_87 **param_1,param_2: u32,param_3: i16,param_4: u16,
               param_5: u16)

{
  let uVar1: u16;
  let uVar2: u32;
  let uVar3: u32;
  let in_DX: *mut u8
  let puVar4: *mut u8
  let extraout_DX: u16;
  let iVar5: i16;
  let uVar6: u16;
  let uVar7: u16;
  let bVar8: u8;
  let local_26: [u8;6];
  let uStack32: u16;
  let uStack30: u16;
  let uStack28: u32;
  let uStack24: u32;
  let uStack20: u32;
  let uStack16: u32;
  astruct_76 *paStack12;
  astruct_76 *paStack8;
  let uStack4: u16;
  
  uStack4 = param_3 - 0xa;
  pass1_1010_878c(param_1,(uStack4 * 0xa + 0x3382),param_4);
  uVar6 = (param_1 >> 0x10);
  if (*(long *)(param_1 + 0x67c) != 0x0) {
    iVar5 = uStack4 * 0xa;
    uVar1 = uStack4;
    pass1_1008_6562(*(u32 **)(param_1 + 0x67c),
                    CONCAT22((iVar5 + 0x3388),
                             (iVar5 + 0x338a)),(iVar5 + 0x3386),
                    uStack4,in_DX);
    paStack8 = (astruct_76 *)CONCAT22(in_DX,uVar1);
    uVar6 = (param_2 >> 0x10);
    paStack12 = *(astruct_76 **)(param_2 + 0x14);
    uStack16 = pass1_1008_4772(paStack12);
    uStack20 = pass1_1008_4772(paStack8);
    puVar4 = (uStack20 >> 0x10);
    uVar2 = (uStack20 + 0x4);
    uVar7 = (uStack16 >> 0x10);
    iVar5 = uStack16;
    if ((long)uVar2 < *(long *)(iVar5 + 0x4)) {
      uVar2 = (iVar5 + 0x4);
    }
    uVar3 = (uStack20 + 0x8);
    if ((long)uVar3 < *(long *)(iVar5 + 0x8)) {
      uVar3 = (iVar5 + 0x8);
    }
    uVar1 = uVar3;
    uStack24 = uVar3 & 0xffff | uVar2 << 0x10;
    bVar8 = 0x1e;
    mem_op_1000_179c(0x1e,puVar4,0x1000);
    if ((puVar4 | uVar1) == 0x0) {
      uVar1 = 0x0;
      uVar7 = 0x0;
    }
    else {
      struct_op_1008_6604((astruct_85 *)CONCAT22(puVar4,uVar1),uStack24,
                          (uStack24 >> 0x10));
      uVar7 = extraout_DX;
    }
    uStack28 = CONCAT22(uVar7,uVar1);
    pass1_1008_431c(CONCAT22(uVar7,uVar1),bVar8);
    uVar7 = (uStack16 >> 0x10);
    uStack30 = (uStack24._2_2_ - (uStack16 + 0x4)) / 0x2;
    uStack32 = uStack24 - (uStack16 + 0x8);
    pass1_1008_3e54((u16 *)CONCAT22(param_5,local_26),0x0,uStack32,uStack30);
    pass1_1008_4480(uStack28,CONCAT22(param_5,local_26),paStack12,param_5);
    pass1_1008_3e76((u16 *)CONCAT22(param_5,local_26),0x0,0x0,0x7);
    pass1_1008_4480(uStack28,CONCAT22(param_5,local_26),paStack8,param_5);
    (param_3 * 0x4 + param_2) = uStack28;
  }
  return;
}



void 
pass1_1010_89f0(param_1: u16,param_2: u16,param_3: u16,param_4: u32,
               HINSTANCE16 param_5,param_6: u16)

{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let puVar4: *mut u8
  let puVar5: *mut u8
  let iVar6: i16;
  let uVar7: u32;
  let uStack22: u32;
  let uStack8: u16;
  
  uVar3 = (param_1 + 0x67c);
  uVar1 = (param_1 + 0x67e);
  if ((uVar1 | uVar3) != 0x0) {
    pass1_1008_64a2((u16 *)CONCAT22(uVar1,uVar3));
    param_5 = 0x1000;
    fn_ptr_1000_17ce((astruct_18 *)CONCAT22(uVar1,uVar3),0x1000);
  }
  uVar7 = set_err_mode_1010_8b14
                    (CONCAT22(param_2,param_1),
                     *(ULONG *)((param_1 + 0xe82) * 0x4 + 0x24be),param_6);
  puVar4 = (uVar7 >> 0x10);
  uVar3 = uVar7;
  iVar6 = (param_1 + 0xe82) * 0x4;
  if (((iVar6 + 0x24be) == uVar3) && (*(uchar **)(iVar6 + 0x24c0) == puVar4)) {
    msg_box_op_1010_8bb4(param_1,param_2,uVar7,param_5,param_6);
  }
  mem_op_1000_179c(0x8,puVar4,0x1000);
  puVar5 = (puVar4 | uVar3);
  if (puVar5 == 0x0) {
    uVar3 = 0x0;
    puVar5 = 0x0;
  }
  else {
    file_1008_6414(CONCAT13((puVar4 >> 0x8),
                                     CONCAT12(puVar4,uVar3)),uVar7,param_6,puVar5);
  }
  (param_1 + 0x67c) = uVar3;
  *(uchar **)(param_1 + 0x67e) = puVar5;
  (param_1 + 0x680) = 0x0;
  if (((param_1 + 0x67e) | (param_1 + 0x67c)) != 0x0) {
    for (uStack8 = 0x1; uStack8 < 0xa; uStack8 += 0x1) {
      iVar6 = uStack8 * 0xa;
      uVar2 = (iVar6 + 0x2558);
      uVar3 = uStack8;
      pass1_1008_64c8(*(u32 **)(param_1 + 0x67c),
                      CONCAT13((uVar2 >> 0x8),
                               CONCAT12(uVar2,(iVar6 + 0x255a))),
                      (iVar6 + 0x2556),uStack8,puVar5);
      uStack22 = CONCAT22(puVar5,uVar3);
      pass1_1010_86de(param_1,param_2,(uchar)param_3,CONCAT22(puVar5,uVar3));
      (uStack8 * 0x4 + param_4) = uStack22;
    }
  }
  return;
}


fn pass1_1010_8c32(astruct_640 *param_1,param_2: u16,param_3: u16,param_4: u16) -> u32

{
  let unaff_DI: i16;
  astruct_79 *paVar1;
  let puVar2: *mut u16;
  
  paVar1 = struct_op_1010_1d48((astruct_79 *)CONCAT22(param_2,param_1),param_3);
  &param_1->field_0xa = 0x0;
  CONCAT22(param_2,param_1) = 0x8ee2;
  param_1->field_0x2 = 0x1010;
  puVar2 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_4,
                           (paVar1 >> 0x10),unaff_DI);
  param_1->field_0xa = puVar2;
  param_1->field_0xc = (puVar2 >> 0x10);
  return CONCAT22(param_2,param_1);
}



fn pass1_1010_8c78(param_1: *mut u16,param_2: u16)
{
  *param_1 = 0x8ee2;
  (param_1 + 0x2) = 0x1010;
  pass1_1010_1d80(param_1,param_2);
  return;
}


fn pass1_1010_8ebc(param_1: u32,param_2: u8) -> u32

{
  let unaff_SS: u16;
  
  pass1_1010_8c78((u16 *)param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_8ef2(param_1: *mut u16,uchar *param_2,param_3: u16)
{
  let uVar1: u16;
  let puVar2: *mut u8
  let extraout_DX: *mut u8
  astruct_170 *iVar3;
  let unaff_DI: i16;
  let uVar3: u16;
  let puVar4: *mut u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_170 *)param_1;
  *param_1 = 0x389a;
  iVar3->field_0x2 = 0x1008;
  uVar1 = 0x0;
  &iVar3->field_0x4 = 0x0;
  &iVar3->field_0x8 = 0x0;
  *param_1 = 0x9254;
  iVar3->field_0x2 = 0x1010;
  mem_op_1000_179c(0x18,param_2,0x1000);
  puVar2 = (param_2 | uVar1);
  if (puVar2 == 0x0) {
    &iVar3->field_0x4 = 0x0;
  }
  else {
    struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_2,uVar1),0x5,0x5);
    iVar3->field_0x4 = uVar1;
    iVar3->field_0x6 = extraout_DX;
    puVar2 = extraout_DX;
  }
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_3,puVar2,unaff_DI);
  iVar3->field_0x8 = puVar4;
  iVar3->field_0xa = (puVar4 >> 0x10);
  return;
}



fn pass1_1010_8f78(param_1: *mut u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_490 *iVar4;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_490 *)param_1;
  *param_1 = 0x9254;
  iVar4->field_0x2 = 0x1010;
  puVar1 = iVar4->field_0x4;
  uVar2 = iVar4->field_0x6;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  *param_1 = 0x389a;
  iVar4->field_0x2 = 0x1008;
  return;
}



fn pass1_1010_8fba(param_1: u32,param_2: u16)
{
  code **ppcVar1;
  let uVar2: u32;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  astruct_411 *iVar3;
  let uVar3: u16;
  let uStack14: u32;
  let uStack10: u32;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_411 *)param_1;
  ppcVar1 = (code **)(*iVar3->field_0x4 + 0x10);
  (**ppcVar1)();
  uStack10 = CONCAT22(extraout_DX,param_2);
  uStack14 = 0x0;
  while( true ) {
    if (uStack10 <= uStack14) {
      return;
    }
    ppcVar1 = (code **)(*iVar3->field_0x4 + 0x4);
    uVar2 = uStack10;
    (**ppcVar1)();
    if ((extraout_DX_00 | uVar2) != 0x0) break;
    uStack14 += 0x1;
  }
  ppcVar1 = (code **)(*iVar3->field_0x4 + 0x8);
  (**ppcVar1)();
  return;
}



fn pass1_1010_9044(param_1: u32)
{
  code **ppcVar1;
  
  ppcVar1 = (code **)((param_1 + 0x4) + 0x10);
  (**ppcVar1)();
  return;
}




fn pass1_1010_9092(param_1: u32,param_2: u16,param_3: u16)
{
  code **ppcVar1;
  let uVar2: u32;
  let extraout_DX: *mut u8
  let puVar3: *mut u8
  let puVar4: *mut u8
  let extraout_DX_00: u16;
  let iVar5: i16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u32;
  let uStack14: u32;
  let uStack6: u32;
  
  uVar6 = (param_1 >> 0x10);
  iVar5 = param_1;
  uVar8 = (iVar5 + 0x4);
  ppcVar1 = (code **)((iVar5 + 0x4) + 0x10);
  (**ppcVar1)();
  uStack6 = CONCAT22(extraout_DX,param_2);
  uVar7 = 0xc;
  puVar3 = extraout_DX;
  mem_op_1000_179c(0xc,extraout_DX,0x1000);
  puVar4 = (puVar3 | param_2);
  if (puVar4 == 0x0) {
    param_2 = 0x0;
    puVar4 = 0x0;
  }
  else {
    pass1_1010_8ef2((u16 *)CONCAT22(puVar3,param_2),puVar4,param_3);
  }
  for (uStack14 = 0x0; uStack14 < uStack6; uStack14 += 0x1) {
    ppcVar1 = (code **)((iVar5 + 0x4) + 0x4);
    uVar2 = uStack6;
    (**ppcVar1)(0x1000,(iVar5 + 0x4),uStack14,uVar7,uVar8);
    if ((extraout_DX_00 | uVar2) != 0x0) {
      ppcVar1 = (code **)((param_2 + 0x4) + 0xc);
      (**ppcVar1)(0x1000,(param_2 + 0x4),uVar2,extraout_DX_00);
    }
  }
  return;
}



void 
pass1_1010_9130(param_1: u32,uchar *param_2,param_3: u16,param_4: u16,param_5: u16,
               param_6: u8)

{
  let uVar1: u32;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  pass1_1030_1d58((param_1 + 0x4));
  if ((uchar *)(param_4 | param_3) != 0x0) {
    uVar1 = (param_1 + 0x8);
    pass1_1010_c3c2(uVar1,(uVar1 >> 0x10),param_2,
                    CONCAT22(param_4,param_3),(param_4 | param_3),param_6,param_5
                   );
    return;
  }
  *param_2 = '\0';
  return;
}



fn pass1_1010_91cc(param_1: u32)
{
  code **ppcVar1;
  let uVar2: u16;
  let lVar3: i32;
  
  uVar2 = (param_1 >> 0x10);
  ppcVar1 = (code **)((param_1 + 0x4) + 0x10);
  lVar3 = (**ppcVar1)();
  if (lVar3 != 0x0) {
    ppcVar1 = (code **)((param_1 + 0x4) + 0x8);
    (**ppcVar1)();
  }
  return;
}



fn pass1_1010_9210(param_1: u32)
{
  code **ppcVar1;
  
  ppcVar1 = (code **)((param_1 + 0x4) + 0xc);
  (**ppcVar1)();
  return;
}



fn pass1_1010_922e(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1010_8f78(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1010_9258(param_1: *mut u16) -> u16

{
  struct_1010_383a(param_1);
  *param_1 = 0x958e;
  (param_1 + 0x2) = 0x1010;
  return param_1;
}



fn pass1_1010_927a(param_1: *mut u16)
{
  *param_1 = 0x958e;
  (param_1 + 0x2) = 0x1010;
  pass1_1010_3880(param_1);
  return;
}



u32 
pass1_1010_9298(astruct_79 *param_1,astruct_79 *param_2,param_3: u16,param_4: u16,
               uchar *param_5,param_6: u16)

{
  struct_1010_2cd2(param_1,param_2,param_3,param_6);
  CONCAT22(param_2,param_1) = 0x9566;
  param_1->field_0x2 = 0x1010;
  mem_op_1000_179c(0x20c,param_5,0x1000);
  param_1[0x9].field_0x2 = param_4;
  *(uchar **)&param_1[0x9].field_0x4 = param_5;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_5,param_1[0x9].field_0x2),(WNDCLASS16 *)0x0
                  ,0x20c);
  return CONCAT22(param_2,param_1);
}



fn pass1_1010_92e6(param_1: *mut u16,param_2: u16)
{
  *param_1 = 0x9566;
  (param_1 + 0x2) = 0x1010;
  pass1_1010_2db2(param_1,param_2);
  return;
}



fn pass1_1010_9304(param_1: u16,param_2: u16,param_3: i16,param_4: u16,uchar *param_5)
{
  if (param_3 != 0x0) {
    mem_op_1000_179c(param_3 << 0x2,param_5,0x1000);
    return;
  }
  mem_op_1000_179c(0x1a,param_5,0x1000);
  if ((param_5 | param_4) != 0x0) {
    pass1_1010_9258((u16 *)CONCAT22(param_5,param_4));
    return;
  }
  return;
}



fn pass1_1010_9348(param_1: u32,param_2: i16)
{
  let iVar1: i16;
  let uVar2: u16;
  
  (param_2 * 0x8 + 0x319e) = param_2;
  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x16) = param_2 * 0x8 + 0x3198;
  (iVar1 + 0x18) = ctx.data_seg;
  (iVar1 + 0x12) = param_2;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_9372(param_1: *mut u32,param_2: u16,param_3: i16,param_4: i16,param_5: i16)
{
  code **ppcVar1;
  let cVar2: u8;
  let uVar3: u16;
  let uVar4: u16;
  let unaff_DI: i16;
  let unaff_SS: u16;
  let bVar5: bool;
  let uVar6: u32;
  let uVar7: u32;
  
  if (0x0 < param_4) {
    if (_PTR_LOOP_1050_3528 == 0x0) {
      ppcVar1 = (code **)(*param_1 + 0x18);
      uVar6 = (**ppcVar1)();
      _PTR_LOOP_1050_3528 =
           mixed_1010_20ba(_PTR_LOOP_1050_0ed0,uVar6,unaff_SS,
                           (uVar6 >> 0x10),unaff_DI);
    }
    uVar6 = (param_1 + 0xc);
    uVar7 = pass1_1010_2e02(_PTR_LOOP_1050_3528,(uVar6 + 0x12));
    uVar3 = param_2 + 0x1;
    uVar4 = param_3 + (0xfffe < param_2);
    for (cVar2 = (param_4 + -0x1) * '\x04'; cVar2 != '\0'; cVar2 += -0x1) {
      bVar5 = CARRY2(uVar3,uVar3);
      uVar3 *= 0x2;
      uVar4 = uVar4 * 0x2 + bVar5;
    }
    pass1_1010_2e30(_PTR_LOOP_1050_3528,uVar3 | uVar7,
                    uVar4 | (uVar7 >> 0x10),param_5);
  }
  return;
}



fn pass1_1010_93f0(param_1: u32,param_2: u16)
{
  let puVar1: *mut u8;
  let uVar2: u16;
  let iVar3: i16;
  let uVar4: u16;
  let puVar5: *mut u16;
  u8 local_1c [0x1a];
  
  uVar4 = (param_1 >> 0x10);
  iVar3 = param_1;
  if (*(long *)(iVar3 + 0x56) == 0x0) {
    puVar5 = pass1_1010_9258((u16 *)CONCAT22(param_2,local_1c));
    uVar2 = (puVar5 >> 0x10);
    puVar1 = local_1c;
    pass1_1010_398e(CONCAT22(param_2,puVar1),0x0,0x0,0x0,puVar1);
    (iVar3 + 0x56) = puVar1;
    (iVar3 + 0x58) = uVar2;
    pass1_1010_927a((u16 *)CONCAT22(param_2,local_1c));
  }
  return;
}



fn pass1_1010_944e(param_1: u16,param_2: u16,param_3: i16)
{
  code **ppcVar1;
  let uVar2: u32;
  
  if (*(long *)(param_1 + 0x56) == 0x0) {
    ppcVar1 = (code **)(CONCAT22(param_2,param_1) + 0x10);
    (**ppcVar1)();
  }
  uVar2 = pass1_1010_2e02(CONCAT22(param_2,param_1),param_3);
  pass1_1010_2e5c(param_1,param_2,uVar2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

bool 
pass1_1010_9488(param_1: u16,param_2: u16,param_3: u32,uchar *param_4,param_5: i16,
               param_6: u16)

{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let puVar4: *mut u16;
  let uVar5: u16;
  let uStack10: u16;
  
  uVar5 = (param_3 + 0x12);
  puVar4 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x3,param_6,param_4,param_5);
  uVar2 = (puVar4 >> 0x10);
  uVar1 = uVar5 - 0x32;
  uStack10 = puVar4;
  uVar3 = uVar2;
  if (uVar1 == 0x0) {
    pass1_1010_a5ca(uStack10,uVar2,0x32,0x0,uVar2);
    if (uVar1 != 0x0) {
      return false;
    }
    uVar5 = 0x4d;
  }
  else {
    uVar1 = uVar5 - 0x3f;
    if (uVar1 == 0x0) {
      pass1_1010_a5ca(uStack10,uVar2,0x3f,0x0,uVar2);
      if (uVar1 != 0x0) {
        return false;
      }
      uVar5 = 0x4e;
    }
  }
  pass1_1010_a5ca(uStack10,uVar2,uVar5,uVar1,uVar3);
  return uVar1 == 0x0;
}



fn pass1_1010_9502(param_1: u32) -> u16

{
  let uVar1: u32;
  
  uVar1 = (param_1 + 0x16);
  return (uVar1 + 0x2);
}



fn pass1_1010_9514(void) -> u16

{
  return 0x31;
}



fn pass1_1010_951a(param_1: *mut u16,param_2: u8) -> u16

{
  pass1_1010_927a(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1010_9540(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_92e6(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1010_95f8(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  astruct_491 *iVar4;
  let uVar4: u16;
  
  uVar4 = (param_1 >> 0x10);
  iVar4 = (astruct_491 *)param_1;
  *param_1 = 0xa1c8;
  iVar4->field_0x2 = 0x1010;
  puVar1 = iVar4->field_0xa;
  uVar2 = iVar4->field_0xc;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar4->field_0xe;
  uVar2 = iVar4->field_0x10;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  puVar1 = iVar4->field_0x12;
  uVar2 = iVar4->field_0x14;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1,param_2);
  return;
}



fn pass1_1010_9674(param_1: u32)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let iVar4: i16;
  let uVar5: u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar4 = param_1;
  puVar1 = (iVar4 + 0x12);
  uVar2 = (iVar4 + 0x14);
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  (iVar4 + 0x12) = 0x0;
  return;
}



fn pass1_1010_96a8(param_1: u32,param_2: i16)
{
  let piVar1: *mut i16;
  let uVar2: u16;
  
  uVar2 = (param_1 >> 0x10);
  piVar1 = (param_1 + 0x1e);
  *piVar1 = *piVar1 - param_2;
  if (*piVar1 < 0x0) {
    (param_1 + 0x1e) = 0x0;
  }
  return;
}



fn pass1_1010_96c2(param_1: u32) -> u16

{
  return (param_1 + 0x1e);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16  pass1_1010_96d0(param_1: u32)

{
  let piVar1: *mut i16;
  let iVar2: i16;
  astruct_690 *iVar3;
  let uVar3: u16;
  let uVar4: u32;
  let iStack8: i16;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_690 *)param_1;
  if (iVar3->field_0x1a != 0x0) {
    if (0x0 < iVar3->field_0x1c) {
      piVar1 = &iVar3->field_0x1c;
      *piVar1 = *piVar1 + -0x1;
    }
    if ((iVar3->field_0x1c == 0x0) && (iVar3->field_0x1e != 0x0)) {
      iStack8 = 0x1;
      uVar4 = pass1_1030_8326();
      iVar2 = (uVar4 >> 0x10);
      if ((iVar2 != 0x0) || (0x32 < uVar4)) {
        iStack8 = 0x5;
      }
      if ((iVar2 != 0x0) || (0x3c < uVar4)) {
        iStack8 = 0xa;
      }
      if (iVar3->field_0x1e < iStack8) {
        iStack8 = iVar3->field_0x1e;
      }
      piVar1 = &iVar3->field_0x1e;
      *piVar1 = *piVar1 - iStack8;
      if (*piVar1 < 0x0) {
        iVar3->field_0x1e = 0x0;
      }
      if (0x0 < iVar3->field_0x1e) {
        return iStack8;
      }
      return -0x1;
    }
  }
  return 0x0;
}



fn pass1_1010_9766(param_1: u32,param_2: u16,param_3: u16,param_4: u16)
{
  let in_AX: i16;
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x1a) = 0x1;
  pass1_1010_a0a0(param_1,param_2,param_3,param_4);
  pass1_1010_9f8c(param_1,0x80,param_4);
  (param_1 + 0x1e) = in_AX * 0x32;
  return;
}



fn pass1_1010_9794(param_1: u32,param_2: u16)
{
  let iVar1: i16;
  code **ppcVar2;
  let puVar3: u32;
  let uVar4: u16;
  astruct_251 *puVar5;
  let puVar6: u32;
  let puVar7: *mut u8
  let extraout_DX: *mut u8
  let extraout_DX_00: u16;
  let uVar8: u16;
  let extraout_DX_01: u16;
  astruct_250 *iVar9;
  let uVar9: u16;
  undefined8 local_a;
  
  uVar9 = (param_1 >> 0x10);
  iVar9 = (astruct_250 *)param_1;
  if (iVar9->field_0x18 == 0x0) {
    iVar9->field_0x18 = 0x1;
    puVar6 = iVar9->field_0xe;
    uVar4 = (&iVar9->field_0xe + 0x2);
    puVar7 = (uVar4 | puVar6);
    if (puVar7 != 0x0) {
      ppcVar2 = (code **)puVar6;
      (**ppcVar2)();
      puVar7 = extraout_DX;
    }
    mem_op_1000_179c(0xc,puVar7,0x1000);
    uVar4 = puVar6;
    if ((puVar7 | uVar4) == 0x0) {
      uVar4 = 0x0;
      uVar8 = 0x0;
    }
    else {
      set_struct_1008_574a
                ((astruct_21 *)(puVar6 & 0xffff | ZEXT24(puVar7) << 0x10));
      uVar8 = extraout_DX_00;
    }
    &iVar9->field_0xe = uVar4;
    (&iVar9->field_0xe + 0x2) = uVar8;
    pass1_1008_5784(CONCAT22(param_2,&local_a),iVar9->field_0xa);
    while( true ) {
      puVar5 = (astruct_251 *)&local_a;
      pass1_1008_5b12(puVar5,param_2);
      if ((extraout_DX_01 | puVar5) == 0x0) break;
      iVar1 = puVar5->field_0x4;
      if ((iVar1 == 0x3e) || (iVar1 == 0x41)) {
        puVar6 = iVar9->field_0xa;
        (puVar6 + 0xa) = 0x0;
        puVar6 = iVar9->field_0xa;
        ppcVar2 = (code **)(*iVar9->field_0xa + 0xc);
        (**ppcVar2)();
        puVar3 = iVar9->field_0xa;
        (puVar3 + 0xa) = 0x1;
        local_a._4_4_ = 0x0;
        ppcVar2 = (code **)(*iVar9->field_0xe + 0x4);
        (**ppcVar2)(0x1008,iVar9->field_0xe,CONCAT22(extraout_DX_01,puVar5),puVar6);
      }
    }
  }
  return;
}



fn pass1_1010_988c(param_1: u32,param_2: i16)
{
  code **ppcVar1;
  let uVar2: u32;
  let iVar3: i16;
  let iVar4: i16;
  let uVar5: u16;
  let iVar6: i16;
  let uVar7: u16;
  let unaff_SS: u16;
  let lVar8: i32;
  let local_a: [u8;8];
  
  uVar7 = (param_1 >> 0x10);
  iVar6 = param_1;
  pass1_1008_5784(CONCAT22(unaff_SS,local_a),(iVar6 + 0xe));
  do {
    lVar8 = pass1_1008_5b12(local_a,unaff_SS);
    uVar5 = (lVar8 >> 0x10);
    iVar3 = lVar8;
    if (lVar8 == 0x0) {
      return;
    }
  } while ((iVar3 + 0x4) != param_2);
  iVar4 = (iVar3 + 0x6) + -0x1;
  (iVar3 + 0x6) = iVar4;
  if ((iVar4 < 0x1) &&
     (ppcVar1 = (code **)((iVar6 + 0xe) + 0xc),
     (**ppcVar1)(0x1008,(iVar6 + 0xe),lVar8),
     uVar2 = (iVar6 + 0xe), (uVar2 + 0x8) == 0x0)) {
    (iVar6 + 0x16) = 0x1;
  }
  return;
}



fn pass1_1010_9f72(param_1: u32,param_2: i16,param_3: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  pass1_1010_9fa6(param_1,uVar1,(param_1 + 0xe),param_2,param_3)
  ;
  return;
}



fn pass1_1010_9f8c(param_1: u32,param_2: i16,param_3: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  pass1_1010_9fa6(param_1,uVar1,(param_1 + 0xa),param_2,param_3)
  ;
  return;
}



fn pass1_1010_9fa6(param_1: u16,param_2: u16,param_3: u32,param_4: i16,param_5: u16) -> u16

{
  let uVar1: u16;
  let lVar2: i32;
  let local_a: [u8;8];
  
  if (param_3 != 0x0) {
    pass1_1008_5784(CONCAT22(param_5,local_a),param_3);
    while( true ) {
      lVar2 = pass1_1008_5b12(local_a,param_5);
      uVar1 = (lVar2 >> 0x10);
      if (lVar2 == 0x0) break;
      if ((lVar2 + 0x4) == param_4) {
        return (lVar2 + 0x6);
      }
    }
  }
  return 0x0;
}



fn pass1_1010_9fee(param_1: u32,param_2: u16,param_3: u16,param_4: u16,uchar *param_5)
{
  code **ppcVar1;
  let puVar2: *mut u8
  let extraout_DX: *mut u8
  astruct_252 *iVar3;
  astruct_253 *iVar4;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let puStack10: *mut u16;
  let puStack6: *mut u16;
  
  uVar3 = (param_1 >> 0x10);
  iVar3 = (astruct_252 *)param_1;
  puVar2 = param_5;
  if (iVar3->field_0x12 == 0x0) {
    mem_op_1000_179c(0xc,param_5,0x1000);
    puVar2 = (param_5 | param_4);
    if (puVar2 == 0x0) {
      iVar3->field_0x12 = 0x0;
    }
    else {
      set_struct_1008_574a((astruct_21 *)CONCAT22(param_5,param_4));
      &iVar3->field_0x12 = param_4;
      *(uchar **)(&iVar3->field_0x12 + 0x2) = extraout_DX;
      puVar2 = extraout_DX;
    }
  }
  uVar5 = 0x8;
  mem_op_1000_179c(0x8,puVar2,0x1000);
  puStack10 = CONCAT22(puVar2,param_4);
  if ((puVar2 | param_4) == 0x0) {
    puStack6 = 0x0;
  }
  else {
    *puStack10 = 0x389a;
    (param_4 + 0x2) = 0x1008;
    *puStack10 = 0xa1c4;
    (param_4 + 0x2) = 0x1010;
    puStack6 = puStack10;
  }
  uVar4 = (puStack6 >> 0x10);
  iVar4 = (astruct_253 *)puStack6;
  iVar4->field_0x4 = param_3;
  iVar4->field_0x6 = param_2;
  ppcVar1 = (code **)(*iVar3->field_0x12 + 0x4);
  (**ppcVar1)(0x1000,iVar3->field_0x12,iVar4,uVar4,uVar5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_a0a0(param_1: u32,uchar *param_2,param_3: i16,param_4: u16)
{
  let piVar1: *mut i16;
  let iVar2: i16;
  let iVar3: i16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let bVar7: bool;
  let bVar8: bool;
  let lVar9: i32;
  let iStack12: i16;
  let local_a: [u8;8];
  
  pass1_1008_5784(CONCAT22(param_4,local_a),(param_1 + 0xa));
  iStack12 = 0x4;
  mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2,param_4,param_2,param_3);
  if ((ctx.PTR_LOOP_1050_13ae != &ctx.PTR_LOOP_1050_0002) &&
     (ctx.PTR_LOOP_1050_13ae != (&ctx.PTR_LOOP_1050_0000 + 0x1))) {
    iStack12 = 0x2;
  }
  do {
    while( true ) {
      lVar9 = pass1_1008_5b12(local_a,param_4);
      uVar6 = (lVar9 >> 0x10);
      iVar4 = lVar9;
      if (lVar9 == 0x0) {
        return;
      }
      iVar2 = (iVar4 + 0x4);
      if (iVar2 != 0x12) break;
      piVar1 = (iVar4 + 0x6);
      bVar8 = SBORROW2(*piVar1,0x2);
      iVar3 = *piVar1 + -0x2;
      bVar7 = iVar3 == 0x0;
//LAB_1010_a151:
      if (!bVar7 && bVar8 == iVar3 < 0x0) {
//LAB_1010_a153:
        piVar1 = (iVar4 + 0x6);
        *piVar1 = *piVar1 - (iVar4 + 0x6) / iStack12;
      }
    }
    if (((iVar2 != 0x3e) && (iVar2 != 0x41)) && (iVar2 != 0x80)) {
      if (iVar2 == 0x83) {
        piVar1 = (iVar4 + 0x6);
        bVar8 = SBORROW2(*piVar1,0x1);
        iVar2 = *piVar1;
        iVar3 = iVar2 + -0x1;
        bVar7 = iVar2 == 0x1;
        goto LAB_1010_a151;
      }
      goto LAB_1010_a153;
    }
    iVar2 = (iVar4 + 0x6);
    uVar5 = iVar2 / 0x2;
    piVar1 = (iVar4 + 0x6);
    *piVar1 = *piVar1 - uVar5;
    if (uVar5 == 0x0) {
      uVar5 = 0x1;
    }
    pass1_1010_9fee(param_1,uVar5,(iVar4 + 0x4),uVar5,iVar2 >> 0xf);
  } while( true );
}



fn pass1_1010_a172(param_1: *mut u16,param_2: u8,param_3: u16) -> u16

{
  pass1_1010_95f8(param_1,param_3);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}



fn pass1_1010_a198(param_1: *mut u16,param_2: u8) -> u16

{
  *param_1 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1010_a478(param_1: *mut u16,param_2: u16)
{
  let puVar1: *mut u16;
  let uVar2: u16;
  astruct_497 *uVar3;
  let uVar4: u16;
  let puStack6: *mut u16;
  
  uVar4 = (param_1 >> 0x10);
  uVar3 = (astruct_497 *)param_1;
  *param_1 = 0xe9cc;
  uVar3->field_0x2 = 0x1010;
  uVar3->field_0xa = 0xe9dc;
  uVar3->field_0xc = 0x1010;
  if (uVar3->field_0x138 != 0x0) {
    if (param_1 == 0x0) {
      puVar1 = 0x0;
      uVar2 = 0x0;
    }
    else {
      puVar1 = &uVar3->field_0xa;
      uVar2 = uVar4;
    }
    pass1_1010_1ea6(uVar3->field_0x138,CONCAT22(uVar2,puVar1),param_2);
  }
  uVar3->field_0x138 = 0x0;
  if (param_1 == 0x0) {
    puVar1 = 0x0;
    uVar4 = 0x0;
  }
  else {
    puVar1 = &uVar3->field_0xa;
  }
  puStack6 = CONCAT22(uVar4,puVar1);
  *puStack6 = 0x389a;
  puVar1[0x1] = 0x1008;
  pass1_1010_1d80(param_1,param_2);
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1010_a50c(astruct_20 *param_1,param_2: u32,param_3: u32)
{
  let iVar1: i16;
  astruct_260 *iVar2;
  let local_8: u32;
  let iStack4: i16;
  
  iVar1 = param_1;
  pass1_1000_4906((astruct_20 *)(param_1 & 0xffff0000 | (iVar1 + 0xa4)),
                  (WNDCLASS16 *)0x0,0x94);
  iVar2 = (astruct_260 *)((param_3 + 0xa) * 0x6 + iVar1);
  local_8 = iVar2->field_0xe;
  iStack4 = iVar2->field_0x12;
  (*local_8)(0x1000,iVar1 + iStack4,param_1._2_2_,param_2,param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1010_a568(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16
               )

{
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,0x8000001);
  pass1_1030_2622(CONCAT22(param_5,param_4),param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1010_a58a(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16
               )

{
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,0x8000001);
  pass1_1030_266c(param_4,CONCAT22(param_3,param_5));
  return;
}



fn pass1_1010_a5ac(param_1: u16,param_2: u16,param_3: u32) -> u16

{
  let uVar1: u32;
  
  uVar1 = struct_op_1030_73a8(param_3);
  return (uVar1 + 0x20);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1010_a5ca(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16
               )

{
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,0x8000001);
  pass1_1030_2242(CONCAT22(param_5,param_4),param_3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_a5ec(param_1: u16,param_2: u16,param_3: u16,param_4: u32,param_5: u16)
{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let extraout_DX: u16;
  let puVar6: u32;
  let uStack6: u32;
  
  uVar2 = param_4._2_2_ | param_4;
  if (param_4 != 0x0) {
    pass1_1030_8344(_PTR_LOOP_1050_5748,
                    (_PTR_LOOP_1050_5748 >> 0x10),0x8000001);
    uStack6 = CONCAT22(param_5,uVar2);
    puVar6 = struct_op_1030_73a8(param_4);
    uVar5 = (puVar6 >> 0x10);
    uVar4 = (puVar6 + 0x20);
    if (uVar4 != param_3) {
      uVar3 = param_3;
      pass1_1010_a5ca(param_1,param_2,uVar4,param_3,uVar5);
      if ((uVar4 != 0x70) && (uVar3 < 0x0)) {
        pass1_1030_25d8(CONCAT22(param_5,uVar2),uVar3 + 0x1,uVar4);
      }
      ppcVar1 = (code **)(*puVar6 + 0x8);
      uVar4 = param_3;
      (**ppcVar1)();
      if (param_3 != 0x70) {
        pass1_1010_a5ca(param_1,param_2,param_3,uVar4,extraout_DX);
        if (uVar4 < 0x0) {
          pass1_1030_25d8(uStack6,uVar4 - 0x1,param_3);
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1010_a69c(param_1: u32,param_2: u16,param_3: i16,uchar *param_4,param_5: i16,
               param_6: u16)

{
  let iVar1: i16;
  let uVar2: u16;
  let puVar3: *mut u8
  let puVar4: *mut u8
  astruct_25 *paVar5;
  astruct_67 *paVar6;
  let puVar7: *mut u16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let uStack22: u16;
  let iStack20: i16;
  
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,0x8000001);
  if (param_2 == 0x1) {
    puVar3 = param_4;
    for (iStack20 = 0x0; iStack20 < 0x83; iStack20 += 0x1) {
      iVar1 = pass1_1030_2242(CONCAT22(param_4,param_3),iStack20);
      if (0x19 < iVar1) {
        uStack22 = iVar1 - 0x5;
        if (uStack22 < 0x19) {
          uStack22 = 0x19;
        }
        pass1_1030_25d8(CONCAT22(param_4,param_3),uStack22,iStack20);
      }
    }
    goto switchD_1010_aaef_caseD_b;
  }
  puVar3 = param_4;
  pass1_1030_25f0(CONCAT22(param_4,param_3),0x0,param_2);
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x35,param_6,puVar3,param_5);
  puVar3 = (puVar7 >> 0x10);
  if (false) goto switchD_1010_aaef_caseD_b;
  uVar2 = param_1;
  uVar8 = (param_1 >> 0x10);
  puVar4 = puVar3;
  switch(param_2) {
  case 0xa:
  case 0xc:
    iVar1 = 0x1b;
    break;
  default:
    goto switchD_1010_aaef_caseD_b;
  case 0x10:
    pass1_1010_682e(puVar7,0x1,0x2d);
    if ((param_3 + 0x160) == 0x0) goto switchD_1010_aaef_caseD_b;
    iVar1 = 0x2d;
    goto LAB_1010_a91f;
  case 0x12:
    pass1_1010_682e(puVar7,0x1,0x16);
    pass1_1010_682e(puVar7,0x1,0x17);
    pass1_1010_682e(puVar7,0x1,0x18);
    pass1_1010_682e(puVar7,0x1,0x40);
    iVar1 = 0x3f;
    goto LAB_1010_a96c;
  case 0x13:
    iVar1 = 0x35;
    goto LAB_1010_a91f;
  case 0x19:
    goto switchD_1010_aaef_caseD_19;
  case 0x1a:
    iVar1 = 0xf;
    goto LAB_1010_a96c;
  case 0x1c:
    iVar1 = 0x11;
    goto LAB_1010_a96c;
  case 0x1d:
  case 0x24:
    pass1_1010_abd2(uVar2,uVar8,0x1e,puVar3,param_5,param_6);
    iVar1 = 0x5b;
    goto LAB_1010_a91f;
  case 0x1e:
    uVar2 = 0x1;
    iVar1 = 0x2;
    puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_6,puVar3,param_5);
    puVar3 = (puVar7 >> 0x10);
    pass1_1010_08c0(puVar7,uVar2,iVar1,param_6);
    paVar5 = (astruct_25 *)
             mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x40,param_6,puVar3,param_5);
    puVar3 = (paVar5 >> 0x10);
    load_str_and_sprintf_1008_b69c(paVar5,param_6,puVar3);
    goto switchD_1010_aaef_caseD_b;
  case 0x22:
    iVar1 = 0x8;
    goto LAB_1010_aabe;
  case 0x23:
    iVar1 = 0xc;
    goto LAB_1010_aabe;
  case 0x25:
    pass1_1010_abd2(uVar2,uVar8,0x14,puVar3,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x1b,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x1e,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x22,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x25,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x28,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x2a,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x2d,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x2f,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x31,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x35,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x38,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x3a,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x3c,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x48,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x4f,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x52,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x54,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x57,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x5b,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x5d,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x62,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x66,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x68,puVar4,param_5,param_6);
    pass1_1010_abd2(uVar2,uVar8,0x6c,puVar4,param_5,param_6);
    goto switchD_1010_aaef_caseD_19;
  case 0x29:
    iVar1 = 0x25;
    break;
  case 0x2a:
    iVar1 = 0xf;
    goto LAB_1010_aabe;
  case 0x2b:
    iVar1 = 0x6e;
    goto LAB_1010_a96c;
  case 0x30:
    iVar1 = 0x54;
    break;
  case 0x33:
    pass1_1010_abd2(uVar2,uVar8,0x31,puVar3,param_5,param_6);
    iVar1 = 0x6c;
    goto LAB_1010_a91f;
  case 0x36:
    iVar1 = 0x13;
    goto LAB_1010_aabe;
  case 0x37:
    iVar1 = 0x2c;
//LAB_1010_a96c:
    pass1_1010_682e(puVar7,0x1,iVar1);
    goto switchD_1010_aaef_caseD_b;
  case 0x38:
    pass1_1010_682e(puVar7,0x1,0x28);
    if ((param_3 + 0x160) == 0x0) goto switchD_1010_aaef_caseD_b;
    iVar1 = 0x28;
    goto LAB_1010_a91f;
  case 0x39:
    iVar1 = 0x10;
    goto LAB_1010_aabe;
  case 0x3a:
    iVar1 = 0x11;
    goto LAB_1010_aabe;
  case 0x3b:
    iVar1 = 0x12;
//LAB_1010_aabe:
    pass1_1010_6814(puVar7,0x1,iVar1);
    goto switchD_1010_aaef_caseD_b;
  case 0x3c:
    pass1_1010_abd2(uVar2,uVar8,0x14,puVar3,param_5,param_6);
    iVar1 = 0x62;
    goto LAB_1010_a91f;
  case 0x3d:
    pass1_1010_682e(puVar7,0x1,0x66);
    if ((param_3 + 0x160) == 0x0) goto switchD_1010_aaef_caseD_b;
    iVar1 = 0x66;
//LAB_1010_a91f:
    pass1_1010_abd2(uVar2,uVar8,iVar1,puVar3,param_5,param_6);
    goto switchD_1010_aaef_caseD_b;
  case 0x3e:
    iVar1 = 0x5d;
    break;
  case 0x3f:
    iVar1 = 0x22;
    break;
  case 0x40:
    iVar1 = 0x57;
    break;
  case 0x41:
    iVar1 = 0x4f;
  }
  pass1_1010_abd2(uVar2,uVar8,iVar1,puVar3,param_5,param_6);
switchD_1010_aaef_caseD_b:
  paVar6 = (astruct_67 *)mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_6,puVar3,param_5);
  puVar3 = (paVar6 >> 0x10);
  uVar2 = pass1_1008_ab12(paVar6,puVar3,param_2);
  if (uVar2 != 0x0) {
    post_win_msg_1008_a0e4(paVar6,0x0,0x0,0x1,0x0,uVar2,0x1008,param_6);
  }
  post_win_msg_1008_a0e4(paVar6,0x0,0x0,0x1,0x0,0x3d,0x1008,param_6);
  uVar10 = 0x400;
  iVar1 = 0x6;
  uVar9 = 0x1;
  puVar7 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2b,param_6,puVar3,param_5);
  pass1_1010_043a(puVar7,CONCAT22(uVar10,uVar9),iVar1,param_6);
  return;
switchD_1010_aaef_caseD_19:
  (puVar7 + 0x148) = 0x34;
  puVar3 = puVar4;
  goto switchD_1010_aaef_caseD_b;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1010_abd2(param_1: u16,param_2: u16,param_3: i16,uchar *param_4,param_5: i16,
               param_6: u16)

{
  let piVar1: *mut i16;
  let uVar2: u16;
  let iStack20: i16;
  let iStack16: i16;
  let iStack14: i16;
  let iStack12: i16;
  let uStack10: u32;
  let puStack6: *mut u16;
  
  puStack6 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x35,param_6,param_4,param_5);
  uVar2 = puStack6 + 0xa;
  uStack10 = puStack6 & 0xffff0000 | uVar2;
  iStack12 = 0x0;
  iStack16 = param_3;
  _iStack20 = CONCAT22(param_6,&stack0x000a);
  while( true ) {
    piVar1 = _iStack20;
    if (iStack16 == 0x0) {
      return;
    }
    if (iStack12 != 0x0) break;
    if ((iStack16 * 0x2 + uVar2) != 0x0) {
      iStack12 = 0x1;
      iStack14 = iStack16;
    }
    _iStack20 = (_iStack20 & 0xffff0000 | (iStack20 + 0x2));
    iStack16 = *piVar1;
  }
  pass1_1010_682e(puStack6,0x0,iStack14);
  pass1_1010_682e(puStack6,0x1,iStack16);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_ac62(param_1: u16,param_2: u16,param_3: i16,param_4: u16,param_5: u16) -> u16

{
  pass1_1030_8344(_PTR_LOOP_1050_5748,(_PTR_LOOP_1050_5748 >> 0x10)
                  ,0x8000001);
  return (param_4 + 0x116 + param_3 * 0x2);
}



fn pass1_1010_acc0(param_1: u16,param_2: u16,param_3: u32) -> u16

{
  let uVar1: u32;
  
  uVar1 = struct_op_1030_73a8(param_3);
  if ((uVar1 + 0x12) != 0x4) {
    return 0x1;
  }
  return 0x0;
}



fn pass1_1010_acec(param_1: u32,param_2: i16,param_3: u16)
{
  if (param_2 == 0x1) {
    (param_1 + 0x12e) = 0x0;
  }
  else {
    if (param_2 != 0x5) {
      return;
    }
  }
  pass1_1010_1f62(param_3,param_1 & 0xffff0000 | (param_1 - 0xa),param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_ad22(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16)
{
  let uVar1: u32;
  let puVar2: *mut u16;
  let uStack8: u16;
  
  uVar1 = (param_1 + 0x138);
  puVar2 = &param_5;
  pass1_1030_627e(param_3,puVar2,param_2,_PTR_LOOP_1050_5740,
                  CONCAT22(param_3,puVar2),*(long *)(uVar1 + 0x20));
  if ((param_2 | puVar2) == 0x0) {
    return;
  }
  uStack8 = param_2;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,puVar2,param_2);
  return;
}



fn pass1_1010_ad64(param_1: u16,param_2: u32,param_3: u32,param_4: u32,param_5: u16)
{
  if (param_3 != 0x0) {
    param_4 = (param_3 + 0x2e);
    if (*(long *)(param_4 + 0x200) == 0x8000002) {
      return;
    }
  }
  pass1_1010_c58as(param_1,param_2,(param_2 >> 0x10),param_3,
                   param_4,param_5);
  return;
}


fn pass1_1010_ae12(param_1: u16,param_2: u16,param_3: u32,param_4: i16,param_5: u16) -> u16

{
  char *pcVar1;
  let iVar2: i16;
  let uStack4: u16;
  
  if (param_4 == 0x6) {
    for (uStack4 = 0x0; uStack4 < 0x15; uStack4 += 0x1) {
      pcVar1 = string_op_1020_c222(uStack4);
      iVar2 = pass1_1000_3d7a(param_3,CONCAT22(param_5,pcVar1));
      if (iVar2 == 0x0) {
        return uStack4;
      }
    }
  }
  else {
    if (param_4 == 0x7) {
      for (uStack4 = 0x0; uStack4 < 0x11; uStack4 += 0x1) {
        pcVar1 = string_op_1020_c2f8(uStack4);
        iVar2 = pass1_1000_3d7a(param_3,CONCAT22(param_5,pcVar1));
        if (iVar2 == 0x0) {
          return uStack4;
        }
      }
    }
  }
  return 0xffff;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1010_ae92(param_1: u32,param_2: u16,param_3: u16,param_4: u32,param_5: i16,
               param_6: u16)

{
  let uVar1: u16;
  let puVar2: *mut u8
  let uVar3: u32;
  astruct_67 *paVar4;
  let uVar5: u16;
  let uVar6: u16;
  let iVar7: i16;
  let uVar8: u8;
  let uVar9: u8;
  let uVar10: u16;
  let uVar11: u16;
  let iVar12: i16;
  
  if (param_3 == 0x15) {
    uVar3 = struct_op_1030_73a8(param_4);
    if (uVar3 != 0x0) {
      (uVar3 + 0x20) = param_2;
      return;
    }
  }
  else {
    if (param_3 < 0x16) {
      if (param_3 == '\x06') {
        pass1_1030_7f1a(param_4,param_2,param_6);
        uVar3 = struct_op_1030_73a8(param_4);
        uVar1 = pass1_1010_b028(param_1,(param_1 >> 0x10),uVar3);
        uVar3 = pass1_1030_8326();
        puVar2 = (uVar3 >> 0x10);
        if (((uVar1 == 0xe) && ((puVar2 != 0x0 || (0x32 < uVar3)))) &&
           ((param_2 == 0x1 ||
            (((param_2 == 0x2 || (param_2 == 0x4)) || (param_2 == 0x3)))))) {
          uVar11 = 0x0;
          iVar12 = 0xb;
          uVar8 = 0x1;
          uVar9 = 0x0;
          uVar10 = 0x0;
          uVar6 = 0x0;
          iVar7 = 0x0;
          uVar5 = 0x0;
          paVar4 = (astruct_67 *)
                   mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x37,param_6,puVar2,param_5);
          post_win_msg_1008_a0e4
                    (paVar4,CONCAT22(uVar6,uVar5),iVar7,CONCAT11(uVar9,uVar8),
                     CONCAT22(uVar11,uVar10),iVar12,0x1008,param_6);
          return;
        }
      }
      else {
        if (param_3 == '\a') {
          pass1_1030_7eda(param_4,param_2,param_6);
          return;
        }
      }
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_af66(param_1: u32,param_2: u16)
{
  let uVar1: u32;
  let uVar2: u32;
  let uVar3: u16;
  let in_stack_00000008: u16;
  
  uVar1 = (param_1 + 0x138);
  uVar2 = (uVar1 + 0x24);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,(uVar2 >> 0x10));
  uVar3 = param_2 | uVar2;
  if (uVar3 == 0x0) {
    return;
  }
  pass1_1038_5050(uVar2 & 0xffff | param_2 << 0x10,in_stack_00000008,uVar2,
                  uVar3);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_afa2(param_1: u32,param_2: u16)
{
  let uVar1: u32;
  let uVar2: u32;
  let in_stack_00000008: u16;
  
  uVar1 = (param_1 + 0x138);
  uVar2 = (uVar1 + 0x24);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,(uVar2 >> 0x10));
  if ((param_2 | uVar2) == 0x0) {
    return;
  }
  pass1_1038_50e0(uVar2 & 0xffff | param_2 << 0x10,in_stack_00000008,uVar2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_afde(param_1: u32,param_2: i16)
{
  let uVar1: u32;
  let uVar2: u32;
  let in_DX: u16;
  let puVar3: u32;
  
  uVar1 = (param_1 + 0x138);
  uVar2 = (uVar1 + 0x24);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar2,(uVar2 >> 0x10));
  if ((in_DX | uVar2) == 0x0) {
    return;
  }
  puVar3 = pass1_1008_c6fa(_PTR_LOOP_1050_06e0,param_2);
  pass1_1038_4e78(puVar3,(puVar3 >> 0x10),
                  uVar2 & 0xffff | in_DX << 0x10,puVar3);
  return;
}



fn pass1_1010_b028(param_1: u16,param_2: u16,param_3: u32) -> u16

{
  return (param_3 + 0xc);
}


fn pass1_1010_bf1e(param_1: u32,i16 *param_2,param_3: i16,uchar *param_4,param_5: u16)
{
  let uVar1: u32;
  let uVar2: u16;
  let puVar3: *mut u8;
  let iVar4: i16;
  let uVar5: u16;
  let uStack40: u32;
  let iStack36: i16;
  let uStack32: u16;
  let puStack26: *mut u16;
  let local_16: [u8;12];
  let iStack4: i16;
  
  bad_1010_bf08(param_1,(param_1 >> 0x10),0x1000000);
  iStack4 = param_3 + -0x1;
  *param_2 = iStack4;
  uVar2 = iStack4 * 0x18;
  mem_op_1000_179c(uVar2,param_4,0x1000);
  uStack40 = CONCAT22(param_4,uVar2);
  uStack32 = param_4 | uVar2;
  iVar4 = param_2;
  uVar5 = (param_2 >> 0x10);
  if (uStack32 == 0x0) {
    (iVar4 + 0x2) = 0x0;
  }
  else {
    pass1_1000_5586((uchar *)0x4092,0x1020,iStack4,0x18,uVar2,param_4);
    (iVar4 + 0x2) = uStack40;
  }
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_5,local_16),0x1,0x0,0x100);
  puStack26 = (iVar4 + 0x2);
  iStack36 = 0x0;
  while( true ) {
    puVar3 = local_16;
    pass1_1028_e4ec(CONCAT22(param_5,puVar3));
    if ((uStack32 | puVar3) == 0x0) break;
    uVar1 = (puVar3 + 0x10);
    uStack32 = uStack32 | puVar3;
    if (uVar1 != 0x0) {
      uStack32 = (uVar1 >> 0x10);
      pass1_1008_3f62(puStack26,(uVar1 & 0xffff0000 | (uVar1 + 0x4))
                     );
      (puStack26 + 0xc) = iStack36;
      iStack36 += 0x1;
      puStack26 =
                  (puStack26 & 0xffff0000 | (puStack26 + 0x18));
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_bffa(param_1: u32,param_2: i16,uchar *param_3,param_4: u16)
{
  let puVar1: *mut u16;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  astruct_257 *puVar5;
  astruct_254 *iVar6;
  astruct_255 *iVar7;
  astruct_256 *iVar8;
  let iVar5: i16;
  let uVar6: u16;
  let uVar7: u16;
  astruct_43 *paVar8;
  let uStack34: u16;
  let iStack30: i16;
  let local_18: [u8;16];
  
  mem_op_1000_179c(0xa,param_3,0x1000);
  local_18._18_4_ = CONCAT22(param_3,param_2);
  bad_1010_bf08(param_1,(param_1 >> 0x10),0x2000000);
  uVar6 = (local_18._18_4_ >> 0x10);
  iVar6 = (astruct_254 *)local_18._18_4_;
  iVar6->field_0x8 = param_2;
  if (param_2 == 0x0) {
    iVar6->field_0x8 = 0x1;
  }
  uVar3 = iVar6->field_0x8 << 0x2;
  mem_op_1000_179c(uVar3,param_3,0x1000);
  uVar6 = (local_18._18_4_ >> 0x10);
  iVar7 = (astruct_255 *)local_18._18_4_;
  local_18._18_4_ = uVar3;
  iVar7->field_0x2 = param_3;
  if ((param_3 | local_18._18_4_) == 0x0) {
    iVar7->field_0x8 = 0x0;
  }
  else {
    uVar4 = iVar7->field_0x8 * 0x2;
    mem_op_1000_179c(uVar4,param_3,0x1000);
    uVar6 = (local_18._18_4_ >> 0x10);
    iVar8 = (astruct_256 *)local_18._18_4_;
    iVar8->field_0x4 = uVar4;
    iVar8->field_0x6 = param_3;
    if ((param_3 | iVar8->field_0x4) != 0x0) {
      paVar8 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,0x1b4,param_4);
      uVar3 = (paVar8 >> 0x10);
      puVar1 = *local_18._18_4_;
      *puVar1 = paVar8;
      (puVar1 + 0x2) = uVar3;
      (local_18._18_4_ + 0x4) = 0x0;
      pass1_1028_dc52((astruct_92 *)CONCAT22(param_4,local_18),0x1,0x0,0x200);
      iStack30 = 0x1;
      while( true ) {
        puVar5 = (astruct_257 *)local_18;
        pass1_1028_e4ec(CONCAT22(param_4,puVar5));
        if ((uVar3 | puVar5) == 0x0) break;
        uVar6 = *puVar5->field_0x10;
        uStack34 = 0x0;
        switch(uVar6) {
        case 0x1:
          uStack34 = 0x1b5;
          break;
        case 0x2:
          uStack34 = 0x1b6;
          break;
        case 0x3:
          uStack34 = 0x1b7;
          break;
        case 0x4:
          uStack34 = 0x1b8;
          break;
        case 0x5:
          uStack34 = 0x1b9;
          break;
        case 0x6:
          uStack34 = 0x1ba;
          break;
        case 0x7:
          uStack34 = 0x1bb;
          break;
        case 0x8:
          uStack34 = 0x1bc;
          break;
        case 0x9:
          uStack34 = 0x1bd;
          break;
        case 0xa:
          uStack34 = 0x1be;
        }
        paVar8 = unk_io_op_1010_830a(_PTR_LOOP_1050_14cc,uStack34,param_4);
        uVar3 = (paVar8 >> 0x10);
        uVar7 = (*local_18._18_4_ >> 0x10);
        iVar5 = *local_18._18_4_;
        (iVar5 + iStack30 * 0x4) = paVar8;
        (iVar5 + iStack30 * 0x4 + 0x2) = uVar3;
        uVar2 = (local_18._18_4_ + 0x4);
        (uVar2 + iStack30 * 0x2) = uVar6;
        iStack30 += 0x1;
      }
      return;
    }
  }
  return;
}



fn pass1_1010_c1ba(param_1: u16,param_2: u16,param_3: i16,param_4: u16,param_5: u16)
{
  let puVar1: *mut u8;
  let iStack28: i16;
  let local_16: [u8;12];
  let uStack4: u16;
  
  uStack4 = 0x0;
  pass1_1028_dc52((astruct_92 *)CONCAT22(param_5,local_16),0x1,0x0,0x200);
  iStack28 = 0x1;
  while( true ) {
    puVar1 = local_16;
    pass1_1028_e4ec(CONCAT22(param_5,puVar1));
    param_4 |= puVar1;
    if ((param_4 == 0x0) || (iStack28 == param_3)) break;
    iStack28 += 0x1;
  }
  return;
}



fn pass1_1010_c234(param_1: u16,uchar *param_2,param_3: i16,param_4: u16) -> *mut u8

{
  char *pcVar1;
  
  pass1_1010_c28a(param_2,param_3,param_4);
  if ((param_2 | param_1) == 0x0) {
    return 0x0;
  }
  pcVar1 = pass1_1038_4d28(CONCAT22(param_2,param_1));
  return pcVar1;
}



void 
pass1_1010_c25e(param_1: u16,param_2: u16,char *param_3,param_4: u16,uchar *param_5,
               param_6: i16,param_7: u16)

{
  if (param_3 != 0x0) {
    pass1_1010_c28a(param_5,param_6,param_7);
    if ((param_5 | param_4) != 0x0) {
      pass1_1038_4d3c(CONCAT22(param_5,param_4),param_3,param_5 | param_4);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_c28a(uchar *param_1,param_2: i16,param_3: u16)
{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let puVar5: *mut u16;
  
  puVar5 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_3,param_1,param_2);
  uVar2 = (puVar5 >> 0x10);
  uVar1 = (puVar5 + 0x24);
  uVar4 = (puVar5 + 0x26);
  uVar3 = uVar4 | uVar1;
  if (uVar3 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,uVar4);
    if ((uVar4 | uVar3) != 0x0) {
      return;
    }
  }
  return;
}



fn pass1_1010_c2d8(param_1: u16,param_2: u16,param_3: i32)
{
  let uVar1: u16;
  let uStack6: u16;
  
  if ((param_3 != 0x0) &&
     (uVar1 = (param_3 >> 0x10),
     uStack6 = (param_3 + 0x2e),
     ((param_3 + 0x30) | uStack6) != 0x0)) {
    return;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_c312(void) -> u32

{
  return CONCAT22((_PTR_LOOP_1050_65e2 + 0x2),*_PTR_LOOP_1050_65e2);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_c320(param_1: u16,param_2: u16,uchar *param_3,param_4: u32,uchar *param_5)
{
  let uVar1: u32;
  let puStack6: *mut u8
  
  puStack6 = param_3;
  if (param_3 == 0x0) {
    mem_op_1000_179c(0x100,param_5,0x1000);
    puStack6 = (param_3 & 0xffff | ZEXT24(param_5) << 0x10);
  }
  uVar1 = struct_op_1030_73a8(param_4);
  switch((uVar1 + 0x12)) {
  case 0x1:
  case 0x2:
  case 0x4:
    break;
  case 0x3:
  case 0x5:
    break;
  case 0x6:
    break;
  case 0x7:
    break;
  case 0x8:
    break;
  case 0x9:
    break;
  default:
    *puStack6 = '\0';
    return;
  }
  load_string_1010_84e0
            (0x1010,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,puStack6,
             (short)(puStack6 >> 0x10));
  return;
}



void 
pass1_1010_c3c2(param_1: u16,param_2: u16,param_3: u32,param_4: u32,uchar *param_5,
               param_6: u8,param_7: u16)

{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let local_40c: [u8;400];
  let uStack12: u16;
  let uStack10: u32;
  let uStack6: u32;
  
  uStack6 = param_3;
  if (param_3 == 0x0) {
    mem_op_1000_179c(0x100,param_5,0x1000);
    uStack6 = param_3 & 0xffff | ZEXT24(param_5) << 0x10;
  }
  uStack10 = struct_op_1030_73a8(param_4);
  uVar2 = (uStack10 >> 0x10);
  uStack12 = (uStack10 + 0xc);
  uVar3 = uVar2;
  uVar1 = pass1_1020_bd80(uStack12);
  unk_str_op_1000_3d3e(CONCAT22(param_7,local_40c),CONCAT22(uVar3,uVar1));
  pass1_1030_8086(param_4);
  sys_1000_3f9c((uchar *)uStack6,(uStack6 >> 0x10),0x38c5,
                ctx.data_seg,local_40c,&stack0xfffe,uVar2,0x1000,
                param_7,param_6);
  return;
}


void 
pass1_1010_c58as(param_1: u16,param_2: u16,param_3: u16,param_4: u32,param_5: u16
                ,param_6: u16)

{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u32;
  let puVar4: *mut u8
  let puVar5: *mut u8
  let unaff_SS: u16;
  let piStack26: *mut i16;
  let uStack18: u16;
  let iStack16: i16;
  u8 **ppuStack14;
  let piStack6: *mut i16;
  
  uVar3 = param_5;
  mem_op_1000_179c(0x18,param_6,0x1000);
  uVar1 = uVar3;
  puVar4 = (param_6 | uVar1);
  if (puVar4 == 0x0) {
    uVar1 = 0x0;
    puVar4 = 0x0;
  }
  else {
    struct_1040_a598((u16 *)(uVar3 & 0xffff | param_6 << 0x10));
  }
  piStack6 = CONCAT22(puVar4,uVar1);
  puVar5 = (puVar4 | uVar1);
  if (puVar5 == 0x0) {
    return;
  }
  ppuStack14 = (u8 **)0x0;
  uStack18 = 0x0;
  iStack16 = 0x0;
  if (true) {
    switch(param_3) {
    case 0x5:
      ppuStack14 = (u8 **)&USHORT_1050_352c;
      uStack18 = 0xfa4;
      iStack16 = 0x30;
      break;
    default:
      goto switchD_1010_c717_caseD_6;
    case 0xa:
      ppuStack14 = (u8 **)&USHORT_1050_358c;
      uStack18 = 0xfb3;
      iStack16 = 0x51;
      break;
    case 0xb:
      ppuStack14 = (u8 **)&USHORT_1050_358c;
      uStack18 = 0xfb4;
      iStack16 = 0x51;
      break;
    case 0xc:
      ppuStack14 = (u8 **)&USHORT_1050_362e;
      uStack18 = 0xfb6;
      iStack16 = 0x30;
      break;
    case 0x10:
      ppuStack14 = &ctx.PTR_DAT_1050_1805_1050_368e;
      uStack18 = 0xfc4;
      iStack16 = 0x3c;
      break;
    case 0x11:
      ppuStack14 = &ctx.PTR_DAT_1050_1805_1050_3706;
      uStack18 = 0xfc1;
      iStack16 = 0x4b;
      break;
    case 0x12:
      ppuStack14 = (u8 **)&USHORT_1050_379c;
      uStack18 = 0xfc5;
      iStack16 = 0x8;
      break;
    case 0x13:
      puVar5 = puVar4;
      pass1_1010_debe(CONCAT22(param_2,param_1),param_3,
                      CONCAT22(puVar4,uVar1 + 0x10),
                      CONCAT22(puVar4,uVar1 + 0xc),param_4,unaff_SS);
      ppuStack14 = (u8 **)&USHORT_1050_37ac;
      uStack18 = 0xfc6;
      iStack16 = 0x1;
      break;
    case 0x15:
      (uVar1 + 0x6) = param_4;
      (uVar1 + 0xa) = param_3;
      break;
    case 0x16:
      ppuStack14 = (u8 **)&USHORT_1050_37ae;
      uStack18 = 0x157;
      iStack16 = 0x4;
      break;
    case 0x17:
      ppuStack14 = (u8 **)&USHORT_1050_37b6;
      iStack16 = 0x2c;
      uStack18 = 0xfd8;
    }
    if (ppuStack14 != (u8 **)0x0) {
      *piStack6 = iStack16;
      uVar2 = iStack16 * 0xa + 0x2;
      mem_op_1000_179c(uVar2,puVar5,0x1000);
      piStack26 = CONCAT22(puVar5,uVar2);
      if ((puVar5 | uVar2) == 0x0) {
        (uVar1 + 0x2) = 0x0;
      }
      else {
        *piStack26 = iStack16;
        pass1_1000_5586((uchar *)0xa564,&ctx.PTR_LOOP_1050_1040,iStack16,0xa,
                        uVar2 + 0x2,puVar5);
        (uVar1 + 0x2) = uVar2 + 0x2;
        *(uchar **)(uVar1 + 0x4) = puVar5;
      }
      (uVar1 + 0x6) = param_4;
      (uVar1 + 0xa) = param_3;
      (uVar1 + 0x12) = uStack18;
      pass1_1010_a50c((astruct_20 *)CONCAT22(param_2,param_1),ppuStack14,
                      CONCAT22(puVar4,uVar1));
    }
    return;
  }
switchD_1010_c717_caseD_6:
  if (piStack6 != 0x0) {
    pass1_1040_a5d0(CONCAT22(puVar4,uVar1));
    fn_ptr_1000_17ce((astruct_18 *)CONCAT22(puVar4,uVar1),0x1000);
  }
  return;
}



fn pass1_1010_c7e2(param_1: u32,param_2: u32,i16 *param_3)
{
  let uVar1: u32;
  char *pcVar2;
  let in_DX: u16;
  let iVar3: i16;
  let unaff_SI: i16;
  let uVar4: u16;
  let puStack8: *mut u16;
  let iStack4: i16;
  
  iStack4 = 0x0;
  while( true ) {
    uVar4 = (param_3 >> 0x10);
    iVar3 = param_3;
    if (*param_3 == iStack4 || *param_3 < iStack4) break;
    uVar1 = (iVar3 + 0x2);
    (iStack4 * 0xa + uVar1 + 0x4) =
         (iStack4 * 0x2 + param_2);
    iStack4 += 0x1;
  }
  puStack8 = (iVar3 + 0x2);
  for (iStack4 = 0x0; *param_3 != iStack4 && iStack4 <= *param_3; iStack4 += 0x1) {
    uVar1 = (iVar3 + 0x6);
    pcVar2 = pass1_1010_b038((uchar *)param_1,uVar1,(uVar1 >> 0x10)
                             ,*(uchar **)(puStack8 + 0x4),unaff_SI);
    string_1040_a626(puStack8,CONCAT22(in_DX,pcVar2),in_DX);
    puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0xa));
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1010_c864(param_1: u32,param_2: *mut u16,astruct_104 *param_3,uchar *param_4,
               uchar *param_5,param_6: u8)

{
  long *plVar1;
  let lVar2: i32;
  code **ppcVar3;
  let uVar4: u32;
  char *pcVar5;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u32;
  let uVar9: u16;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar10: u16;
  let iVar11: i16;
  let iVar12: i16;
  let uVar13: u16;
  let uVar14: u16;
  let uVar15: u16;
  let uVar16: u16;
  let uVar17: u16;
  let local_1f0: u32;
  astruct_18 *paStack412;
  let uStack408: u32;
  let uStack404: u32;
  let uStack400: u16;
  let uStack398: u32;
  let local_18a: u32;
  let local_f6: u32;
  let puStack98: *mut u16;
  let iStack94: i16;
  let uStack92: u32;
  let iStack88: i16;
  let uStack86: u16;
  let local_54: [u8;52];
  
  uStack86 = 0x0;
  uVar13 = (param_3 >> 0x10);
  iVar11 = param_3;
  iStack88 = param_3;
  uVar14 = 0x0;
  uStack92 = 0x0;
  uVar16 = param_1;
  uVar17 = (param_1 >> 0x10);
  pass1_1010_c320(uVar16,uVar17,0x0,(iVar11 + 0x6),param_4);
  unk_str_op_1000_3d3e
            (CONCAT22(param_5,local_54),CONCAT22(param_4,uVar14));
  puStack98 = (iVar11 + 0x2);
  uStack404._2_2_ = (iVar11 + 0x4);
  (puStack98 + 0x4) = *param_2;
  string_1040_a626(puStack98,CONCAT22(param_5,local_54),uStack404._2_2_);
  iStack94 = 0x0;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_f6),(WNDCLASS16 *)0x0,0x94);
  uStack404._0_2_ =
       pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_18a),(WNDCLASS16 *)0x0,0x94);
  uStack398 = 0x0;
  for (uStack400 = 0x1; uStack400 < 0x25; uStack400 += 0x1) {
    uStack404 = (astruct_18 *)
                pass1_1030_7c28((iVar11 + 0x6),uStack400,uStack404
                                ,uStack404._2_2_,param_5);
    uStack404._2_2_ = (uStack404 >> 0x10) | uStack404;
    if (uStack404 != (astruct_18 *)0x0) {
      pcVar5 = string_1020_c0d8(uStack400);
      uStack408 = CONCAT22(uStack404._2_2_,pcVar5);
      uVar9 = uStack404._2_2_ | pcVar5;
      if (uVar9 == 0x0) {
        unk_str_op_1000_3d3e((&local_f6)[uStack398],s_Null_Ptr_1050_38e1);
      }
      else {
        uVar6 = str_op_1008_60e8(CONCAT22(uStack404._2_2_,pcVar5),uVar9);
        (&local_f6 + uStack398) = uVar6;
        (&local_f6 + uStack398 * 0x4 + 0x2) = uVar9;
      }
      (&local_18a + uStack398) = uStack404;
      (&local_18a + uStack398 * 0x4 + 0x2) = uStack404._2_2_;
      uStack398 += 0x1;
    }
  }
  uStack92 = uStack398;
  if (0x13 < uStack398) {
    iStack94 = 0x1;
  }
  uStack86 = pass1_1010_db2e(uVar16,uVar17,0x1,CONCAT22(param_5,&local_f6),
                             CONCAT22(param_5,&local_18a),param_2,param_3,
                             param_5,param_6);
  while (uVar8 = uStack398 - 0x1, uStack398 != 0x0) {
    uStack398._0_2_ = uVar8;
    paStack412 = (astruct_18 *)(&local_f6)[uStack398];
    uStack404 = paStack412;
    uStack398 = uVar8;
    fn_ptr_1000_17ce(paStack412,0x1000);
  }
  uVar15 = 0x1000;
  uStack398 = uVar8;
  pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_18a),(WNDCLASS16 *)0x0,0x54);
  uVar4 = (iVar11 + 0x6);
  uVar14 = (uVar4 >> 0x10);
  iVar12 = uVar4;
  uStack404 = (astruct_18 *)(iVar12 + 0x1e);
  uVar9 = (iVar12 + 0x20) | uStack404;
  uVar8 = uVar9;
  if (uVar9 != 0x0) {
    uStack398 = 0x0;
    while( true ) {
      uVar4 = uStack404;
      ppcVar3 = (code **)(uVar4 + 0x10);
      (**ppcVar3)(uVar15,uStack404,(uStack404 >> 0x10));
      if ((extraout_DX < uStack398._2_2_) ||
         ((extraout_DX <= uStack398._2_2_ && (uVar8 <= uStack398)))) break;
      ppcVar3 = (code **)(uVar4 + 0x4);
      (**ppcVar3)(uVar15,uStack404,uStack398,uStack398._2_2_);
      uVar9 = uVar8;
      uVar10 = extraout_DX_00 | uVar9;
      if (uVar10 != 0x0) {
        uVar15 = SUB42(&USHORT_1050_1028,0x0);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar9,extraout_DX_00);
        uStack408 = CONCAT22(uVar10,uVar9);
        if ((uVar10 | uVar9) == 0x0) {
          return;
        }
        iVar12 = (uVar9 + 0xc);
        uVar8 = (iVar12 - 0x1);
        if (((0x0 < iVar12) && (!SBORROW2(iVar12,0x1))) &&
           (uVar8 = (iVar12 - 0x6),
           iVar12 - 0x6 == 0x0 || (iVar12 - 0x1) < 0x5)) {
          plVar1 = &local_18a + iVar12;
          *plVar1 = *plVar1 + 0x1;
        }
      }
      uStack398 += 0x1;
    }
    uVar9 = extraout_DX;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_f6),(WNDCLASS16 *)0x0,0x54);
    uVar6 = 0x1000;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_1f0),(WNDCLASS16 *)0x0,0x54);
    uStack398 = 0x0;
    for (uStack400 = 0x1; uStack400 < 0x15; uStack400 += 0x1) {
      if ((&local_18a)[uStack400] != 0x0) {
        pcVar5 = string_op_1020_c222(uStack400);
        uVar10 = uVar9 | pcVar5;
        if (uVar10 == 0x0) {
          uVar6 = 0x1000;
          unk_str_op_1000_3d3e((&local_f6)[uStack398],s_Null_Ptr_1050_38ea);
        }
        else {
          uVar6 = 0x1008;
          uVar7 = str_op_1008_60e8(CONCAT22(uVar9,pcVar5),uVar10);
          (&local_f6 + uStack398) = uVar7;
          (&local_f6 + uStack398 * 0x4 + 0x2) = uVar10;
        }
        uVar9 = (&local_18a + uStack400 * 0x4 + 0x2);
        (&local_1f0 + uStack398) =
             (&local_18a + uStack400);
        (&local_1f0 + uStack398 * 0x4 + 0x2) = uVar9;
        uStack398 += 0x1;
      }
    }
    if (iStack94 == 0x0) {
      iVar12 = (uStack92 >> 0x10) + CARRY2(uStack92,uStack398);
      uStack92 = CONCAT22(iVar12,uStack92 + uStack398);
      if ((iVar12 != 0x0) || (0x13 < uStack92 + uStack398)) {
        iStack94 = 0x1;
      }
    }
    if ((uStack86 < iStack88 - 0x2) && (local_1f0 != 0x0)) {
      iVar12 = string_1010_dcac(uVar6,uVar16,uVar17,uStack86,param_2,param_3);
      uStack86 = iVar12 + 0x1;
      uStack86 = pass1_1010_db2e(uVar16,uVar17,uStack86,CONCAT22(param_5,&local_f6),
                                 CONCAT22(param_5,&local_1f0),param_2,
                                 param_3,param_5,param_6);
    }
    while (lVar2 = uStack398 + -0x1, uStack398 != 0x0) {
      uStack398._0_2_ = lVar2;
      paStack412 = (astruct_18 *)(&local_f6)[uStack398];
      uStack398 = lVar2;
      fn_ptr_1000_17ce(paStack412,0x1000);
    }
    if (iStack94 != 0x0) {
      (iVar11 + 0x16) = 0x1;
    }
    uStack398 = lVar2;
    pass1_1010_dc36(uVar16,uVar17,uStack86,param_2,param_3,param_5)
    ;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1010_cc56(param_1: u32,param_2: u32,astruct_104 *param_3,param_4: u16,
               uchar *param_5,param_6: u8)

{
  long *plVar1;
  let uVar2: u32;
  char *pcVar3;
  let uVar4: u16;
  let uVar5: u16;
  let iVar6: i16;
  let uVar7: u32;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let local_1a0: u32;
  let uStack332: u32;
  let uStack328: u16;
  let uStack326: u16;
  let uStack324: u32;
  let uStack320: u16;
  let local_13e: u32;
  let local_aa: u32;
  let uStack22: u16;
  let iStack18: i16;
  let uStack16: u16;
  let iStack14: i16;
  let uStack12: u16;
  let uStack10: u32;
  let uStack6: u32;
  
  uVar10 = (param_1 >> 0x10);
  uVar9 = param_1;
  uVar2 = (uVar9 + 0x138);
  uVar7 = (uVar2 + 0x24);
  uStack6 = uVar7;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar7,(uVar7 >> 0x10));
  uStack10 = uVar7 & 0xffff | param_4 << 0x10;
  uStack324._2_2_ = param_4 | uVar7;
  if (uStack324._2_2_ != 0x0) {
    iStack14 = param_3;
    iStack18 = 0x0;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_aa),(WNDCLASS16 *)0x0,0x94);
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_13e),(WNDCLASS16 *)0x0,0x94);
    uStack12 = 0x0;
    uStack16 = 0x0;
    uStack22 = 0x0;
    for (uStack320 = 0x1; uStack320 < 0x25; uStack320 += 0x1) {
      uStack324 = *(long *)(uStack10 + 0x26 + uStack320 * 0x4);
      if (uStack324 != 0x0) {
        pcVar3 = string_1020_c0d8(uStack320);
        uStack332 = uStack332 & 0xffff | ZEXT24(pcVar3) << 0x10;
        uVar8 = uStack324._2_2_ | pcVar3;
        uStack328 = uStack324._2_2_;
        if (uVar8 == 0x0) {
          unk_str_op_1000_3d3e((&local_aa)[uStack22],s_Null_Ptr_1050_38f3);
        }
        else {
          uVar4 = str_op_1008_60e8(CONCAT22(uStack324._2_2_,pcVar3),uVar8);
          (&local_aa + uStack22) = uVar4;
          (&local_aa + uStack22 * 0x4 + 0x2) = uVar8;
        }
        (&local_13e + uStack22) = uStack324;
        (&local_13e + uStack22 * 0x4 + 0x2) = uStack324._2_2_;
        uStack22 += 0x1;
      }
    }
    uStack16 = uStack22;
    if (0x13 < uStack22) {
      iStack18 = 0x1;
    }
    uStack12 = pass1_1010_db2e(uVar9,uVar10,0x1,CONCAT22(param_5,&local_aa),
                               CONCAT22(param_5,&local_13e),param_2,param_3,param_5
                               ,param_6);
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_13e),(WNDCLASS16 *)0x0,0x54);
    for (uStack332._0_2_ = 0x1; uStack332 < 0x15; uStack332 += 0x1) {
      uStack326 = uStack332;
      if (*(long *)(uStack10 + 0x14e + uStack332 * 0x4) != 0x0) {
        if (((0x0 < uStack332) && (!SBORROW2(uStack332,0x1))) &&
           ((uStack332 - 0x1) < 0x6)) {
          plVar1 = &local_13e + uStack332;
          *plVar1 = *plVar1 + 0x1;
        }
      }
    }
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_aa),(WNDCLASS16 *)0x0,0x54);
    uVar4 = 0x1000;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_1a0),(WNDCLASS16 *)0x0,0x54);
    for (uStack332 = 0x10000; uStack332._2_2_ < 0x15;
        uStack332 = uStack332 & 0xffff | (uStack332._2_2_ + 0x1) << 0x10) {
      if ((&local_13e)[uStack332._2_2_] != 0x0) {
        pcVar3 = string_op_1020_c222(uStack332._2_2_);
        uStack324 = CONCAT22(uStack324._2_2_,pcVar3);
        uVar8 = uStack324._2_2_ | pcVar3;
        if (uVar8 == 0x0) {
          uVar4 = 0x1000;
          unk_str_op_1000_3d3e((&local_aa)[uStack332],s_Null_Ptr_1050_38fc);
        }
        else {
          uVar4 = 0x1008;
          uVar5 = str_op_1008_60e8(CONCAT22(uStack324._2_2_,pcVar3),uVar8);
          (&local_aa + uStack332) = uVar5;
          (&local_aa + uStack332 * 0x4 + 0x2) = uVar8;
        }
        uStack324._2_2_ = (&local_13e + uStack332._2_2_ * 0x4 + 0x2);
        (&local_1a0 + uStack332) =
             (&local_13e + uStack332._2_2_);
        (&local_1a0 + uStack332 * 0x4 + 0x2) = uStack324._2_2_;
        uStack332 = uStack332 & 0xffff0000 | (uStack332 + 0x1);
      }
    }
    if (iStack18 == 0x0) {
      uStack16 += uStack332;
      if (0x13 < uStack16) {
        iStack18 = 0x1;
      }
    }
    if ((uStack12 < iStack14 - 0x2) && (local_1a0 != 0x0)) {
      iVar6 = string_1010_dcac(uVar4,uVar9,uVar10,uStack12,param_2,param_3);
      uStack12 = iVar6 + 0x1;
      uStack12 = pass1_1010_db2e(uVar9,uVar10,uStack12,CONCAT22(param_5,&local_aa),
                                 CONCAT22(param_5,&local_1a0),param_2,param_3,
                                 param_5,param_6);
    }
    if (iStack18 != 0x0) {
      (param_3 + 0x16) = 0x1;
    }
    pass1_1010_dc36(uVar9,uVar10,uStack12,param_2,param_3,param_5);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1010_cf36(param_1: u32,param_2: u32,param_3: *mut u16,param_4: u8,uchar *param_5)
{
  let uVar1: u32;
  let puVar2: *mut u16;
  char *pcVar3;
  let uVar4: u16;
  let uVar5: u16;
  let in_DX: u16;
  let uVar6: u16;
  let unaff_SI: i16;
  let iVar7: i16;
  astruct_494 *iVar9;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u32;
  let uVar12: u32;
  let uVar13: u16;
  let uVar14: u16;
  let uVar15: u16;
  let uStack326: u16;
  let iStack316: i16;
  let uStack314: u16;
  let iStack312: i16;
  u16 local_136 [0x4a];
  let local_a2: u32;
  let iStack14: i16;
  let uStack12: u32;
  let puStack8: *mut u16;
  let iStack4: i16;
  
  iStack4 = 0x0;
  do {
    uVar8 = (param_2 >> 0x10);
    iVar7 = param_2;
    uVar9 = (param_3 >> 0x10);
    iVar9 = (astruct_494 *)param_3;
    puVar2 = iVar9->field_0x2;
    (iStack4 * 0xa + puVar2 + 0x4) =
         (iStack4 * 0x2 + iVar7);
    iStack4 += 0x1;
  } while (iStack4 < 0x8);
  puStack8 = iVar9->field_0x2;
  iStack4 = 0x0;
  do {
    uVar1 = iVar9->field_0x6;
    pcVar3 = pass1_1010_b038((uchar *)param_1,uVar1,(uVar1 >> 0x10),
                             *(uchar **)(puStack8 + 0x4),unaff_SI);
    string_1040_a626(puStack8,CONCAT22(in_DX,pcVar3),in_DX);
    iStack4 += 0x1;
    puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0xa));
  } while (iStack4 < 0x8);
  uVar13 = param_1;
  uVar14 = (param_1 >> 0x10);
  struct_1010_dd5e(uVar13,uVar14,iVar9->field_0x6);
  uStack12 = CONCAT22(in_DX,pcVar3);
  in_DX |= pcVar3;
  if (in_DX != 0x0) {
    iStack14 = 0x0;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,&local_a2),(WNDCLASS16 *)0x0,0x94);
    pass1_1000_4906((astruct_20 *)CONCAT22(param_5,local_136),(WNDCLASS16 *)0x0,0x94);
    uStack314 = 0x0;
    iStack312 = 0x0;
    iStack316 = 0x0;
    uVar1 = iVar9->field_0x6;
    uVar1 = (uVar1 + 0x26);
    uVar12 = uVar1;
    for (uStack326 = 0x1; uStack326 < 0x25; uStack326 += 0x1) {
      uVar15 = (uVar1 >> 0x10);
      if (*(long *)(uStack326 * 0x4 + uStack12) == 0x0) {
        if (uVar1 != 0x0) {
          uVar12 = pass1_1020_bae6(uVar1,CONCAT22(uStack326,uVar15),uVar12,
                                   in_DX,param_5);
          uVar6 = (uVar12 >> 0x10);
          uVar12 &= 0xffff;
          in_DX = uVar6 | uVar12;
          if (in_DX != 0x0) {
            if (iStack14 == 0x0) {
              iStack14 = 0x1;
            }
            pcVar3 = string_1020_c0d8(uStack326);
            uVar4 = in_DX | pcVar3;
            if (uVar4 == 0x0) {
              unk_str_op_1000_3d3e((&local_a2)[iStack312],s_Null_Ptr_1050_390e);
            }
            else {
              uVar5 = str_op_1008_60e8(CONCAT22(in_DX,pcVar3),uVar4);
              (&local_a2 + iStack312) = uVar5;
              (&local_a2 + iStack312 * 0x4 + 0x2) = uVar4;
            }
            local_136[iStack312 * 0x2] = uVar12;
            local_136[iStack312 * 0x2 + 0x1] = uVar6;
            goto LAB_1010_d11d;
          }
        }
      }
      else {
        if (iStack14 == 0x0) {
          iStack14 = 0x1;
        }
        pcVar3 = string_1020_c0d8(uStack326);
        uVar6 = in_DX | pcVar3;
        if (uVar6 == 0x0) {
          unk_str_op_1000_3d3e((&local_a2)[iStack312],s_Null_Ptr_1050_3905);
        }
        else {
          uVar5 = str_op_1008_60e8(CONCAT22(in_DX,pcVar3),uVar6);
          (&local_a2 + iStack312) = uVar5;
          (&local_a2 + iStack312 * 0x4 + 0x2) = uVar6;
        }
        uVar10 = (uStack12 >> 0x10);
        uVar4 = (uStack326 * 0x4 + uStack12);
        uVar6 = (uStack326 * 0x4 + uStack12 + 0x2);
        local_136[iStack312 * 0x2] = uVar4;
        local_136[iStack312 * 0x2 + 0x1] = uVar6;
        if (uVar1 == 0x0) {
          uVar4 = 0x0;
        }
        else {
          uVar11 = pass1_1020_bae6(uVar1,CONCAT22(uStack326,uVar15),uVar4,uVar6,
                                   param_5);
          uVar6 = (uVar11 >> 0x10);
          uVar4 = uVar11;
        }
        uVar12 = uVar4;
        if (uVar4 == 0x0) {
          iStack316 += 0x2;
          in_DX = uVar6;
          iStack312 = iStack312 + 0x1;
        }
        else {
//LAB_1010_d11d:
          iStack312 += 0x1;
          (uVar13 + uStack314 * 0x2 + 0xa4) =
               (iVar7 + iStack316 * 0x2 + 0x10);
          (uVar13 + (uStack314 + 0x1) * 0x2 + 0xa4) =
               (iVar7 + (iStack316 + 0x1) * 0x2 + 0x10);
          iStack316 += 0x2;
          uStack314 += 0x2;
          uVar12 = uStack314;
          in_DX = uVar6;
        }
      }
    }
    uVar6 = pass1_1010_db2e(uVar13,uVar14,0x8,CONCAT22(param_5,&local_a2),
                            CONCAT22(param_5,local_136),param_2,param_3,param_5,
                            param_4);
    if (iStack14 != 0x0) {
      iVar9->field_0x16 = 0x1;
    }
    while (iStack312 != 0x0) {
      fn_ptr_1000_17ce((astruct_18 *)(&local_a2)[iStack312 + -0x1],0x1000);
      iStack312 = iStack312 + -0x1;
    }
    pass1_1010_dc36(uVar13,uVar14,uVar6,param_2,param_3,param_5);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1010_d24a(param_1: u32,param_2: u32,param_3: *mut u16,uchar *param_4,param_5: u8)
{
  let uVar1: u32;
  let puVar2: *mut u16;
  char *pcVar3;
  let puVar4: *mut u16;
  let uVar5: u16;
  let in_DX: u16;
  let puVar6: *mut u16;
  let puVar7: *mut u16;
  let uVar8: u16;
  let unaff_SI: i16;
  astruct_495 *iVar9;
  let uVar9: u16;
  let lVar10: i32;
  let uVar11: u16;
  let uVar12: u16;
  let uStack320: u16;
  let lStack318: i32;
  u16 *local_13a [0x4a];
  let local_a6: u32;
  let iStack18: i16;
  let uStack16: u32;
  char *pcStack12;
  let puStack8: *mut u16;
  let iStack4: i16;
  
  iStack4 = 0x0;
  do {
    uVar9 = (param_3 >> 0x10);
    iVar9 = (astruct_495 *)param_3;
    puVar2 = iVar9->field_0x2;
    (iStack4 * 0xa + puVar2 + 0x4) =
         (iStack4 * 0x2 + param_2);
    iStack4 += 0x1;
  } while (iStack4 < 0x8);
  puStack8 = iVar9->field_0x2;
  iStack4 = 0x0;
  do {
    uVar1 = iVar9->field_0x6;
    pcVar3 = pass1_1010_b038((uchar *)param_1,uVar1,(uVar1 >> 0x10),
                             *(uchar **)(puStack8 + 0x4),unaff_SI);
    string_1040_a626(puStack8,CONCAT22(in_DX,pcVar3),in_DX);
    iStack4 += 0x1;
    puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0xa));
  } while (iStack4 < 0x8);
  uVar11 = param_1;
  uVar12 = (param_1 >> 0x10);
  struct_1010_dd5e(uVar11,uVar12,iVar9->field_0x6);
  puVar6 = (in_DX | pcVar3);
  if (puVar6 != 0x0) {
    pcStack12 = pcVar3;
    pass1_1010_e8f6(uVar11,uVar12,iVar9->field_0x6,param_4);
    uStack16 = CONCAT22(puVar6,pcVar3);
    iStack18 = 0x0;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_4,&local_a6),(WNDCLASS16 *)0x0,0x94);
    puVar4 = pass1_1000_4906((astruct_20 *)CONCAT22(param_4,local_13a),(WNDCLASS16 *)0x0,
                             0x94);
    lStack318 = 0x0;
    for (uStack320 = 0x1; uStack320 < 0x25; uStack320 += 0x1) {
      lVar10 = pass1_1030_7c28(uStack16,uStack320,puVar4,puVar6,
                               param_4);
      puVar7 = (lVar10 >> 0x10);
      puVar4 = lVar10;
      puVar6 = (puVar7 | puVar4);
      if (lVar10 != 0x0) {
        if (iStack18 == 0x0) {
          iStack18 = 0x1;
        }
        pcVar3 = string_1020_c0d8(uStack320);
        uVar8 = puVar6 | pcVar3;
        if (uVar8 == 0x0) {
          unk_str_op_1000_3d3e((&local_a6)[lStack318],s_Null_Ptr_1050_3917);
        }
        else {
          uVar5 = str_op_1008_60e8(CONCAT22(puVar6,pcVar3),uVar8);
          (&local_a6 + lStack318) = uVar5;
          (&local_a6 + lStack318 * 0x4 + 0x2) = uVar8;
        }
        local_13a[lStack318 * 0x2] = puVar4;
        local_13a[lStack318 * 0x2 + 0x1] = puVar7;
        lStack318 += 0x1;
        puVar6 = puVar7;
      }
    }
    uVar8 = pass1_1010_db2e(uVar11,uVar12,0x8,CONCAT22(param_4,&local_a6),
                            CONCAT22(param_4,local_13a),param_2,param_3,param_4,
                            param_5);
    if (iStack18 != 0x0) {
      iVar9->field_0x16 = 0x1;
    }
    while (lStack318 != 0x0) {
      lStack318._0_2_ = (lStack318 + -0x1);
      fn_ptr_1000_17ce((astruct_18 *)(&local_a6)[lStack318],0x1000);
      lStack318 = lStack318 + -0x1;
    }
    pass1_1010_dc36(uVar11,uVar12,uVar8,param_2,param_3,param_4);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

void 
pass1_1010_d448(uchar *param_1,param_2: u32,param_3: *mut u16,uchar *param_4,param_5: u8,
               param_6: i16)

{
  let uVar1: u16;
  let puVar2: *mut u16;
  let uVar3: u32;
  let uVar4: u32;
  let puVar5: *mut u16;
  char *pcVar6;
  let iVar7: i16;
  let uVar8: u16;
  let uVar9: u16;
  let iVar10: i16;
  let uVar11: u16;
  let uVar12: u32;
  let uVar13: u16;
  let local_40c: u16;
  let uStack1034: u32;
  let uStack1030: u32;
  uchar local_402 [0x400];
  
  uVar11 = (param_3 >> 0x10);
  iVar10 = param_3;
  uStack1030 = struct_op_1030_73a8((iVar10 + 0x6));
  uVar8 = (uStack1030 >> 0x10);
  uVar1 = uStack1030;
  if ((uVar8 | uVar1) != 0x0) {
    uStack1034 = (uVar1 + 0x20);
    uVar1 = (uVar1 + 0x22);
    if ((uVar1 | uStack1034) != 0x0) {
      local_40c = 0x0;
      puVar5 = &local_40c;
      uVar13 = (param_1 >> 0x10);
      pass1_1010_d984(param_1,uVar13,CONCAT22(param_4,puVar5),0x3,
                      uStack1034 & 0xffff | uVar1 << 0x10,
                      &ctx.PTR_DAT_1050_1805_1050_368e,param_3,param_4,param_5);
      puVar2 = (iVar10 + 0x2);
      uVar9 = (iVar10 + 0x4);
      (puVar2 + 0x4) = ctx.PTR_DAT_1050_1805_1050_368e;
      uVar3 = (iVar10 + 0x6);
      pcVar6 = pass1_1010_b038(param_1,uVar3,(uVar3 >> 0x10),
                               *(uchar **)(puVar2 + 0x4),param_6);
      unk_str_op_1000_3d3e
                (CONCAT22(param_4,local_402),CONCAT22(uVar9,pcVar6));
      string_1040_a626(puVar2,CONCAT22(param_4,local_402),uVar9);
      uVar4 = (iVar10 + 0x2);
      uVar9 = (iVar10 + 0x4);
      iVar7 = uVar4;
      (iVar7 + 0xe) = ctx.PTR_DAT_1050_1822_1050_3690;
      sys_1000_3f9c(local_402,param_4,0x3920,ctx.data_seg,local_40c,
                    &stack0xfffe,uVar9,0x1000,param_4,param_5);
      string_1040_a626((u16 *)(uVar4 & 0xffff0000 | (iVar7 + 0xa)),
                       CONCAT22(param_4,local_402),uVar9);
      uVar4 = (iVar10 + 0x2);
      uVar11 = (iVar10 + 0x4);
      iVar10 = uVar4;
      (iVar10 + 0x18) = ctx.PTR_DAT_1050_1823_1050_3692;
      uVar12 = pass1_1028_62c8(uStack1030,param_4);
      uVar9 = (uVar12 >> 0x10);
      sys_1000_3f9c(local_402,param_4,0x3923,ctx.data_seg,uVar12,
                    &stack0xfffe,uVar11,0x1000,param_4,param_5);
      string_1040_a626((u16 *)(uVar4 & 0xffff0000 | (iVar10 + 0x14)),
                       CONCAT22(param_4,local_402),uVar9);
      pass1_1010_dc36(param_1,uVar13,puVar5,param_2,param_3,param_4)
      ;
    }
  }
  return;
}



void 
pass1_1010_d5ae(uchar *param_1,param_2: u32,param_3: *mut u16,uchar *param_4,param_5: u8,
               param_6: i16)

{
  let puVar1: *mut u16;
  let uVar2: u32;
  let uVar3: u32;
  let puVar4: *mut u8;
  let puVar5: *mut u16;
  char *pcVar6;
  let iVar7: i16;
  let uVar8: u16;
  let iVar9: i16;
  let uVar10: u16;
  let uVar11: u32;
  let uVar12: u16;
  let local_40c: u16;
  let uStack1034: u16;
  let uStack1032: u16;
  let uStack1030: u32;
  uchar local_402 [0x400];
  
  uVar10 = (param_3 >> 0x10);
  iVar9 = param_3;
  uStack1030 = struct_op_1030_73a8((iVar9 + 0x6));
  uStack1034 = uStack1030;
  uStack1032 = (uStack1030 >> 0x10) | uStack1034;
  if (uStack1032 != 0x0) {
    pass1_1028_45fe(uStack1030,uStack1034,param_4);
    if ((uStack1032 | uStack1034) != 0x0) {
      local_40c = 0x0;
      puVar5 = &local_40c;
      uVar12 = (param_1 >> 0x10);
      pass1_1010_d984(param_1,uVar12,CONCAT22(param_4,puVar5),0x3,
                      CONCAT22(uStack1032,uStack1034),&ctx.PTR_DAT_1050_1805_1050_3706,
                      param_3,param_4,param_5);
      puVar1 = (iVar9 + 0x2);
      uVar8 = (iVar9 + 0x4);
      (puVar1 + 0x4) = ctx.PTR_DAT_1050_1805_1050_3706;
      uVar2 = (iVar9 + 0x6);
      pcVar6 = pass1_1010_b038(param_1,uVar2,(uVar2 >> 0x10),
                               *(uchar **)(puVar1 + 0x4),param_6);
      unk_str_op_1000_3d3e
                (CONCAT22(param_4,local_402),CONCAT22(uVar8,pcVar6));
      string_1040_a626(puVar1,CONCAT22(param_4,local_402),uVar8);
      uVar3 = (iVar9 + 0x2);
      uVar8 = (iVar9 + 0x4);
      iVar7 = uVar3;
      (iVar7 + 0xe) = ctx.PTR_DAT_1050_1822_1050_3708;
      sys_1000_3f9c(local_402,param_4,0x3926,ctx.data_seg,local_40c,
                    &stack0xfffe,uVar8,0x1000,param_4,param_5);
      string_1040_a626((u16 *)(uVar3 & 0xffff0000 | (iVar7 + 0xa)),
                       CONCAT22(param_4,local_402),uVar8);
      puVar4 = ctx.PTR_DAT_1050_1823_1050_370a;
      uVar3 = (iVar9 + 0x2);
      iVar9 = (iVar9 + 0x4);
      iVar7 = uVar3;
      (iVar7 + 0x18) = ctx.PTR_DAT_1050_1823_1050_370a;
      uVar11 = pass1_1028_45e2(uStack1030,puVar4,iVar9,param_4);
      uVar8 = (uVar11 >> 0x10);
      sys_1000_3f9c(local_402,param_4,0x3929,ctx.data_seg,uVar11,
                    &stack0xfffe,iVar9,0x1000,param_4,param_5);
      string_1040_a626((u16 *)(uVar3 & 0xffff0000 | (iVar7 + 0x14)),
                       CONCAT22(param_4,local_402),uVar8);
      pass1_1010_dc36(param_1,uVar12,puVar5,param_2,param_3,param_4)
      ;
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1010_d710(param_1: u32,param_2: u32,param_3: *mut u16,uchar *param_4,param_5: u8)
{
  let uVar1: u32;
  let lVar2: i32;
  let puVar3: *mut u16;
  char *pcVar4;
  let iVar5: i16;
  let uVar6: u16;
  let in_DX: u16;
  let uVar7: u16;
  let unaff_SI: i16;
  let iVar8: i16;
  astruct_496 *iVar9;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  let uVar12: u32;
  let uVar13: u16;
  let uVar14: u16;
  let uStack322: u16;
  let iStack316: i16;
  let iStack314: i16;
  let iStack312: i16;
  u16 local_136 [0x4a];
  let local_a2: u32;
  let iStack14: i16;
  let uStack12: u32;
  let puStack8: *mut u16;
  let iStack4: i16;
  
  iStack4 = 0x0;
  do {
    uVar9 = (param_2 >> 0x10);
    iVar8 = param_2;
    uVar10 = (param_3 >> 0x10);
    iVar9 = (astruct_496 *)param_3;
    puVar3 = iVar9->field_0x2;
    (iStack4 * 0xa + puVar3 + 0x4) =
         (iStack4 * 0x2 + iVar8);
    iStack4 += 0x1;
  } while (iStack4 < 0x4);
  puStack8 = iVar9->field_0x2;
  iStack4 = 0x0;
  do {
    uVar1 = iVar9->field_0x6;
    pcVar4 = pass1_1010_b038((uchar *)param_1,uVar1,(uVar1 >> 0x10),
                             *(uchar **)(puStack8 + 0x4),unaff_SI);
    string_1040_a626(puStack8,CONCAT22(in_DX,pcVar4),in_DX);
    iStack4 += 0x1;
    puStack8 = (puStack8 & 0xffff0000 | (puStack8 + 0xa));
  } while (iStack4 < 0x4);
  uVar13 = param_1;
  uVar14 = (param_1 >> 0x10);
  struct_1010_dd5e(uVar13,uVar14,iVar9->field_0x6);
  uStack12 = CONCAT22(in_DX,pcVar4);
  in_DX |= pcVar4;
  if (in_DX != 0x0) {
    iStack14 = 0x0;
    pass1_1000_4906((astruct_20 *)CONCAT22(param_4,&local_a2),(WNDCLASS16 *)0x0,0x94);
    pass1_1000_4906((astruct_20 *)CONCAT22(param_4,local_136),(WNDCLASS16 *)0x0,0x94);
    iStack314 = 0x0;
    iStack312 = 0x0;
    iStack316 = 0x0;
    uVar1 = iVar9->field_0x6;
    lVar2 = *(long *)(uVar1 + 0x26);
    for (uStack322 = 0x1; uStack322 < 0x25; uStack322 += 0x1) {
      if (*(long *)(uStack322 * 0x4 + uStack12) != 0x0) {
        if (iStack14 == 0x0) {
          iStack14 = 0x1;
        }
        pcVar4 = string_1020_c0d8(uStack322);
        uVar7 = in_DX | pcVar4;
        if (uVar7 == 0x0) {
          unk_str_op_1000_3d3e((&local_a2)[iStack312],s_Null_Ptr_1050_392c);
        }
        else {
          uVar6 = str_op_1008_60e8(CONCAT22(in_DX,pcVar4),uVar7);
          (&local_a2 + iStack312) = uVar6;
          (&local_a2 + iStack312 * 0x4 + 0x2) = uVar7;
        }
        uVar11 = (uStack12 >> 0x10);
        uVar7 = (uStack322 * 0x4 + uStack12);
        in_DX = (uStack322 * 0x4 + uStack12 + 0x2);
        local_136[iStack312 * 0x2] = uVar7;
        local_136[iStack312 * 0x2 + 0x1] = in_DX;
        iStack312 += 0x1;
        if (lVar2 == 0x0) {
          iVar5 = 0x0;
        }
        else {
          uVar12 = pass1_1020_bae6(lVar2,
                                   CONCAT22(uStack322,(lVar2 >> 0x10)),uVar7,
                                   in_DX,param_4);
          in_DX = (uVar12 >> 0x10);
          iVar5 = uVar12;
        }
        if (iVar5 == 0x0) {
          iStack316 += 0x2;
        }
        else {
          (uVar13 + iStack314 * 0x2 + 0xa4) =
               (iVar8 + iStack316 * 0x2 + 0x8);
          (uVar13 + (iStack314 + 0x1) * 0x2 + 0xa4) =
               (iVar8 + (iStack316 + 0x1) * 0x2 + 0x8);
          iStack316 += 0x2;
          iStack314 += 0x2;
        }
      }
    }
    uVar7 = pass1_1010_db2e(uVar13,uVar14,0x4,CONCAT22(param_4,&local_a2),
                            CONCAT22(param_4,local_136),param_2,param_3,param_4,
                            param_5);
    if (iStack14 != 0x0) {
      iVar9->field_0x16 = 0x1;
    }
    while (iStack312 != 0x0) {
      fn_ptr_1000_17ce((astruct_18 *)(&local_a2)[iStack312 + -0x1],0x1000);
      iStack312 = iStack312 + -0x1;
    }
    pass1_1010_dc36(uVar13,uVar14,uVar7,param_2,param_3,param_4);
  }
  return;
}



void 
pass1_1010_d984(param_1: u16,param_2: u16,i16 *param_3,param_4: i16,param_5: u32,
               param_6: u32,param_7: u32,uchar *param_8,param_9: u8)

{
  let puVar1: *mut u8;
  char *pcVar2;
  let iVar3: i16;
  let extraout_DX: u16;
  let uVar4: u16;
  let uVar5: u16;
  let iVar6: i16;
  let iVar7: i16;
  let uVar8: u16;
  let uVar9: u16;
  let puStack1052: *mut u16;
  uchar local_418 [0x400];
  let uStack24: u16;
  char *pcStack22;
  let uStack18: u16;
  let uStack16: u32;
  let local_c: [u8;8];
  let iStack4: i16;
  
  iStack4 = param_4;
  pass1_1008_5784(CONCAT22(param_8,local_c),param_5);
  do {
    puVar1 = local_c;
    pass1_1008_5b12(puVar1,param_8);
    uStack16 = CONCAT22(extraout_DX,puVar1);
    uVar4 = extraout_DX | puVar1;
    if (uVar4 == 0x0) {
      return;
    }
    uStack18 = (puVar1 + 0xa);
    pcStack22 = 0x0;
    if ((puVar1 + 0x4) == 0x0) {
      if ((puVar1 + 0x6) == 0x0) {
        if ((puVar1 + 0x8) == 0x0) {
          return;
        }
        pcVar2 = string_op_1020_c2f8((puVar1 + 0x8));
      }
      else {
        pcVar2 = string_op_1020_c222((puVar1 + 0x6));
      }
    }
    else {
      pcVar2 = string_1020_c0d8((puVar1 + 0x4));
    }
    pcStack22 = CONCAT22(uVar4,pcVar2);
    uStack24 = (uStack16 + 0xc);
    *param_3 = *param_3 + uStack24;
    uVar8 = (param_7 >> 0x10);
    iVar6 = param_7;
    uVar5 = (iVar6 + 0x4);
    iVar3 = (iVar6 + 0x2) + iStack4 * 0xa;
    puStack1052 = CONCAT22(uVar5,iVar3);
    uVar9 = (param_6 >> 0x10);
    iVar7 = param_6;
    (iVar3 + 0x4) = (iStack4 * 0x2 + iVar7);
    sys_1000_3f9c(local_418,param_8,0x3935,ctx.data_seg,uStack18,&stack0xfffe
                  ,uVar5,0x1000,param_8,param_9);
    string_1040_a626(puStack1052,CONCAT22(param_8,local_418),uVar5);
    uVar5 = (iVar6 + 0x4);
    iStack4 += 0x1;
    iVar3 = (iVar6 + 0x2) + iStack4 * 0xa;
    puStack1052 = CONCAT22(uVar5,iVar3);
    (iVar3 + 0x4) = (iStack4 * 0x2 + iVar7);
    unk_str_op_1000_3d3e(CONCAT22(param_8,local_418),pcStack22);
    string_1040_a626(puStack1052,CONCAT22(param_8,local_418),uVar5);
    iVar3 = (iStack4 + 0x1) * 0xa + (iVar6 + 0x2);
    uVar5 = (iVar6 + 0x4);
    puStack1052 = CONCAT22(uVar5,iVar3);
    (iVar3 + 0x4) = ((iStack4 + 0x1) * 0x2 + iVar7);
    iStack4 += 0x2;
    sys_1000_3f9c(local_418,param_8,0x3938,ctx.data_seg,uStack24,&stack0xfffe
                  ,uVar5,0x1000,param_8,param_9);
    string_1040_a626(puStack1052,CONCAT22(param_8,local_418),uVar5);
  } while( true );
}



u16 
pass1_1010_db2e(param_1: u16,param_2: u16,param_3: u16,param_4: u32,param_5: u32,
               param_6: u32,i16 *param_7,uchar *param_8,param_9: u8)

{
  let uVar1: u16;
  astruct_493 *iVar2;
  let iVar3: i16;
  let uVar4: u16;
  astruct_492 *iVar4;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uStack94: u16;
  let iStack92: i16;
  let uStack90: u16;
  let puStack88: *mut u16;
  uchar local_54 [0x52];
  
  uStack94 = param_3;
  uStack90 = param_3;
  iStack92 = 0x0;
  while( true ) {
    uVar7 = (param_7 >> 0x10);
    iVar4 = (astruct_492 *)param_7;
    if (*param_7 - 0x1 < uStack94) {
      return uStack94;
    }
    uVar1 = iVar4->field_0x4;
    iVar2 = (astruct_493 *)(iVar4->field_0x2 + uStack94 * 0xa);
    uVar5 = (param_5 >> 0x10);
    uVar6 = (param_4 >> 0x10);
    if ((*(long *)(iStack92 * 0x4 + param_5) == 0x0) &&
       (*(long *)(iStack92 * 0x4 + param_4) == 0x0)) break;
    uVar4 = uVar1;
    unk_str_op_1000_3d3e
              (CONCAT22(param_8,local_54),
               (iStack92 * 0x4 + param_4));
    uVar6 = (param_6 >> 0x10);
    iVar2->field_0x4 = (uStack90 * 0x2 + param_6);
    string_1040_a626((u16 *)CONCAT22(uVar1,iVar2),CONCAT22(param_8,local_54),
                     uVar4);
    sys_1000_3f9c(local_54,param_8,0x393b,ctx.data_seg,
                  (param_5 + iStack92 * 0x4),&stack0xfffe,
                  uVar5,0x1000,param_8,param_9);
    uVar1 = iVar4->field_0x4;
    iVar3 = iVar4->field_0x2 + (uStack94 + 0x1) * 0xa;
    puStack88 = CONCAT22(uVar1,iVar3);
    (iVar3 + 0x4) = ((uStack90 + 0x1) * 0x2 + param_6);
    string_1040_a626(puStack88,CONCAT22(param_8,local_54),uVar1);
    uStack94 += 0x2;
    uStack90 += 0x2;
    iStack92 += 0x1;
  }
  return uStack94;
}



void 
pass1_1010_dc36(param_1: u16,param_2: u16,param_3: u16,param_4: u32,param_5: *mut u16,
               param_6: u16)

{
  let puVar1: u32;
  let uVar2: u16;
  let uVar3: u32;
  let iVar4: i16;
  let uVar5: u16;
  let puVar6: u32;
  let uVar7: u16;
  let uStack90: u16;
  let local_54: *mut u8;
  let local_52: [u32;0x14];
  
  local_54 = ctx.PTR_s_New_failed_in_Op_Op_1050_0020_1050_393f;
  puVar6 = local_52;
  for (iVar4 = 0x13; iVar4 != 0x0; iVar4 += -0x1) {
    puVar1 = puVar6;
    puVar6 = puVar6 + 0x1;
    *puVar1 = 0x0;
  }
  puVar6 = 0x0;
  *(puVar6 + 0x2) = 0x0;
  uStack90 = param_3;
  while( true ) {
    uVar7 = (param_5 >> 0x10);
    if (*param_5 < uStack90 || *param_5 == uStack90) break;
    uVar3 = (param_5 + 0x2);
    uVar2 = (param_5 + 0x4);
    uVar5 = uVar3 + uStack90 * 0xa;
    (uVar5 + 0x4) = (uStack90 * 0x2 + param_4);
    uStack90 += 0x1;
    string_1040_a626((u16 *)(uVar3 & 0xffff0000 | uVar5),
                     CONCAT22(param_6,&local_54),uVar2);
  }
  return;
}


fn pass1_1010_de78(param_1: u32,param_2: u32)
{
  short in_buf_len_5;
  
  in_buf_len_5 = (short)(param_1 >> 0x10);
  *(param_1 + 0x13c) = 0x0;
  pass1_1030_809c(param_2);
  load_string_1010_84e0
            (0x1030,_PTR_LOOP_1050_14cc,
             (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,
             (param_1 + 0x13c),in_buf_len_5);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1010_debe(param_1: u32,param_2: u16,param_3: *mut u16,param_4: *mut u32,param_5: u32,
               param_6: u16)

{
  let bVar1: u8;
  let uVar2: u16;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let puVar6: *mut u8
  let iVar7: i16;
  let unaff_DI: i16;
  let uVar8: u16;
  let uVar9: u32;
  let puVar10: *mut u16;
  let uVar11: u16;
  let iStack34: i16;
  let uStack30: u16;
  let iStack26: i16;
  let iStack24: i16;
  let iStack22: i16;
  let iStack20: i16;
  
  *param_4 = 0x0;
  *param_3 = 0x0;
  uVar9 = struct_op_1030_73a8(param_5);
  puVar6 = (uVar9 >> 0x10);
  iVar4 = (uVar9 + 0x12);
  uVar5 = param_1;
  uVar11 = (param_1 >> 0x10);
  uVar2 = pass1_1010_b028(uVar5,uVar11,uVar9);
  puVar10 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x35,param_6,puVar6,unaff_DI);
  puVar6 = (puVar10 >> 0x10);
  iVar7 = param_4;
  uVar8 = (param_4 >> 0x10);
  if (param_2 == 0x13) {
    iStack34 = 0x0;
    while (iStack34 += 0x1, iStack34 < 0x43) {
      param_2 = pass1_1010_ac62(uVar5,uVar11,iStack34,param_2,puVar6);
      if (param_2 != 0x0) {
        *param_3 = *param_3 + 0x1;
      }
    }
    uVar2 = *param_3 * 0x2;
    mem_op_1000_179c(uVar2,puVar6,0x1000);
    param_4 = uVar2;
    *(uchar **)(iVar7 + 0x2) = puVar6;
    if ((puVar6 | param_4) != 0x0) {
      iStack34 = 0x0;
      for (uStack30 = 0x0; uVar2 = uStack30,
          *param_3 != uStack30 && uStack30 <= *param_3; uStack30 += 0x1) {
        do {
          iStack34 += 0x1;
          if (0x42 < iStack34) goto LAB_1010_e0d4;
          uVar2 = pass1_1010_ac62(uVar5,uVar11,iStack34,uVar2,puVar6);
        } while (uVar2 == 0x0);
        (uStack30 * 0x2 + *param_4) = iStack34;
//LAB_1010_e0d4:
      }
    }
  }
  else {
    if (param_2 < 0x14) {
      if (param_2 == '\x06') {
        if (((iVar4 == 0x5) || (iVar4 == 0x6)) || (iVar4 == 0x8)) {
          uVar3 = puVar10 + 0x11e;
          if (uVar2 == 0xf) {
            iStack20 = 0xf;
            iStack22 = 0x13;
          }
          else {
            if (uVar2 == 0xe) {
              iStack22 = 0x4;
              iStack20 = 0x1;
            }
            else {
              iStack22 = 0xe;
              iStack20 = 0x1;
            }
          }
          iVar4 = pass1_1010_e128(uVar5,uVar11,iStack22,iStack20,
                                  puVar10 & 0xffff0000 | uVar3);
          *param_3 = iVar4 + 0x1;
          if (iVar4 + 0x1 != 0x0) {
            uVar2 = *param_3 * 0x2;
            mem_op_1000_179c(uVar2,puVar6,0x1000);
            param_4 = uVar2;
            *(uchar **)(iVar7 + 0x2) = puVar6;
            iStack24 = 0x0;
            for (iStack26 = iStack20; iStack26 <= iStack22; iStack26 += 0x1) {
              if ((iStack26 * 0x2 + uVar3) != 0x0) {
                (*param_4 + iStack24 * 0x2) = iStack26;
                iStack24 += 0x1;
              }
            }
            (*param_4 + iStack24 * 0x2) = 0x14;
            return;
          }
        }
      }
      else {
        bVar1 = param_2 - 0x7;
        if ((bVar1 == 0x0) && (((iVar4 == 0x5 || (iVar4 == 0x6)) || (iVar4 == 0x8)))) {
          uVar5 = pass1_1010_ac62(uVar5,uVar11,0x7,param_2 & 0xff00 | bVar1,
                                  puVar6);
          uVar2 = -(uVar5 == 0x0) + 0x10;
          *param_3 = uVar2;
          uVar2 *= 0x2;
          mem_op_1000_179c(uVar2,puVar6,0x1000);
          param_4 = uVar2;
          *(uchar **)(iVar7 + 0x2) = puVar6;
          if ((puVar6 | param_4) == 0x0) {
            *param_3 = 0x0;
            return;
          }
          for (iStack26 = 0x0; iStack26 < (-(uVar5 == 0x0) + 0xf);
              iStack26 += 0x1) {
            (iStack26 * 0x2 + *param_4) = iStack26 + 0x1;
          }
          (iStack26 * 0x2 + *param_4) = 0x10;
          return;
        }
      }
    }
  }
  return;
}



i16 
pass1_1010_e128(param_1: u16,param_2: u16,param_3: i16,param_4: i16,param_5: u32)

{
  let iStack6: i16;
  let iStack4: i16;
  
  iStack4 = 0x0;
  for (iStack6 = param_4; iStack6 <= param_3; iStack6 += 0x1) {
    if ((iStack6 * 0x2 + param_5) != 0x0) {
      iStack4 += 0x1;
    }
  }
  return iStack4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_e15e(param_1: u32,param_2: u16,param_3: u16,param_4: u16,param_5: u16)
{
  code **ppcVar1;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u32;
  let extraout_DX: u16;
  let extraout_DX_00: u16;
  let uVar5: u16;
  let uStack18: u32;
  let uStack14: u32;
  let puStack10: u32;
  
  pass1_1010_afde(param_1,0xc);
  puStack10 = CONCAT22(param_3,param_2);
  ppcVar1 = (code **)(*puStack10 + 0x10);
  uVar2 = param_2;
  (**ppcVar1)(param_4,param_2,param_3);
  uStack14 = CONCAT22(extraout_DX,uVar2);
  for (uStack18 = 0x0; uStack18 < uStack14; uStack18 += 0x1) {
    ppcVar1 = (code **)(*puStack10 + 0x4);
    uVar4 = uStack14;
    (**ppcVar1)(param_4,param_2,param_3,uStack18,(uStack18 >> 0x10));
    uVar3 = uVar4;
    uVar5 = extraout_DX_00;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,extraout_DX_00);
    param_4 = 0x1030;
    pass1_1030_7c28(CONCAT13((uVar5 >> 0x8),CONCAT12(uVar5,uVar3)),0x23,uVar3,
                    uVar5,param_5);
  }
  if (puStack10 != 0x0) {
    ppcVar1 = (code **)*puStack10;
    (**ppcVar1)(param_4,param_2,param_3,0x1);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_e1f4(param_1: u32,param_2: u32,param_3: u16,param_4: u16)
{
  let uVar1: u16;
  let BVar2: bool;
  char *pcVar3;
  let uVar4: u16;
  let uVar5: u16;
  let iVar6: i16;
  short in_buf_len_5;
  let uVar7: u32;
  
  in_buf_len_5 = (short)(param_1 >> 0x10);
  iVar6 = param_1;
  *(iVar6 + 0x13c) = 0x0;
  uVar7 = struct_op_1030_73a8(param_2);
  uVar5 = (uVar7 >> 0x10);
  uVar1 = (uVar7 + 0xc);
  BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0xc);
  if ((((((((BVar2 == 0x0) &&
           (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x14), BVar2 == 0x0)) &&
          (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0xa), BVar2 == 0x0)) &&
         ((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x15), BVar2 == 0x0 &&
          (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0xb), BVar2 == 0x0)))) &&
        (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x16), BVar2 == 0x0)) &&
       (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x17), BVar2 == 0x0 &&
         (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x21), BVar2 == 0x0)) &&
        ((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1c), BVar2 == 0x0 &&
         (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1d), BVar2 == 0x0 &&
           (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x18), BVar2 == 0x0)) &&
          (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x19), BVar2 == 0x0))))))))
      && ((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x4), BVar2 == 0x0 &&
          (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x3), BVar2 == 0x0)))) &&
     (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1e), BVar2 == 0x0 &&
       (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x23), BVar2 == 0x0 &&
         (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1b), BVar2 == 0x0)) &&
        ((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1f), BVar2 == 0x0 &&
         (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x1), BVar2 == 0x0 &&
           (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x2), BVar2 == 0x0)) &&
          (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x13), BVar2 == 0x0))))))))
      && (((BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x20), BVar2 == 0x0 &&
           (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0xe), BVar2 == 0x0)) &&
          (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x10), BVar2 == 0x0)))))) {
    pcVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x12);
    if ((pcVar3 == 0x0) &&
       (pcVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x11),
       pcVar3 == 0x0)) {
      BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x6);
      if (BVar2 == 0x0) {
        BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x5);
        if (BVar2 == 0x0) {
          pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x40);
          goto LAB_1010_e241;
        }
        uVar4 = pass1_1030_7f98(param_2);
        pcVar3 = string_op_1020_c222(uVar4);
      }
      else {
        uVar4 = pass1_1030_7f5a(param_2);
        pcVar3 = string_op_1020_c2f8(uVar4);
      }
    }
    else {
      pass1_1010_e58a(param_1,uVar7,uVar5,param_3,param_4);
    }
    unk_str_op_1000_3d3e
              ((param_1 & 0xffff0000 | (iVar6 + 0x13c)),
               CONCAT22(uVar5,pcVar3));
  }
  else {
//LAB_1010_e241:
    load_string_1010_84e0
              (0x1008,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,(iVar6 + 0x13c),
               in_buf_len_5);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_e58a(param_1: u32,param_2: u32,uchar *param_3,param_4: i16,param_5: u16)
{
  code **ppcVar1;
  let uVar2: u16;
  let BVar3: bool;
  let puVar4: u32;
  let extraout_DX: u16;
  let uVar5: u16;
  let extraout_DX_00: u16;
  let extraout_DX_01: u16;
  let iVar6: i16;
  short in_buf_len_5;
  let uVar7: u16;
  let puVar8: u32;
  
  in_buf_len_5 = (short)(param_1 >> 0x10);
  iVar6 = param_1;
  *(iVar6 + 0x13c) = 0x0;
  uVar7 = (param_2 >> 0x10);
  puVar4 = (param_2 + 0x20);
  uVar7 = (param_2 + 0xc);
  BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar7,0x11);
  if (BVar3 == 0x0) {
    BVar3 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar7,0x12);
    if (BVar3 == 0x0) {
      return;
    }
    puVar8 = 
             mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x31,param_5,param_3,param_4);
    ppcVar1 = (code **)(*puVar8 + 0x14);
    (**ppcVar1)(0x1008,puVar8,(puVar8 >> 0x10),puVar4,puVar4 >> 0xf)
    ;
    uVar5 = extraout_DX_01 | puVar4;
    uVar2 = extraout_DX_01;
  }
  else {
    puVar8 = 
             mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x41,param_5,param_3,param_4);
    ppcVar1 = (code **)(*puVar8 + 0x14);
    (**ppcVar1)(0x1008,puVar8,(puVar8 >> 0x10),puVar4,puVar4 >> 0xf)
    ;
    uVar5 = extraout_DX | puVar4;
    uVar2 = extraout_DX;
  }
  if (uVar5 == 0x0) {
    load_string_1010_84e0
              (0x1008,_PTR_LOOP_1050_14cc,
               (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,(iVar6 + 0x13c),
               in_buf_len_5);
  }
  else {
    ppcVar1 = (code **)(*puVar4 + 0x14);
    (**ppcVar1)(0x1008,puVar4,uVar2);
    unk_str_op_1000_3d3e
              ((param_1 & 0xffff0000 | (iVar6 + 0x13c)),
               CONCAT22(extraout_DX_00,puVar4));
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_e682(param_1: u32,param_2: u32,param_3: u16,param_4: u8)
{
  let uVar1: u16;
  let BVar2: bool;
  let uVar3: u32;
  let uVar4: u16;
  let uVar5: u16;
  let in_buf_len_5: *mut u8
  let uVar6: u16;
  let uVar7: u32;
  let uVar8: u16;
  let uVar9: u16;
  let local_1e: u16;
  let uStack28: u16;
  let local_1a: u16;
  let uStack24: u16;
  let uStack22: u16;
  let uStack20: u32;
  let uStack16: u32;
  let uStack12: u32;
  let uStack8: u16;
  let uStack6: u32;
  
  in_buf_len_5 = (param_1 >> 0x10);
  uVar5 = param_1;
  *(uVar5 + 0x13c) = 0x0;
  uStack6 = struct_op_1030_73a8(param_2);
  uVar6 = (uStack6 >> 0x10);
  uStack8 = (uStack6 + 0xc);
  uVar4 = uVar6;
  uVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x1);
  if (((uVar1 == 0x0) &&
      (uVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x13), uVar1 == 0x0)) &&
     (uVar1 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x2), uVar1 == 0x0)) {
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0xe);
    if (BVar2 != 0x0) {
      uVar7 = (uVar5 + 0x138);
      uVar3 = (uVar7 + 0x24);
      uStack16 = uVar3;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3,(uVar3 >> 0x10));
      uStack12 = uVar3 & 0xffff | uVar4 << 0x10;
      uStack20 = (uVar3 + 0x1f6);
      uVar6 = (uStack20 >> 0x10);
      uVar8 = (uStack20 + 0x1a8);
      uVar9 = 0x3947;
      uStack22 = uVar8;
//LAB_1010_e76d:
      sys_1000_3f9c((uchar *)(uVar5 + 0x13c),in_buf_len_5,uVar9,ctx.data_seg,
                    uVar8,&stack0xfffe,uVar6,0x1000,param_3,param_4);
      return;
    }
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x5);
    if ((BVar2 == 0x0) &&
       (BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x6), BVar2 == 0x0)) {
      BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x10);
      if (BVar2 == 0x0) {
        local_1e = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0xc);
        if ((local_1e == 0x0) &&
           (local_1e = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x14), local_1e == 0x0)
           ) {
          BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0xa);
          if (BVar2 == 0x0) {
            uVar8 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uStack8,0x1e);
            if (uVar8 == 0x0) {
              load_string_1010_84e0
                        (0x1008,_PTR_LOOP_1050_14cc,
                         (_PTR_LOOP_1050_14cc >> 0x10),0x3ff,
                         (uVar5 + 0x13c),(short)in_buf_len_5);
              return;
            }
            pass1_1030_6ddc(param_2);
            uVar9 = 0x395c;
            local_1e = uVar8;
            goto LAB_1010_e76d;
          }
          uVar7 = pass1_1030_7c28(param_2,0x21,BVar2,uVar4,param_3);
          uStack28 = (uVar7 >> 0x10);
          uVar1 = uVar7;
          uVar8 = 0x3958;
          local_1e = uVar1;
        }
        else {
          pass1_1010_e8f6(uVar5,in_buf_len_5,param_2,param_3);
          uStack28 = uVar4;
          uVar7 = pass1_1030_7c28(CONCAT22(uVar4,local_1e),0x23,local_1e,uVar4,param_3);
          uStack24 = (uVar7 >> 0x10);
          uVar1 = uVar7;
          uVar8 = 0x3954;
          local_1a = uVar1;
        }
      }
      else {
        uVar7 = pass1_1030_7c28(param_2,0x1e,BVar2,uVar4,param_3);
        uStack28 = (uVar7 >> 0x10);
        uVar1 = uVar7;
        uVar8 = 0x3950;
        local_1e = uVar1;
      }
    }
    else {
      local_1e = 0x0;
      local_1a = 0x0;
      pass1_1010_e8d0(uVar5,in_buf_len_5,CONCAT22(param_3,&local_1a),
                      CONCAT22(param_3,&local_1e),param_2,&local_1e);
      uVar8 = 0x394a;
      uVar1 = local_1e;
    }
  }
  else {
    pass1_1010_e8f6(uVar5,in_buf_len_5,param_2,param_3);
    uStack12 = CONCAT22(uVar4,uVar1);
    pass1_1030_70f4(CONCAT22(uVar4,uVar1));
    uStack16 = CONCAT22(uVar4,uVar1);
    uVar8 = 0x3943;
  }
  sys_1000_3f9c((uchar *)(uVar5 + 0x13c),in_buf_len_5,uVar8,ctx.data_seg,
                uVar1,&stack0xfffe,uVar6,0x1000,param_3,param_4);
  return;
}



void 
pass1_1010_e8d0(param_1: u16,param_2: u16,param_3: *mut u16,param_4: *mut u16,
               param_5: u32,param_6: u16)

{
  pass1_1030_7064(param_5);
  *param_4 = param_6;
  pass1_1030_70ac(param_5);
  *param_3 = param_6;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_e8f6(param_1: u16,param_2: u16,param_3: u32,param_4: u16)
{
  let uVar1: u16;
  let BVar2: bool;
  let uVar3: u16;
  let uVar4: u32;
  
  uVar4 = struct_op_1030_73a8(param_3);
  uVar1 = (uVar4 + 0xc);
  BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x13);
  if (BVar2 == 0x0) {
    BVar2 = pass1_1008_c6ae(_PTR_LOOP_1050_06e0,uVar1,0x14);
    if (BVar2 == 0x0) {
      return;
    }
    uVar4 = pass1_1028_4faa(uVar4,param_4);
    uVar3 = (uVar4 >> 0x10);
    uVar1 = uVar4;
  }
  else {
    uVar4 = pass1_1028_121e(uVar4,param_4);
    uVar3 = (uVar4 >> 0x10);
    uVar1 = uVar4;
  }
  pass1_1028_b58e(CONCAT22(uVar3,uVar1));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_e964(uchar *param_1,param_2: u16,param_3: i16)
{
  let uVar1: u32;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  puVar3 = mixed_1010_20ba(_PTR_LOOP_1050_0ed0,0x2f,param_2,param_1,param_3);
  uVar2 = (puVar3 >> 0x10);
  uVar1 = (puVar3 + 0x24);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
  pass1_1038_4d28(uVar1 & 0xffff | uVar2 << 0x10);
  return;
}



fn pass1_1010_e99a(param_1: u32,param_2: u8) -> u32

{
  let unaff_SS: u16;
  
  param_1 = param_1 & 0xffff0000 | (param_1 - 0xa);
  pass1_1010_a478((u16 *)param_1,unaff_SS);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce((astruct_18 *)param_1,0x1000);
  }
  return param_1;
}


fn pass1_1010_eb66(param_1: *mut u16,param_2: u16)
{
  let puVar1: u32;
  let uVar2: u16;
  code **ppcVar3;
  let puVar4: *mut u16;
  astruct_499 *iVar5;
  let uVar5: u16;
  let puStack14: *mut u16;
  
  uVar5 = (param_1 >> 0x10);
  iVar5 = (astruct_499 *)param_1;
  *param_1 = 0x558;
  iVar5->field_0x2 = 0x1018;
  iVar5->field_0xa = 0x568;
  iVar5->field_0xc = 0x1018;
  puVar1 = iVar5->field_0xe;
  uVar2 = iVar5->field_0x10;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1018_04f2(param_1);
  fn_ptr_1000_17ce((astruct_18 *)iVar5->field_0x2c,0x1000);
  if (param_1 == 0x0) {
    puVar4 = 0x0;
    uVar5 = 0x0;
  }
  else {
    puVar4 = &iVar5->field_0xa;
  }
  puStack14 = CONCAT22(uVar5,puVar4);
  *puStack14 = 0x389a;
  puVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1,param_2);
  return;
}



fn pass1_1010_ebf8(param_1: u32,param_2: u16,param_3: u16,param_4: i16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  (param_1 + param_4 * 0x4 + 0x34) = param_2;
  (param_1 + param_4 * 0x4 + 0x36) = param_3;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_ec18(param_1: u16,param_2: u16,param_3: u32,param_4: i16,param_5: u16) -> u32

{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_3,(param_3 >> 0x10));
  return CONCAT22((param_4 + 0x12),(param_4 + 0x10));
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_ec40(param_1: u16,param_2: u16,param_3: u32,param_4: i16,param_5: u16) -> u32

{
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_3,(param_3 >> 0x10));
  return CONCAT22((param_4 + 0x12),(param_4 + 0x10));
}



fn pass1_1010_ec68(param_1: u32,param_2: u32,param_3: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x28) = param_2;
  pass1_1010_1f62(param_3,param_1 & 0xffff | uVar1 << 0x10,0x13);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_ec84(param_1: u32,param_2: u16,param_3: u8)
{
  u8 local_10e [0x10c];
  
  pass1_1010_1f62(param_2,param_1,0x14);
  pass1_1030_532e((astruct_100 *)CONCAT22(param_2,local_10e),
                  (param_1 + 0x20),param_2,param_3);
  fn_ptr_1030_835a(_PTR_LOOP_1050_5748,CONCAT22(param_2,local_10e));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void 
pass1_1010_ecc6(param_1: u32,param_2: *mut u16,param_3: i32,param_4: u16,param_5: u16,
               param_6: u16)

{
  let uVar1: u32;
  let iVar2: i16;
  let uVar3: u16;
  
  pass1_1030_627e(param_6,param_4,param_5,_PTR_LOOP_1050_5740,param_2,param_3);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4,param_5);
  uVar1 = (param_4 + 0x2e);
  uVar3 = (uVar1 >> 0x10);
  iVar2 = uVar1;
  if (*(long *)(iVar2 + 0x200) == 0x8000001) {
    pass1_1010_ed22(param_1,(iVar2 + 0x4),param_6);
  }
  return;
}



fn pass1_1010_ed22(param_1: u32,param_2: u32,param_3: u16)
{
  let uVar1: u16;
  
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x24) = param_2;
  pass1_1010_1f62(param_3,param_1 & 0xffff | uVar1 << 0x10,0x12);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1010_ed3e(param_1: u32)
{
  let uVar1: u32;
  
  uVar1 = (param_1 + 0x16);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar1,(uVar1 >> 0x10));
  return;
}
