

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 pass1_1008_7006(param_1: *mut u8,param_2: *mut astruct_802,mut param_3: u16 ,mut param_4: u32,mut param_5: u16 )

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  let mut puVar4: *mut u32;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut in_stack_0000fff8: u16;
  let mut iStack4: i16;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  iStack4 = 0;
  while( true ) {
    if (PTR_LOOP_1050_0334 <= iStack4) {
      return 0x1;
    }
    puVar4 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,
                             (u8 **)CONCAT22(in_stack_0000fff8,(iStack4 * 0x2 + 0x320)),
                             in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
    paVar3 = (astruct_57 *)(paVar3 & 0xffff0000 | puVar4 >> 0x10);
    in_stack_0000fff8 = SUB42(puVar4,0x0);
    ppcVar1 = (code **)(*puVar4 + 0x8);
    iVar2 = (**ppcVar1)(0x1010,in_stack_0000fff8,(puVar4 >> 0x10),param_4);
    if (iVar2 == 0) break;
    iStack4 += 0x1;
  }
  return iVar2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i16 pass1_1008_7056(param_1: *mut u8,mut param_2: u16 ,mut param_3: u16 ,mut param_4: u32)

{
  code **ppcVar1;
  let mut iVar2: i16;
  let mut in_register_0000000a: u16;
  let mut paVar3: *mut Struct57;
  let mut puVar4: *mut u32;
  let mut in_stack_0000fea0: u16;
  let mut in_stack_0000ffc4: u16;
  let mut in_stack_0000ffca: u16;
  let mut in_stack_0000ffce: u16;
  let mut in_stack_0000fff8: u16;
  let mut iStack4: i16;

  paVar3 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  iStack4 = 0;
  while( true ) {
    if (PTR_LOOP_1050_0334 <= iStack4) {
      return 0x1;
    }
    puVar4 = mixed_1010_20ba(paVar3,_u16_1050_0ed0,
                             (u8 **)CONCAT22(in_stack_0000fff8,(iStack4 * 0x2 + 0x320)),
                             in_stack_0000fea0,in_stack_0000ffc4,in_stack_0000ffca,in_stack_0000ffce);
    paVar3 = (astruct_57 *)(paVar3 & 0xffff0000 | puVar4 >> 0x10);
    in_stack_0000fff8 = SUB42(puVar4,0x0);
    ppcVar1 = (code **)(*puVar4 + 0xc);
    iVar2 = (**ppcVar1)(0x1010,in_stack_0000fff8,(puVar4 >> 0x10),param_4);
    if (iVar2 == 0) break;
    iStack4 += 0x1;
  }
  return iVar2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 write_to_file_1008_70a6(astruct_802 *struct802_param_1)

{
  HFILE16 hfile_1;
  let mut uVar1: u16;
  let mut i16_var2: i16;
  astruct_802 *i16_var4;
  let mut i16_var5: u16;
  let mut i16_var6: u16;
  u8 in_AF;
  let mut uVar2: u32;
  HFILE16 hfile_2;

  i16_var5 = (struct802_param_1 >> 0x10);
  i16_var4 = (astruct_802 *)struct802_param_1;
  if (i16_var4->hfile_0x4 != 0xffff) {
    _lclose16(i16_var4->hfile_0x4);
    i16_var4->hfile_0x4 = 0xffff;
  }
  hfile_2 = 0;
  hfile_1 = _lcreat16(0x0,struct802_param_1->filename_0x0);
  i16_var4->hfile_0x4 = hfile_1;
  if (hfile_1 == 0xffff) {
    u16_1050_0310 = 0x6cf;
  }
  else {
    u16_1050_0312 = 0x4;
    sys_1000_3f9c(s__1050_65a0,_PTR_s_SC_03d_1050_0314_1050_031c,0x4);
    uVar1 = str_op_1000_3da4(s__1050_65a0);
    i16_var2 = uVar1 >> 0xf;
    uVar2 = _hwrite16(CONCAT22(0x65a0,i16_var2),CONCAT22(i16_var4->hfile_0x4,0x1050),hfile_2);
    if (uVar2 == CONCAT22(uVar1,i16_var2)) {
      return 0x1;
    }
    u16_1050_0310 = 0x6d0;
  }
  return 0x0;
}



// WARNING: Restarted to delay deadcode elimination for space: stack

BOOL16 read_file_1008_7146(mut param_1: u16 ,astruct_806 *struct_param_1,mut param_3: u16 ,mut param_4: u16 ,mut param_5: u16 )

{
  HFILE16 hfile_1;
  let mut uVar1: u16;
  astruct_806 *struct_1;
  let mut uVar2: u16;

  uVar2 = (struct_param_1 >> 0x10);
  struct_1 = (astruct_806 *)struct_param_1;
  if (struct_1->hfile_0x4 != 0xffff) {
    _lclose16(struct_1->hfile_0x4);
    struct_1->hfile_0x4 = 0xffff;
  }
  hfile_1 = _lopen16(0x0,struct_param_1->path_0x0);
  struct_1->hfile_0x4 = hfile_1;
  if (hfile_1 == 0xffff) {
    u16_1050_0310 = 0x6cf;
  }
  else {
    uVar1 = read_file_1008_71a0((astruct_806 *)(struct_param_1 & 0xffff),param_1);
    if (uVar1 != 0) {
      return 0x1;
    }
  }
  return 0x0;
}



// WARNING: Removing unreachable block (ram,0x100871e6)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 read_file_1008_71a0(param_1: *mut astruct_806,mut param_2: u16 )

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut iVar3: i32;
  let mut iStack26: i16;
  let mut iStack24: i16;
  let mut iStack22: i16;
  char local_e [0x9];
  u8 uStack5;
  let mut uStack4: u16;

  uStack4 = 0x1;
  uVar1 = str_op_1000_3da4(s__1050_65a0);
  iStack22 = 0;
  iVar3 = WIN16_hread(uVar1,(void *)CONCAT22(0x1050,local_e),*(HFILE16 *)(param_1 + 0x4));
  uVar2 = iVar3;
  if (uVar1 < iVar3) {
    uVar2 = uVar1;
  }
  iStack24 = uVar2 - 0x2;
  if (iStack24 < 0x0) {
    iStack24 = 0;
  }
  iStack26 = 0x2;
  while (iStack24 != 0) {
    iStack22 = iStack22 * 0xa + local_e[iStack26] + -0x30;
    iStack26 += 0x1;
    iStack24 = iStack24 + -0x1;
  }
  if (iStack22 == 1) {
    u16_1050_0312 = 0x1;
  }
  else if (iStack22 == 0x4) {
    u16_1050_0312 = 0x4;
  }
  else {
    uStack5 = '\0';
    u16_1050_0312 = 0x1;
    uStack4 = 0;
  }
  sys_1000_3f9c(s__1050_65a0,_PTR_s_SC_03d_1050_0314_1050_031c,u16_1050_0312);
  return uStack4;
}



BOOL16 file_fn_1008_726c(param_1: *mut astruct_802,mut param_2: u16 )

{
  HFILE16 hfile_1;

  if (param_1->hfile_0x4 != 0xffff) {
    hfile_1 = _lclose16(param_1->hfile_0x4);
    if (hfile_1 == 0xffff) {
      u16_1050_0310 = 0x6d1;
      return 0x0;
    }
    param_1->hfile_0x4 = 0xffff;
    u16_1050_0310 = 0;
  }
  return 0x1;
}



u16 * pass1_1008_72a8(param_1: *mut u16,mut param_2: u16 )

{
  *param_1 = param_2;
  return param_1;
}



u16 switch_1008_72bc(HFILE16 *param_1,mut param_2: u16 )

{
  if (u16_1050_0312 < 0x2) {
    switch(param_2) {
    case 0x1:
      param_2 = 0x1;
      break;
    case 0x2:
      param_2 = 0x2;
      break;
    case 0x3:
      param_2 = 0x3;
      break;
    case 0x4:
      param_2 = 0x5;
      break;
    case 0x5:
      param_2 = 0x4;
      break;
    case 0x6:
      param_2 = 0x6;
      break;
    case 0x7:
      param_2 = 0x7;
      break;
    case 0x8:
      param_2 = 0x8;
      break;
    case 0x9:
      param_2 = 0x9;
      break;
    case 0xa:
      param_2 = 0xa;
      break;
    case 0xb:
      param_2 = 0xb;
      break;
    case 0xc:
      param_2 = 0xc;
      break;
    case 0xd:
      param_2 = 0xd;
      break;
    case 0xe:
      param_2 = 0xe;
      break;
    case 0xf:
      param_2 = 0xf;
      break;
    case 0x10:
      return 0x10;
    case 0x11:
      return 0x11;
    case 0x12:
      return 0x12;
    case 0x13:
      return 0x13;
    default:
      return 0x0;
    }
  }
  return param_2;
}



u16 pass1_1008_738c(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 )

{
  let mut uVar1: u16;

  switch(param_3) {
  case 0x1:
    uVar1 = 0x3;
    break;
  case 0x2:
    uVar1 = 0x4;
    break;
  case 0x3:
    return 0x5;
  case 0x4:
    return 0x6;
  case 0x5:
    return 0x8;
  case 0x6:
    return 0x9;
  case 0x7:
    return 0xa;
  default:
    uVar1 = 0;
  }
  return uVar1;
}



i16 switch_1008_73ea(mut param_1: u16 ,mut param_2: u16 ,mut param_3: i16)

{
  let mut iStack4: i16;

  iStack4 = param_3;
  if (u16_1050_0312 < 0x2) {
    switch(param_3) {
    case 0x18:
    case 0x19:
    case 0x1a:
    case 0x1b:
    case 0x1c:
    case 0x1d:
    case 0x1e:
    case 0x1f:
    case 0x20:
    case 0x21:
    case 0x22:
    case 0x23:
    case 0x24:
    case 0x25:
    case 0x26:
    case 0x27:
    case 0x28:
    case 0x29:
    case 0x2a:
    case 0x2b:
    case 0x2c:
    case 0x2d:
    case 0x2e:
    case 0x2f:
    case 0x30:
    case 0x31:
    case 0x32:
    case 0x33:
    case 0x34:
    case 0x35:
    case 0x36:
    case 0x37:
    case 0x38:
    case 0x39:
    case 0x3a:
    case 0x3b:
    case 0x3c:
      iStack4 = param_3 + 0x3;
      break;
    case 0x3d:
    case 0x3e:
      iStack4 = param_3 + 0x4;
      break;
    case 0x3f:
    case 0x40:
    case 0x41:
    case 0x42:
    case 0x43:
    case 0x44:
    case 0x45:
    case 0x46:
    case 0x47:
    case 0x48:
    case 0x49:
    case 0x4a:
    case 0x4b:
    case 0x4c:
    case 0x4d:
    case 0x4e:
    case 0x4f:
    case 0x50:
    case 0x51:
    case 0x52:
    case 0x53:
    case 0x54:
    case 0x55:
    case 0x56:
    case 0x57:
    case 0x58:
    case 0x59:
    case 0x5a:
    case 0x5b:
    case 0x5c:
    case 0x5d:
    case 0x5e:
    case 0x5f:
    case 0x60:
    case 0x61:
    case 0x62:
    case 0x63:
    case 0x64:
    case 0x65:
    case 0x66:
      iStack4 = param_3 + 0x8;
      break;
    case 0x67:
    case 0x68:
    case 0x69:
    case 0x6a:
    case 0x6b:
    case 0x6c:
    case 0x6d:
    case 0x6e:
    case 0x6f:
    case 0x70:
    case 0x71:
    case 0x72:
    case 0x73:
    case 0x74:
    case 0x75:
    case 0x76:
    case 0x77:
    case 0x78:
    case 0x79:
    case 0x7a:
    case 0x7b:
    case 0x7c:
    case 0x7d:
    case 0x7e:
    case 0x7f:
    case 0x80:
      iStack4 = param_3 + 0x9;
    }
  }
  return iStack4;
}
pub fn file_1008_7548(HFILE16 *hfile_param,i32 *param_2,mut param_3: u32)

{
  code **ppcVar1;
  let mut file_read_ok: bool;
  let mut uVar2: u16;
  let mut buffer_3: *mut u8;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut buffer_4: *mut Struct57;
  let mut unaff_CS: u16;
  let mut lVar5: i32;
  let mut read_buffer_1c: *mut u8;
  u16 local_18 [0x5];
  let mut puStack14: *mut u8;
  let mut uStack10: u32;
  let mut read_buffer: *mut u8;

  uVar4 = (param_3 >> 0x10);
  read_buffer = NULL;
  file_read_ok = read_file_1008_7dee(hfile_param,CONCAT22(0x1050,&read_buffer),0x4);
  if (file_read_ok == 0) {
    return;
  }
  if (read_buffer != NULL) {
    buffer_3 = read_buffer;
    if (read_buffer < 0xc8) {
      read_buffer = 0;
      buffer_3 = 0xc8;
    }
    buffer_4 = (astruct_57 *)CONCAT22(uVar4,read_buffer);
    uVar2 = buffer_3;
    uStack10 = buffer_3 & 0xffff | read_buffer << 0x10;
    if (*param_2 == 0) {
      unaff_CS = 0x1000;
      mem_op_1000_179c(0x1e,buffer_4);
      uVar3 = buffer_4 | uVar2;
      if (uVar3 == 0) {
        *param_2 = 0;
      }
      else {
        unaff_CS = 0x1020;
        struct_1020_c444((astruct_75 *)CONCAT22(buffer_4,uVar2),0x64,uStack10);
        param_2 = uVar2;
        (param_2 + 0x2) = uVar3;
      }
    }
    lVar5 = *param_2;
    ppcVar1 = (code **)(*param_2 + 0x24);
    (**ppcVar1)();
    for (puStack14 = NULL; puStack14 < read_buffer; puStack14 = puStack14 + 1) {
      file_read_ok = read_file_1008_7dee(hfile_param,CONCAT22(0x1050,&read_buffer_1c),0x4);
      if ((file_read_ok == 0) ||
         (file_read_ok = read_file_1008_7dee(hfile_param,CONCAT22(0x1050,local_18),0x2), file_read_ok == 0)) {
        ppcVar1 = (code **)(*param_2 + 0x1c);
        (**ppcVar1)(unaff_CS,*param_2);
        return;
      }
      ppcVar1 = (code **)(*param_2 + 0x28);
      (**ppcVar1)(unaff_CS,*param_2,(*param_2 >> 0x10),local_18[0],read_buffer_1c);
    }
    ppcVar1 = (code **)(*param_2 + 0x1c);
    (**ppcVar1)(unaff_CS,*param_2,lVar5);
  }
  return;
}
pub fn pass1_1008_766e(param_1: *mut u8,mut param_2: u32,param_3: *mut astruct_169)

{
  let mut puVar1: *mut u32;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  let mut in_stack_0000fe8c: u16;
  let mut in_stack_0000ffb0: u16;
  let mut in_stack_0000ffb6: u16;
  let mut in_stack_0000ffba: u16;
  let mut in_stack_0000ffe4: u16;
  let mut local_6: u32;
  let mut paVar5: *mut Struct57;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  param_3 = 0;
  local_6 = 0;
  puVar1 = &local_6;
  file_1008_76e4(paVar4,(HFILE16 *)param_2,(i32 *)CONCAT22(0x1050,puVar1));
  if (puVar1 != NULL) {
    if (local_6 != 0) {
      mem_op_1000_179c(0xc,paVar4);
      uVar2 = paVar4 | puVar1;
      paVar5 = (astruct_57 *)(paVar4 & 0xffff0000 | uVar2);
      if (uVar2 == 0) {
        puVar1 = NULL;
        uVar3 = 0;
      }
      else {
        pass1_1010_8ef2(paVar5,(astruct_170 *)CONCAT22(paVar4,puVar1),in_stack_0000ffe4,in_stack_0000fe8c,
                        in_stack_0000ffb0,in_stack_0000ffb6,in_stack_0000ffba);
        uVar3 = SUB42(paVar5,0x0);
      }
      param_3 = puVar1;
      (param_3 + 0x2) = uVar3;
      fn_ptr_1010_905e(*(astruct_169 **)param_3,local_6);
    }
    return;
  }
  return;
}
pub fn file_1008_76e4(param_1: *mut astruct_57,HFILE16 *param_2,i32 *param_3)

{
  code **ppcVar1;
  let mut uVar2: u16;
  let mut BVar3: bool;
  let mut uVar4: u16;
  u8 local_18 [0xe];
  let mut puStack10: *mut u8;
  let mut buffer_6: *mut u8;

  buffer_6 = NULL;
  uVar2 = read_file_1008_7dee(param_2,CONCAT22(0x1050,&buffer_6),0x4);
  if (uVar2 == 0) {
    return;
  }
  if (buffer_6 != NULL) {
    if (*param_3 == 0) {
      mem_op_1000_179c(0x18,param_1);
      uVar4 = param_1 | uVar2;
      if (uVar4 == 0) {
        *param_3 = 0;
      }
      else {
        struct_op_1030_1cd8((astruct_75 *)CONCAT22(param_1,uVar2),0x5,buffer_6);
        param_3 = uVar2;
        (param_3 + 0x2) = uVar4;
      }
    }
    ppcVar1 = (code **)(*param_3 + 0x14);
    (**ppcVar1)();
    for (puStack10 = NULL; puStack10 < buffer_6; puStack10 = puStack10 + 1) {
      BVar3 = read_file_1008_7dee(param_2,CONCAT22(0x1050,local_18),0x4);
      if (BVar3 == 0) {
        return;
      }
      ppcVar1 = (code **)(*param_3 + 0x18);
      (**ppcVar1)();
    }
    ppcVar1 = (code **)(*param_3 + 0x1c);
    (**ppcVar1)();
  }
  return;
}



u16 file_1008_77cc(mut param_1: u16 ,mut param_2: u32,i32 *param_3)

{
  let mut uVar1: u16;
  let mut BVar2: bool;
  let mut uVar3: u16;
  let mut in_register_0000000a: u16;
  let mut paVar4: *mut Struct57;
  u16 local_14 [0x2];
  u32 local_10 [0x2];
  let mut uStack6: u16;
  let mut local_4: u16;

  paVar4 = (astruct_57 *)CONCAT22(in_register_0000000a,param_1);
  local_4 = 0;
  uVar1 = read_file_1008_7dee((HFILE16 *)param_2,CONCAT22(0x1050,&local_4),0x2);
  if (uVar1 == 0) {
    return 0x0;
  }
  if (local_4 != 0) {
    if (*param_3 == 0) {
      mem_op_1000_179c(0xa,paVar4);
      uVar3 = paVar4 | uVar1;
      if (uVar3 == 0) {
        *param_3 = 0;
      }
      else {
        pass1_1020_ba3e(CONCAT22(paVar4,uVar1),0x5,0x5);
        param_3 = uVar1;
        (param_3 + 0x2) = uVar3;
      }
    }
    for (uStack6 = 0; uStack6 < local_4; uStack6 += 1) {
      BVar2 = read_file_1008_7dee((HFILE16 *)param_2,CONCAT22(0x1050,local_14),0x2);
      if (BVar2 == 0) {
        return 0x0;
      }
      BVar2 = read_file_1008_7dee((HFILE16 *)param_2,CONCAT22(0x1050,local_10),0x4);
      if (BVar2 == 0) {
        return 0x0;
      }
      pass1_1020_bb8a((i32 *)*param_3,local_10[0],CONCAT22(local_14[0],(local_10[0] >> 0x10)));
    }
  }
  return 0x1;
}
pub fn pass1_1008_7898(mut param_1: u16 ,mut param_2: u32,u32 *param_3)

{
  code **ppcVar1;
  let mut BVar2: bool;
  let mut extraout_DX: u16;
  let mut uVar3: u16;
  HFILE16 in_stack_0000ffc4;
  let mut local_26: u16;
  u32 local_24 [0x3];
  let mut local_18: u32;
  u16 local_14 [0x5];
  let mut uStack10: u32;
  let mut uStack6: u32;

  if (param_3 == NULL) {
    param_1 = 0;
    uVar3 = 0;
  }
  else {
    ppcVar1 = (code **)(*param_3 + 0x10);
    (**ppcVar1)();
    uVar3 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar3,param_1);
  local_18 = CONCAT22(uVar3,param_1);
  BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_18),0x4,in_stack_0000ffc4);
  if (BVar2 != 0) {
    uStack10 = 0;
    while( true ) {
      if (uStack6 <= uStack10) {
        return;
      }
      pass1_1020_c4a8(param_3,CONCAT22(0x1050,local_14),CONCAT22(0x1050,&local_18),
                      uStack10);
      local_24[0] = local_18;
      BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,local_24),0x4,in_stack_0000ffc4);
      if (BVar2 == 0) break;
      local_26 = local_14[0];
      BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_26),0x2,in_stack_0000ffc4);
      if (BVar2 == 0) {
        u16_1050_0310 = 0x6d0;
        return;
      }
      uStack10 += 0x1;
    }
  }
  u16_1050_0310 = 0x6d0;
  return;
}



u16_t write_to_file_1008_7954(u16_t param_1,param_2: *mut u8,u32 *param_3)

{
  code **ppcVar1;
  let mut BVar2: bool;
  let mut uVar3: u32;
  let mut extraout_DX: u16;
  let mut uVar4: u16;
  let mut extraout_DX_00: u16;
  HFILE16 in_stack_0000ffca;
  let mut local_20: u16;
  let mut uStack30: u16;
  let mut local_18: u16;
  let mut uStack22: u16;
  let mut uStack10: u32;
  let mut uStack6: u32;

  if (param_3 == NULL) {
    param_1 = 0;
    uVar4 = 0;
  }
  else {
    ppcVar1 = (code **)(*param_3 + 0x10);
    (**ppcVar1)();
    uVar4 = extraout_DX;
  }
  uStack6 = CONCAT22(uVar4,param_1);
  local_18 = param_1;
  uStack22 = uVar4;
  BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_18),0x4,in_stack_0000ffca);
  if (BVar2 != 0) {
    uStack10 = 0;
    while( true ) {
      if (uStack6 <= uStack10) {
        return uVar4;
      }
      ppcVar1 = (code **)(*param_3 + 0x4);
      uVar3 = uStack6;
      (**ppcVar1)();
      local_20 = uVar3;
      uVar4 = extraout_DX_00;
      uStack30 = extraout_DX_00;
      local_18 = local_20;
      uStack22 = extraout_DX_00;
      BVar2 = write_to_file_1008_7e1c(param_2,CONCAT22(0x1050,&local_20),0x4,in_stack_0000ffca);
      if (BVar2 == 0) break;
      uStack10 += 0x1;
    }
  }
  u16_1050_0310 = 0x6d0;
  return uVar4;
}
pub fn pass1_1008_79f0(mut param_1: u32,i32 param_2)

{
  let mut uVar1: u16;
  let mut uVar2: u16;
  let mut uStack4: u16;

  if (param_2 == 0) {
    uVar1 = 0;
    uStack4 = 0;
  }
  else {
    uVar2 = (param_2 >> 0x10);
    uVar1 = *(param_2 + 0x4);
    uStack4 = (param_2 + 0x6);
  }
  write_to_file_1008_7954(uVar1,param_1,CONCAT22(uStack4,uVar1));
  return;
}
pub fn write_to_file_1008_7a22(param_1: *mut u8,i32 param_2)

{
  let mut BVar1: bool;
  HFILE16 in_stack_0000ffc6;
  u32 local_24 [0x2];
  u16 local_1c [0x5];
  let mut local_12: u16;
  let mut local_10: u32;
  let mut uStack10: u16;
  let mut uStack6: u16;
  let mut uStack4: u16;

  if (param_2 == 0) {
    uStack4 = 0;
  }
  else {
    uStack4 = (param_2 + 0x4);
  }
  local_12 = uStack4;
  BVar1 = write_to_file_1008_7e1c(param_1,CONCAT22(0x1050,&local_12),0x2,in_stack_0000ffc6);
  if (BVar1 == 0) {
    u16_1050_0310 = 0x6d0;
  }
  else {
    uStack6 = 0;
    while( true ) {
      if (uStack4 <= uStack6) {
        return;
      }
      pass1_1020_bb16(param_2,CONCAT22(0x1050,&local_10),CONCAT22(0x1050,&local_12),uStack6)
      ;
      uStack10 = local_12;
      local_1c[0] = local_12;
      BVar1 = write_to_file_1008_7e1c(param_1,CONCAT22(0x1050,local_1c),0x2,in_stack_0000ffc6);
      if (BVar1 == 0) break;
      local_24[0] = local_10;
      BVar1 = write_to_file_1008_7e1c(param_1,CONCAT22(0x1050,local_24),0x4,in_stack_0000ffc6);
      if (BVar1 == 0) {
        return;
      }
      uStack6 += 0x1;
    }
  }
  return;
}



u16 pass1_1008_7ad4(mut param_1: u32,i32 *param_2)

{
  let mut BVar1: bool;
  u16 local_14 [0x2];
  u32 local_10 [0x2];
  let mut uStack6: u16;
  let mut local_4: u16;

  BVar1 = read_file_1008_7dee((HFILE16 *)param_1,CONCAT22(0x1050,&local_4),0x2);
  if (BVar1 != 0) {
    uStack6 = 0;
    while( true ) {
      if (local_4 <= uStack6) {
        return 0x1;
      }
      BVar1 = read_file_1008_7dee((HFILE16 *)param_1,CONCAT22(0x1050,local_14),0x2);
      if ((BVar1 == 0) ||
         (BVar1 = read_file_1008_7dee((HFILE16 *)param_1,CONCAT22(0x1050,local_10),0x4), BVar1 == 0)) break;
      pass1_1020_bb8a(param_2,local_10[0],CONCAT22(local_14[0],(local_10[0] >> 0x10)));
      uStack6 += 0x1;
    }
  }
  return 0x0;
}



u16 write_to_file_1008_7b4c(param_1: *mut u8,param_2: *mut astruct_615)

{
  let mut BVar1: bool;
  HFILE16 in_stack_0000ffd4;
  u16 local_12 [0x3];
  u16 local_c [0x2];
  let mut local_8: u16;
  let mut local_6: u16;
  let mut local_4: u16;

  pass1_1008_3eb4(param_2,CONCAT22(0x1050,&local_8),CONCAT22(0x1050,&local_6),
                  CONCAT22(0x1050,&local_4));
  local_12[0] = local_4;
  BVar1 = write_to_file_1008_7e1c(param_1,CONCAT22(0x1050,local_12),0x2,in_stack_0000ffd4);
  if (BVar1 != 0) {
    local_c[0] = local_6;
    BVar1 = write_to_file_1008_7e1c(param_1,CONCAT22(0x1050,local_c),0x2,in_stack_0000ffd4);
    if (BVar1 != 0) {
      local_c[0] = local_8;
      BVar1 = write_to_file_1008_7e1c(param_1,CONCAT22(0x1050,local_c),0x2,in_stack_0000ffd4);
      if (BVar1 != 0) {
        return 0x1;
      }
    }
  }
  return 0x0;
}



BOOL16 read_file_1008_7bc8(mut param_1: u32,param_2: *mut u16)

{
  let mut BVar1: bool;
  let mut local_8: u16;
  let mut local_6: u32;

  BVar1 = read_file_1008_7dee((HFILE16 *)param_1,CONCAT22(0x1050,&local_6 + 0x2),0x2);
  if (BVar1 != 0) {
    BVar1 = read_file_1008_7dee((HFILE16 *)param_1,CONCAT22(0x1050,&local_6),0x2);
    if (BVar1 != 0) {
      BVar1 = read_file_1008_7dee((HFILE16 *)param_1,CONCAT22(0x1050,&local_8),0x2);
      if (BVar1 != 0) {
        pass1_1008_3e76(param_2,local_8,local_6,(local_6 >> 0x10));
        return 0x1;
      }
    }
  }
  return 0x0;
}



BOOL16 pass1_1008_7c2a(mut param_1: u32,param_2: *mut c_char)

{
  let mut uVar1: u16;
  let mut BVar2: bool;
  HFILE16 in_stack_0000ffe6;

  if (param_2 != NULL) {
    uVar1 = str_op_1000_3da4(param_2);
    BVar2 = write_to_file_1008_7e1c(param_1,param_2,(uVar1 + 1),in_stack_0000ffe6);
    return BVar2;
  }
  write_to_file_1008_7e1c(param_1,(s_playerName_1050_148e + 0xc),0x1,in_stack_0000ffe6);
  return 0x1;
}
pub fn read_file_1008_7c6e(HFILE16 param_1,mut param_2: u16 ,char *param_3)

{
  let mut pcVar1: *mut c_char;
  char local_c [0xa];

  while( true ) {
    pcVar1 = param_3;
    WIN16_hread(0x1,(void *)CONCAT22(0x1050,local_c),*_param_1);
    if (local_c[0] == '\0') break;
    param_3 = (param_3 & 0xffff0000 | (param_3 + 1));
    *pcVar1 = local_c[0];
  }
  *param_3 = '\0';
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

BOOL16 write_to_file_1008_7cac(param_1: *mut u8)

{
  let mut uVar1: u16;
  let mut BVar2: bool;
  HFILE16 in_stack_0000ffde;
  u8 local_c [0xa];

  sys_1000_3f9c(CONCAT22(0x1050,local_c),s__s_02x_1050_0340,_PTR_s_dcbSC_1050_0336_1050_033c);
  uVar1 = str_op_1000_3da4(CONCAT22(0x1050,local_c));
  BVar2 = write_to_file_1008_7e1c(param_1,CONCAT22(0x1050,local_c),uVar1,in_stack_0000ffde);
  if (BVar2 == 0) {
    u16_1050_0310 = 0x6d0;
    return BVar2;
  }
  return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub fn read_file_1008_7cfe(mut param_1: u16 ,mut param_2: u16 ,mut param_3: u16 )

{
  let mut bVar1: bool;
  let mut uVar2: u16;
  let mut iVar3: i32;
  let mut in_stack_0000fbd6: u16;
  let mut in_stack_0000fbd8: u16;
  let mut uStack1040: u32;
  char local_406 [0x400];
  let mut uStack6: u32;

  uStack6 = 0;
  bVar1 = false;
  loop {
    _llseek16(0x0,uStack6,*(HFILE16 *)CONCAT22(param_2,param_1));
    iVar3 = WIN16_hread(0x400,(void *)CONCAT22(0x1050,local_406),*(HFILE16 *)CONCAT22(param_2,param_1));
    for (uStack1040 = 0; uStack1040 < iVar3; uStack1040 += 1) {
      if (local_406[uStack1040] == *_PTR_s_dcbSC_1050_0336_1050_033c) {
        if (!bVar1) {
          bVar1 = true;
          uStack6 = CONCAT22((uStack6 >> 0x10) + uStack1040 +
                             CARRY2(uStack6,uStack1040),uStack6 + uStack1040);
          break;
        }
        bVar1 = false;
        uVar2 = pass1_1008_7e4a(CONCAT22(0x1050,local_406 + uStack1040),in_stack_0000fbd6,
                                in_stack_0000fbd8);
        if (uVar2 != 0) {
          _llseek16(0x0,uStack1040 + uStack6 + 0x7,*(HFILE16 *)CONCAT22(param_2,param_1));
          return;
        }
      }
    }
    if (!bVar1) {
      if (iVar3 < 0x400) {
        return;
      }
      uStack6 = CONCAT11(uStack6._1_1_ + 0x4,uStack6);
      uStack6 = CONCAT22((uStack6 >> 0x10) + (0xfb < uStack6._1_1_),uStack6);
    }
  } while( true );
}



BOOL16 read_file_1008_7dee(HFILE16 *hfile_param_1,u8 *buffer_param_2,u32 count_param_3)

{
  let mut read_count: i32;

  read_count = WIN16_hread(count_param_3,buffer_param_2,*hfile_param_1);
  if ((read_count == count_param_3) && ((read_count >> 0x10) == count_param_3)) {
    return 0x1;
  }
  return 0x0;
}



BOOL16 write_to_file_1008_7e1c(u8 *buffer,count: u32,char *buf_to_write,HFILE16 param_4)

{
  let mut uVar1: u32;
  let mut uStackY16: u16;
  let mut uVar2: u16;

  uStackY16 = SUB42(buf_to_write,0x0);
  uVar2 = (buf_to_write >> 0x10);
  uVar1 = _hwrite16(CONCAT22(count,uVar2),CONCAT22(buffer,(count >> 0x10)),param_4);
  if (uVar1 != CONCAT22(uStackY16,uVar2)) {
    return 0x0;
  }
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1008_7e4a(param_1: *mut c_char,mut param_2: u16 ,u16_t param_3)

{
  let mut uVar1: u16;

  sys_1000_3f9c(CONCAT22(0x1050,&param_2),s__s_02x_1050_0347,_PTR_s_dcbSC_1050_0336_1050_033c);
  uVar1 = str_op_1000_3da4(CONCAT22(0x1050,&param_2));
  uVar1 = pass1_1000_3de8(param_1,CONCAT22(0x1050,&param_2),uVar1,param_2,param_3);
  if (uVar1 == 0) {
    return 0x1;
  }
  return 0x0;
}



u16 * pass1_1008_7e98(param_1: *mut astruct_460,param_2: u8)

{
  astruct_460 *uVar1;
  astruct_460 *uVar2;

  uVar2 = (astruct_460 *)(param_1 >> 0x10);
  uVar1 = (astruct_460 *)param_1;
  param_1 = 0x380a;
  uVar1->field2_0x2 = 0x1008;
  param_1 = 0x389a;
  uVar1->field2_0x2 = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}



astruct_20 * unk_draw_op_1008_7f62(param_1: *mut astruct_20,mut param_2: u16 ,mut param_3: u32)

{
  HGDIOBJ16 HVar1;
  HCURSOR16 HVar2;
  astruct_20 *iVar4;
  let mut uVar3: u16;
  astruct_20 *uVar4;
  astruct_20 *iVar3;

  set_struct_1008_687a(param_1,param_3);
  uVar3 = (param_1 >> 0x10);
  iVar4 = (astruct_20 *)param_1;
  iVar4->field164_0xde = param_2;
  param_1->offset_0x0 = 0x8042;
  iVar4->base_0x2 = 0x1008;
  unk_str_op_1000_3d3e((param_1 & 0xffff0000 | ZEXT24(&iVar4->field60_0x5b)),s_SOLChildPar_1050_0358);
  HVar1 = GetStockObject16(HOLLOW_BRUSH);
  iVar4->hgdiobj_field_0xc6 = HVar1;
  HVar2 = LoadCursor16(0x7f00,0x0);
  iVar4->hcursor_field_0xc4 = HVar2;
  iVar4->field150_0xc8 = 0x2008;
  iVar4->field139_0xac = 0x44000000;
  iVar4->field145_0xbc = (param_3 + 0x8);
  iVar4->field151_0xca = iVar4->field164_0xde;
  win_ui_reg_class_1008_96d2((StructA *)param_1);
  return param_1;
}
pub fn pass1_1008_7ffa(param_1: *mut astruct_461,param_2: u8)

{
  astruct_461 *uVar1;
  let mut uVar2: u16;

  uVar1 = (astruct_461 *)param_1;
  uVar1 = uVar1 + 1;
  pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(uVar1)));
  uVar2 = (param_1 >> 0x10);
  param_1 = 0x380a;
  uVar1->field2_0x2 = 0x1008;
  param_1 = 0x389a;
  uVar1->field2_0x2 = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return;
}
