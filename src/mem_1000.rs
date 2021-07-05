use crate::sys_api::_SHI_INVOKEERRORHANDLER1;
use crate::util::make_u16_ptr;

fn  mem_op_1000_01b0(param_1: u16, param_2: u16) -> bool

{
  let puVar1: *mut u16;
  let piVar2: *mut i16;
  let BVar3: bool;
  let UVar4: u16;
  let uVar5: u16;
  let DVar6: u32;
  let DVar7: u32;
  let uVar8: u32;
  let uVar9: u16;
  let uVar10: u16;
  let uStack14: u16;
  let uStack12: u16;
  let uStack10: i16;
  let uStack6: u16;
  let iStack4: i16;
  
  uStack14 = 0x0;
  if (((param_1 + 0x40) | (param_1 + 0x3e)) == 0x0) {
    uVar5 = param_1 + 0x36;
    DVar6 = mem_op_1000_1532(param_2);
    DVar7 = DVar6;
  }
  else {
    DVar6 = mem_op_1000_1532(param_2);
    uVar5 = DVar6;
    if (((DVar6 >> 0x10) != 0x0) || (0xffef < uVar5)) {
      pass1_1000_1e61(param_2,0x8,param_1,&USHORT_1050_1050);
      return false;
    }
    if (0x1fff < uVar5) {
      uVar5 = 0x2000;
    }
    while( true ) {
      uVar9 = uVar5;
      DVar7 = DVar6 + 0x18;
      if (((DVar7 >> 0x10) != 0x0) || (0xfff0 < DVar7)) {
        DVar7 = 0xfff0;
      }
      BVar3 = mem_op_1000_14f2((param_1 + 0x16) | 0x1000,DVar7,
                               (DVar7 >> 0x10),param_1,&USHORT_1050_1050);
      iStack4 = (DVar6 >> 0x10);
      uStack6 = DVar6;
      if (BVar3 != 0x0) break;
      uVar5 = uVar9 >> 0x1;
      if (uVar5 < 0xc) {
        UVar4 = pass1_1000_1e61(param_2,0x2,param_1,&USHORT_1050_1050);
        if (UVar4 == 0x0) {
          return (bool)('\x01' - ((param_1 + 0xa) == 0x0));
        }
        DVar6 = mem_op_1000_1532(param_2);
        uVar5 = uVar9 & 0xfffe;
      }
    }
    uVar8 = pass1_1000_5390(uStack6 - 0x42,iStack4 - (uStack6 < 0x42),0xc,0x0);
    uVar5 = uVar8 * 0xc + param_1 + 0x42;
  }
  puVar1 = (param_1 + 0x1e);
  uVar9 = *puVar1;
  *puVar1 = *puVar1 - DVar6;
  piVar2 = (i16 *)(param_1 + 0x20);
  *piVar2 = (*piVar2 - (DVar6 >> 0x10)) - (uVar9 < DVar6);
  if (uVar5 != 0x0) {
    uVar10 = 0x0;
    uVar9 = 0xc;
    DVar7 = mem_op_1000_1532(param_2);
    uVar8 = pass1_1000_5390(DVar7 - 0x42,
                            (DVar7 >> 0x10) - (DVar7 < 0x42),uVar9,uVar10
                           );
    uStack14 = uVar8 * 0xc + param_1 + 0x36;
  }
  iStack10 = (DVar7 >> 0x10);
  uStack12 = DVar7;
  puVar1 = (param_1 + 0x1e);
  uVar9 = *puVar1;
  *puVar1 = *puVar1 + uStack12;
  piVar2 = (i16 *)(param_1 + 0x20);
  *piVar2 = *piVar2 + iStack10 + CARRY2(uVar9,uStack12);
  uVar9 = (param_1 + 0xa);
  do {
    uVar10 = uVar5;
    (uVar10 + 0x4) = uVar9;
    uVar9 = uVar10;
    uVar5 = uVar10 + 0xc;
  } while (uVar10 < uStack14);
  (param_1 + 0xa) = uVar10;
  return true;
}

fn  mem_op_1000_0308(param_1: i16,param_2: i16, unaff_CS: u16) -> i16

{
  let iVar1: i16;
  let iVar2: i16;
  let bVar3: bool;
  let extraout_AH: u8;
  let piVar4: *mut i16;
  
  if ((param_2 + 0xa) == 0x0) {
    bVar3 = mem_op_1000_01b0(param_2,unaff_CS);
    if (CONCAT11(extraout_AH,bVar3) == 0x0) {
      return 0x0;
    }
  }
  iVar1 = (param_2 + 0xa);
  (param_2 + 0xa) = (iVar1 + 0x4);
  piVar4 = (i16 *)(param_1 * 0x2 + param_2);
  if (*piVar4 == 0x0) {
    (iVar1 + 0x6) = iVar1;
    (iVar1 + 0x4) = iVar1;
  }
  else {
    iVar2 = *piVar4;
    (iVar1 + 0x6) = iVar2;
    (iVar1 + 0x4) = (iVar2 + 0x4);
    ((iVar2 + 0x4) + 0x6) = iVar1;
    (iVar2 + 0x4) = iVar1;
  }
  *piVar4 = iVar1;
  return iVar1;
}

fn mem_op_1000_03c6(param_1: u16,param_2: i16,param_3: u16,param_4: u32,
                       param_5: u16,param_6: u8,param_7: u8) -> u32

{
  let puVar1: *mut u16;
  let piVar2: *mut i16;
  let uVar3: u16;
  let uVar4: u16;
  let puVar5: *mut u16;
  let UVar6: u16;
  let uVar7: u16;
  let bVar8: bool;
  let DVar9: u32;
  let uStack20: u16;
  let uVar9: u16;
  
  uVar7 = CONCAT11(param_7,param_6);
  uVar3 = param_1 + 0xfff & 0xf000;
  puVar1 = (param_4 + 0x1e);
  uVar4 = uVar3 + *puVar1;
  uVar3 = param_2 + (0xf000 < param_1) + (param_4 + 0x20) +
          CARRY2(uVar3,*puVar1);
  puVar1 = (param_4 + 0x28);
  bVar8 = uVar3 < *puVar1;
  if ((bVar8) ||
     ((bVar8 || uVar3 == *puVar1 &&
      (puVar1 = (param_4 + 0x26), uVar4 < *puVar1 || uVar4 == *puVar1)))) {
    if (param_3 == 0x3) {
      uStack20 = ((byte)(-((param_6 & 0x1) != 0x0) >> 0x8) & 0x1 | 0x20) <<
                 0x8;
    }
    else {
      uStack20 = 0x1000;
    }
    uStack20 = (param_4 + 0x16) | uStack20;
    mem_op_1000_131c(uStack20,param_1,param_2,param_5);
    if ((uVar3 | uStack20) != 0x0) {
      puVar5 = mem_op_1000_0308(param_3,param_4);
      if (puVar5 != 0x0) {
        puVar5[0x4] = uStack20;
        puVar5[0x5] = uVar3;
        uVar9 = &USHORT_1050_1050;
        &PTR_LOOP_1050_000c = param_3 | 0xcad0;
        0x0 = param_4;
        &PTR_LOOP_1050_0002 = &USHORT_1050_1050;
        *(u16 **)&DAT_1050_0004 = puVar5;
        (&DAT_1050_0004 + 0x2) = &USHORT_1050_1050;
        &PTR_LOOP_1050_000a = 0x0;
        DVar9 = mem_op_1000_1532(param_5);
        UVar6 = DVar9;
        if (param_3 == 0x1) {
          uVar7 = pass1_1000_0782(param_4,UVar6,0x0);
        }
        else {
          if (param_3 == 0x3) {
            pass1_1000_05b4(param_6,0x0);
          }
          else {
            uVar7 = pass1_1000_09ca(UVar6,0x0);
          }
        }
        param_2 = (DVar9 >> 0x10);
        *puVar5 = uVar7;
        puVar5[0x1] = 0x8000;
        puVar1 = (param_4 + 0x1e);
        uVar4 = *puVar1;
        *puVar1 = *puVar1 + UVar6;
        piVar2 = (i16 *)(param_4 + 0x20);
        *piVar2 = *piVar2 + param_2 + CARRY2(uVar4,UVar6);
        return uVar3;
      }
      mem_op_1000_13ce(param_5);
    }
  }
  else {
    pass1_1000_1e61(param_5,0x7,param_4,&USHORT_1050_1050);
  }
  return 0x0;
}



fn mem_op_1000_0510(param_1: u16,param_2: u16,param_3: u16) -> u32

{
  let puVar1: *mut u16;
  let piVar2: *mut i16;
  let bVar3: u8;
  let iVar4: i16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let bVar11: bool;
  let DVar12: u32;
  let lVar13: i32;
  let uVar5: u16;
  
  iVar4 = param_2;
  uVar5 = (param_2 + 0x2);
  uVar6 = (param_2 + 0x4);
  bVar3 = *(byte *)(param_2 + 0xc);
  DVar12 = mem_op_1000_1532(param_3);
  uVar9 = (DVar12 >> 0x10);
  uVar8 = DVar12;
  if (param_1 != 0x0) {
    uVar7 = (iVar4 + 0x1e);
    uVar10 = ((iVar4 + 0x20) - uVar9) - (uVar7 < uVar8);
    puVar1 = (iVar4 + 0x24);
    bVar11 = uVar10 < *puVar1;
    if ((bVar11 || uVar10 == *puVar1) &&
       ((bVar11 || (uVar7 - uVar8 < (iVar4 + 0x22))))) {
      bVar11 = false;
      uVar9 = uVar10;
      goto LAB_1000_0595;
    }
  }
  pass1_1000_0368(uVar6,bVar3 & 0x7,0x0);
  puVar1 = (s_version__d__d_1050_0012 + 0xc);
  uVar7 = *puVar1;
  *puVar1 = *puVar1 - uVar8;
  piVar2 = (i16 *)s_New_failed_in_Op__Op_1050_0020;
  *piVar2 = (*piVar2 - uVar9) - (uVar7 < uVar8);
  bVar11 = true;
LAB_1000_0595:
  if (bVar11) {
    (param_2 + 0xc) = 0x0;
    lVar13 = mem_op_1000_13ce(param_3);
    return CONCAT22((lVar13 >> 0x10),0x1);
  }
  return uVar9 << 0x10;
}

fn mem_op_1000_05e2(param_1: u16,param_2: i16,param_3: u16,param_4: u16,
                      param_5: u16) -> u32

{
  let puVar1: *mut u16;
  let iVar2: i16;
  let uVar3: u16;
  let uVar4: u16;
  let UVar5: u16;
  let UVar6: u32;
  let bVar5: bool;
  let uVar6: u32;
  
  iVar2 = param_2 + (0xffeb < param_1);
  do {
    uVar3 = 0x3;
    UVar6._0_1_ = (u8)param_3;
    UVar6._1_1_ = (u8)(param_3 >> 0x8);
    UVar6._0_2_ = mem_op_1000_03c6(param_1 + 0x14,iVar2,0x3,param_4,param_5,
                                   (u8)UVar6,UVar6._1_1_);
    if ((UVar6 | uVar3) != 0x0) {
      return CONCAT22(UVar6,uVar3 + 0x14);
    }
    uVar6 = mem_op_1000_0052(param_4,param_5);
    uVar3 = param_1 + 0x1013 & 0xf000;
    puVar1 = (param_4 + 0x1e);
    uVar4 = uVar3 + *puVar1;
    uVar3 = iVar2 + (0xf000 < param_1 + 0x14) + (param_4 + 0x20) +
            CARRY2(uVar3,*puVar1);
    puVar1 = (param_4 + 0x28);
    bVar5 = uVar3 < *puVar1;
  } while (((bVar5 || uVar3 == *puVar1) &&
           ((bVar5 || (puVar1 = (param_4 + 0x26),
                      uVar4 < *puVar1 || uVar4 == *puVar1)))) &&
          ((uVar6 != 0x0 ||
           (UVar5 = pass1_1000_1e61(param_5,0x2,param_4,&USHORT_1050_1050),
           UVar5 != 0x0))));
  return 0x0;
}



fn  mem_1000_0668(param_1: u16) -> u32

{
  let uVar1: u32;
  
  uVar1 = mem_op_1000_0510(0x0,0x0,param_1);
  return uVar1;
}



fn mem_1000_0670(param_1: u16,i16 *param_2,param_3: u16,param_4: *mut u32,param_5: i16,
             param_6: u16) -> u16

{
  let puVar1: *mut u16;
  let piVar2: *mut i16;
  let UVar3: u16;
  let UVar4: u16;
  let iVar5: i16;
  let UVar6: u16;
  let uVar7: u32;
  let uVar8: u16;
  let uVar9: u16;
  let UVar10: u16;
  let BVar11: bool;
  let uVar12: u16;
  let uVar13: u16;
  let uVar14: u16;
  let DVar15: u32;
  let DVar16: u32;
  
  UVar3 = param_4;
  UVar4 = (param_4 + 0x2);
  DVar15 = mem_op_1000_1532(param_6);
  UVar6 = param_5 + (0xffeb < param_3);
  uVar7 = *param_4;
  uVar8 = -((param_1 & 0x1) != 0x0) & 0x100 |
          -((param_1 & 0x4) != 0x0) & 0x400 | (uVar7 + 0x16);
  if (param_2 == (i16 *)0x0) {
    BVar11 = mem_op_1000_14f2(uVar8 | 0x2000,param_3 + 0x14,UVar6,param_4,
                              &USHORT_1050_1050);
    if (BVar11 == 0x0) {
      return 0x0;
    }
  }
  else {
    iVar5 = (param_4 + 0x1);
    uVar12 = (param_4 + 0x6);
    uVar14 = uVar12;
    do {
      uVar13 = uVar14;
      uVar9 = uVar8 | 0x2000;
      mem_op_1000_1408(uVar9,param_3 + 0x14,UVar6,param_6);
      uVar14 = uVar13 | uVar9;
      if (uVar14 != 0x0) break;
      UVar10 = pass1_1000_1e61(param_6,0x2,UVar3,UVar4);
    } while (UVar10 != 0x0);
    if ((uVar13 | uVar9) == 0x0) {
      (param_2 + 0x2) = 0x0;
      *param_2 = 0x0;
      return 0x0;
    }
    (iVar5 + 0x8) = uVar9;
    (iVar5 + 0xa) = uVar13;
    *param_2 = uVar9 + 0x14;
    (param_2 + 0x2) = uVar13;
  }
  DVar16 = mem_op_1000_1532(param_6);
  uVar12 = (DVar16 - DVar15);
  puVar1 = (UVar3 + 0x1e);
  uVar8 = *puVar1;
  *puVar1 = *puVar1 + uVar12;
  piVar2 = (i16 *)(UVar3 + 0x20);
  *piVar2 = *piVar2 + (DVar16 - DVar15 >> 0x10) + CARRY2(uVar8,uVar12);
  return 0x1;
}


fn mem_op_1000_0838(param_1: u16,param_2: u16) -> u32
{
  let puVar1: *mut u16;
  let piVar2: *mut i16;
  let iVar3: i16;
  let puVar4: *mut u16;
  let uVar5: u16;
  let uVar6: u16;
  let UVar7: u16;
  let UVar8: u32;
  let piVar9: *mut i16;
  let bVar10: bool;
  let uStack6: u16;
  let piStack4: *mut i16;
  
  piVar9 = *(i16 **)(param_1 + 0x2);
  piStack4 = piVar9;
  if ((param_1 + 0x2) == 0x0) goto LAB_1000_085b;
  do {
    do {
      if (*piVar9 != 0x0) {
        iVar3 = piVar9[0x5];
        puVar4 = &PTR_LOOP_1050_000e;
        if (puVar4 != 0x0) {
          &PTR_LOOP_1050_000e = *puVar4;
          piVar2 = (i16 *)&PTR_LOOP_1050_000a;
          *piVar2 = *piVar2 + 0x1;
          *(i16 **)(param_1 + 0x2) = piVar9;
          return CONCAT22(iVar3,puVar4);
        }
        *piVar9 = 0x0;
      }
      piVar9 = (i16 *)piVar9[0x2];
    } while (piVar9 != piStack4);
LAB_1000_085b:
    if ((param_1 + 0x18) == 0x0) {
      pass1_1000_1e61(param_2,0x4,param_1,&USHORT_1050_1050);
      return 0x0;
    }
    uVar5 = (param_1 + 0x1a);
    while( true ) {
      uStack6 = uVar5;
      uVar5 = 0x1;
      UVar8 = mem_op_1000_03c6(uStack6,0x0,0x1,param_1,param_2,0x0,'\0');
      if ((UVar8 | uVar5) != 0x0) break;
      uVar5 = (param_1 + 0x1e);
      uVar6 = uVar5 + uStack6;
      uVar5 = (param_1 + 0x20) + CARRY2(uVar5,uStack6);
      puVar1 = (param_1 + 0x28);
      bVar10 = *puVar1 <= uVar5;
      if (bVar10) {
        if (bVar10 && uVar5 != *puVar1) {
          return 0x0;
        }
        puVar1 = (param_1 + 0x26);
        if (*puVar1 <= uVar6 && uVar6 != *puVar1) {
          return 0x0;
        }
      }
      uVar5 = uStack6 >> 0x1;
      if (uStack6 >> 0x1 < (param_1 + 0x18) + 0x14U) {
        UVar7 = pass1_1000_1e61(param_2,0x2,param_1,&USHORT_1050_1050);
        uVar5 = uStack6 & 0xfffe;
        if (UVar7 == 0x0) {
          return 0x0;
        }
      }
    }
    piVar9 = *(i16 **)(param_1 + 0x2);
    piStack4 = (i16 *)piVar9[0x2];
  } while( true );
}


fn mem_op_1000_0a48(param_1: u8,param_2: u16,param_3: i16,param_4: u32,param_5: u16, in_stack_00000005: u8) -> i32

{
  let uVar1: u16;
  let puVar2: *mut u16;
  let uVar4: u16;
  let uVar3: u16;
  let UVar4: u16;
  let uVar5: u32;
  let puVar1: *mut u16;
  
  UVar4 = (param_4 >> 0x10);
  if ((param_4 + 0x14) == -0x4153) {
    if ((param_3 != 0x0) ||
       ((true && ((s_version__d__d_1050_0012 + 0x6) < param_2)))) {
      if ((param_3 != 0x0) ||
         ((true && ((s_version__d__d_1050_0012 + 0xa) < param_2)))) {
        uVar5 = mem_op_1000_05e2(param_2,param_3,_param_1 & 0xfffd,0x0,param_5);
      }
      else {
        uVar5 = mem_op_1000_0b20(_param_1 & 0xfffd,0x0,param_2,param_5);
      }
    }
    else {
      if ((false) || (param_2 != 0x0)) {
        uVar5 = mem_op_1000_0838(0x0,param_5);
        uVar3 = (uVar5 >> 0x10);
        puVar2 = uVar5;
        if ((uVar5 != 0x0) && ((param_1 & 0x1) != 0x0)) {
          uVar1 = (s_version__d__d_1050_0012 + 0x6);
          for (uVar4 = uVar1 >> 0x1; uVar4 != 0x0; uVar4 -= 0x1) {
            puVar1 = puVar2;
            puVar2 = puVar2 + 0x1;
            *puVar1 = 0x0;
          }
          if ((uVar1 & 0x1) != 0x0) {
            *puVar2 = 0x0;
          }
        }
      }
      else {
        pass1_1000_1e61(param_5,0x4,param_4,UVar4);
        uVar5 = 0x0;
      }
    }
    return (long)uVar5;
  }
  pass1_1000_1e61(param_5,0xa,0x0,0x0);
  return 0x0;
}



fn mem_op_1000_0b20(param_1: u16,param_2: u16,param_3: u16,param_4: u16) -> u16

{
  let puVar1: *mut u16;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let UVar6: u32;
  let puVar7: *mut u16;
  let uVar8: u16;
  let bVar9: bool;
  let uVar10: u32;
  let uStack20: u16;
  let puStack6: *mut u16;
  
  uVar8 = SUB42(&USHORT_1050_1050,0x0);
  uVar2 = param_1 & 0x2;
  uVar4 = param_3 + 0x5 & 0xfffc;
  uVar4 = uVar4 - 0x8 & ~-(uVar4 < 0x8);
  uVar5 = uVar4 + 0x8;
  puVar7 = *(u16 **)(uVar2 * 0x2 + param_2);
  uStack20 = param_1;
  puStack6 = puVar7;
  if (puVar7 == 0x0) goto LAB_1000_0b64;
  do {
    do {
      if ((uVar5 <= *puVar7) &&
         (uVar10 = pass1_1000_0c32(uVar5,uStack20,0x0), uVar10 != 0x0)) {
        *(u16 **)(uVar2 * 0x2 + param_2) = puVar7;
        return uVar10;
      }
      puVar7 = puVar7[0x2];
    } while (puVar7 != puStack6);
LAB_1000_0b64:
    if ((((uStack20 & 0x2) == 0x0) || ((uStack20 & 0x40) != 0x0)) ||
       ((param_2 + 0x32) == 0x0)) {
LAB_1000_0b9e:
      if (((uStack20 & 0x10) != 0x0) ||
         (uVar3 = uVar2,
         UVar6 = mem_op_1000_03c6((param_2 + 0x1a),0x0,uVar2,param_2,param_4,0x0,
                                  '\0'), (UVar6 | uVar3) == 0x0)) {
        if ((uStack20 & 0x20) == 0x0) {
          uVar2 = uVar4 + 0x1007 & 0xf000;
          puVar1 = (param_2 + 0x1e);
          uVar4 = uVar2 + *puVar1;
          uVar2 = (param_2 + 0x20) + CARRY2(uVar2,*puVar1);
          puVar1 = (param_2 + 0x28);
          bVar9 = uVar2 < *puVar1;
          if ((bVar9 || uVar2 == *puVar1) &&
             ((bVar9 || (puVar1 = (param_2 + 0x26),
                        uVar4 < *puVar1 || uVar4 == *puVar1)))) {
            uVar10 = mem_op_1000_05e2(uVar5,0x0,uStack20,param_2,param_4);
            return uVar10;
          }
        }
        return 0x0;
      }
    }
    else {
      param_4 = 0x1000;
      uVar3 = (**(code **)(param_2 + 0x32))();
      if (uVar3 < uVar5) goto LAB_1000_0b9e;
      uStack20 |= 0x40;
    }
    puVar7 = *(u16 **)(uVar2 * 0x2 + param_2);
    puStack6 = puVar7[0x2];
  } while( true );
}


fn  mem_op_1000_131c(param_1: *mut u16,param_2: u16,param_3: i16,param_4: u16)

{
  let HVar1: HGLOBAL16;
  let bVar2: bool;
  let lVar3: i32;
  let uStack10: u16;
  let uStack8: u16;
  let iStack6: i16;
  
  lVar3 = CONCAT22(uStack8,uStack10);
  iStack6 = 0x1;
  if (((param_1 & 0x1000) != 0x0) && ((param_3 != 0x0 || (0xfff0 < param_2)))) {
    param_2 = 0xfff0;
    param_3 = 0x0;
  }
  if ((param_1 & 0x4) != 0x0) {
    lVar3 = mem_op_1000_1558(param_2,param_3,param_4);
  }
  do {
    HVar1 = GLobalAlloc16(param_4,CONCAT22(param_3,param_2));
    uStack10 = lVar3;
    if (HVar1 != 0x0) break;
    bVar2 = iStack6 != 0x0;
    iStack6 = iStack6 + -0x1;
    param_4 = s_tile2_bmp_1050_1538;
  } while (bVar2);
  if ((param_1 & 0x4) != 0x0) {
    if (HVar1 != 0x0) {
      GlobalPageLock16((HGLOBAL16)s_tile2_bmp_1050_1538);
    }
    pass1_1000_15ce((u16 *)uStack10,(lVar3 >> 0x10),
                    s_tile2_bmp_1050_1538);
  }
  if (HVar1 != 0x0) {
    WIN16_GlobalLock16((HGLOBAL16)s_tile2_bmp_1050_1538);
    return;
  }
  return;
}



fn  mem_op_1000_13ce(param_1: u16) -> i32

{
  let HVar1: HGLOBAL16;
  let uVar2: u16;
  let DVar3: u32;
  
  DVar3 = GlobalHandle16(param_1);
  uVar2 = (DVar3 >> 0x10);
  if (DVar3 != 0x0) {
    HVar1 = GlobalFree16((HGLOBAL16)s_tile2_bmp_1050_1538);
    return CONCAT22(uVar2,(HVar1 == 0x0));
  }
  return (long)(uVar2 << 0x10);
}



fn mem_op_1000_1408(param_1: u16,param_2: u16,param_3: u16,param_4: u16)

{
  let HVar1: HGLOBAL16;
  let bVar2: bool;
  let DVar3: u32;
  let iStack12: i16;
  let uStack8: u16;
  
  DVar3 = GlobalHandle16(param_4);
  uStack8 = 0x32;
  iStack12 = 0x1;
  if (((param_1 & 0x1000) != 0x0) && ((param_3 != 0x0 || (0xfff0 < param_2)))) {
    param_2 = 0xfff0;
    param_3 = 0x0;
  }
  if ((param_1 & 0x100) != 0x0) {
    uStack8 = 0x72;
  }
  if ((param_1 & 0x804) != 0x0) {
    uStack8 &= 0xfffd;
  }
  if (DVar3 != 0x0) {
    if ((param_1 & 0x4) != 0x0) {
      GlobalPageUnlock16((HGLOBAL16)s_tile2_bmp_1050_1538);
    }
    do {
      HVar1 = GlobalReAlloc16((HGLOBAL16)s_tile2_bmp_1050_1538,CONCAT22(param_2,uStack8),
                              param_3);
      if (HVar1 != 0x0) break;
      uStack8 &= 0xffcf;
      bVar2 = iStack12 != 0x0;
      iStack12 = iStack12 + -0x1;
    } while (bVar2);
    if ((HVar1 != 0x0) && ((param_1 & 0x4) != 0x0)) {
      GlobalPageLock16((HGLOBAL16)s_tile2_bmp_1050_1538);
    }
    if (HVar1 != 0x0) {
      WIN16_GlobalLock16((HGLOBAL16)s_tile2_bmp_1050_1538);
      return;
    }
  }
  return;
}



fn mem_op_1000_14f2(param_1: u16,param_2: u16,param_3: i16,param_4: u16,param_5: u16, in_AX: u16, in_DX: u16, unaff_CS: u16) -> bool

{
 
  if (((param_1 & 0x1000) != 0x0) || ((param_3 == 0x0 && (param_2 < 0xfff1)))) {
    mem_op_1000_1408(param_1 & 0xfdff | 0x800,param_2,param_3,unaff_CS);
    if ((in_DX | in_AX) != 0x0) {
      return 0x1;
    }
  }
  return 0x0;
}



fn mem_op_1000_1532(param_1: u16) -> u32

{
  let DVar1: u32;
  
  DVar1 = GlobalHandle16(param_1);
  if (DVar1 != 0x0) {
    DVar1 = GlobalSize16((HGLOBAL16)s_tile2_bmp_1050_1538);
    return DVar1;
  }
  return 0x0;
}



fn  mem_op_1000_1558(param_1: u16,param_2: u16,param_3: u16) -> i32

{
  let uVar1: u16;
  let DVar2: u32;
  let uStack12: u16;
  let uStack10: u16;
  let uStack8: u16;
  
  uStack12 = 0x0;
  uStack10 = 0x0;
  uStack8 = 0x8;
  if ((param_2 < 0x9) && ((param_2 < 0x8 || (param_1 == 0x0)))) {
    do {
      while( true ) {
        DVar2 = CONCAT22(uStack10,param_3);
        param_3 = s_tile2_bmp_1050_1538;
        DVar2 = GlobalDOSAlloc16(DVar2);
        uVar1 = DVar2;
        if (uVar1 == 0x0) break;
        0x0 = 0x0;
        &PTR_LOOP_1050_0002 = uStack12;
        uStack12 = uVar1;
      }
      uVar1 = uStack8 & 0x1;
      uStack8 >>= 0x1;
      uStack10 = uStack10 >> 0x1 | (uVar1 != 0x0) << 0xf;
    } while ((param_2 < uStack8) || ((param_2 <= uStack8 && (param_1 <= uStack10))));
  }
  return (long)(uStack12 << 0x10);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn  mem_op_1000_160a(param_1: u16,param_2: u16) -> *mut u8
{
  let puVar1: *mut u8
  
  puVar1 = ret_true_1000_2146();
  if (puVar1 == 0x0) {
    return puVar1;
  }
  if ((PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) == 0x0) {
    DAT_1050_5f30 = 0x1;
    DAT_1050_5f32 = 0x1;
    _PTR_LOOP_1050_5f2c = mem_op_1000_18ec(DAT_1050_5f46,param_1,param_2);
    if (_PTR_LOOP_1050_5f2c != 0x0) {
      if (PTR_LOOP_1050_5f42 != 0x0) {
        pass1_1000_1a54(PTR_LOOP_1050_5f42,_PTR_LOOP_1050_5f2c,
                        (_PTR_LOOP_1050_5f2c >> 0x10),param_2);
      }
      PTR_LOOP_1050_5f2e = (_PTR_LOOP_1050_5f2c >> 0x10);
      if (DAT_1050_5f44 != 0xffff) {
        pass1_1000_1afe(DAT_1050_5f44,PTR_LOOP_1050_5f2c,
                        PTR_LOOP_1050_5f2e);
      }
    }
  }
  empty_fn_1000_214a();
  return PTR_LOOP_1050_5f2c;
}

fn mem_1000_167a(param_1: u16,param_2: u16,param_3: u16) -> u16

{
  let puVar1: *mut u8
  let lVar2: i32;
  
  if ((PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) == 0x0) {
    puVar1 = mem_op_1000_160a(param_3,param_2);
    if ((param_3 | puVar1) == 0x0) {
      return 0x0;
    }
  }
  lVar2 = mem_op_1000_0a48(0x0,param_1,0x0,CONCAT22(PTR_LOOP_1050_5f2e,PTR_LOOP_1050_5f2c)
                           ,param_2);
  return lVar2;
}











fn mem_op_1000_179c(param_1: u16,uchar *param_2,param_3: u16)

{
  let puVar1: *mut u8
  let puVar2: *mut u8
  puVar1 = PTR_LOOP_1050_5f2c;
  puVar2 = PTR_LOOP_1050_5f2e;
  if ((PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) == 0x0) {
    puVar1 = mem_op_1000_160a(param_2,param_3);
    puVar2 = param_2;
  }
  fn_ptr_op_1000_1708(param_1,0x0,0x0,puVar1,puVar2,param_3);
  return;
}


fn  mem_op_1000_18ec(param_1: u16,param_2: u16,param_3: u16) -> u32

{
  let uVar1: u32;
  
  uVar1 = mem_op_1000_1902(param_1,0x0,0x0,0xc,param_3,param_2);
  return uVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn mem_op_1000_1902(param_1: &mut u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
                param_6: u16) -> *mut u8

{
  let p_uvar1: *mut u16;
  let uvar2: u16;
  let bvar3: bool;
    let u_var3: u16;
  let uvar5: u16;
  let p_uvar6: *mut u16;
  let dvar7: u32;
  let u_var8: u32;
  let pu_var1: *mut u16;
  
  if ((param_1 & 0x8000) != 0x0) && (_SHI_INVOKEERRORHANDLER1 != -0x6f70) {
    *param_1 |= 0x1;
  }
  uvar5 = param_6;
  if true {
    loop {
      u_var3 = uvar5;
      p_uvar1 = make_u16_ptr((*param_1 & 0xfffb | 0x1000) as u32);
      mem_op_1000_131c(p_uvar1, 0x100, 0x0, param_5);
      uvar5 = u_var3 | p_uvar1;
      if uvar5 != 0x0 { break; };
      uvar2 = pass1_1000_1e61(param_5, 0x2, 0x0, 0x0);
          if uvar2 == 0 {
              break;
          }
    } ;
    if (u_var3 | p_uvar1) != 0x0 {
      p_uvar1[0x17] = &PTR_PTR_1050_5f1a;
      p_uvar1[0x18] = &USHORT_1050_1050;
      p_uvar1[0x15] = PTR_LOOP_1050_5f1e;
      p_uvar1[0x16] = PTR_LOOP_1050_5f20;
      p_uvar6 = p_uvar1;
      PTR_LOOP_1050_5f1e = p_uvar1;
      PTR_LOOP_1050_5f20 = u_var3;
      for (iVar4 = 0x5; iVar4 != 0x0; iVar4 += -0x1) {
        pu_var1 = p_uvar6;
        p_uvar6 = p_uvar6 + 0x1;
        *pu_var1 = 0x0;
      }
      p_uvar1[0x5] = 0x0;
      p_uvar1[0x7] = 0x0;
      p_uvar1[0x6] = 0x0;
      p_uvar1[0x9] = 0x0;
      p_uvar1[0x8] = 0x0;
      p_uvar1[0xa] = 0xbead;
      p_uvar1[0xb] = param_1 & 0xfffd;
      p_uvar1[0xc] = 0x0;
      p_uvar1[0xd] = 0x2000;
      p_uvar1[0xe] = 0x800;
      dvar7 = mem_op_1000_1532(param_5);
      p_uvar1[0xf] = dvar7;
      p_uvar1[0x10] = (dvar7 >> 0x10);
      p_uvar1[0x12] = 0x0;
      p_uvar1[0x11] = 0x0;
      p_uvar1[0x13] = 0xfffe;
      p_uvar1[0x14] = 0xffff;
      p_uvar1[0x19] = 0x0;
      p_uvar1[0x1a] = 0x0;
      p_uvar1[0x20] = 0x0;
      p_uvar1[0x1f] = 0x0;
      bvar3 = pass1_1000_1afe(param_4, p_uvar1, u_var3);
      if (bvar3 != 0x0) {
        if ((param_3 | param_2) != 0x0) {
          p_uvar6 = p_uvar1;
          uvar5 = u_var3;
          u_var8 = pass1_1000_52be(param_2, param_3, param_4, 0x0);
          pass1_1000_010c(0x1, u_var8, (u_var8 >> 0x10), p_uvar6, uvar5,
                          param_5);
        }
        return CONCAT22(u_var3, p_uvar1);
      }
      mem_op_1000_1b9a(0x0, p_uvar1, u_var3, param_5);
    }
  }
  return 0x0;
}


fn mem_op_1000_1b68(param_1: u16,param_2: u16,param_3: u16,param_4: u16) -> u32

{
  let uVar1: u32;
  
  if ((param_3 + 0x14) != -0x4153) {
    pass1_1000_1e61(param_2,0xa,0x0,0x0);
    return param_1 << 0x10;
  }
  uVar1 = mem_op_1000_1b9a(0x0,param_3,param_4,param_2);
  return uVar1;
}



fn mem_op_1000_1b9a(param_1: u16,param_2: u32,param_3: u16,param_4: u16) -> u32

{
  let uVar1: u16;
  let uVar2: u32;
  let uVar3: u16;
  let uVar4: u16;
  let iVar5: i16;
  let lVar6: i32;
  let puStack8: *mut u16;
  let uStack4: u16;
  
  (param_2 + 0x14) = 0x0;
  uStack4 = 0x0;
  do {
    iVar5 = (uStack4 * 0x2);
    if (iVar5 != 0x0) {
      do {
        uVar2 = (iVar5 + 0x8);
        (uVar2 + 0xc) = 0x0;
        mem_op_1000_13ce(param_4);
        iVar5 = (iVar5 + 0x4);
      } while ((uStack4 * 0x2) != iVar5);
    }
    uStack4 += 0x1;
  } while (uStack4 < 0x5);
  uVar4 = (param_2 + 0x12);
  uVar3 = (param_2 + 0x10);
  while( true ) {
    puStack8 = CONCAT22(uVar4,uVar3);
    if ((uVar4 | uVar3) == 0x0) break;
    uVar1 = *puStack8;
    uVar4 = (uVar3 + 0x2);
    mem_op_1000_13ce(param_4);
    uVar3 = uVar1;
  }
  pass1_1000_20a2(param_2,param_3);
  lVar6 = mem_op_1000_13ce(param_4);
  return CONCAT22((lVar6 >> 0x10),0x1);
}



fn mem_op_1000_1dfa(param_1: i16,param_2: u8,param_3: u16,param_4: u16) -> bool
{
    let uVar1: u32;
  let uVar2: u16;
  
  if ((param_2 & 0x4) == 0x0) {
    uVar2 = (byte)(((byte)(-((param_2 & 0x2) == 0x0) >> 0x8) & 0xfe) + 0x92)
            << 0x8;
  }
  else {
    uVar2 = 0x1800;
  }
  if (((param_4 == 0x0) || (false)) ||
     ((param_4 & 0xff00 &
      (byte)(((byte)(-((param_2 & 0x4) == 0x0) >> 0x8) & 0x82) + 0x18) << 0x8)
      != uVar2)) {
    return 0x1;
  }
  if (param_1 != 0x0) {
    uVar1 = SegmentLimit(param_4);
    if (CARRY2(param_3,param_1 - 0x1U)) {
      return 0x1;
    }
    if (uVar1 < param_3 + (param_1 - 0x1U)) {
      return 0x1;
    }
  }
  return 0x0;
}


fn mem_op_1000_21b6(param_1: u16,param_2: u16) -> bool

{
  let BVar1: bool;
  
  BVar1 = mem_op_1000_1dfa(0x0,0x4,param_1,param_2);
  return BVar1 == 0x0;
}


fn mem_1000_2bb6(param_1: u16,i16 *param_2,param_3: i16,param_4: u16,param_5: u16,
             param_6: u16,param_7: u8,param_8: u16) -> u16

{
  let piVar1: *mut i16;
  let iVar2: i16;
  let piVar3: *mut i16;
  let bVar4: u8;
  let puVar5: *mut u8
  let puVar6: *mut u8
  let puVar7: *mut u8
  let uStack4: u16;
  let iStack2: i16;
  
  piVar3 = param_2;
  iStack2 = param_3 + 0x1;
  uStack4 = SUB42(&USHORT_1050_1050,0x0);
  bVar4 = *(byte *)(param_2 + 0x5);
  if (((bVar4 & 0x82) != 0x0) && ((bVar4 & 0x40) == 0x0)) {
    param_2[0x2] = 0x0;
    if ((bVar4 & 0x1) != 0x0) {
      if ((bVar4 & 0x10) == 0x0) goto LAB_1000_2c37;
      *param_2 = param_2[0x3];
      bVar4 &= 0xfe;
    }
    *(byte *)(param_2 + 0x5) = bVar4 & 0xef | 0x2;
    puVar7 = *(byte *)(param_2 + 0xb);
    if (((bVar4 & 0x8) == 0x0) &&
       (((bVar4 & 0x4) != 0x0 ||
        (((*(byte *)(param_2 + 0x78) & 0x1) == 0x0 &&
         (((PTR_LOOP_1050_61ec != 0x0 &&
           (((param_2 == (i16 *)0x621c || (param_2 == (i16 *)0x6228)) &&
            ((puVar7[0x5f90] & 0x40) != 0x0)))) ||
          (mem_1000_2ce8(param_2,param_8,param_5), (*(byte *)(piVar3 + 0x5) & 0x8) == 0x0)
          ))))))) {
      puVar5 = mixed_dos3_call_1000_39f2
                         (puVar7,CONCAT22(param_6,&param_1),
                          (&PTR_LOOP_1050_0000 + 0x1),param_4,param_5,
                          param_6,param_7);
      puVar6 = (&PTR_LOOP_1050_0000 + 0x1);
    }
    else {
      iVar2 = piVar3[0x3];
      puVar6 = (*piVar3 - iVar2);
      *piVar3 = iVar2 + 0x1;
      piVar3[0x2] = piVar3[0x79] + -0x1;
      if (puVar6 == 0x0) {
        puVar5 = 0x0;
        if ((puVar7[0x5f90] & 0x20) != 0x0) {
          mixed_dos3_call_1000_3636(puVar7,0x0,0x0,0x2,&iStack2);
          puVar5 = 0x0;
          puVar6 = puVar5;
        }
      }
      else {
        puVar5 = mixed_dos3_call_1000_39f2
                           (puVar7,CONCAT22(piVar3[0x4],piVar3[0x3]),puVar6,
                            param_4,param_5,param_6,param_7);
      }
      **(u8 **)(piVar3 + 0x3) = param_1;
    }
    if (puVar5 == puVar6) {
      return param_1 & 0xff;
    }
  }
LAB_1000_2c37:
  piVar1 = piVar3 + 0x5;
  *(byte *)piVar1 = *(byte *)piVar1 | 0x20;
  return 0xffff;
}


fn mem_1000_2ce8(i16 *param_1,param_2: u16,param_3: u16)
{
  let piVar1: *mut i16;
  let uVar2: u16;
  
  uVar2 = mem_1000_167a(0x200,param_3,param_2);
  if (param_2 == 0x0) {
    piVar1 = param_1 + 0x5;
    *(byte *)piVar1 = *(byte *)piVar1 | 0x4;
    param_1[0x79] = 0x1;
    param_2 = &USHORT_1050_1050;
    uVar2 = param_1 + 0xf1;
  }
  else {
    piVar1 = param_1 + 0x5;
    *(byte *)piVar1 = *(byte *)piVar1 | 0x8;
    param_1[0x79] = 0x200;
  }
  param_1[0x1] = param_2;
  *param_1 = uVar2;
  param_1[0x4] = param_2;
  param_1[0x3] = uVar2;
  param_1[0x2] = 0x0;
  return;
}



u32 
mixed_mem_op_1000_3c51
          (HGLOBAL16 param_1,HGLOBAL16 param_2,param_3: i16,param_4: u16,
          param_5: u16,param_6: i16)

{
  let piVar1: *mut i16;
  char *pcVar2;
  LPCSTR str;
  let piVar3: *mut i16;
  let uVar4: u16;
  let flags: u16;
  HGLOBAL16 HVar5;
  let piVar6: *mut i16;
  char *pcVar7;
  let DVar8: u32;
  HGLOBAL16 HVar9;
  let iVar10: i16;
  let iVar11: i16;
  
  if ((*(byte *)(param_3 + 0x2) & 0x4) == 0x0) {
    HVar9 = *(HGLOBAL16 *)(param_3 + 0x6);
    flags = 0x0;
    HVar5 = param_1;
    if (param_1 == 0x0) {
      if (false) goto LAB_1000_3cb6;
      flags = 0x1;
    }
    uVar4 = 0x2;
    if (true) {
      uVar4 = 0x20;
    }
    param_5 = (u16_t)s_tile2_bmp_1050_1538;
    param_1 = GlobalReAlloc16(0x1000,CONCAT22(param_1,uVar4),flags);
    if (param_1 == 0x0) {
LAB_1000_3cb6:
      return CONCAT22(HVar5,param_1);
    }
    if (param_1 == HVar9) {
      param_5 = (u16_t)s_tile2_bmp_1050_1538;
      param_1 = param_2;
      DVar8 = GlobalSize16((HGLOBAL16)s_tile2_bmp_1050_1538);
      if (DVar8 != 0x0) {
        HVar5 = param_1;
        if ((*(byte *)(HVar9 + 0x2) & 0x4) != 0x0) {
          HVar5 = param_1 - 0x1;
          *(HGLOBAL16 *)(HVar9 - 0x2) = HVar5;
        }
        goto LAB_1000_3cb6;
      }
    }
  }
  iVar11 = 0x12;
  iVar10 = 0x12;
  pass1_1000_25a8(param_4,param_5);
  pass1_1000_2913(iVar10,param_4,param_5);
  str = poss_str_op_1000_28dc(iVar11);
  if (str != 0x0) {
    iVar10 = 0x9;
    if (*str == 'M') {
      iVar10 = 0xf;
    }
    str = str + iVar10;
    iVar10 = 0x22;
    pcVar7 = str;
    do {
      if (iVar10 == 0x0) break;
      iVar10 += -0x1;
      pcVar2 = pcVar7;
      pcVar7 = pcVar7 + 0x1;
    } while (*pcVar2 != '\r');
    pcVar7[-0x1] = '\0';
  }
  FatalAppExit16(param_5,str);
  FatalExit();
  piVar6 = (i16 *)&PTR_LOOP_1050_63fe;
  do {
    piVar1 = piVar6;
    piVar6 = piVar6 + 0x1;
    iVar10 = *piVar1;
    piVar3 = piVar6;
    if ((iVar10 == param_6) || (piVar3 = (i16 *)(iVar10 + 0x1), piVar3 == (i16 *)0x0)) {
      return CONCAT22(param_6,piVar3);
    }
    iVar10 = -0x1;
    do {
      if (iVar10 == 0x0) break;
      iVar10 += -0x1;
      piVar1 = piVar6;
      piVar6 = (i16 *)(piVar6 + 0x1);
    } while (*piVar1 != '\0');
  } while( true );
}



fn free_mem_1000_407a(param_1: u16,param_2: u16,param_3: u16)
{
  let uVar1: u16;
  HGLOBAL16 handle;
  
  handle = 0x1000;
  if (false) {
    uVar1 = pass1_1000_41e0(param_2);
    if (uVar1 == 0x0) {
      return;
    }
    handle = (HGLOBAL16)s_tile2_bmp_1050_1538;
    GlobalUnlock16(0x1000);
  }
  GlobalFree16(handle);
  return;
}


