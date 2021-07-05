
fn file_1010_0c7c(param_1: u32,param_2: u32,param_3: i16,uchar *param_4,param_5: u16)
{
  let puVar1: u32;
  code **ppcVar2;
  let BVar3: bool;
  astruct_229 *uVar4;
  let uVar5: u16;
  let extraout_DX: *mut u8
  astruct_228 *iVar6;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let local_2a: [u16;0x2];
  let uStack38: u16;
  let puStack26: u32;
  let puStack22: u32;
  let local_12: [u16;0x5];
  astruct_229 *paStack8;
  astruct_229 *local_6;
  let uStack4: u16;
  
  uVar7 = param_2;
  uVar8 = (param_2 >> 0x10);
  read_file_1008_7cfe(uVar7,uVar8,0x6,0x1008,param_5);
  if (param_3 == 0x0) {
    PTR_LOOP_1050_0310 = 0x6d4;
  }
  else {
    BVar3 = read_file_1008_7dee(uVar7,uVar8,&local_6,0x0,param_5,0x2,0x1008);
    if (BVar3 != 0x0) {
      paStack8 = (astruct_229 *)0x0;
      while( true ) {
        iVar6 = (astruct_228 *)param_1;
        uVar5 = (param_1 >> 0x10);
        if (local_6 <= paStack8) break;
        uVar4 = local_6;
        mem_op_1000_179c(0xa,param_4,0x1000);
        puStack26 = CONCAT22(param_4,uVar4);
        if ((param_4 | uVar4) == 0x0) {
          puStack22 = 0x0;
        }
        else {
          puStack26 = 0x389a;
          uVar4->field_0x2 = 0x1008;
          puStack26 = 0xea8;
          uVar4->field_0x2 = 0x1010;
          puStack22 = puStack26;
        }
        BVar3 = read_file_1008_7dee(uVar7,uVar8,local_12,0x0,param_5,0x2,0x1008);
        if ((BVar3 == 0x0) ||
           (BVar3 = read_file_1008_7dee(uVar7,uVar8,puStack22 + 0x6,0x0,
                                        (puStack22 >> 0x10),0x4,0x1008),
           BVar3 == 0x0)) {
          puStack26 = puStack22;
          if (puStack22 != 0x0) {
            ppcVar2 = (code **)*puStack22;
            (**ppcVar2)(0x1008,puStack22,(puStack22 >> 0x10),0x1);
          }
          goto LAB_1010_0cb1;
        }
        uVar6 = (puStack22 >> 0x10);
        (puStack22 + 0x4) = local_12[0];
        puVar1 = iVar6->field_0xa;
        ppcVar2 = (code **)(*iVar6->field_0xa + 0x8);
        (**ppcVar2)(0x8,puVar1,(puVar1 >> 0x10),puStack22,uVar6);
        paStack8 = (astruct_229 *)&paStack8->field_0x1;
        param_4 = extraout_DX;
      }
      BVar3 = read_file_1008_7dee(uVar7,uVar8,&iVar6->field_0xe,0x0,uVar5,0x2,0x1008);
      if ((BVar3 != 0x0) &&
         (BVar3 = read_file_1008_7dee(uVar7,uVar8,&iVar6->field_0x10,0x0,uVar5,0x2,0x1008)
         , BVar3 != 0x0)) {
        for (uStack4 = 0x0; uStack4 < 0xa; uStack4 += 0x1) {
          BVar3 = read_file_1008_7dee(uVar7,uVar8,local_2a,0x0,param_5,0x2,0x1008)
          ;
          if (BVar3 == 0x0) goto LAB_1010_0cb1;
          uVar5 = uStack4;
          if (PTR_LOOP_1050_0312 < 0x2) {
            uVar5 = pass1_1008_738c(uVar7,uVar8,uStack4);
          }
          (uVar5 * 0x8 + 0xe28) = local_2a[0];
          uStack38 = uVar5;
        }
        if (0x2 < PTR_LOOP_1050_0312) {
          uStack4 = 0x0;
          do {
            BVar3 = read_file_1008_7dee(uVar7,uVar8,local_2a,0x0,param_5,0x2,
                                        0x1008);
            if (BVar3 == 0x0) goto LAB_1010_0cb1;
            (uStack4 * 0x8 + 0xea8) = local_2a[0];
            uStack4 += 0x1;
          } while (uStack4 < 0x3);
        }
        return;
      }
    }
LAB_1010_0cb1:
    PTR_LOOP_1050_0310 = 0x6d2;
  }
  return;
}


fn write_to_file_1010_6372(param_1: u32,param_2: u32,param_3: u16)
{
  let BVar1: bool;
  astruct_729 *iVar2;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let local_10: [u32;0x2];
  let local_8: u32;
  
  BVar1 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar1 != 0x0) {
    uVar2 = (param_1 >> 0x10);
    iVar2 = (astruct_729 *)param_1;
    local_10[0] = iVar2->field_0xa;
    uVar3 = param_2;
    uVar4 = (param_2 >> 0x10);
    BVar1 = write_to_file_1008_7e1c
                      (uVar3,uVar4,local_10,param_3,0x4,0x1008);
    if (BVar1 != 0x0) {
      local_8 = iVar2->field_0xe;
      BVar1 = write_to_file_1008_7e1c
                        (uVar3,uVar4,&local_8,param_3,0x4,0x1008);
      if (BVar1 != 0x0) {
        local_8 = iVar2->field_0x12;
        BVar1 = write_to_file_1008_7e1c
                          (uVar3,uVar4,&local_8,param_3,0x4,0x1008);
        if (BVar1 != 0x0) {
          local_8 = iVar2->field_0x16;
          BVar1 = write_to_file_1008_7e1c
                            (uVar3,uVar4,&local_8,param_3,0x4,0x1008);
          if (BVar1 != 0x0) {
            local_8 = iVar2->field_0x1a;
            BVar1 = write_to_file_1008_7e1c
                              (uVar3,uVar4,&local_8,param_3,0x4,0x1008);
            if (BVar1 != 0x0) {
              local_8 = iVar2->field_0x1e;
              BVar1 = write_to_file_1008_7e1c
                                (uVar3,uVar4,&local_8,param_3,0x4,0x1008);
              if (BVar1 != 0x0) {
                local_8 = iVar2->field_0x22;
                BVar1 = write_to_file_1008_7e1c
                                  (uVar3,uVar4,&local_8,param_3,0x4,0x1008
                                  );
                if (BVar1 != 0x0) {
                  return;
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


fn write_to_file_1010_6846(param_1: u32,param_2: u32,param_3: u16)
{
  let uVar1: u16;
  let BVar2: bool;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let local_c: [u16;0x5];
  
  BVar2 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar2 != 0x0) {
    iVar3 = param_1;
    uVar1 = (param_1 >> 0x10);
    uVar4 = param_2;
    uVar5 = (param_2 >> 0x10);
    BVar2 = write_to_file_1008_7e1c(uVar4,uVar5,iVar3 + 0xa,uVar1,0x114,0x1008);
    if (BVar2 != 0x0) {
      BVar2 = write_to_file_1008_7e1c(uVar4,uVar5,iVar3 + 0x11e,uVar1,0x2a,0x1008)
      ;
      if (BVar2 != 0x0) {
        local_c[0] = (iVar3 + 0x148);
        BVar2 = write_to_file_1008_7e1c
                          (uVar4,uVar5,local_c,param_3,0x2,0x1008);
        if (BVar2 != 0x0) {
          return;
        }
      }
    }
    PTR_LOOP_1050_0310 = 0x6d0;
  }
  return;
}


astruct_43 * 
unk_io_op_1010_830a(param_1: u32,param_2: u16,param_3: u16)

{
  let in_AX: u16;
  let puVar1: u32;
  let puVar2: u32;
  let in_DX: *mut u8
  let uVar3: u16;
  astruct_45 *iVar3;
  astruct_44 *iVar2;
  let iVar4: i16;
  HINSTANCE16 unaff_CS;
  let uVar5: u16;
  let uVar6: u16;
  let local_2e: u32;
  let uStack10: u32;
  astruct_43 *paStack6;
  
  paStack6 = (astruct_43 *)0x0;
  iVar3 = (astruct_45 *)(param_2 * 0x10);
  uVar5 = param_1;
  uVar6 = (param_1 >> 0x10);
  if (iVar3->field_0x10 == 0x1) {
    uStack10 = set_err_mode_1010_8b14
                                 (param_1,*(ULONG *)&iVar3->field_0x12,param_3);
    uStack10._2_2_ = (uStack10 >> 0x10);
    if ((iVar3->field_0x12 == uStack10) && (iVar3->field_0x14 == uStack10._2_2_)) {
      msg_box_op_1010_8bb4(uVar5,uVar6,uStack10,unaff_CS,param_3);
      return (astruct_43 *)0x0;
    }
    puVar1 = &local_2e;
    struct_op_1008_48fe((astruct_81 *)CONCAT22(param_3,puVar1),0x1,uStack10,uStack10._2_2_
                       );
    mem_op_1000_179c(0x1e,(uStack10 >> 0x10),0x1000);
    uVar3 = (uStack10 >> 0x10) | puVar1;
    if (uVar3 == 0x0) {
      puVar2 = 0x0;
      uVar3 = 0x0;
    }
    else {
      puVar2 = &local_2e;
      struct_op_1008_3f92((astruct_76 *)(uStack10 & 0xffff0000 | ZEXT24(puVar1)),
                          (astruct_83 *)CONCAT22(param_3,puVar2));
    }
    paStack6 = (astruct_43 *)CONCAT22(uVar3,puVar2);
    close_file_1008_496c(&local_2e,param_3);
    local_2e = paStack6;
  }
  else {
    if ((param_2 * 0x10 + 0x10) == 0x2) {
      pass1_1010_878c((astruct_87 **)param_1,(param_2 * 0x10 + 0x16),unaff_CS);
      if (*(long *)(uVar5 + 0x67c) == 0x0) {
        return (astruct_43 *)0x0;
      }
      iVar2 = (astruct_44 *)(param_2 * 0x10);
      pass1_1008_6562(*(u32 **)(uVar5 + 0x67c),
                      CONCAT22(iVar2->field_0x1c,iVar2->field_0x1e),iVar2->field_0x1a,
                      in_AX,in_DX);
      local_2e = (astruct_43 *)CONCAT22(in_DX,in_AX);
    }
    else {
      iVar4 = param_2 * 0x10;
      if ((iVar4 + 0x10) == 0x3) {
        local_2e = (astruct_43 *)
                   set_err_mode_1010_8b14(param_1,*(ULONG *)(iVar4 + 0x12),param_3);
        if (((iVar4 + 0x12) == local_2e) &&
           ((iVar4 + 0x14) == (local_2e >> 0x10))) {
          msg_box_op_1010_8bb4(uVar5,uVar6,local_2e,unaff_CS,param_3);
          local_2e = local_2e;
        }
      }
      else {
        local_2e = paStack6;
        if ((param_2 * 0x10 + 0x10) == 0x4) {
          local_2e = (astruct_43 *)
                     set_err_mode_1010_8b14
                               (param_1,*(ULONG *)(param_2 * 0x10 + 0x12),param_3);
        }
      }
    }
  }
  paStack6 = local_2e;
  return paStack6;
}


fn write_to_file_1010_ed58(param_1: u32,param_2: u32,param_3: u16)
{
  let piVar1: *mut i16;
  let uVar2: u16;
  let BVar3: bool;
  let iVar4: i16;
  let puVar5: u32;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let local_22: u32;
  let uStack30: u16;
  let local_12: [u32;0x2];
  let local_a: u32;
  let iStack4: i16;
  
  BVar3 = write_to_file_1008_7cac(param_2,param_3);
  if (BVar3 != 0x0) {
    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    local_12[0] = (iVar6 + 0x16);
    uVar8 = param_2;
    uVar9 = (param_2 >> 0x10);
    BVar3 = write_to_file_1008_7e1c
                      (uVar8,uVar9,local_12,param_3,0x4,0x1008);
    if (BVar3 != 0x0) {
      local_a = (iVar6 + 0x1a);
      BVar3 = write_to_file_1008_7e1c
                        (uVar8,uVar9,&local_a,param_3,0x4,0x1008);
      if (BVar3 != 0x0) {
        local_a = (iVar6 + 0x20);
        BVar3 = write_to_file_1008_7e1c
                          (uVar8,uVar9,&local_a,param_3,0x4,0x1008);
        if (BVar3 != 0x0) {
          local_a = (iVar6 + 0x24);
          BVar3 = write_to_file_1008_7e1c
                            (uVar8,uVar9,&local_a,param_3,0x4,0x1008);
          if (BVar3 != 0x0) {
            local_a = local_a & 0xffff0000 | (iVar6 + 0x30);
            BVar3 = write_to_file_1008_7e1c
                              (uVar8,uVar9,&local_a,param_3,0x2,0x1008);
            if (BVar3 != 0x0) {
              local_a = local_a & 0xffff0000 | (iVar6 + 0x32);
              BVar3 = write_to_file_1008_7e1c
                                (uVar8,uVar9,&local_a,param_3,0x2,0x1008);
              if (BVar3 != 0x0) {
                iStack4 = 0x0;
                while( true ) {
                  piVar1 = (i16 *)(iVar6 + 0x30);
                  if (*piVar1 == iStack4 || *piVar1 < iStack4) {
                    return;
                  }
                  uVar2 = (iVar6 + 0x2e);
                  puVar5 = ((iVar6 + 0x2c) + iStack4 * 0x6);
                  local_22 = *puVar5;
                  uStack30 = (puVar5 + 0x1);
                  local_12[0] = local_12[0] & 0xffff0000 | ZEXT24(&local_22);
                  iVar4 = write_to_file_1008_7b4c
                                    (param_2,CONCAT22(param_3,&local_22),0x1008,param_3);
                  if (iVar4 == 0x0) break;
                  iStack4 += 0x1;
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
