use crate::win_struct::HINSTANCE16;

fn  pass1_1000_0368(param_1: u16, param_2: u16, param_3: u16)

{
  let puVar1: *mut u16;
  
  if ((param_1 + 0x4) == param_1) {
    (param_3 + param_2 * 0x2) = 0x0;
  }
  else {
    ((param_1 + 0x6) + 0x4) = (param_1 + 0x4);
    ((param_1 + 0x4) + 0x6) = (param_1 + 0x6);
    puVar1 = (param_2 * 0x2 + param_3);
    if (*puVar1 == param_1) {
      *puVar1 = (param_1 + 0x4);
    }
  }
  (param_1 + 0x4) = (param_3 + 0xa);
  (param_3 + 0xa) = param_1;
  return;
}

fn  pass1_1000_05b4(param_1: u8,param_2: i16)

{
  (param_2 + 0xa) = 0x1;
  (param_2 + 0x8) = 0x668;
  *(byte *)(param_2 + 0x13) = -((param_1 & 0x2) != 0x0) & 0x2;
  (param_2 + 0x10) = 0x0;
  (param_2 + 0xe) = 0x0;
  return;
}

fn pass1_1000_0782(param_1: u32,param_2: u16,param_3: i16, in_stack_00000004: u16) -> u16

{
  (param_3 + 0xe) = 0x0;
  (param_3 + 0x10) = param_3 + 0x14;
  (param_3 + 0x8) = 0x9a0;
  pass1_1000_07ac((param_1 + 0x18),param_2,param_3);
  return 0x1;
}



fn  pass1_1000_07ac(param_1: u16,param_2: i16,param_3: i16)

{
  let puVar1: *mut u16;
  let iVar2: i16;
  let uVar3: u16;
  
  puVar1 = *(u16 **)(param_3 + 0x10);
  *(u16 **)(param_3 + 0xe) = puVar1;
  uVar3 = param_2 + (param_3 - puVar1);
  iVar2 = puVar1 + (uVar3 - uVar3 % param_1);
  (param_3 + 0x10) = iVar2;
  while (puVar1 < (iVar2 - param_1)) {
    *puVar1 = (puVar1 + param_1);
    puVar1 = (puVar1 + param_1);
  }
  *puVar1 = 0x0;
  return;
}



unsafe fn  pass1_1000_07fc(param_1: u16,param_2: u32) -> *mut astruct_99

{
  astruct_99 *paVar1;
  
  if ((param_2 + 0x14) != -0x4153) {
    pass1_1000_1e61(param_1,0xa,0x0,0x0);
    return (astruct_99 *)0x0;
  }
  paVar1 = (astruct_99 *)mem_op_1000_0838(0x0,param_1);
  return paVar1;
}

fn  pass1_1000_093a(i16 *param_1,param_2: u16,param_3: u16) -> u16

{
  let piVar1: *mut i16;
  
  if (&PTR_LOOP_1050_000c != -0x352f) {
    pass1_1000_1e61(param_3,0xe,0x0,0x0);
    return 0x0;
  }
  *param_1 = &PTR_LOOP_1050_000e;
  if (*param_1 == 0x0) {
    &DAT_1050_0004 = 0x1;
  }
  *(i16 **)&PTR_LOOP_1050_000e = param_1;
  piVar1 = (i16 *)&PTR_LOOP_1050_000a;
  *piVar1 = *piVar1 + -0x1;
  if (*piVar1 == 0x0) {
    mem_op_1000_0510(0x1,0x0,param_3);
  }
  return 0x1;
}



fn  pass1_1000_09a0(param_1: *mut u16,param_2: u16) -> *mut u8

{
  let puVar1: *mut u8
  let uVar2: u32;
  
  *param_1 = PTR_LOOP_1050_000e;
  if (PTR_LOOP_1050_000e == 0x0) {
    *DAT_1050_0004 = 0x1;
  }
  PTR_LOOP_1050_000a = PTR_LOOP_1050_000a + -0x1;
  puVar1 = PTR_LOOP_1050_000e;
  PTR_LOOP_1050_000e = param_1;
  if (PTR_LOOP_1050_000a == 0x0) {
    uVar2 = mem_op_1000_0510(0x1,0x0,param_2);
    puVar1 = uVar2;
  }
  return puVar1;
}



fn  pass1_1000_09ca(param_1: i16,param_2: *mut u32) -> u16

{
  let puVar1: *mut u16;
  let iVar2: i16;
  let uVar3: u32;
  let puVar4: *mut u16;
  
  puVar1 = param_2 + 0xa;
  puVar4 =
           ((param_2 + (param_1 - puVar1) + -0x6 & 0xfffcU) + puVar1);
  *puVar4 = 0x1;
  param_2[0x7] = puVar1;
  puVar4[0x2] = puVar4;
  puVar4[0x1] = puVar4;
  param_2[0x8] = puVar4;
  if ((*(byte *)(param_2 + 0x6) & 0x7) == 0x2) {
    param_2[0x9] = 0x8;
  }
  else {
    uVar3 = param_2;
    iVar2 = (uVar3 + 0x18);
    param_2[0x9] = (iVar2 - 0x5U & ~-(iVar2 + 0x3U < 0x8)) + 0x8;
  }
  puVar4[-0x1] = (puVar4 - puVar1);
  *puVar1 = (puVar4 - puVar1) | 0x2;
  param_2[0xc] = puVar4;
  param_2[0xb] = puVar4[0x1];
  *(u16 **)(puVar4[0x1] + 0x4) = puVar1;
  puVar4[0x1] = puVar1;
  param_2[0x4] = 0xe08;
  return *puVar1 & 0xfffc;
}


fn  pass1_1000_0c32(param_1: u16,param_2: u16,param_3: u16) -> u32

{
  let puVar1: *mut u16;
  byte *pbVar2;
  let piVar3: *mut i16;
  let uVar4: u32;
  let uVar5: u16;
  let puVar6: *mut u16;
  let iVar7: i16;
  let puVar8: *mut u16;
  let uVar9: u16;
  let uStack14: u16;
  let puStack8: *mut u16;
  let uStack6: u16;
  
  puVar8 = *(u16 **)(param_3 + 0xe);
  uStack6 = 0x0;
  puVar6 = puVar8;
  while( true ) {
    do {
      uVar5 = *puVar6;
      if (param_1 <= uVar5) {
        uVar5 = (uVar5 & 0xfffc) - param_1;
        puVar1 = (param_3 + 0x12);
        puStack8 = puVar6;
        if (*puVar1 < uVar5 || *puVar1 == uVar5) {
          uStack14 = param_1;
          if ((param_2 & 0x6) == 0x0) {
            puStack8 = (uVar5 + puVar6);
            puStack8[-0x1] = uVar5;
            *puVar6 = uVar5 | 0x2;
            puVar8 = puVar6[0x1];
            pbVar2 = (byte *)(puStack8 + param_1);
            *pbVar2 = *pbVar2 | 0x2;
            *puStack8 = param_1 | 0x1;
          }
          else {
            *puVar6 = param_1 & 0xff00 | *(byte *)puVar6 & 0x2 | param_1 & 0xff | 0x1;
            (puVar6[0x2] + 0x2) = puVar6[0x1];
            (puVar6[0x1] + 0x4) = puVar6[0x2];
            puVar8 = (puVar6 + param_1);
            (puVar8 + (uVar5 - 0x2)) = uVar5;
            *puVar8 = uVar5 | 0x2;
            uVar5 = (param_3 + 0x10);
            puVar8[0x2] = uVar5;
            puVar8[0x1] = (uVar5 + 0x2);
            *(u16 **)((uVar5 + 0x2) + 0x4) = puVar8;
            *(u16 **)(uVar5 + 0x2) = puVar8;
          }
        }
        else {
          puVar8 = puVar6[0x1];
          *(u16 **)(puVar6[0x2] + 0x2) = puVar8;
          (puVar6[0x1] + 0x4) = puVar6[0x2];
          puVar1 = puVar6;
          *(byte *)puVar1 = *(byte *)puVar1 | 0x1;
          uStack14 = *puVar6 & 0xfffc;
          (puVar6 + uStack14) = (puVar6 + uStack14) | 0x2;
        }
        *(u16 **)(param_3 + 0xe) = puVar8;
        if ((param_2 & 0x1) != 0x0) {
          puVar6 = puStack8;
          for (uVar5 = uStack14 - 0x2 >> 0x1; puVar6 = puVar6 + 0x1, uVar5 != 0x0;
              uVar5 -= 0x1) {
            *puVar6 = 0x0;
          }
          if ((uStack14 - 0x2 & 0x1) != 0x0) {
            *puVar6 = 0x0;
          }
        }
        if (((param_2 & 0x2) != 0x0) && (puVar8[0x1] == puVar8[0x2])) {
          **(u16 **)(param_3 + 0x4) =
               **(u16 **)((param_3 + 0x10) + 0x2) & 0xfffc;
          uVar4 = (param_3 + 0x4);
          pbVar2 = (byte *)(uVar4 + 0x3);
          *pbVar2 = *pbVar2 | 0x80;
        }
        piVar3 = (i16 *)(param_3 + 0xa);
        *piVar3 = *piVar3 + 0x1;
        return CONCAT22(0x1050,puStack8 + 0x1);
      }
      if (uStack6 < uVar5) {
        uStack6 = uVar5;
      }
      puVar6 = puVar6[0x1];
    } while (puVar6 != puVar8);
    if (((param_2 & 0x2) == 0x0) || ((param_2 & 0x40) != 0x0)) break;
    uVar4 = param_3;
    uVar9 = (uVar4 >> 0x10);
    iVar7 = uVar4;
    if ((iVar7 + 0x34) == 0x0) break;
    uStack6 = (**(code **)(iVar7 + 0x34))();
    if ((uStack6 < param_1) || (puVar6 = *(u16 **)(param_3 + 0xe), puVar6 == 0x0)
       ) break;
  }
  **(u16 **)(param_3 + 0x4) = uStack6 & 0xfffc;
  return 0x0;
}


fn  pass1_1000_0e08(param_1: i16,param_2: u16) -> u16

{
  let puVar1: *mut u16;
  byte *pbVar2;
  let uVar3: u16;
  let puVar4: *mut u16;
  let puVar5: *mut u16;
  let bVar6: bool;
  let uVar7: u32;
  
  puVar5 = (param_1 + -0x2);
  bVar6 = (*(byte *)puVar5 & 0x2) != 0x0;
  if (bVar6) {
    puVar1 = puVar5;
    *(byte *)puVar1 = *(byte *)puVar1 & 0xfe;
  }
  else {
    puVar4 = (puVar5 - (param_1 + -0x4));
    puVar1 = puVar4;
    *puVar1 = *puVar1 + (*puVar5 & 0xfffc);
    puVar5 = puVar4;
  }
  puVar4 = ((*puVar5 & 0xfffc) + puVar5);
  if ((*(byte *)puVar4 & 0x1) == 0x0) {
    puVar1 = puVar5;
    *puVar1 = *puVar1 + (*puVar4 & 0xfffc);
    if (puVar4 == PTR_LOOP_1050_000e) {
      PTR_LOOP_1050_000e = puVar5;
    }
    (puVar4[0x2] + 0x2) = puVar4[0x1];
    (puVar4[0x1] + 0x4) = puVar4[0x2];
    puVar4 = ((*puVar5 & 0xfffc) + puVar5);
  }
  puVar4[-0x1] = *puVar5 & 0xfffc;
  uVar3 = *DAT_1050_0004;
  puVar1 = puVar4 + -0x1;
  if (uVar3 <= *puVar1 && *puVar1 != uVar3) {
    uVar3 = *puVar5 & 0xfffc;
    *DAT_1050_0004 = uVar3;
  }
  puVar1 = puVar4;
  *(byte *)puVar1 = *(byte *)puVar1 & 0xfd;
  if (bVar6) {
    if (*(u8 **)(PTR_LOOP_1050_0010 + 0x2) != PTR_LOOP_1050_0010) {
      pbVar2 = (byte *)(DAT_1050_0004 + 0x3);
      *pbVar2 = *pbVar2 & 0x7f;
    }
    puVar5[0x2] = PTR_LOOP_1050_0010;
    uVar3 = (PTR_LOOP_1050_0010 + 0x2);
    puVar5[0x1] = uVar3;
    *(u16 **)((PTR_LOOP_1050_0010 + 0x2) + 0x4) = puVar5;
    *(u16 **)(PTR_LOOP_1050_0010 + 0x2) = puVar5;
  }
  PTR_LOOP_1050_000a = PTR_LOOP_1050_000a + -0x1;
  if (PTR_LOOP_1050_000a == 0x0) {
    uVar7 = mem_op_1000_0510(0x1,0x0,param_2);
    uVar3 = uVar7;
  }
  return uVar3;
}



fn pass1_1000_0ed4(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
               param_6: *mut u16,param_7: u16) -> i32

{
  let puVar1: *mut u16;
  let puVar2: *mut u16;
  let uVar3: u16;
  u16 **ppuVar4;
  let uVar5: u16;
  let puVar6: *mut u16;
  let puVar7: *mut u16;
 let UVar8: u16;
  let UVar9: u16;
  let UVar10: u16;
  let lStack12: i32;
  let uStack8: u16;
  let UStack: u16;
  let UStack4: u16;
  
  if ((&PTR_LOOP_1050_000c & 0xfff8) == 0xcad0) {
    UStack6 = 0x0;
    UStack4 = &PTR_LOOP_1050_0002;
    if ((param_3 & 0x8) == 0x0) {
      ppuVar4 = &param_6;
    }
    else {
      ppuVar4 = (u16 **)0x0;
      param_2 = 0x0;
    }
    lStack12 = CONCAT22(param_2,ppuVar4);
    uStack8 = pass1_1000_0fb8(param_1,param_4,param_6,param_5,param_3,
                              ppuVar4,param_2);
    if (uStack8 == 0x0) {
      return CONCAT22(param_7,param_6);
    }
    if ((param_3 & 0x8) == 0x0) {
      lStack12 = mem_op_1000_0a48((byte)param_3,param_4,param_5,CONCAT22(UStack4,UStack6),
                                  param_1);
      uVar3 = (lStack12 >> 0x10);
      puVar7 = lStack12;
      if (lStack12 != 0x0) {
        puVar6 = param_6;
        for (uVar5 = uStack8 >> 0x1; uVar5 != 0x0; uVar5 -= 0x1) {
          puVar2 = puVar7;
          puVar7 = puVar7 + 0x1;
          puVar1 = puVar6;
          puVar6 = puVar6 + 0x1;
          *puVar2 = *puVar1;
        }
        for (uVar5 = ((uStack8 & 0x1) != 0x0); uVar5 != 0x0; uVar5 -= 0x1) {
          puVar2 = puVar7;
          puVar7 = (puVar7 + 0x1);
          puVar1 = puVar6;
          puVar6 = (puVar6 + 0x1);
          *puVar2 = *puVar1;
        }
        call_fn_ptr_1000_0dc6(param_6,param_7,param_1);
      }
      return lStack12;
    }
    if ((param_5 | param_4) == 0x0) {
      return 0x0;
    }
    UVar8 = 0x5;
    UVar9 = UStack6;
    UVar10 = UStack4;
  }
  else {
    UVar8 = 0xe;
    UVar9 = 0x0;
    UVar10 = 0x0;
  }
  pass1_1000_1e61(param_1,UVar8,UVar9,UVar10);
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1000_0fb8(param_1: u16,param_2: u16,param_3: i16,param_4: u16,param_5: u16,
                    param_6: *mut u16,param_7: u16) -> u16

{
  let puVar1: *mut u16;
  let bVar2: u8;
  let uVar3: u16;
  let Bvar4: bool;
  let iVar5: i16;
  let uVar6: u16;
  let puVar7: *mut u16;
  let puVar8: *mut u16;
  let uVar9: u32;
  let uStack4: u16;
  
  if ((param_4 | param_2) == 0x0) {
    pass1_1000_1e61(param_1,0x4,PTR_LOOP_1050_0000,PTR_LOOP_1050_0002);
    if ((param_7 | param_6) != 0x0) {
      param_6[0x1] = 0x0;
      *param_6 = 0x0;
      return 0x0;
    }
    return 0x1;
  }
  bVar2 = (byte)PTR_LOOP_1050_000c & 0x7;
  if (((byte)PTR_LOOP_1050_000c & 0x7) != 0x0) {
    if (bVar2 == 0x1) {
      uVar3 = (PTR_LOOP_1050_0000 + 0x18);
      if (false) {
        return 0x0;
      }
      if (param_4 == 0x0) {
        if (param_2 <= uVar3) {
          return 0x0;
        }
        return uVar3;
      }
      return uVar3;
    }
    if (bVar2 != 0x2) {
      if (bVar2 != 0x3) {
        if ((param_7 | param_6) != 0x0) {
          param_6[0x1] = 0x0;
          *param_6 = 0x0;
          return 0x0;
        }
        return 0x1;
      }
      if ((((param_7 | param_6) != 0x0) && (param_4 == 0x0)) &&
         ((false || (param_2 <= (PTR_LOOP_1050_0000 + 0x1c))))) {
        uVar9 = pass1_1000_1284(CONCAT22(0x1050,param_3),param_1);
        uVar3 = uVar9;
        if (uVar9 <= CONCAT22(param_4,param_2)) {
          return uVar3;
        }
        if ((false) && (uVar3 <= param_2)) {
          return uVar3;
        }
        return param_2;
      }
      iVar5 = mem_1000_0670(param_5,(i16 *)CONCAT22(param_7,param_6),param_2,0x0,
                            param_4,param_1);
      if (iVar5 != 0x0) {
        return 0x0;
      }
      if ((param_7 | param_6) != 0x0) {
        return 0x0;
      }
      return 0x1;
    }
  }
  puVar8 = (param_3 + -0x2);
  uVar3 = *puVar8 & 0x7ffc;
  uStack4 = uVar3 - 0x2;
  if ((*(byte *)(param_3 + -0x1) & 0x80) != 0x0) {
    uStack4 = uVar3 - 0x6;
  }
  if ((true) && ((param_4 != 0x0 || (uStack4 < param_2)))) {
    if (true) {
      if (param_4 != 0x0) {
        return uStack4;
      }
      if ((PTR_LOOP_1050_0000 + 0x1c) < param_2) {
        return uStack4;
      }
    }
  }
  BVar4 = pass1_1000_115c(param_2,puVar8);
  if (BVar4 == 0x0) {
    return uStack4;
  }
  if ((param_5 & 0x1) != 0x0) {
    uVar3 = (*puVar8 & 0x7ffc) - 0x2;
    if (uStack4 < param_2) {
      puVar7 = (uStack4 + param_3);
      iVar5 = -uStack4;
    }
    else {
      if (uVar3 <= param_2) {
        return 0x0;
      }
      puVar7 = (param_2 + param_3);
      iVar5 = -param_2;
    }
    uVar3 += iVar5;
    for (uVar6 = uVar3 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
      puVar1 = puVar7;
      puVar7 = puVar7 + 0x1;
      *puVar1 = 0x0;
    }
    if ((uVar3 & 0x1) != 0x0) {
      *puVar7 = 0x0;
    }
  }
  return 0x0;
}



fn  pass1_1000_115c(param_1: i16,param_2: *mut u16) -> bool

{
  byte *pbVar1;
  let puVar2: *mut u16;
  let uVar3: u16;
  let uVar4: u16;
  let puVar5: *mut u16;
  let iVar6: i16;
  let uStack4: u16;
  
  uVar3 = *param_2 & 0x7ffc;
  uVar4 = param_1 + 0x5U & 0xfffc;
  uVar4 = (uVar4 - 0x8 & ~-(uVar4 < 0x8)) + 0x8;
  if (uVar3 < uVar4) {
    puVar5 = (uVar3 + param_2);
    if (((*(byte *)puVar5 & 0x1) != 0x0) || ((*puVar5 & 0xfffc) + uVar3 < uVar4)) {
      return 0x0;
    }
    if (puVar5 == PTR_LOOP_1050_000e) {
      PTR_LOOP_1050_000e = puVar5[0x1];
    }
    (puVar5[0x2] + 0x2) = puVar5[0x1];
    (puVar5[0x1] + 0x4) = puVar5[0x2];
    uStack4 = ((*puVar5 & 0xfffc) + uVar3) - uVar4;
    if (uStack4 < s_version__d__d_1050_0012._0_2_) {
      puVar2 = param_2;
      *puVar2 = *puVar2 + (*puVar5 & 0xfffc);
      pbVar1 = (byte *)(puVar5 + (*puVar5 & 0xfffc));
      *pbVar1 = *pbVar1 | 0x2;
      return 0x1;
    }
  }
  else {
    uStack4 = uVar3 - uVar4;
    if (uStack4 < s_version__d__d_1050_0012._0_2_) {
      return 0x1;
    }
    puVar5 = (uVar3 + param_2);
    if ((*(byte *)puVar5 & 0x1) == 0x0) {
      uStack4 += *puVar5 & 0xfffc;
      if (puVar5 == PTR_LOOP_1050_000e) {
        PTR_LOOP_1050_000e = puVar5[0x1];
      }
      (puVar5[0x2] + 0x2) = puVar5[0x1];
      (puVar5[0x1] + 0x4) = puVar5[0x2];
    }
    if (*DAT_1050_0004 < uStack4) {
      *DAT_1050_0004 = uStack4;
    }
  }
  *param_2 = *param_2 & 0x8003 | uVar4;
  (uVar4 + param_2) = uStack4 | 0x2;
  iVar6 = uVar4 + param_2;
  (iVar6 + 0x4) = PTR_LOOP_1050_0010;
  (iVar6 + 0x2) = (PTR_LOOP_1050_0010 + 0x2);
  ((PTR_LOOP_1050_0010 + 0x2) + 0x4) = iVar6;
  (PTR_LOOP_1050_0010 + 0x2) = iVar6;
  ((byte *)(iVar6 + uStack4) + -0x2) = uStack4;
  pbVar1 = (byte *)(iVar6 + uStack4);
  *pbVar1 = *pbVar1 & 0xfd;
  return 0x1;
}



fn  pass1_1000_1284(param_1: u32,param_2: u16) -> u32

{
  let bVar1: u8;
  let uVar2: u16;
  let uVar3: u32;
  let bVar4: u8;
  let uVar5: u16;
  let bVar6: bool;
  let DVar7: u32;
  let uStack6: u16;
  let iStack4: i16;
  
  if ((&PTR_LOOP_1050_000c & 0xfff8) != 0xcad0) {
    pass1_1000_1e61(param_2,0xe,0x0,0x0);
    return 0xffffffff;
  }
  bVar1 = *(byte *)&PTR_LOOP_1050_000c;
  bVar4 = bVar1 & 0x7;
  if ((bVar1 & 0x7) != 0x0) {
    if (bVar4 == 0x1) {
      uVar3 = 0x0;
      return (uVar3 + 0x18);
    }
    if (bVar4 != 0x2) {
      if (bVar4 != 0x3) {
        return 0xffffffff;
      }
      DVar7 = mem_op_1000_1532(param_2);
      return CONCAT22((DVar7 >> 0x10) - (DVar7 < 0x14),DVar7 - 0x14
                     );
    }
  }
  uVar2 = (param_1 + -0x2);
  uVar5 = uVar2 & 0x7ffc;
  uStack6 = uVar5 - 0x2;
  iStack4 = 0x0;
  if ((uVar2 & 0x8000) != 0x0) {
    bVar6 = uStack6 < 0x4;
    uStack6 = uVar5 - 0x6;
    iStack4 = -bVar6;
  }
  return CONCAT22(iStack4,uStack6);
}


fn pass1_1000_15ce(param_1: *mut u16,param_2: u16,param_3: u16)

{
  let puVar1: *mut u16;
  let uVar2: u16;
  
  uVar2 = param_2 | param_1;
  while (uVar2 != 0x0) {
    puVar1 = *param_1;
    param_2 = param_1[0x1];
    GlobalDOSFree16(param_3);
    param_1 = puVar1;
    param_3 = s_tile2_bmp_1050_1538;
    uVar2 = param_2 | puVar1;
  }
  return;
}



fn pass1_1000_16aa(param_1: *mut u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
               param_6: u16) -> u16

{
  let uVar1: u16;
  let lVar2: i32;
  
  if ((param_2 | param_1) == 0x0) {
    uVar1 = mem_1000_167a(param_3,param_5,param_4);
    return uVar1;
  }
  if (param_3 == 0x0) {
    pass1_1000_16ee(param_1,param_2,param_5);
    return 0x0;
  }
  lVar2 = pass1_1000_0ed4(param_5,param_6,0x0,param_3,0x0,param_1,param_2);
  return lVar2;
}



fn pass1_1000_16ee(param_1: u16,param_2: u16,param_3: u16)
{
  if ((param_2 | param_1) != 0x0) {
    call_fn_ptr_1000_0dc6(param_1,param_2,param_3);
  }
  return;
}


fn  pass1_1000_17e8(uchar *param_1,uchar *param_2) -> *mut u8

{
  let puVar1: *mut u8
  
  puVar1 = PTR_LOOP_1050_5f34;
  PTR_LOOP_1050_5f34 = param_1;
  PTR_LOOP_1050_5f36 = param_2;
  return puVar1;
}



fn  pass1_1000_180c(param_1: u16,param_2: u16,param_3: u16) -> u16

{
  let puVar1: *mut u8
  let lVar2: i32;
  
  if ((PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) == 0x0) {
    puVar1 = mem_op_1000_160a(param_2,param_3);
    if ((param_2 | puVar1) == 0x0) {
      return 0x0;
    }
  }
  lVar2 = mem_op_1000_0a48(0x0,param_1,0x0,CONCAT22(PTR_LOOP_1050_5f2e,PTR_LOOP_1050_5f2c)
                           ,param_3);
  return lVar2;
}



fn pass1_1000_183c(param_1: u16,param_2: u16,param_3: u16) -> u16

{
  let puVar1: *mut u8;
  let lVar2: i32;
  
  puVar1 = 0x0;
  if ((param_2 * param_1 >> 0x10) != 0x0) {
    return 0x0;
  }
  if (((PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) == 0x0) &&
     (PTR_LOOP_1050_5f2c = mem_op_1000_160a(0x0,param_3), PTR_LOOP_1050_5f2e = puVar1,
     (puVar1 | PTR_LOOP_1050_5f2c) == 0x0)) {
    return 0x0;
  }
  lVar2 = mem_op_1000_0a48(0x1,(param_2 * param_1),0x0,
                           CONCAT22(PTR_LOOP_1050_5f2e,PTR_LOOP_1050_5f2c),param_3);
  return lVar2;
}



fn pass1_1000_188e(param_1: *mut u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
               param_6: u16) -> u16
{
  let uVar1: u16;
  let lVar2: i32;
  
  if ((param_2 | param_1) == 0x0) {
    uVar1 = pass1_1000_180c(param_3,param_4,param_5);
    return uVar1;
  }
  if (param_3 == 0x0) {
    pass1_1000_18d2(param_1,param_2,param_5);
    return 0x0;
  }
  lVar2 = pass1_1000_0ed4(param_5,param_6,0x0,param_3,0x0,param_1,param_2);
  return lVar2;
}

fn  pass1_1000_18d2(param_1: u16,param_2: u16,param_3: u16)

{
  if ((param_2 | param_1) != 0x0) {
    call_fn_ptr_1000_0dc6(param_1,param_2,param_3);
  }
  return;
}


fn pass1_1000_1a54(param_1: u16,param_2: i16,param_3: u16,param_4: u16) -> u16

{
  let uVar1: u16;
  let uVar2: u16;
  
  if ((param_2 + 0x14) != -0x4153) {
    pass1_1000_1e61(param_4,0xa,0x0,0x0);
    return 0x0;
  }
  uVar1 = pass1_1000_1ab0(param_1);
  if (uVar1 < (param_2 + 0x18) + 0x14U) {
    uVar2 = 0x0;
  }
  else {
    uVar2 = (param_2 + 0x1a);
    (param_2 + 0x1a) = uVar1;
    (param_2 + 0x1c) = uVar1 >> 0x2;
  }
  return uVar2;
}



fn  pass1_1000_1ab0(param_1: u16) -> u16

{
  let uVar1: u16;
  let uVar2: u16;
  
  if (param_1 == 0x2000) {
    return 0x2000;
  }
  if (param_1 < 0xfff0) {
    if (param_1 < 0x1001) {
      return 0x1000;
    }
    uVar1 = 0x2000;
    if (param_1 < 0x2001) {
      do {
        uVar2 = uVar1;
        uVar1 = uVar2 >> 0x1;
      } while (param_1 <= uVar1);
      return uVar2 & 0xfffe;
    }
    while (uVar1 *= 0x2, uVar1 != 0x0) {
      if (param_1 <= uVar1) {
        return (uVar1 + 0x10 & -(uVar1 < 0xfff0)) - 0x10;
      }
    }
  }
  return 0xfff0;
}



fn  pass1_1000_1afe(param_1: u16,param_2: u32,param_3: u16, unaff_CS: u16) -> bool

{
  let uVar1: u16;

  if (param_1 == 0x0) {
    uVar1 = 0x0;
  }
  else {
    uVar1 = param_1 + 0x1 & 0xfffe;
  }
  if ((param_2 + 0x14) == -0x4153) {
    if ((uVar1 < param_1) || ((param_2 + 0x1a) - 0x14U < uVar1)) {
      pass1_1000_1e61(unaff_CS,0x3,param_2,param_3);
    }
    else {
      if ((param_2 + 0x2) == 0x0) {
        (param_2 + 0x18) = uVar1;
        return 0x1;
      }
    }
    return 0x0;
  }
  pass1_1000_1e61(unaff_CS,0xa,0x0,0x0);
  return 0x0;
}




// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1000_1e61(param_1: u16,param_2: u16,param_3: u16,param_4: u16) -> u16

{
  let iVar1: i16;
  let BVar2: bool;
  let UVar3: u16;
  let UStack64: u16;
  let UStack62: u16;
  let UStack60: u16;
  code *pcStack6;
  let puStack4: *mut u8;
  let uVar3: u16;
  
  uVar3 = &USHORT_1050_1050;
  UStack62 = param_3;
  UStack60 = param_4;
  UStack64 = param_2;
  puStack4 = &USHORT_1050_1050;
  if (true) {
    pcStack6 = (code *)&PTR_PTR_1050_5f1a;
    if ((PTR_LOOP_1050_5f1c | PTR_PTR_1050_5f1a) == 0x0) {
      pcStack6 = (code *)0x0;
      puStack4 = 0x0;
    }
    else {
      iVar1 = mem_op_1000_21b6(PTR_PTR_1050_5f1a,PTR_LOOP_1050_5f1c);
      pcStack6 = (code *)PTR_PTR_1050_5f1a;
      puStack4 = PTR_LOOP_1050_5f1c;
      if (iVar1 == 0x0) {
        PTR_PTR_1050_5f1a = &PTR_PTR_1050_1f7e;
        PTR_LOOP_1050_5f1c = &PTR_LOOP_1050_1000;
        pcStack6 = (code *)&PTR_PTR_1050_1f7e;
        puStack4 = &PTR_LOOP_1050_1000;
      }
    }
    if ((puStack4 | pcStack6) != 0x0) {
      BVar2 = msg_box_op_1000_1f24
                        (&PTR_PTR_1050_5f1a,&USHORT_1050_1050,0x0,0x1000);
      if (BVar2 == 0x0) {
        UVar3 = (*pcStack6)(0x1000,&UStack64);
      }
      else {
        puStack4 = 0x0;
        pcStack6 = (code *)0x0;
        UVar3 = 0x0;
      }
      if ((puStack4 | pcStack6) != 0x0) {
        pass1_1000_1f68(uVar3);
      }
      return UVar3;
    }
  }
  return 0x0;
}


fn pass1_1000_1f68()
{
  if ((true) &&
     (PTR_LOOP_1050_5f26 = PTR_LOOP_1050_5f26 + -0x1, PTR_LOOP_1050_5f26 < 0x0)) {
    PTR_LOOP_1050_5f26 = 0x0;
  }
  return;
}



fn pass1_1000_1f7e(param_1: *mut u16,param_2: u16) -> bool

{
  let cVar1: u8;
  let BVar2: bool;
  let uVar3: u16;
  let iVar4: i16;
  char *pcVar5;
  
  uVar3 = *param_1;
  if (false) {
    return 0x0;
  }
  if (uVar3 == 0xf) {
LAB_1000_1fb6:
    iVar4 = 0x1;
  }
  else {
    if (uVar3 < 0x10) {
      cVar1 = uVar3;
      if (cVar1 == '\x02') goto LAB_1000_1fb6;
      if (('\0' < (cVar1 + -0x2)) && ((char)(cVar1 + -0x3) < '\f')) {
        iVar4 = 0x0;
        goto LAB_1000_1fbe;
      }
    }
    iVar4 = 0x0;
    uVar3 = 0x1;
  }
LAB_1000_1fbe:
  pcVar5 = pass1_1000_1fd2(uVar3);
  BVar2 = msg_box_op_1000_214c
                    (0x0,iVar4,pcVar5,(pcVar5 >> 0x10),param_2);
  return BVar2;
}



fn pass1_1000_1fd2(param_1: i16) -> *mut u8

{
  if (param_1 == 0x2) {
    return "Out of memory.  Please free some memory, then choose retry.";
  }
  return CONCAT22(0x1000,param_1 * 0x17 + 0x1c7a);
}



fn pass1_1000_1fea() -> bool

{
  let puVar1: *mut u8;
  let bVar2: bool;
  
  if (((true) &&
      (puVar1 = PTR_LOOP_1050_5f22 + 0x1, bVar2 = PTR_LOOP_1050_5f22 == 0x0,
      PTR_LOOP_1050_5f22 = puVar1, bVar2)) &&
     ((PTR_LOOP_1050_5f20 | PTR_LOOP_1050_5f1e) != 0x0)) {
    PTR_LOOP_1050_5f22 = &PTR_LOOP_1050_0002;
  }
  if (true) {
    return 0x1;
  }
  return 0x0;
}



fn pass1_1000_201c(param_1: i16,param_2: i16,param_3: u16)
{
  let uVar1: u16;
  let uVar2: u32;
  let uVar3: u16;
  let Bvar4: bool;
  let iVar5: i16;
  let uVar6: u16;
  
  if (param_1 == 0x0) {
    (param_2 + 0x6) = 0x0;
    (param_2 + 0x4) = 0x0;
  }
  uVar3 = (param_2 + 0x6) | (param_2 + 0x4);
  while (uVar3 != 0x0) {
    BVar4 = pass1_1000_206c((param_2 + 0x4),(param_2 + 0x6));
    if (BVar4 == 0x0) {
      uVar2 = (param_2 + 0x4);
      uVar6 = (uVar2 >> 0x10);
      iVar5 = uVar2;
      uVar1 = (iVar5 + 0x2c);
      (param_2 + 0x4) = (iVar5 + 0x2a);
      (param_2 + 0x6) = uVar1;
    }
    else {
      mem_op_1000_1b9a(0x1,(param_2 + 0x4),(param_2 + 0x6),
                       param_3);
    }
    uVar3 = (param_2 + 0x6) | (param_2 + 0x4);
  }
  return;
}



fn pass1_1000_206c(param_1: u16,param_2: u16) -> bool

{
  let uVar1: u16;
  
  uVar1 = pass1_1000_21d2(0x2,0x42,param_1,param_2,0x1);
  if ((uVar1 != 0x0) && ((param_1 + 0x14) == -0x4153)) {
    return 0x1;
  }
  return 0x0;
}



fn pass1_1000_20a2(param_1: u16,param_2: u16)
{
  let iVar1: i16;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uStack8: u16;
  let uStack4: u16;
  
  iVar1 = (param_1 + 0x2e);
  uVar2 = (param_1 + 0x30);
  uStack8 = 0x0;
  uVar3 = (iVar1 + 0x4);
  uStack4 = (iVar1 + 0x6);
  uVar7 = 0x0;
  if ((uStack4 | uVar3) != 0x0) {
    while ((uVar6 = uVar3, uVar4 = uStack4, uVar6 != param_1 || (uStack4 != param_2))) {
      uVar3 = (uVar6 + 0x2a);
      uStack4 = (uVar6 + 0x2c);
      uVar7 = uVar6;
      uStack8 = uVar4;
      if ((uStack4 | uVar3) == 0x0) {
        return;
      }
    }
    if ((uStack8 | uVar7) != 0x0) {
      uVar2 = (uVar6 + 0x2c);
      (uVar7 + 0x2a) = (uVar6 + 0x2a);
      (uVar7 + 0x2c) = uVar2;
      return;
    }
    uVar5 = (uVar6 + 0x2c);
    (iVar1 + 0x4) = (uVar6 + 0x2a);
    (iVar1 + 0x6) = uVar5;
  }
  return;
}


fn pass1_1000_21d2(param_1: u8,param_2: i32,param_3: u16,param_4: u16,u8 param_5) -> u16

{
  undefined3 uVar1;
  let BVar2: bool;
  
  if (true) {
    BVar2 = mem_op_1000_1dfa(0x0,param_1,param_3,param_4);
    if (BVar2 != 0x0) {
      return 0x0;
    }
    if ((param_1 & 0x4) == 0x0) {
      uVar1 = SegmentLimit(param_4);
      if (!(bool)((byte)((uint3)uVar1 >> 0x10) & 0x1)) {
        return 0x0;
      }
      if (param_2 == 0x0) {
        return 0x1;
      }
      if (CARRY4(param_3,param_2 - 0x1U)) {
        return 0x0;
      }
      if (param_3 + (param_2 - 0x1U) <= uVar1) {
        return 0x1;
      }
      return 0x0;
    }
  }
  BVar2 = pass1_1000_22c0(param_3,param_4,param_2,param_2._2_2_,_param_1);
  if (BVar2 == 0x0) {
    return 0x0;
  }
  return 0x1;
}



fn pass1_1000_2242(param_1: u16,param_2: u16,param_3: u16,param_4: i16,param_5: u16,
                     param_6: *mut u8) -> u32

{
  let uVar1: u16;
  let uVar2: u16;
  let bVar3: bool;
  
  uVar1 = param_2 | param_1;
  while( true ) {
    if (uVar1 == 0x0) {
      return 0x0;
    }
    uVar1 = param_1;
    if (param_2 != 0x0) {
      uVar1 = 0xffff;
    }
    if (CARRY2(param_3,uVar1) != false) {
      uVar1 = -param_3;
    }
    bVar3 = param_1 < uVar1;
    param_1 -= uVar1;
    param_2 -= bVar3;
    uVar2 = (*(code *)param_6)(uVar1,param_5,param_3,param_4);
    if (uVar2 != 0x0) break;
    bVar3 = CARRY2(param_3,uVar1);
    param_3 += uVar1;
    param_4 += bVar3 * 0x100;
    uVar1 = param_2 | param_1;
  }
  return CONCAT22(param_2 + CARRY2(uVar2,param_1),uVar2 + param_1);
}



fn pass1_1000_22c0(param_1: u16,param_2: u16,param_3: u16,param_4: u16,
                      param_5: u16 -> bool

{
  let uVar1: u32;
  
  uVar1 = pass1_1000_2242(param_3,param_4,param_1,param_2,param_5,0x1dfa);
  if (uVar1 == 0x0) {
    return 0x1;
  }
  return 0x0;
}


pub fn pass1_1000_24db(param_1: i16,param_2: u16)
{
  code *pcVar1;
  let iVar2: i16;
  let uVar3: u16;
  let cVar4: u8;
  let uVar5: u16;
  
  iVar2 = param_2 + 0x1;
  uVar5 = SUB42(&USHORT_1050_1050,0x0);
  PTR_LOOP_1050_5fc9._0_1_ = 0x0;
  uVar3 = 0x1;
  if (false) {
    fn_ptr_op_1000_2594(0x68b6,0x68b6);
    fn_ptr_op_1000_2594(&PTR_LOOP_1050_6210,0x620c);
    ret_op_1000_55ac(param_1,uVar3,uVar5,iVar2);
  }
  cVar4 = (uVar3 >> 0x8);
  fn_ptr_op_1000_2594(&PTR_LOOP_1050_6210,&PTR_LOOP_1050_6210);
  fn_ptr_op_1000_2594(&PTR_LOOP_1050_6210,&PTR_LOOP_1050_6210);
  dos3_op_1000_256b();
  if (cVar4 == '\0') {
    if (true) {
      pcVar1 = (code *)swi(0x21);
      (*pcVar1)();
    }
    else {
      DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
    }
  }
  return;
}


pub fn pass1_1000_25a8(param_1: u16,param_2: u16)
{
  pass1_1000_2913(0xfc,param_1,param_2);
  pass1_1000_2913(0xff,param_1,param_2);
  return;
}




i16 * pass1_1000_25d2(param_1: i16,param_2: i16,param_3: u16,param_4: u16,param_5: u16
                     ,param_6: *mut u8)

{
  let puVar1: *mut u16;
  let piVar2: *mut i16;
  char *pcVar3;
  let puVar4: *mut u8;
  let uVar5: u16;
  let piVar6: *mut i16;
  LPCSTR str;
  let piVar7: *mut i16;
  char *pcVar8;
  let iVar9: i16;
  
  puVar4 = (param_2 + 0x1U & 0xfffe);
  if ((puVar4 < &param_1) &&
     (uVar5 = -(puVar4 + -&param_1), puVar1 = &PTR_LOOP_1050_000a,
     *puVar1 < uVar5 || *puVar1 == uVar5)) {
    puVar1 = &PTR_LOOP_1050_000c;
    if (uVar5 <= *puVar1 && *puVar1 != uVar5) {
      &PTR_LOOP_1050_000c = uVar5;
    }
                    // WARNING: Could not recover jumptable at 0x100025f0. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    piVar6 = (i16 *)(*(code *)param_6)();
    return piVar6;
  }
  iVar9 = 0x0;
  pass1_1000_25a8(param_3,param_4);
  pass1_1000_2913(iVar9,param_3,param_4);
  str = poss_str_op_1000_28dc(0x0);
  if (str != 0x0) {
    iVar9 = 0x9;
    if (*str == 'M') {
      iVar9 = 0xf;
    }
    str = str + iVar9;
    iVar9 = 0x22;
    pcVar8 = str;
    do {
      if (iVar9 == 0x0) break;
      iVar9 += -0x1;
      pcVar3 = pcVar8;
      pcVar8 = pcVar8 + 0x1;
    } while (*pcVar3 != '\r');
    pcVar8[-0x1] = '\0';
  }
  FatalAppExit16(param_4,str);
  FatalExit();
  piVar6 = (i16 *)&PTR_LOOP_1050_63fe;
  do {
    piVar2 = piVar6;
    piVar6 = piVar6 + 0x1;
    iVar9 = *piVar2;
    piVar7 = piVar6;
    if ((iVar9 == param_1) || (piVar7 = (i16 *)(iVar9 + 0x1), piVar7 == (i16 *)0x0)) {
      return piVar7;
    }
    iVar9 = -0x1;
    do {
      if (iVar9 == 0x0) break;
      iVar9 += -0x1;
      piVar2 = piVar6;
      piVar6 = (i16 *)(piVar6 + 0x1);
    } while (*piVar2 != '\0');
  } while( true );
}



// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING (jumptable): Heritage AFTER dead removal. Example location: r0x10505fc2 :
// 0x1000270c
// WARNING: Unable to track spacebase fully for stack
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: ram

pub fn pass1_1000_262c(
    param_1: *mut u8,
    param_2: *mut u8,
    param_3: &str,
    param_4: HINSTANCE16
)

{
  char *pcVar1;
  let cVar2: u8;
  let uVar3: u16;
  let puVar4: *mut u8;
  let IVar5: i16;
  u16_t uVar6;
  let uVar7: u16;
  let uVar8: u16;
  let in_DX: *mut u8;
  let iVar9: i16;
  char **ppcVar10;
  char *pcVar11;
  char *pcVar12;
  char *pcVar13;
  let unaff_ES: u16;
  let uVar14: u16;
  let puStack4: *mut u8;
  CHAR *pCStack2;
  
  PTR_LOOP_1050_5fd2 = param_1;
  PTR_LOOP_1050_5fd4 = param_2;
  param_2 = 0x263d;
  param_1 = pass1_1000_2950(0x8,in_DX,unaff_ES,param_4);
  pCStack2 = PTR_LOOP_1050_5f4c;
  puStack4 = in_DX;
  PTR_LOOP_1050_5fc2 = param_1;
  PTR_LOOP_1050_5fc4 = in_DX;
  IVar5 = GetModuleFileName16(param_4,(LPSTR)(s_You_may_not_run_a_turn__The_game_1050_00df
                                             + 0x25),param_1);
  puStack4[IVar5] = 0x0;
  iVar9 = 0x1;
  PTR_LOOP_1050_5fb8 = (&PTR_LOOP_1050_0000 + 0x1);
  pcVar11 = (s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);
LAB_1000_266c:
  do {
    do {
      pcVar1 = pcVar11;
      pcVar11 = pcVar11 + 0x1;
      cVar2 = *pcVar1;
    } while (cVar2 == ' ');
  } while (cVar2 == '\t');
  if ((cVar2 != '\r') && (cVar2 != '\0')) {
    PTR_LOOP_1050_5fb8 = PTR_LOOP_1050_5fb8 + 0x1;
    do {
      pcVar11 = pcVar11 + -0x1;
LAB_1000_267f:
      pcVar1 = pcVar11;
      pcVar11 = pcVar11 + 0x1;
      cVar2 = *pcVar1;
      if ((cVar2 == ' ') || (cVar2 == '\t')) goto LAB_1000_266c;
      if ((cVar2 == '\r') || (cVar2 == '\0')) break;
      if (cVar2 == '\"') {
LAB_1000_26b8:
        do {
          while( true ) {
            while( true ) {
              pcVar1 = pcVar11;
              pcVar11 = pcVar11 + 0x1;
              cVar2 = *pcVar1;
              if ((cVar2 == '\r') || (cVar2 == '\0')) goto LAB_1000_26e8;
              if (cVar2 == '\"') goto LAB_1000_267f;
              if (cVar2 == '\\') break;
              iVar9 += 0x1;
            }
            uVar7 = 0x0;
            do {
              pcVar13 = pcVar11;
              uVar7 += 0x1;
              pcVar11 = pcVar13 + 0x1;
              cVar2 = *pcVar13;
            } while (cVar2 == '\\');
            if (cVar2 == '\"') break;
            iVar9 += uVar7;
            pcVar11 = pcVar13;
          }
          iVar9 = iVar9 + (uVar7 >> 0x1) + ((uVar7 & 0x1) != 0x0);
        } while ((uVar7 & 0x1) != 0x0);
        goto LAB_1000_267f;
      }
      if (cVar2 != '\\') {
        iVar9 += 0x1;
        goto LAB_1000_267f;
      }
      uVar7 = 0x0;
      do {
        uVar7 += 0x1;
        pcVar1 = pcVar11;
        pcVar11 = pcVar11 + 0x1;
        cVar2 = *pcVar1;
      } while (cVar2 == '\\');
      if (cVar2 == '\"') {
        iVar9 = iVar9 + (uVar7 >> 0x1) + ((uVar7 & 0x1) != 0x0);
        if ((uVar7 & 0x1) == 0x0) goto LAB_1000_26b8;
        goto LAB_1000_267f;
      }
      iVar9 += uVar7;
    } while( true );
  }
LAB_1000_26e8:
  pCStack2 = &USHORT_1050_1050;
  iVar9 = -((PTR_LOOP_1050_5fb8 +
                  (PTR_LOOP_1050_5fb8 + 0x1) * 0x4 + iVar9 + 0x1) & 0xfffe);
  PTR_LOOP_1050_5fba = (&param_1 + iVar9);
  pcVar13 = (&param_1 + (PTR_LOOP_1050_5fb8 + 0x1) * 0x4 + iVar9);
  PTR_LOOP_1050_5fbc = param_3;
  *(CHAR **)(&pCStack2 + iVar9) = param_3;
  puVar4 = PTR_LOOP_1050_5fc4;
  uVar14 = *(CHAR **)(&pCStack2 + iVar9);
  *(u8 **)(&param_1 + iVar9) = PTR_LOOP_1050_5fc2;
  *(u8 **)(&param_2 + iVar9) = puVar4;
  ppcVar10 = (char **)(&stack0x0004 + iVar9);
  *(CHAR **)(&pCStack2 + iVar9) = (&param_1 + iVar9);
  *(u8 **)(&puStack4 + iVar9) = s_tile2_bmp_1050_1538;
  (&stack0xfffa + iVar9) = 0x271f;
  uVar6 = pass1_1000_29dc(param_3);
  uVar3 = &PTR_LOOP_1050_5f7e;
  pcVar11 = (s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);
LAB_1000_272e:
  do {
    do {
      pcVar1 = pcVar11;
      pcVar11 = pcVar11 + 0x1;
      cVar2 = *pcVar1;
    } while (cVar2 == ' ');
  } while (cVar2 == '\t');
  if ((cVar2 == '\r') || (cVar2 == '\0')) {
LAB_1000_27c1:
    *(CHAR **)(&pCStack2 + iVar9) = s_tile2_bmp_1050_1538;
    *(u8 **)(&puStack4 + iVar9) = 0x27c5;
    uVar6 = pass1_1000_29dc(param_3);
    *ppcVar10 = 0x0;
    ppcVar10[0x1] = 0x0;
                    // WARNING: Could not recover jumptable at 0x100027d2. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    (*(code *)&PTR_LOOP_1050_5fd2)();
    _PTR_LOOP_1050_5fc2 = CONCAT22(PTR_LOOP_1050_5fc4,PTR_LOOP_1050_5fc2);
    return;
  }
  *ppcVar10 = pcVar13;
  ppcVar10[0x1] = param_3;
  ppcVar10 = ppcVar10 + 0x2;
  do {
    pcVar11 = pcVar11 + -0x1;
LAB_1000_274f:
    pcVar1 = pcVar11;
    pcVar11 = pcVar11 + 0x1;
    cVar2 = *pcVar1;
    if ((cVar2 == ' ') || (cVar2 == '\t')) {
      pcVar1 = pcVar13;
      pcVar13 = pcVar13 + 0x1;
      *pcVar1 = '\0';
      goto LAB_1000_272e;
    }
    if ((cVar2 == '\r') || (cVar2 == '\0')) {
LAB_1000_27be:
      *pcVar13 = '\0';
      goto LAB_1000_27c1;
    }
    pcVar12 = pcVar11;
    if (cVar2 == '\"') {
LAB_1000_278b:
      while( true ) {
        pcVar11 = pcVar12 + 0x1;
        cVar2 = *pcVar12;
        if ((cVar2 == '\r') || (cVar2 == '\0')) goto LAB_1000_27be;
        if (cVar2 == '\"') break;
        if (cVar2 == '\\') {
          uVar7 = 0x0;
          do {
            pcVar12 = pcVar11;
            uVar7 += 0x1;
            pcVar11 = pcVar12 + 0x1;
            cVar2 = *pcVar12;
          } while (cVar2 == '\\');
          if (cVar2 == '\"') {
            for (uVar8 = uVar7 >> 0x1; uVar8 != 0x0; uVar8 -= 0x1) {
              pcVar1 = pcVar13;
              pcVar13 = pcVar13 + 0x1;
              *pcVar1 = '\\';
            }
            if ((uVar7 & 0x1) == 0x0) break;
            pcVar1 = pcVar13;
            pcVar13 = pcVar13 + 0x1;
            *pcVar1 = '\"';
            pcVar12 = pcVar11;
          }
          else {
            for (; uVar7 != 0x0; uVar7 -= 0x1) {
              pcVar1 = pcVar13;
              pcVar13 = pcVar13 + 0x1;
              *pcVar1 = '\\';
            }
          }
        }
        else {
          pcVar1 = pcVar13;
          pcVar13 = pcVar13 + 0x1;
          *pcVar1 = cVar2;
          pcVar12 = pcVar11;
        }
      }
      goto LAB_1000_274f;
    }
    if (cVar2 != '\\') {
      pcVar1 = pcVar13;
      pcVar13 = pcVar13 + 0x1;
      *pcVar1 = cVar2;
      goto LAB_1000_274f;
    }
    uVar7 = 0x0;
    do {
      uVar7 += 0x1;
      pcVar1 = pcVar11;
      pcVar11 = pcVar11 + 0x1;
      cVar2 = *pcVar1;
    } while (cVar2 == '\\');
    if (cVar2 == '\"') {
      for (uVar8 = uVar7 >> 0x1; uVar8 != 0x0; uVar8 -= 0x1) {
        pcVar1 = pcVar13;
        pcVar13 = pcVar13 + 0x1;
        *pcVar1 = '\\';
      }
      pcVar12 = pcVar11;
      if ((uVar7 & 0x1) == 0x0) goto LAB_1000_278b;
      pcVar1 = pcVar13;
      pcVar13 = pcVar13 + 0x1;
      *pcVar1 = '\"';
      goto LAB_1000_274f;
    }
    for (; uVar7 != 0x0; uVar7 -= 0x1) {
      pcVar1 = pcVar13;
      pcVar13 = pcVar13 + 0x1;
      *pcVar1 = '\\';
    }
  } while( true );
}



pub fn pass1_1000_27d6(param_1: *mut u16)
{
  let piVar1: *mut i16;
  char *pcVar2;
  let puVar3: *mut u16;
  let piVar4: *mut i16;
  let cVar5: u8;
  SEGPTR SVar6;
  let puVar7: *mut u16;
  u16_t **ppuVar8;
  let iVar9: i16;
  let uVar10: u16;
  let puVar11: *mut u16;
  let iVar12: i16;
  let piVar13: *mut i16;
  let piVar14: *mut i16;
  char *pcVar15;
  let piVar16: *mut i16;
  let bVar17: bool;
  let puVar18: *mut u16;
  
  SVar6 = GetDOSEnvironment16();
  if (SVar6 != 0x0) {
    param_1 = (u16_t *)0x0;
  }
  iVar12 = 0x0;
  pcVar15 = 0x0;
  iVar9 = -0x1;
  if (param_1 != (u16_t *)0x0) {
    cVar5 = *0x0;
    while (cVar5 != '\0') {
      do {
        if (iVar9 == 0x0) break;
        iVar9 += -0x1;
        pcVar2 = pcVar15;
        pcVar15 = pcVar15 + 0x1;
      } while (*pcVar2 != '\0');
      iVar12 += 0x1;
      pcVar2 = pcVar15;
      pcVar15 = pcVar15 + 0x1;
      cVar5 = *pcVar2;
    }
  }
  uVar10 = 0x9;
  puVar11 = param_1;
  puVar7 = pass1_1000_2950(0x9,param_1,param_1,s_tile2_bmp_1050_1538);
  puVar18 = puVar11;
  ppuVar8 = (fn pass1_1000_2950(uVar10,puVar11,param_1,s_tile2_bmp_1050_1538) -> u16;
  piVar13 = (i16 *)0x0;
  PTR_LOOP_1050_5fbe = ppuVar8;
  PTR_LOOP_1050_5fc0 = puVar11;
  do {
    if (iVar12 == 0x0) {
      *ppuVar8 = (u16_t *)0x0;
      ppuVar8[0x1] = (u16_t *)0x0;
      return;
    }
    bVar17 = *piVar13 == s__C_FILE_INFO__1050_5f5c._0_2_;
    if (bVar17) {
      piVar16 = (i16 *)s__C_FILE_INFO__1050_5f5c;
      iVar9 = 0x6;
      piVar14 = piVar13;
      do {
        if (iVar9 == 0x0) break;
        iVar9 += -0x1;
        piVar4 = piVar16;
        piVar16 = piVar16 + 0x1;
        piVar1 = piVar14;
        piVar14 = piVar14 + 0x1;
        bVar17 = *piVar1 == *piVar4;
      } while (bVar17);
      if (!bVar17) goto LAB_1000_2867;
    }
    else {
LAB_1000_2867:
      *ppuVar8 = puVar7;
      ppuVar8[0x1] = puVar18;
      ppuVar8 = ppuVar8 + 0x2;
    }
    do {
      piVar1 = piVar13;
      piVar13 = (i16 *)(piVar13 + 0x1);
      cVar5 = *piVar1;
      puVar3 = puVar7;
      puVar7 = (u16_t *)(puVar7 + 0x1);
      *puVar3 = cVar5;
    } while (cVar5 != '\0');
    iVar12 += -0x1;
  } while( true );
}



pub fn pass1_1000_2913(param_1: i16,param_2: u16,param_3: u16)
{
  char *pcVar1;
  char *pcVar2;
  let iVar3: i16;
  
  if (PTR_LOOP_1050_61ec != 0x0) {
    pcVar2 = poss_str_op_1000_28dc(param_1);
    if (pcVar2 != 0x0) {
      iVar3 = -0x1;
      do {
        if (iVar3 == 0x0) break;
        iVar3 += -0x1;
        pcVar1 = pcVar2;
        pcVar2 = pcVar2 + 0x1;
      } while (*pcVar1 != '\0');
      pass1_1000_55b1(0x2944,param_2,param_3);
    }
  }
  return;
}



fn pass1_1000_2950(param_1: i16,param_2: u16,param_3: u16,param_4: u16) -> *mut u16

{
  let puVar1: *mut u16;
  u16_t uVar2;
  char *pcVar3;
  let puVar4: *mut u8;
  LPCSTR str;
  let iVar5: i16;
  let puVar6: *mut u16;
  let in_AX: u16;
  let puVar7: *mut u16;
  char *pcVar8;
  u16_t uVar9;
  
  puVar4 = PTR_LOOP_1050_6066;
  PTR_LOOP_1050_6066 = &PTR_LOOP_1050_1000;
  uVar9 = param_3;
  puVar7 = (fn mem_1000_167a(in_AX,param_4,param_2) -> u16;
  PTR_LOOP_1050_6066 = puVar4;
  if ((param_2 | puVar7) != 0x0) {
    return puVar7;
  }
  iVar5 = param_1;
  pass1_1000_25a8(param_3,param_4);
  pass1_1000_2913(param_1,param_3,param_4);
  str = poss_str_op_1000_28dc(iVar5);
  if (str != 0x0) {
    iVar5 = 0x9;
    if (*str == 'M') {
      iVar5 = 0xf;
    }
    str = str + iVar5;
    iVar5 = 0x22;
    pcVar8 = str;
    do {
      if (iVar5 == 0x0) break;
      iVar5 += -0x1;
      pcVar3 = pcVar8;
      pcVar8 = pcVar8 + 0x1;
    } while (*pcVar3 != '\r');
    pcVar8[-0x1] = '\0';
  }
  FatalAppExit16(param_4,str);
  FatalExit();
  puVar7 = (u16_t *)&PTR_LOOP_1050_63fe;
  do {
    puVar1 = puVar7;
    puVar7 = puVar7 + 0x1;
    uVar2 = *puVar1;
    puVar6 = puVar7;
    if ((uVar2 == uVar9) ||
       (puVar6 = (u16_t *)(uVar2 + 0x1), puVar6 == (u16_t *)0x0)) {
      return puVar6;
    }
    iVar5 = -0x1;
    do {
      if (iVar5 == 0x0) break;
      iVar5 += -0x1;
      puVar1 = puVar7;
      puVar7 = (u16_t *)(puVar7 + 0x1);
    } while (*puVar1 != '\0');
  } while( true );
}



fn pass1_1000_29af(param_1: u16)
{
  pass1_1000_29b5(param_1 & 0xff);
  return;
}



fn pass1_1000_29b5(param_1: u16)
{
  let cVar1: u8;
  
  PTR_LOOP_1050_5f88._0_1_ = (byte)param_1;
  cVar1 = (param_1 >> 0x8);
  if (cVar1 != '\0') goto LAB_1000_29d2;
  if ((byte)PTR_LOOP_1050_5f88 < 0x22) {
    if ((byte)PTR_LOOP_1050_5f88 < 0x20) {
      if (0x13 < (byte)PTR_LOOP_1050_5f88) goto LAB_1000_29cc;
    }
    else {
      param_1 = 0x5;
    }
  }
  else {
LAB_1000_29cc:
    param_1 = 0x13;
  }
  cVar1 = *((param_1 & 0xff) + 0x5fd6);
LAB_1000_29d2:
  PTR_LOOP_1050_5f78 = cVar1;
  return;
}



fn pass1_1000_29dc(param_1: u16) -> u16

{
  if (___EXPORTEDSTUB != (code)0xb8) {
    return param_1;
  }
  return uRam100029ed;
}



fn pass1_1000_2a00(param_1: *mut u16,param_2: i16,param_3: u16,param_4: u16,param_5: u16,
               param_6: u8) -> u16

{
  let bVar1: bool;
  let piVar2: *mut i16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let puStack20: *mut u8;
  let local_10: u8;
  let uStack15: u8;
  let local_e: [u8;8];
  let uStack6: u16;
  let local_4: u16;
  let iStack2: i16;
  
  iStack2 = param_2 + 0x1;
  local_4 = SUB42(&USHORT_1050_1050,0x0);
  uVar5 = 0xffff;
  if ((*(byte *)(param_1 + 0x5) & 0x40) != 0x0) {
    *(param_1 + 0x5) = 0x0;
    return 0xffff;
  }
  if ((*(byte *)(param_1 + 0x5) & 0x83) == 0x0) goto LAB_1000_2af2;
  uVar5 = pass1_1000_2fa4((i16 *)param_1,param_3,param_4,param_5,param_6);
  uStack6 = param_1[0x7a];
  pass1_1000_2cb0(param_1,param_4);
  if (DAT_1050_5f8a < *(byte *)(param_1 + 0xb)) {
    piVar2 = pass1_1000_55b1(0x2a63,param_3,param_4);
    if (piVar2 < 0x0) goto LAB_1000_2a6a;
LAB_1000_2a82:
    bVar1 = false;
  }
  else {
    iVar3 = dos3_call_op_1000_35fe(*(byte *)(param_1 + 0xb),&iStack2);
    if (-0x1 < iVar3) goto LAB_1000_2a82;
LAB_1000_2a6a:
    bVar1 = true;
  }
  if (!bVar1) {
    if (uStack6 == 0x0) goto LAB_1000_2af2;
    unk_str_op_1000_3d3e(CONCAT22(param_5,&local_10),0x10505fea);
    puStack20 = local_e;
    if (local_10 == '\\') {
      puStack20 = &uStack15;
    }
    else {
      pass1_1000_3cea(CONCAT22(param_5,&local_10),0x10505fec);
    }
    pass1_1000_3e82(uStack6,CONCAT22(param_5,puStack20),0xa);
    uVar4 = dos3_call_1000_514e(&iStack2);
    if (uVar4 == 0x0) goto LAB_1000_2af2;
  }
  uVar5 = 0xffff;
LAB_1000_2af2:
  *(param_1 + 0x5) = 0x0;
  return uVar5;
}



fn pass1_1000_2b02(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u8,
               param_6: u16,param_7: i16) -> *mut u16

{
  let puVar1: *mut u16;
  let iStack2: i16;
  
  iStack2 = param_7 + 0x1;
  puVar1 = pass1_1000_35aa();
  if ((param_6 | puVar1) == 0x0) {
    puVar1 = 0x0;
  }
  else {
    puVar1 = pass1_1000_2d34(param_1,param_2,(byte *)CONCAT22(param_4,param_3),param_5,
                             puVar1,&iStack2);
  }
  return puVar1;
}



fn pass1_1000_2b3c(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16
               ,param_6: i16)

{
  let iStack2: i16;
  
  iStack2 = param_6 + 0x1;
  pass1_1000_2b02(param_1,param_2,param_3,param_4,0x0,param_5,&iStack2);
  return;
}



fn pass1_1000_2b5c(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16
               ,param_6: i16,param_7: u16,param_8: u16) -> u16

{
  let uVar1: u16;
  let uVar2: u16;
  let in_AF: u8;
  let iStack2: i16;
  
  iStack2 = param_6 + 0x1;
  uVar1 = pass1_1000_2e74((u16 *)param_1,param_7);
  uVar2 = sys_1000_30b4(param_1,&USHORT_1050_1050,
                        (byte *)CONCAT22(param_4,param_3),&iStack2,param_1,param_5,
                        param_7,param_8);
  pass1_1000_2f00(uVar1,(i16 *)param_1,param_5,param_7,param_8,in_AF);
  return uVar2;
}



fn pass1_1000_2ba0(param_1: u16,param_2: u16,param_3: u16,param_4: u8)
{
  pass1_1000_3024(param_1,param_2,param_3,param_4);
  if ((char)PTR_LOOP_1050_5fc9 != '\0') {
    pass1_1000_3f5c(&stack0xfffe,param_1,param_2,param_3,param_4);
  }
  return;
}



fn pass1_1000_2cb0(param_1: *mut u16,param_2: u16)
{
  let puVar1: *mut u16;
  let bVar2: u8;
  
  bVar2 = *(byte *)(param_1 + 0x5);
  if (((bVar2 & 0x83) != 0x0) && ((bVar2 & 0x8) != 0x0)) {
    pass1_1000_16ee(param_1[0x3],param_1[0x4],param_2);
    puVar1 = param_1 + 0x5;
    *(byte *)puVar1 = *(byte *)puVar1 & 0xf7;
    param_1[0x3] = 0x0;
    param_1[0x4] = 0x0;
    *param_1 = 0x0;
    param_1[0x1] = 0x0;
    param_1[0x2] = 0x0;
  }
  return;
}


fn pass1_1000_2d34(param_1: u16,param_2: u16,param_3: *mut u8,param_4: u8,param_5: *mut u16,
               param_6: i16) -> *mut u16

{
  let bVar1: u8;
  let bVar2: bool;
  let bVar3: bool;
  let uVar4: u16;
  let uStack14: u8;
  let bStack8: u8;
  let uStack6: u8;
  let iStack2: i16;
  
  iStack2 = param_6 + 0x1;
  bStack8 = (byte)PTR_LOOP_1050_6062;
  bVar3 = false;
  bVar1 = *param_3;
  if (bVar1 == 0x77) {
    uVar4 = 0x301;
  }
  else {
    if (0x77 < bVar1) {
      return 0x0;
    }
    if (bVar1 != 0x61) {
      if (bVar1 != 0x72) {
        return 0x0;
      }
      uVar4 = 0x0;
      uStack6 = 0x1;
      goto LAB_1000_2d6c;
    }
    uVar4 = 0x109;
  }
  uStack6 = 0x2;
LAB_1000_2d6c:
  bVar2 = true;
LAB_1000_2d71:
  param_3 = (byte *)(param_3 & 0xffff0000 | (param_3 + 0x1));
  if ((*param_3 == 0x0) || (!bVar2)) {
    uVar4 = mixed_dos3_call_1000_370a(param_1,param_2,uVar4,param_4,0x1a4,&iStack2);
    if (uVar4 < 0x0) {
      return 0x0;
    }
    PTR_LOOP_1050_5fee = PTR_LOOP_1050_5fee + 0x1;
    *(param_5 + 0x5) = uStack6;
    param_5[0x1] = 0x0;
    *param_5 = 0x0;
    param_5[0x4] = 0x0;
    param_5[0x3] = 0x0;
    uStack14 = (u8)uVar4;
    *(param_5 + 0xb) = uStack14;
    *(byte *)(param_5 + 0x78) = bStack8;
    param_5[0x2] = 0x0;
    param_5[0x7a] = 0x0;
    return param_5;
  }
  bVar1 = *param_3;
  if (bVar1 == 0x74) {
    if ((uVar4 & 0xc000) == 0x0) {
      uVar4 |= 0x4000;
      goto LAB_1000_2d71;
    }
  }
  else {
    if (0x74 < bVar1) goto LAB_1000_2da4;
    if (bVar1 == 0x2b) {
      if ((uVar4 & 0x2) != 0x0) goto LAB_1000_2da4;
      uVar4 = uVar4 & 0xfffe | 0x2;
      uStack6 = 0x80;
      goto LAB_1000_2d71;
    }
    if (bVar1 == 0x62) {
      if ((uVar4 & 0xc000) == 0x0) {
        uVar4 |= 0x8000;
        goto LAB_1000_2d71;
      }
    }
    else {
      if (bVar1 != 0x63) {
        if ((bVar1 != 0x6e) || (bVar3)) goto LAB_1000_2da4;
        bVar3 = true;
        bStack8 &= 0xbf;
        goto LAB_1000_2d71;
      }
      if (!bVar3) {
        bVar3 = true;
        bStack8 |= 0x40;
        goto LAB_1000_2d71;
      }
    }
  }
LAB_1000_2da4:
  bVar2 = false;
  goto LAB_1000_2d71;
}



fn pass1_1000_2e74(param_1: *mut u16,param_2: u16) -> u16

{
  let puVar1: *mut u16;
  let uVar2: u16;
  let uVar3: u16;
  let puVar4: *mut u16;
  let puVar5: *mut u16;
  
  if (PTR_LOOP_1050_61ec != 0x0) {
    puVar5 = param_1 + 0x78;
    puVar4 = 0x5ff2;
    if ((param_1 == 0x621c) ||
       (puVar4 = &PTR_LOOP_1050_5ff6, param_1 == 0x6228)) {
      if (((*(byte *)(param_1 + 0x5) & 0xc) == 0x0) && ((*(byte *)puVar5 & 0x1) == 0x0)) {
        uVar2 = *puVar4;
        uVar3 = puVar4[0x1];
        if ((uVar2 | uVar3) == 0x0) {
          uVar2 = mem_1000_167a(0x200,param_2,uVar3);
          if (uVar3 == 0x0) {
            return 0x0;
          }
          *puVar4 = uVar2;
          puVar4[0x1] = uVar3;
        }
        param_1[0x3] = uVar2;
        param_1[0x4] = uVar3;
        *param_1 = uVar2;
        param_1[0x1] = uVar3;
        param_1[0x2] = 0x200;
        param_1[0x79] = 0x200;
        puVar1 = param_1 + 0x5;
        *(byte *)puVar1 = *(byte *)puVar1 | 0x2;
        *(byte *)puVar5 = 0x11;
        return 0x1;
      }
    }
    else {
      if ((byte)DAT_1050_5f8a <= *(byte *)(param_1 + 0xb)) {
        puVar1 = puVar5;
        *(byte *)puVar1 = *(byte *)puVar1 | 0x10;
      }
    }
  }
  return 0x0;
}



fn pass1_1000_2f00(param_1: i16,i16 *param_2,param_3: u16,param_4: u16,param_5: u16,
               param_6: u8)

{
  if (((*(byte *)(param_2 + 0x78) & 0x10) != 0x0) &&
     ((*(byte *)(*(byte *)(param_2 + 0xb) + 0x5f90) & 0x40) != 0x0)) {
    pass1_1000_2fa4(param_2,param_3,param_4,param_5,param_6);
    if (param_1 != 0x0) {
      *(byte *)(param_2 + 0x78) = 0x0;
      param_2[0x79] = 0x0;
      *param_2 = 0x0;
      param_2[0x1] = 0x0;
      param_2[0x3] = 0x0;
      param_2[0x4] = 0x0;
    }
  }
  return;
}



u16 
pass1_1000_2f48(param_1: i32,param_2: i16,param_3: u16,param_4: u16,param_5: u16,
               param_6: u8)

{
  let uVar1: u16;
  let puVar2: *mut u8
  let iStack2: i16;
  
  iStack2 = param_2 + 0x1;
  if (param_1 == 0x0) {
    uVar1 = pass1_1000_3038(0x0,param_3,param_4,param_5,param_6);
  }
  else {
    uVar1 = pass1_1000_2fa4((i16 *)param_1,param_3,param_4,param_5,param_6);
    if (uVar1 == 0x0) {
      if ((*(byte *)((i16 *)param_1 + 0x78) & 0x40) != 0x0) {
        puVar2 = pass1_1000_400a(*(byte *)((i16 *)param_1 + 0xb),
                                 &iStack2);
        uVar1 = -(puVar2 != 0x0);
      }
    }
    else {
      uVar1 = 0xffff;
    }
  }
  return uVar1;
}



fn pass1_1000_2fa4(i16 *param_1,param_2: u16,param_3: u16,param_4: u16,param_5: u8) -> u16

{
  let piVar1: *mut i16;
  let bVar2: u8;
  let iVar3: i16;
  let puVar4: *mut u8
  let puVar5: *mut u8
  let uVar6: u16;
  
  uVar6 = 0x0;
  bVar2 = *(byte *)(param_1 + 0x5);
  if (((bVar2 & 0x3) == 0x2) &&
     (((bVar2 & 0x8) != 0x0 || ((*(byte *)(param_1 + 0x78) & 0x1) != 0x0)))) {
    puVar4 = (*param_1 - param_1[0x3]);
    if (0x0 < puVar4) {
      puVar5 = mixed_dos3_call_1000_39f2
                         ((uchar *)*(byte *)(param_1 + 0xb),
                          CONCAT22(param_1[0x4],param_1[0x3]),puVar4,param_2,
                          param_3,param_4,param_5);
      if (puVar5 == puVar4) {
        if ((*(byte *)(param_1 + 0x5) & 0x80) != 0x0) {
          piVar1 = param_1 + 0x5;
          *(byte *)piVar1 = *(byte *)piVar1 & 0xfd;
        }
      }
      else {
        piVar1 = param_1 + 0x5;
        *(byte *)piVar1 = *(byte *)piVar1 | 0x20;
        uVar6 = 0xffff;
      }
    }
  }
  iVar3 = param_1[0x4];
  *param_1 = param_1[0x3];
  param_1[0x1] = iVar3;
  param_1[0x2] = 0x0;
  return uVar6;
}



fn pass1_1000_3024(param_1: u16,param_2: u16,param_3: u16,param_4: u8)
{
  pass1_1000_3038(0x1,param_1,param_2,param_3,param_4);
  return;
}



i16 pass1_1000_3038(param_1: i16,param_2: u16,param_3: u16,param_4: u16,param_5: u8
                   )

{
  let uVar1: u16;
  let puVar2: *mut u8;
  let iVar3: i16;
  let iStack4: i16;
  
  iVar3 = 0x0;
  iStack4 = 0x0;
  for (puVar2 = &PTR_LOOP_1050_6210; puVar2 <= PTR_LOOP_1050_5ff0;
      puVar2 = puVar2 + 0xc) {
    if ((param_1 == 0x1) && ((puVar2[0xa] & 0x83) != 0x0)) {
      uVar1 = pass1_1000_2f48(CONCAT22(0x1050,puVar2),&stack0xfffe,param_2,param_3,
                              param_4,param_5);
      if (uVar1 != 0xffff) {
        iVar3 += 0x1;
      }
    }
    else {
      if ((param_1 == 0x0) &&
         (((puVar2[0xa] & 0x2) != 0x0 &&
          (uVar1 = pass1_1000_2f48(CONCAT22(0x1050,puVar2),&stack0xfffe,param_2,
                                   param_3,param_4,param_5), uVar1 == 0xffff)))) {
        iStack4 = -0x1;
      }
    }
  }
  if (param_1 == 0x1) {
    iStack4 = iVar3;
  }
  return iStack4;
}



// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING: Unable to track spacebase fully for stack

u16 pass1_1000_30a4(param_1: i16,param_2: u16,param_3: u16,param_4: u16,param_5: i16,
                      param_6: u16,param_7: u16,param_8: u16,param_9: u16,
                      param_10: u8)

{
  let puVar1: *mut u16;
  let cVar2: u8;
  char *pcVar3;
  let bVar4: u8;
  let uVar5: u16;
  let puVar6: *mut u16;
  
  puVar6 = (param_5 + (param_3 + param_6) + param_10);
  puVar1 = puVar6;
  *puVar1 = *puVar1 ^ puVar6;
  puVar1 = (puVar6 + param_3 + 0x31);
  *puVar1 = *puVar1 ^ param_4;
  puVar1 = (puVar6 + -0x3acf);
  *puVar1 = *puVar1 ^ param_3;
  puVar1 = puVar6 + -0x3794;
  *puVar1 = *puVar1 ^ param_2;
  (param_1 + -0x2) = param_4 + 0x1;
  (param_1 + -0x4) = &USHORT_1050_1050;
  (param_1 + -0x6) = param_8;
  (param_1 + -0x8) = 0x30c5;
  exit_1000_25f2((param_1 + -0x8),(param_1 + -0x6),
                 (param_1 + -0x4),0x214,param_7,param_8,param_9);
  *(u16 **)(param_1 + -0x6) = puVar6;
  (param_1 + -0x8) = param_6 ^ puVar6;
  (param_1 + -0xc) = 0x0;
  *(param_1 + -0x9) = 0x0;
  pcVar3 = *(char **)(param_1 + 0x8);
  cVar2 = *pcVar3;
  (param_1 + 0x8) = pcVar3 + 0x1;
  *(param_1 + -0x6) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < (param_1 + -0xc))) {
    if ((byte)(cVar2 - 0x20U) < 0x59) {
      bVar4 = *(byte *)((byte)(cVar2 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = *(byte *)((byte)(bVar4 * '\b' + *(param_1 + -0x9)) + 0x5ffe) >>
            0x4;
    *(byte *)(param_1 + -0x9) = bVar4;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar5 = (**(code **)((char)bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return (param_1 + -0xc);
}




// WARNING (jumptable): Unable to track spacebase fully for stack

fn pass1_1000_3113(param_1: u16,param_2: u16) -> u16

{
  let cVar1: u8;
  char *pcVar2;
  let bVar3: u8;
  let uVar4: u16;
  
  pass1_1000_3552(0x1,param_1,param_2);
  pcVar2 = *(char **)(param_1 + 0xa);
  cVar1 = *pcVar2;
  (param_1 + 0xa) = pcVar2 + 0x1;
  *(param_1 - 0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < (param_1 - 0xa))) {
    if ((byte)(cVar1 - 0x20U) < 0x59) {
      bVar3 = *(byte *)((byte)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = *(byte *)((byte)(bVar3 * '\b' + *(param_1 - 0x7)) + 0x5ffe) >>
            0x4;
    *(byte *)(param_1 - 0x7) = bVar3;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar4 = (**(code **)((char)bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return (param_1 - 0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is
// not used

fn pass1_1000_311e(param_1: i16,param_2: u16) -> u16

{
  let cVar1: u8;
  char *pcVar2;
  let bVar3: u8;
  let uVar4: u16;
  
  (param_1 + -0x12) = 0x0;
  (param_1 + -0xc) = 0x0;
  (param_1 + -0x14) = 0x0;
  (param_1 + -0x6) = 0x20;
  (param_1 + -0xe) = 0xffff;
  pcVar2 = *(char **)(param_1 + 0xa);
  cVar1 = *pcVar2;
  (param_1 + 0xa) = pcVar2 + 0x1;
  *(param_1 + -0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < (param_1 + -0xa))) {
    if ((byte)(cVar1 - 0x20U) < 0x59) {
      bVar3 = *(byte *)((byte)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = *(byte *)((byte)(bVar3 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >>
            0x4;
    *(byte *)(param_1 + -0x7) = bVar3;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar4 = (**(code **)((char)bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return (param_1 + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is
// not used

fn pass1_1000_3134(param_1: i16,param_2: u16) -> u16

{
  byte *pbVar1;
  let cVar2: u8;
  char *pcVar3;
  let bVar4: u8;
  let uVar5: u16;
  
  cVar2 = *(param_1 + -0x4);
  if (cVar2 == '-') {
    pbVar1 = (byte *)(param_1 + -0x6);
    *pbVar1 = *pbVar1 | 0x4;
  }
  else {
    if (cVar2 == '+') {
      pbVar1 = (byte *)(param_1 + -0x6);
      *pbVar1 = *pbVar1 | 0x1;
    }
    else {
      if (cVar2 == ' ') {
        pbVar1 = (byte *)(param_1 + -0x6);
        *pbVar1 = *pbVar1 | 0x2;
      }
      else {
        if (cVar2 == '#') {
          pbVar1 = (byte *)(param_1 + -0x6);
          *pbVar1 = *pbVar1 | 0x80;
        }
        else {
          pbVar1 = (byte *)(param_1 + -0x6);
          *pbVar1 = *pbVar1 | 0x8;
        }
      }
    }
  }
  pcVar3 = *(char **)(param_1 + 0xa);
  cVar2 = *pcVar3;
  (param_1 + 0xa) = pcVar3 + 0x1;
  *(param_1 + -0x4) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < (param_1 + -0xa))) {
    if ((byte)(cVar2 - 0x20U) < 0x59) {
      bVar4 = *(byte *)((byte)(cVar2 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = *(byte *)((byte)(bVar4 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >>
            0x4;
    *(byte *)(param_1 + -0x7) = bVar4;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar5 = (**(code **)((char)bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return (param_1 + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

fn pass1_1000_3168(param_1: i16,param_2: u16) -> u16

{
  byte *pbVar1;
  let cVar2: u8;
  char *pcVar3;
  let bVar4: u8;
  let uVar5: u16;
  
  cVar2 = *(param_1 + -0x4);
  if (cVar2 == '*') {
    uVar5 = pass1_1000_34cf(param_1,param_2);
    if (uVar5 < 0x0) {
      uVar5 = -uVar5;
      pbVar1 = (byte *)(param_1 + -0x6);
      *pbVar1 = *pbVar1 | 0x4;
    }
  }
  else {
    uVar5 = (param_1 + -0xc) * 0xa + (byte)(cVar2 - 0x30);
  }
  (param_1 + -0xc) = uVar5;
  pcVar3 = *(char **)(param_1 + 0xa);
  cVar2 = *pcVar3;
  (param_1 + 0xa) = pcVar3 + 0x1;
  *(param_1 + -0x4) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < (param_1 + -0xa))) {
    if ((byte)(cVar2 - 0x20U) < 0x59) {
      bVar4 = *(byte *)((byte)(cVar2 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = *(byte *)((byte)(bVar4 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >>
            0x4;
    *(byte *)(param_1 + -0x7) = bVar4;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar5 = (**(code **)((char)bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return (param_1 + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is
// not used

fn pass1_1000_3194(param_1: i16,param_2: u16) -> u16

{
  let cVar1: u8;
  char *pcVar2;
  let bVar3: u8;
  let uVar4: u16;
  
  (param_1 + -0xe) = 0x0;
  pcVar2 = *(char **)(param_1 + 0xa);
  cVar1 = *pcVar2;
  (param_1 + 0xa) = pcVar2 + 0x1;
  *(param_1 + -0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < (param_1 + -0xa))) {
    if ((byte)(cVar1 - 0x20U) < 0x59) {
      bVar3 = *(byte *)((byte)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = *(byte *)((byte)(bVar3 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >>
            0x4;
    *(byte *)(param_1 + -0x7) = bVar3;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar4 = (**(code **)((char)bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return (param_1 + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

fn pass1_1000_319c(param_1: i16,param_2: u16) -> u16

{
  let cVar1: u8;
  char *pcVar2;
  let bVar3: u8;
  let uVar4: u16;
  
  cVar1 = *(param_1 + -0x4);
  if (cVar1 == '*') {
    uVar4 = pass1_1000_34cf(param_1,param_2);
    if (uVar4 < 0x0) {
      uVar4 = 0xffff;
    }
  }
  else {
    uVar4 = (param_1 + -0xe) * 0xa + (byte)(cVar1 - 0x30);
  }
  (param_1 + -0xe) = uVar4;
  pcVar2 = *(char **)(param_1 + 0xa);
  cVar1 = *pcVar2;
  (param_1 + 0xa) = pcVar2 + 0x1;
  *(param_1 + -0x4) = cVar1;
  if ((cVar1 != '\0') && (-0x1 < (param_1 + -0xa))) {
    if ((byte)(cVar1 - 0x20U) < 0x59) {
      bVar3 = *(byte *)((byte)(cVar1 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar3 = 0x0;
    }
    bVar3 = *(byte *)((byte)(bVar3 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >>
            0x4;
    *(byte *)(param_1 + -0x7) = bVar3;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar4 = (**(code **)((char)bVar3 * 0x2 + 0x30a4))();
    return uVar4;
  }
  return (param_1 + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is
// not used

fn pass1_1000_31c5(param_1: i16,param_2: u16) -> u16

{
  byte *pbVar1;
  let cVar2: u8;
  char *pcVar3;
  let bVar4: u8;
  let uVar5: u16;
  
  cVar2 = *(param_1 + -0x4);
  if (cVar2 == 'l') {
    pbVar1 = (byte *)(param_1 + -0x6);
    *pbVar1 = *pbVar1 | 0x10;
  }
  else {
    if (cVar2 == 'F') {
      pbVar1 = (byte *)(param_1 + -0x6);
      *pbVar1 = *pbVar1 | 0x20;
    }
    else {
      if (cVar2 == 'N') {
        pbVar1 = (byte *)(param_1 + -0x5);
        *pbVar1 = *pbVar1 | 0x10;
      }
      else {
        if (cVar2 == 'L') {
          pbVar1 = (byte *)(param_1 + -0x5);
          *pbVar1 = *pbVar1 | 0x4;
        }
        else {
          pbVar1 = (byte *)(param_1 + -0x5);
          *pbVar1 = *pbVar1 | 0x8;
        }
      }
    }
  }
  pcVar3 = *(char **)(param_1 + 0xa);
  cVar2 = *pcVar3;
  (param_1 + 0xa) = pcVar3 + 0x1;
  *(param_1 + -0x4) = cVar2;
  if ((cVar2 != '\0') && (-0x1 < (param_1 + -0xa))) {
    if ((byte)(cVar2 - 0x20U) < 0x59) {
      bVar4 = *(byte *)((byte)(cVar2 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar4 = 0x0;
    }
    bVar4 = *(byte *)((byte)(bVar4 * '\b' + *(param_1 + -0x7)) + 0x5ffe) >>
            0x4;
    *(byte *)(param_1 + -0x7) = bVar4;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar5 = (**(code **)((char)bVar4 * 0x2 + 0x30a4))();
    return uVar5;
  }
  return (param_1 + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

fn pass1_1000_31f7(param_1: u16,param_2: i16,param_3: *mut u16,param_4: i16,param_5: u16) -> u16

{
  let piVar1: *mut i16;
  byte *pbVar2;
  let puVar3: *mut u16;
  let cVar4: u8;
  char *pcVar5;
  let bVar6: u8;
  let uVar7: u16;
  let iVar8: i16;
  let iVar9: i16;
  let iVar10: i16;
  let puVar11: *mut u16;
  let uVar12: u16;
  char *pcVar13;
  char *pcVar14;
  let bVar15: bool;
  let uVar16: u32;
  
  cVar4 = *(param_2 + -0x4);
  if ((cVar4 == 'd') || (cVar4 == 'i')) {
    pbVar2 = (byte *)(param_2 + -0x6);
    *pbVar2 = *pbVar2 | 0x40;
LAB_1000_3399:
    *(param_2 + -0x8) = 0xa;
LAB_1000_33d4:
    if ((*(byte *)(param_2 + -0x6) & 0x10) == 0x0) {
      uVar7 = pass1_1000_34cf(param_2,param_5);
      if ((*(byte *)(param_2 + -0x6) & 0x40) == 0x0) {
        uVar16 = uVar7;
      }
      else {
        uVar16 = SEXT24(uVar7);
      }
    }
    else {
      uVar16 = pass1_1000_34d8(param_2,param_5);
    }
    if (((*(byte *)(param_2 + -0x6) & 0x40) != 0x0) && ((long)uVar16 < 0x0)) {
      pbVar2 = (byte *)(param_2 + -0x5);
      *pbVar2 = *pbVar2 | 0x1;
      uVar16 = CONCAT22(-((uVar16 >> 0x10) + (uVar16 != 0x0)),-uVar16
                       );
    }
    if ((param_2 + -0xe) < 0x0) {
      (param_2 + -0xe) = 0x1;
    }
    else {
      pbVar2 = (byte *)(param_2 + -0x6);
      *pbVar2 = *pbVar2 & 0xf7;
    }
    if (uVar16 == 0x0) {
      (param_2 + -0x12) = 0x0;
    }
    puVar11 = *(byte *)(param_2 + -0x8);
    pass1_1000_356e(uVar16,puVar11,(uVar16 >> 0x10),param_2,
                    (param_2 + -0xe),(byte *)(param_2 + -0x17),param_5,param_5);
    if (((*(byte *)(param_2 + -0x5) & 0x2) != 0x0) &&
       ((puVar11 == 0x0 || (*(byte *)(param_2 + -0x17) != 0x30)))) {
      *(param_2 + -0x18) = 0x30;
      puVar11 = (puVar11 + 0x1);
    }
  }
  else {
    if (cVar4 == 'u') goto LAB_1000_3399;
    if (cVar4 == 'X') {
      *(param_2 + -0x3) = 0x7;
LAB_1000_33a9:
      if ((*(byte *)(param_2 + -0x6) & 0x80) != 0x0) {
        (param_2 + -0x12) = 0x2;
        *(param_2 + -0x10) = 0x30;
        *(param_2 + -0xf) = *(param_2 + -0x3) + 'Q';
      }
      *(param_2 + -0x8) = 0x10;
      goto LAB_1000_33d4;
    }
    if (cVar4 == 'x') {
      *(param_2 + -0x3) = 0x27;
      goto LAB_1000_33a9;
    }
    if (cVar4 == 'o') {
      if ((*(byte *)(param_2 + -0x6) & 0x80) != 0x0) {
        pbVar2 = (byte *)(param_2 + -0x5);
        *pbVar2 = *pbVar2 | 0x2;
      }
      *(param_2 + -0x8) = 0x8;
      goto LAB_1000_33d4;
    }
    if (cVar4 == 'c') {
      uVar7 = pass1_1000_34cf(param_2,param_5);
      *(param_2 + -0x216) = uVar7;
      puVar11 = (&PTR_LOOP_1050_0000 + 0x1);
    }
    else {
      if (cVar4 == 's') {
        pass1_1000_34e6(param_1,param_2,param_5);
        if ((param_3 != 0x0) || (puVar11 = DAT_1050_605d, param_4 != 0x0)) {
          iVar10 = (param_2 + -0xe);
          puVar11 = param_3;
          if (iVar10 != 0x0) {
            bVar15 = true;
            do {
              if (iVar10 == 0x0) break;
              iVar10 += -0x1;
              puVar3 = puVar11;
              puVar11 = (puVar11 + 0x1);
              bVar15 = *puVar3 == '\0';
            } while (!bVar15);
            if (bVar15) {
              puVar11 = (puVar11 + -0x1);
            }
          }
          puVar11 = (puVar11 - param_3);
        }
      }
      else {
        if (cVar4 == 'n') {
          pass1_1000_34e6(param_1,param_2,param_5);
          *param_3 = (param_2 + -0xa);
          if ((*(byte *)(param_2 + -0x6) & 0x10) != 0x0) {
            param_3[0x1] = 0x0;
          }
          goto LAB_1000_30cf;
        }
        if (cVar4 == 'p') {
          if ((*(byte *)(param_2 + -0x6) & 0x30) == 0x0) {
            uVar7 = pass1_1000_34cf(param_2,param_5);
            uVar16 = uVar7;
          }
          else {
            uVar16 = pass1_1000_34d8(param_2,param_5);
            uVar12 = (uVar16 >> 0x10);
            if ((*(byte *)(param_2 + -0x5) & 0x18) == 0x0) {
              *(param_2 + -0x3) = 0x7;
              pass1_1000_356e(uVar16,0x10,0x0,param_2,0x4,(byte *)(param_2 + -0x20e)
                              ,param_5,param_5);
              pass1_1000_356e(uVar12,0x10,0x0,param_2,0x4,(byte *)(param_2 + -0x213),
                              param_5,param_5);
              *(param_2 + -0x212) = 0x3a;
              puVar11 = &DAT_1050_0009;
              goto LAB_1000_3444;
            }
          }
          *(param_2 + -0x3) = 0x7;
          pass1_1000_356e(uVar16,0x10,0x0,param_2,0x4,(byte *)(param_2 + -0x213),
                          param_5,param_5);
          puVar11 = &DAT_1050_0004;
        }
        else {
          if ((cVar4 == 'E') || (cVar4 == 'G')) {
            piVar1 = (i16 *)(param_2 + -0x14);
            *piVar1 = *piVar1 + 0x1;
          }
          pbVar2 = (byte *)(param_2 + -0x6);
          *pbVar2 = *pbVar2 | 0x40;
          bVar6 = *(byte *)(param_2 + -0x4) | 0x20;
          iVar10 = (param_2 + -0xe);
          if (iVar10 < 0x1) {
            if (iVar10 == 0x0) {
              if (bVar6 == 0x67) {
                (param_2 + -0xe) = 0x1;
              }
            }
            else {
              (param_2 + -0xe) = 0x6;
            }
          }
          pcVar13 = (param_2 + -0x216);
          if ((*(byte *)(param_2 + -0x5) & 0x4) == 0x0) {
            (*(code *)PTR_s_3_wav_1050_25cc_1050_6068)();
            piVar1 = (i16 *)(param_2 + 0xe);
            *piVar1 = *piVar1 + 0x8;
          }
          else {
            (*(code *)PTR_s_3_wav_1050_25cc_1050_607c)();
            piVar1 = (i16 *)(param_2 + 0xe);
            *piVar1 = *piVar1 + 0xa;
          }
          if (((*(byte *)(param_2 + -0x6) & 0x80) != 0x0) &&
             ((param_2 + -0xe) == 0x0)) {
            (*(code *)PTR_s_3_wav_1050_25cc_1050_6074)();
          }
          if ((bVar6 == 0x67) && (((param_2 + -0x6) & 0x80) == 0x0)) {
            (*(code *)PTR_s_3_wav_1050_25cc_1050_6070)();
          }
          if (*pcVar13 == '-') {
            pcVar13 = (param_2 + -0x215);
            pbVar2 = (byte *)(param_2 + -0x5);
            *pbVar2 = *pbVar2 | 0x1;
          }
          iVar10 = -0x1;
          pcVar14 = pcVar13;
          do {
            if (iVar10 == 0x0) break;
            iVar10 += -0x1;
            pcVar5 = pcVar14;
            pcVar14 = pcVar14 + 0x1;
          } while (*pcVar5 != '\0');
          puVar11 = (pcVar14 + (-0x1 - pcVar13));
        }
      }
    }
  }
LAB_1000_3444:
  if ((*(byte *)(param_2 + -0x6) & 0x40) != 0x0) {
    if ((*(byte *)(param_2 + -0x5) & 0x1) == 0x0) {
      if ((*(byte *)(param_2 + -0x6) & 0x1) == 0x0) {
        if ((*(byte *)(param_2 + -0x6) & 0x2) != 0x0) {
          *(param_2 + -0x10) = 0x20;
          (param_2 + -0x12) = 0x1;
        }
      }
      else {
        *(param_2 + -0x10) = 0x2b;
        (param_2 + -0x12) = 0x1;
      }
    }
    else {
      *(param_2 + -0x10) = 0x2d;
      (param_2 + -0x12) = 0x1;
    }
  }
  iVar8 = (param_2 + -0xc) - puVar11;
  iVar10 = (param_2 + -0x12);
  iVar9 = iVar8 - iVar10;
  if (iVar8 < iVar10) {
    iVar9 = 0x0;
  }
  if ((*(byte *)(param_2 + -0x6) & 0xc) == 0x0) {
    pass1_1000_3552(iVar9,param_2,param_5);
  }
  pass1_1000_3534((param_2 + -0x12),param_2,param_5);
  if (((*(byte *)(param_2 + -0x6) & 0x8) != 0x0) &&
     ((*(byte *)(param_2 + -0x6) & 0x4) == 0x0)) {
    pass1_1000_3552(iVar9,param_2,param_5);
  }
  pass1_1000_3534(puVar11,param_2,param_5);
  if ((*(byte *)(param_2 + -0x6) & 0x4) != 0x0) {
    pass1_1000_3552(iVar9,param_2,param_5);
  }
LAB_1000_30cf:
  pcVar5 = *(char **)(param_2 + 0xa);
  cVar4 = *pcVar5;
  (param_2 + 0xa) = pcVar5 + 0x1;
  *(param_2 + -0x4) = cVar4;
  if ((cVar4 != '\0') && (-0x1 < (param_2 + -0xa))) {
    if ((byte)(cVar4 - 0x20U) < 0x59) {
      bVar6 = *(byte *)((byte)(cVar4 - 0x20U) + 0x5ffe) & 0xf;
    }
    else {
      bVar6 = 0x0;
    }
    bVar6 = *(byte *)((byte)(bVar6 * '\b' + *(param_2 + -0x7)) + 0x5ffe) >>
            0x4;
    *(byte *)(param_2 + -0x7) = bVar6;
                    // WARNING: Could not recover jumptable at 0x1000310e. Too many
                    // branches
                    // WARNING: Treating indirect jump as call
    uVar7 = (**(code **)((char)bVar6 * 0x2 + 0x30a4))();
    return uVar7;
  }
  return (param_2 + -0xa);
}



fn pass1_1000_34cf(param_1: i16,param_2: u16) -> u16

{
  let uVar1: u16;
  let puVar2: *mut u16;
  
  puVar2 = *(u16 **)(param_1 + 0xe);
  uVar1 = *puVar2;
  (param_1 + 0xe) = puVar2 + 0x2;
  return uVar1;
}



fn pass1_1000_34d8(param_1: i16,param_2: u16) -> u32

{
  let uVar1: u16;
  let uVar2: u16;
  let puVar3: *mut u16;
  
  puVar3 = (param_1 + 0xe);
  uVar1 = *puVar3;
  uVar2 = (puVar3 + 0x2);
  (param_1 + 0xe) = puVar3 + 0x4;
  return CONCAT22(uVar2,uVar1);
}



fn pass1_1000_34e6(param_1: u16,param_2: i16,param_3: u16) -> u32

{
  let uVar1: u16;
  let uVar2: u32;
  
  if ((*(byte *)(param_2 + -0x6) & 0x20) != 0x0) {
    uVar2 = pass1_1000_34d8(param_2,param_3);
    return uVar2;
  }
  uVar1 = pass1_1000_34cf(param_2,param_3);
  if (uVar1 == 0x0) {
    return param_1 << 0x10;
  }
  return CONCAT22(param_1,uVar1);
}



i16 
pass1_1000_3503(char param_1,param_2: u16,param_3: i16,param_4: u16,param_5: u16,
               param_6: u8)

{
  let piVar1: *mut i16;
  char *pcVar2;
  char **ppcVar3;
  let uVar4: u16;
  let piVar5: *mut i16;
  let uVar6: u16;
  
  ppcVar3 = (char **)*(i16 **)(param_3 + 0x6);
  uVar6 = (ppcVar3 >> 0x10);
  piVar5 = (i16 *)ppcVar3;
  piVar1 = piVar5 + 0x2;
  *piVar1 = *piVar1 + -0x1;
  if (*piVar1 < 0x0) {
    uVar4 = mem_1000_2bb6(param_1,piVar5,param_3,uVar6,param_4,param_5,param_6,
                          param_2);
    if (uVar4 == 0xffff) {
      return -0x1;
    }
  }
  else {
    pcVar2 = *ppcVar3;
    *ppcVar3 = *ppcVar3 + 0x1;
    *pcVar2 = param_1;
  }
  return 0x0;
}



fn pass1_1000_3534(param_1: i16,param_2: i16,param_3: u16)
{
  let piVar1: *mut i16;
  let puVar2: *mut u8;
  let uVar3: u16;
  let in_DX: u16;
  let unaff_DI: *mut u8;
  let uVar4: u16;
  let unaff_ES: u16;
  let unaff_CS: u16;
  let in_AF: u8;
  
  if (param_1 != 0x0) {
    piVar1 = (i16 *)(param_2 + -0xa);
    *piVar1 = *piVar1 + param_1;
    uVar4 = 0x0;
    do {
      puVar2 = unaff_DI;
      unaff_DI = unaff_DI + 0x1;
      uVar3 = pass1_1000_3503(*puVar2,in_DX,param_2,unaff_CS,param_3,in_AF);
      uVar4 |= uVar3;
      param_1 += -0x1;
    } while (param_1 != 0x0);
    if (uVar4 != 0x0) {
      (param_2 + -0xa) = 0xffff;
    }
  }
  return;
}



fn pass1_1000_3552(param_1: i16,param_2: i16,param_3: u16)
{
  let piVar1: *mut i16;
  let uVar2: u16;
  let in_DX: u16;
  let uVar3: u16;
  let unaff_CS: u16;
  let in_AF: u8;
  
  if (param_1 != 0x0) {
    piVar1 = (i16 *)(param_2 + -0xa);
    *piVar1 = *piVar1 + param_1;
    uVar3 = 0x0;
    do {
      uVar2 = pass1_1000_3503((char)in_DX,in_DX,param_2,unaff_CS,param_3,in_AF);
      uVar3 |= uVar2;
      param_1 += -0x1;
    } while (param_1 != 0x0);
    if (uVar3 != 0x0) {
      (param_2 + -0xa) = 0xffff;
    }
  }
  return;
}



void 
pass1_1000_356e(param_1: u16,param_2: u16,param_3: u16,param_4: i16,param_5: i16,
               param_6: *mut u8,param_7: u16,param_8: u16)

{
  byte *pbVar1;
  let uVar2: u32;
  let bVar3: u8;
  
  while (((0x0 < param_5 || (param_1 != 0x0)) || (param_3 != 0x0))) {
    uVar2 = param_3;
    param_3 /= param_2;
    uVar2 = uVar2 % param_2 << 0x10 | param_1;
    param_1 = (uVar2 / param_2);
    bVar3 = (uVar2 % param_2) + 0x30;
    if (0x39 < bVar3) {
      bVar3 += *(param_4 + -0x3);
    }
    pbVar1 = param_6;
    param_6 = param_6 + -0x1;
    *pbVar1 = bVar3;
    param_5 += -0x1;
  }
  return;
}



fn pass1_1000_35aa(void) -> u16

{
  let puVar1: *mut u16;
  
  puVar1 = &PTR_LOOP_1050_6210;
  while( true ) {
    if (PTR_LOOP_1050_5ff0 < puVar1) {
      return 0x0;
    }
    if ((*(byte *)(puVar1 + 0x5) & 0x83) == 0x0) break;
    puVar1 = puVar1 + 0x6;
  }
  *(puVar1 + 0x5) = 0x0;
  puVar1[0x2] = 0x0;
  puVar1[0x4] = 0x0;
  puVar1[0x3] = 0x0;
  puVar1[0x1] = 0x0;
  *puVar1 = 0x0;
  *(puVar1 + 0xb) = 0xff;
  return puVar1;
}




fn pass1_1000_39e1(void)
{
  return;
}


i16  pass1_1000_3bac(void)

{
  let iVar1: i16;
  
  if (PTR_LOOP_1050_5f48 < &stack0x0004) {
    iVar1 = -(PTR_LOOP_1050_5f48 + -&stack0x0004);
  }
  else {
    iVar1 = 0x0;
  }
  return iVar1;
}



void 
pass1_1000_3bc0(param_1: i16,param_2: i16,param_3: *mut u16,param_4: u16,param_5: u16,
               param_6: u16)

{
  let piVar1: *mut i16;
  let uVar2: u16;
  let uVar3: u16;
  let uVar4: u16;
  let iVar5: i16;
  let puVar6: *mut u16;
  let bVar7: bool;
  let uVar8: u32;
  
  if ((*(byte *)(param_2 + 0x2) & 0x1) != 0x0) {
    pass1_1000_3cb7(param_2);
    uVar4 = *param_3;
    if ((uVar4 & 0x1) != 0x0) {
      param_1 = (param_1 - uVar4) + -0x1;
    }
    uVar4 = (param_2 + 0x4);
    if (uVar4 != 0x0) {
      uVar3 = param_1 + 0x2U + uVar4;
      if (!CARRY2(param_1 + 0x2U,uVar4)) {
        param_4 = pass1_1000_29dc(param_6);
        uVar4 = &PTR_LOOP_1050_6066;
        if (uVar4 == 0x1000) goto LAB_1000_3c12;
        uVar2 = 0x8000;
        while (uVar4 <= uVar2) {
          uVar2 >>= 0x1;
          if (uVar2 == 0x0) goto LAB_1000_3c2b;
        }
        if (uVar2 < 0x8) goto LAB_1000_3c2b;
        uVar4 = uVar2 << 0x1;
        goto LAB_1000_3c12;
      }
      uVar2 = 0x0;
      uVar4 = 0xfff0;
      if (uVar3 == 0x0) {
        while( true ) {
          bVar7 = false;
          uVar8 = mixed_mem_op_1000_3c51(uVar2,uVar3,param_2,param_4,param_5,0x3c23);
          if (!bVar7) break;
          if (uVar4 == 0xfff0) {
            return;
          }
LAB_1000_3c2b:
          uVar4 = 0x10;
LAB_1000_3c12:
          uVar4 -= 0x1;
          uVar2 = uVar4 + uVar3;
          if (CARRY2(uVar4,uVar3)) {
            uVar2 = 0x0;
          }
          uVar4 = ~uVar4;
          uVar2 &= uVar4;
        }
        iVar5 = uVar8 - (param_2 + 0x4);
        (param_2 + 0x4) = uVar8;
        *(u16 **)(param_2 + 0xa) = param_3;
        piVar1 = *(i16 **)(param_2 + 0xc);
        *piVar1 = iVar5 + -0x1;
        puVar6 = (piVar1 + iVar5);
        *puVar6 = 0xfffe;
        *(u16 **)(param_2 + 0xc) = puVar6;
      }
    }
  }
  return;
}


fn pass1_1000_3cb7(param_1: i16)
{
  let uVar1: u16;
  let puVar2: *mut u16;
  
  puVar2 = *(u16 **)(param_1 + 0xa);
  if (puVar2 == *(u16 **)(param_1 + 0xc)) {
    puVar2 = *(u16 **)(param_1 + 0x8);
  }
  while( true ) {
    uVar1 = *puVar2;
    if (uVar1 == 0xfffe) break;
    puVar2 = (puVar2 + (uVar1 & 0xfffe) + 0x2);
  }
  return;
}



fn pass1_1000_3cd8(param_1: u16,param_2: u16)
{
  free_mem_1000_407a(param_1,param_2,&stack0xfffe);
  return;
}



fn pass1_1000_3cea(Uparam_1: i32,Uparam_2: i32) -> u16

{
  let pUVar1: *mut u16;
  char *pcVar2;
  let pUVar3: *mut u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let pUVar7: *mut u16;
  char *pcVar8;
  let pUVar9: *mut u16;
  let pUVar10: *mut u16;
  let uVar11: u16;
  let uVar12: u16;
  let bVar13: bool;
  
  uVar11 = (param_1 >> 0x10);
  bVar13 = true;
  iVar4 = -0x1;
  pUVar7 = param_1;
  do {
    if (iVar4 == 0x0) break;
    iVar4 += -0x1;
    pUVar1 = pUVar7;
    pUVar7 = (pUVar7 + 0x1);
    bVar13 = *pUVar1 == '\0';
  } while (!bVar13);
  pUVar10 = (pUVar7 + -0x1);
  uVar12 = (param_2 >> 0x10);
  pcVar8 = param_2;
  uVar5 = 0xffff;
  do {
    if (uVar5 == 0x0) break;
    uVar5 -= 0x1;
    pcVar2 = pcVar8;
    pcVar8 = pcVar8 + 0x1;
    bVar13 = *pcVar2 == '\0';
  } while (!bVar13);
  uVar5 = ~uVar5;
  if (!bVar13) {
    pcVar8 = pcVar8 + -uVar5;
    uVar5 += 0x1;
  }
  pUVar9 = (pcVar8 + -uVar5);
  if (uVar5 == 0x0) {
    pUVar1 = pUVar9;
    pUVar9 = pUVar9 + 0x1;
    *pUVar10 = *pUVar1;
    uVar5 = 0xfffe;
    pUVar10 = (pUVar7 + 0x1);
  }
  else {
    if ((pUVar9 & 0x1) != 0x0) {
      pUVar1 = pUVar9;
      pUVar9 = (pUVar9 + 0x1);
      *pUVar10 = *pUVar1;
      uVar5 -= 0x1;
      pUVar10 = pUVar7;
    }
  }
  for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
    pUVar3 = pUVar10;
    pUVar10 = pUVar10 + 0x1;
    pUVar1 = pUVar9;
    pUVar9 = pUVar9 + 0x1;
    *pUVar3 = *pUVar1;
  }
  for (uVar5 = ((uVar5 & 0x1) != 0x0); uVar5 != 0x0; uVar5 -= 0x1) {
    pUVar3 = pUVar10;
    pUVar10 = (pUVar10 + 0x1);
    pUVar1 = pUVar9;
    pUVar9 = (pUVar9 + 0x1);
    *pUVar3 = *pUVar1;
  }
  return param_1;
}



i16  pass1_1000_3d7a(param_1: u32,param_2: u32)

{
  byte *pbVar1;
  char *pcVar2;
  byte *pbVar3;
  let iVar4: i16;
  let uVar5: u16;
  char *pcVar6;
  byte *pbVar7;
  char *pcVar8;
  byte *pbVar9;
  let uVar10: u16;
  let bVar11: bool;
  let bVar12: bool;
  
  pbVar7 = (byte *)param_1;
  uVar10 = (param_2 >> 0x10);
  pcVar8 = param_2;
  iVar4 = 0x0;
  uVar5 = 0xffff;
  do {
    if (uVar5 == 0x0) break;
    uVar5 -= 0x1;
    pcVar2 = pcVar8;
    pcVar8 = pcVar8 + 0x1;
  } while (*pcVar2 != '\0');
  pcVar6 = ~uVar5;
  bVar11 = pcVar8 < pcVar6;
  pbVar9 = (byte *)(pcVar8 + -pcVar6);
  bVar12 = pbVar9 == (byte *)0x0;
  do {
    if (pcVar6 == 0x0) break;
    pcVar6 = pcVar6 + -0x1;
    pbVar3 = pbVar9;
    pbVar9 = pbVar9 + 0x1;
    pbVar1 = pbVar7;
    pbVar7 = pbVar7 + 0x1;
    bVar11 = *pbVar1 < *pbVar3;
    bVar12 = *pbVar1 == *pbVar3;
  } while (bVar12);
  if (!bVar12) {
    iVar4 = (0x1 - bVar11) - (bVar11 != 0x0);
  }
  return iVar4;
}



u16 
pass1_1000_3de8(char *param_1,char *param_2,param_3: u16,param_4: u16,param_5: u16
               )

{
  byte *pbVar1;
  char *pcVar2;
  char *pcVar3;
  let bVar4: u8;
  let uVar5: u16;
  let iVar6: i16;
  char *pcVar7;
  char *pcVar8;
  let uVar9: u16;
  let uVar10: u16;
  let bVar11: bool;
  
  if (param_3 != 0x0) {
    uVar9 = (param_1 >> 0x10);
    pcVar8 = param_1;
    uVar5 = param_3;
    pcVar7 = pcVar8;
    do {
      if (uVar5 == 0x0) break;
      uVar5 -= 0x1;
      pcVar2 = pcVar7;
      pcVar7 = pcVar7 + 0x1;
    } while (*pcVar2 != '\0');
    iVar6 = param_3 - uVar5;
    uVar10 = (param_2 >> 0x10);
    pcVar7 = param_2;
    do {
      if (iVar6 == 0x0) break;
      iVar6 += -0x1;
      pcVar3 = pcVar8;
      pcVar8 = pcVar8 + 0x1;
      pcVar2 = pcVar7;
      pcVar7 = pcVar7 + 0x1;
    } while (*pcVar2 == *pcVar3);
    bVar4 = pcVar7[-0x1];
    uVar5 = 0x0;
    pbVar1 = (byte *)(pcVar8 + -0x1);
    bVar11 = bVar4 == *pbVar1;
    if (bVar4 < *pbVar1 || bVar11) {
      if (bVar11) {
        return 0x0;
      }
      uVar5 = 0xfffe;
    }
    param_3 = ~uVar5;
  }
  return param_3;
}



i16  pass1_1000_3e2c(param_1: u32)

{
  byte *pbVar1;
  let bVar2: u8;
  let bVar3: u8;
  let iVar4: i16;
  byte *pbVar5;
  let uVar6: u16;
  
  uVar6 = (param_1 >> 0x10);
  pbVar5 = (byte *)param_1;
  iVar4 = 0x0;
  do {
    do {
      pbVar1 = pbVar5;
      pbVar5 = pbVar5 + 0x1;
      bVar2 = *pbVar1;
    } while (bVar2 == 0x20);
  } while (bVar2 == 0x9);
  if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) goto LAB_1000_3e4d;
  while( true ) {
    pbVar1 = pbVar5;
    pbVar5 = pbVar5 + 0x1;
    bVar3 = *pbVar1;
LAB_1000_3e4d:
    if ((0x39 < bVar3) || (bVar3 < 0x30)) break;
    iVar4 = iVar4 * 0xa + (byte)(bVar3 - 0x30);
  }
  if (bVar2 == 0x2d) {
    iVar4 = -iVar4;
  }
  return iVar4;
}



i16  pass1_1000_3e2c(param_1: u32)

{
  byte *pbVar1;
  let bVar2: u8;
  let bVar3: u8;
  let iVar4: i16;
  byte *pbVar5;
  let uVar6: u16;
  
  uVar6 = (param_1 >> 0x10);
  pbVar5 = (byte *)param_1;
  iVar4 = 0x0;
  do {
    do {
      pbVar1 = pbVar5;
      pbVar5 = pbVar5 + 0x1;
      bVar2 = *pbVar1;
    } while (bVar2 == 0x20);
  } while (bVar2 == 0x9);
  if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) goto LAB_1000_3e4d;
  while( true ) {
    pbVar1 = pbVar5;
    pbVar5 = pbVar5 + 0x1;
    bVar3 = *pbVar1;
LAB_1000_3e4d:
    if ((0x39 < bVar3) || (bVar3 < 0x30)) break;
    iVar4 = iVar4 * 0xa + (byte)(bVar3 - 0x30);
  }
  if (bVar2 == 0x2d) {
    iVar4 = -iVar4;
  }
  return iVar4;
}



i16  pass1_1000_3e2c(param_1: u32)

{
  byte *pbVar1;
  let bVar2: u8;
  let bVar3: u8;
  let iVar4: i16;
  byte *pbVar5;
  let uVar6: u16;
  
  uVar6 = (param_1 >> 0x10);
  pbVar5 = (byte *)param_1;
  iVar4 = 0x0;
  do {
    do {
      pbVar1 = pbVar5;
      pbVar5 = pbVar5 + 0x1;
      bVar2 = *pbVar1;
    } while (bVar2 == 0x20);
  } while (bVar2 == 0x9);
  if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) goto LAB_1000_3e4d;
  while( true ) {
    pbVar1 = pbVar5;
    pbVar5 = pbVar5 + 0x1;
    bVar3 = *pbVar1;
LAB_1000_3e4d:
    if ((0x39 < bVar3) || (bVar3 < 0x30)) break;
    iVar4 = iVar4 * 0xa + (byte)(bVar3 - 0x30);
  }
  if (bVar2 == 0x2d) {
    iVar4 = -iVar4;
  }
  return iVar4;
}



byte *  pass1_1000_3e82(param_1: u16,uchar *param_2,param_3: u16)

{
  byte *pbVar1;
  let uVar2: u32;
  let bVar3: u8;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  byte *pbVar8;
  byte *pbVar9;
  byte *pbVar10;
  byte *pbVar11;
  let uVar12: u16;
  let bVar13: bool;
  let cVar4: u8;
  
  uVar6 = 0x0;
  if (param_3 == 0xa) {
    uVar6 = param_1 >> 0xf;
  }
  uVar12 = (param_2 >> 0x10);
  pbVar9 = (byte *)param_2;
  pbVar10 = pbVar9;
  pbVar8 = pbVar9;
  if (((true) && (param_3 == 0xa)) && (uVar6 < 0x0)) {
    pbVar10 = pbVar9 + 0x1;
    *param_2 = '-';
    bVar13 = param_1 != 0x0;
    param_1 = -param_1;
    uVar6 = -(uVar6 + bVar13);
    pbVar8 = pbVar10;
  }
  do {
    uVar7 = 0x0;
    uVar5 = uVar6;
    if (uVar6 != 0x0) {
      uVar5 = uVar6 / param_3;
      uVar7 = uVar6 % param_3;
    }
    uVar2 = CONCAT22(uVar7,param_1);
    param_1 = (uVar2 / param_3);
    cVar4 = (uVar2 % param_3);
    bVar3 = cVar4 + 0x30;
    if (0x39 < bVar3) {
      bVar3 = cVar4 + 0x57;
    }
    pbVar11 = pbVar10 + 0x1;
    *pbVar10 = bVar3;
    uVar6 = uVar5;
    pbVar10 = pbVar11;
  } while ((uVar5 | param_1) != 0x0);
  *pbVar11 = 0x0;
  do {
    pbVar11 = pbVar11 + -0x1;
    pbVar1 = pbVar11;
    bVar3 = *pbVar1;
    *pbVar1 = *pbVar8;
    *pbVar8 = bVar3;
    pbVar10 = pbVar8 + 0x2;
    pbVar8 = pbVar8 + 0x1;
  } while (pbVar10 < pbVar11);
  return pbVar9;
}


i16  pass1_1000_3ec0(param_1: u16,param_2: u16)

{
  let uVar1: u16;
  let uVar2: u16;
  u16_t uVar3;
  u16_t unaff_SI;
  u16_t uVar4;
  let puVar4: u32;
  
  puVar4 = CONCAT22(PTR_LOOP_1050_5fc0,PTR_LOOP_1050_5fbe);
  if (((PTR_LOOP_1050_5fc0 | PTR_LOOP_1050_5fbe) != 0x0) &&
     ((param_2 | param_1) != 0x0)) {
    uVar1 = str_op_1000_3da4(CONCAT22(param_2,param_1));
    while( true ) {
      uVar4 = (u16_t)(puVar4 >> 0x10);
      uVar3 = (u16_t)puVar4;
      if (((uVar3 + 0x2) | puVar4) == 0x0) break;
      uVar2 = str_op_1000_3da4(CONCAT22((uVar3 + 0x2),
                                                puVar4));
      if (((uVar1 < uVar2) && (*(*puVar4 + uVar1) == '=')) &&
         (uVar2 = pass1_1000_3de8(CONCAT22((uVar3 + 0x2),
                                                   puVar4),
                                  CONCAT22(param_2,param_1),uVar1,unaff_SI,uVar3),
         uVar2 == 0x0)) {
        return puVar4 + uVar1 + 0x1;
      }
      puVar4 = (puVar4 & 0xffff0000 | (uVar3 + 0x4));
    }
  }
  return 0x0;
}



i16 
pass1_1000_3f5c(param_1: i16,param_2: u16,param_3: u16,param_4: u16,param_5: u8)

{
  let uVar1: u16;
  let puVar2: *mut u16;
  let iVar3: i16;
  let iStack2: i16;
  
  iStack2 = param_1 + 0x1;
  iVar3 = 0x0;
  if (PTR_LOOP_1050_61ec == 0x0) {
    puVar2 = &PTR_LOOP_1050_6210;
  }
  else {
    puVar2 = 0x6234;
  }
  for (; puVar2 <= PTR_LOOP_1050_5ff0; puVar2 = puVar2 + 0x6) {
    uVar1 = pass1_1000_2a00(puVar2,&iStack2,param_2,param_3,param_4,param_5);
    if (uVar1 != 0xffff) {
      iVar3 += 0x1;
    }
  }
  return iVar3;
}



uchar *  pass1_1000_400a(param_1: i16,param_2: u16)

{
  let puVar1: *mut u8
  let iStack2: i16;
  
  iStack2 = param_2 + 0x1;
  if ((param_1 < 0x0) || (PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e <= param_1)) {
    PTR_LOOP_1050_5f78 = &DAT_1050_0009;
    puVar1 = 0xffff;
  }
  else {
    if (((PTR_LOOP_1050_61ec == 0x0) ||
        ((param_1 < DAT_1050_5f8a && (0x2 < param_1)))) &&
       (0x31d < CONCAT11(DAT_1050_5f83,DAT_1050_5f82))) {
      puVar1 = PTR_LOOP_1050_5f88;
      if (((*(byte *)(param_1 + 0x5f90) & 0x1) == 0x0) ||
         (puVar1 = dos3_call_1000_5174(&iStack2), puVar1 != 0x0)
         ) {
        PTR_LOOP_1050_5f88 = puVar1;
        PTR_LOOP_1050_5f78 = &DAT_1050_0009;
        puVar1 = 0xffff;
      }
    }
    else {
      puVar1 = 0x0;
    }
  }
  return puVar1;
}


fn pass1_1000_41e0(param_1: i16) -> u16

{
  let piStack6: *mut i16;
  
  piStack6 = (i16 *)CONCAT22(PTR_LOOP_1050_6192,PTR_LOOP_1050_6190);
  while( true ) {
    if (PTR_LOOP_1050_6190 + (PTR_LOOP_1050_6194 & 0xfffc) <= piStack6)
    {
      return 0x0;
    }
    if (*piStack6 == param_1) break;
    piStack6 = (i16 *)(piStack6 & 0xffff0000 | ZEXT24(piStack6 + 0x4))
    ;
  }
  *piStack6 = 0x0;
  return (piStack6 + 0x2);
}



// WARNING: Could not reconcile some variable overlaps

fn pass1_1000_422a(param_1: i16,param_2: u16,param_3: u16,param_4: u16) -> i16

{
  let puVar1: *mut u8;
  let puVar2: *mut u8;
  let puVar3: *mut u8;
  let puVar4: *mut u8;
  let piStack6: *mut i16;
  
  piStack6 = (i16 *)CONCAT22(PTR_LOOP_1050_6192,PTR_LOOP_1050_6190);
  while( true ) {
    if (PTR_LOOP_1050_6190 + (PTR_LOOP_1050_6194 & 0xfffc) <= piStack6)
    {
      puVar2 = PTR_LOOP_1050_6194 + 0x28;
      puVar4 = PTR_LOOP_1050_6192;
      puVar3 = 
               pass1_1000_16aa((u16 *)PTR_LOOP_1050_6190,PTR_LOOP_1050_6192,
                               puVar2,PTR_LOOP_1050_6192,param_3,param_4);
      if ((puVar4 | puVar3) == 0x0) {
        param_1 = 0x0;
      }
      else {
        puVar1 = puVar3 + (PTR_LOOP_1050_6194 & 0xfffc);
        piStack6 = (i16 *)CONCAT22(puVar4,puVar1);
        PTR_LOOP_1050_6190 = puVar3;
        PTR_LOOP_1050_6192 = puVar4;
        *piStack6 = param_1;
        (puVar1 + 0x2) = param_2;
        PTR_LOOP_1050_6194 = puVar2;
        pass1_1000_4906((astruct_20 *)CONCAT22(puVar4,puVar1 + 0x4),(WNDCLASS16 *)0x0,0x24
                       );
      }
      return param_1;
    }
    if (*piStack6 == 0x0) break;
    piStack6 = (i16 *)(piStack6 & 0xffff0000 | ZEXT24(piStack6 + 0x4))
    ;
  }
  (piStack6 + 0x2) = param_2;
  *piStack6 = param_1;
  return param_1;
}


fn pass1_1000_43f0(param_1: u16,param_2: u16)
{
  if (PTR_LOOP_1050_68b4 == 0x0) {
    pass1_1000_440c(param_2);
    PTR_LOOP_1050_68b4 = PTR_LOOP_1050_68b4 + 0x1;
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn pass1_1000_440c(param_1: u16)
{
  let cVar1: u8;
  char *pcVar2;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let lVar6: i32;
  let uVar7: u16;
  let uVar8: u16;
  char *pcStack8;
  
  uVar3 = pass1_1000_3ec0(0x61ca,&USHORT_1050_1050);
  pcStack8 = CONCAT22(param_1,uVar3);
  if (((param_1 | uVar3) != 0x0) &&
     (_DAT_1050_61ce = CONCAT22(PTR_LOOP_1050_61d0,DAT_1050_61ce), *pcStack8 != '\0')) {
    str_op_1000_3dbe(CONCAT13((char)(PTR_USHORT_1050_1050_1050_61de >> 0x8),
                                      CONCAT12((char)PTR_USHORT_1050_1050_1050_61de,
                                               PTR_PTR_DAT_1050_5350_1050_61d4_1050_61dc))
                     ,CONCAT22(param_1,uVar3),0x3);
    pcStack8 = CONCAT22(param_1,uVar3 + 0x3);
    cVar1 = *pcStack8;
    if (cVar1 == '-') {
      pcStack8 = CONCAT22(param_1,uVar3 + 0x4);
    }
    uVar5 = 0x0;
    uVar8 = 0x0;
    uVar7 = 0xe10;
    iVar4 = pass1_1000_3e2c(pcStack8 & 0xffff | param_1 << 0x10);
    _DAT_1050_61ce = pass1_1000_52be((char)iVar4,uVar5,uVar7,uVar8);
    for (; (pcVar2 = pcStack8, *pcStack8 == '+' ||
           (('/' < *pcStack8 && (*pcStack8 < ':'))));
        pcStack8 = (pcStack8 & 0xffff0000 | (pcStack8 + 0x1)))
    {
    }
    if (*pcStack8 == ':') {
      uVar5 = 0x0;
      uVar8 = 0x0;
      uVar7 = 0x3c;
      pcStack8 = (pcStack8 & 0xffff0000 | (pcStack8 + 0x1));
      iVar4 = pass1_1000_3e2c(pcVar2 & 0xffff0000 | (pcStack8 + 0x1));
      lVar6 = pass1_1000_52be((char)iVar4,uVar5,uVar7,uVar8);
      uVar5 = (lVar6 >> 0x10);
      _DAT_1050_61ce += lVar6;
      for (; (pcVar2 = pcStack8, '/' < *pcStack8 && (*pcStack8 < ':'));
          pcStack8 = (pcStack8 & 0xffff0000 | (pcStack8 + 0x1))
          ) {
      }
      if (*pcStack8 == ':') {
        pcStack8 = (pcStack8 & 0xffff0000 | (pcStack8 + 0x1));
        iVar4 = pass1_1000_3e2c(pcVar2 & 0xffff0000 | (pcStack8 + 0x1))
        ;
        _DAT_1050_61ce += CONCAT22(uVar5,iVar4);
        for (; ('/' < *pcStack8 && (*pcStack8 < ':'));
            pcStack8 = (pcStack8 & 0xffff0000 |
                               (pcStack8 + 0x1))) {
        }
      }
    }
    PTR_LOOP_1050_61d0 = (_DAT_1050_61ce >> 0x10);
    if (cVar1 == '-') {
      _DAT_1050_61ce =
           CONCAT22(-(PTR_LOOP_1050_61d0 + (DAT_1050_61ce != 0x0)),-DAT_1050_61ce);
    }
    DAT_1050_61d2 = *pcStack8;
    if (DAT_1050_61d2 == 0x0) {
      *_PTR_PTR_1050_61e0 = '\0';
    }
    else {
      str_op_1000_3dbe(_PTR_PTR_1050_61e0,pcStack8,0x3);
    }
  }
  PTR_LOOP_1050_61d0 = (_DAT_1050_61ce >> 0x10);
  return;
}



fn pass1_1000_455a(param_1: u32,param_2: u16) -> u16

{
  let piVar1: *mut i16;
  let iVar2: i16;
  let uVar3: u16;
  let iVar4: i16;
  let UVar5: u16;
  let uVar6: u32;
  let iStack6: i16;
  
  if ((((param_1 + 0xa) < 0x43) || ((param_1 + 0x8) < 0x3)) ||
     (0x9 < (param_1 + 0x8))) goto LAB_1000_4623;
  if (((param_1 + 0x8) < 0x4) || (0x8 < (param_1 + 0x8))) {
    uVar3 = (param_1 + 0xa);
    if ((uVar3 < 0x57) || ((param_1 + 0x8) != 0x3)) {
      iStack6 = ((param_1 + 0x8) * 0x2 + 0x61b2);
    }
    else {
      iStack6 = ((param_1 + 0x8) * 0x2 + 0x61b0) + 0x7;
    }
    if ((uVar3 & 0x3) == 0x0) {
      iStack6 += 0x1;
    }
    uVar3 = (uVar3 - 0x46) * 0x16d + ((uVar3 - 0x1) >> 0x2) + iStack6;
    uVar6 = pass1_1000_52f0(uVar3 - 0xd,(uVar3 >> 0xf) - (uVar3 < 0xd),0x7,0x0)
    ;
    iStack6 = uVar6 - iStack6;
    iVar4 = -iStack6;
    if ((param_1 + 0x8) == 0x3) {
      iVar2 = (param_1 + 0xe);
      if ((iVar4 < iVar2) || ((-iVar2 == iStack6 && (0x1 < (param_1 + 0x4)))))
      goto LAB_1000_460e;
    }
    else {
      piVar1 = (i16 *)(param_1 + 0xe);
      iVar2 = *piVar1;
      if ((SBORROW2(*piVar1,iVar4) != iVar2 + iStack6 < 0x0) ||
         ((iVar2 == iVar4 && ((param_1 + 0x4) < 0x1)))) goto LAB_1000_460e;
    }
LAB_1000_4623:
    UVar5 = 0x0;
  }
  else {
LAB_1000_460e:
    UVar5 = 0x1;
  }
  return UVar5;
}



i16 
pass1_1000_462e(param_1: u16,param_2: i16,param_3: u16,param_4: u16,param_5: u16,
               param_6: i16,param_7: i16,param_8: u16,param_9: u16)

{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let iVar4: i16;
  let UVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u32;
  let iStack26: i16;
  let local_16: [u8;4];
  let uStack18: u16;
  let iStack14: i16;
  let iStack12: i16;
  let iStack8: i16;
  let local_4: u16;
  let iStack2: i16;
  let uVar10: u16;
  let uVar11: u16;
  let uVar12: u16;
  let uVar13: u16;
  
  iStack2 = param_7 + 0x1;
  local_4 = &USHORT_1050_1050;
  uVar8 = (param_2 * 0x2 + 0x61ae);
  if (((param_1 & 0x3) == 0x0) && (0x2 < param_2)) {
    uVar8 += 0x1;
  }
  pass1_1000_43f0((u16_t)&iStack2,param_9);
  uVar13 = 0x0;
  uVar12 = 0x3c;
  uVar11 = 0x0;
  uVar10 = 0x3c;
  uVar1 = ((long)param_1 * 0x16d);
  uVar2 = (param_1 + 0x3) >> 0x2;
  uVar3 = uVar2 + param_3;
  uVar6 = uVar1 + uVar3;
  uVar7 = uVar6 + uVar8;
  uVar9 = pass1_1000_52be(uVar7 + 0xe44,
                          (((long)param_1 * 0x16d) >> 0x10) +
                          ((param_1 + 0x3) >> 0xf) + (param_3 >> 0xf) +
                          CARRY2(uVar2,param_3) + CARRY2(uVar1,uVar3) +
                          (uVar8 >> 0xf) + CARRY2(uVar6,uVar8) +
                          (0xf1bb < uVar7),0x18,0x0);
  uVar9 = pass1_1000_52be(uVar9 + param_4,
                          (uVar9 >> 0x10) + (param_4 >> 0xf) +
                          CARRY2(uVar9,param_4),uVar10,uVar11);
  iVar4 = pass1_1000_52be(uVar9 + param_5,
                          (uVar9 >> 0x10) + (param_5 >> 0xf) +
                          CARRY2(uVar9,param_5),uVar12,uVar13);
  iStack26 = iVar4 + param_6 + DAT_1050_61ce;
  iStack8 = param_3 + uVar8;
  iStack12 = param_1 + 0x50;
  iStack14 = param_2 + -0x1;
  uStack18 = param_4;
  if (DAT_1050_61d2 != 0x0) {
    UVar5 = pass1_1000_455a(local_16,param_8);
    if (UVar5 != 0x0) {
      iStack26 += -0xe10;
    }
  }
  return iStack26;
}



fn pass1_1000_472c(param_1: u32,char param_2) -> *mut u8

{
  char *pcVar1;
  let uVar2: u16;
  char *pcVar3;
  char *pcVar4;
  let uVar5: u16;
  let bVar6: bool;
  
  uVar5 = (param_1 >> 0x10);
  pcVar3 = param_1;
  bVar6 = true;
  uVar2 = 0xffff;
  pcVar4 = pcVar3;
  do {
    if (uVar2 == 0x0) break;
    uVar2 -= 0x1;
    pcVar1 = pcVar4;
    pcVar4 = pcVar4 + 0x1;
    bVar6 = *pcVar1 == '\0';
  } while (!bVar6);
  uVar2 = ~uVar2;
  do {
    if (uVar2 == 0x0) break;
    uVar2 -= 0x1;
    pcVar1 = pcVar3;
    pcVar3 = pcVar3 + 0x1;
    bVar6 = param_2 == *pcVar1;
  } while (!bVar6);
  if (!bVar6) {
    if (param_2 != '\0') {
      return 0x0;
    }
    pcVar3 = pcVar3 + 0x1;
  }
  return pcVar3 + -0x1;
}



i16  pass1_1000_475e(param_1: u32,param_2: u32)

{
  char *pcVar1;
  let cVar2: u8;
  let cVar3: u8;
  let bVar4: u8;
  astruct_235 *bVar3;
  astruct_236 *bVar5;
  char *pcVar5;
  char *pcVar6;
  
  pcVar6 = param_2;
  pcVar5 = param_1;
  bVar5 = (astruct_236 *)(s_You_may_not_run_a_turn__The_game_1050_00df + 0x20);
  do {
    do {
      cVar3 = bVar5;
      if (cVar3 == '\0') goto LAB_1000_479d;
      pcVar1 = pcVar6;
      pcVar6 = pcVar6 + 0x1;
      cVar3 = *pcVar1;
      cVar2 = *pcVar5;
      bVar5 = (astruct_236 *)CONCAT11(cVar2,cVar3);
      pcVar5 = pcVar5 + 0x1;
    } while (cVar2 == cVar3);
    bVar4 = cVar3 + 0xbfU + (-((byte)(cVar3 + 0xbfU) < 0x1a) & 0x20U) + 0x41;
    bVar3._0_1_ = cVar2 + 0xbf;
    bVar5._0_1_ = (byte)bVar3 + (-((byte)bVar3 < 0x1a) & 0x20U) + 0x41;
    bVar5 = (astruct_236 *)CONCAT11(bVar4,(byte)bVar5);
  } while ((byte)bVar5 == bVar4);
  cVar3 = ((byte)bVar5 < bVar4) * -0x2 + '\x01';
LAB_1000_479d:
  return cVar3;
}



fn pass1_1000_47a4(param_1: u32,param_2: u32,param_3: u16) -> u16

{
  byte *pbVar1;
  let bVar2: u8;
  let puVar3: *mut u16;
  byte *pbVar4;
  let iVar5: i16;
  byte *pbVar6;
  let puVar7: *mut u16;
  let uVar8: u16;
  let local_22: [u16;0x10];
  
  puVar7 = local_22;
  for (iVar5 = 0x10; iVar5 != 0x0; iVar5 += -0x1) {
    puVar3 = puVar7;
    puVar7 = puVar7 + 0x1;
    *puVar3 = 0x0;
  }
  pbVar6 = (byte *)param_2;
  while( true ) {
    pbVar1 = pbVar6;
    pbVar6 = pbVar6 + 0x1;
    bVar2 = *pbVar1;
    if (bVar2 == 0x0) break;
    pbVar1 = (byte *)(local_22 + (bVar2 >> 0x3));
    *pbVar1 = *pbVar1 | '\x01' << (bVar2 & 0x7);
  }
  pbVar1 = (byte *)param_1;
  if (param_1 == 0x0) {
    pbVar1 = pbRam105061e4;
  }
  do {
    pbRam105061e4 = pbVar1;
    uVar8 = (pbRam105061e4 >> 0x10);
    pbVar6 = (byte *)(pbRam105061e4 + 0x1);
    bVar2 = *pbRam105061e4;
    if (bVar2 == 0x0) {
      return 0x0;
    }
    pbVar1 = (byte *)(pbRam105061e4 & 0xffff0000 | ZEXT24(pbVar6));
  } while (('\x01' << (bVar2 & 0x7) & *(byte *)(local_22 + (bVar2 >> 0x3))) !=
           0x0);
  do {
    pbVar4 = pbVar6;
    bVar2 = *pbVar4;
    if (bVar2 == 0x0) goto LAB_1000_483c;
    pbVar6 = pbVar4 + 0x1;
  } while (('\x01' << (bVar2 & 0x7) & *(byte *)(local_22 + (bVar2 >> 0x3))) ==
           0x0);
  *pbVar4 = 0x0;
  pbVar4 = pbVar4 + 0x1;
LAB_1000_483c:
  pbRam105061e4 = (byte *)(pbRam105061e4 & 0xffff0000 | ZEXT24(pbVar4));
  return pbRam105061e4;
}



fn pass1_1000_484c(param_1: u32,param_2: u32,param_3: u16) -> u16

{
  byte *pbVar1;
  byte *pbVar2;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  byte *pbVar6;
  byte *pbVar7;
  let iVar8: i16;
  let bVar9: bool;
  let bVar10: bool;
  
  if (param_3 == 0x0) {
    return 0x0;
  }
  do {
    iVar8 = (param_2 >> 0x10);
    pbVar7 = (byte *)param_2;
    iVar3 = (param_1 >> 0x10);
    pbVar6 = (byte *)param_1;
    uVar4 = ~pbVar7;
    uVar4 = ((param_3 - 0x1) - uVar4 & -(param_3 - 0x1 < uVar4)) + uVar4;
    uVar5 = ~pbVar6;
    uVar4 = (uVar4 - uVar5 & -(uVar4 < uVar5)) + uVar5 + 0x1;
    bVar9 = param_3 < uVar4;
    param_3 -= uVar4;
    bVar10 = param_3 == 0x0;
    do {
      if (uVar4 == 0x0) break;
      uVar4 -= 0x1;
      pbVar2 = pbVar7;
      pbVar7 = pbVar7 + 0x1;
      pbVar1 = pbVar6;
      pbVar6 = pbVar6 + 0x1;
      bVar9 = *pbVar1 < *pbVar2;
      bVar10 = *pbVar1 == *pbVar2;
    } while (bVar10);
    param_2 = param_2 & 0xffff0000 | ZEXT24(pbVar7);
    if (!bVar10) {
      return (0x1 - bVar9) - (bVar9 != 0x0);
    }
    if (param_3 == 0x0) {
      return uVar4;
    }
    if (pbVar6 == (byte *)0x0) {
      iVar3 += 0x6c;
    }
    param_1 = CONCAT22(iVar3,pbVar6);
    if (pbVar7 == (byte *)0x0) {
      param_2 = (iVar8 + 0x6c) << 0x10;
      param_1 = CONCAT22(iVar3,pbVar6);
    }
  } while( true );
}



fn pass1_1000_48a8(param_1: u32,param_2: u32,param_3: i16) -> u16

{
  let puVar1: *mut u16;
  let puVar2: *mut u16;
  let iVar3: i16;
  let uVar4: u16;
  let uVar5: u16;
  let puVar6: *mut u16;
  let puVar7: *mut u16;
  let iVar8: i16;
  
  if (param_3 != 0x0) {
    while( true ) {
      iVar3 = (param_2 >> 0x10);
      puVar6 = param_2;
      iVar8 = (param_1 >> 0x10);
      puVar7 = param_1;
      uVar4 = ~puVar7;
      uVar4 = ((param_3 - 0x1U) - uVar4 & -(param_3 - 0x1U < uVar4)) + uVar4;
      uVar5 = ~puVar6;
      uVar4 = (uVar4 - uVar5 & -(uVar4 < uVar5)) + uVar5 + 0x1;
      param_3 -= uVar4;
      for (uVar5 = uVar4 >> 0x1; uVar5 != 0x0; uVar5 -= 0x1) {
        puVar2 = puVar7;
        puVar7 = puVar7 + 0x1;
        puVar1 = puVar6;
        puVar6 = puVar6 + 0x1;
        *puVar2 = *puVar1;
      }
      for (uVar4 = ((uVar4 & 0x1) != 0x0); uVar4 != 0x0; uVar4 -= 0x1) {
        puVar2 = puVar7;
        puVar7 = (puVar7 + 0x1);
        puVar1 = puVar6;
        puVar6 = (puVar6 + 0x1);
        *puVar2 = *puVar1;
      }
      if (param_3 == 0x0) break;
      if (puVar6 == 0x0) {
        iVar3 += 0x6c;
      }
      param_1 = param_1 & 0xffff0000 | ZEXT24(puVar7);
      param_2 = CONCAT22(iVar3,puVar6);
      if (puVar7 == 0x0) {
        param_1 = (iVar8 + 0x6c) << 0x10;
        param_2 = CONCAT22(iVar3,puVar6);
      }
    }
  }
  return param_1;
}



fn pass1_1000_4906(astruct_20 *param_1,WNDCLASS16 *in_wnd_class,param_3: u16) -> u16

{
  let puVar1: *mut u16;
  let uVar2: u8;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let puVar7: *mut u16;
  let iVar8: i16;
  
  if (param_3 != 0x0) {
    iVar8 = (param_1 >> 0x10);
    uVar5 = -param_1;
    uVar6 = param_3;
    if (uVar5 != 0x0) {
      uVar6 = (uVar5 - param_3 & -(uVar5 < param_3)) + param_3;
      uVar5 = param_3 - uVar6;
    }
    uVar3 = in_wnd_class & 0xff | in_wnd_class << 0x8;
    puVar7 = param_1;
    for (uVar4 = uVar6 >> 0x1; uVar4 != 0x0; uVar4 -= 0x1) {
      puVar1 = puVar7;
      puVar7 = puVar7 + 0x1;
      *puVar1 = uVar3;
    }
    for (uVar6 = ((uVar6 & 0x1) != 0x0);
        uVar2 = (u8)(in_wnd_class & 0xff), uVar6 != 0x0; uVar6 -= 0x1) {
      puVar1 = puVar7;
      puVar7 = (puVar7 + 0x1);
      *puVar1 = uVar2;
    }
    if (uVar5 != 0x0) {
      for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
        puVar1 = puVar7;
        puVar7 = puVar7 + 0x1;
        *puVar1 = uVar3;
      }
      for (uVar6 = ((uVar5 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1) {
        puVar1 = puVar7;
        puVar7 = (puVar7 + 0x1);
        *puVar1 = uVar2;
      }
    }
  }
  return param_1;
}



i16  pass1_1000_49b2(param_1: u16)

{
  return (param_1 ^ param_1 >> 0xf) - (param_1 >> 0xf);
}



u16 
pass1_1000_49c6(param_1: u16,param_2: u16,param_3: u16,param_4: u16,param_5: u16,
               param_6: u16,uchar *param_7,param_8: i16)

{
  let uVar1: u16;
  let uVar2: u16;
  let uVar3: u16;
  let iVar4: i16;
  let lVar5: i32;
  let uStack20: u16;
  let uStack18: u16;
  let uStack14: u16;
  let uStack12: u16;
  let uStack10: i16;
  let uStack8: u16;
  let uStack6: u16;
  let local_4: u16;
  let iStack2: i16;
  
  iStack2 = param_8 + 0x1;
  local_4 = SUB42(&USHORT_1050_1050,0x0);
  uStack20 = param_3;
  uStack18 = param_4;
  lVar5 = pass1_1000_52be(param_5 - 0x1,-(param_5 == 0x0),param_6,0x0);
  uStack8 = (lVar5 + 0x8);
  uStack6 = ((lVar5 + 0x8) >> 0x10) * 0x100 + param_4;
  while( true ) {
    if (uStack6 < uStack18) {
      return 0x0;
    }
    if ((uStack6 <= uStack18) && (uStack8 < uStack20)) {
      return 0x0;
    }
    uStack14 = param_5 >> 0x1;
    if (uStack14 == 0x0) {
      if ((param_5 != 0x0) && (iVar4 = (*(code *)param_7)(), iVar4 == 0x0)) {
        return uStack20;
      }
      return 0x0;
    }
    uVar1 = uStack14;
    if ((param_5 & 0x1) == 0x0) {
      uVar1 = uStack14 - 0x1;
    }
    uVar2 = (uVar1 * param_6);
    uVar3 = uVar2 + uStack20;
    iStack10 = ((uVar1 * param_6 >> 0x10) +
               CARRY2(uVar2,uStack20)) * 0x100 + uStack18;
    uStack12 = uVar3;
    iVar4 = (*(code *)param_7)();
    if (iVar4 == 0x0) break;
    if (iVar4 < 0x0) {
      uStack8 = -param_6 + uStack12;
      uStack6 = (CARRY2(-param_6,uStack12) - (param_6 != 0x0)) * 0x100 +
                iStack10;
      uVar1 = param_5 & 0x1;
      param_5 = uStack14;
      if (uVar1 == 0x0) {
        param_5 = uStack14 - 0x1;
      }
    }
    else {
      uStack20 = param_6 + uStack12;
      uStack18 = CARRY2(param_6,uStack12) * 0x100 + iStack10;
      param_5 = uStack14;
    }
  }
  return uVar3;
}



void 
pass1_1000_4aea(param_1: u16,param_2: u16,param_3: i16,param_4: u16,uchar *param_5,
               param_6: i16,param_7: i16,param_8: u16,param_9: u16,param_10: u16)

{
  let puVar1: *mut u16;
  code **ppcVar2;
  let lVar3: i32;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let iVar8: i16;
  let uVar9: u16;
  let uVar10: u16;
  astruct_171 *puVar11;
  let uVar11: u16;
  let uVar12: u16;
  let bVar13: bool;
  let uVar14: u16;
  let uVar15: u16;
  let uVar16: u16;
  let uVar17: u16;
  let uVar18: u16;
  let uVar19: u16;
  let uStack4: u16;
  let iStack2: i16;
  
  iStack2 = param_6 + 0x1;
  uStack4 = SUB42(&USHORT_1050_1050,0x0);
  uVar12 = SUB42(&USHORT_1050_1050,0x0);
  if ((param_4 != 0x0) && (param_3 != 0x0)) {
    for (iVar8 = param_3 + -0x1; iVar8 != 0x0; iVar8 += -0x1) {
      iVar4 = (*(code *)param_5)(param_9);
      if (iVar4 < 0x0) {
        uVar5 = param_3 - 0x1;
        iVar8 = 0x0;
        do {
          uVar5 >>= 0x1;
          iVar8 += -0x1;
        } while (iVar8 != 0x0 && uVar5 != 0x0);
        if (((-iVar8 * 0x8 >> 0x10) != 0x0) ||
           (uVar5 = pass1_1000_3bac(), uVar5 < (-iVar8 * 0x8))) {
          exit_1000_25f2(0x4b7b,param_9,param_7,-0x4,param_8,param_9,param_10);
          return;
        }
        puVar11 = (astruct_171 *)&stack0xfff6;
        lVar3 = (param_3 - 0x1) * param_4;
        uVar6 = lVar3;
        uVar5 = uVar6 + param_1;
        uVar6 = ((lVar3 >> 0x10) + CARRY2(uVar6,param_1)) * 0x100 +
                param_2;
LAB_1000_4b7d:
        if (puVar11 <= (astruct_171 *)&stack0xffee) {
          return;
        }
LAB_1000_4b81:
        if ((param_2 < uVar6) || ((param_2 <= uVar6 && (param_1 < uVar5)))) {
          puVar1 = &puVar11->field_0x14;
          uVar10 = uVar5 + *puVar1;
          uVar9 = uVar6 + (-CARRY2(uVar5,*puVar1) & 0x6c);
          uVar14 = param_1;
          uVar15 = param_2;
          uVar18 = uVar5;
          uVar19 = uVar6;
          uVar7 = param_1;
          uVar11 = param_2;
LAB_1000_4bbc:
          do {
            puVar1 = &puVar11->field_0x14;
            bVar13 = CARRY2(param_1,*puVar1);
            param_1 += *puVar1;
            param_2 += -bVar13 & 0x6c;
            if ((param_1 != uVar18) || (param_2 != uVar19)) {
              ppcVar2 = (code **)&puVar11->field_0x16;
              iVar8 = (**ppcVar2)(param_9,param_1,param_2,uVar7,uVar11);
              if (iVar8 < 0x1) {
                if (iVar8 != 0x0) {
                  uVar14 = param_1;
                  uVar15 = param_2;
                }
                goto LAB_1000_4bbc;
              }
            }
            do {
              uVar17 = uVar6;
              uVar16 = uVar5;
              puVar1 = &puVar11->field_0x14;
              bVar13 = uVar10 < *puVar1;
              uVar10 -= *puVar1;
              uVar9 -= -bVar13 & 0x6c;
              ppcVar2 = (code **)&puVar11->field_0x16;
              iVar8 = (**ppcVar2)(param_9,uVar7,uVar11,uVar10,uVar9);
              if (0x0 < iVar8) break;
              uVar5 = uVar10;
              uVar6 = uVar9;
            } while (((iVar8 != 0x0) || (uVar5 = uVar16, uVar6 = uVar17, uVar10 != uVar7))
                    || (uVar9 != uVar11));
            if ((uVar9 < param_2) || ((uVar9 <= param_2 && (uVar10 <= param_1))))
            goto LAB_1000_4c58;
            pass1_1000_4ceb(puVar11->field_0x14,param_1,uVar10,uVar9);
            uVar14 = param_1;
            uVar15 = param_2;
            uVar5 = uVar10;
            uVar6 = uVar9;
          } while( true );
        }
        goto LAB_1000_4b7d;
      }
    }
  }
  return;
LAB_1000_4c58:
  param_1 = uVar7;
  param_2 = uVar11;
  pass1_1000_4ceb(puVar11->field_0x14,uVar7,uVar10,uVar9);
  uVar11 = ((uVar19 - (-(uVar18 < uVar16) & 0x6c)) - uVar17) +
           (-CARRY2(uVar18 - uVar16,param_1) & 0x6c) + param_2;
  uVar7 = -((uVar18 - uVar16) + param_1 < uVar14) & 0x6c;
  uVar5 = uVar14;
  uVar6 = uVar15;
  if ((uVar7 <= uVar11) && (uVar15 <= uVar11 - uVar7)) {
    uVar5 = uVar18;
    uVar6 = uVar19;
    param_1 = uVar16;
    param_2 = uVar17;
  }
  goto LAB_1000_4b81;
}



fn pass1_1000_4ceb(param_1: u16,param_2: i16,param_3: i16,param_4: u16)
{
  let puVar1: *mut u8;
  let puVar2: *mut u16;
  let uVar3: u8;
  let uVar4: u16;
  
  if ((param_1 & 0x1) != 0x0) {
    param_1 -= 0x1;
    puVar1 = (param_1 + param_3);
    uVar3 = *puVar1;
    *puVar1 = *(param_1 + param_2);
    *(param_1 + param_2) = uVar3;
    if (param_1 == 0x0) {
      return;
    }
  }
  do {
    param_1 -= 0x2;
    puVar2 = (param_1 + param_3);
    uVar4 = *puVar2;
    *puVar2 = (param_1 + param_2);
    (param_1 + param_2) = uVar4;
  } while (param_1 != 0x0);
  return;
}



fn pass1_1000_4d0c(param_1: u16)
{
  DAT_1050_61e8 = param_1;
  PTR_LOOP_1050_61ea = 0x0;
  return;
}



fn pass1_1000_4d24(void) -> u16

{
  let lVar1: i32;
  
  lVar1 = pass1_1000_52be(DAT_1050_61e8,PTR_LOOP_1050_61ea,
                          (s_TPPOPMENU_1050_43fa + 0x3),0x3);
  PTR_LOOP_1050_61ea = ((lVar1 + 0x269ec3) >> 0x10);
  DAT_1050_61e8 = (lVar1 + 0x269ec3);
  return PTR_LOOP_1050_61ea & 0x7fff;
}


fn pass1_1000_4f1a(param_1: i16,param_2: u16,param_3: u16) -> *mut i16

{
  let piVar1: *mut i16;
  char *pcVar2;
  LPCSTR str;
  let piVar3: *mut i16;
  let piVar4: *mut i16;
  char *pcVar5;
  let iVar6: i16;
  let iVar7: i16;
  
  iVar7 = 0x19;
  iVar6 = 0x19;
  pass1_1000_25a8(param_2,param_3);
  pass1_1000_2913(iVar6,param_2,param_3);
  str = poss_str_op_1000_28dc(iVar7);
  if (str != 0x0) {
    iVar6 = 0x9;
    if (*str == 'M') {
      iVar6 = 0xf;
    }
    str = str + iVar6;
    iVar6 = 0x22;
    pcVar5 = str;
    do {
      if (iVar6 == 0x0) break;
      iVar6 += -0x1;
      pcVar2 = pcVar5;
      pcVar5 = pcVar5 + 0x1;
    } while (*pcVar2 != '\r');
    pcVar5[-0x1] = '\0';
  }
  FatalAppExit16(param_3,str);
  FatalExit();
  piVar4 = (i16 *)&PTR_LOOP_1050_63fe;
  do {
    piVar1 = piVar4;
    piVar4 = piVar4 + 0x1;
    iVar6 = *piVar1;
    piVar3 = piVar4;
    if ((iVar6 == param_1) || (piVar3 = (i16 *)(iVar6 + 0x1), piVar3 == (i16 *)0x0)) {
      return piVar3;
    }
    iVar6 = -0x1;
    do {
      if (iVar6 == 0x0) break;
      iVar6 += -0x1;
      piVar1 = piVar4;
      piVar4 = (i16 *)(piVar4 + 0x1);
    } while (*piVar1 != '\0');
  } while( true );
}


fn pass1_1000_4f2e(param_1: u16) -> u16

{
  code *pcVar1;
  let uVar2: u16;
  let uVar3: u8;
  
  uVar2 = 0x3b50;
  uVar3 = 0x0;
  if (true) {
    pcVar1 = (code *)swi(0x21);
    uVar2 = (*pcVar1)(&USHORT_1050_1050,param_1 + 0x1);
  }
  else {
    DOS3Call((CONTEXT *)&PTR_LOOP_1050_1000);
  }
  if (!(bool)uVar3) {
    return 0x0;
  }
  pass1_1000_29b5(uVar2);
  return 0xffff;
}

fn pass1_1000_5008(param_1: u16,param_2: u16,param_3: u16,param_4: i16)
{
  let unaff_CS: u16;
  let unaff_SS: u16;
  let iStack2: i16;
  
  iStack2 = param_4 + 0x1;
  pass1_1000_5026(0x0,param_1,param_2,param_3,&iStack2,unaff_CS,unaff_SS);
  return;
}



fn pass1_1000_5224(param_1: u16,param_2: u16,param_3: u16,param_4: u16) -> u32

{
  let uVar1: u32;
  let lVar2: i32;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let bVar10: bool;
  let cVar11: u8;
  let uVar9: u16;
  
  cVar11 = param_2 < 0x0;
  if ((bool)cVar11) {
    bVar10 = param_1 != 0x0;
    param_1 = -param_1;
    param_2 = -bVar10 - param_2;
  }
  if (param_4 < 0x0) {
    cVar11 += '\x01';
    bVar10 = param_3 != 0x0;
    param_3 = -param_3;
    param_4 = -bVar10 - param_4;
  }
  uVar3 = param_1;
  uVar5 = param_3;
  uVar6 = param_2;
  uVar9 = param_4;
  if (param_4 == 0x0) {
    uVar3 = param_2 / param_3;
    iVar4 = ((param_2 % param_3 << 0x10 | param_1) /
                 param_3);
  }
  else {
    do {
      uVar8 = uVar9 >> 0x1;
      uVar5 = uVar5 >> 0x1 | ((uVar9 & 0x1) != 0x0) << 0xf;
      uVar7 = uVar6 >> 0x1;
      uVar3 = uVar3 >> 0x1 | ((uVar6 & 0x1) != 0x0) << 0xf;
      uVar6 = uVar7;
      uVar9 = uVar8;
    } while (uVar8 != 0x0);
    uVar1 = CONCAT22(uVar7,uVar3) / uVar5;
    iVar4 = uVar1;
    lVar2 = param_3 * (uVar1 & 0xffff);
    uVar3 = (lVar2 >> 0x10);
    uVar5 = uVar3 + iVar4 * param_4;
    if (((CARRY2(uVar3,iVar4 * param_4)) || (param_2 < uVar5)) ||
       ((param_2 <= uVar5 && (param_1 < lVar2)))) {
      iVar4 += -0x1;
    }
    uVar3 = 0x0;
  }
  if (cVar11 == '\x01') {
    bVar10 = iVar4 != 0x0;
    iVar4 = -iVar4;
    uVar3 = -bVar10 - uVar3;
  }
  return CONCAT22(uVar3,iVar4);
}



fn pass1_1000_52be(param_1: u16,param_2: u16,param_3: u16,param_4: u16) -> u32

{
  if ((param_4 | param_2) == 0x0) {
    return param_1 * param_3;
  }
  return param_1 * param_3 & 0xffff |
         ((param_1 * param_3 >> 0x10) +
                param_2 * param_3 + param_1 * param_4) << 0x10;
}



fn pass1_1000_52f0(param_1: u16,param_2: u16,param_3: u16,param_4: u16) -> u32

{
  let uVar1: u32;
  let lVar2: i32;
  let uVar3: u16;
  let uVar4: u16;
  let iVar5: i16;
  let iVar6: i16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let uVar11: u16;
  let bVar12: bool;
  let bVar13: bool;
  
  bVar13 = param_2 < 0x0;
  if (bVar13) {
    bVar12 = param_1 != 0x0;
    param_1 = -param_1;
    param_2 = -bVar12 - param_2;
  }
  uVar11 = bVar13;
  if (param_4 < 0x0) {
    bVar13 = param_3 != 0x0;
    param_3 = -param_3;
    param_4 = -bVar13 - param_4;
  }
  uVar3 = param_1;
  uVar4 = param_3;
  uVar8 = param_2;
  uVar9 = param_4;
  if (param_4 == 0x0) {
    iVar5 = ((param_2 % param_3 << 0x10 | param_1) %
                 param_3);
    iVar6 = 0x0;
    if ((uVar11 - 0x1) < 0x0) goto LAB_1000_538a;
  }
  else {
    do {
      uVar10 = uVar9 >> 0x1;
      uVar4 = uVar4 >> 0x1 | ((uVar9 & 0x1) != 0x0) << 0xf;
      uVar7 = uVar8 >> 0x1;
      uVar3 = uVar3 >> 0x1 | ((uVar8 & 0x1) != 0x0) << 0xf;
      uVar8 = uVar7;
      uVar9 = uVar10;
    } while (uVar10 != 0x0);
    uVar1 = CONCAT22(uVar7,uVar3) / uVar4;
    uVar3 = uVar1 * param_4;
    lVar2 = (uVar1 & 0xffff) * param_3;
    uVar8 = (lVar2 >> 0x10);
    uVar4 = lVar2;
    uVar9 = uVar8 + uVar3;
    if (((CARRY2(uVar8,uVar3)) || (param_2 < uVar9)) ||
       ((param_2 <= uVar9 && (param_1 < uVar4)))) {
      bVar13 = uVar4 < param_3;
      uVar4 -= param_3;
      uVar9 = (uVar9 - param_4) - bVar13;
    }
    iVar5 = uVar4 - param_1;
    iVar6 = (uVar9 - param_2) - (uVar4 < param_1);
    if (-0x1 < (uVar11 - 0x1)) goto LAB_1000_538a;
  }
  bVar13 = iVar5 != 0x0;
  iVar5 = -iVar5;
  iVar6 = -bVar13 - iVar6;
LAB_1000_538a:
  return CONCAT22(iVar6,iVar5);
}



fn pass1_1000_5390(param_1: u16,param_2: u16,param_3: u16,param_4: u16) -> u32

{
  let uVar1: u32;
  let lVar2: i32;
  let uVar3: u16;
  let iVar4: i16;
  let uVar5: u16;
  let uVar6: u16;
  let uVar7: u16;
  let uVar8: u16;
  let uVar9: u16;
  
  uVar3 = param_1;
  uVar8 = param_4;
  uVar6 = param_2;
  uVar9 = param_3;
  if (param_4 == 0x0) {
    uVar3 = param_2 / param_3;
    iVar4 = ((param_2 % param_3 << 0x10 | param_1) /
                 param_3);
  }
  else {
    do {
      uVar5 = uVar8 >> 0x1;
      uVar9 = uVar9 >> 0x1 | ((uVar8 & 0x1) != 0x0) << 0xf;
      uVar7 = uVar6 >> 0x1;
      uVar3 = uVar3 >> 0x1 | ((uVar6 & 0x1) != 0x0) << 0xf;
      uVar8 = uVar5;
      uVar6 = uVar7;
    } while (uVar5 != 0x0);
    uVar1 = CONCAT22(uVar7,uVar3) / uVar9;
    iVar4 = uVar1;
    lVar2 = param_3 * (uVar1 & 0xffff);
    uVar3 = (lVar2 >> 0x10);
    uVar8 = uVar3 + iVar4 * param_4;
    if (((CARRY2(uVar3,iVar4 * param_4)) || (param_2 < uVar8)) ||
       ((param_2 <= uVar8 && (param_1 < lVar2)))) {
      iVar4 += -0x1;
    }
    uVar3 = 0x0;
  }
  return CONCAT22(uVar3,iVar4);
}



fn pass1_1000_53f0(param_1: u16,param_2: u16,param_3: u16,param_4: u16) -> u32

{
  let uVar1: u32;
  let lVar2: i32;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let iVar6: i16;
  let iVar7: i16;
  let uVar8: u16;
  let uVar9: u16;
  let uVar10: u16;
  let bVar11: bool;
  
  uVar3 = param_1;
  uVar4 = param_4;
  uVar9 = param_2;
  uVar10 = param_3;
  if (param_4 == 0x0) {
    iVar6 = ((param_2 % param_3 << 0x10 | param_1) %
                 param_3);
    iVar7 = 0x0;
  }
  else {
    do {
      uVar5 = uVar4 >> 0x1;
      uVar10 = uVar10 >> 0x1 | ((uVar4 & 0x1) != 0x0) << 0xf;
      uVar8 = uVar9 >> 0x1;
      uVar3 = uVar3 >> 0x1 | ((uVar9 & 0x1) != 0x0) << 0xf;
      uVar4 = uVar5;
      uVar9 = uVar8;
    } while (uVar5 != 0x0);
    uVar1 = CONCAT22(uVar8,uVar3) / uVar10;
    uVar3 = uVar1 * param_4;
    lVar2 = (uVar1 & 0xffff) * param_3;
    uVar9 = (lVar2 >> 0x10);
    uVar4 = lVar2;
    uVar10 = uVar9 + uVar3;
    if (((CARRY2(uVar9,uVar3)) || (param_2 < uVar10)) ||
       ((param_2 <= uVar10 && (param_1 < uVar4)))) {
      bVar11 = uVar4 < param_3;
      uVar4 -= param_3;
      uVar10 = (uVar10 - param_4) - bVar11;
    }
    iVar6 = -(uVar4 - param_1);
    iVar7 = -(uVar4 - param_1 != 0x0) -
            ((uVar10 - param_2) - (uVar4 < param_1));
  }
  return CONCAT22(iVar7,iVar6);
}



i16  pass1_1000_545a(param_1: u32,param_2: u32)

{
  byte *pbVar1;
  let bVar2: u8;
  let bVar3: u8;
  let bVar4: u8;
  byte *pbVar5;
  byte *pbVar6;
  
  pbVar6 = (byte *)param_2;
  pbVar5 = (byte *)param_1;
  bVar4 = 0xff;
  do {
    do {
      if (bVar4 == 0x0) goto LAB_1000_5499;
      pbVar1 = pbVar6;
      pbVar6 = pbVar6 + 0x1;
      bVar4 = *pbVar1;
      bVar3 = *pbVar5;
      pbVar5 = pbVar5 + 0x1;
    } while (bVar3 == bVar4);
    bVar2 = bVar4 + 0xbf + (-((byte)(bVar4 + 0xbf) < 0x1a) & 0x20U) + 0x41;
    bVar3 += 0xbf;
    bVar4 = bVar3 + (-(bVar3 < 0x1a) & 0x20U) + 0x41;
  } while (bVar4 == bVar2);
  bVar4 = (bVar4 < bVar2) * -0x2 + 0x1;
LAB_1000_5499:
  return bVar4;
}



fn pass1_1000_54a0(param_1: u32,param_2: u16,param_3: u16) -> u16

{
  let puVar1: *mut u16;
  let uVar2: u8;
  let uVar3: u16;
  let uVar4: u16;
  let uVar5: u16;
  let uVar6: u16;
  let puVar7: *mut u16;
  let iVar8: i16;
  
  if (param_3 != 0x0) {
    iVar8 = (param_1 >> 0x10);
    uVar5 = -param_1;
    uVar6 = param_3;
    if (uVar5 != 0x0) {
      uVar6 = (uVar5 - param_3 & -(uVar5 < param_3)) + param_3;
      uVar5 = param_3 - uVar6;
    }
    uVar3 = param_2 & 0xff | param_2 << 0x8;
    puVar7 = param_1;
    for (uVar4 = uVar6 >> 0x1; uVar4 != 0x0; uVar4 -= 0x1) {
      puVar1 = puVar7;
      puVar7 = puVar7 + 0x1;
      *puVar1 = uVar3;
    }
    for (uVar6 = ((uVar6 & 0x1) != 0x0); uVar2 = (u8)(param_2 & 0xff),
        uVar6 != 0x0; uVar6 -= 0x1) {
      puVar1 = puVar7;
      puVar7 = (puVar7 + 0x1);
      *puVar1 = uVar2;
    }
    if (uVar5 != 0x0) {
      for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
        puVar1 = puVar7;
        puVar7 = puVar7 + 0x1;
        *puVar1 = uVar3;
      }
      for (uVar6 = ((uVar5 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1) {
        puVar1 = puVar7;
        puVar7 = (puVar7 + 0x1);
        *puVar1 = uVar2;
      }
    }
  }
  return param_1;
}



void 
pass1_1000_54e8(uchar *param_1,param_2: u16,param_3: i16,param_4: i16,param_5: i16,
               param_6: u16)

{
  let iVar1: i16;
  let iVar2: i16;
  let uStack14: u16;
  let uStack10: i16;
  let uStack8: u16;
  
  iVar2 = param_5 + param_4 * param_3;
  iVar1 = param_3;
  while (iVar1 += -0x1, -0x1 < iVar1) {
    iVar2 -= param_4;
    uStack8 = param_6;
    uStack14 = 0x5506;
    iStack10 = iVar2;
    (*(code *)param_1)();
  }
  return;
}



fn pass1_1000_5512(uchar *param_1,param_2: u16,param_3: i16,param_4: i16,param_5: u16)
{
  let bVar1: bool;
  let uStack4: u16;
  
  pass1_1000_52be(param_3,param_4,param_5,0x0);
  while( true ) {
    bVar1 = param_3 == 0x0;
    param_3 += -0x1;
    param_4 -= bVar1;
    if (param_4 < 0x0) break;
    uStack4 = param_2;
    (*(code *)param_1)();
  }
  return;
}



void 
pass1_1000_5586(uchar *param_1,param_2: u16,param_3: i16,param_4: i16,param_5: i16,
               param_6: u16)

{
  let iVar1: i16;
  let iVar2: i16;
  let uStack14: u16;
  let uStack10: i16;
  let uStack8: u16;
  
  iVar1 = param_5;
  iVar2 = param_3;
  while (iVar2 += -0x1, -0x1 < iVar2) {
    uStack8 = param_6;
    uStack14 = 0x559b;
    iStack10 = iVar1;
    (*(code *)param_1)();
    iVar1 += param_4;
  }
  return;
}


fn pass1_1000_55b1(param_1: i16,param_2: u16,param_3: u16) -> *mut i16

{
  let piVar2: *mut i16;
  char *pcVar3;
  LPCSTR str;
  let piVar4: *mut i16;
  let piVar5: *mut i16;
  char *pcVar6;
  let iVar7: i16;
  let iVar8: i16;
  char *piVar1;
  
  iVar8 = 0x14;
  iVar7 = 0x14;
  pass1_1000_25a8(param_2,param_3);
  pass1_1000_2913(iVar7,param_2,param_3);
  str = poss_str_op_1000_28dc(iVar8);
  if (str != 0x0) {
    iVar7 = 0x9;
    if (*str == 'M') {
      iVar7 = 0xf;
    }
    str = str + iVar7;
    iVar7 = 0x22;
    pcVar6 = str;
    do {
      if (iVar7 == 0x0) break;
      iVar7 += -0x1;
      pcVar3 = pcVar6;
      pcVar6 = pcVar6 + 0x1;
    } while (*pcVar3 != '\r');
    pcVar6[-0x1] = '\0';
  }
  FatalAppExit16(param_3,str);
  FatalExit();
  piVar5 = (i16 *)&PTR_LOOP_1050_63fe;
  do {
    piVar2 = piVar5;
    piVar5 = piVar5 + 0x1;
    iVar7 = *piVar2;
    piVar4 = piVar5;
    if ((iVar7 == param_1) || (piVar4 = (i16 *)(iVar7 + 0x1), piVar4 == (i16 *)0x0)) {
      return piVar4;
    }
    iVar7 = -0x1;
    do {
      if (iVar7 == 0x0) break;
      iVar7 += -0x1;
      piVar1 = piVar5;
      piVar5 = (i16 *)(piVar5 + 0x1);
    } while (*piVar1 != '\0');
  } while( true );
}
