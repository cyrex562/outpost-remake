
pub fn unk_destroy_win_op_1010_305a(mut param_1: u16 ,param_2: *mut astruct_27,mut param_3: i16,param_4: *mut astruct_65)

{
  astruct_874 **ppaVar1;
  let mut piVar2: *mut i16;
  let mut UVar3: u32;
  let mut lVar4: i32;
  let mut uVar5: u32;
  let mut bVar6: bool;
  let mut uVar8: u16;
  astruct_27 *iVar4;
  let mut iVar9: i16;
  astruct_27 *uVar7;
  let mut uVar10: u16;
  let mut iStack10: i16;
  astruct_874 *paStack8;
  astruct_874 *paStack6;
  astruct_874 *iVar7;

  uVar8 = pass1_1040_c60e(param_4);
  uVar7 = (astruct_27 *)(param_2 >> 0x10);
  iVar4 = (astruct_27 *)param_2;
  iVar4.field18_0x12 = uVar8;
  iVar4.field19_0x14 = 0x0;
  paStack6 = NULL;
  bVar6 = false;
  iVar4.field33_0x28 = 0x0;
  paStack8 = NULL;
  loop {
    ppaVar1 = (astruct_874 **)&iVar4.field20_0x16;
    if (*ppaVar1 == paStack8 || *ppaVar1 < paStack8) {//
LAB_1010_30ad:
      iVar7 = paStack6;
      if (bVar6) {
        while (paStack8 = iVar7 + 0x1, ppaVar1 = (astruct_874 **)&iVar4.field20_0x16,
              *ppaVar1 != paStack8 && paStack8 <= *ppaVar1) {
          UVar3 = (&iVar4.field36_0x2e)[iVar7];
          DestroyWindow16(*(HWND16 *)(UVar3 + 0x18));
          (&iVar4.field36_0x2e)[iVar7] = 0x0;
          iVar7 = paStack8;
        }
        iVar4.field20_0x16 = (paStack6 + 0x1);
        pass1_1010_1f62(param_2,0x9);
      }
      else {
        iVar9 = iVar4.field20_0x16;
        (&iVar4.field34_0x2a)[iVar9 * 0x2] = param_4;
        (&iVar4.field35_0x2c)[iVar9 * 0x2] = (param_4 >> 0x10);
        iStack10 = 0xa;
        piVar2 = &iVar4.field20_0x16;
        *piVar2 = *piVar2 + 0x1;
        if (0x1 < iVar4.field20_0x16) {
          UVar3 = (&iVar4.field30_0x22)[iVar4.field20_0x16];
          iVar9 = UVar3;
          uVar10 = (UVar3 >> 0x10);
          iStack10 = (iVar9 + 0x20) + (iVar9 + 0x24) + 0x8;
        }
        mov_update_win_1040_93aa(param_4,iStack10,iVar4.field23_0x1a);
      }
      if (!bVar6) {
        pass1_1010_1f62(param_2,0xa);
      }
      if (param_3 == 0x0) {
        if (iVar4.field69_0x52 != 0x0) {
          paStack8 = NULL;
          loop {
            lVar4 = iVar4.field69_0x52;
            uVar10 = (lVar4 >> 0x10);
            iVar9 = lVar4;
            if ((*(i32 *)(iVar9 + paStack8 * 0x4) != 0x0) &&
               (*(astruct_65 **)(iVar9 + paStack8 * 0x4) != param_4)) {
              lVar4 = iVar4.field69_0x52;
              uVar5 = (lVar4 + paStack8 * 0x4);
              DestroyWindow16(*(HWND16 *)(uVar5 + 0x18));
            }
            lVar4 = iVar4.field69_0x52;
            (lVar4 + paStack8 * 0x4) = 0x0;
            paStack8 = (astruct_874 *)(paStack8 + 0x1);
          } while (paStack8 < 0xa);
        }
        pass1_1010_32da(param_2,param_4);
        pass1_1010_1f62(param_2,0x8);
      }
      return;
    }
    if (*(astruct_65 **)(&iVar4.field34_0x2a + paStack8 * 0x2) == param_4) {
      bVar6 = true;
      paStack6 = paStack8;
  // TODO: goto LAB_1010_30ad;
    }
    paStack8 = paStack8 + 0x1;
  } while( true );
}
pub fn win_ui_op_1010_3202(param_1: *mut astruct_27,mut param_2: i16)

{
  let mut piVar1: *mut i16;
  let mut lVar2: i32;
  let mut uVar3: u32;
  astruct_27 *iVar3;
  let mut uVar4: u16;
  let mut iStack4: i16;

  iVar3 = (astruct_27 *)param_1;
  uVar4 = (param_1 >> 0x10);
  if (param_2 == 0x0) {
    piVar1 = &iVar3.field33_0x28;
    *piVar1 = *piVar1 + -0xa;
    if (*piVar1 < 0x0) {
      iVar3.field33_0x28 = 0x0;
    }
  }
  else {
    piVar1 = &iVar3.field33_0x28;
    *piVar1 = *piVar1 + &iVar3.field_0x18;
  }
  if (iVar3.field69_0x52 != 0x0) {
    iStack4 = 0x0;
    loop {
      lVar2 = iVar3.field69_0x52;
      if (*(i32 *)(lVar2 + iStack4 * 0x4) != 0x0) {
        lVar2 = iVar3.field69_0x52;
        uVar3 = (lVar2 + iStack4 * 0x4);
        DestroyWindow16(*(HWND16 *)(uVar3 + 0x18));
        lVar2 = iVar3.field69_0x52;
        (lVar2 + iStack4 * 0x4) = 0x0;
      }
      iStack4 += 0x1;
    } while (iStack4 < 0xa);
  }
  if (iVar3.field20_0x16 == 0x0) {
    pass1_1010_32f4(param_1,*(astruct_65 **)(iVar3 + 0x1));
  }
  else {
    pass1_1010_32da(param_1,*(astruct_65 **)(&iVar3.field_0x26 + iVar3.field20_0x16 * 0x4));
  }
  pass1_1010_1f62(param_1,0x8);
  return;
}
pub fn pass1_1010_32c0(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x28) = 0x0;
  (param_1 + 0x12) = param_2;
  return;
}
pub fn pass1_1010_32da(param_1: *mut astruct_27,param_2: *mut astruct_65)

{
  pass1_1010_32f4(param_1,*(astruct_65 **)(param_2 + 0x42));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_32f4(param_1: *mut astruct_27,param_2: *mut astruct_65)

{
  let mut piVar1: *mut i16;
  let mut puVar2: *mut u32;
  let mut uVar3: u32;
  let mut uVar4: u32;
  code **ppcVar5;
  let mut uVar6: u32;
  let mut lVar7: i32;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut iVar11: i16;
  StructD *in_EDX;
  astruct_27 *iVar10;
  let mut iVar12: i16;
  let mut iVar13: i16;
  let mut uVar14: u16;
  let mut uVar15: u16;
  let mut unaff_CS: u16;
  let mut uVar16: u16;
  let mut piStack48: *mut i16;
  let mut iStack16: i16;
  let mut iStack12: i16;

  uVar14 = (param_1 >> 0x10);
  iVar10 = (astruct_27 *)param_1;
  if (iVar10.field69_0x52 != 0x0) {
    unaff_CS = 0x1000;
    fn_ptr_1000_17ce(iVar10.field69_0x52);
    iVar10.field69_0x52 = 0x0;
    &iVar10.field_0x18 = 0x0;
  }
  uVar8 = param_2 | param_2;
  if ((param_2 != NULL) &&
     (ppcVar5 = (code **)(param_1 + 0x24), (**ppcVar5)(unaff_CS,param_1,param_2), uVar8 != 0x0)) {
    ppcVar5 = (code **)(param_2 + 0x4);
    (**ppcVar5)(unaff_CS,param_2);
    &iVar10.field_0x18 = uVar8;
    if (uVar8 != 0x0) {
      (&iVar10.field30_0x22 + 0x2) = 0x0;
      &iVar10.field_0x26 = 0x0;
      piVar1 = &iVar10.field_0x18;
      *piVar1 = *piVar1 - iVar10.field33_0x28;
      if (0xa < &iVar10.field_0x18) {
        &iVar10.field_0x26 = 0x1;
        &iVar10.field_0x18 = 0xa;
      }
      if (0x1 < iVar10.field33_0x28) {
        (&iVar10.field30_0x22 + 0x2) = 0x1;
      }
      if (_PTR_LOOP_1050_5f2c == 0x0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(in_EDX);
      }
      else {
        in_EDX = (_PTR_LOOP_1050_5f2c >> 0x10);
      }
      uVar16 = 0x1000;
      uVar9 = fn_ptr_op_1000_1708(0x28,0x0,0x1,PTR_LOOP_1050_5f2c,in_EDX);
      &iVar10.field69_0x52 = uVar9;
      (&iVar10.field69_0x52 + 0x2) = in_EDX;
      if (((&iVar10.field69_0x52 + 0x2) | &iVar10.field69_0x52) != 0x0) {
        uVar3 = (param_2 + 0x8);
        iVar11 = &iVar10.field_0x10;
        iStack12 = 0x0;
        for (iStack16 = 0x0; piVar1 = &iVar10.field_0x18, *piVar1 != iStack16 && iStack16 <= *piVar1;
            iStack16 += 0x1) {
          uVar6 = iVar10.field69_0x52;
          uVar8 = uVar6 + iStack16 * 0x4;
          uVar6 &= 0xffff0000;
          piStack48 = (uVar6 | uVar8);
          uVar4 = ((iVar10.field33_0x28 + iStack16) * 0x4 + uVar3);
          ppcVar5 = (code **)(param_1 + 0x1c);
          iVar13 = iStack16;
          (**ppcVar5)(uVar16,param_1,uVar4,(uVar4 >> 0x10),&iVar10.field30_0x22);
          *piStack48 = iVar13;
          (uVar8 + 0x2) = in_EDX;
          lVar7 = iVar10.field69_0x52;
          uVar4 = (lVar7 + iStack16 * 0x4);
          iStack12 += (uVar4 + 0x24) + 0x8;
          if (iVar11 + -0xa < iStack12) {
            uVar16 = 0x1008;
            debug_print_1008_6048(in_EDX,s_overflow_on_node__d_1050_11ca);
            &iVar10.field_0x18 = iStack16 + -0x1;
            &iVar10.field_0x26 = 0x1;
            lVar7 = iVar10.field69_0x52;
            uVar15 = (lVar7 >> 0x10);
            iVar13 = lVar7;
            puVar2 = (iVar13 + iStack16 * 0x4);
            uVar8 = (iVar13 + iStack16 * 0x4 + 0x2);
            in_EDX = (uVar8 | puVar2);
            if ((uVar8 | puVar2) != 0x0) {
              ppcVar5 = (code **)*puVar2;
              (**ppcVar5)(0x1008,puVar2,uVar8,0x1);
            }
            lVar7 = iVar10.field69_0x52;
            iVar13 = iStack16 * 0x4;
            (lVar7 + iVar13) = 0x0;
            if (0x0 < iStack16) {
              lVar7 = iVar10.field69_0x52;
              uVar15 = (lVar7 >> 0x10);
              iVar12 = lVar7;
              puVar2 = (iVar12 + iVar13 + -0x4);
              uVar8 = (iVar12 + iVar13 + -0x2);
              in_EDX = (uVar8 | puVar2);
              if ((uVar8 | puVar2) != 0x0) {
                ppcVar5 = (code **)*puVar2;
                (**ppcVar5)(0x1008,puVar2,uVar8,0x1);
              }
              lVar7 = iVar10.field69_0x52;
              (iStack16 * 0x4 + lVar7 + -0x4) = 0x0;
            }
          }
        }
        &iVar10.field_0x20 = 0xa;
        uVar9 = &iVar10.field_0x1e;
        mov_update_win_1040_93aa(*(astruct_65 **)iVar10.field69_0x52,0xa,uVar9);
        for (iStack16 = 0x1; piVar1 = &iVar10.field_0x18, *piVar1 != iStack16 && iStack16 <= *piVar1;
            iStack16 += 0x1) {
          lVar7 = iVar10.field69_0x52;
          uVar3 = (iStack16 * 0x4 + lVar7 + -0x4);
          iVar11 = uVar3;
          uVar16 = (uVar3 >> 0x10);
          lVar7 = iVar10.field69_0x52;
          mov_update_win_1040_93aa
                    (*(astruct_65 **)(lVar7 + iStack16 * 0x4),
                     (iVar11 + 0x20) + (iVar11 + 0x24) + 0x8,uVar9);
        }
      }
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
pub fn pass1_1010_35a4(mut param_1: u16 ,u32 *param_2,mut param_3: u32)

{
  code **ppcVar1;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut puVar4: *mut u32;
  let mut uVar5: u16;
  let mut in_register_0000000a: u16;
  let mut paVar6: *mut Struct57;
  let mut paVar8: *mut Struct57;
  let mut uVar9: u16;
  let mut in_stack_0000ffa6: u16;
  let mut in_stack_0000ffb0: u16;
  let mut uStack12: u32;
  let mut puStack8: *mut u32;
  let mut paVar7: *mut Struct57;

  paVar6 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  uVar9 = (param_2 >> 0x10);
  uVar2 = (param_2 + 0x56);
  uVar2 = (uVar2 + 0x8);
  puStack8 = (uVar2 + (param_2 + 0x5a) * 0x4);
  uStack12 = param_3;
  if (param_3 != 0x0) {
    uVar9 = 0x1000;
    mem_op_1000_179c(0x4a,paVar6);
    uVar3 = param_3;
    uVar5 = paVar6 | uVar3;
    paVar8 = (astruct_57 *)(paVar6 & 0xffff0000);
    paVar7 = (astruct_57 *)(paVar8 | uVar5);
    if (uVar5 == 0x0) {
      uVar3 = 0x0;
    }
    else {
      uVar9 = SUB42(&PTR_LOOP_1050_1040,0x0);
      pass1_1040_c54a((astruct_65 *)(param_3 & 0xffff | (long)paVar6 << 0x10),0x1,puStack8,in_stack_0000ffb0,paVar7);
      paVar8 = paVar7;
    }
    ppcVar1 = (code **)(*param_2 + 0x18);
    (**ppcVar1)(uVar9,param_2,0x1,uVar3,paVar8);
    for (; (uStack12 & 0xf) != 0x0; uStack12 >>= 0x4) {
      uVar2 = (puStack8 + 0x8);
      puStack8 = (((uStack12 & 0xf) - 0x1) * 0x4 + uVar2);
      uVar9 = 0x1000;
      puVar4 = puStack8;
      mem_op_1000_179c(0x4a,paVar8);
      uVar3 = puVar4;
      uVar5 = paVar8 | uVar3;
      paVar6 = (astruct_57 *)((astruct_57 *)(paVar8 & 0xffff0000) | uVar5);
      if (uVar5 == 0x0) {
        uVar3 = 0x0;
        paVar8 = (astruct_57 *)(paVar8 & 0xffff0000);
      }
      else {
        uVar9 = SUB42(&PTR_LOOP_1050_1040,0x0);
        pass1_1040_c54a((astruct_65 *)(puVar4 & 0xffff | (long)paVar8 << 0x10),0x1,puStack8,in_stack_0000ffa6,
                        paVar6);
        paVar8 = paVar6;
      }
      ppcVar1 = (code **)(*param_2 + 0x18);
      (**ppcVar1)(uVar9,param_2,0x1,uVar3);
    }
  }
  return;
}
pub fn pass1_1010_3680(mut param_1: u16 ,u8 *param_2,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,mut param_6: u16 )

{
  let mut uVar1: u16;
  let mut in_register_0000000a: u16;
  let mut paVar2: *mut Struct57;
  let mut in_stack_0000ffc0: u16;

  paVar2 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  mem_op_1000_179c(0x4a,paVar2);
  uVar1 = paVar2 | param_1;
  if (uVar1 != 0x0) {
    pass1_1040_c54a((astruct_65 *)CONCAT22(paVar2,param_1),0x1,CONCAT22(param_6,param_5),
                    in_stack_0000ffc0,paVar2 & 0xffff0000 | uVar1);
    return;
  }
  return;
}



StructD * pass1_1010_36b4(StructD *param_1,param_2: u8)

{
  pass1_1010_2db2((astruct_455 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * pass1_1010_3702(param_1: *mut astruct_19,mut param_2: u16 )

{
  struct_op_1010_1d48(param_1,param_2);
  (param_1 + 0xa) = 0x0;
  param_1.offset_0x0 = 0x37c4;
  (param_1 + 0x2) = 0x1010;
  return &param_1.offset_0x0;
}
pub fn pass1_1010_3730(param_1: *mut u16)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  *param_1 = 0x37c4;
  (param_1 + 0x2) = 0x1010;
  fn_ptr_1000_17ce(*(param_1 + 0xa));
  pass1_1010_1d80(param_1);
  return;
}



pub fn pass1_1010_375e(mut param_1: u32) -> u32

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0xc),(param_1 + 0xa));
}
pub fn pass1_1010_3770(mut param_1: u16 ,param_2: *mut astruct_477,char *param_3)

{
  let mut uVar1: u16;
  astruct_477 *iVar3;
  astruct_477 *uVar2;

  uVar2 = (astruct_477 *)(param_2 >> 0x10);
  iVar3 = (astruct_477 *)param_2;
  fn_ptr_1000_17ce(*&iVar3.field10_0xa);
  uVar1 = str_op_1008_60e8(param_1,param_3);
  iVar3.field10_0xa = uVar1;
  iVar3.field11_0xc = param_1;
  return;
}



u16 * pass1_1010_379e(param_1: *mut u16,param_2: u8,mut param_3: u16 )

{
  pass1_1010_3730(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_223 * pass1_1010_37d4(param_1: *mut astruct_223)

{
  let mut uVar1: u16;

  struct_1010_383a(param_1);
  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x16) = 0x0;
  param_1.field0_0x0 = 0x3b3e;
  (param_1 + 0x2) = 0x1010;
  return param_1;
}
pub fn pass1_1010_3800(StructD *param_1)

{
  StructD *iVar2;
  StructD *uVar1;

  uVar1 = (param_1 >> 0x10);
  iVar2 = param_1;
  param_1.address_offset_field_0x0 = 0x3b3e;
  iVar2.address_offset_field_0x2 = 0x1010;
  if (*(i32 *)(&iVar2.field12_0x14 + 0x2) != 0x0) {
    fn_ptr_1000_17ce(*(&iVar2.field12_0x14 + 0x2));
  }
  pass1_1010_3880(param_1);
  return;
}
pub fn struct_1010_383a(param_1: *mut astruct_223)

{
  astruct_223 *iVar1;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = (astruct_223 *)param_1;
  param_1.field0_0x0 = 0x389a;
  iVar1.field1_0x2 = 0x1008;
  iVar1.field2_0x4 = 0x0;
  iVar1.field3_0x8 = 0x0;
  iVar1.field4_0xc = 0x0;
  iVar1.field5_0x10 = 0x0;
  iVar1.field6_0x12 = 0x0;
  iVar1.field7_0x14 = 0x0;
  param_1.field0_0x0 = 0x3b5e;
  iVar1.field1_0x2 = 0x1010;
  return;
}
pub fn pass1_1010_3880(StructD *param_1)

{
  let mut piVar1: *mut i16;
  let mut puVar2: *mut u32;
  let mut uVar3: u16;
  code **ppcVar4;
  let mut uVar5: u32;
  StructD *iVar6;
  let mut iVar7: i16;
  StructD *uVar8;
  let mut uVar9: u16;
  let mut iStack4: i16;

  uVar8 = (param_1 >> 0x10);
  iVar6 = param_1;
  param_1.address_offset_field_0x0 = 0x3b5e;
  iVar6.address_offset_field_0x2 = 0x1010;
  if (*(i32 *)&iVar6.field5_0x8 != 0x0) {
    iStack4 = 0x0;
    while( true ) {
      piVar1 = &iVar6.field_0x10;
      if (*piVar1 == iStack4 || *piVar1 < iStack4) break;
      uVar5 = &iVar6.field5_0x8;
      uVar9 = (uVar5 >> 0x10);
      iVar7 = uVar5;
      puVar2 = (iVar7 + iStack4 * 0x4);
      uVar3 = (iVar7 + iStack4 * 0x4 + 0x2);
      if ((uVar3 | puVar2) != 0x0) {
        ppcVar4 = (code **)*puVar2;
        (**ppcVar4)();
      }
      iStack4 += 0x1;
    }
    fn_ptr_1000_17ce(*&iVar6.field5_0x8);
  }
  param_1.address_offset_field_0x0 = 0x389a;
  iVar6.address_offset_field_0x2 = 0x1008;
  return;
}



astruct_57 * struct_1010_38f8(mut param_1: u16 ,param_2: *mut astruct_57,param_3: *mut astruct_240,mut param_4: i16)

{
  let mut uVar1: u16;
  astruct_240 *iVar2;
  astruct_240 *uVar2;
  let mut paVar2: *mut Struct57;

  if (param_4 != 0x0) {
    uVar1 = param_4 << 0x2;
    mem_op_1000_179c(uVar1,param_2);
    uVar2 = (astruct_240 *)(param_3 >> 0x10);
    iVar2 = (astruct_240 *)param_3;
    iVar2.field8_0x8 = uVar1;
    iVar2.field9_0xa = (astruct_57 *)param_2;
    return (astruct_57 *)CONCAT22((astruct_57 *)param_2,iVar2.field8_0x8);
  }
  mem_op_1000_179c(0x1a,param_2);
  if ((param_2 | param_1) != 0x0) {
    paVar2 = (astruct_57 *)pass1_1010_37d4((astruct_223 *)CONCAT22(param_2,param_1));
    return paVar2;
  }
  return NULL;
}
pub fn pass1_1010_394a(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16)

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  if (param_5 != 0x0) {
    mem_op_1000_179c(param_5 << 0x2,paVar1);
    return;
  }
  mem_op_1000_179c(0x16,paVar1);
  if ((paVar1 | param_1) != 0x0) {
    struct_1010_383a((astruct_223 *)CONCAT22(paVar1,param_1));
    return;
  }
  return;
}
pub fn pass1_1010_398e(mut param_1: u16 ,u32 *param_2,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u32)

{
  let mut piVar1: *mut i16;
  code **ppcVar2;
  let mut uVar3: u32;
  let mut uVar4: u32;
  let mut iVar5: i16;
  let mut uVar6: u16;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut iVar7: i16;
  let mut iVar8: i16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uStack12: u16;
  let mut puStack6: *mut u32;

  uVar9 = (param_2 >> 0x10);
  uVar3 = *param_2;
  ppcVar2 = (code **)(uVar3 + 0x8);
  (**ppcVar2)();
  puStack6 = CONCAT22(extraout_DX,param_1);
  if ((extraout_DX | param_1) == 0x0) {
    return;
  }
  (param_1 + 0xc) = param_5;
  iVar7 = *puStack6;
  ppcVar2 = (code **)(iVar7 + 0xc);
  (**ppcVar2)();
  iVar5 = (param_2 + 0x14);
  piVar1 = (param_2 + 0x14);
  *piVar1 = *piVar1 + 0x1;
  ppcVar2 = (code **)(iVar7 + 0x10);
  (**ppcVar2)();
  ppcVar2 = (code **)(iVar7 + 0x4);
  (**ppcVar2)();
  if (iVar5 != 0x0) {
    ppcVar2 = (code **)(uVar3 + 0x8);
    iVar7 = iVar5;
    (**ppcVar2)();
    (param_1 + 0x8) = iVar7;
    (param_1 + 0xa) = extraout_DX_00;
    PTR_LOOP_1050_11de = PTR_LOOP_1050_11de + 0x1;
    uVar9 = extraout_DX_00;
    for (uStack12 = 0x0; uStack12 < iVar5; uStack12 += 0x1) {
      uVar6 = uStack12;
      pass1_1010_398e(uStack12,param_2,uStack12,uStack12 >> 0xf,puStack6);
      uVar4 = (param_1 + 0x8);
      uVar10 = (uVar4 >> 0x10);
      iVar7 = uVar4;
      iVar8 = uStack12 * 0x4;
      (iVar7 + iVar8) = uVar6;
      (iVar7 + iVar8 + 0x2) = uVar9;
      uVar4 = (param_1 + 0x8);
      if (*(i32 *)(uVar4 + iVar8) == 0x0) break;
    }
    PTR_LOOP_1050_11de = PTR_LOOP_1050_11de + -0x1;
  }
  return;
}



u16 pass1_1010_3a86(mut param_1: u32)

{
  return (param_1 + 0x10);
}
pub fn pass1_1010_3a94(mut param_1: u32,mut param_2: u16 )

{
  (param_1 + 0x12) = param_2;
  return;
}
pub fn FUN_1010_3aa6()

{
  return;
}



pub fn pass1_1010_3aaa(mut param_1: u32) -> u32

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  return CONCAT22((param_1 + 0x6),(param_1 + 0x4));
}



u16 FUN_1010_3abc()

{
  return 0x0;
}
pub fn pass1_1010_3ac2(mut param_1: u32,mut param_2: u16 ,mut param_3: u32)

{
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  (param_1 + 0x16) = param_3;
  (param_1 + 0x12) = param_2;
  return;
}



pub fn pass1_1010_3adc(mut param_1: u32) -> u32

{
  let mut puVar1: *mut u16;

  puVar1 = (param_1 + 0x16);
  return CONCAT22((puVar1 + 0x2),*puVar1);
}



StructD * pass1_1010_3af2(StructD *param_1,param_2: u8)

{
  pass1_1010_3800(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



StructD * pass1_1010_3b18(StructD *param_1,param_2: u8)

{
  pass1_1010_3880(param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}
pub fn struct_1010_3b7a(param_1: *mut astruct_19,mut param_2: u16 )

{
  struct_op_1010_1d48(param_1,param_2);
  (param_1 + 0xa) = 0x389a;
  (param_1 + 0xc) = 0x1008;
  (param_1 + 0xa) = 0x3aa8;
  (param_1 + 0xc) = 0x1008;
  (param_1 + 0xe) = 0x0;
  (param_1 + 0x12) = 0x0;
  (param_1 + 0x14) = 0x0;
  (param_1 + 0x16) = 0x0;
  param_1.offset_0x0 = 0x3d6a;
  (param_1 + 0x2) = 0x1010;
  (param_1 + 0xa) = 0x3d7a;
  (param_1 + 0xc) = 0x1010;
  return;
}
pub fn pass1_1010_3bde(param_1: *mut astruct_455,mut param_2: u16 )

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut puVar4: *mut u16;
  astruct_455 *iVar4;
  let mut uVar5: u16;
  let mut puStack14: *mut u16;

  uVar5 = (param_1 >> 0x10);
  iVar4 = (astruct_455 *)param_1;
  param_1.field0_0x0 = 0x3d6a;
  iVar4.field1_0x2 = 0x1010;
  iVar4[0x1].field1_0x2 = 0x3d7a;
  iVar4[0x1].field2_0x4 = 0x1010;
  puVar1 = iVar4[0x1].field3_0x6;
  uVar2 = (iVar4 + 0x2)->field0_0x0;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  if (param_1 == NULL) {
    puVar4 = NULL;
    uVar5 = 0x0;
  }
  else {
    puVar4 = &iVar4[0x1].field1_0x2;
  }
  puStack14 = CONCAT22(uVar5,puVar4);
  *puStack14 = 0x389a;
  puVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn struct_1010_3c52(mut param_1: u32,mut param_2: u32,mut param_3: u16 )

{
  let mut uVar1: u16;
  code **ppcVar2;
  let mut puVar3: *mut u32;
  let mut uVar4: u16;
  astruct_274 *iVar4;
  let mut uVar6: u16;
  let mut unaff_CS: u16;
  let mut uVar5: u32;

  uVar6 = (param_2 >> 0x10);
  iVar4 = (astruct_274 *)param_2;
  iVar4.field18_0x14 = param_3;
  puVar3 = iVar4.field14_0xe;
  uVar1 = iVar4.field15_0x10;
  uVar4 = uVar1 | puVar3;
  uVar5 = param_1 & 0xffff0000 | uVar4;
  if (uVar4 != 0x0) {
    ppcVar2 = (code **)*puVar3;
    puVar3 = (**ppcVar2)();
  }
  puVar3 = FUN_1010_830a(puVar3,uVar5,unaff_CS,_u16_1050_14cc,iVar4.field18_0x14);
  iVar4.field14_0xe = puVar3;
  iVar4.field15_0x10 = uVar5;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_3c9e(i32 param_1)

{
  let mut paVar1: *mut Struct57;
  let mut in_EDX: u32;
  let mut paVar2: *mut Struct57;

  if (param_1 == 0x0) {
    paVar1 = NULL;
    paVar2 = (astruct_57 *)(in_EDX & 0xffff0000);
  }
  else {
    paVar2 = (astruct_57 *)(in_EDX & 0xffff0000 | param_1);
    paVar1 = (astruct_57 *)(param_1 + 0xa);
  }
  pass1_1008_9262(paVar1,paVar2,_PTR_LOOP_1050_0388,(_PTR_LOOP_1050_0388 >> 0x10),
                  (param_1 + 0x12),CONCAT22(paVar2,paVar1));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_3cd0(i32 param_1)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  if (_PTR_LOOP_1050_0388 != 0x0) {
    if (param_1 == 0x0) {
      iVar1 = 0x0;
      uVar2 = 0x0;
    }
    else {
      iVar1 = param_1 + 0xa;
      uVar2 = param_1;
    }
    pass1_1008_92b2(_PTR_LOOP_1050_0388,(param_1 + 0x12),CONCAT22(uVar2,iVar1));
  }
  return;
}
pub fn pass1_1010_3d0a(mut param_1: i16,mut param_2: u16 ,mut param_3: i16,mut param_4: u16 )

{
  if (param_3 == 0x2) {
    pass1_1010_3cd0(CONCAT22(param_2,param_1 + -0xa));
    pass1_1010_1f62((astruct_27 *)CONCAT22(param_2,param_1 + -0xa),0x2);
  }
  return;
}



StructD * pass1_1010_3d38(StructD *param_1,param_2: u8)

{
  param_1 = (param_1 & 0xffff0000 | (param_1 - 0xa));
  pass1_1010_3bde((astruct_455 *)param_1,&DAT_1050_1050);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



StructD * FUN_1010_3d44(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  pass1_1010_3bde((astruct_455 *)param_2,&DAT_1050_1050);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_2);
  }
  return param_2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1010_3d82(mut param_1: u32,param_2: *mut astruct_19,param_3: *mut astruct_19,mut param_4: u16 ) -> u32

{
  let mut IVar1: i16;
  let mut uVar3: u16;
  let mut uVar2: u32;
  let mut unaff_CS: u16;
  astruct_19 *paVar4;

  uVar3 = (param_1 >> 0x10);
  paVar4 = struct_op_1010_1d48((astruct_19 *)CONCAT22(param_3,param_2),param_4);
  uVar2 = CONCAT22(uVar3,(paVar4 >> 0x10));
  &param_2.horiz_res_0xa = 0x0;
  CONCAT22(param_3,param_2) = 0x3e2c;
  param_2.segment_0x2 = 0x1010;
  IVar1 = FUN_1010_830a(paVar4,uVar2,unaff_CS,_u16_1050_14cc,0x99);
  param_2.horiz_res_0xa = IVar1;
  param_2.ver_res_0xc = uVar2;
  return CONCAT22(param_3,param_2);
}
pub fn pass1_1010_3dc8(param_1: *mut astruct_455,mut param_2: u16 )

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u32;
  code **ppcVar3;
  astruct_455 *iVar4;
  astruct_455 *uVar4;

  uVar4 = (astruct_455 *)(param_1 >> 0x10);
  iVar4 = (astruct_455 *)param_1;
  param_1.field0_0x0 = 0x3e2c;
  iVar4.field1_0x2 = 0x1010;
  puVar1 = iVar4[0x1].field1_0x2;
  puVar2 = iVar4[0x1].field2_0x4;
  if ((puVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  pass1_1010_1d80(param_1);
  return;
}



StructD * pass1_1010_3e06(StructD *param_1,param_2: u8)

{
  pass1_1010_3dc8((astruct_455 *)param_1,&DAT_1050_1050);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_3e3c(param_1: *mut astruct_19,mut param_2: u16 ,mut param_3: u16 )

{
  let mut puVar1: *mut u16;
  let mut uVar2: u16;
  let mut in_EDX: u32;
  let mut uVar3: u32;
  astruct_19 *iVar1;
  astruct_19 *uVar1;
  let mut puVar4: *mut u16;

  uVar2 = (in_EDX >> 0x10);
  get_sys_metrics_1018_4b1e(param_1,0x6,param_2);
  uVar1 = (astruct_19 *)(param_1 >> 0x10);
  iVar1 = (astruct_19 *)param_1;
  iVar1.field17_0x20 = 0x389a;
  iVar1.field18_0x22 = 0x1008;
  iVar1.field17_0x20 = 0x3aa8;
  iVar1.field18_0x22 = 0x1008;
  iVar1.field19_0x24 = 0x0;
  (&iVar1[0x1].field2_0x4 + 0x2) = 0x0;
  iVar1[0x1].horiz_res_0xa = 0x4;
  &iVar1[0x1].ver_res_0xc = 0x0;
  &iVar1[0x1].field8_0x10 = 0x0;
  iVar1[0x1].field10_0x14 = 0x0;
  puVar4 = pass1_1008_3e54((param_1 & 0xffff0000 | ZEXT24(&iVar1[0x1].field11_0x16)),0x0,0x3,0x5);
  uVar3 = CONCAT22(uVar2,(puVar4 >> 0x10));
  &iVar1[0x1].field15_0x1c = 0x0;
    // 0x4a46
  param_1.offset_0x0 = &PTR_LOOP_1050_4a46;
  iVar1.segment_0x2 = 0x1010;
    // just 0x4a82
  iVar1.field17_0x20 = &PTR_LOOP_1050_4a82;
  iVar1.field18_0x22 = 0x1010;
  puVar1 = pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(&iVar1.field20_0x26)),NULL,0x40);
  uVar2 = FUN_1010_830a(puVar1,uVar3,0x1000,_u16_1050_14cc,0x1a1);
  (&iVar1[0x1].field2_0x4 + 0x2) = uVar2;
  iVar1[0x1].field3_0x8 = uVar3;
  pass1_1018_4b78(param_1);
  return;
}
pub fn pass1_1010_3f00(StructD *param_1)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut puVar4: *mut u16;
  StructD *iVar5;
  StructD *uVar5;
  let mut puStack16: *mut u16;
  let mut iStack4: i16;

  uVar5 = (param_1 >> 0x10);
  iVar5 = param_1;
  param_1.address_offset_field_0x0 = &PTR_LOOP_1050_4a46;
  iVar5.address_offset_field_0x2 = 0x1010;
  iVar5.field19_0x20 = &PTR_LOOP_1050_4a82;
  iVar5.field20_0x22 = 0x1010;
  iStack4 = 0x0;
  loop {
    puVar1 = (&iVar5.field_0x26 + iStack4 * 0x4);
    uVar2 = (&iVar5.field_0x28 + iStack4 * 0x4);
    if ((uVar2 | puVar1) != 0x0) {
      ppcVar3 = (code **)*puVar1;
      (**ppcVar3)();
    }
    iStack4 += 0x1;
  } while (iStack4 < 0x10);
  puVar1 = &iVar5.field_0x66;
  uVar2 = &iVar5->field_0x68;
  if ((uVar2 | puVar1) != 0x0) {
    ppcVar3 = (code **)*puVar1;
    (**ppcVar3)();
  }
  fn_ptr_1000_17ce(*&iVar5->field_0x70);
  if (param_1 == NULL) {
    puVar4 = NULL;
    uVar5 = NULL;
  }
  else {
    puVar4 = &iVar5->field19_0x20;
  }
  puStack16 = CONCAT22(uVar5,puVar4);
  *puStack16 = 0x389a;
  puVar4[0x1] = 0x1008;
  pass1_1010_1d80(param_1);
  return;
}



u16 FUN_1010_3fc2(mut param_1: u16 ,mut param_2: u32,u8 *param_3)

{
  let mut BVar1: bool;
  let mut iVar2: i16;
  let mut uVar3: u16;
  HFILE16 in_stack_0000ffda;
  u16 local_c [0x3];
  u16 local_6 [0x2];

  BVar1 = write_to_file_1008_7cac(param_3);
  if (BVar1 != 0x0) {
    uVar3 = (param_2 >> 0x10);
    iVar2 = param_2;
    local_c[0] = (iVar2 + 0x24);
    BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_c),0x2,in_stack_0000ffda);
    if (BVar1 != 0x0) {
      local_6[0] = (iVar2 + 0x6a);
      BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_6),0x2,in_stack_0000ffda);
      if (BVar1 != 0x0) {
        local_6[0] = (iVar2 + 0x7e);
        BVar1 = write_to_file_1008_7e1c(param_3,CONCAT22(0x1050,local_6),0x2,in_stack_0000ffda);
        if (BVar1 != 0x0) {
          return 0x1;
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return 0x0;
}
