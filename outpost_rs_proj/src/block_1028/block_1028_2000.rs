
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_2042(param_1: *mut astruct_15)

{
  astruct_21 *paVar1;
  let mut uVar2: u16;
  code **ppcVar3;
  let mut uVar4: u32;
  astruct_15 *iVar5;
  astruct_15 *uVar5;
  let mut uVar6: u32;

  uVar5 = (astruct_15 *)(param_1 >> 0x10);
  iVar5 = (astruct_15 *)param_1;
  param_1.field0_0x0 = 0x2572;
  iVar5.field1_0x2 = 0x1028;
  uVar4 = &iVar5.field24_0x20;
  (uVar4 + 0xa) = 0x1;
  paVar1 = iVar5.field24_0x20;
  uVar2 = iVar5.field25_0x22;
  if ((uVar2 | paVar1) != 0x0) {
    ppcVar3 = (code **)paVar1;
    (**ppcVar3)();
  }
  if ((_PTR_LOOP_1050_65e2 != 0x0) && (_PTR_LOOP_1050_5a64 != 0x0)) {
    uVar6 = pass1_1028_b58e(param_1);
    pass1_1038_79f2(_PTR_LOOP_1050_5a64,uVar6,&DAT_1050_1050);
  }
  pass1_1028_b418(&param_1.field0_0x0);
  return;
}



u16 pass1_1028_20b0(void)

{
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_20b6(param_1: *mut astruct_15)

{
  let mut uVar1: u16;
  u8 *puVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 auStack22 [0x2];
  let mut iStack20: i16;
  let mut iStack18: i16;
  let mut uStack16: u32;
  let mut uStack12: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  pass1_1028_be9e(param_1);
  uVar4 = (param_1 >> 0x10);
  uVar3 = param_1;
  if ((uVar3 + 0x12) == 0x5) {
    uStack6 = pass1_1028_bb24((astruct_15 *)(param_1 & 0xffff | uVar4 << 0x10));
    uStack10 = pass1_1028_b58e(param_1);
    puVar2 = (uStack10 >> 0x10);
    uStack16 = (uStack10 + 0xc);
    uStack12 = (uStack10 + 0x10);
    pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&uStack16),(u16 *)CONCAT22(0x1050,auStack22),
                    (u16 *)CONCAT22(0x1050,&iStack20),(u16 *)CONCAT22(0x1050,&iStack18));
    uStack16 = uStack16 & 0xffff | (iStack20 - 0x1) << 0x10;
    uVar1 = pass1_1028_21ba(&uStack16,puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,&uStack16),uStack6);
    if (uVar1 == 0x0) {
      uStack16 = uStack16 & 0xffff | (iStack20 + 0x1) << 0x10;
      uVar1 = pass1_1028_21ba(&uStack16,puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,&uStack16),uStack6);
      if (uVar1 == 0x0) {
        uStack16 = CONCAT22(iStack20,iStack18 + -0x1);
        uVar1 = pass1_1028_21ba(&uStack16,puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,&uStack16),uStack6);
        if (uVar1 == 0x0) {
          uStack16 = uStack16 & 0xffff0000 | (iStack18 + 0x1);
          uVar1 = pass1_1028_21ba(&uStack16,puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,&uStack16),uStack6)
          ;
          if (uVar1 == 0x0) {
            return;
          }
        }
      }
    }
    pass1_1038_79b2(uVar1,puVar2,_PTR_LOOP_1050_5a64,uStack10);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1028_21ba(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,param_5: *mut u16,i32 param_6)

{
  let mut uVar1: u16;
  let mut uVar2: u32;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_5,param_6);
  uVar1 = param_2 | param_1;
  if (uVar1 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    if ((uVar1 | param_1) != 0x0) {
      uVar2 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar1,param_1),uVar1 | param_1,uVar1);
      if ((uVar2 != 0x0) && ((uVar2 + 0xc) == 0x40)) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16
pass1_1028_2220(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 ,mut param_5: i16,param_6: *mut u16,i32 param_7
               )

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u32;

  pass1_1030_627e(param_1,param_2,_PTR_LOOP_1050_5740,param_6,param_7);
  uVar2 = param_2 | param_1;
  if (uVar2 != 0x0) {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
    if ((uVar2 | param_1) != 0x0) {
      uVar3 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar2,param_1),uVar2 | param_1,uVar2);
      if ((uVar3 != 0x0) && ((iVar1 = (uVar3 + 0xc), iVar1 == 0x40 || (iVar1 == param_5)))) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



// WARNING: Could not reconcile some variable overlaps

i16 pass1_1028_2290(mut param_1: u32,u32 *param_2,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,i32 param_6)

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  u8 local_e [0x2];
  let mut local_c: i16;
  let mut local_a: i16;
  let mut local_8: u32;
  let mut uStack4: u16;

  local_8 = *param_2;
  uStack4 = (param_2 + 0x1);
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,local_e),
                  (u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_a));
  local_8 = local_8 & 0xffff | (local_c - 0x1) << 0x10;
  uVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  iVar1 = pass1_1028_2220(&local_8,param_3,uVar2,uVar3,0x16,(u16 *)CONCAT22(0x1050,&local_8),param_6);
  if (iVar1 == 0x0) {
    local_8 = local_8 & 0xffff | (local_c + 0x1) << 0x10;
    iVar1 = pass1_1028_2220(&local_8,param_3,uVar2,uVar3,0x16,(u16 *)CONCAT22(0x1050,&local_8),param_6);
    if (iVar1 == 0x0) {
      local_8 = local_a + -0x1;
      local_8 = local_c;
      iVar1 = pass1_1028_2220(&local_8,param_3,uVar2,uVar3,0x17,(u16 *)CONCAT22(0x1050,&local_8),param_6);
      if (iVar1 == 0x0) {
        local_8 = CONCAT22(local_8,local_a + 0x1);
        iVar1 = pass1_1028_2220(&local_8,param_3,uVar2,uVar3,0x17,(u16 *)CONCAT22(0x1050,&local_8),param_6);
        if (iVar1 == 0x0) {
          return iVar1;
        }
      }
    }
  }
  return 0x1;
}



u16 pass1_1028_236a(param_1: *mut astruct_15)

{
  let mut uVar1: u16;
  astruct_398 *paVar2;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;

  uVar1 = (param_1 >> 0x10);
  if ((*(param_1 + 0x1a) & 0x2) == 0x0) {
    uVar4 = 0x0;
    uVar5 = 0x23;
    uVar3 = 0x1;
    paVar2 = (astruct_398 *)pass1_1028_b58e((astruct_15 *)(param_1 & 0xffff | uVar1 << 0x10));
    pass1_1030_7d7c(paVar2,(paVar2 >> 0x10),paVar2,uVar3,CONCAT22(uVar5,uVar4));
    pass1_1028_bdac(param_1,0x6);
    return 0x0;
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_23a8(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16,mut param_4: u32,i32 param_5)

{
  let mut iVar1: i16;
  u8 *puVar2;
  let mut uVar3: u32;
  let mut uVar4: u16;
  let mut uVar5: u32;
  u8 local_4 [0x2];

  uVar5 = pass1_1030_bcae(local_4,&DAT_1050_1050);
  uVar4 = (uVar5 >> 0x10);
  iVar1 = uVar5;
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,param_4);
  uVar3 = (iVar1 + 0x10);
  pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar3);
  puVar2 = local_4;
  pass1_1030_bcde(puVar2,&DAT_1050_1050,uVar3 & 0xffff | uVar4 << 0x10,param_3,param_5);
  if (puVar2 < 0x0) {
    PTR_LOOP_1050_50ca = 0x6af;
    return;
  }
  return;
}



BOOL16 pass1_1028_2418(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u32;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut uVar4: u32;
  HFILE16 in_stack_0000ffce;
  u16 local_1c [0x6];
  let mut uStack16: u16;
  let mut iStack14: i16;
  let mut uStack12: u16;
  u8 local_a [0x8];

  BVar2 = write_to_file_1028_b5ec((astruct_731 *)param_1,param_2);
  if (BVar2 != 0x0) {
    uVar3 = (param_1 >> 0x10);
    pass1_1008_5784(CONCAT22(0x1050,local_a),(param_1 + 0x20));
    uVar1 = (param_1 + 0x20);
    local_1c[0] = (uVar1 + 0x8);
    uStack16 = local_1c[0];
    BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_1c),0x2,in_stack_0000ffce);
    if (BVar2 == 0x0) {
      u16_1050_0310 = 0x6d0;
      return BVar2;
    }
    while( true ) {
      uVar4 = pass1_1008_5b12(CONCAT22(0x1050,local_a));
      iStack14 = uVar4;
      if (uVar4 == 0x0) break;
      pass1_1038_75ca(iStack14,uVar4,param_2);
      uStack12 = (uVar4 >> 0x10);
      if ((BOOL16)uVar4 == 0x0) {
        return (BOOL16)uVar4;
      }
    }
    BVar2 = 0x1;
  }
  return BVar2;
}



BOOL16 file_1028_24a2(mut param_1: i16,u8 *param_2,param_3: *mut astruct_373,HFILE16 *param_4)

{
  code **ppcVar1;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut in_register_0000000a: u16;
  let mut paVar5: *mut Struct57;
  HFILE16 *pHVar7;
  let mut uStack6: u16;
  let mut local_4: u16;
  let mut paVar6: *mut Struct57;

  paVar5 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  file_1028_b81a(param_1,param_2,param_3,param_4);
  if (param_1 == 0x0) {
    return 0x0;
  }
  if (0x1 < u16_1050_0312) {
    BVar2 = read_file_1008_7dee(param_4,CONCAT22(0x1050,&local_4),0x2);
    if (BVar2 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return 0x0;
    }
    for (uStack6 = 0x0; uStack6 < local_4; uStack6 += 0x1) {
      uVar3 = local_4;
      pHVar7 = param_4;
      mem_op_1000_179c(0x2a,paVar5);
      uVar4 = paVar5 | uVar3;
      paVar6 = (astruct_57 *)(paVar5 & 0xffff0000 | uVar4);
      if (uVar4 == 0x0) {
        uVar3 = 0x0;
        paVar5 = (astruct_57 *)(paVar5 & 0xffff0000);
      }
      else {
        struct_1038_6520((astruct_308 *)CONCAT22(paVar5,uVar3));
        paVar5 = paVar6;
      }
      file_1038_774e(paVar5,(astruct_169 *)CONCAT22(paVar5,uVar3),pHVar7);
      if (uVar3 == 0x0) {
        return 0x0;
      }
      ppcVar1 = (code **)((param_3 + 0x20) + 0x8);
      (**ppcVar1)();
    }
  }
  return 0x1;
}



StructD * pass1_1028_254c(StructD *param_1,param_2: u8)

{
  pass1_1028_2042((astruct_15 *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * struct_1028_25da(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
    // just 0x264c
  param_1.field0_0x0 = s_fem16_wav_1050_2644 + 0x8;
  (param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_25fc(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = s_fem16_wav_1050_2644 + 0x8;
  (param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}



StructD * pass1_1028_2626(StructD *param_1,param_2: u8)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * struct_1028_26b4(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x2788;
  (param_1 + 0x2) = 0x1028;
  return &param_1.field0_0x0;
}



u16 * pass1_1028_26d6(StructD *param_1,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e(param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x2788;
  (param_2 + 0x2) = 0x1028;
  return &param_2.field0_0x0;
}
pub fn pass1_1028_2700(param_1: *mut astruct_15)

{
  let mut uVar1: u16;
  let mut uVar2: u32;

  pass1_1028_be2a(param_1);
  uVar1 = (param_1 >> 0x10);
  if ((param_1 + 0x12) == 0x5) {
    uVar2 = pass1_1028_b4f2((astruct_15 *)(param_1 & 0xffff | uVar1 << 0x10));
    (uVar2 + 0x204) = 0x1;
  }
  return;
}
pub fn pass1_1028_272e(param_1: *mut astruct_15)

{
  let mut uVar1: u32;

  pass1_1028_be9e(param_1);
  uVar1 = pass1_1028_b4f2(param_1);
  if ((param_1 + 0x12) == 0x5) {
    (uVar1 + 0x204) = 0x1;
  }
  return;
}



StructD * pass1_1028_2762(StructD *param_1,param_2: u8)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_180 * struct_1028_27f0(param_1: *mut astruct_180)

{
  struct_1028_b354(param_1);
  param_1.field0_0x0 = 0x2a92;
  (param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_2812(mut param_1: u16 ,param_2: *mut astruct_12,mut param_3: i16,mut param_4: u32)

{
  pass1_1028_b39e((StructD *)param_1,param_2,param_3,param_4);
  param_2.field0_0x0 = 0x2a92;
  (param_2 + 0x2) = 0x1028;
  (param_2 + 0xe) = (param_2 + 0xc);
  return &param_2.field0_0x0;
}



// WARNING: Could not reconcile some variable overlaps

u16 pass1_1028_2844(mut param_1: u32,u32 *param_2,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 ,i32 param_6)

{
  let mut BVar1: bool;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  u8 local_e [0x2];
  let mut local_c: i16;
  let mut local_a: i16;
  let mut local_8: u32;
  let mut uStack4: u16;

  local_8 = *param_2;
  uStack4 = (param_2 + 0x1);
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,local_e),
                  (u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_a));
  local_8 = local_8 & 0xffff | (local_c - 0x1) << 0x10;
  uVar3 = param_1;
  uVar4 = (param_1 >> 0x10);
  BVar1 = pass1_1028_c5a6(&local_8,param_3,uVar3,uVar4,0x7b,(u16 *)CONCAT22(0x1050,&local_8),param_6);
  if (BVar1 == 0x0) {
    uVar2 = pass1_1028_297c(&local_8,param_3,param_1,(u16 *)CONCAT22(0x1050,&local_8),param_6);
    if (uVar2 == 0x0) {
      local_8 = local_8 & 0xffff | (local_c + 0x1) << 0x10;
      BVar1 = pass1_1028_c5a6(&local_8,param_3,uVar3,uVar4,0x7b,(u16 *)CONCAT22(0x1050,&local_8),param_6);
      if (BVar1 == 0x0) {
        uVar2 = pass1_1028_297c(&local_8,param_3,param_1,(u16 *)CONCAT22(0x1050,&local_8),param_6);
        if (uVar2 == 0x0) {
          local_8 = local_a + -0x1;
          local_8 = local_c;
          BVar1 = pass1_1028_c5a6(&local_8,param_3,uVar3,uVar4,0x7c,(u16 *)CONCAT22(0x1050,&local_8),param_6);
          if (BVar1 == 0x0) {
            uVar2 = pass1_1028_297c(&local_8,param_3,param_1,(u16 *)CONCAT22(0x1050,&local_8),param_6);
            if (uVar2 == 0x0) {
              local_8 = CONCAT22(local_8,local_a + 0x1);
              BVar1 = pass1_1028_c5a6(&local_8,param_3,uVar3,uVar4,0x7c,(u16 *)CONCAT22(0x1050,&local_8),
                                      param_6);
              if (BVar1 == 0x0) {
                uVar3 = pass1_1028_297c(&local_8,param_3,param_1,(u16 *)CONCAT22(0x1050,&local_8),param_6);
                if (uVar3 == 0x0) {
                  return uVar3;
                }
              }
            }
          }
        }
      }
    }
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1028_297c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,param_4: *mut u16,i32 param_5)

{
  char cVar1;
  let mut uVar2: u16;
  let mut uVar3: u32;

  pass1_1028_c7b6(param_2,param_3,(param_3 >> 0x10),param_4,param_5);
  if (param_1 == 0x0) {
    pass1_1030_627e(0x0,param_2,_PTR_LOOP_1050_5740,param_4,param_5);
    uVar2 = param_2 | param_1;
    if (uVar2 != 0x0) {
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,CONCAT22(param_2,param_1));
      uVar3 = struct_op_1030_73a8((astruct_419 *)CONCAT22(uVar2,param_1),param_1,uVar2);
      uVar2 = (uVar3 + 0xc);
      if (0x4a < uVar2) {
        switch(uVar2) {
        case 0x4c:
        case 0x4d:
        case 0x4e:
        case 0x60:
        case 0x61:
        case 0x62:
        case 0x63:
        case 0x6e:
        case 0x73:
        case 0x74:
        case 0x75:
        case 0x76:
        case 0x77:
        case 0x78:
        case 0x79:
          goto switchD_1028_2a0b_caseD_4c;
        default:
          goto switchD_1028_2a0b_caseD_4f;
        }
      }
      if ((uVar2 < 0x48) && (uVar2 != 0x41)) {
        if (uVar2 < 0x42) {
          cVar1 = (char)uVar2;
          if (cVar1 < '5') {
            if ('2' < cVar1) {
              return 0x0;
            }
            cVar1 += -0x10;
          }
          else {
            cVar1 += -0x3e;
          }
          if (cVar1 == '\0') {
            return 0x0;
          }
        }
switchD_1028_2a0b_caseD_4f:
        return 0x1;
      }
    }
  }
switchD_1028_2a0b_caseD_4c:
  return 0x0;
}



StructD * pass1_1028_2a6c(StructD *param_1,param_2: u8)

{
  pass1_1028_b418(&param_1.address_offset_field_0x0);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



u16 * struct_1028_2afa(param_1: *mut u16)

{
  struct_1030_dc96((astruct_180 *)param_1);
  *param_1 = 0x2b74;
  (param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_2b1c(mut param_1: u16 ,mut param_2: i16,mut param_3: u16 ,mut param_4: i16,mut param_5: u32)

{
  pass1_1030_dcc2(param_1,(astruct_12 *)CONCAT22(param_3,param_2),param_4,param_5);
  CONCAT22(param_3,param_2) = 0x2b74;
  (param_2 + 0x2) = 0x1028;
  return (u16 *)CONCAT22(param_3,param_2);
}
pub fn FUN_1028_2b46(void)

{
  return;
}
pub fn FUN_1028_2b4a(void)

{
  return;
}



StructD * pass1_1028_2b4e(mut param_1: u16 ,StructD *param_2,param_3: u8)

{
  pass1_1030_dcf4(param_1,(astruct_15 *)param_2);
  if ((param_3 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_2);
  }
  return param_2;
}



u16 * struct_1028_2bdc(param_1: *mut u16)

{
  struct_1028_0954((astruct_180 *)param_1);
  *param_1 = 0x341c;
  (param_1 + 0x2) = 0x1028;
  return param_1;
}



u16 * pass1_1028_2bfe(mut param_1: u16 ,param_2: *mut astruct_179,mut param_3: u16 ,mut param_4: i16,mut param_5: u32)

{
  pass1_1028_0982(param_1,(astruct_12 *)CONCAT22(param_3,param_2),param_4,param_5);
  CONCAT22(param_3,param_2) = 0x341c;
  param_2.field2_0x2 = 0x1028;
  return (u16 *)CONCAT22(param_3,param_2);
}



u16 pass1_1028_2c28(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u32,u32 *param_4,mut param_5: u32,mut param_6: u32)

{
  let mut puVar1: *mut u32;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut local_e: u16;
  let mut local_c: u16;
  let mut local_a: u16;
  let mut local_8: u32;
  let mut uStack4: u16;

  pass1_1028_09d4(param_1,param_2,param_3,(u16 *)param_4,param_5,param_6);
  if (param_1 != 0x0) {
    local_8 = *param_4;
    uStack4 = (param_4 + 0x4);
    puVar2 = &local_e;
    pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_8),(u16 *)CONCAT22(0x1050,puVar2),
                    (u16 *)CONCAT22(0x1050,&local_c),(u16 *)CONCAT22(0x1050,&local_a));
    pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c - 0x1,local_a - 0x1);
    puVar1 = &local_8;
    uVar3 = param_3;
    uVar4 = (param_3 >> 0x10);
    pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
    if (puVar1 != NULL) {
      pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c - 0x1,local_a);
      puVar1 = &local_8;
      pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
      if (puVar1 != NULL) {
        pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c - 0x1,local_a + 0x1);
        puVar1 = &local_8;
        pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
        if (puVar1 != NULL) {
          pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c,local_a - 0x1);
          puVar1 = &local_8;
          pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
          if (puVar1 != NULL) {
            pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c,local_a);
            puVar1 = &local_8;
            pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
            if (puVar1 != NULL) {
              pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c,local_a + 0x1);
              puVar1 = &local_8;
              pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
              if (puVar1 != NULL) {
                pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c + 0x1,local_a - 0x1);
                puVar1 = &local_8;
                pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
                if (puVar1 != NULL) {
                  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c + 0x1,local_a);
                  puVar1 = &local_8;
                  pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
                  if (puVar1 != NULL) {
                    pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_8),local_e,local_c + 0x1,local_a + 0x1);
                    puVar1 = &local_8;
                    pass1_1028_c7b6(puVar2,uVar3,uVar4,(u16 *)CONCAT22(0x1050,puVar1),param_6);
                    if (puVar1 != NULL) {
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
    PTR_LOOP_1050_50ca = 0x6a8;
  }
  return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_2e40(uchar param_1,mut param_2: i16,u8 *param_3,u32 *param_4)

{
  let mut in_register_0000000a: u16;
  let mut paVar1: *mut Struct57;
  astruct_27 *paVar2;
  let mut in_stack_0000fe9e: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000ffcc: u16;
  let mut uVar3: u16;
  let mut iVar4: i16;

  paVar1 = (astruct_57 *)CONCAT22(in_register_0000000a,param_3);
  pass1_1028_be9e((astruct_15 *)param_4);
  if ((param_4 + 0x12) == 0x5) {
    pass1_1028_2f18(param_1,param_2,(astruct_15 *)param_4);
    pass1_1028_3246(param_2,paVar1,(astruct_15 *)param_4);
    uVar3 = 0x1;
    iVar4 = 0x5;
    paVar2 = (astruct_27 *)
             mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fe9e,in_stack_0000ffc2,
                             in_stack_0000ffc8,in_stack_0000ffcc);
    pass1_1010_089e(paVar2,uVar3,iVar4);
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_2e84(StructD *param_1,param_2: *mut astruct_15,mut param_3: u16 ,mut param_4: i16)

{
  let mut paVar1: *mut Struct57;
  astruct_67 *paVar2;
  astruct_27 *paVar3;
  let mut puVar4: *mut u32;
  let mut in_stack_0000fe88: u16;
  let mut in_stack_0000fe90: u16;
  let mut in_stack_0000fe96: u16;
  let mut in_stack_0000ffac: u16;
  let mut in_stack_0000ffb2: u16;
  let mut in_stack_0000ffb4: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut iVar7: i16;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut iVar11: i16;

  pass1_1028_09b8(param_2);
  if (param_4 == 0x0) {
    uVar10 = 0x0;
    iVar11 = 0x8;
    uVar8 = 0x1;
    uVar9 = 0x0;
    uVar6 = 0x0;
    iVar7 = 0x0;
    uVar5 = 0x0;
    paVar2 = (astruct_67 *)
             mixed_1010_20ba((astruct_57 *)param_1,_u16_1050_0ed0,(u8 **)0x37,in_stack_0000fe88,in_stack_0000ffac
                             ,in_stack_0000ffb2,in_stack_0000ffb6);
    paVar1 = (astruct_57 *)(param_1 & 0xffff0000 | paVar2 >> 0x10);
    post_win_msg_1008_a0e4(paVar2,CONCAT22(uVar6,uVar5),iVar7,uVar8,CONCAT22(uVar10,uVar9),iVar11);
    uVar6 = 0x400;
    iVar7 = 0x3;
    uVar5 = 0x1;
    paVar3 = (astruct_27 *)
             mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)0x1002b,in_stack_0000fe90,in_stack_0000ffb4,
                             in_stack_0000ffba,in_stack_0000ffbe);
    paVar1 = (astruct_57 *)(paVar1 & 0xffff0000 | paVar3 >> 0x10);
    pass1_1010_043a(paVar3,CONCAT22(uVar6,uVar5),iVar7);
    pass1_1010_043a(paVar3,0x4000001,0x4);
    puVar4 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x2f),in_stack_0000fe96,
                             in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
    paVar1 = (astruct_57 *)(paVar1 & 0xffff0000 | puVar4 >> 0x10);
    pass1_1010_ec84(puVar4);
    puVar4 = mixed_1010_20ba(paVar1,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x8),in_stack_0000fe96,
                             in_stack_0000ffba,in_stack_0000ffc0,in_stack_0000ffc4);
    pass1_1010_9766((puVar4 >> 0x10),puVar4);
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1028_2f18(uchar param_1,mut param_2: i16,param_3: *mut astruct_15)

{
  let mut iVar1: i16;
  let mut puVar2: *mut u32;
  let mut extraout_DX: u16;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut puVar5: *mut u16;
  u8 local_944 [0x124];
  u8 local_820 [0x124];
  u8 local_6fc [0x124];
  u8 local_5d8 [0x124];
  u8 local_4b4 [0x124];
  let mut local_390: u32;
  u8 local_38a [0x124];
  u8 local_266 [0x124];
  u8 local_142 [0x124];
  let mut local_1e: u32;
  let mut local_1a: u16;
  let mut local_18: u32;
  let mut uStack20: u16;
  let mut uStack18: u32;
  let mut uStack14: u32;
  let mut uStack10: u32;
  let mut uStack6: u32;

  uStack6 = pass1_1028_bb24(param_3);
  iVar1 = uStack6;
  pass1_1028_b58e(param_3);
  uStack10 = CONCAT22(extraout_DX,iVar1);
  uStack14 = (iVar1 + 0x2e);
  uStack18 = (uStack14 + 0x4);
  local_18 = (iVar1 + 0xc);
  uStack20 = (iVar1 + 0x10);
  pass1_1008_3eb4((astruct_615 *)CONCAT22(0x1050,&local_18),(u16 *)CONCAT22(0x1050,&local_1e),
                  (u16 *)CONCAT22(0x1050,&local_1e + 0x2),(u16 *)CONCAT22(0x1050,&local_1a));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_18),local_1e,local_1e - 0x1,local_1a - 0x1);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_142),0x0,0x0,0xd,&local_18,&DAT_1050_1050,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_142));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_18),local_1e,local_1e + 0x1,local_1a + 0x1);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_266),0x0,0x0,0xc,&local_18,&DAT_1050_1050,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_266));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_18),local_1e,local_1e + 0x1,local_1a - 0x1);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_38a),0x0,0x0,0xe,&local_18,&DAT_1050_1050,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_38a));
  puVar5 = pass1_1008_3e54((u16 *)CONCAT22(0x1050,&local_390),local_1e,local_1e - 0x1,local_1a + 0x1);
  uVar3 = (puVar5 >> 0x10);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_4b4),0x0,0x0,0xb,&local_390,&DAT_1050_1050,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_4b4));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_18),local_1e,local_1e - 0x1,local_1a);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_5d8),0x0,0x0,0x7a,&local_18,&DAT_1050_1050,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_5d8));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_18),local_1e,(local_1e >> 0x10),local_1a + 0x1);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_6fc),0x0,0x0,0x7a,&local_18,&DAT_1050_1050,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_6fc));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_18),local_1e,local_1e + 0x1,local_1a);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_820),0x0,0x0,0x7a,&local_18,&DAT_1050_1050,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_820));
  pass1_1008_3e76((u16 *)CONCAT22(0x1050,&local_18),local_1e,(local_1e >> 0x10),local_1a - 0x1);
  struct_op_1028_87f0((astruct_97 *)CONCAT22(0x1050,local_944),0x0,0x0,0x7a,&local_18,&DAT_1050_1050,uStack18,
                      uStack6);
  fn_ptr_1030_835a(_u16_1050_5748,CONCAT22(0x1050,local_944));
  puVar2 = &local_390;
  pass1_1030_627e(puVar2,uVar3,_PTR_LOOP_1050_5740,(u16 *)CONCAT22(0x1050,puVar2),uStack6);
  uVar4 = (uStack14 >> 0x10);
  (u32*)(uStack14 + 0x10) = puVar2;
  (uStack14 + 0x12) = uVar3;
  return;
}

