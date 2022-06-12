
pub fn pass1_1010_6006(mut param_1: u16 ,param_2: *mut astruct_486,char *param_3)

{
  let mut uVar1: u16;
  astruct_486 *iVar3;
  astruct_486 *uVar2;

  uVar2 = (astruct_486 *)(param_2 >> 0x10);
  iVar3 = (astruct_486 *)param_2;
  fn_ptr_1000_17ce(*(char **)&iVar3->field108_0x6c);
  uVar1 = str_op_1008_60e8(param_1,param_3);
  iVar3->field108_0x6c = uVar1;
  iVar3->field109_0x6e = param_1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_6034(mut param_1: u16 ,param_2: *mut astruct_19)

{
  let mut puVar1: *mut u16;
  astruct_19 *struct_1;
  astruct_19 *struct_1_hi;

  struct_1_hi = (astruct_19 *)(param_2 >> 0x10);
  struct_1 = (astruct_19 *)param_2;
  struct_1->field16_0x1e = 0x1;
  struct_1->field17_0x20 = 0x1;
  struct_1[0x1].field9_0x12 = 0x1;
  struct_1[0x1].field10_0x14 = 0x1;
  pass1_1010_60a0(param_2);
  puVar1 = pass1_1000_4906((StructD *)(param_2 & 0xffff0000 | ZEXT24(&struct_1->field18_0x22)),NULL,0x40);
  load_string_1010_84ac(_u16_1050_14cc,(INT16)(_u16_1050_14cc >> 0x10),0x630);
  struct_1[0x1].field3_0x8 = puVar1;
  struct_1[0x1].horiz_res_0xa = param_1;
  load_string_1010_84ac(_u16_1050_14cc,(INT16)(_u16_1050_14cc >> 0x10),0x62f);
  struct_1[0x1].ver_res_0xc = puVar1;
  &struct_1[0x1].field_0xe = param_1;
  return;
}
pub fn pass1_1010_60a0(param_1: *mut astruct_19)

{
  (param_1 + 0x76) = 0x5;
  return;
}



u16 pass1_1010_60b4(void)

{
  return 0x1;
}



u16 pass1_1010_60ba(void)

{
  return 0x1;
}



u16 pass1_1010_60c0(void)

{
  return 0x1;
}



u16 pass1_1010_60c6(void)

{
  return 0x1;
}
pub fn pass1_1010_60cc(mut param_1: u16 ,param_2: *mut astruct_487,char *param_3)

{
  let mut uVar1: u16;
  astruct_487 *iVar3;
  astruct_487 *uVar2;

  uVar2 = (astruct_487 *)(param_2 >> 0x10);
  iVar3 = (astruct_487 *)param_2;
  fn_ptr_1000_17ce(*(char **)&iVar3->field26_0x1a);
  uVar1 = str_op_1008_60e8(param_1,param_3);
  iVar3->field26_0x1a = uVar1;
  iVar3->field27_0x1c = param_1;
  return;
}
pub fn pass1_1010_60fa(mut param_1: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  (iVar1 + 0x7e) = 0x1;
  (iVar1 + 0x7c) = (iVar1 + 0x20);
  (iVar1 + 0x20) = 0x1;
  return;
}
pub fn pass1_1010_6118(mut param_1: u32)

{
  let mut iVar1: i16;
  let mut uVar2: u16;

  uVar2 = (param_1 >> 0x10);
  iVar1 = param_1;
  if ((iVar1 + 0x7e) != 0x0) {
    (iVar1 + 0x20) = (iVar1 + 0x7c);
  }
  return;
}
pub fn get_private_profile_string_1010_6132(param_1: *mut astruct_19,mut param_2: i16)

{
  let mut uVar2: u16;
  let mut iVar1: i16;
  let mut uVar3: u16;
  let mut iVar2: i16;
  let mut uVar4: u16;
  let mut iVar3: i16;
  let mut uVar5: u16;
  let mut iVar4: i16;
  let mut in_DX: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut uVar8: u16;
  astruct_19 *iVar5;
  let mut iVar6: i16;
  let mut uVar9: u16;
  let mut unaff_SS: u16;
  let mut uVar1: u32;

  uVar9 = (param_1 >> 0x10);
  iVar5 = (astruct_19 *)param_1;
  GetPrivateProfileString16
            (*(char **)&iVar5->horiz_res_0xa,0x100,*(char **)&iVar5->field_0xe,s_playerName_1050_148e + 0xc,
             *(char **)(param_2 * 0x4 + 0x1446),s_windows_1050_13b8);
  if (**(char **)&iVar5->field_0xe != '\0') {
    uVar2 = pass1_1000_47a4(&iVar5->field_0xe,s___1050_14a6);
    uVar6 = in_DX | uVar2;
    if (uVar6 != 0x0) {
      iVar1 = pass1_1000_3e2c(CONCAT22(in_DX,uVar2));
      (&iVar5->field18_0x22)[param_2 * 0x4] = iVar1;
      uVar3 = pass1_1000_47a4(0x0,s___1050_14a8);
      uVar7 = uVar6 | uVar3;
      if (uVar7 != 0x0) {
        iVar2 = pass1_1000_3e2c(CONCAT22(uVar6,uVar3));
        (&iVar5->field19_0x24)[param_2 * 0x4] = iVar2;
        uVar4 = pass1_1000_47a4(0x0,s___1050_14aa);
        uVar8 = uVar7 | uVar4;
        if (uVar8 != 0x0) {
          iVar3 = pass1_1000_3e2c(CONCAT22(uVar7,uVar4));
          (&iVar5->field20_0x26 + param_2 * 0x2) = iVar3;
          uVar5 = pass1_1000_47a4(0x0,s___1050_14ac);
          if ((uVar8 | uVar5) != 0x0) {
            iVar4 = pass1_1000_3e2c(CONCAT22(uVar8,uVar5));
            (&iVar5->field20_0x26 + param_2 * 0x8 + 0x2) = iVar4;
          }
        }
      }
    }
  }
  return;
}
pub fn caseD_13(mut param_1: u32,mut param_2: i16)

{
  astruct_833 *iVar1;

  iVar1 = (astruct_833 *)(param_2 * 0x8 + param_1);
  if ((((iVar1->field34_0x22 != 0x0) || (iVar1->field35_0x24 != 0x0)) || (iVar1->field36_0x26 != 0x0)) ||
     (iVar1->field37_0x28 != 0x0)) {
    sys_1000_3f9c(*(char **)(param_1 + 0xe),s__d__d__d__d_1050_14ae,
                  (param_2 * 0x8 + param_1 + 0x22));
    WritePrivateProfileString16
              (*(char **)(param_1 + 0xa),*(char **)(param_1 + 0xe),*(char **)(param_2 * 0x4 + 0x1446),
               s_windows_1050_13b8);
  }
  return;
}
pub fn pass1_1010_62a4(param_1: *mut astruct_488,param_2: u8)

{
  astruct_488 *uVar2;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  uVar2 = (astruct_488 *)param_1;
  param_1 = 0x6322;
  uVar2->field2_0x2 = 0x1010;
  fn_ptr_1000_17ce(uVar2->field3_0x4);
  param_1 = 0x389a;
  uVar2->field2_0x2 = 0x1008;
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return;
}



StructD * pass1_1010_62ec(undefined1 param_1,mut param_2: u16 ,StructD *param_3,param_4: u8)

{
  write_private_profile_str_1010_5b10(param_2,param_3);
  if ((param_4 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_3);
  }
  return param_3;
}
pub fn struct_1010_6326(param_1: *mut astruct_19,mut param_2: u16 )

{
  struct_op_1010_1d48(param_1,param_2);
  (param_1 + 0xa) = 0x0;
  (param_1 + 0xe) = 0x0;
  (param_1 + 0x12) = 0x0;
  (param_1 + 0x16) = 0x0;
  (param_1 + 0x1a) = 0x0;
  (param_1 + 0x1e) = 0x0;
  (param_1 + 0x22) = 0x0;
  param_1->offset_0x0 = 0x66f0;
  (param_1 + 0x2) = 0x1010;
  return;
}
pub fn write_to_file_1010_6372(param_1: *mut astruct_729,mut param_2: u32)

{
  let mut BVar1: bool;
  astruct_729 *iVar2;
  let mut uVar2: u16;
  HFILE16 in_stack_0000ffce;
  u32 local_10 [0x2];
  let mut local_8: u32;

  BVar1 = write_to_file_1008_7cac(param_2);
  if (BVar1 != 0x0) {
    uVar2 = (param_1 >> 0x10);
    iVar2 = (astruct_729 *)param_1;
    local_10[0] = iVar2->field10_0xa;
    BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_10),0x4,in_stack_0000ffce);
    if (BVar1 != 0x0) {
      local_8 = iVar2->field11_0xe;
      BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffce);
      if (BVar1 != 0x0) {
        local_8 = iVar2->field12_0x12;
        BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffce);
        if (BVar1 != 0x0) {
          local_8 = iVar2->field13_0x16;
          BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffce);
          if (BVar1 != 0x0) {
            local_8 = iVar2->field14_0x1a;
            BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffce);
            if (BVar1 != 0x0) {
              local_8 = iVar2->field15_0x1e;
              BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffce);
              if (BVar1 != 0x0) {
                local_8 = iVar2->field16_0x22;
                BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_8),0x4,in_stack_0000ffce);
                if (BVar1 != 0x0) {
                  return;
                }
              }
            }
          }
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}
pub fn pass1_1010_648a(mut param_1: i16,mut param_2: u32,mut param_3: u32)

{
  let mut iVar1: i16;
  let mut BVar2: bool;

  read_file_1008_7cfe(param_3,(param_3 >> 0x10),0x7);
  if (param_1 != 0x0) {
    iVar1 = param_2;
    BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(param_2 & 0xffff0000 | (iVar1 + 0xa)),0x4);
    if (BVar2 != 0x0) {
      BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(param_2 & 0xffff0000 | (iVar1 + 0xe)),0x4);
      if (BVar2 != 0x0) {
        BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(param_2 & 0xffff0000 | (iVar1 + 0x12)),0x4);
        if (BVar2 != 0x0) {
          BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(param_2 & 0xffff0000 | (iVar1 + 0x16)),0x4);
          if (BVar2 != 0x0) {
            BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(param_2 & 0xffff0000 | (iVar1 + 0x1a)),0x4);
            if (BVar2 != 0x0) {
              BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(param_2 & 0xffff0000 | (iVar1 + 0x1e)),0x4);
              if (BVar2 != 0x0) {
                BVar2 = read_file_1008_7dee((HFILE16 *)param_3,(param_2 & 0xffff0000 | (iVar1 + 0x22)),0x4)
                ;
                if (BVar2 != 0x0) {
                  return;
                }
              }
            }
          }
        }
      }
    }
    u16_1050_0310 = 0x6d2;
  }
  return;
}
pub fn pass1_1010_6566(mut param_1: u32,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut local_4: i16;

  uVar1 = param_1;
  uVar2 = (param_1 >> 0x10);
  switch_1010_6646(uVar1,uVar2,(u16 *)CONCAT22(0x1050,&local_4),param_4);
  if (local_4 != 0x0) {
    (uVar1 + local_4) = param_3;
    (uVar1 + local_4 + 0x2) = param_2;
  }
  return;
}



i16 pass1_1010_659a(mut param_1: u32,mut param_2: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut local_4: i16;

  uVar1 = param_1;
  uVar2 = (param_1 >> 0x10);
  switch_1010_6646(uVar1,uVar2,(u16 *)CONCAT22(0x1050,&local_4),param_2);
  if (local_4 == 0x0) {
    return 0x0;
  }
  return (uVar1 + local_4) - (uVar1 + local_4 + 0x2);
}



u16 pass1_1010_65d0(mut param_1: u32,mut param_2: u16 )

{
  let mut uVar1: u16;
  let mut local_4: i16;

  uVar1 = (param_1 >> 0x10);
  switch_1010_6646(param_1,uVar1,(u16 *)CONCAT22(0x1050,&local_4),param_2);
  if (local_4 == 0x0) {
    return 0x0;
  }
  return (param_1 + local_4 + 0x2);
}
pub fn pass1_1010_6604(mut param_1: u32,mut param_2: u16 )

{
  let mut iVar1: i16;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut local_4: i16;

  uVar2 = param_1;
  uVar3 = (param_1 >> 0x10);
  switch_1010_6646(uVar2,uVar3,(u16 *)CONCAT22(0x1050,&local_4),param_2);
  if (local_4 != 0x0) {
    iVar1 = (uVar2 + local_4 + 0x2);
    (uVar2 + local_4) = (uVar2 + local_4);
    (uVar2 + local_4 + 0x2) = iVar1 + 0x1;
    pass1_1010_1f62((astruct_27 *)(param_1 & 0xffff | uVar3 << 0x10),0x15);
  }
  return;
}
pub fn switch_1010_6646(mut param_1: u16 ,mut param_2: u16 ,param_3: *mut u16,mut param_4: u16 )

{
  switch(param_4) {
  case 0x83:
    *param_3 = 0xa;
    break;
  case 0x84:
    *param_3 = 0xe;
    break;
  case 0x85:
    *param_3 = 0x12;
    break;
  case 0x86:
    *param_3 = 0x16;
    return;
  case 0x87:
    *param_3 = 0x1a;
    return;
  case 0x88:
    *param_3 = 0x1e;
    return;
  case 0x89:
    *param_3 = 0x22;
    return;
  default:
    *param_3 = 0x0;
    return;
  }
  return;
}



u16 * pass1_1010_66ca(param_1: *mut u16,param_2: u8)

{
  pass1_1010_1d80((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_19 * pass1_1010_6700(param_1: *mut astruct_19,mut param_2: u16 )

{
  struct_op_1010_1d48(param_1,param_2);
  (param_1 + 0x148) = 0x33;
    //        1010:6aac  86  6a  10  10      addr         pass1_1010_6a86
    //
  param_1->offset_0x0 = 0x6aac;
  (param_1 + 0x2) = 0x1010;
  pass1_1000_4906((StructD *)(param_1 & 0xffff0000 | (param_1 + 0xa)),NULL,0x114);
  (param_1 + 0x32) = 0x1;
  (param_1 + 0x40) = 0x1;
  (param_1 + 0x46) = 0x1;
  (param_1 + 0x4e) = 0x1;
  (param_1 + 0x54) = 0x1;
  (param_1 + 0x5e) = 0x1;
  (param_1 + 0x68) = 0x1;
  (param_1 + 0x6c) = 0x1;
  (param_1 + 0x74) = 0x1;
  (param_1 + 0x78) = 0x1;
  (param_1 + 0x7a) = 0x1;
  (param_1 + 0x7e) = 0x1;
  (param_1 + 0x82) = 0x1;
  (param_1 + 0xa2) = 0x1;
  (param_1 + 0xa4) = 0x1;
  (param_1 + 0xa6) = 0x1;
  (param_1 + 0xa8) = 0x1;
  (param_1 + 0xae) = 0x1;
  (param_1 + 0xb2) = 0x1;
  (param_1 + 0xb8) = 0x1;
  (param_1 + 0xbe) = 0x1;
  (param_1 + 0xc0) = 0x1;
  (param_1 + 0xc4) = 0x1;
  (param_1 + 0xd4) = 0x1;
  (param_1 + 0xda) = 0x1;
  (param_1 + 0xe2) = 0x1;
  (param_1 + 0xfe) = 0x1;
  (param_1 + 0x100) = 0x1;
  (param_1 + 0x102) = 0x1;
  (param_1 + 0x104) = 0x1;
  (param_1 + 0x106) = 0x1;
  (param_1 + 0x108) = 0x1;
  pass1_1000_4906((StructD *)(param_1 & 0xffff0000 | (param_1 + 0x11e)),NULL,0x2a);
  (param_1 + 0x120) = 0x1;
  (param_1 + 0x122) = 0x1;
  (param_1 + 0x124) = 0x1;
  (param_1 + 0x126) = 0x1;
  (param_1 + 0x128) = 0x1;
  (param_1 + 0x12c) = 0x1;
  (param_1 + 0x138) = 0x1;
  return param_1;
}
pub fn pass1_1010_6814(mut param_1: u32,mut param_2: u16 ,mut param_3: i16)

{
  (param_1 + param_3 * 0x2 + 0x11e) = param_2;
  return;
}
pub fn pass1_1010_682e(mut param_1: u32,mut param_2: u16 ,mut param_3: i16)

{
  (param_1 + param_3 * 0x2 + 0xa) = param_2;
  return;
}
pub fn write_to_file_1010_6846(mut param_1: u32,u8 *param_2)

{
  let mut BVar1: bool;
  let mut iVar2: i16;
  HFILE16 in_stack_0000ffde;
  u16 local_c [0x5];

  BVar1 = write_to_file_1008_7cac(param_2);
  if (BVar1 != 0x0) {
    iVar2 = param_1;
    BVar1 = write_to_file_1008_7e1c(param_2,param_1 & 0xffff0000 | (iVar2 + 0xa),0x114,in_stack_0000ffde)
    ;
    if (BVar1 != 0x0) {
      BVar1 = write_to_file_1008_7e1c
                        (param_2,param_1 & 0xffff0000 | (iVar2 + 0x11e),0x2a,in_stack_0000ffde);
      if (BVar1 != 0x0) {
        local_c[0] = (iVar2 + 0x148);
        BVar1 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_c),0x2,in_stack_0000ffde);
        if (BVar1 != 0x0) {
          return;
        }
      }
    }
    u16_1050_0310 = 0x6d0;
  }
  return;
}
pub fn pass1_1010_68c6(mut param_1: u16 ,u8 *param_2,mut param_3: u32,mut param_4: u32)

{
  astruct_248 *iVar2;
  let mut BVar1: bool;
  let mut iVar3: i16;
  let mut BVar4: bool;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut in_register_0000000a: u16;
  let mut paVar7: *mut Struct57;
  let mut uVar8: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut uVar11: u16;
  let mut uVar12: u16;
  let mut pcStack18: *mut c_char;
  let mut pcStack10: *mut c_char;
  let mut local_6: i16;
  let mut uStack4: u16;

  paVar7 = (astruct_57 *)CONCAT22(in_register_0000000a,param_2);
  uVar9 = param_4;
  uVar10 = (param_4 >> 0x10);
  read_file_1008_7cfe(uVar9,uVar10,0x3);
  if (param_1 == 0x0) {
    u16_1050_0310 = 0x6d4;
    return;
  }
  iVar2 = (astruct_248 *)param_3;
  uVar8 = (param_3 >> 0x10);
  if (u16_1050_0312 < 0x2) {
    uVar11 = 0x102;
    uVar12 = 0x0;
    mem_op_1000_179c(0x102,paVar7);
    uVar6 = paVar7;
    pcStack10 = CONCAT22(uVar6,param_1);
    BVar1 = read_file_1008_7dee((HFILE16 *)param_4,CONCAT22(uVar6,param_1),CONCAT22(uVar12,uVar11));
    pcStack18 = pcStack10;
    if (BVar1 == 0x0) goto LAB_1010_692c;
    uStack4 = 0x1;
    do {
      iVar3 = switch_1008_73ea(uVar9,uVar10,uStack4);
      (&iVar2->field_0xa + iVar3 * 0x2) = (uStack4 * 0x2 + param_1);
      uStack4 += 0x1;
    } while (uStack4 < 0x81);
    fn_ptr_1000_17ce(pcStack10);
    BVar1 = (BOOL16)pcStack10;
  }
  else {
    BVar1 = read_file_1008_7dee((HFILE16 *)param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar2->field_0xa)),0x114);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
  }
  if (u16_1050_0312 < 0x2) {
    uVar11 = 0x2a;
    uVar12 = 0x0;
    mem_op_1000_179c(0x2a,paVar7);
    uVar6 = paVar7;
    pcStack18 = CONCAT22(uVar6,BVar1);
    BVar4 = read_file_1008_7dee((HFILE16 *)param_4,CONCAT22(uVar6,BVar1),CONCAT22(uVar12,uVar11));
    if (BVar4 == 0x0) {//
LAB_1010_692c:
      u16_1050_0310 = 0x6d2;
      fn_ptr_1000_17ce((pcStack18 & 0xffff | uVar6 << 0x10));
      return;
    }
    uStack4 = 0x0;
    do {
      uVar5 = switch_1008_72bc((HFILE16 *)param_4,uStack4);
      (&iVar2->field_0x11e + uVar5 * 0x2) = (uStack4 * 0x2 + BVar1);
      uStack4 += 0x1;
    } while (uStack4 < 0x15);
    fn_ptr_1000_17ce(pcStack18);
  }
  else {
    BVar1 = read_file_1008_7dee((HFILE16 *)param_4,(param_3 & 0xffff0000 | ZEXT24(&iVar2->field_0x11e)),0x2a);
    if (BVar1 == 0x0) {
      u16_1050_0310 = 0x6d2;
      return;
    }
  }
  BVar1 = read_file_1008_7dee((HFILE16 *)param_4,CONCAT22(0x1050,&local_6),0x2);
  if (BVar1 == 0x0) {
    u16_1050_0310 = 0x6d2;
    return;
  }
  BVar1 = switch_1008_73ea(uVar9,uVar10,local_6);
  iVar2->field328_0x148 = BVar1;
  return;
}



u16 * pass1_1010_6a86(param_1: *mut u16,param_2: u8)

{
  pass1_1010_1d80((StructD *)param_1);
  if ((param_2 & 0x1) != 0x0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn pass1_1010_6abc(param_1: *mut astruct_19,mut param_2: u16 ,mut param_3: u16 )

{
  code **ppcVar1;
  let mut in_EDX: u32;
  let mut uVar3: u16;
  let mut paVar2: *mut Struct57;
  astruct_19 *paVar4;
  let mut puVar5: *mut u32;
  let mut in_stack_0000fe94: u16;
  let mut in_stack_0000fe9e: u16;
  let mut in_stack_0000ffb8: u16;
  let mut in_stack_0000ffbe: u16;
  let mut in_stack_0000ffc2: u16;
  let mut in_stack_0000ffc8: u16;
  let mut in_stack_0000ffcc: u16;

  uVar3 = (in_EDX >> 0x10);
  paVar4 = struct_op_1010_1d48(param_1,param_2);
  paVar2 = (astruct_57 *)CONCAT22(uVar3,(paVar4 >> 0x10));
    //        1008:389a  7e  37  08  10      addr         pass1_1008_377e
    //
  (param_1 + 0xa) = 0x389a;
  (param_1 + 0xc) = 0x1008;
    //        1008:3aa8  14  3a  08  10      addr *       pass1_1008_3a14
  (param_1 + 0xa) = 0x3aa8;
  (param_1 + 0xc) = 0x1008;
  (param_1 + 0xe) = 0x0;
  (param_1 + 0x10) = 0x0;
  (param_1 + 0x14) = 0x0;
  (param_1 + 0x1c) = 0x0;
  (param_1 + 0x20) = 0x0;
    //        1010:7e28  fe  7d  10  10      addr         FUN_1010_7dfe
    //
  (param_1 + 0x22) = 0x0;
  param_1->offset_0x0 = 0x7e28;
  (param_1 + 0x2) = 0x1010;
    //        1010:7e38  c6  7d  10  10      addr         pass1_1010_7dc6
    //
  (param_1 + 0xa) = 0x7e38;
  (param_1 + 0xc) = 0x1010;
  puVar5 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(param_3,0x3),in_stack_0000fe9e,in_stack_0000ffc2
                           ,in_stack_0000ffc8,in_stack_0000ffcc);
  paVar2 = (astruct_57 *)(paVar2 & 0xffff0000);
  (param_1 + 0x14) = puVar5;
  (param_1 + 0x16) = (puVar5 >> 0x10);
  if (param_1 != NULL) {
    paVar2 = (astruct_57 *)(paVar2 | param_1);
  }
  uVar3 = (param_1 + 0x14);
  ppcVar1 = (code **)((param_1 + 0x14) + 0x4);
  (**ppcVar1)();
  puVar5 = mixed_1010_20ba(paVar2,_u16_1050_0ed0,(u8 **)CONCAT22(uVar3,0x2f),in_stack_0000fe94,in_stack_0000ffb8,
                           in_stack_0000ffbe,in_stack_0000ffc2);
  (param_1 + 0x22) = puVar5;
  (param_1 + 0x24) = (puVar5 >> 0x10);
  ppcVar1 = (code **)((param_1 + 0x22) + 0x4);
  (**ppcVar1)();
  return;
}
pub fn pass1_1010_6bb2(param_1: *mut u16)

{
  let mut puVar1: *mut u32;
  code **ppcVar2;
  let mut uVar3: u16;
  let mut iVar4: i16;
  let mut uVar5: u16;
  let mut uVar6: u16;
  let mut uVar7: u16;
  let mut puStack14: *mut u16;

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
  if (*(i32 *)(uVar6 + 0x14) != 0x0) {
    uVar3 = uVar7 | uVar6;
    if (param_1 == NULL) {
      uVar5 = 0x0;
    }
    else {
      uVar3 = uVar6 + 0xa;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6((uVar6 + 0x14),(StructD *)CONCAT22(uVar5,uVar3));
  }
  if (*(i32 *)(uVar6 + 0x22) != 0x0) {
    uVar3 = uVar7 | uVar6;
    if (param_1 == NULL) {
      uVar5 = 0x0;
    }
    else {
      uVar3 = uVar6 + 0xa;
      uVar5 = uVar7;
    }
    pass1_1010_1ea6((uVar6 + 0x22),(StructD *)CONCAT22(uVar5,uVar3));
  }
  (uVar6 + 0x14) = 0x0;
  (uVar6 + 0x22) = 0x0;
  if (param_1 == NULL) {
    iVar4 = 0x0;
    uVar7 = 0x0;
  }
  else {
    iVar4 = uVar6 + 0xa;
  }
  puStack14 = (u16 *)CONCAT22(uVar7,iVar4);
  *puStack14 = 0x389a;
  (iVar4 + 0x2) = 0x1008;
  pass1_1010_1d80((StructD *)param_1);
  return;
}



// WARNING: Could not reconcile some variable overlaps

u16 pass1_1010_6ca2(mut param_1: u16 ,mut param_2: u32,mut param_3: i16)

{
  let mut uVar1: u32;
  let mut puVar2: *mut u16;
  let mut uVar3: u16;
  let mut iStack10: i16;
  let mut puStack8: *mut u16;

  puStack8 = (u16 *)CONCAT22(0x1050,&stack0x000a);
  iStack10 = param_3;
  do {
    puVar2 = puStack8;
    if (iStack10 == 0x0) {
      return 0x1;
    }
    puStack8 = (u16 *)(puStack8 & 0xffff0000 | (puStack8 + 0x2));
    uVar3 = *puVar2;
    uVar1 = (param_2 + 0x14);
    pass1_1010_a5ca(uVar3,param_1,uVar1,(uVar1 >> 0x10),uVar3);
    iStack10 = iStack10 + -0x1;
  } while (uVar3 == 0x0);
  return 0x0;
}



u16 pass1_1010_6cf8(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32,mut param_5: i16)

{
  let mut uVar1: u16;

  switch(param_5) {
  case 0x1:
    pass1_1010_715c(param_1,param_2,param_4,0x1);
    send_msg_1010_7c9e(param_4,0x12);
    return 0x1;
  default:
    return 0x0;
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
    pass1_1010_715c(param_1,param_2,param_4,0x6);
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
    if (param_5 == 0x75) {
      pass1_1010_715c(param_1,param_2,param_4,0x8);
      pass1_1010_715c(param_1,param_2,param_4,0x9);
    }
    uVar1 = pass1_1010_6ca2(param_2,param_4,0x7);
    if (uVar1 != 0x0) {
      pass1_1010_715c(uVar1,param_2,param_4,0x10);
    }
    param_1 = pass1_1010_6ca2(param_2,param_4,0x3);
    if (param_1 != 0x0) {
      pass1_1010_715c(param_1,param_2,param_4,0x11);
    }
    if (param_5 == 0x21) {
      pass1_1010_715c(param_1,param_2,param_4,0x14);
    }
    if (param_5 != 0xc) {
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
    if (param_5 == 0x7e) {
      pass1_1010_715c(param_1,param_2,param_4,0x2c);
    }
    if (param_5 == 0x5b) {
      pass1_1010_715c(param_1,param_2,param_4,0x38);
    }
    if (param_5 == 0x1f) {
      pass1_1010_715c(param_1,param_2,param_4,0x3f);
    }
    if (param_5 == 0x7f) {
      pass1_1010_715c(param_1,param_2,param_4,0x42);
    }
    param_1 = pass1_1010_6ca2(param_2,param_4,0x5);
    if ((param_1 == 0x0) && (param_1 = pass1_1010_6ca2(param_2,param_4,0x5), param_1 == 0x0)) {
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
    uVar1 = pass1_1010_6ca2(param_2,param_4,0x2);
    if (uVar1 != 0x0) {
      pass1_1010_715c(uVar1,param_2,param_4,0x12);
    }
    uVar1 = pass1_1010_6ca2(param_2,param_4,0x8);
    if (uVar1 != 0x0) {
      pass1_1010_715c(uVar1,param_2,param_4,0x1a);
    }
    if (param_5 == 0x2c) {
      pass1_1010_715c(uVar1,param_2,param_4,0x1d);
    }
    param_1 = pass1_1010_6ca2(param_2,param_4,0x2);
    if (param_1 == 0x0) {
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
    pass1_1010_715c(param_1,param_2,param_4,0x17);
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
    param_1 = pass1_1010_6ca2(param_2,param_4,0x2);
    if (param_1 == 0x0) {
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
    pass1_1010_715c(param_1,param_2,param_4,0x23);
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
    pass1_1010_715c(param_1,param_2,param_4,0x24);
    uVar1 = 0x3d;
    break;
  case 0x39:
    uVar1 = 0x25;
    break;
  case 0x3e:
    pass1_1010_715c(param_1,param_2,param_4,0x26);
    pass1_1010_715c(param_1,param_2,param_4,0x28);
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
    pass1_1010_715c(param_1,param_2,param_4,0x3c);
    uVar1 = 0x3e;
    break;
  case 0x5d:
    pass1_1010_715c(param_1,param_2,param_4,0x2f);
    uVar1 = 0x40;
    break;
  case 0x5e:
  case 0x60:
    uVar1 = 0x2f;
    break;
  case 0x5f:
    pass1_1010_715c(param_1,param_2,param_4,0x34);
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
    pass1_1010_715c(param_1,param_2,param_4,uVar1);
    uVar1 = 0xb;
  }
  pass1_1010_715c(param_1,param_2,param_4,uVar1);
  return 0x1;
}
